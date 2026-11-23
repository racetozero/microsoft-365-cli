//! Option validation that reproduces upstream's zod v4 schemas.
//!
//! Field checks run in schema order. A wrong type, a missing required value
//! or an invalid enum value is an *aborting* issue; format checks and
//! refinements are not. Object-level refinements run only when no field has
//! an aborting issue, which is how zod v4 behaves.

use crate::json::{Obj, js_len, js_number, num_value, to_js_string, truthy};
use crate::manifest::{Check, CmdMeta, FType, FieldMeta, Lit, Msg, RefineCode, Rule, Validator};
use crate::validation as v;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Issue {
    pub path: Option<String>,
    pub message: String,
    /// The value is missing (`invalid_type`/`invalid_value` on an absent key).
    pub missing: bool,
    pub code: RefineCode,
    pub set_options: &'static [&'static str],
}

impl Issue {
    fn field(path: &str, message: String) -> Issue {
        Issue {
            path: Some(path.to_string()),
            message,
            missing: false,
            code: RefineCode::None,
            set_options: &[],
        }
    }

    pub fn render(&self, missing_as_required: bool) -> String {
        let m = if missing_as_required && self.missing {
            "Required option not specified"
        } else {
            &self.message
        };
        match &self.path {
            Some(p) if !p.is_empty() => format!("{p}: {m}"),
            _ => m.to_string(),
        }
    }
}

/// A field predicate: `(check index, value, all options)`.
pub type FieldHook = fn(u16, &Value, &Obj) -> FieldCheck;
/// An object refinement: `(refine index, options)`.
pub type RefineHook = fn(usize, &Obj) -> Result<bool, String>;

/// Command-specific predicates that the manifest cannot express.
#[derive(Clone, Copy, Default)]
pub struct Hooks {
    /// `Check::Custom(index, ..)` on a field: whether the value passes.
    /// Return [`fail`] or [`fail_with`] from a [`FieldCheck`] closure.
    pub field_check: Option<FieldHook>,
    /// Unclassified object refinement at `index` in `CmdMeta::refines`:
    /// `Ok(pass)` or `Err(message)` for a failing refinement with a computed message.
    pub refine: Option<RefineHook>,
}

/// Result of a command's field check: `Ok(())` passes, `Err(None)` fails
/// with the upstream message template, `Err(Some(m))` fails with `m`.
pub type FieldCheck = std::result::Result<(), Option<String>>;

/// Converts a predicate into a [`FieldCheck`].
pub fn check(pass: bool) -> FieldCheck {
    if pass { Ok(()) } else { Err(None) }
}

pub fn received(v: &Value) -> &'static str {
    match v {
        Value::Null => "NaN",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn template(m: Msg, input: &Value, default: impl FnOnce() -> String) -> String {
    match m {
        Msg::Text(t) => t.replace("{input}", &to_js_string(input)),
        Msg::Default | Msg::Dynamic => default(),
    }
}

fn expected(t: FType) -> &'static str {
    match t {
        FType::String | FType::Enum => "string",
        FType::Boolean => "boolean",
        FType::Number => "number",
        FType::Array => "array",
        _ => "unknown",
    }
}

fn type_ok(t: FType, v: &Value) -> bool {
    match t {
        FType::String => v.is_string(),
        FType::Boolean => v.is_boolean(),
        FType::Number => v.is_number(),
        FType::Array => v.is_array(),
        _ => true,
    }
}

fn join_values(values: &[&str]) -> String {
    values
        .iter()
        .map(|x| format!("\"{x}\""))
        .collect::<Vec<_>>()
        .join("|")
}

fn validator(val: Validator, s: &str) -> v::Valid {
    use v::Valid;
    match val {
        Validator::Guid => Valid::from_bool(v::is_valid_guid(s)),
        Validator::GuidArray => v::is_valid_guid_array(s),
        Validator::UserPrincipalName => Valid::from_bool(v::is_valid_user_principal_name(s)),
        Validator::UserPrincipalNameArray => v::is_valid_user_principal_name_array(s),
        Validator::SharePointUrl => v::is_valid_sharepoint_url(s),
        Validator::ISODateTime => Valid::from_bool(v::is_valid_iso_date_time(s)),
        Validator::ISODuration => Valid::from_bool(v::is_valid_iso_duration(s)),
        Validator::ISODate => Valid::from_bool(v::is_valid_iso_date(s)),
        Validator::ISODateDashOnly => Valid::from_bool(v::is_valid_iso_date_dash_only(s)),
        Validator::GraphDateTime => Valid::from_bool(v::is_valid_graph_date_time(s)),
        Validator::PowerPagesUrl => Valid::from_bool(v::is_valid_power_pages_url(s)),
        Validator::PositiveInteger => Valid::from_bool(v::is_valid_positive_integer(s)),
        Validator::PositiveIntegerArray => v::is_valid_positive_integer_array(s),
        Validator::JsonOrEmpty => Valid::from_bool(v::is_valid_json_or_empty(s)),
        Validator::Theme => Valid::from_bool(v::is_valid_theme(s)),
        Validator::TeamsChannelId => Valid::from_bool(v::is_valid_teams_channel_id(s)),
        Validator::TeamsChatId => Valid::from_bool(v::is_valid_teams_chat_id(s)),
        Validator::Permission => v::is_valid_permission(s),
        Validator::MailNickname => Valid::from_bool(v::is_valid_mail_nickname(s)),
    }
}

static UUID_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(r"^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$").unwrap()
});

static GUID_RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
    regex::Regex::new(
        r"^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})$",
    )
    .unwrap()
});

/// Converts a value with `z.coerce.number()` semantics.
fn coerce_number(v: &Value) -> Value {
    match v {
        Value::Number(_) | Value::Null => v.clone(),
        Value::Bool(b) => Value::from(if *b { 1 } else { 0 }),
        Value::String(s) => {
            let n = js_number(s);
            if n.is_nan() {
                Value::Null
            } else {
                num_value(n)
            }
        }
        _ => Value::Null,
    }
}

struct FieldResult {
    value: Value,
    aborted: bool,
}

fn check_field(
    f: &FieldMeta,
    raw: &Value,
    opts: &Obj,
    hooks: &Hooks,
    issues: &mut Vec<Issue>,
    deferred: &mut Vec<Issue>,
) -> FieldResult {
    let mut value = raw.clone();
    if f.coerce {
        value = coerce_number(&value);
    }
    // type
    match f.t {
        FType::Enum => {
            if f.ci
                && let Value::String(s) = &value
            {
                let l = s.to_lowercase();
                value = f
                    .values
                    .iter()
                    .find(|x| x.to_lowercase() == l)
                    .map(|x| Value::from(*x))
                    .unwrap_or(Value::Null);
            }
            let ok = match &value {
                Value::String(s) => f.values.contains(&s.as_str()),
                _ => false,
            };
            if !ok {
                let msg = template(f.type_msg, raw, || {
                    if f.values.len() == 1 {
                        format!("Invalid input: expected \"{}\"", f.values[0])
                    } else {
                        format!("Invalid option: expected one of {}", join_values(f.values))
                    }
                });
                issues.push(Issue::field(f.name, msg));
                return FieldResult {
                    value,
                    aborted: true,
                };
            }
        }
        FType::Union => {
            let ok = f.union.iter().any(|t| type_ok(*t, &value));
            if !ok {
                issues.push(Issue::field(
                    f.name,
                    template(f.type_msg, raw, || "Invalid input".into()),
                ));
                return FieldResult {
                    value,
                    aborted: true,
                };
            }
        }
        FType::Number => {
            if !value.is_number() {
                let msg = template(f.type_msg, raw, || {
                    format!(
                        "Invalid input: expected number, received {}",
                        received(&value)
                    )
                });
                issues.push(Issue::field(f.name, msg));
                return FieldResult {
                    value,
                    aborted: true,
                };
            }
        }
        t => {
            if !type_ok(t, &value) {
                let msg = template(f.type_msg, raw, || {
                    format!(
                        "Invalid input: expected {}, received {}",
                        expected(t),
                        received(&value)
                    )
                });
                issues.push(Issue::field(f.name, msg));
                return FieldResult {
                    value,
                    aborted: true,
                };
            }
        }
    }
    let mut aborted = false;
    for c in f.checks {
        if aborted {
            break;
        }
        let failed: Option<String> = match *c {
            Check::MinLen(n, m) => value.as_str().filter(|s| js_len(s) < n as usize).map(|_| {
                template(m, &value, || {
                    format!("Too small: expected string to have >={n} characters")
                })
            }),
            Check::MaxLen(n, m) => value.as_str().filter(|s| js_len(s) > n as usize).map(|_| {
                template(m, &value, || {
                    format!("Too big: expected string to have <={n} characters")
                })
            }),
            Check::Gt(n, incl, m) => value
                .as_f64()
                .filter(|x| if incl { *x < n } else { *x <= n })
                .map(|_| {
                    template(m, &value, || {
                        format!(
                            "Too small: expected number to be {}{}",
                            if incl { ">=" } else { ">" },
                            crate::json::number_to_string(n)
                        )
                    })
                }),
            Check::Lt(n, incl, m) => value
                .as_f64()
                .filter(|x| if incl { *x > n } else { *x >= n })
                .map(|_| {
                    template(m, &value, || {
                        format!(
                            "Too big: expected number to be {}{}",
                            if incl { "<=" } else { "<" },
                            crate::json::number_to_string(n)
                        )
                    })
                }),
            Check::SafeInt(m) => match value.as_f64() {
                Some(x) if x.fract() != 0.0 => {
                    aborted = true;
                    Some(template(m, &value, || {
                        "Invalid input: expected int, received number".into()
                    }))
                }
                Some(x) if x > 9_007_199_254_740_991.0 => Some(template(m, &value, || {
                    "Too big: expected number to be <=9007199254740991".into()
                })),
                Some(x) if x < -9_007_199_254_740_991.0 => Some(template(m, &value, || {
                    "Too small: expected number to be >=-9007199254740991".into()
                })),
                _ => None,
            },
            Check::Uuid(m) => value
                .as_str()
                .filter(|s| !UUID_RE.is_match(s))
                .map(|_| template(m, &value, || "Invalid UUID".into())),
            Check::Guid(m) => value
                .as_str()
                .filter(|s| !GUID_RE.is_match(s))
                .map(|_| template(m, &value, || "Invalid GUID".into())),
            Check::Regex(p, m) => value.as_str().and_then(|s| {
                let ok = fancy_regex::Regex::new(p)
                    .ok()
                    .and_then(|r| r.is_match(s).ok())
                    .unwrap_or(true);
                (!ok).then(|| {
                    template(m, &value, || {
                        format!("Invalid string: must match pattern /{p}/")
                    })
                })
            }),
            Check::Pred(val, strict, m) => {
                let s = to_js_string(&value);
                let r = validator(val, &s);
                let pass = if strict { r.is_true() } else { r.truthy() };
                // array validators return the invalid items, which the
                // upstream messages print instead of the whole input
                let shown = match &r {
                    v::Valid::Msg(list) if val != Validator::SharePointUrl => {
                        Value::from(list.as_str())
                    }
                    _ => value.clone(),
                };
                (!pass).then(|| template(m, &shown, || "Invalid input".into()))
            }
            Check::Custom(idx, _, m, is_async) => {
                let r = hooks
                    .field_check
                    .map(|h| h(idx, &value, opts))
                    .unwrap_or(Ok(()));
                let msg = r.err().map(|custom| {
                    custom.unwrap_or_else(|| template(m, &value, || "Invalid input".into()))
                });
                if is_async {
                    if let Some(msg) = msg {
                        deferred.push(Issue::field(f.name, msg));
                    }
                    None
                } else {
                    msg
                }
            }
            Check::Expr(_, e, m) => (!crate::expr::field(e, &value))
                .then(|| template(m, &value, || "Invalid input".into())),
            Check::Overwrite(src) => {
                value = overwrite(src, value);
                None
            }
            Check::ItemsEnum(values) => {
                // the CLI reports an item issue under a string option as missing
                for (i, item) in to_js_string(&value).split(',').enumerate() {
                    if !values.contains(&item) {
                        // an invalid enum value aborts, as for a plain enum
                        aborted = true;
                        issues.push(Issue::field(
                            &format!("{}.{i}", f.name),
                            "Required option not specified".into(),
                        ));
                    }
                }
                None
            }
        };
        if let Some(msg) = failed {
            issues.push(Issue::field(f.name, msg));
        }
    }
    FieldResult { value, aborted }
}

fn overwrite(src: &str, value: Value) -> Value {
    match (&value, src) {
        (Value::String(s), x) if x.contains("toLowerCase") => Value::from(s.to_lowercase()),
        (Value::String(s), x) if x.contains("toUpperCase") => Value::from(s.to_uppercase()),
        (Value::String(s), x) if x.contains("trim") => Value::from(s.trim()),
        _ => value,
    }
}

/// `{input.name}` in an object refinement's message: the option value.
fn input_props(t: &str, opts: &Obj) -> String {
    let mut out = String::with_capacity(t.len());
    let mut rest = t;
    while let Some(i) = rest.find("{input.") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 7..];
        match after.find('}') {
            Some(j) => {
                out.push_str(
                    &opts
                        .get(&after[..j])
                        .map(to_js_string)
                        .unwrap_or_else(|| "undefined".into()),
                );
                rest = &after[j + 1..];
            }
            None => {
                out.push_str(&rest[i..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

fn lit(l: Lit) -> Value {
    match l {
        Lit::Bool(b) => Value::Bool(b),
        Lit::Str(s) => Value::from(s),
        Lit::Num(n) => num_value(n),
    }
}

fn defined(opts: &Obj, k: &str) -> bool {
    opts.get(k).is_some()
}

fn rule_passes(rule: Rule, opts: &Obj) -> bool {
    match rule {
        Rule::ExactlyOne(names) => names.iter().filter(|n| defined(opts, n)).count() == 1,
        Rule::AtMostOne(names) => names.iter().filter(|n| defined(opts, n)).count() <= 1,
        Rule::AtLeastOne(names) => names.iter().any(|n| defined(opts, n)),
        Rule::AtLeastOneTruthy(names) => names.iter().any(|n| opts.get(*n).is_some_and(truthy)),
        Rule::ExactlyOneTruthy(names) => {
            names
                .iter()
                .filter(|n| opts.get(**n).is_some_and(truthy))
                .count()
                == 1
        }
        Rule::AtMostOneTruthy(names) => {
            names
                .iter()
                .filter(|n| opts.get(**n).is_some_and(truthy))
                .count()
                <= 1
        }
        Rule::NotAllTruthy(names) => !names.iter().all(|n| opts.get(*n).is_some_and(truthy)),
        Rule::Implies(names) => {
            !opts.get(names[0]).is_some_and(truthy) || opts.get(names[1]).is_some_and(truthy)
        }
        Rule::Expr(e) => crate::expr::refine(e, opts),
    }
}

/// Validates `opts` against the command schema. On success returns the
/// parsed options in schema order with defaults applied.
pub fn validate(meta: &CmdMeta, opts: &Obj, hooks: &Hooks) -> Result<Obj, Vec<Issue>> {
    let mut issues = Vec::new();
    let mut out = Obj::new();
    let mut aborted = false;
    let mut deferred: Vec<Issue> = Vec::new();
    for f in meta.fields {
        match opts.get(f.name) {
            None => {
                if let Some(d) = f.default {
                    out.insert(f.name.to_string(), lit(d));
                } else if !f.optional {
                    let mut i = Issue::field(
                        f.name,
                        match f.type_msg {
                            Msg::Text(t) => t.replace("{input}", "undefined"),
                            _ => format!(
                                "Invalid input: expected {}, received undefined",
                                expected(f.t)
                            ),
                        },
                    );
                    i.missing = true;
                    issues.push(i);
                    aborted = true;
                }
            }
            Some(raw) => {
                let r = check_field(f, raw, opts, hooks, &mut issues, &mut deferred);
                aborted |= r.aborted;
                out.insert(f.name.to_string(), r.value);
            }
        }
    }
    if meta.strict && !meta.allow_unknown {
        let unknown: Vec<&String> = opts.keys().filter(|k| meta.field(k).is_none()).collect();
        if !unknown.is_empty() {
            let keys = unknown
                .iter()
                .map(|k| format!("\"{k}\""))
                .collect::<Vec<_>>()
                .join(", ");
            issues.push(Issue {
                path: None,
                message: format!(
                    "Unrecognized key{}: {keys}",
                    if unknown.len() > 1 { "s" } else { "" }
                ),
                missing: false,
                code: RefineCode::None,
                set_options: &[],
            });
        }
    }
    issues.extend(deferred);
    if meta.allow_unknown || meta.loose {
        for (k, v) in opts {
            if meta.field(k).is_none() {
                out.insert(k.clone(), v.clone());
            }
        }
    }
    if !aborted {
        for (i, r) in meta.refines.iter().enumerate() {
            let result = match r.rule {
                Some(rule) => Ok(rule_passes(rule, &out)),
                None => hooks.refine.map(|h| h(i, &out)).unwrap_or(Ok(true)),
            };
            let message = match result {
                Ok(true) => continue,
                Ok(false) => match r.msg {
                    // `e.input` of an object refinement is the options object
                    Msg::Text(t) => input_props(t, &out).replace("{input}", "[object Object]"),
                    _ => "Invalid input".into(),
                },
                Err(m) => m,
            };
            issues.push(Issue {
                path: (!r.path.is_empty()).then(|| r.path.join(".")),
                message,
                missing: false,
                code: r.code,
                set_options: r.set_options,
            });
        }
    }
    if issues.is_empty() {
        Ok(out)
    } else {
        Err(issues)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(v: Value) -> Obj {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn user_get_messages_match_upstream() {
        let meta = crate::manifest::find("entra user get").unwrap();
        let e = validate(meta, &obj(serde_json::json!({})), &Hooks::default()).unwrap_err();
        assert_eq!(
            e.iter().map(|i| i.render(true)).collect::<Vec<_>>(),
            vec!["Specify either 'id', 'userName', or 'email'."]
        );
        let e = validate(
            meta,
            &obj(serde_json::json!({"id":"abc","userName":"x"})),
            &Hooks::default(),
        )
        .unwrap_err();
        assert_eq!(
            e.iter().map(|i| i.render(true)).collect::<Vec<_>>(),
            vec![
                "id: Invalid UUID",
                "userName: 'x' is not a valid userName.",
                "Specify either 'id', 'userName', or 'email'."
            ]
        );
        let e = validate(
            meta,
            &obj(serde_json::json!({"output":"xml","id":"1"})),
            &Hooks::default(),
        )
        .unwrap_err();
        assert_eq!(
            e.iter().map(|i| i.render(true)).collect::<Vec<_>>(),
            vec![
                "output: Invalid option: expected one of \"csv\"|\"json\"|\"md\"|\"text\"|\"none\"",
                "id: Invalid UUID"
            ]
        );
        let ok = validate(
            meta,
            &obj(serde_json::json!({"id":"0b0d2ba1-1a25-4b1c-8a6b-8e2c3a6c8f5e","output":"json"})),
            &Hooks::default(),
        )
        .unwrap();
        assert_eq!(
            crate::json::stringify(&Value::Object(ok)),
            r#"{"output":"json","debug":false,"verbose":false,"id":"0b0d2ba1-1a25-4b1c-8a6b-8e2c3a6c8f5e"}"#
        );
        let meta = crate::manifest::find("spo group get").unwrap();
        let e = validate(
            meta,
            &obj(serde_json::json!({"webUrl":"https://x","id":1.5})),
            &Hooks::default(),
        )
        .unwrap_err();
        assert_eq!(
            e[0].render(true),
            "id: Invalid input: expected int, received number"
        );
        let e = validate(meta, &obj(serde_json::json!({"id":1})), &Hooks::default()).unwrap_err();
        assert_eq!(e[0].render(true), "webUrl: Required option not specified");
    }
}

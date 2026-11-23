//! Evaluator for the refinement predicates that `scripts/js-expr.mjs`
//! compiles from upstream JavaScript into JSON expression trees. Values follow
//! JavaScript semantics (truthiness, strict and loose equality, UTF-16 string
//! comparison) closely enough for option validation.

use std::borrow::Cow;

use crate::json::{js_len, js_number, number_to_string, parse_int};
use crate::validation::{self as v, Valid};
use crate::{Obj, Value};

#[derive(Clone, Debug)]
enum J<'a> {
    Undef,
    Null,
    Bool(bool),
    Num(f64),
    Str(Cow<'a, str>),
    Arr(Vec<J<'a>>),
    Obj(&'a Obj),
    Fn(&'a Value),
    Boolean,
    Re(&'a str, &'a str),
    /// An object produced at run time (`JSON.parse`).
    Json(std::rc::Rc<Value>),
}

/// A value produced by the evaluator itself.
fn owned(v: &Value) -> J<'static> {
    match v {
        Value::Null => J::Null,
        Value::Bool(b) => J::Bool(*b),
        Value::Number(n) => J::Num(n.as_f64().unwrap_or(f64::NAN)),
        Value::String(s) => J::Str(Cow::Owned(s.clone())),
        Value::Array(a) => J::Arr(a.iter().map(owned).collect()),
        Value::Object(_) => J::Json(std::rc::Rc::new(v.clone())),
    }
}

impl<'a> J<'a> {
    fn from(v: Option<&'a Value>) -> J<'a> {
        match v {
            None => J::Undef,
            Some(Value::Null) => J::Null,
            Some(Value::Bool(b)) => J::Bool(*b),
            Some(Value::Number(n)) => J::Num(n.as_f64().unwrap_or(f64::NAN)),
            Some(Value::String(s)) => J::Str(Cow::Borrowed(s)),
            Some(Value::Array(a)) => J::Arr(a.iter().map(|x| J::from(Some(x))).collect()),
            Some(Value::Object(o)) => J::Obj(o),
        }
    }

    fn truthy(&self) -> bool {
        match self {
            J::Undef | J::Null => false,
            J::Bool(b) => *b,
            J::Num(n) => *n != 0.0 && !n.is_nan(),
            J::Str(s) => !s.is_empty(),
            _ => true,
        }
    }

    fn string(&self) -> String {
        match self {
            J::Undef => "undefined".into(),
            J::Null => "null".into(),
            J::Bool(b) => b.to_string(),
            J::Num(n) => number_to_string(*n),
            J::Str(s) => s.to_string(),
            J::Arr(a) => a
                .iter()
                .map(|x| {
                    if matches!(x, J::Undef | J::Null) {
                        String::new()
                    } else {
                        x.string()
                    }
                })
                .collect::<Vec<_>>()
                .join(","),
            J::Obj(_) => "[object Object]".into(),
            J::Fn(_) | J::Boolean => "function".into(),
            J::Re(p, f) => format!("/{p}/{f}"),
            J::Json(_) => "[object Object]".into(),
        }
    }

    fn number(&self) -> f64 {
        match self {
            J::Undef => f64::NAN,
            J::Null => 0.0,
            J::Bool(b) => f64::from(u8::from(*b)),
            J::Num(n) => *n,
            J::Str(s) => js_number(s),
            J::Arr(a) if a.is_empty() => 0.0,
            J::Arr(a) if a.len() == 1 => js_number(&a[0].string()),
            _ => f64::NAN,
        }
    }
}

fn strict_eq(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Undef, J::Undef) | (J::Null, J::Null) => true,
        (J::Bool(x), J::Bool(y)) => x == y,
        (J::Num(x), J::Num(y)) => x == y,
        (J::Str(x), J::Str(y)) => x == y,
        (J::Obj(x), J::Obj(y)) => std::ptr::eq(*x, *y),
        _ => false,
    }
}

fn loose_eq(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Undef | J::Null, J::Undef | J::Null) => true,
        (J::Undef | J::Null, _) | (_, J::Undef | J::Null) => false,
        (J::Num(_), J::Str(_)) | (J::Str(_), J::Num(_)) | (J::Bool(_), _) | (_, J::Bool(_)) => {
            a.number() == b.number()
        }
        _ => strict_eq(a, b),
    }
}

fn compare(a: &J, b: &J) -> Option<std::cmp::Ordering> {
    if let (J::Str(x), J::Str(y)) = (a, b) {
        return Some(x.encode_utf16().cmp(y.encode_utf16()));
    }
    a.number().partial_cmp(&b.number())
}

fn validator(name: &str, args: &[J]) -> J<'static> {
    let s = args
        .first()
        .map(J::string)
        .unwrap_or_else(|| "undefined".into());
    let b = |x: bool| J::Bool(x);
    let r = |x: Valid| match x {
        Valid::Yes => J::Bool(true),
        Valid::No => J::Bool(false),
        Valid::Msg(m) => J::Str(Cow::Owned(m)),
    };
    match name {
        "isValidGuid" => b(v::is_valid_guid(&s)),
        "isValidGuidArray" => r(v::is_valid_guid_array(&s)),
        "isValidTeamsChannelId" => b(v::is_valid_teams_channel_id(&s)),
        "isValidTeamsChatId" => b(v::is_valid_teams_chat_id(&s)),
        "isValidUserPrincipalName" => b(v::is_valid_user_principal_name(&s)),
        "isValidUserPrincipalNameArray" => r(v::is_valid_user_principal_name_array(&s)),
        "isValidPositiveInteger" => b(v::is_valid_positive_integer(&s)),
        "isValidPositiveIntegerArray" => r(v::is_valid_positive_integer_array(&s)),
        "isValidISODate" => b(v::is_valid_iso_date(&s)),
        "isValidISODateDashOnly" => b(v::is_valid_iso_date_dash_only(&s)),
        "isValidISODateTime" => b(v::is_valid_iso_date_time(&s)),
        "isValidGraphDateTime" => b(v::is_valid_graph_date_time(&s)),
        "isValidBoolean" => b(v::is_valid_boolean(&s)),
        "isValidSharePointUrl" => r(v::is_valid_sharepoint_url(&s)),
        "isValidMailNickname" => b(v::is_valid_mail_nickname(&s)),
        "isValidISODuration" => b(v::is_valid_iso_duration(&s)),
        "isValidPermission" => r(v::is_valid_permission(&s)),
        "isValidPowerPagesUrl" => b(v::is_valid_power_pages_url(&s)),
        "isValidJsonOrEmpty" => b(v::is_valid_json_or_empty(&s)),
        "isValidJson" => b(v::is_valid_json(&s)),
        "isValidTheme" => b(v::is_valid_theme(&s)),
        "isDateInRange" => b(v::is_date_in_range(
            &s,
            args.get(1).map(|x| x.number() as i32).unwrap_or(0),
        )),
        _ => J::Undef,
    }
}

/// A JavaScript regular expression source in Rust syntax: inside character
/// classes `[`, `&` and `~` are literals in JavaScript but class syntax in Rust.
fn js_regex(p: &str) -> String {
    let mut out = String::with_capacity(p.len() + 8);
    let mut in_class = false;
    let mut chars = p.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                out.push(c);
                if let Some(n) = chars.next() {
                    out.push(n);
                }
            }
            '[' if !in_class => {
                in_class = true;
                out.push(c);
                if chars.peek() == Some(&'^') {
                    out.push(chars.next().unwrap_or('^'));
                }
                // a leading `]` closes nothing in Rust but is literal in JS only when escaped
            }
            ']' if in_class => {
                in_class = false;
                out.push(c);
            }
            '[' | '&' | '~' if in_class => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

struct Ev<'a> {
    arg: J<'a>,
    env: Vec<(&'a str, J<'a>)>,
    /// A JavaScript exception is propagating (`JSON.parse` failed).
    thrown: bool,
}

impl<'a> Ev<'a> {
    fn call(&mut self, f: &J<'a>, args: Vec<J<'a>>) -> J<'a> {
        match f {
            J::Boolean => J::Bool(args.first().is_some_and(J::truthy)),
            J::Fn(node) => {
                let params = node["p"].as_array().map(Vec::as_slice).unwrap_or(&[]);
                let mark = self.env.len();
                let mut args = args.into_iter();
                for p in params {
                    self.env
                        .push((p.as_str().unwrap_or(""), args.next().unwrap_or(J::Undef)));
                }
                let r = self.eval(&node["b"]);
                self.env.truncate(mark);
                r
            }
            _ => J::Undef,
        }
    }

    fn eval(&mut self, n: &'a Value) -> J<'a> {
        if self.thrown {
            return J::Undef;
        }
        let t = n["t"].as_str().unwrap_or("");
        match t {
            "try" => {
                let r = self.eval(&n["b"]);
                if self.thrown {
                    self.thrown = false;
                    return self.eval(&n["c"]);
                }
                r
            }
            "seq" => {
                self.eval(&n["e"]);
                self.eval(&n["b"])
            }
            "let" => {
                let v = self.eval(&n["v"]);
                let mark = self.env.len();
                self.env.push((n["n"].as_str().unwrap_or(""), v));
                let r = self.eval(&n["b"]);
                self.env.truncate(mark);
                r
            }
            "lit" => J::from(n.get("v")),
            "null" => J::Null,
            "undef" => J::Undef,
            "arg" => self.arg.clone(),
            "var" => {
                let name = n["n"].as_str().unwrap_or("");
                self.env
                    .iter()
                    .rev()
                    .find(|(k, _)| *k == name)
                    .map(|(_, v)| v.clone())
                    .unwrap_or(J::Undef)
            }
            "global" => J::Boolean,
            // a Date is modelled by its time value: comparisons use valueOf()
            "date" => {
                let args: Vec<J> = n["a"]
                    .as_array()
                    .map(|a| a.iter().map(|x| self.eval(x)).collect())
                    .unwrap_or_default();
                J::Num(match args.first() {
                    None => crate::time::now_ms() as f64,
                    Some(J::Num(ms)) => *ms,
                    Some(J::Undef) => f64::NAN,
                    Some(x) => crate::time::parse_js_date(&x.string())
                        .map(|d| d.timestamp_millis() as f64)
                        .unwrap_or(f64::NAN),
                })
            }
            "re" => J::Re(n["p"].as_str().unwrap_or(""), n["f"].as_str().unwrap_or("")),
            "fn" => J::Fn(n),
            "not" => J::Bool(!self.eval(&n["e"]).truthy()),
            "neg" => J::Num(-self.eval(&n["e"]).number()),
            "typeof" => J::Str(Cow::Borrowed(match self.eval(&n["e"]) {
                J::Undef => "undefined",
                J::Bool(_) => "boolean",
                J::Num(_) => "number",
                J::Str(_) => "string",
                J::Fn(_) | J::Boolean => "function",
                _ => "object",
            })),
            "if" => {
                if self.eval(&n["c"]).truthy() {
                    self.eval(&n["a"])
                } else {
                    self.eval(&n["b"])
                }
            }
            "arr" => J::Arr(
                n["a"]
                    .as_array()
                    .map(|a| a.iter().map(|x| self.eval(x)).collect())
                    .unwrap_or_default(),
            ),
            "bin" => {
                let op = n["op"].as_str().unwrap_or("");
                let l = self.eval(&n["l"]);
                match op {
                    "||" => return if l.truthy() { l } else { self.eval(&n["r"]) },
                    "&&" => return if l.truthy() { self.eval(&n["r"]) } else { l },
                    "??" => {
                        return if matches!(l, J::Undef | J::Null) {
                            self.eval(&n["r"])
                        } else {
                            l
                        };
                    }
                    _ => {}
                }
                let r = self.eval(&n["r"]);
                use std::cmp::Ordering::*;
                match op {
                    "===" => J::Bool(strict_eq(&l, &r)),
                    "!==" => J::Bool(!strict_eq(&l, &r)),
                    "==" => J::Bool(loose_eq(&l, &r)),
                    "!=" => J::Bool(!loose_eq(&l, &r)),
                    "<" => J::Bool(compare(&l, &r) == Some(Less)),
                    "<=" => J::Bool(matches!(compare(&l, &r), Some(Less | Equal))),
                    ">" => J::Bool(compare(&l, &r) == Some(Greater)),
                    ">=" => J::Bool(matches!(compare(&l, &r), Some(Greater | Equal))),
                    "+" => match (&l, &r) {
                        (J::Num(a), J::Num(b)) => J::Num(a + b),
                        _ => J::Str(Cow::Owned(l.string() + &r.string())),
                    },
                    "-" => J::Num(l.number() - r.number()),
                    _ => J::Undef,
                }
            }
            "get" => {
                let o = self.eval(&n["o"]);
                let k = n["k"].as_str().unwrap_or("");
                match (&o, k) {
                    (J::Obj(m), _) => J::from(m.get(k)),
                    (J::Json(v), _) => v.get(k).map(owned).unwrap_or(J::Undef),
                    (J::Undef | J::Null, _) if !n["opt"].as_bool().unwrap_or(false) => {
                        // TypeError: Cannot read properties of undefined
                        self.thrown = true;
                        J::Undef
                    }
                    (J::Str(s), "length") => J::Num(js_len(s) as f64),
                    (J::Arr(a), "length") => J::Num(a.len() as f64),
                    _ => J::Undef,
                }
            }
            "idx" => {
                let o = self.eval(&n["o"]);
                let i = self.eval(&n["i"]);
                match o {
                    J::Obj(m) => J::from(m.get(&i.string())),
                    J::Arr(a) => a.get(i.number() as usize).cloned().unwrap_or(J::Undef),
                    _ => J::Undef,
                }
            }
            "valid" => {
                let args: Vec<J> = n["a"]
                    .as_array()
                    .map(|a| a.iter().map(|x| self.eval(x)).collect())
                    .unwrap_or_default();
                validator(n["f"].as_str().unwrap_or(""), &args)
            }
            "call" => {
                let args: Vec<J> = n["a"]
                    .as_array()
                    .map(|a| a.iter().map(|x| self.eval(x)).collect())
                    .unwrap_or_default();
                let f = n["f"].as_str().unwrap_or("");
                if f == "path.resolve" {
                    let mut p = std::env::current_dir().unwrap_or_default();
                    for a in &args {
                        p.push(a.string());
                    }
                    return J::Str(Cow::Owned(crate::util::resolve(&p.to_string_lossy())));
                }
                let a0 = args.into_iter().next().unwrap_or(J::Undef);
                match f {
                    "path.dirname" => {
                        let s = a0.string();
                        let d = std::path::Path::new(&s)
                            .parent()
                            .map(|d| d.to_string_lossy().into_owned())
                            .unwrap_or_default();
                        J::Str(Cow::Owned(if d.is_empty() {
                            if s.starts_with('/') {
                                "/".into()
                            } else {
                                ".".into()
                            }
                        } else {
                            d
                        }))
                    }
                    "JSON.parse" => match serde_json::from_str::<Value>(&a0.string()) {
                        Ok(v) => owned(&v),
                        Err(_) => {
                            self.thrown = true;
                            J::Undef
                        }
                    },
                    "formatting.splitAndTrim" => J::Arr(
                        a0.string()
                            .split(',')
                            .map(|s| J::Str(Cow::Owned(s.trim().to_string())))
                            .collect(),
                    ),
                    "Number" => J::Num(a0.number()),
                    "isNaN" => J::Bool(a0.number().is_nan()),
                    "parseInt" => J::Num(
                        parse_int(&a0.string())
                            .map(|x| x as f64)
                            .unwrap_or(f64::NAN),
                    ),
                    "Number.isInteger" => {
                        J::Bool(matches!(a0, J::Num(x) if x.is_finite() && x.fract() == 0.0))
                    }
                    "Boolean" => J::Bool(a0.truthy()),
                    "fs.existsSync" => J::Bool(
                        !matches!(a0, J::Undef | J::Null)
                            && std::path::Path::new(&a0.string()).exists(),
                    ),
                    "Array.isArray" => J::Bool(matches!(a0, J::Arr(_))),
                    "Object.values" => match a0 {
                        J::Obj(m) => J::Arr(m.values().map(|x| J::from(Some(x))).collect()),
                        J::Arr(a) => J::Arr(a),
                        _ => J::Arr(Vec::new()),
                    },
                    "Object.keys" => match a0 {
                        J::Obj(m) => J::Arr(
                            m.keys()
                                .map(|k| J::Str(Cow::Borrowed(k.as_str())))
                                .collect(),
                        ),
                        J::Arr(a) => J::Arr(
                            (0..a.len())
                                .map(|i| J::Str(Cow::Owned(i.to_string())))
                                .collect(),
                        ),
                        _ => J::Arr(Vec::new()),
                    },
                    _ => J::Undef,
                }
            }
            "method" => {
                let o = self.eval(&n["o"]);
                let args: Vec<J> = n["a"]
                    .as_array()
                    .map(|a| a.iter().map(|x| self.eval(x)).collect())
                    .unwrap_or_default();
                self.method(n["m"].as_str().unwrap_or(""), o, args)
            }
            _ => J::Undef,
        }
    }

    fn method(&mut self, m: &str, o: J<'a>, args: Vec<J<'a>>) -> J<'a> {
        let a0 = args.first().cloned().unwrap_or(J::Undef);
        match o {
            J::Num(n) if m == "getTime" => J::Num(n),
            J::Str(s) => match m {
                "toLowerCase" | "toLocaleLowerCase" => J::Str(Cow::Owned(s.to_lowercase())),
                "toUpperCase" => J::Str(Cow::Owned(s.to_uppercase())),
                "trim" => J::Str(Cow::Owned(s.trim().to_string())),
                "includes" => J::Bool(s.contains(a0.string().as_str())),
                "startsWith" => J::Bool(s.starts_with(a0.string().as_str())),
                "endsWith" => J::Bool(s.ends_with(a0.string().as_str())),
                "indexOf" => {
                    let needle = a0.string();
                    J::Num(
                        s.find(needle.as_str())
                            .map(|b| js_len(&s[..b]) as f64)
                            .unwrap_or(-1.0),
                    )
                }
                "split" => {
                    let sep = a0.string();
                    if matches!(a0, J::Undef) {
                        J::Arr(vec![J::Str(Cow::Owned(s.to_string()))])
                    } else if sep.is_empty() {
                        J::Arr(
                            s.chars()
                                .map(|c| J::Str(Cow::Owned(c.to_string())))
                                .collect(),
                        )
                    } else {
                        J::Arr(
                            s.split(sep.as_str())
                                .map(|p| J::Str(Cow::Owned(p.to_string())))
                                .collect(),
                        )
                    }
                }
                _ => J::Undef,
            },
            J::Re(p, f) if m == "test" => {
                let flags = if f.contains('i') { "(?i)" } else { "" };
                let ok = fancy_regex::Regex::new(&format!("{flags}{}", js_regex(p)))
                    .ok()
                    .and_then(|r| r.is_match(&a0.string()).ok())
                    .unwrap_or(false);
                J::Bool(ok)
            }
            J::Arr(items) => match m {
                "includes" => J::Bool(items.iter().any(|x| {
                    strict_eq(x, &a0)
                        || matches!((x, &a0), (J::Num(a), J::Num(b)) if a.is_nan() && b.is_nan())
                })),
                "indexOf" => J::Num(
                    items
                        .iter()
                        .position(|x| strict_eq(x, &a0))
                        .map(|i| i as f64)
                        .unwrap_or(-1.0),
                ),
                "join" => {
                    let sep = if matches!(a0, J::Undef) {
                        ",".to_string()
                    } else {
                        a0.string()
                    };
                    J::Str(Cow::Owned(
                        items
                            .iter()
                            .map(|x| {
                                if matches!(x, J::Undef | J::Null) {
                                    String::new()
                                } else {
                                    x.string()
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(&sep),
                    ))
                }
                "filter" => {
                    let mut out = Vec::new();
                    for (i, x) in items.into_iter().enumerate() {
                        if self.call(&a0, vec![x.clone(), J::Num(i as f64)]).truthy() {
                            out.push(x);
                        }
                    }
                    J::Arr(out)
                }
                "map" => J::Arr(
                    items
                        .into_iter()
                        .enumerate()
                        .map(|(i, x)| self.call(&a0, vec![x, J::Num(i as f64)]))
                        .collect(),
                ),
                "some" => J::Bool(
                    items
                        .into_iter()
                        .enumerate()
                        .any(|(i, x)| self.call(&a0, vec![x, J::Num(i as f64)]).truthy()),
                ),
                "every" => J::Bool(
                    items
                        .into_iter()
                        .enumerate()
                        .all(|(i, x)| self.call(&a0, vec![x, J::Num(i as f64)]).truthy()),
                ),
                _ => J::Undef,
            },
            _ => J::Undef,
        }
    }
}

/// Whether the object-level predicate `expr` (JSON) holds for `opts`.
pub fn refine(expr: &str, opts: &Obj) -> bool {
    let Ok(tree) = serde_json::from_str::<Value>(expr) else {
        return true;
    };
    let mut ev = Ev {
        arg: J::Obj(opts),
        env: Vec::new(),
        thrown: false,
    };
    ev.eval(&tree).truthy()
}

/// Whether the field predicate `expr` (JSON) holds for `value`.
pub fn field(expr: &str, value: &Value) -> bool {
    let Ok(tree) = serde_json::from_str::<Value>(expr) else {
        return true;
    };
    let mut ev = Ev {
        arg: J::from(Some(value)),
        env: Vec::new(),
        thrown: false,
    };
    ev.eval(&tree).truthy()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn o(v: Value) -> Obj {
        v.as_object().cloned().unwrap_or_default()
    }

    #[test]
    fn exactly_one_when_truthy() {
        // !opts.planTitle || [opts.ownerGroupId, opts.ownerGroupName].filter(x => x !== undefined).length === 1
        let e = r#"{"t":"bin","op":"||","l":{"t":"not","e":{"t":"get","o":{"t":"arg"},"k":"planTitle"}},"r":{"t":"bin","op":"===","l":{"t":"get","o":{"t":"method","m":"filter","o":{"t":"arr","a":[{"t":"get","o":{"t":"arg"},"k":"ownerGroupId"},{"t":"get","o":{"t":"arg"},"k":"ownerGroupName"}]},"a":[{"t":"fn","p":["x"],"b":{"t":"bin","op":"!==","l":{"t":"var","n":"x"},"r":{"t":"undef"}}}]},"k":"length"},"r":{"t":"lit","v":1}}}"#;
        assert!(refine(e, &o(serde_json::json!({}))));
        assert!(!refine(e, &o(serde_json::json!({"planTitle": "a"}))));
        assert!(refine(
            e,
            &o(serde_json::json!({"planTitle": "a", "ownerGroupName": "b"}))
        ));
    }
}

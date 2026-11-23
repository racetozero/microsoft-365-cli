//! Output formatting: a port of `cli.formatOutput` and the `getTextOutput`,
//! `getJsonOutput`, `getCsvOutput` and `getMdOutput` methods of upstream's
//! `Command` class.

use crate::config::{self, names};
use crate::json::{self, Obj, keys, stringify, stringify_pretty, to_js_string};
use crate::manifest::CmdMeta;
use crate::term;
use serde_json::Value;
use unicode_width::UnicodeWidthStr;

/// `getJsonOutput`: pretty JSON with upstream's newline rewrite (#2807).
pub fn json_output(v: &Value) -> String {
    rewrite_newlines(&stringify_pretty(v, "  "))
}

/// `.replace(/([^\\])\\n/g, '$1\\\\\\n')`: every `\n` escape that follows a
/// character other than a backslash becomes `\\\n`. Matches do not overlap,
/// as with a global regular expression.
fn rewrite_newlines(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < chars.len() {
        if i + 2 < chars.len() && chars[i] != '\\' && chars[i + 1] == '\\' && chars[i + 2] == 'n' {
            out.push(chars[i]);
            out.push_str("\\\\\\n");
            i += 3;
        } else {
            out.push(chars[i]);
            i += 1;
        }
    }
    out
}

/// `getTextOutput` for a list of values.
pub fn text_output(items: &[Value]) -> String {
    if items.len() == 1 {
        let obj = match &items[0] {
            Value::Object(o) => o.clone(),
            _ => Obj::new(),
        };
        let mut props: Vec<&String> = keys(&obj);
        let longest = props
            .iter()
            .map(|p| p.encode_utf16().count())
            .max()
            .unwrap_or(0);
        props.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
        let mut out = String::new();
        for (i, p) in props.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            let v = &obj[p.as_str()];
            let len = p.encode_utf16().count();
            out.push_str(p);
            for _ in len..longest {
                out.push(' ');
            }
            out.push_str(": ");
            match v {
                Value::Array(_) | Value::Object(_) | Value::Null => out.push_str(&stringify(v)),
                other => out.push_str(&to_js_string(other)),
            }
        }
        out.push('\n');
        return out;
    }
    easy_table(items)
}

fn cell_string(v: Option<&Value>) -> String {
    match v {
        None => String::new(),
        Some(v) => to_js_string(v),
    }
}

fn width(s: &str) -> usize {
    strip_ansi(s).width()
}

fn strip_ansi(s: &str) -> std::borrow::Cow<'_, str> {
    if !s.contains('\u{1b}') {
        return std::borrow::Cow::Borrowed(s);
    }
    static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"[\u001B\u009B][\[\]()#;?]*(?:(?:(?:[a-zA-Z\d]*(?:;[-a-zA-Z\d/#&.:=?%@~_]*)*)?\u0007)|(?:(?:\d{1,4}(?:;\d{0,4})*)?[\dA-PR-TZcf-nq-uy=><~]))").unwrap()
    });
    std::borrow::Cow::Owned(RE.replace_all(s, "").into_owned())
}

/// The `easy-table` package's `Table.toString()`.
fn easy_table(items: &[Value]) -> String {
    // rows of (column, value) in JavaScript key order
    let rows: Vec<Vec<(String, Option<Value>)>> = items
        .iter()
        .map(|r| match r {
            Value::Object(o) => {
                let mut row: Vec<(String, Option<Value>)> = Vec::new();
                // `row` is a fresh object whose `for...in` order puts integer keys first
                let mut keyed = Obj::new();
                for k in keys(o) {
                    keyed.insert(k.clone(), o[k.as_str()].clone());
                }
                for k in keys(&keyed) {
                    row.push((k.clone(), Some(keyed[k.as_str()].clone())));
                }
                row
            }
            // non-objects are skipped by `cell`, but `newRow` still adds an empty row
            _ => Vec::new(),
        })
        .collect();
    // columns(): two passes
    let mut cols: Vec<(String, usize)> = Vec::new();
    for _ in 0..2 {
        for row in &rows {
            let mut idx = 0usize;
            for (k, _) in row {
                let cur = cols
                    .iter()
                    .find(|(c, _)| c == k)
                    .map(|(_, i)| *i)
                    .unwrap_or(0);
                idx = idx.max(cur);
                match cols.iter_mut().find(|(c, _)| c == k) {
                    Some(e) => e.1 = idx,
                    None => cols.push((k.clone(), idx)),
                }
                idx += 1;
            }
        }
    }
    // Object.keys(cols) order, then stable sort by index
    let mut ordered: Vec<(String, usize)> = {
        let mut m = Obj::new();
        for (c, i) in &cols {
            m.insert(c.clone(), Value::from(*i));
        }
        keys(&m)
            .into_iter()
            .map(|k| (k.clone(), m[k.as_str()].as_u64().unwrap() as usize))
            .collect()
    };
    ordered.sort_by_key(|(_, i)| *i);
    let columns: Vec<String> = ordered.into_iter().map(|(c, _)| c).collect();

    let mut widths: Vec<usize> = columns.iter().map(|c| width(c)).collect();
    for row in &rows {
        for (k, v) in row {
            if let Some(ci) = columns.iter().position(|c| c == k) {
                widths[ci] = widths[ci].max(width(&cell_string(v.as_ref())));
            }
        }
    }
    let mut out = String::new();
    let pad = |s: &str, w: usize, ch: char, left: bool| -> String {
        let len = width(s);
        let p: String = std::iter::repeat_n(ch, w.saturating_sub(len)).collect();
        if left {
            format!("{p}{s}")
        } else {
            format!("{s}{p}")
        }
    };
    // header
    let line: Vec<String> = columns
        .iter()
        .enumerate()
        .map(|(i, c)| pad(c, widths[i], ' ', false))
        .collect();
    out.push_str(&line.join("  "));
    out.push('\n');
    let line: Vec<String> = columns
        .iter()
        .enumerate()
        .map(|(i, _)| pad("", widths[i], '-', true))
        .collect();
    out.push_str(&line.join("  "));
    out.push('\n');
    for row in &rows {
        let line: Vec<String> = columns
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let s = row
                    .iter()
                    .find(|(k, _)| k == c)
                    .map(|(_, v)| cell_string(v.as_ref()))
                    .unwrap_or_default();
                pad(&s, widths[i], ' ', false)
            })
            .collect();
        out.push_str(&line.join("  "));
        out.push('\n');
    }
    out
}

/// `getCsvOutput`.
pub fn csv_output(items: &mut [Value], has_query: bool) -> String {
    if !items.is_empty() && !has_query {
        for item in items.iter_mut() {
            if let Value::Object(o) = item {
                o.retain(|_, v| !matches!(v, Value::Object(_) | Value::Array(_)));
            }
        }
    }
    let header = config::get_bool(names::CSV_HEADER, true);
    let escape = config::get_string(names::CSV_ESCAPE, "\"");
    let quote = config::get(names::CSV_QUOTE)
        .map(|v| to_js_string(&v))
        .unwrap_or_else(|| "\"".into());
    let quoted = config::get_bool(names::CSV_QUOTED, false);
    let quoted_empty = config::get_bool(names::CSV_QUOTED_EMPTY, false);
    let columns: Vec<String> = match items.first() {
        Some(Value::Object(o)) => keys(o).into_iter().cloned().collect(),
        _ => Vec::new(),
    };
    let field = |v: Option<&Value>| -> String {
        let (value, is_empty_string): (Option<String>, bool) = match v {
            None | Some(Value::Null) => (None, false),
            Some(Value::Bool(b)) => (Some(if *b { "1".into() } else { "0".into() }), false),
            Some(Value::Number(_)) => (Some(to_js_string(v.unwrap())), false),
            Some(Value::String(s)) => (Some(s.clone()), s.is_empty()),
            Some(other) => (Some(stringify(other)), false),
        };
        match value {
            Some(s) if s.is_empty() && is_empty_string => {
                if quoted_empty {
                    format!("{quote}{quote}")
                } else {
                    String::new()
                }
            }
            Some(s) if !s.is_empty() => {
                let contains_delim = s.contains(',');
                let contains_quote = !quote.is_empty() && s.contains(&quote);
                let contains_escape = s.contains(&escape) && escape != quote;
                let contains_rd = s.contains('\n') || s.contains('\r');
                let should_quote = contains_quote || contains_delim || contains_rd || quoted;
                let mut s = s;
                if should_quote && contains_escape {
                    s = s.replace(&escape, &format!("{escape}{escape}"));
                }
                if contains_quote {
                    s = s.replace(&quote, &format!("{escape}{quote}"));
                }
                if should_quote {
                    s = format!("{quote}{s}{quote}");
                }
                s
            }
            _ => {
                if quoted_empty {
                    format!("{quote}{quote}")
                } else {
                    String::new()
                }
            }
        }
    };
    let mut out = String::new();
    if header && !columns.is_empty() {
        let h: Vec<String> = columns
            .iter()
            .map(|c| field(Some(&Value::from(c.as_str()))))
            .collect();
        out.push_str(&h.join(","));
        out.push('\n');
    }
    for item in items.iter() {
        match item {
            Value::Object(o) => {
                let r: Vec<String> = columns.iter().map(|c| field(o.get(c))).collect();
                out.push_str(&r.join(","));
                out.push('\n');
            }
            Value::Array(a) => {
                let r: Vec<String> = a.iter().map(|x| field(Some(x))).collect();
                out.push_str(&r.join(","));
                out.push('\n');
            }
            _ => {}
        }
    }
    out
}

/// `md.escapeMd`.
pub fn escape_md(v: Option<&Value>) -> String {
    match v {
        None => "undefined".into(),
        Some(Value::String(s)) if s == json::UNDEFINED => "undefined".into(),
        Some(v) if !json::truthy(v) => to_js_string(v),
        Some(v) => {
            let s = to_js_string(v);
            let mut out = String::with_capacity(s.len());
            for c in s.chars() {
                match c {
                    '_' | '*' | '~' | '`' | '|' => {
                        out.push('\\');
                        out.push(c);
                    }
                    '\n' => out.push_str("<br>"),
                    c => out.push(c),
                }
            }
            out
        }
    }
}

fn first_of<'a>(o: &'a Obj, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|n| {
        o.get(*n)
            .filter(|v| !v.is_null() && v.as_str() != Some(json::UNDEFINED))
    })
}

/// `getMdOutput`.
pub fn md_output(items: &[Value], command_name: &str, options: &Obj, has_query: bool) -> String {
    let eol = term::EOL;
    let args: Vec<String> = options
        .iter()
        .filter(|(k, _)| k.as_str() != "output")
        .map(|(k, v)| format!("--{k} \"{}\"", to_js_string(v)))
        .collect();
    let mut out = format!(
        "# {command_name} {}{eol}{eol}Date: {}{eol}{eol}",
        args.join(" "),
        crate::time::locale_date_string(chrono::Local::now())
    );
    for l in items {
        if !json::truthy(l) {
            continue;
        }
        let o = match l {
            Value::Object(o) => o.clone(),
            _ => Obj::new(),
        };
        let title = first_of(
            &o,
            &[
                "title",
                "Title",
                "displayName",
                "DisplayName",
                "name",
                "Name",
            ],
        )
        .map(to_js_string);
        let id = first_of(&o, &["id"])
            .or_else(|| {
                o.get("Id")
                    .and_then(|i| i.get("StringValue"))
                    .filter(|v| !v.is_null())
            })
            .or_else(|| {
                first_of(
                    &o,
                    &[
                        "Id", "ID", "uniqueId", "UniqueId", "objectId", "ObjectId", "url", "Url",
                        "URL",
                    ],
                )
            })
            .map(to_js_string);
        match (&title, &id) {
            (Some(t), Some(i)) => out.push_str(&format!("## {t} ({i}){eol}{eol}")),
            (Some(t), None) => out.push_str(&format!("## {t}{eol}{eol}")),
            (None, Some(i)) => out.push_str(&format!("## {i}{eol}{eol}")),
            _ => {}
        }
        out.push_str(&format!("Property | Value{eol}---------|-------{eol}"));
        let rows: Vec<String> = keys(&o)
            .into_iter()
            .filter(|k| {
                has_query
                    || !matches!(
                        o[k.as_str()],
                        Value::Object(_) | Value::Array(_) | Value::Null
                    )
            })
            .map(|k| {
                format!(
                    "{} | {}",
                    escape_md(Some(&Value::from(k.as_str()))),
                    escape_md(o.get(k.as_str()))
                )
            })
            .collect();
        out.push_str(&rows.join(eol));
        out.push_str(eol);
        out.push_str(eol);
    }
    out.trim_end().to_string()
}

/// Output settings resolved for one command run.
#[derive(Clone, Debug)]
pub struct OutputOptions {
    pub output: String,
    pub query: Option<String>,
    /// final options, used for the md header
    pub options: Obj,
}

/// `cli.formatOutput`. Returns `None` for `undefined`.
pub fn format_output(
    meta: Option<&CmdMeta>,
    override_props: Option<&'static [&'static str]>,
    command_name: &str,
    value: Value,
    opts: &OutputOptions,
) -> Result<String, String> {
    let original_props: Vec<String> = {
        let first = match &value {
            Value::Array(a) => a.first(),
            other => Some(other),
        };
        match first {
            Some(Value::Object(o)) => keys(o).into_iter().cloned().collect(),
            _ => Vec::new(),
        }
    };
    let mut value = value;
    if let Some(q) = &opts.query {
        value = crate::query::search(&value, q).map_err(|e| format!("JMESPath query error. {e}. See https://jmespath.org/specification.html for more information"))?;
    }
    if opts.output.is_empty() || opts.output == "json" {
        return Ok(json_output(&value));
    }
    let (mut items, array_type): (Vec<Value>, &str) = match value {
        Value::Array(a) => {
            let t = a
                .iter()
                .map(|x| match x {
                    Value::Array(_) => "array",
                    Value::Object(_) | Value::Null => "object",
                    Value::String(_) => "string",
                    Value::Number(_) => "number",
                    Value::Bool(_) => "boolean",
                })
                .next()
                .unwrap_or("");
            (a, t)
        }
        other => {
            let t = match &other {
                Value::Object(_) | Value::Null => "object",
                Value::String(_) => "string",
                Value::Number(_) => "number",
                Value::Bool(_) => "boolean",
                Value::Array(_) => "array",
            };
            (vec![other], t)
        }
    };
    if array_type != "object" {
        let lines: Vec<String> = items
            .iter()
            .map(|x| match x {
                Value::Null => String::new(),
                other => to_js_string(other),
            })
            .collect();
        return Ok(lines.join(term::EOL));
    }
    if opts.output == "text"
        && let Some(props) = override_props.or_else(|| meta.and_then(|m| m.default_properties))
    {
        let first_props: Vec<String> = match items.first() {
            Some(Value::Object(o)) => keys(o).into_iter().cloned().collect(),
            _ => Vec::new(),
        };
        if first_props == original_props {
            if let Some(Value::Array(inner)) = items.first().and_then(|f| f.get("value")) {
                items = inner.clone();
            }
            items = items
                .into_iter()
                .map(|s| match s {
                    Value::Object(o) => Value::Object(crate::util::filter_object(&o, props)),
                    other => other,
                })
                .collect();
        }
    }
    Ok(match opts.output.as_str() {
        "csv" => csv_output(&mut items, opts.query.is_some()),
        "md" => md_output(&items, command_name, &opts.options, opts.query.is_some()),
        _ => text_output(&items),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn newline_rewrite_matches_regex() {
        assert_eq!(rewrite_newlines(r#""a\nb""#), r#""a\\\nb""#);
        assert_eq!(rewrite_newlines(r#""x\n\ny""#), r#""x\\\n\ny""#);
        assert_eq!(rewrite_newlines(r#""x\\ny""#), r#""x\\ny""#);
    }

    #[test]
    fn text_and_table() {
        assert_eq!(
            text_output(&[json!({"b": 1, "a": [1], "cc": null})]),
            "a : [1]\nb : 1\ncc: null\n"
        );
        assert_eq!(
            text_output(&[json!({"a": "x", "b": 22}), json!({"a": "yyy"})]),
            "a    b \n---  --\nx    22\nyyy    \n"
        );
    }
}

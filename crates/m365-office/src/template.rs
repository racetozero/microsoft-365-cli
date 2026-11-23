//! The part of the Adaptive Cards templating language (`adaptivecards-templating`)
//! that `adaptivecard send` cards use: `${path}` bindings against `$root`
//! and `$data`, `$data` repetition over arrays and `$when` conditions.
//! Expressions the subset does not understand are left as written, as the
//! SDK does for bindings it cannot resolve.

use m365_core::Value;
use m365_core::json::{Obj, to_js_string, truthy};

struct Ctx<'a> {
    root: &'a Value,
    data: &'a Value,
    index: Option<usize>,
}

/// `new Template(card).expand({ $root: data })`.
pub fn expand(card: &Value, root: &Value) -> Value {
    let cx = Ctx {
        root,
        data: root,
        index: None,
    };
    expand_value(card, &cx).unwrap_or(Value::Null)
}

fn expand_value(v: &Value, cx: &Ctx) -> Option<Value> {
    match v {
        Value::String(s) => Some(expand_string(s, cx)),
        Value::Array(items) => {
            let mut out = Vec::new();
            for i in items {
                match i {
                    Value::Object(o) if o.contains_key("$data") => {
                        out.extend(expand_data_object(o, cx))
                    }
                    other => {
                        if let Some(x) = expand_value(other, cx) {
                            out.push(x);
                        }
                    }
                }
            }
            Some(Value::Array(out))
        }
        Value::Object(o) if o.contains_key("$data") => {
            let mut r = expand_data_object(o, cx);
            // a repeated object outside an array keeps the first copy
            if r.is_empty() {
                None
            } else {
                Some(r.remove(0))
            }
        }
        Value::Object(o) => expand_object(o, cx),
        other => Some(other.clone()),
    }
}

fn expand_data_object(o: &Obj, cx: &Ctx) -> Vec<Value> {
    let data = match &o["$data"] {
        Value::String(s) => match binding(s) {
            Some(expr) => resolve(expr, cx),
            None => Some(Value::String(s.clone())),
        },
        other => Some(other.clone()),
    };
    let mut rest = o.clone();
    rest.shift_remove("$data");
    // an unresolved `$data` keeps the object, bindings unresolved
    let Some(data) = data else {
        return expand_object(&rest, cx).into_iter().collect();
    };
    match &data {
        Value::Array(items) => items
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                expand_object(
                    &rest,
                    &Ctx {
                        root: cx.root,
                        data: d,
                        index: Some(i),
                    },
                )
            })
            .collect(),
        d => expand_object(
            &rest,
            &Ctx {
                root: cx.root,
                data: d,
                index: cx.index,
            },
        )
        .into_iter()
        .collect(),
    }
}

fn expand_object(o: &Obj, cx: &Ctx) -> Option<Value> {
    if let Some(w) = o.get("$when") {
        let keep = match w {
            Value::String(s) => match binding(s) {
                Some(expr) => resolve(expr, cx).is_some_and(|v| truthy(&v)),
                None => truthy(w),
            },
            other => truthy(other),
        };
        if !keep {
            return None;
        }
    }
    let mut out = Obj::new();
    for (k, v) in o {
        if k == "$when" {
            continue;
        }
        if let Some(x) = expand_value(v, cx) {
            out.insert(k.clone(), x);
        }
    }
    Some(Value::Object(out))
}

/// The expression of a string that is exactly one `${...}` binding.
fn binding(s: &str) -> Option<&str> {
    let inner = s.strip_prefix("${")?.strip_suffix('}')?;
    (!inner.contains("${")).then_some(inner)
}

fn expand_string(s: &str, cx: &Ctx) -> Value {
    if let Some(expr) = binding(s) {
        return resolve(expr, cx).unwrap_or_else(|| Value::from(s));
    }
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("${") {
        out.push_str(&rest[..i]);
        let after = &rest[i + 2..];
        match after.find('}') {
            Some(j) => {
                let expr = &after[..j];
                match resolve(expr, cx) {
                    Some(Value::String(v)) => out.push_str(&v),
                    Some(v) => out.push_str(&to_js_string(&v)),
                    None => out.push_str(&rest[i..i + 2 + j + 1]),
                }
                rest = &after[j + 1..];
            }
            None => {
                out.push_str(&rest[i..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    Value::from(out)
}

/// A property path such as `title`, `$root.items[0].name`, `$data.x` or `$index`.
fn resolve(expr: &str, cx: &Ctx) -> Option<Value> {
    let expr = expr.trim();
    if expr == "$index" {
        return cx.index.map(Value::from);
    }
    let (mut cur, path) = if let Some(p) = expr.strip_prefix("$root") {
        (cx.root, p)
    } else if let Some(p) = expr.strip_prefix("$data") {
        (cx.data, p)
    } else {
        (cx.data, expr)
    };
    let path = path.strip_prefix('.').unwrap_or(path);
    if path.is_empty() {
        return Some(cur.clone());
    }
    for seg in path.split('.') {
        let (name, idx) = match seg.find('[') {
            Some(b) => (&seg[..b], Some(&seg[b..])),
            None => (seg, None),
        };
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '$' || c == '@')
        {
            return None;
        }
        cur = cur.get(name)?;
        if let Some(mut idx) = idx {
            while let Some(r) = idx.strip_prefix('[') {
                let end = r.find(']')?;
                let n: usize = r[..end].trim().parse().ok()?;
                cur = cur.get(n)?;
                idx = &r[end + 1..];
            }
            if !idx.is_empty() {
                return None;
            }
        }
    }
    Some(cur.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bindings() {
        let card = json!({"body": [{"type": "TextBlock", "text": "Hi ${name}!"}, {"$data": "${items}", "text": "${$data}", "n": "${$index}"}, {"$when": "${show}", "x": 1}], "t": "${$root.name}"});
        let data = json!({"name": "Ann", "items": ["a", "b"], "show": false});
        let out = expand(&card, &data);
        assert_eq!(
            out,
            json!({"body": [{"type": "TextBlock", "text": "Hi Ann!"}, {"text": "a", "n": 0}, {"text": "b", "n": 1}], "t": "Ann"})
        );
    }
}

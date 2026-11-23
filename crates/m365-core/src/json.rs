//! JavaScript-compatible JSON helpers.
//!
//! Upstream prints values with `JSON.stringify` and converts them to strings
//! with template literals. These helpers reproduce both: number formatting
//! (`1` not `1.0`, `1e+21`), object key order (integer-like keys first, as
//! JavaScript engines order them) and `String(value)` conversion.

use serde_json::{Map, Value};
use std::fmt::Write as _;

pub use serde_json::json;

pub type Obj = Map<String, Value>;

/// Formats a number the way JavaScript's `Number.prototype.toString` does.
pub fn number_to_string(f: f64) -> String {
    if f.is_nan() {
        return "NaN".into();
    }
    if f.is_infinite() {
        return if f > 0.0 {
            "Infinity".into()
        } else {
            "-Infinity".into()
        };
    }
    let mut b = ryu_js::Buffer::new();
    b.format(f).to_string()
}

fn num(n: &serde_json::Number) -> String {
    if let Some(i) = n.as_i64() {
        if i.unsigned_abs() <= (1u64 << 53) {
            return i.to_string();
        }
        return number_to_string(i as f64);
    }
    if let Some(u) = n.as_u64() {
        if u <= (1u64 << 53) {
            return u.to_string();
        }
        return number_to_string(u as f64);
    }
    number_to_string(n.as_f64().unwrap_or(f64::NAN))
}

/// True for keys that JavaScript treats as array indices and therefore
/// orders first, ascending.
fn is_index_key(k: &str) -> Option<u32> {
    if k.is_empty() || k.len() > 10 {
        return None;
    }
    if k.len() > 1 && k.starts_with('0') {
        return None;
    }
    if !k.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let v: u64 = k.parse().ok()?;
    if v < u32::MAX as u64 {
        Some(v as u32)
    } else {
        None
    }
}

/// Object keys in JavaScript enumeration order.
pub fn keys(map: &Obj) -> Vec<&String> {
    let mut idx: Vec<(u32, &String)> = Vec::new();
    let mut rest: Vec<&String> = Vec::with_capacity(map.len());
    for k in map.keys() {
        match is_index_key(k) {
            Some(i) => idx.push((i, k)),
            None => rest.push(k),
        }
    }
    if idx.is_empty() {
        return rest;
    }
    idx.sort_by_key(|(i, _)| *i);
    let mut out: Vec<&String> = idx.into_iter().map(|(_, k)| k).collect();
    out.extend(rest);
    out
}

fn escape_into(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_value(out: &mut String, v: &Value, indent: &str, level: usize) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => out.push_str(&num(n)),
        Value::String(s) => escape_into(out, s),
        Value::Array(a) => {
            if a.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, x) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, indent, level + 1);
                write_value(out, x, indent, level + 1);
            }
            newline(out, indent, level);
            out.push(']');
        }
        Value::Object(m) => {
            if m.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push('{');
            for (i, k) in keys(m).into_iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, indent, level + 1);
                escape_into(out, k);
                out.push(':');
                if !indent.is_empty() {
                    out.push(' ');
                }
                write_value(out, &m[k.as_str()], indent, level + 1);
            }
            newline(out, indent, level);
            out.push('}');
        }
    }
}

fn newline(out: &mut String, indent: &str, level: usize) {
    if indent.is_empty() {
        return;
    }
    out.push('\n');
    for _ in 0..level {
        out.push_str(indent);
    }
}

/// `JSON.stringify(v)`.
pub fn stringify(v: &Value) -> String {
    let mut s = String::new();
    write_value(&mut s, v, "", 0);
    s
}

/// `JSON.stringify(v, null, indent)`.
pub fn stringify_pretty(v: &Value, indent: &str) -> String {
    let mut s = String::new();
    write_value(&mut s, v, indent, 0);
    s
}

/// `String(v)` / `${v}` in JavaScript.
pub fn to_js_string(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => num(n),
        Value::String(s) => s.clone(),
        Value::Array(a) => a
            .iter()
            .map(|x| match x {
                Value::Null => String::new(),
                other => to_js_string(other),
            })
            .collect::<Vec<_>>()
            .join(","),
        Value::Object(_) => "[object Object]".into(),
    }
}

/// JavaScript truthiness.
pub fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0 && !f.is_nan()),
        Value::String(s) => !s.is_empty(),
        _ => true,
    }
}

/// `{ ...a, ...b }`: the keys of `b` over those of `a` (an object, or none).
pub fn merge(a: Value, b: &Value) -> Value {
    let mut a = match a {
        Value::Object(o) => o,
        _ => Obj::new(),
    };
    if let Some(b) = b.as_object() {
        for (k, v) in b {
            a.insert(k.clone(), v.clone());
        }
    }
    Value::Object(a)
}

/// `o[k] = v`, where `v` may be `undefined`: the key is then dropped, as
/// `JSON.stringify` leaves it out.
pub fn assign(o: &mut Obj, k: &str, v: Option<Value>) {
    match v {
        Some(v) => {
            o.insert(k.to_string(), v);
        }
        None => {
            o.shift_remove(k);
        }
    }
}

/// `JSON.parse` that tolerates a leading byte-order mark.
pub fn parse(s: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str(s.strip_prefix('\u{feff}').unwrap_or(s))
}

/// `Number(s)` in JavaScript.
pub fn js_number(s: &str) -> f64 {
    let t = s.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}');
    if t.is_empty() {
        return 0.0;
    }
    let lower = t.to_ascii_lowercase();
    for (p, radix) in [("0x", 16), ("0o", 8), ("0b", 2)] {
        if let Some(rest) = lower.strip_prefix(p) {
            return u64::from_str_radix(rest, radix)
                .map(|v| v as f64)
                .unwrap_or(f64::NAN);
        }
    }
    match t {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    if t.bytes()
        .any(|b| b.is_ascii_alphabetic() && b != b'e' && b != b'E')
    {
        return f64::NAN;
    }
    t.parse::<f64>().unwrap_or(f64::NAN)
}

/// `parseInt(s, 10)` in JavaScript.
pub fn parse_int(s: &str) -> Option<i64> {
    let t = s.trim_start();
    let (neg, t) = match t.as_bytes().first() {
        Some(b'-') => (true, &t[1..]),
        Some(b'+') => (false, &t[1..]),
        _ => (false, t),
    };
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    let v: i64 = digits.parse().ok()?;
    Some(if neg { -v } else { v })
}

/// Converts an f64 into a JSON number value, keeping integers integral.
pub fn num_value(f: f64) -> Value {
    if f.fract() == 0.0 && f.abs() < 9.007_199_254_740_992e15 {
        Value::from(f as i64)
    } else {
        serde_json::Number::from_f64(f)
            .map(Value::Number)
            .unwrap_or(Value::Null)
    }
}

/// Length of a string in UTF-16 code units, as JavaScript reports it.
pub fn js_len(s: &str) -> usize {
    s.encode_utf16().count()
}

/// Stands in for a JavaScript property set to `undefined`: the key keeps its
/// position, `JSON.stringify` would drop it, and markdown output prints it as
/// `undefined` (the heading's `??` skips it).
pub const UNDEFINED: &str = "\u{0}undefined\u{0}";

/// Builds a JSON object; `"key": ?expr` inserts the key only when the
/// `Option` is `Some` (JavaScript drops `undefined` properties).
///
/// ```
/// let name: Option<&str> = None;
/// let v = m365_core::obj!{ "a": 1, "b": ?name, "c": "x" };
/// assert_eq!(m365_core::json::stringify(&v), r#"{"a":1,"c":"x"}"#);
/// ```
#[macro_export]
macro_rules! obj {
    (@put $o:ident;) => {};
    (@put $o:ident; , $($rest:tt)*) => { $crate::obj!(@put $o; $($rest)*) };
    (@put $o:ident; $k:literal : ? $v:expr , $($rest:tt)*) => {
        if let Some(v) = $v { $o.insert($k.to_string(), $crate::Value::from(v)); }
        $crate::obj!(@put $o; $($rest)*)
    };
    (@put $o:ident; $k:literal : $v:expr , $($rest:tt)*) => {
        $o.insert($k.to_string(), $crate::Value::from($v));
        $crate::obj!(@put $o; $($rest)*)
    };
    ($($t:tt)*) => {{
        #[allow(unused_mut)]
        let mut o = $crate::json::Obj::new();
        $crate::obj!(@put o; $($t)* ,);
        $crate::Value::Object(o)
    }};
}

/// `value[key]` as an array (empty when missing).
pub fn arr<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    v.get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}

/// String of `value[key]`, empty when missing.
pub fn s<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

/// The message V8 gives for a `JSON.parse` failure of `text` (without the
/// `SyntaxError: ` prefix), e.g. `Unexpected token 'i', "invalid" is not valid JSON`.
pub fn v8_parse_error(text: &str) -> String {
    // V8 reports these whole-string values without a token
    if matches!(text, "undefined" | "NaN" | "Infinity" | "[object Object]") {
        return format!("\"{text}\" is not valid JSON");
    }
    let c: Vec<u16> = text.encode_utf16().collect();
    let mut p = V8 { c: &c, i: 0 };
    match p.document() {
        Ok(()) => "Unexpected end of JSON input".into(),
        Err(e) => e,
    }
}

struct V8<'a> {
    c: &'a [u16],
    i: usize,
}

impl V8<'_> {
    fn at(&self, msg: &str, pos: usize) -> String {
        let mut line = 1;
        let mut col = 1;
        for &ch in &self.c[..pos.min(self.c.len())] {
            if ch == b'\n' as u16 {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        format!("{msg} in JSON at position {pos} (line {line} column {col})")
    }
    fn unexpected(&self, pos: usize) -> String {
        if pos >= self.c.len() {
            return "Unexpected end of JSON input".into();
        }
        let ch = String::from_utf16_lossy(&self.c[pos..pos + 1]);
        let len = self.c.len();
        let ctx = if len <= 20 {
            format!("\"{}\"", String::from_utf16_lossy(self.c))
        } else {
            let start = pos.saturating_sub(10);
            let end = (pos + 10).min(len);
            format!(
                "{}\"{}\"{}",
                if start > 0 { "..." } else { "" },
                String::from_utf16_lossy(&self.c[start..end]),
                if end < len { "..." } else { "" }
            )
        };
        format!("Unexpected token '{ch}', {ctx} is not valid JSON")
    }
    fn ws(&mut self) {
        while self.i < self.c.len() && matches!(self.c[self.i], 0x20 | 0x09 | 0x0a | 0x0d) {
            self.i += 1;
        }
    }
    fn peek(&self) -> Option<u16> {
        self.c.get(self.i).copied()
    }
    fn document(&mut self) -> Result<(), String> {
        self.ws();
        self.value()?;
        self.ws();
        if self.i < self.c.len() {
            return Err(self.at("Unexpected non-whitespace character after JSON", self.i));
        }
        Ok(())
    }
    fn value(&mut self) -> Result<(), String> {
        match self.peek() {
            None => Err("Unexpected end of JSON input".into()),
            Some(0x7b) => self.object(),
            Some(0x5b) => self.array(),
            Some(0x22) => self.string(),
            Some(ch) if ch == b'-' as u16 || (b'0' as u16..=b'9' as u16).contains(&ch) => {
                self.number()
            }
            Some(ch) if ch == b't' as u16 => self.literal("true"),
            Some(ch) if ch == b'f' as u16 => self.literal("false"),
            Some(ch) if ch == b'n' as u16 => self.literal("null"),
            Some(_) => Err(self.unexpected(self.i)),
        }
    }
    fn literal(&mut self, word: &str) -> Result<(), String> {
        for b in word.bytes() {
            match self.peek() {
                None => return Err("Unexpected end of JSON input".into()),
                Some(ch) if ch == b as u16 => self.i += 1,
                Some(_) => return Err(self.unexpected(self.i)),
            }
        }
        Ok(())
    }
    fn digit(&self) -> bool {
        self.peek()
            .is_some_and(|ch| (b'0' as u16..=b'9' as u16).contains(&ch))
    }
    fn number(&mut self) -> Result<(), String> {
        if self.peek() == Some(b'-' as u16) {
            self.i += 1;
            if !self.digit() {
                return Err(self.at("No number after minus sign", self.i));
            }
        }
        if self.peek() == Some(b'0' as u16) {
            self.i += 1;
            if self.digit() {
                return Err(self.at("Unexpected number", self.i));
            }
        } else {
            while self.digit() {
                self.i += 1;
            }
        }
        if self.peek() == Some(b'.' as u16) {
            self.i += 1;
            if !self.digit() {
                return Err(self.at("Unterminated fractional number", self.i));
            }
            while self.digit() {
                self.i += 1;
            }
        }
        if matches!(self.peek(), Some(0x65 | 0x45)) {
            self.i += 1;
            if matches!(self.peek(), Some(0x2b | 0x2d)) {
                self.i += 1;
            }
            if !self.digit() {
                return Err(self.at("Exponent part is missing a number", self.i));
            }
            while self.digit() {
                self.i += 1;
            }
        }
        Ok(())
    }
    fn string(&mut self) -> Result<(), String> {
        self.i += 1;
        loop {
            match self.peek() {
                None => return Err(self.at("Unterminated string", self.i)),
                Some(0x22) => {
                    self.i += 1;
                    return Ok(());
                }
                Some(0x5c) => {
                    self.i += 1;
                    match self.peek() {
                        None => return Err(self.at("Unterminated string", self.i)),
                        Some(ch) if b"\"\\/bfnrt".iter().any(|b| *b as u16 == ch) => self.i += 1,
                        Some(ch) if ch == b'u' as u16 => {
                            self.i += 1;
                            for _ in 0..4 {
                                match self.peek() {
                                    Some(h)
                                        if (h as u8 as u16 == h)
                                            && (h as u8).is_ascii_hexdigit() =>
                                    {
                                        self.i += 1
                                    }
                                    _ => return Err(self.at("Bad Unicode escape", self.i)),
                                }
                            }
                        }
                        Some(_) => return Err(self.at("Bad escaped character", self.i)),
                    }
                }
                Some(ch) if ch < 0x20 => {
                    return Err(self.at("Bad control character in string literal", self.i));
                }
                Some(_) => self.i += 1,
            }
        }
    }
    fn object(&mut self) -> Result<(), String> {
        self.i += 1;
        self.ws();
        if self.peek() == Some(0x7d) {
            self.i += 1;
            return Ok(());
        }
        let mut first = true;
        loop {
            self.ws();
            if self.peek() != Some(0x22) {
                return Err(self.at(
                    if first {
                        "Expected property name or '}'"
                    } else {
                        "Expected double-quoted property name"
                    },
                    self.i,
                ));
            }
            first = false;
            self.string()?;
            self.ws();
            if self.peek() != Some(b':' as u16) {
                return Err(self.at("Expected ':' after property name", self.i));
            }
            self.i += 1;
            self.ws();
            self.value()?;
            self.ws();
            match self.peek() {
                Some(0x2c) => self.i += 1,
                Some(0x7d) => {
                    self.i += 1;
                    return Ok(());
                }
                _ => return Err(self.at("Expected ',' or '}' after property value", self.i)),
            }
        }
    }
    fn array(&mut self) -> Result<(), String> {
        self.i += 1;
        self.ws();
        if self.peek() == Some(0x5d) {
            self.i += 1;
            return Ok(());
        }
        loop {
            self.ws();
            self.value()?;
            self.ws();
            match self.peek() {
                Some(0x2c) => self.i += 1,
                Some(0x5d) => {
                    self.i += 1;
                    return Ok(());
                }
                _ => return Err(self.at("Expected ',' or ']' after array element", self.i)),
            }
        }
    }
}

/// `JSON.parse` with V8's error message (`SyntaxError: ...`) on failure.
pub fn js_parse(text: &str) -> Result<Value, String> {
    serde_json::from_str(text).map_err(|_| format!("SyntaxError: {}", v8_parse_error(text)))
}

/// `JSON.parse` failing with a [`crate::error::Error::Syntax`] carrying V8's message.
pub fn jparse(text: &str) -> Result<Value, crate::error::Error> {
    serde_json::from_str(text).map_err(|_| crate::error::Error::Syntax(v8_parse_error(text)))
}

/// `${value[key]}`: the property as JavaScript prints it, `undefined` when
/// missing.
pub fn u(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(x) => to_js_string(x),
        None => "undefined".into(),
    }
}

/// `v[key].method(...)` on an array, failing as JavaScript does when the
/// property is missing (`Cannot read properties of undefined`) or not an
/// array (`... is not a function`).
pub fn need_arr<'a>(
    v: &'a Value,
    key: &str,
    method: &str,
) -> Result<&'a [Value], crate::error::Error> {
    match v.get(key) {
        Some(Value::Array(a)) => Ok(a),
        None | Some(Value::Null) => Err(crate::error::Error::js(format!(
            "Cannot read properties of undefined (reading '{method}')"
        ))),
        Some(_) => Err(crate::error::Error::js(format!(
            "{key}.{method} is not a function"
        ))),
    }
}

/// `v.a.b.c` with JavaScript's errors: reading through `undefined` or `null`
/// throws `Cannot read properties of undefined (reading 'k')`; the last
/// property may be missing (`Ok(None)`).
pub fn dig<'a>(v: &'a Value, keys: &[&str]) -> Result<Option<&'a Value>, crate::error::Error> {
    let mut cur = Some(v);
    for k in keys {
        cur = match cur {
            None => {
                return Err(crate::error::Error::js(format!(
                    "Cannot read properties of undefined (reading '{k}')"
                )));
            }
            Some(Value::Null) => {
                return Err(crate::error::Error::js(format!(
                    "Cannot read properties of null (reading '{k}')"
                )));
            }
            Some(Value::Array(a)) => k.parse::<usize>().ok().and_then(|i| a.get(i)),
            Some(c) => c.get(*k),
        };
    }
    Ok(cur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_and_keys_follow_javascript() {
        let v: Value = serde_json::from_str(r#"{"b":1.0,"2":1e21,"1":0.1,"a":[]}"#).unwrap();
        assert_eq!(stringify(&v), r#"{"1":0.1,"2":1e+21,"b":1,"a":[]}"#);
        assert_eq!(
            stringify_pretty(&json!({"a":{"b":[1,{}]}}), "  "),
            "{\n  \"a\": {\n    \"b\": [\n      1,\n      {}\n    ]\n  }\n}"
        );
        assert_eq!(to_js_string(&json!([1, "a", null])), "1,a,");
        assert_eq!(js_number("0x1f"), 31.0);
        assert!(js_number("abc").is_nan());
        assert_eq!(js_number(""), 0.0);
    }
}

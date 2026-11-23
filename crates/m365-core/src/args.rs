//! A port of the parts of `yargs-parser` 22 that upstream relies on, with the
//! configuration upstream passes in `src/cli/cli.ts`:
//! `parse-numbers`, `strip-aliased`, `boolean-negation`, no dot notation, no
//! camel-case expansion, duplicate options collected into arrays.
//!
//! Values are `serde_json::Value`s. `Value::Null` stands for JavaScript's
//! `NaN`, which yargs produces for a number option with a non-numeric value;
//! nothing else in parsing produces `null`.

use crate::json::{Obj, js_number, num_value};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct Flags {
    pub bools: HashSet<String>,
    pub strings: HashSet<String>,
    pub numbers: HashSet<String>,
    /// long name -> short name
    pub aliases: HashMap<String, String>,
}

impl Flags {
    fn names(&self, key: &str) -> Vec<String> {
        let mut v = vec![key.to_string()];
        if let Some(s) = self.aliases.get(key) {
            v.push(s.clone());
        }
        for (l, s) in &self.aliases {
            if s == key {
                v.push(l.clone());
            }
        }
        v
    }

    fn is(&self, set: &HashSet<String>, key: &str) -> bool {
        if set.is_empty() {
            return false;
        }
        self.names(key).iter().any(|k| set.contains(k))
    }

    fn is_bool(&self, key: &str) -> bool {
        self.is(&self.bools, key)
    }
    fn is_string(&self, key: &str) -> bool {
        self.is(&self.strings, key)
    }
    fn is_number(&self, key: &str) -> bool {
        self.is(&self.numbers, key)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Parsed {
    pub positional: Vec<Value>,
    pub options: Obj,
}

fn is_negative(s: &str) -> bool {
    // /^-([0-9]+(\.[0-9]+)?|\.[0-9]+)$/
    let Some(rest) = s.strip_prefix('-') else {
        return false;
    };
    if let Some(frac) = rest.strip_prefix('.') {
        return !frac.is_empty() && frac.bytes().all(|b| b.is_ascii_digit());
    }
    let (int, frac) = match rest.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (rest, None),
    };
    !int.is_empty()
        && int.bytes().all(|b| b.is_ascii_digit())
        && frac.is_none_or(|f| !f.is_empty() && f.bytes().all(|b| b.is_ascii_digit()))
}

/// yargs-parser's `looksLikeNumber`.
pub fn looks_like_number(x: &str) -> bool {
    let b = x.as_bytes();
    if b.len() > 2 && b[0] == b'0' && (b[1] == b'x' || b[1] == b'X') {
        return b[2..].iter().all(|c| c.is_ascii_hexdigit());
    }
    if b.len() >= 2 && b[0] == b'0' && b[1] != b'.' {
        return false;
    }
    // /^[-]?(?:\d+(?:\.\d*)?|\.\d+)(e[-+]?\d+)?$/
    let mut i = 0;
    if i < b.len() && b[i] == b'-' {
        i += 1;
    }
    let start = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i > start {
        if i < b.len() && b[i] == b'.' {
            i += 1;
            while i < b.len() && b[i].is_ascii_digit() {
                i += 1;
            }
        }
    } else {
        if i >= b.len() || b[i] != b'.' {
            return false;
        }
        i += 1;
        let s = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == s {
            return false;
        }
    }
    if i < b.len() && b[i] == b'e' {
        i += 1;
        if i < b.len() && (b[i] == b'-' || b[i] == b'+') {
            i += 1;
        }
        let s = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == s {
            return false;
        }
    }
    i == b.len()
}

enum Raw {
    Str(String),
    Bool(bool),
    Undefined,
}

struct Parser<'a> {
    flags: &'a Flags,
    out: Parsed,
}

impl Parser<'_> {
    fn coerce(&self, key: &str, v: Raw) -> Option<Value> {
        let v = match v {
            Raw::Str(s) if self.flags.is_bool(key) => Raw::Bool(s == "true"),
            other => other,
        };
        match v {
            Raw::Bool(b) => Some(Value::Bool(b)),
            Raw::Undefined => None,
            Raw::Str(s) => {
                if key != "_" && (self.flags.is_string(key) || self.flags.is_bool(key)) {
                    return Some(Value::String(s));
                }
                let safe = looks_like_number(&s) && {
                    let f = parse_float(&s).floor();
                    f.is_finite() && f.abs() <= 9_007_199_254_740_991.0
                };
                if safe || (key != "_" && self.flags.is_number(key)) {
                    let n = js_number(&s);
                    return Some(if n.is_nan() {
                        Value::Null
                    } else {
                        num_value(n)
                    });
                }
                Some(Value::String(s))
            }
        }
    }

    fn set_arg(&mut self, key: &str, v: Raw) {
        let value = self.coerce(key, v);
        let mut keys = vec![key.to_string()];
        if let Some(s) = self.flags.aliases.get(key) {
            keys.push(s.clone());
        }
        for (l, s) in &self.flags.aliases {
            if s == key {
                keys.push(l.clone());
            }
        }
        let is_bool = self.flags.is_bool(key);
        for k in keys {
            let value = value.clone();
            let opts = &mut self.out.options;
            match (opts.get_mut(&k), value) {
                (Some(Value::Array(a)), Some(v)) if !is_bool => a.push(v),
                (Some(existing), Some(v)) if !is_bool && !existing.is_null() => {
                    let old = existing.take();
                    *existing = Value::Array(vec![old, v]);
                }
                (_, Some(v)) => {
                    opts.insert(k, v);
                }
                (_, None) => {
                    // yargs stores `undefined`; an absent key behaves the same
                }
            }
        }
    }

    fn default_value(&self, key: &str) -> Raw {
        if self.flags.is_string(key) {
            Raw::Str(String::new())
        } else if self.flags.is_number(key) {
            Raw::Undefined
        } else {
            Raw::Bool(true)
        }
    }

    fn push_positional(&mut self, arg: &str) {
        if let Some(v) = self.coerce("_", Raw::Str(arg.to_string())) {
            self.out.positional.push(v);
        }
    }
}

fn parse_float(s: &str) -> f64 {
    // enough for looksLikeNumber inputs
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        let _ = h;
        return 0.0;
    }
    s.parse::<f64>().unwrap_or(f64::NAN)
}

/// Parses `args` like `yargs-parser(args, { alias, boolean, string, number })`.
pub fn parse(args: &[String], flags: &Flags) -> Parsed {
    let mut p = Parser {
        flags,
        out: Parsed::default(),
    };
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        let next = args.get(i + 1).map(String::as_str);
        let dashes = arg.bytes().take_while(|b| *b == b'-').count();
        if dashes >= 3 && (arg.len() == dashes || arg.as_bytes()[dashes] == b'=') {
            p.push_positional(arg);
        } else if arg.starts_with("--")
            && arg.len() > 2
            && arg[2..].contains('=')
            && arg.as_bytes()[2] != b'='
        {
            let body = &arg[2..];
            let (k, v) = body.split_once('=').unwrap();
            p.set_arg(k, Raw::Str(v.to_string()));
        } else if let Some(k) = arg.strip_prefix("--no-").filter(|k| !k.is_empty()) {
            p.set_arg(k, Raw::Bool(false));
        } else if arg.starts_with("--") && arg.len() > 2 {
            let key = &arg[2..];
            match next {
                Some(n) if (!n.starts_with('-') || is_negative(n)) && !flags.is_bool(key) => {
                    p.set_arg(key, Raw::Str(n.to_string()));
                    i += 1;
                }
                Some(n @ ("true" | "false")) => {
                    p.set_arg(key, Raw::Str(n.to_string()));
                    i += 1;
                }
                _ => {
                    let d = p.default_value(key);
                    p.set_arg(key, d);
                }
            }
        } else if arg.len() > 3
            && arg.starts_with('-')
            && arg.as_bytes()[2] == b'.'
            && arg[1..].contains('=')
            && !arg.starts_with("--")
        {
            let body = &arg[1..];
            let (k, v) = body.split_once('=').unwrap();
            p.set_arg(k, Raw::Str(v.to_string()));
        } else if arg.len() > 3
            && arg.starts_with('-')
            && !arg.starts_with("--")
            && arg.as_bytes()[2] == b'.'
            && !is_negative(arg)
        {
            let key = &arg[1..];
            match next {
                Some(n) if !n.starts_with('-') && !flags.is_bool(key) => {
                    p.set_arg(key, Raw::Str(n.to_string()));
                    i += 1;
                }
                _ => {
                    let d = p.default_value(key);
                    p.set_arg(key, d);
                }
            }
        } else if arg.len() > 1
            && arg.starts_with('-')
            && !arg.starts_with("--")
            && !is_negative(arg)
        {
            let chars: Vec<char> = arg.chars().collect();
            let letters: Vec<char> = chars[1..chars.len() - 1].to_vec();
            let mut broken = false;
            let mut j = 0;
            while j < letters.len() {
                let rest: String = chars[(j + 2).min(chars.len())..].iter().collect();
                let l = letters[j].to_string();
                if letters.get(j + 1) == Some(&'=') {
                    let value: String = chars[(j + 3).min(chars.len())..].iter().collect();
                    p.set_arg(&l, Raw::Str(value));
                    broken = true;
                    break;
                }
                if rest == "-" {
                    p.set_arg(&l, Raw::Str(rest));
                    j += 1;
                    continue;
                }
                if letters[j].is_ascii_alphabetic()
                    && is_short_number(&rest)
                    && !flags.is_bool(&rest)
                {
                    p.set_arg(&l, Raw::Str(rest));
                    broken = true;
                    break;
                }
                if letters
                    .get(j + 1)
                    .is_some_and(|c| !(c.is_ascii_alphanumeric() || *c == '_'))
                {
                    p.set_arg(&l, Raw::Str(rest));
                    broken = true;
                    break;
                }
                let d = p.default_value(&l);
                p.set_arg(&l, d);
                j += 1;
            }
            let key = chars[chars.len() - 1].to_string();
            if !broken && key != "-" {
                match next {
                    Some(n) if (!starts_flag(n) || is_negative(n)) && !flags.is_bool(&key) => {
                        p.set_arg(&key, Raw::Str(n.to_string()));
                        i += 1;
                    }
                    Some(n @ ("true" | "false")) => {
                        p.set_arg(&key, Raw::Str(n.to_string()));
                        i += 1;
                    }
                    _ => {
                        let d = p.default_value(&key);
                        p.set_arg(&key, d);
                    }
                }
            }
        } else if arg.len() == 2
            && arg.starts_with('-')
            && arg.as_bytes()[1].is_ascii_digit()
            && flags.is_bool(&arg[1..])
        {
            let key = &arg[1..];
            let d = p.default_value(key);
            p.set_arg(key, d);
        } else if arg == "--" {
            for rest in &args[i + 1..] {
                p.out.positional.push(Value::String(rest.clone()));
            }
            break;
        } else {
            p.push_positional(arg);
        }
        i += 1;
    }
    // strip-aliased: remove the short names
    for short in flags.aliases.values() {
        p.out.options.shift_remove(short);
    }
    p.out
}

/// `/^-?\d+(\.\d*)?(e-?\d+)?$/`
fn is_short_number(s: &str) -> bool {
    let b = s.as_bytes();
    let mut i = 0;
    if b.first() == Some(&b'-') {
        i = 1;
    }
    let st = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    if i == st {
        return false;
    }
    if i < b.len() && b[i] == b'.' {
        i += 1;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i < b.len() && b[i] == b'e' {
        i += 1;
        if i < b.len() && b[i] == b'-' {
            i += 1;
        }
        let s2 = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        if i == s2 {
            return false;
        }
    }
    i == b.len()
}

/// `/^(-|--)[^-]/`
fn starts_flag(s: &str) -> bool {
    let b = s.as_bytes();
    (b.len() >= 2 && b[0] == b'-' && b[1] != b'-')
        || (b.len() >= 3 && b[0] == b'-' && b[1] == b'-' && b[2] != b'-')
}

/// `formatting.rewriteBooleanValue`.
pub fn rewrite_boolean(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => "true".into(),
        "0" | "false" | "no" | "off" => "false".into(),
        _ => value.into(),
    }
}

/// `getRewrittenArgs` in `cli.ts`: values after boolean options are mapped to
/// `true`/`false`, or rejected.
pub fn rewrite_args(args: &[String], booleans: &[&str]) -> Result<Vec<String>, String> {
    if booleans.is_empty() {
        return Ok(args.to_vec());
    }
    let mut out = Vec::with_capacity(args.len());
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with('-') || i == 0 {
            out.push(arg.clone());
            continue;
        }
        let prev = &args[i - 1];
        if booleans
            .iter()
            .any(|t| *prev == format!("--{t}") || *prev == format!("-{t}"))
        {
            let r = rewrite_boolean(arg);
            if r != "true" && r != "false" {
                return Err(format!(
                    "The value '{arg}' for option '{prev}' is not a valid boolean"
                ));
            }
            out.push(r);
        } else {
            out.push(arg.clone());
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn parses_like_yargs() {
        let mut f = Flags::default();
        f.strings.insert("id".into());
        f.bools.insert("withManager".into());
        f.numbers.insert("top".into());
        f.aliases.insert("id".into(), "i".into());
        let p = parse(
            &s(&[
                "entra",
                "user",
                "get",
                "-i",
                "123",
                "--withManager",
                "--top",
                "x",
                "--other",
                "5",
                "--no-flag",
            ]),
            &f,
        );
        assert_eq!(
            p.positional,
            vec![
                Value::from("entra"),
                Value::from("user"),
                Value::from("get")
            ]
        );
        assert_eq!(p.options["id"], Value::from("123"));
        assert_eq!(p.options["withManager"], Value::Bool(true));
        assert_eq!(p.options["top"], Value::Null);
        assert_eq!(p.options["other"], Value::from(5));
        assert_eq!(p.options["flag"], Value::Bool(false));
        assert!(!p.options.contains_key("i"));
        let p = parse(
            &s(&["--a", "1", "--a", "2", "--b=x=y", "-o", "json", "--id"]),
            &f,
        );
        assert_eq!(p.options["a"], serde_json::json!([1, 2]));
        assert_eq!(p.options["b"], Value::from("x=y"));
        assert_eq!(p.options["o"], Value::from("json"));
        assert_eq!(p.options["id"], Value::from(""));
        let p = parse(&s(&["--n", "-1", "--z", "0123"]), &f);
        assert_eq!(p.options["n"], Value::from(-1));
        assert_eq!(p.options["z"], Value::from("0123"));
    }
}

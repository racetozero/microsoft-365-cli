//! `yaml.stringify` (yaml 2.9, default options) for the JSON-like documents
//! the workflow and pipeline commands write: block maps and sequences, and
//! a port of `stringifyString`/`foldFlowLines` for scalars.

use m365_core::prelude::*;
use regex::Regex;
use std::sync::LazyLock;

const LINE_WIDTH: usize = 80;
const MIN_CONTENT_WIDTH: usize = 20;
const DQ_MIN_MULTILINE: usize = 40;

#[derive(Clone, Copy, PartialEq)]
enum Fold {
    Flow,
    Block,
    Quoted,
}

struct Ctx<'a> {
    indent: &'a str,
    implicit_key: bool,
    indent_at_start: Option<usize>,
}

fn re(s: &str) -> Regex {
    Regex::new(s).expect("yaml regex")
}

static DOC_MARKER: LazyLock<Regex> = LazyLock::new(|| re(r"(?m)^(%|---|\.\.\.)"));
static PLAIN_UNSAFE: LazyLock<Regex> = LazyLock::new(|| {
    re(r#"^[\n\t ,\[\]{}#&*!|>'"%@`]|^[?-]$|^[?-][ \t]|[\n:][ \t]|[ \t]\n|[\n\t ]#|[\n\t :]$"#)
});
static TAGS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"^(?:~|[Nn]ull|NULL)?$",
        r"^(?:[Tt]rue|TRUE|[Ff]alse|FALSE)$",
        r"^0o[0-7]+$",
        r"^[-+]?[0-9]+$",
        r"^0x[0-9a-fA-F]+$",
        r"^(?:[-+]?\.(?:inf|Inf|INF)|\.nan|\.NaN|\.NAN)$",
        r"^[-+]?(?:\.[0-9]+|[0-9]+(?:\.[0-9]*)?)[eE][-+]?[0-9]+$",
        r"^[-+]?(?:\.[0-9]+|[0-9]+\.[0-9]*)$",
    ]
    .iter()
    .map(|s| re(s))
    .collect()
});

fn contains_doc_marker(s: &str) -> bool {
    DOC_MARKER.is_match(s)
}

/// `foldFlowLines` on UTF-16-ish char indexes (the documents are ASCII).
fn fold(
    text: &str,
    indent: &str,
    mode: Fold,
    indent_at_start: Option<usize>,
    on_overflow: Option<&mut bool>,
) -> String {
    let t: Vec<char> = text.chars().collect();
    let min_content = if LINE_WIDTH < MIN_CONTENT_WIDTH {
        0
    } else {
        MIN_CONTENT_WIDTH
    };
    let end_step = (1 + min_content).max((1 + LINE_WIDTH).saturating_sub(indent.len()));
    if t.len() <= end_step {
        return text.to_string();
    }
    let mut folds: Vec<usize> = Vec::new();
    let mut escaped = std::collections::HashSet::new();
    let mut end = LINE_WIDTH as i64 - indent.len() as i64;
    if let Some(ias) = indent_at_start {
        if ias > LINE_WIDTH - 2usize.max(min_content) {
            folds.push(0);
        } else {
            end = LINE_WIDTH as i64 - ias as i64;
        }
    }
    let mut split: Option<usize> = None;
    let mut prev: Option<char> = None;
    let mut overflow = false;
    let mut i: i64 = -1;
    let (mut esc_start, mut esc_end): (i64, i64) = (-1, -1);
    if mode == Fold::Block {
        i = more_indented(&t, i, indent.len());
        if i != -1 {
            end = i + end_step as i64;
        }
    }
    loop {
        i += 1;
        let Some(&c0) = t.get(i as usize) else { break };
        let mut ch = c0;
        if mode == Fold::Quoted && ch == '\\' {
            esc_start = i;
            i += match t.get(i as usize + 1) {
                Some('x') => 3,
                Some('u') => 5,
                Some('U') => 9,
                _ => 1,
            };
            esc_end = i;
        }
        if ch == '\n' {
            if mode == Fold::Block {
                i = more_indented(&t, i, indent.len());
            }
            end = i + indent.len() as i64 + end_step as i64;
            split = None;
        } else {
            if ch == ' ' && prev.is_some_and(|p| p != ' ' && p != '\n' && p != '\t') {
                let next = t.get(i as usize + 1);
                if next.is_some_and(|n| *n != ' ' && *n != '\n' && *n != '\t') {
                    split = Some(i as usize);
                }
            }
            if i >= end {
                if let Some(s) = split.filter(|s| *s != 0) {
                    folds.push(s);
                    end = (s + end_step) as i64;
                    split = None;
                } else if mode == Fold::Quoted {
                    while prev == Some(' ') || prev == Some('\t') {
                        prev = Some(ch);
                        i += 1;
                        ch = t.get(i as usize).copied().unwrap_or('\0');
                        overflow = true;
                    }
                    let j = if i > esc_end + 1 {
                        i - 2
                    } else {
                        esc_start - 1
                    };
                    if escaped.contains(&j) {
                        return text.to_string();
                    }
                    folds.push(j.max(0) as usize);
                    escaped.insert(j);
                    end = j + end_step as i64;
                    split = None;
                } else {
                    overflow = true;
                }
            }
        }
        prev = Some(ch);
    }
    if overflow && let Some(o) = on_overflow {
        *o = true;
    }
    if folds.is_empty() {
        return text.to_string();
    }
    let s = |a: usize, b: usize| t[a.min(t.len())..b.min(t.len())].iter().collect::<String>();
    let mut res = s(0, folds[0]);
    for (k, &f) in folds.iter().enumerate() {
        let e = folds
            .get(k + 1)
            .copied()
            .filter(|x| *x != 0)
            .unwrap_or(t.len());
        if f == 0 {
            res = format!("\n{indent}{}", s(0, e));
        } else {
            if mode == Fold::Quoted && escaped.contains(&(f as i64)) {
                res.push(t[f]);
                res.push('\\');
            }
            res.push_str(&format!("\n{indent}{}", s(f + 1, e)));
        }
    }
    res
}

fn more_indented(t: &[char], mut i: i64, indent: usize) -> i64 {
    let mut end = i;
    let mut start = i + 1;
    let mut ch = t.get(start as usize).copied();
    while ch == Some(' ') || ch == Some('\t') {
        if i < start + indent as i64 {
            i += 1;
            ch = t.get(i as usize).copied();
        } else {
            loop {
                i += 1;
                ch = t.get(i as usize).copied();
                if ch.is_none() || ch == Some('\n') {
                    break;
                }
            }
            end = i;
            start = i + 1;
            ch = t.get(start as usize).copied();
        }
    }
    end
}

fn double_quoted(value: &str, ctx: &Ctx) -> String {
    let json: Vec<char> = stringify(&Value::from(value)).chars().collect();
    let indent = if !ctx.indent.is_empty() {
        ctx.indent.to_string()
    } else if contains_doc_marker(value) {
        "  ".into()
    } else {
        String::new()
    };
    let mut out = String::new();
    let mut start = 0usize;
    let mut i = 0usize;
    let slice = |a: usize, b: usize| json[a..b.min(json.len())].iter().collect::<String>();
    while i < json.len() {
        let mut ch = json[i];
        if ch == ' ' && json.get(i + 1) == Some(&'\\') && json.get(i + 2) == Some(&'n') {
            out.push_str(&slice(start, i));
            out.push_str("\\ ");
            i += 1;
            start = i;
            ch = '\\';
        }
        if ch == '\\' {
            match json.get(i + 1) {
                Some('u') => {
                    out.push_str(&slice(start, i));
                    let code = slice(i + 2, i + 6);
                    out.push_str(&match code.as_str() {
                        "0000" => "\\0".to_string(),
                        "0007" => "\\a".into(),
                        "000b" => "\\v".into(),
                        "001b" => "\\e".into(),
                        "0085" => "\\N".into(),
                        "00a0" => "\\_".into(),
                        "2028" => "\\L".into(),
                        "2029" => "\\P".into(),
                        c if c.starts_with("00") => format!("\\x{}", &c[2..]),
                        _ => slice(i, i + 6),
                    });
                    i += 5;
                    start = i + 1;
                }
                Some('n') => {
                    if ctx.implicit_key
                        || json.get(i + 2) == Some(&'"')
                        || json.len() < DQ_MIN_MULTILINE
                    {
                        i += 1;
                    } else {
                        out.push_str(&slice(start, i));
                        out.push_str("\n\n");
                        while json.get(i + 2) == Some(&'\\')
                            && json.get(i + 3) == Some(&'n')
                            && json.get(i + 4) != Some(&'"')
                        {
                            out.push('\n');
                            i += 2;
                        }
                        out.push_str(&indent);
                        if json.get(i + 2) == Some(&' ') {
                            out.push('\\');
                        }
                        i += 1;
                        start = i + 1;
                    }
                }
                _ => i += 1,
            }
        }
        i += 1;
    }
    let s = if start > 0 {
        format!("{out}{}", slice(start, json.len()))
    } else {
        json.iter().collect()
    };
    if ctx.implicit_key {
        s
    } else {
        fold(&s, &indent, Fold::Quoted, ctx.indent_at_start, None)
    }
}

fn single_quoted(value: &str, ctx: &Ctx) -> String {
    static WS_NL: LazyLock<Regex> = LazyLock::new(|| re(r"[ \t]\n|\n[ \t]"));
    if (ctx.implicit_key && value.contains('\n')) || WS_NL.is_match(value) {
        return double_quoted(value, ctx);
    }
    let indent = if !ctx.indent.is_empty() {
        ctx.indent.to_string()
    } else if contains_doc_marker(value) {
        "  ".into()
    } else {
        String::new()
    };
    static NLS: LazyLock<Regex> = LazyLock::new(|| re(r"\n+"));
    let body = value.replace('\'', "''");
    let body = NLS.replace_all(&body, |c: &regex::Captures| format!("{}\n{indent}", &c[0]));
    let res = format!("'{body}'");
    if ctx.implicit_key {
        res
    } else {
        fold(&res, &indent, Fold::Flow, ctx.indent_at_start, None)
    }
}

fn quoted(value: &str, ctx: &Ctx) -> String {
    let (d, s) = (value.contains('"'), value.contains('\''));
    if d && !s {
        single_quoted(value, ctx)
    } else {
        double_quoted(value, ctx)
    }
}

fn block(value: &str, ctx: &Ctx) -> String {
    static TRAIL_WS: LazyLock<Regex> = LazyLock::new(|| re(r"\n[\t ]+$"));
    if TRAIL_WS.is_match(value) {
        return quoted(value, ctx);
    }
    let indent = if !ctx.indent.is_empty() || contains_doc_marker(value) {
        if ctx.indent.is_empty() {
            "  ".to_string()
        } else {
            ctx.indent.to_string()
        }
    } else {
        String::new()
    };
    let literal = !line_over_limit(value, indent.len());
    if value.is_empty() {
        return if literal { "|\n".into() } else { ">\n".into() };
    }
    let chars: Vec<char> = value.chars().collect();
    let mut end_start = chars.len();
    while end_start > 0 && matches!(chars[end_start - 1], '\n' | '\t' | ' ') {
        end_start -= 1;
    }
    let mut end: String = chars[end_start..].iter().collect();
    let chomp = match end.find('\n') {
        None => "-",
        Some(p) if value == end || p != end.len() - 1 => "+",
        Some(_) => "",
    };
    let mut value: String = value.to_string();
    if !end.is_empty() {
        value = chars[..end_start].iter().collect();
        if end.ends_with('\n') {
            end.pop();
        }
        // `(^|(?<!\n))\n+(?!\n|$)` -> `$&<indent>`: runs of newlines not at the end
        end = indent_inner_newline_runs(&end, &indent);
    }
    let vchars: Vec<char> = value.chars().collect();
    let mut start_with_space = false;
    let mut start_nl: i64 = -1;
    let mut start_end = 0;
    while start_end < vchars.len() {
        match vchars[start_end] {
            ' ' => start_with_space = true,
            '\n' => start_nl = start_end as i64,
            _ => break,
        }
        start_end += 1;
    }
    let cut = if start_nl < start_end as i64 {
        (start_nl + 1) as usize
    } else {
        start_end
    };
    let mut start: String = vchars[..cut].iter().collect();
    static NLS: LazyLock<Regex> = LazyLock::new(|| re(r"\n+"));
    if !start.is_empty() {
        value = vchars[cut..].iter().collect();
        start = NLS
            .replace_all(&start, |c: &regex::Captures| format!("{}{indent}", &c[0]))
            .into_owned();
    }
    let indent_size = if indent.is_empty() { "1" } else { "2" };
    let header = format!("{}{chomp}", if start_with_space { indent_size } else { "" });
    if !literal {
        static MORE: LazyLock<Regex> =
            LazyLock::new(|| re(r"(?:^|\n)([\t ].*)(?:([\n\t ]*)\n([^\n\t ]))?"));
        let folded = NLS
            .replace_all(&value, |c: &regex::Captures| format!("\n{}", &c[0]))
            .into_owned();
        let folded = more_indented_unfold(&folded, &MORE);
        let folded = NLS
            .replace_all(&folded, |c: &regex::Captures| format!("{}{indent}", &c[0]))
            .into_owned();
        let mut fallback = false;
        let body = fold(
            &format!("{start}{folded}{end}"),
            &indent,
            Fold::Block,
            Some(indent.len()),
            Some(&mut fallback),
        );
        if !fallback {
            return format!(">{header}\n{indent}{body}");
        }
    }
    let value = NLS
        .replace_all(&value, |c: &regex::Captures| format!("{}{indent}", &c[0]))
        .into_owned();
    format!("|{header}\n{indent}{start}{value}{end}")
}

/// `.replace(/(?:^|\n)([\t ].*)(?:([\n\t ]*)\n(?![\n\t ]))?/g, '$1$2')`.
fn more_indented_unfold(s: &str, re: &Regex) -> String {
    // the lookahead `(?![\n\t ])` is emulated by capturing the next char
    // and putting it back
    let mut out = String::new();
    let mut last = 0;
    let mut pos = 0;
    while let Some(c) = re.captures_at(s, pos) {
        let m = c.get(0).expect("match");
        out.push_str(&s[last..m.start()]);
        out.push_str(&c[1]);
        if let Some(g2) = c.get(2) {
            out.push_str(g2.as_str());
        }
        let mut e = m.end();
        if let Some(g3) = c.get(3) {
            e = g3.start();
        }
        last = e;
        pos = if e > m.start() { e } else { m.start() + 1 };
        if pos > s.len() {
            break;
        }
    }
    out.push_str(&s[last.min(s.len())..]);
    out
}

fn indent_inner_newline_runs(s: &str, indent: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '\n' && (i == 0 || chars[i - 1] != '\n') {
            let mut j = i;
            while j < chars.len() && chars[j] == '\n' {
                j += 1;
            }
            out.extend(&chars[i..j]);
            if j < chars.len() {
                out.push_str(indent);
            }
            i = j;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn line_over_limit(s: &str, indent_len: usize) -> bool {
    let limit = LINE_WIDTH - indent_len;
    let c: Vec<char> = s.chars().collect();
    if c.len() <= limit {
        return false;
    }
    let mut start = 0;
    for (i, ch) in c.iter().enumerate() {
        if *ch == '\n' {
            if i - start > limit {
                return true;
            }
            start = i + 1;
            if c.len() - start <= limit {
                return false;
            }
        }
    }
    true
}

fn plain(value: &str, ctx: &Ctx) -> String {
    if ctx.implicit_key && value.contains('\n') {
        return quoted(value, ctx);
    }
    if PLAIN_UNSAFE.is_match(value) {
        return if ctx.implicit_key || !value.contains('\n') {
            quoted(value, ctx)
        } else {
            block(value, ctx)
        };
    }
    if !ctx.implicit_key && value.contains('\n') {
        // the default string type is PLAIN, so a multi-line plain string
        // stays plain (folded with blank lines)
    }
    if contains_doc_marker(value) && ctx.indent.is_empty() {
        return block(
            value,
            &Ctx {
                indent: "  ",
                ..*ctx
            },
        );
    }
    static NLS: LazyLock<Regex> = LazyLock::new(|| re(r"\n+"));
    let s = NLS
        .replace_all(value, |c: &regex::Captures| {
            format!("{}\n{}", &c[0], ctx.indent)
        })
        .into_owned();
    if TAGS.iter().any(|t| t.is_match(&s)) {
        return quoted(value, ctx);
    }
    if ctx.implicit_key {
        s
    } else {
        fold(&s, ctx.indent, Fold::Flow, ctx.indent_at_start, None)
    }
}

fn scalar(v: &Value, ctx: &Ctx) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(_) => to_js_string(v),
        Value::String(s) => {
            static CTRL: LazyLock<Regex> = LazyLock::new(|| re(r"[\x00-\x08\x0b-\x1f\x7f-\x9f]"));
            if CTRL.is_match(s) {
                double_quoted(s, ctx)
            } else {
                plain(s, ctx)
            }
        }
        _ => String::new(),
    }
}

fn is_collection(v: &Value) -> bool {
    matches!(v, Value::Object(o) if !o.is_empty()) || matches!(v, Value::Array(a) if !a.is_empty())
}

/// A block collection at `indent` (its first line carries no indentation).
fn collection(v: &Value, indent: &str) -> String {
    let lines: Vec<String> = match v {
        Value::Object(o) => o.iter().map(|(k, val)| pair(k, val, indent)).collect(),
        Value::Array(a) => {
            let item_indent = format!("{indent}  ");
            a.iter()
                .map(|item| {
                    let s = if is_collection(item) {
                        collection(item, &item_indent)
                    } else {
                        empty_or_scalar(
                            item,
                            &Ctx {
                                indent: &item_indent,
                                implicit_key: false,
                                indent_at_start: None,
                            },
                        )
                    };
                    format!("- {s}")
                })
                .collect()
        }
        _ => vec![],
    };
    let mut out = String::new();
    for (i, l) in lines.iter().enumerate() {
        if i > 0 {
            out.push('\n');
            out.push_str(indent);
        }
        out.push_str(l);
    }
    out
}

fn empty_or_scalar(v: &Value, ctx: &Ctx) -> String {
    match v {
        Value::Object(_) => "{}".into(),
        Value::Array(_) => "[]".into(),
        other => scalar(other, ctx),
    }
}

fn pair(key: &str, value: &Value, indent: &str) -> String {
    let inner = format!("{indent}  ");
    let k = plain(
        key,
        &Ctx {
            indent: &inner,
            implicit_key: true,
            indent_at_start: None,
        },
    );
    let head = format!("{k}:");
    if is_collection(value) {
        return format!("{head}\n{inner}{}", collection(value, &inner));
    }
    let v = empty_or_scalar(
        value,
        &Ctx {
            indent: &inner,
            implicit_key: false,
            indent_at_start: Some(head.len() + 1),
        },
    );
    // block scalars start on the key line; others after a space
    format!("{head} {v}")
}

/// `yaml.stringify(doc)`.
pub fn stringify_doc(v: &Value) -> String {
    format!("{}\n", collection(v, ""))
}

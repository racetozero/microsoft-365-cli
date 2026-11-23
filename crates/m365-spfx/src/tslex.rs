//! A token scanner for the TypeScript-based upgrade rules (`FN016*`,
//! `FN025*`). Those rules only look at identifiers, string literals,
//! import declarations, `new X(...)` and `a.b.c(...)` call shapes, so
//! tokens with their source offsets are enough; positions follow
//! `getLineAndCharacterOfPosition(node.getStart())` (UTF-16 characters).

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Ident,
    /// A string literal (the token text includes the quotes).
    Str,
    Num,
    Template,
    Regex,
    Punct,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    /// Byte range in the source.
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn text<'a>(&self, src: &'a str) -> &'a str {
        &src[self.start..self.end]
    }

    /// The node's `.text`: identifier name or literal value.
    pub fn value(&self, src: &str) -> String {
        let t = self.text(src);
        match self.kind {
            Kind::Str => unescape(&t[1..t.len().saturating_sub(1).max(1)]),
            _ => t.to_string(),
        }
    }
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match it.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some(o) => out.push(o),
            None => {}
        }
    }
    out
}

fn ident_start(c: char) -> bool {
    c == '_' || c == '$' || c.is_alphabetic()
}

fn ident_part(c: char) -> bool {
    ident_start(c) || c.is_ascii_digit() || c.is_alphanumeric()
}

/// Tokenizes `src`; unterminated literals end at the line end, like the
/// TypeScript scanner's error recovery.
pub fn tokenize(src: &str) -> Vec<Token> {
    let b: Vec<(usize, char)> = src.char_indices().collect();
    let at = |i: usize| b.get(i).map(|x| x.1);
    let off = |i: usize| b.get(i).map_or(src.len(), |x| x.0);
    let mut out: Vec<Token> = Vec::new();
    let mut i = 0;
    // template nesting: brace depth at each `${`
    let mut templates: Vec<usize> = Vec::new();
    let mut depth = 0usize;
    while i < b.len() {
        let c = b[i].1;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '/' && at(i + 1) == Some('/') {
            while i < b.len() && !matches!(b[i].1, '\n' | '\r' | '\u{2028}' | '\u{2029}') {
                i += 1;
            }
            continue;
        }
        if c == '/' && at(i + 1) == Some('*') {
            i += 2;
            while i < b.len() && !(b[i].1 == '*' && at(i + 1) == Some('/')) {
                i += 1;
            }
            i = (i + 2).min(b.len());
            continue;
        }
        let start = i;
        let push = |out: &mut Vec<Token>, kind: Kind, s: usize, e: usize| {
            out.push(Token {
                kind,
                start: off(s),
                end: off(e),
            })
        };
        if ident_start(c) || c == '#' {
            i += 1;
            while at(i).is_some_and(ident_part) {
                i += 1;
            }
            push(&mut out, Kind::Ident, start, i);
            continue;
        }
        if c.is_ascii_digit() || (c == '.' && at(i + 1).is_some_and(|d| d.is_ascii_digit())) {
            i += 1;
            while at(i).is_some_and(|d| d.is_ascii_alphanumeric() || d == '.' || d == '_') {
                i += 1;
            }
            push(&mut out, Kind::Num, start, i);
            continue;
        }
        if c == '"' || c == '\'' {
            i += 1;
            while let Some(d) = at(i) {
                if d == '\\' {
                    i += 2;
                    continue;
                }
                if d == c {
                    i += 1;
                    break;
                }
                if d == '\n' || d == '\r' {
                    break;
                }
                i += 1;
            }
            push(&mut out, Kind::Str, start, i.min(b.len()));
            continue;
        }
        if c == '`' || (c == '}' && templates.last() == Some(&depth)) {
            if c == '}' {
                templates.pop();
            }
            i += 1;
            while let Some(d) = at(i) {
                if d == '\\' {
                    i += 2;
                    continue;
                }
                if d == '`' {
                    i += 1;
                    break;
                }
                if d == '$' && at(i + 1) == Some('{') {
                    i += 2;
                    templates.push(depth);
                    break;
                }
                i += 1;
            }
            push(&mut out, Kind::Template, start, i.min(b.len()));
            continue;
        }
        if c == '/' && regex_allowed(out.last(), src) {
            i += 1;
            let mut class = false;
            while let Some(d) = at(i) {
                match d {
                    '\\' => i += 1,
                    '[' => class = true,
                    ']' => class = false,
                    '/' if !class => {
                        i += 1;
                        break;
                    }
                    '\n' | '\r' => break,
                    _ => {}
                }
                i += 1;
            }
            while at(i).is_some_and(|d| d.is_ascii_alphabetic()) {
                i += 1;
            }
            push(&mut out, Kind::Regex, start, i.min(b.len()));
            continue;
        }
        match c {
            '{' => depth += 1,
            '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        i += 1;
        push(&mut out, Kind::Punct, start, i);
    }
    out
}

fn regex_allowed(prev: Option<&Token>, src: &str) -> bool {
    match prev {
        None => true,
        Some(t) => match t.kind {
            Kind::Ident => matches!(
                t.text(src),
                "return"
                    | "typeof"
                    | "case"
                    | "do"
                    | "else"
                    | "in"
                    | "of"
                    | "new"
                    | "delete"
                    | "void"
                    | "throw"
                    | "yield"
                    | "await"
            ),
            Kind::Punct => !matches!(t.text(src), ")" | "]" | "}"),
            _ => false,
        },
    }
}

/// `getLineAndCharacterOfPosition(offset)` as 1-based `(line, character)`.
pub fn line_char(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut line_start = 0;
    let mut it = src[..offset].char_indices().peekable();
    while let Some((i, c)) = it.next() {
        match c {
            '\r' => {
                if let Some(&(_, '\n')) = it.peek() {
                    it.next();
                    line_start = i + 2;
                } else {
                    line_start = i + 1;
                }
                line += 1;
            }
            '\n' | '\u{2028}' | '\u{2029}' => {
                line += 1;
                line_start = i + c.len_utf8();
            }
            _ => {}
        }
    }
    (line, src[line_start..offset].encode_utf16().count() + 1)
}

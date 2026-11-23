//! The json-to-ast 2.1 tree that upgrade rules use for finding positions
//! (`loc.start.line`/`loc.start.column`, both 1-based, columns counted in
//! UTF-16 code units), and `JsonRule.getAstNodeForProperty`.

use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub enum Node {
    Object {
        children: Vec<Node>,
        start: Pos,
    },
    Array {
        children: Vec<Node>,
        start: Pos,
    },
    Property {
        key: String,
        value: Box<Node>,
        start: Pos,
    },
    Literal {
        value: Value,
        start: Pos,
    },
}

impl Node {
    pub fn start(&self) -> Pos {
        match self {
            Node::Object { start, .. }
            | Node::Array { start, .. }
            | Node::Property { start, .. }
            | Node::Literal { start, .. } => *start,
        }
    }

    fn children(&self) -> Option<&[Node]> {
        match self {
            Node::Object { children, .. } | Node::Array { children, .. } => Some(children),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tok {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    Str(String),
    Num(String),
    True,
    False,
    Null,
}

struct Token {
    tok: Tok,
    start: Pos,
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = input.chars().collect();
    let (mut i, mut line, mut column) = (0usize, 1usize, 1usize);
    let mut out = Vec::new();
    let w = |c: char| c.len_utf16();
    while i < chars.len() {
        let c = chars[i];
        match c {
            '\r' => {
                i += 1;
                line += 1;
                column = 1;
                if chars.get(i) == Some(&'\n') {
                    i += 1;
                }
                continue;
            }
            '\n' => {
                i += 1;
                line += 1;
                column = 1;
                continue;
            }
            ' ' | '\t' => {
                i += 1;
                column += 1;
                continue;
            }
            _ => {}
        }
        let start = Pos { line, column };
        let punct = match c {
            '{' => Some(Tok::LBrace),
            '}' => Some(Tok::RBrace),
            '[' => Some(Tok::LBracket),
            ']' => Some(Tok::RBracket),
            ':' => Some(Tok::Colon),
            ',' => Some(Tok::Comma),
            _ => None,
        };
        if let Some(t) = punct {
            out.push(Token { tok: t, start });
            i += 1;
            column += 1;
            continue;
        }
        let rest: String = chars[i..chars.len().min(i + 5)].iter().collect();
        let kw = [
            ("true", Tok::True),
            ("false", Tok::False),
            ("null", Tok::Null),
        ]
        .into_iter()
        .find(|(k, _)| rest.starts_with(k));
        if let Some((k, t)) = kw {
            out.push(Token { tok: t, start });
            i += k.len();
            column += k.len();
            continue;
        }
        if c == '"' {
            let mut j = i + 1;
            let mut s = String::new();
            loop {
                let Some(&d) = chars.get(j) else {
                    return Err(format!("Unexpected symbol <\"> at {line}:{column}"));
                };
                match d {
                    '\\' => {
                        let e = *chars.get(j + 1).ok_or("Unexpected end")?;
                        match e {
                            'u' => {
                                let hex: String = chars
                                    .get(j + 2..j + 6)
                                    .ok_or("bad escape")?
                                    .iter()
                                    .collect();
                                let code =
                                    u32::from_str_radix(&hex, 16).map_err(|_| "bad escape")?;
                                // lone surrogates cannot be represented; json-to-ast keeps them
                                s.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                                j += 6;
                            }
                            '"' | '\\' | '/' => {
                                s.push(e);
                                j += 2;
                            }
                            'b' => {
                                s.push('\u{8}');
                                j += 2;
                            }
                            'f' => {
                                s.push('\u{c}');
                                j += 2;
                            }
                            'n' => {
                                s.push('\n');
                                j += 2;
                            }
                            'r' => {
                                s.push('\r');
                                j += 2;
                            }
                            't' => {
                                s.push('\t');
                                j += 2;
                            }
                            _ => return Err(format!("Unexpected symbol <\\> at {line}:{column}")),
                        }
                    }
                    '"' => {
                        j += 1;
                        break;
                    }
                    d => {
                        s.push(d);
                        j += 1;
                    }
                }
            }
            column += chars[i..j].iter().map(|c| w(*c)).sum::<usize>();
            out.push(Token {
                tok: Tok::Str(s),
                start,
            });
            i = j;
            continue;
        }
        if c == '-' || c.is_ascii_digit() {
            let mut j = i;
            if chars[j] == '-' {
                j += 1;
            }
            while chars
                .get(j)
                .is_some_and(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-'))
            {
                j += 1;
            }
            let text: String = chars[i..j].iter().collect();
            column += j - i;
            out.push(Token {
                tok: Tok::Num(text),
                start,
            });
            i = j;
            continue;
        }
        return Err(format!("Unexpected symbol <{c}> at {line}:{column}"));
    }
    Ok(out)
}

struct Parser {
    toks: Vec<Token>,
    i: usize,
}

impl Parser {
    fn value(&mut self) -> Result<Node, String> {
        let t = self.toks.get(self.i).ok_or("Unexpected end of input")?;
        let start = t.start;
        match &t.tok {
            Tok::LBrace => {
                self.i += 1;
                let mut children = Vec::new();
                if self.peek() == Some(&Tok::RBrace) {
                    self.i += 1;
                    return Ok(Node::Object { children, start });
                }
                loop {
                    let k = self.toks.get(self.i).ok_or("Unexpected end of input")?;
                    let (Tok::Str(key), kstart) = (&k.tok, k.start) else {
                        return Err("Unexpected token".into());
                    };
                    let key = key.clone();
                    self.i += 1;
                    if self.peek() != Some(&Tok::Colon) {
                        return Err("Unexpected token".into());
                    }
                    self.i += 1;
                    let value = self.value()?;
                    children.push(Node::Property {
                        key,
                        value: Box::new(value),
                        start: kstart,
                    });
                    match self.peek() {
                        Some(Tok::Comma) => self.i += 1,
                        Some(Tok::RBrace) => {
                            self.i += 1;
                            return Ok(Node::Object { children, start });
                        }
                        _ => return Err("Unexpected token".into()),
                    }
                }
            }
            Tok::LBracket => {
                self.i += 1;
                let mut children = Vec::new();
                if self.peek() == Some(&Tok::RBracket) {
                    self.i += 1;
                    return Ok(Node::Array { children, start });
                }
                loop {
                    children.push(self.value()?);
                    match self.peek() {
                        Some(Tok::Comma) => self.i += 1,
                        Some(Tok::RBracket) => {
                            self.i += 1;
                            return Ok(Node::Array { children, start });
                        }
                        _ => return Err("Unexpected token".into()),
                    }
                }
            }
            tok => {
                let value = match tok {
                    Tok::Str(s) => Value::from(s.as_str()),
                    Tok::Num(n) => m365_core::json::num_value(n.parse::<f64>().unwrap_or(f64::NAN)),
                    Tok::True => Value::Bool(true),
                    Tok::False => Value::Bool(false),
                    Tok::Null => Value::Null,
                    _ => return Err("Unexpected token".into()),
                };
                self.i += 1;
                Ok(Node::Literal { value, start })
            }
        }
    }

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.i).map(|t| &t.tok)
    }
}

/// `parse(source)` from json-to-ast.
pub fn parse(source: &str) -> Result<Node, String> {
    let toks = tokenize(source)?;
    let mut p = Parser { toks, i: 0 };
    let v = p.value()?;
    if p.i < p.toks.len() {
        return Err("Unexpected token".into());
    }
    Ok(v)
}

/// `JsonRule.getAstNodeForProperty`. `Err` mirrors upstream's TypeErrors.
pub fn node_for_property<'a>(
    node: &'a Node,
    json_property: &str,
) -> Result<Option<&'a Node>, String> {
    let Some(children) = node.children().filter(|c| !c.is_empty()) else {
        return Ok(Some(node));
    };
    if json_property.is_empty() {
        return Ok(Some(node));
    }
    let mut chunks: Vec<&str> = json_property.split('.').collect();
    let mut current = chunks[0].replace(";#", ".");
    let mut array_element: Option<String> = None;
    if current.ends_with(']') {
        let pos = current.find('[').map_or(0, |p| p + 1);
        array_element = Some(current[pos..current.len() - 1].to_string());
        current = current[..pos.saturating_sub(1)].to_string();
    }
    for child in children {
        let Node::Property { key, value, .. } = child else {
            return Err("Cannot read properties of undefined (reading 'value')".into());
        };
        if *key != current {
            continue;
        }
        let mut current_node: Option<&Node> = Some(child);
        if let Some(el) = &array_element {
            let elements = value
                .children()
                .ok_or("Cannot read properties of undefined (reading 'length')")?;
            match parse_int(el) {
                None => {
                    if let Some(e) = elements.iter().find(
                        |e| matches!(e, Node::Literal { value: Value::String(s), .. } if s == el),
                    ) {
                        current_node = Some(e);
                    }
                }
                Some(i) => {
                    if i < elements.len() as i64 {
                        current_node = if i < 0 {
                            None
                        } else {
                            elements.get(i as usize)
                        };
                    }
                }
            }
        }
        if chunks.len() == 1 {
            return Ok(current_node);
        }
        chunks.remove(0);
        let next = match (array_element.is_some(), current_node) {
            (true, Some(n)) => n,
            (false, Some(Node::Property { value, .. })) => value,
            (false, Some(n)) => n,
            (true, None) => {
                return Err("Cannot read properties of undefined (reading 'children')".into());
            }
            (false, None) => {
                return Err("Cannot read properties of undefined (reading 'value')".into());
            }
        };
        return node_for_property(next, &chunks.join("."));
    }
    Ok(Some(node))
}

/// JavaScript `parseInt(s)` for the integers the property paths use.
fn parse_int(s: &str) -> Option<i64> {
    let t = s.trim_start();
    let (neg, digits) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let d: String = digits.chars().take_while(|c| c.is_ascii_digit()).collect();
    let n: i64 = d.parse().ok()?;
    Some(if neg { -n } else { n })
}

/// `getPositionFromNode`: `{ line, character }`, 1/1 without a node.
pub fn position(node: Option<&Node>) -> (usize, usize) {
    node.map_or((1, 1), |n| {
        let p = n.start();
        (p.line, p.column)
    })
}

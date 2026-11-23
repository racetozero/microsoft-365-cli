//! JMESPath, ported from `jmespath.js` 0.16 (the library upstream uses), so
//! that `--query` results keep object key order and follow the JavaScript
//! library's comparison and sorting quirks.

use crate::json::{Obj, js_number, keys, num_value, stringify, to_js_string};
use serde_json::Value;
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Eof,
    Unquoted(String),
    Quoted(String),
    Rbracket,
    Rparen,
    Comma,
    Colon,
    Rbrace,
    Number(i64),
    Current,
    Expref,
    Pipe,
    Or,
    And,
    Eq,
    Gt,
    Lt,
    Gte,
    Lte,
    Ne,
    Flatten,
    Star,
    Filter,
    Dot,
    Not,
    Lbrace,
    Lbracket,
    Lparen,
    Literal(Value),
}

impl Tok {
    fn name(&self) -> &'static str {
        match self {
            Tok::Eof => "EOF",
            Tok::Unquoted(_) => "UnquotedIdentifier",
            Tok::Quoted(_) => "QuotedIdentifier",
            Tok::Rbracket => "Rbracket",
            Tok::Rparen => "Rparen",
            Tok::Comma => "Comma",
            Tok::Colon => "Colon",
            Tok::Rbrace => "Rbrace",
            Tok::Number(_) => "Number",
            Tok::Current => "Current",
            Tok::Expref => "Expref",
            Tok::Pipe => "Pipe",
            Tok::Or => "Or",
            Tok::And => "And",
            Tok::Eq => "EQ",
            Tok::Gt => "GT",
            Tok::Lt => "LT",
            Tok::Gte => "GTE",
            Tok::Lte => "LTE",
            Tok::Ne => "NE",
            Tok::Flatten => "Flatten",
            Tok::Star => "Star",
            Tok::Filter => "Filter",
            Tok::Dot => "Dot",
            Tok::Not => "Not",
            Tok::Lbrace => "Lbrace",
            Tok::Lbracket => "Lbracket",
            Tok::Lparen => "Lparen",
            Tok::Literal(_) => "Literal",
        }
    }

    fn value(&self) -> String {
        match self {
            Tok::Unquoted(s) | Tok::Quoted(s) => s.clone(),
            Tok::Number(n) => n.to_string(),
            Tok::Literal(v) => to_js_string(v),
            Tok::Eof => String::new(),
            Tok::Rbracket => "]".into(),
            Tok::Rparen => ")".into(),
            Tok::Comma => ",".into(),
            Tok::Colon => ":".into(),
            Tok::Rbrace => "}".into(),
            Tok::Current => "@".into(),
            Tok::Expref => "&".into(),
            Tok::Pipe => "|".into(),
            Tok::Or => "||".into(),
            Tok::And => "&&".into(),
            Tok::Eq => "==".into(),
            Tok::Gt => ">".into(),
            Tok::Lt => "<".into(),
            Tok::Gte => ">=".into(),
            Tok::Lte => "<=".into(),
            Tok::Ne => "!=".into(),
            Tok::Flatten => "[]".into(),
            Tok::Star => "*".into(),
            Tok::Filter => "[?".into(),
            Tok::Dot => ".".into(),
            Tok::Not => "!".into(),
            Tok::Lbrace => "{".into(),
            Tok::Lbracket => "[".into(),
            Tok::Lparen => "(".into(),
        }
    }

    fn bp(&self) -> u8 {
        match self {
            Tok::Pipe => 1,
            Tok::Or => 2,
            Tok::And => 3,
            Tok::Eq | Tok::Gt | Tok::Lt | Tok::Gte | Tok::Lte | Tok::Ne => 5,
            Tok::Flatten => 9,
            Tok::Star => 20,
            Tok::Filter => 21,
            Tok::Dot => 40,
            Tok::Not => 45,
            Tok::Lbrace => 50,
            Tok::Lbracket => 55,
            Tok::Lparen => 60,
            _ => 0,
        }
    }
}

fn lex(s: &str) -> Result<Vec<Tok>, String> {
    let c: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < c.len() {
        let ch = c[i];
        if ch.is_ascii_alphabetic() || ch == '_' {
            let st = i;
            i += 1;
            while i < c.len() && (c[i].is_ascii_alphanumeric() || c[i] == '_') {
                i += 1;
            }
            out.push(Tok::Unquoted(c[st..i].iter().collect()));
        } else if let Some(t) = match ch {
            '.' => Some(Tok::Dot),
            '*' => Some(Tok::Star),
            ',' => Some(Tok::Comma),
            ':' => Some(Tok::Colon),
            '{' => Some(Tok::Lbrace),
            '}' => Some(Tok::Rbrace),
            ']' => Some(Tok::Rbracket),
            '(' => Some(Tok::Lparen),
            ')' => Some(Tok::Rparen),
            '@' => Some(Tok::Current),
            _ => None,
        } {
            out.push(t);
            i += 1;
        } else if ch == '-' || ch.is_ascii_digit() {
            let st = i;
            i += 1;
            while i < c.len() && c[i].is_ascii_digit() {
                i += 1;
            }
            let t: String = c[st..i].iter().collect();
            out.push(Tok::Number(t.parse().unwrap_or(0)));
        } else if ch == '[' {
            i += 1;
            match c.get(i) {
                Some('?') => {
                    i += 1;
                    out.push(Tok::Filter)
                }
                Some(']') => {
                    i += 1;
                    out.push(Tok::Flatten)
                }
                _ => out.push(Tok::Lbracket),
            }
        } else if ch == '\'' {
            i += 1;
            let st = i;
            while i < c.len() && c[i] != '\'' {
                if c[i] == '\\' && i + 1 < c.len() && c[i + 1] == '\'' {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let lit: String = c[st..i.min(c.len())].iter().collect();
            i += 1;
            out.push(Tok::Literal(Value::String(lit.replace("\\'", "'"))));
        } else if ch == '"' {
            let st = i;
            i += 1;
            while i < c.len() && c[i] != '"' {
                if c[i] == '\\' && i + 1 < c.len() && (c[i + 1] == '\\' || c[i + 1] == '"') {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            i += 1;
            let raw: String = c[st..i.min(c.len())].iter().collect();
            let v: String = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
            out.push(Tok::Quoted(v));
        } else if ch == '`' {
            i += 1;
            let st = i;
            while i < c.len() && c[i] != '`' {
                if c[i] == '\\' && i + 1 < c.len() && c[i + 1] == '`' {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let lit: String = c[st..i.min(c.len())]
                .iter()
                .collect::<String>()
                .trim_start()
                .replace("\\`", "`");
            i += 1;
            let v = match serde_json::from_str::<Value>(&lit) {
                Ok(v) if looks_like_json(&lit) => v,
                _ => serde_json::from_str::<Value>(&format!("\"{lit}\""))
                    .map_err(|e| e.to_string())?,
            };
            out.push(Tok::Literal(v));
        } else if matches!(ch, '<' | '>' | '=' | '!') {
            i += 1;
            let eq = c.get(i) == Some(&'=');
            let t = match (ch, eq) {
                ('!', true) => Tok::Ne,
                ('!', false) => Tok::Not,
                ('<', true) => Tok::Lte,
                ('<', false) => Tok::Lt,
                ('>', true) => Tok::Gte,
                ('>', false) => Tok::Gt,
                ('=', true) => Tok::Eq,
                _ => return Err(format!("Unknown character:{ch}")),
            };
            if eq {
                i += 1;
            }
            out.push(t);
        } else if ch == '&' {
            i += 1;
            if c.get(i) == Some(&'&') {
                i += 1;
                out.push(Tok::And)
            } else {
                out.push(Tok::Expref)
            }
        } else if ch == '|' {
            i += 1;
            if c.get(i) == Some(&'|') {
                i += 1;
                out.push(Tok::Or)
            } else {
                out.push(Tok::Pipe)
            }
        } else if matches!(ch, ' ' | '\t' | '\n') {
            i += 1;
        } else {
            return Err(format!("Unknown character:{ch}"));
        }
    }
    out.push(Tok::Eof);
    Ok(out)
}

fn looks_like_json(s: &str) -> bool {
    let t = s.trim_start();
    matches!(t.chars().next(), Some('[' | '{' | '"' | '-' | '0'..='9'))
        || ["true", "false", "null"].contains(&t.trim_end())
}

#[derive(Debug, Clone)]
enum Node {
    Field(String),
    Subexpression(Box<Node>, Box<Node>),
    IndexExpression(Box<Node>, Box<Node>),
    Index(i64),
    Slice(Option<i64>, Option<i64>, Option<i64>),
    Projection(Box<Node>, Box<Node>),
    ValueProjection(Box<Node>, Box<Node>),
    FilterProjection(Box<Node>, Box<Node>, Box<Node>),
    Flatten(Box<Node>),
    Identity,
    Current,
    MultiSelectList(Vec<Node>),
    MultiSelectHash(Vec<(String, Node)>),
    Or(Box<Node>, Box<Node>),
    And(Box<Node>, Box<Node>),
    Not(Box<Node>),
    Literal(Value),
    Pipe(Box<Node>, Box<Node>),
    Comparator(Tok, Box<Node>, Box<Node>),
    Function(String, Vec<Node>),
    Expref(Box<Node>),
}

struct Parser {
    toks: Vec<Tok>,
    i: usize,
}

type PResult = Result<Node, String>;

impl Parser {
    fn la(&self, n: usize) -> &Tok {
        self.toks.get(self.i + n).unwrap_or(&Tok::Eof)
    }
    fn advance(&mut self) -> Tok {
        let t = self.la(0).clone();
        self.i += 1;
        t
    }
    fn expect(&mut self, name: &str) -> Result<(), String> {
        let t = self.la(0);
        if t.name() == name {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("Expected {name}, got: {}", t.name()))
        }
    }
    fn error_token(&self, t: &Tok) -> String {
        format!("Invalid token ({}): \"{}\"", t.name(), t.value())
    }

    fn expression(&mut self, rbp: u8) -> PResult {
        let t = self.advance();
        let mut left = self.nud(t)?;
        while rbp < self.la(0).bp() {
            let t = self.advance();
            left = self.led(t, left)?;
        }
        Ok(left)
    }

    fn nud(&mut self, t: Tok) -> PResult {
        Ok(match t {
            Tok::Literal(v) => Node::Literal(v),
            Tok::Unquoted(s) => Node::Field(s),
            Tok::Quoted(s) => {
                if *self.la(0) == Tok::Lparen {
                    return Err("Quoted identifier not allowed for function names.".into());
                }
                Node::Field(s)
            }
            Tok::Not => Node::Not(Box::new(self.expression(Tok::Not.bp())?)),
            Tok::Star => {
                let right = if *self.la(0) == Tok::Rbracket {
                    Node::Identity
                } else {
                    self.projection_rhs(Tok::Star.bp())?
                };
                Node::ValueProjection(Box::new(Node::Identity), Box::new(right))
            }
            Tok::Filter => self.led(Tok::Filter, Node::Identity)?,
            Tok::Lbrace => self.multiselect_hash()?,
            Tok::Flatten => {
                let left = Node::Flatten(Box::new(Node::Identity));
                let right = self.projection_rhs(Tok::Flatten.bp())?;
                Node::Projection(Box::new(left), Box::new(right))
            }
            Tok::Lbracket => {
                if matches!(self.la(0), Tok::Number(_) | Tok::Colon) {
                    let right = self.index_expression()?;
                    self.project_if_slice(Node::Identity, right)?
                } else if *self.la(0) == Tok::Star && *self.la(1) == Tok::Rbracket {
                    self.i += 2;
                    let right = self.projection_rhs(Tok::Star.bp())?;
                    Node::Projection(Box::new(Node::Identity), Box::new(right))
                } else {
                    self.multiselect_list()?
                }
            }
            Tok::Current => Node::Current,
            Tok::Expref => Node::Expref(Box::new(self.expression(Tok::Expref.bp())?)),
            Tok::Lparen => {
                let e = self.expression(0)?;
                self.expect("Rparen")?;
                e
            }
            other => return Err(self.error_token(&other)),
        })
    }

    fn led(&mut self, t: Tok, left: Node) -> PResult {
        Ok(match t {
            Tok::Dot => {
                if *self.la(0) != Tok::Star {
                    let right = self.dot_rhs(Tok::Dot.bp())?;
                    Node::Subexpression(Box::new(left), Box::new(right))
                } else {
                    self.advance();
                    let right = self.projection_rhs(Tok::Dot.bp())?;
                    Node::ValueProjection(Box::new(left), Box::new(right))
                }
            }
            Tok::Pipe => Node::Pipe(Box::new(left), Box::new(self.expression(Tok::Pipe.bp())?)),
            Tok::Or => Node::Or(Box::new(left), Box::new(self.expression(Tok::Or.bp())?)),
            Tok::And => Node::And(Box::new(left), Box::new(self.expression(Tok::And.bp())?)),
            Tok::Lparen => {
                let name = match left {
                    Node::Field(n) => n,
                    _ => String::new(),
                };
                let mut args = Vec::new();
                while *self.la(0) != Tok::Rparen {
                    let e = if *self.la(0) == Tok::Current {
                        self.advance();
                        Node::Current
                    } else {
                        self.expression(0)?
                    };
                    if *self.la(0) == Tok::Comma {
                        self.expect("Comma")?;
                    }
                    args.push(e);
                    if *self.la(0) == Tok::Eof {
                        return Err("Expected Rparen, got: EOF".into());
                    }
                }
                self.expect("Rparen")?;
                Node::Function(name, args)
            }
            Tok::Filter => {
                let cond = self.expression(0)?;
                self.expect("Rbracket")?;
                let right = if *self.la(0) == Tok::Flatten {
                    Node::Identity
                } else {
                    self.projection_rhs(Tok::Filter.bp())?
                };
                Node::FilterProjection(Box::new(left), Box::new(right), Box::new(cond))
            }
            Tok::Flatten => {
                let l = Node::Flatten(Box::new(left));
                let right = self.projection_rhs(Tok::Flatten.bp())?;
                Node::Projection(Box::new(l), Box::new(right))
            }
            Tok::Eq | Tok::Ne | Tok::Gt | Tok::Gte | Tok::Lt | Tok::Lte => {
                let right = self.expression(t.bp())?;
                Node::Comparator(t, Box::new(left), Box::new(right))
            }
            Tok::Lbracket => {
                if matches!(self.la(0), Tok::Number(_) | Tok::Colon) {
                    let right = self.index_expression()?;
                    self.project_if_slice(left, right)?
                } else {
                    self.expect("Star")?;
                    self.expect("Rbracket")?;
                    let right = self.projection_rhs(Tok::Star.bp())?;
                    Node::Projection(Box::new(left), Box::new(right))
                }
            }
            other => return Err(self.error_token(&other)),
        })
    }

    fn index_expression(&mut self) -> PResult {
        if *self.la(0) == Tok::Colon || *self.la(1) == Tok::Colon {
            return self.slice_expression();
        }
        let n = match self.advance() {
            Tok::Number(n) => n,
            other => return Err(self.error_token(&other)),
        };
        self.expect("Rbracket")?;
        Ok(Node::Index(n))
    }

    fn project_if_slice(&mut self, left: Node, right: Node) -> PResult {
        let is_slice = matches!(right, Node::Slice(..));
        let ie = Node::IndexExpression(Box::new(left), Box::new(right));
        if is_slice {
            let r = self.projection_rhs(Tok::Star.bp())?;
            Ok(Node::Projection(Box::new(ie), Box::new(r)))
        } else {
            Ok(ie)
        }
    }

    fn slice_expression(&mut self) -> PResult {
        let mut parts: [Option<i64>; 3] = [None, None, None];
        let mut idx = 0;
        while *self.la(0) != Tok::Rbracket && idx < 3 {
            match self.la(0).clone() {
                Tok::Colon => {
                    idx += 1;
                    self.advance();
                }
                Tok::Number(n) => {
                    parts[idx] = Some(n);
                    self.advance();
                }
                other => return Err(self.error_token(&other)),
            }
        }
        if idx > 2 {
            return Err("Syntax error, too many colons".into());
        }
        self.expect("Rbracket")?;
        Ok(Node::Slice(parts[0], parts[1], parts[2]))
    }

    fn dot_rhs(&mut self, rbp: u8) -> PResult {
        match self.la(0) {
            Tok::Unquoted(_) | Tok::Quoted(_) | Tok::Star => self.expression(rbp),
            Tok::Lbracket => {
                self.advance();
                self.multiselect_list()
            }
            Tok::Lbrace => {
                self.advance();
                self.multiselect_hash()
            }
            other => Err(self.error_token(&other.clone())),
        }
    }

    fn projection_rhs(&mut self, rbp: u8) -> PResult {
        if self.la(0).bp() < 10 {
            return Ok(Node::Identity);
        }
        match self.la(0) {
            Tok::Lbracket | Tok::Filter => self.expression(rbp),
            Tok::Dot => {
                self.advance();
                self.dot_rhs(rbp)
            }
            other => Err(self.error_token(&other.clone())),
        }
    }

    fn multiselect_list(&mut self) -> PResult {
        let mut items = Vec::new();
        while *self.la(0) != Tok::Rbracket {
            items.push(self.expression(0)?);
            if *self.la(0) == Tok::Comma {
                self.expect("Comma")?;
                if *self.la(0) == Tok::Rbracket {
                    return Err("Unexpected token Rbracket".into());
                }
            }
            if *self.la(0) == Tok::Eof {
                return Err("Expected Rbracket, got: EOF".into());
            }
        }
        self.expect("Rbracket")?;
        Ok(Node::MultiSelectList(items))
    }

    fn multiselect_hash(&mut self) -> PResult {
        let mut pairs = Vec::new();
        loop {
            let key = match self.advance() {
                Tok::Unquoted(s) | Tok::Quoted(s) => s,
                other => {
                    return Err(format!(
                        "Expecting an identifier token, got: {}",
                        other.name()
                    ));
                }
            };
            self.expect("Colon")?;
            let v = self.expression(0)?;
            pairs.push((key, v));
            if *self.la(0) == Tok::Comma {
                self.expect("Comma")?;
            } else if *self.la(0) == Tok::Rbrace {
                self.expect("Rbrace")?;
                break;
            } else {
                let t = self.la(0).clone();
                return Err(format!("Expected Comma, got: {}", t.name()));
            }
        }
        Ok(Node::MultiSelectHash(pairs))
    }
}

fn parse(expr: &str) -> PResult {
    let toks = lex(expr)?;
    let mut p = Parser { toks, i: 0 };
    let n = p.expression(0)?;
    if *p.la(0) != Tok::Eof {
        let t = p.la(0).clone();
        return Err(format!(
            "Unexpected token type: {}, value: {}",
            t.name(),
            t.value()
        ));
    }
    Ok(n)
}

fn is_false(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::Bool(b) => !b,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        Value::Object(o) => o.is_empty(),
        Value::Number(_) => false,
    }
}

/// JavaScript `ToPrimitive` + `ToNumber` used by `<`, `>`.
fn to_num(v: &Value) -> f64 {
    match v {
        Value::Null => 0.0,
        Value::Bool(b) => *b as i32 as f64,
        Value::Number(n) => n.as_f64().unwrap_or(f64::NAN),
        Value::String(s) => js_number(s),
        Value::Array(_) => js_number(&to_js_string(v)),
        Value::Object(_) => f64::NAN,
    }
}

fn js_compare(a: &Value, b: &Value) -> Option<Ordering> {
    let prim = |v: &Value| -> Value {
        match v {
            Value::Array(_) | Value::Object(_) => Value::String(to_js_string(v)),
            other => other.clone(),
        }
    };
    let (a, b) = (prim(a), prim(b));
    if let (Value::String(x), Value::String(y)) = (&a, &b) {
        return Some(x.encode_utf16().cmp(y.encode_utf16()));
    }
    to_num(&a).partial_cmp(&to_num(&b))
}

fn deep_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| deep_eq(p, q))
        }
        (Value::Object(x), Value::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.get(k).is_some_and(|w| deep_eq(v, w)))
        }
        _ => a == b,
    }
}

fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[derive(Clone)]
enum Arg {
    V(Value),
    E(Node),
}

impl Arg {
    fn v(&self) -> &Value {
        static NULL: Value = Value::Null;
        match self {
            Arg::V(v) => v,
            Arg::E(_) => &NULL,
        }
    }
}

fn visit(node: &Node, value: &Value) -> Result<Value, String> {
    Ok(match node {
        Node::Field(name) => match value {
            Value::Object(o) => o.get(name).cloned().unwrap_or(Value::Null),
            _ => Value::Null,
        },
        Node::Subexpression(a, b) => {
            let r = visit(a, value)?;
            if r.is_null() {
                return Ok(Value::Null);
            }
            visit(b, &r)?
        }
        Node::IndexExpression(a, b) => {
            let l = visit(a, value)?;
            visit(b, &l)?
        }
        Node::Index(i) => match value {
            Value::Array(a) => {
                let idx = if *i < 0 { a.len() as i64 + i } else { *i };
                if idx < 0 {
                    Value::Null
                } else {
                    a.get(idx as usize).cloned().unwrap_or(Value::Null)
                }
            }
            _ => Value::Null,
        },
        Node::Slice(start, stop, step) => match value {
            Value::Array(a) => slice(a, *start, *stop, *step)?,
            _ => Value::Null,
        },
        Node::Projection(a, b) => {
            let base = visit(a, value)?;
            let Value::Array(items) = base else {
                return Ok(Value::Null);
            };
            let mut out = Vec::new();
            for it in &items {
                let c = visit(b, it)?;
                if !c.is_null() {
                    out.push(c);
                }
            }
            Value::Array(out)
        }
        Node::ValueProjection(a, b) => {
            let base = visit(a, value)?;
            let Value::Object(o) = base else {
                return Ok(Value::Null);
            };
            let mut out = Vec::new();
            for k in keys(&o) {
                let c = visit(b, &o[k.as_str()])?;
                if !c.is_null() {
                    out.push(c);
                }
            }
            Value::Array(out)
        }
        Node::FilterProjection(a, b, cond) => {
            let base = visit(a, value)?;
            let Value::Array(items) = base else {
                return Ok(Value::Null);
            };
            let mut out = Vec::new();
            for it in &items {
                if !is_false(&visit(cond, it)?) {
                    let c = visit(b, it)?;
                    if !c.is_null() {
                        out.push(c);
                    }
                }
            }
            Value::Array(out)
        }
        Node::Comparator(op, a, b) => {
            let (x, y) = (visit(a, value)?, visit(b, value)?);
            Value::Bool(match op {
                Tok::Eq => deep_eq(&x, &y),
                Tok::Ne => !deep_eq(&x, &y),
                Tok::Gt => js_compare(&x, &y) == Some(Ordering::Greater),
                Tok::Gte => matches!(
                    js_compare(&x, &y),
                    Some(Ordering::Greater | Ordering::Equal)
                ),
                Tok::Lt => js_compare(&x, &y) == Some(Ordering::Less),
                Tok::Lte => matches!(js_compare(&x, &y), Some(Ordering::Less | Ordering::Equal)),
                _ => return Err(format!("Unknown comparator: {}", op.name())),
            })
        }
        Node::Flatten(a) => {
            let base = visit(a, value)?;
            let Value::Array(items) = base else {
                return Ok(Value::Null);
            };
            let mut out = Vec::new();
            for it in items {
                match it {
                    Value::Array(inner) => out.extend(inner),
                    other => out.push(other),
                }
            }
            Value::Array(out)
        }
        Node::Identity | Node::Current => value.clone(),
        Node::MultiSelectList(items) => {
            if value.is_null() {
                return Ok(Value::Null);
            }
            Value::Array(
                items
                    .iter()
                    .map(|n| visit(n, value))
                    .collect::<Result<_, _>>()?,
            )
        }
        Node::MultiSelectHash(pairs) => {
            if value.is_null() {
                return Ok(Value::Null);
            }
            let mut o = Obj::new();
            for (k, n) in pairs {
                o.insert(k.clone(), visit(n, value)?);
            }
            Value::Object(o)
        }
        Node::Or(a, b) => {
            let m = visit(a, value)?;
            if is_false(&m) { visit(b, value)? } else { m }
        }
        Node::And(a, b) => {
            let m = visit(a, value)?;
            if is_false(&m) { m } else { visit(b, value)? }
        }
        Node::Not(a) => Value::Bool(is_false(&visit(a, value)?)),
        Node::Literal(v) => v.clone(),
        Node::Pipe(a, b) => {
            let l = visit(a, value)?;
            visit(b, &l)?
        }
        Node::Function(name, args) => {
            let mut resolved = Vec::with_capacity(args.len());
            for a in args {
                resolved.push(match a {
                    Node::Expref(inner) => Arg::E((**inner).clone()),
                    other => Arg::V(visit(other, value)?),
                });
            }
            call(name, &resolved)?
        }
        Node::Expref(_) => Value::Null,
    })
}

fn slice(
    a: &[Value],
    start: Option<i64>,
    stop: Option<i64>,
    step: Option<i64>,
) -> Result<Value, String> {
    let len = a.len() as i64;
    let step = step.unwrap_or(1);
    if step == 0 {
        return Err("Invalid slice, step cannot be 0".into());
    }
    let cap = |v: i64| -> i64 {
        if v < 0 {
            let v = v + len;
            if v < 0 {
                if step < 0 { -1 } else { 0 }
            } else {
                v
            }
        } else if v >= len {
            if step < 0 { len - 1 } else { len }
        } else {
            v
        }
    };
    let start = match start {
        None => {
            if step < 0 {
                len - 1
            } else {
                0
            }
        }
        Some(s) => cap(s),
    };
    let stop = match stop {
        None => {
            if step < 0 {
                -1
            } else {
                len
            }
        }
        Some(s) => cap(s),
    };
    let mut out = Vec::new();
    let mut i = start;
    if step > 0 {
        while i < stop {
            out.push(a[i as usize].clone());
            i += step;
        }
    } else {
        while i > stop {
            out.push(a[i as usize].clone());
            i += step;
        }
    }
    Ok(Value::Array(out))
}

#[derive(Clone, Copy, PartialEq)]
enum T {
    Number,
    Any,
    String,
    Array,
    Object,
    Expref,
    ArrayNumber,
    ArrayString,
}

impl T {
    fn name(self) -> &'static str {
        match self {
            T::Number => "number",
            T::Any => "any",
            T::String => "string",
            T::Array => "array",
            T::Object => "object",
            T::Expref => "expref",
            T::ArrayNumber => "Array<number>",
            T::ArrayString => "Array<string>",
        }
    }
}

fn arg_type(a: &Arg) -> &'static str {
    match a {
        Arg::E(_) => "expref",
        Arg::V(v) => type_name(v),
    }
}

fn type_matches(t: T, a: &Arg) -> bool {
    match (t, a) {
        (T::Any, _) => true,
        (T::Expref, Arg::E(_)) => true,
        (_, Arg::E(_)) => false,
        (T::Number, Arg::V(v)) => v.is_number(),
        (T::String, Arg::V(v)) => v.is_string(),
        (T::Array, Arg::V(v)) => v.is_array(),
        (T::Object, Arg::V(v)) => v.is_object(),
        (T::ArrayNumber, Arg::V(Value::Array(a))) => a.iter().all(Value::is_number),
        (T::ArrayString, Arg::V(Value::Array(a))) => a.iter().all(Value::is_string),
        _ => false,
    }
}

fn signature(name: &str) -> Option<(Vec<Vec<T>>, bool)> {
    use T::*;
    Some(match name {
        "abs" | "ceil" | "floor" => (vec![vec![Number]], false),
        "avg" | "sum" => (vec![vec![ArrayNumber]], false),
        "contains" => (vec![vec![String, Array], vec![Any]], false),
        "ends_with" | "starts_with" => (vec![vec![String], vec![String]], false),
        "length" => (vec![vec![String, Array, Object]], false),
        "map" => (vec![vec![Expref], vec![Array]], false),
        "max" | "min" | "sort" => (vec![vec![ArrayNumber, ArrayString]], false),
        "merge" => (vec![vec![Object]], true),
        "max_by" | "min_by" | "sort_by" => (vec![vec![Array], vec![Expref]], false),
        "type" | "to_array" | "to_string" | "to_number" => (vec![vec![Any]], false),
        "keys" | "values" => (vec![vec![Object]], false),
        "join" => (vec![vec![String], vec![ArrayString]], false),
        "reverse" => (vec![vec![String, Array]], false),
        "not_null" => (vec![vec![Any]], true),
        _ => return None,
    })
}

fn call(name: &str, args: &[Arg]) -> Result<Value, String> {
    let Some((sig, variadic)) = signature(name) else {
        return Err(format!("Unknown function: {name}()"));
    };
    if variadic {
        if args.len() < sig.len() {
            let p = if sig.len() == 1 {
                " argument"
            } else {
                " arguments"
            };
            return Err(format!(
                "ArgumentError: {name}() takes at least {}{p} but received {}",
                sig.len(),
                args.len()
            ));
        }
    } else if args.len() != sig.len() {
        let p = if sig.len() == 1 {
            " argument"
        } else {
            " arguments"
        };
        return Err(format!(
            "ArgumentError: {name}() takes {}{p} but received {}",
            sig.len(),
            args.len()
        ));
    }
    for (i, a) in args.iter().enumerate() {
        let types = &sig[i.min(sig.len() - 1)];
        if !types.iter().any(|t| type_matches(*t, a)) {
            let expected: Vec<&str> = types.iter().map(|t| t.name()).collect();
            return Err(format!(
                "TypeError: {name}() expected argument {} to be type {} but received type {} instead.",
                i + 1,
                expected.join(","),
                arg_type(a)
            ));
        }
    }
    let f = |i: usize| args[i].v().as_f64().unwrap_or(0.0);
    let expr = |i: usize| -> Node {
        match &args[i] {
            Arg::E(n) => n.clone(),
            _ => Node::Identity,
        }
    };
    Ok(match name {
        "abs" => num_value(f(0).abs()),
        "ceil" => num_value(f(0).ceil()),
        "floor" => num_value(f(0).floor()),
        "avg" => {
            let a = args[0].v().as_array().unwrap();
            let s: f64 = a.iter().map(|x| x.as_f64().unwrap_or(0.0)).sum();
            num_value(s / a.len() as f64)
        }
        "sum" => num_value(
            args[0]
                .v()
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_f64().unwrap_or(0.0))
                .sum(),
        ),
        "contains" => Value::Bool(match args[0].v() {
            Value::String(s) => s.contains(&to_js_string(args[1].v())),
            Value::Array(a) => a.iter().any(|x| strict_eq(x, args[1].v())),
            _ => false,
        }),
        "ends_with" => Value::Bool(
            args[0]
                .v()
                .as_str()
                .unwrap()
                .ends_with(args[1].v().as_str().unwrap()),
        ),
        "starts_with" => Value::Bool(
            args[0]
                .v()
                .as_str()
                .unwrap()
                .starts_with(args[1].v().as_str().unwrap()),
        ),
        "length" => Value::from(match args[0].v() {
            Value::String(s) => s.encode_utf16().count(),
            Value::Array(a) => a.len(),
            Value::Object(o) => o.len(),
            _ => 0,
        }),
        "map" => {
            let e = expr(0);
            let a = args[1].v().as_array().unwrap();
            Value::Array(a.iter().map(|x| visit(&e, x)).collect::<Result<_, _>>()?)
        }
        "max" | "min" => {
            let a = args[0].v().as_array().unwrap();
            if a.is_empty() {
                return Ok(Value::Null);
            }
            let want = if name == "max" {
                Ordering::Greater
            } else {
                Ordering::Less
            };
            let mut best = a[0].clone();
            for x in &a[1..] {
                if js_compare(x, &best) == Some(want) {
                    best = x.clone();
                }
            }
            best
        }
        "merge" => {
            let mut o = Obj::new();
            for a in args {
                if let Value::Object(m) = a.v() {
                    for (k, v) in m {
                        o.insert(k.clone(), v.clone());
                    }
                }
            }
            Value::Object(o)
        }
        "max_by" | "min_by" => {
            let e = expr(1);
            let a = args[0].v().as_array().unwrap();
            let want = if name == "max_by" {
                Ordering::Greater
            } else {
                Ordering::Less
            };
            let mut best: Option<(Value, Value)> = None;
            for x in a {
                let k = visit(&e, x)?;
                if !(k.is_number() || k.is_string()) {
                    return Err(format!(
                        "TypeError: expected one of [number, string], received {}",
                        type_name(&k)
                    ));
                }
                match &best {
                    None => best = Some((k, x.clone())),
                    Some((bk, _)) if js_compare(&k, bk) == Some(want) => {
                        best = Some((k, x.clone()))
                    }
                    _ => {}
                }
            }
            best.map(|(_, v)| v).unwrap_or(Value::Null)
        }
        "type" => Value::from(arg_type(&args[0])),
        "keys" => Value::Array(
            keys(args[0].v().as_object().unwrap())
                .into_iter()
                .map(|k| Value::from(k.as_str()))
                .collect(),
        ),
        "values" => {
            let o = args[0].v().as_object().unwrap();
            Value::Array(keys(o).into_iter().map(|k| o[k.as_str()].clone()).collect())
        }
        "sort" => {
            let mut a = args[0].v().as_array().unwrap().clone();
            a.sort_by(|x, y| {
                to_js_string(x)
                    .encode_utf16()
                    .cmp(to_js_string(y).encode_utf16())
            });
            Value::Array(a)
        }
        "sort_by" => {
            let e = expr(1);
            let a = args[0].v().as_array().unwrap();
            let mut decorated: Vec<(usize, Value, Value)> = Vec::with_capacity(a.len());
            let mut ty: Option<&'static str> = None;
            for (i, x) in a.iter().enumerate() {
                let k = visit(&e, x)?;
                let t = type_name(&k);
                if !(t == "number" || t == "string") {
                    return Err("TypeError".into());
                }
                if let Some(prev) = ty
                    && prev != t
                {
                    return Err("TypeError".into());
                }
                ty = Some(t);
                decorated.push((i, k, x.clone()));
            }
            decorated.sort_by(|a, b| match js_compare(&a.1, &b.1) {
                Some(Ordering::Equal) | None => a.0.cmp(&b.0),
                Some(o) => o,
            });
            Value::Array(decorated.into_iter().map(|(_, _, v)| v).collect())
        }
        "join" => {
            let glue = args[0].v().as_str().unwrap();
            let a = args[1].v().as_array().unwrap();
            Value::from(a.iter().map(to_js_string).collect::<Vec<_>>().join(glue))
        }
        "reverse" => match args[0].v() {
            Value::String(s) => {
                let mut u: Vec<u16> = s.encode_utf16().collect();
                u.reverse();
                Value::from(String::from_utf16_lossy(&u))
            }
            Value::Array(a) => Value::Array(a.iter().rev().cloned().collect()),
            _ => Value::Null,
        },
        "to_array" => match args[0].v() {
            Value::Array(_) => args[0].v().clone(),
            other => Value::Array(vec![other.clone()]),
        },
        "to_string" => match args[0].v() {
            Value::String(_) => args[0].v().clone(),
            other => Value::from(stringify(other)),
        },
        "to_number" => match args[0].v() {
            Value::Number(_) => args[0].v().clone(),
            Value::String(s) => {
                let n = js_number(s);
                if n.is_nan() {
                    Value::Null
                } else {
                    num_value(n)
                }
            }
            _ => Value::Null,
        },
        "not_null" => args
            .iter()
            .map(|a| a.v())
            .find(|v| !v.is_null())
            .cloned()
            .unwrap_or(Value::Null),
        _ => Value::Null,
    })
}

fn strict_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => x.as_f64() == y.as_f64(),
        (Value::Array(_), _) | (Value::Object(_), _) => false,
        _ => a == b,
    }
}

/// `jmespath.search(data, expression)`.
pub fn search(data: &Value, expression: &str) -> Result<Value, String> {
    let node = parse(expression)?;
    visit(&node, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn queries() {
        let d = json!([{"displayName": "b", "id": 2}, {"displayName": "a", "id": 1}]);
        assert_eq!(
            search(&d, "[].{Name: displayName, Id: id}").unwrap(),
            json!([{"Name":"b","Id":2},{"Name":"a","Id":1}])
        );
        assert_eq!(
            stringify(&search(&d, "[].{Name: displayName, Id: id}").unwrap()),
            r#"[{"Name":"b","Id":2},{"Name":"a","Id":1}]"#
        );
        assert_eq!(search(&d, "[?id > `1`].displayName").unwrap(), json!(["b"]));
        assert_eq!(
            search(&d, "sort_by(@, &displayName)[0].id").unwrap(),
            json!(1)
        );
        assert_eq!(search(&d, "length(@)").unwrap(), json!(2));
        assert_eq!(search(&d, "[0:1].id").unwrap(), json!([2]));
        assert_eq!(
            search(&d, "[?contains(displayName, 'a')] | [0].id").unwrap(),
            json!(1)
        );
        assert!(search(&d, "[?").is_err());
    }
}

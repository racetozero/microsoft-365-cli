//! A small XML DOM: the parts of `@xmldom/xmldom` the CLI uses to edit
//! SharePoint's `client_LocationBasedDefaults.html` (parse, find elements by
//! tag name, edit attributes and text, serialize with `toString()`).

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Element(Element),
    Text(String),
    /// Comments, processing instructions and doctypes, kept verbatim.
    Raw(String),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
    pub fn set_attr(&mut self, name: &str, value: &str) {
        match self.attrs.iter_mut().find(|(k, _)| k == name) {
            Some(a) => a.1 = value.into(),
            None => self.attrs.push((name.into(), value.into())),
        }
    }
    /// `textContent` of the element.
    pub fn text(&self) -> String {
        let mut s = String::new();
        for c in &self.children {
            match c {
                Node::Text(t) => s.push_str(t),
                Node::Element(e) => s.push_str(&e.text()),
                Node::Raw(_) => {}
            }
        }
        s
    }
    /// `textContent = value`.
    pub fn set_text(&mut self, value: &str) {
        self.children = if value.is_empty() {
            Vec::new()
        } else {
            vec![Node::Text(value.into())]
        };
    }
    /// Paths (child indexes) of the descendants named `tag`, in document order.
    pub fn find_all(&self, tag: &str) -> Vec<Vec<usize>> {
        let mut out = Vec::new();
        fn walk(e: &Element, tag: &str, path: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
            for (i, c) in e.children.iter().enumerate() {
                if let Node::Element(ch) = c {
                    path.push(i);
                    if ch.name == tag {
                        out.push(path.clone());
                    }
                    walk(ch, tag, path, out);
                    path.pop();
                }
            }
        }
        walk(self, tag, &mut Vec::new(), &mut out);
        out
    }
    pub fn at(&self, path: &[usize]) -> Option<&Element> {
        let mut cur = self;
        for &i in path {
            match cur.children.get(i) {
                Some(Node::Element(e)) => cur = e,
                _ => return None,
            }
        }
        Some(cur)
    }
    pub fn at_mut(&mut self, path: &[usize]) -> Option<&mut Element> {
        let mut cur = self;
        for &i in path {
            match cur.children.get_mut(i) {
                Some(Node::Element(e)) => cur = e,
                _ => return None,
            }
        }
        Some(cur)
    }
    /// Removes the node at `path` from its parent.
    pub fn remove_at(&mut self, path: &[usize]) {
        if let Some((last, parent)) = path.split_last()
            && let Some(p) = self.at_mut(parent)
            && *last < p.children.len()
        {
            p.children.remove(*last);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Document {
    /// Nodes before and after the root element.
    pub prolog: Vec<Node>,
    pub root: Option<Element>,
    pub epilog: Vec<Node>,
}

fn decode(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let Some(end) = rest.find(';') else {
            out.push_str(rest);
            return out;
        };
        let ent = &rest[1..end];
        let ch = match ent {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ if ent.starts_with("#x") || ent.starts_with("#X") => {
                u32::from_str_radix(&ent[2..], 16)
                    .ok()
                    .and_then(char::from_u32)
            }
            _ if ent.starts_with('#') => ent[1..].parse().ok().and_then(char::from_u32),
            _ => None,
        };
        match ch {
            Some(c) => out.push(c),
            None => out.push_str(&rest[..=end]),
        }
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    out
}

struct P<'a> {
    s: &'a str,
    i: usize,
}

impl P<'_> {
    fn rest(&self) -> &str {
        &self.s[self.i..]
    }
    fn ws(&mut self) {
        while self.rest().starts_with(|c: char| c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
    fn name(&mut self) -> String {
        let start = self.i;
        while let Some(c) = self.rest().chars().next() {
            if c.is_whitespace() || "/>=".contains(c) {
                break;
            }
            self.i += c.len_utf8();
        }
        self.s[start..self.i].to_string()
    }
    fn until(&mut self, pat: &str) -> String {
        match self.rest().find(pat) {
            Some(j) => {
                let t = self.s[self.i..self.i + j].to_string();
                self.i += j + pat.len();
                t
            }
            None => {
                let t = self.rest().to_string();
                self.i = self.s.len();
                t
            }
        }
    }
    fn nodes(&mut self, close: Option<&str>) -> Vec<Node> {
        let mut out = Vec::new();
        while self.i < self.s.len() {
            let r = self.rest();
            if r.starts_with("</") {
                self.i += 2;
                let _ = self.name();
                self.until(">");
                if close.is_some() {
                    return out;
                }
                continue;
            }
            if r.starts_with("<!--") {
                self.i += 4;
                let c = self.until("-->");
                out.push(Node::Raw(format!("<!--{c}-->")));
            } else if r.starts_with("<![CDATA[") {
                self.i += 9;
                out.push(Node::Text(self.until("]]>")));
            } else if r.starts_with("<?") || r.starts_with("<!") {
                let (lead, open) = if r.starts_with("<?") {
                    ("<?", "?>")
                } else {
                    ("<!", ">")
                };
                self.i += 2;
                let c = self.until(open);
                out.push(Node::Raw(format!("{lead}{c}{open}")));
            } else if r.starts_with('<') {
                self.i += 1;
                let mut e = Element::new(&self.name());
                loop {
                    self.ws();
                    let r = self.rest();
                    if r.starts_with("/>") {
                        self.i += 2;
                        break;
                    }
                    if r.starts_with('>') || r.is_empty() {
                        self.i += 1.min(r.len());
                        let name = e.name.clone();
                        e.children = self.nodes(Some(&name));
                        break;
                    }
                    let k = self.name();
                    self.ws();
                    let mut v = String::new();
                    if self.rest().starts_with('=') {
                        self.i += 1;
                        self.ws();
                        let q = self.rest().chars().next().unwrap_or('"');
                        if q == '"' || q == '\'' {
                            self.i += 1;
                            v = decode(&self.until(&q.to_string()));
                        } else {
                            v = self.name();
                        }
                    }
                    if k.is_empty() {
                        self.i += 1;
                        continue;
                    }
                    e.attrs.push((k, v));
                }
                out.push(Node::Element(e));
            } else {
                let j = r.find('<').unwrap_or(r.len());
                out.push(Node::Text(decode(&r[..j])));
                self.i += j;
            }
        }
        out
    }
}

/// `new DOMParser().parseFromString(xml, 'application/xml')`.
pub fn parse(xml: &str) -> Document {
    let nodes = P { s: xml, i: 0 }.nodes(None);
    let mut doc = Document::default();
    for n in nodes {
        match n {
            Node::Element(e) if doc.root.is_none() => doc.root = Some(e),
            other if doc.root.is_none() => doc.prolog.push(other),
            other => doc.epilog.push(other),
        }
    }
    doc
}

fn esc_text(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            c => out.push(c),
        }
    }
}

fn esc_attr(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
}

fn write(n: &Node, out: &mut String) {
    match n {
        Node::Text(t) => esc_text(t, out),
        Node::Raw(r) => out.push_str(r),
        Node::Element(e) => write_el(e, out),
    }
}

fn write_el(e: &Element, out: &mut String) {
    out.push('<');
    out.push_str(&e.name);
    for (k, v) in &e.attrs {
        out.push(' ');
        out.push_str(k);
        out.push_str("=\"");
        esc_attr(v, out);
        out.push('"');
    }
    if e.children.is_empty() {
        out.push_str("/>");
        return;
    }
    out.push('>');
    for c in &e.children {
        write(c, out);
    }
    out.push_str("</");
    out.push_str(&e.name);
    out.push('>');
}

/// `doc.toString()` (xmldom's `XMLSerializer`).
pub fn serialize(doc: &Document) -> String {
    let mut out = String::new();
    for n in &doc.prolog {
        write(n, &mut out);
    }
    if let Some(r) = &doc.root {
        write_el(r, &mut out);
    }
    for n in &doc.epilog {
        write(n, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_location_defaults() {
        let x = r#"<MetadataDefaults><a href="/sites/s/Docs"><DefaultValue FieldName="F">a &amp; b</DefaultValue></a></MetadataDefaults>"#;
        let d = parse(x);
        assert_eq!(serialize(&d), x);
        let r = d.root.as_ref().unwrap();
        let p = r.find_all("DefaultValue");
        assert_eq!(r.at(&p[0]).unwrap().text(), "a & b");
        assert_eq!(
            serialize(&parse("<MetadataDefaults />")),
            "<MetadataDefaults/>"
        );
        // xmldom's output for the same edit
        let mut d = parse("<MetadataDefaults><a href=\"x&quot;y\"></a></MetadataDefaults>");
        let mut n = Element::new("DefaultValue");
        n.set_attr("FieldName", "q\"<&>");
        n.set_text("v<&>\"");
        d.root.as_mut().unwrap().children.push(Node::Element(n));
        assert_eq!(
            serialize(&d),
            "<MetadataDefaults><a href=\"x&quot;y\"/><DefaultValue FieldName=\"q&quot;&lt;&amp;&gt;\">v&lt;&amp;&gt;\"</DefaultValue></MetadataDefaults>"
        );
    }
}

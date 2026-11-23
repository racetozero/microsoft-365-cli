//! A port of upstream's `src/utils/md.ts` (`md2plain`): Markdown to plain
//! text, used where upstream prints converted Markdown in command output
//! (`graph changelog list`).

use crate::term::{self, EOL};
use fancy_regex::{Captures, Regex};
use std::sync::LazyLock;

fn re(p: &str) -> Regex {
    Regex::new(p).unwrap()
}

macro_rules! lazy_re {
    ($n:ident, $p:expr) => {
        static $n: LazyLock<Regex> = LazyLock::new(|| re($p));
    };
}

lazy_re!(DEF_LIST, r"(```\r?\n)\r?\n(```md definition-list\r?\n)");
lazy_re!(TITLE, r"(?m)^#\s+(.*)");
lazy_re!(HEADING, r"(?m)^(#+)\s+(.*)");
lazy_re!(
    ADMONITION,
    r"(?m)^([ \t]*):::(\w+)(?:\[([^\]]+)\])?([\s\S]*?)^\1:::$"
);
lazy_re!(DD, r"(?m)^:\s(.*)");
lazy_re!(LINK, r"(?m)(?!\[1m)(?!\[22m)\[([^\]]+)\]\(([^)]+)\)");
lazy_re!(TAB_ITEM, r#"(?m)<TabItem value="([^"]+)">"#);
lazy_re!(TABS_LINE, r"(?-s).*\n?</?(Tabs|TabItem)>.*\n?");
lazy_re!(FENCE_INNER, r"```(?:\w+)?\s*([\s\S]*?)\s*```");
lazy_re!(CODE_FENCE, r"(?ms)^```.*?(?:\r?\n)(.*?)```(?:\r?\n)");
lazy_re!(LINE, r"(?m)^(.+)$");
lazy_re!(INLINE, r"(?-s)([`]|\*{1,3}|_{1,3}|~{2})(.*?)\1");
lazy_re!(FRONTMATTER, r"(?m)^---[\s\S]*?---");
lazy_re!(IMPORTS, r"(?ms)^import .+;$");

fn replace_all(r: &Regex, s: &str, f: impl Fn(&Captures) -> String) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut pos = 0;
    while pos <= s.len() {
        let Ok(Some(c)) = r.captures_from_pos(s, pos) else {
            break;
        };
        let m = c.get(0).unwrap();
        out.push_str(&s[last..m.start()]);
        out.push_str(&f(&c));
        last = m.end();
        pos = if m.end() == m.start() {
            // advance one character on empty matches
            match s[m.end()..].chars().next() {
                Some(ch) => {
                    out.push(ch);
                    last = m.end() + ch.len_utf8();
                    m.end() + ch.len_utf8()
                }
                None => break,
            }
        } else {
            m.end()
        };
    }
    out.push_str(&s[last.min(s.len())..]);
    out
}

fn g<'a>(c: &'a Captures, i: usize) -> &'a str {
    c.get(i).map(|m| m.as_str()).unwrap_or("")
}

fn js_upper(s: &str) -> String {
    s.to_uppercase()
}

fn include_content(md: &str) -> String {
    let mut md = md.to_string();
    for (tag, loc) in [
        ("<Global />", "cmd/_global.mdx"),
        ("<CLISettings />", "_clisettings.mdx"),
    ] {
        if let Some(pos) = md.find(tag) {
            // the docs pages are not embedded any more; the tags never
            // occur in the text this module converts (Graph changelog items)
            let _ = loc;
            md.replace_range(pos..pos + tag.len(), "");
        }
        md = replace_all(&DEF_LIST, &md, |c| format!("{}{}", g(c, 1), g(c, 2)));
    }
    md
}

fn convert_title(md: &str) -> String {
    replace_all(&TITLE, md, |c| {
        let t = g(c, 1);
        format!(
            "{}{EOL}{}",
            term::bold(&js_upper(t)),
            "=".repeat(t.encode_utf16().count())
        )
    })
}

fn convert_headings(md: &str) -> String {
    replace_all(&HEADING, md, |c| {
        format!("{EOL}{}", term::bold(&js_upper(g(c, 2))))
    })
}

fn convert_admonitions(md: &str) -> String {
    replace_all(&ADMONITION, md, |c| {
        let indent = g(c, 1);
        let label = js_upper(g(c, 2));
        let title = c
            .get(3)
            .map(|m| format!("{EOL}{EOL}{indent}{}", m.as_str()))
            .unwrap_or_default();
        format!("{indent}{label}{title}{EOL}{EOL}{indent}{}", g(c, 4).trim())
    })
}

fn convert_dd(md: &str) -> String {
    replace_all(&DD, md, |c| format!("  {}", g(c, 1)))
}

fn convert_hyperlinks(md: &str) -> String {
    replace_all(&LINK, md, |c| {
        let (label, url) = (g(c, 1), g(c, 2));
        if label == url {
            url.to_string()
        } else if !url.starts_with("http:") && !url.starts_with("https:") {
            label.to_string()
        } else {
            format!("{label} ({url})")
        }
    })
}

fn convert_code_fences(md: &str) -> String {
    replace_all(&CODE_FENCE, md, |c| {
        format!(
            "{}{EOL}",
            replace_all(&LINE, g(c, 1), |l| format!("  {}", g(l, 1)))
        )
    })
}

fn convert_content_tabs(md: &str) -> String {
    let md = replace_all(&TAB_ITEM, md, |c| g(c, 1).to_string());
    let md = replace_all(&TABS_LINE, &md, |_| String::new());
    let md = replace_all(&FENCE_INNER, &md, |c| g(c, 1).to_string());
    js_trim(&md).to_string()
}

fn remove_inline_markup(md: &str) -> String {
    replace_all(&INLINE, md, |c| g(c, 2).to_string())
}

fn remove_too_many_empty_lines(md: &str) -> String {
    let r = re(&format!("({}){{4,}}", regex::escape(EOL)));
    replace_all(&r, md, |_| EOL.repeat(3))
}

fn js_trim(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || c == '\u{feff}')
}

fn remove_frontmatter(md: &str) -> String {
    js_trim(&replace_all(&FRONTMATTER, md, |_| String::new())).to_string()
}

fn remove_imports(md: &str) -> String {
    js_trim(&replace_all(&IMPORTS, md, |_| String::new())).to_string()
}

/// `md.md2plain`.
pub fn md2plain(md: &str) -> String {
    let mut md = include_content(md);
    for f in [
        convert_title,
        convert_headings,
        convert_admonitions,
        convert_dd,
        convert_hyperlinks,
        convert_code_fences,
        convert_content_tabs,
        remove_inline_markup,
        remove_too_many_empty_lines,
        remove_frontmatter,
        remove_imports,
    ] {
        md = f(&md);
    }
    md
}

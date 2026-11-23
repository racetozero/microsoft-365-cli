//! The parts of node-semver 7.8.5 (strict mode) that the upgrade rules use:
//! `new Range`, `Range#test`, `intersects`, `minVersion`, `gt`, `valid`.
//! Regular expressions are built exactly like `internal/re.js`.

use regex::{Captures, Regex};
use std::cmp::Ordering;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq)]
pub enum Ident {
    Num(f64),
    Str(String),
}

impl Ident {
    fn text(&self) -> String {
        match self {
            Ident::Num(n) => m365_core::json::number_to_string(*n),
            Ident::Str(s) => s.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SemVer {
    pub major: f64,
    pub minor: f64,
    pub patch: f64,
    pub prerelease: Vec<Ident>,
    pub version: String,
}

struct Re {
    full: Regex,
    build: Regex,
    hyphen: Regex,
    comparator_trim: Regex,
    tilde_trim: Regex,
    caret_trim: Regex,
    tilde: Regex,
    caret: Regex,
    xrange: Regex,
    star: Regex,
    gte0: Regex,
    comparator: Regex,
}

static RE: LazyLock<Re> = LazyLock::new(|| {
    let ldn = "[a-zA-Z0-9-]";
    let numeric = "0|[1-9]\\d*";
    let numeric_loose = "\\d+";
    let non_numeric = format!("\\d*[a-zA-Z-]{ldn}*");
    let main = format!("({numeric})\\.({numeric})\\.({numeric})");
    let main_loose = format!("({numeric_loose})\\.({numeric_loose})\\.({numeric_loose})");
    let pre_id = format!("(?:{non_numeric}|{numeric})");
    let pre_id_loose = format!("(?:{non_numeric}|{numeric_loose})");
    let pre = format!("(?:-({pre_id}(?:\\.{pre_id})*))");
    let pre_loose = format!("(?:-?({pre_id_loose}(?:\\.{pre_id_loose})*))");
    let build_id = format!("{ldn}+");
    let build = format!("(?:\\+({build_id}(?:\\.{build_id})*))");
    let full_plain = format!("v?{main}{pre}?{build}?");
    let loose_plain = format!("[v=\\s]*{main_loose}{pre_loose}?{build}?");
    let gtlt = "((?:<|>)?=?)";
    let xid = format!("{numeric}|x|X|\\*");
    let xplain = format!("[v=\\s]*({xid})(?:\\.({xid})(?:\\.({xid})(?:{pre})?{build}?)?)?");
    let r = |s: String| Regex::new(&s).expect("semver regex");
    Re {
        full: r(format!("^{full_plain}$")),
        build: r(build.clone()),
        hyphen: r(format!("^\\s*({xplain})\\s+-\\s+({xplain})\\s*$")),
        comparator_trim: r(format!("(\\s*){gtlt}\\s*({loose_plain}|{xplain})")),
        tilde_trim: r("(\\s*)(?:~>?)\\s+".into()),
        caret_trim: r("(\\s*)(?:\\^)\\s+".into()),
        tilde: r(format!("^(?:~>?){xplain}$")),
        caret: r(format!("^(?:\\^){xplain}$")),
        xrange: r(format!("^{gtlt}\\s*{xplain}$")),
        star: r("(<|>)?=?\\s*\\*".into()),
        gte0: r("^\\s*>=\\s*0\\.0\\.0\\s*$".into()),
        comparator: r(format!("^{gtlt}\\s*({full_plain})$|^$")),
    }
});

fn js_num(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(f64::NAN)
}

fn num(n: f64) -> String {
    m365_core::json::number_to_string(n)
}

impl SemVer {
    /// `new SemVer(version)`; `None` where node-semver throws.
    pub fn parse(version: &str) -> Option<SemVer> {
        if version.len() > 256 {
            return None;
        }
        let m = RE.full.captures(version.trim())?;
        let part = |i: usize| js_num(&m[i]);
        let (major, minor, patch) = (part(1), part(2), part(3));
        if [major, minor, patch]
            .iter()
            .any(|p| *p > 9007199254740991.0 || *p < 0.0)
        {
            return None;
        }
        let prerelease = m.get(4).map_or_else(Vec::new, |p| {
            p.as_str()
                .split('.')
                .map(|id| {
                    if !id.is_empty() && id.bytes().all(|b| b.is_ascii_digit()) {
                        let n = js_num(id);
                        if (0.0..9007199254740991.0).contains(&n) {
                            return Ident::Num(n);
                        }
                    }
                    Ident::Str(id.to_string())
                })
                .collect()
        });
        let mut v = SemVer {
            major,
            minor,
            patch,
            prerelease,
            version: String::new(),
        };
        v.format();
        Some(v)
    }

    fn format(&mut self) {
        self.version = format!(
            "{}.{}.{}",
            num(self.major),
            num(self.minor),
            num(self.patch)
        );
        if !self.prerelease.is_empty() {
            self.version.push('-');
            self.version.push_str(
                &self
                    .prerelease
                    .iter()
                    .map(Ident::text)
                    .collect::<Vec<_>>()
                    .join("."),
            );
        }
    }

    pub fn compare(&self, o: &SemVer) -> Ordering {
        if o.version == self.version {
            return Ordering::Equal;
        }
        self.compare_main(o).then_with(|| self.compare_pre(o))
    }

    fn compare_main(&self, o: &SemVer) -> Ordering {
        let c = |a: f64, b: f64| a.partial_cmp(&b).unwrap_or(Ordering::Equal);
        c(self.major, o.major)
            .then(c(self.minor, o.minor))
            .then(c(self.patch, o.patch))
    }

    fn compare_pre(&self, o: &SemVer) -> Ordering {
        match (self.prerelease.is_empty(), o.prerelease.is_empty()) {
            (false, true) => return Ordering::Less,
            (true, false) => return Ordering::Greater,
            (true, true) => return Ordering::Equal,
            _ => {}
        }
        let mut i = 0;
        loop {
            match (self.prerelease.get(i), o.prerelease.get(i)) {
                (None, None) => return Ordering::Equal,
                (Some(_), None) => return Ordering::Greater,
                (None, Some(_)) => return Ordering::Less,
                (Some(a), Some(b)) if a == b => {}
                (Some(a), Some(b)) => return compare_identifiers(a, b),
            }
            i += 1;
        }
    }
}

fn compare_identifiers(a: &Ident, b: &Ident) -> Ordering {
    let as_num = |i: &Ident| match i {
        Ident::Num(n) => Some(*n),
        Ident::Str(s) if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => Some(js_num(s)),
        Ident::Str(_) => None,
    };
    match (as_num(a), as_num(b)) {
        (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => a.text().cmp(&b.text()),
    }
}

/// `gt(a, b)`.
pub fn gt(a: &SemVer, b: &SemVer) -> bool {
    a.compare(b) == Ordering::Greater
}

/// `semver.valid(v)`.
pub fn valid(v: &str) -> Option<String> {
    SemVer::parse(v).map(|s| s.version)
}

#[derive(Clone, Debug)]
pub struct Comparator {
    pub operator: String,
    /// `None` is `Comparator.ANY`.
    pub semver: Option<SemVer>,
    pub value: String,
}

impl Comparator {
    fn new(comp: &str) -> Result<Comparator, String> {
        let comp = comp.split_whitespace().collect::<Vec<_>>().join(" ");
        let m = RE
            .comparator
            .captures(&comp)
            .ok_or_else(|| format!("Invalid comparator: {comp}"))?;
        let mut operator = m.get(1).map_or("", |x| x.as_str()).to_string();
        if operator == "=" {
            operator.clear();
        }
        let semver = match m.get(2).filter(|x| !x.as_str().is_empty()) {
            None => None,
            Some(v) => Some(
                SemVer::parse(v.as_str())
                    .ok_or_else(|| format!("Invalid Version: {}", v.as_str()))?,
            ),
        };
        let value = match &semver {
            None => String::new(),
            Some(s) => format!("{operator}{}", s.version),
        };
        Ok(Comparator {
            operator,
            semver,
            value,
        })
    }

    fn test(&self, v: &SemVer) -> bool {
        match &self.semver {
            None => true,
            Some(s) => cmp(v, &self.operator, s),
        }
    }

    fn intersects(&self, c: &Comparator) -> bool {
        if self.operator.is_empty() {
            if self.value.is_empty() {
                return true;
            }
            return Range::new(&c.value)
                .is_ok_and(|r| SemVer::parse(&self.value).is_some_and(|v| r.test(&v)));
        } else if c.operator.is_empty() {
            if c.value.is_empty() {
                return true;
            }
            return Range::new(&self.value)
                .is_ok_and(|r| c.semver.as_ref().is_none_or(|v| r.test(v)));
        }
        if self.value.starts_with("<0.0.0") || c.value.starts_with("<0.0.0") {
            return false;
        }
        let (a, b) = (&self.operator, &c.operator);
        if a.starts_with('>') && b.starts_with('>') || a.starts_with('<') && b.starts_with('<') {
            return true;
        }
        let (Some(x), Some(y)) = (&self.semver, &c.semver) else {
            return false;
        };
        if x.version == y.version && a.contains('=') && b.contains('=') {
            return true;
        }
        if cmp(x, "<", y) && a.starts_with('>') && b.starts_with('<') {
            return true;
        }
        cmp(x, ">", y) && a.starts_with('<') && b.starts_with('>')
    }
}

fn cmp(a: &SemVer, op: &str, b: &SemVer) -> bool {
    let o = a.compare(b);
    match op {
        "===" => a.version == b.version,
        "!==" => a.version != b.version,
        "" | "=" | "==" => o == Ordering::Equal,
        "!=" => o != Ordering::Equal,
        ">" => o == Ordering::Greater,
        ">=" => o != Ordering::Less,
        "<" => o == Ordering::Less,
        "<=" => o != Ordering::Greater,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct Range {
    pub set: Vec<Vec<Comparator>>,
}

fn is_x(id: Option<&str>) -> bool {
    match id {
        None => true,
        Some(s) => s.is_empty() || s.eq_ignore_ascii_case("x") || s == "*",
    }
}

fn g<'a>(c: &'a Captures, i: usize) -> Option<&'a str> {
    c.get(i).map(|m| m.as_str())
}

/// `String#replace` with a non-global regex: the first match only.
fn replace_first(s: &str, re: &Regex, f: impl Fn(&Captures) -> String) -> String {
    match re.captures(s) {
        None => s.to_string(),
        Some(c) => {
            let m = c.get(0).expect("match");
            format!("{}{}{}", &s[..m.start()], f(&c), &s[m.end()..])
        }
    }
}

fn plus1(s: Option<&str>) -> String {
    num(js_num(s.unwrap_or("")) + 1.0)
}

fn replace_tilde(comp: &str) -> String {
    replace_first(comp, &RE.tilde, |c| {
        let (mj, mi, p, pr) = (g(c, 1), g(c, 2), g(c, 3), g(c, 4));
        let (sm, si, sp) = (mj.unwrap_or(""), mi.unwrap_or(""), p.unwrap_or(""));
        if is_x(mj) {
            String::new()
        } else if is_x(mi) {
            format!(">={sm}.0.0 <{}.0.0-0", plus1(mj))
        } else if is_x(p) {
            format!(">={sm}.{si}.0 <{sm}.{}.0-0", plus1(mi))
        } else if let Some(pr) = pr.filter(|x| !x.is_empty()) {
            format!(">={sm}.{si}.{sp}-{pr} <{sm}.{}.0-0", plus1(mi))
        } else {
            format!(">={sm}.{si}.{sp} <{sm}.{}.0-0", plus1(mi))
        }
    })
}

fn replace_caret(comp: &str) -> String {
    replace_first(comp, &RE.caret, |c| {
        let (mj, mi, p, pr) = (g(c, 1), g(c, 2), g(c, 3), g(c, 4));
        let (sm, si, sp) = (mj.unwrap_or(""), mi.unwrap_or(""), p.unwrap_or(""));
        let pr = pr.filter(|x| !x.is_empty());
        if is_x(mj) {
            String::new()
        } else if is_x(mi) {
            format!(">={sm}.0.0 <{}.0.0-0", plus1(mj))
        } else if is_x(p) {
            if sm == "0" {
                format!(">={sm}.{si}.0 <{sm}.{}.0-0", plus1(mi))
            } else {
                format!(">={sm}.{si}.0 <{}.0.0-0", plus1(mj))
            }
        } else {
            let from = match pr {
                Some(pr) => format!(">={sm}.{si}.{sp}-{pr}"),
                None => format!(">={sm}.{si}.{sp}"),
            };
            let to = if sm == "0" {
                if si == "0" {
                    format!("<{sm}.{si}.{}-0", plus1(p))
                } else {
                    format!("<{sm}.{}.0-0", plus1(mi))
                }
            } else {
                format!("<{}.0.0-0", plus1(mj))
            };
            format!("{from} {to}")
        }
    })
}

fn replace_xrange(comp: &str) -> String {
    let comp = comp.trim();
    replace_first(comp, &RE.xrange, |c| {
        let ret = c[0].to_string();
        let (mut gtlt, mj, mi, p) = (g(c, 1).unwrap_or("").to_string(), g(c, 2), g(c, 3), g(c, 4));
        if (is_x(mj) && !is_x(mi)) || (is_x(mi) && p.is_some_and(|p| !p.is_empty()) && !is_x(p)) {
            return comp.to_string();
        }
        let xm_ = is_x(mj);
        let xm = xm_ || is_x(mi);
        let xp = xm || is_x(p);
        if gtlt == "=" && xp {
            gtlt.clear();
        }
        let mut pr = "";
        if xm_ {
            if gtlt == ">" || gtlt == "<" {
                "<0.0.0-0".into()
            } else {
                "*".into()
            }
        } else if !gtlt.is_empty() && xp {
            let mut big = js_num(mj.unwrap_or(""));
            let mut m = if xm { 0.0 } else { js_num(mi.unwrap_or("")) };
            let mut pp = 0.0;
            if gtlt == ">" {
                gtlt = ">=".into();
                if xm {
                    big += 1.0;
                    m = 0.0;
                    pp = 0.0;
                } else {
                    m += 1.0;
                    pp = 0.0;
                }
            } else if gtlt == "<=" {
                gtlt = "<".into();
                if xm {
                    big += 1.0;
                } else {
                    m += 1.0;
                }
            }
            if gtlt == "<" {
                pr = "-0";
            }
            format!("{gtlt}{}.{}.{}{pr}", num(big), num(m), num(pp))
        } else if xm {
            format!(">={}.0.0{pr} <{}.0.0-0", mj.unwrap_or(""), plus1(mj))
        } else if xp {
            format!(
                ">={}.{}.0{pr} <{}.{}.0-0",
                mj.unwrap_or(""),
                mi.unwrap_or(""),
                mj.unwrap_or(""),
                plus1(mi)
            )
        } else {
            ret
        }
    })
}

fn hyphen_replace(c: &Captures) -> String {
    let (from, fmj, fmi, fp, _fpr) = (g(c, 1).unwrap_or(""), g(c, 2), g(c, 3), g(c, 4), g(c, 5));
    let (to, tmj, tmi, tp, tpr) = (g(c, 7).unwrap_or(""), g(c, 8), g(c, 9), g(c, 10), g(c, 11));
    let s = |x: Option<&str>| x.unwrap_or("").to_string();
    let from = if is_x(fmj) {
        String::new()
    } else if is_x(fmi) {
        format!(">={}.0.0", s(fmj))
    } else if is_x(fp) {
        format!(">={}.{}.0", s(fmj), s(fmi))
    } else {
        // with or without a prerelease (`includePrerelease` is off)
        format!(">={from}")
    };
    let to = if is_x(tmj) {
        String::new()
    } else if is_x(tmi) {
        format!("<{}.0.0-0", plus1(tmj))
    } else if is_x(tp) {
        format!("<{}.{}.0-0", s(tmj), plus1(tmi))
    } else if let Some(tpr) = tpr.filter(|x| !x.is_empty()) {
        format!("<={}.{}.{}-{tpr}", s(tmj), s(tmi), s(tp))
    } else {
        format!("<={to}")
    };
    format!("{from} {to}").trim().to_string()
}

fn split_ws(s: &str) -> Vec<&str> {
    // JavaScript `s.split(/\s+/)` keeps empty edge entries
    let re: &Regex = &WS;
    re.split(s).collect()
}

static WS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").expect("ws"));

fn parse_comparator(comp: &str) -> String {
    let comp = replace_first(comp, &RE.build, |_| String::new());
    let comp = split_ws(comp.trim())
        .into_iter()
        .map(replace_caret)
        .collect::<Vec<_>>()
        .join(" ");
    let comp = split_ws(comp.trim())
        .into_iter()
        .map(replace_tilde)
        .collect::<Vec<_>>()
        .join(" ");
    let comp = split_ws(&comp)
        .into_iter()
        .map(replace_xrange)
        .collect::<Vec<_>>()
        .join(" ");
    replace_first(comp.trim(), &RE.star, |_| String::new())
}

fn parse_range(range: &str) -> Result<Vec<Comparator>, String> {
    let range = RE.build.replace_all(range, "").into_owned();
    let range = replace_first(&range, &RE.hyphen, hyphen_replace);
    let range = RE
        .comparator_trim
        .replace_all(&range, "$1$2$3")
        .into_owned();
    let range = RE.tilde_trim.replace_all(&range, "${1}~").into_owned();
    let range = RE.caret_trim.replace_all(&range, "${1}^").into_owned();
    let joined = range
        .split(' ')
        .map(parse_comparator)
        .collect::<Vec<_>>()
        .join(" ");
    let list: Vec<String> = split_ws(&joined)
        .into_iter()
        .map(|c| replace_first(c.trim(), &RE.gte0, |_| String::new()))
        .collect();
    let mut out: Vec<Comparator> = Vec::new();
    for c in &list {
        let comp = Comparator::new(c)?;
        if comp.value == "<0.0.0-0" {
            return Ok(vec![comp]);
        }
        match out.iter_mut().find(|x| x.value == comp.value) {
            Some(x) => *x = comp,
            None => out.push(comp),
        }
    }
    if out.len() > 1 && out.iter().any(|c| c.value.is_empty()) {
        out.retain(|c| !c.value.is_empty());
    }
    Ok(out)
}

thread_local! {
    /// Parsed ranges by source text (node-semver keeps an LRU of `parseRange`).
    static RANGES: std::cell::RefCell<std::collections::HashMap<String, Result<Range, String>>> = Default::default();
}

impl Range {
    /// `new Range(range)`; `Err` where node-semver throws.
    pub fn new(range: &str) -> Result<Range, String> {
        if let Some(r) = RANGES.with(|c| c.borrow().get(range).cloned()) {
            return r;
        }
        let r = Range::parse(range);
        RANGES.with(|c| c.borrow_mut().insert(range.to_string(), r.clone()));
        r
    }

    fn parse(range: &str) -> Result<Range, String> {
        let raw = WS.replace_all(range.trim(), " ").into_owned();
        let mut set = Vec::new();
        for r in raw.split("||") {
            let c = parse_range(r.trim())?;
            if !c.is_empty() {
                set.push(c);
            }
        }
        if set.is_empty() {
            return Err(format!("Invalid SemVer Range: {raw}"));
        }
        if set.len() > 1 {
            let first = set[0].clone();
            set.retain(|c| c[0].value != "<0.0.0-0");
            if set.is_empty() {
                set = vec![first];
            } else if set.len() > 1
                && let Some(any) = set
                    .iter()
                    .find(|c| c.len() == 1 && c[0].value.is_empty())
                    .cloned()
            {
                set = vec![any];
            }
        }
        Ok(Range { set })
    }

    pub fn test(&self, v: &SemVer) -> bool {
        self.set.iter().any(|s| test_set(s, v))
    }

    fn intersects(&self, o: &Range) -> bool {
        self.set.iter().any(|a| {
            satisfiable(a)
                && o.set
                    .iter()
                    .any(|b| satisfiable(b) && a.iter().all(|x| b.iter().all(|y| x.intersects(y))))
        })
    }
}

fn satisfiable(c: &[Comparator]) -> bool {
    let mut rest = c.to_vec();
    let mut test = rest.pop();
    let mut result = true;
    while result && !rest.is_empty() {
        let t = test.expect("comparator");
        result = rest.iter().all(|o| t.intersects(o));
        test = rest.pop();
    }
    result
}

fn test_set(set: &[Comparator], v: &SemVer) -> bool {
    if !set.iter().all(|c| c.test(v)) {
        return false;
    }
    if !v.prerelease.is_empty() {
        for c in set {
            let Some(s) = &c.semver else { continue };
            if !s.prerelease.is_empty()
                && s.major == v.major
                && s.minor == v.minor
                && s.patch == v.patch
            {
                return true;
            }
        }
        return false;
    }
    true
}

/// `semver.intersects(a, b)`.
pub fn intersects(a: &str, b: &str) -> Result<bool, String> {
    Ok(Range::new(a)?.intersects(&Range::new(b)?))
}

/// `semver.minVersion(range)`.
pub fn min_version(range: &str) -> Result<Option<SemVer>, String> {
    let r = Range::new(range)?;
    for v in ["0.0.0", "0.0.0-0"] {
        let v = SemVer::parse(v).expect("version");
        if r.test(&v) {
            return Ok(Some(v));
        }
    }
    let mut min: Option<SemVer> = None;
    for comps in &r.set {
        let mut set_min: Option<SemVer> = None;
        for c in comps {
            let Some(s) = &c.semver else {
                // ANY: `new SemVer(ANY.version)` throws
                return Err("Invalid Version: undefined".into());
            };
            let mut v = SemVer::parse(&s.version).expect("version");
            match c.operator.as_str() {
                ">" | "" | ">=" => {
                    if c.operator == ">" {
                        if v.prerelease.is_empty() {
                            v.patch += 1.0;
                        } else {
                            v.prerelease.push(Ident::Num(0.0));
                        }
                        v.format();
                    }
                    if set_min.as_ref().is_none_or(|m| gt(&v, m)) {
                        set_min = Some(v);
                    }
                }
                _ => {}
            }
        }
        if let Some(sm) = set_min
            && min.as_ref().is_none_or(|m| gt(m, &sm))
        {
            min = Some(sm);
        }
    }
    Ok(min.filter(|m| r.test(m)))
}

/// `semver.satisfies(version, range)` (false for an invalid range or
/// version).
pub fn satisfies(version: &str, range: &str) -> bool {
    match (Range::new(range), SemVer::parse(version)) {
        (Ok(r), Some(v)) => r.test(&v),
        _ => false,
    }
}

static COERCE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(^|[^\d])(\d{1,16})(?:\.(\d{1,16}))?(?:\.(\d{1,16}))?(?:$|[^\d])").expect("coerce")
});

/// `semver.coerce(version)` for a string.
pub fn coerce(version: &str) -> Option<SemVer> {
    let m = COERCE.captures(version)?;
    let n = |i: usize| m.get(i).map_or("0", |x| x.as_str());
    SemVer::parse(&format!("{}.{}.{}", n(2), n(3), n(4)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_match_node_semver() {
        let mv = |r: &str| min_version(r).ok().flatten().map(|v| v.version);
        assert_eq!(mv("^1.2.3").as_deref(), Some("1.2.3"));
        assert_eq!(mv("~5.8.0").as_deref(), Some("5.8.0"));
        assert_eq!(mv(">=1.0.0").as_deref(), Some("1.0.0"));
        assert_eq!(mv(">1.0.0").as_deref(), Some("1.0.1"));
        assert_eq!(mv("1.x").as_deref(), Some("1.0.0"));
        assert_eq!(mv("*").as_deref(), Some("0.0.0"));
        assert_eq!(mv("1.2.3 - 2.0.0").as_deref(), Some("1.2.3"));
        assert_eq!(mv("<2.0.0").as_deref(), Some("0.0.0"));
        assert!(intersects("^1.2.0", "1.4.x").unwrap());
        assert!(!intersects("^1.2.0", "^2.0.0").unwrap());
        assert!(intersects("~5.8.0", "5.8.3").unwrap());
        assert!(!intersects("1.24.1", "1.22.0").unwrap());
        assert!(intersects(">=1.0.0 <2.0.0", "^1.9.0").unwrap());
        assert!(Range::new("not a range").is_err());
        assert_eq!(valid("1.2.3-beta.1").as_deref(), Some("1.2.3-beta.1"));
        assert_eq!(valid("^1.2.3"), None);
        assert!(gt(
            &SemVer::parse("1.10.0").unwrap(),
            &SemVer::parse("1.9.0").unwrap()
        ));
        assert!(gt(
            &SemVer::parse("1.0.0").unwrap(),
            &SemVer::parse("1.0.0-rc.1").unwrap()
        ));
        let set: Vec<String> = Range::new("^1.2.3").unwrap().set[0]
            .iter()
            .map(|c| c.value.clone())
            .collect();
        assert_eq!(
            set,
            ["1.2.3".replace("1.2.3", ">=1.2.3"), "<2.0.0-0".into()]
        );
    }
}

#[cfg(test)]
mod differential {
    use super::*;

    /// Every dependency version in upstream's test projects against every rule
    /// version, with node-semver's answers (`ERR` where it throws).
    #[test]
    fn matches_node_semver_on_fixture_versions() {
        let cases: serde_json::Value =
            serde_json::from_str(include_str!("../tests/semver-cases.json")).unwrap();
        let mut bad = Vec::new();
        for (v, want) in cases["min"].as_object().unwrap() {
            let got = match min_version(v) {
                Err(_) => serde_json::Value::from("ERR"),
                Ok(None) => serde_json::Value::Null,
                Ok(Some(s)) => serde_json::Value::from(s.version),
            };
            if &got != want {
                bad.push(format!("minVersion({v:?}) = {got} want {want}"));
            }
        }
        for (v, want) in cases["valid"].as_object().unwrap() {
            let got = valid(v)
                .map(serde_json::Value::from)
                .unwrap_or(serde_json::Value::Null);
            if &got != want {
                bad.push(format!("valid({v:?}) = {got} want {want}"));
            }
        }
        for c in cases["intersects"].as_array().unwrap() {
            let (a, b, want) = (c[0].as_str().unwrap(), c[1].as_str().unwrap(), &c[2]);
            let got = match intersects(a, b) {
                Err(_) => serde_json::Value::from("ERR"),
                Ok(x) => serde_json::Value::from(x),
            };
            if &got != want {
                bad.push(format!("intersects({a:?}, {b:?}) = {got} want {want}"));
            }
        }
        assert!(
            bad.is_empty(),
            "{} mismatches:\n{}",
            bad.len(),
            bad.iter().take(30).cloned().collect::<Vec<_>>().join("\n")
        );
    }
}

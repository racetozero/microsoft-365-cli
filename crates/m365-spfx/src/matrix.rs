//! `SpfxCompatibilityMatrix` (extracted to `spfx-matrix.json`) and
//! `spfx.getHighestNodeVersion`.

use crate::semver;
use m365_core::prelude::*;
use std::sync::LazyLock;

pub static MATRIX: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!("spfx-matrix.json")).expect("spfx-matrix.json")
});

/// `versions[version]`.
pub fn version(v: &str) -> Option<&'static Value> {
    MATRIX["versions"].get(v)
}

/// `SharePointVersion[n]` / `SharePointVersion[name]`.
pub fn sharepoint_version(k: &str) -> Option<&'static Value> {
    MATRIX["SharePointVersion"].get(k)
}

/// `semver.validRange(r)`: the normalized range or `None`.
fn valid_range(r: &str) -> Option<String> {
    let range = semver::Range::new(r).ok()?;
    let s = range
        .set
        .iter()
        .map(|c| {
            c.iter()
                .map(|x| x.value.trim().to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("||");
    Some(if s.is_empty() { "*".into() } else { s })
}

/// `spfx.getHighestNodeVersion(versionRange)`.
pub fn highest_node_version(range: &str) -> Result<String> {
    if range.is_empty() {
        return Err(Error::js("Node version range was not provided."));
    }
    let ranges: Vec<&str> = range
        .split("||")
        .map(str::trim)
        .filter(|r| !r.is_empty())
        .collect();
    let simple = |r: &str| {
        let parts: Vec<&str> = r.split('.').collect();
        parts.len() <= 3
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
    };
    if ranges.iter().all(|r| simple(r)) && !ranges.is_empty() {
        let mut best = ranges[0];
        for curr in &ranges[1..] {
            if let (Some(c), Some(b)) = (semver::coerce(curr), semver::coerce(best))
                && semver::gt(&c, &b)
            {
                best = curr;
            }
        }
        match best.split('.').count() {
            n if n >= 3 => return Ok(best.to_string()),
            2 => return Ok(format!("{best}.x")),
            _ => {}
        }
    }
    let mut highest: Option<f64> = None;
    let mut exact: Option<String> = None;
    for r in &ranges {
        let Some(normalized) = valid_range(r) else {
            continue;
        };
        let Ok(range) = semver::Range::new(&normalized) else {
            continue;
        };
        let mut max_major: f64 = 0.0;
        for set in &range.set {
            for c in set {
                let Some(v) = &c.semver else { continue };
                match c.operator.as_str() {
                    "<" => max_major = max_major.max(v.major - 1.0),
                    "<=" => {
                        max_major = max_major.max(v.major);
                        if highest.is_none_or(|h| v.major > h) {
                            exact = Some(v.version.clone());
                        }
                    }
                    ">=" | ">" => max_major = max_major.max(v.major),
                    _ => {}
                }
            }
        }
        highest = Some(highest.unwrap_or(0.0).max(max_major));
    }
    let Some(h) = highest else {
        return Err(Error::js(format!(
            "Unable to resolve the highest Node version for range '{range}'."
        )));
    };
    Ok(exact.unwrap_or_else(|| format!("{}.x", json::number_to_string(h))))
}

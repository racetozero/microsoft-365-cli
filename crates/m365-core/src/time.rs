//! JavaScript `Date` behaviour that upstream output depends on.

use chrono::{
    DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike, Utc,
};

/// `new Date(s)` for the formats commands use (ISO 8601 date and date-time,
/// with or without offset). Date-only strings are UTC; date-times without an
/// offset are local time, as in JavaScript.
pub fn parse_js_date(s: &str) -> Option<DateTime<Utc>> {
    let s = s.trim();
    // V8 only accepts `T` in strict ISO strings (two-digit components);
    // its legacy parser rejects the rest.
    if s.contains('T') && !strict_iso_prefix(s) {
        return None;
    }
    if !strict_iso_prefix(s)
        && let Some(d) = ["%Y-%m-%d"]
            .iter()
            .find_map(|f| NaiveDate::parse_from_str(s, f).ok())
    {
        return Local
            .from_local_datetime(&d.and_time(NaiveTime::MIN))
            .single()
            .map(|d| d.with_timezone(&Utc));
    }
    if let Ok(d) = DateTime::parse_from_rfc3339(s) {
        return Some(d.with_timezone(&Utc));
    }
    for f in [
        "%Y-%m-%dT%H:%M:%S%.f%:z",
        "%Y-%m-%dT%H:%M%:z",
        "%Y-%m-%dT%H:%M:%S%.fZ",
        "%Y-%m-%dT%H:%MZ",
    ] {
        if let Ok(d) = DateTime::parse_from_str(s, f) {
            return Some(d.with_timezone(&Utc));
        }
        if let Ok(d) = NaiveDateTime::parse_from_str(s, f) {
            return Some(Utc.from_utc_datetime(&d));
        }
    }
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&d.and_time(NaiveTime::MIN)));
    }
    if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m") {
        return Some(Utc.from_utc_datetime(&d.and_time(NaiveTime::MIN)));
    }
    for f in [
        "%Y-%m-%dT%H:%M:%S%.f",
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%dT%H:%M",
        "%Y-%m-%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
    ] {
        if let Ok(d) = NaiveDateTime::parse_from_str(s, f) {
            return Local
                .from_local_datetime(&d)
                .single()
                .map(|d| d.with_timezone(&Utc));
        }
    }
    for f in ["%Y/%m/%d", "%m/%d/%Y"] {
        if let Ok(d) = NaiveDate::parse_from_str(s, f) {
            return Local
                .from_local_datetime(&d.and_time(NaiveTime::MIN))
                .single()
                .map(|d| d.with_timezone(&Utc));
        }
    }
    if let Ok(d) = DateTime::parse_from_rfc2822(s) {
        return Some(d.with_timezone(&Utc));
    }
    None
}

/// `YYYY-MM-DD` followed by nothing or `THH:MM`.
fn strict_iso_prefix(s: &str) -> bool {
    let b = s.as_bytes();
    let d = |r: std::ops::Range<usize>| {
        b.get(r.clone())
            .is_some_and(|x| x.iter().all(u8::is_ascii_digit))
    };
    let date = b.len() >= 10 && d(0..4) && b[4] == b'-' && d(5..7) && b[7] == b'-' && d(8..10);
    date && (b.len() == 10
        || (b.len() >= 16 && b[10] == b'T' && d(11..13) && b[13] == b':' && d(14..16)))
}

/// `Date.prototype.toISOString()`.
pub fn to_iso(d: DateTime<Utc>) -> String {
    d.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

pub fn now_iso() -> String {
    to_iso(Utc::now())
}

/// `date.setMonth(date.getMonth() + months)` with JavaScript overflow rules.
pub fn add_months(d: DateTime<Utc>, months: i32) -> DateTime<Utc> {
    let total = d.year() * 12 + d.month0() as i32 + months;
    let (y, m0) = (total.div_euclid(12), total.rem_euclid(12) as u32);
    let first = NaiveDate::from_ymd_opt(y, m0 + 1, 1).unwrap();
    let date = first + chrono::Duration::days(d.day() as i64 - 1);
    Utc.from_utc_datetime(&date.and_time(d.time()))
}

/// The default locale Node's ICU picks: the first of `LC_ALL`,
/// `LC_MESSAGES` and `LANG` that is set (`de_DE.UTF-8` -> `de-DE`; set but
/// empty -> `und`; `C`/`POSIX` or none -> `en-US`).
fn default_locale() -> String {
    let v = ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .find_map(|k| std::env::var(k).ok());
    match v {
        None => "en-US".into(),
        Some(v) if v.is_empty() => "und".into(),
        Some(v) => {
            let id = v.split(['.', '@']).next().unwrap_or("");
            if id.is_empty() || id == "C" || id == "POSIX" {
                "en-US".into()
            } else {
                id.replace('_', "-")
            }
        }
    }
}

/// Date and time patterns of `toLocaleDateString()`/`toLocaleString()`:
/// `Y` year, `M`/`MM` month, `D`/`DD` day, `h` 12-hour, `H`/`HH` 24-hour,
/// `mm` minutes, `ss` seconds, `A` the day period, `'...'` literal text.
/// Covers common locales;
/// other languages get ICU's root patterns.
fn locale_patterns(locale: &str) -> (&'static str, &'static str) {
    let lang = locale.split('-').next().unwrap_or("");
    match locale {
        "und" => return ("Y-MM-DD", "Y-MM-DD h:mm:ss A"),
        "en-GB" | "en-IE" | "en-NZ" | "en-IN" => return ("DD/MM/Y", "DD/MM/Y, HH:mm:ss"),
        "en-CA" => return ("Y-MM-DD", "Y-MM-DD, h:mm:ss a.m."),
        "en-AU" => return ("DD/MM/Y", "DD/MM/Y, h:mm:ss am"),
        "fr-CA" => return ("Y-MM-DD", "Y-MM-DD HH 'h' mm 'min' ss 's'"),
        "pt-BR" => return ("DD/MM/Y", "DD/MM/Y, HH:mm:ss"),
        "zh-CN" => return ("Y/M/D", "Y/M/D HH:mm:ss"),
        _ => {}
    }
    match lang {
        "en" => ("M/D/Y", "M/D/Y, h:mm:ss A"),
        "de" | "nb" => ("D.M.Y", "D.M.Y, HH:mm:ss"),
        "fr" => ("DD/MM/Y", "DD/MM/Y HH:mm:ss"),
        "nl" => ("D-M-Y", "D-M-Y, HH:mm:ss"),
        "ja" => ("Y/M/D", "Y/M/D H:mm:ss"),
        "es" => ("D/M/Y", "D/M/Y, H:mm:ss"),
        "it" | "pt" => ("DD/MM/Y", "DD/MM/Y, HH:mm:ss"),
        "sv" => ("Y-MM-DD", "Y-MM-DD HH:mm:ss"),
        "da" => ("D.M.Y", "D.M.Y, HH.mm.ss"),
        "fi" => ("D.M.Y", "D.M.Y klo H.mm.ss"),
        "pl" => ("D.MM.Y", "D.MM.Y, HH:mm:ss"),
        "zh" => ("Y/M/D", "Y/M/D HH:mm:ss"),
        _ => ("Y-MM-DD", "Y-MM-DD HH:mm:ss"),
    }
}

fn apply_pattern<T: Datelike + Timelike>(p: &str, d: &T) -> String {
    let mut out = String::new();
    let mut it = p.chars().peekable();
    while let Some(c) = it.next() {
        let double = it.peek() == Some(&c);
        let two = |n: u32| format!("{n:02}");
        match c {
            '\'' => {
                for l in it.by_ref() {
                    if l == '\'' {
                        break;
                    }
                    out.push(l);
                }
            }
            'Y' => out.push_str(&d.year().to_string()),
            'M' | 'D' | 'H' | 'm' | 's' => {
                let n = match c {
                    'M' => d.month(),
                    'D' => d.day(),
                    'H' => d.hour(),
                    'm' => d.minute(),
                    _ => d.second(),
                };
                if double {
                    it.next();
                    out.push_str(&two(n));
                } else {
                    out.push_str(&n.to_string());
                }
            }
            'h' => out.push_str(&d.hour12().1.to_string()),
            'A' => out.push_str(if d.hour12().0 { "PM" } else { "AM" }),
            'a' if p.contains("a.m.") || p.ends_with("am") => {
                let pm = d.hour12().0;
                if p.contains("a.m.") {
                    // consume "a.m."
                    for _ in 0..3 {
                        it.next();
                    }
                    out.push_str(if pm { "p.m." } else { "a.m." });
                } else {
                    it.next();
                    out.push_str(if pm { "pm" } else { "am" });
                }
            }
            c => out.push(c),
        }
    }
    out
}

/// `new Date().toLocaleDateString()` in Node's default locale.
pub fn locale_date_string(d: DateTime<Local>) -> String {
    apply_pattern(locale_patterns(&default_locale()).0, &d)
}

/// `Date.prototype.toString()`, e.g. `Thu Nov 20 2026 09:53:00 GMT+0100 (Central European Standard Time)`.
pub fn to_js_string(d: DateTime<Local>) -> String {
    let off = d.offset().local_minus_utc();
    let sign = if off < 0 { '-' } else { '+' };
    let off = off.abs();
    format!(
        "{} GMT{}{:02}{:02}",
        d.format("%a %b %d %Y %H:%M:%S"),
        sign,
        off / 3600,
        (off % 3600) / 60
    )
}

/// Milliseconds since the epoch, like `Date.now()`.
pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

/// `new Date(ms).toLocaleString()` in Node's default locale (`Invalid Date`
/// when out of range).
pub fn locale_string_ms(ms: f64) -> String {
    if !ms.is_finite() || ms.abs() > 8.64e15 {
        return "Invalid Date".into();
    }
    let Some(u) = DateTime::<Utc>::from_timestamp_millis(ms.trunc() as i64) else {
        return "Invalid Date".into();
    };
    let d: DateTime<Local> = u.with_timezone(&Local);
    apply_pattern(locale_patterns(&default_locale()).1, &d)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Node 26 (full ICU) output for 2026-01-05 09:04:09 and 2026-11-25 00:04:09.
    #[test]
    fn locale_patterns_match_node() {
        let a = NaiveDate::from_ymd_opt(2026, 1, 5)
            .unwrap()
            .and_hms_opt(9, 4, 9)
            .unwrap();
        let b = NaiveDate::from_ymd_opt(2026, 11, 25)
            .unwrap()
            .and_hms_opt(0, 4, 9)
            .unwrap();
        let cases = [
            (
                "en-US",
                "1/5/2026",
                "1/5/2026, 9:04:09 AM",
                "11/25/2026, 12:04:09 AM",
            ),
            (
                "en-GB",
                "05/01/2026",
                "05/01/2026, 09:04:09",
                "25/11/2026, 00:04:09",
            ),
            (
                "en-CA",
                "2026-01-05",
                "2026-01-05, 9:04:09 a.m.",
                "2026-11-25, 12:04:09 a.m.",
            ),
            (
                "en-AU",
                "05/01/2026",
                "05/01/2026, 9:04:09 am",
                "25/11/2026, 12:04:09 am",
            ),
            (
                "de-DE",
                "5.1.2026",
                "5.1.2026, 09:04:09",
                "25.11.2026, 00:04:09",
            ),
            (
                "fr-FR",
                "05/01/2026",
                "05/01/2026 09:04:09",
                "25/11/2026 00:04:09",
            ),
            (
                "nl-NL",
                "5-1-2026",
                "5-1-2026, 09:04:09",
                "25-11-2026, 00:04:09",
            ),
            (
                "ja-JP",
                "2026/1/5",
                "2026/1/5 9:04:09",
                "2026/11/25 0:04:09",
            ),
            (
                "es-ES",
                "5/1/2026",
                "5/1/2026, 9:04:09",
                "25/11/2026, 0:04:09",
            ),
            (
                "sv-SE",
                "2026-01-05",
                "2026-01-05 09:04:09",
                "2026-11-25 00:04:09",
            ),
            (
                "da-DK",
                "5.1.2026",
                "5.1.2026, 09.04.09",
                "25.11.2026, 00.04.09",
            ),
            (
                "fi-FI",
                "5.1.2026",
                "5.1.2026 klo 9.04.09",
                "25.11.2026 klo 0.04.09",
            ),
            (
                "pl-PL",
                "5.01.2026",
                "5.01.2026, 09:04:09",
                "25.11.2026, 00:04:09",
            ),
            (
                "zh-CN",
                "2026/1/5",
                "2026/1/5 09:04:09",
                "2026/11/25 00:04:09",
            ),
            (
                "fr-CA",
                "2026-01-05",
                "2026-01-05 09 h 04 min 09 s",
                "2026-11-25 00 h 04 min 09 s",
            ),
            (
                "xx-YY",
                "2026-01-05",
                "2026-01-05 09:04:09",
                "2026-11-25 00:04:09",
            ),
            (
                "und",
                "2026-01-05",
                "2026-01-05 9:04:09 AM",
                "2026-11-25 12:04:09 AM",
            ),
        ];
        for (loc, date, dt_a, dt_b) in cases {
            let (dp, tp) = locale_patterns(loc);
            assert_eq!(apply_pattern(dp, &a), date, "{loc} date");
            assert_eq!(apply_pattern(tp, &a), dt_a, "{loc} datetime");
            assert_eq!(apply_pattern(tp, &b), dt_b, "{loc} datetime");
        }
    }
}

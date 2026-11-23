//! Port of upstream `src/utils/validation.ts`.

use crate::json::{js_number, parse};
use regex::Regex;
use std::sync::LazyLock;

macro_rules! re {
    ($name:ident, $pat:expr) => {
        static $name: LazyLock<Regex> = LazyLock::new(|| Regex::new($pat).unwrap());
    };
}

re!(
    GUID,
    r"(?i)^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$"
);
re!(CHANNEL, r"(?i)^19:[0-9a-zA-Z\-_]+@thread\.(skype|tacv2)$");
re!(
    CHAT,
    r"(?i)^19:[0-9a-zA-Z\-_]+(@thread\.v2|@unq\.gbl\.spaces)$"
);
re!(
    UPN,
    r"(?i)^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$"
);
re!(
    ISO_DATE,
    r"(?i)^(19|20)\d\d[- /.](0[1-9]|1[012])[- /.](0[1-9]|[12][0-9]|3[01])$"
);
re!(
    ISO_DASH_DT,
    r"(?i)^(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d:[0-5]\d\.\d+([+-][0-2]\d:[0-5]\d|Z))|(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d:[0-5]\d([+-][0-2]\d:[0-5]\d|Z))|(\d{4}-[01]\d-[0-3]\dT[0-2]\d:[0-5]\d([+-][0-2]\d:[0-5]\d|Z))$"
);
re!(
    ISO_DASH_D,
    r"(?i)^(19|20)\d\d[-](0[1-9]|1[012])[-](0[1-9]|[12][0-9]|3[01])$"
);
const DAYS: &str = r"[0-9]{4}-((0[13578]|1[02])-(0[1-9]|[12][0-9]|3[01])|(0[469]|11)-(0[1-9]|[12][0-9]|30)|(02)-(0[1-9]|[12][0-9]))";
static ISO_DT: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"T(0[0-9]|1[0-9]|2[0-3]):(0[0-9]|[1-5][0-9]):(0[0-9]|[1-5][0-9])\.[0-9]{7}Z$",
        r"T(0[0-9]|1[0-9]|2[0-3]):(0[0-9]|[1-5][0-9]):(0[0-9]|[1-5][0-9])\.[0-9]{3}Z$",
        r"T(0[0-9]|1[0-9]|2[0-3]):(0[0-9]|[1-5][0-9]):(0[0-9]|[1-5][0-9])Z$",
        r"T(0[0-9]|1[0-9]|2[0-3]):(0[0-9]|[1-5][0-9])Z$",
        r"T(0[0-9]|1[0-9]|2[0-3])Z$",
        r"$",
    ]
    .iter()
    .map(|t| Regex::new(&format!("^{DAYS}{t}")).unwrap())
    .collect()
});
static GRAPH_DT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(
        "^{DAYS}T(0[0-9]|1[0-9]|2[0-3]):(0[0-9]|[1-5][0-9]):(0[0-9]|[1-5][0-9])$"
    ))
    .unwrap()
});
re!(MAIL_NICK, r"(?i)^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]*$");
re!(
    POWER_PAGES,
    r"^https://[a-zA-Z0-9-]+\.powerappsportals\.com$"
);
re!(HEX_COLOR, r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$");
static DURATION: LazyLock<fancy_regex::Regex> = LazyLock::new(|| {
    fancy_regex::Regex::new(r"^P(?!$)((\d+Y)|(\d+\.\d+Y$))?((\d+M)|(\d+\.\d+M$))?((\d+W)|(\d+\.\d+W$))?((\d+D)|(\d+\.\d+D$))?(T(?=\d)((\d+H)|(\d+\.\d+H$))?((\d+M)|(\d+\.\d+M$))?(\d+(\.\d+)?S)?)??$").unwrap()
});

/// A validator result: `true`, `false` or a message (truthy in JavaScript).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Valid {
    Yes,
    No,
    Msg(String),
}

impl Valid {
    pub fn from_bool(b: bool) -> Valid {
        if b { Valid::Yes } else { Valid::No }
    }
    /// JavaScript truthiness of the result.
    pub fn truthy(&self) -> bool {
        match self {
            Valid::Yes => true,
            Valid::No => false,
            Valid::Msg(m) => !m.is_empty(),
        }
    }
    pub fn is_true(&self) -> bool {
        *self == Valid::Yes
    }
}

pub fn is_valid_guid(guid: &str) -> bool {
    if guid.is_empty() {
        return false;
    }
    GUID.is_match(guid) || guid.to_lowercase().trim() == "@meid"
}

/// The comma-separated items that fail `ok`, joined for the message.
fn invalid_items(s: &str, ok: fn(&str) -> bool) -> Valid {
    let bad: Vec<&str> = s.split(',').map(str::trim).filter(|i| !ok(i)).collect();
    if bad.is_empty() {
        Valid::Yes
    } else {
        Valid::Msg(bad.join(", "))
    }
}

pub fn is_valid_guid_array(s: &str) -> Valid {
    invalid_items(s, is_valid_guid)
}

pub fn is_valid_teams_channel_id(s: &str) -> bool {
    CHANNEL.is_match(s)
}

pub fn is_valid_teams_chat_id(s: &str) -> bool {
    CHAT.is_match(s)
}

pub fn is_valid_user_principal_name(upn: &str) -> bool {
    UPN.is_match(upn) || upn.to_lowercase().trim() == "@meusername"
}

pub fn is_valid_user_principal_name_array(s: &str) -> Valid {
    invalid_items(s, is_valid_user_principal_name)
}

pub fn is_valid_positive_integer(s: &str) -> bool {
    let n = js_number(s);
    !n.is_nan() && n.fract() == 0.0 && n.is_finite() && n > 0.0
}

pub fn is_valid_positive_integer_array(s: &str) -> Valid {
    invalid_items(s, is_valid_positive_integer)
}

pub fn is_valid_iso_date(s: &str) -> bool {
    ISO_DATE.is_match(s)
}

pub fn is_valid_iso_date_dash_only(s: &str) -> bool {
    ISO_DASH_DT.is_match(s) || ISO_DASH_D.is_match(s)
}

pub fn is_valid_iso_date_time(s: &str) -> bool {
    ISO_DT.iter().any(|r| r.is_match(s))
}

pub fn is_valid_graph_date_time(s: &str) -> bool {
    if !GRAPH_DT.is_match(s) {
        return false;
    }
    let date = s.split('T').next().unwrap_or("");
    let parts: Vec<i64> = date.split('-').map(|p| p.parse().unwrap_or(0)).collect();
    let (y, m, d) = (parts[0], parts[1], parts[2]);
    let leap = y % 4 == 0 && (y % 100 != 0 || y % 400 == 0);
    let days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    d <= days[(m - 1) as usize]
}

pub fn is_valid_boolean(s: &str) -> bool {
    let l = s.to_lowercase();
    l == "true" || l == "false"
}

pub fn is_valid_sharepoint_url(url: &str) -> Valid {
    if url.starts_with("https://") {
        Valid::Yes
    } else {
        Valid::Msg(format!(
            "'{url}' is not a valid SharePoint Online site URL."
        ))
    }
}

pub fn is_valid_mail_nickname(s: &str) -> bool {
    MAIL_NICK.is_match(s)
}

pub fn is_valid_iso_duration(s: &str) -> bool {
    DURATION.is_match(s).unwrap_or(false)
}

pub fn is_valid_permission(s: &str) -> Valid {
    let bad: Vec<&str> = s.split(' ').filter(|p| !p.contains('/')).collect();
    if bad.is_empty() {
        Valid::Yes
    } else {
        Valid::Msg(bad.join(", "))
    }
}

pub fn is_valid_power_pages_url(s: &str) -> bool {
    POWER_PAGES.is_match(s)
}

pub fn is_valid_json_or_empty(s: &str) -> bool {
    s.is_empty() || parse(s).is_ok()
}

pub fn is_valid_json(s: &str) -> bool {
    parse(s).is_ok()
}

const THEME_PROPS: &[&str] = &[
    "themePrimary",
    "themeLighterAlt",
    "themeLighter",
    "themeLight",
    "themeTertiary",
    "themeSecondary",
    "themeDarkAlt",
    "themeDark",
    "themeDarker",
    "neutralLighterAlt",
    "neutralLighter",
    "neutralLight",
    "neutralQuaternaryAlt",
    "neutralQuaternary",
    "neutralTertiaryAlt",
    "neutralTertiary",
    "neutralSecondary",
    "neutralPrimaryAlt",
    "neutralPrimary",
    "neutralDark",
    "black",
    "white",
];

pub fn is_valid_theme(s: &str) -> bool {
    let Ok(v) = parse(s) else { return false };
    let Some(o) = v.as_object() else { return false };
    THEME_PROPS.iter().all(|p| {
        o.get(*p)
            .is_some_and(|x| HEX_COLOR.is_match(&crate::json::to_js_string(x)))
    })
}

/// `validation.isDateInRange(date, monthOffset)`.
pub fn is_date_in_range(date: &str, month_offset: i32) -> bool {
    let Some(d) = crate::time::parse_js_date(date) else {
        return false;
    };
    let cutoff = crate::time::add_months(chrono::Utc::now(), -month_offset);
    d > cutoff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validators() {
        assert!(is_valid_guid("0b0d2ba1-1a25-4b1c-8a6b-8e2c3a6c8f5e"));
        assert!(is_valid_guid("@meId"));
        assert!(!is_valid_guid("x"));
        assert!(is_valid_user_principal_name("a@b.com"));
        assert!(is_valid_iso_date_time("2020-01-01T00:00:00Z"));
        assert!(is_valid_iso_date_time("2020-01-01"));
        assert!(!is_valid_iso_date_time("2020-02-30"));
        assert!(is_valid_iso_duration("PT1H"));
        assert!(!is_valid_iso_duration("P"));
        assert!(is_valid_teams_channel_id("19:abc@thread.skype"));
    }
}

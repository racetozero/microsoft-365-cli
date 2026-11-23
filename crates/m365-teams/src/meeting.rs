//! `teams funsettings`, `guestsettings`, `membersettings`,
//! `messagingsettings` and `teams meeting *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

async fn settings_list(cx: &Cx, prop: &str) -> Result<()> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/teams/{}?$select={prop}",
        eqp(&cx.u("teamId"))
    ))
    .graph_json()
    .data()
    .await?;
    match r.get(prop) {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

async fn settings_set(
    cx: &Cx,
    prop: &str,
    keys: &[&str],
    extra: Option<(&str, Option<String>)>,
) -> Result<()> {
    let mut inner = Obj::new();
    for k in keys {
        if let Some(v) = cx.opts.get(*k) {
            inner.insert(k.to_string(), v.clone());
        }
    }
    if let Some((k, Some(v))) = extra {
        inner.insert(k.to_string(), Value::from(v));
    }
    let mut data = Obj::new();
    data.insert(prop.to_string(), Value::Object(inner));
    Request::patch(format!("{GRAPH}/v1.0/teams/{}", eqp(&cx.u("teamId"))))
        .graph_json()
        .json(Value::Object(data))
        .send()
        .await?;
    Ok(())
}

pub async fn funsettings_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Listing fun settings for team {}", cx.u("teamId")));
    settings_list(cx, "funSettings").await
}

pub async fn funsettings_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating fun settings for team {}",
        cx.u("teamId")
    ));
    settings_set(
        cx,
        "funSettings",
        &["allowGiphy", "allowStickersAndMemes", "allowCustomMemes"],
        Some(("giphyContentRating", cx.s("giphyContentRating"))),
    )
    .await
}

pub async fn guestsettings_list(cx: &mut Cx) -> Result<()> {
    settings_list(cx, "guestSettings").await
}

pub async fn guestsettings_set(cx: &mut Cx) -> Result<()> {
    settings_set(
        cx,
        "guestSettings",
        &["allowCreateUpdateChannels", "allowDeleteChannels"],
        None,
    )
    .await
}

pub async fn membersettings_list(cx: &mut Cx) -> Result<()> {
    settings_list(cx, "memberSettings").await
}

pub async fn membersettings_set(cx: &mut Cx) -> Result<()> {
    settings_set(
        cx,
        "memberSettings",
        &[
            "allowAddRemoveApps",
            "allowCreateUpdateChannels",
            "allowCreateUpdateRemoveConnectors",
            "allowCreateUpdateRemoveTabs",
            "allowDeleteChannels",
        ],
        None,
    )
    .await
}

pub async fn messagingsettings_list(cx: &mut Cx) -> Result<()> {
    settings_list(cx, "messagingSettings").await
}

pub async fn messagingsettings_set(cx: &mut Cx) -> Result<()> {
    settings_set(
        cx,
        "messagingSettings",
        &[
            "allowUserEditMessages",
            "allowUserDeleteMessages",
            "allowOwnerDeleteMessages",
            "allowTeamMentions",
            "allowChannelMentions",
        ],
        None,
    )
    .await
}

fn app_only() -> bool {
    auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true)
}

fn date(v: &Value) -> Option<chrono::DateTime<chrono::Utc>> {
    m365_core::time::parse_js_date(&to_js_string(v))
}

pub fn add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    match i {
        0 | 1 => check(date(v).is_some_and(|d| d > chrono::Utc::now())),
        _ => match m365_core::validation::is_valid_user_principal_name_array(&to_js_string(v)) {
            m365_core::validation::Valid::Yes => Ok(()),
            m365_core::validation::Valid::Msg(m) => Err(Some(format!(
                "The following user principal names are invalid for the option 'participantUserNames': {m}."
            ))),
            m365_core::validation::Valid::No => Err(None),
        },
    }
}

pub fn add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(!t("startTime")
        || !t("endTime")
        || matches!((date(&o["startTime"]), date(&o["endTime"])), (Some(a), Some(b)) if a < b))
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let app = app_only();
    if app && cx.s("organizerEmail").is_none() {
        return Err(Error::msg(
            "The option 'organizerEmail' is required when creating a meeting using app only permissions",
        ));
    }
    if !app && cx.s("organizerEmail").is_some() {
        return Err(Error::msg(
            "The option 'organizerEmail' is not supported when creating a meeting using delegated permissions",
        ));
    }
    let mut url = format!("{GRAPH}/v1.0/me");
    if let Some(e) = cx.s("organizerEmail") {
        cx.verbose_log("Retrieving Organizer Id...");
        url = format!("{GRAPH}/v1.0/users/{}", entra::user::id_by_email(&e).await?);
    }
    cx.verbose_log("Creating the meeting...");
    let mut data = Obj::new();
    if let Some(p) = cx.s("participantUserNames") {
        let list: Vec<String> = p
            .trim()
            .to_lowercase()
            .split(',')
            .map(|n| n.trim().to_string())
            .collect();
        data.insert(
            "participants".into(),
            obj! { "attendees": list.into_iter().map(|u| obj! { "upn": u }).collect::<Vec<_>>() },
        );
    }
    if let Some(s) = cx.s("startTime") {
        data.insert("startDateTime".into(), Value::from(s));
    }
    if let Some(e) = cx.s("endTime") {
        data.insert("endDateTime".into(), Value::from(e));
        if cx.s("startTime").is_none() {
            data.insert(
                "startDateTime".into(),
                Value::from(m365_core::time::now_iso()),
            );
        }
    }
    if let Some(s) = cx.s("subject") {
        data.insert("subject".into(), Value::from(s));
    }
    if cx.has("recordAutomatically") {
        data.insert("recordAutomatically".into(), Value::Bool(true));
    }
    let r = Request::post(format!("{url}/onlineMeetings"))
        .graph_json()
        .header("content-type", "application/json")
        .json(Value::Object(data))
        .data()
        .await?;
    cx.log(r)
}

async fn user_id(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("userId") {
        return Ok(i);
    }
    if let Some(n) = cx.s("userName") {
        return entra::user::id_by_upn(&n).await;
    }
    entra::user::id_by_email(&cx.u("email")).await
}

fn has_user(cx: &Cx) -> bool {
    cx.s("userId").is_some() || cx.s("userName").is_some() || cx.s("email").is_some()
}

pub async fn attendancereport_get(cx: &mut Cx) -> Result<()> {
    let app = app_only();
    if app && !has_user(cx) {
        return Err(Error::msg(
            "The option 'userId', 'userName' or 'email' is required when retrieving meeting attendance report using app only permissions.",
        ));
    } else if !app && has_user(cx) {
        return Err(Error::msg(
            "The options 'userId', 'userName' and 'email' cannot be used when retrieving meeting attendance report using delegated permissions.",
        ));
    }
    let who = if app {
        format!(
            "specific user {}.",
            cx.s("userId")
                .or_else(|| cx.s("userName"))
                .or_else(|| cx.s("email"))
                .unwrap_or_else(|| "undefined".into())
        )
    } else {
        "currently logged in user".into()
    };
    cx.verbose_log(&format!("Retrieving attendance report for {who}."));
    let user = if app {
        format!("users/{}", user_id(cx).await?)
    } else {
        "me".into()
    };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/{user}/onlineMeetings/{}/attendanceReports/{}?$expand=attendanceRecords",
        cx.u("meetingId"),
        cx.u("id")
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn attendancereport_list(cx: &mut Cx) -> Result<()> {
    let app = app_only();
    if app && !has_user(cx) {
        return Err(Error::msg(
            "The option 'userId', 'userName' or 'email' is required when retrieving meeting attendance report using app only permissions",
        ));
    } else if !app && has_user(cx) {
        return Err(Error::msg(
            "The options 'userId', 'userName' and 'email' cannot be used when retrieving meeting attendance reports using delegated permissions",
        ));
    }
    cx.verbose_log(&format!(
        "Retrieving attendance report for {}",
        if app {
            "specific user"
        } else {
            "currently logged in user"
        }
    ));
    let mut url = format!("{GRAPH}/v1.0/");
    if app {
        url.push_str("users/");
        match cx.s("userId") {
            Some(i) => url.push_str(&i),
            None => {
                let o = obj! { "email": ?cx.str("email"), "userName": ?cx.str("userName"), "output": "json", "debug": cx.debug(), "verbose": cx.verbose() };
                let (out, _) = m365_core::runner::run_nested(
                    "entra user get",
                    o.as_object().unwrap().clone(),
                    true,
                )
                .await?;
                let v = json::parse(&out).unwrap_or(Value::Null);
                url.push_str(&u(&v, "id"));
            }
        }
    } else {
        url.push_str("me");
    }
    url.push_str(&format!(
        "/onlineMeetings/{}/attendanceReports",
        cx.u("meetingId")
    ));
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let app = app_only();
    if app && !has_user(cx) {
        return Err(Error::msg(
            "The option 'userId', 'userName' or 'email' is required when retrieving meetings using app only permissions",
        ));
    } else if !app && has_user(cx) {
        return Err(Error::msg(
            "The options 'userId', 'userName' and 'email' cannot be used when retrieving meetings using delegated permissions",
        ));
    }
    cx.verbose_log(&format!(
        "Retrieving meeting for {}",
        if app {
            "specific user"
        } else {
            "currently logged in user"
        }
    ));
    let mut url = format!("{GRAPH}/v1.0/");
    if app {
        url.push_str(&format!("users/{}", user_id(cx).await?));
    } else {
        url.push_str("me");
    }
    url.push_str(&format!(
        "/onlineMeetings?$filter=JoinWebUrl eq '{}'",
        eqp(&cx.u("joinUrl"))
    ));
    let r = Request::get(url).graph_json().data().await?;
    match arr(&r, "value").first() {
        Some(m) => cx.log(m.clone()),
        None => Err(Error::msg("The specified meeting was not found")),
    }
}

pub fn list_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(!o.get("endDateTime").is_some_and(truthy) || {
        let a: Vec<u16> = to_js_string(&o["startDateTime"]).encode_utf16().collect();
        let b: Vec<u16> = to_js_string(&o["endDateTime"]).encode_utf16().collect();
        a <= b
    })
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let app = app_only();
    if app && !has_user(cx) {
        return Err(Error::msg(
            "The option 'userId', 'userName' or 'email' is required when retrieving meetings using app only permissions",
        ));
    } else if !app && has_user(cx) {
        return Err(Error::msg(
            "The options 'userId', 'userName' and 'email' cannot be used when retrieving meetings using delegated permissions",
        ));
    }
    cx.verbose_log(&format!(
        "Retrieving meetings for user: {}...",
        cx.s("userId")
            .or_else(|| cx.s("userName"))
            .or_else(|| cx.s("email"))
            .unwrap_or_else(|| auth::jwt::user_name(
                &auth::default_access_token().unwrap_or_default()
            ))
    ));
    let mut base = format!("{GRAPH}/v1.0/");
    if let Some(u) = cx.s("userId").or_else(|| cx.s("userName")) {
        base.push_str(&format!("users/{u}"));
    } else if let Some(e) = cx.s("email") {
        base.push_str(&format!("users/{}", entra::user::id_by_email(&e).await?));
    } else {
        base.push_str("me");
    }
    let mut url = format!(
        "{base}/events?$filter=start/dateTime ge '{}'",
        cx.u("startDateTime")
    );
    if let Some(e) = cx.s("endDateTime") {
        url.push_str(&format!(" and end/dateTime lt '{e}'"));
    }
    if cx.bool("isOrganizer") {
        url.push_str(" and isOrganizer eq true");
    }
    url.push_str("&$select=onlineMeeting");
    let items = odata::get_all_items(&url).await?;
    let urls: Vec<String> = items
        .iter()
        .filter(|i| i.get("onlineMeeting").is_some_and(truthy))
        .map(|i| u(&i["onlineMeeting"], "joinUrl"))
        .collect();
    let rel = base.replacen(&format!("{GRAPH}/v1.0/"), "", 1);
    let mut result: Vec<Value> = Vec::new();
    for (bi, chunk) in urls.chunks(20).enumerate() {
        let i = bi * 20;
        cx.verbose_log(&format!(
            "Retrieving meetings {}-{}...",
            i + 1,
            (i + 20).min(urls.len())
        ));
        let reqs: Vec<Value> = chunk
            .iter()
            .enumerate()
            .map(|(idx, url)| obj! { "id": i + idx, "method": "GET", "url": format!("{rel}/onlineMeetings?$filter=joinWebUrl eq '{}'", eqp(url)) })
            .collect();
        let res = Request::post(format!("{GRAPH}/v1.0/$batch"))
            .accept("application/json")
            .header("content-type", "application/json")
            .json(obj! { "requests": reqs })
            .json_response()
            .data()
            .await?;
        for r in arr(&res, "responses") {
            if r.get("status").and_then(Value::as_u64) == Some(200) {
                result.push(
                    r.get("body")
                        .and_then(|b| b.get("value"))
                        .and_then(|v| v.get(0))
                        .cloned()
                        .unwrap_or(Value::Null),
                );
            } else {
                let body = r.get("body").cloned().unwrap_or(Value::Null);
                if !body
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .is_some_and(truthy)
                {
                    return Err(Error::Thrown(
                        body.get("error")
                            .and_then(|e| e.get("code"))
                            .cloned()
                            .unwrap_or(Value::Null),
                    ));
                }
                return Err(Error::Thrown(body));
            }
        }
    }
    result.sort_by(|a, b| {
        let (x, y) = (
            to_js_string(a.get("startDateTime").unwrap_or(&Value::Null)),
            to_js_string(b.get("startDateTime").unwrap_or(&Value::Null)),
        );
        x.encode_utf16().cmp(y.encode_utf16())
    });
    cx.log(Value::Array(result))
}

pub fn transcript_get_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let p = to_js_string(v);
    let dir = std::path::Path::new(&p).parent().map(|d| {
        if d.as_os_str().is_empty() {
            std::path::Path::new(".")
        } else {
            d
        }
    });
    check(dir.is_some_and(|d| d.exists()))
}

async fn transcript_base(cx: &Cx, msg_required: &str, msg_delegated: &str) -> Result<String> {
    let mut url = format!("{GRAPH}/beta/");
    if app_only() {
        if !has_user(cx) {
            return Err(Error::msg(msg_required));
        }
        url.push_str("users/");
        if let Some(i) = cx.s("userId") {
            url.push_str(&i);
        } else if let Some(n) = cx.s("userName") {
            url.push_str(&n);
        } else if let Some(e) = cx.s("email") {
            cx.verbose_log(&format!("Getting user ID for user with email '{e}'."));
            url.push_str(&entra::user::id_by_email(&e).await?);
        }
    } else {
        if has_user(cx) {
            return Err(Error::msg(msg_delegated));
        }
        url.push_str("me");
    }
    Ok(url)
}

pub async fn transcript_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving transcript for the given meeting...");
    let mut url = transcript_base(
        cx,
        "The option 'userId', 'userName' or 'email' is required when retrieving meeting transcript using app only permissions",
        "The options 'userId', 'userName', and 'email' cannot be used while retrieving meeting transcript using delegated permissions",
    )
    .await?;
    url.push_str(&format!(
        "/onlineMeetings/{}/transcripts/{}",
        cx.u("meetingId"),
        cx.u("id")
    ));
    match cx.s("outputFile") {
        Some(f) => {
            url.push_str("/content?$format=text/vtt");
            let r = Request::get(url)
                .accept("application/json;odata.metadata=none")
                .bytes_response()
                .send()
                .await?;
            std::fs::write(&f, &r.bytes)?;
            cx.verbose_log(&format!("File saved to path {f}"));
            Ok(())
        }
        None => {
            let r = Request::get(url).graph_json().data().await?;
            if truthy(&r) {
                cx.log(r)
            } else {
                Err(Error::msg("The specified meeting transcript was not found"))
            }
        }
    }
}

pub async fn transcript_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving transcript list for the given meeting...");
    let mut url = transcript_base(
        cx,
        "The option 'userId', 'userName' or 'email' is required when retrieving meeting transcripts list using app only permissions",
        "The options 'userId', 'userName' and 'email' cannot be used while retrieving meeting transcripts using delegated permissions",
    )
    .await?;
    url.push_str(&format!(
        "/onlineMeetings/{}/transcripts",
        cx.u("meetingId")
    ));
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

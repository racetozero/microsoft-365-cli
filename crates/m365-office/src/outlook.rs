//! `outlook calendar *`, `outlook calendargroup *`, `outlook event *`,
//! `outlook mail *`, `outlook mailbox *`, `outlook message *`, `outlook room*`.

use m365_core::prelude::*;

const APP_ONLY_USER: &str =
    "When running with application permissions either userId or userName is required.";

fn token() -> String {
    auth::default_access_token().unwrap_or_default()
}

fn user_ident(cx: &Cx) -> Option<String> {
    cx.str("userId").or_else(|| cx.str("userName"))
}

/// `calendarGroup.getUserCalendarGroupByName`.
async fn group_by_name(user: &str, name: &str, properties: Option<&str>) -> Result<Value> {
    let mut url = format!(
        "{GRAPH}/v1.0/users('{user}')/calendarGroups?$filter=name eq '{}'",
        eqp(name)
    );
    if let Some(p) = properties {
        url.push_str(&format!("&$select={p}"));
    }
    let items = odata::get_all_items(&url).await?;
    m365_core::runner::pick_one(
        &items,
        "id",
        Error::js(format!(
            "The specified calendar group '{name}' does not exist."
        )),
        &format!("Multiple calendar groups with name '{name}' found."),
    )
}

/// `calendar.getUserCalendarByName`.
async fn calendar_by_name(
    user: &str,
    name: &str,
    group: Option<&str>,
    properties: Option<&str>,
) -> Result<Value> {
    let g = group
        .map(|g| format!("calendarGroups/{g}/"))
        .unwrap_or_default();
    let mut url = format!(
        "{GRAPH}/v1.0/users('{}')/{g}calendars?$filter=name eq '{}'",
        eqp(user),
        eqp(name)
    );
    if let Some(p) = properties {
        url.push_str(&format!("&$select={p}"));
    }
    let items = odata::get_all_items(&url).await?;
    m365_core::runner::pick_one(
        &items,
        "id",
        Error::js(format!("The specified calendar '{name}' does not exist.")),
        &format!("Multiple calendars with name '{name}' found."),
    )
}

async fn group_id_opt(cx: &Cx, user: &str) -> Result<Option<String>> {
    match cx.s("calendarGroupName") {
        Some(n) => Ok(Some(u(&group_by_name(user, &n, Some("id")).await?, "id"))),
        None => Ok(cx.s("calendarGroupId")),
    }
}

fn group_seg(g: &Option<String>) -> String {
    g.as_ref()
        .map(|g| format!("calendarGroups/{g}/"))
        .unwrap_or_default()
}

/// Delegated access to another user's calendars needs a `.Shared` scope.
fn check_other_user(cx: &Cx, token: &str, ci_id: bool, scopes: &[&str], msg: &str) -> Result<()> {
    let id = auth::jwt::user_id(token);
    let name = auth::jwt::user_name(token);
    let other_id = cx.s("userId").is_some_and(|u| {
        if ci_id {
            u.to_lowercase() != id.to_lowercase()
        } else {
            u != id
        }
    });
    let other_name = cx
        .s("userName")
        .is_some_and(|u| u.to_lowercase() != name.to_lowercase());
    if (other_id || other_name)
        && !auth::jwt::scopes(token)
            .iter()
            .any(|s| scopes.contains(&s.as_str()))
    {
        return Err(Error::msg(msg));
    }
    Ok(())
}

pub async fn calendar_add(cx: &mut Cx) -> Result<()> {
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    let mut url = format!("{GRAPH}/v1.0/users('{user}')/");
    if cx.s("calendarGroupId").is_some() || cx.s("calendarGroupName").is_some() {
        let g = group_id_opt(cx, &user)
            .await?
            .unwrap_or_else(|| "undefined".into());
        url.push_str(&format!("calendarGroups/{g}/calendars"));
    } else {
        url.push_str("calendars");
    }
    cx.verbose_log(&format!("Creating a calendar for the user {user}..."));
    let r = Request::post(url)
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! {
            "name": ?cx.val("name").cloned(),
            "color": ?cx.val("color").cloned(),
            "defaultOnlineMeetingProvider": ?cx.val("defaultOnlineMeetingProvider").cloned(),
            "isDefaultCalendar": ?cx.val("default").cloned(),
        })
        .data()
        .await?;
    cx.log(r)
}

pub async fn calendar_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting calendar...");
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    let g = group_id_opt(cx, &user).await?;
    let r = match cx.s("id") {
        Some(id) => {
            Request::get(format!(
                "{GRAPH}/v1.0/users('{user}')/{}calendars/{id}",
                group_seg(&g)
            ))
            .graph_json()
            .data()
            .await?
        }
        None => calendar_by_name(&user, &cx.u("name"), g.as_deref(), None).await?,
    };
    cx.log(r)
}

pub async fn calendar_list(cx: &mut Cx) -> Result<()> {
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Retrieving calendars for {user}..."));
    let g = group_id_opt(cx, &user).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/users('{user}')/{}calendars",
            group_seg(&g)
        ))
        .await?,
    ))
}

pub async fn calendar_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .str("id")
        .or_else(|| cx.str("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove calendar '{what}'?"),
    )? {
        return Ok(());
    }
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    let g = group_id_opt(cx, &user).await?;
    let id = match cx.s("name") {
        Some(n) => u(
            &calendar_by_name(&user, &n, g.as_deref(), Some("id")).await?,
            "id",
        ),
        None => cx.u("id"),
    };
    cx.verbose_log(&format!("Removing calendar with ID {id}..."));
    let mut url = format!(
        "{GRAPH}/v1.0/users('{user}')/{}calendars/{id}",
        group_seg(&g)
    );
    if cx.bool("permanent") {
        url.push_str("/permanentDelete");
        Request::post(url)
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    } else {
        Request::delete(url)
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    }
    Ok(())
}

pub async fn calendar_set(cx: &mut Cx) -> Result<()> {
    let token = token();
    let base = if auth::jwt::is_app_only(&token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(APP_ONLY_USER));
        }
        format!(
            "{GRAPH}/v1.0/users('{}')",
            user_ident(cx).unwrap_or_default()
        )
    } else if cx.s("userId").is_some() || cx.s("userName").is_some() {
        check_other_user(
            cx,
            &token,
            false,
            &["Calendars.ReadWrite.Shared"],
            "To update calendars of other users, the Entra ID application used for authentication must have the Calendars.ReadWrite.Shared delegated permission assigned.",
        )?;
        format!(
            "{GRAPH}/v1.0/users('{}')",
            user_ident(cx).unwrap_or_default()
        )
    } else {
        format!("{GRAPH}/v1.0/me")
    };
    cx.verbose_log(&format!("Updating calendar '{}'...", cx.u("id")));
    let g = match cx.s("calendarGroupName") {
        Some(n) => {
            let who = user_ident(cx).unwrap_or_else(|| auth::jwt::user_id(&token));
            Some(u(&group_by_name(&who, &n, Some("id")).await?, "id"))
        }
        None => cx.s("calendarGroupId"),
    };
    let url = match g {
        Some(g) => format!("{base}/calendarGroups/{g}/calendars/{}", cx.u("id")),
        None => format!("{base}/calendars/{}", cx.u("id")),
    };
    let data = obj! { "name": ?cx.val("name").cloned(), "color": ?cx.val("color").cloned(), "isDefaultCalendar": ?cx.val("isDefault").cloned() };
    let r = Request::patch(url).graph_json().json(data).data().await?;
    cx.log(r)
}

// ---- calendargroup -------------------------------------------------------------

/// The user URL and id for calendar group commands: app-only needs a user,
/// delegated access to other users needs `Calendars.ReadWrite.Shared`.
fn group_user(cx: &Cx, token: &str, verb: &str, ci_id: bool) -> Result<(String, String)> {
    if auth::jwt::is_app_only(token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(APP_ONLY_USER));
        }
        let id = user_ident(cx).unwrap_or_default();
        cx.verbose_log(&format!(
            "{verb} calendar group using application permissions for user '{id}'..."
        ));
        Ok((format!("{GRAPH}/v1.0/users('{}')", eqp(&id)), id))
    } else if cx.s("userId").is_some() || cx.s("userName").is_some() {
        let what = if verb == "Creating" {
            "create"
        } else {
            "update"
        };
        check_other_user(
            cx,
            token,
            ci_id,
            &["Calendars.ReadWrite.Shared"],
            &format!(
                "To {what} calendar groups of other users, the Entra ID application used for authentication must have the Calendars.ReadWrite.Shared delegated permission assigned."
            ),
        )?;
        let id = user_ident(cx).unwrap_or_default();
        cx.verbose_log(&format!(
            "{verb} calendar group using delegated permissions for user '{id}'..."
        ));
        Ok((format!("{GRAPH}/v1.0/users('{}')", eqp(&id)), id))
    } else {
        cx.verbose_log(&format!("{verb} calendar group for the signed-in user..."));
        Ok((format!("{GRAPH}/v1.0/me"), auth::jwt::user_id(token)))
    }
}

pub async fn calendargroup_add(cx: &mut Cx) -> Result<()> {
    let token = auth::ensure_access_token(&auth::default_resource(), false).await?;
    let (url, _) = group_user(cx, &token, "Creating", true)?;
    let r = Request::post(format!("{url}/calendarGroups"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "name": ?cx.val("name").cloned() })
        .data()
        .await?;
    cx.log(r)
}

const READ_SHARED: [&str; 2] = ["Calendars.Read.Shared", "Calendars.ReadWrite.Shared"];
const READ_MSG: &str = "To retrieve calendar groups of other users, the Entra ID application used for authentication must have either the Calendars.Read.Shared or Calendars.ReadWrite.Shared delegated permission assigned.";

pub async fn calendargroup_get(cx: &mut Cx) -> Result<()> {
    let token = token();
    let user = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        user_ident(cx)
    } else {
        None
    };
    if auth::jwt::is_app_only(&token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(APP_ONLY_USER));
        }
    } else if user.is_some() {
        check_other_user(cx, &token, false, &READ_SHARED, READ_MSG)?;
    }
    let path = user
        .map(|u| format!("users('{}')", eqp(&u)))
        .unwrap_or_else(|| "me".into());
    if let Some(n) = cx.s("name") {
        cx.verbose_log(&format!("Retrieving calendar group '{n}'..."));
        let items = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/{path}/calendarGroups?$filter=name eq '{}'",
            eqp(&n)
        ))
        .await?;
        return match items.into_iter().next() {
            Some(g) => cx.log(g),
            None => Err(Error::msg(format!(
                "The specified calendar group '{n}' does not exist."
            ))),
        };
    }
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving calendar group '{id}'..."));
    let r = Request::get(format!("{GRAPH}/v1.0/{path}/calendarGroups/{id}"))
        .graph_json()
        .data()
        .await?;
    cx.log(r)
}

pub async fn calendargroup_list(cx: &mut Cx) -> Result<()> {
    let token = token();
    let url = if auth::jwt::is_app_only(&token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(APP_ONLY_USER));
        }
        let user = user_ident(cx).unwrap_or_default();
        cx.verbose_log(&format!("Retrieving calendar groups for user '{user}'..."));
        format!("{GRAPH}/v1.0/users('{user}')/calendarGroups")
    } else if cx.s("userId").is_some() || cx.s("userName").is_some() {
        check_other_user(cx, &token, false, &READ_SHARED, READ_MSG)?;
        let user = user_ident(cx).unwrap_or_default();
        cx.verbose_log(&format!("Retrieving calendar groups for user '{user}'..."));
        format!("{GRAPH}/v1.0/users('{user}')/calendarGroups")
    } else {
        cx.verbose_log("Retrieving calendar groups for the signed-in user...");
        format!("{GRAPH}/v1.0/me/calendarGroups")
    };
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn calendargroup_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove calendar group '{what}'?"),
    )? {
        return Ok(());
    }
    let token = token();
    if auth::jwt::is_app_only(&token) == Some(true)
        && cx.s("userId").is_none()
        && cx.s("userName").is_none()
    {
        return Err(Error::msg(APP_ONLY_USER));
    }
    let user = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        user_ident(cx).unwrap_or_default()
    } else {
        auth::jwt::user_id(&token)
    };
    let endpoint = format!("{GRAPH}/v1.0/users('{}')", eqp(&user));
    let id = match cx.s("name") {
        Some(n) => {
            cx.verbose_log(&format!("Retrieving calendar group by name '{n}'..."));
            u(&group_by_name(&user, &n, None).await?, "id")
        }
        None => cx.u("id"),
    };
    cx.verbose_log(&format!("Removing calendar group '{id}'..."));
    Request::delete(format!("{endpoint}/calendarGroups/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn calendargroup_set(cx: &mut Cx) -> Result<()> {
    let token = token();
    let (url, user) = group_user(cx, &token, "Updating", false)?;
    let id = match cx.s("id") {
        Some(i) => i,
        None => u(&group_by_name(&user, &cx.u("name"), None).await?, "id"),
    };
    cx.verbose_log(&format!("Updating calendar group '{id}'..."));
    Request::patch(format!("{url}/calendarGroups/{id}"))
        .graph_json()
        .json(obj! { "name": ?cx.val("newName").cloned() })
        .send()
        .await?;
    Ok(())
}

// ---- event ---------------------------------------------------------------------

/// Delegated tokens may only touch the signed-in user's events.
fn own_events_only(cx: &Cx, token: &str, verb: &str, ci_id: bool) -> Result<()> {
    if let Some(id) = cx.s("userId") {
        let cur = auth::jwt::user_id(token);
        if (ci_id && id.to_lowercase() != cur.to_lowercase()) || (!ci_id && id != cur) {
            return Err(Error::msg(format!(
                "You can only {verb} your own events when using delegated permissions. The specified userId '{id}' does not match the current user '{cur}'."
            )));
        }
    }
    if let Some(n) = cx.s("userName") {
        let cur = auth::jwt::user_name(token);
        if n.to_lowercase() != cur.to_lowercase() {
            return Err(Error::msg(format!(
                "You can only {verb} your own events when using delegated permissions. The specified userName '{n}' does not match the current user '{cur}'."
            )));
        }
    }
    Ok(())
}

/// `values.every(Boolean)` after splitting on commas and trimming.
pub fn event_add_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(to_js_string(v).split(',').all(|c| !c.trim().is_empty()))
}

fn split_trim(cx: &Cx, k: &str) -> Option<Vec<String>> {
    cx.str(k)
        .map(|v| v.split(',').map(|c| c.trim().to_string()).collect())
}

pub async fn event_add(cx: &mut Cx) -> Result<()> {
    let token = token();
    let app = auth::jwt::is_app_only(&token) == Some(true);
    if app {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(
                "The option 'userId' or 'userName' is required when creating an event using application permissions.",
            ));
        }
    } else {
        own_events_only(cx, &token, "create", true)?;
    }
    let (user, principal) = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        let u = user_ident(cx).unwrap_or_default();
        let p = format!("users('{}')", eqp(&u));
        (u, p)
    } else {
        (auth::jwt::user_name(&token), "me".to_string())
    };
    cx.verbose_log(&format!("Creating event for user {user}..."));
    let cal = match cx.s("calendarName") {
        Some(n) => Some(u(&calendar_by_name(&user, &n, None, None).await?, "id")),
        None => cx.s("calendarId"),
    };
    let url = match &cal {
        Some(c) => format!("{GRAPH}/v1.0/{principal}/calendars/{}/events", eqp(c)),
        None => format!("{GRAPH}/v1.0/{principal}/events"),
    };
    let mut body = Obj::new();
    body.insert("subject".into(), Value::from(cx.u("subject").trim()));
    let mut start = Obj::new();
    start.insert(
        "dateTime".into(),
        cx.val("start").cloned().unwrap_or(Value::Null),
    );
    let mut end = Obj::new();
    end.insert(
        "dateTime".into(),
        cx.val("end").cloned().unwrap_or(Value::Null),
    );
    if let Some(b) = cx.str("bodyContents") {
        let content = match b.strip_prefix('@') {
            // read outside the command's try/catch: printed as `Error: ${err}`
            Some(f) => std::fs::read_to_string(f).map_err(|e| {
                Error::msg(format!("Error: {}", util::node_fs_error(&e, "open", f)))
            })?,
            None => b,
        };
        let ct = if cx
            .str("bodyContentType")
            .is_some_and(|t| t.eq_ignore_ascii_case("text"))
        {
            "text"
        } else {
            "html"
        };
        body.insert(
            "body".into(),
            obj! { "contentType": ct, "content": content },
        );
    }
    if cx.opt_bool("allowNewTimeProposals") == Some(false) {
        body.insert("allowNewTimeProposals".into(), Value::Bool(false));
    }
    if let Some(c) = split_trim(cx, "categories") {
        body.insert("categories".into(), Value::from(c));
    }
    if cx.bool("hideAttendees") {
        body.insert("hideAttendees".into(), Value::Bool(true));
    }
    if let Some(i) = cx.s("importance") {
        body.insert("importance".into(), Value::from(i.to_lowercase()));
    }
    if cx.bool("isAllDay") {
        body.insert("isAllDay".into(), Value::Bool(true));
    }
    if cx.bool("isOnlineMeeting") {
        body.insert("isOnlineMeeting".into(), Value::Bool(true));
    }
    if cx.opt_bool("isReminderOn") == Some(false) {
        body.insert("isReminderOn".into(), Value::Bool(false));
    }
    if cx.s("location").is_some() || cx.s("locationEmailAddress").is_some() {
        body.insert("location".into(), obj! { "displayName": ?cx.s("location"), "locationEmailAddress": ?cx.s("locationEmailAddress") });
    }
    if let Some(l) = split_trim(cx, "locations") {
        body.insert(
            "locations".into(),
            Value::Array(l.into_iter().map(|d| obj! { "displayName": d }).collect()),
        );
    }
    let provider = cx
        .str("onlineMeetingProvider")
        .unwrap_or_else(|| "unknown".into());
    let provider = [
        "teamsForBusiness",
        "skypeForBusiness",
        "skypeForConsumer",
        "unknown",
    ]
    .iter()
    .find(|p| p.eq_ignore_ascii_case(&provider))
    .map(|p| p.to_string())
    .unwrap_or(provider);
    if provider != "unknown" {
        body.insert("onlineMeetingProvider".into(), Value::from(provider));
    }
    if cx.s("optionalAttendees").is_some()
        || cx.s("requiredAttendees").is_some()
        || cx.s("resources").is_some()
    {
        let mut attendees = Vec::new();
        for (k, t) in [
            ("optionalAttendees", "optional"),
            ("requiredAttendees", "required"),
            ("resources", "resource"),
        ] {
            if let Some(v) = cx.s(k) {
                for a in v.split(',') {
                    let a = a.trim();
                    let address = if !a.eq_ignore_ascii_case("@meusername") {
                        a.to_string()
                    } else if app {
                        return Err(Error::msg(format!(
                            "It's not possible to use {a} with application permissions"
                        )));
                    } else {
                        auth::jwt::user_name(&token)
                    };
                    attendees.push(obj! { "emailAddress": obj! { "address": address }, "type": t });
                }
            }
        }
        body.insert("attendees".into(), Value::Array(attendees));
    }
    if let Some(r) = cx.s("recurrence") {
        let text = match r.strip_prefix('@') {
            Some(_) => {
                let f = r.replacen('@', "", 1);
                std::fs::read_to_string(&f).map_err(|e| {
                    Error::msg(format!("Error: {}", util::node_fs_error(&e, "open", &f)))
                })?
            }
            None => r,
        };
        body.insert(
            "recurrence".into(),
            json::js_parse(&text).map_err(Error::msg)?,
        );
    }
    if cx.opt_bool("isReminderOn") != Some(false)
        && let Some(m) = cx.val("reminderMinutesBeforeStart")
    {
        body.insert("reminderMinutesBeforeStart".into(), m.clone());
    }
    if cx.opt_bool("responseRequested") == Some(false) {
        body.insert("responseRequested".into(), Value::Bool(false));
    }
    if let Some(s) = cx.s("sensitivity") {
        body.insert("sensitivity".into(), Value::from(s.to_lowercase()));
    }
    let show = cx.str("showAs").unwrap_or_else(|| "busy".into());
    let show = [
        "free",
        "tentative",
        "busy",
        "oof",
        "workingElsewhere",
        "unknown",
    ]
    .iter()
    .find(|p| p.eq_ignore_ascii_case(&show))
    .map(|p| p.to_string())
    .unwrap_or(show);
    if show != "busy" {
        body.insert("showAs".into(), Value::from(show));
    }
    let tz = cx.str("timeZone").unwrap_or_else(|| "UTC".into());
    if !tz.is_empty() {
        start.insert("timeZone".into(), Value::from(tz.as_str()));
        end.insert("timeZone".into(), Value::from(tz.as_str()));
    }
    body.insert("start".into(), Value::Object(start));
    body.insert("end".into(), Value::Object(end));
    // keep upstream's key order: subject, start, end, then the rest
    let mut ordered = Obj::new();
    for k in ["subject", "start", "end"] {
        if let Some(v) = body.shift_remove(k) {
            ordered.insert(k.into(), v);
        }
    }
    ordered.extend(body);
    if let Some(t) = cx.s("transactionId") {
        ordered.insert("transactionId".into(), Value::from(t));
    }
    let r = Request::post(url)
        .graph_json()
        .header("content-type", "application/json")
        .json(Value::Object(ordered))
        .data()
        .await?;
    cx.log(r)
}

pub async fn event_cancel(cx: &mut Cx) -> Result<()> {
    let token = token();
    if auth::jwt::is_app_only(&token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(
                "The option 'userId' or 'userName' is required when cancelling an event using application permissions.",
            ));
        }
    } else {
        own_events_only(cx, &token, "cancel", false)?;
    }
    let principal = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        format!("users('{}')", user_ident(cx).unwrap_or_default())
    } else {
        "me".into()
    };
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to cancel event with id '{id}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Cancelling event with id '{id}'..."));
    Request::post(format!("{GRAPH}/v1.0/{principal}/events/{id}/cancel"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json")
        .json(obj! { "comment": ?cx.val("comment").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn event_get(cx: &mut Cx) -> Result<()> {
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Retrieving event {} for user {user}...",
        cx.u("id")
    ));
    let cal = match cx.s("calendarName") {
        Some(n) => Some(u(&calendar_by_name(&user, &n, None, None).await?, "id")),
        None => cx.s("calendarId"),
    };
    let base = format!("{GRAPH}/v1.0/users('{}')", eqp(&user));
    let url = match cal {
        Some(c) => format!("{base}/calendars/{c}/events/{}", cx.u("id")),
        None => format!("{base}/events/{}", cx.u("id")),
    };
    let mut req = Request::get(url).graph_json();
    if let Some(tz) = cx.s("timeZone") {
        req = req.header("Prefer", format!("outlook.timezone=\"{tz}\""));
    }
    cx.log(req.data().await?)
}

pub async fn event_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting a list of the events...");
    let mut q = select_expand(cx.s("properties").as_deref());
    if cx.s("filter").is_some() || cx.s("startDateTime").is_some() || cx.s("endDateTime").is_some()
    {
        let mut f = cx.s("filter").unwrap_or_default();
        if let Some(s) = cx.s("startDateTime") {
            f.push_str(&format!(
                "{}start/dateTime ge '{s}'",
                if f.is_empty() { "" } else { " and " }
            ));
        }
        if let Some(e) = cx.s("endDateTime") {
            f.push_str(&format!(
                "{}start/dateTime lt '{e}'",
                if f.is_empty() { "" } else { " and " }
            ));
        }
        q.push(format!("$filter={f}"));
    }
    let qs = qs(&q);
    let user = user_ident(cx).unwrap_or_else(|| "undefined".into());
    let cal = match cx.s("calendarName") {
        Some(n) => Some(u(&calendar_by_name(&user, &n, None, None).await?, "id")),
        None => cx.s("calendarId"),
    };
    let url = match cal {
        Some(c) => format!("{GRAPH}/v1.0/users('{user}')/calendars/{c}/events{qs}"),
        None => format!("{GRAPH}/v1.0/users('{user}')/events{qs}"),
    };
    let events = match cx.s("timeZone") {
        Some(tz) => {
            odata::get_all_items_req(
                Request::get(url)
                    .graph_json()
                    .header("Prefer", format!("outlook.timezone=\"{tz}\"")),
            )
            .await?
        }
        None => odata::get_all_items(&url).await?,
    };
    cx.log(Value::Array(events))
}

pub async fn event_remove(cx: &mut Cx) -> Result<()> {
    let token = token();
    if auth::jwt::is_app_only(&token) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(
                "The option 'userId' or 'userName' is required when removing an event using application permissions.",
            ));
        }
    } else {
        own_events_only(cx, &token, "remove", false)?;
    }
    let principal = match user_ident(cx).filter(|u| !u.is_empty()) {
        Some(u) => format!("users('{u}')"),
        None => "me".into(),
    };
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove event with id '{id}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing event with id '{id}'..."));
    let url = format!("{GRAPH}/v1.0/{principal}/events/{id}");
    if cx.bool("permanent") {
        Request::post(format!("{url}/permanentDelete"))
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    } else {
        Request::delete(url)
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    }
    Ok(())
}

// ---- mail, mailbox -------------------------------------------------------------

pub async fn mail_searchfolder_add(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/me/mailFolders/searchFolders/childFolders");
    if auth::jwt::is_app_only(&token()) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(
                "When running with application permissions either userId or userName is required",
            ));
        }
        let user = user_ident(cx).unwrap_or_default();
        url = format!("{GRAPH}/v1.0/users('{user}')/mailFolders/searchFolders/childFolders");
        cx.verbose_log(&format!(
            "Creating a mail search folder in the mailbox of the user {user}..."
        ));
    } else if cx.s("userId").is_some() || cx.s("userName").is_some() {
        return Err(Error::msg(
            "You can create mail search folder for other users only if CLI is authenticated in app-only mode",
        ));
    }
    let r = Request::post(url)
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! {
            "@odata.type": "#microsoft.graph.mailSearchFolder",
            "displayName": ?cx.val("folderName").cloned(),
            "includeNestedFolders": ?cx.val("includeNestedFolders").cloned(),
            "filterQuery": ?cx.val("messageFilter").cloned(),
            "sourceFolderIds": ?cx.str("sourceFoldersIds").map(|s| s.split(',').map(String::from).collect::<Vec<_>>()),
        })
        .data()
        .await?;
    cx.log(r)
}

fn attachments(o: &Obj) -> Vec<String> {
    match o.get("attachment") {
        Some(Value::Array(a)) => a.iter().map(to_js_string).collect(),
        Some(v) => vec![to_js_string(v)],
        None => Vec::new(),
    }
}

fn recipient(email: &str) -> Value {
    obj! { "emailAddress": obj! { "address": email.trim() } }
}

fn mail_body(o: &Obj) -> std::result::Result<Value, String> {
    use base64::Engine;
    let s = |k: &str| o.get(k).map(to_js_string);
    let list = |k: &str| s(k).map(|v| Value::Array(v.split(',').map(recipient).collect()));
    let mut atts = Vec::new();
    for a in attachments(o) {
        let bytes = std::fs::read(&a).map_err(|e| util::node_fs_error(&e, "open", &a))?;
        let name = std::path::Path::new(&a)
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default();
        atts.push(obj! { "@odata.type": "#microsoft.graph.fileAttachment", "name": name, "contentBytes": base64::engine::general_purpose::STANDARD.encode(bytes) });
    }
    Ok(obj! {
        "message": obj! {
            "subject": ?o.get("subject").cloned(),
            "body": obj! { "contentType": o.get("bodyContentType").filter(|v| truthy(v)).cloned().unwrap_or(Value::from("Text")), "content": ?o.get("bodyContents").cloned() },
            "from": ?s("mailbox").filter(|m| !m.is_empty()).map(|m| recipient(&m)),
            "toRecipients": ?list("to"),
            "ccRecipients": ?list("cc"),
            "bccRecipients": ?list("bcc"),
            "importance": ?o.get("importance").cloned(),
            "attachments": ?(o.contains_key("attachment")).then_some(Value::Array(atts)),
        },
        "saveToSentItems": ?o.get("saveToSentItems").cloned(),
    })
}

pub fn mail_send_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    if !o.get("attachment").is_some_and(truthy) {
        return Ok(true);
    }
    for a in attachments(o) {
        let p = std::path::Path::new(&a);
        if !p.exists() {
            return Err(format!("File with path '{a}' was not found."));
        }
        if !p.is_file() {
            return Err(format!("'{a}' is not a file."));
        }
    }
    let body = mail_body(o)?;
    if json::js_len(&stringify(&body)) > 4_194_304 {
        return Err("Exceeded the max total size of attachments which is 3MB.".into());
    }
    Ok(true)
}

pub async fn mail_send(cx: &mut Cx) -> Result<()> {
    if auth::jwt::is_app_only(&token()) == Some(true) && cx.s("sender").is_none() {
        return Err(Error::msg(
            "Specify a upn or user id in the 'sender' option when using app only authentication.",
        ));
    }
    let who = cx
        .s("sender")
        .map(|s| format!("users/{}", eqp(&s)))
        .unwrap_or_else(|| "me".into());
    let body = mail_body(&cx.opts).map_err(Error::js)?;
    Request::post(format!("{GRAPH}/v1.0/{who}/sendMail"))
        .graph_json()
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    Ok(())
}

fn mailbox_url(cx: &Cx, verb: &str, other: &str) -> Result<String> {
    if auth::jwt::is_app_only(&token()) == Some(true) {
        if cx.s("userId").is_none() && cx.s("userName").is_none() {
            return Err(Error::msg(
                "When running with application permissions either userId or userName is required",
            ));
        }
        let user = user_ident(cx).unwrap_or_default();
        cx.verbose_log(&format!("{verb} mailbox settings for user {user}..."));
        return Ok(format!("{GRAPH}/v1.0/users('{user}')/mailboxSettings"));
    }
    if cx.s("userId").is_some() || cx.s("userName").is_some() {
        return Err(Error::msg(format!(
            "You can {other} mailbox settings of other users only if CLI is authenticated in app-only mode"
        )));
    }
    Ok(format!("{GRAPH}/v1.0/me/mailboxSettings"))
}

pub async fn mailbox_settings_get(cx: &mut Cx) -> Result<()> {
    let url = mailbox_url(cx, "Retrieving", "retrieve")?;
    cx.log(Request::get(url).graph_json().data().await?)
}

pub async fn mailbox_settings_set(cx: &mut Cx) -> Result<()> {
    let url = mailbox_url(cx, "Updating", "update")?;
    let g = |k: &str| cx.s(k);
    let mut d = Obj::new();
    for k in [
        "dateFormat",
        "timeFormat",
        "timeZone",
        "delegateMeetingMessageDeliveryOptions",
    ] {
        if let Some(v) = g(k) {
            d.insert(k.into(), Value::from(v));
        }
    }
    if let Some(l) = g("language") {
        d.insert("language".into(), obj! { "locale": l });
    }
    if [
        "workingDays",
        "workingHoursStartTime",
        "workingHoursEndTime",
        "workingHoursTimeZone",
    ]
    .iter()
    .any(|k| g(k).is_some())
    {
        d.insert(
            "workingHours".into(),
            obj! {
                "daysOfWeek": ?g("workingDays").map(|w| w.split(',').map(String::from).collect::<Vec<_>>()),
                "startTime": ?g("workingHoursStartTime"),
                "endTime": ?g("workingHoursEndTime"),
                "timeZone": ?g("workingHoursTimeZone").map(|t| obj! { "name": t }),
            },
        );
    }
    let ar = [
        "autoReplyStatus",
        "autoReplyExternalAudience",
        "autoReplyExternalMessage",
        "autoReplyInternalMessage",
        "autoReplyStartDateTime",
        "autoReplyStartTimeZone",
        "autoReplyEndDateTime",
        "autoReplyEndTimeZone",
    ];
    if ar.iter().any(|k| g(k).is_some()) {
        let sched = |dt: &str, tz: &str| {
            (g(dt).is_some() || g(tz).is_some())
                .then(|| obj! { "dateTime": ?g(dt), "timeZone": ?g(tz) })
        };
        d.insert(
            "automaticRepliesSetting".into(),
            obj! {
                "status": ?g("autoReplyStatus"),
                "externalAudience": ?g("autoReplyExternalAudience"),
                "externalReplyMessage": ?g("autoReplyExternalMessage"),
                "internalReplyMessage": ?g("autoReplyInternalMessage"),
                "scheduledStartDateTime": ?sched("autoReplyStartDateTime", "autoReplyStartTimeZone"),
                "scheduledEndDateTime": ?sched("autoReplyEndDateTime", "autoReplyEndTimeZone"),
            },
        );
    }
    cx.log(
        Request::patch(url)
            .graph_json()
            .json(Value::Object(d))
            .data()
            .await?,
    )
}

// ---- message, room -------------------------------------------------------------

const WELL_KNOWN: [&str; 17] = [
    "archive",
    "clutter",
    "conflicts",
    "conversationhistory",
    "deleteditems",
    "drafts",
    "inbox",
    "junkemail",
    "localfailures",
    "msgfolderroot",
    "outbox",
    "recoverableitemsdeletions",
    "scheduled",
    "searchfolders",
    "sentitems",
    "serverfailures",
    "syncissues",
];

pub async fn message_get(cx: &mut Cx) -> Result<()> {
    let app = auth::jwt::is_app_only(&token()) == Some(true);
    cx.verbose_log(&format!(
        "Retrieving message with id {} using {}",
        cx.u("id"),
        if app {
            "app only permissions"
        } else {
            "delegated permissions"
        }
    ));
    let (uid, un) = (cx.s("userId"), cx.s("userName"));
    let mut url = String::new();
    if app {
        if uid.is_none() && un.is_none() {
            return Err(Error::msg(
                "The option 'userId' or 'userName' is required when retrieving an email using app only credentials",
            ));
        }
        if uid.is_some() && un.is_some() {
            return Err(Error::msg(
                "Both options 'userId' and 'userName' cannot be set when retrieving an email using app only credentials",
            ));
        }
        url.push_str(&format!("users/{}", uid.or(un).unwrap_or_default()));
    } else {
        if uid.is_some() && un.is_some() {
            return Err(Error::msg(
                "Both options 'userId' and 'userName' cannot be set when retrieving an email using delegated credentials",
            ));
        }
        match uid.or(un) {
            Some(u) => url.push_str(&format!("users/{u}")),
            None => url.push_str("me"),
        }
    }
    url.push_str(&format!("/messages/{}", cx.u("id")));
    cx.log(
        Request::get(format!("{GRAPH}/v1.0/{url}"))
            .graph_json()
            .data()
            .await?,
    )
}

async fn folder_by_name(user_url: &str, name: &str) -> Result<String> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/{user_url}/mailFolders?$filter=displayName eq '{}'&$select=id",
        eqp(name)
    ))
    .graph_json()
    .data()
    .await?;
    let v = arr(&r, "value");
    Ok(u(
        &m365_core::runner::pick_one(
            v,
            "id",
            Error::msg(format!("Folder with name '{name}' not found")),
            &format!("Multiple folders with name '{name}' found."),
        )?,
        "id",
    ))
}

pub async fn message_list(cx: &mut Cx) -> Result<()> {
    if cx.s("userId").is_none()
        && cx.s("userName").is_none()
        && auth::jwt::is_app_only(&token()) == Some(true)
    {
        return Err(Error::msg(
            "You must specify either the userId or userName option when using app-only permissions.",
        ));
    }
    let user_url = match (cx.s("userId"), cx.s("userName")) {
        (Some(i), _) => format!("users/{i}"),
        (None, Some(n)) => format!("users/{}", eqp(&n)),
        _ => "me".into(),
    };
    let folder = if let Some(f) = cx.s("folderId") {
        f
    } else if let Some(n) = cx.s("folderName") {
        if WELL_KNOWN.contains(&n.to_lowercase().as_str()) {
            n
        } else {
            folder_by_name(&user_url, &n).await?
        }
    } else {
        String::new()
    };
    let folder_url = if folder.is_empty() {
        String::new()
    } else {
        format!("/mailFolders/{folder}")
    };
    let mut url = format!("{GRAPH}/v1.0/{user_url}{folder_url}/messages?$top=100");
    let mut filters = Vec::new();
    if let Some(s) = cx.s("startTime") {
        filters.push(format!("receivedDateTime ge {s}"));
    }
    if let Some(e) = cx.s("endTime") {
        filters.push(format!("receivedDateTime lt {e}"));
    }
    if !filters.is_empty() {
        url.push_str(&format!("&$filter={}", filters.join(" and ")));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn message_move(cx: &mut Cx) -> Result<()> {
    async fn folder(id: Option<String>, name: Option<String>) -> Result<String> {
        if let Some(i) = id {
            return Ok(i);
        }
        let n = name.unwrap_or_else(|| "undefined".into());
        if WELL_KNOWN.contains(&n.as_str()) {
            return Ok(n);
        }
        folder_by_name("me", &n).await
    }
    let src = folder(cx.s("sourceFolderId"), cx.str("sourceFolderName")).await?;
    let tgt = folder(cx.s("targetFolderId"), cx.str("targetFolderName")).await?;
    Request::post(format!(
        "{GRAPH}/v1.0/me/mailFolders/{src}/messages/{}/move",
        cx.u("id")
    ))
    .graph_json()
    .json(obj! { "destinationId": tgt })
    .send()
    .await?;
    Ok(())
}

pub async fn message_remove(cx: &mut Cx) -> Result<()> {
    let app = auth::jwt::is_app_only(&token()) == Some(true);
    let (uid, un) = (cx.s("userId"), cx.s("userName"));
    if app {
        if uid.is_none() && un.is_none() {
            return Err(Error::msg(
                "The option 'userId' or 'userName' is required when removing a message using application permissions.",
            ));
        }
        if uid.is_some() && un.is_some() {
            return Err(Error::msg(
                "Both options 'userId' and 'userName' cannot be used together when removing a message using application permissions.",
            ));
        }
    } else if uid.is_some() && un.is_some() {
        return Err(Error::msg(
            "Both options 'userId' and 'userName' cannot be used together when removing a message using delegated permissions.",
        ));
    }
    let principal = match (uid, un) {
        (Some(i), _) => format!("users/{i}"),
        (None, Some(n)) => format!("users/{}", eqp(&n)),
        _ => "me".into(),
    };
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove message with id '{id}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing message with id '{id}' using {} permissions.",
        if app { "application" } else { "delegated" }
    ));
    Request::delete(format!("{GRAPH}/v1.0/{principal}/messages/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn room_list(cx: &mut Cx) -> Result<()> {
    let url = match cx.s("roomlistEmail") {
        Some(e) => format!("{GRAPH}/v1.0/places/{e}/microsoft.graph.roomlist/rooms"),
        None => format!("{GRAPH}/v1.0/places/microsoft.graph.room"),
    };
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn roomlist_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/places/microsoft.graph.roomlist")).await?,
    ))
}

// ---- onedrive ------------------------------------------------------------------

pub async fn onedrive_list(cx: &mut Cx) -> Result<()> {
    use m365_core::ut::spo;
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Retrieving list of OneDrive sites...");
    let mut all = Vec::new();
    let mut start = "0".to_string();
    loop {
        let d = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
        let body = format!(
            r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties /></ChildItemQuery></Query></Actions><ObjectPaths><Constructor Id="1" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /><Method Id="3" ParentId="1" Name="GetSitePropertiesFromSharePointByFilters"><Parameters><Parameter TypeId="{{b92aeee2-c92c-4b67-abcc-024e471bc140}}"><Property Name="IncludeDetail" Type="Boolean">false</Property><Property Name="IncludePersonalSite" Type="Enum">1</Property><Property Name="StartIndex" Type="String">{start}</Property><Property Name="Template" Type="String">SPSPERS</Property></Parameter></Parameters></Method></ObjectPaths></Request>"#,
            spo::application_name()
        );
        let text = spo::process_query(&admin, &d.value, body).await?;
        let json = spo::parse_csom(&text)?;
        if let Some(e) = spo::csom_error(&json) {
            return Err(Error::value(e));
        }
        let sites = json.last().cloned().unwrap_or(Value::Null);
        all.extend(
            need_arr(&sites, "_Child_Items_", "iterator")?
                .iter()
                .cloned(),
        );
        match sites
            .get("NextStartIndexFromSharePoint")
            .filter(|v| truthy(v))
        {
            Some(n) => start = to_js_string(n),
            None => break,
        }
    }
    cx.log(Value::Array(all))
}

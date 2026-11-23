//! `spo list view *`, `spo list webhook *` and `spo list sitescript get`.

use crate::list::validate_url_guid;
use crate::sp::{self, first, ne, opt};
use m365_core::prelude::*;
use m365_core::time;
use m365_core::ut::url as urlutil;
use m365_core::validation as v;

fn so(o: &Obj, k: &str) -> Option<String> {
    o.get(k).filter(|v| truthy(v)).map(to_js_string)
}

/// `lists(guid'...')` | `lists/GetByTitle('...')` | `GetList('...')`.
fn selector(cx: &Cx, by_title: &str) -> String {
    sp::list_segment(cx, &cx.u("webUrl"), sp::LIST, by_title).unwrap_or_default()
}

fn guid_msg(o: &Obj, k: &str, suffix: &str) -> std::result::Result<(), String> {
    match so(o, k) {
        Some(g) if !v::is_valid_guid(&g) => Err(format!("{g}{suffix}")),
        _ => Ok(()),
    }
}

// ---- validators ----------------------------------------------------------------

pub fn field_add_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")?;
    guid_msg(o, "viewId", " is not a valid GUID")?;
    guid_msg(o, "id", " is not a valid GUID")?;
    if let Some(p) = so(o, "position")
        && json::parse_int(&p).is_none()
    {
        return Err(format!("{p} is not a number"));
    }
    Ok(())
}

pub fn field_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")?;
    guid_msg(o, "viewId", " is not a valid GUID")?;
    if let Some(g) = so(o, "id")
        && !v::is_valid_guid(&g)
    {
        return Err(format!(
            "{} is not a valid GUID",
            o.get("viewId")
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into())
        ));
    }
    Ok(())
}

pub fn field_set_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")?;
    guid_msg(o, "viewId", " is not a valid GUID")?;
    guid_msg(o, "id", " is not a valid GUID")?;
    let p = o
        .get("position")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    if json::parse_int(&p).is_none() {
        return Err(format!("{p} is not a number"));
    }
    Ok(())
}

pub fn view_get_validate(o: &Obj) -> std::result::Result<(), String> {
    crate::list::url_validate(o)?;
    guid_msg(o, "listId", " in option listId is not a valid GUID")?;
    guid_msg(o, "id", " in option id is not a valid GUID")
}

pub fn view_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")?;
    guid_msg(o, "id", " is not a valid GUID")
}

fn id_first(o: &Obj) -> std::result::Result<(), String> {
    let id = o
        .get("id")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    if !v::is_valid_guid(&id) {
        return Err(format!("{id} is not a valid GUID"));
    }
    validate_url_guid(o, "listId")
}

pub fn webhook_id_validate(o: &Obj) -> std::result::Result<(), String> {
    id_first(o)
}

fn max_expiration() -> chrono::DateTime<chrono::Utc> {
    let now = chrono::Local::now();
    (now + chrono::Duration::days(180)).with_timezone(&chrono::Utc)
}

pub fn webhook_add_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")?;
    let raw = so(o, "expirationDateTime");
    let parsed = o
        .get("expirationDateTime")
        .map(to_js_string)
        .and_then(|s| time::parse_js_date(&s));
    if raw.is_some() && parsed.is_none_or(|d| d.timestamp_millis() == 0) {
        return Err("Provide the date in one of the following formats:\n          'YYYY-MM-DD'\n          'YYYY-MM-DDThh:mm'\n          'YYYY-MM-DDThh:mmZ'\n          'YYYY-MM-DDThh:mm±hh:mm'".into());
    }
    if let Some(d) = parsed
        && (d.timestamp_millis() < time::now_ms() || d >= max_expiration())
    {
        return Err("Provide an expiration date which is a date time in the future and within 6 months from now".into());
    }
    Ok(())
}

pub fn webhook_set_validate(o: &Obj) -> std::result::Result<(), String> {
    id_first(o)?;
    if so(o, "notificationUrl").is_none()
        && so(o, "expirationDateTime").is_none()
        && so(o, "clientState").is_none()
    {
        return Err("Specify notificationUrl, expirationDateTime, clientState or multiple, at least one is required".into());
    }
    if let Some(e) = so(o, "expirationDateTime")
        && time::parse_js_date(&e).is_none_or(|d| d.timestamp_millis() == 0)
    {
        let g = m365_core::term::gray;
        return Err(format!(
            "{e} is not a valid date format. Provide the date in one of the following formats:\n      {}\n      {}\n      {}\n      {}",
            g("YYYY-MM-DD"),
            g("YYYY-MM-DDThh:mm"),
            g("YYYY-MM-DDThh:mmZ"),
            g("YYYY-MM-DDThh:mm±hh:mm")
        ));
    }
    Ok(())
}

// ---- sitescript ----------------------------------------------------------------

pub async fn sitescript_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Extracting Site Script from list {} in site at {web}...",
        first(cx, &["listId", "listTitle", "listUrl"])
    ));
    let srv = match ne(cx, "listUrl") {
        Some(l) => {
            cx.debug_log(&format!("Retrieving List from URL '{l}'..."));
            urlutil::get_server_relative_path(&web, &l)
        }
        None => {
            let mut url = format!("{web}/_api/web/");
            if let Some(i) = ne(cx, "listId") {
                cx.debug_log(&format!("Retrieving List from Id '{i}'..."));
                url.push_str(&format!("lists(guid'{}')?$expand=RootFolder", eqp(&i)));
            } else if let Some(t) = ne(cx, "listTitle") {
                cx.debug_log(&format!("Retrieving List from Title '{t}'..."));
                url.push_str(&format!(
                    "lists/GetByTitle('{}')?$expand=RootFolder",
                    eqp(&t)
                ));
            }
            let r = Request::get(url).spo_json().data().await?;
            match dig(&r, &["RootFolder", "ServerRelativeUrl"])? {
                Some(v) => to_js_string(v),
                None => "undefined".into(),
            }
        }
    };
    let abs = urlutil::get_absolute_url(&web, &srv);
    let r = Request::post(format!("{web}/_api/Microsoft_SharePoint_Utilities_WebTemplateExtensions_SiteScriptUtility_GetSiteScriptFromList"))
        .spo_json()
        .header("content-type", "application/json;odata=nometadata")
        .json(obj! { "listUrl": abs })
        .data()
        .await?;
    match r.get("value").filter(|v| truthy(v)) {
        Some(s) => cx.log(s.clone()),
        None => Err(Error::msg(format!(
            "An error has occurred, the site script could not be extracted from list '{}'",
            first(cx, &["listId", "listTitle"])
        ))),
    }
}

// ---- views ---------------------------------------------------------------------

pub async fn view_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Adding view '{}' to list...", opt(cx, "title")));
    let url = format!(
        "{}/_api/web/{}/views/add",
        cx.u("webUrl"),
        selector(cx, "getByTitle")
    );
    let mut fields: Vec<String> = cx
        .s("fields")
        .map(|f| f.split(',').map(|x| x.trim().to_string()).collect())
        .unwrap_or_default();
    let mut p = obj! {
        "Title": ?cx.val("title").cloned(),
        "ViewFields": obj! { "results": fields.clone() },
        "Query": cx.val("viewQuery").cloned().unwrap_or(Value::Null),
        "PersonalView": cx.bool("personal"),
        "SetAsDefaultView": cx.bool("default"),
        "Paged": cx.val("paged").cloned().unwrap_or(Value::Bool(true)),
        "RowLimit": cx.val("rowLimit").cloned().unwrap_or(Value::from(30)),
        "CustomFormatter": cx.val("customFormatter").cloned().unwrap_or(Value::Null),
    };
    match cx.str("type").as_deref() {
        Some("calendar") => {
            let def = cx
                .s("calendarDefaultLayout")
                .unwrap_or_else(|| "month".into());
            let title = opt(cx, "calendarTitleField");
            let sub = cx.s("calendarSubTitleField").unwrap_or_default();
            let (start, end) = (
                opt(cx, "calendarStartDateField"),
                opt(cx, "calendarEndDateField"),
            );
            let mut vf: Vec<String> = [start.clone(), end.clone(), title.clone(), sub.clone()]
                .into_iter()
                .filter(|f| !f.is_empty())
                .collect();
            let extra: Vec<String> = fields
                .iter()
                .filter(|f| !vf.contains(&f.trim().to_string()))
                .cloned()
                .collect();
            vf.extend(extra);
            let b = |x: &str| if def == x { "TRUE" } else { "FALSE" };
            p["CalendarViewStyles"] = Value::from(format!(
                r#"<CalendarViewStyle Title="Day" Type="day" Template="CalendarViewdayChrome" Sequence="1" Default="{}" /><CalendarViewStyle Title="Week" Type="week" Template="CalendarViewweekChrome" Sequence="2" Default="{}" /><CalendarViewStyle Title="Month" Type="month" Template="CalendarViewmonthChrome" Sequence="3" Default="{}" /><CalendarViewStyle Title="Work week" Type="workweek" Template="CalendarViewweekChrome" Sequence="4" Default="{}" />"#,
                b("day"),
                b("week"),
                b("month"),
                b("workWeek")
            ));
            p["Query"] = Value::from(format!(
                "<Where><DateRangesOverlap><FieldRef Name='{start}' /><FieldRef Name='{end}' /><Value Type='DateTime'><Month /></Value></DateRangesOverlap></Where>"
            ));
            p["ViewData"] = Value::from(format!(
                r#"<FieldRef Name="{title}" Type="CalendarMonthTitle" /><FieldRef Name="{title}" Type="CalendarWeekTitle" /><FieldRef Name="{sub}" Type="CalendarWeekLocation" /><FieldRef Name="{title}" Type="CalendarDayTitle" /><FieldRef Name="{sub}" Type="CalendarDayLocation" />"#
            ));
            p["ViewFields"]["results"] = Value::from(vf);
            p["ViewType2"] = Value::from("MODERNCALENDAR");
        }
        Some("gallery") => p["ViewType2"] = Value::from("TILES"),
        Some("kanban") => {
            let bucket = opt(cx, "kanbanBucketField");
            if !fields.contains(&bucket) {
                fields.push(bucket.clone());
            }
            p["ViewFields"]["results"] = Value::from(fields);
            if ne(cx, "customFormatter").is_none() {
                p["CustomFormatter"] = Value::from("{}");
            }
            p["ViewData"] = Value::from(format!(
                r#"<FieldRef Name="{bucket}" Type="KanbanPivotColumn" />"#
            ));
            p["ViewType2"] = Value::from("KANBAN");
        }
        _ => {}
    }
    // `undefined` placeholders keep their key position but are not serialized
    if let Some(o) = p.as_object_mut() {
        for k in ["Query", "CustomFormatter"] {
            if o.get(k) == Some(&Value::Null) {
                o.shift_remove(k);
            }
        }
    }
    let r = Request::post(url)
        .header("content-type", "application/json;odata=verbose")
        .spo_json()
        .json(obj! { "parameters": p })
        .data()
        .await?;
    cx.log(r)
}

async fn field_internal_name(cx: &Cx, sel: &str) -> Result<Option<Value>> {
    let fs = match ne(cx, "id") {
        Some(i) => format!("/getbyid('{}')", eqp(&i)),
        None => format!("/getbyinternalnameortitle('{}')", eqp(&opt(cx, "title"))),
    };
    let f = Request::get(format!("{}/_api/web/{sel}/fields{fs}", cx.u("webUrl")))
        .spo_json()
        .data()
        .await?;
    Ok(f.get("InternalName").cloned())
}

fn view_sel(cx: &Cx) -> String {
    match ne(cx, "viewId") {
        Some(v) => format!("('{}')", eqp(&v)),
        None => format!("/GetByTitle('{}')", eqp(&opt(cx, "viewTitle"))),
    }
}

pub async fn field_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let sel = selector(cx, "GetByTitle");
    let who = first(cx, &["id", "title"]);
    cx.verbose_log(&format!("Getting field {who}..."));
    let field = field_internal_name(cx, &sel).await?;
    let name = field
        .as_ref()
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let view = first(cx, &["viewId", "viewTitle"]);
    cx.verbose_log(&format!("Adding the field {who} to the view {view}..."));
    let vs = view_sel(cx);
    Request::post(format!(
        "{web}/_api/web/{sel}/views{vs}/viewfields/addviewfield('{name}')"
    ))
    .spo_json()
    .send()
    .await?;
    let Some(pos) = cx.val("position").cloned() else {
        cx.debug_log("No field position.");
        return Ok(());
    };
    if cx.debug() {
        cx.stderr("moveField request...");
        m365_core::log::log_stderr_value(&pos);
    }
    cx.verbose_log(&format!(
        "Moving the field {who} to the position {} from view {view}...",
        to_js_string(&pos)
    ));
    Request::post(format!(
        "{web}/_api/web/{sel}/views{vs}/viewfields/moveviewfieldto"
    ))
    .spo_json()
    .json(obj! { "field": ?field, "index": pos })
    .send()
    .await?;
    Ok(())
}

pub async fn field_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let who = first(cx, &["id", "title"]);
    let view = first(cx, &["viewId", "viewTitle"]);
    if !cx.proceed("force", &format!("Are you sure you want to remove the field {who} from the view {view} from list {} in site {web}?", first(cx, &["listId", "listTitle"])))? {
        return Ok(());
    }
    cx.verbose_log(&format!("Getting field {who}..."));
    let sel = selector(cx, "GetByTitle");
    let name = field_internal_name(cx, &sel)
        .await?
        .as_ref()
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Removing field {who} from view {view}..."));
    Request::post(format!(
        "{web}/_api/web/{sel}/views{}/viewfields/removeviewfield('{name}')",
        view_sel(cx)
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn field_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let sel = selector(cx, "GetByTitle");
    let vs = view_sel(cx);
    let who = first(cx, &["id", "title"]);
    cx.verbose_log(&format!("Getting field {who}..."));
    let name = field_internal_name(cx, &sel).await?;
    cx.verbose_log(&format!(
        "Moving the field {who} in view {} to position {}...",
        first(cx, &["viewId", "viewTitle"]),
        opt(cx, "position")
    ));
    Request::post(format!(
        "{web}/_api/web/{sel}/views{vs}/viewfields/moveviewfieldto"
    ))
    .spo_json()
    .json(obj! { "field": ?name, "index": ?cx.val("position").cloned() })
    .send()
    .await?;
    Ok(())
}

pub async fn view_get(cx: &mut Cx) -> Result<()> {
    let sel = selector(cx, "getByTitle");
    let list = if sel.is_empty() {
        sel
    } else {
        format!("/{sel}")
    };
    let view = match ne(cx, "id") {
        Some(i) => format!("getById('{}')", eqp(&i)),
        None => format!("getByTitle('{}')", eqp(&opt(cx, "title"))),
    };
    cx.log(
        Request::get(format!("{}/_api/web{list}/views/{view}", cx.u("webUrl")))
            .spo_json()
            .data()
            .await?,
    )
}

pub async fn view_list(cx: &mut Cx) -> Result<()> {
    let sel = selector(cx, "GetByTitle");
    let url = if sel.is_empty() {
        format!("{}/_api/web", cx.u("webUrl"))
    } else {
        format!("{}/_api/web/{sel}/views", cx.u("webUrl"))
    };
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn view_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let (who, list) = (
        first(cx, &["id", "title"]),
        first(cx, &["listId", "listTitle", "listUrl"]),
    );
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the view {who} from the list {list} in site {web}?"
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing view {who} from list {list}"));
    let vs = match ne(cx, "id") {
        Some(i) => format!("(guid'{}')", eqp(&i)),
        None => format!("/GetByTitle('{}')", eqp(&opt(cx, "title"))),
    };
    Request::post(format!(
        "{web}/_api/web/{}/views{vs}",
        selector(cx, "GetByTitle")
    ))
    .header("X-HTTP-Method", "DELETE")
    .header("If-Match", "*")
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn view_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let view = match ne(cx, "id") {
        Some(i) => format!("/views/GetById('{}')", eqp(&i)),
        None => format!("/views/GetByTitle('{}')", eqp(&opt(cx, "title"))),
    };
    let d = crate::sp::digest(&web).await?;
    const EXCLUDE: [&str; 9] = [
        "webUrl",
        "listId",
        "listTitle",
        "listUrl",
        "id",
        "title",
        "debug",
        "verbose",
        "output",
    ];
    let payload: Obj = cx
        .opts
        .iter()
        .filter(|(k, _)| !EXCLUDE.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    Request::patch(format!(
        "{web}/_api/web/{}{view}",
        selector(cx, "GetByTitle")
    ))
    .header("X-RequestDigest", d)
    .header("content-type", "application/json;odata=nometadata")
    .spo_json()
    .json(Value::Object(payload))
    .send()
    .await?;
    Ok(())
}

// ---- webhooks ------------------------------------------------------------------

fn subs(cx: &Cx, id: bool) -> String {
    let sel = selector(cx, "GetByTitle");
    if sel.is_empty() {
        return format!("{}/_api/web", cx.u("webUrl"));
    }
    let tail = if id {
        format!("('{}')", eqp(&opt(cx, "id")))
    } else {
        String::new()
    };
    format!("{}/_api/web/{sel}/Subscriptions{tail}", cx.u("webUrl"))
}

pub async fn webhook_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Adding webhook to list {} located at site {}...",
        first(cx, &["listId", "listTitle", "listUrl"]),
        cx.u("webUrl")
    ));
    let resource = ["listId", "listTitle", "listUrl"]
        .iter()
        .find_map(|k| cx.val(k).filter(|v| truthy(v)).cloned());
    let exp = match ne(cx, "expirationDateTime") {
        Some(e) => match time::parse_js_date(&e) {
            Some(d) => time::to_iso(d),
            None => return Err(Error::js("Invalid time value")),
        },
        None => time::to_iso(max_expiration()),
    };
    let mut b = obj! { "resource": ?resource, "notificationUrl": ?cx.val("notificationUrl").cloned(), "expirationDateTime": exp };
    if let Some(c) = ne(cx, "clientState") {
        b["clientState"] = Value::from(c);
    }
    cx.log(
        Request::post(subs(cx, false))
            .spo_json()
            .json(b)
            .data()
            .await?,
    )
}

pub async fn webhook_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information for webhook {} belonging to list {} in site at {}...",
        opt(cx, "id"),
        first(cx, &["listId", "listTitle", "listUrl"]),
        cx.u("webUrl")
    ));
    match Request::get(subs(cx, true)).spo_json().data().await {
        Ok(r) => cx.log(r),
        Err(e) => {
            cx.verbose_log("Specified webhook not found");
            Err(e)
        }
    }
}

pub async fn webhook_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving webhook information for list {} in site at {}...",
        first(cx, &["listTitle", "listId", "listUrl"]),
        cx.u("webUrl")
    ));
    let mut r = odata::get_all_items(&subs(cx, false)).await?;
    for w in r.iter_mut() {
        let c = w
            .get("clientState")
            .filter(|v| truthy(v))
            .cloned()
            .unwrap_or(Value::from(""));
        if let Some(o) = w.as_object_mut() {
            o.insert("clientState".into(), c);
        }
    }
    cx.log(Value::Array(r))
}

pub async fn webhook_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let id = opt(cx, "id");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove webhook {id} from list {} located at site {web}?",
            first(cx, &["listTitle", "listId", "listUrl"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Webhook {id} is about to be removed from list {} located at site {web}...",
        first(cx, &["listId", "listUrl"])
    ));
    Request::delete(subs(cx, true)).spo_json().send().await?;
    Ok(())
}

pub async fn webhook_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating webhook {} belonging to list {} located at site {}...",
        opt(cx, "id"),
        first(cx, &["listId", "listTitle", "listUrl"]),
        cx.u("webUrl")
    ));
    let b = obj! { "notificationUrl": ?cx.val("notificationUrl").cloned(), "expirationDateTime": ?cx.val("expirationDateTime").cloned(), "clientState": ?cx.val("clientState").cloned() };
    Request::patch(subs(cx, true))
        .spo_json()
        .json(b)
        .send()
        .await?;
    Ok(())
}

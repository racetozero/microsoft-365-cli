//! `purview *`.

use chrono::{DateTime, Duration, Local, Utc};
use m365_core::prelude::*;
use m365_core::time::to_iso;

const MANAGE: &str = "https://manage.office.com";

/// `date.setDate(date.getDate() + n)`: whole days in local time.
fn add_local_days(d: DateTime<Utc>, n: i64) -> DateTime<Utc> {
    let local = d.with_timezone(&Local).naive_local() + Duration::days(n);
    local
        .and_local_timezone(Local)
        .earliest()
        .map(|l| l.with_timezone(&Utc))
        .unwrap_or(d + Duration::days(n))
}

pub fn auditlog_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let limit = add_local_days(Utc::now(), -7) - Duration::hours(1);
    check(m365_core::time::parse_js_date(&to_js_string(v)).is_some_and(|d| d >= limit))
}

pub async fn auditlog_list(cx: &mut Cx) -> Result<()> {
    let now = Utc::now();
    let parse = |k: &str| cx.s(k).and_then(|v| m365_core::time::parse_js_date(&v));
    let start = parse("startTime").unwrap_or_else(|| add_local_days(now, -1));
    let end = parse("endTime").unwrap_or(now);
    let ct = cx.u("contentType");
    cx.verbose_log(&format!(
        "Getting audit logs for content type '{ct}' within a time frame from '{}' to '{}'.",
        to_iso(start),
        to_iso(end)
    ));
    let tenant = auth::jwt::tenant_id(&auth::default_access_token().unwrap_or_default());
    let ctv = if ct == "DLP" {
        "DLP.All".to_string()
    } else {
        format!("Audit.{ct}")
    };
    let subs = Request::get(format!(
        "{MANAGE}/api/v1.0/{tenant}/activity/feed/subscriptions/list"
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let active = subs
        .as_array()
        .ok_or_else(|| Error::js("subscriptions.some is not a function"))?
        .iter()
        .any(|s| {
            s.get("contentType") == Some(&Value::from(ctv.as_str()))
                && s.get("status") == Some(&Value::from("enabled"))
        });
    if !active {
        let r = Request::post(format!(
            "{MANAGE}/api/v1.0/{tenant}/activity/feed/subscriptions/start?contentType={ctv}"
        ))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
        if s(&r, "status") != "enabled" {
            return Err(Error::msg(format!("Unable to start subscription '{ctv}'")));
        }
    }
    cx.verbose_log(&format!("'{ct}' subscription is active."));
    let mut uris: Vec<String> = Vec::new();
    let mut t = start;
    while t < end {
        let batch_end = t + std::cmp::min(end - t, Duration::days(1));
        cx.verbose_log(&format!(
            "Get content URIs for date range from '{}' to '{}'.",
            to_iso(t),
            to_iso(batch_end)
        ));
        let mut url = Some(format!(
            "{MANAGE}/api/v1.0/{tenant}/activity/feed/subscriptions/content?contentType={ctv}&startTime={}&endTime={}",
            to_iso(t),
            to_iso(batch_end)
        ));
        while let Some(u0) = url {
            let r = Request::get(u0)
                .accept("application/json")
                .json_response()
                .send()
                .await?;
            let data = r.data();
            let list = data
                .as_array()
                .ok_or_else(|| Error::js("response.data.map is not a function"))?;
            uris.extend(list.iter().map(|d| u(d, "contentUri")));
            url = r
                .header("nextpageuri")
                .filter(|s| !s.is_empty())
                .map(String::from);
        }
        t = add_local_days(t, 1);
    }
    cx.verbose_log(&format!("Get content from {} content URIs.", uris.len()));
    let mut logs: Vec<Value> = Vec::new();
    for (i, chunk) in uris.chunks(30).enumerate() {
        let done = i * 30;
        cx.verbose_log(&format!(
            "Retrieving content from next {} content URIs. Progress: {}%",
            chunk.len(),
            (done as f64 / uris.len() as f64 * 100.0).round()
        ));
        for uri in chunk {
            let r = Request::get(uri.clone())
                .accept("application/json")
                .json_response()
                .data()
                .await?;
            match r {
                Value::Array(a) => logs.extend(a),
                other => logs.push(other),
            }
        }
    }
    logs.sort_by(|a, b| {
        let (x, y) = (
            a.get("CreationTime").map(to_js_string).unwrap_or_default(),
            b.get("CreationTime").map(to_js_string).unwrap_or_default(),
        );
        x.encode_utf16().cmp(y.encode_utf16())
    });
    cx.log(Value::Array(logs))
}

pub async fn retentionevent_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Creating retention event...");
    let mut queries = Vec::new();
    if let Some(a) = cx.s("assetIds") {
        queries.push(obj! { "queryType": "files", "query": a });
    }
    if let Some(k) = cx.s("keywords") {
        queries.push(obj! { "queryType": "messages", "query": k });
    }
    // resolved outside the command's try/catch
    let type_id = match cx.s("eventTypeId") {
        Some(i) => i,
        None => {
            let n = cx.u("eventTypeName");
            cx.verbose_log(&format!("Retrieving the event type id for event type {n}"));
            let items = odata::get_all_items(&format!(
                "{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes"
            ))
            .await
            .map_err(Error::uncaught)?;
            match items.iter().find(|x| s(x, "displayName") == n) {
                Some(t) => u(t, "id"),
                None => {
                    return Err(Error::msg(format!(
                        "The specified event type '{n}' does not exist."
                    )));
                }
            }
        }
    };
    let data = obj! {
        "retentionEventType@odata.bind": format!("https://graph.microsoft.com/v1.0/security/triggerTypes/retentionEventTypes/{type_id}"),
        "displayName": ?cx.val("displayName").cloned(),
        "description": ?cx.val("description").cloned(),
        "eventQueries": queries,
        "eventTriggerDateTime": ?cx.val("triggerDateTime").cloned(),
    };
    cx.log(
        Request::post(format!("{GRAPH}/v1.0/security/triggers/retentionEvents"))
            .graph_json()
            .json(data)
            .data()
            .await?,
    )
}

pub async fn retentionevent_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving retention event with id {id}"));
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/security/triggers/retentionEvents/{id}"
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn retentionevent_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving Purview retention events");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/security/triggers/retentionEvents")).await?,
    ))
}

pub async fn retentionevent_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the retention event {id}?"),
    )? {
        return Ok(());
    }
    Request::delete(format!(
        "{GRAPH}/v1.0/security/triggers/retentionEvents/{id}"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn retentioneventtype_add(cx: &mut Cx) -> Result<()> {
    let r = Request::post(format!("{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes"))
        .graph_json()
        .json(obj! { "displayName": ?cx.val("displayName").cloned(), "description": ?cx.val("description").cloned() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn retentioneventtype_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving retention event type with id {id}"));
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes/{id}"
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn retentioneventtype_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes"
        ))
        .await?,
    ))
}

pub async fn retentioneventtype_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the retention event type with id {id}?"),
    )? {
        return Ok(());
    }
    Request::delete(format!(
        "{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes/{id}"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn retentioneventtype_set(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if cx.verbose() {
        cx.log(Value::from(format!(
            "Updating retention event type with id {id}"
        )))?;
    }
    Request::patch(format!(
        "{GRAPH}/v1.0/security/triggerTypes/retentionEventTypes/{id}"
    ))
    .accept("application/json")
    .json_response()
    .json(obj! { "description": ?cx.val("description").cloned() })
    .send()
    .await?;
    Ok(())
}

pub async fn retentionlabel_add(cx: &mut Cx) -> Result<()> {
    let mut body = obj! {
        "displayName": ?cx.val("displayName").cloned(),
        "behaviorDuringRetentionPeriod": ?cx.val("behaviorDuringRetentionPeriod").cloned(),
        "actionAfterRetentionPeriod": ?cx.val("actionAfterRetentionPeriod").cloned(),
        "retentionTrigger": cx.s("retentionTrigger").unwrap_or_else(|| "dateLabeled".into()),
        "retentionDuration": obj! { "@odata.type": "#microsoft.graph.security.retentionDurationInDays", "days": json::num_value(json::js_number(&cx.u("retentionDuration"))) },
        "defaultRecordBehavior": cx.s("defaultRecordBehavior").unwrap_or_else(|| "startLocked".into()),
    };
    let o = body.as_object_mut().expect("object");
    if cx.str("retentionTrigger").as_deref() == Some("dateOfEvent") {
        // resolved outside the command's try/catch
        let id = match cx.s("eventTypeId") {
            Some(i) => i,
            None => {
                let n = cx.u("eventTypeName");
                cx.verbose_log(&format!("Retrieving the event type id for event type {n}"));
                let items = odata::get_all_items(&format!(
                    "{GRAPH}/beta/security/triggerTypes/retentionEventTypes"
                ))
                .await
                .map_err(Error::uncaught)?;
                match items.iter().find(|x| s(x, "displayName") == n) {
                    Some(t) => u(t, "id"),
                    None => {
                        return Err(Error::msg(format!(
                            "The specified retention event type '{n}' does not exist."
                        )));
                    }
                }
            }
        };
        o.insert(
            "retentionEventType@odata.bind".into(),
            Value::from(format!(
                "https://graph.microsoft.com/beta/security/triggerTypes/retentionEventTypes/{id}"
            )),
        );
    }
    for (k, msg) in [
        ("descriptionForAdmins", ""),
        ("descriptionForUsers", ""),
        ("labelToBeApplied", "..."),
    ] {
        if let Some(v) = cx.s(k) {
            cx.verbose_log(&format!("Using '{v}' as {k}{msg}"));
            o.insert(k.into(), Value::from(v));
        }
    }
    cx.log(
        Request::post(format!("{GRAPH}/beta/security/labels/retentionLabels"))
            .graph_json()
            .json(body)
            .data()
            .await?,
    )
}

pub async fn retentionlabel_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving retention label with id {id}"));
    cx.log(
        Request::get(format!("{GRAPH}/beta/security/labels/retentionLabels/{id}"))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn retentionlabel_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/beta/security/labels/retentionLabels")).await?,
    ))
}

pub async fn retentionlabel_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the retention label {id}?"),
    )? {
        return Ok(());
    }
    Request::delete(format!("{GRAPH}/beta/security/labels/retentionLabels/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn retentionlabel_set(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if cx.verbose() {
        cx.log(Value::from(format!(
            "Starting to update retention label with id {id}"
        )))?;
    }
    let mut body = Obj::new();
    for (k, v) in &cx.opts {
        if !["debug", "verbose", "output", "id", "retentionDuration"].contains(&k.as_str()) {
            body.insert(k.clone(), Value::from(to_js_string(v)));
        }
    }
    if let Some(d) = cx.s("retentionDuration") {
        body.insert("retentionDuration".into(), obj! { "@odata.type": "microsoft.graph.security.retentionDurationInDays", "days": json::num_value(json::js_number(&d)) });
    }
    Request::patch(format!("{GRAPH}/beta/security/labels/retentionLabels/{id}"))
        .accept("application/json")
        .json_response()
        .json(Value::Object(body))
        .send()
        .await?;
    Ok(())
}

fn app_only_needs_user(cx: &Cx) -> Result<()> {
    if auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true)
        && cx.s("userId").is_none()
        && cx.s("userName").is_none()
    {
        return Err(Error::msg(
            "Specify at least 'userId' or 'userName' when using application permissions.",
        ));
    }
    Ok(())
}

fn ip_base(cx: &Cx) -> String {
    match cx.s("userId").or_else(|| cx.s("userName")) {
        Some(u) => format!("{GRAPH}/beta/users/{u}/security/informationProtection"),
        None => format!("{GRAPH}/beta/me/security/informationProtection"),
    }
}

pub async fn sensitivitylabel_get(cx: &mut Cx) -> Result<()> {
    app_only_needs_user(cx)?;
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving sensitivity label with id {id}"));
    cx.log(
        Request::get(format!("{}/sensitivityLabels/{id}", ip_base(cx)))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn sensitivitylabel_list(cx: &mut Cx) -> Result<()> {
    app_only_needs_user(cx)?;
    cx.log(Value::Array(
        odata::get_all_items(&format!("{}/sensitivityLabels", ip_base(cx))).await?,
    ))
}

pub async fn sensitivitylabel_policysettings_list(cx: &mut Cx) -> Result<()> {
    app_only_needs_user(cx)?;
    cx.log(
        Request::get(format!("{}/labelPolicySettings", ip_base(cx)))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn threatassessment_add(cx: &mut Cx) -> Result<()> {
    use base64::Engine;
    if auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true) {
        return Err(Error::msg(
            "This command currently does not support app only permissions.",
        ));
    }
    let (ty, ea, cat) = (cx.u("type"), cx.u("expectedAssessment"), cx.u("category"));
    cx.verbose_log(&format!(
        "Adding threat assessment of type {ty} with expected assessment {ea} and category {cat}"
    ));
    let (content, name) = match cx.s("path") {
        Some(p) => {
            let b =
                std::fs::read(&p).map_err(|e| Error::js(util::node_fs_error(&e, "open", &p)))?;
            let n = std::path::Path::new(&p)
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();
            (
                Some(base64::engine::general_purpose::STANDARD.encode(b)),
                Some(n),
            )
        }
        None => (None, None),
    };
    let mut body = obj! { "expectedAssessment": ea, "category": cat, "url": ?cx.val("url").cloned(), "contentData": ?content, "fileName": ?name };
    let t = match ty.as_str() {
        "file" => Some("#microsoft.graph.fileAssessmentRequest"),
        "url" => Some("#microsoft.graph.urlAssessmentRequest"),
        _ => None,
    };
    if let (Some(t), Some(o)) = (t, body.as_object_mut()) {
        o.insert("@odata.type".into(), Value::from(t));
    }
    cx.log(
        Request::post(format!(
            "{GRAPH}/v1.0/informationProtection/threatAssessmentRequests"
        ))
        .graph_json()
        .json(body)
        .data()
        .await?,
    )
}

pub async fn threatassessment_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving threat assessment with id {id}"));
    let q = if cx.bool("withResults") {
        "?$expand=results"
    } else {
        ""
    };
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/informationProtection/threatAssessmentRequests/{id}{q}"
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn threatassessment_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving a list of threat assessments");
    let ty = cx.str("type");
    let filter = match ty.as_deref() {
        None => String::new(),
        Some("emailFile") => "?$filter=contentType eq 'mail'".into(),
        Some(t) => format!("?$filter=contentType eq '{t}'"),
    };
    let items = odata::get_all_items_meta(
        &format!("{GRAPH}/v1.0/informationProtection/threatAssessmentRequests{filter}"),
        "minimal",
    )
    .await?;
    let keep = |i: &Value| match ty.as_deref() {
        Some("mail") => s(i, "@odata.type") == "#microsoft.graph.mailAssessmentRequest",
        Some("emailFile") => s(i, "@odata.type") == "#microsoft.graph.emailFileAssessmentRequest",
        _ => true,
    };
    let out: Vec<Value> = items
        .into_iter()
        .filter(keep)
        .map(|mut i| {
            let t = match s(&i, "@odata.type") {
                "#microsoft.graph.mailAssessmentRequest" => "mail",
                "#microsoft.graph.fileAssessmentRequest" => "file",
                "#microsoft.graph.emailFileAssessmentRequest" => "emailFile",
                "#microsoft.graph.urlAssessmentRequest" => "url",
                _ => "Unknown",
            };
            if let Some(o) = i.as_object_mut() {
                o.insert("type".into(), Value::from(t));
                o.shift_remove("@odata.type");
            }
            i
        })
        .collect();
    cx.log(Value::Array(out))
}

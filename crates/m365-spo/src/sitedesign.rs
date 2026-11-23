//! `spo sitedesign *`.

use crate::sp;
use crate::sp::ne;
use m365_core::prelude::*;
use m365_core::ut::spo;

const SSU: &str = "_api/Microsoft.Sharepoint.Utilities.WebTemplateExtensions.SiteScriptUtility";

fn json_ct(r: Request) -> Request {
    r.header("content-type", "application/json;charset=utf-8")
        .spo_json()
}

fn value_of(cx: &Cx, r: &Value) -> Result<()> {
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub fn scripts_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    if i == 1 {
        let s = to_js_string(v);
        return check(!s.trim().is_empty() && json::js_number(&s).is_finite());
    }
    let s = to_js_string(v);
    let bad: Vec<&str> = s
        .split(',')
        .map(str::trim)
        .filter(|x| !m365_core::validation::is_valid_guid(x))
        .collect();
    if bad.is_empty() {
        Ok(())
    } else {
        Err(Some(format!(
            "The following values are not valid GUIDs: {}",
            bad.join(", ")
        )))
    }
}

fn template(t: &str) -> &'static str {
    if t == "TeamSite" { "64" } else { "68" }
}

fn scripts(cx: &Cx) -> Vec<Value> {
    cx.u("siteScripts")
        .split(',')
        .map(|i| Value::from(i.trim()))
        .collect()
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    let mut info = obj! { "Title": ?cx.val("title").cloned(), "WebTemplate": template(&cx.u("webTemplate")), "SiteScriptIds": scripts(cx) };
    for (k, o) in [
        ("Description", "description"),
        ("PreviewImageUrl", "previewImageUrl"),
        ("PreviewImageAltText", "previewImageAltText"),
        ("ThumbnailUrl", "thumbnailUrl"),
    ] {
        if let Some(v) = ne(cx, o) {
            info[k] = Value::from(v);
        }
    }
    if cx.bool("isDefault") {
        info["IsDefault"] = Value::Bool(true);
    }
    let r = json_ct(
        Request::post(format!("{spo_url}/{SSU}.CreateSiteDesign")).header("X-RequestDigest", d),
    )
    .json(obj! { "info": info })
    .data()
    .await?;
    cx.log(r)
}

pub async fn apply(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let m = if cx.bool("asTask") {
        "AddSiteDesignTask"
    } else {
        "ApplySiteDesign"
    };
    let r = json_ct(Request::post(format!("{spo_url}/{SSU}.{m}")))
        .json(obj! { "siteDesignId": ?cx.val("id").cloned(), "webUrl": ?cx.val("webUrl").cloned() })
        .data()
        .await?;
    match r.get("value").filter(|v| truthy(v)) {
        Some(v) => cx.log(v.clone()),
        None => cx.log(r),
    }
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let id = match ne(cx, "id") {
        Some(i) => i,
        None => {
            let r = Request::post(format!("{spo_url}/{SSU}.GetSiteDesigns"))
                .spo_json()
                .data()
                .await?;
            let title = cx.val("title").cloned();
            let m: Vec<Value> = need_arr(&r, "value", "filter")?
                .iter()
                .filter(|x| x.get("Title") == title.as_ref())
                .cloned()
                .collect();
            u(
                &m365_core::runner::pick_one(
                    &m,
                    "Id",
                    Error::msg("The specified site design does not exist"),
                    &format!(
                        "Multiple site designs with title '{}' found.",
                        cx.u("title")
                    ),
                )?,
                "Id",
            )
        }
    };
    cx.log(
        json_ct(Request::post(format!(
            "{spo_url}/{SSU}.GetSiteDesignMetadata"
        )))
        .json(obj! { "id": id })
        .data()
        .await?,
    )
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let r = Request::post(format!("{spo_url}/{SSU}.GetSiteDesigns"))
        .spo_json()
        .data()
        .await?;
    value_of(cx, &r)
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the site design {id}?"),
    )? {
        return Ok(());
    }
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    json_ct(
        Request::post(format!("{spo_url}/{SSU}.DeleteSiteDesign")).header("X-RequestDigest", d),
    )
    .json(obj! { "id": id })
    .send()
    .await?;
    Ok(())
}

fn principals(cx: &Cx) -> Vec<Value> {
    cx.u("principals")
        .split(',')
        .map(|p| Value::from(p.trim()))
        .collect()
}

pub async fn rights_grant(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    json_ct(Request::post(format!("{spo_url}/{SSU}.GrantSiteDesignRights")).header("X-RequestDigest", d))
        .json(obj! { "id": ?cx.val("siteDesignId").cloned(), "principalNames": principals(cx), "grantedRights": "1" })
        .send()
        .await?;
    Ok(())
}

pub async fn rights_list(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    let r = json_ct(
        Request::post(format!("{spo_url}/{SSU}.GetSiteDesignRights")).header("X-RequestDigest", d),
    )
    .json(obj! { "id": ?cx.val("siteDesignId").cloned() })
    .data()
    .await?;
    let mut v = need_arr(&r, "value", "map")?.to_vec();
    for p in v.iter_mut() {
        if let Some(o) = p.as_object_mut() {
            if o.get("Rights").and_then(Value::as_str) == Some("1") {
                o.insert("Rights".into(), Value::from("View"));
            } else if !o.contains_key("Rights") {
                // `p.Rights = p.Rights` creates the key as undefined: not serialized
            }
        }
    }
    cx.log(Value::Array(v))
}

pub async fn rights_revoke(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to revoke access to site design {} from the specified users?",
            cx.u("siteDesignId")
        ),
    )? {
        return Ok(());
    }
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    json_ct(
        Request::post(format!("{spo_url}/{SSU}.RevokeSiteDesignRights"))
            .header("X-RequestDigest", d),
    )
    .json(obj! { "id": ?cx.val("siteDesignId").cloned(), "principalNames": principals(cx) })
    .send()
    .await?;
    Ok(())
}

pub async fn run_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let data = obj! { "siteDesignId": ?ne(cx, "siteDesignId") };
    let r = Request::post(format!("{web}/{SSU}.GetSiteDesignRun"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(data)
        .data()
        .await?;
    if cx.str("output").as_deref() != Some("json") {
        let mut v = need_arr(&r, "value", "forEach")?.to_vec();
        for d in v.iter_mut() {
            let st = d
                .get("StartTime")
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into());
            let ms = json::parse_int(&st).map(|n| n as f64).unwrap_or(f64::NAN);
            if let Some(o) = d.as_object_mut() {
                o.insert(
                    "StartTime".into(),
                    Value::from(m365_core::time::locale_string_ms(ms)),
                );
            }
        }
        return cx.log(Value::Array(v));
    }
    value_of(cx, &r)
}

pub async fn run_status_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let r = Request::post(format!("{web}/{SSU}.GetSiteDesignRunStatus"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(obj! { "runId": ?cx.val("runId").cloned() })
        .data()
        .await?;
    value_of(cx, &r)
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let mut info = obj! { "Id": ?cx.val("id").cloned() };
    for (k, o) in [("Title", "title"), ("Description", "description")] {
        if let Some(v) = ne(cx, o) {
            info[k] = Value::from(v);
        }
    }
    if ne(cx, "siteScripts").is_some() {
        info["SiteScriptIds"] = Value::Array(scripts(cx));
    }
    for (k, o) in [
        ("PreviewImageUrl", "previewImageUrl"),
        ("PreviewImageAltText", "previewImageAltText"),
        ("ThumbnailUrl", "thumbnailUrl"),
    ] {
        if let Some(v) = ne(cx, o) {
            info[k] = Value::from(v);
        }
    }
    if let Some(t) = ne(cx, "webTemplate") {
        info["WebTemplate"] = Value::from(template(&t));
    }
    if let Some(v) = ne(cx, "version") {
        info["Version"] = json::num_value(json::js_number(&v));
    }
    if let Some(d) = cx.val("isDefault") {
        info["IsDefault"] = d.clone();
    }
    let r = json_ct(Request::post(format!("{spo_url}/{SSU}.UpdateSiteDesign")))
        .header("responseType", "json")
        .json(obj! { "updateInfo": info })
        .data()
        .await?;
    cx.log(r)
}

pub async fn task_get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let r = Request::post(format!("{spo_url}/{SSU}.GetSiteDesignTask"))
        .spo_json()
        .json(obj! { "taskId": ?cx.val("id").cloned() })
        .data()
        .await?;
    if !r.get("odata.null").is_some_and(truthy) {
        cx.log(r)?;
    }
    Ok(())
}

pub async fn task_list(cx: &mut Cx) -> Result<()> {
    let r = Request::post(format!("{}/{SSU}.GetSiteDesignTasks", cx.u("webUrl")))
        .spo_json()
        .data()
        .await?;
    value_of(cx, &r)
}

pub async fn task_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the site design task {id}?"),
    )? {
        return Ok(());
    }
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    Request::post(format!("{spo_url}/{SSU}.RemoveSiteDesignTask"))
        .spo_json()
        .json(obj! { "taskId": id })
        .send()
        .await?;
    Ok(())
}

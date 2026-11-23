//! `spo page *`: modern pages. The HTML canvas model lives in
//! `clientsidepages.rs`; commands that edit the JSON canvas
//! (`CanvasContent1` as JSON) work on plain values here.

use crate::clientsidepages::{self as csp, ClientSidePage, Part, Section};
use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::spo;
use m365_core::ut::url as urlutil;
use m365_core::{auth, term, time};

const MODERN_PAGE_APP: &str = "b6917cb1-93a0-4b97-a84d-7cf49975d4ec";
const DEFAULT_CANVAS: &str = r#"[{"controlType":0,"pageSettingsSlice":{"isDefaultDescription":true,"isDefaultThumbnail":true}}]"#;
const BANNER_WEB_PART_ID: &str = "cbe7b0a9-3504-44dd-a3a3-0e5cacd07788";
/// A JavaScript `undefined` property (see [`strip_undefined`]).
const UNDEF: &str = json::UNDEFINED;

const STANDARD_WEB_PARTS: [(&str, &str); 27] = [
    ("ContentRollup", "daf0b71c-6de8-4ef7-b511-faae7c388708"),
    ("BingMap", "e377ea37-9047-43b9-8cdb-a761be2f8e09"),
    ("ContentEmbed", "490d7c76-1824-45b2-9de3-676421c997fa"),
    ("DocumentEmbed", "b7dd04e1-19ce-4b24-9132-b60a1c2b910d"),
    ("Image", "d1d91016-032f-456d-98a4-721247c305e8"),
    ("ImageGallery", "af8be689-990e-492a-81f7-ba3e4cd3ed9c"),
    ("LinkPreview", "6410b3b6-d440-4663-8744-378976dc041e"),
    ("NewsFeed", "0ef418ba-5d19-4ade-9db0-b339873291d0"),
    ("NewsReel", "a5df8fdf-b508-4b66-98a6-d83bc2597f63"),
    ("NewsReel", "8c88f208-6c77-4bdb-86a0-0c47b4316588"),
    ("PowerBIReportEmbed", "58fcd18b-e1af-4b0a-b23b-422c2c52d5a2"),
    ("QuickChart", "91a50c94-865f-4f5c-8b4e-e49659e69772"),
    ("SiteActivity", "eb95c819-ab8f-4689-bd03-0c2d65d47b1f"),
    ("VideoEmbed", "275c0095-a77e-4f6d-a2a0-6a7626911518"),
    ("YammerEmbed", "31e9537e-f9dc-40a4-8834-0e3b7df418bc"),
    ("Events", "20745d7d-8581-4a6c-bf26-68279bc123fc"),
    ("GroupCalendar", "6676088b-e28e-4a90-b9cb-d0d0303cd2eb"),
    ("Hero", "c4bd7b2f-7b6e-4599-8485-16504575f590"),
    ("List", "f92bf067-bc19-489e-a556-7fe95f508720"),
    ("PageTitle", "cbe7b0a9-3504-44dd-a3a3-0e5cacd07788"),
    ("People", "7f718435-ee4d-431c-bdbf-9c4ff326f46e"),
    ("QuickLinks", "c70391ea-0b10-4ee9-b2b4-006d3fcad0cd"),
    (
        "CustomMessageRegion",
        "71c19a43-d08c-4178-8218-4df8554c0b0e",
    ),
    ("Divider", "2161a1c6-db61-4731-b97c-3cdb303f7cbb"),
    ("MicrosoftForms", "b19b3b9e-8d13-4fec-a93c-401a091c0707"),
    ("Spacer", "8654b779-4886-46d4-8ffb-b5ed960ee986"),
    ("ClientWebPart", "243166f5-4dc3-4fe2-9df2-a7971b546a0a"),
];

fn standard_web_part_id(name: &str) -> Option<&'static str> {
    STANDARD_WEB_PARTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, id)| *id)
}

/// `standardWebPart` refine of `spo page clientsidewebpart add`.
pub fn clientsidewebpart_add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    match i {
        0 => check(standard_web_part_id(&to_js_string(v)).is_some()),
        _ => Ok(()),
    }
}

// ---- helpers -------------------------------------------------------------------

/// JavaScript `a === b` for values that may be `undefined`.
fn seq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(Value::Number(x)), Some(Value::Number(y))) => x.as_f64() == y.as_f64(),
        (Some(x @ (Value::String(_) | Value::Bool(_) | Value::Null)), Some(y)) => x == y,
        _ => false,
    }
}

fn num_eq(v: Option<&Value>, n: f64) -> bool {
    v.and_then(Value::as_f64) == Some(n)
}

fn jsu(v: Option<&Value>) -> String {
    v.map(to_js_string).unwrap_or_else(|| "undefined".into())
}

fn jnum(n: f64) -> String {
    json::number_to_string(n)
}

/// `JSON.parse(x)` where `x` is any value (`String(x)` first).
fn parse_value(v: Option<&Value>) -> Result<Value> {
    json::jparse(&jsu(v))
}

/// `name` with `.aspx` appended when it does not contain it.
fn with_aspx(name: &str) -> String {
    if name.contains(".aspx") {
        name.to_string()
    } else {
        format!("{name}.aspx")
    }
}

/// `urlUtil.removeLeadingSlashes(name)` + `.aspx` unless it ends with one.
fn leading_trimmed_aspx(name: &str) -> String {
    let p = urlutil::remove_leading_slashes(name);
    if p.to_lowercase().ends_with(".aspx") {
        p
    } else {
        format!("{p}.aspx")
    }
}

fn by_url(web: &str, page: &str) -> String {
    format!(
        "{web}/_api/sitepages/pages/GetByUrl('sitepages/{}')",
        eqp(page)
    )
}

fn not_function(what: &str, method: &str) -> Error {
    Error::js(format!("{what}.{method} is not a function"))
}

fn as_arr<'a>(v: &'a Value, name: &str, method: &str) -> Result<&'a Vec<Value>> {
    match v {
        Value::Array(a) => Ok(a),
        _ => Err(not_function(name, method)),
    }
}

fn as_arr_mut<'a>(v: &'a mut Value, name: &str, method: &str) -> Result<&'a mut Vec<Value>> {
    match v {
        Value::Array(a) => Ok(a),
        _ => Err(not_function(name, method)),
    }
}

/// Assigns `o[k] = v` with `undefined` keeping the key (see [`UNDEF`]).
fn assign(o: &mut Value, k: &str, v: Option<Value>) {
    if let Some(m) = o.as_object_mut() {
        match v {
            Some(v) => {
                m.insert(k.into(), v);
            }
            None => {
                m.insert(k.into(), Value::from(UNDEF));
            }
        }
    }
}

/// Reads `o[k]`, treating [`UNDEF`] as missing.
fn get<'a>(o: &'a Value, k: &str) -> Option<&'a Value> {
    o.get(k).filter(|v| v.as_str() != Some(UNDEF))
}

/// Drops [`UNDEF`] properties the way `JSON.stringify` drops `undefined`.
fn strip_undefined(v: &mut Value) {
    match v {
        Value::Object(o) => {
            o.retain(|_, v| v.as_str() != Some(UNDEF));
            o.values_mut().for_each(strip_undefined);
        }
        Value::Array(a) => a.iter_mut().for_each(|v| {
            if v.as_str() == Some(UNDEF) {
                *v = Value::Null;
            } else {
                strip_undefined(v);
            }
        }),
        _ => {}
    }
}

fn stringify_js(v: &Value) -> String {
    let mut v = v.clone();
    strip_undefined(&mut v);
    stringify(&v)
}

/// `Page.getPage`.
async fn get_page(cx: &Cx, name: &str, web: &str) -> Result<ClientSidePage> {
    cx.verbose_log("Retrieving information about the page...");
    let page_name = with_aspx(name);
    let res = Request::get(format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}/SitePages/{}')?$expand=ListItemAllFields/ClientSideApplicationId",
        urlutil::get_server_relative_site_url(web),
        eqp(&page_name)
    ))
    .header("content-type", "application/json;charset=utf-8")
    .spo_json()
    .data()
    .await?;
    if dig(&res, &["ListItemAllFields", "ClientSideApplicationId"])?.and_then(Value::as_str)
        != Some(MODERN_PAGE_APP)
    {
        return Err(Error::Thrown(Value::from(format!(
            "Page {name} is not a modern page."
        ))));
    }
    match res
        .get("ListItemAllFields")
        .and_then(|l| l.get("CanvasContent1"))
    {
        Some(Value::String(html)) => ClientSidePage::from_html(html),
        _ => Ok(ClientSidePage::default()),
    }
}

/// `Page.checkout`.
async fn checkout(cx: &Cx, name: &str, web: &str) -> Result<Value> {
    if cx.verbose() {
        cx.log(Value::from(format!("Checking out {name} page...")))?;
    }
    let page = Request::post(format!("{}/checkoutpage", by_url(web, &with_aspx(name))))
        .spo_json()
        .data()
        .await?;
    if !truthy(&page) {
        return Err(Error::Thrown(Value::from(format!(
            "Page {name} information not retrieved with the checkout"
        ))));
    }
    if cx.verbose() {
        cx.log(Value::from(format!("Page {name} is now checked out")))?;
    }
    Ok(page)
}

/// `Page.publishPage`.
async fn publish_page(web: &str, page_name: &str) -> Result<()> {
    let file = format!(
        "{}/SitePages/{page_name}",
        urlutil::get_server_relative_site_url(web)
    );
    Request::post(format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/Publish()",
        eqp(&file)
    ))
    .accept("application/json;odata=nometadata")
    .send()
    .await?;
    Ok(())
}

/// `getFileAsListItemByUrl`.
async fn file_as_list_item(cx: &Cx, web: &str, url: &str) -> Result<Value> {
    cx.verbose_log(&format!("Getting the file properties with url {url}"));
    let path = urlutil::get_server_relative_path(web, url);
    let f = Request::get(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl=@f)?$expand=ListItemAllFields&@f='{}'", eqp(&path)))
        .spo_json()
        .data()
        .await?;
    Ok(dig(&f, &["ListItemAllFields"])?
        .cloned()
        .unwrap_or(Value::from(UNDEF)))
}

fn json_output(cx: &Cx) -> bool {
    cx.str("output").as_deref() != Some("text")
}

/// A `MERGE` request with the digest, as most page saves send it.
fn merge(url: String, digest: &str) -> Request {
    Request::post(url)
        .header("X-RequestDigest", digest)
        .header("X-HTTP-Method", "MERGE")
        .header("IF-MATCH", "*")
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
}

fn with_digest(url: String, digest: &str) -> Request {
    Request::post(url)
        .header("X-RequestDigest", digest)
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
}

/// `{ k: v }` for each truthy value, in order.
fn truthy_fields(fields: &[(&str, Option<&Value>)]) -> Value {
    let mut o = Obj::new();
    for (k, v) in fields {
        if let Some(v) = v.filter(|v| truthy(v)) {
            o.insert((*k).into(), v.clone());
        }
    }
    Value::Object(o)
}

/// `getControlTypeDisplayName(c.controlType || 0)`.
fn control_type_display(c: &Value) -> String {
    match c.get("controlType").filter(|v| truthy(v)) {
        Some(v) => csp::control_type_name(v),
        None => csp::control_type_name(&Value::from(0)),
    }
}

/// The `{ id, type, title, controlType, order, controlData }` of a control.
fn control_summary(c: &Value) -> Result<Value> {
    let order = dig(c, &["position", "sectionIndex"])?.cloned();
    Ok(obj! {
        "id": ?c.get("id").cloned(),
        "type": control_type_display(c),
        "title": ?c.get("webPartData").and_then(|w| w.get("title")).cloned(),
        "controlType": ?c.get("controlType").cloned(),
        "order": ?order,
        "controlData": c.clone(),
    })
}

/// `c.id?.toLowerCase()`.
fn lower_id(c: &Value) -> Result<Option<String>> {
    match c.get("id") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.to_lowercase())),
        Some(_) => Err(Error::js("c.id?.toLowerCase is not a function")),
    }
}

// ---- add / set ---------------------------------------------------------------

pub async fn add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let name = cx.u("name");
    let file_no_ext = name.replacen(".aspx", "", 1);
    let title =
        cx.s("title")
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| match name.find(".aspx") {
                Some(i) => name[..i].to_string(),
                None => name.clone(),
            });
    let description = cx
        .s("description")
        .filter(|d| !d.is_empty())
        .unwrap_or_default();
    let page_name = if name.ends_with(".aspx") {
        name.clone()
    } else {
        format!("{name}.aspx")
    };
    let list_srv = format!("{}/sitepages", urlutil::get_server_relative_site_url(&web));
    let file_srv = format!("{list_srv}/{page_name}");
    let promote = cx.s("promoteAs");
    let digest = u(&spo::get_request_digest(&web).await?, "FormDigestValue");
    let template = Request::post(format!("{web}/_api/sitepages/pages"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(obj! {
            "PageLayoutType": cx.s("layoutType").filter(|l| !l.is_empty()).unwrap_or_else(|| "Article".into()),
            "Name": page_name.as_str(),
            "PromotedState": if promote.as_deref() == Some("NewsPage") { 2 } else { 0 },
            "Title": title.as_str(),
        })
        .data()
        .await?;
    let item_id = u(&template, "UniqueId");
    let file = file_as_list_item(cx, &web, &file_srv).await?;
    if file.as_str() == Some(UNDEF) {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'Id')",
        ));
    }
    let list_item_id = jsu(get(&file, "Id"));
    let props = checkout(cx, &page_name, &web).await?;
    let mut page_id = Some(props.get("Id").cloned().unwrap_or(Value::from(UNDEF)));
    let mut banner = props.get("BannerImageUrl").cloned();
    let mut canvas = props.get("CanvasContent1").cloned();
    let mut layout = props.get("LayoutWebpartsContent").cloned();
    let comments_disabled = !cx.bool("commentsEnabled");
    match promote.as_deref() {
        Some("HomePage") => {
            merge(format!("{web}/_api/web/rootfolder"), &digest)
                .json(obj! { "WelcomePage": format!("SitePages/{page_name}") })
                .send()
                .await?;
        }
        Some("NewsPage") => {
            let list_url = urlutil::get_server_relative_path(&web, &list_srv);
            let request_url = format!("{web}/_api/web/GetList('{}')", eqp(&list_url));
            let mut p = Obj::new();
            p.insert("FirstPublishedDate".into(), Value::from(time::now_iso()));
            sp::system_update_list_item(cx, &request_url, &list_item_id, Some(&p), None).await?;
        }
        Some("Template") => {
            let t = Request::post(format!(
                "{web}/_api/SitePages/Pages({list_item_id})/SavePageAsTemplate"
            ))
            .header("X-RequestDigest", &digest)
            .header("content-type", "application/json;odata=nometadata")
            .header("X-HTTP-Method", "POST")
            .header("IF-MATCH", "*")
            .spo_json()
            .data()
            .await?;
            banner = t.get("BannerImageUrl").cloned();
            canvas = t.get("CanvasContent1").cloned();
            layout = t.get("LayoutWebpartsContent").cloned();
            page_id = Some(t.get("Id").cloned().unwrap_or(Value::from(UNDEF)));
            with_digest(format!("{web}/_api/web/getfilebyid('{}')/ListItemAllFields/SetCommentsDisabled({comments_disabled})", u(&t, "UniqueId")), &digest).send().await?;
        }
        _ => {}
    }
    let pid = page_id.as_ref().filter(|v| v.as_str() != Some(UNDEF));
    let pid_s = page_id
        .as_ref()
        .map(|v| {
            if v.as_str() == Some(UNDEF) {
                "undefined".into()
            } else {
                to_js_string(v)
            }
        })
        .unwrap_or_else(|| "null".into());
    merge(format!("{web}/_api/SitePages/Pages({pid_s})/SavePage"), &digest)
        .json(obj! { "BannerImageUrl": ?banner.clone(), "CanvasContent1": ?canvas.clone(), "LayoutWebpartsContent": ?layout.clone(), "Description": description.as_str() })
        .send()
        .await?;
    if promote.as_deref() == Some("Template") {
        merge(format!("{web}/_api/SitePages/Pages({pid_s})/SavePageAsDraft"), &digest)
            .json(obj! { "Title": file_no_ext.as_str(), "BannerImageUrl": ?banner.clone(), "CanvasContent1": ?canvas.clone(), "LayoutWebpartsContent": ?layout.clone(), "Description": description.as_str() })
            .send()
            .await?;
    }
    let comments = || {
        with_digest(
            format!(
                "{web}/_api/web/getfilebyid('{item_id}')/ListItemAllFields/SetCommentsDisabled({comments_disabled})"
            ),
            &digest,
        )
    };
    comments().send().await?;
    let last = if !cx.bool("publish") {
        if promote.as_deref() != Some("Template") && pid.is_some_and(truthy) {
            Request::post(format!("{web}/_api/SitePages/Pages({pid_s})/SavePageAsDraft"))
                .header("content-type", "application/json;odata=nometadata")
                .spo_json()
                .json(obj! { "Title": title.as_str(), "Description": description.as_str(), "BannerImageUrl": ?banner, "CanvasContent1": ?canvas, "LayoutWebpartsContent": ?layout })
        } else {
            // upstream posts the previous request (SetCommentsDisabled) again
            comments()
        }
    } else {
        let msg = eqp(&cx.s("publishMessage").unwrap_or_default()).replace('\'', "%39");
        with_digest(
            format!(
                "{web}/_api/web/getfilebyid('{item_id}')/CheckIn(comment=@a1,checkintype=@a2)?@a1='{msg}'&@a2=1"
            ),
            &digest,
        )
    };
    last.send().await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let resource = auth::resource_from_url(&web);
    let name = cx.u("name");
    let file_no_ext = name.replacen(".aspx", "", 1);
    let page_name = if name.ends_with(".aspx") {
        name.clone()
    } else {
        format!("{name}.aspx")
    };
    let list_srv = format!("{}/sitepages", urlutil::get_server_relative_site_url(&web));
    let file_srv = format!("{list_srv}/{page_name}");
    let list_url = urlutil::get_server_relative_path(&web, &list_srv);
    let request_url = format!("{web}/_api/web/GetList('{}')", eqp(&list_url));
    let ne = |k: &str| cx.s(k).filter(|s| !s.is_empty());
    let needs_save =
        ne("title").is_some() || ne("description").is_some() || ne("content").is_some();
    let promote = cx.s("promoteAs");
    let digest = u(&spo::get_request_digest(&web).await?, "FormDigestValue");
    let page = checkout(cx, &name, &web).await?;
    let title = ne("title")
        .map(Value::from)
        .or_else(|| page.get("Title").cloned());
    let mut page_id = page.get("Id").cloned();
    let banner = page.get("BannerImageUrl").cloned();
    let canvas = ne("content")
        .map(Value::from)
        .or_else(|| page.get("CanvasContent1").cloned());
    let layout = page.get("LayoutWebpartsContent").cloned();
    let description = ne("description")
        .map(Value::from)
        .or_else(|| page.get("Description").cloned());
    let topic = page.get("TopicHeader").cloned();
    let byline = page.get("AuthorByline").cloned();
    let pid = |p: &Option<Value>| jsu(p.as_ref());
    if let Some(l) = ne("layoutType") {
        let mut p = Obj::new();
        p.insert("PageLayoutType".into(), Value::from(l.as_str()));
        if l == "Article" {
            p.insert("PromotedState".into(), Value::from(0));
            p.insert("BannerImageUrl".into(), Value::from(format!("{resource}/_layouts/15/images/sitepagethumbnail.png, /_layouts/15/images/sitepagethumbnail.png")));
        }
        sp::system_update_list_item(cx, &request_url, &pid(&page_id), Some(&p), None).await?;
    }
    let mut data = Obj::new();
    match promote.as_deref() {
        Some("HomePage") => {
            merge(format!("{web}/_api/web/rootfolder"), &digest)
                .json(obj! { "WelcomePage": format!("SitePages/{page_name}") })
                .send()
                .await?;
        }
        Some("NewsPage") => {
            let mut p = Obj::new();
            p.insert("PromotedState".into(), Value::from(2));
            p.insert("FirstPublishedDate".into(), Value::from(time::now_iso()));
            sp::system_update_list_item(cx, &request_url, &pid(&page_id), Some(&p), None).await?;
        }
        Some("Template") => {
            let r = Request::post(format!(
                "{web}/_api/SitePages/Pages({})/SavePageAsTemplate",
                pid(&page_id)
            ))
            .header("X-RequestDigest", &digest)
            .header("content-type", "application/json;odata=nometadata")
            .header("X-HTTP-Method", "POST")
            .header("IF-MATCH", "*")
            .spo_json()
            .data()
            .await?;
            if !file_no_ext.is_empty() {
                data.insert("Title".into(), Value::from(file_no_ext.as_str()));
            }
            if let Some(d) = description.as_ref().filter(|d| truthy(d)) {
                data.insert("Description".into(), d.clone());
            }
            for k in ["BannerImageUrl", "LayoutWebpartsContent", "CanvasContent1"] {
                if let Some(v) = r.get(k).filter(|v| truthy(v)) {
                    data.insert(k.into(), v.clone());
                }
            }
            page_id = r.get("Id").cloned();
        }
        _ => {}
    }
    if promote.as_deref() != Some("Template") {
        let extra = truthy_fields(&[
            ("Title", title.as_ref()),
            ("Description", description.as_ref()),
            ("BannerImageUrl", banner.as_ref()),
            ("CanvasContent1", canvas.as_ref()),
            ("LayoutWebpartsContent", layout.as_ref()),
            ("TopicHeader", topic.as_ref()),
            ("AuthorByline", byline.as_ref()),
        ]);
        if let Value::Object(o) = extra {
            data.extend(o);
        }
    }
    let data = Value::Object(data);
    if needs_save {
        merge(
            format!("{web}/_api/SitePages/Pages({})/SavePage", pid(&page_id)),
            &digest,
        )
        .json(data.clone())
        .send()
        .await?;
    }
    if promote.as_deref() == Some("Template") {
        merge(
            format!(
                "{web}/_api/SitePages/Pages({})/SavePageAsDraft",
                pid(&page_id)
            ),
            &digest,
        )
        .json(data.clone())
        .send()
        .await?;
    }
    if let Some(enabled) = cx.opt_bool("commentsEnabled") {
        with_digest(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{file_srv}')/ListItemAllFields/SetCommentsDisabled({})", !enabled), &digest)
            .send()
            .await?;
    }
    if cx.s("demoteFrom").as_deref() == Some("NewsPage") {
        let mut p = Obj::new();
        p.insert("PromotedState".into(), Value::from(0));
        sp::system_update_list_item(cx, &request_url, &pid(&page_id), Some(&p), None).await?;
    }
    if let Some(r) = cx.opt_bool("isRetired") {
        let mut p = Obj::new();
        p.insert("_SPIsRetired".into(), Value::Bool(r));
        sp::system_update_list_item(cx, &request_url, &pid(&page_id), Some(&p), None).await?;
    }
    let req = if !cx.bool("publish") {
        if promote.as_deref() == Some("Template") || !page_id.as_ref().is_some_and(truthy) {
            return Ok(());
        }
        Request::post(format!(
            "{web}/_api/SitePages/Pages({})/SavePageAsDraft",
            pid(&page_id)
        ))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(data)
    } else {
        with_digest(
            format!(
                "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{file_srv}')/CheckIn(comment=@a1,checkintype=@a2)?@a1='{}'&@a2=1",
                eqp(&cx.s("publishMessage").unwrap_or_default())
            ),
            &digest,
        )
    };
    req.send().await?;
    Ok(())
}

// ---- simple file operations ------------------------------------------------------

pub async fn publish(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page_name = leading_trimmed_aspx(&cx.u("name"));
    cx.verbose_log(&format!("Publishing page {page_name}..."));
    publish_page(&web, &page_name).await
}

fn page_file_url(web: &str, page_name: &str) -> String {
    let file = format!(
        "{}/SitePages/{page_name}",
        urlutil::get_server_relative_site_url(web)
    );
    format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')",
        eqp(&file)
    )
}

pub async fn unpublish(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page_name = leading_trimmed_aspx(&cx.u("name"));
    cx.verbose_log(&format!("Unpublishing page {page_name}..."));
    Request::post(format!("{}/UnPublish", page_file_url(&web, &page_name)))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove page '{}'?", cx.u("name")),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    let page_name = leading_trimmed_aspx(&cx.u("name"));
    cx.verbose_log(&format!("Removing page {page_name}..."));
    let url = page_file_url(&web, &page_name);
    let mut r = if cx.bool("permanent") {
        Request::delete(url)
    } else {
        Request::post(format!("{url}/Recycle"))
    }
    .spo_json();
    if cx.bool("bypassSharedLock") {
        r = r.header("Prefer", "bypass-shared-lock");
    }
    r.send().await?;
    Ok(())
}

pub async fn copy(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let web = web.strip_suffix('/').unwrap_or(&web).to_string();
    let mut source = cx.u("sourceName").to_lowercase();
    let target = cx.u("targetUrl").to_lowercase();
    let split: Vec<&str> = target.split("sitepages/").collect();
    let (site, page) = if target.starts_with("http") {
        (split[0].to_string(), split.get(1).map(|s| s.to_string()))
    } else {
        (
            web.clone(),
            Some(if split.len() > 1 {
                split[1].to_string()
            } else {
                target.clone()
            }),
        )
    };
    if !source.contains(".aspx") {
        source.push_str(".aspx");
    }
    let Some(mut page) = page else {
        return Err(Error::msg(
            "TypeError: Cannot read properties of undefined (reading 'indexOf')",
        ));
    };
    if !page.contains(".aspx") {
        page.push_str(".aspx");
    }
    let site = site.strip_suffix('/').unwrap_or(&site).to_string();
    if let Some(p) = page.strip_prefix('/') {
        page = p.to_string();
    }
    cx.verbose_log("Creating page copy...");
    Request::post(format!("{web}/_api/SP.MoveCopyUtil.CopyFileByPath()"))
        .spo_json()
        .json(obj! {
            "srcPath": obj! { "DecodedUrl": format!("{web}/sitepages/{source}") },
            "destPath": obj! { "DecodedUrl": format!("{site}/sitepages/{page}") },
            "options": obj! { "ResetAuthorAndCreatedOnCopy": true, "ShouldBypassSharedLocks": true },
            "overwrite": cx.bool("overwrite"),
        })
        .send()
        .await?;
    let res = Request::get(format!(
        "{site}/_api/sitepages/pages/GetByUrl('sitepages/{page}')"
    ))
    .spo_json()
    .data()
    .await?;
    cx.log(res)?;
    cx.verbose_log(&term::green("DONE"));
    Ok(())
}

// ---- get / list --------------------------------------------------------------------

pub async fn get_cmd(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log("Retrieving information about the page...");
    let mut page_name = String::new();
    if let Some(n) = cx.s("name").filter(|n| !n.is_empty()) {
        page_name = if n.ends_with(".aspx") {
            n
        } else {
            format!("{n}.aspx")
        };
    } else if cx.bool("default") {
        let r = Request::get(format!("{web}/_api/Web/RootFolder?$select=WelcomePage"))
            .spo_json()
            .data()
            .await?;
        match r.get("WelcomePage") {
            Some(Value::String(w)) => page_name = w.rsplit('/').next().unwrap_or("").to_string(),
            None | Some(Value::Null) => {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'split')",
                ));
            }
            Some(_) => return Err(Error::js("WelcomePage.split is not a function")),
        }
    }
    let mut page = Request::get(format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}/SitePages/{}')?$expand=ListItemAllFields/ClientSideApplicationId,ListItemAllFields/PageLayoutType,ListItemAllFields/CommentsDisabled",
        urlutil::get_server_relative_site_url(&web),
        eqp(&page_name)
    ))
    .header("content-type", "application/json;charset=utf-8")
    .spo_json()
    .data()
    .await?;
    if dig(&page, &["ListItemAllFields", "ClientSideApplicationId"])?.and_then(Value::as_str)
        != Some(MODERN_PAGE_APP)
    {
        return Err(Error::Thrown(Value::from(format!(
            "Page {page_name} is not a modern page."
        ))));
    }
    let li = page["ListItemAllFields"].clone();
    if let Some(o) = page
        .get_mut("ListItemAllFields")
        .and_then(Value::as_object_mut)
    {
        o.shift_remove("ID");
    }
    let mut out = page.clone();
    out["commentsDisabled"] = li
        .get("CommentsDisabled")
        .cloned()
        .unwrap_or(Value::from(UNDEF));
    out["title"] = li.get("Title").cloned().unwrap_or(Value::from(UNDEF));
    if let Some(l) = li.get("PageLayoutType").filter(|v| truthy(v)) {
        out["layoutType"] = l.clone();
    }
    if !cx.bool("metadataOnly") {
        let res = Request::get(format!("{web}/_api/SitePages/Pages({})", jsu(li.get("Id"))))
            .header("content-type", "application/json;charset=utf-8")
            .spo_json()
            .data()
            .await?;
        let data = parse_value(res.get("CanvasContent1"))?;
        out["canvasContentJson"] = res
            .get("CanvasContent1")
            .cloned()
            .unwrap_or(Value::from(UNDEF));
        match &data {
            Value::Array(a) if !a.is_empty() => {
                out["numControls"] = Value::from(a.len());
                let mut zones: Vec<Option<&Value>> = Vec::new();
                for c in a.iter().filter(|c| c.get("position").is_some_and(truthy)) {
                    let z = c["position"].get("zoneIndex");
                    if !zones.iter().any(|x| seq(*x, z)) {
                        zones.push(z);
                    }
                }
                out["numSections"] = Value::from(zones.len());
            }
            Value::String(s) if !s.is_empty() => {
                out["numControls"] = Value::from(json::js_len(s));
                return Err(Error::js("canvasData.filter is not a function"));
            }
            _ => {}
        }
    }
    strip_undefined(&mut out);
    cx.log(out)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log("Retrieving client-side pages...");
    let mut pages =
        odata::get_all_items(&format!("{web}/_api/sitepages/pages?$orderby=Title")).await?;
    let files = odata::get_all_items(&format!("{web}/_api/web/lists/SitePages/rootfolder/files?$expand=ListItemAllFields/ClientSideApplicationId&$orderby=Name")).await?;
    if !files.is_empty() {
        let mut csp_files = Vec::new();
        for f in &files {
            if dig(f, &["ListItemAllFields", "ClientSideApplicationId"])?.and_then(Value::as_str)
                == Some(MODERN_PAGE_APP)
            {
                csp_files.push(f);
            }
        }
        for p in pages.iter_mut() {
            let found = csp_files.iter().find(|cp| {
                cp.get("ListItemAllFields").is_some_and(truthy)
                    && seq(cp["ListItemAllFields"].get("Id"), p.get("Id"))
            });
            if let (Some(cp), Some(po)) = (found, p.as_object()) {
                let mut merged = cp.as_object().cloned().unwrap_or_default();
                for (k, v) in po {
                    merged.insert(k.clone(), v.clone());
                }
                *p = Value::Object(merged);
            }
        }
        for p in pages.iter_mut() {
            if let Some(o) = p
                .get_mut("ListItemAllFields")
                .filter(|v| truthy(v))
                .and_then(Value::as_object_mut)
            {
                o.shift_remove("ID");
            }
        }
    }
    cx.log(Value::Array(pages))
}

pub async fn template_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log("Retrieving templates...");
    match odata::get_all_items(&format!("{web}/_api/sitepages/pages/templates")).await {
        Ok(r) => cx.log(Value::Array(r)),
        Err(e) if e.http().is_some_and(|h| h.status == 404) => cx.log(Value::Array(Vec::new())),
        Err(e) => Err(e),
    }
}

// ---- sections and columns (HTML canvas) ----------------------------------------------

fn section_index(page: &ClientSidePage, n: Option<f64>) -> Option<usize> {
    page.sections.iter().position(|s| match n {
        Some(n) => num_eq(Some(&s.order), n),
        None => false,
    })
}

pub async fn section_get(cx: &mut Cx) -> Result<()> {
    let page = get_page(cx, &cx.u("pageName"), &cx.u("webUrl")).await?;
    if let Some(i) = section_index(&page, cx.num("section")) {
        cx.log(page.sections[i].info(json_output(cx)))?;
    }
    Ok(())
}

pub async fn section_list(cx: &mut Cx) -> Result<()> {
    let page = get_page(cx, &cx.u("pageName"), &cx.u("webUrl")).await?;
    let j = json_output(cx);
    let out: Vec<Value> = page.sections.iter().map(|s| s.info(j)).collect();
    if j {
        return cx.log(Value::Array(out));
    }
    let out = out
        .iter()
        .map(|s| {
            let mut o = obj! { "order": s["order"].clone(), "columns": arr(s, "columns").len() };
            if s.get("isVertical").is_some_and(truthy) {
                o["isVertical"] = s["isVertical"].clone();
            }
            o
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn section_remove(cx: &mut Cx) -> Result<()> {
    let name = cx.u("pageName");
    let n = cx.num("section");
    let ns = n.map(jnum).unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove section {ns} from '{name}'?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Removing modern page section {name} - {ns}..."));
    let digest = u(&spo::get_request_digest(&web).await?, "FormDigestValue");
    let mut page = get_page(cx, &name, &web).await?;
    let Some(i) = section_index(&page, n) else {
        return Err(Error::js(format!("Section {ns} not found")));
    };
    page.sections.remove(i);
    let html = page.to_html();
    // `return request.post(...)` inside the try: a rejection skips the catch
    merge(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}/sitepages/{name}')/ListItemAllFields", urlutil::get_server_relative_site_url(&web)), &digest)
        .json(obj! { "CanvasContent1": html })
        .send()
        .await
        .map_err(Error::uncaught)?;
    Ok(())
}

pub async fn column_get(cx: &mut Cx) -> Result<()> {
    let page = get_page(cx, &cx.u("pageName"), &cx.u("webUrl")).await?;
    let Some(si) = section_index(&page, cx.num("section")) else {
        return Ok(());
    };
    let j = json_output(cx);
    let s = &page.sections[si];
    let Some(col) = s
        .columns
        .iter()
        .find(|c| cx.num("column").is_some_and(|n| num_eq(Some(&c.order), n)))
    else {
        return Ok(());
    };
    let mut out = col.info(s, j);
    let controls: Vec<Value> = col.controls.iter().map(|c| c.info(j)).collect();
    out["controls"] = if j {
        Value::Array(controls)
    } else {
        Value::from(
            controls
                .iter()
                .map(|c| format!("{} ({})", jsu(c.get("id")), jsu(c.get("title"))))
                .collect::<Vec<_>>()
                .join(", "),
        )
    };
    cx.log(out)
}

pub async fn column_list(cx: &mut Cx) -> Result<()> {
    let page = get_page(cx, &cx.u("pageName"), &cx.u("webUrl")).await?;
    let j = json_output(cx);
    let out = match section_index(&page, cx.num("section")) {
        Some(si) => {
            let s = &page.sections[si];
            s.columns
                .iter()
                .map(|c| {
                    let mut o = c.info(s, j);
                    o["controls"] = Value::from(c.controls.len());
                    o
                })
                .collect()
        }
        None => Vec::new(),
    };
    cx.log(Value::Array(out))
}

pub async fn text_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page_name = with_aspx(&cx.u("pageName"));
    cx.verbose_log("Retrieving request digest...");
    let digest = u(&spo::get_request_digest(&web).await?, "FormDigestValue");
    cx.verbose_log(&format!("Retrieving modern page {page_name}..."));
    let mut page = get_page(cx, &page_name, &web).await?;
    let section = cx
        .num("section")
        .filter(|n| *n != 0.0 && !n.is_nan())
        .unwrap_or(1.0)
        - 1.0;
    let column = cx
        .num("column")
        .filter(|n| *n != 0.0 && !n.is_nan())
        .unwrap_or(1.0)
        - 1.0;
    if page.sections.is_empty() {
        let mut s = Section::new(Value::from(1), None);
        s.add_column(Value::from(12));
        page.sections.push(s);
    }
    if section >= page.sections.len() as f64 {
        return Err(Error::js(format!(
            "Invalid section '{}'",
            jnum(section + 1.0)
        )));
    }
    let s = &mut page.sections[section as usize];
    if column >= s.columns.len() as f64 {
        return Err(Error::js(format!(
            "Invalid column '{}'",
            jnum(column + 1.0)
        )));
    }
    let col = &mut s.columns[column as usize];
    let mut text = Part::text(&cx.u("text"));
    match cx.num("order") {
        Some(o) if !(o - 1.0 < 0.0 || o - 1.0 >= col.controls.len() as f64) => {
            text.order = json::num_value(o - 1.0);
            col.controls.insert((o - 1.0) as usize, text);
        }
        _ => col.controls.push(text),
    }
    let html = page.to_html();
    if cx.debug() {
        cx.stderr("Updated canvas content: ");
        cx.stderr(&html);
        cx.stderr("");
    }
    merge(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}/sitepages/{page_name}')/ListItemAllFields", urlutil::get_server_relative_site_url(&web)), &digest)
        .json(obj! { "CanvasContent1": html })
        .send()
        .await?;
    Ok(())
}

// ---- controls (JSON canvas) ------------------------------------------------------------

async fn canvas_page(web: &str, page_name: &str) -> Result<Value> {
    Request::get(format!(
        "{web}/_api/SitePages/Pages/GetByUrl('sitepages/{}')",
        eqp(page_name)
    ))
    .spo_json()
    .data()
    .await
}

fn canvas_of(page: &Value) -> Result<Value> {
    match page.get("CanvasContent1").filter(|v| truthy(v)) {
        Some(c) => parse_value(Some(c)),
        None => Ok(Value::Array(Vec::new())),
    }
}

pub async fn control_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page = canvas_page(&web, &with_aspx(&cx.u("pageName"))).await?;
    let data = canvas_of(&page)?;
    let id = cx.u("id").to_lowercase();
    let mut found = None;
    for c in as_arr(&data, "canvasData", "find")? {
        if lower_id(c)?.as_deref() == Some(id.as_str()) {
            found = Some(c);
            break;
        }
    }
    match found {
        Some(c) => {
            let mut out = control_summary(c)?;
            // `title`/`controlType` stay as `undefined` keys: text and md print
            // `undefined`, csv an empty cell, json drops them
            let fill = match cx.str("output").as_deref() {
                Some("text") => Some("undefined"),
                Some("md") => Some(UNDEF),
                Some("csv") => Some(""),
                _ => None,
            };
            if let Some(fill) = fill {
                for k in ["title", "controlType"] {
                    if out.get(k).is_none() {
                        out[k] = Value::from(fill);
                    }
                }
                let order = ["id", "type", "title", "controlType", "order", "controlData"];
                let mut o = Obj::new();
                for k in order {
                    if let Some(v) = out.get(k) {
                        o.insert(k.into(), v.clone());
                    }
                }
                out = Value::Object(o);
            }
            cx.log(out)
        }
        None => {
            cx.verbose_log(&format!(
                "Control with ID {} not found on page {}",
                cx.u("id"),
                cx.u("pageName")
            ));
            Ok(())
        }
    }
}

pub async fn control_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page = canvas_page(&web, &with_aspx(&cx.u("pageName"))).await?;
    let data = canvas_of(&page)?;
    let mut out = Vec::new();
    for c in as_arr(&data, "canvasData", "filter")?
        .iter()
        .filter(|c| c.get("position").is_some_and(truthy))
    {
        out.push(control_summary(c)?);
    }
    cx.log(Value::Array(out))
}

pub async fn control_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to delete control '{id}' on page '{}'?",
            cx.u("pageName")
        ),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Getting page properties for page '{}'...",
        cx.u("pageName")
    ));
    let raw = cx.u("pageName");
    let page_name = urlutil::remove_leading_slashes(&if raw.to_lowercase().ends_with(".aspx") {
        raw.clone()
    } else {
        format!("{raw}.aspx")
    });
    let url = format!(
        "{web}/_api/SitePages/Pages/GetByUrl('SitePages/{}')",
        eqp(&page_name)
    );
    let props = Request::get(format!("{url}?$select=CanvasContent1"))
        .spo_json()
        .data()
        .await?;
    let Some(content) = props.get("CanvasContent1").filter(|v| truthy(v)) else {
        return Err(Error::Thrown(Value::from(format!(
            "Page '{page_name}' doesn't contain canvas control '{id}'."
        ))));
    };
    let controls = parse_value(Some(content))?;
    let lid = id.to_lowercase();
    let mut has = false;
    for c in as_arr(&controls, "pageControls", "some")? {
        if lower_id(c)?.as_deref() == Some(lid.as_str()) {
            has = true;
            break;
        }
    }
    if !has {
        return Err(Error::Thrown(Value::from(format!(
            "Control with ID '{id}' was not found on page '{page_name}'."
        ))));
    }
    cx.verbose_log("Checking out page...");
    let page = checkout(cx, &page_name, &web).await?;
    let canvas = parse_value(page.get("CanvasContent1"))?;
    cx.verbose_log(&format!("Removing control with ID '{id}' from page..."));
    let mut kept = Vec::new();
    for c in as_arr(&canvas, "canvasContent", "filter")? {
        let keep = match c.get("id").filter(|v| truthy(v)) {
            None => true,
            Some(Value::String(s)) => s.to_lowercase() != lid,
            Some(_) => return Err(Error::js("control.id.toLowerCase is not a function")),
        };
        if keep {
            kept.push(c.clone());
        }
    }
    Request::patch(format!("{url}/SavePageAsDraft"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(obj! { "CanvasContent1": stringify(&Value::Array(kept)) })
        .send()
        .await?;
    if !cx.bool("draft") {
        cx.verbose_log("Republishing page...");
        publish_page(&web, &page_name).await?;
    }
    Ok(())
}

/// `{ ...a, ...b }` for object values.
fn spread(a: Option<&Value>, b: &Value) -> Value {
    let mut o = a.and_then(Value::as_object).cloned().unwrap_or_default();
    if let Some(b) = b.as_object() {
        for (k, v) in b {
            o.insert(k.clone(), v.clone());
        }
    }
    Value::Object(o)
}

fn save_draft(web: &str, page_name: &str, data: Value) -> Request {
    Request::post(format!("{}/SavePageAsDraft", by_url(web, page_name)))
        .header("X-HTTP-Method", "MERGE")
        .header("IF-MATCH", "*")
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(data)
}

pub async fn control_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let id = cx.u("id");
    let lid = id.to_lowercase();
    let page_name = with_aspx(&cx.u("pageName"));
    let res = canvas_page(&web, &page_name).await?;
    let Some(content) = res.get("CanvasContent1").filter(|v| truthy(v)) else {
        return Err(Error::Thrown(Value::from(format!(
            "Page {page_name} doesn't contain canvas controls."
        ))));
    };
    let controls = parse_value(Some(content))?;
    let mut found = false;
    for c in as_arr(&controls, "pageControls", "find")? {
        if let Some(v) = c.get("id").filter(|v| truthy(v)) {
            let s = v
                .as_str()
                .ok_or_else(|| Error::js("control.id.toLowerCase is not a function"))?;
            if s.to_lowercase() == lid {
                found = true;
                break;
            }
        }
    }
    if !found {
        return Err(Error::Thrown(Value::from(format!(
            "Control with ID {id} not found on page {page_name}"
        ))));
    }
    cx.verbose_log(&format!("Control with ID {id} found on the page"));
    let page = checkout(cx, &page_name, &web).await?;
    let mut canvas = parse_value(page.get("CanvasContent1"))?;
    if cx.debug() {
        m365_core::log::log_stderr_value(&canvas);
    }
    let list = as_arr_mut(&mut canvas, "canvasContent", "find")?;
    let mut idx = None;
    for (i, c) in list.iter().enumerate() {
        let s = match c.get("id") {
            Some(Value::String(s)) => s.to_lowercase(),
            None | Some(Value::Null) => {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'toLowerCase')",
                ));
            }
            Some(_) => return Err(Error::js("c.id.toLowerCase is not a function")),
        };
        if s == lid {
            idx = Some(i);
            break;
        }
    }
    let Some(i) = idx else {
        return Err(Error::Thrown(Value::from(format!(
            "Control with ID {id} not found on page {page_name}"
        ))));
    };
    let control = &mut list[i];
    if let Some(d) = cx.s("webPartData").filter(|s| !s.is_empty()) {
        if cx.verbose() {
            cx.stderr("web part data:");
            cx.stderr(&d);
            cx.stderr("");
        }
        let parsed = json::jparse(&d)?;
        let wid = dig(control, &["webPartData", "id"])?.cloned();
        let wpd = control.get("webPartData");
        let iid = wpd.and_then(|w| w.get("instanceId")).cloned();
        let mut merged = spread(wpd, &parsed);
        assign(&mut merged, "id", wid);
        assign(&mut merged, "instanceId", iid);
        strip_undefined(&mut merged);
        control["webPartData"] = merged;
        if cx.verbose() {
            cx.stderr("Updated web part data:");
            m365_core::log::log_stderr_value(&control["webPartData"]);
            cx.stderr("");
        }
    } else if let Some(p) = cx.s("webPartProperties").filter(|s| !s.is_empty()) {
        if cx.verbose() {
            cx.stderr("web part properties data:");
            cx.stderr(&p);
            cx.stderr("");
        }
        let parsed = json::jparse(&p)?;
        let props = dig(control, &["webPartData", "properties"])?.cloned();
        let merged = spread(props.as_ref(), &parsed);
        control["webPartData"]["properties"] = merged;
        if cx.verbose() {
            cx.stderr("Updated web part properties:");
            m365_core::log::log_stderr_value(&control["webPartData"]["properties"]);
            cx.stderr("");
        }
    } else if let Some(h) = cx.s("innerHtml").filter(|s| !s.is_empty()) {
        if !num_eq(control.get("controlType"), 4.0) && !control.get("innerHTML").is_some_and(truthy)
        {
            return Err(Error::Thrown(Value::from(
                "The innerHtml option can only be used with text web parts.",
            )));
        }
        control["innerHTML"] = Value::from(h);
    }
    let mut data = truthy_fields(&[
        ("AuthorByline", page.get("AuthorByline")),
        ("BannerImageUrl", page.get("BannerImageUrl")),
        ("Description", page.get("Description")),
        ("Title", page.get("Title")),
        ("TopicHeader", page.get("TopicHeader")),
        ("LayoutWebpartsContent", page.get("LayoutWebpartsContent")),
    ]);
    data["CanvasContent1"] = Value::from(stringify(&canvas));
    save_draft(&web, &page_name, data).send().await?;
    Ok(())
}

// ---- clientsidewebpart add ----------------------------------------------------------------

/// `c.position && c.position.zoneIndex === zone && c.position.sectionIndex === column`.
fn in_column(c: &Value, zone: Option<&Value>, column: Option<&Value>) -> bool {
    match c.get("position").filter(|p| truthy(p)) {
        Some(p) => seq(p.get("zoneIndex"), zone) && seq(p.get("sectionIndex"), column),
        None => false,
    }
}

/// Unique `position.zoneIndex` values of positioned controls, sorted.
fn zone_indices(canvas: &[Value]) -> Vec<Option<Value>> {
    let mut z: Vec<Option<Value>> = Vec::new();
    for c in canvas
        .iter()
        .filter(|c| c.get("position").is_some_and(truthy))
    {
        let v = c["position"].get("zoneIndex").cloned();
        if !z.iter().any(|x| seq(x.as_ref(), v.as_ref())) {
            z.push(v);
        }
    }
    // `(a, b) => a - b`; undefined sorts last like V8 does for NaN-free runs
    z.sort_by(|a, b| {
        match (
            a.as_ref().and_then(Value::as_f64),
            b.as_ref().and_then(Value::as_f64),
        ) {
            (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
            _ => std::cmp::Ordering::Equal,
        }
    });
    z
}

async fn get_web_part(cx: &Cx, web: &str) -> Result<Value> {
    let std = cx.s("standardWebPart").filter(|s| !s.is_empty());
    let wp_id = match &std {
        Some(s) => standard_web_part_id(s).map(String::from),
        None => cx.s("webPartId"),
    };
    let wp = wp_id.clone().unwrap_or_else(|| "undefined".into());
    if cx.debug() {
        cx.stderr(&format!(
            "StandardWebPart: {}",
            std.as_deref().unwrap_or("undefined")
        ));
        cx.stderr(&format!("WebPartId: {wp}"));
    }
    let res = Request::get(format!("{web}/_api/web/getclientsidewebparts()"))
        .spo_json()
        .data()
        .await?;
    let lw = wp.to_lowercase();
    let mut defs = Vec::new();
    for c in need_arr(&res, "value", "filter")? {
        let id = match c.get("Id") {
            Some(Value::String(s)) => s.to_lowercase(),
            None | Some(Value::Null) => {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'toLowerCase')",
                ));
            }
            Some(_) => return Err(Error::js("c.Id.toLowerCase is not a function")),
        };
        if id == lw || id == format!("{{{lw}}}") {
            defs.push(c.clone());
        }
    }
    if defs.is_empty() {
        return Err(Error::js(format!(
            "There is no available WebPart with Id {wp}."
        )));
    }
    if cx.debug() {
        cx.stderr("WebPart definition:");
        m365_core::log::log_stderr_value(&Value::Array(defs.clone()));
        cx.stderr("");
    }
    cx.verbose_log(&format!(
        "Creating instance from definition of WebPart {wp}..."
    ));
    let component = &defs[0];
    let id = util::new_guid();
    let cid = s(component, "Id").trim_start_matches('{').to_string();
    let cid = cid.strip_suffix('}').unwrap_or(&cid).to_lowercase();
    let manifest = parse_value(component.get("Manifest"))?;
    let Some(e) = dig(&manifest, &["preconfiguredEntries", "0"])?.cloned() else {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'description')",
        ));
    };
    let description = dig(&e, &["description", "default"])?.cloned();
    let title = dig(&e, &["title", "default"])?.cloned();
    Ok(obj! {
        "id": id.as_str(),
        "webPartData": obj! {
            "dataVersion": "1.0",
            "description": ?description,
            "id": cid.as_str(),
            "instanceId": id.as_str(),
            "properties": ?e.get("properties").cloned(),
            "title": ?title,
        },
        "webPartId": cid.as_str(),
    })
}

pub async fn clientsidewebpart_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page_name = with_aspx(&cx.u("pageName"));
    cx.verbose_log("Retrieving page information...");
    let mut page = Request::get(by_url(&web, &page_name))
        .spo_json()
        .data()
        .await?;
    if !page
        .get("IsPageCheckedOutToCurrentUser")
        .is_some_and(truthy)
    {
        page = checkout(cx, &page_name, &web).await?;
    }
    let (layout, byline, banner, description, title, topic) = if truthy(&page) {
        (
            page.get("LayoutWebpartsContent").cloned(),
            page.get("AuthorByline").cloned(),
            page.get("BannerImageUrl").cloned(),
            page.get("Description").cloned(),
            page.get("Title").cloned(),
            page.get("TopicHeader").cloned(),
        )
    } else {
        (
            Some(Value::from("")),
            Some(Value::Array(vec![Value::from("")])),
            Some(Value::from("")),
            Some(Value::from("")),
            Some(Value::from("")),
            Some(Value::from("")),
        )
    };
    cx.verbose_log(&format!(
        "Retrieving definition for web part {}...",
        cx.s("webPartId")
            .filter(|s| !s.is_empty())
            .or_else(|| cx.s("standardWebPart"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let mut canvas = match page.get("CanvasContent1").filter(|v| truthy(v)) {
        Some(c) => parse_value(Some(c))?,
        None => json::jparse(DEFAULT_CANVAS)?,
    };
    let mut web_part = get_web_part(cx, &web).await?;
    cx.verbose_log("Setting client-side web part layout and properties...");
    if let Some(p) = cx.s("webPartProperties").filter(|s| !s.is_empty()) {
        if cx.debug() {
            cx.stderr("WebPart properties: ");
            cx.stderr(&p);
            cx.stderr("");
        }
        let parsed = json::jparse(&p)?;
        let parsed = if parsed.is_null() {
            Value::Object(Obj::new())
        } else {
            parsed
        };
        let target = web_part["webPartData"].get("properties").cloned();
        web_part["webPartData"]["properties"] = extend_own(target, &parsed)?;
    }
    if let Some(d) = cx.s("webPartData").filter(|s| !s.is_empty()) {
        if cx.debug() {
            cx.stderr("WebPart data:");
            cx.stderr(&d);
            cx.stderr("");
        }
        let parsed = json::jparse(&d)?;
        web_part["webPartData"] = extend_own(Some(web_part["webPartData"].clone()), &parsed)?;
        web_part["id"] = web_part["webPartData"]
            .get("instanceId")
            .cloned()
            .unwrap_or(Value::from(UNDEF));
    }
    let list = as_arr_mut(&mut canvas, "canvasContent", "indexOf")?;
    let ci = correct_control(list, cx)?;
    let control = list[ci].clone();
    let position = control.get("position").cloned();
    let zone = dig(&control, &["position", "zoneIndex"])?.cloned();
    let column = position
        .as_ref()
        .and_then(|p| p.get("sectionIndex"))
        .cloned();
    let mut wpc = obj! {
        "controlType": 3,
        "displayMode": 2,
        "id": ?web_part.get("id").cloned(),
        "position": spread(position.as_ref(), &Value::Null),
        "webPartId": ?web_part.get("webPartId").cloned(),
        "emphasis": obj! {},
        "zoneGroupMetadata": ?control.get("zoneGroupMetadata").cloned(),
    };
    if let Some(o) = web_part.as_object() {
        for (k, v) in o {
            wpc[k.as_str()] = v.clone();
        }
    }
    if !control.get("controlType").is_some_and(truthy) {
        wpc["position"]["controlIndex"] = Value::from(1);
        list[ci] = wpc;
    } else {
        let mut indices: Vec<Option<Value>> = list
            .iter()
            .filter(|c| in_column(c, zone.as_ref(), column.as_ref()))
            .map(|c| c["position"].get("controlIndex").cloned())
            .collect();
        indices.sort_by(|a, b| {
            match (
                a.as_ref().and_then(Value::as_f64),
                b.as_ref().and_then(Value::as_f64),
            ) {
                (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(std::cmp::Ordering::Equal),
                _ => std::cmp::Ordering::Equal,
            }
        });
        let order = cx.num("order").filter(|n| *n != 0.0 && !n.is_nan());
        let find = |list: &[Value], idx: Option<&Value>| {
            list.iter()
                .position(|c| {
                    in_column(c, zone.as_ref(), column.as_ref())
                        && seq(c["position"].get("controlIndex"), idx)
                })
                .map(|i| i as i64)
                .unwrap_or(-1)
        };
        let at = match order {
            Some(o) if o <= indices.len() as f64 => {
                let idx = indices.get((o - 1.0) as usize).cloned().flatten();
                find(list, idx.as_ref())
            }
            _ => {
                let idx = indices.pop().flatten();
                find(list, idx.as_ref()) + 1
            }
        };
        let len = list.len() as i64;
        let at = if at < 0 {
            (len + at).max(0)
        } else {
            at.min(len)
        } as usize;
        list.insert(at, wpc);
        for (i, c) in list
            .iter_mut()
            .filter(|c| in_column(c, zone.as_ref(), column.as_ref()))
            .enumerate()
        {
            c["position"]["controlIndex"] = Value::from(i + 1);
        }
    }
    let mut data = truthy_fields(&[
        ("AuthorByline", byline.as_ref()),
        ("BannerImageUrl", banner.as_ref()),
        ("Description", description.as_ref()),
        ("Title", title.as_ref()),
        ("TopicHeader", topic.as_ref()),
        ("LayoutWebpartsContent", layout.as_ref()),
    ]);
    data["CanvasContent1"] = Value::from(stringify_js(&canvas));
    save_draft(&web, &page_name, data).send().await?;
    Ok(())
}

/// `Object.getOwnPropertyNames(source).reduce(... t[v] = source[v] ...)`.
fn extend_own(target: Option<Value>, source: &Value) -> Result<Value> {
    let mut t = match target {
        Some(Value::Object(o)) => Value::Object(o),
        Some(Value::Null) | None => {
            return match source.as_object() {
                Some(o) if o.is_empty() => Ok(target.unwrap_or(Value::from(UNDEF))),
                _ => Err(Error::js(format!(
                    "Cannot set properties of {} (setting '{}')",
                    if target.is_some() {
                        "null"
                    } else {
                        "undefined"
                    },
                    source
                        .as_object()
                        .and_then(|o| o.keys().next().cloned())
                        .unwrap_or_default()
                ))),
            };
        }
        Some(other) => other,
    };
    if let (Some(to), Some(so)) = (t.as_object_mut(), source.as_object()) {
        for (k, v) in so {
            to.insert(k.clone(), v.clone());
        }
    }
    Ok(t)
}

/// `getCorrectControl`: the index of the control to replace or follow.
fn correct_control(canvas: &mut Vec<Value>, cx: &Cx) -> Result<usize> {
    if cx.bool("verticalSection") {
        if let Some(i) = canvas
            .iter()
            .position(|c| num_eq(c.get("position").and_then(|p| p.get("layoutIndex")), 2.0))
        {
            return Ok(i);
        }
        canvas.insert(0, obj! { "position": obj! { "controlIndex": 1, "sectionIndex": 1, "zoneIndex": 1, "sectionFactor": 12, "layoutIndex": 2 }, "emphasis": obj! {}, "displayMode": 2 });
        return Ok(0);
    }
    if canvas.len() == 1 {
        canvas.insert(0, obj! { "position": obj! { "controlIndex": 1, "sectionIndex": 1, "zoneIndex": 1, "sectionFactor": 12, "layoutIndex": 1 }, "emphasis": obj! {}, "displayMode": 2 });
    }
    let zones = zone_indices(canvas);
    let section = cx
        .num("section")
        .filter(|n| *n != 0.0 && !n.is_nan())
        .unwrap_or(zones.len() as f64);
    if section > zones.len() as f64 {
        return Err(Error::Thrown(Value::from(format!(
            "Invalid section '{}'",
            jnum(section)
        ))));
    }
    let zone = if section >= 1.0 {
        zones.get(section as usize - 1).cloned().flatten()
    } else {
        None
    };
    let column = json::num_value(
        cx.num("column")
            .filter(|n| *n != 0.0 && !n.is_nan())
            .unwrap_or(1.0),
    );
    match canvas
        .iter()
        .position(|c| in_column(c, zone.as_ref(), Some(&column)))
    {
        Some(i) => Ok(i),
        None => Err(Error::Thrown(Value::from(format!(
            "Invalid column '{}'",
            cx.num("column")
                .map(jnum)
                .unwrap_or_else(|| "undefined".into())
        )))),
    }
}

// ---- section add (JSON canvas) --------------------------------------------------------------

const FILL_MODES: [&str; 4] = ["ScaleToFill", "ScaleToFit", "Tile", "OriginalSize"];
const REFLOW: [&str; 2] = ["TopToBottom", "LeftToRight"];

fn column_value(cx: &Cx, zone: Value, section: i64, factor: i64, zone_id: Option<&str>) -> Value {
    let mut c = obj! {
        "displayMode": 2,
        "position": obj! { "zoneIndex": zone, "sectionIndex": section, "sectionFactor": factor, "layoutIndex": 1, "zoneId": ?zone_id },
        "emphasis": obj! {},
    };
    if let Some(e) = cx.s("zoneEmphasis").filter(|s| !s.is_empty())
        && let Some(i) = ["none", "neutral", "soft", "strong"]
            .iter()
            .position(|x| *x == e.to_lowercase())
    {
        c["emphasis"] = obj! { "zoneEmphasis": i };
    }
    if cx.bool("isCollapsibleSection") {
        let right = cx
            .s("iconAlignment")
            .is_some_and(|a| a.to_lowercase() == "right");
        c["zoneGroupMetadata"] = obj! {
            "type": 1,
            "isExpanded": cx.bool("isExpanded"),
            "showDividerLine": cx.bool("showDivider"),
            "iconAlignment": if right { "right" } else { "left" },
            "displayName": ?cx.val("collapsibleTitle").cloned(),
            "headingLevel": cx.num("headingLevel").filter(|n| *n != 0.0 && !n.is_nan()).map(json::num_value).unwrap_or(Value::from(2)),
        };
    }
    c
}

fn section_columns(cx: &Cx, zone: Value, zone_id: Option<&str>) -> Vec<Value> {
    let col = |s: i64, f: i64| column_value(cx, zone.clone(), s, f, zone_id);
    match cx.s("sectionTemplate").as_deref() {
        Some("OneColumnFullWidth") => vec![col(1, 0)],
        Some("TwoColumn") => vec![col(1, 6), col(2, 6)],
        Some("ThreeColumn") => vec![col(1, 4), col(2, 4), col(3, 4)],
        Some("TwoColumnLeft") => vec![col(1, 8), col(2, 4)],
        Some("TwoColumnRight") => vec![col(1, 4), col(2, 8)],
        Some("Vertical") => {
            let mut c = column_value(cx, Value::from(1), 1, 12, zone_id);
            c["position"]["isLayoutReflowOnTop"] =
                Value::Bool(cx.val("isLayoutReflowOnTop").is_some());
            c["position"]["layoutIndex"] = Value::from(2);
            c["position"]["controlIndex"] = Value::from(1);
            vec![c]
        }
        Some("Flexible") => {
            let mut c = col(1, 100);
            let axis = cx
                .s("zoneReflowStrategy")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    REFLOW
                        .iter()
                        .position(|x| *x == s)
                        .map(|i| i as i64)
                        .unwrap_or(-1)
                })
                .unwrap_or(0);
            c["zoneReflowStrategy"] = obj! { "axis": axis };
            if let Some(h) = cx.num("zoneHeight").filter(|n| *n != 0.0 && !n.is_nan()) {
                c["zoneHeight"] = json::num_value(h);
            }
            vec![c]
        }
        _ => vec![col(1, 12)],
    }
}

fn background_details(cx: &Cx) -> Value {
    let mut d = obj! {};
    if let Some(g) = cx.s("gradientText").filter(|s| !s.is_empty()) {
        d["type"] = Value::from("gradient");
        d["gradient"] = Value::from(g);
    }
    if cx.s("imageUrl").is_some_and(|s| !s.is_empty()) {
        let n = |k: &str, def: i64| {
            cx.num(k)
                .filter(|n| *n != 0.0 && !n.is_nan())
                .map(json::num_value)
                .unwrap_or(Value::from(def))
        };
        d["type"] = Value::from("image");
        d["imageData"] = obj! { "source": 2, "fileName": "sectionbackground.jpg", "height": n("imageHeight", 955), "width": n("imageWidth", 555) };
        d["fillMode"] = Value::from(
            cx.s("fillMode")
                .filter(|s| !s.is_empty())
                .map(|f| {
                    FILL_MODES
                        .iter()
                        .position(|x| *x == f)
                        .map(|i| i as i64)
                        .unwrap_or(-1)
                })
                .unwrap_or(0),
        );
    }
    d
}

fn set_background(cx: &Cx, zone_id: &str, bg: &mut Value) -> Result<()> {
    let mut entry = background_details(cx);
    entry["useLightText"] = Value::Bool(cx.bool("useLightText"));
    entry["overlay"] = obj! {
        "color": cx.s("overlayColor").filter(|s| !s.is_empty()).unwrap_or_else(|| "#FFFFFF".into()),
        "opacity": cx.num("overlayOpacity").filter(|n| *n != 0.0 && !n.is_nan()).map(json::num_value).unwrap_or(Value::from(60)),
    };
    let zb = dig(bg, &["webPartData", "properties", "zoneBackground"])?;
    if zb.is_none_or(|z| !z.is_object()) {
        return Err(Error::js(format!(
            "Cannot set properties of {} (setting '{zone_id}')",
            if zb.is_some_and(Value::is_null) {
                "null"
            } else {
                "undefined"
            }
        )));
    }
    bg["webPartData"]["properties"]["zoneBackground"][zone_id] = entry;
    if let Some(img) = cx.s("imageUrl").filter(|s| !s.is_empty()) {
        let spc = dig(bg, &["webPartData", "serverProcessedContent"])?;
        if spc.is_none() {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'imageSources')",
            ));
        }
        if bg["webPartData"]["serverProcessedContent"]
            .get("imageSources")
            .is_some_and(truthy)
        {
            bg["webPartData"]["serverProcessedContent"]["imageSources"]
                [format!("zoneBackground.{zone_id}.imageData.url")] = Value::from(img);
        }
    }
    Ok(())
}

pub async fn section_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let mut page_name = cx.u("pageName").to_lowercase();
    if !page_name.ends_with(".aspx") {
        page_name.push_str(".aspx");
    }
    cx.verbose_log("Retrieving page information...");
    let res = Request::get(format!(
        "{}?$select=CanvasContent1,IsPageCheckedOutToCurrentUser",
        by_url(&web, &page_name)
    ))
    .spo_json()
    .data()
    .await?;
    let mut canvas = match res.get("CanvasContent1").filter(|v| truthy(v)) {
        Some(c) => parse_value(Some(c))?,
        None => json::jparse(DEFAULT_CANVAS)?,
    };
    let template = cx.s("sectionTemplate");
    let pos = |c: &Value, k: &str| c.get("position").and_then(|p| p.get(k)).cloned();
    if template.as_deref() == Some("OneColumnFullWidth")
        && as_arr(&canvas, "canvasContent", "some")?.iter().any(|c| {
            num_eq(pos(c, "layoutIndex").as_ref(), 2.0)
                && num_eq(pos(c, "sectionFactor").as_ref(), 12.0)
        })
    {
        return Err(Error::Thrown(Value::from(
            "A vertical section already exists on the page. A full-width section cannot be added to a page that already has a vertical section.",
        )));
    }
    if template.as_deref() == Some("Vertical")
        && as_arr(&canvas, "canvasContent", "some")?.iter().any(|c| {
            num_eq(pos(c, "layoutIndex").as_ref(), 1.0)
                && num_eq(pos(c, "sectionFactor").as_ref(), 0.0)
        })
    {
        return Err(Error::Thrown(Value::from(
            "A full-width section already exists on the page. A vertical section cannot be added to a page that already has a full-width section.",
        )));
    }
    if !res.get("IsPageCheckedOutToCurrentUser").is_some_and(truthy) {
        checkout(cx, &page_name, &web).await?;
    }
    let list = as_arr_mut(&mut canvas, "canvasContent", "filter")?;
    let mut zones = zone_indices(list);
    let next = match zones.last() {
        Some(z) => json::num_value(z.as_ref().and_then(Value::as_f64).unwrap_or(f64::NAN) + 1.0),
        None => Value::from(1),
    };
    zones.push(Some(next));
    let mut section = cx
        .num("order")
        .filter(|n| *n != 0.0 && !n.is_nan())
        .unwrap_or(zones.len() as f64);
    if section > zones.len() as f64 {
        section = zones.len() as f64;
    }
    let zone = zones[section as usize - 1]
        .clone()
        .unwrap_or(Value::from(UNDEF));
    let zf = zone.as_f64();
    let mut zone_id = None;
    if cx
        .s("zoneEmphasis")
        .is_some_and(|s| ["image", "gradient"].contains(&s.to_lowercase().as_str()))
    {
        let id = util::new_guid();
        match list.iter().position(|c| num_eq(c.get("controlType"), 14.0)) {
            Some(i) => set_background(cx, &id, &mut list[i])?,
            None => {
                let mut bg = obj! {
                    "controlType": 14,
                    "webPartData": obj! {
                        "properties": obj! { "zoneBackground": obj! {} },
                        "serverProcessedContent": obj! { "htmlStrings": obj! {}, "searchablePlainTexts": obj! {}, "imageSources": obj! {}, "links": obj! {} },
                        "dataVersion": "1.0",
                    },
                };
                set_background(cx, &id, &mut bg)?;
                list.push(bg);
            }
        }
        zone_id = Some(id);
    }
    for c in list.iter_mut() {
        if let Some(p) = c.get_mut("position").filter(|p| truthy(p))
            && let (Some(z), Some(target)) = (p.get("zoneIndex").and_then(Value::as_f64), zf)
            && z >= target
        {
            p["zoneIndex"] = json::num_value(z + 1.0);
        }
    }
    let columns = section_columns(
        cx,
        if zone.as_str() == Some(UNDEF) {
            Value::from(UNDEF)
        } else {
            zone.clone()
        },
        zone_id.as_deref(),
    );
    let at = list
        .iter()
        .position(|c| c.get("position").is_some_and(truthy) && matches!((pos(c, "zoneIndex").and_then(|v| v.as_f64()), zf), (Some(a), Some(b)) if a >= b))
        .map(|i| i as i64)
        .unwrap_or(list.len() as i64 - 1);
    let len = list.len() as i64;
    let at = if at < 0 { (len + at).max(0) } else { at } as usize;
    for (k, c) in columns.into_iter().enumerate() {
        list.insert(at + k, c);
    }
    Request::post(format!("{}/savepage", by_url(&web, &page_name)))
        .accept("application/json;odata=nometadata")
        .header("content-type", "application/json;odata=nometadata")
        .json_response()
        .json(obj! { "CanvasContent1": stringify_js(&canvas) })
        .send()
        .await?;
    Ok(())
}

// ---- header set ------------------------------------------------------------------------------

fn header_template(kind: &str) -> Value {
    let base_props = |layout: &str| {
        obj! { "title": "", "imageSourceType": 4, "layoutType": layout, "textAlignment": "Left", "showTopicHeader": false, "showPublishDate": false, "showTimeToRead": false, "topicHeader": "" }
    };
    let spc = obj! { "htmlStrings": obj! {}, "searchablePlainTexts": obj! {}, "imageSources": obj! {}, "links": obj! {} };
    let mut h = obj! {
        "id": BANNER_WEB_PART_ID, "instanceId": BANNER_WEB_PART_ID, "title": "Title Region", "description": "Title Region Description",
        "serverProcessedContent": spc, "dataVersion": "1.4",
    };
    match kind {
        "None" => h["properties"] = base_props("NoImage"),
        "Custom" => {
            h["serverProcessedContent"]["imageSources"] = obj! { "imageSource": "" };
            h["serverProcessedContent"]["customMetadata"] = obj! { "imageSource": obj! { "siteId": "", "webId": "", "listId": "", "uniqueId": "" } };
            h["properties"] = obj! {
                "title": "", "imageSourceType": 2, "layoutType": "FullWidthImage", "textAlignment": "Left", "showTopicHeader": false, "showPublishDate": false,
                "showTimeToRead": false, "topicHeader": "", "authors": Value::Array(Vec::new()), "altText": "", "webId": "", "siteId": "", "listId": "",
                "uniqueId": "", "translateX": 0, "translateY": 0,
            };
        }
        _ => h["properties"] = base_props("FullWidthImage"),
    }
    h
}

pub async fn header_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let mut page_name = cx.u("pageName").to_lowercase();
    if !page_name.ends_with(".aspx") {
        page_name.push_str(".aspx");
    }
    let body = header_body(cx, &web, &page_name).await.map_err(|e| {
        if e.status() == Some(404) {
            Error::msg(format!("The specified page '{page_name}' does not exist."))
        } else {
            e
        }
    })?;
    // `return request.post(...)` inside the try: a rejection skips the catch
    save_draft(&web, &page_name, body)
        .send()
        .await
        .map_err(Error::uncaught)?;
    Ok(())
}

async fn header_body(cx: &Cx, web: &str, page_name: &str) -> Result<Value> {
    let ty = cx.s("type");
    let mut byline: Option<Value> = Some(Value::Array(
        match cx.s("authors").filter(|s| !s.is_empty()) {
            Some(a) => a.split(',').map(|x| Value::from(x.trim())).collect(),
            None => Vec::new(),
        },
    ));
    let mut topic: Value = Value::from(
        cx.s("topicHeader")
            .filter(|s| !s.is_empty())
            .unwrap_or_default(),
    );
    cx.verbose_log("Retrieving information about the page...");
    let url = by_url(web, page_name);
    let page = Request::get(format!("{url}?$select=IsPageCheckedOutToCurrentUser,Title"))
        .spo_json()
        .data()
        .await?;
    let mut title = page.get("Title").cloned();
    let data = if page
        .get("IsPageCheckedOutToCurrentUser")
        .is_some_and(truthy)
    {
        Request::get(format!("{url}?$expand=ListItemAllFields"))
            .spo_json()
            .data()
            .await?
    } else {
        checkout(cx, page_name, web).await?
    };
    let (mut canvas, mut banner, mut description) = (
        Some(Value::from("")),
        Some(Value::from("")),
        Some(Value::from("")),
    );
    if truthy(&data) {
        canvas = data.get("CanvasContent1").cloned();
        if byline
            .as_ref()
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty)
        {
            byline = data.get("AuthorByline").cloned();
        }
        banner = data.get("BannerImageUrl").cloned();
        description = data.get("Description").cloned();
        title = data.get("Title").cloned();
        if !truthy(&topic) {
            topic = data
                .get("TopicHeader")
                .filter(|v| truthy(v))
                .cloned()
                .unwrap_or(Value::from(""));
        }
    }
    let mut controls = parse_value(data.get("CanvasContent1"))?;
    let header_idx = match &controls {
        Value::Array(a) => a.iter().position(|c| {
            num_eq(c.get("position").and_then(|p| p.get("zoneIndex")), 1.0)
                && num_eq(c.get("position").and_then(|p| p.get("sectionFactor")), 0.0)
                && c.get("webPartId").and_then(Value::as_str) == Some(BANNER_WEB_PART_ID)
        }),
        _ => return Err(not_function("pageControls", "find")),
    };
    let lwc = data.get("LayoutWebpartsContent");
    let standard = lwc.and_then(Value::as_str) != Some("[]");
    let existing: Option<Value> = if standard && let Some(Value::String(l)) = lwc {
        dig(&json::jparse(l)?, &["0"])?.cloned()
    } else {
        header_idx.and_then(|i| controls[i].get("webPartData").cloned())
    };
    let existing = existing.filter(truthy);
    let mut header = match ty.as_deref() {
        Some(t @ ("None" | "Custom" | "Default")) => header_template(t),
        _ => existing
            .clone()
            .unwrap_or_else(|| header_template("Default")),
    };
    let ex_prop =
        |e: &Value, k: &str| -> Result<Option<Value>> { Ok(dig(e, &["properties", k])?.cloned()) };
    let set_prop = |h: &mut Value, k: &str, v: Option<Value>| -> Result<()> {
        match h.get_mut("properties") {
            Some(p) if p.is_object() => {
                assign(p, k, v);
                Ok(())
            }
            Some(Value::Null) => Err(Error::js(format!(
                "Cannot set properties of null (setting '{k}')"
            ))),
            Some(_) => Ok(()),
            None => Err(Error::js(format!(
                "Cannot set properties of undefined (setting '{k}')"
            ))),
        }
    };
    let hprop = |h: &Value, k: &str| h.get("properties").and_then(|p| get(p, k)).cloned();
    if let Some(e) = &existing
        && ty.as_deref() != Some("None")
    {
        for (opt, k) in [
            ("textAlignment", "textAlignment"),
            ("showTopicHeader", "showTopicHeader"),
            ("topicHeader", "topicHeader"),
            ("showPublishDate", "showPublishDate"),
            ("showTimeToRead", "showTimeToRead"),
            ("layout", "layoutType"),
        ] {
            if cx.val(opt).is_none() {
                let v = ex_prop(e, k)?;
                set_prop(&mut header, k, v)?;
            }
        }
        if num_eq(hprop(&header, "imageSourceType").as_ref(), 2.0)
            && num_eq(ex_prop(e, "imageSourceType")?.as_ref(), 2.0)
        {
            for (opt, k) in [
                ("altText", "altText"),
                ("translateX", "translateX"),
                ("translateY", "translateY"),
            ] {
                if cx.val(opt).is_none() {
                    let v = ex_prop(e, k)?;
                    set_prop(&mut header, k, v)?;
                }
            }
            if cx.val("imageUrl").is_none() {
                for k in ["listId", "siteId", "uniqueId", "webId"] {
                    let v = ex_prop(e, k)?;
                    set_prop(&mut header, k, v)?;
                }
                assign(
                    &mut header,
                    "serverProcessedContent",
                    e.get("serverProcessedContent").cloned(),
                );
            }
        }
    }
    set_prop(&mut header, "title", title.clone())?;
    for (opt, k) in [
        ("textAlignment", "textAlignment"),
        ("showTopicHeader", "showTopicHeader"),
        ("topicHeader", "topicHeader"),
        ("showPublishDate", "showPublishDate"),
        ("showTimeToRead", "showTimeToRead"),
    ] {
        if let Some(v) = cx.val(opt) {
            set_prop(&mut header, k, Some(v.clone()))?;
        }
    }
    if ty.as_deref() != Some("None")
        && let Some(v) = cx.val("layout")
    {
        set_prop(&mut header, "layoutType", Some(v.clone()))?;
    }
    if num_eq(hprop(&header, "imageSourceType").as_ref(), 2.0) {
        for (opt, k) in [
            ("altText", "altText"),
            ("translateX", "translateX"),
            ("translateY", "translateY"),
        ] {
            if let Some(v) = cx.val(opt) {
                set_prop(&mut header, k, Some(v.clone()))?;
            }
        }
        if let Some(img) = cx.val("imageUrl").cloned() {
            match header
                .get_mut("serverProcessedContent")
                .filter(|s| s.as_str() != Some(UNDEF))
            {
                Some(s) if s.is_object() => s["imageSources"] = obj! { "imageSource": img.clone() },
                Some(Value::Null) => {
                    return Err(Error::js(
                        "Cannot set properties of null (setting 'imageSources')",
                    ));
                }
                Some(_) => {}
                None => {
                    return Err(Error::js(
                        "Cannot set properties of undefined (setting 'imageSources')",
                    ));
                }
            }
            let (site, webid, list, unique) = if !truthy(&img) {
                (
                    Value::from(""),
                    Value::from(""),
                    Value::from(""),
                    Value::from(""),
                )
            } else {
                let site = Value::from(sp::site_id_by_sp_api(cx, web).await?);
                let webid = Value::from(sp::web_id(cx, web).await?);
                cx.verbose_log("Retrieving information about the header image...");
                let info = Request::get(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')?$select=ListId,UniqueId", eqp(&to_js_string(&img)))).spo_json().data().await?;
                (
                    site,
                    webid,
                    info.get("ListId").cloned().unwrap_or(Value::from(UNDEF)),
                    info.get("UniqueId").cloned().unwrap_or(Value::from(UNDEF)),
                )
            };
            if let Some(s) = header
                .get_mut("serverProcessedContent")
                .filter(|s| s.is_object())
            {
                s["customMetadata"] = obj! { "imageSource": obj! { "siteId": site.clone(), "webId": webid.clone(), "listId": list.clone(), "uniqueId": unique.clone() } };
            }
            for (k, v) in [
                ("listId", list),
                ("siteId", site),
                ("uniqueId", unique),
                ("webId", webid),
            ] {
                set_prop(&mut header, k, Some(v))?;
            }
        }
    }
    let mut body = obj! { "LayoutWebpartsContent": stringify_js(&Value::Array(vec![header.clone()])), "CanvasContent1": ?canvas };
    if !standard {
        body["LayoutWebpartsContent"] = Value::from("[]");
        set_prop(&mut header, "title", Some(topic.clone()))?;
        let list = as_arr_mut(&mut controls, "pageControls", "push")?;
        match header_idx {
            Some(i) => list[i]["webPartData"] = header,
            None => {
                for c in list.iter_mut() {
                    if let Some(p) = c.get_mut("position").filter(|p| p.is_object())
                        && let Some(s) = p
                            .get("sectionIndex")
                            .filter(|v| truthy(v))
                            .and_then(Value::as_f64)
                    {
                        p["sectionIndex"] = json::num_value(s + s);
                    }
                }
                list.push(obj! {
                    "id": BANNER_WEB_PART_ID, "controlType": 3, "displayMode": 2, "emphasis": obj! {},
                    "position": obj! { "zoneIndex": 1, "sectionFactor": 0, "layoutIndex": 1, "controlIndex": 1, "sectionIndex": 1 },
                    "webPartId": BANNER_WEB_PART_ID, "webPartData": header,
                });
            }
        }
        body["CanvasContent1"] = Value::from(stringify_js(&controls));
    }
    for (k, v) in [
        ("Title", title.as_ref()),
        ("TopicHeader", Some(&topic)),
        ("Description", description.as_ref()),
        ("AuthorByline", byline.as_ref()),
        ("BannerImageUrl", banner.as_ref()),
    ] {
        if let Some(v) = v.filter(|v| truthy(v)) {
            body[k] = v.clone();
        }
    }
    strip_undefined(&mut body);
    Ok(body)
}

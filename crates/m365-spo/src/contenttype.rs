//! `spo contenttype *` and `spo contenttypehub get`.

use crate::sp;
use crate::sp::{ex, ne};
use m365_core::prelude::*;
use m365_core::ut::spo;
use m365_core::ut::url as urlutil;

fn has_list(cx: &Cx) -> bool {
    ne(cx, "listId").is_some() || ne(cx, "listTitle").is_some() || ne(cx, "listUrl").is_some()
}

fn list_path(cx: &Cx, web: &str) -> String {
    sp::list_segment(cx, web, sp::LIST, "getByTitle")
        .map(|s| format!("/{s}"))
        .unwrap_or_default()
}

async fn list_id(cx: &Cx, web: &str) -> Result<String> {
    sp::list_id(
        cx,
        web,
        ne(cx, "listTitle").as_deref(),
        ne(cx, "listUrl").as_deref(),
    )
    .await
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let parent = if !has_list(cx) {
        r#"<Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" />"#.to_string()
    } else {
        let site = sp::site_id_by_sp_api(cx, &web).await?;
        let wid = sp::web_id(cx, &web).await?;
        let lid = match ne(cx, "listId") {
            Some(l) => l,
            None => list_id(cx, &web).await?,
        };
        format!(
            r#"<Identity Id="5" Name="1a48869e-c092-0000-1f61-81ec89809537|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{site}:web:{wid}:list:{lid}" />"#
        )
    };
    cx.verbose_log("Retrieving request digest...");
    let d = sp::digest(&web).await?;
    let desc = match ne(cx, "description") {
        Some(s) => format!(
            r#"<Property Name="Description" Type="String">{}</Property>"#,
            util::escape_xml(&s)
        ),
        None => r#"<Property Name="Description" Type="Null" />"#.into(),
    };
    let group = match ne(cx, "group") {
        Some(s) => format!(
            r#"<Property Name="Group" Type="String">{}</Property>"#,
            util::escape_xml(&s)
        ),
        None => r#"<Property Name="Group" Type="Null" />"#.into(),
    };
    let body = sp::request(
        r#"<ObjectPath Id="8" ObjectPathId="7" /><ObjectPath Id="10" ObjectPathId="9" /><ObjectIdentityQuery Id="11" ObjectPathId="9" />"#,
        &format!(
            r#"<Property Id="7" ParentId="5" Name="ContentTypes" /><Method Id="9" ParentId="7" Name="Add"><Parameters><Parameter TypeId="{{168f3091-4554-4f14-8866-b20d48e45b54}}">{desc}{group}<Property Name="Id" Type="String">{}</Property><Property Name="Name" Type="String">{}</Property><Property Name="ParentContentType" Type="Null" /></Parameter></Parameters></Method>{parent}"#,
            ex(cx, "id"),
            ex(cx, "name")
        ),
    );
    let json = sp::post(&web, &d, body).await?;
    sp::check_first(&json)?;
    let o = obj! {
        "webUrl": web.as_str(), "listTitle": ?cx.val("listTitle").cloned(), "listUrl": ?cx.val("listUrl").cloned(), "listId": ?cx.val("listId").cloned(),
        "id": ?cx.val("id").cloned(), "output": "json", "debug": cx.debug(), "verbose": cx.verbose(),
    };
    let (out, err) = m365_core::runner::run_nested(
        "spo contenttype get",
        o.as_object().cloned().unwrap_or_default(),
        true,
    )
    .await?;
    if cx.debug() {
        cx.stderr(&err);
    }
    cx.log(json::jparse(&out)?)
}

pub async fn field_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let who = ne(cx, "contentTypeId")
        .or_else(|| cx.s("contentTypeName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Retrieving fields for content type '{who}' in site {web}..."
    ));
    let mut url = format!("{web}/_api/web{}/contentTypes", list_path(cx, &web));
    let id = match ne(cx, "contentTypeId") {
        Some(i) => i,
        None => {
            let n = cx.u("contentTypeName");
            cx.verbose_log(&format!(
                "Retrieving content type id for content type '{n}'."
            ));
            let r = odata::get_all_items(&format!(
                "{url}?$filter=Name eq '{}'&$select=StringId",
                eqp(&n)
            ))
            .await?;
            match r.first() {
                Some(f) => u(f, "StringId"),
                None => return Err(Error::msg(format!("Content type with name {n} not found."))),
            }
        }
    };
    url.push_str(&format!("('{}')/fields", eqp(&id)));
    if let Some(p) = ne(cx, "properties") {
        url.push_str(&format!("?$select={p}"));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn field_remove(cx: &mut Cx) -> Result<()> {
    let (id, ct) = (cx.u("id"), cx.u("contentTypeId"));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the column {id} from content type {ct}?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    cx.debug_log("Get SiteId required by ProcessQuery endpoint.");
    let site = sp::site_id_by_sp_api(cx, &web).await?;
    cx.debug_log(&format!("SiteId: {site}"));
    cx.debug_log("Get WebId required by ProcessQuery endpoint.");
    let wid = sp::web_id(cx, &web).await?;
    cx.debug_log(&format!("WebId: {wid}"));
    let mut lid = ne(cx, "listId");
    if ne(cx, "listTitle").is_some() || ne(cx, "listUrl").is_some() {
        lid = Some(list_id(cx, &web).await?);
    }
    cx.debug_log(&format!(
        "ListId: {}",
        lid.clone().unwrap_or_else(|| "undefined".into())
    ));
    let d = sp::digest(&web).await?;
    let update_children =
        !has_list(cx) && cx.val("updateChildContentTypes") == Some(&Value::Bool(true));
    if cx.debug() {
        let extra = if let Some(t) = ne(cx, "listTitle") {
            format!("; ListTitle='{t}'")
        } else if let Some(t) = ne(cx, "listId") {
            format!("; ListId='{t}'")
        } else if let Some(t) = ne(cx, "listUrl") {
            format!("; ListUrl='{t}'")
        } else {
            format!(" ; UpdateChildContentTypes='{update_children}")
        };
        cx.stderr(&format!(
            "Remove FieldLink from ContentType. Id='{id}' ; ContentTypeId='{ct}' {extra}"
        ));
        cx.stderr("Execute ProcessQuery.");
    }
    let (eid, ect) = (util::escape_xml(&id), util::escape_xml(&ct));
    let body = match lid.filter(|l| !l.is_empty()) {
        Some(l) => format!(
            r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName=".NET Library" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="18" ObjectPathId="17" /><ObjectPath Id="20" ObjectPathId="19" /><Method Name="DeleteObject" Id="21" ObjectPathId="19" /><Method Name="Update" Id="22" ObjectPathId="15"><Parameters><Parameter Type="Boolean">{update_children}</Parameter></Parameters></Method></Actions><ObjectPaths><Property Id="17" ParentId="15" Name="FieldLinks" /><Method Id="19" ParentId="17" Name="GetById"><Parameters><Parameter Type="Guid">{{{eid}}}</Parameter></Parameters></Method><Identity Id="15" Name="09eec89e-709b-0000-558c-c222dcaf9162|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{site}:web:{wid}:list:{l}:contenttype:{ect}" /></ObjectPaths></Request>"#
        ),
        None => format!(
            r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName=".NET Library" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="77" ObjectPathId="76" /><ObjectPath Id="79" ObjectPathId="78" /><Method Name="DeleteObject" Id="80" ObjectPathId="78" /><Method Name="Update" Id="81" ObjectPathId="24"><Parameters><Parameter Type="Boolean">{update_children}</Parameter></Parameters></Method></Actions><ObjectPaths><Property Id="76" ParentId="24" Name="FieldLinks" /><Method Id="78" ParentId="76" Name="GetById"><Parameters><Parameter Type="Guid">{{{eid}}}</Parameter></Parameters></Method><Identity Id="24" Name="6b3ec69e-00a7-0000-55a3-61f8d779d2b3|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{site}:web:{wid}:contenttype:{ect}" /></ObjectPaths></Request>"#
        ),
    };
    let json = sp::post(&web, &d, body).await?;
    sp::check_first(&json)
}

async fn post_csom(web: &str, digest: Option<&str>, body: String) -> Result<()> {
    // axios sends `X-RequestDigest: undefined` as an empty header
    let r = Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
        .text(body)
        .header("X-RequestDigest", digest.unwrap_or(""));
    let text = r.send().await?.text();
    sp::check_first(&sp::parse(&text)?)
}

pub async fn field_set(cx: &mut Cx) -> Result<()> {
    match field_set_inner(cx).await {
        Err(Error::Thrown(Value::String(s))) if s == "DONE" => Ok(()),
        r => r,
    }
}

async fn field_set_inner(cx: &mut Cx) -> Result<()> {
    let (web, id, ct) = (cx.u("webUrl"), cx.u("id"), cx.u("contentTypeId"));
    let mut digest: Option<String> = None;
    let mut site: Option<String> = None;
    let mut wid: Option<String> = None;
    cx.verbose_log(&format!("Retrieving field link for field {id}..."));
    let fl_url = format!(
        "{web}/_api/web/contenttypes('{}')/fieldlinks('{id}')",
        eqp(&ct)
    );
    let fl = Request::get(&fl_url).spo_json().data().await?;
    let mut link = None;
    if fl.get("odata.null") != Some(&Value::Bool(true)) {
        cx.verbose_log("Field link found");
        link = Some(fl);
    } else {
        cx.verbose_log("Field link not found. Creating...");
        cx.verbose_log(&format!("Retrieving information about site column {id}..."));
        let f = Request::get(format!(
            "{web}/_api/web/fields('{id}')?$select=SchemaXmlWithResourceTokens"
        ))
        .spo_json()
        .data()
        .await?;
        let schema = f
            .get("SchemaXmlWithResourceTokens")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        // createFieldLink
        let re = regex::Regex::new(r"(<Field[^>]+>)(.*)").unwrap();
        let Some(m) = re.captures(&schema) else {
            return Err(Error::js("Cannot read properties of null (reading '1')"));
        };
        let mut x = m[1].to_string();
        let rest = m.get(2).map_or("", |g| g.as_str()).to_string();
        let ad = regex::Regex::new(r#"AllowDeletion="([^"]+)""#).unwrap();
        let mut requires = false;
        match ad
            .captures(&x)
            .map(|c| (c[0].to_string(), c[1].to_string()))
        {
            None => {
                requires = true;
                x = x.replacen('>', " AllowDeletion=\"TRUE\">", 1) + &rest;
            }
            Some((all, v)) if v != "TRUE" => {
                requires = true;
                x = x.replacen(&all, "AllowDeletion=\"TRUE\"", 1) + &rest;
            }
            _ => {}
        }
        if !requires {
            cx.verbose_log(&format!("Schema of field {id} is already up-to-date"));
        } else {
            ensure_digest(cx, &web, &mut digest).await?;
            cx.verbose_log("Updating field schema...");
            Request::post(format!("{web}/_api/web/fields('{id}')"))
                .spo_json()
                .header("content-type", "application/json;odata=nometadata")
                .header("X-HTTP-Method", "MERGE")
                .header("x-requestdigest", digest.clone().unwrap_or_default())
                .json(obj! { "SchemaXml": x })
                .send()
                .await?;
        }
        site = Some(sp::site_id_by_sp_api(cx, &web).await?);
        wid = Some(sp::web_id(cx, &web).await?);
        ensure_digest(cx, &web, &mut digest).await?;
        let (s, w) = (site.clone().unwrap(), wid.clone().unwrap());
        post_csom(
            &web,
            digest.as_deref(),
            sp::request(
                r#"<ObjectPath Id="5" ObjectPathId="4" /><ObjectIdentityQuery Id="6" ObjectPathId="4" /><Method Name="Update" Id="7" ObjectPathId="1"><Parameters><Parameter Type="Boolean">true</Parameter></Parameters></Method>"#,
                &format!(
                    r#"<Identity Id="2" Name="d6667b9e-50fb-0000-2693-032ae7a0df25|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{s}:web:{w}:field:{id}" /><Method Id="4" ParentId="3" Name="Add"><Parameters><Parameter TypeId="{{63fb2c92-8f65-4bbb-a658-b6cd294403f4}}"><Property Name="Field" ObjectPathId="2" /></Parameter></Parameters></Method><Identity Id="1" Name="d6667b9e-80f4-0000-2693-05528ff416bf|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{s}:web:{w}:contenttype:{}" /><Property Id="3" ParentId="1" Name="FieldLinks" />"#,
                    util::escape_xml(&ct)
                ),
            ),
        )
        .await?;
    }
    let link = match link {
        Some(l) if truthy(&l) => l,
        _ => {
            cx.verbose_log(&format!(
                "Retrieving information about field link for field {id}..."
            ));
            let r = Request::get(&fl_url).spo_json().data().await?;
            if truthy(&r) && r.get("odata.null") != Some(&Value::Bool(true)) {
                r
            } else {
                return Err(Error::msg(format!(
                    "Couldn't find field link for field {id}"
                )));
            }
        }
    };
    let differs = |k: &str, opt: &str| cx.val(opt).is_some_and(|v| link.get(k) != Some(v));
    let (uh, ur) = (differs("Hidden", "hidden"), differs("Required", "required"));
    if !uh && !ur {
        cx.verbose_log("Field link already up-to-date");
        return Err(Error::Thrown(Value::from("DONE")));
    }
    let s = match site {
        Some(s) if !s.is_empty() => s,
        _ => sp::site_id_by_sp_api(cx, &web).await?,
    };
    let w = match wid {
        Some(w) if !w.is_empty() => w,
        _ => sp::web_id(cx, &web).await?,
    };
    cx.verbose_log("Updating field link...");
    let req = if ur {
        format!(
            r#"<SetProperty Id="122" ObjectPathId="121" Name="Required"><Parameter Type="Boolean">{}</Parameter></SetProperty>"#,
            to_js_string(cx.val("required").unwrap())
        )
    } else {
        String::new()
    };
    let hid = if uh {
        format!(
            r#"<SetProperty Id="123" ObjectPathId="121" Name="Hidden"><Parameter Type="Boolean">{}</Parameter></SetProperty>"#,
            to_js_string(cx.val("hidden").unwrap())
        )
    } else {
        String::new()
    };
    let ect = util::escape_xml(&ct);
    let flid = link
        .get("Id")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    post_csom(
        &web,
        digest.as_deref(),
        sp::request(
            &format!(r#"{req}{hid}<Method Name="Update" Id="124" ObjectPathId="19"><Parameters><Parameter Type="Boolean">true</Parameter></Parameters></Method>"#),
            &format!(r#"<Identity Id="121" Name="716a7b9e-3012-0000-22fb-84acfcc67d04|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{s}:web:{w}:contenttype:{ect}:fl:{flid}" /><Identity Id="19" Name="716a7b9e-3012-0000-22fb-84acfcc67d04|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{s}:web:{w}:contenttype:{ect}" />"#),
        ),
    )
    .await
}

async fn ensure_digest(cx: &Cx, web: &str, d: &mut Option<String>) -> Result<()> {
    if d.as_deref().is_some_and(|s| !s.is_empty()) {
        cx.debug_log("Request digest already present");
        return Ok(());
    }
    cx.debug_log("Retrieving request digest...");
    *d = Some(sp::digest(web).await?);
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let mut url = format!("{web}/_api/web{}/contenttypes", list_path(cx, &web));
    let (id, name) = (ne(cx, "id"), ne(cx, "name"));
    if let Some(i) = &id {
        url.push_str(&format!("('{}')?$expand=Parent", eqp(i)));
    } else if let Some(n) = &name {
        url.push_str(&format!("?$filter=Name eq '{}'&$expand=Parent", eqp(n)));
    }
    let mut r = Request::get(url).spo_json().data().await?;
    let mut err = String::new();
    if let Some(n) = &name {
        let v = need_arr(&r, "value", "length")?;
        if v.is_empty() {
            err = format!("Content type with name {n} not found");
        } else {
            r = v[0].clone();
        }
    }
    if let Some(i) = &id
        && r.get("odata.null") == Some(&Value::Bool(true))
    {
        err = format!("Content type with ID {i} not found");
    }
    if !err.is_empty() {
        return Err(Error::msg(err));
    }
    cx.log(r)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{}/_api/web/ContentTypes?$expand=Parent", cx.u("webUrl"));
    if let Some(c) = ne(cx, "category") {
        url.push_str(&format!("&$filter=Group eq '{}'", eqp(&c)));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let id = ne(cx, "id");
    let label = match &id {
        Some(i) => format!("with id {i}"),
        None => format!(
            "with name {}",
            cx.s("name").unwrap_or_else(|| "undefined".into())
        ),
    };
    let who = id
        .clone()
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the content type {who}?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    cx.debug_log(&format!(
        "Retrieving information about the content type {label}..."
    ));
    let found = match &id {
        Some(i) => obj! { "value": vec![obj! { "StringId": i.as_str() }] },
        None => {
            cx.verbose_log(&format!("Looking up the ID of content type {label}..."));
            Request::get(format!(
                "{web}/_api/web/availableContentTypes?$filter=(Name eq '{}')",
                eqp(&cx.u("name"))
            ))
            .spo_json()
            .data()
            .await?
        }
    };
    let first = found
        .get("value")
        .and_then(Value::as_array)
        .and_then(|v| v.first())
        .cloned();
    let r = match first {
        Some(f) => {
            Request::post(format!(
                "{web}/_api/web/contenttypes('{}')",
                eqp(&u(&f, "StringId"))
            ))
            .header("X-HTTP-Method", "DELETE")
            .header("If-Match", "*")
            .spo_json()
            .data()
            .await?
        }
        None => obj! { "odata.null": true },
    };
    if r.get("odata.null") == Some(&Value::Bool(true)) {
        return Err(Error::msg("Content type not found"));
    }
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let ctid = match ne(cx, "id") {
        Some(i) => i,
        None => {
            cx.verbose_log("Retrieving content type to update...");
            let mut url = format!("{web}/_api/Web");
            if let Some(i) = ne(cx, "listId") {
                url.push_str(&format!("/Lists/GetById('{}')", eqp(&i)));
            } else if let Some(t) = ne(cx, "listTitle") {
                url.push_str(&format!("/Lists/GetByTitle('{}')", eqp(&t)));
            } else if let Some(l) = ne(cx, "listUrl") {
                url.push_str(&format!(
                    "/GetList('{}')",
                    eqp(&urlutil::get_server_relative_path(&web, &l))
                ));
            }
            let n = cx.u("name");
            url.push_str(&format!(
                "/ContentTypes?$filter=Name eq '{}'&$select=Id",
                eqp(&n)
            ));
            let r = Request::get(url).spo_json().data().await?;
            let v = need_arr(&r, "value", "length")?;
            match v.first() {
                None => {
                    return Err(Error::msg(format!(
                        "The specified content type '{n}' does not exist"
                    )));
                }
                Some(f) => match f.get("Id") {
                    Some(i) => i
                        .get("StringValue")
                        .map(to_js_string)
                        .unwrap_or_else(|| "undefined".into()),
                    None => {
                        return Err(Error::js(
                            "Cannot read properties of undefined (reading 'StringValue')",
                        ));
                    }
                },
            }
        }
    };
    let site = sp::site_id_by_sp_api(cx, &web).await?;
    let wid = sp::web_id(cx, &web).await?;
    cx.verbose_log("Updating content type...");
    const EXCLUDE: [&str; 11] = [
        "webUrl",
        "id",
        "name",
        "listTitle",
        "listId",
        "listUrl",
        "query",
        "debug",
        "verbose",
        "output",
        "updateChildren",
    ];
    let mut i = 12;
    let mut payload = String::new();
    for (k, v) in &cx.opts {
        if !EXCLUDE.contains(&k.as_str()) {
            payload.push_str(&format!(r#"<SetProperty Id="{i}" ObjectPathId="9" Name="{k}"><Parameter Type="String">{}</Parameter></SetProperty>"#, util::escape_xml_value(Some(v))));
            i += 1;
        }
    }
    payload.push_str(&format!(r#"<Method Name="Update" Id="{i}" ObjectPathId="9"><Parameters><Parameter Type="Boolean">{}</Parameter></Parameters></Method>"#, cx.bool("updateChildren")));
    let list = if !has_list(cx) {
        String::new()
    } else if let Some(l) = ne(cx, "listId") {
        format!(":list:{l}")
    } else {
        format!(":list:{}", list_id(cx, &web).await?)
    };
    let body = sp::request(
        &payload,
        &format!(
            r#"<Identity Id="9" Name="fc4179a0-e0d7-5000-c38b-bc3506fbab6f|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{site}:web:{wid}{list}:contenttype:{}" />"#,
            util::escape_xml(&ctid)
        ),
    );
    let text = Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
        .header("Content-Type", "text/xml")
        .text(body)
        .send()
        .await?
        .text();
    sp::check_first(&sp::parse(&text)?)
}

pub async fn sync(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let url = url::Url::parse(&web).map_err(|_| Error::js("Invalid URL"))?;
    let origin = url.origin().ascii_serialization();
    let host = url
        .host_str()
        .map(|h| match url.port() {
            Some(p) => format!("{h}:{p}"),
            None => h.to_string(),
        })
        .unwrap_or_default();
    let site = sp::site_id_by_sp_api(cx, &web).await?;
    let wid = sp::web_id(cx, &web).await?;
    let target = match ne(cx, "listId") {
        Some(l) => Some(l),
        None if ne(cx, "listTitle").is_some() || ne(cx, "listUrl").is_some() => {
            Some(list_id(cx, &web).await?).filter(|s| !s.is_empty())
        }
        None => None,
    };
    let lp = target.map(|t| format!("/lists/{t}")).unwrap_or_default();
    let ctid = match ne(cx, "id") {
        Some(i) => i,
        None => {
            cx.verbose_log("Retrieving content type Id by name...");
            let n = cx.u("name");
            let r = odata::get_all_items(&format!(
                "{web}/_api/web/AvailableContentTypes?$filter=Name eq '{}'&$select=StringId,Name",
                eqp(&n)
            ))
            .await?;
            match r.first() {
                Some(f) => u(f, "StringId"),
                None => return Err(Error::msg(format!("Content type with name {n} not found."))),
            }
        }
    };
    cx.verbose_log("Adding or syncing the content type...");
    let r = Request::post(format!(
        "{origin}/_api/v2.0/sites/{host},{site},{wid}{lp}/contenttypes/addCopyFromContentTypeHub"
    ))
    .accept("application/json;odata.metadata=minimal;odata.streaming=true;IEEE754Compatible=false")
    .json_response()
    .json(obj! { "contentTypeId": ctid })
    .data()
    .await?;
    if matches!(r, Value::Object(_) | Value::Array(_) | Value::Null) {
        cx.log(r)?;
    }
    Ok(())
}

pub async fn hub_get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    cx.verbose_log("Retrieving Content Type Hub URL");
    let body = format!(
        "<Request xmlns=\"http://schemas.microsoft.com/sharepoint/clientquery/2009\" AddExpandoFieldTypeSuffix=\"true\" SchemaVersion=\"15.0.0.0\" LibraryVersion=\"16.0.0.0\" ApplicationName=\"{}\">
<Actions>
  <ObjectPath Id=\"2\" ObjectPathId=\"1\" />
  <ObjectIdentityQuery Id=\"3\" ObjectPathId=\"1\" />
  <ObjectPath Id=\"5\" ObjectPathId=\"4\" />
  <ObjectIdentityQuery Id=\"6\" ObjectPathId=\"4\" />
  <Query Id=\"7\" ObjectPathId=\"4\">
    <Query SelectAllProperties=\"false\">
      <Properties>
        <Property Name=\"ContentTypePublishingHub\" ScalarProperty=\"true\" />
      </Properties>
    </Query>
  </Query>
</Actions>
<ObjectPaths>
  <StaticMethod Id=\"1\" Name=\"GetTaxonomySession\" TypeId=\"{{981cbc68-9edc-4f8d-872f-71146fcbb84f}}\" />
  <Method Id=\"4\" ParentId=\"1\" Name=\"GetDefaultSiteCollectionTermStore\" />
</ObjectPaths>
</Request>",
        spo::application_name()
    );
    let json = sp::post(&spo_url, &d, body).await?;
    sp::check_first(&json)?;
    let last = json.last().ok_or_else(|| {
        Error::js("Cannot read properties of undefined (reading 'ContentTypePublishingHub')")
    })?;
    cx.log(obj! { "ContentTypePublishingHub": ?last.get("ContentTypePublishingHub").cloned() })
}

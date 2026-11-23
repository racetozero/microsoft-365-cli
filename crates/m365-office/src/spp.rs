//! `spp *`: SharePoint Premium (Syntex) models, content centers and autofill columns.

use m365_core::prelude::*;
use m365_core::ut::{spo, url as urlutil};

/// Writes to stdout when verbose (these commands use `logger.log`).
fn vlog(cx: &Cx, m: &str) -> Result<()> {
    if cx.verbose() {
        cx.log(Value::from(m))?;
    }
    Ok(())
}

async fn assert_content_center(cx: &Cx, site: &str) -> Result<()> {
    vlog(
        cx,
        &format!("Checking if '{site}' is a valid content center site..."),
    )?;
    let r = Request::get(format!("{site}/_api/web?$select=WebTemplateConfiguration"))
        .spo_json()
        .data()
        .await?;
    if s(&r, "WebTemplateConfiguration") != "CONTENTCTR#0" {
        return Err(Error::js(format!("{site} is not a content site.")));
    }
    Ok(())
}

fn classifier(title: &str) -> String {
    let t = title.to_lowercase();
    if t.ends_with(".classifier") {
        t
    } else {
        format!("{t}.classifier")
    }
}

async fn model(cx: &Cx, site: &str) -> Result<Value> {
    vlog(cx, "Retrieving model information...")?;
    match cx.s("title") {
        Some(t) => {
            let r = Request::get(format!(
                "{site}/_api/machinelearning/models/getbytitle('{}')",
                eqp(&classifier(&t))
            ))
            .spo_json()
            .data()
            .await?;
            if r.get("odata.null") == Some(&Value::Bool(true)) {
                return Err(Error::js(format!("Model '{t}' was not found.")));
            }
            Ok(r)
        }
        None => {
            Request::get(format!(
                "{site}/_api/machinelearning/models/getbyuniqueid('{}')",
                cx.u("id")
            ))
            .spo_json()
            .data()
            .await
        }
    }
}

fn list_url(web: &str, cx: &Cx) -> String {
    let mut u = format!("{web}/_api/web");
    if let Some(i) = cx.s("listId") {
        u.push_str(&format!("/lists(guid'{}')", eqp(&i)));
    } else if let Some(t) = cx.s("listTitle") {
        u.push_str(&format!("/lists/getByTitle('{}')", eqp(&t)));
    } else if let Some(l) = cx.s("listUrl") {
        u.push_str(&format!(
            "/GetList('{}')",
            eqp(&urlutil::get_server_relative_path(web, &l))
        ));
    }
    u
}

pub async fn autofillcolumn_set(cx: &mut Cx) -> Result<()> {
    vlog(cx, "Applying an autofill column to a column...")?;
    let site = urlutil::remove_trailing_slashes(&cx.u("siteUrl"));
    let list = Request::get(format!("{}?$select=Id,BaseType", list_url(&site, cx)))
        .spo_json()
        .data()
        .await?;
    if list.get("BaseType").and_then(Value::as_f64) != Some(1.0) {
        return Err(Error::js("The specified list is not a document library."));
    }
    let lid = u(&list, "Id");
    let field = match cx.s("columnId") {
        Some(c) => format!("/getbyid('{}')", eqp(&c)),
        None => format!(
            "/getbyinternalnameortitle('{}')",
            eqp(&cx
                .s("columnTitle")
                .or_else(|| cx.s("columnInternalName"))
                .unwrap_or_default())
        ),
    };
    let col = Request::get(format!("{site}/_api/web/lists(guid'{}')/fields{field}?&$select=Id,Title,FieldTypeKind,AutofillInfo", eqp(&lid))).spo_json().data().await?;
    let kind = col.get("FieldTypeKind").and_then(Value::as_f64);
    if !kind.is_some_and(|k| {
        [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 9.0, 10.0, 11.0, 12.0, 15.0, 16.0,
        ]
        .contains(&k)
    }) {
        return Err(Error::js("The specified column has incorrect type."));
    }
    if let Some(info) = col.get("AutofillInfo").filter(|v| truthy(v)) {
        let info = json::jparse(&to_js_string(info))?;
        let llm = info
            .get("LLM")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'Prompt')"))?;
        let data = obj! {
            "autofillPrompt": cx.val("prompt").cloned().or_else(|| llm.get("Prompt").cloned()).unwrap_or(Value::Null),
            "columnId": ?col.get("Id").cloned(),
            "docLibId": format!("{{{lid}}}"),
            "isEnabled": cx.val("isEnabled").cloned().or_else(|| llm.get("IsEnabled").cloned()).unwrap_or(Value::Null),
        };
        Request::post(format!("{site}/_api/machinelearning/SetColumnLLMInfo"))
            .spo_json()
            .json(data)
            .send()
            .await?;
        return Ok(());
    }
    let Some(prompt) = cx.s("prompt") else {
        return Err(Error::js(
            "The prompt parameter is required when setting the autofill column for the first time.",
        ));
    };
    let prompts = obj! { "columnId": ?col.get("Id").cloned(), "columnName": ?col.get("Title").cloned(), "prompt": prompt, "isEnabled": cx.opt_bool("isEnabled").unwrap_or(true) };
    Request::post(format!("{site}/_api/machinelearning/SetSyntexPoweredColumnPrompts"))
        .accept("application/json;odata=nometadata")
        .json(obj! { "docLibId": format!("{{{lid}}}"), "syntexPoweredColumnPrompts": stringify(&Value::Array(vec![prompts])) })
        .send()
        .await?;
    Ok(())
}

pub async fn contentcenter_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of content centers...");
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
    let mut all = Vec::new();
    let mut start = "0".to_string();
    loop {
        let body = format!(
            r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties /></ChildItemQuery></Query></Actions><ObjectPaths><Constructor Id="1" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /><Method Id="3" ParentId="1" Name="GetSitePropertiesFromSharePointByFilters"><Parameters><Parameter TypeId="{{b92aeee2-c92c-4b67-abcc-024e471bc140}}"><Property Name="Filter" Type="String"></Property><Property Name="IncludeDetail" Type="Boolean">false</Property><Property Name="IncludePersonalSite" Type="Enum">0</Property><Property Name="StartIndex" Type="String">{start}</Property><Property Name="Template" Type="String">CONTENTCTR#0</Property></Parameter></Parameters></Method></ObjectPaths></Request>"#,
            spo::application_name()
        );
        let text = spo::process_query(&admin, &d.value, body).await?;
        let json = spo::parse_csom(&text)?;
        if let Some(e) = spo::csom_error(&json) {
            return Err(Error::Thrown(e));
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

pub async fn model_apply(cx: &mut Cx) -> Result<()> {
    let center = urlutil::remove_trailing_slashes(&cx.u("contentCenterUrl"));
    assert_content_center(cx, &center).await?;
    let m = model(cx, &center).await?;
    vlog(cx, "Retrieving list information...")?;
    let web = cx.u("webUrl");
    let list = Request::get(format!(
        "{}?$select=BaseType,RootFolder/ServerRelativeUrl&$expand=RootFolder",
        list_url(&web, cx)
    ))
    .spo_json()
    .data()
    .await?;
    if list.get("BaseType").and_then(Value::as_f64) != Some(1.0) {
        return Err(Error::msg("The specified list is not a document library."));
    }
    let root = list
        .get("RootFolder")
        .ok_or_else(|| {
            Error::js("Cannot read properties of undefined (reading 'ServerRelativeUrl')")
        })?
        .get("ServerRelativeUrl")
        .cloned()
        .unwrap_or(Value::Null);
    vlog(
        cx,
        &format!(
            "Applying model '{}' to document library '{}'...",
            u(&m, "ModelName"),
            if root.is_null() {
                "undefined".into()
            } else {
                to_js_string(&root)
            }
        ),
    )?;
    let data = obj! {
        "__metadata": obj! { "type": "Microsoft.Office.Server.ContentCenter.SPMachineLearningPublicationsEntityData" },
        "Publications": obj! { "results": [obj! {
            "ModelUniqueId": ?m.get("UniqueId").cloned(),
            "TargetSiteUrl": web.as_str(),
            "TargetWebServerRelativeUrl": urlutil::get_server_relative_site_url(&web),
            "TargetLibraryServerRelativeUrl": root,
            "ViewOption": cx.s("viewOption").unwrap_or_else(|| "NewViewAsDefault".into()),
        }] },
    };
    let r = Request::post(format!("{center}/_api/machinelearning/publications"))
        .spo_json()
        .header("Content-Type", "application/json;odata=verbose")
        .json(data)
        .data()
        .await?;
    if let Some(m) = r
        .get("Details")
        .filter(|d| truthy(d))
        .and_then(|d| d.get(0))
        .and_then(|d| d.get("ErrorMessage"))
        .filter(|m| truthy(m))
    {
        return Err(Error::Thrown(m.clone()));
    }
    Ok(())
}

pub async fn model_get(cx: &mut Cx) -> Result<()> {
    let site = urlutil::remove_trailing_slashes(&cx.u("siteUrl"));
    assert_content_center(cx, &site).await?;
    let mut m = model(cx, &site).await?;
    if cx.bool("withPublications") {
        vlog(cx, "Retrieving publications for model...")?;
        let p = odata::get_all_items(&format!(
            "{site}/_api/machinelearning/publications/getbymodeluniqueid('{}')",
            u(&m, "UniqueId")
        ))
        .await?;
        m.as_object_mut()
            .map(|o| o.insert("Publications".into(), Value::Array(p)));
    }
    let mut o = m.as_object().cloned().unwrap_or_default();
    for k in [
        "ConfidenceScore",
        "Explanations",
        "Schemas",
        "ModelSettings",
    ] {
        let v = match o.get(k).filter(|v| truthy(v)) {
            Some(v) => json::jparse(&to_js_string(v))?,
            None => Value::Null,
        };
        o.insert(k.into(), v);
    }
    cx.log(Value::Object(o))
}

pub async fn model_list(cx: &mut Cx) -> Result<()> {
    vlog(
        cx,
        &format!("Retrieving models from {}...", cx.u("siteUrl")),
    )?;
    let site = urlutil::remove_trailing_slashes(&cx.u("siteUrl"));
    assert_content_center(cx, &site).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!("{site}/_api/machinelearning/models")).await?,
    ))
}

pub async fn model_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .str("title")
        .or_else(|| cx.str("id"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove model '{what}'?"),
    )? {
        return Ok(());
    }
    vlog(cx, &format!("Removing model from {}...", cx.u("siteUrl")))?;
    let site = urlutil::remove_trailing_slashes(&cx.u("siteUrl"));
    assert_content_center(cx, &site).await?;
    let url = match cx.s("title") {
        Some(t) => format!(
            "{site}/_api/machinelearning/models/getbytitle('{}')",
            eqp(&classifier(&t))
        ),
        None => format!(
            "{site}/_api/machinelearning/models/getbyuniqueid('{}')",
            cx.u("id")
        ),
    };
    let r = Request::delete(url)
        .spo_json()
        .header("if-match", "*")
        .data()
        .await?;
    if r.get("odata.null") == Some(&Value::Bool(true)) {
        return Err(Error::msg("Model not found."));
    }
    Ok(())
}

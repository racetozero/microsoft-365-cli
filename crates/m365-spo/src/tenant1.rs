//! `spo homesite *`, `spo hidedefaultthemes *`, `spo orgassetslibrary *`,
//! `spo orgnewssite *` and `spo knowledgehub *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::url as urlutil;
use m365_core::ut::{entra, spo};

const TENANT: &str = r#"<Constructor Id="{id}" TypeId="{268004ae-ef6b-4e9b-8425-127220d84719}" />"#;

fn tenant(id: u32) -> String {
    TENANT.replace("{id}", &id.to_string())
}

async fn audience_ids(names: &str) -> Result<Vec<String>> {
    let mut ids = Vec::new();
    for n in names.split(',') {
        ids.push(entra::group::id_by_display_name(n.trim()).await?);
    }
    Ok(ids)
}

pub async fn homesite_add(cx: &mut Cx) -> Result<()> {
    // resolved outside the command's try/catch
    let audiences: Vec<String> = if let Some(i) = cx.s("audienceIds") {
        i.split(',').map(|x| x.trim().to_string()).collect()
    } else if let Some(n) = cx.s("audienceNames") {
        audience_ids(&n).await.map_err(Error::uncaught)?
    } else {
        Vec::new()
    };
    let admin = spo::get_spo_admin_url(cx.verbose()).await?;
    cx.verbose_log(&format!("Adding home site with URL: {}...", cx.u("url")));
    let data = obj! {
        "siteUrl": ?cx.val("url").cloned(),
        "audiences": audiences,
        "vivaConnectionsDefaultStart": cx.opt_bool("vivaConnectionsDefaultStart").unwrap_or(true),
        "isInDraftMode": cx.opt_bool("isInDraftMode").unwrap_or(false),
        "order": ?cx.val("order").cloned(),
    };
    cx.log(
        Request::post(format!("{admin}/_api/SPHSite/AddHomeSite"))
            .spo_json()
            .json(data)
            .data()
            .await?,
    )
}

pub async fn homesite_get(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.verbose()).await?;
    if cx.verbose() {
        cx.log(Value::from("Retrieving home sites..."))?;
    }
    let sites =
        odata::get_all_items(&format!("{admin}/_api/SPO.Tenant/GetTargetedSitesDetails")).await?;
    let url = cx.u("url");
    let want = urlutil::remove_trailing_slashes(&url).to_lowercase();
    match sites
        .into_iter()
        .find(|s| urlutil::remove_trailing_slashes(&u(s, "Url")).to_lowercase() == want)
    {
        Some(s) => cx.log(s),
        None => Err(Error::msg(format!("Home site with URL '{url}' not found."))),
    }
}

pub async fn homesite_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.verbose()).await?;
    cx.verbose_log("Retrieving all home sites...");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{admin}/_api/SPO.Tenant/GetTargetedSitesDetails")).await?,
    ))
}

pub async fn homesite_remove(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove '{url}' as home site?"),
    )? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let props = sp::site_admin_properties(cx, &url, false).await?;
    cx.verbose_log(&format!("Removing '{url}' as home site..."));
    Request::post(format!("{admin}/_api/SPO.Tenant/RemoveTargetedSite"))
        .accept("application/json;odata=nometadata")
        .json(obj! { "siteId": ?props.get("SiteId").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn homesite_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Configuring SharePoint home site: {}...",
        cx.u("url")
    ));
    cx.verbose_log("Attempting to retrieve the SharePoint admin URL.");
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let mut c = Obj::new();
    if let Some(v) = cx.val("vivaConnectionsDefaultStart") {
        c.insert(
            "IsVivaConnectionsDefaultStartPresent".into(),
            Value::Bool(true),
        );
        c.insert("vivaConnectionsDefaultStart".into(), v.clone());
    }
    if let Some(v) = cx.val("draftMode") {
        c.insert("IsInDraftModePresent".into(), Value::Bool(true));
        c.insert("isInDraftMode".into(), v.clone());
    }
    if let Some(i) = cx.str("audienceIds") {
        c.insert("IsAudiencesPresent".into(), Value::Bool(true));
        c.insert(
            "Audiences".into(),
            Value::from(
                i.split(',')
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<_>>(),
            ),
        );
    }
    if let Some(n) = cx.str("audienceNames") {
        c.insert("IsAudiencesPresent".into(), Value::Bool(true));
        let ids = if n.trim().is_empty() {
            Vec::new()
        } else {
            audience_ids(&n).await?
        };
        c.insert("Audiences".into(), Value::from(ids));
    }
    if let Some(t) = cx.str("targetedLicenseType") {
        c.insert("IsTargetedLicenseTypePresent".into(), Value::Bool(true));
        let n = match t.as_str() {
            "everyone" => Value::from(0),
            "frontLineWorkers" => Value::from(1),
            "informationWorkers" => Value::from(2),
            _ => Value::Null,
        };
        c.insert("TargetedLicenseType".into(), n);
    }
    if let Some(o) = cx.val("order") {
        c.insert("IsOrderPresent".into(), Value::Bool(true));
        c.insert("Order".into(), o.clone());
    }
    let r = Request::post(format!("{admin}/_api/SPO.Tenant/UpdateTargetedSite"))
        .spo_json()
        .header("content-Type", "application/json")
        .json(obj! { "siteUrl": ?cx.val("url").cloned(), "configurationParam": Value::Object(c) })
        .data()
        .await?;
    cx.log(r)
}

pub async fn hidedefaultthemes_get(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Getting the current value of the HideDefaultThemes setting...");
    let r = Request::post(format!("{admin}/_api/thememanager/GetHideDefaultThemes"))
        .spo_json()
        .data()
        .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn hidedefaultthemes_set(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let v = cx.str("hideDefaultThemes").as_deref() == Some("true");
    cx.verbose_log(&format!(
        "Setting the value of the HideDefaultThemes setting to {v}..."
    ));
    Request::post(format!("{admin}/_api/thememanager/SetHideDefaultThemes"))
        .spo_json()
        .json(obj! { "hideDefaultThemes": v })
        .send()
        .await?;
    Ok(())
}

pub async fn orgassetslibrary_add(cx: &mut Cx) -> Result<()> {
    let cdn = if cx.s("cdnType").unwrap_or_else(|| "Private".into()) == "Private" {
        1
    } else {
        0
    };
    let thumb = match cx.str("thumbnailUrl") {
        None => r#"<Parameter Type="Null" />"#.to_string(),
        Some(t) => format!(r#"<Parameter Type="String">{t}</Parameter>"#),
    };
    let ty = match cx.str("orgAssetType").as_deref() {
        Some("OfficeTemplateLibrary") => 2,
        Some("OfficeFontLibrary") => 4,
        _ => 1,
    };
    let body = sp::request(
        &format!(
            r#"<Method Name="AddToOrgAssetsLibAndCdnWithType" Id="11" ObjectPathId="8"><Parameters><Parameter Type="Enum">{cdn}</Parameter><Parameter Type="String">{}</Parameter>{thumb}<Parameter Type="Enum">{ty}</Parameter></Parameters></Method>"#,
            cx.u("libraryUrl")
        ),
        &tenant(8),
    );
    let json = sp::admin_post(cx, false, body).await?;
    sp::check_first(&json)
}

pub async fn orgassetslibrary_list(cx: &mut Cx) -> Result<()> {
    let body = sp::request(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query></Query><Method Name="GetOrgAssets" Id="6" ObjectPathId="3" />"#,
        &tenant(3),
    );
    let json = sp::admin_post(cx, false, body).await?;
    sp::check_first(&json)?;
    let r = json.last().cloned().unwrap_or(Value::Null);
    if r.is_null() || r.get("OrgAssetsLibraries").is_none() {
        return cx.log(Value::Array(Vec::new()));
    }
    let undef = |w: &str| {
        Error::js(format!(
            "Cannot read properties of undefined (reading '{w}')"
        ))
    };
    let url = r
        .get("Url")
        .ok_or_else(|| undef("DecodedUrl"))?
        .get("DecodedUrl")
        .cloned();
    let libs = need_arr(&r["OrgAssetsLibraries"], "_Child_Items_", "map")?;
    let mut out = Vec::new();
    for t in libs {
        let lib = t
            .get("LibraryUrl")
            .ok_or_else(|| undef("DecodedUrl"))?
            .get("DecodedUrl")
            .cloned();
        let thumb = match t.get("ThumbnailUrl") {
            Some(Value::Null) => Value::Null,
            Some(v) => v.get("DecodedUrl").cloned().unwrap_or(Value::Null),
            None => return Err(undef("DecodedUrl")),
        };
        out.push(obj! { "DisplayName": ?t.get("DisplayName").cloned(), "LibraryUrl": ?lib, "ListId": ?t.get("ListId").cloned(), "ThumbnailUrl": thumb });
    }
    cx.log(obj! { "Url": ?url, "Libraries": out })
}

pub async fn orgassetslibrary_remove(cx: &mut Cx) -> Result<()> {
    let lib = cx.u("libraryUrl");
    if !cx.proceed("force", &format!("Are you sure you want to remove the library {lib} as a central location for organization assets?"))? {
        return Ok(());
    }
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="9" ObjectPathId="8" /><Method Name="RemoveFromOrgAssets" Id="10" ObjectPathId="8"><Parameters><Parameter Type="String">{lib}</Parameter><Parameter Type="Guid">{{00000000-0000-0000-0000-000000000000}}</Parameter></Parameters></Method>"#
        ),
        &tenant(8),
    );
    let json = sp::admin_post(cx, false, body).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn orgnewssite_list(cx: &mut Cx) -> Result<()> {
    let json = sp::admin_post(cx, false, sp::request(r#"<ObjectPath Id="58" ObjectPathId="57" /><Method Name="GetOrgNewsSites" Id="59" ObjectPathId="57" />"#, &tenant(57))).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn orgnewssite_remove(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove {url} from the list of organizational news sites?"
        ),
    )? {
        return Ok(());
    }
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="64" ObjectPathId="63" /><Method Name="RemoveOrgNewsSite" Id="65" ObjectPathId="63"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            util::escape_xml(&url)
        ),
        &tenant(63),
    );
    let json = sp::admin_post(cx, false, body).await?;
    sp::check_first(&json)
}

pub async fn orgnewssite_set(cx: &mut Cx) -> Result<()> {
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="61" ObjectPathId="60" /><Method Name="SetOrgNewsSite" Id="62" ObjectPathId="60"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            util::escape_xml(&cx.u("url"))
        ),
        &tenant(60),
    );
    let json = sp::admin_post(cx, false, body).await?;
    sp::check_first(&json)
}

pub async fn knowledgehub_get(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log("Getting the Knowledge Hub Site settings for your tenant");
    let body = format!(
        r#"<Request xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009" AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}"><Actions><ObjectPath Id="5" ObjectPathId="4"/><Method Name="GetKnowledgeHubSite" Id="6" ObjectPathId="4"/></Actions><ObjectPaths><Constructor Id="4" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}"/></ObjectPaths></Request>"#,
        spo::application_name()
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let last = json.last().cloned().unwrap_or(Value::Null);
    cx.log(if truthy(&last) { last } else { Value::from("") })
}

pub async fn knowledgehub_remove(cx: &mut Cx) -> Result<()> {
    if cx.bool("force") {
        cx.debug_log("Confirmation bypassed by entering confirm option. Removing Knowledge Hub Site setting...");
    } else if !cx.confirm("Are you sure you want to remove Knowledge Hub Site from your tenant?")? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log("Removing Knowledge Hub Site settings from your tenant");
    let json = sp::post(&admin, &d, sp::request(r#"<ObjectPath Id="29" ObjectPathId="28"/><Method Name="RemoveKnowledgeHubSite" Id="30" ObjectPathId="28"/>"#, r#"<Constructor Id="28" TypeId="{268004ae-ef6b-4e9b-8425-127220d84719}"/>"#)).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn knowledgehub_set(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!("Adding {site} as the Knowledge Hub Site"));
    let body = format!(
        r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions> <ObjectPath Id="35" ObjectPathId="34" /> <Method Name="SetKnowledgeHubSite" Id="36" ObjectPathId="34"> <Parameters> <Parameter Type="String">{}</Parameter> </Parameters> </Method> </Actions> <ObjectPaths> <Constructor Id="34" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /> </ObjectPaths></Request>"#,
        spo::application_name(),
        util::escape_xml(&site)
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

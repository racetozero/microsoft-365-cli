//! `spo site *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::spo::{self, FormDigest};
use m365_core::validation::{self as v, Valid};

pub(crate) fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

pub(crate) fn opt(cx: &Cx, k: &str) -> String {
    cx.val(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn so(o: &Obj, k: &str) -> Option<String> {
    o.get(k).filter(|v| truthy(v)).map(to_js_string)
}

fn t(o: &Obj, k: &str) -> bool {
    o.get(k).is_some_and(truthy)
}

fn sp_url_check(u: &str) -> std::result::Result<(), String> {
    match v::is_valid_sharepoint_url(u) {
        Valid::Yes => Ok(()),
        Valid::Msg(m) => Err(m),
        Valid::No => Err("false".into()),
    }
}

const TENANT: &str = "{268004ae-ef6b-4e9b-8425-127220d84719}";

pub async fn accessrequest_setting_set(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!("Updating access requests for site '{site}'..."));
    let email = cx.s("email").filter(|s| !s.is_empty()).unwrap_or_default();
    cx.verbose_log(&format!("Updating RequestAccessEmail to '{email}'..."));
    Request::patch(format!("{site}/_api/Web"))
        .spo_json()
        .json(obj! { "RequestAccessEmail": email })
        .send()
        .await?;
    let def = cx.bool("ownerGroup");
    cx.verbose_log(&format!("Updating UseAccessRequestDefault to '{def}'..."));
    Request::post(format!(
        "{site}/_api/Web/SetUseAccessRequestDefaultAndUpdate"
    ))
    .spo_json()
    .header("content-type", "application/json;odata=nometadata")
    .json(obj! { "useAccessRequestDefault": def })
    .send()
    .await?;
    if let Some(m) = cx.val("message").cloned() {
        cx.verbose_log(&format!(
            "Updating access request message to '{}'...",
            to_js_string(&m)
        ));
        Request::post(format!(
            "{site}/_api/Web/SetAccessRequestSiteDescriptionAndUpdate"
        ))
        .spo_json()
        .header("content-type", "application/json;odata=nometadata")
        .json(obj! { "description": m })
        .send()
        .await?;
    }
    Ok(())
}

// ---- add -----------------------------------------------------------------------

const LCIDS: [i64; 49] = [
    1025, 1068, 1069, 5146, 1026, 1027, 2052, 1028, 1050, 1029, 1030, 1043, 1033, 1061, 1035, 1036,
    1110, 1031, 1032, 1037, 1081, 1038, 1057, 2108, 1040, 1041, 1087, 1042, 1062, 1063, 1071, 1086,
    1044, 1045, 1046, 2070, 1048, 1049, 10266, 2074, 1051, 1060, 3082, 1053, 1054, 1055, 1058,
    1066, 1106,
];

fn not_number(o: &Obj, k: &str) -> Option<String> {
    o.get(k)
        .filter(|v| truthy(v) && !v.is_number())
        .map(|v| format!("{} is not a number", to_js_string(v)))
}

fn num(o: &Obj, k: &str) -> f64 {
    o.get(k)
        .map(|v| {
            v.as_f64()
                .unwrap_or_else(|| json::js_number(&to_js_string(v)))
        })
        .unwrap_or(f64::NAN)
}

pub fn add_validate(o: &Obj) -> std::result::Result<(), String> {
    let ty = o.get("type").map(to_js_string);
    let classic = ty.as_deref() == Some("ClassicSite");
    let comm = matches!(
        ty.as_deref(),
        Some("CommunicationSite") | Some("BrandCenter")
    );
    if let Some(ty) = o.get("type").filter(|v| truthy(v)).map(to_js_string)
        && ![
            "TeamSite",
            "CommunicationSite",
            "ClassicSite",
            "BrandCenter",
        ]
        .contains(&ty.as_str())
    {
        return Err(format!(
            "{ty} is not a valid site type. Allowed types are TeamSite, CommunicationSite, ClassicSite, and BrandCenter"
        ));
    }
    let any = |ks: &[&str]| ks.iter().any(|k| t(o, k));
    if !comm && !classic {
        if !t(o, "alias") {
            return Err("Required option alias missing".into());
        }
        if any(&[
            "url",
            "siteDesign",
            "removeDeletedSite",
            "wait",
            "shareByEmailEnabled",
            "siteDesignId",
            "timeZone",
            "resourceQuota",
            "resourceQuotaWarningLevel",
            "storageQuota",
            "storageQuotaWarningLevel",
            "webTemplate",
            "force",
        ]) {
            return Err("Type TeamSite supports only the parameters title, lcid, alias, owners, classification, isPublic, and description".into());
        }
    } else if comm {
        let Some(url) = so(o, "url") else {
            return Err("Required option url missing".into());
        };
        sp_url_check(&url)?;
        if let Some(d) = so(o, "siteDesign")
            && !["Topic", "Showcase", "Blank"].contains(&d.as_str())
        {
            return Err(format!(
                "{d} is not a valid communication site type. Allowed types are Topic, Showcase and Blank"
            ));
        }
        if so(o, "owners").is_some_and(|w| w.contains(',')) {
            return Err(
                "The CommunicationSite supports only one owner in the owners option".into(),
            );
        }
        if let Some(d) = so(o, "siteDesignId")
            && !v::is_valid_guid(&d)
        {
            return Err(format!("{d} is not a valid GUID"));
        }
        if t(o, "siteDesign") && t(o, "siteDesignId") {
            return Err("Specify siteDesign or siteDesignId but not both".into());
        }
        if any(&[
            "timeZone",
            "isPublic",
            "removeDeletedSite",
            "wait",
            "alias",
            "resourceQuota",
            "resourceQuotaWarningLevel",
            "storageQuota",
            "storageQuotaWarningLevel",
            "webTemplate",
        ]) {
            return Err("Type CommunicationSite supports only the parameters url, title, lcid, classification, siteDesign, shareByEmailEnabled, siteDesignId, owners, description, and force".into());
        }
    } else {
        let Some(url) = so(o, "url") else {
            return Err("Required option url missing".into());
        };
        sp_url_check(&url)?;
        let Some(owners) = so(o, "owners") else {
            return Err("Required option owner missing".into());
        };
        if owners.contains(',') {
            return Err("The ClassicSite supports only one owner in the owners options".into());
        }
        if !t(o, "timeZone") {
            return Err("Required option timeZone missing".into());
        }
        if !o.get("timeZone").is_some_and(Value::is_number) {
            return Err(format!("{} is not a number", to_js_string(&o["timeZone"])));
        }
        for (k, base) in [
            ("resourceQuota", ""),
            ("resourceQuotaWarningLevel", "resourceQuota"),
            ("storageQuota", ""),
            ("storageQuotaWarningLevel", "storageQuota"),
        ] {
            if let Some(m) = not_number(o, k) {
                return Err(m);
            }
            if !base.is_empty() {
                if t(o, k) && !t(o, base) {
                    return Err(format!("You cannot specify {k} without specifying {base}"));
                }
                if num(o, k) > num(o, base) {
                    return Err(format!("{k} cannot exceed {base}"));
                }
            }
        }
        if any(&[
            "classification",
            "shareByEmailEnabled",
            "siteDesignId",
            "alias",
            "isPublic",
            "force",
        ]) {
            return Err("Type ClassicSite supports only the parameters url, title, lcid, storageQuota, storageQuotaWarningLevel, resourceQuota, resourceQuotaWarningLevel, webTemplate, owners, and description".into());
        }
    }
    if t(o, "lcid") {
        let l = num(o, "lcid");
        let s = to_js_string(&o["lcid"]);
        if l.is_nan() {
            return Err(format!("{s} is not a number"));
        }
        if l < 0.0 {
            return Err(format!("LCID must be greater than 0 ({s})"));
        }
        if !o["lcid"].is_number() || !LCIDS.iter().any(|x| *x as f64 == l) {
            return Err(format!(
                "LCID {s} is not valid. See https://support.microsoft.com/en-us/office/languages-supported-by-sharepoint-dfbf3652-2902-4809-be21-9080b6512fff for the languages supported by SharePoint."
            ));
        }
    }
    Ok(())
}

fn csom_last(json: &[Value]) -> Value {
    json.last().cloned().unwrap_or(Value::Null)
}

pub(crate) async fn wait_op(
    cx: &Cx,
    admin: &str,
    op: &Value,
    ctx: Option<FormDigest>,
) -> Result<()> {
    let ms = op
        .get("PollingInterval")
        .and_then(Value::as_f64)
        .unwrap_or(0.0);
    util::sleep_ms(ms.max(0.0) as u64).await;
    // `JSON.stringify(undefined)` is undefined: `operationId.replace` throws
    let Some(id) = op.get("_ObjectIdentity_") else {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'replace')",
        ));
    };
    spo::wait_until_finished(&json::stringify(id), admin, ctx, cx.debug(), cx.verbose()).await
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let classic = cx.str("type").as_deref() == Some("ClassicSite");
    let url = if classic {
        create_classic(cx).await.map_err(Error::plain)?
    } else {
        create_modern(cx).await?
    };
    if let Some(u) = url.as_ref().filter(|u| truthy(u))
        && cx.bool("withAppCatalog")
    {
        add_app_catalog(cx, &to_js_string(u))
            .await
            .map_err(Error::plain)?;
    }
    match url {
        Some(u) => cx.log(u),
        None => cx.log_undefined(),
    }
}

async fn create_modern(cx: &Cx) -> Result<Option<Value>> {
    let ty = cx.str("type");
    let team = !matches!(
        ty.as_deref(),
        Some("CommunicationSite") | Some("BrandCenter")
    );
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    cx.verbose_log("Creating new site...");
    let req = if team {
        let mut results: Vec<Value> = Vec::new();
        if cx.val("lcid").is_some_and(truthy) {
            results.push(Value::from(format!("SPSiteLanguage:{}", opt(cx, "lcid"))));
        }
        let mut params = obj! {
            "Description": ne(cx, "description").unwrap_or_default(),
            "CreationOptions": obj! { "results": results, "Classification": ne(cx, "classification").unwrap_or_default() },
        };
        if let Some(o) = ne(cx, "owners") {
            params["Owners"] =
                obj! { "results": o.split(',').map(|x| x.trim().to_string()).collect::<Vec<_>>() };
        }
        Request::post(format!("{spo_url}/_api/GroupSiteManager/CreateGroupEx"))
            .header("content-type", "application/json; odata=verbose; charset=utf-8")
            .spo_json()
            .json(obj! { "displayName": ?cx.val("title").cloned(), "alias": ?cx.val("alias").cloned(), "isPublic": ?cx.val("isPublic").cloned(), "optionalParams": params })
    } else {
        let design = match ne(cx, "siteDesignId") {
            Some(d) => d,
            None => match ne(cx, "siteDesign").as_deref() {
                Some("Showcase") => "6142d2a0-63a5-4ba0-aede-d9fefca2c767".into(),
                Some("Blank") => "f6cc5403-0d63-442e-96c0-285923709ffc".into(),
                Some("Topic") => "00000000-0000-0000-0000-000000000000".into(),
                Some(_) => String::new(),
                None => "00000000-0000-0000-0000-000000000000".into(),
            },
        };
        let mut r = obj! {
            "Title": ?cx.val("title").cloned(), "Url": ?cx.val("url").cloned(), "ShareByEmailEnabled": ?cx.val("shareByEmailEnabled").cloned(),
            "Description": ne(cx, "description").unwrap_or_default(), "Classification": ne(cx, "classification").unwrap_or_default(),
            "WebTemplate": "SITEPAGEPUBLISHING#0", "SiteDesignId": design,
        };
        if let Some(l) = cx.val("lcid").filter(|v| truthy(v)) {
            r["Lcid"] = l.clone();
        }
        if let Some(o) = cx.val("owners").filter(|v| truthy(v)) {
            r["Owner"] = o.clone();
        }
        if ty.as_deref() == Some("BrandCenter") {
            cx.debug_log("Retrieving brand center configuration...");
            let admin = spo::get_spo_admin_url(cx.debug()).await?;
            let conf = Request::get(format!(
                "{admin}/_api/SPO.Tenant/GetBrandCenterConfiguration"
            ))
            .spo_json()
            .data()
            .await?;
            cx.debug_log("Successfully retrieved brand center configuration");
            if conf
                .get("IsBrandCenterSiteFeatureEnabled")
                .is_some_and(truthy)
            {
                return Err(Error::js(
                    "Brand center site is already created in the tenant.",
                ));
            }
            let warning = "You agree to activate this site as your official brand center site and turn on the brand center app for use in your organization. Storage locations will be created for uploading files to brand center and managing them. Any uploaded files will be stored in the cloud and managed in a public content delivery network (CDN). The files will be accessible to anyone who is able to extract the URLs that point to them.\nDon't use this feature if your files contain proprietary information, or if you don't have the necessary cloud hosting rights to use them. After creation, that site cannot be deleted.";
            if cx.bool("force") {
                cx.stderr(warning);
            } else if !cx.confirm(&format!("{warning}\n\nDo you want to proceed?"))? {
                return Err(Error::js("Operation cancelled by the user."));
            }
            r["AdditionalSiteFeatureIds"] =
                Value::from(vec!["99cd6e8b-189b-4611-ae89-f89105876e43"]);
        }
        Request::post(format!("{spo_url}/_api/SPSiteManager/Create"))
            .header("content-type", "application/json;odata=nometadata")
            .spo_json()
            .json(obj! { "request": r })
    };
    let res = req.data().await?;
    if team {
        match res.get("ErrorMessage") {
            Some(Value::Null) => {}
            None => {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'error')",
                ));
            }
            Some(m) => return Err(Error::Thrown(m.clone())),
        }
    } else if res.get("SiteStatus").and_then(Value::as_f64) != Some(2.0) {
        return Err(Error::msg("An error has occurred while creating the site"));
    }
    Ok(res.get("SiteUrl").cloned())
}

async fn create_classic(cx: &Cx) -> Result<Option<Value>> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let mut ctx = Some(spo::ensure_form_digest(&admin, None, cx.debug()).await?);
    let url = opt(cx, "url");
    let exists = if cx.bool("removeDeletedSite") {
        site_exists(cx, &admin, &url, &mut ctx).await?
    } else {
        false
    };
    if exists {
        cx.verbose_log("Site exists in the recycle bin");
        delete_from_recycle_bin(cx, &admin, &url, cx.bool("wait"), &mut ctx).await?;
    } else {
        cx.verbose_log("Site not found");
    }
    let d = spo::ensure_form_digest(&admin, ctx.as_ref(), cx.debug()).await?;
    ctx = Some(d.clone());
    cx.verbose_log(&format!("Creating site collection {url}..."));
    let n = |k: &str, def: &str| {
        cx.val(k)
            .filter(|v| v.is_number())
            .map(to_js_string)
            .unwrap_or_else(|| def.into())
    };
    let body = sp::request(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectPath Id="6" ObjectPathId="5" /><Query Id="7" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query></Query><Query Id="8" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
        &format!(
            r#"<Constructor Id="3" TypeId="{TENANT}" /><Method Id="5" ParentId="3" Name="CreateSite"><Parameters><Parameter TypeId="{{11f84fff-b8cf-47b6-8b50-34e692656606}}"><Property Name="CompatibilityLevel" Type="Int32">0</Property><Property Name="Lcid" Type="UInt32">{}</Property><Property Name="Owner" Type="String">{}</Property><Property Name="StorageMaximumLevel" Type="Int64">{}</Property><Property Name="StorageWarningLevel" Type="Int64">{}</Property><Property Name="Template" Type="String">{}</Property><Property Name="TimeZoneId" Type="Int32">{}</Property><Property Name="Title" Type="String">{}</Property><Property Name="Url" Type="String">{}</Property><Property Name="UserCodeMaximumLevel" Type="Double">{}</Property><Property Name="UserCodeWarningLevel" Type="Double">{}</Property></Parameter></Parameters></Method>"#,
            n("lcid", "1033"),
            util::escape_xml_value(cx.val("owners")),
            n("storageQuota", "100"),
            n("storageQuotaWarningLevel", "100"),
            util::escape_xml(&ne(cx, "webTemplate").unwrap_or_else(|| "STS#0".into())),
            opt(cx, "timeZone"),
            util::escape_xml_value(cx.val("title")),
            util::escape_xml_value(cx.val("url")),
            n("resourceQuota", "0"),
            n("resourceQuotaWarningLevel", "0"),
        ),
    );
    let json = sp::post(&admin, &d.value, body).await?;
    sp::check_first(&json)?;
    let op = csom_last(&json);
    if (!cx.bool("wait") && !cx.bool("withAppCatalog")) || op.get("IsComplete").is_some_and(truthy)
    {
        return Ok(cx.val("url").cloned());
    }
    wait_op(cx, &admin, &op, ctx).await?;
    Ok(cx.val("url").cloned())
}

fn error_type(json: &[Value]) -> Option<(Value, String)> {
    let f = json.first()?;
    let e = f.get("ErrorInfo").filter(|e| truthy(e))?;
    Some((
        e.get("ErrorMessage").cloned().unwrap_or(Value::Null),
        e.get("ErrorTypeName").map(to_js_string).unwrap_or_default(),
    ))
}

async fn site_exists(
    cx: &Cx,
    admin: &str,
    url: &str,
    ctx: &mut Option<FormDigest>,
) -> Result<bool> {
    let d = spo::ensure_form_digest(admin, ctx.as_ref(), cx.debug()).await?;
    *ctx = Some(d.clone());
    cx.verbose_log(&format!("Checking if the site {url} exists..."));
    let body = sp::request(
        r#"<ObjectPath Id="197" ObjectPathId="196" /><ObjectPath Id="199" ObjectPathId="198" /><Query Id="200" ObjectPathId="198"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        &format!(
            r#"<Constructor Id="196" TypeId="{TENANT}" /><Method Id="198" ParentId="196" Name="GetSitePropertiesByUrl"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Boolean">false</Parameter></Parameters></Method>"#,
            util::escape_xml(url)
        ),
    );
    let json = sp::post(admin, &d.value, body).await?;
    if json.is_empty() {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'ErrorInfo')",
        ));
    }
    if let Some((m, ty)) = error_type(&json) {
        if ty == "Microsoft.Online.SharePoint.Common.SpoNoSiteException" {
            cx.verbose_log(&format!(
                "Site doesn't exist. Checking if the site {url} exists in the recycle bin..."
            ));
            let body = sp::request(
                r#"<ObjectPath Id="181" ObjectPathId="180" /><Query Id="182" ObjectPathId="180"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
                &format!(
                    r#"<Method Id="180" ParentId="175" Name="GetDeletedSitePropertiesByUrl"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="175" TypeId="{TENANT}" />"#,
                    util::escape_xml(url)
                ),
            );
            let json = sp::post(admin, &d.value, body).await?;
            if json.is_empty() {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'ErrorInfo')",
                ));
            }
            if let Some((m, ty)) = error_type(&json) {
                if ty == "Microsoft.SharePoint.Client.UnknownError" {
                    return Ok(false);
                }
                return Err(Error::Thrown(m));
            }
            return Ok(csom_last(&json).get("Status").and_then(Value::as_str) == Some("Recycled"));
        }
        return Err(Error::Thrown(m));
    }
    Ok(csom_last(&json).get("Status").and_then(Value::as_str) == Some("Recycled"))
}

async fn delete_from_recycle_bin(
    cx: &Cx,
    admin: &str,
    url: &str,
    wait: bool,
    ctx: &mut Option<FormDigest>,
) -> Result<()> {
    let d = spo::ensure_form_digest(admin, ctx.as_ref(), cx.debug()).await?;
    *ctx = Some(d.clone());
    cx.verbose_log(&format!("Deleting site {url} from the recycle bin..."));
    let body = sp::request(
        r#"<ObjectPath Id="185" ObjectPathId="184" /><Query Id="186" ObjectPathId="184"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
        &format!(
            r#"<Method Id="184" ParentId="175" Name="RemoveDeletedSite"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="175" TypeId="{TENANT}" />"#,
            util::escape_xml(url)
        ),
    );
    let json = sp::post(admin, &d.value, body).await?;
    sp::check_first(&json)?;
    let op = csom_last(&json);
    if !wait || op.get("IsComplete").is_some_and(truthy) {
        return Ok(());
    }
    wait_op(cx, admin, &op, ctx.clone()).await
}

async fn add_app_catalog(cx: &Cx, url: &str) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
    cx.verbose_log("Adding site collection app catalog...");
    let u = util::escape_xml(url);
    let body = sp::request(
        r#"<ObjectPath Id="38" ObjectPathId="37" /><ObjectPath Id="40" ObjectPathId="39" /><ObjectPath Id="42" ObjectPathId="41" /><ObjectPath Id="44" ObjectPathId="43" /><ObjectPath Id="46" ObjectPathId="45" /><ObjectPath Id="48" ObjectPathId="47" />"#,
        &format!(
            r#"<Constructor Id="37" TypeId="{TENANT}" /><Method Id="39" ParentId="37" Name="GetSiteByUrl"><Parameters><Parameter Type="String">{u}</Parameter></Parameters></Method><Property Id="41" ParentId="39" Name="RootWeb" /><Property Id="43" ParentId="41" Name="TenantAppCatalog" /><Property Id="45" ParentId="43" Name="SiteCollectionAppCatalogsSites" /><Method Id="47" ParentId="45" Name="Add"><Parameters><Parameter Type="String">{u}</Parameter></Parameters></Method>"#
        ),
    );
    let json = sp::post(&admin, &d.value, body).await?;
    sp::check_first(&json)?;
    cx.verbose_log("Site collection app catalog created");
    Ok(())
}

// ---- admins --------------------------------------------------------------------

fn guid_msg(o: &Obj, k: &str, f: impl Fn(&str) -> String) -> std::result::Result<(), String> {
    match so(o, k) {
        Some(g) if !v::is_valid_guid(&g) => Err(f(&g)),
        _ => Ok(()),
    }
}

pub fn admin_add_validate(o: &Obj) -> std::result::Result<(), String> {
    guid_msg(o, "userId", |g| {
        format!("'{g}' is not a valid GUID for option 'userId'")
    })?;
    if let Some(u) = so(o, "userName")
        && !v::is_valid_user_principal_name(&u)
    {
        return Err(format!("'{u}' is not a valid 'userName'"));
    }
    guid_msg(o, "groupId", |g| {
        format!("'{g}' is not a valid GUID for option 'groupId'")
    })?;
    sp_url_check(
        &o.get("siteUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
}

pub fn admin_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    guid_msg(o, "userId", |g| format!("{g} is not a valid GUID"))?;
    if let Some(u) = so(o, "userName")
        && !v::is_valid_user_principal_name(&u)
    {
        return Err(format!("{u} is not a valid userName"));
    }
    guid_msg(o, "groupId", |g| format!("{g} is not a valid GUID"))?;
    sp_url_check(
        &o.get("siteUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
}

pub fn site_url_validate(o: &Obj) -> std::result::Result<(), String> {
    sp_url_check(
        &o.get("siteUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
}

async fn login_name(cx: &Cx) -> Result<String> {
    if ne(cx, "userId").is_some() || ne(cx, "userName").is_some() {
        let upn = match ne(cx, "userName") {
            Some(u) => u,
            None => m365_core::ut::entra::user::upn_by_id(&opt(cx, "userId"), false).await?,
        };
        if upn.is_empty() {
            return Err(Error::msg("User not found."));
        }
        return Ok(format!("i:0#.f|membership|{upn}"));
    }
    let g = match ne(cx, "groupId") {
        Some(i) => m365_core::ut::entra::group::by_id(&i, None).await?,
        None => m365_core::ut::entra::group::by_display_name(&opt(cx, "groupName"), None).await?,
    };
    Ok(if g.get("mail").is_some_and(truthy) {
        format!("c:0o.c|federateddirectoryclaimprovider|{}", u(&g, "id"))
    } else {
        format!("c:0t.c|tenant|{}", u(&g, "id"))
    })
}

async fn site_admins(admin: &str, site_id: &str, json_response: bool) -> Result<Vec<Value>> {
    let mut r = Request::post(format!(
        "{admin}/_api/SPO.Tenant/GetSiteAdministrators?siteId='{site_id}'"
    ))
    .spo_json()
    .header("content-type", "application/json;charset=utf-8");
    if !json_response {
        r.response_type = m365_core::http::ResponseType::Text;
    }
    let res = r.send().await?;
    let v = if json_response {
        res.data()
    } else {
        json::jparse(&res.text())?
    };
    Ok(need_arr(&v, "value", "map")?.to_vec())
}

async fn set_admins(
    admin: &str,
    site_id: Option<Value>,
    names: Vec<Value>,
    json_response: bool,
) -> Result<()> {
    let mut r = Request::post(format!("{admin}/_api/SPOInternalUseOnly.Tenant/SetSiteSecondaryAdministrators"))
        .spo_json()
        .header("content-type", "application/json;charset=utf-8")
        .json(obj! { "secondaryAdministratorsFieldsData": obj! { "siteId": ?site_id, "secondaryAdministratorLoginNames": names } });
    if !json_response {
        r.response_type = m365_core::http::ResponseType::Text;
    }
    r.send().await?;
    Ok(())
}

async fn primary_admin_as_admin(cx: &Cx, admin: &str, site_id: &str) -> Result<Option<Value>> {
    cx.verbose_log("Getting the primary admin login name...");
    Ok(Request::get(format!(
        "{admin}/_api/SPO.Tenant/sites('{site_id}')?$select=OwnerLoginName"
    ))
    .spo_json()
    .data()
    .await?
    .get("OwnerLoginName")
    .cloned())
}

async fn primary_owner(cx: &Cx, site: &str) -> Result<Option<Value>> {
    cx.verbose_log("Getting the primary admin login name...");
    Ok(Request::get(format!("{site}/_api/site/owner"))
        .spo_json()
        .data()
        .await?
        .get("LoginName")
        .cloned())
}

async fn site_user_admin(site: &str, login: &str, is_admin: bool) -> Result<()> {
    Request::post(format!("{site}/_api/web/siteusers('{}')", eqp(login)))
        .accept("application/json")
        .json_response()
        .header("X-Http-Method", "MERGE")
        .header("If-Match", "*")
        .json(obj! { "IsSiteAdmin": is_admin })
        .send()
        .await?;
    Ok(())
}

pub async fn admin_add(cx: &mut Cx) -> Result<()> {
    let login = login_name(cx).await?;
    let site = cx.u("siteUrl");
    if cx.bool("asAdmin") {
        cx.verbose_log("Adding site administrator as an administrator...");
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let props = sp::site_admin_properties(cx, &site, false).await?;
        let id = u(&props, "SiteId");
        let mut names: Vec<Value> = site_admins(&admin, &id, false)
            .await?
            .iter()
            .map(|x| x.get("loginName").cloned().unwrap_or(Value::Null))
            .collect();
        names.push(Value::from(login.as_str()));
        set_admins(&admin, props.get("SiteId").cloned(), names, false).await?;
        if cx.bool("primary") {
            Request::patch(format!("{admin}/_api/SPO.Tenant/sites('{id}')"))
                .accept("application/json;odata=nometadata")
                .header("content-type", "application/json;charset=utf-8")
                .json(obj! { "Owner": login, "SetOwnerWithoutUpdatingSecondaryAdmin": true })
                .send()
                .await?;
        }
        return Ok(());
    }
    cx.verbose_log("Adding site administrator...");
    let user = Request::post(format!("{site}/_api/web/ensureuser"))
        .spo_json()
        .json(obj! { "logonName": login.as_str() })
        .data()
        .await?;
    site_user_admin(&site, &login, true).await?;
    if cx.bool("primary") {
        let id = sp::site_id_by_sp_api(cx, &site).await?;
        let prev = Request::get(format!("{site}/_api/site/owner?$select=LoginName"))
            .spo_json()
            .data()
            .await?;
        let d = spo::ensure_form_digest(&site, None, cx.debug()).await?;
        let body = format!(
            r#"<Request xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}"><Actions><SetProperty Id="10" ObjectPathId="2" Name="Owner"><Parameter ObjectPathId="3" /></SetProperty></Actions><ObjectPaths><Property Id="2" ParentId="0" Name="Site" /><Identity Id="3" Name="6d452ba1-40a8-8000-e00d-46e1adaa12bf|740c6a0b-85e2-48a0-a494-e0f1759d4aa7:site:{id}:u:{}" /><StaticProperty Id="0" TypeId="{{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}}" Name="Current" /></ObjectPaths></Request>"#,
            spo::application_name(),
            u(&user, "Id")
        );
        spo::process_query(&site, &d.value, body).await?;
        site_user_admin(&site, &u(&prev, "LoginName"), true).await?;
    }
    Ok(())
}

const PRINCIPAL_TYPES: [(i64, &str); 6] = [
    (0, "None"),
    (1, "User"),
    (2, "DistributionList"),
    (4, "SecurityGroup"),
    (8, "SharePointGroup"),
    (15, "All"),
];

pub async fn admin_list(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    if cx.bool("asAdmin") {
        cx.verbose_log("Retrieving site administrators as an administrator...");
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let props = sp::site_admin_properties(cx, &site, false).await?;
        let id = u(&props, "SiteId");
        let r = Request::post(format!(
            "{admin}/_api/SPO.Tenant/GetSiteAdministrators?siteId='{id}'"
        ))
        .spo_json()
        .data()
        .await?;
        let primary = primary_admin_as_admin(cx, &admin, &id).await?;
        let out: Vec<Value> = need_arr(&r, "value", "map")?
            .iter()
            .map(|x| obj! { "Email": ?x.get("email").cloned(), "LoginName": ?x.get("loginName").cloned(), "Title": ?x.get("name").cloned(), "IsPrimaryAdmin": x.get("loginName").cloned() == primary })
            .collect();
        return cx.log(Value::Array(out));
    }
    cx.verbose_log("Retrieving site administrators...");
    let r = Request::get(format!(
        "{site}/_api/web/siteusers?$filter=IsSiteAdmin eq true"
    ))
    .spo_json()
    .data()
    .await?;
    let primary = primary_owner(cx, &site).await?;
    let out: Vec<Value> = need_arr(&r, "value", "map")?
        .iter()
        .map(|x| {
            let pt = x.get("PrincipalType").and_then(Value::as_i64).and_then(|n| PRINCIPAL_TYPES.iter().find(|(k, _)| *k == n)).map(|(_, s)| Value::from(*s));
            obj! {
                "Id": ?x.get("Id").cloned(), "LoginName": ?x.get("LoginName").cloned(), "Title": ?x.get("Title").cloned(),
                "PrincipalType": ?x.get("PrincipalType").cloned(), "PrincipalTypeString": ?pt, "Email": ?x.get("Email").cloned(),
                "IsPrimaryAdmin": x.get("LoginName").cloned() == primary,
            }
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn admin_remove(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    if !cx.bool("force") {
        let what = if ne(cx, "groupId").is_some() || ne(cx, "groupName").is_some() {
            "group"
        } else {
            "user"
        };
        if !cx.confirm(&format!("Are you sure you want to remove specified {what} from the site administrators list {site}?"))? {
            return Ok(());
        }
    }
    let login = login_name(cx).await?;
    if cx.bool("asAdmin") {
        cx.verbose_log("Removing site administrator as an administrator...");
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let props = sp::site_admin_properties(cx, &site, false).await?;
        let id = u(&props, "SiteId");
        let primary = primary_admin_as_admin(cx, &admin, &id).await?;
        if primary.as_ref().and_then(Value::as_str) == Some(login.as_str()) {
            return Err(Error::msg(
                "You cannot remove the primary site collection administrator.",
            ));
        }
        let mut keep = Vec::new();
        for a in site_admins(&admin, &id, true).await? {
            let l = match a.get("loginName") {
                Some(Value::String(s)) => s.to_lowercase(),
                _ => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading 'toLowerCase')",
                    ));
                }
            };
            if l != login.to_lowercase() {
                keep.push(a.get("loginName").cloned().unwrap_or(Value::Null));
            }
        }
        return set_admins(&admin, props.get("SiteId").cloned(), keep, true).await;
    }
    cx.verbose_log("Removing site administrator...");
    let primary = primary_owner(cx, &site).await?;
    if primary.as_ref().and_then(Value::as_str) == Some(login.as_str()) {
        return Err(Error::msg(
            "You cannot remove the primary site collection administrator.",
        ));
    }
    site_user_admin(&site, &login, false).await
}

// ---- app catalog ---------------------------------------------------------------

async fn catalog_call(cx: &Cx, actions: String, paths: String) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let json = sp::post(&admin, &d, sp::request(&actions, &paths)).await?;
    sp::check_first(&json)
}

pub async fn appcatalog_add(cx: &mut Cx) -> Result<()> {
    let url = util::escape_xml(&cx.u("siteUrl"));
    let r = async {
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let d = sp::digest(&admin).await?;
        cx.verbose_log("Adding site collection app catalog...");
        let json = sp::post(
            &admin,
            &d,
            sp::request(
                r#"<ObjectPath Id="38" ObjectPathId="37" /><ObjectPath Id="40" ObjectPathId="39" /><ObjectPath Id="42" ObjectPathId="41" /><ObjectPath Id="44" ObjectPathId="43" /><ObjectPath Id="46" ObjectPathId="45" /><ObjectPath Id="48" ObjectPathId="47" />"#,
                &format!(r#"<Constructor Id="37" TypeId="{TENANT}" /><Method Id="39" ParentId="37" Name="GetSiteByUrl"><Parameters><Parameter Type="String">{url}</Parameter></Parameters></Method><Property Id="41" ParentId="39" Name="RootWeb" /><Property Id="43" ParentId="41" Name="TenantAppCatalog" /><Property Id="45" ParentId="43" Name="SiteCollectionAppCatalogsSites" /><Method Id="47" ParentId="45" Name="Add"><Parameters><Parameter Type="String">{url}</Parameter></Parameters></Method>"#),
            ),
        )
        .await?;
        sp::check_first(&json)
    }
    .await;
    r?;
    cx.verbose_log("Site collection app catalog created");
    Ok(())
}

pub async fn appcatalog_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving site collection app catalogs...");
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let mut cats = odata::get_all_items(&format!(
        "{spo_url}/_api/Web/TenantAppCatalog/SiteCollectionAppCatalogsSites"
    ))
    .await?;
    if cx.bool("excludeDeletedSites") {
        cx.verbose_log("Excluding inaccessible sites from the results...");
        let mut active = Vec::new();
        for c in cats {
            let abs = u(&c, "AbsoluteUrl");
            match sp::get_web(cx, &abs).await {
                Ok(_) => active.push(c),
                Err(e) => {
                    if cx.debug() {
                        m365_core::log::log_stderr_value(&e.command_message());
                    }
                    if matches!(e.status(), Some(404) | Some(403)) {
                        cx.verbose_log(&format!(
                            "Site at '{abs}' is inaccessible. Excluding from results..."
                        ));
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        cats = active;
    }
    cx.log(Value::Array(cats))
}

pub async fn appcatalog_remove(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the app catalog from {site}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log("Disabling site collection app catalog...");
    let url = util::escape_xml(&site);
    catalog_call(
        cx,
        format!(r#"<ObjectPath Id="50" ObjectPathId="49" /><ObjectPath Id="52" ObjectPathId="51" /><ObjectPath Id="54" ObjectPathId="53" /><ObjectPath Id="56" ObjectPathId="55" /><ObjectPath Id="58" ObjectPathId="57" /><Method Name="Remove" Id="59" ObjectPathId="57"><Parameters><Parameter Type="String">{url}</Parameter></Parameters></Method>"#),
        format!(r#"<Constructor Id="49" TypeId="{TENANT}" /><Method Id="51" ParentId="49" Name="GetSiteByUrl"><Parameters><Parameter Type="String">{url}</Parameter></Parameters></Method><Property Id="53" ParentId="51" Name="RootWeb" /><Property Id="55" ParentId="53" Name="TenantAppCatalog" /><Property Id="57" ParentId="55" Name="SiteCollectionAppCatalogsSites" />"#),
    )
    .await?;
    cx.verbose_log("Site collection app catalog disabled");
    Ok(())
}

// ---- app permissions -----------------------------------------------------------

const GRAPH: &str = "https://graph.microsoft.com/v1.0";
const ROLES: [&str; 4] = ["read", "write", "manage", "fullcontrol"];

/// `Array.prototype.toString` of a JSON array (nested arrays flattened, null/undefined empty).
fn js_join(v: &Value) -> String {
    match v {
        Value::Array(a) => a.iter().map(js_join).collect::<Vec<_>>().join(","),
        Value::Null => String::new(),
        other => to_js_string(other),
    }
}

pub fn apppermission_add_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(a) = so(o, "appId")
        && !v::is_valid_guid(&a)
    {
        return Err(format!("{a} is not a valid GUID"));
    }
    let p = o
        .get("permission")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    if !o.get("permission").is_some_and(Value::is_string) || !ROLES.contains(&p.as_str()) {
        return Err(format!(
            "{p} is not a valid permission value. Allowed values are {}",
            ROLES.join("|")
        ));
    }
    site_url_validate(o)
}

pub fn apppermission_list_validate(o: &Obj) -> std::result::Result<(), String> {
    if t(o, "appId") && t(o, "appDisplayName") {
        return Err("Provide either appId or appDisplayName, not both".into());
    }
    site_url_validate(o)
}

pub fn apppermission_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(a) = so(o, "appId")
        && !v::is_valid_guid(&a)
    {
        return Err(format!("{a} is not a valid GUID"));
    }
    site_url_validate(o)
}

fn filtered(cx: &Cx, perms: &[Value]) -> Result<Vec<Value>> {
    let (prop, val) = match ne(cx, "appId") {
        Some(a) => ("id", Some(Value::from(a))),
        None => ("displayName", cx.val("appDisplayName").cloned()),
    };
    let mut out = Vec::new();
    for p in perms {
        let ids = need_arr(p, "grantedToIdentities", "some")?;
        let mut hit = false;
        for i in ids {
            let app = i.get("application");
            match app {
                Some(a) if !a.is_null() => {
                    if a.get(prop).cloned() == val {
                        hit = true;
                        break;
                    }
                }
                _ => {
                    return Err(Error::js(format!(
                        "Cannot read properties of {} (reading '{prop}')",
                        if app.is_some() { "null" } else { "undefined" }
                    )));
                }
            }
        }
        if hit {
            out.push(p.clone());
        }
    }
    Ok(out)
}

fn transpose(perms: &[Value], join_roles: bool) -> Result<Vec<Value>> {
    let mut out = Vec::new();
    for p in perms {
        for i in need_arr(p, "grantedToIdentities", "forEach")? {
            let app = i
                .get("application")
                .filter(|a| !a.is_null())
                .ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'displayName')")
                })?;
            let roles = if join_roles {
                match p.get("roles") {
                    Some(r @ Value::Array(_)) => Some(Value::from(js_join(r))),
                    _ => {
                        return Err(Error::js(
                            "Cannot read properties of undefined (reading 'join')",
                        ));
                    }
                }
            } else {
                p.get("roles").cloned()
            };
            out.push(obj! { "appDisplayName": ?app.get("displayName").cloned(), "appId": ?app.get("id").cloned(), "permissionId": ?p.get("id").cloned(), "roles": ?roles });
        }
    }
    Ok(out)
}

pub async fn apppermission_add(cx: &mut Cx) -> Result<()> {
    let site = spo::graph_site_id(&cx.u("siteUrl")).await?;
    let (app_id, name) = if let (Some(a), Some(n)) = (ne(cx, "appId"), ne(cx, "appDisplayName")) {
        (Some(Value::from(a)), Some(Value::from(n)))
    } else {
        let filter = match ne(cx, "appId") {
            Some(a) => format!("appId eq '{}'", eqp(&a)),
            None => format!("displayName eq '{}'", eqp(&opt(cx, "appDisplayName"))),
        };
        let r = Request::get(format!(
            "{GRAPH}/myorganization/servicePrincipals?$select=appId,displayName&$filter={filter}"
        ))
        .graph_json()
        .data()
        .await?;
        let v = need_arr(&r, "value", "0")?;
        let Some(first) = v.first().filter(|x| truthy(x)) else {
            return Err(Error::msg(
                "The specified Microsoft Entra app does not exist",
            ));
        };
        if v.len() > 1 {
            return Err(Error::msg(format!(
                "Multiple Microsoft Entra apps with displayName {} found: {}",
                opt(cx, "appDisplayName"),
                js_join(&Value::Array(
                    v.iter()
                        .map(|x| x.get("appId").cloned().unwrap_or(Value::Null))
                        .collect()
                ))
            )));
        }
        (
            first.get("appId").cloned(),
            first.get("displayName").cloned(),
        )
    };
    let perm = cx.u("permission");
    let elevate = perm == "manage" || perm == "fullcontrol";
    let mut r = Request::post(format!("{GRAPH}/sites/{site}/permissions"))
        .graph_json()
        .header("content-type", "application/json;odata=nometadata")
        .json(obj! { "roles": vec![if elevate { "write".to_string() } else { perm.clone() }], "grantedToIdentities": vec![obj! { "application": obj! { "id": ?app_id, "displayName": ?name } }] })
        .data()
        .await?;
    if elevate {
        r = Request::patch(format!("{GRAPH}/sites/{site}/permissions/{}", u(&r, "id")))
            .graph_json()
            .header("content-type", "application/json;odata=nometadata")
            .json(obj! { "roles": vec![perm] })
            .data()
            .await?;
    }
    cx.log(r)
}

pub async fn apppermission_get(cx: &mut Cx) -> Result<()> {
    let site = spo::graph_site_id(&cx.u("siteUrl")).await?;
    let p = Request::get(format!(
        "{GRAPH}/sites/{site}/permissions/{}",
        opt(cx, "id")
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(Value::Array(transpose(&[p], true)?))
}

pub async fn apppermission_list(cx: &mut Cx) -> Result<()> {
    let site = spo::graph_site_id(&cx.u("siteUrl")).await?;
    let r = Request::get(format!("{GRAPH}/sites/{site}/permissions"))
        .graph_json()
        .data()
        .await?;
    let mut perms = need_arr(&r, "value", "map")?.to_vec();
    if ne(cx, "appId").is_some() || ne(cx, "appDisplayName").is_some() {
        perms = filtered(cx, &perms)?;
    }
    let mut full = Vec::new();
    for p in &perms {
        full.push(
            Request::get(format!("{GRAPH}/sites/{site}/permissions/{}", u(p, "id")))
                .graph_json()
                .data()
                .await?,
        );
    }
    cx.log(Value::Array(transpose(&full, false)?))
}

pub async fn apppermission_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the specified application permission from site {}?",
            opt(cx, "siteUrl")
        ),
    )? {
        return Ok(());
    }
    let site = spo::graph_site_id(&cx.u("siteUrl")).await?;
    let ids: Vec<String> = match ne(cx, "id") {
        Some(i) => vec![i],
        None => {
            let r = Request::get(format!("{GRAPH}/sites/{site}/permissions"))
                .graph_json()
                .data()
                .await?;
            let mut perms = need_arr(&r, "value", "map")?.to_vec();
            if ne(cx, "appId").is_some() || ne(cx, "appDisplayName").is_some() {
                perms = filtered(cx, &perms)?;
            }
            perms.iter().map(|p| u(p, "id")).collect()
        }
    };
    for id in ids {
        Request::delete(format!("{GRAPH}/sites/{site}/permissions/{id}"))
            .graph_json()
            .send()
            .await?;
    }
    Ok(())
}

pub async fn apppermission_set(cx: &mut Cx) -> Result<()> {
    let site = spo::graph_site_id(&cx.u("siteUrl")).await?;
    let id = match ne(cx, "id") {
        Some(i) => i,
        None => {
            let r = Request::get(format!("{GRAPH}/sites/{site}/permissions"))
                .graph_json()
                .data()
                .await?;
            let all = need_arr(&r, "value", "filter")?.to_vec();
            let items = filtered(cx, &all)?;
            if items.is_empty() {
                return Err(Error::msg("The specified app permission does not exist"));
            }
            if items.len() > 1 {
                let mut ids = Vec::new();
                for x in &all {
                    let g = need_arr(x, "grantedToIdentities", "map")?;
                    ids.push(Value::Array(
                        g.iter()
                            .map(|y| {
                                y.get("application")
                                    .and_then(|a| a.get("id"))
                                    .cloned()
                                    .unwrap_or(Value::Null)
                            })
                            .collect(),
                    ));
                }
                return Err(Error::msg(format!(
                    "Multiple app permissions with displayName {} found: {}",
                    opt(cx, "appDisplayName"),
                    js_join(&Value::Array(ids))
                )));
            }
            u(&items[0], "id")
        }
    };
    let r = Request::patch(format!("{GRAPH}/sites/{site}/permissions/{id}"))
        .graph_json()
        .header("content-type", "application/json;odata=nometadata")
        .json(obj! { "roles": vec![cx.val("permission").cloned().unwrap_or(Value::Null)] })
        .data()
        .await?;
    cx.log(r)
}

// ---- archive / chrome / commsite / get / groupify ------------------------------

pub fn url_validate(o: &Obj) -> std::result::Result<(), String> {
    sp_url_check(
        &o.get("url")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
}

pub async fn archive(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to archive site '{url}'?"),
    )? {
        return Ok(());
    }
    let r = async {
        cx.verbose_log(&format!("Archiving site {url}..."));
        let admin = spo::get_spo_admin_url(cx.verbose()).await?;
        let d = sp::digest(&admin).await?;
        let body = format!(
            "<Request AddExpandoFieldTypeSuffix=\"true\" SchemaVersion=\"15.0.0.0\" LibraryVersion=\"16.0.0.0\" ApplicationName=\"{}\" xmlns=\"http://schemas.microsoft.com/sharepoint/clientquery/2009\">\n                  <Actions>\n                    <ObjectPath Id=\"2\" ObjectPathId=\"1\" />\n                    <ObjectPath Id=\"4\" ObjectPathId=\"3\" />\n                    <Query Id=\"5\" ObjectPathId=\"3\">\n                      <Query SelectAllProperties=\"true\">\n                        <Properties />\n                      </Query>\n                    </Query>\n                  </Actions>\n                  <ObjectPaths>\n                    <Constructor Id=\"1\" TypeId=\"{TENANT}\" />\n                    <Method Id=\"3\" ParentId=\"1\" Name=\"ArchiveSiteByUrl\">\n                      <Parameters>\n                        <Parameter Type=\"String\">{url}</Parameter>\n                      </Parameters>\n                    </Method>\n                  </ObjectPaths>\n                  </Request>",
            spo::application_name()
        );
        sp::check_first(&sp::post(&admin, &d, body).await?)
    }
    .await;
    r.map_err(Error::plain)
}

const HEADER: [(&str, i64); 4] = [
    ("Standard", 1),
    ("Compact", 2),
    ("Minimal", 3),
    ("Extended", 4),
];
const FOOTER: [(&str, i64); 2] = [("Simple", 1), ("Extended", 2)];
const ALIGN: [(&str, i64); 3] = [("Left", 0), ("Center", 1), ("Right", 2)];
const EMPHASIS: [(&str, i64); 4] = [("Lightest", 0), ("Light", 1), ("Dark", 2), ("Darkest", 3)];

/// `Enum[key]` for a numeric TS enum (name -> number, number -> name).
fn enum_lookup(e: &[(&str, i64)], v: &Value) -> Option<Value> {
    let k = to_js_string(v);
    if let Some((_, n)) = e.iter().find(|(name, _)| *name == k) {
        return Some(Value::from(*n));
    }
    e.iter()
        .find(|(_, n)| n.to_string() == k)
        .map(|(name, _)| Value::from(*name))
}

pub fn chrome_validate(o: &Obj) -> std::result::Result<(), String> {
    site_url_validate(o)?;
    for (k, e, allowed) in [
        (
            "footerEmphasis",
            &EMPHASIS[..],
            "Lightest|Light|Dark|Darkest",
        ),
        ("footerLayout", &FOOTER[..], "Simple|Extended"),
        (
            "headerEmphasis",
            &EMPHASIS[..],
            "Lightest|Light|Dark|Darkest",
        ),
        (
            "headerLayout",
            &HEADER[..],
            "Standard|Compact|Minimal|Extended",
        ),
        ("logoAlignment", &ALIGN[..], "Left|Center|Right"),
    ] {
        if let Some(v) = o.get(k)
            && enum_lookup(e, v).is_none()
        {
            return Err(format!(
                "{} is not a valid option for {k}. Allowed values {allowed}",
                to_js_string(v)
            ));
        }
    }
    Ok(())
}

pub async fn chrome_set(cx: &mut Cx) -> Result<()> {
    let get = |k: &str, e: &[(&str, i64)]| {
        cx.val(k)
            .filter(|v| truthy(v))
            .map(|v| enum_lookup(e, v).unwrap_or(Value::Null))
    };
    let mut b = Obj::new();
    for (k, e) in [
        ("headerLayout", &HEADER[..]),
        ("headerEmphasis", &EMPHASIS[..]),
        ("logoAlignment", &ALIGN[..]),
        ("footerLayout", &FOOTER[..]),
    ] {
        if let Some(v) = get(k, e).filter(|v| !v.is_null()) {
            b.insert(k.into(), v);
        }
    }
    if let Some(v) = get("footerEmphasis", &EMPHASIS).filter(|v| !v.is_null()) {
        let n = json::parse_int(&to_js_string(&v))
            .map(|n| json::num_value(3.0 - n as f64))
            .unwrap_or(Value::Null);
        b.insert("footerEmphasis".into(), n);
    }
    if let Some(v) = cx.val("hideTitleInHeader").filter(|v| !v.is_null()) {
        b.insert("hideTitleInHeader".into(), v.clone());
    }
    if let Some(v) = cx.val("disableMegaMenu").filter(|v| !v.is_null()) {
        b.insert("megaMenuEnabled".into(), Value::Bool(!truthy(v)));
    }
    if let Some(v) = cx.val("disableFooter").filter(|v| !v.is_null()) {
        b.insert("footerEnabled".into(), Value::Bool(!truthy(v)));
    }
    Request::post(format!("{}/_api/web/SetChromeOptions", cx.u("siteUrl")))
        .spo_json()
        .json(Value::Object(b))
        .send()
        .await?;
    Ok(())
}

pub fn commsite_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(d) = so(o, "designPackageId")
        && !v::is_valid_guid(&d)
    {
        return Err(format!("{d} is not a valid GUID."));
    }
    if let Some(d) = so(o, "designPackage")
        && !["Topic", "Showcase", "Blank"].contains(&d.as_str())
    {
        return Err(format!(
            "{d} is not a valid designPackage. Allowed values are Topic|Showcase|Blank"
        ));
    }
    url_validate(o)
}

pub async fn commsite_enable(cx: &mut Cx) -> Result<()> {
    let id = match ne(cx, "designPackageId") {
        Some(i) => i,
        None => match cx.str("designPackage").as_deref() {
            Some("Blank") => "f6cc5403-0d63-442e-96c0-285923709ffc".into(),
            Some("Showcase") => "6142d2a0-63a5-4ba0-aede-d9fefca2c767".into(),
            _ => "96c933ac-3698-44c7-9f4a-5fd17d71af9e".into(),
        },
    };
    let url = cx.u("url");
    cx.verbose_log(&format!(
        "Enabling communication site with design package '{id}' at '{url}'..."
    ));
    Request::post(format!("{url}/_api/sitepages/communicationsite/enable"))
        .spo_json()
        .json(obj! { "designPackageId": id })
        .send()
        .await?;
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    cx.log(
        Request::get(format!("{}/_api/site", cx.u("url")))
            .spo_json()
            .data()
            .await?,
    )
}

pub async fn groupify(cx: &mut Cx) -> Result<()> {
    let mut p = Obj::new();
    if let Some(d) = ne(cx, "description") {
        p.insert("Description".into(), Value::from(d));
    }
    if let Some(c) = ne(cx, "classification") {
        p.insert("Classification".into(), Value::from(c));
    }
    if cx.bool("keepOldHomepage") {
        p.insert(
            "CreationOptions".into(),
            Value::from(vec!["SharePointKeepOldHomepage"]),
        );
    }
    let body = obj! { "displayName": ?cx.val("displayName").cloned(), "alias": ?cx.val("alias").cloned(), "isPublic": cx.val("isPublic") == Some(&Value::Bool(true)), "optionalParams": Value::Object(p) };
    let r = Request::post(format!(
        "{}/_api/GroupSiteManager/CreateGroupForSite",
        cx.u("url")
    ))
    .header("content-type", "application/json;odata=nometadata")
    .spo_json()
    .header("responseType", "json")
    .json(body)
    .data()
    .await?;
    cx.log(r)
}

// ---- hub sites / theme / records / list / membership ---------------------------

const EMPTY_GUID: &str = "00000000-0000-0000-0000-000000000000";

async fn hub_join(cx: &Cx, id: &str) -> Result<()> {
    let site = cx.u("siteUrl");
    if !cx.bool("asAdmin") {
        Request::post(format!("{site}/_api/site/JoinHubSite('{}')", eqp(id)))
            .spo_json()
            .send()
            .await?;
    } else {
        let admin = spo::get_spo_admin_url(cx.verbose()).await?;
        Request::post(format!("{admin}/_api/SPO.Tenant/ConnectSiteToHubSiteById"))
            .spo_json()
            .json(obj! { "siteUrl": site, "hubSiteId": id })
            .send()
            .await?;
    }
    Ok(())
}

pub async fn hubsite_connect(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Connecting site collection {} to hub site {}...",
        opt(cx, "siteUrl"),
        opt(cx, "id")
    ));
    hub_join(cx, &cx.u("id")).await
}

pub async fn hubsite_disconnect(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to disconnect the site collection {} from its hub site?",
            opt(cx, "siteUrl")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Disconnecting site collection {} from its hub site...",
        opt(cx, "siteUrl")
    ));
    hub_join(cx, EMPTY_GUID).await
}

pub fn web_url_validate(o: &Obj) -> std::result::Result<(), String> {
    sp_url_check(
        &o.get("webUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
}

pub async fn hubsite_theme_sync(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Syncing hub site theme...");
    Request::post(format!("{}/_api/web/SyncHubSiteTheme", cx.u("webUrl")))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn inplacerecordsmanagement_set(cx: &mut Cx) -> Result<()> {
    let enabled = cx.val("enabled").is_some_and(truthy);
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "{} in-place records management for site {site}",
        if enabled {
            "Activating"
        } else {
            "Deactivating"
        }
    ));
    Request::post(format!(
        "{site}/_api/site/features/{}",
        if enabled { "add" } else { "remove" }
    ))
    .spo_json()
    .json(obj! { "featureId": "da2e115b-07e4-49d9-bb2c-35e93bb9fca9", "force": true })
    .send()
    .await?;
    Ok(())
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let template = match ne(cx, "webTemplate") {
        Some(w) => w,
        None if cx.bool("withOneDriveSites") => String::new(),
        None => match cx.str("type").as_deref() {
            Some("TeamSite") => "GROUP#0".into(),
            Some("CommunicationSite") => "SITEPAGEPUBLISHING#0".into(),
            _ => String::new(),
        },
    };
    let personal = if cx.bool("withOneDriveSites") {
        "1"
    } else {
        "0"
    };
    let provided = ne(cx, "filter").unwrap_or_default();
    let state = match cx.str("state").as_deref() {
        Some("active") => "ArchiveStatus -eq 'NotArchived'",
        Some("fullyArchived") => "ArchiveStatus -eq 'FullyArchived'",
        Some("recentlyArchived") => "ArchiveStatus -eq 'RecentlyArchived'",
        Some("archived") => "ArchiveStatus -ne 'NotArchived'",
        _ => "",
    };
    let filter = if !state.is_empty() && !provided.is_empty() {
        format!("{provided} and {state}")
    } else if !state.is_empty() {
        state.to_string()
    } else {
        provided
    };
    let filter = util::escape_xml_value(Some(&Value::from(filter)));
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Retrieving list of site collections...");
    let mut all = Vec::new();
    let mut start = "0".to_string();
    loop {
        let d = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
        let body = sp::request(
            r#"<ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties /></ChildItemQuery></Query>"#,
            &format!(
                r#"<Constructor Id="1" TypeId="{TENANT}" /><Method Id="3" ParentId="1" Name="GetSitePropertiesFromSharePointByFilters"><Parameters><Parameter TypeId="{{b92aeee2-c92c-4b67-abcc-024e471bc140}}"><Property Name="Filter" Type="String">{filter}</Property><Property Name="IncludeDetail" Type="Boolean">false</Property><Property Name="IncludePersonalSite" Type="Enum">{personal}</Property><Property Name="StartIndex" Type="String">{start}</Property><Property Name="Template" Type="String">{template}</Property></Parameter></Parameters></Method>"#
            ),
        );
        let json = sp::post(&admin, &d.value, body).await?;
        sp::check_first(&json)?;
        let last = csom_last(&json);
        let items = match last.get("_Child_Items_") {
            Some(Value::Array(a)) => a.clone(),
            _ if last.is_null() => {
                return Err(Error::js(
                    "Cannot read properties of null (reading '_Child_Items_')",
                ));
            }
            _ => {
                return Err(Error::js(
                    "Spread syntax requires ...iterable[Symbol.iterator] to be a function",
                ));
            }
        };
        all.extend(items);
        match last
            .get("NextStartIndexFromSharePoint")
            .filter(|v| truthy(v))
        {
            Some(n) => start = to_js_string(n),
            None => break,
        }
    }
    cx.log(Value::Array(all))
}

pub fn membership_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(r) = so(o, "role")
        && !["owner", "member", "visitor"].contains(&r.to_lowercase().as_str())
    {
        return Err(format!(
            "'{r}' is not a valid value for option 'role'. Valid values are: Owner, Member, Visitor"
        ));
    }
    site_url_validate(o)
}

pub async fn membership_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.verbose()).await?;
    let role = cx.s("role").map(|r| r.to_lowercase());
    let ids = match role.as_deref() {
        Some("owner") => "0",
        Some("member") => "1",
        Some("visitor") => "2",
        _ => "0,1,2",
    };
    let props = sp::site_admin_properties(cx, &cx.u("siteUrl"), false).await?;
    let r = odata::get_all_items(&format!(
        "{admin}/_api/SPO.Tenant/sites/GetSiteUserGroups?siteId='{}'&userGroupIds=[{ids}]",
        u(&props, "SiteId")
    ))
    .await?;
    let group = |i: usize| -> Result<Option<Value>> {
        match r.get(i) {
            Some(g) => Ok(g.get("userGroup").cloned()),
            None => Err(Error::js(
                "Cannot read properties of undefined (reading 'userGroup')",
            )),
        }
    };
    let single: Option<(&str, &str)> = match role.as_deref() {
        Some("owner") => Some(("AssociatedOwnerGroup", "Owner")),
        Some("member") => Some(("AssociatedMemberGroup", "Member")),
        Some("visitor") => Some(("AssociatedVisitorGroup", "Visitor")),
        _ => None,
    };
    if cx.str("output").as_deref() == Some("json") {
        let out = match single {
            Some((k, _)) => {
                let mut o = Obj::new();
                if let Some(g) = group(0)? {
                    o.insert(k.into(), g);
                }
                Value::Object(o)
            }
            None => {
                obj! { "AssociatedOwnerGroup": ?group(0)?, "AssociatedMemberGroup": ?group(1)?, "AssociatedVisitorGroup": ?group(2)? }
            }
        };
        return cx.log(out);
    }
    let map = |i: usize, ty: &str| -> Result<Vec<Value>> {
        let g = group(i)?;
        let Some(Value::Array(users)) = g else {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'map')",
            ));
        };
        Ok(users
            .into_iter()
            .map(|mut u| {
                if let Some(o) = u.as_object_mut() {
                    o.insert("associatedGroupType".into(), Value::from(ty));
                }
                u
            })
            .collect())
    };
    let out = match single {
        Some((_, ty)) => map(0, ty)?,
        None => {
            let mut v = map(0, "Owner")?;
            v.extend(map(1, "Member")?);
            v.extend(map(2, "Visitor")?);
            v
        }
    };
    cx.log(Value::Array(out))
}

// ---- recycle bin ---------------------------------------------------------------

pub async fn recyclebinitem_clear(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to clear the recycle bin of site {site}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Permanently removing all items in recycle bin of site {site}..."
    ));
    let url = if cx.bool("secondary") {
        cx.verbose_log("Removing all items from the second-stage recycle bin");
        format!("{site}/_api/site/RecycleBin/DeleteAllSecondStageItems")
    } else {
        cx.verbose_log("Removing all items from the first-stage recycle bin");
        format!("{site}/_api/web/RecycleBin/DeleteAll")
    };
    let r = Request::post(url).spo_json().data().await?;
    if r.get("odata.null") != Some(&Value::Bool(true)) {
        return Err(Error::Thrown(r));
    }
    Ok(())
}

const ITEM_TYPES: [(i32, &str); 3] = [(1, "files"), (3, "listItems"), (5, "folders")];

pub fn recyclebinitem_list_validate(o: &Obj) -> std::result::Result<(), String> {
    site_url_validate(o)?;
    if let Some(t) = o.get("type")
        && !ITEM_TYPES.iter().any(|(_, v)| t.as_str() == Some(v))
    {
        return Err(format!(
            "{} is not a valid value. Allowed values are files, listItems, folders",
            to_js_string(t)
        ));
    }
    Ok(())
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "Retrieving all items from recycle bin at {site}..."
    ));
    let state = if cx.bool("secondary") { "2" } else { "1" };
    let mut url = format!("{site}/_api/site/RecycleBin?$filter=(ItemState eq {state})");
    if let Some((id, _)) = cx
        .val("type")
        .and_then(|t| ITEM_TYPES.iter().find(|(_, v)| t.as_str() == Some(v)))
    {
        url.push_str(&format!(" and (ItemType eq {id})"));
    }
    let r = Request::get(url).spo_json().data().await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

fn guid_array_check(o: &Obj) -> std::result::Result<(), String> {
    if let Some(i) = so(o, "ids") {
        match v::is_valid_guid_array(&i) {
            Valid::Yes => {}
            Valid::Msg(m) => {
                return Err(format!(
                    "The following GUIDs are invalid for the option 'ids': {m}."
                ));
            }
            Valid::No => {
                return Err("The following GUIDs are invalid for the option 'ids': false.".into());
            }
        }
    }
    Ok(())
}

pub fn recyclebinitem_move_validate(o: &Obj) -> std::result::Result<(), String> {
    site_url_validate(o)?;
    guid_array_check(o)
}

pub fn recyclebinitem_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    site_url_validate(o)?;
    match v::is_valid_guid_array(
        &o.get("ids")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    ) {
        Valid::Yes => Ok(()),
        Valid::Msg(m) => Err(format!(
            "The following GUIDs are invalid for the option 'ids': {m}."
        )),
        Valid::No => Err("The following GUIDs are invalid for the option 'ids': false.".into()),
    }
}

pub async fn recyclebinitem_move(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to move these items to the second-stage recycle bin?",
    )? {
        return Ok(());
    }
    let mut r = Request::post(format!(
        "{}/_api/web/recycleBin/MoveAllToSecondStage",
        cx.u("siteUrl")
    ))
    .spo_json();
    if let Some(ids) = cx.s("ids") {
        let ids: Vec<&str> = ids.split(',').collect();
        cx.verbose_log(&format!(
            "Moving {} items to the second-stage recycle bin.",
            ids.len()
        ));
        r = r.json(obj! { "ids": ids });
    } else {
        cx.verbose_log("Moving all items to the second-stage recycle bin.");
    }
    r.send().await?;
    Ok(())
}

pub async fn recyclebinitem_remove(cx: &mut Cx) -> Result<()> {
    let ids = cx.u("ids");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to permanently delete {} item(s) from the site recycle bin?",
            ids.split(',').count()
        ),
    )? {
        return Ok(());
    }
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "Permanently deleting items from the site recycle bin at site {site}..."
    ));
    Request::post(format!("{site}/_api/site/RecycleBin/DeleteByIds"))
        .spo_json()
        .json(obj! { "ids": ids.split(',').collect::<Vec<_>>() })
        .send()
        .await?;
    Ok(())
}

pub fn recyclebinitem_restore_validate(o: &Obj) -> std::result::Result<(), String> {
    site_url_validate(o)?;
    if let Some(i) = so(o, "ids") {
        let bad: Vec<String> = util::split_and_trim(&i)
            .into_iter()
            .filter(|x| !v::is_valid_guid(x))
            .collect();
        if !bad.is_empty() {
            return Err(format!("The following IDs are invalid: {}", bad.join(", ")));
        }
    }
    let (ids, p, s) = (t(o, "ids"), t(o, "allPrimary"), t(o, "allSecondary"));
    if (!ids && !p && !s) || (ids && (p || s)) {
        return Err("Option 'ids' cannot be used with 'allPrimary' or 'allSecondary'.".into());
    }
    Ok(())
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!("Restoring items from recycle bin at {site}..."));
    let base = format!("{site}/_api");
    let post = |u: String| {
        Request::post(u)
            .spo_json()
            .header("content-type", "application/json")
    };
    if let Some(ids) = ne(cx, "ids") {
        let ids = util::split_and_trim(&ids);
        for chunk in ids.chunks(20) {
            post(format!("{base}/site/RecycleBin/RestoreByIds"))
                .json(obj! { "ids": chunk.to_vec() })
                .send()
                .await?;
        }
    } else {
        let (p, s) = (cx.bool("allPrimary"), cx.bool("allSecondary"));
        let url = if p && s {
            format!("{base}/site/RecycleBin/RestoreAll")
        } else if p {
            format!("{base}/web/RecycleBin/RestoreAll")
        } else {
            format!("{base}/site/GetRecycleBinItems(rowLimit=2000000000,itemState=2)/RestoreAll")
        };
        let mut r = post(url);
        r.response_type = m365_core::http::ResponseType::Text;
        r.send().await?;
    }
    Ok(())
}

// ---- remove / rename -----------------------------------------------------------

pub fn remove_validate(o: &Obj) -> std::result::Result<(), String> {
    let url = o
        .get("url")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    sp_url_check(&url)?;
    let parsed = url::Url::parse(&url).map_err(|_| "Invalid URL".to_string())?;
    let root = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or(""));
    if root.to_lowercase() == m365_core::ut::url::remove_trailing_slashes(&url.to_lowercase()) {
        return Err("The root site cannot be deleted.".into());
    }
    if t(o, "fromRecycleBin") && t(o, "permanent") {
        return Err("Specify either fromRecycleBin or permanent, but not both.".into());
    }
    Ok(())
}

async fn group_in_recycle_bin(cx: &Cx, id: &str) -> Result<bool> {
    cx.verbose_log(&format!(
        "Checking if group '{id}' is in the Microsoft Entra recycle bin."
    ));
    match Request::get(format!(
        "{GRAPH}/directory/deletedItems/Microsoft.Graph.Group/{id}?$select=id"
    ))
    .graph_json()
    .send()
    .await
    {
        Ok(_) => Ok(true),
        Err(e) if e.status() == Some(404) => Ok(false),
        Err(e) => Err(e),
    }
}

async fn purge_group(cx: &Cx, id: &str) -> Result<()> {
    cx.verbose_log(&format!("Permanently deleting group '{id}'."));
    Request::delete(format!(
        "{GRAPH}/directory/deletedItems/Microsoft.Graph.Group/{id}"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

async fn purge_site(cx: &Cx, admin: &str, url: &str) -> Result<()> {
    cx.verbose_log("Permanently deleting site from the recycle bin.");
    Request::post(format!(
        "{admin}/_api/Microsoft.Online.SharePoint.TenantAdministration.Tenant/RemoveDeletedSite"
    ))
    .spo_json()
    .header("Content-Type", "application/json")
    .json(obj! { "siteUrl": url })
    .send()
    .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the site '{url}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing site '{url}'..."));
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let site = m365_core::ut::url::remove_trailing_slashes(&url);
    cx.verbose_log("Retrieving site info.");
    let sites = odata::get_all_items(&format!(
        "{admin}/_api/web/lists/GetByTitle('DO_NOT_DELETE_SPLIST_TENANTADMIN_AGGREGATED_SITECOLLECTIONS')/items?$filter=SiteUrl eq '{}'&$select=GroupId,TimeDeleted,SiteId",
        eqp(&site)
    ))
    .await?;
    let Some(details) = sites.first() else {
        return Err(Error::msg("Site not found in the tenant."));
    };
    let group = details
        .get("GroupId")
        .filter(|g| truthy(g))
        .map(to_js_string)
        .filter(|g| g != EMPTY_GUID);
    let deleted = details.get("TimeDeleted").is_some_and(truthy);
    if cx.bool("fromRecycleBin") {
        if !deleted {
            return Err(Error::msg(
                "Site is currently not in the recycle bin. Remove --fromRecycleBin if you want to remove it as active site.",
            ));
        }
        if let Some(g) = &group {
            cx.verbose_log(&format!(
                "Checking if group '{g}' is already permanently deleted from recycle bin."
            ));
            if group_in_recycle_bin(cx, g).await? {
                purge_group(cx, g).await?;
            }
        }
        return purge_site(cx, &admin, &site).await;
    }
    if deleted {
        return Err(Error::msg(
            "Site is already in the recycle bin. Use --fromRecycleBin to permanently delete it.",
        ));
    }
    let permanent = cx.bool("permanent");
    match &group {
        Some(g) => {
            cx.verbose_log("Removing groupified site.");
            Request::post(format!(
                "{admin}/_api/GroupSiteManager/Delete?siteUrl='{}'",
                eqp(&site)
            ))
            .spo_json()
            .send()
            .await?;
            if permanent {
                let mut found = group_in_recycle_bin(cx, g).await?;
                let mut polls = 0;
                while !found && polls < 20 {
                    util::sleep_ms(5000).await;
                    found = group_in_recycle_bin(cx, g).await?;
                    polls += 1;
                }
                if found {
                    purge_group(cx, g).await?;
                }
            }
        }
        None => {
            cx.verbose_log("Deleting site.");
            Request::post(format!(
                "{admin}/_api/Microsoft.Online.SharePoint.TenantAdministration.Tenant/RemoveSite"
            ))
            .spo_json()
            .json(obj! { "siteUrl": site.as_str() })
            .send()
            .await?;
        }
    }
    if permanent {
        purge_site(cx, &admin, &site).await?;
    }
    Ok(())
}

pub fn rename_validate(o: &Obj) -> std::result::Result<(), String> {
    let a = o.get("url").map(to_js_string).unwrap_or_default();
    let b = o.get("newUrl").map(to_js_string).unwrap_or_default();
    if a.to_lowercase() == b.to_lowercase() {
        return Err("The new URL cannot be the same as the target URL.".into());
    }
    Ok(())
}

pub async fn rename(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log("Scheduling rename job...");
    let mut bits = 0;
    if cx.bool("suppressMarketplaceAppCheck") {
        bits |= 8;
    }
    if cx.bool("suppressWorkflow2013Check") {
        bits |= 16;
    }
    let url = cx.u("url");
    let mut op = Request::post(format!("{admin}/_api/SiteRenameJobs?api-version=1.4.7"))
        .header("X-RequestDigest", d)
        .header("Content-Type", "application/json")
        .json_response()
        .json(obj! {
            "SourceSiteUrl": url.as_str(), "TargetSiteUrl": ?cx.val("newUrl").cloned(),
            "TargetSiteTitle": cx.val("newTitle").filter(|v| truthy(v)).cloned().unwrap_or(Value::Null),
            "Option": bits, "Reserve": Value::Null, "SkipGestures": Value::Null, "OperationId": EMPTY_GUID,
        })
        .data()
        .await?;
    if cx.val("verbose").is_some_and(truthy) {
        m365_core::log::log_stderr_value(&op);
    }
    if op.get("JobState").and_then(Value::as_str) == Some("Error") {
        return Err(Error::Thrown(
            op.get("ErrorDescription").cloned().unwrap_or(Value::Null),
        ));
    }
    if cx.bool("wait") && op.get("JobState").and_then(Value::as_str) != Some("Success") {
        let mut i = 0;
        loop {
            i += 1;
            let r = Request::get(format!(
                "{admin}/_api/SiteRenameJobs/GetJobsBySiteUrl(url='{}')?api-version=1.4.7",
                eqp(&url)
            ))
            .header("X-AttemptNumber", i.to_string())
            .json_response()
            .data()
            .await?;
            op = match r.get("value") {
                Some(Value::Array(a)) => a.first().cloned().ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'ErrorDescription')")
                })?,
                _ => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading '0')",
                    ));
                }
            };
            if let Some(e) = op.get("ErrorDescription").filter(|e| truthy(e)) {
                return Err(Error::Thrown(e.clone()));
            }
            if op.get("JobState").and_then(Value::as_str) == Some("Success") {
                break;
            }
            util::sleep_ms(5000).await;
        }
    }
    cx.log(op)
}

// ---- set -----------------------------------------------------------------------

const SHARING: [&str; 4] = [
    "Disabled",
    "ExternalUserSharingOnly",
    "ExternalUserAndGuestSharing",
    "ExistingExternalUserSharingOnly",
];

pub fn set_validate(o: &Obj) -> std::result::Result<(), String> {
    url_validate(o)?;
    const ANY: [&str; 19] = [
        "classification",
        "disableFlows",
        "socialBarOnSitePagesDisabled",
        "title",
        "description",
        "isPublic",
        "owners",
        "shareByEmailEnabled",
        "siteDesignId",
        "sharingCapability",
        "siteLogoUrl",
        "siteThumbnailUrl",
        "resourceQuota",
        "resourceQuotaWarningLevel",
        "storageQuota",
        "storageQuotaWarningLevel",
        "noScriptSite",
        "allowSelfServiceUpgrade",
        "lockState",
    ];
    if !ANY.iter().any(|k| o.contains_key(*k)) {
        return Err("Specify at least one property to update".into());
    }
    for k in ["siteLogoUrl", "siteThumbnailUrl"] {
        if let Some(v) = o.get(k)
            && !v.is_string()
        {
            return Err(format!(
                "{} is not a valid value for the {k} option. Specify the logo URL or an empty string \"\" to unset the logo.",
                to_js_string(v)
            ));
        }
    }
    if let Some(d) = so(o, "siteDesignId")
        && !v::is_valid_guid(&d)
    {
        return Err(format!("{d} is not a valid GUID"));
    }
    if let Some(s) = so(o, "sharingCapability")
        && !SHARING.contains(&s.as_str())
    {
        return Err(format!(
            "{s} is not a valid value for the sharingCapability option. Allowed values are {}",
            SHARING.join("|")
        ));
    }
    for (k, base) in [
        ("resourceQuota", ""),
        ("resourceQuotaWarningLevel", "resourceQuota"),
        ("storageQuota", ""),
        ("storageQuotaWarningLevel", "storageQuota"),
    ] {
        if let Some(m) = not_number(o, k) {
            return Err(m);
        }
        if !base.is_empty() && t(o, base) && t(o, k) && num(o, k) > num(o, base) {
            return Err(format!("{k} must not exceed the {base}"));
        }
    }
    if let Some(l) = so(o, "lockState")
        && !["Unlock", "NoAdditions", "ReadOnly", "NoAccess"].contains(&l.as_str())
    {
        return Err(format!(
            "{l} is not a valid value for the lockState option. Allowed values Unlock|NoAdditions|ReadOnly|NoAccess"
        ));
    }
    Ok(())
}

async fn wait_csom(
    cx: &Cx,
    admin: &str,
    ctx: &Option<FormDigest>,
    res: Option<String>,
) -> Result<()> {
    let Some(res) = res.filter(|r| !r.is_empty()) else {
        return Ok(());
    };
    let json = sp::parse(&res)?;
    sp::check_first(&json)?;
    let op = csom_last(&json);
    if !cx.bool("wait") || op.get("IsComplete").is_some_and(truthy) {
        return Ok(());
    }
    wait_op(cx, admin, &op, ctx.clone()).await
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    set_inner(cx).await.map_err(Error::plain)
}

async fn set_inner(cx: &Cx) -> Result<()> {
    let url = cx.u("url");
    let tenant = spo::get_tenant_id(cx.debug()).await?;
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let mut ctx = Some(spo::ensure_form_digest(&admin, None, cx.debug()).await?);
    let dg = |c: &Option<FormDigest>| c.as_ref().map(|c| c.value.clone()).unwrap_or_default();
    cx.debug_log("Loading site IDs...");
    let info = Request::get(format!("{url}/_api/site?$select=GroupId,Id"))
        .spo_json()
        .data()
        .await?;
    let (group_id, site_id) = (info.get("GroupId").cloned(), info.get("Id").cloned());
    cx.debug_log(&format!(
        "Retrieved site IDs. siteId: {}, groupId: {}",
        site_id
            .as_ref()
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
        group_id
            .as_ref()
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    ));
    let gid = group_id
        .as_ref()
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let group_connected = group_id.as_ref().and_then(Value::as_str) != Some(EMPTY_GUID);
    if group_connected {
        cx.debug_log(&format!("Site attached to group {gid}"));
        if ["title", "description", "isPublic", "owners"]
            .iter()
            .any(|k| cx.has(k))
        {
            if let Some(title) = cx.val("title") {
                Request::post(format!("{admin}/_api/SPOGroup/UpdateGroupPropertiesBySiteId"))
                    .spo_json()
                    .header("content-type", "application/json;charset=utf-8")
                    .header("X-RequestDigest", dg(&ctx))
                    .json(obj! { "groupId": ?group_id.clone(), "siteId": ?site_id.clone(), "displayName": title.clone() })
                    .send()
                    .await?;
            }
            if let Some(p) = cx.val("isPublic") {
                m365_core::ut::entra::group::set(
                    &gid,
                    Some(p == &Value::Bool(false)),
                    None,
                    None,
                    cx.verbose(),
                )
                .await?;
            }
            if let Some(d) = ne(cx, "description") {
                let mut r = Request::patch(format!("{GRAPH}/groups/{gid}"))
                    .header("content-type", "application/json;odata.metadata=none")
                    .json(obj! { "description": d });
                r.response_type = m365_core::http::ResponseType::Text;
                r.send().await?;
            }
            if let Some(owners) = cx.s("owners") {
                let owners: Vec<String> = owners.split(',').map(|o| o.trim().to_string()).collect();
                cx.verbose_log("Retrieving user information to set group owners...");
                let filter = owners
                    .iter()
                    .map(|o| format!("userPrincipalName eq '{o}'"))
                    .collect::<Vec<_>>()
                    .join(" or ");
                let r = Request::get(format!("{GRAPH}/users?$filter={filter}&$select=id"))
                    .header("content-type", "application/json;odata.metadata=none")
                    .json_response()
                    .data()
                    .await?;
                for user in need_arr(&r, "value", "length")? {
                    Request::post(format!("{admin}/_api/SP.Directory.DirectorySession/Group('{gid}')/Owners/Add(objectId='{}', principalName='')", u(user, "id")))
                        .header("content-type", "application/json;odata=verbose")
                        .send()
                        .await?;
                }
            }
        }
    } else {
        cx.debug_log("Site is not group connected");
        if cx.has("isPublic") {
            return Err(Error::msg(
                "The isPublic option can't be set on a site that is not groupified",
            ));
        }
        if let Some(d) = ne(cx, "description") {
            cx.verbose_log(&format!("Setting site description {url}..."));
            Request::post(format!("{url}/_api/web"))
                .header("IF-MATCH", "*")
                .header("Accept", "application/json;odata=nometadata")
                .header("content-type", "application/json;odata=nometadata")
                .header("X-RequestDigest", dg(&ctx))
                .header("X-HTTP-Method", "MERGE")
                .json(obj! { "Description": d })
                .send()
                .await?;
        }
        if let Some(owners) = ne(cx, "owners") {
            cx.verbose_log(&format!("Updating site owners {url}..."));
            for o in owners.split(',') {
                let body = sp::request(
                    r#"<ObjectPath Id="48" ObjectPathId="47" />"#,
                    &format!(
                        r#"<Method Id="47" ParentId="34" Name="SetSiteAdmin"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter><Parameter Type="Boolean">true</Parameter></Parameters></Method><Constructor Id="34" TypeId="{TENANT}" />"#,
                        util::escape_xml(&url),
                        util::escape_xml(o.trim())
                    ),
                );
                sp::check_first(&sp::post(&admin, &dg(&ctx), body).await?)?;
            }
        }
    }
    // updateSiteProperties
    let shared = [
        "classification",
        "disableFlows",
        "socialBarOnSitePagesDisabled",
        "shareByEmailEnabled",
        "sharingCapability",
        "noScriptSite",
    ];
    let site_props = [
        "title",
        "resourceQuota",
        "resourceQuotaWarningLevel",
        "storageQuota",
        "storageQuotaWarningLevel",
        "allowSelfServiceUpgrade",
    ];
    let any = shared.iter().any(|k| cx.has(k))
        || (!group_connected && site_props.iter().any(|k| cx.has(k)));
    let mut response: Option<String> = None;
    if any {
        let d = spo::ensure_form_digest(&admin, ctx.as_ref(), cx.debug()).await?;
        ctx = Some(d.clone());
        cx.verbose_log(&format!("Updating site {url} properties..."));
        let mut id = 27;
        let (mut payload, mut site_payload) = (Vec::new(), Vec::new());
        let mut sp_ = |p: &mut Vec<String>, name: &str, ty: &str, val: String| {
            p.push(format!(r#"<SetProperty Id="{id}" ObjectPathId="5" Name="{name}"><Parameter Type="{ty}">{val}</Parameter></SetProperty>"#));
            id += 1;
        };
        if !group_connected {
            if let Some(tt) = ne(cx, "title") {
                sp_(&mut payload, "Title", "String", util::escape_xml(&tt));
            }
            for (k, name, ty) in [
                ("resourceQuota", "UserCodeMaximumLevel", "Double"),
                (
                    "resourceQuotaWarningLevel",
                    "UserCodeWarningLevel",
                    "Double",
                ),
                ("storageQuota", "StorageMaximumLevel", "Int64"),
                ("storageQuotaWarningLevel", "StorageWarningLevel", "Int64"),
            ] {
                if let Some(v) = cx.val(k).filter(|v| truthy(v)) {
                    sp_(&mut payload, name, ty, to_js_string(v));
                }
            }
            if let Some(v) = cx.val("allowSelfServiceUpgrade") {
                sp_(
                    &mut payload,
                    "AllowSelfServiceUpgrade",
                    "Boolean",
                    to_js_string(v),
                );
            }
        }
        if let Some(Value::String(c)) = cx.val("classification") {
            sp_(
                &mut site_payload,
                "Classification",
                "String",
                util::escape_xml(c),
            );
        }
        if let Some(v) = cx.val("disableFlows") {
            sp_(
                &mut payload,
                "DisableFlows",
                "Enum",
                if truthy(v) { "1".into() } else { "2".into() },
            );
        }
        if let Some(v) = cx.val("socialBarOnSitePagesDisabled") {
            sp_(
                &mut payload,
                "SocialBarOnSitePagesDisabled",
                "Boolean",
                to_js_string(v),
            );
        }
        if let Some(v) = cx.val("shareByEmailEnabled") {
            sp_(
                &mut site_payload,
                "ShareByEmailEnabled",
                "Boolean",
                to_js_string(v),
            );
        }
        if let Some(s) = ne(cx, "sharingCapability") {
            let n = SHARING
                .iter()
                .position(|x| *x == s)
                .map(|i| i.to_string())
                .unwrap_or_else(|| "undefined".into());
            sp_(&mut payload, "SharingCapability", "Enum", n);
        }
        if let Some(v) = cx.val("noScriptSite") {
            payload.push(format!(r#"<SetProperty Id="{id}" ObjectPathId="5" Name="DenyAddAndCustomizePages"><Parameter Type="Enum">{}</Parameter></SetProperty>"#, if truthy(v) { 2 } else { 1 }));
        }
        let mut site_response = None;
        if !site_payload.is_empty() {
            let body = sp::request(
                &site_payload.join(""),
                r#"<StaticProperty Id="1" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" /><Property Id="5" ParentId="1" Name="Site" />"#,
            );
            site_response = Some(spo::process_query(&url, &d.value, body).await?);
        }
        if !payload.is_empty() {
            let pos = tenant.find('|').map(|p| p + 1).unwrap_or(0);
            let end = tenant.find('&').unwrap_or(0);
            let mid = if end > pos { &tenant[pos..end] } else { "" };
            let body = sp::request(
                &format!(
                    r#"{}<ObjectPath Id="14" ObjectPathId="13" /><ObjectIdentityQuery Id="15" ObjectPathId="5" /><Query Id="16" ObjectPathId="13"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
                    payload.join("")
                ),
                &format!(
                    r#"<Identity Id="5" Name="53d8499e-d0d2-5000-cb83-9ade5be42ca4|{mid}&#xA;SiteProperties&#xA;{}" /><Method Id="13" ParentId="5" Name="Update" />"#,
                    eqp(&url)
                ),
            );
            response = Some(spo::process_query(&admin, &d.value, body).await?);
        }
        response = response.filter(|r| !r.is_empty()).or(site_response);
    }
    wait_csom(cx, &admin, &ctx, response).await?;
    if let Some(d) = cx.val("siteDesignId") {
        // upstream passes (url, siteDesignId) to applySiteDesign(id, webUrl)
        cx.verbose_log(&format!("Applying site design {url}..."));
        let spo_url = spo::get_spo_url(cx.verbose()).await?;
        Request::post(format!("{spo_url}/_api/Microsoft.Sharepoint.Utilities.WebTemplateExtensions.SiteScriptUtility.ApplySiteDesign"))
            .header("content-type", "application/json;charset=utf-8")
            .spo_json()
            .json(obj! { "siteDesignId": url.as_str(), "webUrl": d.clone() })
            .send()
            .await?;
    }
    for (k, aspect, label) in [
        ("siteLogoUrl", 1, "Setting the site its logo..."),
        ("siteThumbnailUrl", 0, "Setting the site thumbnail..."),
    ] {
        if let Some(v) = cx.val(k) {
            cx.debug_log(label);
            let rel = if truthy(v) {
                m365_core::ut::url::get_url_relative_path(&to_js_string(v))
            } else {
                String::new()
            };
            Request::post(format!("{url}/_api/siteiconmanager/setsitelogo"))
                .spo_json()
                .json(obj! { "aspect": aspect, "relativeLogoUrl": rel, "type": 0 })
                .send()
                .await?;
        }
    }
    let lock = match ne(cx, "lockState") {
        None => None,
        Some(l) => {
            cx.verbose_log(&format!("Setting site lock state {url}..."));
            let body = sp::request(
                &format!(
                    r#"<SetProperty Id="7" ObjectPathId="5" Name="LockState"><Parameter Type="String">{}</Parameter></SetProperty><ObjectPath Id="9" ObjectPathId="8" /><ObjectIdentityQuery Id="10" ObjectPathId="5" /><Query Id="11" ObjectPathId="8"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
                    util::escape_xml(&l)
                ),
                &format!(
                    r#"<Method Id="5" ParentId="3" Name="GetSitePropertiesByUrl"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Boolean">false</Parameter></Parameters></Method><Method Id="8" ParentId="5" Name="Update" /><Constructor Id="3" TypeId="{TENANT}" />"#,
                    util::escape_xml(&url)
                ),
            );
            Some(spo::process_query(&admin, &dg(&ctx), body).await?)
        }
    };
    wait_csom(cx, &admin, &ctx, lock).await
}

// ---- sharing permission / unarchive / version policy ---------------------------

pub async fn sharingpermission_set(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "Updating sharing permissions for site '{site}'..."
    ));
    let cap = cx.u("capability");
    cx.verbose_log("Updating site sharing permissions...");
    Request::patch(format!("{site}/_api/Web"))
        .spo_json()
        .json(obj! { "MembersCanShare": cap == "full" || cap == "limited" })
        .send()
        .await?;
    cx.verbose_log("Updating associated member group sharing permissions...");
    Request::patch(format!("{site}/_api/Web/AssociatedMemberGroup"))
        .spo_json()
        .json(obj! { "AllowMembersEditMembership": cap == "full" })
        .send()
        .await?;
    Ok(())
}

pub async fn unarchive(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed("force", &format!("Are you sure you want to unarchive site '{url}'? You won't be able to rearchive this site for 120 days."))? {
        return Ok(());
    }
    cx.verbose_log(&format!("Unarchiving site '{url}'..."));
    let r = async {
        let admin = spo::get_spo_admin_url(cx.verbose()).await?;
        let d = sp::digest(&admin).await?;
        let body = format!(
            "<Request AddExpandoFieldTypeSuffix=\"true\" SchemaVersion=\"15.0.0.0\" LibraryVersion=\"16.0.0.0\" ApplicationName=\"{}\" xmlns=\"http://schemas.microsoft.com/sharepoint/clientquery/2009\">\n                <Actions>\n                  <ObjectPath Id=\"2\" ObjectPathId=\"1\" />\n                  <ObjectPath Id=\"4\" ObjectPathId=\"3\" />\n                </Actions>\n                <ObjectPaths>\n                  <Constructor Id=\"1\" TypeId=\"{TENANT}\" />\n                  <Method Id=\"3\" ParentId=\"1\" Name=\"UnarchiveSiteByUrl\">\n                    <Parameters>\n                      <Parameter Type=\"String\">{url}</Parameter>\n                    </Parameters>\n                  </Method>\n                </ObjectPaths>\n              </Request>",
            spo::application_name()
        );
        sp::check_first(&sp::post(&admin, &d, body).await?)
    }
    .await;
    r.map_err(Error::plain)
}

pub async fn versionpolicy_get(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "Retrieving version policy settings for site '{site}'..."
    ));
    let r = Request::get(format!(
        "{site}/_api/site/VersionPolicyForNewLibrariesTemplate?$expand=VersionPolicies"
    ))
    .spo_json()
    .data()
    .await?;
    let vp = r.get("VersionPolicies").filter(|v| truthy(v));
    let mode = if r.get("MajorVersionLimit").and_then(Value::as_f64) == Some(-1.0) {
        "inheritTenant"
    } else {
        match vp
            .and_then(|v| v.get("DefaultTrimMode"))
            .and_then(Value::as_f64)
        {
            Some(1.0) => "age",
            Some(2.0) => "automatic",
            _ => "number",
        }
    };
    let days = vp
        .and_then(|v| v.get("DefaultExpireAfterDays"))
        .cloned()
        .unwrap_or(Value::Null);
    cx.log(obj! { "defaultTrimMode": mode, "defaultExpireAfterDays": days, "majorVersionLimit": ?r.get("MajorVersionLimit").cloned() })
}

// ---- ensure (spo.addSite / spo.updateSite) -------------------------------------

async fn util_wait(cx: &Cx, admin: &str, op: &Value, ctx: FormDigest) -> Result<()> {
    util::sleep_ms(3000).await;
    let Some(id) = op.get("_ObjectIdentity_") else {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'replace')",
        ));
    };
    let v = cx.verbose();
    spo::wait_until_finished(&json::stringify(id), admin, Some(ctx), v, v).await
}

async fn util_site_exists(cx: &Cx, url: &str) -> Result<bool> {
    let v = cx.verbose();
    let admin = spo::get_spo_admin_url(v).await?;
    let d = spo::ensure_form_digest(&admin, None, v).await?;
    cx.verbose_log(&format!("Checking if the site {url} exists..."));
    let body = sp::request(
        r#"<ObjectPath Id="197" ObjectPathId="196" /><ObjectPath Id="199" ObjectPathId="198" /><Query Id="200" ObjectPathId="198"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        &format!(
            r#"<Constructor Id="196" TypeId="{TENANT}" /><Method Id="198" ParentId="196" Name="GetSitePropertiesByUrl"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Boolean">false</Parameter></Parameters></Method>"#,
            util::escape_xml(url)
        ),
    );
    let json = sp::post(&admin, &d.value, body).await?;
    if let Some((m, ty)) = error_type(&json) {
        if ty != "Microsoft.Online.SharePoint.Common.SpoNoSiteException" {
            return Err(Error::Thrown(m));
        }
        cx.verbose_log(&format!(
            "Site doesn't exist. Checking if the site {url} exists in the recycle bin..."
        ));
        let admin = spo::get_spo_admin_url(v).await?;
        let d = spo::ensure_form_digest(&admin, None, v).await?;
        let body = sp::request(
            r#"<ObjectPath Id="181" ObjectPathId="180" /><Query Id="182" ObjectPathId="180"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<Method Id="180" ParentId="175" Name="GetDeletedSitePropertiesByUrl"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="175" TypeId="{TENANT}" />"#,
                util::escape_xml(url)
            ),
        );
        let json = sp::post(&admin, &d.value, body).await?;
        if let Some((m, ty)) = error_type(&json) {
            if ty == "Microsoft.SharePoint.Client.UnknownError" {
                return Ok(false);
            }
            return Err(Error::Thrown(m));
        }
        return Ok(csom_last(&json).get("Status").and_then(Value::as_str) == Some("Recycled"));
    }
    if json.is_empty() {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'ErrorInfo')",
        ));
    }
    Ok(csom_last(&json).get("Status").and_then(Value::as_str) == Some("Recycled"))
}

async fn add_site(cx: &Cx, url: Option<Value>) -> Result<Value> {
    let v = cx.verbose();
    let ty = cx.str("type");
    let wait = cx.bool("wait");
    if ty.as_deref() == Some("ClassicSite") {
        let admin = spo::get_spo_admin_url(v).await?;
        let ctx = spo::ensure_form_digest(&admin, None, v).await?;
        let u_ = url
            .as_ref()
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        let exists = if cx.bool("removeDeletedSite") {
            util_site_exists(cx, &u_).await?
        } else {
            false
        };
        if exists {
            cx.verbose_log("Site exists in the recycle bin");
            let admin2 = spo::get_spo_admin_url(v).await?;
            let d = spo::ensure_form_digest(&admin2, None, v).await?;
            cx.verbose_log(&format!("Deleting site {u_} from the recycle bin..."));
            let body = sp::request(
                r#"<ObjectPath Id="185" ObjectPathId="184" /><Query Id="186" ObjectPathId="184"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
                &format!(
                    r#"<Method Id="184" ParentId="175" Name="RemoveDeletedSite"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="175" TypeId="{TENANT}" />"#,
                    util::escape_xml(&u_)
                ),
            );
            let json = sp::post(&admin2, &d.value, body).await?;
            sp::check_first(&json)?;
            let op = csom_last(&json);
            if wait && !op.get("IsComplete").is_some_and(truthy) {
                util_wait(cx, &admin2, &op, d).await?;
            }
        } else {
            cx.verbose_log("Site not found");
        }
        let ctx = spo::ensure_form_digest(&admin, Some(&ctx), v).await?;
        cx.verbose_log(&format!("Creating site collection {u_}..."));
        let n = |k: &str, def: &str| {
            cx.val(k)
                .filter(|v| v.is_number())
                .map(to_js_string)
                .unwrap_or_else(|| def.into())
        };
        let body = sp::request(
            r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectPath Id="6" ObjectPathId="5" /><Query Id="7" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query></Query><Query Id="8" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(
                r#"<Constructor Id="3" TypeId="{TENANT}" /><Method Id="5" ParentId="3" Name="CreateSite"><Parameters><Parameter TypeId="{{11f84fff-b8cf-47b6-8b50-34e692656606}}"><Property Name="CompatibilityLevel" Type="Int32">0</Property><Property Name="Lcid" Type="UInt32">{}</Property><Property Name="Owner" Type="String">{}</Property><Property Name="StorageMaximumLevel" Type="Int64">{}</Property><Property Name="StorageWarningLevel" Type="Int64">{}</Property><Property Name="Template" Type="String">{}</Property><Property Name="TimeZoneId" Type="Int32">{}</Property><Property Name="Title" Type="String">{}</Property><Property Name="Url" Type="String">{}</Property><Property Name="UserCodeMaximumLevel" Type="Double">{}</Property><Property Name="UserCodeWarningLevel" Type="Double">{}</Property></Parameter></Parameters></Method>"#,
                n("lcid", "1033"),
                util::escape_xml_value(cx.val("owners")),
                n("storageQuota", "100"),
                n("storageQuotaWarningLevel", "100"),
                util::escape_xml(&ne(cx, "webTemplate").unwrap_or_else(|| "STS#0".into())),
                opt(cx, "timeZone"),
                util::escape_xml_value(cx.val("title")),
                util::escape_xml_value(url.as_ref()),
                n("resourceQuota", "0"),
                n("resourceQuotaWarningLevel", "0"),
            ),
        );
        let json = sp::post(&admin, &ctx.value, body).await?;
        sp::check_first(&json)?;
        let op = csom_last(&json);
        if wait && !op.get("IsComplete").is_some_and(truthy) {
            util_wait(cx, &admin, &op, ctx).await?;
        }
        return Ok(Value::Null);
    }
    let team = ty.as_deref() != Some("CommunicationSite");
    let spo_url = spo::get_spo_url(v).await?;
    cx.verbose_log("Creating new site...");
    let req = if team {
        let mut results: Vec<Value> = Vec::new();
        if cx.val("lcid").is_some_and(truthy) {
            results.push(Value::from(format!("SPSiteLanguage:{}", opt(cx, "lcid"))));
        }
        let mut params = obj! {
            "Description": ne(cx, "description").unwrap_or_default(),
            "CreationOptions": obj! { "results": results, "Classification": ne(cx, "classification").unwrap_or_default() },
        };
        if let Some(o) = ne(cx, "owners") {
            params["Owners"] =
                obj! { "results": o.split(',').map(|x| x.trim().to_string()).collect::<Vec<_>>() };
        }
        Request::post(format!("{spo_url}/_api/GroupSiteManager/CreateGroupEx"))
            .header("content-type", "application/json; odata=verbose; charset=utf-8")
            .spo_json()
            .json(obj! { "displayName": ?cx.val("title").cloned(), "alias": ?cx.val("alias").cloned(), "isPublic": ?cx.val("isPublic").cloned(), "optionalParams": params })
    } else {
        let design = match ne(cx, "siteDesignId") {
            Some(d) => Value::from(d),
            None => match ne(cx, "siteDesign").as_deref() {
                Some("Showcase") => Value::from("6142d2a0-63a5-4ba0-aede-d9fefca2c767"),
                Some("Blank") => Value::from("f6cc5403-0d63-442e-96c0-285923709ffc"),
                Some("Topic") | None => Value::from(EMPTY_GUID),
                Some(_) => cx.val("siteDesignId").cloned().unwrap_or(Value::Null),
            },
        };
        let mut r = obj! {
            "Title": ?cx.val("title").cloned(), "Url": ?url.clone(), "ShareByEmailEnabled": ?cx.val("shareByEmailEnabled").cloned(),
            "Description": ne(cx, "description").unwrap_or_default(), "Classification": ne(cx, "classification").unwrap_or_default(),
            "WebTemplate": "SITEPAGEPUBLISHING#0", "SiteDesignId": ?Some(design).filter(|d| !d.is_null()),
        };
        if let Some(l) = cx.val("lcid").filter(|v| truthy(v)) {
            r["Lcid"] = l.clone();
        }
        if let Some(o) = cx.val("owners").filter(|v| truthy(v)) {
            r["Owner"] = o.clone();
        }
        Request::post(format!("{spo_url}/_api/SPSiteManager/Create"))
            .header("content-type", "application/json;odata=nometadata")
            .spo_json()
            .json(obj! { "request": r })
    };
    let res = req.data().await?;
    if team {
        match res.get("ErrorMessage") {
            Some(Value::Null) => Ok(res.get("SiteUrl").cloned().unwrap_or(Value::Null)),
            None => Err(Error::js(
                "Cannot read properties of undefined (reading 'error')",
            )),
            Some(m) => Err(Error::Thrown(m.clone())),
        }
    } else if res.get("SiteStatus").and_then(Value::as_f64) == Some(2.0) {
        Ok(res.get("SiteUrl").cloned().unwrap_or(Value::Null))
    } else {
        Err(Error::msg("An error has occurred while creating the site"))
    }
}

async fn update_site(cx: &Cx, url: &str) -> Result<()> {
    let v = cx.verbose();
    let tenant = spo::get_tenant_id(v).await?;
    let admin = spo::get_spo_admin_url(v).await?;
    let ctx = spo::ensure_form_digest(&admin, None, v).await?;
    cx.verbose_log("Loading site IDs...");
    let info = Request::get(format!("{url}/_api/site?$select=GroupId,Id"))
        .spo_json()
        .data()
        .await?;
    let (group_id, site_id) = (info.get("GroupId").cloned(), info.get("Id").cloned());
    let gid = group_id
        .as_ref()
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let group_connected = group_id.as_ref().and_then(Value::as_str) != Some(EMPTY_GUID);
    cx.verbose_log(&format!(
        "Retrieved site IDs. siteId: {}, groupId: {gid}",
        site_id
            .as_ref()
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    ));
    let (title, is_public, owners) = (
        cx.val("title").cloned(),
        cx.val("isPublic").cloned(),
        cx.s("owners"),
    );
    if group_connected {
        cx.verbose_log(&format!("Site attached to group {gid}"));
        if title.is_some() && is_public.is_some() && owners.is_some() {
            Request::post(format!("{admin}/_api/SPOGroup/UpdateGroupPropertiesBySiteId"))
                .spo_json()
                .header("content-type", "application/json;charset=utf-8")
                .header("X-RequestDigest", ctx.value.clone())
                .json(obj! { "groupId": ?group_id.clone(), "siteId": ?site_id.clone(), "displayName": ?title.clone() })
                .send()
                .await?;
            m365_core::ut::entra::group::set(
                &gid,
                Some(is_public == Some(Value::Bool(false))),
                None,
                None,
                v,
            )
            .await?;
            let list: Vec<String> = owners
                .unwrap_or_default()
                .split(',')
                .map(|o| o.trim().to_string())
                .collect();
            cx.verbose_log("Retrieving user information to set group owners...");
            let filter = list
                .iter()
                .map(|o| format!("userPrincipalName eq '{o}'"))
                .collect::<Vec<_>>()
                .join(" or ");
            let r = Request::get(format!("{GRAPH}/users?$filter={filter}&$select=id"))
                .header("content-type", "application/json;odata.metadata=none")
                .json_response()
                .data()
                .await?;
            for user in need_arr(&r, "value", "length")? {
                Request::post(format!("{admin}/_api/SP.Directory.DirectorySession/Group('{gid}')/Owners/Add(objectId='{}', principalName='')", u(user, "id"))).header("content-type", "application/json;odata=verbose").send().await?;
            }
        }
    } else {
        cx.verbose_log("Site is not group connected");
        if is_public.is_some() {
            return Err(Error::msg(
                "The isPublic option can't be set on a site that is not groupified",
            ));
        }
        if let Some(o) = owners.filter(|o| !o.is_empty()) {
            for p in o.split(',') {
                let body = sp::request(
                    r#"<ObjectPath Id="48" ObjectPathId="47" />"#,
                    &format!(
                        r#"<Method Id="47" ParentId="34" Name="SetSiteAdmin"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter><Parameter Type="Boolean">true</Parameter></Parameters></Method><Constructor Id="34" TypeId="{TENANT}" />"#,
                        util::escape_xml(url),
                        util::escape_xml(p.trim())
                    ),
                );
                sp::check_first(&sp::post(&admin, &ctx.value, body).await?)?;
            }
        }
    }
    let ctx = spo::ensure_form_digest(&admin, Some(&ctx), v).await?;
    cx.verbose_log(&format!("Updating site {url} properties..."));
    let updated = if !group_connected {
        title.is_some()
    } else {
        [
            "classification",
            "disableFlows",
            "shareByEmailEnabled",
            "sharingCapability",
        ]
        .iter()
        .any(|k| cx.has(k))
    };
    if updated {
        let mut id = 27;
        let mut payload = Vec::new();
        let mut push = |name: &str, ty: &str, val: String, inc: bool| {
            payload.push(format!(r#"<SetProperty Id="{id}" ObjectPathId="5" Name="{name}"><Parameter Type="{ty}">{val}</Parameter></SetProperty>"#));
            if inc {
                id += 1;
            }
        };
        if !group_connected && let Some(t_) = title.as_ref().filter(|t| truthy(t)) {
            push("Title", "String", util::escape_xml(&to_js_string(t_)), true);
        }
        if let Some(Value::String(c)) = cx.val("classification") {
            push("Classification", "String", util::escape_xml(c), true);
        }
        if let Some(d) = cx.val("disableFlows") {
            push("DisableFlows", "Boolean", to_js_string(d), true);
        }
        if let Some(s) = cx.val("shareByEmailEnabled") {
            push("ShareByEmailEnabled", "Boolean", to_js_string(s), true);
        }
        if let Some(s) = ne(cx, "sharingCapability") {
            push(
                "SharingCapability",
                "Enum",
                SHARING
                    .iter()
                    .position(|x| *x == s)
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| "undefined".into()),
                false,
            );
        }
        let pos = tenant.find('|').map(|p| p + 1).unwrap_or(0);
        let end = tenant.find('&').unwrap_or(0);
        let mid = if end > pos { &tenant[pos..end] } else { "" };
        let body = sp::request(
            &format!(
                r#"{}<ObjectPath Id="14" ObjectPathId="13" /><ObjectIdentityQuery Id="15" ObjectPathId="5" /><Query Id="16" ObjectPathId="13"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
                payload.join("")
            ),
            &format!(
                r#"<Identity Id="5" Name="53d8499e-d0d2-5000-cb83-9ade5be42ca4|{mid}&#xA;SiteProperties&#xA;{}" /><Method Id="13" ParentId="5" Name="Update" />"#,
                eqp(url)
            ),
        );
        let json = sp::post(&admin, &ctx.value, body).await?;
        sp::check_first(&json)?;
        let op = csom_last(&json);
        if !op.get("IsComplete").is_some_and(truthy) {
            util_wait(cx, &admin, &op, ctx).await?;
        }
    }
    if let Some(d) = ne(cx, "siteDesignId") {
        cx.verbose_log(&format!("Applying site design {d}..."));
        let spo_url = spo::get_spo_url(v).await?;
        Request::post(format!("{spo_url}/_api/Microsoft.Sharepoint.Utilities.WebTemplateExtensions.SiteScriptUtility.ApplySiteDesign"))
            .header("content-type", "application/json;charset=utf-8")
            .spo_json()
            .json(obj! { "siteDesignId": d, "webUrl": url })
            .send()
            .await?;
    }
    Ok(())
}

pub async fn ensure(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    cx.verbose_log(&format!("Checking if site {url} exists..."));
    let web = sp::get_web(cx, &url).await;
    let res = match web {
        Err(e) => {
            if cx.debug() {
                m365_core::log::log_stderr_value(&e.command_message());
            }
            let not_found = e
                .http()
                .and_then(|h| h.error.as_ref())
                .and_then(Value::as_str)
                == Some("404 FILE NOT FOUND");
            if !not_found {
                return Err(e);
            }
            cx.verbose_log(&format!("No site found at {url}"));
            cx.verbose_log("Creating site...");
            let site_url = match cx.str("type").as_deref() {
                None | Some("TeamSite") => None,
                _ => cx.val("url").cloned(),
            };
            add_site(cx, site_url).await?
        }
        Ok(w) => {
            if cx.debug() {
                m365_core::log::log_stderr_value(&w);
            }
            cx.verbose_log(&format!(
                "Site found at {url}. Checking if site matches conditions..."
            ));
            if let Some(ty) = ne(cx, "type") {
                let expected = match ty.as_str() {
                    "TeamSite" => "GROUP#0".to_string(),
                    "CommunicationSite" => "SITEPAGEPUBLISHING#0".to_string(),
                    "ClassicSite" => ne(cx, "webTemplate").unwrap_or_else(|| "STS#0".into()),
                    _ => {
                        return Err(Error::msg(format!(
                            "{ty} is not a valid site type. Allowed types are TeamSite,CommunicationSite,ClassicSite"
                        )));
                    }
                };
                let current = format!(
                    "{}#{}",
                    w.get("WebTemplate")
                        .map(to_js_string)
                        .unwrap_or_else(|| "undefined".into()),
                    w.get("Configuration")
                        .map(to_js_string)
                        .unwrap_or_else(|| "undefined".into())
                );
                if expected != current {
                    return Err(Error::msg(format!(
                        "Expected web template {expected} but site found at {url} is based on {current}"
                    )));
                }
            }
            cx.verbose_log("Site matches conditions. Updating...");
            cx.verbose_log("Updating site...");
            update_site(cx, &url).await?;
            Value::Null
        }
    };
    if res.is_null() {
        cx.log_undefined()?;
    } else {
        cx.log(res)?;
    }
    cx.verbose_log(&m365_core::term::green("DONE"));
    Ok(())
}

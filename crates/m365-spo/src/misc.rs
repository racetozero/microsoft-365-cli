//! `spo propertybag *`, `spo storageentity *`, `spo cdn *` and `spo theme *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::spo;

// ---- propertybag ---------------------------------------------------------------

/// `formatProperty`: strips the `$  Int32` suffix, parses CSOM dates and booleans.
fn format_property(key: &str, value: &Value) -> (String, Value) {
    if key.contains("$  Int32") {
        let v = json::parse_int(&to_js_string(value))
            .map(Value::from)
            .unwrap_or(Value::Null);
        return (key.replacen("$  Int32", "", 1), v);
    }
    if let Value::String(s) = value {
        if s.contains("/Date(") {
            let parts: Vec<f64> = s
                .replacen("/Date(", "", 1)
                .replacen(")/", "", 1)
                .split(',')
                .map(json::js_number)
                .collect();
            let g = |i: usize| parts.get(i).copied().unwrap_or(f64::NAN);
            let d = (|| {
                use chrono::TimeZone;
                let nd = chrono::NaiveDate::from_ymd_opt(g(0) as i32, 1, 1)?
                    .checked_add_months(chrono::Months::new(g(1) as u32))?
                    .checked_add_days(chrono::Days::new((g(2) as u64).saturating_sub(1)))?;
                let t = nd.and_hms_milli_opt(g(3) as u32, g(4) as u32, g(5) as u32, g(6) as u32)?;
                chrono::Local
                    .from_local_datetime(&t)
                    .earliest()
                    .map(|d| d.with_timezone(&chrono::Utc))
            })();
            return (
                key.to_string(),
                d.map(|d| Value::from(m365_core::time::to_iso(d)))
                    .unwrap_or(Value::Null),
            );
        }
        if s == "true" || s == "false" {
            return (key.to_string(), Value::Bool(s == "true"));
        }
    }
    (key.to_string(), value.clone())
}

async fn property_bag(
    cx: &Cx,
    web: &str,
    digest: &str,
    identity: &sp::Identity,
    folder: Option<String>,
) -> Result<Value> {
    let (body, key) = match folder {
        Some(f) => {
            let srv = if identity.server_relative_url != "/" {
                format!("{}{f}", identity.server_relative_url)
            } else {
                f
            };
            (
                sp::request(
                    r#"<ObjectPath Id="10" ObjectPathId="9" /><ObjectIdentityQuery Id="11" ObjectPathId="9" /><Query Id="12" ObjectPathId="9"><Query SelectAllProperties="false"><Properties><Property Name="Properties" SelectAll="true"><Query SelectAllProperties="false"><Properties /></Query></Property></Properties></Query></Query>"#,
                    &format!(
                        r#"<Method Id="9" ParentId="5" Name="GetFolderByServerRelativeUrl"><Parameters><Parameter Type="String">{srv}</Parameter></Parameters></Method><Identity Id="5" Name="{}" />"#,
                        identity.object_identity
                    ),
                ),
                "Properties",
            )
        }
        None => (
            sp::request(
                r#"<Query Id="97" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="ServerRelativeUrl" ScalarProperty="true" /><Property Name="AllProperties" SelectAll="true"><Query SelectAllProperties="false"><Properties /></Query></Property></Properties></Query></Query>"#,
                &format!(r#"<Identity Id="5" Name="{}" />"#, identity.object_identity),
            ),
            "AllProperties",
        ),
    };
    let text = spo::process_query(web, digest, body).await?;
    cx.verbose_log(&format!("Attempt to get {key} key values"));
    let json = sp::parse(&text)?;
    sp::check(&json)?;
    match sp::find(&json, key) {
        Some(o) => Ok(o[key].clone()),
        None => Err(Error::msg(format!("Cannot proceed. {key} not found"))),
    }
}

async fn bag(cx: &Cx) -> Result<Value> {
    let web = cx.u("webUrl");
    let d = sp::digest(&web).await?;
    let id = sp::web_identity(&web, &d).await?;
    property_bag(cx, &web, &d, &id, cx.s("folder")).await
}

pub async fn propertybag_get(cx: &mut Cx) -> Result<()> {
    let b = bag(cx).await?;
    let key = cx.u("key");
    let found = b.as_object().and_then(|o| {
        o.iter()
            .map(|(k, v)| format_property(k, v))
            .find(|(k, _)| *k == key)
    });
    match found {
        Some((_, v)) => cx.log(v),
        None => {
            cx.verbose_log("Property not found.");
            Ok(())
        }
    }
}

pub async fn propertybag_list(cx: &mut Cx) -> Result<()> {
    let b = bag(cx).await?;
    let out: Vec<Value> = b
        .as_object()
        .map(|o| {
            o.iter()
                .filter(|(k, _)| *k != "_ObjectType_")
                .map(|(k, v)| format_property(k, v))
                .map(|(k, v)| obj! { "key": k, "value": v })
                .collect()
        })
        .unwrap_or_default();
    cx.log(Value::Array(out))
}

pub async fn propertybag_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the {} property?",
            cx.u("key")
        ),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    let d = sp::digest(&web).await?;
    let mut id = sp::web_identity(&web, &d).await?;
    let folder = cx.s("folder");
    if let Some(f) = &folder {
        id = sp::folder_identity(&id.object_identity, &web, f, &d).await?;
    }
    let object_type = if folder.is_some() {
        "Properties"
    } else {
        "AllProperties"
    };
    let body = sp::request(
        &format!(
            r#"<Method Name="SetFieldValue" Id="206" ObjectPathId="205"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Null" /></Parameters></Method><Method Name="Update" Id="207" ObjectPathId="198" />"#,
            util::escape_xml(&cx.u("key"))
        ),
        &format!(
            r#"<Property Id="205" ParentId="198" Name="{object_type}" /><Identity Id="198" Name="{}" />"#,
            id.object_identity
        ),
    );
    let json = sp::post(&web, &d, body).await?;
    sp::check(&json)
}

pub async fn propertybag_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let r: Result<()> = async {
        let d = sp::digest(&web).await?;
        let mut id = sp::web_identity(&web, &d).await?;
        let folder = cx.s("folder");
        if let Some(f) = &folder {
            id = sp::folder_identity(&id.object_identity, &web, f, &d).await?;
        }
        sp::set_property(
            &cx.u("key"),
            &cx.u("value"),
            &web,
            &d,
            &id,
            folder.as_deref(),
        )
        .await?;
        Ok(())
    }
    .await;
    if let Err(e) = &r
        && e.js_message().contains("E_ACCESSDENIED")
    {
        cx.stderr("Tip: If a site has NoScript enabled, setting the property bag value may result in exceptions.");
    }
    r
}

// ---- storageentity -------------------------------------------------------------

async fn app_catalog(cx: &Cx) -> Result<String> {
    if let Some(u) = cx.s("appCatalogUrl") {
        return Ok(u);
    }
    let url = spo::get_spo_url(cx.debug()).await?;
    let r = Request::get(format!("{url}/_api/SP_TenantSettings_Current"))
        .spo_json()
        .data()
        .await?;
    match r.get("CorporateCatalogUrl").filter(|v| truthy(v)) {
        Some(u) => Ok(to_js_string(u)),
        None => Err(Error::msg(
            "Tenant app catalog URL not found. Specify the URL of the app catalog site using the appCatalogUrl option.",
        )),
    }
}

pub async fn storageentity_get(cx: &mut Cx) -> Result<()> {
    let cat = app_catalog(cx).await?;
    let key = cx.u("key");
    let p = Request::get(format!("{cat}/_api/web/GetStorageEntity('{}')", eqp(&key)))
        .spo_json()
        .data()
        .await?;
    if p.get("odata.null") == Some(&Value::Bool(true)) {
        cx.verbose_log(&format!("Property with key {key} not found"));
        return Ok(());
    }
    cx.log(obj! { "Key": key, "Value": ?p.get("Value").cloned(), "Description": ?p.get("Description").cloned(), "Comment": ?p.get("Comment").cloned() })
}

pub async fn storageentity_list(cx: &mut Cx) -> Result<()> {
    let cat = app_catalog(cx).await?;
    cx.verbose_log(&format!(
        "Retrieving details for all tenant properties in {cat}..."
    ));
    let web = Request::get(format!(
        "{cat}/_api/web/AllProperties?$select=storageentitiesindex"
    ))
    .spo_json()
    .data()
    .await?;
    let idx = web
        .get("storageentitiesindex")
        .filter(|v| truthy(v))
        .map(to_js_string)
        .filter(|s| !s.trim().is_empty());
    let Some(idx) = idx else {
        cx.verbose_log("No tenant properties found");
        return cx.log(Value::Array(Vec::new()));
    };
    let props = json::jparse(&idx)?;
    let o = props.as_object().cloned().unwrap_or_default();
    if o.is_empty() {
        cx.verbose_log("No tenant properties found");
        return cx.log(Value::Array(Vec::new()));
    }
    let out: Vec<Value> = o
        .iter()
        .map(|(k, v)| obj! { "Key": k.as_str(), "Value": ?v.get("Value").cloned(), "Description": ?v.get("Description").cloned(), "Comment": ?v.get("Comment").cloned() })
        .collect();
    cx.log(Value::Array(out))
}

async fn admin_csom(cx: &Cx, actions: &str, paths: &str) -> Result<Vec<Value>> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    sp::post(&admin, &d, sp::request(actions, paths)).await
}

pub async fn storageentity_remove(cx: &mut Cx) -> Result<()> {
    let key = cx.u("key");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to delete the {key} tenant property?"),
    )? {
        return Ok(());
    }
    let cat = app_catalog(cx).await?;
    cx.verbose_log(&format!("Removing tenant property {key} from {cat}..."));
    let json = admin_csom(
        cx,
        &format!(r#"<ObjectPath Id="31" ObjectPathId="30" /><ObjectPath Id="33" ObjectPathId="32" /><ObjectPath Id="35" ObjectPathId="34" /><Method Name="RemoveStorageEntity" Id="36" ObjectPathId="34"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#, util::escape_xml(&key)),
        &format!(
            r#"<Constructor Id="30" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /><Method Id="32" ParentId="30" Name="GetSiteByUrl"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Property Id="34" ParentId="32" Name="RootWeb" />"#,
            util::escape_xml(&cat)
        ),
    )
    .await?;
    sp::check_first(&json)
}

pub async fn storageentity_set(cx: &mut Cx) -> Result<()> {
    let cat = app_catalog(cx).await?;
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let key = cx.u("key");
    cx.verbose_log(&format!("Setting tenant property {key} in {cat}..."));
    let e = |k: &str| util::escape_xml(&cx.s(k).unwrap_or_default());
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="24" ObjectPathId="23" /><ObjectPath Id="26" ObjectPathId="25" /><ObjectPath Id="28" ObjectPathId="27" /><Method Name="SetStorageEntity" Id="29" ObjectPathId="27"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            util::escape_xml(&key),
            util::escape_xml(&cx.u("value")),
            e("description"),
            e("comment")
        ),
        &format!(
            r#"<Constructor Id="23" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /><Method Id="25" ParentId="23" Name="GetSiteByUrl"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Property Id="27" ParentId="25" Name="RootWeb" />"#,
            util::escape_xml(&cat)
        ),
    );
    let json = sp::post(&admin, &d, body).await?;
    let first = json
        .first()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'ErrorInfo')"))?;
    if let Some(ei) = first.get("ErrorInfo").filter(|e| truthy(e)) {
        let m = u(ei, "ErrorMessage");
        if cx.verbose() && m.contains("Access denied.") {
            cx.stderr("");
            cx.stderr("This error is often caused by invalid URL of the app catalog site. Verify, that the URL you specified as an argument of the spo storageentity set command is a valid app catalog URL and try again.");
            cx.stderr("");
        }
        return Err(Error::Thrown(
            ei.get("ErrorMessage").cloned().unwrap_or(Value::Null),
        ));
    }
    Ok(())
}

// ---- cdn -----------------------------------------------------------------------

fn cdn_type(cx: &Cx, key: &str) -> (i32, &'static str) {
    if cx.str(key).as_deref() == Some("Private") {
        (1, "Private")
    } else {
        (0, "Public")
    }
}

/// Tenant identity, admin URL and digest, as every CDN command fetches them.
async fn tenant_ctx(cx: &Cx) -> Result<(String, String, String)> {
    let tenant = spo::get_tenant_id(cx.debug()).await?;
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    Ok((tenant, admin, d))
}

pub async fn cdn_get(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "type");
    let (tenant, admin, d) = tenant_ctx(cx).await?;
    cx.verbose_log(&format!("Retrieving status of {name} CDN..."));
    let json = sp::post(
        &admin,
        &d,
        sp::request(&format!(r#"<Method Name="GetTenantCdnEnabled" Id="12" ObjectPathId="8"><Parameters><Parameter Type="Enum">{t}</Parameter></Parameters></Method>"#), &format!(r#"<Identity Id="8" Name="{tenant}" />"#)),
    )
    .await?;
    sp::check_first(&json)?;
    let r = json.last().cloned().unwrap_or(Value::Null);
    cx.verbose_log(&format!(
        "{name} CDN at {admin} is {}",
        if r == Value::Bool(true) {
            "enabled"
        } else {
            "disabled"
        }
    ));
    cx.log(r)
}

async fn cdn_call(
    cx: &Cx,
    method: &str,
    id: u32,
    path_id: u32,
    params: &str,
    log: String,
) -> Result<Vec<Value>> {
    let (tenant, admin, d) = tenant_ctx(cx).await?;
    cx.verbose_log(&log);
    let json = sp::post(
        &admin,
        &d,
        sp::request(&format!(r#"<Method Name="{method}" Id="{id}" ObjectPathId="{path_id}"><Parameters>{params}</Parameters></Method>"#), &format!(r#"<Identity Id="{path_id}" Name="{tenant}" />"#)),
    )
    .await?;
    sp::check_first(&json)?;
    Ok(json)
}

pub async fn cdn_origin_add(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "type");
    let o = cx.u("origin");
    cdn_call(
        cx,
        "AddTenantCdnOrigin",
        27,
        23,
        &format!(
            r#"<Parameter Type="Enum">{t}</Parameter><Parameter Type="String">{}</Parameter>"#,
            util::escape_xml(&o)
        ),
        format!("Adding origin {o} to the {name} CDN. Please wait, this might take a moment..."),
    )
    .await?;
    Ok(())
}

pub async fn cdn_origin_list(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "type");
    let json = cdn_call(
        cx,
        "GetTenantCdnOrigins",
        22,
        18,
        &format!(r#"<Parameter Type="Enum">{t}</Parameter>"#),
        format!("Retrieving configured origins for {name} CDN..."),
    )
    .await?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn cdn_origin_remove(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "type");
    let o = cx.u("origin");
    if cx.bool("force") {
        cx.debug_log("Confirmation suppressed through the confirm option. Removing CDN origin...");
    } else if !cx.confirm(&format!(
        "Are you sure you want to delete the {o} CDN origin?"
    ))? {
        return Ok(());
    }
    cdn_call(
        cx,
        "RemoveTenantCdnOrigin",
        33,
        29,
        &format!(
            r#"<Parameter Type="Enum">{t}</Parameter><Parameter Type="String">{}</Parameter>"#,
            util::escape_xml(&o)
        ),
        format!(
            "Removing origin {o} from the {name} CDN. Please wait, this might take a moment..."
        ),
    )
    .await?;
    Ok(())
}

pub async fn cdn_policy_list(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "cdnType");
    let json = cdn_call(
        cx,
        "GetTenantCdnPolicies",
        7,
        3,
        &format!(r#"<Parameter Type="Enum">{t}</Parameter>"#),
        format!("Retrieving configured policies for {name} CDN..."),
    )
    .await?;
    let list = json
        .last()
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| Error::js("result.map is not a function"))?;
    cx.verbose_log("Configured policies:");
    let out: Vec<Value> = list
        .iter()
        .map(|o| {
            let s = to_js_string(o);
            let kv: Vec<&str> = s.split(';').collect();
            obj! { "Policy": ?kv.first().copied(), "Value": ?kv.get(1).copied() }
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn cdn_policy_set(cx: &mut Cx) -> Result<()> {
    let (t, name) = cdn_type(cx, "cdnType");
    let policy = match cx.str("policy").as_deref() {
        Some("IncludeFileExtensions") => 0,
        Some("ExcludeRestrictedSiteClassifications") => 1,
        _ => -1,
    };
    cdn_call(
        cx,
        "SetTenantCdnPolicy",
        12,
        8,
        &format!(r#"<Parameter Type="Enum">{t}</Parameter><Parameter Type="Enum">{policy}</Parameter><Parameter Type="String">{}</Parameter>"#, util::escape_xml(&cx.u("value"))),
        format!("Configuring policy on the {name} CDN. Please wait, this might take a moment..."),
    )
    .await?;
    Ok(())
}

pub async fn cdn_set(cx: &mut Cx) -> Result<()> {
    let ty = match cx.str("type").as_deref() {
        Some("Private") => 1,
        Some("Both") => 2,
        _ => 0,
    };
    let enabled = cx.bool("enabled");
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let verb = if enabled { "Enabling" } else { "Disabling" };
    let ctor = |id: u32| {
        format!(r#"<Constructor Id="{id}" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" />"#)
    };
    let set = |id: u32, path: u32, t: i32| {
        format!(
            r#"<Method Name="SetTenantCdnEnabled" Id="{id}" ObjectPathId="{path}"><Parameters><Parameter Type="Enum">{t}</Parameter><Parameter Type="Boolean">{enabled}</Parameter></Parameters></Method>"#
        )
    };
    let origins = |id: u32, path: u32, t: i32| {
        format!(
            r#"<Method Name="CreateTenantCdnDefaultOrigins" Id="{id}" ObjectPathId="{path}"><Parameters><Parameter Type="Enum">{t}</Parameter></Parameters></Method>"#
        )
    };
    let body = if ty == 2 {
        if cx.bool("noDefaultOrigins") {
            cx.verbose_log(&format!("{verb} Public and Private CDNs without default origins. Please wait, this might take a moment..."));
            sp::request(
                &format!(
                    r#"<ObjectPath Id="12" ObjectPathId="11" />{}{}"#,
                    set(13, 11, 1),
                    set(14, 11, 0)
                ),
                &ctor(11),
            )
        } else {
            cx.verbose_log(&format!("{verb} Public and Private CDNs with default origins. Please wait, this might take a moment..."));
            let extra = if enabled {
                format!("{}{}", origins(99, 95, 1), origins(100, 95, 0))
            } else {
                String::new()
            };
            sp::request(
                &format!(
                    r#"<ObjectPath Id="96" ObjectPathId="95" />{}{}{extra}"#,
                    set(97, 95, 1),
                    set(98, 95, 0)
                ),
                &ctor(95),
            )
        }
    } else {
        let name = if ty == 1 { "Private" } else { "Public" };
        if cx.bool("noDefaultOrigins") {
            cx.verbose_log(&format!("{verb} {name} CDN without default origins. Please wait, this might take a moment..."));
            sp::request(
                &format!(
                    r#"<ObjectPath Id="19" ObjectPathId="18" />{}"#,
                    set(20, 18, ty)
                ),
                &ctor(18),
            )
        } else {
            cx.verbose_log(&format!(
                "{verb} {name} CDN. Please wait, this might take a moment..."
            ));
            let extra = if enabled {
                origins(21, 18, if ty == 1 { 1 } else { 0 })
            } else {
                String::new()
            };
            sp::request(
                &format!(
                    r#"<ObjectPath Id="19" ObjectPathId="18" />{}{extra}"#,
                    set(20, 18, ty)
                ),
                &ctor(18),
            )
        }
    };
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)
}

// ---- theme ---------------------------------------------------------------------

pub fn theme_apply_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let name = o.get("name").map(to_js_string).unwrap_or_default();
    Ok(!o.get("sharePointTheme").is_some_and(truthy)
        || crate::themes::PALETTES.iter().any(|(n, _)| *n == name))
}

pub async fn theme_apply(cx: &mut Cx) -> Result<()> {
    let sp_theme = cx.bool("sharePointTheme");
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = if sp_theme {
        String::new()
    } else {
        sp::digest(&admin).await?
    };
    let (name, web) = (cx.u("name"), cx.u("webUrl"));
    cx.verbose_log(&format!("Applying theme {name} to the {web} site..."));
    if sp_theme {
        let palette = crate::themes::PALETTES
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, p)| *p)
            .unwrap_or("");
        let body = format!(
            "{{\n      'name': '{name}' ,\n      'themeJson': '{{\"palette\": {{{palette}}}}}'\n    }}"
        );
        let text = Request::post(format!("{web}/_api/ThemeManager/ApplyTheme"))
            .accept("application/json;odata=nometadata")
            .header("Content-Type", "application/json;odata=nometadata")
            .text(body)
            .data()
            .await?;
        let j = match &text {
            Value::String(s) => json::jparse(s)?,
            other => json::jparse(&to_js_string(other))?,
        };
        if let Some(e) = j.get("error").filter(|e| truthy(e)) {
            return Err(Error::Thrown(e.clone()));
        }
        return match j.get("value") {
            Some(v) => cx.log(v.clone()),
            None => cx.log_undefined(),
        };
    }
    let json = sp::post(
        &admin,
        &d,
        sp::request(
            &format!(
                r#"<ObjectPath Id="10" ObjectPathId="9" /><Method Name="SetWebTheme" Id="11" ObjectPathId="9"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                util::escape_xml(&name),
                util::escape_xml(&web)
            ),
            r#"<Constructor Id="9" TypeId="{268004ae-ef6b-4e9b-8425-127220d84719}" />"#,
        ),
    )
    .await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn theme_get(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let name = cx.u("name");
    cx.verbose_log(&format!("Getting {name} theme from tenant..."));
    let json = sp::post(
        &admin,
        &d,
        sp::request(
            r#"<ObjectPath Id="12" ObjectPathId="11" /><ObjectPath Id="14" ObjectPathId="13" /><Query Id="15" ObjectPathId="13"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<Constructor Id="11" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /><Method Id="13" ParentId="11" Name="GetTenantTheme"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                util::escape_xml(&name)
            ),
        ),
    )
    .await?;
    sp::check(&json)?;
    let mut theme = json
        .get(6)
        .cloned()
        .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?;
    if let Some(o) = theme.as_object_mut() {
        o.shift_remove("_ObjectType_");
    }
    cx.log(theme)
}

pub async fn theme_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Retrieving themes from tenant store...");
    let r = Request::post(format!("{admin}/_api/thememanager/GetTenantThemingOptions"))
        .spo_json()
        .data()
        .await?;
    cx.log(
        r.get("themePreviews")
            .filter(|v| truthy(v))
            .cloned()
            .unwrap_or(Value::Array(Vec::new())),
    )
}

pub async fn theme_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove the theme")? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Removing theme from tenant...");
    Request::post(format!("{admin}/_api/thememanager/DeleteTenantTheme"))
        .spo_json()
        .json(obj! { "name": ?cx.val("name").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn theme_set(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let palette = json::jparse(&cx.u("theme"))?;
    if cx.debug() {
        cx.stderr("");
        cx.stderr("Palette");
        cx.stderr(&stringify(&palette));
    }
    let name = util::escape_xml(&cx.u("name"));
    let body = format!(
        r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="10" ObjectPathId="9" /><Method Name="UpdateTenantTheme" Id="11" ObjectPathId="9"><Parameters><Parameter Type="String">{name}</Parameter><Parameter Type="String">{{"isInverted":{},"name":"{name}","palette":{}}}</Parameter></Parameters></Method></Actions><ObjectPaths><Constructor Id="9" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}"/></ObjectPaths></Request>"#,
        spo::application_name(),
        cx.bool("isInverted"),
        stringify(&palette)
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check(&json)
}

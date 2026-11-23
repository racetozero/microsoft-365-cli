//! `entra app *`.

use m365_core::prelude::*;
use m365_core::ut::entra_app::{self, AppCreationOptions};

fn opt(cx: &Cx, k: &str) -> Option<String> {
    cx.str(k)
}

pub fn creation_options(cx: &Cx) -> AppCreationOptions {
    AppCreationOptions {
        apis_application: opt(cx, "apisApplication"),
        apis_delegated: opt(cx, "apisDelegated"),
        implicit_flow: cx.bool("implicitFlow"),
        multitenant: cx.bool("multitenant"),
        name: opt(cx, "name"),
        platform: opt(cx, "platform"),
        redirect_uris: opt(cx, "redirectUris"),
        certificate_file: opt(cx, "certificateFile"),
        certificate_base64_encoded: opt(cx, "certificateBase64Encoded"),
        certificate_display_name: opt(cx, "certificateDisplayName"),
        allow_public_client_flows: cx.bool("allowPublicClientFlows"),
        bundle_id: opt(cx, "bundleId"),
        signature_hash: opt(cx, "signatureHash"),
    }
}

pub fn add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(match i {
        4 => !t("certificateDisplayName") || t("certificateFile") || t("certificateBase64Encoded"),
        5 => {
            !t("certificateFile")
                || std::path::Path::new(&to_js_string(&o["certificateFile"])).exists()
        }
        9 => {
            if !t("manifest") {
                return Ok(true);
            }
            match json::parse(&to_js_string(&o["manifest"])) {
                Ok(m) => {
                    if t("name") || m.get("name").is_some_and(truthy) {
                        true
                    } else {
                        return Err("Specify the name of the app to create either through the 'name' option or the 'name' property in the manifest".into());
                    }
                }
                Err(_) => {
                    return Err(format!(
                        "Error while parsing the specified manifest: {}",
                        util::js_json_error(&to_js_string(&o["manifest"]))
                    ));
                }
            }
        }
        10 => o.get("platform").and_then(Value::as_str) != Some("apple") || t("bundleId"),
        11 => {
            o.get("platform").and_then(Value::as_str) != Some("android")
                || (t("bundleId") && t("signatureHash"))
        }
        2 => {
            let p = o.get("platform").and_then(Value::as_str);
            !t("platform")
                || !matches!(p, Some("spa" | "web" | "publicClient"))
                || t("redirectUris")
        }
        _ => true,
    })
}

fn translate_platform(p: &str) -> String {
    if p == "publicClient" {
        return "InstalledClient".into();
    }
    let mut c = p.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn transform_manifest(v2: &Value) -> Value {
    let mut g = v2.clone();
    let o = g.as_object_mut().unwrap();
    o.entry("api").or_insert_with(|| obj! {});
    o.entry("info").or_insert_with(|| obj! {});
    if !o.get("web").is_some_and(truthy) {
        o.insert(
            "web".into(),
            obj! { "implicitGrantSettings": obj!{}, "redirectUris": Vec::<Value>::new() },
        );
    }
    if !o.get("spa").is_some_and(truthy) {
        o.insert("spa".into(), obj! { "redirectUris": Vec::<Value>::new() });
    }
    for p in [
        "accessTokenAcceptedVersion",
        "disabledByMicrosoftStatus",
        "errorUrl",
        "oauth2RequirePostResponse",
        "oauth2AllowUrlPathMatching",
        "orgRestrictions",
        "samlMetadataUrl",
    ] {
        o.shift_remove(p);
    }
    let get = |k: &str| v2.get(k).cloned();
    // `graph.outer.k = v2.x`: an undefined value drops the property
    let set_in = |o: &mut Obj, outer: &str, k: &str, v: Option<Value>| {
        if let Some(Value::Object(inner)) = o.get_mut(outer) {
            match v {
                Some(v) => {
                    inner.insert(k.to_string(), v);
                }
                None => {
                    inner.shift_remove(k);
                }
            }
        }
    };
    set_in(o, "api", "acceptMappedClaims", get("acceptMappedClaims"));
    o.shift_remove("acceptMappedClaims");
    match get("allowPublicClient") {
        Some(v) => {
            o.insert("isFallbackPublicClient".into(), v);
        }
        None => {
            o.shift_remove("isFallbackPublicClient");
        }
    }
    o.shift_remove("allowPublicClient");
    let iu = v2.get("informationalUrls");
    set_in(
        o,
        "info",
        "termsOfServiceUrl",
        iu.and_then(|x| x.get("termsOfService")).cloned(),
    );
    set_in(
        o,
        "info",
        "supportUrl",
        iu.and_then(|x| x.get("support")).cloned(),
    );
    set_in(
        o,
        "info",
        "privacyStatementUrl",
        iu.and_then(|x| x.get("privacy")).cloned(),
    );
    set_in(
        o,
        "info",
        "marketingUrl",
        iu.and_then(|x| x.get("marketing")).cloned(),
    );
    o.shift_remove("informationalUrls");
    set_in(
        o,
        "api",
        "knownClientApplications",
        get("knownClientApplications"),
    );
    o.shift_remove("knownClientApplications");
    set_in(o, "info", "logoUrl", get("logoUrl"));
    o.shift_remove("logoUrl");
    set_in(o, "web", "logoutUrl", get("logoutUrl"));
    o.shift_remove("logoutUrl");
    match get("name") {
        Some(v) => {
            o.insert("displayName".into(), v);
        }
        None => {
            o.shift_remove("displayName");
        }
    }
    o.shift_remove("name");
    if let Some(Value::Object(web)) = o.get_mut("web")
        && let Some(Value::Object(igs)) = web.get_mut("implicitGrantSettings")
    {
        for (src, dst) in [
            ("oauth2AllowImplicitFlow", "enableAccessTokenIssuance"),
            ("oauth2AllowIdTokenImplicitFlow", "enableIdTokenIssuance"),
        ] {
            match v2.get(src) {
                Some(v) => {
                    igs.insert(dst.into(), v.clone());
                }
                None => {
                    igs.shift_remove(dst);
                }
            }
        }
    }
    o.shift_remove("oauth2AllowImplicitFlow");
    o.shift_remove("oauth2AllowIdTokenImplicitFlow");
    set_in(o, "api", "oauth2PermissionScopes", get("oauth2Permissions"));
    o.shift_remove("oauth2Permissions");
    if let Some(Value::Object(api)) = o.get_mut("api")
        && let Some(Value::Array(scopes)) = api.get_mut("oauth2PermissionScopes")
    {
        for sc in scopes.iter_mut() {
            if let Some(so) = sc.as_object_mut() {
                so.shift_remove("lang");
                so.shift_remove("origin");
            }
        }
    }
    o.shift_remove("oauth2RequiredPostResponse");
    o.shift_remove("preAuthorizedApplications");
    if let Some(Value::Array(urls)) = v2.get("replyUrlsWithType") {
        for u in urls {
            let target = match u.get("type").and_then(Value::as_str) {
                Some("Web") => "web",
                Some("Spa") => "spa",
                _ => continue,
            };
            if let Some(Value::Object(t)) = o.get_mut(target)
                && let Some(Value::Array(r)) = t.get_mut("redirectUris")
            {
                r.push(u.get("url").cloned().unwrap_or(Value::Null));
            }
        }
        o.shift_remove("replyUrlsWithType");
    }
    set_in(o, "web", "homePageUrl", get("signInUrl"));
    o.shift_remove("signInUrl");
    if let Some(Value::Array(roles)) = o.get_mut("appRoles") {
        for r in roles.iter_mut() {
            if let Some(ro) = r.as_object_mut() {
                ro.shift_remove("lang");
            }
        }
    }
    g
}

async fn create_secret(
    app_object_id: &str,
    display_name: Option<&str>,
    expiration: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<Value> {
    let exp = expiration.unwrap_or_else(|| m365_core::time::add_months(chrono::Utc::now(), 12));
    let name = display_name.unwrap_or("Default").to_string();
    let res = Request::post(format!("{GRAPH}/v1.0/myorganization/applications/{app_object_id}/addPassword"))
        .header("content-type", "application/json")
        .json(obj! { "passwordCredential": obj! { "displayName": name.as_str(), "endDateTime": m365_core::time::to_iso(exp) } })
        .json_response()
        .data()
        .await?;
    Ok(obj! { "displayName": name, "value": ?res.get("secretText").cloned() })
}

fn save_to_rc(cx: &Cx, app_id: &Value, name: &Value, dedupe: bool) {
    let path = ".m365rc.json";
    cx.verbose_log(&format!(
        "Saving Microsoft Entra app registration information to the {path} file..."
    ));
    let mut rc = Value::Object(Obj::new());
    if std::path::Path::new(path).exists() {
        cx.debug_log(&format!("Reading existing {path}..."));
        match std::fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|t| {
                if t.is_empty() {
                    Ok(obj! {})
                } else {
                    json::parse(&t).map_err(|_| util::js_json_error(&t))
                }
            }) {
            Ok(v) => rc = v,
            Err(e) => {
                cx.stderr(&format!(
                    "Error reading {path}: {e}. Please add app info to {path} manually."
                ));
                return;
            }
        }
    }
    let o = rc.as_object_mut().unwrap();
    let apps = o.entry("apps").or_insert_with(|| Value::Array(vec![]));
    if let Value::Array(a) = apps {
        if dedupe && a.iter().any(|x| x.get("appId") == Some(app_id)) {
            return;
        }
        a.push(obj! { "appId": app_id.clone(), "name": name.clone() });
    }
    if let Err(e) = std::fs::write(path, json::stringify_pretty(&rc, "  ")) {
        cx.stderr(&format!(
            "Error writing {path}: Error: {e}. Please add app info to {path} manually."
        ));
    }
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let manifest = match cx.str("manifest") {
        Some(m) => Some(json::parse(&m)?),
        None => None,
    };
    if !cx.bool("name")
        && let Some(n) = manifest.as_ref().and_then(|m| m.get("name"))
    {
        cx.opts.insert("name".into(), n.clone());
    }
    let app_name = cx.opts.get("name").cloned().unwrap_or(Value::Null);
    let options = creation_options(cx);
    let apis =
        entra_app::resolve_apis(&options, manifest.as_ref(), cx.verbose(), cx.debug()).await?;
    let unknown = ut::options::unknown_options(&cx.opts, cx.meta);
    let mut app =
        entra_app::create_app_registration(&options, &unknown, &apis, cx.verbose(), cx.debug())
            .await?;
    let tenant = auth::jwt::tenant_id(&auth::default_access_token().unwrap_or_default());
    if let Some(o) = app.as_object_mut() {
        o.insert("tenantId".into(), Value::from(tenant));
    }
    let id = s(&app, "id").to_string();
    // updateAppFromManifest
    if let Some(m) = &manifest {
        let mut v2 = m.clone();
        let v2o = v2
            .as_object_mut()
            .ok_or_else(|| Error::msg("manifest must be an object"))?;
        for k in ["id", "appId", "publisherDomain"] {
            v2o.shift_remove(k);
        }
        let mut secrets: Vec<(Value, chrono::DateTime<chrono::Utc>)> = Vec::new();
        if let Some(Value::Array(pcs)) = v2o.get("passwordCredentials").cloned()
            && !pcs.is_empty()
        {
            for c in &pcs {
                let start = c
                    .get("startDate")
                    .and_then(Value::as_str)
                    .and_then(m365_core::time::parse_js_date);
                let end = c
                    .get("endDate")
                    .and_then(Value::as_str)
                    .and_then(m365_core::time::parse_js_date);
                let now = chrono::Utc::now();
                let exp = match (start, end) {
                    (Some(s), Some(e)) => {
                        let base = now.with_timezone(&chrono::Utc);
                        let base = base
                            - chrono::Duration::milliseconds(base.timestamp_subsec_millis() as i64);
                        base + (e - s)
                    }
                    _ => now,
                };
                secrets.push((c.get("displayName").cloned().unwrap_or(Value::Null), exp));
            }
            v2o.shift_remove("passwordCredentials");
        }
        if cx.s("apisApplication").is_some() || cx.s("apisDelegated").is_some() {
            v2o.insert(
                "requiredResourceAccess".into(),
                app.get("requiredResourceAccess")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
        }
        if let Some(r) = cx.s("redirectUris") {
            let t = translate_platform(&cx.u("platform"));
            v2o.insert(
                "replyUrlsWithType".into(),
                Value::Array(
                    r.split(',')
                        .map(|u| obj! { "url": u.trim(), "type": t.as_str() })
                        .collect(),
                ),
            );
        }
        if cx.bool("multitenant") {
            v2o.insert("signInAudience".into(), Value::from("AzureADMultipleOrgs"));
        }
        if cx.bool("implicitFlow") {
            v2o.shift_remove("oauth2AllowIdTokenImplicitFlow");
            v2o.shift_remove("oauth2AllowImplicitFlow");
        }
        if cx.s("scopeName").is_some() {
            v2o.shift_remove("oauth2Permissions");
        }
        if cx.s("certificateFile").is_some() || cx.s("certificateBase64Encoded").is_some() {
            v2o.shift_remove("keyCredentials");
        }
        let graph = transform_manifest(&v2);
        Request::patch(format!("{GRAPH}/v1.0/myorganization/applications/{id}"))
            .header("content-type", "application/json")
            .json(graph)
            .json_response()
            .send()
            .await?;
        if let Some(Value::Array(pre)) = v2.get("preAuthorizedApplications")
            && !pre.is_empty()
        {
            let pre: Vec<Value> = pre
                .iter()
                .map(|p| {
                    let mut p = p.clone();
                    if let Some(po) = p.as_object_mut() {
                        let ids = po.get("permissionIds").cloned();
                        match ids {
                            Some(v) => po.insert("delegatedPermissionIds".into(), v),
                            None => None,
                        };
                        po.shift_remove("permissionIds");
                    }
                    p
                })
                .collect();
            Request::patch(format!("{GRAPH}/v1.0/myorganization/applications/{id}"))
                .header("content-type", "application/json")
                .json(obj! { "api": obj! { "preAuthorizedApplications": pre } })
                .json_response()
                .send()
                .await?;
        }
        if !secrets.is_empty() {
            let mut out = Vec::new();
            for (name, exp) in &secrets {
                out.push(create_secret(&id, Some(&to_js_string(name)), Some(*exp)).await?);
            }
            app.as_object_mut()
                .unwrap()
                .insert("secrets".into(), Value::Array(out));
        }
    }
    entra_app::grant_admin_consent(&app, cx.bool("grantAdminConsent"), cx.debug()).await?;
    // configureUri
    if let Some(uri) = cx.s("uri") {
        cx.verbose_log("Configuring Microsoft Entra application ID URI...");
        let mut info =
            obj! { "identifierUris": vec![Value::from(uri.replace("_appId_", s(&app, "appId")))] };
        if let Some(scope) = cx.s("scopeName") {
            info.as_object_mut().unwrap().insert(
                "api".into(),
                obj! { "oauth2PermissionScopes": vec![obj! {
                    "adminConsentDescription": ?cx.str("scopeAdminConsentDescription"),
                    "adminConsentDisplayName": ?cx.str("scopeAdminConsentDisplayName"),
                    "id": util::new_guid(),
                    "type": if cx.str("scopeConsentBy").as_deref() == Some("adminsAndUsers") { "User" } else { "Admin" },
                    "value": scope
                }] },
            );
        }
        Request::patch(format!("{GRAPH}/v1.0/myorganization/applications/{id}"))
            .header("content-type", "application/json;odata.metadata=none")
            .json(info)
            .json_response()
            .send()
            .await?;
    }
    // configureSecret
    let has_secrets = !arr(&app, "secrets").is_empty();
    if cx.bool("withSecret") && !has_secrets {
        cx.verbose_log("Configure Microsoft Entra app secret...");
        let secret = create_secret(&id, None, None).await?;
        let o = app.as_object_mut().unwrap();
        match o.get_mut("secrets") {
            Some(Value::Array(a)) => a.push(secret),
            _ => {
                o.insert("secrets".into(), Value::Array(vec![secret]));
            }
        }
    }
    if cx.bool("save") {
        save_to_rc(
            cx,
            app.get("appId").unwrap_or(&Value::Null),
            &app_name,
            false,
        );
    }
    let mut out = obj! {
        "appId": app.get("appId").cloned().unwrap_or(Value::Null),
        "objectId": app.get("id").cloned().unwrap_or(Value::Null),
        "tenantId": app.get("tenantId").cloned().unwrap_or(Value::Null)
    };
    if let Some(sec) = app.get("secrets").filter(|s| truthy(s)) {
        out.as_object_mut()
            .unwrap()
            .insert("secrets".into(), sec.clone());
    }
    cx.log(out)
}

async fn app_by(
    cx: &Cx,
    object_id: &str,
    app_id: &str,
    name: &str,
    props: Option<&[&str]>,
) -> Result<Value> {
    let (o, a, n) = (cx.s(object_id), cx.s(app_id), cx.s(name));
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Entra app {}...",
        o.clone()
            .or_else(|| a.clone())
            .or_else(|| n.clone())
            .unwrap_or_else(|| "undefined".into())
    ));
    if let Some(o) = o {
        entra_app::get_by_object_id(&o, props).await
    } else if let Some(a) = a {
        entra_app::get_by_app_id(&a, props).await
    } else {
        entra_app::get_by_app_name(&n.unwrap_or_else(|| "undefined".into()), props).await
    }
}

async fn object_id_of(cx: &Cx, object_id: &str, app_id: &str, name: &str) -> Result<String> {
    if let Some(o) = cx.s(object_id) {
        return Ok(o);
    }
    let (a, n) = (cx.s(app_id), cx.s(name));
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Entra app {}...",
        a.clone()
            .or_else(|| n.clone())
            .unwrap_or_else(|| "undefined".into())
    ));
    let app = match a {
        Some(a) => entra_app::get_by_app_id(&a, Some(&["id"])).await?,
        None => {
            entra_app::get_by_app_name(&n.unwrap_or_else(|| "undefined".into()), Some(&["id"]))
                .await?
        }
    };
    Ok(s(&app, "id").to_string())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let props_s = cx.str("properties");
    let props: Option<Vec<&str>> = props_s.as_deref().map(|p| p.split(',').collect());
    let app = app_by(cx, "objectId", "appId", "name", props.as_deref()).await?;
    if cx.bool("save") {
        save_to_rc(
            cx,
            app.get("appId").unwrap_or(&Value::Null),
            app.get("displayName").unwrap_or(&Value::Null),
            true,
        );
    }
    cx.log(app)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let q = qs(&ut::entra::select_query(cx.str("properties").as_deref())
        .into_iter()
        .collect::<Vec<_>>());
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/applications{q}")).await?,
    ))
}

#[derive(Clone)]
struct Perm {
    resource_id: String,
    resource_access: Vec<Value>,
    scope: Vec<String>,
}

fn resolve_permissions(
    cx: &Cx,
    sps: &[Value],
    apis: &str,
    scope_type: &str,
    perms: &mut Vec<Perm>,
) -> Result<Vec<Value>> {
    let mut resolved: Vec<Value> = Vec::new();
    for api in apis.split(' ').map(str::trim) {
        let pos = api.rfind('/').map(|p| p as isize).unwrap_or(-1);
        let perm_name = &api[(pos + 1) as usize..];
        let sp_name = if pos < 0 { "" } else { &api[..pos as usize] };
        cx.verbose_log(&format!("Resolving {api}..."));
        cx.verbose_log(&format!("Permission name: {perm_name}"));
        cx.verbose_log(&format!("Service principal name: {sp_name}"));
        let sp = sps
            .iter()
            .find(|sp| {
                let n: Vec<String> = arr(sp, "servicePrincipalNames")
                    .iter()
                    .map(to_js_string)
                    .collect();
                n.iter().any(|x| x == sp_name) || n.iter().any(|x| *x == format!("{sp_name}/"))
            })
            .ok_or_else(|| Error::msg(format!("Service principal {sp_name} not found")))?;
        let list = if scope_type == "Scope" {
            "oauth2PermissionScopes"
        } else {
            "appRoles"
        };
        let p = arr(sp, list)
            .iter()
            .find(|x| s(x, "value") == perm_name)
            .ok_or_else(|| {
                Error::msg(format!(
                    "Permission {perm_name} for service principal {sp_name} not found"
                ))
            })?;
        let app_id = sp.get("appId").cloned().unwrap_or(Value::Null);
        let idx = match resolved
            .iter()
            .position(|r| r.get("resourceAppId") == Some(&app_id))
        {
            Some(i) => i,
            None => {
                resolved
                    .push(obj! { "resourceAppId": app_id, "resourceAccess": Vec::<Value>::new() });
                resolved.len() - 1
            }
        };
        let access = obj! { "id": p.get("id").cloned().unwrap_or(Value::Null), "type": scope_type };
        if let Some(Value::Array(a)) = resolved[idx].get_mut("resourceAccess") {
            a.push(access.clone());
        }
        let sp_id = s(sp, "id").to_string();
        let pi = match perms.iter().position(|x| x.resource_id == sp_id) {
            Some(i) => i,
            None => {
                perms.push(Perm {
                    resource_id: sp_id,
                    resource_access: Vec::new(),
                    scope: Vec::new(),
                });
                perms.len() - 1
            }
        };
        let value = s(p, "value").to_string();
        if scope_type == "Scope" && !value.is_empty() && !perms[pi].scope.contains(&value) {
            perms[pi].scope.push(value);
        }
        if !perms[pi]
            .resource_access
            .iter()
            .any(|r| r.get("id") == access.get("id"))
        {
            perms[pi].resource_access.push(access);
        }
    }
    Ok(resolved)
}

pub async fn permission_add(cx: &mut Cx) -> Result<()> {
    let mut app = app_by(
        cx,
        "appObjectId",
        "appId",
        "appName",
        Some(&["id", "appId", "requiredResourceAccess"]),
    )
    .await?;
    let sps = odata::get_all_items(&format!("{GRAPH}/v1.0/servicePrincipals?$select=appId,appRoles,id,oauth2PermissionScopes,servicePrincipalNames")).await?;
    let mut perms: Vec<Perm> = Vec::new();
    let mut rra: Vec<Value> = arr(&app, "requiredResourceAccess").to_vec();
    let add_to = |resolved: Vec<Value>, rra: &mut Vec<Value>| {
        for r in resolved {
            match rra
                .iter_mut()
                .find(|x| x.get("resourceAppId") == r.get("resourceAppId"))
            {
                Some(existing) => {
                    for ra in arr(&r, "resourceAccess") {
                        if let Some(Value::Array(dst)) = existing.get_mut("resourceAccess")
                            && !dst.iter().any(|x| x.get("id") == ra.get("id"))
                        {
                            dst.push(ra.clone());
                        }
                    }
                }
                None => rra.push(r),
            }
        }
    };
    if let Some(d) = cx.s("delegatedPermissions") {
        let r = resolve_permissions(cx, &sps, &d, "Scope", &mut perms)?;
        add_to(r, &mut rra);
    }
    if let Some(a) = cx.s("applicationPermissions") {
        let r = resolve_permissions(cx, &sps, &a, "Role", &mut perms)?;
        add_to(r, &mut rra);
    }
    if let Some(o) = app.as_object_mut() {
        o.insert("requiredResourceAccess".into(), Value::Array(rra.clone()));
    }
    Request::patch(format!("{GRAPH}/v1.0/applications/{}", s(&app, "id")))
        .graph_json()
        .json(obj! { "requiredResourceAccess": rra })
        .send()
        .await?;
    if cx.bool("grantAdminConsent") {
        let app_id = s(&app, "appId").to_string();
        let sp = match sps.iter().find(|sp| s(sp, "appId") == app_id) {
            Some(sp) => sp.clone(),
            None => {
                cx.verbose_log(&format!("Creating service principal for app {app_id}..."));
                ut::entra_sp::create(&app_id).await?
            }
        };
        let sp_id = s(&sp, "id").to_string();
        for p in &perms {
            if !p.scope.is_empty() {
                cx.verbose_log(&format!("Granting consent for delegated permission(s) with resourceId {} and scope(s) {}", p.resource_id, p.scope.join(" ")));
                Request::post(format!("{GRAPH}/v1.0/oauth2PermissionGrants"))
                    .graph_json()
                    .json(obj! { "clientId": sp_id.as_str(), "consentType": "AllPrincipals", "principalId": Value::Null, "resourceId": p.resource_id.as_str(), "scope": p.scope.join(" ") })
                    .send()
                    .await?;
            }
            for a in p.resource_access.iter().filter(|a| s(a, "type") == "Role") {
                cx.verbose_log(&format!("Granting consent for application permission with resourceId {} and appRoleId {}", p.resource_id, s(a, "id")));
                Request::post(format!("{GRAPH}/v1.0/servicePrincipals/{sp_id}/appRoleAssignments"))
                    .header("content-type", "application/json;odata.metadata=none")
                    .json(obj! { "appRoleId": a.get("id").cloned().unwrap_or(Value::Null), "principalId": sp_id.as_str(), "resourceId": p.resource_id.as_str() })
                    .json_response()
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn permission_list(cx: &mut Cx) -> Result<()> {
    let object_id = object_id_of(cx, "appObjectId", "appId", "appName").await?;
    let ptype = cx.s("type").unwrap_or_else(|| "all".into());
    let app = Request::get(format!(
        "{GRAPH}/v1.0/myorganization/applications/{object_id}"
    ))
    .graph_json()
    .data()
    .await?;
    let rra = arr(&app, "requiredResourceAccess").to_vec();
    if rra.is_empty() {
        return cx.log(Value::Array(vec![]));
    }
    let mut sps: Vec<Option<Value>> = Vec::new();
    for r in &rra {
        let app_id = s(r, "resourceAppId").to_string();
        cx.verbose_log(&format!("Retrieving service principal {app_id}"));
        let res = Request::get(format!("{GRAPH}/v1.0/servicePrincipals?$filter=appId eq '{app_id}'&$select=appId,id,displayName")).graph_json().data().await?;
        let vals = arr(&res, "value");
        if !app_id.is_empty() && vals.is_empty() {
            sps.push(None);
            continue;
        }
        let mut sp = vals.first().cloned().unwrap_or(Value::Null);
        let sp_id = s(&sp, "id").to_string();
        cx.verbose_log(&format!(
            "Retrieving permissions for service principal {sp_id}..."
        ));
        if ptype == "all" || ptype == "delegated" {
            let p = Request::get(format!(
                "{GRAPH}/v1.0/servicePrincipals/{sp_id}/oauth2PermissionScopes"
            ))
            .graph_json()
            .data()
            .await?;
            sp.as_object_mut().map(|o| {
                o.insert(
                    "oauth2PermissionScopes".into(),
                    p.get("value").cloned().unwrap_or(Value::Null),
                )
            });
        }
        if ptype == "all" || ptype == "application" {
            let p = Request::get(format!("{GRAPH}/v1.0/servicePrincipals/{sp_id}/appRoles"))
                .graph_json()
                .data()
                .await?;
            sp.as_object_mut().map(|o| {
                o.insert(
                    "appRoles".into(),
                    p.get("value").cloned().unwrap_or(Value::Null),
                )
            });
        }
        sps.push(Some(sp));
    }
    let mut out = Vec::new();
    for r in &rra {
        let app_id = r.get("resourceAppId").cloned().unwrap_or(Value::Null);
        let sp = sps
            .iter()
            .flatten()
            .find(|sp| sp.get("appId") == Some(&app_id));
        let resource = sp
            .and_then(|sp| sp.get("displayName").filter(|v| !v.is_null()).cloned())
            .unwrap_or(app_id.clone());
        for p in arr(r, "resourceAccess") {
            let t = s(p, "type");
            if (ptype == "application" && t == "Scope") || (ptype == "delegated" && t == "Role") {
                continue;
            }
            let pid = p.get("id").cloned().unwrap_or(Value::Null);
            let name = match sp {
                None => pid.clone(),
                Some(sp) => {
                    let list = if t == "Role" {
                        "appRoles"
                    } else {
                        "oauth2PermissionScopes"
                    };
                    arr(sp, list)
                        .iter()
                        .find(|x| x.get("id") == Some(&pid))
                        .and_then(|x| x.get("value").filter(|v| !v.is_null()).cloned())
                        .unwrap_or(pid.clone())
                }
            };
            out.push(obj! { "resource": resource.clone(), "resourceId": app_id.clone(), "permission": name, "type": if t == "Role" { "Application" } else { "Delegated" } });
        }
    }
    cx.log(Value::Array(out))
}

pub async fn permission_remove(cx: &mut Cx) -> Result<()> {
    let which = cx.first_u(&["appId", "appObjectId", "appName"]);
    if !cx.proceed("force", &format!("Are you sure you want to remove the permissions from the specified application {which}?"))? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing permissions from application {which}..."));
    let app = app_by(
        cx,
        "appObjectId",
        "appId",
        "appName",
        Some(&["id", "appId", "requiredResourceAccess"]),
    )
    .await?;
    let sps = odata::get_all_items(&format!("{GRAPH}/v1.0/servicePrincipals?$select=appId,appRoles,id,oauth2PermissionScopes,servicePrincipalNames")).await?;
    let mut perms: Vec<Perm> = Vec::new();
    let mut rra: Vec<Value> = arr(&app, "requiredResourceAccess").to_vec();
    let remove_from = |resolved: Vec<Value>, rra: &mut Vec<Value>| {
        for r in resolved {
            if let Some(existing) = rra
                .iter_mut()
                .find(|x| x.get("resourceAppId") == r.get("resourceAppId"))
            {
                for ra in arr(&r, "resourceAccess") {
                    if let Some(Value::Array(dst)) = existing.get_mut("resourceAccess") {
                        dst.retain(|x| x.get("id") != ra.get("id"));
                    }
                }
            }
        }
    };
    if let Some(d) = cx.s("delegatedPermissions") {
        let r = resolve_permissions(cx, &sps, &d, "Scope", &mut perms)?;
        remove_from(r, &mut rra);
    }
    if let Some(a) = cx.s("applicationPermissions") {
        let r = resolve_permissions(cx, &sps, &a, "Role", &mut perms)?;
        remove_from(r, &mut rra);
    }
    // the upstream loop removes while iterating (skipping the next item)
    let mut i = 0;
    while i < rra.len() {
        if rra[i]
            .get("resourceAccess")
            .and_then(Value::as_array)
            .is_some_and(|a| a.is_empty())
        {
            rra.remove(i);
        }
        i += 1;
    }
    Request::patch(format!("{GRAPH}/v1.0/applications/{}", s(&app, "id")))
        .graph_json()
        .json(obj! { "requiredResourceAccess": rra })
        .send()
        .await?;
    if cx.bool("revokeAdminConsent") {
        let app_id = s(&app, "appId").to_string();
        let Some(sp) = sps.iter().find(|sp| s(sp, "appId") == app_id) else {
            cx.debug_log(&format!("No service principal found for the appId: {app_id}. Skipping revoking admin consent."));
            return Ok(());
        };
        let sp_id = s(sp, "id").to_string();
        let mut roles = Vec::new();
        let mut grants = Vec::new();
        if perms
            .iter()
            .any(|p| p.resource_access.iter().any(|a| s(a, "type") == "Role"))
        {
            roles = odata::get_all_items(&format!("{GRAPH}/v1.0/servicePrincipals/{sp_id}/appRoleAssignments?$select=id,appRoleId,resourceId")).await?;
        }
        if perms.iter().any(|p| !p.scope.is_empty()) {
            grants = odata::get_all_items(&format!("{GRAPH}/v1.0/servicePrincipals/{sp_id}/oAuth2PermissionGrants?$select=id,resourceId,scope")).await?;
        }
        for p in &perms {
            if !p.scope.is_empty() {
                cx.verbose_log(&format!("Revoking consent for delegated permission(s) with resourceId {} and scope(s) {}", p.resource_id, p.scope.join(" ")));
                if let Some(g) = grants.iter().find(|g| s(g, "resourceId") == p.resource_id) {
                    let mut scopes: Vec<String> =
                        s(g, "scope").split(' ').map(str::to_string).collect();
                    for sc in &p.scope {
                        // splice(indexOf(scope), 1): -1 removes the last element
                        match scopes.iter().position(|x| x == sc) {
                            Some(i) => {
                                scopes.remove(i);
                            }
                            None => {
                                scopes.pop();
                            }
                        }
                    }
                    let mut g = g.clone();
                    g.as_object_mut()
                        .map(|o| o.insert("scope".into(), Value::from(scopes.join(" "))));
                    Request::patch(format!(
                        "{GRAPH}/v1.0/oauth2PermissionGrants/{}",
                        s(&g, "id")
                    ))
                    .graph_json()
                    .json(g.clone())
                    .send()
                    .await?;
                }
            }
            for a in p.resource_access.iter().filter(|a| s(a, "type") == "Role") {
                cx.verbose_log(&format!("Revoking consent for application permission with resourceId {} and appRoleId {}", p.resource_id, s(a, "id")));
                if let Some(r) = roles.iter().find(|r| {
                    s(r, "resourceId") == p.resource_id && r.get("appRoleId") == a.get("id")
                }) {
                    Request::delete(format!(
                        "{GRAPH}/v1.0/servicePrincipals/{sp_id}/appRoleAssignments/{}",
                        s(r, "id")
                    ))
                    .graph_json()
                    .send()
                    .await?;
                }
            }
        }
    }
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove the app?")? {
        return Ok(());
    }
    let id = object_id_of(cx, "objectId", "appId", "name").await?;
    cx.verbose_log(&format!("Deleting Microsoft Entra app {id}..."));
    Request::delete(format!("{GRAPH}/v1.0/myorganization/applications/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub fn role_add_field(idx: u16, v: &Value, _o: &Obj) -> m365_core::schema::FieldCheck {
    m365_core::schema::check({
        let c = to_js_string(v);
        match idx {
            0 => !c.starts_with('.'),
            _ => {
                static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                    regex::Regex::new(r"^[\w:!#$%&'()*+,\-./:;<=>?@\[\]^+_`{|}~]+$").unwrap()
                });
                RE.is_match(&c)
            }
        }
    })
}

pub async fn role_add(cx: &mut Cx) -> Result<()> {
    let app = app_by(
        cx,
        "appObjectId",
        "appId",
        "appName",
        Some(&["id", "appRoles"]),
    )
    .await?;
    cx.verbose_log(&format!(
        "Adding role {} to Microsoft Entra app {}...",
        cx.u("name"),
        s(&app, "id")
    ));
    let members: Vec<Value> = match cx.str("allowedMembers").as_deref() {
        Some("usersGroups") => vec!["User".into()],
        Some("applications") => vec!["Application".into()],
        Some("both") => vec!["User".into(), "Application".into()],
        _ => vec![],
    };
    let mut roles = arr(&app, "appRoles").to_vec();
    roles.push(obj! { "displayName": ?cx.str("name"), "description": ?cx.str("description"), "id": util::new_guid(), "value": ?cx.str("claim"), "allowedMemberTypes": members });
    Request::patch(format!(
        "{GRAPH}/v1.0/myorganization/applications/{}",
        s(&app, "id")
    ))
    .graph_json()
    .json(obj! { "appRoles": roles })
    .send()
    .await?;
    Ok(())
}

pub async fn role_list(cx: &mut Cx) -> Result<()> {
    let id = object_id_of(cx, "appObjectId", "appId", "appName").await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/myorganization/applications/{id}/appRoles"
        ))
        .await?,
    ))
}

pub async fn role_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove the app role?")? {
        return Ok(());
    }
    let app = app_by(
        cx,
        "appObjectId",
        "appId",
        "appName",
        Some(&["id", "appRoles"]),
    )
    .await?;
    let label = if let Some(n) = cx.s("name") {
        format!("name '{n}'")
    } else if let Some(c) = cx.s("claim") {
        format!("claim '{c}'")
    } else {
        format!("id '{}'", cx.u("id"))
    };
    let app_id = s(&app, "id").to_string();
    cx.verbose_log(&format!(
        "Deleting role with {label} from Microsoft Entra app {app_id}..."
    ));
    let (prop, val) = if let Some(n) = cx.s("name") {
        ("displayName", n)
    } else if let Some(c) = cx.s("claim") {
        ("value", c)
    } else {
        ("id", cx.u("id"))
    };
    let mut roles = arr(&app, "appRoles").to_vec();
    let mut matching: Vec<Value> = roles
        .iter()
        .filter(|r| s(r, prop) == val)
        .cloned()
        .collect();
    if cx.s("name").is_some() && matching.len() > 1 {
        matching[0] = cx.handle_multiple_results(
            &format!("Multiple roles with name '{}' found.", cx.u("name")),
            &util::array_to_hash_table("id", &matching),
        )?;
    }
    let Some(role) = matching.first().cloned() else {
        return Err(Error::msg(format!("No app role with {label} found.")));
    };
    let rid = role.get("id").cloned().unwrap_or(Value::Null);
    if role.get("isEnabled").is_some_and(truthy) {
        cx.verbose_log("Disabling the app role");
        if let Some(r) = roles.iter_mut().find(|r| r.get("id") == Some(&rid)) {
            r.as_object_mut()
                .map(|o| o.insert("isEnabled".into(), Value::Bool(false)));
        }
        Request::patch(format!("{GRAPH}/v1.0/myorganization/applications/{app_id}"))
            .graph_json()
            .json(obj! { "appRoles": roles.clone() })
            .send()
            .await?;
    }
    cx.verbose_log("Deleting the app role.");
    let updated: Vec<Value> = roles
        .into_iter()
        .filter(|r| r.get("id") != Some(&rid))
        .collect();
    Request::patch(format!("{GRAPH}/v1.0/myorganization/applications/{app_id}"))
        .graph_json()
        .json(obj! { "appRoles": updated })
        .send()
        .await?;
    Ok(())
}

pub fn set_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(match i {
        3 => !t("certificateDisplayName") || t("certificateFile") || t("certificateBase64Encoded"),
        4 => {
            !o.get("certificateFile").is_some_and(truthy)
                || std::path::Path::new(&to_js_string(&o["certificateFile"])).exists()
        }
        _ => true,
    })
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let id = object_id_of(cx, "objectId", "appId", "name").await?;
    let url = format!("{GRAPH}/v1.0/myorganization/applications/{id}");
    let patch = |body: Value| {
        Request::patch(url.clone())
            .header("content-type", "application/json;odata.metadata=none")
            .json(body)
            .json_response()
    };
    let unknown = ut::options::unknown_options(&cx.opts, cx.meta);
    if !unknown.is_empty() {
        patch(Value::Object(unknown)).send().await?;
    }
    if let Some(uris) = cx.s("uris") {
        cx.verbose_log("Configuring Microsoft Entra application ID URI...");
        patch(obj! { "identifierUris": uris.split(',').map(|u| Value::from(u.trim())).collect::<Vec<_>>() }).send().await?;
    }
    if cx.s("redirectUris").is_some() || cx.s("redirectUrisToRemove").is_some() {
        cx.verbose_log("Configuring Microsoft Entra application redirect URIs...");
        let app = Request::get(url.clone())
            .header("content-type", "application/json;odata.metadata=none")
            .json_response()
            .data()
            .await?;
        // `application.publicClient.redirectUris` etc.: a missing platform throws
        for k in ["publicClient", "spa", "web"] {
            if app.get(k).is_none_or(Value::is_null) {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'redirectUris')",
                ));
            }
        }
        let list = |k: &str| -> Option<Vec<Value>> {
            app.get(k)
                .and_then(|p| p.get("redirectUris"))
                .and_then(Value::as_array)
                .cloned()
        };
        let (pc, spa, web) = (list("publicClient"), list("spa"), list("web"));
        let mut patch_pc = pc.clone();
        let mut patch_spa = spa.clone();
        let mut patch_web = web.clone();
        if let Some(r) = cx.s("redirectUrisToRemove") {
            let rm: Vec<String> = r.split(',').map(|u| u.trim().to_string()).collect();
            let f = |l: &Option<Vec<Value>>| {
                l.as_ref().map(|l| {
                    l.iter()
                        .filter(|u| !rm.contains(&to_js_string(u)))
                        .cloned()
                        .collect::<Vec<_>>()
                })
            };
            patch_pc = f(&pc);
            patch_spa = f(&spa);
            patch_web = f(&web);
        }
        if let Some(r) = cx.s("redirectUris") {
            let add: Vec<String> = r.split(',').map(|u| u.trim().to_string()).collect();
            let push = |target: &mut Option<Vec<Value>>, existing: &Option<Vec<Value>>| {
                let ex: Vec<String> = existing
                    .as_ref()
                    .map(|e| e.iter().map(to_js_string).collect())
                    .unwrap_or_default();
                if let Some(t) = target.as_mut() {
                    t.extend(
                        add.iter()
                            .filter(|u| !ex.contains(u))
                            .map(|u| Value::from(u.as_str())),
                    );
                }
            };
            match cx.str("platform").as_deref() {
                Some("spa") => push(&mut patch_spa, &spa),
                Some("publicClient") => push(&mut patch_pc, &pc),
                Some("web") => push(&mut patch_web, &web),
                _ => {}
            }
        }
        let wrap = |v: Option<Vec<Value>>| obj! { "redirectUris": ?v };
        patch(
            obj! { "publicClient": wrap(patch_pc), "spa": wrap(patch_spa), "web": wrap(patch_web) },
        )
        .send()
        .await?;
    }
    if let Some(b) = cx.opt_bool("allowPublicClientFlows") {
        cx.verbose_log("Configuring Entra application AllowPublicClientFlows option...");
        patch(obj! { "isFallbackPublicClient": b }).send().await?;
    }
    if cx.s("certificateFile").is_some() || cx.s("certificateBase64Encoded").is_some() {
        cx.verbose_log("Setting certificate for Microsoft Entra app...");
        let cert = match cx.s("certificateBase64Encoded") {
            Some(c) => c,
            None => {
                let f = cx.u("certificateFile");
                cx.debug_log(&format!("Reading existing {f}..."));
                let b = std::fs::read(&f).map_err(|e| {
                    Error::js(format!(
                        "Error reading certificate file: Error: {}. Please add the certificate using base64 option '--certificateBase64Encoded'.",
                        util::node_fs_error(&e, "open", &f)
                    ))
                })?;
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b)
            }
        };
        cx.verbose_log("Retrieving current keyCredentials list for app");
        let app = Request::get(format!("{url}?$select=keyCredentials"))
            .header("content-type", "application/json;odata.metadata=none")
            .json_response()
            .data()
            .await?;
        cx.verbose_log("Adding new keyCredential to list");
        let mut creds: Vec<Value> = app
            .get("keyCredentials")
            .filter(|v| truthy(v))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        creds.retain(|c| c.get("key").and_then(Value::as_str) != Some(cert.as_str()));
        creds.push(obj! { "type": "AsymmetricX509Cert", "usage": "Verify", "displayName": ?cx.str("certificateDisplayName"), "key": cert });
        cx.verbose_log("Updating keyCredentials in Microsoft Entra app");
        patch(obj! { "keyCredentials": creds }).send().await?;
    }
    Ok(())
}

//! `src/utils/entraApp.ts`: app registrations and admin consent.

use crate::error::{Error, Result};
use crate::http::Request;
use crate::json::{Obj, stringify_pretty};
use crate::log;
use crate::odata;
use crate::util::{array_to_hash_table, encode_query_parameter};
use serde_json::Value;
use std::sync::Mutex;

#[derive(Clone, Debug, Default)]
pub struct AppCreationOptions {
    pub apis_application: Option<String>,
    pub apis_delegated: Option<String>,
    pub implicit_flow: bool,
    pub multitenant: bool,
    pub name: Option<String>,
    pub platform: Option<String>,
    pub redirect_uris: Option<String>,
    pub certificate_file: Option<String>,
    pub certificate_base64_encoded: Option<String>,
    pub certificate_display_name: Option<String>,
    pub allow_public_client_flows: bool,
    pub bundle_id: Option<String>,
    pub signature_hash: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AppPermission {
    pub resource_id: String,
    /// `{ id, type }` objects
    pub resource_access: Vec<Value>,
    pub scope: Vec<String>,
}

/// `entraApp.appPermissions`, filled while resolving APIs.
pub static APP_PERMISSIONS: Mutex<Vec<AppPermission>> = Mutex::new(Vec::new());

fn update_app_permissions(sp_id: &str, access: &Value, oauth_value: Option<&str>) {
    let mut perms = APP_PERMISSIONS.lock().unwrap();
    let idx = match perms.iter().position(|p| p.resource_id == sp_id) {
        Some(i) => i,
        None => {
            perms.push(AppPermission {
                resource_id: sp_id.to_string(),
                resource_access: Vec::new(),
                scope: Vec::new(),
            });
            perms.len() - 1
        }
    };
    let p = &mut perms[idx];
    if access.get("type").and_then(Value::as_str) == Some("Scope")
        && let Some(v) = oauth_value
        && !p.scope.iter().any(|s| s == v)
    {
        p.scope.push(v.to_string());
    }
    if !p
        .resource_access
        .iter()
        .any(|r| r.get("id") == access.get("id"))
    {
        p.resource_access.push(access.clone());
    }
}

fn str_list(v: &Value, k: &str) -> Vec<String> {
    v.get(k)
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn required_resource_access_for_apis(
    sps: &[Value],
    apis: Option<&str>,
    scope_type: &str,
    debug: bool,
) -> Result<Vec<Value>> {
    let Some(apis) = apis.filter(|a| !a.is_empty()) else {
        return Ok(Vec::new());
    };
    let mut resolved: Vec<Value> = Vec::new();
    for api in apis.split(',').map(str::trim) {
        let pos = api.rfind('/').map(|p| p as isize).unwrap_or(-1);
        let permission_name = &api[(pos + 1) as usize..];
        let sp_name = if pos < 0 { "" } else { &api[..pos as usize] };
        if debug {
            log::log_stderr(&format!("Resolving {api}..."));
            log::log_stderr(&format!("Permission name: {permission_name}"));
            log::log_stderr(&format!("Service principal name: {sp_name}"));
        }
        let sp = sps
            .iter()
            .find(|sp| {
                let names = str_list(sp, "servicePrincipalNames");
                names.iter().any(|n| n == sp_name)
                    || names.iter().any(|n| *n == format!("{sp_name}/"))
            })
            .ok_or_else(|| Error::msg(format!("Service principal {sp_name} not found")))?;
        let list_key = if scope_type == "Scope" {
            "oauth2PermissionScopes"
        } else {
            "appRoles"
        };
        let permission = crate::json::arr(sp, list_key)
            .iter()
            .find(|s| s.get("value").and_then(Value::as_str) == Some(permission_name))
            .ok_or_else(|| {
                Error::msg(format!(
                    "Permission {permission_name} for service principal {sp_name} not found"
                ))
            })?;
        let app_id = sp.get("appId").cloned().unwrap_or(Value::Null);
        let idx = match resolved
            .iter()
            .position(|r| r.get("resourceAppId") == Some(&app_id))
        {
            Some(i) => i,
            None => {
                resolved.push(crate::obj! { "resourceAppId": app_id.clone(), "resourceAccess": Value::Array(vec![]) });
                resolved.len() - 1
            }
        };
        let access = crate::obj! { "id": permission.get("id").cloned().unwrap_or(Value::Null), "type": scope_type };
        if let Some(Value::Array(a)) = resolved[idx].get_mut("resourceAccess") {
            a.push(access.clone());
        }
        update_app_permissions(
            crate::json::s(sp, "id"),
            &access,
            permission.get("value").and_then(Value::as_str),
        );
    }
    Ok(resolved)
}

/// `entraApp.resolveApis`.
pub async fn resolve_apis(
    options: &AppCreationOptions,
    manifest: Option<&Value>,
    verbose: bool,
    debug: bool,
) -> Result<Vec<Value>> {
    let manifest_rra = manifest
        .and_then(|m| m.get("requiredResourceAccess"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let has_apis = options
        .apis_delegated
        .as_deref()
        .is_some_and(|s| !s.is_empty())
        || options
            .apis_application
            .as_deref()
            .is_some_and(|s| !s.is_empty());
    if !has_apis && manifest_rra.is_empty() {
        return Ok(Vec::new());
    }
    if verbose {
        log::log_stderr("Resolving requested APIs...");
    }
    let sps = odata::get_all_items(
        "https://graph.microsoft.com/v1.0/myorganization/servicePrincipals?$select=appId,appRoles,id,oauth2PermissionScopes,servicePrincipalNames",
    )
    .await?;
    let mut resolved: Vec<Value>;
    if has_apis {
        resolved = required_resource_access_for_apis(
            &sps,
            options.apis_delegated.as_deref(),
            "Scope",
            debug,
        )?;
        if verbose {
            log::log_stderr(&format!(
                "Resolved delegated permissions: {}",
                stringify_pretty(&Value::Array(resolved.clone()), "  ")
            ));
        }
        let app_apis = required_resource_access_for_apis(
            &sps,
            options.apis_application.as_deref(),
            "Role",
            debug,
        )?;
        if verbose {
            log::log_stderr(&format!(
                "Resolved application permissions: {}",
                stringify_pretty(&Value::Array(app_apis.clone()), "  ")
            ));
        }
        for r in app_apis {
            match resolved
                .iter_mut()
                .find(|x| x.get("resourceAppId") == r.get("resourceAppId"))
            {
                Some(existing) => {
                    if let (Some(Value::Array(dst)), Some(Value::Array(src))) =
                        (existing.get_mut("resourceAccess"), r.get("resourceAccess"))
                    {
                        dst.extend(src.iter().cloned());
                    }
                }
                None => resolved.push(r),
            }
        }
    } else {
        resolved = Vec::new();
        for api in manifest_rra {
            resolved.push(api.clone());
            if let Some(app) = sps
                .iter()
                .find(|sp| sp.get("appId") == api.get("resourceAppId"))
            {
                for res in crate::json::arr(&api, "resourceAccess") {
                    let access = crate::obj! { "id": res.get("id").cloned().unwrap_or(Value::Null), "type": res.get("type").cloned().unwrap_or(Value::Null) };
                    let oauth = crate::json::arr(app, "oauth2PermissionScopes")
                        .iter()
                        .find(|s| s.get("id") == res.get("id"))
                        .and_then(|s| s.get("value").and_then(Value::as_str));
                    update_app_permissions(crate::json::s(app, "id"), &access, oauth);
                }
            }
        }
    }
    if verbose {
        let perms = APP_PERMISSIONS.lock().unwrap().clone();
        log::log_stderr(&format!(
            "Merged delegated and application permissions: {}",
            stringify_pretty(&Value::Array(resolved.clone()), "  ")
        ));
        let roles: Vec<Value> = perms
            .iter()
            .flat_map(|p| {
                p.resource_access
                    .iter()
                    .filter(|a| a.get("type").and_then(Value::as_str) == Some("Role"))
                    .cloned()
            })
            .collect();
        log::log_stderr(&format!(
            "App role assignments: {}",
            stringify_pretty(&Value::Array(roles), "  ")
        ));
        let scopes: Vec<Value> = perms
            .iter()
            .flat_map(|p| p.scope.iter().map(|s| Value::from(s.as_str())))
            .collect();
        log::log_stderr(&format!(
            "OAuth2 permissions: {}",
            stringify_pretty(&Value::Array(scopes), "  ")
        ));
    }
    Ok(resolved)
}

fn certificate_base64(options: &AppCreationOptions, debug: bool) -> Result<String> {
    if let Some(c) = options
        .certificate_base64_encoded
        .as_deref()
        .filter(|c| !c.is_empty())
    {
        return Ok(c.to_string());
    }
    let f = options.certificate_file.clone().unwrap_or_default();
    if debug {
        log::log_stderr(&format!("Reading existing {f}..."));
    }
    std::fs::read(&f).map(|b| base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b)).map_err(|e| {
        Error::js(format!(
            "Error reading certificate file: Error: {}. Please add the certificate using base64 option '--certificateBase64Encoded'.",
            crate::util::node_fs_error(&e, "open", &f)
        ))
    })
}

/// `entraApp.createAppRegistration`.
pub async fn create_app_registration(
    options: &AppCreationOptions,
    unknown: &Obj,
    apis: &[Value],
    verbose: bool,
    debug: bool,
) -> Result<Value> {
    let mut info = Obj::new();
    info.insert(
        "displayName".into(),
        options.name.clone().map(Value::from).unwrap_or(Value::Null),
    );
    if options.name.is_none() {
        info.shift_remove("displayName");
    }
    info.insert(
        "signInAudience".into(),
        Value::from(if options.multitenant {
            "AzureADMultipleOrgs"
        } else {
            "AzureADMyOrg"
        }),
    );
    if !apis.is_empty() {
        info.insert("requiredResourceAccess".into(), Value::Array(apis.to_vec()));
    }
    if let Some(r) = options.redirect_uris.as_deref().filter(|r| !r.is_empty()) {
        let uris: Vec<Value> = r.split(',').map(|u| Value::from(u.trim())).collect();
        info.insert(
            options
                .platform
                .clone()
                .unwrap_or_else(|| "undefined".into()),
            crate::obj! { "redirectUris": uris },
        );
    }
    if options.platform.as_deref() == Some("android") {
        info.insert(
            "publicClient".into(),
            crate::obj! { "redirectUris": vec![Value::from(format!("msauth://{}/{}", options.bundle_id.clone().unwrap_or_default(), encode_query_parameter(options.signature_hash.as_deref().unwrap_or(""))))] },
        );
    }
    if options.platform.as_deref() == Some("apple") {
        let b = options.bundle_id.clone().unwrap_or_default();
        info.insert(
            "publicClient".into(),
            crate::obj! { "redirectUris": vec![Value::from(format!("msauth://code/msauth.{b}%3A%2F%2Fauth")), Value::from(format!("msauth.{b}://auth"))] },
        );
    }
    if options.implicit_flow {
        let web = info
            .entry("web".to_string())
            .or_insert_with(|| Value::Object(Obj::new()));
        if let Some(w) = web.as_object_mut() {
            w.insert(
                "implicitGrantSettings".into(),
                crate::obj! { "enableAccessTokenIssuance": true, "enableIdTokenIssuance": true },
            );
        }
    }
    if options
        .certificate_file
        .as_deref()
        .is_some_and(|s| !s.is_empty())
        || options
            .certificate_base64_encoded
            .as_deref()
            .is_some_and(|s| !s.is_empty())
    {
        let key = certificate_base64(options, debug)?;
        info.insert(
            "keyCredentials".into(),
            Value::Array(vec![crate::obj! { "type": "AsymmetricX509Cert", "usage": "Verify", "displayName": ?options.certificate_display_name.clone(), "key": key }]),
        );
    }
    if options.allow_public_client_flows {
        info.insert("isFallbackPublicClient".into(), Value::Bool(true));
    }
    crate::ut::options::add_unknown_options_to_payload(&mut info, unknown);
    if verbose {
        log::log_stderr("Creating Microsoft Entra app registration...");
    }
    Request::post("https://graph.microsoft.com/v1.0/myorganization/applications")
        .accept("application/json;odata.metadata=none")
        .json(Value::Object(info))
        .json_response()
        .data()
        .await
}

/// `entraApp.grantAdminConsent`.
pub async fn grant_admin_consent(app_info: &Value, admin_consent: bool, debug: bool) -> Result<()> {
    let perms = APP_PERMISSIONS.lock().unwrap().clone();
    if !admin_consent || perms.is_empty() {
        return Ok(());
    }
    let sp = Request::post("https://graph.microsoft.com/v1.0/myorganization/servicePrincipals")
        .header("content-type", "application/json")
        .json(crate::obj! { "appId": app_info.get("appId").cloned().unwrap_or(Value::Null) })
        .json_response()
        .data()
        .await?;
    let sp_id = crate::json::s(&sp, "id").to_string();
    if debug {
        log::log_stderr(&format!(
            "Service principal created, returned object id: {sp_id}"
        ));
    }
    for p in &perms {
        if !p.scope.is_empty() {
            Request::post("https://graph.microsoft.com/v1.0/myorganization/oauth2PermissionGrants")
                .accept("application/json;odata.metadata=none")
                .json_response()
                .json(crate::obj! { "clientId": sp_id.as_str(), "consentType": "AllPrincipals", "principalId": Value::Null, "resourceId": p.resource_id.as_str(), "scope": p.scope.join(" ") })
                .send()
                .await?;
            if debug {
                log::log_stderr(&format!(
                    "Admin consent granted for following resource {}, with delegated permissions: {}",
                    p.resource_id,
                    p.scope.join(",")
                ));
            }
        }
        for a in p
            .resource_access
            .iter()
            .filter(|a| a.get("type").and_then(Value::as_str) == Some("Role"))
        {
            Request::post(format!("https://graph.microsoft.com/v1.0/myorganization/servicePrincipals/{sp_id}/appRoleAssignments"))
                .header("Content-Type", "application/json")
                .json_response()
                .json(crate::obj! { "appRoleId": a.get("id").cloned().unwrap_or(Value::Null), "principalId": sp_id.as_str(), "resourceId": p.resource_id.as_str() })
                .send()
                .await?;
            if debug {
                log::log_stderr(&format!(
                    "Admin consent granted for following resource {}, with application permission: {}",
                    p.resource_id,
                    crate::json::s(a, "id")
                ));
            }
        }
    }
    Ok(())
}

pub async fn get_by_app_id(app_id: &str, properties: Option<&[&str]>) -> Result<Value> {
    let mut url =
        format!("https://graph.microsoft.com/v1.0/applications?$filter=appId eq '{app_id}'");
    if let Some(p) = properties {
        url.push_str(&format!("&$select={}", p.join(",")));
    }
    odata::get_all_items(&url)
        .await?
        .into_iter()
        .next()
        .ok_or_else(|| {
            Error::js(format!(
                "App with appId '{app_id}' not found in Microsoft Entra ID."
            ))
        })
}

pub async fn get_by_app_name(name: &str, properties: Option<&[&str]>) -> Result<Value> {
    let mut url = format!(
        "https://graph.microsoft.com/v1.0/applications?$filter=displayName eq '{}'",
        encode_query_parameter(name)
    );
    if let Some(p) = properties {
        url.push_str(&format!("&$select={}", p.join(",")));
    }
    let apps = odata::get_all_items(&url).await?;
    match apps.len() {
        0 => Err(Error::js(format!(
            "App with name '{name}' not found in Microsoft Entra ID."
        ))),
        1 => Ok(apps.into_iter().next().unwrap()),
        _ => crate::runner::handle_multiple_results(
            &format!("Multiple apps with name '{name}' found in Microsoft Entra ID."),
            &array_to_hash_table("id", &apps),
        ),
    }
}

pub async fn get_by_object_id(object_id: &str, properties: Option<&[&str]>) -> Result<Value> {
    let mut url = format!("https://graph.microsoft.com/v1.0/applications/{object_id}");
    if let Some(p) = properties {
        url.push_str(&format!("?$select={}", p.join(",")));
    }
    match Request::get(url).graph_json().data().await {
        Ok(v) => Ok(v),
        Err(e) if e.status() == Some(404) => Err(Error::js(format!(
            "App with objectId '{object_id}' not found in Microsoft Entra ID."
        ))),
        Err(e) => Err(e),
    }
}

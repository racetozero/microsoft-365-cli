//! `pa *`: Power Apps.

use m365_core::prelude::*;
use m365_core::ut::entra;

pub const POWERAPPS: &str = "https://api.powerapps.com";
pub const BAP: &str = "https://api.bap.microsoft.com";

pub async fn app_consent_set(cx: &mut Cx) -> Result<()> {
    let (name, bypass) = (cx.u("name"), cx.u("bypass"));
    cx.verbose_log(&format!(
        "Setting the bypass consent for the Microsoft Power App {name}... to {bypass}"
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you bypass the consent for the Microsoft Power App {name} to {bypass}?"
        ),
    )? {
        return Ok(());
    }
    Request::post(format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/scopes/admin/environments/{}/apps/{name}/setPowerAppConnectionDirectConsentBypass?api-version=2021-02-01",
        cx.u("environmentName")
    ))
    .graph_json()
    .json(obj! { "bypassconsent": ?cx.val("bypass").cloned() })
    .send()
    .await?;
    Ok(())
}

/// Replaces characters not allowed in file names.
pub fn safe_file_name(s: &str) -> String {
    s.chars()
        .map(|c| if r#"\/:*?"<>|"#.contains(c) { '_' } else { c })
        .collect()
}

pub async fn app_export(cx: &mut Cx) -> Result<()> {
    let (name, env) = (cx.u("name"), cx.u("environmentName"));
    cx.verbose_log(&format!(
        "Initiating package export for Microsoft Power App {name}..."
    ));
    cx.verbose_log("Getting the Microsoft Power App resources...");
    let base = format!(
        "{BAP}/providers/Microsoft.BusinessAppPlatform/environments/{}",
        eqp(&env)
    );
    let mut res = Request::post(format!(
        "{base}/listPackageResources?api-version=2016-11-01"
    ))
    .accept("application/json")
    .json_response()
    .json(obj! { "baseResourceIds": [format!("/providers/Microsoft.PowerApps/apps/{name}")] })
    .data()
    .await?;
    let resources = res
        .get_mut("resources")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?;
    for v in resources.values_mut() {
        v.as_object_mut()
            .map(|o| o.insert("suggestedCreationType".into(), Value::from("Update")));
    }
    let resources = Value::Object(resources.clone());
    let r = Request::post(format!("{base}/exportPackage?api-version=2016-11-01"))
        .accept("application/json")
        .json_response()
        .json(obj! {
            "includedResourceIds": [format!("/providers/Microsoft.PowerApps/apps/{name}")],
            "details": obj! {
                "creator": ?cx.val("packageCreatedBy").cloned(),
                "description": ?cx.val("packageDescription").cloned(),
                "displayName": cx.s("packageDisplayName").unwrap_or_else(|| name.clone()),
                "sourceEnvironment": ?cx.val("packageSourceEnvironment").cloned(),
            },
            "resources": resources,
        })
        .send()
        .await?;
    let location = r.header("location").unwrap_or("undefined").to_string();
    cx.verbose_log("Retrieving the package link and waiting on the exported package.");
    let mut link = String::from("undefined");
    loop {
        let r = Request::get(location.clone())
            .accept("application/json")
            .json_response()
            .data()
            .await?;
        let props = r
            .get("properties")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'status')"))?;
        let status = props
            .get("status")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        if status == "Succeeded" {
            link = props
                .get("packageLink")
                .and_then(|p| p.get("value"))
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into());
        } else {
            util::sleep_ms(5000).await;
        }
        cx.verbose_log(&format!("Current status of the get package link: {status}"));
        if status != "Running" {
            break;
        }
    }
    let filename = cx
        .s("packageDisplayName")
        .map(|d| safe_file_name(&d))
        .unwrap_or(name);
    let file = Request::get(link)
        .anonymous()
        .bytes_response()
        .send()
        .await?;
    let mut path = cx.s("path").unwrap_or_else(|| "./".into());
    if !path.ends_with('/') {
        path.push('/');
    }
    path.push_str(&format!("{filename}.zip"));
    std::fs::write(&path, &file.bytes)
        .map_err(|e| Error::js(util::node_fs_error(&e, "open", &path)))?;
    cx.verbose_log(&format!("File saved to path '{path}'"));
    Ok(())
}

fn set_app_properties(mut app: Value) -> Result<Value> {
    let props = app
        .get("properties")
        .cloned()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'displayName')"))?;
    let owner = props
        .get("owner")
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'email')"))?;
    let or_empty = |v: Option<&Value>| v.filter(|x| truthy(x)).cloned().unwrap_or(Value::from(""));
    if let Some(o) = app.as_object_mut() {
        o.insert(
            "displayName".into(),
            props.get("displayName").cloned().unwrap_or(Value::Null),
        );
        o.insert("description".into(), or_empty(props.get("description")));
        match props.get("appVersion") {
            Some(v) => o.insert("appVersion".into(), v.clone()),
            None => o.shift_remove("appVersion"),
        };
        o.insert("owner".into(), or_empty(owner.get("email")));
    }
    Ok(app)
}

pub async fn app_get(cx: &mut Cx) -> Result<()> {
    if let Some(name) = cx.s("name") {
        let mut ep = format!("{POWERAPPS}/providers/Microsoft.PowerApps");
        if cx.bool("asAdmin") {
            ep.push_str(&format!(
                "/scopes/admin/environments/{}",
                eqp(&cx.u("environmentName"))
            ));
        }
        ep.push_str(&format!("/apps/{}?api-version=2016-11-01", eqp(&name)));
        cx.verbose_log(&format!(
            "Retrieving information about Microsoft Power App with name '{name}'..."
        ));
        let r = Request::get(ep)
            .accept("application/json")
            .json_response()
            .data()
            .await?;
        return cx.log(set_app_properties(r)?);
    }
    let dn = cx.u("displayName");
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Power App with displayName '{dn}'..."
    ));
    cx.verbose_log("Retrieving all apps...");
    let o = obj! { "output": "json", "debug": cx.debug(), "verbose": cx.verbose(), "environmentName": ?cx.val("environmentName").cloned(), "asAdmin": ?cx.val("asAdmin").cloned() };
    let (out, _) = m365_core::runner::run_nested(
        "pa app list",
        o.as_object().cloned().unwrap_or_default(),
        true,
    )
    .await?;
    let apps = if out.is_empty() {
        Vec::new()
    } else {
        json::parse(&out)?.as_array().cloned().unwrap_or_default()
    };
    if apps.is_empty() {
        return Err(Error::msg("No apps found."));
    }
    let found = apps.into_iter().find(|a| {
        a.get("properties")
            .and_then(|p| p.get("displayName"))
            .map(to_js_string)
            .map(|d| d.to_lowercase())
            == Some(dn.to_lowercase())
    });
    match found {
        Some(a) => cx.log(set_app_properties(a)?),
        None => Err(Error::msg(format!("No app found with displayName '{dn}'."))),
    }
}

pub async fn app_list(cx: &mut Cx) -> Result<()> {
    let admin = if cx.bool("asAdmin") {
        "/scopes/admin"
    } else {
        ""
    };
    let env = cx
        .s("environmentName")
        .map(|e| format!("/environments/{}", eqp(&e)))
        .unwrap_or_default();
    let mut apps = odata::get_all_items(&format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps{admin}{env}/apps?api-version=2017-08-01"
    ))
    .await?;
    for a in apps.iter_mut() {
        let d = a
            .get("properties")
            .ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'displayName')")
            })?
            .get("displayName")
            .cloned();
        if let Some(o) = a.as_object_mut() {
            match d {
                Some(d) => o.insert("displayName".into(), d),
                None => o.shift_remove("displayName"),
            };
        }
    }
    cx.log(Value::Array(apps))
}

pub async fn app_owner_set(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("userId")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Setting new owner {who} for Power Apps app {}...",
        cx.u("appName")
    ));
    let uid = match cx.s("userId") {
        Some(u) => u,
        None => entra::user::id_by_upn(&cx.u("userName")).await?,
    };
    Request::post(format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/scopes/admin/environments/{}/apps/{}/modifyAppOwner?api-version=2022-11-01",
        cx.u("environmentName"),
        cx.u("appName")
    ))
    .accept("application/json")
    .header("Content-Type", "application/json")
    .json_response()
    .json(obj! { "roleForOldAppOwner": ?cx.val("roleForOldAppOwner").cloned(), "newAppOwner": uid })
    .send()
    .await?;
    Ok(())
}

fn principal_label(cx: &Cx) -> String {
    cx.s("userId")
        .or_else(|| cx.s("userName"))
        .or_else(|| cx.s("groupId"))
        .or_else(|| cx.s("groupName"))
        .unwrap_or_else(|| match cx.val("tenant") {
            // `(tenant && 'everyone')`
            Some(v) if truthy(v) => "everyone".into(),
            Some(v) => to_js_string(v),
            None => "undefined".into(),
        })
}

async fn principal_id(cx: &Cx, tenant_prefix: bool) -> Result<String> {
    if let Some(g) = cx.s("groupId") {
        return Ok(g);
    }
    if let Some(u) = cx.s("userId") {
        return Ok(u);
    }
    if let Some(n) = cx.s("groupName") {
        return Ok(u(&entra::group::by_display_name(&n, None).await?, "id"));
    }
    if let Some(n) = cx.s("userName") {
        return entra::user::id_by_upn(&n).await;
    }
    let t = auth::jwt::tenant_id(&auth::default_access_token().unwrap_or_default());
    Ok(if tenant_prefix {
        format!("tenant-{t}")
    } else {
        t
    })
}

fn modify_url(cx: &Cx) -> String {
    let admin = if cx.bool("asAdmin") {
        format!("scopes/admin/environments/{}/", cx.u("environmentName"))
    } else {
        String::new()
    };
    format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/{admin}apps/{}/modifyPermissions?api-version=2022-11-01",
        cx.u("appName")
    )
}

pub async fn app_permission_ensure(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Assigning/updating permissions for '{}' to the Power Apps app '{}'...",
        principal_label(cx),
        cx.u("appName")
    ));
    let pid = principal_id(cx, false).await?;
    let ty = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        "User"
    } else if cx.s("groupId").is_some() || cx.s("groupName").is_some() {
        "Group"
    } else {
        "Tenant"
    };
    let data = obj! { "put": [obj! { "properties": obj! {
        "principal": obj! { "id": pid, "type": ty },
        "NotifyShareTargetOption": if cx.bool("sendInvitationMail") { "Notify" } else { "DoNotNotify" },
        "roleName": ?cx.val("roleName").cloned(),
    } }] };
    Request::post(modify_url(cx))
        .accept("application/json")
        .json_response()
        .json(data)
        .send()
        .await?;
    Ok(())
}

pub async fn app_permission_list(cx: &mut Cx) -> Result<()> {
    let app = cx.u("appName");
    let role = cx.s("roleName");
    cx.verbose_log(&format!(
        "Retrieving permissions for app {app}{}",
        role.as_ref()
            .map(|r| format!(" with role name {r}"))
            .unwrap_or_default()
    ));
    let admin = if cx.bool("asAdmin") {
        "/scopes/admin"
    } else {
        ""
    };
    let env = cx
        .s("environmentName")
        .map(|e| format!("/environments/{}", eqp(&e)))
        .unwrap_or_default();
    let mut perms = odata::get_all_items(&format!("{POWERAPPS}/providers/Microsoft.PowerApps{admin}{env}/apps/{app}/permissions?api-version=2022-11-01")).await?;
    if let Some(r) = &role {
        perms.retain(|p| {
            p.get("properties")
                .and_then(|x| x.get("roleName"))
                .map(to_js_string)
                .as_deref()
                == Some(r.as_str())
        });
    }
    if cx.str("output").as_deref() != Some("json") {
        for p in perms.iter_mut() {
            let props = p.get("properties").cloned().unwrap_or(Value::Null);
            let principal = props
                .get("principal")
                .cloned()
                .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'id')"))?;
            if let Some(o) = p.as_object_mut() {
                for (k, v) in [
                    ("roleName", props.get("roleName")),
                    ("principalId", principal.get("id")),
                    ("principalType", principal.get("type")),
                ] {
                    match v {
                        Some(v) => o.insert(k.into(), v.clone()),
                        None => o.shift_remove(k),
                    };
                }
            }
        }
    }
    cx.log(Value::Array(perms))
}

pub async fn app_permission_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the permissions of '{}' from the Power App '{}'?",
            principal_label(cx),
            cx.u("appName")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing permissions for '{}' for the Power Apps app {}...",
        principal_label(cx),
        cx.u("appName")
    ));
    let pid = principal_id(cx, true).await?;
    Request::post(modify_url(cx))
        .accept("application/json")
        .json_response()
        .json(obj! { "delete": [obj! { "id": pid }] })
        .send()
        .await?;
    Ok(())
}

pub async fn app_remove(cx: &mut Cx) -> Result<()> {
    let name = cx.u("name");
    cx.verbose_log(&format!("Removing Microsoft Power App {name}..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the Microsoft Power App {name}?"),
    )? {
        return Ok(());
    }
    let mut ep = format!("{POWERAPPS}/providers/Microsoft.PowerApps");
    if cx.bool("asAdmin") {
        ep.push_str(&format!(
            "/scopes/admin/environments/{}",
            eqp(&cx.u("environmentName"))
        ));
    }
    ep.push_str(&format!("/apps/{}?api-version=2017-08-01", eqp(&name)));
    match Request::delete(ep)
        .accept("application/json")
        .json_response()
        .send()
        .await
    {
        Err(e) if e.status() == Some(403) => {
            Err(Error::msg(format!("App '{name}' does not exist")))
        }
        other => other.map(|_| ()),
    }
}

pub async fn connector_export(cx: &mut Cx) -> Result<()> {
    let (name, env) = (cx.u("name"), cx.u("environmentName"));
    let folder = util::resolve(
        &std::path::Path::new(&cx.s("outputFolder").unwrap_or_else(|| ".".into()))
            .join(&name)
            .to_string_lossy(),
    );
    cx.verbose_log("Downloading connector...");
    let c = Request::get(format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/apis/{}?api-version=2016-11-01&$filter=environment%20eq%20%27{}%27%20and%20IsCustomApi%20eq%20%27True%27",
        eqp(&name),
        eqp(&env)
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let Some(props) = c.get("properties").filter(|p| truthy(p)).cloned() else {
        return Err(Error::msg(
            "Properties not present in the api registration information.",
        ));
    };
    cx.verbose_log(&format!("Creating output folder {folder}..."));
    std::fs::create_dir(&folder)
        .map_err(|e| Error::js(util::node_fs_error(&e, "mkdir", &folder)))?;
    let write = |file: &str, data: &[u8]| -> Result<()> {
        let p = std::path::Path::new(&folder).join(file);
        std::fs::write(&p, data)
            .map_err(|e| Error::js(util::node_fs_error(&e, "open", &p.to_string_lossy())))
    };
    let settings = obj! {
        "apiDefinition": "apiDefinition.swagger.json",
        "apiProperties": "apiProperties.json",
        "connectorId": name.as_str(),
        "environment": env.as_str(),
        "icon": "icon.png",
        "powerAppsApiVersion": "2016-11-01",
        "powerAppsUrl": "https://api.powerapps.com",
    };
    cx.verbose_log("Exporting settings...");
    write(
        "settings.json",
        json::stringify_pretty(&settings, "  ").as_bytes(),
    )?;
    let mut kept = Obj::new();
    if let Some(o) = props.as_object() {
        for (k, v) in o {
            if [
                "connectionParameters",
                "iconBrandColor",
                "capabilities",
                "policyTemplateInstances",
            ]
            .contains(&k.as_str())
            {
                kept.insert(k.clone(), v.clone());
            }
        }
    }
    cx.verbose_log("Exporting API properties...");
    write(
        "apiProperties.json",
        json::stringify_pretty(&obj! { "properties": Value::Object(kept) }, "  ").as_bytes(),
    )?;
    let mut swagger = String::new();
    match props
        .get("apiDefinitions")
        .and_then(|a| a.get("originalSwaggerUrl"))
        .filter(|u| truthy(u))
        .map(to_js_string)
    {
        Some(url) => {
            cx.verbose_log(&format!("Downloading swagger from {url}..."));
            let r = Request::get(url).anonymous().data().await?;
            swagger = match r {
                Value::String(s) => s,
                Value::Null => String::new(),
                other => to_js_string(&other),
            };
        }
        None => cx.debug_log("originalSwaggerUrl not set. Skipping"),
    }
    if !swagger.is_empty() {
        cx.debug_log("Downloaded swagger");
        cx.debug_log(&swagger);
        cx.verbose_log("Exporting swagger...");
        write("apiDefinition.swagger.json", swagger.as_bytes())?;
    }
    let mut icon: Vec<u8> = Vec::new();
    match props.get("iconUri").filter(|u| truthy(u)).map(to_js_string) {
        Some(url) => {
            cx.verbose_log(&format!("Downloading icon from {url}..."));
            icon = Request::get(url)
                .anonymous()
                .bytes_response()
                .send()
                .await?
                .bytes;
        }
        None => cx.debug_log("iconUri not set. Skipping"),
    }
    if !icon.is_empty() {
        cx.verbose_log("Exporting icon...");
        write("icon.png", &icon)?;
    } else {
        cx.debug_log("No icon retrieved");
    }
    Ok(())
}

pub async fn connector_list(cx: &mut Cx) -> Result<()> {
    let mut items = odata::get_all_items(&format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/apis?api-version=2016-11-01&$filter=environment%20eq%20%27{}%27%20and%20IsCustomApi%20eq%20%27True%27",
        eqp(&cx.u("environmentName"))
    ))
    .await?;
    copy_display_name(&mut items)?;
    cx.log(Value::Array(items))
}

/// `item.displayName = item.properties.displayName`.
pub fn copy_display_name(items: &mut [Value]) -> Result<()> {
    for i in items.iter_mut() {
        let d = i
            .get("properties")
            .ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'displayName')")
            })?
            .get("displayName")
            .cloned();
        if let Some(o) = i.as_object_mut() {
            match d {
                Some(d) => o.insert("displayName".into(), d),
                None => o.shift_remove("displayName"),
            };
        }
    }
    Ok(())
}

/// Copies environment properties to the top level.
pub fn env_props(mut r: Value) -> Result<Value> {
    let props = r
        .get("properties")
        .cloned()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'displayName')"))?;
    if let Some(o) = r.as_object_mut() {
        for k in [
            "displayName",
            "provisioningState",
            "environmentSku",
            "azureRegionHint",
            "isDefault",
        ] {
            match props.get(k) {
                Some(v) => o.insert(k.into(), v.clone()),
                None => o.shift_remove(k),
            };
        }
    }
    Ok(r)
}

pub async fn environment_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Power Apps environment {}...",
        cx.s("name").unwrap_or_else(|| "default".into())
    ));
    let env = if cx.bool("default") {
        "~default".to_string()
    } else {
        eqp(&cx.u("name"))
    };
    let r = Request::get(format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/environments/{env}?api-version=2016-11-01"
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    cx.log(env_props(r)?)
}

pub async fn environment_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of Microsoft Power Apps environments...");
    let r = Request::get(format!(
        "{POWERAPPS}/providers/Microsoft.PowerApps/environments?api-version=2017-08-01"
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let mut items = r
        .get("value")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'length')"))?;
    copy_display_name(&mut items)?;
    cx.log(Value::Array(items))
}

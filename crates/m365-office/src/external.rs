//! `external *`, `exo *`, `app *` and `adaptivecard send`.

use m365_core::prelude::*;
use m365_core::ut::{entra, entra_app, entra_sp};
use m365_core::util::CheckStatus;

const CONNECTIONS: &str = "https://graph.microsoft.com/v1.0/external/connections";
const INVALID_IDS: [&str; 16] = [
    "None",
    "Directory",
    "Exchange",
    "ExchangeArchive",
    "LinkedIn",
    "Mailbox",
    "OneDriveBusiness",
    "SharePoint",
    "Teams",
    "Yammer",
    "Connectors",
    "TaskFabric",
    "PowerBI",
    "Assistant",
    "TopicEngine",
    "MSFT_All_Connectors",
];

pub fn connection_add_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(!INVALID_IDS.contains(&to_js_string(v).as_str()))
}

pub async fn connection_add(cx: &mut Cx) -> Result<()> {
    let ids: Vec<String> = match cx.str("authorizedAppIds") {
        Some(a) if !a.is_empty() => a.split(',').map(String::from).collect(),
        _ => Vec::new(),
    };
    let data = obj! {
        "id": ?cx.val("id").cloned(),
        "name": ?cx.val("name").cloned(),
        "description": ?cx.val("description").cloned(),
        "configuration": obj! { "authorizedAppIds": ids },
    };
    Request::post(CONNECTIONS)
        .graph_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

struct CheckDef {
    id: &'static str,
    text: &'static str,
    auto: bool,
    required: bool,
}

pub async fn connection_doctor(cx: &mut Cx) -> Result<()> {
    let ux = cx.str("ux").unwrap_or_else(|| "all".into());
    let output = cx.str("output").unwrap_or_else(|| "json".into());
    let show = output == "text";
    let c = |id, text, auto, required| CheckDef {
        id,
        text,
        auto,
        required,
    };
    let mut checks = vec![
        c("loadExternalConnection", "Load connection", true, true),
        c("loadSchema", "Load schema", true, true),
    ];
    if ux == "copilot" || ux == "all" {
        checks.extend([
            c(
                "copilotRequiredSemanticLabels",
                "Required semantic labels",
                true,
                true,
            ),
            c("searchableProperties", "Searchable properties", true, true),
            c("contentIngested", "Items have content ingested", true, true),
            c(
                "enabledForInlineResults",
                "Connection configured for inline results",
                false,
                true,
            ),
            c(
                "itemsHaveActivities",
                "Items have activities recorded",
                false,
                false,
            ),
            c(
                "meaningfulNameAndDescription",
                "Meaningful connection name and description",
                false,
                true,
            ),
        ]);
    }
    if ux == "search" || ux == "all" {
        checks.extend([
            c("semanticLabels", "Semantic labels", true, false),
            c("searchableProperties", "Searchable properties", true, false),
            c("resultType", "Result type", true, false),
            c(
                "contentIngested",
                "Items have content ingested",
                true,
                false,
            ),
            c(
                "itemsHaveActivities",
                "Items have activities recorded",
                false,
                false,
            ),
        ]);
    }
    checks.push(c(
        "urlToItemResolver",
        "urlToItemResolver configured",
        true,
        false,
    ));
    let mut seen = Vec::new();
    checks.retain(|ch| {
        let fresh = !seen.contains(&ch.id);
        seen.push(ch.id);
        fresh
    });
    let id = cx.u("id");
    let mut connection = Value::Null;
    let mut schema = Value::Null;
    let mut results: Vec<Value> = Vec::new();
    for ch in &checks {
        cx.debug_log(&format!("Running check {}...", ch.id));
        let base = obj! { "id": ch.id, "text": ch.text, "type": if ch.required { "required" } else { "recommended" } };
        if !ch.auto {
            if show {
                cx.log(Value::from(util::status(
                    CheckStatus::Information,
                    &format!("{} (manual)", ch.text),
                )))?;
            }
            results.push(planner_merge(base, &obj! { "status": "manual" }));
            continue;
        }
        // (status passed, errorMessage, error, shouldStop)
        let fail = |m: &str| (false, Some(m.to_string()), None::<Value>, false);
        // `schema.properties?.find(...)`: a non-array value throws
        if !matches!(
            schema.get("properties"),
            None | Some(Value::Null) | Some(Value::Array(_))
        ) && [
            "copilotRequiredSemanticLabels",
            "searchableProperties",
            "semanticLabels",
        ]
        .contains(&ch.id)
        {
            return Err(Error::js("schema.properties?.find is not a function"));
        }
        let props = || arr(&schema, "properties").to_vec();
        let (passed, msg, error, stop) = match ch.id {
            "loadExternalConnection" => match Request::get(format!("{CONNECTIONS}/{id}"))
                .graph_json()
                .data()
                .await
            {
                Ok(v) => {
                    connection = v;
                    (true, None, None, false)
                }
                Err(e) => (
                    false,
                    Some("Connection not found".into()),
                    inner_error(&e),
                    true,
                ),
            },
            "loadSchema" => match Request::get(format!("{CONNECTIONS}/{id}/schema"))
                .graph_json()
                .header("prefer", "include-unknown-enum-members")
                .data()
                .await
            {
                Ok(v) => {
                    schema = v;
                    (true, None, None, false)
                }
                Err(e) => (
                    false,
                    Some("Schema not found".into()),
                    inner_error(&e),
                    true,
                ),
            },
            "copilotRequiredSemanticLabels" => {
                let missing = ["title", "url", "iconUrl"].into_iter().find(|l| {
                    !props()
                        .iter()
                        .any(|p| arr(p, "labels").iter().any(|x| to_js_string(x) == *l))
                });
                match missing {
                    Some(l) => fail(&format!("Missing label {l}")),
                    None => (true, None, None, false),
                }
            }
            "searchableProperties" => {
                if props()
                    .iter()
                    .any(|p| p.get("isSearchable").is_some_and(truthy))
                {
                    (true, None, None, false)
                } else {
                    fail("Schema does not have any searchable properties")
                }
            }
            "contentIngested" => content_ingested(&id).await,
            "urlToItemResolver" => {
                let ok = connection
                    .get("activitySettings")
                    .and_then(|a| a.get("urlToItemResolvers"))
                    .and_then(Value::as_array)
                    .is_some_and(|r| r.iter().any(truthy));
                if ok {
                    (true, None, None, false)
                } else {
                    fail("urlToItemResolver is not configured")
                }
            }
            "semanticLabels" => {
                if props().iter().any(|p| arr(p, "labels").iter().any(truthy)) {
                    (true, None, None, false)
                } else {
                    fail("Schema does not have semantic labels")
                }
            }
            _ => {
                let ok = connection
                    .get("searchSettings")
                    .and_then(|a| a.get("searchResultTemplates"))
                    .and_then(Value::as_array)
                    .is_some_and(|r| r.iter().any(truthy));
                if ok {
                    (true, None, None, false)
                } else {
                    fail("Connection has no result types")
                }
            }
        };
        let mut r = Obj::new();
        r.insert("id".into(), Value::from(ch.id));
        if let Some(e) = error {
            r.insert("error".into(), e);
        }
        if let Some(m) = &msg {
            r.insert("errorMessage".into(), Value::from(m.as_str()));
        }
        r.insert(
            "status".into(),
            Value::from(if passed { "passed" } else { "failed" }),
        );
        results.push(planner_merge(base, &Value::Object(r)));
        if passed {
            if show {
                cx.log(Value::from(util::status(CheckStatus::Success, ch.text)))?;
            }
            continue;
        }
        if show {
            if ch.required {
                cx.log(Value::from(util::status(
                    CheckStatus::Failure,
                    &format!(
                        "{}: {}",
                        ch.text,
                        msg.clone().unwrap_or_else(|| "undefined".into())
                    ),
                )))?;
            } else {
                cx.log(Value::from(util::status(CheckStatus::Warning, ch.text)))?;
            }
        }
        if stop {
            break;
        }
    }
    if show || output == "none" {
        return Ok(());
    }
    match output.as_str() {
        "json" => cx.log(Value::Array(results)),
        "md" => {
            let eol = m365_core::term::EOL;
            let opts = m365_core::log::get_state().options;
            let args: Vec<String> = opts
                .iter()
                .filter(|(k, _)| k.as_str() != "output")
                .map(|(k, v)| format!("--{k} \"{}\"", to_js_string(v)))
                .collect();
            let mut out = format!(
                "# {} {}{eol}{eol}Date: {}{eol}{eol}",
                cx.meta.name,
                args.join(" "),
                m365_core::time::locale_date_string(chrono::Local::now())
            );
            if !results.is_empty() {
                out.push_str(&format!(
                    "Check|Type|Status|Error message{eol}:----|:--:|:----:|:------------{eol}"
                ));
                for r in &results {
                    let cells: Vec<String> = ["text", "type", "status", "errorMessage"]
                        .iter()
                        .map(|p| {
                            r.get(*p)
                                .filter(|v| !v.is_null())
                                .map(to_js_string)
                                .unwrap_or_default()
                        })
                        .collect();
                    out.push_str(&format!("{}{eol}", cells.join("|")));
                }
            }
            cx.log_raw(out.trim_end());
            Ok(())
        }
        "csv" => {
            for r in results.iter_mut() {
                if let Some(o) = r.as_object_mut()
                    && !o.contains_key("errorMessage")
                {
                    o.insert("errorMessage".into(), Value::from(""));
                }
            }
            cx.log(Value::Array(results))
        }
        _ => Ok(()),
    }
}

fn planner_merge(a: Value, b: &Value) -> Value {
    let mut a = match a {
        Value::Object(o) => o,
        _ => Obj::new(),
    };
    if let Some(b) = b.as_object() {
        for (k, v) in b {
            a.insert(k.clone(), v.clone());
        }
    }
    Value::Object(a)
}

/// `ex?.response?.data?.error?.innerError?.message` (axios errors here have no `response`).
fn inner_error(_e: &Error) -> Option<Value> {
    None
}

async fn content_ingested(id: &str) -> (bool, Option<String>, Option<Value>, bool) {
    let r: Result<(bool, Option<String>)> = async {
        let body = obj! { "requests": [obj! {
            "entityTypes": ["externalItem"],
            "contentSources": [format!("/external/connections/{id}")],
            "query": obj! { "queryString": "*" },
            "from": 0,
            "size": 1,
        }] };
        let res = Request::post(format!("{GRAPH}/v1.0/search/query")).graph_json().json(body).data().await?;
        let hit = res.get("value").and_then(|v| v.get(0)).map(|v| v.get("hitsContainers").and_then(|h| h.get(0)).and_then(|h| h.get("hits")).and_then(|h| h.get(0)).cloned());
        let hit = match hit {
            // `result.value?.[0].hitsContainers`: a missing first value throws
            None if res.get("value").is_some_and(|v| v.as_array().is_some_and(|a| a.is_empty())) => return Err(Error::js("Cannot read properties of undefined")),
            Some(Some(h)) if truthy(&h) => h,
            _ => return Ok((false, Some("No items found that belong to the connection".into()))),
        };
        let item_id = hit
            .get("resource")
            .and_then(|r| r.get("properties"))
            .and_then(|p| p.get("substrateContentDomainId"))
            .and_then(Value::as_str)
            .and_then(|s| s.split(',').nth(1).map(String::from))
            .filter(|s| !s.is_empty());
        let Some(item_id) = item_id else {
            return Ok((false, Some("Item does not have substrateContentDomainId property or the property is invalid".into())));
        };
        let item = Request::get(format!("{CONNECTIONS}/{id}/items/{item_id}")).graph_json().data().await?;
        if !item.get("content").and_then(|c| c.get("value")).is_some_and(truthy) {
            return Ok((false, Some("Item does not have content or content is empty".into())));
        }
        Ok((true, None))
    }
    .await;
    match r {
        Ok((p, m)) => (p, m, None, false),
        Err(_) => (
            false,
            Some("Error while checking if content is ingested".into()),
            None,
            false,
        ),
    }
}

pub async fn connection_get(cx: &mut Cx) -> Result<()> {
    let url = match cx.s("id") {
        Some(i) => format!("{CONNECTIONS}/{}", eqp(&i)),
        None => format!("{CONNECTIONS}?$filter=name eq '{}'", eqp(&cx.u("name"))),
    };
    let mut res = Request::get(url).graph_json().data().await?;
    if let Some(n) = cx.s("name") {
        match arr(&res, "value").first() {
            Some(v) => res = v.clone(),
            None => {
                return Err(Error::msg(format!(
                    "External connection with name '{n}' not found"
                )));
            }
        }
    }
    cx.log(res)
}

pub async fn connection_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(odata::get_all_items(CONNECTIONS).await?))
}

pub async fn connection_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the external connection '{what}'?"),
    )? {
        return Ok(());
    }
    let id = match cx.s("id") {
        Some(i) => i,
        None => {
            let n = cx.u("name");
            let r = Request::get(format!(
                "{CONNECTIONS}?$filter=name eq '{}'&$select=id",
                eqp(&n)
            ))
            .graph_json()
            .data()
            .await?;
            let v = arr(&r, "value");
            match v.len() {
                1 => u(&v[0], "id"),
                0 => return Err(Error::msg("The specified connection does not exist")),
                _ => u(
                    &cx.handle_multiple_results(
                        &format!("Multiple external connections with name {n} found."),
                        &util::array_to_hash_table("id", v),
                    )?,
                    "id",
                ),
            }
        }
    };
    Request::delete(format!("{CONNECTIONS}/{}", eqp(&id)))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn connection_schema_add(cx: &mut Cx) -> Result<()> {
    let cid = cx.u("externalConnectionId");
    cx.verbose_log(&format!(
        "Adding schema to external connection with id {cid}"
    ));
    let res = Request::patch(format!("{CONNECTIONS}/{cid}/schema"))
        .graph_json()
        .text(cx.u("schema"))
        .send()
        .await?;
    let location = res.header("location").map(String::from);
    match &location {
        Some(l) => cx.log(Value::from(l.as_str()))?,
        None => cx.log_undefined()?,
    }
    if !cx.bool("wait") {
        return Ok(());
    }
    let location = location.unwrap_or_else(|| "undefined".into());
    loop {
        cx.verbose_log("Waiting 60 seconds...");
        util::sleep_ms(60_000).await;
        cx.debug_log("Checking schema operation status...");
        let op = Request::get(location.clone()).graph_json().data().await?;
        let status = op.get("status").cloned().unwrap_or(Value::Null);
        cx.verbose_log(&format!(
            "Schema operation status: {}",
            if status.is_null() {
                "undefined".into()
            } else {
                to_js_string(&status)
            }
        ));
        if status == "failed" {
            return Err(Error::msg(format!(
                "Provisioning schema failed: {}",
                op.get("error")
                    .and_then(|e| e.get("message"))
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into())
            )));
        }
        if status != "inprogress" {
            return Ok(());
        }
    }
}

pub async fn connection_urltoitemresolver_add(cx: &mut Cx) -> Result<()> {
    let base: Vec<String> = cx
        .u("baseUrls")
        .split(',')
        .map(|b| b.trim().to_string())
        .collect();
    let resolver = obj! {
        "itemId": ?cx.val("itemId").cloned(),
        "priority": ?cx.val("priority").cloned(),
        "urlMatchInfo": obj! { "baseUrls": base, "urlPattern": ?cx.val("urlPattern").cloned() },
        "@odata.type": "#microsoft.graph.externalConnectors.itemIdResolver",
    };
    Request::patch(format!("{CONNECTIONS}/{}", cx.u("externalConnectionId")))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "activitySettings": obj! { "urlToItemResolvers": [resolver] } })
        .send()
        .await?;
    Ok(())
}

pub async fn item_add(cx: &mut Cx) -> Result<()> {
    let acls: Vec<Value> = cx
        .u("acls")
        .split(';')
        .map(|a| {
            let p: Vec<&str> = a.split(',').collect();
            obj! { "accessType": ?p.first().copied(), "type": ?p.get(1).copied(), "value": ?p.get(2).copied() }
        })
        .collect();
    let names: Vec<String> = cx.opts.keys().cloned().collect();
    for name in names {
        if let Some(i) = name.find("@odata") {
            let base = name[..i].to_string();
            if let Some(v) = cx.opts.get(&base).filter(|v| truthy(v)).map(to_js_string) {
                cx.opts.insert(
                    base,
                    Value::from(v.split(";#").map(String::from).collect::<Vec<_>>()),
                );
            }
        }
    }
    let mut props = obj! {};
    cx.add_unknown_options(&mut props);
    let body = obj! {
        "id": ?cx.val("id").cloned(),
        "content": obj! { "value": ?cx.val("content").cloned(), "type": cx.val("contentType").cloned().unwrap_or(Value::from("text")) },
        "acl": acls,
        "properties": props,
    };
    let mut item = Request::put(format!(
        "{CONNECTIONS}/{}/items/{}",
        cx.u("externalConnectionId"),
        cx.u("id")
    ))
    .graph_json()
    .header("content-type", "application/json")
    .json(body)
    .data()
    .await?;
    let out = cx.str("output").unwrap_or_default();
    if out == "csv" || out == "md" {
        let props = item
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?;
        if let Some(o) = item.as_object_mut() {
            for (k, v) in props {
                let v = match v {
                    Value::Array(a) => {
                        Value::from(a.iter().map(to_js_string).collect::<Vec<_>>().join(", "))
                    }
                    other => other,
                };
                o.insert(k, v);
            }
        }
    }
    cx.log(item)
}

// ---- exo -----------------------------------------------------------------------

pub async fn exo_approleassignment_add(cx: &mut Cx) -> Result<()> {
    let role = match cx.s("roleDefinitionId") {
        Some(r) => r,
        None => {
            let n = cx.u("roleDefinitionName");
            cx.verbose_log(&format!("Retrieving role definition by its name '{n}'"));
            u(
                &entra::exchange_role_definition_by_display_name(&n, None).await?,
                "id",
            )
        }
    };
    let principal = match cx.s("principalId") {
        Some(p) => format!("/ServicePrincipals/{p}"),
        None => {
            let n = cx.u("principalName");
            cx.verbose_log(&format!("Retrieving service principal by its name '{n}'"));
            format!(
                "/ServicePrincipals/{}",
                u(&entra_sp::get_by_app_name(&n, Some("id")).await?, "id")
            )
        }
    };
    let scope = cx.u("scope");
    let directory = if scope == "custom" {
        Value::Null
    } else {
        let (prefix, id) = match scope.as_str() {
            "user" => (
                "/users/",
                match (cx.s("userId"), cx.s("userName")) {
                    (Some(i), _) => i,
                    (None, Some(n)) => entra::user::id_by_upn(&n).await?,
                    _ => String::new(),
                },
            ),
            "group" => (
                "/groups/",
                match (cx.s("groupId"), cx.s("groupName")) {
                    (Some(i), _) => i,
                    (None, Some(n)) => entra::group::id_by_display_name(&n).await?,
                    _ => String::new(),
                },
            ),
            "administrativeUnit" => (
                "/administrativeUnits/",
                match (cx.s("administrativeUnitId"), cx.s("administrativeUnitName")) {
                    (Some(i), _) => i,
                    (None, Some(n)) => u(
                        &entra::administrative_unit_by_display_name(&n, None).await?,
                        "id",
                    ),
                    _ => String::new(),
                },
            ),
            _ => ("/", String::new()),
        };
        Value::from(format!("{prefix}{id}"))
    };
    let app_scope = if scope != "custom" {
        Value::Null
    } else if let Some(i) = cx.s("customAppScopeId") {
        Value::from(i)
    } else {
        let n = cx.u("customAppScopeName");
        cx.verbose_log(&format!(
            "Retrieving custom application scope by its name '{n}'"
        ));
        let items = odata::get_all_items(&format!("{GRAPH}/beta/roleManagement/exchange/customAppScopes?$filter=displayName eq '{}'&$select=id", eqp(&n))).await?;
        let s = match items.len() {
            0 => {
                return Err(Error::js(format!(
                    "The specified custom application scope '{n}' does not exist."
                )));
            }
            1 => items.into_iter().next().unwrap_or(Value::Null),
            _ => cx.handle_multiple_results(
                &format!("Multiple custom application scopes with name '{n}' found."),
                &util::array_to_hash_table("id", &items),
            )?,
        };
        s.get("id").cloned().unwrap_or(Value::Null)
    };
    let data = obj! { "roleDefinitionId": role, "principalId": principal, "directoryScopeId": directory, "appScopeId": app_scope };
    let r = Request::post(format!(
        "{GRAPH}/beta/roleManagement/exchange/roleAssignments"
    ))
    .graph_json()
    .json(data)
    .data()
    .await?;
    cx.log(r)
}

// ---- app -----------------------------------------------------------------------

fn app_id(cx: &Cx) -> String {
    cx.app_id.clone().unwrap_or_else(|| "undefined".into())
}

pub async fn app_get(cx: &mut Cx) -> Result<()> {
    cx.log(entra_app::get_by_app_id(&app_id(cx), None).await?)
}

pub async fn app_open(cx: &mut Cx) -> Result<()> {
    let prefix = if cx.opt_bool("preview") == Some(true) {
        "preview."
    } else {
        ""
    };
    let url = format!(
        "https://{prefix}portal.azure.com/#blade/Microsoft_AAD_RegisteredApps/ApplicationMenuBlade/Overview/appId/{}/isMSAApp/",
        app_id(cx)
    );
    if !config::get_bool(config::names::AUTO_OPEN_LINKS_IN_BROWSER, false) {
        return cx.log(Value::from(format!(
            "Use a web browser to open the page {url}"
        )));
    }
    cx.log(Value::from(format!(
        "Opening the following page in your browser: {url}"
    )))?;
    let _ = util::open_browser(&url);
    Ok(())
}

pub async fn app_permission_add(cx: &mut Cx) -> Result<()> {
    let id = app_id(cx);
    let apps = odata::get_all_items(&format!("{GRAPH}/v1.0/myorganization/applications?$filter=appId eq '{}'&$select=id,requiredResourceAccess", eqp(&id))).await?;
    let Some(mut app) = apps.into_iter().next() else {
        return Err(Error::msg(format!(
            "App with id {id} not found in Microsoft Entra ID."
        )));
    };
    let sps = odata::get_all_items(&format!("{GRAPH}/v1.0/myorganization/servicePrincipals?$select=appId,appRoles,id,oauth2PermissionScopes,servicePrincipalNames")).await?;
    let mut existing: Vec<Value> = arr(&app, "requiredResourceAccess").to_vec();
    // resourceId, resourceAccess, scope
    let mut perms: Vec<(String, Vec<Value>, Vec<String>)> = Vec::new();
    for (opt, kind) in [
        ("delegatedPermissions", "Scope"),
        ("applicationPermissions", "Role"),
    ] {
        let Some(apis) = cx.s(opt) else { continue };
        let mut resolved: Vec<Value> = Vec::new();
        for api in apis.split(' ').map(str::trim) {
            let pos = api.rfind('/');
            let name = pos.map(|p| &api[p + 1..]).unwrap_or(api);
            let sp_name = pos.map(|p| &api[..p]).unwrap_or("");
            cx.verbose_log(&format!("Resolving {api}..."));
            cx.verbose_log(&format!("Permission name: {name}"));
            cx.verbose_log(&format!("Service principal name: {sp_name}"));
            let with_slash = format!("{sp_name}/");
            let Some(sp) = sps.iter().find(|s| {
                arr(s, "servicePrincipalNames")
                    .iter()
                    .any(|n| n == sp_name || *n == with_slash.as_str())
            }) else {
                return Err(Error::msg(format!("Service principal {sp_name} not found")));
            };
            let list = if kind == "Scope" {
                "oauth2PermissionScopes"
            } else {
                "appRoles"
            };
            let Some(perm) = arr(sp, list).iter().find(|p| s(p, "value") == name) else {
                return Err(Error::msg(format!(
                    "Permission {name} for service principal {sp_name} not found"
                )));
            };
            let access = obj! { "id": ?perm.get("id").cloned(), "type": kind };
            let rid = sp.get("appId").cloned().unwrap_or(Value::Null);
            match resolved
                .iter_mut()
                .find(|r| r.get("resourceAppId") == Some(&rid))
            {
                Some(r) => {
                    if let Some(Value::Array(a)) = r.get_mut("resourceAccess") {
                        a.push(access.clone());
                    }
                }
                None => {
                    resolved.push(obj! { "resourceAppId": rid, "resourceAccess": [access.clone()] })
                }
            }
            let spid = u(sp, "id");
            let idx = match perms.iter().position(|p| p.0 == spid) {
                Some(i) => i,
                None => {
                    perms.push((spid, Vec::new(), Vec::new()));
                    perms.len() - 1
                }
            };
            let pv = s(perm, "value").to_string();
            if kind == "Scope" && !pv.is_empty() && !perms[idx].2.contains(&pv) {
                perms[idx].2.push(pv);
            }
            if !perms[idx].1.iter().any(|r| r.get("id") == access.get("id")) {
                perms[idx].1.push(access);
            }
        }
        for r in resolved {
            match existing
                .iter_mut()
                .find(|e| e.get("resourceAppId") == r.get("resourceAppId"))
            {
                Some(e) => {
                    for a in arr(&r, "resourceAccess") {
                        if let Some(Value::Array(list)) = e.get_mut("resourceAccess")
                            && !list.iter().any(|x| x.get("id") == a.get("id"))
                        {
                            list.push(a.clone());
                        }
                    }
                }
                None => existing.push(r),
            }
        }
    }
    if let Some(o) = app.as_object_mut() {
        o.insert(
            "requiredResourceAccess".into(),
            Value::Array(existing.clone()),
        );
    }
    Request::patch(format!(
        "{GRAPH}/v1.0/myorganization/applications/{}",
        u(&app, "id")
    ))
    .graph_json()
    .json(obj! { "requiredResourceAccess": existing })
    .send()
    .await?;
    if cx.bool("grantAdminConsent") {
        let sp = match sps
            .iter()
            .find(|s| s.get("appId").map(to_js_string).as_deref() == Some(id.as_str()))
        {
            Some(s) => s.clone(),
            None => {
                cx.verbose_log(&format!("Creating service principal for app {id}..."));
                entra_sp::create(&id).await?
            }
        };
        let spid = u(&sp, "id");
        for (rid, access, scope) in &perms {
            if !scope.is_empty() {
                cx.verbose_log(&format!("Granting consent for delegated permission(s) with resourceId {rid} and scope(s) {}", scope.join(" ")));
                Request::post(format!("{GRAPH}/v1.0/myorganization/oauth2PermissionGrants"))
                    .graph_json()
                    .json(obj! { "clientId": spid.as_str(), "consentType": "AllPrincipals", "principalId": Value::Null, "resourceId": rid.as_str(), "scope": scope.join(" ") })
                    .send()
                    .await?;
            }
            for a in access.iter().filter(|a| s(a, "type") == "Role") {
                cx.verbose_log(&format!("Granting consent for application permission with resourceId {rid} and appRoleId {}", u(a, "id")));
                Request::post(format!("{GRAPH}/v1.0/myorganization/servicePrincipals/{spid}/appRoleAssignments"))
                    .header("content-type", "application/json")
                    .json(obj! { "appRoleId": ?a.get("id").cloned(), "principalId": spid.as_str(), "resourceId": rid.as_str() })
                    .json_response()
                    .send()
                    .await?;
            }
        }
    }
    Ok(())
}

/// `getServicePrincipal` of `app permission list`.
async fn sp_info(
    cx: &Cx,
    app_id: Option<&str>,
    id: Option<&str>,
    definitions: bool,
) -> Result<Option<Value>> {
    cx.verbose_log(&format!(
        "Retrieving service principal {}",
        app_id.or(id).unwrap_or("undefined")
    ));
    let lookup = match app_id {
        Some(a) => format!("?$filter=appId eq '{a}'&"),
        None => format!("/{}?", id.unwrap_or("undefined")),
    };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/servicePrincipals{lookup}$select=appId,id,displayName"
    ))
    .graph_json()
    .data()
    .await?;
    let mut sp = if app_id.is_some() {
        match arr(&r, "value").first() {
            Some(s) => s.clone(),
            None => return Ok(None),
        }
    } else {
        if !truthy(&r) {
            return Ok(None);
        }
        r
    };
    let spid = u(&sp, "id");
    cx.verbose_log(&format!(
        "Retrieving permissions for service principal {spid}..."
    ));
    let (a, b) = if definitions {
        ("oauth2PermissionScopes", "appRoles")
    } else {
        ("appRoleAssignments", "oauth2PermissionGrants")
    };
    let ra = Request::get(format!("{GRAPH}/v1.0/servicePrincipals/{spid}/{a}"))
        .graph_json()
        .data()
        .await?;
    let rb = Request::get(format!("{GRAPH}/v1.0/servicePrincipals/{spid}/{b}"))
        .graph_json()
        .data()
        .await?;
    if let Some(o) = sp.as_object_mut() {
        o.insert(a.into(), ra.get("value").cloned().unwrap_or(Value::Null));
        o.insert(b.into(), rb.get("value").cloned().unwrap_or(Value::Null));
    }
    Ok(Some(sp))
}

pub async fn app_permission_list(cx: &mut Cx) -> Result<()> {
    let id = app_id(cx);
    let perms = match sp_info(cx, Some(&id), None, false).await? {
        Some(sp) => {
            cx.verbose_log("Resolving permissions for the service principal...");
            let mut out: Vec<(String, String, &'static str)> = Vec::new();
            let mut lookup = Obj::new();
            let mut resolve: Vec<String> = Vec::new();
            for a in arr(&sp, "appRoleAssignments") {
                let rid = u(a, "resourceId");
                lookup.insert(
                    rid.clone(),
                    a.get("resourceDisplayName").cloned().unwrap_or(Value::Null),
                );
                if !resolve.contains(&rid) {
                    resolve.push(rid);
                }
                out.push((
                    u(a, "resourceDisplayName"),
                    u(a, "appRoleId"),
                    "Application",
                ));
            }
            for g in arr(&sp, "oauth2PermissionGrants") {
                let rid = u(g, "resourceId");
                let res = lookup
                    .get(&rid)
                    .filter(|v| !v.is_null())
                    .map(to_js_string)
                    .unwrap_or_else(|| rid.clone());
                if res == rid && !resolve.contains(&rid) {
                    resolve.push(rid.clone());
                }
                for scope in u(g, "scope").split(' ') {
                    out.push((res.clone(), scope.to_string(), "Delegated"));
                }
            }
            let mut resolved = Vec::new();
            for r in &resolve {
                resolved.push(
                    sp_info(cx, None, Some(r), true)
                        .await?
                        .unwrap_or(Value::Null),
                );
            }
            for s in &resolved {
                let sid = u(s, "id");
                let dn = u(s, "displayName");
                for p in out.iter_mut() {
                    if p.0 == sid {
                        p.0 = dn.clone();
                    }
                    if p.0 == dn
                        && p.2 == "Application"
                        && let Some(v) = arr(s, "appRoles")
                            .iter()
                            .find(|r| u(r, "id") == p.1)
                            .and_then(|r| r.get("value"))
                            .filter(|v| !v.is_null())
                    {
                        p.1 = to_js_string(v);
                    }
                }
            }
            out.into_iter()
                .map(|(r, p, t)| obj! { "resource": r, "permission": p, "type": t })
                .collect::<Vec<_>>()
        }
        None => {
            cx.verbose_log(&format!(
                "Retrieving the Entra application registration with appId '{id}'"
            ));
            let app = entra_app::get_by_app_id(&id, None).await?;
            let rra = arr(&app, "requiredResourceAccess").to_vec();
            let mut sps = Vec::new();
            for r in &rra {
                sps.push(sp_info(cx, Some(&u(r, "resourceAppId")), None, true).await?);
            }
            let mut out = Vec::new();
            for r in &rra {
                let rid = u(r, "resourceAppId");
                let sp = sps.iter().flatten().find(|s| u(s, "appId") == rid);
                let name = sp
                    .and_then(|s| s.get("displayName"))
                    .filter(|v| !v.is_null())
                    .map(to_js_string)
                    .unwrap_or_else(|| rid.clone());
                for p in arr(r, "resourceAccess") {
                    let pid = u(p, "id");
                    let ty = u(p, "type");
                    let perm = match sp {
                        None => pid.clone(),
                        Some(s) => {
                            let list = match ty.as_str() {
                                "Role" => "appRoles",
                                "Scope" => "oauth2PermissionScopes",
                                _ => "",
                            };
                            arr(s, list)
                                .iter()
                                .find(|x| u(x, "id") == pid)
                                .and_then(|x| x.get("value"))
                                .filter(|v| !v.is_null())
                                .map(to_js_string)
                                .unwrap_or(pid.clone())
                        }
                    };
                    out.push(obj! { "resource": name.as_str(), "permission": perm, "type": if ty == "Role" { "Application" } else { "Delegated" } });
                }
            }
            out
        }
    };
    cx.log(Value::Array(perms))
}

// ---- adaptivecard --------------------------------------------------------------

pub async fn adaptivecard_send(cx: &mut Cx) -> Result<()> {
    let unknown = m365_core::ut::options::unknown_options(&cx.opts, cx.meta);
    let names: Vec<String> = unknown.keys().cloned().collect();
    let card = if let Some(c) = cx.s("card") {
        let card = json::js_parse(&c).map_err(Error::msg)?;
        let data = if let Some(d) = cx.s("cardData") {
            Some(json::js_parse(&d).map_err(Error::msg)?)
        } else if !names.is_empty() {
            Some(Value::Object(unknown.clone()))
        } else if cx.s("title").is_none()
            && cx.s("description").is_none()
            && cx.s("imageUrl").is_none()
            && cx.s("actionUrl").is_none()
        {
            None
        } else {
            Some(
                obj! { "title": ?cx.s("title"), "description": ?cx.s("description"), "imageUrl": ?cx.s("imageUrl"), "actionUrl": ?cx.s("actionUrl") },
            )
        };
        match data {
            Some(d) => crate::template::expand(&card, &d),
            None => card,
        }
    } else {
        let mut body = Vec::new();
        if let Some(t) = cx.s("title") {
            body.push(
                obj! { "type": "TextBlock", "size": "Medium", "weight": "Bolder", "text": t },
            );
        }
        if let Some(i) = cx.s("imageUrl") {
            body.push(obj! { "type": "Image", "url": i, "size": "Stretch" });
        }
        if let Some(d) = cx.s("description") {
            body.push(obj! { "type": "TextBlock", "text": d, "wrap": true });
        }
        if !names.is_empty() {
            let facts: Vec<Value> = names
                .iter()
                .map(|n| obj! { "title": format!("{n}:"), "value": ?unknown.get(n).cloned() })
                .collect();
            body.push(obj! { "type": "FactSet", "facts": facts });
        }
        let mut card = obj! { "type": "AdaptiveCard", "$schema": "http://adaptivecards.io/schemas/adaptive-card.json", "version": "1.2", "body": body };
        if let Some(a) = cx.s("actionUrl") {
            card.as_object_mut().map(|o| {
                o.insert(
                    "actions".into(),
                    Value::Array(vec![
                        obj! { "type": "Action.OpenUrl", "title": "View", "url": a },
                    ]),
                )
            });
        }
        card
    };
    let res = Request::post(cx.u("url"))
        .anonymous()
        .header("content-type", "application/json")
        .json(obj! { "type": "message", "attachments": [obj! { "contentType": "application/vnd.microsoft.card.adaptive", "content": card }] })
        .json_response()
        .data()
        .await?;
    if let Value::String(s) = &res {
        if s.is_empty() {
            return Ok(());
        }
        if s.contains("failed") {
            return Err(Error::Thrown(res.clone()));
        }
        return cx.log(res.clone());
    }
    Ok(())
}

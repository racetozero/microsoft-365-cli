//! `entra approleassignment *` and `entra enterpriseapp *`.

use m365_core::prelude::*;
use m365_core::term::EOL;

fn resource_filter(resource: &str) -> String {
    let mut r = eqp(resource);
    match resource.to_lowercase().as_str() {
        "sharepoint" => r = "Office 365 SharePoint Online".into(),
        "intune" => r = "Microsoft Intune API".into(),
        "exchange" => r = "Office 365 Exchange Online".into(),
        _ => {}
    }
    let mut f = format!("$filter=(displayName eq '{r}' or startswith(displayName,'{r}'))");
    if m365_core::validation::is_valid_guid(&r) {
        f.push_str(&format!(" or appId eq '{r}' or id eq '{r}'"));
    }
    f
}

fn scopes(cx: &Cx) -> Vec<String> {
    cx.u("scopes").split(',').map(str::to_string).collect()
}

fn missing_scope(cx: &Cx, scope: &str, roles: &[String]) -> Error {
    let available: String = roles.iter().map(|r| format!("{EOL}{r}")).collect();
    Error::msg(format!(
        "The scope value '{scope}' you have specified does not exist for {}. {EOL}Available scopes (application permissions) are: {available}",
        cx.u("resource")
    ))
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let filter = if let Some(a) = cx.s("appId") {
        format!("$filter=appId eq '{}'", eqp(&a))
    } else if let Some(o) = cx.s("appObjectId") {
        format!("$filter=id eq '{}'", eqp(&o))
    } else {
        format!("$filter=displayName eq '{}'", eqp(&cx.u("appDisplayName")))
    };
    let sps = Request::get(format!("{GRAPH}/v1.0/servicePrincipals?{filter}"))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    let vals = arr(&sps, "value");
    let object_id = match vals.len() {
        0 => return Err(Error::msg("The specified service principal doesn't exist")),
        1 => s(&vals[0], "id").to_string(),
        _ => s(
            &cx.handle_multiple_results(
                "Multiple service principal found.",
                &util::array_to_hash_table("id", vals),
            )?,
            "id",
        )
        .to_string(),
    };
    let res = Request::get(format!(
        "{GRAPH}/v1.0/servicePrincipals?{}",
        resource_filter(&cx.u("resource"))
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let mut found: Vec<(String, String, String)> = Vec::new();
    for sp in arr(&res, "value") {
        for role in arr(sp, "appRoles") {
            found.push((
                s(sp, "id").to_string(),
                s(role, "id").to_string(),
                s(role, "value").to_string(),
            ));
        }
    }
    if found.is_empty() {
        return Err(Error::msg(format!(
            "The resource '{}' does not have any application permissions available.",
            cx.u("resource")
        )));
    }
    let mut roles = Vec::new();
    for scope in scopes(cx) {
        match found
            .iter()
            .find(|r| r.2.to_lowercase() == scope.to_lowercase().trim())
        {
            Some(r) => roles.push(r.clone()),
            None => {
                return Err(missing_scope(
                    cx,
                    &scope,
                    &found.iter().map(|r| r.2.clone()).collect::<Vec<_>>(),
                ));
            }
        }
    }
    let mut added = Vec::new();
    for (resource_id, role_id, _) in roles {
        added.push(
            Request::post(format!("{GRAPH}/v1.0/servicePrincipals/{object_id}/appRoleAssignments"))
                .header("Content-Type", "application/json")
                .json(obj! { "appRoleId": role_id, "principalId": object_id.as_str(), "resourceId": resource_id })
                .json_response()
                .data()
                .await?,
        );
    }
    if cx.str("output").is_some_and(|o| o.to_lowercase() == "json") {
        cx.log(Value::Array(added))
    } else {
        cx.log(Value::Array(
            added
                .iter()
                .map(|r| obj! { "objectId": ?r.get("id").cloned(), "principalDisplayName": ?r.get("principalDisplayName").cloned(), "resourceDisplayName": ?r.get("resourceDisplayName").cloned() })
                .collect(),
        ))
    }
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let assignments: Vec<Value> = if let Some(o) = cx.s("appObjectId") {
        let r = Request::get(format!(
            "{GRAPH}/v1.0/servicePrincipals/{o}/appRoleAssignments"
        ))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
        let v = arr(&r, "value").to_vec();
        if v.is_empty() {
            return Err(Error::msg("no app role assignments found"));
        }
        v
    } else {
        let q = match cx.s("appId") {
            Some(a) => format!("appId eq '{}'", eqp(&a)),
            None => format!("displayName eq '{}'", eqp(&cx.u("appDisplayName"))),
        };
        let r = Request::get(format!(
            "{GRAPH}/v1.0/servicePrincipals?$expand=appRoleAssignments&$filter={q}"
        ))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
        let v = arr(&r, "value");
        if v.is_empty() {
            return Err(Error::msg("app registration not found"));
        }
        need_arr(&v[0], "appRoleAssignments", "map")?.to_vec()
    };
    let mut resources = Vec::new();
    for a in &assignments {
        resources.push(
            Request::get(format!(
                "{GRAPH}/v1.0/servicePrincipals/{}",
                u(a, "resourceId")
            ))
            .accept("application/json")
            .json_response()
            .data()
            .await?,
        );
    }
    let mut out = Vec::new();
    for a in &assignments {
        let Some(res) = resources
            .iter()
            .find(|r| r.get("id") == a.get("resourceId"))
        else {
            continue;
        };
        let Some(role) = arr(res, "appRoles")
            .iter()
            .find(|r| r.get("id") == a.get("appRoleId"))
        else {
            continue;
        };
        out.push(obj! {
            "appRoleId": ?a.get("appRoleId").cloned(),
            "resourceDisplayName": ?a.get("resourceDisplayName").cloned(),
            "resourceId": ?a.get("resourceId").cloned(),
            "roleId": ?role.get("id").cloned(),
            "roleName": ?role.get("value").cloned(),
            "created": ?a.get("createdDateTime").cloned(),
            "deleted": ?a.get("deletedDateTime").cloned()
        });
    }
    cx.log(Value::Array(out))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", &format!("Are you sure you want to remove the appRoleAssignment with scope(s) {} for resource {}?", cx.u("scopes"), cx.u("resource")))? {
        return Ok(());
    }
    let q = if let Some(a) = cx.s("appId") {
        format!("appId eq '{}'", eqp(&a))
    } else if let Some(o) = cx.s("appObjectId") {
        format!("id eq '{}'", eqp(&o))
    } else {
        format!("displayName eq '{}'", eqp(&cx.u("appDisplayName")))
    };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/servicePrincipals?$expand=appRoleAssignments&$filter={q}"
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let Some(sp) = arr(&r, "value").first().cloned() else {
        return Err(Error::msg("app registration not found"));
    };
    let res = Request::get(format!(
        "{GRAPH}/v1.0/servicePrincipals?{}",
        resource_filter(&cx.u("resource"))
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let Some(first) = arr(&res, "value").first() else {
        return Err(Error::msg("Resource not found"));
    };
    let found = arr(first, "appRoles").to_vec();
    if found.is_empty() {
        return Err(Error::msg(format!(
            "The resource '{}' does not have any application permissions available.",
            cx.u("resource")
        )));
    }
    let mut to_delete = Vec::new();
    for scope in scopes(cx) {
        match found
            .iter()
            .find(|r| s(r, "value").to_lowercase() == scope.to_lowercase().trim())
        {
            Some(r) => to_delete.push(r.clone()),
            None => {
                return Err(missing_scope(
                    cx,
                    &scope,
                    &found
                        .iter()
                        .map(|r| s(r, "value").to_string())
                        .collect::<Vec<_>>(),
                ));
            }
        }
    }
    let mut ids = Vec::new();
    for role in &to_delete {
        match arr(&sp, "appRoleAssignments")
            .iter()
            .find(|a| a.get("appRoleId") == role.get("id"))
        {
            Some(a) => ids.push(s(a, "id").to_string()),
            None => return Err(Error::msg("App role assignment not found")),
        }
    }
    for id in ids {
        Request::delete(format!(
            "{GRAPH}/v1.0/servicePrincipals/{}/appRoleAssignments/{id}",
            s(&sp, "id")
        ))
        .graph_json()
        .send()
        .await?;
    }
    Ok(())
}

pub async fn enterpriseapp_add(cx: &mut Cx) -> Result<()> {
    let app_id = match cx.s("id") {
        Some(i) => i,
        None => {
            let q = if let Some(n) = cx.s("displayName") {
                format!("displayName eq '{}'", eqp(&n))
            } else if let Some(o) = cx.s("objectId") {
                format!("id eq '{}'", eqp(&o))
            } else {
                String::new()
            };
            let r = Request::get(format!("{GRAPH}/v1.0/applications?$filter={q}"))
                .graph_json()
                .data()
                .await?;
            let v = arr(&r, "value");
            match v.len() {
                0 => return Err(Error::msg("The specified Entra app doesn't exist")),
                1 => s(&v[0], "appId").to_string(),
                _ => s(
                    &cx.handle_multiple_results(
                        &format!(
                            "Multiple Entra apps with name '{}' found.",
                            cx.u("displayName")
                        ),
                        &util::array_to_hash_table("appId", v),
                    )?,
                    "appId",
                )
                .to_string(),
            }
        }
    };
    let r = Request::post(format!("{GRAPH}/v1.0/servicePrincipals"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json;odata=nometadata")
        .json(obj! { "appId": app_id })
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

pub async fn enterpriseapp_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving enterprise application information...");
    let id = match cx.s("objectId") {
        Some(o) => o,
        None => {
            let q = if let Some(n) = cx.s("displayName") {
                format!("displayName eq '{}'", eqp(&n))
            } else if let Some(i) = cx.s("id") {
                format!("appId eq '{}'", eqp(&i))
            } else {
                String::new()
            };
            let r = Request::get(format!("{GRAPH}/v1.0/servicePrincipals?$filter={q}"))
                .graph_json()
                .data()
                .await?;
            let v = arr(&r, "value");
            match v.len() {
                0 => return Err(Error::msg("The specified Entra app does not exist")),
                1 => s(&v[0], "id").to_string(),
                _ => s(
                    &cx.handle_multiple_results(
                        &format!(
                            "Multiple Entra apps with name '{}' found.",
                            cx.u("displayName")
                        ),
                        &util::array_to_hash_table("id", v),
                    )?,
                    "id",
                )
                .to_string(),
            }
        }
    };
    let r = Request::get(format!("{GRAPH}/v1.0/servicePrincipals/{id}"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json;odata.metadata=none")
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

pub async fn enterpriseapp_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving enterprise application information...");
    let mut url = format!("{GRAPH}/v1.0/servicePrincipals");
    let mut f = Vec::new();
    if let Some(t) = cx.s("tag") {
        f.push(format!("(tags/any(t:t eq '{t}'))"));
    }
    if let Some(n) = cx.s("displayName") {
        f.push(format!("(displayName eq '{n}')"));
    }
    if !f.is_empty() {
        url.push_str(&format!("?$filter={}", f.join(" and ")));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn enterpriseapp_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .or_else(|| cx.s("objectId"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove enterprise application '{what}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing enterprise application {what}..."));
    let url = if let Some(i) = cx.s("id") {
        format!("{GRAPH}/v1.0/servicePrincipals(appId='{i}')")
    } else {
        let id = match cx.s("objectId") {
            Some(o) => o,
            None => {
                let n = cx.u("displayName");
                let items = odata::get_all_items(&format!(
                    "{GRAPH}/v1.0/servicePrincipals?$filter=displayName eq '{}'&$select=id",
                    eqp(&n)
                ))
                .await?;
                match items.len() {
                    0 => {
                        return Err(Error::msg(
                            "The specified enterprise application does not exist.",
                        ));
                    }
                    1 => s(&items[0], "id").to_string(),
                    _ => s(
                        &cx.handle_multiple_results(
                            &format!("Multiple enterprise applications with name '{n}' found."),
                            &util::array_to_hash_table("id", &items),
                        )?,
                        "id",
                    )
                    .to_string(),
                }
            }
        };
        format!("{GRAPH}/v1.0/servicePrincipals/{id}")
    };
    Request::delete(url).graph_json().send().await?;
    Ok(())
}

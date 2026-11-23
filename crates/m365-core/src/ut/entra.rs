//! `entraUser.ts`, `entraGroup.ts`, `entraAdministrativeUnit.ts`,
//! `entraDevice.ts`, `roleAssignment.ts`, `roleDefinition.ts` and
//! `directoryExtension.ts`.

use crate::error::{Error, Result};
use crate::http::Request;
use crate::json::{arr, s};
use crate::odata;
use crate::runner::handle_multiple_results;
use crate::util::{array_to_hash_table, encode_query_parameter as eqp};
use crate::{Value, obj};

const GRAPH: &str = "https://graph.microsoft.com";

/// `$select` of the properties without `/` (expanded ones), or `None`.
pub fn select_query(properties: Option<&str>) -> Option<String> {
    let p = properties.filter(|p| !p.is_empty())?;
    let sel: Vec<&str> = p.split(',').filter(|x| !x.contains('/')).collect();
    (!sel.is_empty()).then(|| format!("$select={}", sel.join(",")))
}

fn pick_one(items: Vec<Value>, not_found: String, multiple: String) -> Result<Value> {
    match items.len() {
        0 => Err(Error::js(not_found)),
        1 => Ok(items.into_iter().next().unwrap()),
        _ => handle_multiple_results(&multiple, &array_to_hash_table("id", &items)),
    }
}

pub mod user {
    use super::*;

    pub async fn id_by_upn(upn: &str) -> Result<String> {
        let res = Request::get(format!(
            "{GRAPH}/v1.0/users?$filter=userPrincipalName eq '{}'&$select=Id",
            eqp(upn)
        ))
        .graph_json()
        .data()
        .await?;
        match arr(&res, "value").first() {
            Some(u) => Ok(s(u, "id").to_string()),
            None => Err(Error::js(format!(
                "The specified user with user name {upn} does not exist."
            ))),
        }
    }

    async fn batch_ids(
        items: &[String],
        url: impl Fn(&str) -> String,
        err: impl Fn(&str) -> String,
    ) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        for chunk in items.chunks(20) {
            let requests: Vec<Value> = chunk
                .iter()
                .enumerate()
                .map(|(i, x)| obj! { "id": i + 1, "method": "GET", "url": url(x), "headers": obj! { "accept": "application/json;odata.metadata=none" } })
                .collect();
            let res = Request::post(format!("{GRAPH}/v1.0/$batch"))
                .graph_json()
                .json(obj! { "requests": requests })
                .data()
                .await?;
            for r in arr(&res, "responses") {
                let idx = r.get("id").and_then(Value::as_u64).unwrap_or(1) as usize;
                if r.get("status").and_then(Value::as_u64) != Some(200) {
                    return Err(Error::js(err(chunk
                        .get(idx.wrapping_sub(1))
                        .map(String::as_str)
                        .unwrap_or("undefined"))));
                }
                ids.push(
                    r.get("body")
                        .map(|b| s(b, "id").to_string())
                        .unwrap_or_default(),
                );
            }
        }
        Ok(ids)
    }

    pub async fn ids_by_upns(upns: &[String]) -> Result<Vec<String>> {
        batch_ids(
            upns,
            |u| format!("/users/{}?$select=id", eqp(u)),
            |u| format!("The specified user with user name '{u}' does not exist."),
        )
        .await
    }

    pub async fn id_by_email(email: &str) -> Result<String> {
        let res = Request::get(format!(
            "{GRAPH}/v1.0/users?$filter=mail eq '{}'&$select=id",
            eqp(email)
        ))
        .graph_json()
        .data()
        .await?;
        match arr(&res, "value").first() {
            Some(u) => Ok(s(u, "id").to_string()),
            None => Err(Error::js(format!(
                "The specified user with email {email} does not exist"
            ))),
        }
    }

    pub async fn ids_by_emails(emails: &[String]) -> Result<Vec<String>> {
        batch_ids(
            emails,
            |e| format!("/users?$filter=mail eq '{}'&$select=id", eqp(e)),
            |e| format!("The specified user with mail '{e}' does not exist."),
        )
        .await
    }

    pub async fn upn_by_id(id: &str, verbose: bool) -> Result<String> {
        if verbose {
            crate::log::log_stderr(&format!("Retrieving the upn for user {id}"));
        }
        let res = Request::get(format!("{GRAPH}/v1.0/users/{id}?$select=userPrincipalName"))
            .graph_json()
            .data()
            .await?;
        Ok(s(&res, "userPrincipalName").to_string())
    }
}

pub mod group {
    use super::*;

    pub async fn by_id(id: &str, properties: Option<&str>) -> Result<Value> {
        let q = select_query(properties)
            .map(|q| format!("?{q}"))
            .unwrap_or_default();
        Request::get(format!("{GRAPH}/v1.0/groups/{id}{q}"))
            .graph_json()
            .data()
            .await
    }

    pub async fn by_display_name_all(name: &str, properties: Option<&str>) -> Result<Vec<Value>> {
        let q = select_query(properties)
            .map(|q| format!("&{q}"))
            .unwrap_or_default();
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/groups?$filter=displayName eq '{}'{q}",
            eqp(name)
        ))
        .await
    }

    pub async fn by_display_name(name: &str, properties: Option<&str>) -> Result<Value> {
        let g = by_display_name_all(name, properties).await?;
        pick_one(
            g,
            format!("The specified group '{name}' does not exist."),
            format!("Multiple groups with name '{name}' found."),
        )
    }

    pub async fn id_by_display_name(name: &str) -> Result<String> {
        let g = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/groups?$filter=displayName eq '{}'&$select=id",
            eqp(name)
        ))
        .await?;
        let g = pick_one(
            g,
            format!("The specified group '{name}' does not exist."),
            format!("Multiple groups with name '{name}' found."),
        )?;
        Ok(s(&g, "id").to_string())
    }

    pub async fn id_by_mail_nickname(nick: &str) -> Result<String> {
        let g = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/groups?$filter=mailNickname eq '{}'&$select=id",
            eqp(nick)
        ))
        .await?;
        let g = pick_one(
            g,
            format!("The specified group '{nick}' does not exist."),
            format!("Multiple groups with mail nickname '{nick}' found."),
        )?;
        Ok(s(&g, "id").to_string())
    }

    pub async fn set(
        id: &str,
        is_private: Option<bool>,
        display_name: Option<&str>,
        description: Option<&str>,
        verbose: bool,
    ) -> Result<()> {
        if verbose {
            crate::log::log_stderr(&format!("Updating Microsoft 365 Group {id}..."));
        }
        let body = obj! {
            "displayName": ?display_name.filter(|s| !s.is_empty()),
            "description": ?description.filter(|s| !s.is_empty()),
            "visibility": ?is_private.map(|p| if p { "Private" } else { "Public" })
        };
        Request::patch(format!("{GRAPH}/v1.0/groups/{id}"))
            .graph_json()
            .json(body)
            .send()
            .await?;
        Ok(())
    }

    pub async fn is_unified(id: &str) -> Result<bool> {
        let g = Request::get(format!("{GRAPH}/v1.0/groups/{id}?$select=groupTypes"))
            .graph_json()
            .data()
            .await?;
        Ok(arr(&g, "groupTypes")
            .iter()
            .any(|t| t.as_str() == Some("Unified")))
    }

    pub async fn ids_by_display_names(names: &[String]) -> Result<Vec<String>> {
        let mut ids = Vec::new();
        for chunk in names.chunks(20) {
            let requests: Vec<Value> = chunk
                .iter()
                .enumerate()
                .map(|(i, n)| obj! { "id": i + 1, "method": "GET", "url": format!("/groups?$filter=displayName eq '{}'&$select=id", eqp(n)), "headers": obj! { "accept": "application/json;odata.metadata=none" } })
                .collect();
            let res = Request::post(format!("{GRAPH}/v1.0/$batch"))
                .graph_json()
                .json(obj! { "requests": requests })
                .data()
                .await?;
            for r in arr(&res, "responses") {
                let idx = r.get("id").and_then(Value::as_u64).unwrap_or(1) as usize;
                let name = chunk
                    .get(idx.wrapping_sub(1))
                    .map(String::as_str)
                    .unwrap_or("undefined");
                let vals = r
                    .get("body")
                    .map(|b| arr(b, "value").to_vec())
                    .unwrap_or_default();
                match vals.len() {
                    1 => ids.push(s(&vals[0], "id").to_string()),
                    0 => {
                        return Err(Error::js(format!(
                            "The specified group with name '{name}' does not exist."
                        )));
                    }
                    _ => {
                        let g = handle_multiple_results(
                            &format!("Multiple groups with the name '{name}' found."),
                            &array_to_hash_table("id", &vals),
                        )?;
                        ids.push(s(&g, "id").to_string());
                    }
                }
            }
        }
        Ok(ids)
    }
}

pub async fn administrative_unit_by_display_name(
    name: &str,
    properties: Option<&str>,
) -> Result<Value> {
    let q = select_query(properties)
        .map(|q| format!("&{q}"))
        .unwrap_or_default();
    let items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/directory/administrativeUnits?$filter=displayName eq '{}'{q}",
        eqp(name)
    ))
    .await?;
    pick_one(
        items,
        format!("The specified administrative unit '{name}' does not exist."),
        format!("Multiple administrative units with name '{name}' found."),
    )
}

pub async fn device_by_display_name(name: &str) -> Result<Value> {
    let items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/devices?$filter=displayName eq '{}'",
        eqp(name)
    ))
    .await?;
    pick_one(
        items,
        format!("The specified device '{name}' does not exist."),
        format!("Multiple devices with name '{name}' found."),
    )
}

pub async fn create_role_assignment(
    role_definition_id: &str,
    principal_id: &str,
    scope: &str,
) -> Result<Value> {
    Request::post(format!("{GRAPH}/v1.0/roleManagement/directory/roleAssignments"))
        .graph_json()
        .json(obj! { "roleDefinitionId": role_definition_id, "principalId": principal_id, "directoryScopeId": scope })
        .data()
        .await
}

pub async fn role_definition_by_display_name(
    name: &str,
    properties: Option<&str>,
) -> Result<Value> {
    let mut url = format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleDefinitions?$filter=displayName eq '{}'",
        eqp(name)
    );
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("&$select={p}"));
    }
    let items = odata::get_all_items(&url).await?;
    pick_one(
        items,
        format!("The specified role definition '{name}' does not exist."),
        format!("Multiple role definitions with name '{name}' found."),
    )
}

pub async fn role_definition_by_id(id: &str, properties: Option<&str>) -> Result<Value> {
    let mut url = format!("{GRAPH}/v1.0/roleManagement/directory/roleDefinitions/{id}");
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("?$select={p}"));
    }
    Request::get(url).graph_json().data().await
}

pub async fn exchange_role_definition_by_display_name(
    name: &str,
    properties: Option<&str>,
) -> Result<Value> {
    let mut url = format!(
        "{GRAPH}/beta/roleManagement/exchange/roleDefinitions?$filter=displayName eq '{}'",
        eqp(name)
    );
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("&$select={p}"));
    }
    let items = odata::get_all_items(&url).await?;
    pick_one(
        items,
        format!("The specified role definition '{name}' does not exist."),
        format!("Multiple role definitions with name '{name}' found."),
    )
}

pub async fn directory_extension_by_name(
    name: &str,
    app_object_id: &str,
    properties: Option<&[&str]>,
) -> Result<Value> {
    let mut url = format!(
        "{GRAPH}/v1.0/applications/{app_object_id}/extensionProperties?$filter=name eq '{}'",
        eqp(name)
    );
    if let Some(p) = properties {
        url.push_str(&format!("&$select={}", p.join(",")));
    }
    let items = odata::get_all_items(&url).await?;
    items.into_iter().next().ok_or_else(|| {
        Error::js(format!(
            "The specified directory extension '{name}' does not exist."
        ))
    })
}

/// Batched group membership updates shared by `entra group`,
/// `entra m365group` and `teams` commands.
pub mod members {
    use super::*;

    fn batch() -> Request {
        Request::post(format!("{GRAPH}/v1.0/$batch"))
            .header("content-type", "application/json;odata.metadata=none")
    }

    fn check(res: &Value, ok: u64) -> Result<()> {
        for r in arr(res, "responses") {
            if r.get("status").and_then(Value::as_u64) != Some(ok) {
                return Err(Error::Thrown(r.get("body").cloned().unwrap_or(Value::Null)));
            }
        }
        Ok(())
    }

    /// PATCHes `<role>@odata.bind` in batches of 20 requests with 20 users.
    pub async fn add_bound(group_id: &str, role: &str, ids: &[String], accept: bool) -> Result<()> {
        for b in ids.chunks(400) {
            let mut requests = Vec::new();
            for (j, chunk) in b.chunks(20).enumerate() {
                let mut headers = obj! { "content-type": "application/json;odata.metadata=none" };
                if accept {
                    headers.as_object_mut().unwrap().insert(
                        "accept".into(),
                        Value::from("application/json;odata.metadata=none"),
                    );
                }
                let mut body = crate::json::Obj::new();
                body.insert(
                    format!("{role}@odata.bind"),
                    Value::Array(
                        chunk
                            .iter()
                            .map(|u| Value::from(format!("{GRAPH}/v1.0/directoryObjects/{u}")))
                            .collect(),
                    ),
                );
                requests.push(obj! { "id": j * 20 + 1, "method": "PATCH", "url": format!("/groups/{group_id}"), "headers": headers, "body": Value::Object(body) });
            }
            let mut req = batch();
            if accept {
                req = req.accept("application/json;odata.metadata=none");
            }
            let res = req
                .json(obj! { "requests": requests })
                .json_response()
                .data()
                .await?;
            check(&res, 204)?;
        }
        Ok(())
    }

    /// Removes `ids` that currently hold `current_role` (the `user set`
    /// commands). As upstream, the DELETE batches slice `ids`, not the
    /// users found in the role.
    pub async fn remove_from_role(
        group_id: &str,
        current_role: &str,
        ids: &[String],
    ) -> Result<()> {
        let mut to_remove: Vec<String> = Vec::new();
        for chunk in ids.chunks(20) {
            let requests: Vec<Value> = chunk
                .iter()
                .map(|u| obj! { "id": u.as_str(), "method": "GET", "url": format!("/groups/{group_id}/{current_role}/$count?$filter=id eq '{u}'"), "headers": obj! { "ConsistencyLevel": "eventual" } })
                .collect();
            let res = batch()
                .json(obj! { "requests": requests })
                .json_response()
                .data()
                .await?;
            for r in arr(&res, "responses") {
                if r.get("status").and_then(Value::as_u64) == Some(200) {
                    if r.get("body").and_then(Value::as_f64) == Some(1.0) {
                        to_remove.push(crate::json::to_js_string(
                            r.get("id").unwrap_or(&Value::Null),
                        ));
                    }
                } else {
                    return Err(Error::Thrown(r.get("body").cloned().unwrap_or(Value::Null)));
                }
            }
        }
        let mut i = 0;
        while i < to_remove.len() {
            let requests: Vec<Value> = ids.iter().skip(i).take(20).map(|u| obj! { "id": u.as_str(), "method": "DELETE", "url": format!("/groups/{group_id}/{current_role}/{u}/$ref") }).collect();
            let res = batch()
                .json(obj! { "requests": requests })
                .json_response()
                .data()
                .await?;
            check(&res, 204)?;
            i += 20;
        }
        Ok(())
    }

    /// Makes `role` hold exactly `ids` (the `set` commands).
    pub async fn update_users(
        group_id: &str,
        role: &str,
        ids: &[String],
        verbose: bool,
    ) -> Result<()> {
        let current = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/groups/{group_id}/{role}/microsoft.graph.user?$select=id"
        ))
        .await?;
        let to_add: Vec<String> = ids
            .iter()
            .filter(|u| !current.iter().any(|c| s(c, "id") == u.as_str()))
            .cloned()
            .collect();
        let to_remove: Vec<String> = current
            .iter()
            .filter(|c| !ids.iter().any(|u| s(c, "id") == u.as_str()))
            .map(|c| s(c, "id").to_string())
            .collect();
        if verbose {
            crate::log::log_stderr(&format!("Adding {} {role}...", to_add.len()));
        }
        add_bound(group_id, role, &to_add, true).await?;
        if verbose {
            crate::log::log_stderr(&format!("Removing {} {role}...", to_remove.len()));
        }
        for chunk in to_remove.chunks(20) {
            let requests: Vec<Value> = chunk.iter().map(|u| obj! { "id": u.as_str(), "method": "DELETE", "url": format!("/groups/{group_id}/{role}/{u}/$ref") }).collect();
            let res = batch()
                .accept("application/json;odata.metadata=none")
                .json(obj! { "requests": requests })
                .json_response()
                .data()
                .await?;
            check(&res, 204)?;
        }
        Ok(())
    }
}

/// `src/utils/teams.ts`.
pub mod teams {
    use super::*;

    pub async fn by_display_name(name: &str) -> Result<Value> {
        let t = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/teams?$filter=displayName eq '{}'",
            eqp(name)
        ))
        .await?;
        pick_one(
            t,
            format!("The specified team '{name}' does not exist."),
            format!("Multiple teams with name '{name}' found."),
        )
    }

    pub async fn id_by_display_name(name: &str) -> Result<String> {
        let t = odata::get_all_items(&format!(
            "{GRAPH}/v1.0/teams?$filter=displayName eq '{}'&$select=id",
            eqp(name)
        ))
        .await?;
        let t = pick_one(
            t,
            format!("The specified team '{name}' does not exist."),
            format!("Multiple teams with name '{name}' found."),
        )?;
        Ok(crate::json::u(&t, "id"))
    }

    pub async fn channel_by_display_name(team_id: &str, name: &str) -> Result<Value> {
        let r = odata::get_all_items_req(
            Request::get(format!(
                "{GRAPH}/v1.0/teams/{team_id}/channels?$filter=displayName eq '{}'",
                eqp(name)
            ))
            .graph_json(),
        )
        .await?;
        r.into_iter().next().ok_or_else(|| Error::js(format!("The channel '{name}' does not exist in the Microsoft Teams team with ID '{team_id}'.")))
    }

    pub async fn channel_id_by_display_name(team_id: &str, name: &str) -> Result<String> {
        let r = odata::get_all_items_req(
            Request::get(format!(
                "{GRAPH}/v1.0/teams/{team_id}/channels?$filter=displayName eq '{}'&$select=id",
                eqp(name)
            ))
            .graph_json(),
        )
        .await?;
        match r.first() {
            Some(c) => Ok(crate::json::u(c, "id")),
            None => Err(Error::js(format!(
                "The channel '{name}' does not exist in the Microsoft Teams team with ID '{team_id}'."
            ))),
        }
    }
}

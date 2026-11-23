//! `teams message *`, `teams tab *` and `teams user *`.

use m365_core::prelude::*;
use m365_core::ut::entra::teams;

pub async fn get(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/messages/{}",
        cx.u("teamId"),
        cx.u("channelId"),
        cx.u("id")
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

fn hoist_body(cx: &Cx, items: &mut [Value]) {
    if cx.str("output").as_deref() != Some("json") {
        for i in items.iter_mut() {
            let c = i.get("body").and_then(|b| b.get("content")).cloned();
            if let Some(o) = i.as_object_mut() {
                m365_core::json::assign(o, "body", c);
            }
        }
    }
}

pub fn list_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(m365_core::validation::is_date_in_range(&to_js_string(v), 8))
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let delta = match cx.str("since") {
        Some(s) => format!("/delta?$filter=lastModifiedDateTime gt {s}"),
        None => String::new(),
    };
    let mut items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/messages{delta}",
        cx.u("teamId"),
        cx.u("channelId")
    ))
    .await?;
    hoist_body(cx, &mut items);
    cx.log(Value::Array(items))
}

async fn team_and_channel(cx: &Cx, verbose: bool) -> Result<(String, String)> {
    let tid = match cx.s("teamId") {
        Some(t) => t,
        None => {
            if verbose {
                cx.verbose_log("Getting the Team ID.");
            }
            teams::id_by_display_name(&cx.u("teamName")).await?
        }
    };
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => {
            if verbose {
                cx.verbose_log("Getting the channel ID.");
            }
            teams::channel_id_by_display_name(&tid, &cx.u("channelName")).await?
        }
    };
    Ok((tid, cid))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove this message?")? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing message '{}' from channel '{}' in team '{}'.",
        cx.u("id"),
        cx.s("channelId")
            .or_else(|| cx.s("channelName"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("teamId")
            .or_else(|| cx.s("teamName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let r = async {
        let (tid, cid) = team_and_channel(cx, false).await?;
        Request::post(format!(
            "{GRAPH}/v1.0/teams/{tid}/channels/{}/messages/{}/softDelete",
            eqp(&cid),
            cx.u("id")
        ))
        .graph_json()
        .send()
        .await?;
        Ok::<(), Error>(())
    }
    .await;
    match r {
        Err(e)
            if e.body()
                .and_then(|b| b.get("error"))
                .and_then(|e| e.get("code"))
                .and_then(Value::as_str)
                == Some("NotFound") =>
        {
            Err(Error::msg(
                "The message was not found in the Teams channel.",
            ))
        }
        other => other,
    }
}

pub async fn reply_list(cx: &mut Cx) -> Result<()> {
    let mut items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/messages/{}/replies",
        cx.u("teamId"),
        cx.u("channelId"),
        cx.u("messageId")
    ))
    .await?;
    hoist_body(cx, &mut items);
    cx.log(Value::Array(items))
}

pub async fn restore(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Restoring deleted message '{}' from channel '{}' in the Microsoft Teams team '{}'.",
        cx.u("id"),
        cx.s("channelId")
            .or_else(|| cx.s("channelName"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("teamId")
            .or_else(|| cx.s("teamName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let (tid, cid) = team_and_channel(cx, true).await?;
    Request::post(format!(
        "{GRAPH}/v1.0/teams/{tid}/channels/{cid}/messages/{}/undoSoftDelete",
        cx.u("id")
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn send(cx: &mut Cx) -> Result<()> {
    let r = Request::post(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/messages",
        cx.u("teamId"),
        cx.u("channelId")
    ))
    .graph_json()
    .header("content-type", "application/json")
    .json(obj! { "body": obj! { "contentType": "html", "content": ?cx.str("message") } })
    .data()
    .await?;
    cx.log(r)
}

// ---- tab -----------------------------------------------------------------------

pub async fn tab_add(cx: &mut Cx) -> Result<()> {
    let exclude = [
        "debug",
        "verbose",
        "teamId",
        "channelId",
        "appId",
        "appName",
        "entityId",
        "contentUrl",
        "removeUrl",
        "websiteUrl",
        "output",
    ];
    let mut conf = Obj::new();
    for k in ["contentUrl", "entityId", "removeUrl", "websiteUrl"] {
        if let Some(v) = cx.s(k) {
            conf.insert(k.into(), Value::from(v));
        }
    }
    for (k, v) in &cx.opts {
        if !exclude.contains(&k.as_str()) {
            conf.insert(k.clone(), Value::from(to_js_string(v)));
        }
    }
    let mut body = Obj::new();
    body.insert("configuration".into(), Value::Object(conf));
    if let Some(n) = cx.s("appName") {
        body.insert("displayName".into(), Value::from(n));
    }
    if let Some(a) = cx.s("appId") {
        body.insert(
            "teamsApp@odata.bind".into(),
            Value::from(format!(
                "https://graph.microsoft.com/v1.0/appCatalogs/teamsApps/{a}"
            )),
        );
    }
    let r = Request::post(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/tabs",
        eqp(&cx.u("teamId")),
        cx.u("channelId")
    ))
    .accept("application/json;odata.metadata=none")
    .header("content-type", "application/json;odata=nometadata")
    .json(Value::Object(body))
    .json_response()
    .data()
    .await?;
    cx.log(r)
}

pub async fn tab_get(cx: &mut Cx) -> Result<()> {
    let tid = match cx.s("teamId") {
        Some(t) => t,
        None => teams::id_by_display_name(&cx.u("teamName")).await?,
    };
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => {
            let r = Request::get(format!(
                "{GRAPH}/v1.0/teams/{}/channels?$filter=displayName eq '{}'",
                eqp(&tid),
                eqp(&cx.u("channelName"))
            ))
            .graph_json()
            .data()
            .await?;
            match arr(&r, "value").first() {
                Some(c) => u(c, "id"),
                None => {
                    return Err(Error::msg(
                        "The specified channel does not exist in the Microsoft Teams team",
                    ));
                }
            }
        }
    };
    let tab = match cx.s("id") {
        Some(i) => i,
        None => {
            let r = Request::get(format!(
                "{GRAPH}/v1.0/teams/{}/channels/{}/tabs?$filter=displayName eq '{}'",
                eqp(&tid),
                eqp(&cid),
                eqp(&cx.u("name"))
            ))
            .graph_json()
            .data()
            .await?;
            match arr(&r, "value").first() {
                Some(t) => u(t, "id"),
                None => {
                    return Err(Error::msg(
                        "The specified tab does not exist in the Microsoft Teams team channel",
                    ));
                }
            }
        }
    };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/tabs/{}?$expand=teamsApp",
        eqp(&tid),
        eqp(&cid),
        eqp(&tab)
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn tab_list(cx: &mut Cx) -> Result<()> {
    let mut items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/tabs?$expand=teamsApp",
        cx.u("teamId"),
        eqp(&cx.u("channelId"))
    ))
    .await?;
    if cx.str("output").as_deref() != Some("json") {
        for i in items.iter_mut() {
            let id = i.get("teamsApp").and_then(|t| t.get("id")).cloned();
            if let (Some(o), Some(id)) = (i.as_object_mut(), id) {
                o.insert("teamsAppTabId".into(), id);
            }
        }
    }
    cx.log(Value::Array(items))
}

pub async fn tab_remove(cx: &mut Cx) -> Result<()> {
    let (id, ch, team) = (cx.u("id"), cx.u("channelId"), cx.u("teamId"));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the tab with id {id} from channel {ch} in team {team}?"
        ),
    )? {
        return Ok(());
    }
    Request::delete(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{ch}/tabs/{}",
        eqp(&team),
        eqp(&id)
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

// ---- user ----------------------------------------------------------------------

async fn catalog_app_id(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("id") {
        return Ok(i);
    }
    let n = cx.u("name");
    let r = Request::get(format!(
        "{GRAPH}/v1.0/appCatalogs/teamsApps?$filter=displayName eq '{}'",
        eqp(&n)
    ))
    .graph_json()
    .data()
    .await?;
    let v = arr(&r, "value");
    match v.len() {
        1 => Ok(u(&v[0], "id")),
        0 => Err(Error::msg("The specified Teams app does not exist")),
        _ => Ok(u(
            &cx.handle_multiple_results(
                &format!("Multiple Teams apps with name '{n}' found."),
                &util::array_to_hash_table("id", v),
            )?,
            "id",
        )),
    }
}

fn user_ident(cx: &Cx) -> String {
    cx.str("userId")
        .or_else(|| cx.str("userName"))
        .unwrap_or_else(|| "undefined".into())
}

pub async fn user_app_add(cx: &mut Cx) -> Result<()> {
    let app = catalog_app_id(cx).await?;
    cx.verbose_log(&format!(
        "Adding app with ID {app} for user {}",
        cx.u("userId")
    ));
    Request::post(format!(
        "{GRAPH}/v1.0/users/{}/teamwork/installedApps",
        eqp(&user_ident(cx))
    ))
    .header("content-type", "application/json;odata=nometadata")
    .accept("application/json;odata.metadata=none")
    .json(obj! { "teamsApp@odata.bind": format!("{GRAPH}/v1.0/appCatalogs/teamsApps/{app}") })
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn user_app_list(cx: &mut Cx) -> Result<()> {
    let uid = match cx.s("userId") {
        Some(i) => i,
        None => {
            let r = Request::get(format!("{GRAPH}/v1.0/users/{}/id", eqp(&cx.u("userName"))))
                .graph_json()
                .data()
                .await?;
            u(&r, "value")
        }
    };
    let mut items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/users/{}/teamwork/installedApps?$expand=teamsAppDefinition,teamsApp",
        eqp(&uid)
    ))
    .await?;
    for i in items.iter_mut() {
        let raw = m365_core::cert::lenient_base64(&u(i, "id"));
        let ascii: String = raw.iter().map(|b| (b & 0x7f) as char).collect();
        let app_id = match ascii.find("##") {
            Some(p) => ascii[p + 2..].to_string(),
            None => ascii[1.min(ascii.len())..].to_string(),
        };
        i.as_object_mut()
            .map(|o| o.insert("appId".into(), Value::from(app_id)));
    }
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(items));
    }
    let out: Vec<Value> = items
        .iter()
        .map(|i| {
            let d = i.get("teamsAppDefinition").cloned().unwrap_or(Value::Null);
            obj! { "id": ?i.get("id").cloned(), "appId": ?i.get("appId").cloned(), "displayName": ?d.get("displayName").cloned(), "version": ?d.get("version").cloned() }
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn user_app_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the app with id {} for user {}?",
            cx.u("id"),
            user_ident(cx)
        ),
    )? {
        return Ok(());
    }
    let app = match cx.s("id") {
        Some(i) => i,
        None => {
            let n = cx.u("name");
            let r = Request::get(format!(
                "{GRAPH}/v1.0/users/{}/teamwork/installedApps?$expand=teamsAppDefinition&$filter=teamsAppDefinition/displayName eq '{}'",
                cx.u("userId"),
                eqp(&n)
            ))
            .graph_json()
            .data()
            .await?;
            let v = arr(&r, "value");
            match v.len() {
                1 => u(&v[0], "id"),
                0 => return Err(Error::msg("The specified Teams app does not exist")),
                _ => u(
                    &cx.handle_multiple_results(
                        &format!("Multiple Teams apps with name '{n}' found."),
                        &util::array_to_hash_table("id", v),
                    )?,
                    "id",
                ),
            }
        }
    };
    cx.verbose_log(&format!(
        "Removing app with ID {} for user {}",
        cx.u("id"),
        cx.u("userId")
    ));
    Request::delete(format!(
        "{GRAPH}/v1.0/users/{}/teamwork/installedApps/{app}",
        eqp(&user_ident(cx))
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn user_app_upgrade(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Upgrading app {} for user {}",
        cx.s("id")
            .or_else(|| cx.s("name"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("userId")
            .or_else(|| cx.s("userName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    cx.verbose_log("Retrieving app ID");
    let who = cx.first_u(&["userId", "userName"]);
    let app = match cx.s("id") {
        Some(i) => i,
        None => {
            let n = cx.u("name");
            let items = odata::get_all_items(&format!(
                "{GRAPH}/v1.0/users/{}/teamwork/installedApps?$expand=teamsAppDefinition&$filter=teamsAppDefinition/displayName eq '{}'&$select=id",
                eqp(&who),
                eqp(&n)
            ))
            .await?;
            match items.len() {
                1 => u(&items[0], "id"),
                0 => {
                    return Err(Error::msg(format!(
                        "The specified Teams app {n} does not exist or is not installed for the user"
                    )));
                }
                _ => u(
                    &cx.handle_multiple_results(
                        &format!("Multiple installed Teams apps with name '{n}' found."),
                        &util::array_to_hash_table("id", &items),
                    )?,
                    "id",
                ),
            }
        }
    };
    Request::post(format!(
        "{GRAPH}/v1.0/users/{}/teamwork/installedApps/{app}/upgrade",
        eqp(&who)
    ))
    .accept("application/json;odata.metadata=none")
    .send()
    .await?;
    Ok(())
}

pub async fn user_list(cx: &mut Cx) -> Result<()> {
    let tid = cx.u("teamId");
    let mut items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/groups/{tid}/owners?$select=id,displayName,userPrincipalName,userType"
    ))
    .await?;
    for i in items.iter_mut() {
        i.as_object_mut()
            .map(|o| o.insert("userType".into(), Value::from("Owner")));
    }
    if cx.str("role").as_deref() != Some("Owner") {
        items.extend(odata::get_all_items(&format!("{GRAPH}/v1.0/groups/{tid}/members?$select=id,displayName,userPrincipalName,userType")).await?);
    }
    let mut dedup: Vec<Value> = Vec::new();
    for i in items {
        if !dedup
            .iter()
            .any(|t| t.get("id") == i.get("id") && t.get("displayName") == i.get("displayName"))
        {
            dedup.push(i);
        }
    }
    if let Some(r) = cx.s("role") {
        dedup.retain(|i| s(i, "userType") == r);
    }
    cx.log(Value::Array(dedup))
}

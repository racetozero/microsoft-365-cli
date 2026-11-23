//! `teams channel *` and `teams chat *`.

use m365_core::prelude::*;
use m365_core::ut::entra::teams;
use m365_core::validation as val;

async fn team_id(cx: &Cx) -> Result<String> {
    match cx.s("teamId") {
        Some(t) => Ok(t),
        None => teams::id_by_display_name(&cx.u("teamName")).await,
    }
}

pub fn add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = o.get("type").and_then(Value::as_str);
    let private = matches!(t, Some("private" | "shared"));
    let owner = o.get("owner").is_some_and(truthy);
    Ok(match i {
        1 => !(private && !owner),
        2 => !(!private && owner),
        _ => true,
    })
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let t = cx.s("type");
    let mut data = obj! { "membershipType": t.clone().unwrap_or_else(|| "standard".into()), "displayName": ?cx.str("name") };
    if matches!(t.as_deref(), Some("private" | "shared")) {
        data.as_object_mut().unwrap().insert(
            "members".into(),
            Value::Array(vec![obj! {
                "@odata.type": "#microsoft.graph.aadUserConversationMember",
                "user@odata.bind": format!("{GRAPH}/v1.0/users('{}')", cx.u("owner")),
                "roles": vec![Value::from("owner")]
            }]),
        );
    }
    let r = Request::post(format!("{GRAPH}/v1.0/teams/{tid}/channels"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json;odata=nometadata")
        .json(data)
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let ch = if cx.bool("primary") || cx.s("id").is_some() {
        let path = if cx.bool("primary") {
            "primaryChannel".to_string()
        } else {
            format!("channels/{}", cx.u("id"))
        };
        Request::get(format!("{GRAPH}/v1.0/teams/{tid}/{path}"))
            .graph_json()
            .data()
            .await?
    } else {
        teams::channel_by_display_name(&tid, &cx.u("name")).await?
    };
    cx.log(ch)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let mut ep = format!("{GRAPH}/v1.0/teams/{tid}/channels");
    if let Some(t) = cx.s("type") {
        ep.push_str(&format!(
            "?$filter=membershipType eq '{}'",
            t.to_lowercase()
        ));
    }
    cx.log(Value::Array(odata::get_all_items(&ep).await?))
}

async fn channel_id_by_name(
    cx: &Cx,
    tid: &str,
    name_key: &str,
    not_found: &str,
    require_private: bool,
) -> Result<String> {
    let name = cx.u(name_key);
    let r = Request::get(format!(
        "{GRAPH}/v1.0/teams/{}/channels?$filter=displayName eq '{}'",
        eqp(tid),
        eqp(&name)
    ))
    .graph_json()
    .data()
    .await?;
    let Some(c) = arr(&r, "value").first() else {
        return Err(Error::msg(
            not_found.replace("{name}", &name).replace("{team}", tid),
        ));
    };
    if require_private && s(c, "membershipType") != "private" {
        return Err(Error::msg("The specified channel is not a private channel"));
    }
    Ok(u(c, "id"))
}

pub async fn member_add(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => channel_id_by_name(cx, &tid, "channelName", "The specified channel '{name}' does not exist in the Microsoft Teams team with ID '{team}'", true).await?,
    };
    let ids: Vec<String> = match cx.s("userIds") {
        Some(i) => i.split(',').map(|u| u.trim().to_string()).collect(),
        None => {
            let mut v = Vec::new();
            for name in cx.u("userDisplayNames").split(',').map(str::trim) {
                let r = Request::get(format!(
                    "{GRAPH}/v1.0/users?$filter=displayName eq '{}'",
                    eqp(name)
                ))
                .graph_json()
                .data()
                .await?;
                let vals = arr(&r, "value");
                match vals.len() {
                    0 => {
                        return Err(Error::msg(format!(
                            "The specified user '{name}' does not exist"
                        )));
                    }
                    1 => v.push(u(&vals[0], "id")),
                    _ => v.push(u(
                        &cx.handle_multiple_results(
                            &format!("Multiple users with display name '{name}' found."),
                            &util::array_to_hash_table("id", vals),
                        )?,
                        "id",
                    )),
                }
            }
            v
        }
    };
    let ep = format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}/members",
        eqp(&tid),
        eqp(&cid)
    );
    let roles: Vec<Value> = if cx.bool("owner") {
        vec!["owner".into()]
    } else {
        vec![]
    };
    for id in ids {
        Request::post(ep.clone())
            .header("content-type", "application/json;odata=nometadata")
            .accept("application/json;odata.metadata=none")
            .json(obj! { "@odata.type": "#microsoft.graph.aadUserConversationMember", "roles": roles.clone(), "user@odata.bind": format!("{GRAPH}/v1.0/users('{id}')") })
            .json_response()
            .send()
            .await?;
    }
    Ok(())
}

pub async fn member_list(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => {
            channel_id_by_name(
                cx,
                &tid,
                "channelName",
                "The specified channel does not exist in the Microsoft Teams team",
                false,
            )
            .await?
        }
    };
    let mut m =
        odata::get_all_items(&format!("{GRAPH}/v1.0/teams/{tid}/channels/{cid}/members")).await?;
    if let Some(r) = cx.s("role") {
        if r == "member" {
            for i in m.iter() {
                need_arr(i, "roles", "length")?;
            }
            m.retain(|i| arr(i, "roles").is_empty());
        } else {
            for i in m.iter() {
                need_arr(i, "roles", "indexOf")?;
            }
            m.retain(|i| {
                arr(i, "roles")
                    .iter()
                    .any(|x| x.as_str() == Some(r.as_str()))
            });
        }
    }
    cx.log(Value::Array(m))
}

async fn member_id(cx: &Cx, tid: &str, cid: &str) -> Result<String> {
    if let Some(i) = cx.s("id") {
        return Ok(i);
    }
    let r = Request::get(format!("{GRAPH}/v1.0/teams/{tid}/channels/{cid}/members"))
        .graph_json()
        .data()
        .await?;
    let uid = cx.s("userId").map(|x| x.to_lowercase());
    let un = cx.s("userName").map(|x| x.to_lowercase());
    let found: Vec<Value> = arr(&r, "value")
        .iter()
        .filter(|x| {
            uid.as_ref().is_some_and(|u| {
                x.get("userId")
                    .and_then(Value::as_str)
                    .map(str::to_lowercase)
                    .as_deref()
                    == Some(u.as_str())
            }) || un.as_ref().is_some_and(|u| {
                x.get("email")
                    .and_then(Value::as_str)
                    .map(str::to_lowercase)
                    .as_deref()
                    == Some(u.as_str())
            })
        })
        .cloned()
        .collect();
    Ok(u(
        &m365_core::runner::pick_one(
            &found,
            "id",
            Error::msg("The specified member does not exist in the Microsoft Teams channel"),
            &format!(
                "Multiple Microsoft Teams channel members with name {} found.",
                cx.u("userName")
            ),
        )?,
        "id",
    ))
}

pub async fn member_remove(cx: &mut Cx) -> Result<()> {
    if !cx.bool("force") {
        let user = cx.first_u(&["userName", "userId", "id"]);
        let team = cx.first_u(&["teamName", "teamId"]);
        let ch = cx.first_u(&["channelName", "channelId"]);
        if !cx.confirm(&format!("Are you sure you want to remove the member {user} from the channel {ch} in team {team}?"))? {
            return Ok(());
        }
    }
    cx.verbose_log(&format!(
        "Removing member {} from channel {} from team {}",
        cx.s("userId")
            .or_else(|| cx.s("id"))
            .or_else(|| cx.s("userName"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("channelId")
            .or_else(|| cx.s("channelName"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("teamId")
            .or_else(|| cx.s("teamName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let tid = team_id(cx).await?;
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => {
            channel_id_by_name(
                cx,
                &tid,
                "channelName",
                "The specified channel does not exist in the Microsoft Teams team",
                true,
            )
            .await?
        }
    };
    let mid = member_id(cx, &tid, &cid).await?;
    Request::delete(format!(
        "{GRAPH}/v1.0/teams/{tid}/channels/{cid}/members/{mid}"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn member_set(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let cid = match cx.s("channelId") {
        Some(c) => c,
        None => {
            channel_id_by_name(
                cx,
                &tid,
                "channelName",
                "The specified channel does not exist in the Microsoft Teams team",
                true,
            )
            .await?
        }
    };
    let mid = member_id(cx, &tid, &cid).await?;
    let r = Request::patch(format!("{GRAPH}/v1.0/teams/{tid}/channels/{cid}/members/{mid}"))
        .graph_json()
        .header("Prefer", "return=representation")
        .json(obj! { "@odata.type": "#microsoft.graph.aadUserConversationMember", "roles": vec![Value::from(cx.u("role"))] })
        .data()
        .await?;
    cx.log(r)
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let ch = cx.first_u(&["name", "id"]);
    let team = cx.first_u(&["teamId", "teamName"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the channel {ch} from team {team}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing channel {} from team {team}",
        cx.s("id")
            .or_else(|| cx.s("name"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let tid = team_id(cx).await?;
    let cid = match cx.s("id") {
        Some(i) => i,
        None => {
            channel_id_by_name(
                cx,
                &tid,
                "name",
                "The specified channel does not exist in this Microsoft Teams team",
                false,
            )
            .await?
        }
    };
    Request::delete(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}",
        eqp(&tid),
        eqp(&cid)
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub fn set_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(to_js_string(v).to_lowercase() != "general")
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let tid = team_id(cx).await?;
    let cid = match cx.s("id") {
        Some(i) => i,
        None => {
            channel_id_by_name(
                cx,
                &tid,
                "name",
                "The specified channel does not exist in this Microsoft Teams team",
                false,
            )
            .await?
        }
    };
    let data = obj! { "displayName": ?cx.s("newName"), "description": ?cx.s("description") };
    Request::patch(format!(
        "{GRAPH}/v1.0/teams/{}/channels/{}",
        eqp(&tid),
        eqp(&cid)
    ))
    .graph_json()
    .json(data)
    .send()
    .await?;
    Ok(())
}

// ---- chat ----------------------------------------------------------------------

fn emails(v: &str) -> Vec<String> {
    v.trim()
        .to_lowercase()
        .split(',')
        .filter(|e| !e.is_empty())
        .map(str::to_string)
        .collect()
}

pub fn emails_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let e = emails(&to_js_string(v));
    check(!e.is_empty() && e.iter().all(|x| val::is_valid_user_principal_name(x)))
}

async fn chats_by_participants(expected: &[String]) -> Result<Vec<Value>> {
    let t = if expected.len() == 2 {
        "oneOnOne"
    } else {
        "group"
    };
    let chats = odata::get_all_items(&format!("{GRAPH}/v1.0/chats?$filter=chatType eq '{t}'&$expand=members&$select=id,topic,createdDateTime,members")).await?;
    for c in &chats {
        need_arr(c, "members", "length")?;
    }
    Ok(chats
        .into_iter()
        .filter(|c| {
            let m = arr(c, "members");
            m.len() == expected.len() && {
                let mails: Vec<Option<String>> = m
                    .iter()
                    .map(|x| {
                        x.get("email")
                            .and_then(Value::as_str)
                            .map(str::to_lowercase)
                    })
                    .collect();
                expected
                    .iter()
                    .all(|e| mails.iter().any(|m| m.as_deref() == Some(e.as_str())))
            }
        })
        .collect())
}

async fn chats_by_name(name: &str) -> Result<Vec<Value>> {
    odata::get_all_items(&format!(
        "{GRAPH}/v1.0/chats?$filter=topic eq '{}'&$expand=members&$select=id,topic,createdDateTime,chatType",
        eqp(name).replacen('\'', "''", 1)
    ))
    .await
}

fn current_user_email() -> String {
    auth::jwt::user_name(&auth::default_access_token().unwrap_or_default()).to_lowercase()
}

fn pick_chat(chats: Vec<Value>, none: &str, many: &str) -> Result<String> {
    Ok(u(
        &m365_core::runner::pick_one(&chats, "id", Error::msg(none), many)?,
        "id",
    ))
}

pub async fn chat_get(cx: &mut Cx) -> Result<()> {
    let id = if let Some(i) = cx.s("id") {
        i
    } else if let Some(p) = cx.s("participants") {
        let mut all = vec![current_user_email()];
        all.extend(emails(&p));
        let chats = chats_by_participants(&all).await?;
        pick_chat(
            chats,
            "No chat conversation was found with these participants.",
            "Multiple chat conversations with these participants found.",
        )?
    } else {
        let chats = chats_by_name(&cx.u("name")).await?;
        pick_chat(
            chats,
            "No chat conversation was found with this name.",
            "Multiple chat conversations with this name found.",
        )?
    };
    let r = Request::get(format!("{GRAPH}/v1.0/chats/{}", eqp(&id)))
        .graph_json()
        .data()
        .await?;
    cx.log(r)
}

pub async fn chat_list(cx: &mut Cx) -> Result<()> {
    let app =
        auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true);
    let user = cx.s("userId").or_else(|| cx.s("userName"));
    if app && user.is_none() {
        return Err(Error::msg(
            "The option 'userId' or 'userName' is required when obtaining chats using app only permissions",
        ));
    } else if !app && user.is_some() {
        return Err(Error::msg(
            "The options 'userId' or 'userName' cannot be used when obtaining chats using delegated permissions",
        ));
    }
    let mut url = format!(
        "{GRAPH}/v1.0/{}/chats",
        if !app {
            "me".to_string()
        } else {
            format!("users/{}", user.unwrap_or_else(|| "undefined".into()))
        }
    );
    if let Some(t) = cx.s("type") {
        url.push_str(&format!("?$filter=chatType eq '{t}'"));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub fn chat_member_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(!(o.get("visibleHistoryStartDateTime").is_some_and(truthy)
        && o.get("withAllHistory").is_some_and(truthy)))
}

pub async fn chat_member_add(cx: &mut Cx) -> Result<()> {
    let chat = cx.u("chatId");
    cx.verbose_log(&format!(
        "Adding member {} to chat with id {chat}...",
        cx.s("userId")
            .or_else(|| cx.s("userName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let user = cx.s("userId").unwrap_or_else(|| eqp(&cx.u("userName")));
    let visible = if cx.bool("withAllHistory") {
        Some("0001-01-01T00:00:00Z".to_string())
    } else {
        cx.str("visibleHistoryStartDateTime")
    };
    Request::post(format!("{GRAPH}/v1.0/chats/{chat}/members"))
        .graph_json()
        .json(obj! {
            "@odata.type": "#microsoft.graph.aadUserConversationMember",
            "user@odata.bind": format!("https://graph.microsoft.com/v1.0/users/{user}"),
            "visibleHistoryStartDateTime": ?visible,
            "roles": vec![Value::from(cx.s("role").unwrap_or_else(|| "owner".into()))]
        })
        .send()
        .await?;
    Ok(())
}

pub async fn chat_member_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/chats/{}/members", cx.u("chatId"))).await?,
    ))
}

pub async fn chat_member_remove(cx: &mut Cx) -> Result<()> {
    let who = cx.first_u(&["id", "userId", "userName"]);
    let chat = cx.u("chatId");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove member {who} from chat with id {chat}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing member {who} from chat with id {chat}..."
    ));
    let mid = match cx.s("id") {
        Some(i) => i,
        None => {
            let members =
                odata::get_all_items(&format!("{GRAPH}/v1.0/chats/{chat}/members")).await?;
            if let Some(n) = cx.s("userName") {
                match members
                    .iter()
                    .find(|m| s(m, "email").to_lowercase() == n.to_lowercase())
                {
                    Some(m) => u(m, "id"),
                    None => {
                        return Err(Error::msg(format!(
                            "Member with userName '{n}' could not be found in the chat."
                        )));
                    }
                }
            } else {
                let uid = cx.u("userId");
                match members
                    .iter()
                    .find(|m| s(m, "userId").to_lowercase() == uid.to_lowercase())
                {
                    Some(m) => u(m, "id"),
                    None => {
                        return Err(Error::msg(format!(
                            "Member with userId '{uid}' could not be found in the chat."
                        )));
                    }
                }
            }
        }
    };
    Request::delete(format!("{GRAPH}/v1.0/chats/{chat}/members/{mid}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub fn chat_message_list_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(match i {
        1 => !(t("createdEndDateTime") && (t("modifiedStartDateTime") || t("modifiedEndDateTime"))),
        2 => !(t("endDateTime") && (t("modifiedStartDateTime") || t("modifiedEndDateTime"))),
        _ => true,
    })
}

/// Hoists `body.content` to `body` and adds `shortBody` for non-JSON output.
pub fn shorten_bodies(items: &mut [Value]) {
    for i in items.iter_mut() {
        let content = i.get("body").and_then(|b| b.get("content")).cloned();
        let Some(o) = i.as_object_mut() else { continue };
        match &content {
            Some(c) => {
                o.insert("body".into(), c.clone());
            }
            None => {
                o.shift_remove("body");
            }
        }
        let short = content.filter(truthy).map(|c| {
            let b = to_js_string(&c);
            let len = b.encode_utf16().count();
            let (max, dots) = if len < 50 { (len, "") } else { (50, "...") };
            let replaced: Vec<u16> = b.replace('\n', " ").encode_utf16().collect();
            format!(
                "{}{dots}",
                String::from_utf16_lossy(&replaced[..max.min(replaced.len())])
            )
        });
        if let Some(sb) = short {
            o.insert("shortBody".into(), Value::from(sb));
        }
    }
}

pub async fn chat_message_list(cx: &mut Cx) -> Result<()> {
    if cx.s("endDateTime").is_some() {
        cx.warn("Option 'endDateTime' is deprecated. Please use 'createdEndDateTime' instead.");
        let e = cx.opts.get("endDateTime").cloned().unwrap();
        cx.opts.insert("createdEndDateTime".into(), e);
    }
    let mut url = format!("{GRAPH}/v1.0/chats/{}/messages", cx.u("chatId"));
    if let Some(c) = cx.s("createdEndDateTime") {
        url.push_str(&format!(
            "?$filter=createdDateTime lt {c}&$orderby=createdDateTime desc"
        ));
    } else if cx.s("modifiedStartDateTime").is_some() || cx.s("modifiedEndDateTime").is_some() {
        let mut f = Vec::new();
        if let Some(m) = cx.s("modifiedStartDateTime") {
            f.push(format!("lastModifiedDateTime gt {m}"));
        }
        if let Some(m) = cx.s("modifiedEndDateTime") {
            f.push(format!("lastModifiedDateTime lt {m}"));
        }
        url.push_str(&format!(
            "?$filter={}&$orderby=lastModifiedDateTime desc",
            f.join(" and ")
        ));
    }
    let mut items = odata::get_all_items(&url).await?;
    if cx.s("output").is_some_and(|o| o != "json") {
        shorten_bodies(&mut items);
    }
    cx.log(Value::Array(items))
}

pub async fn chat_message_send(cx: &mut Cx) -> Result<()> {
    let chat = if let Some(c) = cx.s("chatId") {
        c
    } else if let Some(e) = cx.s("userEmails") {
        let mut all = vec![current_user_email()];
        all.extend(emails(&e));
        let chats = chats_by_participants(&all).await?;
        if chats.is_empty() {
            let members: Vec<Value> = all
                .iter()
                .map(|m| obj! { "@odata.type": "#microsoft.graph.aadUserConversationMember", "roles": vec![Value::from("owner")], "user@odata.bind": format!("https://graph.microsoft.com/v1.0/users/{m}") })
                .collect();
            let body = obj! { "chatType": if all.len() > 2 { "group" } else { "oneOnOne" }, "members": members };
            let mut retried = 0;
            loop {
                match Request::post(format!("{GRAPH}/v1.0/chats"))
                    .graph_json()
                    .header("content-type", "application/json;odata=nometadata")
                    .json(body.clone())
                    .data()
                    .await
                {
                    Ok(c) => break u(&c, "id"),
                    Err(e) if e.js_message().contains("404") && retried < 4 => retried += 1,
                    Err(e) => return Err(e),
                }
            }
        } else {
            pick_chat(
                chats,
                "",
                "Multiple chat conversations with this name found.",
            )?
        }
    } else {
        let chats = chats_by_name(&cx.u("chatName")).await?;
        pick_chat(
            chats,
            "No chat conversation was found with this name.",
            "Multiple chat conversations with this name found.",
        )?
    };
    Request::post(format!("{GRAPH}/v1.0/chats/{chat}/messages"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "body": obj! { "contentType": cx.s("contentType").unwrap_or_else(|| "text".into()), "content": ?cx.str("message") } })
        .send()
        .await?;
    Ok(())
}

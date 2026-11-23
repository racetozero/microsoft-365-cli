//! `entra group *` and `entra license list`.

use m365_core::prelude::*;
use m365_core::ut::entra;

fn split_trim(s: &str) -> Vec<String> {
    s.split(',').map(|x| x.trim().to_string()).collect()
}

async fn user_ids(
    cx: &Cx,
    ids: Option<String>,
    names: Option<String>,
    verbose_msgs: bool,
) -> Result<Vec<String>> {
    if let Some(i) = ids {
        return Ok(split_trim(&i));
    }
    let Some(n) = names else {
        if verbose_msgs {
            cx.verbose_log("No users to validate, skipping.");
        }
        return Ok(Vec::new());
    };
    if verbose_msgs {
        cx.verbose_log("Retrieving user information.");
    }
    cx.verbose_log("Retrieving ID(s) of user(s)...");
    entra::user::ids_by_upns(&split_trim(&n)).await
}

use entra::members::add_bound;

pub fn add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(
        o.get("type").and_then(Value::as_str) != Some("microsoft365")
            || o.contains_key("visibility"),
    )
}

pub fn mail_nickname_field(_i: u16, v: &Value, _o: &Obj) -> m365_core::schema::FieldCheck {
    m365_core::schema::check({ json::js_len(&to_js_string(v)) <= 64 })
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    let t = cx.u("type");
    let mut body = obj! {
        "displayName": ?cx.str("displayName"),
        "description": ?cx.str("description"),
        "mailNickName": cx.str("mailNickname").unwrap_or_else(|| format!("Group{}", (rand_f64() * 1_000_000.0).floor() as u64)),
        "visibility": cx.str("visibility").unwrap_or_else(|| "Public".into()),
        "groupTypes": if t == "microsoft365" { vec![Value::from("Unified")] } else { vec![] },
        "mailEnabled": t != "security",
        "securityEnabled": true
    };
    cx.add_unknown_options(&mut body);
    let owners = user_ids(cx, cx.s("ownerIds"), cx.s("ownerUserNames"), true).await?;
    let members = user_ids(cx, cx.s("memberIds"), cx.s("memberUserNames"), true).await?;
    let group = Request::post(format!("{GRAPH}/v1.0/groups"))
        .graph_json()
        .json(body)
        .data()
        .await?;
    let id = s(&group, "id").to_string();
    if !owners.is_empty() {
        add_bound(&id, "owners", &owners, false).await?;
    }
    if !members.is_empty() {
        add_bound(&id, "members", &members, false).await?;
    }
    cx.log(group)
}

fn rand_f64() -> f64 {
    let b = uuid::Uuid::new_v4();
    let n = u64::from_le_bytes(b.as_bytes()[..8].try_into().unwrap());
    (n >> 11) as f64 / (1u64 << 53) as f64
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let props = cx.str("properties");
    let g = match cx.s("id") {
        Some(id) => entra::group::by_id(&id, props.as_deref()).await?,
        None => entra::group::by_display_name(&cx.u("displayName"), props.as_deref()).await?,
    };
    cx.log(g)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/groups");
    let mut consistency = false;
    match cx.str("type").as_deref() {
        Some("microsoft365") => url.push_str("?$filter=groupTypes/any(c:c+eq+'Unified')"),
        Some("security") => {
            url.push_str("?$filter=securityEnabled eq true and mailEnabled eq false")
        }
        Some("distribution") => {
            consistency = true;
            url.push_str("?$filter=securityEnabled eq false and mailEnabled eq true and not(groupTypes/any(t:t eq 'Unified'))&$count=true");
        }
        Some("mailEnabledSecurity") => {
            consistency = true;
            url.push_str("?$filter=securityEnabled eq true and mailEnabled eq true and not(groupTypes/any(t:t eq 'Unified'))&$count=true");
        }
        _ => {}
    }
    url.push_str(&qs(&entra::select_query(cx.str("properties").as_deref())
        .into_iter()
        .collect::<Vec<_>>()));
    let mut groups = if consistency {
        odata::get_all_items_req(
            Request::get(url)
                .accept("application/json;odata.metadata=none")
                .header("ConsistencyLevel", "eventual")
                .json_response(),
        )
        .await?
    } else {
        odata::get_all_items(&url).await?
    };
    if cx.str("output").as_deref() == Some("text") {
        for g in groups.iter_mut() {
            let unified = arr(g, "groupTypes")
                .iter()
                .any(|t| t.as_str() == Some("Unified"));
            let mail = g.get("mailEnabled").is_some_and(truthy);
            let sec = g.get("securityEnabled").is_some_and(truthy);
            let t = if unified {
                Some("Microsoft 365")
            } else if mail && sec {
                Some("Mail enabled security")
            } else if sec {
                Some("Security")
            } else if mail {
                Some("Distribution")
            } else {
                None
            };
            if let (Some(t), Some(o)) = (t, g.as_object_mut()) {
                o.insert("groupType".into(), Value::from(t));
            }
        }
    }
    cx.log(Value::Array(groups))
}

async fn group_id(cx: &Cx, msg: &str) -> Result<String> {
    if let Some(id) = cx.s("groupId") {
        return Ok(id);
    }
    let n = cx.u("groupName");
    cx.verbose_log(&msg.replace("{name}", &n));
    entra::group::id_by_display_name(&n).await
}

pub fn member_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(!(t("subgroupIds") || t("subgroupNames"))
        || o.get("role").and_then(Value::as_str) != Some("Owner"))
}

pub async fn member_add(cx: &mut Cx) -> Result<()> {
    let who = cx.first_u(&["userIds", "userNames", "subgroupIds", "subgroupNames"]);
    let grp = cx.first_u(&["groupId", "groupName"]);
    cx.verbose_log(&format!("Adding member(s) {who} to group {grp}..."));
    let gid = group_id(cx, "Retrieving ID of group {name}...").await?;
    let ids = if cx.s("userIds").is_some() || cx.s("userNames").is_some() {
        match cx.s("userIds") {
            Some(i) => split_trim(&i),
            None => {
                cx.verbose_log("Retrieving ID(s) of user(s)...");
                entra::user::ids_by_upns(&split_trim(&cx.u("userNames"))).await?
            }
        }
    } else {
        match cx.s("subgroupIds") {
            Some(i) => split_trim(&i),
            None => {
                cx.verbose_log("Retrieving ID(s) of group(s)...");
                entra::group::ids_by_display_names(&split_trim(&cx.u("subgroupNames"))).await?
            }
        }
    };
    let role = if cx.str("role").as_deref() == Some("Member") {
        "members"
    } else {
        "owners"
    };
    add_bound(&gid, role, &ids, false).await
}

pub async fn member_list(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    let gid = match cx.s("groupId") {
        Some(id) => id,
        None => {
            cx.verbose_log("Retrieving Group Id...");
            entra::group::id_by_display_name(&cx.u("groupName")).await?
        }
    };
    let role = cx.s("role");
    let get_users = move |r: &'static str| {
        let props = cx.s("properties");
        let filter = cx.s("filter");
        let gid = gid.clone();
        async move {
            cx.verbose_log(&format!("Retrieving {r} of the group with id {gid}"));
            let sel = match &props {
                Some(p) => {
                    let mut v: Vec<String> = p
                        .split(',')
                        .filter(|f| f.to_lowercase() != "id")
                        .map(str::to_string)
                        .collect();
                    v.push("id".into());
                    v.iter()
                        .map(|p| p.trim().to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                }
                None => "id,displayName,userPrincipalName,givenName,surname".into(),
            };
            let all: Vec<&str> = sel.split(',').collect();
            let expand: Vec<String> = all
                .iter()
                .filter(|i| i.contains('/'))
                .map(|p| {
                    let parts: Vec<&str> = p.split('/').collect();
                    format!("{}($select={})", parts[0], parts[1])
                })
                .collect();
            let expand = if expand.is_empty() {
                String::new()
            } else {
                format!("&$expand={}", expand.join(","))
            };
            let select: Vec<&str> = all.iter().copied().filter(|i| !i.contains('/')).collect();
            let endpoint = format!(
                "{GRAPH}/v1.0/groups/{gid}/{r}?$select={}{expand}",
                select.join(",")
            );
            match filter {
                Some(f) => {
                    odata::get_all_items_req(
                        Request::get(format!("{endpoint}&$filter={}&$count=true", enc(&f)))
                            .accept("application/json;odata.metadata=none")
                            .header("ConsistencyLevel", "eventual")
                            .json_response(),
                    )
                    .await
                }
                None => odata::get_all_items(&endpoint).await,
            }
        }
    };
    let mut users: Vec<Value> = Vec::new();
    if role.is_none() || role.as_deref() == Some("Owner") {
        for mut o in get_users("Owners").await? {
            o.as_object_mut()
                .map(|x| x.insert("roles".into(), Value::Array(vec!["Owner".into()])));
            users.push(o);
        }
    }
    if role.is_none() || role.as_deref() == Some("Member") {
        for mut m in get_users("Members").await? {
            match users.iter_mut().find(|u| u.get("id") == m.get("id")) {
                Some(u) => {
                    if let Some(Value::Array(r)) = u.get_mut("roles") {
                        r.push("Member".into());
                    }
                }
                None => {
                    m.as_object_mut()
                        .map(|x| x.insert("roles".into(), Value::Array(vec!["Member".into()])));
                    users.push(m);
                }
            }
        }
    }
    cx.log(Value::Array(users))
}

pub fn member_remove_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(
        !(o.contains_key("subgroupIds") || o.contains_key("subgroupNames"))
            || o.get("role")
                .and_then(Value::as_str)
                .is_some_and(|r| r.to_lowercase() == "member"),
    )
}

pub async fn member_remove(cx: &mut Cx) -> Result<()> {
    let principals = cx
        .s("userIds")
        .or_else(|| cx.s("userNames"))
        .or_else(|| cx.s("subgroupIds"))
        .or_else(|| cx.s("subgroupNames"));
    let grp = cx.first_u(&["groupId", "groupName"]);
    if !cx.bool("force") {
        let n = principals.as_deref().unwrap_or("").split(',').count();
        if !cx.confirm(&format!(
            "Are you sure you want to remove {n} principal(s) from group '{grp}'?"
        ))? {
            return Ok(());
        }
    }
    cx.verbose_log(&format!(
        "Removing user(s) {} from group {grp}...",
        principals.clone().unwrap_or_else(|| "undefined".into())
    ));
    let gid = group_id(cx, "Retrieving ID of group '{name}'...").await?;
    let ids: Vec<String> = if let Some(i) = cx.s("userIds") {
        split_trim(&i)
    } else if let Some(i) = cx.s("subgroupIds") {
        split_trim(&i)
    } else if let Some(n) = cx.s("userNames") {
        cx.verbose_log("Retrieving ID(s) of user(s)...");
        entra::user::ids_by_upns(&split_trim(&n)).await?
    } else {
        cx.verbose_log("Retrieving ID(s) of subgroup(s)...");
        let mut v = Vec::new();
        for n in cx.u("subgroupNames").split(',') {
            v.push(entra::group::id_by_display_name(n.trim()).await?);
        }
        v
    };
    let role = cx.s("role");
    let mut endpoints = Vec::new();
    if role.is_none() || role.as_deref() == Some("Owner") {
        endpoints.extend(
            ids.iter()
                .map(|id| format!("/groups/{gid}/owners/{id}/$ref")),
        );
    }
    if role.is_none() || role.as_deref() == Some("Member") {
        endpoints.extend(
            ids.iter()
                .map(|id| format!("/groups/{gid}/members/{id}/$ref")),
        );
    }
    for batch in endpoints.chunks(20) {
        let requests: Vec<Value> = batch
            .iter()
            .enumerate()
            .map(|(i, ep)| obj! { "id": i + 1, "method": "DELETE", "url": ep.as_str(), "headers": obj! { "content-type": "application/json;odata.metadata=none" } })
            .collect();
        let res = Request::post(format!("{GRAPH}/v1.0/$batch"))
            .header("content-type", "application/json;odata.metadata=none")
            .json(obj! { "requests": requests })
            .json_response()
            .data()
            .await?;
        for r in arr(&res, "responses") {
            let st = r.get("status").and_then(Value::as_u64);
            if st != Some(204) && (!cx.bool("suppressNotFound") || st != Some(404)) {
                return Err(Error::Thrown(r.get("body").cloned().unwrap_or(Value::Null)));
            }
        }
    }
    Ok(())
}

pub async fn member_set(cx: &mut Cx) -> Result<()> {
    let who = cx.first_u(&["userIds", "userNames"]);
    let grp = cx.first_u(&["groupId", "groupName"]);
    cx.verbose_log(&format!(
        "Adding member(s) {who} to role {} of group {grp}...",
        cx.u("role")
    ));
    let gid = group_id(cx, "Retrieving ID of group {name}...").await?;
    let ids = match cx.s("userIds") {
        Some(i) => split_trim(&i),
        None => {
            cx.verbose_log("Retrieving ID(s) of user(s)...");
            entra::user::ids_by_upns(&split_trim(&cx.u("userNames"))).await?
        }
    };
    let is_member = cx.str("role").as_deref() == Some("Member");
    add_bound(
        &gid,
        if is_member { "members" } else { "owners" },
        &ids,
        false,
    )
    .await?;
    let current = if is_member { "owners" } else { "members" };
    cx.verbose_log(&format!("Removing members from the old role '{current}'."));
    entra::members::remove_from_role(&gid, current, &ids).await
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "displayName"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove group '{what}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing group {what}..."));
    let id = match cx.s("displayName") {
        Some(n) => entra::group::id_by_display_name(&n).await?,
        None => cx.u("id"),
    };
    Request::delete(format!("{GRAPH}/v1.0/groups/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    let id = match cx.s("displayName") {
        Some(n) => {
            cx.verbose_log("Retrieving group id...");
            entra::group::id_by_display_name(&n).await?
        }
        None => cx.u("id"),
    };
    let description = match cx.str("description") {
        Some(d) if d.is_empty() => Some(Value::Null),
        Some(d) => Some(Value::from(d)),
        None => None,
    };
    let mut body = obj! {
        "displayName": ?cx.str("newDisplayName"),
        "description": ?description,
        "mailNickName": ?cx.str("mailNickname"),
        "visibility": ?cx.str("visibility")
    };
    cx.add_unknown_options(&mut body);
    Request::patch(format!("{GRAPH}/v1.0/groups/{id}"))
        .accept("application/json;odata.metadata=none")
        .json(body)
        .send()
        .await?;
    for (role, ids_k, names_k) in [
        ("owners", "ownerIds", "ownerUserNames"),
        ("members", "memberIds", "memberUserNames"),
    ] {
        let ids = match (cx.s(ids_k), cx.s(names_k)) {
            (Some(i), _) => util::split_and_trim(&i),
            (None, Some(n)) => {
                cx.verbose_log("Retrieving user IDs...");
                entra::user::ids_by_upns(&util::split_and_trim(&n)).await?
            }
            _ => Vec::new(),
        };
        if ids.is_empty() {
            cx.verbose_log(&format!("No {role} to update."));
            continue;
        }
        entra::members::update_users(&id, role, &ids, cx.verbose()).await?;
    }
    Ok(())
}

pub async fn license_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving the commercial subscriptions that an organization has acquired");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/subscribedSkus")).await?,
    ))
}

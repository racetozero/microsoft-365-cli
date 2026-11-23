//! `teams team *` and `teams report pstncalls|directroutingcalls`.

use m365_core::prelude::*;
use m365_core::ut::entra;

const POLLING_MS: u64 = 30_000;

pub fn add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let template = o.get("template").is_some_and(truthy);
    Ok(template || o.contains_key(if i == 0 { "name" } else { "description" }))
}

async fn members_to_add(
    members: &mut Vec<Value>,
    role: &str,
    emails: Option<String>,
    ids: Option<String>,
    names: Option<String>,
) -> Result<()> {
    let items = if let Some(e) = emails {
        entra::user::ids_by_emails(&util::split_and_trim(&e)).await?
    } else if let Some(i) = ids {
        util::split_and_trim(&i)
    } else if let Some(n) = names {
        entra::user::ids_by_upns(&util::split_and_trim(&n)).await?
    } else {
        Vec::new()
    };
    for item in items {
        let bind = format!("https://graph.microsoft.com/v1.0/users('{item}')");
        match members.iter_mut().find(|m| s(m, "user@odata.bind") == bind) {
            Some(m) => {
                if let Some(Value::Array(r)) = m.get_mut("roles") {
                    r.push(Value::from(role));
                }
            }
            None => members.push(obj! { "@odata.type": "#microsoft.graph.aadUserConversationMember", "user@odata.bind": bind, "roles": [role] }),
        }
    }
    Ok(())
}

fn is_owner(m: &Value) -> bool {
    arr(m, "roles").iter().any(|r| r == "owner")
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let app =
        auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true);
    let any_owner = cx.s("ownerUserNames").is_some()
        || cx.s("ownerIds").is_some()
        || cx.s("ownerEmails").is_some();
    if app && !any_owner {
        return Err(Error::msg(
            "Specify at least 'ownerUserNames', 'ownerIds' or 'ownerEmails' when using application permissions.",
        ));
    }
    let mut body = if let Some(t) = cx.str("template") {
        cx.verbose_log("Using template...");
        let mut b = json::js_parse(&t).map_err(Error::msg)?;
        if let Some(n) = cx.s("name") {
            cx.verbose_log(&format!("Using '{n}' as name..."));
            b.as_object_mut()
                .map(|o| o.insert("displayName".into(), Value::from(n)));
        }
        if let Some(d) = cx.s("description") {
            cx.verbose_log(&format!("Using '{d}' as description..."));
            b.as_object_mut()
                .map(|o| o.insert("description".into(), Value::from(d)));
        }
        b
    } else {
        cx.verbose_log(&format!(
            "Creating team with name {} and description {}",
            cx.u("name"),
            cx.u("description")
        ));
        obj! {
            "template@odata.bind": "https://graph.microsoft.com/v1.0/teamsTemplates('standard')",
            "displayName": ?cx.val("name").cloned(),
            "description": ?cx.val("description").cloned(),
        }
    };
    let mut members: Vec<Value> = Vec::new();
    if any_owner {
        members_to_add(
            &mut members,
            "owner",
            cx.s("ownerEmails"),
            cx.s("ownerIds"),
            cx.s("ownerUserNames"),
        )
        .await?;
    }
    if cx.s("memberEmails").is_some()
        || cx.s("memberIds").is_some()
        || cx.s("memberUserNames").is_some()
    {
        members_to_add(
            &mut members,
            "member",
            cx.s("memberEmails"),
            cx.s("memberIds"),
            cx.s("memberUserNames"),
        )
        .await?;
    }
    if app && let Some(pos) = members.iter().position(is_owner) {
        let owner = members.remove(pos);
        body.as_object_mut()
            .map(|o| o.insert("members".into(), Value::Array(vec![owner])));
    }
    let res = Request::post(format!("{GRAPH}/v1.0/teams"))
        .accept("application/json;odata.metadata=none")
        .json(body)
        .bytes_response()
        .send()
        .await?;
    let loc = res.header("location").unwrap_or("undefined").to_string();
    let op_url = format!("{GRAPH}/v1.0{loc}");
    let get_op = || {
        Request::get(op_url.clone())
            .accept("application/json;odata.metadata=minimal")
            .json_response()
            .data()
    };
    let mut op = get_op().await?;
    if !cx.bool("wait") && members.is_empty() {
        return cx.log(op);
    }
    loop {
        if s(&op, "status") == "succeeded" {
            cx.verbose_log("Team provisioned succesfully. Returning...");
            break;
        }
        if let Some(e) = op.get("error").filter(|e| truthy(e)) {
            return Err(Error::Thrown(e.clone()));
        }
        cx.verbose_log(&format!(
            "Team still provisioning. Retrying in {} seconds...",
            POLLING_MS / 1000
        ));
        util::sleep_ms(POLLING_MS).await;
        op = get_op().await?;
    }
    let target = u(&op, "targetResourceId");
    let group = loop {
        match entra::group::by_id(&target, None).await {
            Ok(g) => break g,
            Err(_) => {
                cx.verbose_log(&format!(
                    "Error occurred on retrieving the aad group. Retrying in {} seconds.",
                    POLLING_MS / 1000
                ));
                util::sleep_ms(POLLING_MS).await;
            }
        }
    };
    if !members.is_empty() {
        cx.verbose_log("Adding members to the team...");
        let gid = u(&group, "id");
        for m in members {
            Request::post(format!("{GRAPH}/v1.0/teams/{gid}/members"))
                .header("content-type", "application/json;odata.metadata=none")
                .json(m)
                .json_response()
                .send()
                .await?;
        }
    }
    cx.log(group)
}

async fn team_id_from_get(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("teamId") {
        return Ok(i);
    }
    let o = obj! { "name": ?cx.str("teamName"), "debug": cx.debug(), "verbose": cx.verbose() };
    let (out, _) = m365_core::runner::run_nested(
        "teams team get",
        o.as_object().cloned().unwrap_or_default(),
        true,
    )
    .await?;
    let v = json::parse(&out)?;
    Ok(u(&v, "id"))
}

pub async fn app_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving installed apps for team '{}'",
        cx.s("teamId")
            .or_else(|| cx.s("teamName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let tid = team_id_from_get(cx).await?;
    let res = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/teams/{tid}/installedApps?$expand=teamsApp,teamsAppDefinition"
    ))
    .await?;
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(res));
    }
    let out: Vec<Value> = res
        .iter()
        .map(|i| {
            let t = i.get("teamsApp").cloned().unwrap_or(Value::Null);
            obj! { "id": ?i.get("id").cloned(), "displayName": ?t.get("displayName").cloned(), "distributionMethod": ?t.get("distributionMethod").cloned() }
        })
        .collect();
    cx.log(Value::Array(out))
}

async fn team_id(cx: &Cx) -> Result<String> {
    match cx.s("id") {
        Some(i) => Ok(i),
        None => entra::teams::id_by_display_name(&cx.u("name")).await,
    }
}

pub async fn archive(cx: &mut Cx) -> Result<()> {
    let ro = cx.opt_bool("shouldSetSpoSiteReadOnlyForMembers") == Some(true);
    let id = team_id(cx).await?;
    Request::post(format!("{GRAPH}/v1.0/teams/{}/archive", eqp(&id)))
        .header("content-type", "application/json;odata=nometadata")
        .accept("application/json;odata.metadata=none")
        .json(obj! { "shouldSetSpoSiteReadOnlyForMembers": ro })
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub fn clone_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let s: String = to_js_string(v)
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    check(s.split(',').all(|p| {
        ["apps", "channels", "members", "settings", "tabs"].contains(&p.to_lowercase().as_str())
    }))
}

pub async fn clone(cx: &mut Cx) -> Result<()> {
    let name = cx.u("name");
    let nick: String = name.chars().filter(char::is_ascii_alphanumeric).collect();
    let rnd = u32::from_le_bytes(
        uuid::Uuid::new_v4().as_bytes()[..4]
            .try_into()
            .unwrap_or_default(),
    ) % 9999;
    let data = obj! {
        "displayName": ?cx.val("name").cloned(),
        "mailNickname": format!("{nick}{rnd}"),
        "partsToClone": ?cx.val("partsToClone").cloned(),
        "description": ?cx.s("description"),
        "classification": ?cx.s("classification"),
        "visibility": ?cx.s("visibility"),
    };
    Request::post(format!("{GRAPH}/v1.0/teams/{}/clone", eqp(&cx.u("id"))))
        .header("content-type", "application/json")
        .accept("application/json;odata.metadata=none")
        .json(data)
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let team = match cx.s("id") {
        Some(i) => {
            Request::get(format!("{GRAPH}/v1.0/teams/{}", eqp(&i)))
                .graph_json()
                .data()
                .await?
        }
        None => entra::teams::by_display_name(&cx.u("name")).await?,
    };
    cx.log(team)
}

pub fn list_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(!(t("userId") || t("userName")) || t("joined") || t("associated"))
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let (joined, assoc) = (cx.bool("joined"), cx.bool("associated"));
    let user = cx.s("userId").or_else(|| cx.s("userName"));
    if !joined && !assoc {
        cx.verbose_log("Retrieving Microsoft Teams in the tenant...");
    } else {
        cx.verbose_log(&format!(
            "Retrieving Microsoft Teams {} {}...",
            if joined {
                "joined by"
            } else {
                "associated with"
            },
            user.clone().unwrap_or_else(|| "me".into())
        ));
    }
    let mut ep = format!("{GRAPH}/v1.0");
    if joined || assoc {
        match (cx.s("userId"), cx.s("userName")) {
            (Some(i), _) => ep.push_str(&format!("/users/{i}")),
            (None, Some(n)) => ep.push_str(&format!("/users/{}", eqp(&n))),
            _ => ep.push_str("/me"),
        }
        ep.push_str(if joined {
            "/joinedTeams"
        } else {
            "/teamwork/associatedTeams"
        });
        ep.push_str("?$select=id");
    } else {
        ep.push_str("/groups?$select=id&$filter=resourceProvisioningOptions/Any(x:x eq 'Team')");
    }
    let ids: Vec<String> = odata::get_all_items(&ep)
        .await?
        .iter()
        .map(|g| u(g, "id"))
        .collect();
    cx.verbose_log(&format!(
        "Retrieved {} Microsoft Teams, getting additional information...",
        ids.len()
    ));
    let mut teams: Vec<Value> = Vec::new();
    for chunk in ids.chunks(20) {
        let reqs: Vec<Value> = chunk
            .iter()
            .enumerate()
            .map(|(i, id)| obj! { "id": i.to_string(), "method": "GET", "headers": obj! { "accept": "application/json;odata.metadata=none" }, "url": format!("/teams/{id}") })
            .collect();
        let r = Request::post(format!("{GRAPH}/v1.0/$batch"))
            .accept("application/json;odata.metadata=none")
            .header("content-type", "application/json")
            .json(obj! { "requests": reqs })
            .json_response()
            .data()
            .await?;
        let resp = arr(&r, "responses").to_vec();
        if let Some(bad) = resp
            .iter()
            .find(|i| i.get("status").and_then(Value::as_f64) != Some(200.0))
        {
            return Err(Error::Thrown(
                bad.get("body").cloned().unwrap_or(Value::Null),
            ));
        }
        teams.extend(
            resp.into_iter()
                .map(|r| r.get("body").cloned().unwrap_or(Value::Null)),
        );
    }
    teams.sort_by(|x, y| util::locale_compare(s(x, "displayName"), s(y, "displayName")));
    cx.log(Value::Array(teams))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the team {what}?"),
    )? {
        return Ok(());
    }
    let id = team_id(cx).await?;
    Request::delete(format!("{GRAPH}/v1.0/groups/{}", eqp(&id)))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let data = obj! {
        "displayName": ?cx.s("name"),
        "description": ?cx.s("description"),
        "mailNickName": ?cx.s("mailNickName"),
        "classification": ?cx.s("classification"),
        "visibility": ?cx.s("visibility"),
    };
    Request::patch(format!("{GRAPH}/v1.0/groups/{}", eqp(&cx.u("id"))))
        .graph_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

pub async fn unarchive(cx: &mut Cx) -> Result<()> {
    let id = team_id(cx).await?;
    Request::post(format!("{GRAPH}/v1.0/teams/{}/unarchive", eqp(&id)))
        .header("content-type", "application/json;odata=nometadata")
        .accept("application/json;odata.metadata=none")
        .json_response()
        .send()
        .await?;
    Ok(())
}

// ---- report --------------------------------------------------------------------

pub fn calls_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let now = chrono::Utc::now();
    let to = o.get("toDateTime").filter(|v| truthy(v)).map(to_js_string);
    let to = match to {
        Some(t) => m365_core::time::parse_js_date(&t),
        None => Some(now),
    };
    let from = o
        .get("fromDateTime")
        .map(to_js_string)
        .and_then(|f| m365_core::time::parse_js_date(&f));
    // NaN <= 90 is false
    Ok(match (to, from) {
        (Some(t), Some(f)) => ((t - f).num_milliseconds() as f64 / 86_400_000.0).ceil() <= 90.0,
        _ => false,
    })
}

async fn calls(cx: &Cx, func: &str) -> Result<()> {
    let to = cx.s("toDateTime").unwrap_or_else(m365_core::time::now_iso);
    let r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/communications/callRecords/{func}(fromDateTime={},toDateTime={})",
        eqp(&cx.u("fromDateTime")),
        eqp(&to)
    ))
    .await?;
    cx.log(Value::Array(r))
}

pub async fn report_directroutingcalls(cx: &mut Cx) -> Result<()> {
    calls(cx, "getDirectRoutingCalls").await
}

pub async fn report_pstncalls(cx: &mut Cx) -> Result<()> {
    calls(cx, "getPstnCalls").await
}

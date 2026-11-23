//! `viva connections app create` and `viva engage *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

const YAMMER: &str = "https://www.yammer.com/api";

fn resolve(p: &str) -> String {
    util::resolve(p)
}

pub fn app_create_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let p = resolve(&to_js_string(v));
    if std::path::Path::new(&p).exists() {
        Ok(())
    } else {
        Err(Some(format!("File {p} doesn't exist")))
    }
}

pub fn app_create_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let file = resolve(&format!(
        "{}.zip",
        o.get("name")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    ));
    if std::path::Path::new(&file).exists() && !o.get("force").is_some_and(truthy) {
        return Err(format!(
            "File {file} already exists. Delete the file or use the --force option to overwrite the existing file"
        ));
    }
    Ok(true)
}

pub async fn connections_app_create(cx: &mut Cx) -> Result<()> {
    let portal = cx.u("portalUrl");
    cx.verbose_log(&format!("Checking if site {portal} exists..."));
    let web = Request::get(format!("{portal}/_api/web"))
        .spo_json()
        .data()
        .await?;
    cx.debug_log(&stringify(&web));
    cx.verbose_log(&format!(
        "Site found at {portal}. Checking if it's a communication site..."
    ));
    if s(&web, "WebTemplate") != "SITEPAGEPUBLISHING"
        || web.get("Configuration").and_then(Value::as_f64) != Some(0.0)
    {
        return Err(Error::msg(format!(
            "Site {portal} is not a Communication Site. Please specify a different site and try again."
        )));
    }
    cx.verbose_log(&format!(
        "Site {portal} is a Communication Site. Building app..."
    ));
    let url = url::Url::parse(&portal).map_err(|_| Error::js("Invalid URL"))?;
    let app_portal = format!(
        "{portal}{}app=portals",
        if portal.contains('?') { "&" } else { "?" }
    );
    let domain = url.host_str().unwrap_or("").to_string();
    let mut search = domain.clone();
    let path = url.path();
    if path.contains("/teams") || path.contains("/sites") {
        let segs: Vec<&str> = path.splitn(4, '/').collect();
        if segs.len() >= 3 && !segs[1].is_empty() && !segs[2].is_empty() {
            search.push_str(&format!("/{}/{}", segs[1], segs[2]));
        }
    }
    let colored = resolve(&cx.u("coloredIconPath"));
    let outline = resolve(&cx.u("outlineIconPath"));
    let file_name = |p: &str| {
        std::path::Path::new(p)
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    let (cname, oname) = (file_name(&colored), file_name(&outline));
    let id = util::new_guid();
    let name = cx.u("name");
    let manifest = obj! {
        "$schema": "https://developer.microsoft.com/en-us/json-schemas/teams/v1.9/MicrosoftTeams.schema.json",
        "manifestVersion": "1.9",
        "version": "1.0",
        "id": id.as_str(),
        "packageName": format!("com.microsoft.teams.{name}"),
        "developer": obj! {
            "name": ?cx.val("companyName").cloned(),
            "websiteUrl": ?cx.val("companyWebsiteUrl").cloned(),
            "privacyUrl": cx.s("privacyPolicyUrl").unwrap_or_else(|| "https://privacy.microsoft.com/en-us/privacystatement".into()),
            "termsOfUseUrl": cx.s("termsOfUseUrl").unwrap_or_else(|| "https://go.microsoft.com/fwlink/?linkid=2039674".into()),
        },
        "icons": obj! { "color": cname.as_str(), "outline": oname.as_str() },
        "name": obj! { "short": name.as_str(), "full": name.as_str() },
        "description": obj! { "short": cx.u("description"), "full": cx.u("longDescription") },
        "accentColor": cx.s("accentColor").unwrap_or_else(|| "#40497E".into()),
        "isFullScreen": true,
        "staticTabs": [obj! {
            "entityId": format!("sharepointportal_{id}"),
            "name": format!("Portals-{name}"),
            "contentUrl": format!("https://{domain}/_layouts/15/teamslogon.aspx?spfx=true&dest={app_portal}"),
            "websiteUrl": url.as_str(),
            "searchUrl": format!("https://{search}/_layouts/15/search.aspx?q={{searchQuery}}"),
            "scopes": ["personal"],
            "supportedPlatform": ["desktop"],
        }],
        "permissions": ["identity", "messageTeamMembers"],
        "validDomains": [domain.as_str(), "*.login.microsoftonline.com", "*.sharepoint.com", "*.sharepoint-df.com", "spoppe-a.akamaihd.net", "spoprod-a.akamaihd.net", "resourceseng.blob.core.windows.net", "msft.spoppe.com"],
        "webApplicationInfo": obj! { "id": "00000003-0000-0ff1-ce00-000000000000", "resource": format!("https://{domain}") },
    };
    let text = json::stringify_pretty(&manifest, "  ");
    let read =
        |p: &str| std::fs::read(p).map_err(|e| Error::msg(util::node_fs_error(&e, "open", p)));
    let (c, o) = (read(&colored)?, read(&outline)?);
    let zip = m365_core::zip::write(&[
        ("manifest.json", text.as_bytes()),
        (&cname, &c),
        (&oname, &o),
    ]);
    let out = format!("{name}.zip");
    std::fs::write(&out, zip).map_err(|e| Error::msg(util::node_fs_error(&e, "open", &out)))?;
    Ok(())
}

// ---- communities ---------------------------------------------------------------

async fn community_by_id(id: &str, select: &str) -> Result<Value> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/employeeExperience/communities/{id}?$select={select}"
    ))
    .graph_json()
    .data()
    .await?;
    if !truthy(&r) {
        return Err(Error::js(format!(
            "The specified Viva Engage community with ID '{id}' does not exist."
        )));
    }
    Ok(r)
}

async fn community_by_name(name: &str, select: &str) -> Result<Value> {
    let items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/employeeExperience/communities?$filter=displayName eq '{}'&$select={select}",
        eqp(name)
    ))
    .await?;
    match items.len() {
        0 => Err(Error::js(format!(
            "The specified Viva Engage community '{name}' does not exist."
        ))),
        1 => Ok(items.into_iter().next().unwrap_or(Value::Null)),
        _ => m365_core::runner::handle_multiple_results(
            &format!("Multiple Viva Engage communities with name '{name}' found."),
            &util::array_to_hash_table("id", &items),
        ),
    }
}

async fn community_by_group(group: &str, select: &str) -> Result<Value> {
    let sel = if select.split(',').any(|p| p == "groupId") {
        select.to_string()
    } else {
        format!("{select},groupId")
    };
    let items = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/employeeExperience/communities?$select={sel}"
    ))
    .await?;
    items
        .into_iter()
        .find(|c| s(c, "groupId") == group)
        .ok_or_else(|| Error::js(format!("The Microsoft Entra group with id '{group}' is not associated with any Viva Engage community.")))
}

async fn role_id_by_name(name: &str) -> Result<String> {
    let roles: Vec<Value> = odata::get_all_items(&format!("{GRAPH}/beta/employeeExperience/roles"))
        .await?
        .into_iter()
        .filter(|r| s(r, "displayName").to_lowercase() == name.to_lowercase())
        .collect();
    match roles.len() {
        0 => Err(Error::js(format!(
            "The specified Viva Engage role '{name}' does not exist."
        ))),
        1 => Ok(u(&roles[0], "id")),
        _ => Ok(u(
            &m365_core::runner::handle_multiple_results(
                &format!("Multiple Viva Engage roles with name '{name}' found."),
                &util::array_to_hash_table("id", &roles),
            )?,
            "id",
        )),
    }
}

pub fn community_add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let (k, what, valid): (&str, &str, fn(&str) -> bool) = if i == 2 {
        (
            "adminEntraIds",
            "GUIDs",
            m365_core::validation::is_valid_guid,
        )
    } else {
        (
            "adminEntraUserNames",
            "user principal names",
            m365_core::validation::is_valid_user_principal_name,
        )
    };
    if let Some(v) = o.get(k).filter(|v| truthy(v)).map(to_js_string) {
        let invalid: Vec<&str> = v.split(',').map(str::trim).filter(|x| !valid(x)).collect();
        if !invalid.is_empty() {
            return Err(format!(
                "The following {what} are invalid for the option '{k}': {}",
                invalid.join(", ")
            ));
        }
    }
    Ok(true)
}

pub async fn community_add(cx: &mut Cx) -> Result<()> {
    let app =
        auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true);
    if app && cx.s("adminEntraIds").is_none() && cx.s("adminEntraUserNames").is_none() {
        return Err(Error::msg(
            "Specify at least one admin using either adminEntraIds or adminEntraUserNames options when using application permissions.",
        ));
    }
    cx.verbose_log(&format!(
        "Creating a Viva Engage community with display name '{}'...",
        cx.u("displayName")
    ));
    let ids = if let Some(i) = cx.s("adminEntraIds") {
        util::split_and_trim(&i)
    } else if let Some(n) = cx.s("adminEntraUserNames") {
        entra::user::ids_by_upns(&util::split_and_trim(&n)).await?
    } else {
        Vec::new()
    };
    let mut data = obj! { "displayName": ?cx.val("displayName").cloned(), "description": ?cx.val("description").cloned(), "privacy": ?cx.val("privacy").cloned() };
    if !ids.is_empty() {
        let urls: Vec<String> = ids
            .iter()
            .map(|i| format!("{GRAPH}/beta/users/{i}"))
            .collect();
        data.as_object_mut()
            .map(|o| o.insert("owners@odata.bind".into(), Value::from(urls)));
    }
    let res = Request::post(format!("{GRAPH}/beta/employeeExperience/communities"))
        .graph_json()
        .header("content-type", "application/json")
        .json(data)
        .send()
        .await?;
    let Some(location) = res.header("location").map(String::from) else {
        return if cx.bool("wait") {
            Err(Error::js("Invalid URL"))
        } else {
            cx.log_undefined()
        };
    };
    if !cx.bool("wait") {
        return cx.log(Value::from(location));
    }
    loop {
        cx.verbose_log("Community still provisioning. Retrying in 5 seconds...");
        util::sleep_ms(5000).await;
        cx.verbose_log("Checking create community operation status...");
        let op = Request::get(location.clone()).graph_json().data().await?;
        let status = op
            .get("status")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        cx.verbose_log(&format!("Community creation operation status: {status}"));
        match status.as_str() {
            "failed" => {
                return Err(Error::msg(format!(
                    "Community creation failed: {}",
                    u(&op, "statusDetail")
                )));
            }
            "succeeded" => {
                cx.log(op)?;
                return Ok(());
            }
            "notStarted" | "running" => {}
            _ => return Ok(()),
        }
    }
}

pub async fn community_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!(
        "Getting the information of Viva Engage community with id '{id}'..."
    ));
    cx.log(
        Request::get(format!("{GRAPH}/beta/employeeExperience/communities/{id}"))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn community_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting all Viva Engage communities...");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/employeeExperience/communities")).await?,
    ))
}

async fn community_id(cx: &Cx) -> Result<String> {
    if let Some(n) = cx.s("displayName") {
        return Ok(u(&community_by_name(&n, "id").await?, "id"));
    }
    if let Some(g) = cx.s("entraGroupId") {
        return Ok(u(&community_by_group(&g, "id").await?, "id"));
    }
    Ok(cx.u("id"))
}

pub async fn community_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .or_else(|| cx.s("entraGroupId"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove Viva Engage community '{what}'?"),
    )? {
        return Ok(());
    }
    let id = community_id(cx).await?;
    cx.verbose_log(&format!("Removing Viva Engage community with ID {id}..."));
    Request::delete(format!("{GRAPH}/v1.0/employeeExperience/communities/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn community_set(cx: &mut Cx) -> Result<()> {
    // resolved outside the command's try/catch
    let id = community_id(cx).await.map_err(Error::uncaught)?;
    cx.verbose_log(&format!("Updating Viva Engage community with ID {id}..."));
    Request::patch(format!("{GRAPH}/v1.0/employeeExperience/communities/{id}"))
        .graph_json()
        .json(obj! { "description": ?cx.val("description").cloned(), "displayName": ?cx.val("newDisplayName").cloned(), "privacy": ?cx.val("privacy").cloned() })
        .send()
        .await?;
    Ok(())
}

/// The Entra group of the community; `communityId` wins over the display name.
async fn group_id(cx: &Cx, id_last: bool) -> Result<String> {
    let mut g = cx.s("entraGroupId");
    if id_last {
        if let Some(n) = cx.s("communityDisplayName") {
            g = Some(u(&community_by_name(&n, "groupId").await?, "groupId"));
        }
        if let Some(i) = cx.s("communityId") {
            g = Some(u(&community_by_id(&i, "groupId").await?, "groupId"));
        }
    } else if let Some(n) = cx.s("communityDisplayName") {
        g = Some(u(&community_by_name(&n, "groupId").await?, "groupId"));
    } else if let Some(i) = cx.s("communityId") {
        g = Some(u(&community_by_id(&i, "groupId").await?, "groupId"));
    }
    Ok(g.unwrap_or_else(|| "undefined".into()))
}

pub async fn community_user_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Adding users to community...");
    let gid = group_id(cx, true).await?;
    let ids = match cx.s("ids") {
        Some(i) => util::split_and_trim(&i),
        None => entra::user::ids_by_upns(&util::split_and_trim(&cx.u("userNames"))).await?,
    };
    let role = if cx.str("role").as_deref() == Some("Member") {
        "members"
    } else {
        "owners"
    };
    for batch in ids.chunks(400) {
        let reqs: Vec<Value> = batch
            .chunks(20)
            .enumerate()
            .map(|(j, chunk)| {
                let mut body = Obj::new();
                body.insert(format!("{role}@odata.bind"), Value::from(chunk.iter().map(|u| format!("{GRAPH}/v1.0/directoryObjects/{u}")).collect::<Vec<_>>()));
                obj! { "id": j * 20 + 1, "method": "PATCH", "url": format!("/groups/{gid}"), "headers": obj! { "content-type": "application/json;odata.metadata=none" }, "body": body }
            })
            .collect();
        let r = Request::post(format!("{GRAPH}/v1.0/$batch"))
            .header("content-type", "application/json;odata.metadata=none")
            .json(obj! { "requests": reqs })
            .json_response()
            .data()
            .await?;
        for resp in arr(&r, "responses") {
            if resp.get("status").and_then(Value::as_f64) != Some(204.0) {
                return Err(Error::Thrown(
                    resp.get("body").cloned().unwrap_or(Value::Null),
                ));
            }
        }
    }
    Ok(())
}

pub async fn community_user_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting list of users in community...");
    let gid = group_id(cx, true).await?;
    let get = |kind: &str| {
        odata::get_all_items_req(
            Request::get(format!("{GRAPH}/v1.0/groups/{gid}/{kind}")).graph_json(),
        )
    };
    let members = get("members").await?;
    let owners = get("owners").await?;
    let tag = |items: Vec<Value>, role: &str| -> Vec<Value> {
        items
            .into_iter()
            .map(|m| crate::planner::merge(m, &obj! { "roles": [role] }))
            .collect()
    };
    let (m, o) = (tag(members, "Member"), tag(owners, "Admin"));
    let users = match cx.str("role").as_deref() {
        Some("Member") => m,
        Some("Admin") => o,
        Some(_) => Vec::new(),
        None => o.into_iter().chain(m).collect(),
    };
    cx.log(Value::Array(users))
}

pub async fn community_user_remove(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("id")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    let community = cx
        .s("communityDisplayName")
        .or_else(|| cx.s("communityId"))
        .or_else(|| cx.s("entraGroupId"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the user {who} from the community {community}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log("Removing user from community...");
    let gid = group_id(cx, false).await?;
    let uid = match cx.s("id") {
        Some(i) => i,
        None => entra::user::id_by_upn(&cx.u("userName")).await?,
    };
    for role in ["owners", "members"] {
        if let Err(e) = Request::delete(format!("{GRAPH}/v1.0/groups/{gid}/{role}/{uid}/$ref"))
            .graph_json()
            .send()
            .await
            && e.status() != Some(404)
        {
            return Err(match e.body() {
                Some(b) => Error::Thrown(b.clone()),
                None => e,
            });
        }
    }
    Ok(())
}

// ---- Yammer API ----------------------------------------------------------------

fn yammer(method: Method, path: &str) -> Request {
    Request::new(method, format!("{YAMMER}{path}"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json;odata=nometadata")
        .json_response()
}

pub async fn message_add(cx: &mut Cx) -> Result<()> {
    let r = yammer(Method::Post, "/v1/messages.json")
        .json(obj! {
            "body": ?cx.val("body").cloned(),
            "replied_to_id": ?cx.val("repliedToId").cloned(),
            "direct_to_user_ids": ?cx.val("directToUserIds").cloned(),
            "group_id": ?cx.val("groupId").cloned(),
            "network_id": ?cx.val("networkId").cloned(),
        })
        .data()
        .await?;
    let m = arr(&r, "messages");
    cx.log(if m.len() == 1 {
        m[0].clone()
    } else {
        Value::Null
    })
}

pub async fn message_get(cx: &mut Cx) -> Result<()> {
    cx.log(
        yammer(Method::Get, &format!("/v1/messages/{}.json", cx.u("id")))
            .data()
            .await?,
    )
}

pub async fn message_like_set(cx: &mut Cx) -> Result<()> {
    let enable = cx.opt_bool("enable") != Some(false);
    if !enable
        && !cx.proceed(
            "force",
            &format!(
                "Are you sure you want to unlike message {}?",
                cx.u("messageId")
            ),
        )?
    {
        return Ok(());
    }
    let req = yammer(
        if enable { Method::Post } else { Method::Delete },
        "/v1/messages/liked_by/current.json",
    )
    .json(obj! { "message_id": ?cx.val("messageId").cloned() });
    req.send().await?;
    Ok(())
}

pub async fn message_list(cx: &mut Cx) -> Result<()> {
    let mut items: Vec<Value> = Vec::new();
    let mut older: Option<String> = None;
    let limit = cx.num("limit").filter(|l| *l != 0.0 && !l.is_nan());
    loop {
        let mut ep = String::from("/v1");
        if let Some(t) = cx.s("threadId") {
            ep.push_str(&format!("/messages/in_thread/{t}.json"));
        } else if let Some(g) = cx.s("groupId") {
            ep.push_str(&format!("/messages/in_group/{g}.json"));
        } else {
            ep.push_str(match cx.s("feedType").as_deref() {
                Some("Top") => "/messages/algo.json",
                Some("My") => "/messages/my_feed.json",
                Some("Following") => "/messages/following.json",
                Some("Sent") => "/messages/sent.json",
                Some("Private") => "/messages/private.json",
                Some("Received") => "/messages/received.json",
                _ => "/messages.json",
            });
        }
        if let Some(o) = &older {
            ep.push_str(&format!("?older_than={o}"));
        } else if let Some(o) = cx.s("olderThanId") {
            ep.push_str(&format!("?older_than={o}"));
        }
        if cx.bool("threaded") {
            ep.push_str(if ep.contains('?') { "&" } else { "?" });
            ep.push_str("threaded=true");
        }
        let r = yammer(Method::Get, &ep).data().await?;
        match r.get("messages") {
            Some(Value::Array(a)) => items.extend(a.iter().cloned()),
            Some(v) => items.push(v.clone()),
            None => items.push(Value::Null),
        }
        if let Some(l) = limit
            && items.len() as f64 > l
        {
            items.truncate(l.max(0.0) as usize);
            break;
        }
        let more = r
            .get("meta")
            .ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'older_available')")
            })?
            .get("older_available")
            == Some(&Value::Bool(true));
        if !more {
            break;
        }
        older = Some(
            items
                .last()
                .map(|m| u(m, "id"))
                .unwrap_or_else(|| "undefined".into()),
        );
    }
    for m in items.iter_mut() {
        let plain = m
            .get("body")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'plain')"))?
            .get("plain")
            .filter(|p| truthy(p))
            .map(to_js_string);
        let short = plain.map(|p| {
            let (max, dots) = if json::js_len(&p) < 35 {
                (json::js_len(&p), "")
            } else {
                (35, "...")
            };
            let flat = p.replace('\n', " ");
            format!(
                "{}{dots}",
                String::from_utf16_lossy(&flat.encode_utf16().take(max).collect::<Vec<u16>>())
            )
        });
        if let Some(o) = m.as_object_mut() {
            match short {
                Some(s) => o.insert("shortBody".into(), Value::from(s)),
                None => o.shift_remove("shortBody"),
            };
        }
    }
    cx.log(Value::Array(items))
}

pub async fn message_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the Viva Engage message {id}?"),
    )? {
        return Ok(());
    }
    yammer(Method::Delete, &format!("/v1/messages/{id}.json"))
        .send()
        .await?;
    Ok(())
}

pub async fn network_list(cx: &mut Cx) -> Result<()> {
    let r = yammer(Method::Get, "/v1/networks/current.json")
        .json(obj! { "includeSuspended": cx.opt_bool("withSuspended").unwrap_or(false) })
        .data()
        .await?;
    cx.log(r)
}

pub async fn role_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting all Viva Engage roles...");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/beta/employeeExperience/roles")).await?,
    ))
}

async fn role_and_user(cx: &Cx) -> Result<(String, String)> {
    let mut role = cx.u("roleId");
    let mut user = cx.u("userId");
    if let Some(n) = cx.s("roleName") {
        cx.verbose_log(&format!(
            "Retrieving Viva Engage role ID for role name '{n}'..."
        ));
        role = role_id_by_name(&n).await?;
    }
    if let Some(n) = cx.s("userName") {
        cx.verbose_log(&format!(
            "Retrieving Viva Engage user ID for user name '{n}'..."
        ));
        user = entra::user::id_by_upn(&n).await?;
    }
    Ok((role, user))
}

pub async fn role_member_add(cx: &mut Cx) -> Result<()> {
    let (role, user) = role_and_user(cx).await?;
    cx.verbose_log(&format!(
        "Assigning user {user} to a Viva Engage role {role}..."
    ));
    Request::post(format!(
        "{GRAPH}/beta/employeeExperience/roles/{role}/members"
    ))
    .header("content-type", "application/json;odata=nometadata")
    .graph_json()
    .json(obj! { "user@odata.bind": format!("{GRAPH}/beta/users('{user}')") })
    .send()
    .await?;
    Ok(())
}

pub async fn role_member_list(cx: &mut Cx) -> Result<()> {
    let mut role = cx.u("roleId");
    if let Some(n) = cx.s("roleName") {
        cx.verbose_log(&format!(
            "Retrieving Viva Engage role ID for role name '{n}'..."
        ));
        role = role_id_by_name(&n).await?;
    }
    cx.verbose_log(&format!(
        "Getting all users assigned to a Viva Engage role {role}..."
    ));
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/beta/employeeExperience/roles/{role}/members"
        ))
        .await?,
    ))
}

pub async fn role_member_remove(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("userId")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    let role = cx
        .s("roleId")
        .or_else(|| cx.s("roleName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove user '{who}' from role '{role}'?"),
    )? {
        return Ok(());
    }
    let (role, user) = role_and_user(cx).await?;
    cx.verbose_log(&format!(
        "Removing user {user} from a Viva Engage role {role}..."
    ));
    Request::delete(format!(
        "{GRAPH}/v1.0/employeeExperience/roles/{role}/members/{user}"
    ))
    .accept("application/json;odata.metadata=none")
    .send()
    .await?;
    Ok(())
}

pub async fn search(cx: &mut Cx) -> Result<()> {
    let q = cx.u("queryText");
    let limit = cx.num("limit").filter(|l| *l != 0.0 && !l.is_nan());
    let mut summary = obj! { "messages": 0, "topics": 0, "users": 0, "groups": 0 };
    let (mut messages, mut topics, mut groups, mut users): (
        Vec<Value>,
        Vec<Value>,
        Vec<Value>,
        Vec<Value>,
    ) = Default::default();
    let mut page = 1;
    loop {
        let r = Request::get(format!(
            "{YAMMER}/v1/search.json?search={}&page={page}",
            eqp(&q)
        ))
        .accept("application/json;odata=nometadata")
        .json_response()
        .data()
        .await?;
        let undef = |what: &str| {
            Error::js(format!(
                "Cannot read properties of undefined (reading '{what}')"
            ))
        };
        if page == 1 {
            let c = r.get("count").ok_or_else(|| undef("messages"))?;
            summary = obj! { "messages": ?c.get("messages").cloned(), "topics": ?c.get("topics").cloned(), "users": ?c.get("users").cloned(), "groups": ?c.get("groups").cloned() };
        }
        let list = |v: Option<&Value>, what: &str| -> Result<Vec<Value>> {
            match v {
                Some(Value::Array(a)) => Ok(a.clone()),
                Some(_) => Ok(Vec::new()),
                None => Err(undef(what)),
            }
        };
        let rm = list(
            r.get("messages")
                .ok_or_else(|| undef("messages"))?
                .get("messages"),
            "length",
        )?;
        let rt = list(r.get("topics"), "length")?;
        let rg = list(r.get("groups"), "length")?;
        let ru = list(r.get("users"), "length")?;
        let add = |acc: &mut Vec<Value>, new: &[Value]| {
            if !new.is_empty() {
                acc.extend(new.iter().cloned());
                if let Some(l) = limit
                    && acc.len() as f64 > l
                {
                    acc.truncate(l.max(0.0) as usize);
                }
            }
        };
        add(&mut messages, &rm);
        add(&mut topics, &rt);
        add(&mut groups, &rg);
        add(&mut users, &ru);
        if !(rm.len() == 20 || ru.len() == 20 || rg.len() == 20 || rt.len() == 20) {
            break;
        }
        page += 1;
    }
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(obj! { "summary": summary, "messages": messages, "users": users, "topics": topics, "groups": groups });
    }
    let show = cx.str("show").map(|s| s.to_lowercase());
    if show.as_deref() == Some("summary") {
        return cx.log(summary);
    }
    let want = |k: &str| show.is_none() || show.as_deref() == Some(k);
    let mut results = Vec::new();
    if want("messages") {
        for m in &messages {
            let d = m.get("content_excerpt").map(to_js_string).map(|t| {
                let t = if json::js_len(&t) >= 80 {
                    format!(
                        "{}...",
                        String::from_utf16_lossy(&t.encode_utf16().take(80).collect::<Vec<u16>>())
                    )
                } else {
                    t
                };
                t.replace('\n', " ")
            });
            results.push(obj! { "id": ?m.get("id").cloned(), "description": ?d, "type": "message", "web_url": ?m.get("web_url").cloned() });
        }
    }
    for (k, items, ty) in [
        ("topics", &topics, "topic"),
        ("users", &users, "user"),
        ("groups", &groups, "group"),
    ] {
        if want(k) {
            for i in items {
                results.push(obj! { "id": ?i.get("id").cloned(), "description": ?i.get("name").cloned(), "type": ty, "web_url": ?i.get("web_url").cloned() });
            }
        }
    }
    cx.log(Value::Array(results))
}

pub async fn user_get(cx: &mut Cx) -> Result<()> {
    let ep = if let Some(i) = cx.s("id") {
        format!("/v1/users/{i}.json")
    } else if let Some(e) = cx.s("email") {
        format!("/v1/users/by_email.json?email={}", eqp(&e))
    } else {
        "/v1/users/current.json".into()
    };
    cx.log(yammer(Method::Get, &ep).data().await?)
}

pub async fn user_list(cx: &mut Cx) -> Result<()> {
    let mut items: Vec<Value> = Vec::new();
    let limit = cx.num("limit");
    let mut page = 1;
    loop {
        let mut ep = match cx.str("groupId") {
            Some(g) => format!("/v1/users/in_group/{g}.json"),
            None => "/v1/users.json".into(),
        };
        ep.push_str(&format!("?page={page}"));
        if cx.bool("reverse") {
            ep.push_str("&reverse=true");
        }
        if let Some(s) = cx.str("sortBy") {
            ep.push_str(&format!("&sort_by={s}"));
        }
        if let Some(l) = cx.str("letter") {
            ep.push_str(&format!("&letter={l}"));
        }
        let r = yammer(Method::Get, &ep).data().await?;
        let out = r
            .get("users")
            .filter(|u| truthy(u))
            .cloned()
            .unwrap_or_else(|| r.clone());
        match out {
            Value::Array(a) => items.extend(a),
            other => items.push(other),
        }
        if let Some(l) = limit
            && items.len() as f64 > l
        {
            items.truncate(l.max(0.0) as usize);
            break;
        }
        if r.get("more_available") == Some(&Value::Bool(true)) || items.len().is_multiple_of(50) {
            page += 1;
            continue;
        }
        break;
    }
    cx.log(Value::Array(items))
}

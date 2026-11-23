//! `entra m365group *`.

use m365_core::prelude::*;
use m365_core::ut::{entra, spo};

fn not_unified(id: &str) -> Error {
    Error::js(format!(
        "Specified group with id '{id}' is not a Microsoft 365 group."
    ))
}

async fn ensure_unified(id: &str) -> Result<()> {
    if entra::group::is_unified(id).await? {
        Ok(())
    } else {
        Err(not_unified(id))
    }
}

fn logo_message(v: &Value) -> String {
    let full = std::path::absolute(to_js_string(v)).unwrap_or_default();
    if full.exists() {
        format!("Path '{}' points to a directory", full.display())
    } else {
        format!("File '{}' not found", full.display())
    }
}

fn logo_ok(v: &Value) -> bool {
    let full = std::path::absolute(to_js_string(v)).unwrap_or_default();
    full.exists() && !full.is_dir()
}

fn first_without_at(v: &Value) -> String {
    to_js_string(v)
        .split(',')
        .map(str::trim)
        .find(|o| !o.contains('@'))
        .unwrap_or("undefined")
        .to_string()
}

pub fn add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    match i {
        0 | 1 => {
            if to_js_string(v)
                .split(',')
                .map(str::trim)
                .all(|o| o.contains('@'))
            {
                Ok(())
            } else {
                Err(Some(format!(
                    "{} is not a valid userPrincipalName",
                    first_without_at(v)
                )))
            }
        }
        _ => {
            if logo_ok(v) {
                Ok(())
            } else {
                Err(Some(logo_message(v)))
            }
        }
    }
}

pub fn add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(!o
        .get("mailNickname")
        .map(to_js_string)
        .unwrap_or_default()
        .contains(' '))
}

fn image_content_type(path: &str) -> &'static str {
    let ext = path
        .rfind('.')
        .map(|i| path[i..].to_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        ".png" => "image/png",
        ".gif" => "image/gif",
        _ => "image/jpeg",
    }
}

async fn set_logo(group_id: &str, full: &str) -> Result<()> {
    let bytes = std::fs::read(full)?;
    let mut retry = 15;
    loop {
        let r = Request::put(format!("{GRAPH}/v1.0/groups/{group_id}/photo/$value"))
            .header("content-type", image_content_type(full))
            .bytes(bytes.clone())
            .send()
            .await;
        match r {
            Ok(_) => return Ok(()),
            Err(e) => {
                retry -= 1;
                if retry > 0 {
                    util::sleep_ms(500 * (15 - retry) as u64).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    cx.verbose_log("Creating Microsoft 365 Group...");
    let mut behaviors = Vec::new();
    for (k, v) in [
        ("allowMembersToPost", "AllowOnlyMembersToPost"),
        ("hideGroupInOutlook", "HideGroupInOutlook"),
        ("subscribeNewGroupMembers", "SubscribeNewGroupMembers"),
        ("welcomeEmailDisabled", "WelcomeEmailDisabled"),
    ] {
        if cx.bool(k) {
            behaviors.push(Value::from(v));
        }
    }
    let body = obj! {
        "description": ?cx.str("description"),
        "displayName": ?cx.str("displayName"),
        "groupTypes": vec![Value::from("Unified")],
        "mailEnabled": true,
        "mailNickname": ?cx.str("mailNickname"),
        "resourceBehaviorOptions": behaviors,
        "securityEnabled": false,
        "visibility": cx.s("visibility").unwrap_or_else(|| "Public".into())
    };
    let get_ids = |users: Option<String>| async move {
        let Some(users) = users else {
            cx.debug_log("No users to validate, skipping.");
            return Ok::<Vec<String>, Error>(Vec::new());
        };
        cx.verbose_log("Retrieving user information.");
        let arr_users: Vec<String> = users.split(',').map(|o| o.trim().to_string()).collect();
        let mut res = Vec::new();
        for u in &arr_users {
            res.push(
                Request::get(format!("{GRAPH}/v1.0/users?$filter=userPrincipalName eq '{}'&$select=id,userPrincipalName", eqp(u)))
                    .header("content-type", "application/json")
                    .json_response()
                    .data()
                    .await?,
            );
        }
        let upns: Vec<Option<String>> = res
            .iter()
            .map(|r| {
                arr(r, "value")
                    .first()
                    .and_then(|v| v.get("userPrincipalName"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .collect();
        let ids: Vec<String> = res
            .iter()
            .map(|r| {
                arr(r, "value")
                    .first()
                    .map(|v| s(v, "id").to_string())
                    .unwrap_or_else(|| "undefined".into())
            })
            .collect();
        let invalid: Vec<&String> = arr_users
            .iter()
            .filter(|u| {
                !upns
                    .iter()
                    .flatten()
                    .any(|p| p.to_lowercase() == u.to_lowercase())
            })
            .collect();
        if !invalid.is_empty() {
            return Err(Error::msg(format!(
                "Cannot proceed with group creation. The following users provided are invalid : {}",
                invalid
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            )));
        }
        Ok(ids)
    };
    let owners = get_ids(cx.s("owners")).await?;
    let members = get_ids(cx.s("members")).await?;
    let group = Request::post(format!("{GRAPH}/v1.0/groups"))
        .graph_json()
        .json(body)
        .data()
        .await?;
    let gid = s(&group, "id").to_string();
    match cx.s("logoPath") {
        None => cx.debug_log("logoPath not set. Skipping"),
        Some(p) => {
            let full = std::path::absolute(&p)
                .unwrap_or_default()
                .display()
                .to_string();
            cx.verbose_log(&format!("Setting group logo {full}..."));
            set_logo(&gid, &full).await?;
        }
    }
    for (ids, role) in [(&owners, "owners"), (&members, "members")] {
        for id in ids {
            Request::post(format!("{GRAPH}/v1.0/groups/{gid}/{role}/$ref"))
                .header("content-type", "application/json")
                .json(obj! { "@odata.id": format!("https://graph.microsoft.com/v1.0/users/{id}") })
                .json_response()
                .send()
                .await?;
        }
    }
    cx.log(group)
}

pub async fn conversation_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving conversations for Microsoft 365 Group: {}...",
        cx.s("groupId")
            .or_else(|| cx.s("groupName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let gid = match cx.s("groupName") {
        Some(n) => entra::group::id_by_display_name(&n).await?,
        None => cx.u("groupId"),
    };
    ensure_unified(&gid).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/groups/{gid}/conversations")).await?,
    ))
}

pub async fn conversation_post_list(cx: &mut Cx) -> Result<()> {
    let gid = match cx.s("groupId") {
        Some(i) => eqp(&i),
        None => s(
            &entra::group::by_display_name(&cx.u("groupName"), None).await?,
            "id",
        )
        .to_string(),
    };
    ensure_unified(&gid).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/groups/{gid}/threads/{}/posts",
            cx.u("threadId")
        ))
        .await?,
    ))
}

fn site_url_of(web_url: Option<&str>) -> String {
    match web_url.filter(|w| !w.is_empty()) {
        Some(w) => w[..w.rfind('/').unwrap_or(0)].to_string(),
        None => String::new(),
    }
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let mut group = match cx.s("id") {
        Some(id) => entra::group::by_id(&id, None).await?,
        None => entra::group::by_display_name(&cx.u("displayName"), None).await?,
    };
    let gid = s(&group, "id").to_string();
    ensure_unified(&gid).await?;
    let ext = Request::get(format!(
        "{GRAPH}/v1.0/groups/{gid}?$select=allowExternalSenders,autoSubscribeNewMembers,hideFromAddressLists,hideFromOutlookClients,isSubscribedByMail"
    ))
    .graph_json()
    .data()
    .await?;
    if let (Some(g), Some(e)) = (group.as_object_mut(), ext.as_object()) {
        for (k, v) in e {
            g.insert(k.clone(), v.clone());
        }
    }
    if cx.bool("withSiteUrl") {
        let r = Request::get(format!("{GRAPH}/v1.0/groups/{gid}/drive?$select=webUrl"))
            .graph_json()
            .data()
            .await?;
        group.as_object_mut().map(|g| {
            g.insert(
                "siteUrl".into(),
                Value::from(site_url_of(r.get("webUrl").and_then(Value::as_str))),
            )
        });
    }
    cx.log(group)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let dn = cx
        .s("displayName")
        .map(|d| format!(" and startswith(DisplayName,'{}')", eqp(&d)))
        .unwrap_or_default();
    let mn = cx
        .s("mailNickname")
        .map(|d| format!(" and startswith(MailNickname,'{}')", eqp(&d)))
        .unwrap_or_default();
    let exp = if cx.bool("orphaned") {
        "&$expand=owners"
    } else {
        ""
    };
    let mut groups = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/groups?$filter=groupTypes/any(c:c+eq+'Unified'){dn}{mn}{exp}&$top=100"
    ))
    .await?;
    if cx.bool("orphaned") {
        groups.retain(|g| arr(g, "owners").is_empty());
    }
    if cx.bool("withSiteUrl") {
        for g in groups.iter_mut() {
            let id = s(g, "id").to_string();
            let r = Request::get(format!("{GRAPH}/v1.0/groups/{id}/drive?$select=webUrl"))
                .graph_json()
                .data()
                .await?;
            g.as_object_mut().map(|o| {
                o.insert(
                    "siteUrl".into(),
                    Value::from(site_url_of(r.get("webUrl").and_then(Value::as_str))),
                )
            });
        }
    }
    cx.log(Value::Array(groups))
}

pub async fn recyclebinitem_clear(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to clear all M365 Groups from recycle bin?",
    )? {
        return Ok(());
    }
    let items = odata::get_all_items(&format!("{GRAPH}/v1.0/directory/deletedItems/Microsoft.Graph.Group?$filter=groupTypes/any(c:c+eq+'Unified')&$top=100")).await?;
    for g in items {
        Request::delete(format!(
            "{GRAPH}/v1.0/directory/deletedItems/{}",
            s(&g, "id")
        ))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    }
    Ok(())
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    let dn = cx
        .s("groupName")
        .map(|d| {
            format!(
                " and startswith(DisplayName,'{}')",
                eqp(&d).replace('\'', "''")
            )
        })
        .unwrap_or_default();
    let mn = cx
        .s("groupMailNickname")
        .map(|d| {
            format!(
                " and startswith(MailNickname,'{}')",
                eqp(&d).replace('\'', "''")
            )
        })
        .unwrap_or_default();
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/directory/deletedItems/Microsoft.Graph.Group?$filter=groupTypes/any(c:c+eq+'Unified'){dn}{mn}&$top=100")).await?,
    ))
}

async fn deleted_group_id(cx: &Cx, as_error: bool) -> Result<String> {
    if let Some(id) = cx.s("id") {
        return Ok(id);
    }
    let (dn, mn) = (cx.s("displayName"), cx.s("mailNickname"));
    let mut filter = String::new();
    if let Some(d) = &dn {
        filter = format!("displayName eq '{}'", eqp(d));
    }
    if let Some(m) = &mn {
        filter = format!("mailNickname eq '{}'", eqp(m));
    }
    let r = Request::get(format!(
        "{GRAPH}/v1.0/directory/deletedItems/Microsoft.Graph.Group?$filter={filter}"
    ))
    .graph_json()
    .data()
    .await?;
    let groups = arr(&r, "value");
    let label = dn.or(mn).unwrap_or_else(|| "undefined".into());
    match groups.len() {
        0 => {
            let m = format!("The specified group '{label}' does not exist.");
            Err(if as_error {
                Error::js(m)
            } else {
                Error::msg(m)
            })
        }
        1 => Ok(s(&groups[0], "id").to_string()),
        _ => Ok(s(
            &cx.handle_multiple_results(
                &format!("Multiple groups with name '{label}' found."),
                &util::array_to_hash_table("id", groups),
            )?,
            "id",
        )
        .to_string()),
    }
}

pub async fn recyclebinitem_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .or_else(|| cx.s("mailNickname"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the group '{what}'?"),
    )? {
        return Ok(());
    }
    let id = deleted_group_id(cx, true).await?;
    Request::delete(format!("{GRAPH}/v1.0/directory/deletedItems/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .or_else(|| cx.s("mailNickname"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Restoring Microsoft 365 Group: {what}..."));
    let id = deleted_group_id(cx, false).await?;
    Request::post(format!("{GRAPH}/v1.0/directory/deleteditems/{id}/restore"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json")
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the group {what}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing Microsoft 365 Group: {what}..."));
    let gid = match cx.s("displayName") {
        Some(n) => entra::group::id_by_display_name(&n).await?,
        None => cx.u("id"),
    };
    ensure_unified(&gid).await?;
    cx.verbose_log(&format!(
        "Getting the site URL of Microsoft 365 Group: {gid}..."
    ));
    let r = Request::get(format!("{GRAPH}/v1.0/groups/{gid}/drive?$select=webUrl"))
        .graph_json()
        .data()
        .await?;
    let web = s(&r, "webUrl");
    let site_url = web[..web.rfind('/').unwrap_or(0)].to_string();
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log(&format!("Deleting the group site: '{site_url}'..."));
    Request::post(format!(
        "{admin}/_api/GroupSiteManager/Delete?siteUrl='{}'",
        eqp(&site_url)
    ))
    .spo_json()
    .send()
    .await?;
    if cx.bool("permanent") {
        let mut done = false;
        for retries in 0..10 {
            let present = match Request::get(format!("{GRAPH}/v1.0/directory/deletedItems/{gid}"))
                .graph_json()
                .data()
                .await
            {
                Ok(v) => v.get("id").is_some_and(truthy),
                Err(e) if e.status() == Some(404) => false,
                Err(e) => return Err(e),
            };
            if present {
                cx.verbose_log("Group has been deleted and is now available in the deleted groups list. Removing permanently...");
                Request::delete(format!("{GRAPH}/v1.0/directory/deletedItems/{gid}"))
                    .accept("application/json;odata.metadata=none")
                    .send()
                    .await?;
                done = true;
                break;
            }
            cx.verbose_log(&format!("Group has not been deleted yet. Waiting for 5 seconds before the next attempt. {} attempts remaining...", 10 - retries));
            util::sleep_ms(5000).await;
        }
        if !done {
            cx.stderr("Group has been successfully deleted, but it couldn't be permanently removed from the recycle bin after all retries. It will still appear in the deleted groups list.");
        }
        cx.verbose_log(&format!(
            "Deleting the M365 group site '{site_url}' from the recycle bin..."
        ));
        let d = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
        let body = spo::csom(
            r#"<ObjectPath Id="185" ObjectPathId="184" /><Query Id="186" ObjectPathId="184"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(
                r#"<Method Id="184" ParentId="175" Name="RemoveDeletedSite"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="175" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" />"#,
                util::escape_xml(&site_url)
            ),
        );
        let res = spo::process_query(&admin, &d.value, body).await?;
        if let Some(e) = spo::csom_error(&spo::parse_csom(&res)?) {
            return Err(Error::value(e));
        }
    }
    Ok(())
}

pub async fn renew(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Renewing Microsoft 365 group's expiration: {}...",
        cx.s("id")
            .or_else(|| cx.s("displayName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let gid = match cx.s("displayName") {
        Some(n) => entra::group::id_by_display_name(&n).await?,
        None => cx.u("id"),
    };
    ensure_unified(&gid).await?;
    Request::post(format!("{GRAPH}/v1.0/groups/{gid}/renew/"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub fn set_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    if logo_ok(v) {
        Ok(())
    } else {
        Err(Some(logo_message(v)))
    }
}

pub fn set_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(o.get("newDisplayName").is_some_and(truthy)
        || [
            "description",
            "ownerIds",
            "ownerUserNames",
            "memberIds",
            "memberUserNames",
            "isPrivate",
            "logoPath",
            "allowExternalSenders",
            "autoSubscribeNewMembers",
            "hideFromAddressLists",
            "hideFromOutlookClients",
        ]
        .iter()
        .any(|k| o.contains_key(*k)))
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    if (cx.has("allowExternalSenders") || cx.has("autoSubscribeNewMembers"))
        && auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true)
    {
        return Err(Error::msg(
            "Option 'allowExternalSenders' and 'autoSubscribeNewMembers' can only be used when using delegated permissions.",
        ));
    }
    let gid = match cx.s("id") {
        Some(i) => i,
        None => entra::group::id_by_display_name(&cx.u("displayName")).await?,
    };
    ensure_unified(&gid).await?;
    cx.verbose_log(&format!(
        "Updating Microsoft 365 Group {}...",
        cx.s("id")
            .or_else(|| cx.s("displayName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    if cx.s("newDisplayName").is_some() || cx.has("description") || cx.has("isPrivate") {
        let desc = match cx.str("description") {
            Some(d) if !d.is_empty() => Some(Value::from(d)),
            Some(_) => Some(Value::Null),
            None => None,
        };
        let mut update = obj! { "displayName": ?cx.str("newDisplayName"), "description": ?desc };
        // `undefined !== ''` keeps `description: undefined`, which JSON drops
        if !cx.has("description") {
            update.as_object_mut().unwrap().shift_remove("description");
        }
        if let Some(p) = cx.opt_bool("isPrivate") {
            update.as_object_mut().unwrap().insert(
                "visibility".into(),
                Value::from(if p { "Private" } else { "Public" }),
            );
        }
        Request::patch(format!("{GRAPH}/v1.0/groups/{gid}"))
            .graph_json()
            .json(update)
            .send()
            .await?;
    }
    if [
        "allowExternalSenders",
        "autoSubscribeNewMembers",
        "hideFromAddressLists",
        "hideFromOutlookClients",
    ]
    .iter()
    .any(|k| cx.has(k))
    {
        let body = obj! {
            "allowExternalSenders": ?cx.opt_bool("allowExternalSenders"),
            "autoSubscribeNewMembers": ?cx.opt_bool("autoSubscribeNewMembers"),
            "hideFromAddressLists": ?cx.opt_bool("hideFromAddressLists"),
            "hideFromOutlookClients": ?cx.opt_bool("hideFromOutlookClients")
        };
        Request::patch(format!("{GRAPH}/v1.0/groups/{gid}"))
            .graph_json()
            .json(body)
            .send()
            .await?;
    }
    match cx.s("logoPath") {
        Some(p) => {
            let full = std::path::absolute(&p)
                .unwrap_or_default()
                .display()
                .to_string();
            cx.verbose_log(&format!("Setting group logo {full}..."));
            set_logo(&gid, &full).await?;
        }
        None => cx.debug_log("logoPath not set. Skipping"),
    }
    let ids = |i: Option<String>, n: Option<String>| async move {
        match (i, n) {
            (Some(i), _) => Ok::<Vec<String>, Error>(util::split_and_trim(&i)),
            (None, Some(n)) => {
                cx.verbose_log("Retrieving user IDs...");
                entra::user::ids_by_upns(&util::split_and_trim(&n)).await
            }
            _ => Ok(Vec::new()),
        }
    };
    let owners = ids(cx.s("ownerIds"), cx.s("ownerUserNames")).await?;
    let members = ids(cx.s("memberIds"), cx.s("memberUserNames")).await?;
    if !owners.is_empty() {
        entra::members::update_users(&gid, "owners", &owners, cx.verbose()).await?;
    }
    if !members.is_empty() {
        entra::members::update_users(&gid, "members", &members, cx.verbose()).await?;
    }
    Ok(())
}

pub async fn teamify(cx: &mut Cx) -> Result<()> {
    let gid = if let Some(i) = cx.s("id") {
        i
    } else if let Some(n) = cx.s("displayName") {
        entra::group::id_by_display_name(&n).await?
    } else {
        entra::group::id_by_mail_nickname(&cx.u("mailNickname")).await?
    };
    ensure_unified(&gid).await?;
    let data = obj! {
        "memberSettings": obj! { "allowCreatePrivateChannels": true, "allowCreateUpdateChannels": true },
        "messagingSettings": obj! { "allowUserEditMessages": true, "allowUserDeleteMessages": true },
        "funSettings": obj! { "allowGiphy": true, "giphyContentRating": "strict" }
    };
    Request::put(format!("{GRAPH}/v1.0/groups/{}/team", eqp(&gid)))
        .graph_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

async fn group_or_team_id(cx: &Cx, verbose_msg: &str) -> Result<String> {
    if let Some(g) = cx.s("groupId") {
        return Ok(g);
    }
    if let Some(t) = cx.s("teamId") {
        return Ok(t);
    }
    let name = cx
        .s("groupName")
        .or_else(|| cx.s("teamName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&verbose_msg.replace("{name}", &name));
    entra::group::id_by_display_name(&name).await
}

async fn ids_or_names(cx: &Cx, msg: &str) -> Result<Vec<String>> {
    match cx.s("ids") {
        Some(i) => Ok(util::split_and_trim(&i)),
        None => {
            cx.verbose_log(msg);
            entra::user::ids_by_upns(&util::split_and_trim(&cx.u("userNames"))).await
        }
    }
}

pub async fn user_add(cx: &mut Cx) -> Result<()> {
    let gid = group_or_team_id(cx, "Retrieving Group ID by display name...").await?;
    ensure_unified(&gid).await?;
    let ids = ids_or_names(cx, "Retrieving user ID(s) by username(s)...").await?;
    cx.verbose_log(&format!(
        "Adding user(s) {} to group {}...",
        cx.s("ids")
            .or_else(|| cx.s("userNames"))
            .unwrap_or_else(|| "undefined".into()),
        cx.s("groupId")
            .or_else(|| cx.s("groupName"))
            .or_else(|| cx.s("teamId"))
            .or_else(|| cx.s("teamName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let role = if cx.str("role").unwrap_or_default().to_lowercase() == "owner" {
        "owners"
    } else {
        "members"
    };
    entra::members::add_bound(&gid, role, &ids, false).await
}

pub async fn user_list(cx: &mut Cx) -> Result<()> {
    let cx: &Cx = cx;
    let gid = match cx.s("groupId") {
        Some(g) => g,
        None => {
            cx.verbose_log("Retrieving Group Id...");
            entra::group::id_by_display_name(&cx.u("groupDisplayName")).await?
        }
    };
    if !entra::group::is_unified(&gid).await? {
        return Err(Error::js(format!(
            "Specified group '{}' is not a Microsoft 365 group.",
            cx.s("groupId")
                .or_else(|| cx.s("groupDisplayName"))
                .unwrap_or_else(|| "undefined".into())
        )));
    }
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
                None => "id,displayName,userPrincipalName,givenName,surname,userType".into(),
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
                "{GRAPH}/v1.0/groups/{gid}/{r}/microsoft.graph.user?$select={}{expand}",
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
    if let Some(r) = &role {
        users.retain(|u| {
            arr(u, "roles")
                .iter()
                .any(|x| x.as_str() == Some(r.as_str()))
        });
    }
    cx.log(Value::Array(users))
}

pub async fn user_remove(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("userNames")
        .or_else(|| cx.s("ids"))
        .unwrap_or_else(|| "undefined".into());
    let from = cx
        .s("groupId")
        .or_else(|| cx.s("groupName"))
        .or_else(|| cx.s("teamId"))
        .or_else(|| cx.s("teamName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove {who} from {from}?"),
    )? {
        return Ok(());
    }
    let gid = match cx.s("groupId").or_else(|| cx.s("teamId")) {
        Some(i) => i,
        None => {
            let name = cx
                .str("groupName")
                .or_else(|| cx.str("teamName"))
                .unwrap_or_else(|| "undefined".into());
            cx.verbose_log(&format!("Retrieving Group ID by display name {name}..."));
            entra::group::id_by_display_name(&name).await?
        }
    };
    ensure_unified(&gid).await?;
    let ids = ids_or_names(cx, "Retrieving user IDs for {userNames}...").await?;
    for role in ["owners", "members"] {
        for id in &ids {
            match Request::delete(format!("{GRAPH}/v1.0/groups/{gid}/{role}/{id}/$ref"))
                .accept("application/json;odata.metadata=none")
                .send()
                .await
            {
                Ok(_) => {}
                Err(e) if e.status() == Some(404) => {}
                Err(e) => return Err(Error::Thrown(e.body().cloned().unwrap_or(Value::Null))),
            }
        }
    }
    Ok(())
}

pub async fn user_set(cx: &mut Cx) -> Result<()> {
    let gid = group_or_team_id(cx, "Retrieving Group ID by display name...").await?;
    ensure_unified(&gid).await?;
    let ids = ids_or_names(cx, "Retrieving user ID(s) by username(s)...").await?;
    let role = cx.u("role").to_lowercase();
    entra::members::add_bound(
        &gid,
        if role == "member" {
            "members"
        } else {
            "owners"
        },
        &ids,
        false,
    )
    .await?;
    let current = if role == "member" {
        "owners"
    } else {
        "members"
    };
    cx.verbose_log(&format!("Removing users from the old role '{current}'."));
    entra::members::remove_from_role(&gid, current, &ids).await
}

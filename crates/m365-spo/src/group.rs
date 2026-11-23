//! `spo user *`, `spo userprofile *` and `spo group *`.

use m365_core::prelude::*;
use m365_core::ut::{entra, spo};

fn text_output(cx: &Cx) -> bool {
    cx.str("output").as_deref() == Some("text")
}

async fn entra_group(cx: &Cx, id: &str, name: &str) -> Result<Value> {
    match cx.s(id) {
        Some(i) => entra::group::by_id(&i, None).await,
        None => entra::group::by_display_name(&cx.u(name), None).await,
    }
}

fn group_login(g: &Value) -> String {
    if g.get("mailEnabled").is_some_and(truthy) {
        format!("c:0o.c|federateddirectoryclaimprovider|{}", u(g, "id"))
    } else {
        format!("c:0t.c|tenant|{}", u(g, "id"))
    }
}

pub async fn user_ensure(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let who = [
        "entraId",
        "userName",
        "loginName",
        "entraGroupId",
        "entraGroupName",
    ]
    .iter()
    .find_map(|k| cx.s(k))
    .unwrap_or_default();
    cx.verbose_log(&format!("Ensuring user {who} at site {web}"));
    let upn = if let Some(n) = cx.s("userName") {
        n
    } else if let Some(i) = cx.s("entraId") {
        entra::user::upn_by_id(&i, false).await?
    } else if let Some(l) = cx.s("loginName") {
        l
    } else if cx.has("entraGroupId") || cx.has("entraGroupName") {
        group_login(&entra_group(cx, "entraGroupId", "entraGroupName").await?)
    } else {
        String::new()
    };
    cx.log(
        Request::post(format!("{web}/_api/web/ensureuser"))
            .spo_json()
            .json(obj! { "logonName": upn })
            .data()
            .await?,
    )
}

pub async fn user_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving information for user in site '{web}'..."
    ));
    let base = format!("{web}/_api/web/");
    let path = if let Some(i) = cx.s("id") {
        format!("siteusers/GetById('{}')", eqp(&i))
    } else if let Some(e) = cx.s("email") {
        format!("siteusers/GetByEmail('{}')", eqp(&e))
    } else if let Some(l) = cx.s("loginName") {
        format!("siteusers/GetByLoginName('{}')", eqp(&l))
    } else if let Some(n) = cx.s("userName") {
        let r = Request::get(format!(
            "{base}siteusers?$filter=UserPrincipalName eq ('{}')",
            eqp(&n)
        ))
        .spo_json()
        .data()
        .await
        .map_err(Error::uncaught)?;
        let v = need_arr(&r, "value", "0").map_err(Error::uncaught)?;
        let Some(user) = v.first().filter(|x| truthy(x)) else {
            return Err(Error::uncaught(Error::msg(format!("User not found: {n}"))));
        };
        format!("siteusers/GetById('{}')", eqp(&u(user, "Id")))
    } else if cx.has("entraGroupId") || cx.has("entraGroupName") {
        let g = entra_group(cx, "entraGroupId", "entraGroupName")
            .await
            .map_err(Error::uncaught)?;
        match g.get("mail").filter(|m| truthy(m)) {
            Some(m) => format!("siteusers/GetByEmail('{}')", eqp(&to_js_string(m))),
            None => format!("siteusers/GetByLoginName('c:0t.c|tenant|{}')", u(&g, "id")),
        }
    } else {
        "currentuser".into()
    };
    cx.log(
        Request::get(format!("{base}{path}"))
            .spo_json()
            .data()
            .await?,
    )
}

pub async fn user_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Retrieving users from web {web}..."));
    let r = Request::get(format!("{web}/_api/web/siteusers"))
        .spo_json()
        .data()
        .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn user_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove specified user from the site {web}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing user from  subsite {web} ..."));
    let mut url = format!("{}/_api/web/siteusers/", util::enc_uri(&web));
    if let Some(i) = cx.s("id") {
        url.push_str(&format!("removebyid({i})"));
    } else if let Some(l) = cx.s("loginName") {
        url.push_str(&format!("removeByLoginName('{}')", eqp(&l)));
    } else if let Some(e) = cx.s("email") {
        let user = crate::sp::user_by_email(cx, &web, &e).await?;
        url.push_str(&format!("removebyid({})", u(&user, "Id")));
    } else if let Some(n) = cx.s("userName") {
        let r = Request::get(format!(
            "{web}/_api/web/siteusers?$filter=UserPrincipalName eq ('{}')",
            eqp(&n)
        ))
        .spo_json()
        .data()
        .await?;
        let v = need_arr(&r, "value", "0")?;
        let Some(user) = v.first().filter(|x| truthy(x)) else {
            return Err(Error::js(format!("User not found: {n}")));
        };
        cx.verbose_log(&format!("Removing user {} ...", u(user, "Title")));
        url.push_str(&format!("removebyid({})", u(user, "Id")));
    } else if cx.has("entraGroupId") || cx.has("entraGroupName") {
        let g = entra_group(cx, "entraGroupId", "entraGroupName").await?;
        cx.verbose_log(&format!(
            "Removing entra group {} ...",
            u(&g, "displayName")
        ));
        if g.get("mail").is_some_and(truthy) {
            url.push_str(&format!(
                "removeByLoginName('c:0o.c|federateddirectoryclaimprovider|{}')",
                u(&g, "id")
            ));
        } else {
            url.push_str(&format!(
                "removeByLoginName('c:0t.c|tenant|{}')",
                u(&g, "id")
            ));
        }
    }
    Request::post(url).spo_json().send().await?;
    Ok(())
}

pub async fn userprofile_get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let name = format!("i:0#.f|membership|{}", cx.u("userName"));
    let mut r = Request::get(format!(
        "{spo_url}/_api/SP.UserProfiles.PeopleManager/GetPropertiesFor(accountName=@v)?@v='{}'",
        eqp(&name)
    ))
    .spo_json()
    .data()
    .await?;
    if text_output(cx)
        && let Some(o) = r.as_object_mut()
        && let Some(s) = o.get("UserProfileProperties").map(json::stringify)
    {
        o.insert("UserProfileProperties".into(), Value::from(s));
    }
    cx.log(r)
}

pub async fn userprofile_set(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = crate::sp::digest(&spo_url).await?;
    let values: Vec<String> = cx
        .u("propertyValue")
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let mut data = obj! { "accountName": format!("i:0#.f|membership|{}", cx.u("userName")), "propertyName": cx.u("propertyName") };
    let ty = if values.len() > 1 {
        data["propertyValues"] = Value::from(values);
        "SetMultiValuedProfileProperty"
    } else {
        data["propertyValue"] = Value::from(values[0].clone());
        "SetSingleValueProfileProperty"
    };
    Request::post(format!("{spo_url}/_api/SP.UserProfiles.PeopleManager/{ty}"))
        .header("Accept", "application/json;odata=nometadata")
        .header("Content-type", "application/json;odata=verbose")
        .header("X-RequestDigest", d)
        .json(data)
        .send()
        .await?;
    Ok(())
}

fn group_props(cx: &Cx, title: &str) -> Value {
    obj! {
        "Title": ?cx.val(title).cloned(),
        "Description": ?cx.val("description").cloned(),
        "AllowMembersEditMembership": ?cx.val("allowMembersEditMembership").cloned(),
        "OnlyAllowMembersViewMembership": ?cx.val("onlyAllowMembersViewMembership").cloned(),
        "AllowRequestToJoinLeave": ?cx.val("allowRequestToJoinLeave").cloned(),
        "AutoAcceptRequestToJoinLeave": ?cx.val("autoAcceptRequestToJoinLeave").cloned(),
        "RequestToJoinLeaveEmailSetting": ?cx.val("requestToJoinLeaveEmailSetting").cloned(),
    }
}

pub async fn group_add(cx: &mut Cx) -> Result<()> {
    let r = Request::post(format!("{}/_api/web/sitegroups", cx.u("webUrl")))
        .spo_json()
        .json(group_props(cx, "name"))
        .data()
        .await?;
    cx.log(r)
}

pub async fn group_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving information for group in site at {web}..."
    ));
    let url = if let Some(i) = cx.s("id").filter(|i| i != "0") {
        format!("{web}/_api/web/sitegroups/GetById('{i}')")
    } else if let Some(n) = cx.s("name") {
        format!("{web}/_api/web/sitegroups/GetByName('{}')", eqp(&n))
    } else {
        match cx.s("associatedGroup").map(|a| a.to_lowercase()).as_deref() {
            Some("owner") => format!("{web}/_api/web/AssociatedOwnerGroup"),
            Some("member") => format!("{web}/_api/web/AssociatedMemberGroup"),
            Some("visitor") => format!("{web}/_api/web/AssociatedVisitorGroup"),
            _ => String::new(),
        }
    };
    cx.log(Request::get(url).spo_json().data().await?)
}

pub async fn group_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving list of groups for specified web at {web}..."
    ));
    let base = format!("{web}/_api/web");
    if !cx.bool("associatedGroupsOnly") {
        return cx.log(Value::Array(
            odata::get_all_items(&format!("{base}/sitegroups")).await?,
        ));
    }
    let r = Request::get(format!("{base}?$expand=AssociatedOwnerGroup,AssociatedMemberGroup,AssociatedVisitorGroup&$select=AssociatedOwnerGroup,AssociatedMemberGroup,AssociatedVisitorGroup")).spo_json().data().await?;
    if !text_output(cx) {
        return cx.log(r);
    }
    let out: Vec<Value> = match r.as_object() {
        Some(o) => o
            .iter()
            .map(|(k, v)| {
                let mut m = Obj::new();
                m.insert("Type".into(), Value::from(k.as_str()));
                if let Some(vo) = v.as_object() {
                    m.extend(vo.clone());
                }
                Value::Object(m)
            })
            .collect(),
        None => Vec::new(),
    };
    cx.log(Value::Array(out))
}

fn group_seg(cx: &Cx) -> String {
    match cx.s("groupId").filter(|i| i != "0") {
        Some(i) => format!("GetById('{i}')"),
        None => format!("GetByName('{}')", eqp(&cx.u("groupName"))),
    }
}

pub async fn group_member_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let mut logins = Vec::new();
    if let Some(n) = cx.s("userNames").or_else(|| cx.s("emails")) {
        logins.extend(
            util::split_and_trim(&n)
                .into_iter()
                .map(|u| Some(Value::from(format!("i:0#.f|membership|{u}")))),
        );
    } else if let Some(g) = cx.s("entraGroupIds").or_else(|| cx.s("entraGroupNames")) {
        cx.verbose_log(&format!(
            "Resolving {} group(s)...",
            g.encode_utf16().count()
        ));
        let mut groups = Vec::new();
        if let Some(ids) = cx.s("entraGroupIds") {
            for i in util::split_and_trim(&ids) {
                groups.push(entra::group::by_id(&i, None).await?);
            }
        } else {
            for n in util::split_and_trim(&cx.u("entraGroupNames")) {
                groups.push(entra::group::by_display_name(&n, None).await?);
            }
        }
        logins.extend(groups.iter().map(|g| Some(Value::from(group_login(g)))));
    } else if let Some(ids) = cx.s("userIds") {
        let ids = util::split_and_trim(&ids);
        cx.verbose_log(&format!("Resolving {} user(s)...", ids.len()));
        for i in ids {
            let n = json::parse_int(&i)
                .map(|n| n.to_string())
                .unwrap_or_else(|| "NaN".into());
            let r = Request::get(format!(
                "{web}/_api/web/SiteUsers/GetById({n})?$select=LoginName"
            ))
            .spo_json()
            .data()
            .await?;
            logins.push(r.get("LoginName").cloned());
        }
    }
    let api = match cx.s("groupId").filter(|i| i != "0") {
        Some(i) => format!("{web}/_api/web/SiteGroups/GetById({i})/users"),
        None => format!(
            "{web}/_api/web/SiteGroups/GetByName('{}')/users",
            eqp(&cx.u("groupName"))
        ),
    };
    cx.verbose_log("Adding members to group...");
    let mut out = Vec::new();
    for l in logins {
        out.push(
            Request::post(&api)
                .spo_json()
                .json(obj! { "LoginName": ?l })
                .data()
                .await?,
        );
    }
    cx.log(Value::Array(out))
}

pub async fn group_member_list(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("groupId")
        .filter(|i| i != "0")
        .or_else(|| cx.s("groupName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Retrieving the list of members from the SharePoint group :  {who}"
    ));
    let url = format!(
        "{}/_api/web/sitegroups/{}/users",
        cx.u("webUrl"),
        group_seg(cx)
    );
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn group_member_remove(cx: &mut Cx) -> Result<()> {
    let first = |keys: &[&str]| {
        keys.iter()
            .find_map(|k| cx.s(k).filter(|v| !v.is_empty() && v != "0"))
            .unwrap_or_else(|| "undefined".into())
    };
    let who = first(&[
        "userName",
        "userId",
        "email",
        "entraGroupId",
        "entraGroupName",
    ]);
    if cx.bool("force") {
        cx.debug_log("Confirmation bypassed by entering confirm option. Removing the user from SharePoint Group...");
    } else if !cx.confirm(&format!(
        "Are you sure you want to remove user {who} from the SharePoint group?"
    ))? {
        return Ok(());
    }
    let who2 = first(&[
        "userName",
        "email",
        "userId",
        "entraGroupId",
        "entraGroupName",
    ]);
    let grp = first(&["groupId", "groupName"]);
    cx.verbose_log(&format!("Removing User {who2} from Group: {grp}"));
    let web = cx.u("webUrl");
    let mut url = format!("{web}/_api/web/sitegroups/{}", group_seg(cx));
    if let Some(i) = cx.s("userId").filter(|i| i != "0") {
        url.push_str(&format!("/users/removeById({i})"));
    } else if cx.s("userName").filter(|s| !s.is_empty()).is_some()
        || cx.s("email").filter(|s| !s.is_empty()).is_some()
    {
        let name = match cx.s("userName").filter(|s| !s.is_empty()) {
            Some(n) => n,
            None => {
                let e = cx.u("email");
                cx.verbose_log(&format!("Retrieving information about the user {e}"));
                let o = obj! { "email": e, "output": "json", "debug": ?cx.val("debug").cloned(), "verbose": ?cx.val("verbose").cloned() };
                let (out, _) = m365_core::runner::run_nested(
                    "entra user get",
                    o.as_object().cloned().unwrap_or_default(),
                    true,
                )
                .await
                .map_err(Error::uncaught)?;
                u(
                    &json::jparse(&out).map_err(Error::uncaught)?,
                    "userPrincipalName",
                )
            }
        };
        url.push_str(&format!(
            "/users/removeByLoginName(@LoginName)?@LoginName='{}'",
            eqp(&format!("i:0#.f|membership|{name}"))
        ));
    } else {
        let mut o = obj! { "webUrl": web.as_str(), "output": "json", "debug": cx.debug(), "verbose": cx.verbose() };
        match cx.val("groupId").filter(|v| truthy(v)) {
            Some(g) => o["groupId"] = g.clone(),
            None => o["groupName"] = cx.val("groupName").cloned().unwrap_or(Value::Null),
        }
        let (out, _) = m365_core::runner::run_nested(
            "spo group member list",
            o.as_object().cloned().unwrap_or_default(),
            true,
        )
        .await
        .map_err(Error::uncaught)?;
        let list = json::jparse(&out).map_err(Error::uncaught)?;
        let is_grp = |l: &str| {
            l.starts_with("c:0o.c|federateddirectoryclaimprovider|")
                || l.starts_with("c:0t.c|tenant|")
        };
        let items = list.as_array().cloned().unwrap_or_default();
        let found: Vec<&Value> = items
            .iter()
            .filter(|x| {
                let l = u(x, "LoginName");
                match cx.s("entraGroupId").filter(|s| !s.is_empty()) {
                    Some(id) => l.contains(&id) && is_grp(&l),
                    None => x.get("Title") == cx.val("entraGroupName") && is_grp(&l),
                }
            })
            .collect();
        let Some(g) = found.first() else {
            return Err(Error::uncaught(Error::msg(format!(
                "The Microsoft Entra group {} is not found in SharePoint group {grp}",
                first(&["entraGroupId", "entraGroupName"])
            ))));
        };
        url.push_str(&format!("/users/RemoveById({})", u(g, "Id")));
    }
    Request::post(url).spo_json().send().await?;
    Ok(())
}

pub async fn group_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let who = cx
        .s("id")
        .filter(|i| i != "0")
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the group {who} from web {web}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing group in web at {web}..."));
    let id = match cx.s("name").filter(|n| !n.is_empty()) {
        Some(n) => u(
            &Request::get(format!(
                "{web}/_api/web/sitegroups/GetByName('{n}')?$select=Id"
            ))
            .accept("application/json")
            .json_response()
            .data()
            .await?,
            "Id",
        ),
        None => cx.s("id").unwrap_or_else(|| "undefined".into()),
    };
    Request::post(format!("{web}/_api/web/sitegroups/RemoveById({id})"))
        .header("content-length", "0")
        .accept("application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn group_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let id = cx.s("id").filter(|i| i != "0");
    cx.verbose_log(&format!(
        "Setting properties for group {}",
        id.clone()
            .or_else(|| cx.s("name"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let seg = match &id {
        Some(i) => format!("GetById({i})"),
        None => format!("GetByName('{}')", cx.u("name")),
    };
    let base = format!("{web}/_api/web/sitegroups/{seg}");
    Request::patch(&base)
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json")
        .json(group_props(cx, "newName"))
        .send()
        .await?;
    if cx.s("ownerEmail").filter(|s| !s.is_empty()).is_some()
        || cx.s("ownerUserName").filter(|s| !s.is_empty()).is_some()
    {
        let o = obj! { "userName": ?cx.val("ownerUserName").cloned(), "email": ?cx.val("ownerEmail").cloned(), "output": "json", "debug": ?cx.val("debug").cloned(), "verbose": ?cx.val("verbose").cloned() };
        let (out, _) = m365_core::runner::run_nested(
            "entra user get",
            o.as_object().cloned().unwrap_or_default(),
            true,
        )
        .await?;
        let user = json::jparse(&out)?;
        let r = Request::post(format!(
            "{web}/_api/web/ensureUser('{}')?$select=Id",
            u(&user, "userPrincipalName")
        ))
        .accept("application/json")
        .header("content-type", "application/json")
        .json_response()
        .data()
        .await?;
        Request::post(format!("{base}/SetUserAsOwner({})", u(&r, "Id")))
            .accept("application/json;odata.metadata=none")
            .header("content-type", "application/json")
            .send()
            .await?;
    }
    Ok(())
}

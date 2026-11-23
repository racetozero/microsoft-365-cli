//! `spo web *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::entra;

pub fn add_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(json::parse_int(&to_js_string(v)).is_some())
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let parent = cx.u("parentWebUrl");
    let d = sp::digest(&parent).await?;
    let url = cx.u("url");
    let data = obj! { "parameters": obj! {
        "Url": ?cx.val("url").cloned(),
        "Title": ?cx.val("title").cloned(),
        "Description": ?cx.val("description").cloned(),
        "Language": ?cx.val("locale").cloned(),
        "WebTemplate": ?cx.val("webTemplate").cloned(),
        "UseUniquePermissions": ?cx.val("breakInheritance").cloned(),
    } };
    cx.verbose_log(&format!("Creating subsite {parent}/{url}..."));
    let site = Request::post(format!("{parent}/_api/web/webinfos/add"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .header("X-RequestDigest", d)
        .json(data)
        .data()
        .await?;
    if cx.bool("inheritNavigation") {
        cx.verbose_log("Setting inheriting navigation from the parent site...");
        let sub = format!("{parent}/{}", eqp(&url));
        let p = Request::get(format!("{sub}/_api/web/effectivebasepermissions"))
            .spo_json()
            .data()
            .await?;
        let perms = sp::BasePermissions::new(p.get("High"), p.get("Low"));
        if perms.has(sp::ADD_AND_CUSTOMIZE_PAGES) {
            let d = sp::digest(&sub).await?;
            let body = format!(
                r#"<Request xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}"><Actions><ObjectPath Id="1" ObjectPathId="0" /><ObjectPath Id="3" ObjectPathId="2" /><ObjectPath Id="5" ObjectPathId="4" /><SetProperty Id="6" ObjectPathId="4" Name="UseShared"><Parameter Type="Boolean">true</Parameter></SetProperty></Actions><ObjectPaths><StaticProperty Id="0" TypeId="{{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}}" Name="Current" /><Property Id="2" ParentId="0" Name="Web" /><Property Id="4" ParentId="2" Name="Navigation" /></ObjectPaths></Request>"#,
                m365_core::ut::spo::application_name()
            );
            let json = sp::post(&sub, &d, body).await?;
            sp::check_first(&json)?;
        } else {
            cx.verbose_log("No script is enabled. Skipping the InheritParentNavigation settings.");
        }
    }
    cx.log(site)
}

pub async fn clientsidewebpart_list(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!("{}/_api/web/GetClientSideWebParts", cx.u("webUrl")))
        .spo_json()
        .data()
        .await?;
    let mut out = Vec::new();
    for c in need_arr(&r, "value", "forEach")? {
        if c.get("ComponentType").and_then(Value::as_f64) == Some(1.0) {
            let manifest = json::jparse(&u(c, "Manifest"))?;
            let title = manifest
                .get("preconfiguredEntries")
                .and_then(|p| p.get(0))
                .and_then(|p| p.get("title"))
                .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'title')"))?
                .get("default")
                .cloned();
            out.push(obj! { "Id": u(c, "Id").replacen('{', "", 1).replacen('}', "", 1), "Name": ?c.get("Name").cloned(), "Title": ?title });
        }
    }
    cx.log(Value::Array(out))
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    let q = if cx.bool("withGroups") {
        "?$expand=AssociatedMemberGroup,AssociatedOwnerGroup,AssociatedVisitorGroup"
    } else {
        ""
    };
    let mut web = Request::get(format!("{url}/_api/web{q}"))
        .spo_json()
        .data()
        .await?;
    if cx.bool("withPermissions") {
        let r = Request::get(format!(
            "{url}/_api/web/RoleAssignments?$expand=Member,RoleDefinitionBindings"
        ))
        .spo_json()
        .data()
        .await?;
        let mut items = need_arr(&r, "value", "forEach")?.to_vec();
        for i in items.iter_mut() {
            let mut b = i
                .get("RoleDefinitionBindings")
                .and_then(Value::as_array)
                .cloned()
                .ok_or_else(|| Error::js("response.forEach is not a function"))?;
            sp::set_friendly_permissions(&mut b)?;
            i.as_object_mut()
                .map(|o| o.insert("RoleDefinitionBindings".into(), Value::Array(b)));
        }
        web.as_object_mut()
            .map(|o| o.insert("RoleAssignments".into(), Value::Array(items)));
    }
    cx.log(web)
}

pub async fn installedlanguage_list(cx: &mut Cx) -> Result<()> {
    let w = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving all installed languages on site {w}..."
    ));
    let r = Request::get(format!("{w}/_api/web/RegionalSettings/InstalledLanguages"))
        .spo_json()
        .data()
        .await?;
    match r.get("Items") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    cx.verbose_log(&format!("Retrieving all webs in site at {url}..."));
    let mut r = format!("{url}/_api/web/webs");
    if cx.str("output").as_deref() != Some("json") {
        r.push_str("?$select=Title,Id,URL");
    }
    cx.log(Value::Array(odata::get_all_items(&r).await?))
}

pub async fn reindex(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    let d = sp::digest(&url).await?;
    cx.debug_log("Retrieved request digest. Retrieving web identity...");
    let id = sp::web_identity(&url, &d).await?;
    cx.debug_log("Retrieved web identity.");
    cx.verbose_log("Checking if the site is a no-script site...");
    if sp::is_no_script_site(cx, &url, &d, &id).await? {
        cx.verbose_log("Site is a no-script site. Reindexing lists instead...");
        cx.debug_log("Retrieving information about lists...");
        let r = Request::get(format!("{url}/_api/web/lists?$select=NoCrawl,Title,RootFolder/Properties,RootFolder/ServerRelativeUrl&$expand=RootFolder/Properties")).spo_json().data().await?;
        for l in need_arr(&r, "value", "map")? {
            if l.get("NoCrawl").is_some_and(truthy) {
                cx.debug_log(&format!("List {} is excluded from crawling", u(l, "Title")));
                continue;
            }
            let root = l.get("RootFolder").ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'ServerRelativeUrl')")
            })?;
            let srv = u(root, "ServerRelativeUrl");
            let fid = sp::folder_identity(&id.object_identity, &url, &srv, &d).await?;
            let props = root.get("Properties").ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'vti_x005f_searchversion')")
            })?;
            let v = props
                .get("vti_x005f_searchversion")
                .filter(|v| truthy(v))
                .map(|v| json::js_number(&to_js_string(v)))
                .unwrap_or(0.0)
                + 1.0;
            sp::set_property(
                "vti_searchversion",
                &json::number_to_string(v),
                &url,
                &d,
                &fid,
                Some(&srv),
            )
            .await?;
        }
    }
    cx.verbose_log("Site is not a no-script site. Reindexing site...");
    let p = Request::get(format!("{url}/_api/web/allproperties"))
        .spo_json()
        .data()
        .await?;
    let v = p
        .get("vti_x005f_searchversion")
        .filter(|v| truthy(v))
        .map(|v| json::js_number(&to_js_string(v)))
        .unwrap_or(0.0)
        + 1.0;
    sp::set_property(
        "vti_searchversion",
        &json::number_to_string(v),
        &url,
        &d,
        &id,
        None,
    )
    .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the subsite {url}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Deleting subsite {url} ..."));
    Request::post(format!("{}/_api/web", util::enc_uri(&url)))
        .spo_json()
        .header("X-HTTP-Method", "DELETE")
        .send()
        .await?;
    Ok(())
}

pub async fn retentionlabel_list(cx: &mut Cx) -> Result<()> {
    let w = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving all retention labels that are available on {w}..."
    ));
    cx.log(Value::Array(odata::get_all_items(&format!("{w}/_api/SP.CompliancePolicy.SPPolicyStoreProxy.GetAvailableTagsForSite(siteUrl=@a1)?@a1='{}'", eqp(&w))).await?))
}

/// The principal of a role assignment command.
async fn principal(cx: &Cx, web: &str) -> Result<Option<String>> {
    if let Some(p) = cx
        .str("principalId")
        .filter(|_| cx.val("principalId").is_some_and(truthy))
    {
        return Ok(Some(p));
    }
    if let Some(upn) = cx.s("upn") {
        return Ok(Some(u(&sp::user_by_email(cx, web, &upn).await?, "Id")));
    }
    if let Some(g) = cx.s("groupName") {
        return Ok(Some(u(&sp::group_by_name(cx, web, &g).await?, "Id")));
    }
    if cx.s("entraGroupId").is_some() || cx.s("entraGroupName").is_some() {
        cx.verbose_log("Retrieving group information...");
        let g = match cx.s("entraGroupId") {
            Some(id) => entra::group::by_id(&id, None).await?,
            None => entra::group::by_display_name(&cx.u("entraGroupName"), None).await?,
        };
        return Ok(Some(u(&sp::ensure_entra_group(web, &g).await?, "Id")));
    }
    Ok(None)
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Adding role assignment to web {web}..."));
    let role = match cx.s("roleDefinitionName") {
        Some(n) => u(&sp::role_definition_by_name(cx, &web, &n).await?, "Id"),
        None => cx.u("roleDefinitionId"),
    };
    if let Some(p) = principal(cx, &web).await? {
        cx.verbose_log("Adding role assignment...");
        Request::post(format!(
            "{web}/_api/web/roleassignments/addroleassignment(principalid='{p}',roledefid='{role}')"
        ))
        .spo_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    }
    Ok(())
}

pub async fn roleassignment_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove role assignment from web {web}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing role assignment from web {web}..."));
    if let Some(p) = principal(cx, &web).await? {
        cx.verbose_log("Removing role assignment...");
        Request::post(format!(
            "{web}/_api/web/roleassignments/removeroleassignment(principalid='{p}')"
        ))
        .spo_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    }
    Ok(())
}

pub async fn roleinheritance_break(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Break role inheritance of subsite with URL {web}..."
    ));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to break the role inheritance of subsite {web}?"),
    )? {
        return Ok(());
    }
    Request::post(format!(
        "{web}/_api/web/breakroleinheritance({})",
        !cx.bool("clearExistingPermissions")
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_reset(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Restore role inheritance of subsite at {web}..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to reset the role inheritance of {web}?"),
    )? {
        return Ok(());
    }
    Request::post(format!("{web}/_api/web/resetroleinheritance"))
        .spo_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn rule_get(cx: &mut Cx) -> Result<()> {
    let (web, id) = (cx.u("webUrl"), cx.u("id"));
    cx.verbose_log(&format!(
        "Retrieving rule with id '{id}' from site '{web}'..."
    ));
    cx.log(
        Request::get(format!("{web}/_api/web/alerts/GetById('{}')?$expand=List,User,List/Rootfolder&$select=*,List/Id,List/Title,List/Rootfolder/ServerRelativeUrl", eqp(&id)))
            .spo_json()
            .data()
            .await?,
    )
}

pub async fn rule_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let list = cx
        .s("listId")
        .or_else(|| cx.s("listTitle"))
        .or_else(|| cx.s("listUrl"));
    let user = cx.s("userName").or_else(|| cx.s("userId"));
    let mut msg = format!("Retrieving alerts from site '{web}'");
    if let Some(l) = &list {
        msg.push_str(&format!(" for list '{l}'"));
    }
    if let Some(u0) = &user {
        msg.push_str(&format!(
            "{} for user '{u0}'",
            if list.is_some() { " and" } else { "" }
        ));
    }
    cx.verbose_log(&format!("{msg}..."));
    let mut url = format!(
        "{web}/_api/web/alerts?$expand=List,User,List/Rootfolder,Item&$select=*,List/Id,List/Title,List/Rootfolder/ServerRelativeUrl,Item/ID,Item/FileRef,Item/Guid"
    );
    let mut filters = Vec::new();
    let lid = if let Some(i) = cx.s("listId") {
        Some(i)
    } else if cx.s("listUrl").is_some() || cx.s("listTitle").is_some() {
        Some(
            sp::list_id(
                cx,
                &web,
                cx.str("listTitle").as_deref(),
                cx.str("listUrl").as_deref(),
            )
            .await?,
        )
    } else {
        None
    };
    if let Some(l) = lid.filter(|l| !l.is_empty()) {
        filters.push(format!("List/Id eq guid'{}'", eqp(&l)));
    }
    if let Some(n) = cx.s("userName") {
        filters.push(format!("User/UserPrincipalName eq '{}'", eqp(&n)));
    } else if let Some(i) = cx.s("userId") {
        // looked up outside the command's try/catch
        let upn = entra::user::upn_by_id(&i, false)
            .await
            .map_err(Error::uncaught)?;
        filters.push(format!("User/UserPrincipalName eq '{}'", eqp(&upn)));
    }
    if !filters.is_empty() {
        url.push_str(&format!("&$filter={}", filters.join(" and ")));
    }
    let mut items = odata::get_all_items(&url).await?;
    let trim = cx.str("output").as_deref() == Some("text");
    for a in items.iter_mut() {
        if let Some(i) = a
            .get_mut("Item")
            .filter(|i| truthy(i))
            .and_then(Value::as_object_mut)
        {
            i.shift_remove("ID");
        }
        if trim {
            let upn = a
                .get("User")
                .and_then(|u| u.get("UserPrincipalName"))
                .cloned();
            if let Some(o) = a.as_object_mut() {
                m365_core::json::assign(o, "UserPrincipalName", upn);
            }
        }
    }
    cx.log(Value::Array(items))
}

pub async fn rule_remove(cx: &mut Cx) -> Result<()> {
    let (web, id) = (cx.u("webUrl"), cx.u("id"));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the alert with id '{id}' from site '{web}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing alert with ID '{id}' from site '{web}'..."
    ));
    Request::delete(format!("{web}/_api/web/Alerts/DeleteAlert('{}')", eqp(&id)))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let mut p = obj! {};
    cx.add_unknown_options(&mut p);
    let o = p.as_object_mut().expect("object");
    if let Some(t) = cx.s("title") {
        o.insert("Title".into(), Value::from(t));
    }
    if let Some(d) = cx.s("description") {
        o.insert("Description".into(), Value::from(d));
    }
    for (k, v) in [
        ("siteLogoUrl", "SiteLogoUrl"),
        ("quickLaunchEnabled", "QuickLaunchEnabled"),
    ] {
        if let Some(x) = cx.val(k) {
            o.insert(v.into(), x.clone());
        }
    }
    if let Some(h) = cx.str("headerEmphasis") {
        o.insert(
            "HeaderEmphasis".into(),
            json::num_value(json::js_number(&h)),
        );
    }
    if let Some(h) = cx.str("headerLayout") {
        o.insert(
            "HeaderLayout".into(),
            Value::from(if h == "standard" { 1 } else { 2 }),
        );
    }
    for (k, v) in [
        ("megaMenuEnabled", "MegaMenuEnabled"),
        ("footerEnabled", "FooterEnabled"),
        ("navAudienceTargetingEnabled", "NavAudienceTargetingEnabled"),
    ] {
        if let Some(x) = cx.val(k) {
            o.insert(v.into(), x.clone());
        }
    }
    if let Some(sc) = cx.str("searchScope") {
        let i = ["defaultscope", "tenant", "hub", "site"]
            .iter()
            .position(|x| *x == sc.to_lowercase())
            .map(|i| i as i64)
            .unwrap_or(-1);
        o.insert("SearchScope".into(), Value::from(i));
    }
    let url = cx.u("url");
    cx.verbose_log(&format!("Updating properties of subsite {url}..."));
    Request::patch(format!("{url}/_api/web"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(p)
        .send()
        .await?;
    if let Some(w) = cx.val("welcomePage").cloned() {
        cx.verbose_log(&format!("Setting welcome page to: {}...", to_js_string(&w)));
        Request::patch(format!("{url}/_api/web/RootFolder"))
            .header("content-type", "application/json;odata=nometadata")
            .spo_json()
            .json(obj! { "WelcomePage": w })
            .send()
            .await?;
    }
    Ok(())
}

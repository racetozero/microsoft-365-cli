//! `entra pim *` and `entra roledefinition *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

pub fn assignment_add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(match i {
        1 => {
            (o.get("noExpiration").is_some_and(truthy) as usize
                + o.contains_key("endDateTime") as usize
                + o.contains_key("duration") as usize)
                <= 1
        }
        _ => true,
    })
}

fn is_app_only() -> bool {
    auth::jwt::is_app_only(&auth::default_access_token().unwrap_or_default()) == Some(true)
}

async fn role_definition_id(cx: &Cx) -> Result<String> {
    if let Some(r) = cx.s("roleDefinitionId") {
        return Ok(r);
    }
    let n = cx.u("roleDefinitionName");
    cx.verbose_log(&format!("Retrieving role definition by its name '{n}'"));
    Ok(u(
        &entra::role_definition_by_display_name(&n, None).await?,
        "id",
    ))
}

async fn principal_id(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("userId").or_else(|| cx.s("groupId")) {
        return Ok(i);
    }
    if let Some(n) = cx.s("userName") {
        cx.verbose_log(&format!("Retrieving user by its name '{n}'"));
        return entra::user::id_by_upn(&n).await;
    }
    if let Some(n) = cx.s("groupName") {
        cx.verbose_log(&format!("Retrieving group by its name '{n}'"));
        return entra::group::id_by_display_name(&n).await;
    }
    cx.verbose_log("Retrieving id of the current user");
    Ok(auth::jwt::user_id(
        &auth::default_access_token().unwrap_or_default(),
    ))
}

fn directory_scope(cx: &Cx) -> String {
    if let Some(a) = cx.s("administrativeUnitId") {
        return format!("/administrativeUnits/{a}");
    }
    if let Some(a) = cx.s("applicationId") {
        return format!("/{a}");
    }
    "/".into()
}

fn no_principal(cx: &Cx) -> bool {
    cx.s("userId").is_none()
        && cx.s("userName").is_none()
        && cx.s("groupId").is_none()
        && cx.s("groupName").is_none()
}

pub async fn assignment_add(cx: &mut Cx) -> Result<()> {
    if is_app_only() {
        return Err(Error::msg(
            "When running with application permissions either userId, userName, groupId or groupName is required",
        ));
    }
    let role = role_definition_id(cx).await?;
    let principal = principal_id(cx).await?;
    let duration = if cx.s("duration").is_none()
        && cx.s("endDateTime").is_none()
        && !cx.bool("noExpiration")
    {
        Some("PT8H".to_string())
    } else {
        cx.str("duration")
    };
    let etype = if cx.s("endDateTime").is_some() {
        "afterDateTime"
    } else if cx.bool("noExpiration") {
        "noExpiration"
    } else {
        "afterDuration"
    };
    let body = obj! {
        "principalId": principal,
        "roleDefinitionId": role,
        "directoryScopeId": directory_scope(cx),
        "action": if no_principal(cx) { "selfActivate" } else { "adminAssign" },
        "justification": ?cx.str("justification"),
        "scheduleInfo": obj! {
            "startDateTime": ?cx.str("startDateTime"),
            "expiration": obj! { "duration": ?duration, "endDateTime": ?cx.str("endDateTime"), "type": etype }
        },
        "ticketInfo": obj! { "ticketNumber": ?cx.str("ticketNumber"), "ticketSystem": ?cx.str("ticketSystem") }
    };
    let r = Request::post(format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleAssignmentScheduleRequests"
    ))
    .graph_json()
    .json(body)
    .data()
    .await?;
    cx.log(r)
}

async fn principal_filter(cx: &Cx) -> Result<Option<String>> {
    let mut p = cx.s("userId");
    if let Some(n) = cx.s("userName") {
        cx.verbose_log(&format!("Retrieving user by its name '{n}'"));
        p = Some(entra::user::id_by_upn(&n).await?);
    } else if let Some(g) = cx.s("groupId") {
        p = Some(g);
    } else if let Some(n) = cx.s("groupName") {
        cx.verbose_log(&format!("Retrieving group by its name '{n}'"));
        p = Some(entra::group::id_by_display_name(&n).await?);
    }
    Ok(p.filter(|x| !x.is_empty()))
}

fn who(cx: &Cx) -> String {
    cx.s("userId")
        .or_else(|| cx.s("userName"))
        .or_else(|| cx.s("groupId"))
        .or_else(|| cx.s("groupName"))
        .unwrap_or_else(|| "all users".into())
}

fn move_role_name(items: &mut [Value]) {
    for c in items.iter_mut() {
        if let Some(o) = c.as_object_mut() {
            if let Some(rd) = o.get("roleDefinition").filter(|v| truthy(v)).cloned() {
                o.insert(
                    "roleDefinitionName".into(),
                    rd.get("displayName").cloned().unwrap_or(Value::Null),
                );
            }
            o.shift_remove("roleDefinition");
        }
    }
}

pub async fn assignment_eligibility_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving list of eligible roles for {}...",
        who(cx)
    ));
    let mut q = Vec::new();
    if let Some(p) = principal_filter(cx).await? {
        q.push(format!("$filter=principalId eq '{p}'"));
    }
    let mut exp = vec!["roleDefinition($select=displayName)".to_string()];
    if cx.bool("withPrincipalDetails") {
        exp.push("principal".into());
    }
    q.push(format!("$expand={}", exp.join(",")));
    let mut r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleEligibilityScheduleInstances?{}",
        q.join("&")
    ))
    .await?;
    move_role_name(&mut r);
    cx.log(Value::Array(r))
}

pub async fn assignment_list(cx: &mut Cx) -> Result<()> {
    let mut filters = Vec::new();
    if let Some(p) = principal_filter(cx).await? {
        filters.push(format!("principalId eq '{p}'"));
    }
    if let Some(d) = cx.s("startDateTime") {
        filters.push(format!("startDateTime ge {d}"));
    }
    let mut q = Vec::new();
    if !filters.is_empty() {
        q.push(format!("$filter={}", filters.join(" and ")));
    }
    let mut exp = vec!["roleDefinition($select=displayName)".to_string()];
    if cx.bool("withPrincipalDetails") {
        exp.push("principal".into());
    }
    q.push(format!("$expand={}", exp.join(",")));
    let r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleAssignmentScheduleInstances?{}",
        q.join("&")
    ))
    .await?;
    cx.log(Value::Array(r))
}

pub async fn assignment_remove(cx: &mut Cx) -> Result<()> {
    if is_app_only() && no_principal(cx) {
        return Err(Error::msg(
            "When running with application permissions either userId, userName, groupId or groupName is required",
        ));
    }
    let role = role_definition_id(cx).await?;
    let principal = principal_id(cx).await?;
    let body = obj! {
        "principalId": principal,
        "roleDefinitionId": role,
        "directoryScopeId": directory_scope(cx),
        "action": if no_principal(cx) { "selfDeactivate" } else { "adminRemove" },
        "justification": ?cx.str("justification"),
        "ticketInfo": obj! { "ticketNumber": ?cx.str("ticketNumber"), "ticketSystem": ?cx.str("ticketSystem") }
    };
    let r = Request::post(format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleAssignmentScheduleRequests"
    ))
    .graph_json()
    .json(body)
    .data()
    .await?;
    cx.log(r)
}

pub async fn request_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving list of PIM roles requests for {}...",
        who(cx)
    ));
    let mut filters = Vec::new();
    if let Some(p) = principal_filter(cx).await? {
        filters.push(format!("principalId eq '{p}'"));
    }
    if let Some(d) = cx.s("createdDateTime") {
        filters.push(format!("createdDateTime ge {d}"));
    }
    if let Some(st) = cx.s("status") {
        filters.push(format!("status eq '{st}'"));
    }
    let mut q = Vec::new();
    if !filters.is_empty() {
        q.push(format!("$filter={}", filters.join(" and ")));
    }
    let mut exp = vec!["roleDefinition($select=displayName)".to_string()];
    if cx.bool("withPrincipalDetails") {
        exp.push("principal".into());
    }
    q.push(format!("$expand={}", exp.join(",")));
    let mut r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleAssignmentScheduleRequests?{}",
        q.join("&")
    ))
    .await?;
    move_role_name(&mut r);
    cx.log(Value::Array(r))
}

pub async fn roledefinition_add(cx: &mut Cx) -> Result<()> {
    if cx.bool("verbose") {
        cx.stderr(&format!(
            "Creating custom role definition with name {}...",
            cx.u("displayName")
        ));
    }
    let actions: Vec<Value> = cx
        .u("allowedResourceActions")
        .split(',')
        .map(Value::from)
        .collect();
    let body = obj! {
        "displayName": ?cx.str("displayName"),
        "rolePermissions": vec![obj! { "allowedResourceActions": actions }],
        "description": ?cx.str("description"),
        "isEnabled": cx.opt_bool("enabled").unwrap_or(true),
        "version": ?cx.str("version")
    };
    let r = Request::post(format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleDefinitions"
    ))
    .graph_json()
    .json(body)
    .data()
    .await?;
    cx.log(r)
}

pub fn roledefinition_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(match i {
        0 => !t("id") != !t("displayName"),
        2 => {
            (!t("id") && !t("displayName"))
                || t("displayName")
                || (t("id") && m365_core::validation::is_valid_guid(&to_js_string(&o["id"])))
        }
        3 => [
            "newDisplayName",
            "description",
            "allowedResourceActions",
            "enabled",
            "version",
        ]
        .iter()
        .any(|k| o.contains_key(*k)),
        _ => true,
    })
}

pub async fn roledefinition_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting Microsoft Entra ID role definition...");
    let p = cx.str("properties");
    let r = match cx.s("id") {
        Some(id) => entra::role_definition_by_id(&id, p.as_deref()).await?,
        None => entra::role_definition_by_display_name(&cx.u("displayName"), p.as_deref()).await?,
    };
    cx.log(r)
}

pub async fn roledefinition_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting Microsoft Entra ID role definitions...");
    let mut q = Vec::new();
    if let Some(p) = cx.s("properties") {
        q.push(format!("$select={p}"));
    }
    if let Some(f) = cx.s("filter") {
        q.push(format!("$filter={f}"));
    }
    let r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleDefinitions{}",
        qs(&q)
    ))
    .await?;
    cx.log(Value::Array(r))
}

async fn rd_id(cx: &Cx) -> Result<String> {
    match cx.s("displayName") {
        Some(n) => Ok(u(
            &entra::role_definition_by_display_name(&n, Some("id")).await?,
            "id",
        )),
        None => Ok(cx.u("id")),
    }
}

pub async fn roledefinition_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("displayName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove role definition '{what}'?"),
    )? {
        return Ok(());
    }
    let id = rd_id(cx).await?;
    if cx.bool("verbose") {
        cx.stderr(&format!("Removing role definition with ID {id}..."));
    }
    Request::delete(format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleDefinitions/{id}"
    ))
    .accept("application/json;odata.metadata=none")
    .send()
    .await?;
    Ok(())
}

pub async fn roledefinition_set(cx: &mut Cx) -> Result<()> {
    let id = rd_id(cx).await?;
    if cx.bool("verbose") {
        cx.stderr(&format!("Updating custom role definition with ID {id}..."));
    }
    let mut data = obj! { "displayName": ?cx.str("newDisplayName"), "description": ?cx.str("description"), "isEnabled": ?cx.opt_bool("enabled"), "version": ?cx.str("version") };
    if let Some(a) = cx.s("allowedResourceActions") {
        data.as_object_mut().unwrap().insert("rolePermissions".into(), Value::Array(vec![obj! { "allowedResourceActions": a.split(',').map(Value::from).collect::<Vec<_>>() }]));
    }
    Request::patch(format!(
        "{GRAPH}/v1.0/roleManagement/directory/roleDefinitions/{id}"
    ))
    .graph_json()
    .json(data)
    .send()
    .await?;
    Ok(())
}

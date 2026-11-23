//! `entra administrativeunit *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

pub async fn add(cx: &mut Cx) -> Result<()> {
    let mut body = obj! {
        "description": ?cx.str("description"),
        "displayName": cx.u("displayName"),
        "visibility": if cx.bool("hiddenMembership") { Value::from("HiddenMembership") } else { Value::Null }
    };
    cx.add_unknown_options(&mut body);
    let r = Request::post(format!("{GRAPH}/v1.0/directory/administrativeUnits"))
        .graph_json()
        .json(body)
        .data()
        .await?;
    cx.log(r)
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let r = if let Some(id) = cx.s("id") {
        let q = qs(&entra::select_query(cx.str("properties").as_deref())
            .into_iter()
            .collect::<Vec<_>>());
        Request::get(format!(
            "{GRAPH}/v1.0/directory/administrativeUnits/{id}{q}"
        ))
        .graph_json()
        .data()
        .await?
    } else {
        entra::administrative_unit_by_display_name(&cx.u("displayName"), None).await?
    };
    cx.log(r)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let q = qs(&entra::select_query(cx.str("properties").as_deref())
        .into_iter()
        .collect::<Vec<_>>());
    let r = odata::get_all_items(&format!("{GRAPH}/v1.0/directory/administrativeUnits{q}")).await?;
    cx.log(Value::Array(r))
}

async fn unit_id(cx: &Cx) -> Result<String> {
    if let Some(n) = cx.s("administrativeUnitName") {
        cx.verbose_log("Retrieving Administrative Unit Id...");
        return Ok(s(
            &entra::administrative_unit_by_display_name(&n, None).await?,
            "id",
        )
        .to_string());
    }
    Ok(cx.u("administrativeUnitId"))
}

pub async fn member_add(cx: &mut Cx) -> Result<()> {
    let au = unit_id(cx).await?;
    let (member_type, member_id) = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        let id = match cx.s("userName") {
            Some(u) => {
                cx.verbose_log("Retrieving User Id...");
                entra::user::id_by_upn(&u).await?
            }
            None => cx.u("userId"),
        };
        ("users", id)
    } else if cx.s("groupId").is_some() || cx.s("groupName").is_some() {
        let id = match cx.s("groupName") {
            Some(g) => {
                cx.verbose_log("Retrieving Group Id...");
                entra::group::id_by_display_name(&g).await?
            }
            None => cx.u("groupId"),
        };
        ("groups", id)
    } else if cx.s("deviceId").is_some() || cx.s("deviceName").is_some() {
        let id = match cx.s("deviceName") {
            Some(d) => {
                cx.verbose_log(&format!(
                    "Device with name {d} retrieved, returned id: {}",
                    cx.u("deviceId")
                ));
                s(&entra::device_by_display_name(&d).await?, "id").to_string()
            }
            None => cx.u("deviceId"),
        };
        ("devices", id)
    } else {
        ("undefined", "undefined".to_string())
    };
    Request::post(format!(
        "{GRAPH}/v1.0/directory/administrativeUnits/{au}/members/$ref"
    ))
    .accept("application/json;odata.metadata=none")
    .json(
        obj! { "@odata.id": format!("https://graph.microsoft.com/v1.0/{member_type}/{member_id}") },
    )
    .send()
    .await?;
    Ok(())
}

pub async fn member_get(cx: &mut Cx) -> Result<()> {
    let au = unit_id(cx).await?;
    let q = qs(&select_expand(cx.str("properties").as_deref()));
    let mut r = Request::get(format!(
        "{GRAPH}/v1.0/directory/administrativeUnits/{au}/members/{}{q}",
        cx.u("id")
    ))
    .accept("application/json;odata.metadata=minimal")
    .json_response()
    .data()
    .await?;
    odata_type_to_type(&mut r);
    if let Some(o) = r.as_object_mut() {
        o.shift_remove("@odata.context");
    }
    cx.log(r)
}

pub async fn member_list(cx: &mut Cx) -> Result<()> {
    let au = match cx.s("administrativeUnitName") {
        Some(n) => s(
            &entra::administrative_unit_by_display_name(&n, None).await?,
            "id",
        )
        .to_string(),
        None => cx.u("administrativeUnitId"),
    };
    let mut q = select_expand(cx.str("properties").as_deref());
    if let Some(f) = cx.s("filter") {
        q.push(format!("$filter={f}"));
        q.push("$count=true".into());
    }
    let q = qs(&q);
    let results = match cx.s("type") {
        Some(t) => {
            let url = format!(
                "{GRAPH}/v1.0/directory/administrativeUnits/{au}/members/microsoft.graph.{t}{q}"
            );
            if cx.s("filter").is_some() {
                odata::get_all_items_req(
                    Request::get(url)
                        .accept("application/json;odata.metadata=none")
                        .header("ConsistencyLevel", "eventual")
                        .json_response(),
                )
                .await?
            } else {
                odata::get_all_items(&url).await?
            }
        }
        None => {
            let mut r = odata::get_all_items_meta(
                &format!("{GRAPH}/v1.0/directory/administrativeUnits/{au}/members{q}"),
                "minimal",
            )
            .await?;
            r.iter_mut().for_each(odata_type_to_type);
            r
        }
    };
    cx.log(Value::Array(results))
}

pub async fn member_remove(cx: &mut Cx) -> Result<()> {
    let target = cx.first_u(&["id", "userName", "groupName", "deviceName"]);
    let unit = cx.first_u(&["administrativeUnitId", "administrativeUnitName"]);
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove member '{target}' from administrative unit '{unit}'?"
        ),
    )? {
        return Ok(());
    }
    let au = unit_id(cx).await?;
    let mut member = cx.u("id");
    if cx.s("userId").is_some() || cx.s("userName").is_some() {
        member = match cx.s("userName") {
            Some(u) => {
                cx.verbose_log("Retrieving User Id...");
                entra::user::id_by_upn(&u).await?
            }
            None => cx.u("userId"),
        };
    } else if cx.s("groupId").is_some() || cx.s("groupName").is_some() {
        member = match cx.s("groupName") {
            Some(g) => {
                cx.verbose_log("Retrieving Group Id...");
                entra::group::id_by_display_name(&g).await?
            }
            None => cx.u("groupId"),
        };
    } else if cx.s("deviceId").is_some() || cx.s("deviceName").is_some() {
        member = match cx.s("deviceName") {
            Some(d) => {
                cx.verbose_log("Retrieving Device Id");
                s(&entra::device_by_display_name(&d).await?, "id").to_string()
            }
            None => cx.u("deviceId"),
        };
    }
    Request::delete(format!(
        "{GRAPH}/v1.0/directory/administrativeUnits/{au}/members/{member}/$ref"
    ))
    .accept("application/json;odata.metadata=none")
    .send()
    .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "displayName"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove administrative unit '{what}'?"),
    )? {
        return Ok(());
    }
    let id = match cx.s("displayName") {
        Some(n) => s(
            &entra::administrative_unit_by_display_name(&n, None).await?,
            "id",
        )
        .to_string(),
        None => cx.u("id"),
    };
    Request::delete(format!("{GRAPH}/v1.0/directory/administrativeUnits/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let mut au = cx.u("administrativeUnitId");
    let mut role = cx.u("roleDefinitionId");
    let mut user = cx.u("userId");
    if let Some(n) = cx.s("administrativeUnitName") {
        cx.verbose_log(&format!("Retrieving administrative unit by its name '{n}'"));
        au = s(
            &entra::administrative_unit_by_display_name(&n, None).await?,
            "id",
        )
        .to_string();
    }
    if let Some(n) = cx.s("roleDefinitionName") {
        cx.verbose_log(&format!("Retrieving role definition by its name '{n}'"));
        role = s(
            &entra::role_definition_by_display_name(&n, None).await?,
            "id",
        )
        .to_string();
    }
    if let Some(n) = cx.s("userName") {
        cx.verbose_log(&format!("Retrieving user by UPN '{n}'"));
        user = entra::user::id_by_upn(&n).await?;
    }
    let r =
        entra::create_role_assignment(&role, &user, &format!("/administrativeUnits/{au}")).await?;
    cx.log(r)
}

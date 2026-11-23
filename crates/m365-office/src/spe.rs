//! `spe *`: SharePoint Embedded containers and container types.

use m365_core::prelude::*;
use m365_core::ut::{entra, spo};

const FS: &str = "https://graph.microsoft.com/v1.0/storage/fileStorage";
const FS_BETA: &str = "https://graph.microsoft.com/beta/storage/fileStorage";
const OVERRIDABLES: [&str; 6] = [
    "urlTemplate",
    "isDiscoverabilityEnabled",
    "isSearchEnabled",
    "isItemVersioningEnabled",
    "itemMajorVersionLimit",
    "maxStoragePerContainerInBytes",
];

fn pick(items: Vec<Value>, name: &str, what: &str, multiple: &str, key: &str) -> Result<Value> {
    m365_core::runner::pick_one(
        &items,
        key,
        Error::js(format!("The specified {what} '{name}' does not exist.")),
        &format!("Multiple {multiple} with name '{name}' found."),
    )
}

/// `spe.getContainerTypeIdByName`.
async fn container_type_id_by_name(name: &str) -> Result<String> {
    let items = odata::get_all_items(&format!(
        "{FS}/containerTypes?$select=id,name&$filter=name eq '{}'",
        eqp(name)
    ))
    .await?;
    Ok(u(
        &pick(items, name, "container type", "container types", "id")?,
        "id",
    ))
}

/// `spe.getContainerIdByName` (or deleted containers).
async fn container_id_by_name(type_id: &str, name: &str, deleted: bool) -> Result<String> {
    let set = if deleted {
        "deletedContainers"
    } else {
        "containers"
    };
    let items: Vec<Value> = odata::get_all_items(&format!(
        "{FS}/{set}?$filter=containerTypeId eq {type_id}&$select=id,displayName"
    ))
    .await?
    .into_iter()
    .filter(|c| s(c, "displayName").to_lowercase() == name.to_lowercase())
    .collect();
    Ok(u(
        &pick(items, name, "container", "containers", "id")?,
        "id",
    ))
}

async fn type_id(cx: &Cx, log: &str) -> Result<String> {
    match cx.s("containerTypeId") {
        Some(t) => Ok(t),
        None => {
            let n = cx.u("containerTypeName");
            if !log.is_empty() {
                cx.verbose_log(&log.replace("{}", &n));
            }
            container_type_id_by_name(&n).await
        }
    }
}

const TYPE_LOG: &str = "Getting container type with name '{}'...";

/// `containerId`/`containerName` (permission commands): the name lookup log
/// comes after the container type.
async fn named_container(
    cx: &Cx,
    id_key: &str,
    name_key: &str,
    log_before: bool,
) -> Result<String> {
    if let Some(i) = cx.s(id_key) {
        return Ok(i);
    }
    let n = cx.u(name_key);
    if log_before {
        cx.verbose_log(&format!("Resolving container id from name '{n}'..."));
    }
    let t = type_id(cx, TYPE_LOG).await?;
    if !log_before {
        cx.verbose_log(&format!(
            "Getting container ID for container with name '{n}'..."
        ));
    }
    container_id_by_name(&t, &n, false).await
}

pub async fn container_activate(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Activating a container with id '{id}'..."));
    Request::post(format!("{FS}/containers/{}/activate", eqp(&id)))
        .header("content-type", "application/json;odata=nometadata")
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn container_add(cx: &mut Cx) -> Result<()> {
    let t = type_id(cx, TYPE_LOG).await?;
    cx.verbose_log(&format!(
        "Creating container with name '{}'...",
        cx.u("name")
    ));
    let data = obj! {
        "displayName": ?cx.val("name").cloned(),
        "description": ?cx.val("description").cloned(),
        "containerTypeId": t,
        "settings": obj! { "isOcrEnabled": ?cx.val("ocrEnabled").cloned(), "itemMajorVersionLimit": ?cx.val("itemMajorVersionLimit").cloned(), "isItemVersioningEnabled": ?cx.val("itemVersioningEnabled").cloned() },
    };
    cx.log(
        Request::post(format!("{FS}/containers"))
            .graph_json()
            .json(data)
            .data()
            .await?,
    )
}

pub async fn container_get(cx: &mut Cx) -> Result<()> {
    let id = match cx.s("id") {
        Some(i) => i,
        None => {
            let n = cx.u("name");
            cx.verbose_log(&format!("Resolving container id from name '{n}'..."));
            let t = type_id(cx, TYPE_LOG).await?;
            container_id_by_name(&t, &n, false).await?
        }
    };
    cx.verbose_log(&format!("Getting a container with id '{id}'..."));
    cx.log(
        Request::get(format!("{FS}/containers/{id}"))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn container_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of Containers...");
    let t = type_id(cx, "").await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{FS}/containers?$filter=containerTypeId eq {}",
            eqp(&t)
        ))
        .await?,
    ))
}

fn roles(cx: &Cx) -> Value {
    Value::from(
        cx.u("roles")
            .split(',')
            .map(|r| r.trim().to_string())
            .collect::<Vec<_>>(),
    )
}

pub async fn container_permission_add(cx: &mut Cx) -> Result<()> {
    let id = named_container(cx, "containerId", "containerName", false).await?;
    cx.verbose_log(&format!(
        "Adding permissions to container with ID '{id}'..."
    ));
    let upn = match cx.s("userId") {
        Some(u) => Some(entra::user::upn_by_id(&u, false).await?),
        None => cx.s("userName"),
    };
    let data = obj! { "roles": roles(cx), "grantedToV2": obj! { "user": obj! { "userPrincipalName": ?upn } } };
    cx.log(
        Request::post(format!("{FS}/containers/{}/permissions", eqp(&id)))
            .graph_json()
            .json(data)
            .data()
            .await?,
    )
}

pub async fn container_permission_list(cx: &mut Cx) -> Result<()> {
    let id = named_container(cx, "containerId", "containerName", true).await?;
    cx.verbose_log(&format!(
        "Retrieving permissions of a SharePoint Embedded Container with id '{id}'..."
    ));
    let items = odata::get_all_items(&format!("{FS}/containers/{}/permissions", eqp(&id))).await?;
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(items));
    }
    let mut out = Vec::new();
    for i in &items {
        let r = i
            .get("roles")
            .and_then(Value::as_array)
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'join')"))?;
        let upn = i
            .get("grantedToV2")
            .and_then(|g| g.get("user"))
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'user')"))?
            .get("userPrincipalName")
            .cloned();
        out.push(obj! { "id": ?i.get("id").cloned(), "roles": r.iter().map(to_js_string).collect::<Vec<_>>().join(","), "userPrincipalName": ?upn });
    }
    cx.log(Value::Array(out))
}

pub async fn container_permission_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["containerId", "containerName"]);
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove permission '{}' from container '{what}'?",
            cx.u("id")
        ),
    )? {
        return Ok(());
    }
    let id = named_container(cx, "containerId", "containerName", false).await?;
    cx.verbose_log(&format!(
        "Removing permissions from container with ID '{id}'..."
    ));
    Request::delete(format!(
        "{FS}/containers/{}/permissions/{}",
        eqp(&id),
        eqp(&cx.u("id"))
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn container_permission_set(cx: &mut Cx) -> Result<()> {
    let id = named_container(cx, "containerId", "containerName", false).await?;
    cx.verbose_log(&format!(
        "Updating permission for container with ID '{id}'..."
    ));
    Request::patch(format!(
        "{FS}/containers/{}/permissions/{}",
        eqp(&id),
        eqp(&cx.u("id"))
    ))
    .graph_json()
    .json(obj! { "roles": roles(cx) })
    .send()
    .await?;
    Ok(())
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    let t = type_id(
        cx,
        "Retrieving container type id for container type '{}'...",
    )
    .await?;
    cx.verbose_log(&format!(
        "Retrieving deleted containers of container type with ID '{t}'..."
    ));
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{FS}/deletedContainers?$filter=containerTypeId eq {t}"
        ))
        .await?,
    ))
}

async fn deleted_container(cx: &Cx) -> Result<String> {
    if let Some(i) = cx.s("id") {
        return Ok(i);
    }
    let n = cx.u("name");
    cx.verbose_log(&format!("Retrieving container with name '{n}'..."));
    let t = type_id(cx, TYPE_LOG).await?;
    container_id_by_name(&t, &n, true).await
}

pub async fn recyclebinitem_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to permanently remove deleted container '{what}'?"),
    )? {
        return Ok(());
    }
    let id = deleted_container(cx).await?;
    cx.verbose_log(&format!(
        "Permanently removing deleted container with ID '{id}'..."
    ));
    Request::delete(format!("{FS}/deletedContainers/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let id = deleted_container(cx).await?;
    cx.verbose_log(&format!("Restoring deleted container with ID '{id}'..."));
    Request::post(format!("{FS}/deletedContainers/{id}/restore"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn container_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    let verb = if cx.bool("permanent") {
        "permanently remove"
    } else {
        "recycle"
    };
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to {verb} container '{what}'?"),
    )? {
        return Ok(());
    }
    let id = named_container(cx, "id", "name", false).await?;
    cx.verbose_log(&format!("Removing container with ID '{id}'..."));
    let url = format!("{FS}/containers/{id}");
    if cx.bool("permanent") {
        Request::post(format!("{url}/permanentDelete"))
            .graph_json()
            .send()
            .await?;
    } else {
        Request::delete(url).graph_json().send().await?;
    }
    Ok(())
}

pub async fn container_set(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Updating container '{id}'..."));
    let mut data = obj! { "displayName": ?cx.val("newName").cloned(), "description": ?cx.val("description").cloned() };
    let settings = obj! { "isOcrEnabled": ?cx.val("isOcrEnabled").cloned(), "isItemVersioningEnabled": ?cx.val("isItemVersioningEnabled").cloned(), "itemMajorVersionLimit": ?cx.val("itemMajorVersionLimit").cloned() };
    if settings.as_object().is_some_and(|o| !o.is_empty())
        && let Some(o) = data.as_object_mut()
    {
        o.insert("settings".into(), settings);
    }
    cx.log(
        Request::patch(format!("{FS}/containers/{id}"))
            .graph_json()
            .json(data)
            .data()
            .await?,
    )
}

pub fn containertype_add_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(
        to_js_string(v)
            .split(',')
            .all(|x| OVERRIDABLES.contains(&x.trim())),
    )
}

pub async fn containertype_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Creating a new Container Type for your app with name '{}'.",
        cx.u("name")
    ));
    let app = cx.s("appId").or_else(|| auth::connection().app_id.clone());
    let data = obj! {
        "name": ?cx.val("name").cloned(),
        "owningAppId": ?app,
        "billingClassification": ?cx.val("billingType").cloned(),
        "settings": obj! {
            "consumingTenantOverridables": ?cx.str("consumingTenantOverridables").map(|c| c.split(',').map(str::trim).collect::<Vec<_>>().join(",")),
            "isDiscoverabilityEnabled": ?cx.val("isDiscoverabilityEnabled").cloned(),
            "isItemVersioningEnabled": ?cx.val("isItemVersioningEnabled").cloned(),
            "isSearchEnabled": ?cx.val("isSearchEnabled").cloned(),
            "isSharingRestricted": ?cx.val("isSharingRestricted").cloned(),
            "itemMajorVersionLimit": ?cx.val("itemMajorVersionLimit").cloned(),
            "maxStoragePerContainerInBytes": ?cx.val("maxStoragePerContainerInBytes").cloned(),
            "sharingCapability": ?cx.val("sharingCapability").cloned(),
            "urlTemplate": ?cx.val("urlTemplate").cloned(),
        },
    };
    cx.log(
        Request::post(format!("{FS_BETA}/containerTypes"))
            .graph_json()
            .json(data)
            .data()
            .await?,
    )
}

pub async fn containertype_get(cx: &mut Cx) -> Result<()> {
    let r = match cx.s("name") {
        Some(n) => {
            let items = odata::get_all_items(&format!(
                "{FS_BETA}/containerTypes?$filter=name eq '{}'",
                eqp(&n)
            ))
            .await?;
            match items.len() {
                0 => {
                    return Err(Error::msg(format!(
                        "The specified container type '{n}' does not exist."
                    )));
                }
                1 => items.into_iter().next().unwrap_or(Value::Null),
                _ => cx.handle_multiple_results(
                    &format!("Multiple container types with name '{n}' found."),
                    &util::array_to_hash_table("id", &items),
                )?,
            }
        }
        None => {
            Request::get(format!("{FS_BETA}/containerTypes/{}", cx.u("id")))
                .graph_json()
                .data()
                .await?
        }
    };
    cx.log(r)
}

pub async fn containertype_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of Container types...");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{FS_BETA}/containerTypes")).await?,
    ))
}

async fn csom(admin: &str, body: String) -> Result<Value> {
    Request::post(format!("{admin}/_vti_bin/client.svc/ProcessQuery"))
        .spo_json()
        .text(body)
        .data()
        .await
}

pub async fn containertype_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove container type {what}?"),
    )? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.verbose()).await?;
    let app = spo::application_name();
    let id = match cx.s("id") {
        Some(i) => i,
        None => {
            let n = cx.u("name");
            cx.verbose_log(&format!(
                "Retrieving container type id for container type '{n}'..."
            ));
            let json = csom(&admin, format!(r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{app}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="46" ObjectPathId="45" /><Method Name="GetSPOContainerTypes" Id="47" ObjectPathId="45"><Parameters><Parameter Type="Enum">1</Parameter></Parameters></Method></Actions><ObjectPaths><Constructor Id="45" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /></ObjectPaths></Request>"#)).await?;
            let first = json.get(0).ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'ErrorInfo')")
            })?;
            if let Some(e) = first.get("ErrorInfo").filter(|e| truthy(e)) {
                return Err(Error::js(u(e, "ErrorMessage")));
            }
            let all = json
                .as_array()
                .and_then(|a| a.last())
                .cloned()
                .unwrap_or(Value::Null);
            let types: Vec<Value> = all
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .filter(|t| s(t, "DisplayName").to_lowercase() == n.to_lowercase())
                .collect();
            let t = match types.len() {
                0 => {
                    return Err(Error::js(format!(
                        "The specified container type '{n}' does not exist."
                    )));
                }
                1 => types.into_iter().next().unwrap_or(Value::Null),
                _ => cx.handle_multiple_results(
                    &format!("Multiple container types with name '{n}' found."),
                    &util::array_to_hash_table("ContainerTypeId", &types),
                )?,
            };
            util::extract_csom_guid(&u(&t, "ContainerTypeId"))
        }
    };
    cx.verbose_log(&format!("Removing container type '{what}'..."));
    let r = csom(&admin, format!(r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{app}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions><ObjectPath Id="7" ObjectPathId="6" /><Method Name="RemoveSPOContainerType" Id="8" ObjectPathId="6"><Parameters><Parameter TypeId="{{b66ab1ca-fd51-44f9-8cfc-01f5c2a21f99}}"><Property Name="ContainerTypeId" Type="Guid">{{{id}}}</Property></Parameter></Parameters></Method></Actions><ObjectPaths><Constructor Id="6" TypeId="{{268004ae-ef6b-4e9b-8425-127220d84719}}" /></ObjectPaths></Request>"#)).await?;
    let first = r
        .get(0)
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'ErrorInfo')"))?;
    if let Some(e) = first.get("ErrorInfo").filter(|e| truthy(e)) {
        return Err(Error::Thrown(
            e.get("ErrorMessage").cloned().unwrap_or(Value::Null),
        ));
    }
    Ok(())
}

//! `src/utils/entraServicePrincipal.ts`.

use crate::error::{Error, Result};
use crate::http::Request;
use crate::odata;
use crate::util::encode_query_parameter;
use serde_json::Value;

pub async fn get_by_app_id(app_id: &str, properties: Option<&str>) -> Result<Value> {
    let mut url =
        format!("https://graph.microsoft.com/v1.0/servicePrincipals?$filter=appId eq '{app_id}'");
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("&$select={p}"));
    }
    let apps = odata::get_all_items(&url).await?;
    apps.into_iter().next().ok_or_else(|| {
        Error::js(format!(
            "Service principal with appId '{app_id}' not found in Microsoft Entra ID."
        ))
    })
}

pub async fn get_by_app_name(name: &str, properties: Option<&str>) -> Result<Value> {
    let mut url = format!(
        "https://graph.microsoft.com/v1.0/servicePrincipals?$filter=displayName eq '{}'",
        encode_query_parameter(name)
    );
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("&$select={p}"));
    }
    let apps = odata::get_all_items(&url).await?;
    crate::runner::pick_one(
        &apps,
        "id",
        Error::js(format!(
            "Service principal with name '{name}' not found in Microsoft Entra ID."
        )),
        &format!("Multiple service principals with name '{name}' found in Microsoft Entra ID."),
    )
}

pub async fn get_all(properties: Option<&str>) -> Result<Vec<Value>> {
    let mut url = "https://graph.microsoft.com/v1.0/servicePrincipals".to_string();
    if let Some(p) = properties.filter(|p| !p.is_empty()) {
        url.push_str(&format!("?$select={p}"));
    }
    odata::get_all_items(&url).await
}

pub async fn create(app_id: &str) -> Result<Value> {
    Request::post("https://graph.microsoft.com/v1.0/servicePrincipals")
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json;odata=nometadata")
        .json(crate::obj! { "appId": app_id })
        .json_response()
        .data()
        .await
}

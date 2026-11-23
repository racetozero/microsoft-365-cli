//! `planner.ts`.

use crate::Value;
use crate::error::{Error, Result};
use crate::http::Request;
use crate::json::{s, u};
use crate::odata;

const GRAPH: &str = "https://graph.microsoft.com";

fn get(url: String, metadata: &str) -> Request {
    Request::get(url)
        .accept(&format!("application/json;odata.metadata={metadata}"))
        .json_response()
}

pub async fn plan_by_id(id: &str, metadata: &str) -> Result<Value> {
    get(format!("{GRAPH}/v1.0/planner/plans/{id}"), metadata)
        .data()
        .await
        .map_err(|_| Error::js(format!("Planner plan with id '{id}' was not found.")))
}

pub async fn plans_by_group_id(group_id: &str, metadata: &str) -> Result<Vec<Value>> {
    odata::get_all_items_meta(
        &format!("{GRAPH}/v1.0/groups/{group_id}/planner/plans"),
        metadata,
    )
    .await
}

pub async fn plan_by_roster_id(roster_id: &str, metadata: &str) -> Result<Value> {
    let plans = odata::get_all_items_meta(
        &format!("{GRAPH}/beta/planner/rosters/{roster_id}/plans"),
        metadata,
    )
    .await?;
    plans.into_iter().next().ok_or_else(|| {
        Error::js(format!(
            "The specified roster '{roster_id}' does not have a plan."
        ))
    })
}

pub async fn plan_id_by_roster_id(roster_id: &str) -> Result<String> {
    let plans = odata::get_all_items(&format!(
        "{GRAPH}/beta/planner/rosters/{roster_id}/plans?$select=id"
    ))
    .await?;
    plans.first().map(|p| u(p, "id")).ok_or_else(|| {
        Error::js(format!(
            "The specified roster '{roster_id}' does not have a plan."
        ))
    })
}

fn pick(items: Vec<Value>, key: &str, title: &str, what: &str, multiple: String) -> Result<Value> {
    let lower = title.to_lowercase();
    let f: Vec<Value> = items
        .into_iter()
        .filter(|p| !s(p, key).is_empty() && s(p, key).to_lowercase() == lower)
        .collect();
    crate::runner::pick_one(
        &f,
        "id",
        Error::js(format!("The specified {what} '{title}' does not exist.")),
        &multiple,
    )
}

pub async fn plan_by_title(title: &str, group_id: &str, metadata: &str) -> Result<Value> {
    let plans = plans_by_group_id(group_id, metadata).await?;
    pick(
        plans,
        "title",
        title,
        "plan",
        format!("Multiple plans with title '{title}' found."),
    )
}

pub async fn plan_id_by_title(title: &str, group_id: &str) -> Result<String> {
    let plans = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/groups/{group_id}/planner/plans?$select=id,title"
    ))
    .await?;
    Ok(u(
        &pick(
            plans,
            "title",
            title,
            "plan",
            format!("Multiple plans with title '{title}' found."),
        )?,
        "id",
    ))
}

pub async fn bucket_by_title(title: &str, plan_id: &str, metadata: &str) -> Result<Value> {
    let b = odata::get_all_items_meta(
        &format!("{GRAPH}/v1.0/planner/plans/{plan_id}/buckets"),
        metadata,
    )
    .await?;
    pick(
        b,
        "name",
        title,
        "bucket",
        format!("Multiple buckets with name '{title}' found."),
    )
}

pub async fn bucket_id_by_title(title: &str, plan_id: &str) -> Result<String> {
    let b = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/planner/plans/{plan_id}/buckets?$select=id,name"
    ))
    .await?;
    Ok(u(
        &pick(
            b,
            "name",
            title,
            "bucket",
            format!("Multiple buckets with name '{title}' found."),
        )?,
        "id",
    ))
}

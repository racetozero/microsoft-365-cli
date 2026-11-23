//! `onenote *` and `booking *`.

use m365_core::prelude::*;
use m365_core::ut::{entra, spo};

async fn onenote_base(cx: &Cx, by_group_object: bool) -> Result<String> {
    let mut e = format!("{GRAPH}/v1.0/");
    if let Some(u) = cx.s("userId") {
        e.push_str(&format!("users/{u}"));
    } else if let Some(n) = cx.s("userName") {
        e.push_str(&format!("users/{}", eqp(&n)));
    } else if let Some(g) = cx.s("groupId") {
        e.push_str(&format!("groups/{g}"));
    } else if let Some(n) = cx.s("groupName") {
        let id = if by_group_object {
            u(&entra::group::by_display_name(&n, None).await?, "id")
        } else {
            entra::group::id_by_display_name(&n).await?
        };
        e.push_str(&format!("groups/{id}"));
    } else if let Some(w) = cx.s("webUrl") {
        e.push_str(&format!("sites/{}", spo::graph_site_id(&w).await?));
    } else {
        e.push_str("me");
    }
    Ok(e)
}

pub async fn notebook_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Creating OneNote notebook {}", cx.u("name")));
    let url = format!("{}/onenote/notebooks", onenote_base(cx, false).await?);
    let r = Request::post(url)
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "displayName": ?cx.val("name").cloned() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn notebook_list(cx: &mut Cx) -> Result<()> {
    let url = format!("{}/onenote/notebooks", onenote_base(cx, true).await?);
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn page_list(cx: &mut Cx) -> Result<()> {
    let url = format!("{}/onenote/pages", onenote_base(cx, false).await?);
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn business_get(cx: &mut Cx) -> Result<()> {
    let id = match cx.s("id") {
        Some(i) => i,
        None => {
            let name = cx.u("name");
            let r = Request::get(format!("{GRAPH}/v1.0/solutions/bookingBusinesses"))
                .graph_json()
                .data()
                .await?;
            let f: Vec<Value> = arr(&r, "value")
                .iter()
                .filter(|b| {
                    b.get("displayName")
                        .and_then(Value::as_str)
                        .map(str::to_lowercase)
                        == Some(name.to_lowercase())
                })
                .cloned()
                .collect();
            match f.len() {
                0 => {
                    return Err(Error::msg(format!(
                        "The specified business with name {name} does not exist."
                    )));
                }
                1 => u(&f[0], "id"),
                _ => u(
                    &cx.handle_multiple_results(
                        &format!("Multiple businesses with name '{name}' found."),
                        &util::array_to_hash_table("id", &f),
                    )?,
                    "id",
                ),
            }
        }
    };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/solutions/bookingBusinesses/{}",
        eqp(&id)
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn business_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/solutions/bookingBusinesses")).await?,
    ))
}

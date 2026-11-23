//! `entra groupsetting *` and `entra groupsettingtemplate *`.

use m365_core::prelude::*;

fn values(cx: &Cx, exclude: &[&str], existing: &[Value], value_key: &str) -> Vec<Value> {
    let mut v: Vec<Value> = cx
        .opts
        .iter()
        .filter(|(k, _)| !exclude.contains(&k.as_str()))
        .map(|(k, val)| obj! { "name": k.as_str(), "value": val.clone() })
        .collect();
    for e in existing {
        if !v.iter().any(|x| x.get("name") == e.get("name")) {
            v.push(obj! { "name": ?e.get("name").cloned(), "value": ?e.get(value_key).cloned() });
        }
    }
    v
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let tid = cx.u("templateId");
    cx.verbose_log(&format!(
        "Retrieving group setting template with id '{tid}'..."
    ));
    let t = Request::get(format!("{GRAPH}/v1.0/groupSettingTemplates/{tid}"))
        .graph_json()
        .data()
        .await?;
    let vals = values(
        cx,
        &["templateId", "debug", "verbose", "output"],
        need_arr(&t, "values", "forEach")?,
        "defaultValue",
    );
    let r = Request::post(format!("{GRAPH}/v1.0/groupSettings"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json")
        .json(obj! { "templateId": tid, "values": vals })
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!("{GRAPH}/v1.0/groupSettings/{}", cx.u("id")))
        .graph_json()
        .data()
        .await?;
    cx.log(r)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/groupSettings")).await?,
    ))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the group setting {id}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing group setting: {id}..."));
    Request::delete(format!("{GRAPH}/v1.0/groupSettings/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving group setting with id '{id}'..."));
    let g = Request::get(format!("{GRAPH}/v1.0/groupSettings/{id}"))
        .graph_json()
        .data()
        .await?;
    let vals = values(
        cx,
        &["id", "debug", "verbose", "output"],
        need_arr(&g, "values", "forEach")?,
        "value",
    );
    Request::patch(format!("{GRAPH}/v1.0/groupSettings/{id}"))
        .accept("application/json;odata.metadata=none")
        .header("content-type", "application/json")
        .json(obj! { "displayName": ?g.get("displayName").cloned(), "templateId": ?g.get("templateId").cloned(), "values": vals })
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub async fn template_get(cx: &mut Cx) -> Result<()> {
    let all = odata::get_all_items(&format!("{GRAPH}/v1.0/groupSettingTemplates")).await?;
    let id = cx.s("id");
    let mut found: Vec<Value> = all
        .into_iter()
        .filter(|t| match &id {
            Some(i) => s(t, "id") == i,
            None => t.get("displayName").map(to_js_string) == cx.str("displayName"),
        })
        .collect();
    match found.pop() {
        Some(t) => cx.log(t),
        None => Err(Error::msg(format!(
            "Resource '{}' does not exist.",
            id.or_else(|| cx.s("displayName"))
                .unwrap_or_else(|| "undefined".into())
        ))),
    }
}

pub async fn template_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/groupSettingTemplates")).await?,
    ))
}

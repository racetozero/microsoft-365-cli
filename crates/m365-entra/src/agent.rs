//! `entra agent *`.

use m365_core::prelude::*;

fn props(cx: &Cx, strip_quotes: bool) -> Vec<String> {
    cx.s("properties")
        .map(|p| {
            p.split(',')
                .map(|x| {
                    if strip_quotes {
                        x.replace(['\'', '"'], "").trim().to_string()
                    } else {
                        x.trim().to_string()
                    }
                })
                .filter(|x| !x.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

pub async fn blueprint_list(cx: &mut Cx) -> Result<()> {
    let sel: Vec<String> = props(cx, false)
        .into_iter()
        .filter(|p| !p.contains('/'))
        .collect();
    let q = if sel.is_empty() {
        String::new()
    } else {
        format!("?$select={}", eqp(&sel.join(",")))
    };
    cx.verbose_log("Retrieving agent identity blueprints...");
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/applications/microsoft.graph.agentIdentityBlueprint{q}"
        ))
        .await?,
    ))
}

pub async fn blueprintprincipal_list(cx: &mut Cx) -> Result<()> {
    let mut sel: Vec<String> = Vec::new();
    for p in props(cx, false) {
        let base = p.split('/').next().unwrap_or("").to_string();
        if !sel.contains(&base) {
            sel.push(base);
        }
    }
    let q = if sel.is_empty() {
        String::new()
    } else {
        format!(
            "?$select={}",
            sel.iter().map(|p| eqp(p)).collect::<Vec<_>>().join(",")
        )
    };
    if !sel.is_empty() {
        m365_core::log::set_default_properties(sel.clone());
    }
    cx.verbose_log("Retrieving a list of agent blueprint principals...");
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/servicePrincipals/microsoft.graph.agentIdentityBlueprintPrincipal{q}"
        ))
        .await?,
    ))
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let sel: Vec<String> = props(cx, true)
        .into_iter()
        .filter(|p| !p.contains('/'))
        .collect();
    let q = if sel.is_empty() {
        String::new()
    } else {
        format!("?$select={}", sel.join(","))
    };
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/users/microsoft.graph.agentUser{q}")).await?,
    ))
}

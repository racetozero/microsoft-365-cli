//! `spo navigation *`, `spo eventreceiver *`, `spo feature *` and `spo roledefinition *`.

use crate::sp;
use m365_core::prelude::*;

pub async fn node_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let title = cx.u("title");
    cx.verbose_log("Adding navigation node...");
    let parent = cx.s("parentNodeId");
    let (mut state, path) = match &parent {
        Some(p) => {
            let (s, path) = sp::menu_node_by_key(&web, p, None).await?;
            (s, Some(path))
        }
        None => {
            let s = if cx.str("location").as_deref() == Some("TopNavigationBar") {
                sp::top_navigation_menu_state(&web).await?
            } else {
                sp::quick_launch_menu_state(&web).await?
            };
            (s, None)
        }
    };
    let siblings = match &path {
        Some(p) => sp::menu_node_mut(&mut state, p).and_then(|n| n.get_mut("Nodes")),
        None => state.get_mut("Nodes"),
    };
    let Some(Value::Array(siblings)) = siblings else {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'map')",
        ));
    };
    let existing: Vec<Value> = siblings
        .iter()
        .map(|n| n.get("Key").cloned().unwrap_or(Value::Null))
        .collect();
    let audience: Vec<Value> = cx
        .s("audienceIds")
        .map(|a| a.split(',').map(Value::from).collect())
        .unwrap_or_default();
    siblings.push(obj! {
        "AudienceIds": audience,
        "IsDeleted": false,
        "IsHidden": false,
        "Key": Value::Null,
        "Nodes": Vec::<Value>::new(),
        "NodeType": 0,
        "OpenInNewWindow": if cx.bool("openInNewWindow") { Value::Bool(true) } else { Value::Null },
        "SimpleUrl": cx.str("url").unwrap_or_default(),
        "Title": title.as_str(),
    });
    sp::save_menu_state(&web, &state).await?;
    cx.verbose_log("Retrieving the added navigation node...");
    let start = state
        .get("StartingNodeKey")
        .filter(|v| truthy(v))
        .map(to_js_string);
    let updated = sp::menu_state(&web, start.as_deref()).await?;
    let nodes = need_arr(&updated, "Nodes", "iterator")?;
    let found_siblings: Option<Vec<Value>> = match &parent {
        Some(p) => match sp::find_menu_node(nodes, p)? {
            Some(path) => {
                let mut tmp = updated.clone();
                sp::menu_node_mut(&mut tmp, &path)
                    .and_then(|n| n.get("Nodes"))
                    .and_then(Value::as_array)
                    .cloned()
            }
            None => None,
        },
        None => Some(nodes.to_vec()),
    };
    let added = found_siblings.and_then(|s| {
        s.into_iter().find(|n| {
            !existing.contains(n.get("Key").unwrap_or(&Value::Null)) && s_eq(n, "Title", &title)
        })
    });
    match added {
        Some(n) => cx.log(n),
        None => Err(Error::msg(
            "The navigation node was added, but it couldn't be retrieved.",
        )),
    }
}

fn s_eq(v: &Value, k: &str, want: &str) -> bool {
    v.get(k).and_then(Value::as_str) == Some(want)
}

pub async fn node_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!(
        "Retrieving information about navigation node with id {id}"
    ));
    let (mut state, path) =
        sp::menu_node_by_key(&cx.u("webUrl"), &id, cx.str("location").as_deref()).await?;
    cx.log(
        sp::menu_node_mut(&mut state, &path)
            .cloned()
            .unwrap_or(Value::Null),
    )
}

pub async fn node_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving navigation nodes...");
    let web = cx.u("webUrl");
    let s = if cx.str("location").as_deref() == Some("TopNavigationBar") {
        sp::top_navigation_menu_state(&web).await?
    } else {
        sp::quick_launch_menu_state(&web).await?
    };
    match s.get("Nodes") {
        Some(n) => cx.log(n.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn node_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the node {id} from the navigation?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    let d = sp::digest(&web).await?;
    cx.verbose_log("Removing navigation node...");
    Request::delete(format!(
        "{web}/_api/web/navigation/{}/getbyid({id})",
        cx.u("location").to_lowercase()
    ))
    .spo_json()
    .header("X-RequestDigest", d)
    .send()
    .await?;
    Ok(())
}

pub async fn node_set(cx: &mut Cx) -> Result<()> {
    let (web, id) = (cx.u("webUrl"), cx.u("id"));
    cx.verbose_log(&format!("Updating navigation node with id {id}..."));
    let (mut state, path) = sp::menu_node_by_key(&web, &id, cx.str("location").as_deref()).await?;
    if let Some(n) = sp::menu_node_mut(&mut state, &path).and_then(Value::as_object_mut) {
        if let Some(t) = cx.val("title") {
            n.insert("Title".into(), t.clone());
        }
        if let Some(u) = cx.val("url") {
            n.insert("SimpleUrl".into(), u.clone());
        }
        if let Some(a) = cx.str("audienceIds") {
            n.insert(
                "AudienceIds".into(),
                if a.is_empty() {
                    Value::Array(Vec::new())
                } else {
                    Value::from(a.split(',').map(String::from).collect::<Vec<_>>())
                },
            );
        }
        if let Some(o) = cx.val("openInNewWindow") {
            n.insert("OpenInNewWindow".into(), o.clone());
        }
    }
    sp::save_menu_state(&web, &state).await
}

// ---- eventreceiver -------------------------------------------------------------

fn list_part(cx: &Cx, web: &str) -> String {
    sp::list_segment(cx, web, sp::LIST, "getByTitle")
        .map(|s| format!("/{s}"))
        .unwrap_or_default()
}

pub async fn eventreceiver_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let scope = if cx.str("scope").as_deref() == Some("site") {
        "/site"
    } else {
        "/web"
    };
    let base = format!("{web}/_api{scope}{}/eventreceivers", list_part(cx, &web));
    let r = if let Some(id) = cx.s("id") {
        let r = Request::get(format!("{base}(guid'{id}')"))
            .spo_json()
            .data()
            .await?;
        if r.get("odata.null").is_some_and(truthy) {
            return Err(Error::msg(format!(
                "The specified event receiver '{id}' does not exist."
            )));
        }
        r
    } else {
        let n = cx.u("name");
        let r = Request::get(format!("{base}?$filter=receivername eq '{n}'"))
            .spo_json()
            .data()
            .await?;
        let v = need_arr(&r, "value", "length")?;
        if v.is_empty() {
            return Err(Error::msg(format!(
                "The specified event receiver '{n}' does not exist."
            )));
        }
        if v.len() > 1 {
            cx.handle_multiple_results(
                &format!("Multiple event receivers with name '{n}' found."),
                &util::array_to_hash_table("ReceiverId", v),
            )?
        } else {
            v[0].clone()
        }
    };
    cx.log(r)
}

pub async fn eventreceiver_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let url = if cx.str("scope").as_deref() == Some("site") {
        format!("{web}/_api/site/eventreceivers")
    } else {
        let lp = list_part(cx, &web);
        let lp = if lp.is_empty() {
            String::new()
        } else {
            format!("{}/", &lp[1..])
        };
        format!("{web}/_api/web/{lp}eventreceivers")
    };
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn eventreceiver_remove(cx: &mut Cx) -> Result<()> {
    let what = match cx.s("id") {
        Some(i) => format!("id {i}"),
        None => format!("name {}", cx.u("name")),
    };
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove event receiver with {what}?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    let mut url = format!(
        "{web}/_api/{}",
        cx.s("scope").unwrap_or_else(|| "web".into())
    );
    if let Some(i) = cx.s("listId") {
        url.push_str(&format!("/lists('{i}')"));
    } else {
        url.push_str(&list_part(cx, &web));
    }
    let id = match cx.s("id") {
        Some(i) => i,
        None => {
            let o = obj! {
                "webUrl": web.as_str(), "listId": ?cx.val("listId").cloned(), "listTitle": ?cx.val("listTitle").cloned(), "listUrl": ?cx.val("listUrl").cloned(),
                "scope": ?cx.val("scope").cloned(), "name": ?cx.val("name").cloned(), "debug": cx.debug(), "verbose": cx.verbose(),
            };
            let (out, _) = m365_core::runner::run_nested(
                "spo eventreceiver get",
                o.as_object().cloned().unwrap_or_default(),
                true,
            )
            .await?;
            u(&json::jparse(&out)?, "ReceiverId")
        }
    };
    Request::delete(format!("{url}/eventreceivers('{id}')"))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

// ---- feature -------------------------------------------------------------------

async fn feature(cx: &Cx, verb: &str, op: &str) -> Result<()> {
    let scope = cx.s("scope").unwrap_or_else(|| "Web".into());
    let force = cx.bool("force");
    let (web, id) = (cx.u("webUrl"), cx.u("id"));
    cx.verbose_log(&format!(
        "{verb} feature '{id}' on scope '{scope}' for url '{web}' (force='{force}')..."
    ));
    Request::post(format!(
        "{web}/_api/{scope}/features/{op}(featureId=guid'{id}',force={force})"
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn feature_disable(cx: &mut Cx) -> Result<()> {
    feature(cx, "Disabling", "remove").await
}

pub async fn feature_enable(cx: &mut Cx) -> Result<()> {
    feature(cx, "Enabling", "add").await
}

pub async fn feature_list(cx: &mut Cx) -> Result<()> {
    let scope = cx.s("scope").unwrap_or_else(|| "Web".into());
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{}/_api/{scope}/Features?$select=DisplayName,DefinitionId",
            cx.u("webUrl")
        ))
        .await?,
    ))
}

// ---- roledefinition ------------------------------------------------------------

/// `PermissionKind[name]`: the number for a name, the name for a numeric key.
enum Kind {
    Num(i32),
    Name,
}

fn kind(name: &str) -> Option<Kind> {
    if let Some((_, k)) = sp::PERMISSION_KINDS
        .iter()
        .find(|(n, _)| !n.is_empty() && *n == name)
    {
        return (*k != 0).then_some(Kind::Num(*k));
    }
    let n: i32 = name.parse().ok()?;
    (n.to_string() == name
        && sp::PERMISSION_KINDS
            .iter()
            .any(|(nm, k)| !nm.is_empty() && *k == n))
    .then_some(Kind::Name)
}

pub fn roledefinition_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let Some(r) = o.get("rights").filter(|v| truthy(v)).map(to_js_string) else {
        return Ok(true);
    };
    let parts: Vec<&str> = r.split(',').collect();
    match parts.iter().find(|p| kind(p.trim()).is_none()) {
        None => Ok(true),
        Some(bad) => {
            let names: Vec<&str> = sp::PERMISSION_KINDS
                .iter()
                .filter(|(n, _)| !n.is_empty())
                .map(|(n, _)| *n)
                .collect();
            Err(format!(
                "Rights option '{bad}' is not recognized as valid PermissionKind choice. Please note it is case-sensitive. Allowed values are {}.",
                names.join("|")
            ))
        }
    }
}

pub async fn roledefinition_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Adding role definition to {web}..."));
    let mut p = sp::BasePermissions::default();
    if let Some(r) = cx.s("rights") {
        for item in r.split(',') {
            match kind(item.trim()) {
                Some(Kind::Num(k)) => p.set(k),
                // `set('ViewListItems')`: NaN arithmetic ends in `high |= 1`
                Some(Kind::Name) | None => p.high = ((p.high as i32) | 1) as i64,
            }
        }
    }
    let data = obj! {
        "BasePermissions": obj! { "High": p.high.to_string(), "Low": p.low.to_string() },
        "Description": cx.s("description").unwrap_or_default(),
        "Name": cx.u("name"),
    };
    Request::post(format!("{web}/_api/web/roledefinitions"))
        .spo_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

pub async fn roledefinition_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Getting role definition from {web}..."));
    let mut r = Request::get(format!("{web}/_api/web/roledefinitions({})", cx.u("id")))
        .spo_json()
        .data()
        .await?;
    sp::friendly_role(&mut r)?;
    cx.log(r)
}

pub async fn roledefinition_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Getting role definitions list from {web}..."));
    let mut items = odata::get_all_items(&format!("{web}/_api/web/roledefinitions")).await?;
    sp::set_friendly_permissions(&mut items)?;
    cx.log(Value::Array(items))
}

pub async fn roledefinition_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the role definition with id {} from site {web}?",
            cx.u("id")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing role definition from site {web}..."));
    Request::delete(format!("{web}/_api/web/roledefinitions({})", cx.u("id")))
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json()
        .send()
        .await?;
    Ok(())
}

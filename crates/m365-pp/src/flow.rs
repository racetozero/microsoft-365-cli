//! `flow *`: Power Automate.

use crate::pa::{BAP, copy_display_name, env_props, safe_file_name};
use m365_core::prelude::*;
use m365_core::ut::entra;

const FLOW: &str = "https://api.flow.microsoft.com/providers/Microsoft.ProcessSimple";

fn admin(cx: &Cx) -> &'static str {
    if cx.bool("asAdmin") {
        "scopes/admin/"
    } else {
        ""
    }
}

fn flow_url(cx: &Cx, flow_key: &str) -> String {
    format!(
        "{FLOW}/{}environments/{}/flows/{}",
        admin(cx),
        eqp(&cx.u("environmentName")),
        eqp(&cx.u(flow_key))
    )
}

pub async fn disable(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Disables Microsoft Flow {}...", cx.u("name")));
    Request::post(format!(
        "{}/stop?api-version=2016-11-01",
        flow_url(cx, "name")
    ))
    .accept("application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn enable(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Enables Microsoft Flow {}...", cx.u("name")));
    Request::post(format!(
        "{}/start?api-version=2016-11-01",
        flow_url(cx, "name")
    ))
    .accept("application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn environment_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Flow environment {}...",
        cx.str("name").unwrap_or_else(|| "default".into())
    ));
    let env = if cx.bool("default") {
        "~default".to_string()
    } else {
        eqp(&cx.u("name"))
    };
    let r = Request::get(format!("{FLOW}/environments/{env}?api-version=2016-11-01"))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    cx.log(if cx.str("output").as_deref() != Some("json") {
        env_props(r)?
    } else {
        r
    })
}

pub async fn environment_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of Microsoft Flow environments...");
    let mut items =
        odata::get_all_items(&format!("{FLOW}/environments?api-version=2016-11-01")).await?;
    if cx.str("output").as_deref() != Some("json") {
        copy_display_name(&mut items)?;
    }
    cx.log(Value::Array(items))
}

pub async fn export(cx: &mut Cx) -> Result<()> {
    let format = cx
        .str("format")
        .map(|f| f.to_lowercase())
        .unwrap_or_default();
    let json_fmt = format == "json";
    let (name, env) = (cx.u("name"), cx.u("environmentName"));
    cx.verbose_log(&format!(
        "Retrieving package resources for Microsoft Flow {name}..."
    ));
    let bap = format!(
        "{BAP}/providers/Microsoft.BusinessAppPlatform/environments/{}",
        eqp(&env)
    );
    let mut res = Value::Null;
    if json_fmt {
        cx.verbose_log("format = json, skipping listing package resources step.");
    } else {
        res = Request::post(format!("{bap}/listPackageResources?api-version=2016-11-01"))
            .accept("application/json")
            .json_response()
            .json(obj! { "baseResourceIds": [format!("/providers/Microsoft.Flow/flows/{name}")] })
            .data()
            .await?;
        if let Some(e) = res
            .get("errors")
            .and_then(Value::as_array)
            .filter(|a| !a.is_empty())
        {
            return Err(Error::Thrown(
                e[0].get("message").cloned().unwrap_or(Value::Null),
            ));
        }
    }
    cx.verbose_log(&format!(
        "Initiating package export for Microsoft Flow {name}..."
    ));
    let flow = format!("{FLOW}/environments/{}/flows/{}", eqp(&env), eqp(&name));
    let res = if json_fmt {
        Request::get(format!("{flow}?api-version=2016-11-01"))
            .accept("application/json")
            .json_response()
            .data()
            .await?
    } else {
        let resources = res
            .get_mut("resources")
            .and_then(Value::as_object_mut)
            .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?;
        for v in resources.values_mut() {
            let kind = if s(v, "type") == "Microsoft.Flow/flows" {
                "Update"
            } else {
                "Existing"
            };
            v.as_object_mut()
                .map(|o| o.insert("suggestedCreationType".into(), Value::from(kind)));
        }
        let data = obj! {
            "includedResourceIds": [format!("/providers/Microsoft.Flow/flows/{name}")],
            "details": obj! {
                "displayName": ?cx.val("packageDisplayName").cloned(),
                "description": ?cx.val("packageDescription").cloned(),
                "creator": ?cx.val("packageCreatedBy").cloned(),
                "sourceEnvironment": ?cx.val("packageSourceEnvironment").cloned(),
            },
            "resources": Value::Object(resources.clone()),
        };
        Request::post(format!("{bap}/exportPackage?api-version=2016-11-01"))
            .accept("application/json")
            .json_response()
            .json(data)
            .data()
            .await?
    };
    cx.verbose_log(&format!("Getting file for Microsoft Flow {name}..."));
    let download = if json_fmt {
        String::new()
    } else {
        res.get("packageLink")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'value')"))?
            .get("value")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    };
    let file_name = if json_fmt {
        let dn = res
            .get("properties")
            .ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'displayName')")
            })?
            .get("displayName")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        format!("{dn}.json")
    } else {
        static ZIP: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new(r"(?i)[^/]+\.zip").unwrap());
        ZIP.find(&download)
            .map_or_else(|| "output.zip".into(), |m| m.as_str().to_string())
    };
    let file_name = safe_file_name(&file_name);
    cx.verbose_log(&format!("Filename from PowerApps API: {file_name}."));
    cx.verbose_log("");
    let file = if json_fmt {
        Request::post(format!("{flow}/exportToARMTemplate?api-version=2016-11-01"))
            .accept("application/json")
            .bytes_response()
            .send()
            .await?
    } else {
        Request::get(download)
            .anonymous()
            .bytes_response()
            .send()
            .await?
    };
    let path = cx.s("path").unwrap_or_else(|| format!("./{file_name}"));
    std::fs::write(&path, &file.bytes)
        .map_err(|e| Error::js(util::node_fs_error(&e, "open", &path)))?;
    if cx.s("path").is_none() || cx.verbose() {
        if cx.verbose() {
            cx.stderr(&format!("File saved to path '{path}'."));
        } else {
            cx.log(Value::from(path))?;
        }
    }
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information about Microsoft Flow {}...",
        cx.u("name")
    ));
    let mut r = Request::get(format!(
        "{}?api-version=2016-11-01&$expand=swagger,properties.connectionreferences.apidefinition,properties.definitionsummary.operations.apioperation,operationDefinition,plan,properties.throttleData,properties.estimatedsuspensiondata,properties.licenseData",
        flow_url(cx, "name")
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    let undef = |w: &str| {
        Error::js(format!(
            "Cannot read properties of undefined (reading '{w}')"
        ))
    };
    let props = r
        .get("properties")
        .cloned()
        .ok_or_else(|| undef("displayName"))?;
    let summary = props
        .get("definitionSummary")
        .cloned()
        .ok_or_else(|| undef("description"))?;
    let join = |k: &str, sub: &str| -> Result<String> {
        let list = summary
            .get(k)
            .and_then(Value::as_array)
            .ok_or_else(|| undef("map"))?;
        Ok(list
            .iter()
            .map(|t| {
                let extra = t
                    .get(sub)
                    .filter(|x| truthy(x))
                    .map(|x| format!("-{}", to_js_string(x)))
                    .unwrap_or_default();
                format!("{}{extra}", u(t, "type"))
            })
            .collect::<Vec<_>>()
            .join(", "))
    };
    let (triggers, actions) = (
        join("triggers", "kind")?,
        join("actions", "swaggerOperationId")?,
    );
    if let Some(o) = r.as_object_mut() {
        match props.get("displayName") {
            Some(d) => o.insert("displayName".into(), d.clone()),
            None => o.shift_remove("displayName"),
        };
        o.insert(
            "description".into(),
            summary
                .get("description")
                .filter(|d| truthy(d))
                .cloned()
                .unwrap_or(Value::from("")),
        );
        o.insert("triggers".into(), Value::from(triggers));
        o.insert("actions".into(), Value::from(actions));
    }
    cx.log(r)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let env = cx.u("environmentName");
    cx.verbose_log(&format!(
        "Getting Power Automate flows{} in environment '{env}'...",
        if cx.bool("asAdmin") { " as admin" } else { "" }
    ));
    let url = |filter: Option<&str>| {
        let seg = format!("/environments/{}", eqp(&env));
        let base = if cx.bool("asAdmin") {
            format!("/scopes/admin{seg}/v2")
        } else {
            seg
        };
        let f = filter
            .map(|f| format!("&$filter=search('{f}')"))
            .unwrap_or_default();
        let inc = if cx.bool("withSolutions") {
            "&include=includeSolutionCloudFlows"
        } else {
            ""
        };
        format!("{FLOW}{base}/flows?api-version=2016-11-01{f}{inc}")
    };
    let mut items = match cx.str("sharingStatus").as_deref() {
        Some("personal") => odata::get_all_items(&url(Some("personal"))).await?,
        Some("sharedWithMe") => odata::get_all_items(&url(Some("team"))).await?,
        Some("all") => {
            let mut a = odata::get_all_items(&url(Some("personal"))).await?;
            a.extend(odata::get_all_items(&url(Some("team"))).await?);
            a
        }
        _ => odata::get_all_items(&url(None)).await?,
    };
    let mut seen: Vec<Option<Value>> = Vec::new();
    items.retain(|f| {
        let id = f.get("id").cloned();
        if seen.contains(&id) {
            false
        } else {
            seen.push(id);
            true
        }
    });
    if cx.str("output").is_some_and(|o| o != "json") {
        copy_display_name(&mut items)?;
    }
    cx.log(Value::Array(items))
}

async fn principal(cx: &Cx) -> Result<String> {
    if let Some(u) = cx.s("userId") {
        return Ok(u);
    }
    if let Some(n) = cx.s("userName") {
        return entra::user::id_by_upn(&n).await;
    }
    if let Some(g) = cx.s("groupId") {
        return Ok(g);
    }
    entra::group::id_by_display_name(&cx.u("groupName")).await
}

pub async fn owner_ensure(cx: &mut Cx) -> Result<()> {
    let who = cx.first_u(&["userId", "userName", "groupId", "groupName"]);
    cx.verbose_log(&format!(
        "Assigning permissions for {who} with permissions {} to Power Automate flow {}",
        cx.u("roleName"),
        cx.u("flowName")
    ));
    let id = principal(cx).await?;
    let ty = if cx.s("userId").is_some() || cx.s("userName").is_some() {
        "User"
    } else {
        "Group"
    };
    let data = obj! { "put": [obj! { "properties": obj! { "principal": obj! { "id": id, "type": ty }, "roleName": ?cx.val("roleName").cloned() } }] };
    Request::post(format!(
        "{}/modifyPermissions?api-version=2016-11-01",
        flow_url(cx, "flowName")
    ))
    .accept("application/json")
    .json_response()
    .json(data)
    .send()
    .await?;
    Ok(())
}

pub async fn owner_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Listing owners for flow {} in environment {}",
        cx.u("flowName"),
        cx.u("environmentName")
    ));
    let items = odata::get_all_items(&format!(
        "{}/permissions?api-version=2016-11-01",
        flow_url(cx, "flowName")
    ))
    .await?;
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(items));
    }
    let mut out = Vec::new();
    for r in &items {
        let p = r
            .get("properties")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'roleName')"))?;
        let pr = p
            .get("principal")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'id')"))?;
        out.push(obj! { "roleName": ?p.get("roleName").cloned(), "id": ?pr.get("id").cloned(), "type": ?pr.get("type").cloned() });
    }
    cx.log(Value::Array(out))
}

pub async fn owner_remove(cx: &mut Cx) -> Result<()> {
    let who = cx.first_u(&["userId", "userName", "groupId", "groupName"]);
    cx.verbose_log(&format!(
        "Removing owner {who} from flow {} in environment {}",
        cx.u("flowName"),
        cx.u("environmentName")
    ));
    let what = cx.first_u(&["groupId", "groupName", "userId", "userName"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove owner '{what}' from the specified flow?"),
    )? {
        return Ok(());
    }
    let id = principal(cx).await?;
    Request::post(format!(
        "{}/modifyPermissions?api-version=2016-11-01",
        flow_url(cx, "flowName")
    ))
    .accept("application/json")
    .json_response()
    .json(obj! { "delete": [obj! { "id": id }] })
    .send()
    .await?;
    Ok(())
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    let env = cx.u("environmentName");
    cx.verbose_log(&format!(
        "Getting list of soft-deleted flows in environment {env}..."
    ));
    let flows = odata::get_all_items(&format!("{FLOW}/scopes/admin/environments/{}/v2/flows?api-version=2016-11-01&include=softDeletedFlows", eqp(&env))).await?;
    let mut deleted = Vec::new();
    for f in flows {
        let p = f
            .get("properties")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'state')"))?;
        if s(p, "state") == "Deleted" {
            deleted.push(f);
        }
    }
    if cx.str("output").as_deref() == Some("text") {
        copy_display_name(&mut deleted)?;
    }
    cx.log(Value::Array(deleted))
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let (flow, env) = (cx.u("flowName"), cx.u("environmentName"));
    cx.verbose_log(&format!(
        "Restoring soft-deleted flow {flow} from environment {env}..."
    ));
    Request::post(format!(
        "{FLOW}/scopes/admin/environments/{}/flows/{flow}/restore?api-version=2016-11-01",
        eqp(&env)
    ))
    .accept("application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let name = cx.u("name");
    cx.verbose_log(&format!("Removing Microsoft Flow {name}..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the Microsoft Flow {name}?"),
    )? {
        return Ok(());
    }
    // upstream checks `rawRes.statusCode === 204`, which an axios response
    // never has, so a 204 is a success
    Request::delete(format!("{}?api-version=2016-11-01", flow_url(cx, "name")))
        .accept("application/json")
        .json_response()
        .send()
        .await?;
    Ok(())
}

fn run_url(cx: &Cx) -> String {
    format!(
        "{FLOW}/environments/{}/flows/{}/runs/{}",
        eqp(&cx.u("environmentName")),
        eqp(&cx.u("flowName")),
        eqp(&cx.u("name"))
    )
}

pub async fn run_cancel(cx: &mut Cx) -> Result<()> {
    if cx.verbose() {
        cx.log(Value::from(format!(
            "Cancelling run {} of Microsoft Flow {}...",
            cx.u("name"),
            cx.u("flowName")
        )))?;
    }
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to cancel the flow run {}?",
            cx.u("name")
        ),
    )? {
        return Ok(());
    }
    Request::post(format!("{}/cancel?api-version=2016-11-01", run_url(cx)))
        .accept("application/json")
        .json_response()
        .send()
        .await?;
    Ok(())
}

async fn additional(uri: String) -> Result<Value> {
    let r = Request::get(uri)
        .accept("application/json")
        .anonymous()
        .json_response()
        .data()
        .await?;
    Ok(match r.get("body").filter(|b| truthy(b)) {
        Some(b) => b.clone(),
        None => r,
    })
}

pub async fn run_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving information about run {} of Microsoft Flow {}...",
        cx.u("name"),
        cx.u("flowName")
    ));
    let with_actions = cx.val("withActions").cloned().filter(truthy);
    let p = if with_actions.is_some() {
        "$expand=properties%2Factions&"
    } else {
        ""
    };
    let mut r = Request::get(format!("{}?{p}api-version=2016-11-01", run_url(cx)))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    let undef = |w: &str| {
        Error::js(format!(
            "Cannot read properties of undefined (reading '{w}')"
        ))
    };
    let props = r
        .get("properties")
        .cloned()
        .ok_or_else(|| undef("startTime"))?;
    let trigger = props.get("trigger").cloned().ok_or_else(|| undef("name"))?;
    // `res.x = undefined` leaves the key out of the output
    let mut extra: Vec<(&str, Option<Value>)> = vec![
        ("startTime", props.get("startTime").cloned()),
        (
            "endTime",
            Some(
                props
                    .get("endTime")
                    .filter(|e| truthy(e))
                    .cloned()
                    .unwrap_or(Value::from("")),
            ),
        ),
        ("status", props.get("status").cloned()),
        ("triggerName", trigger.get("name").cloned()),
    ];
    if cx.bool("withTrigger")
        && let Some(link) = trigger.get("outputsLink").filter(|l| truthy(l))
    {
        extra.push((
            "triggerInformation",
            Some(additional(u(link, "uri")).await?),
        ));
    }
    if let Some(wa) = &with_actions {
        let chosen: Option<Vec<String>> = wa
            .as_str()
            .map(|s| s.split(',').map(String::from).collect());
        let mut res = Obj::new();
        if let Some(actions) = props.get("actions").and_then(Value::as_object) {
            for (k, a) in actions {
                if !truthy(a) || chosen.as_ref().is_some_and(|c| !c.contains(k)) {
                    continue;
                }
                let mut a = a.clone();
                if let Some(uri) = a
                    .get("inputsLink")
                    .and_then(|l| l.get("uri"))
                    .filter(|u| truthy(u))
                    .map(to_js_string)
                {
                    let v = additional(uri).await?;
                    a.as_object_mut().map(|o| o.insert("input".into(), v));
                }
                if let Some(uri) = a
                    .get("outputsLink")
                    .and_then(|l| l.get("uri"))
                    .filter(|u| truthy(u))
                    .map(to_js_string)
                {
                    let v = additional(uri).await?;
                    a.as_object_mut().map(|o| o.insert("output".into(), v));
                }
                res.insert(k.clone(), a);
            }
        }
        extra.push(("actions", Some(Value::Object(res))));
    }
    if let Some(o) = r.as_object_mut() {
        for (k, v) in extra {
            m365_core::json::assign(o, k, v);
        }
    }
    cx.log(r)
}

pub async fn run_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving list of runs for Microsoft Flow {}...",
        cx.u("flowName")
    ));
    let mut url = format!("{}/runs?api-version=2016-11-01", flow_url(cx, "flowName"));
    let mut filters = Vec::new();
    if let Some(s) = cx.s("status") {
        filters.push(format!("status eq '{s}'"));
    }
    if let Some(s) = cx.s("triggerStartTime") {
        filters.push(format!("startTime ge {s}"));
    }
    if let Some(e) = cx.s("triggerEndTime") {
        filters.push(format!("startTime lt {e}"));
    }
    if !filters.is_empty() {
        url.push_str(&format!("&$filter={}", filters.join(" and ")));
    }
    let mut items = odata::get_all_items(&url).await?;
    if cx.bool("withTrigger") {
        for i in items.iter_mut() {
            let uri = i
                .get("properties")
                .and_then(|p| p.get("trigger"))
                .and_then(|t| t.get("outputsLink"))
                .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'uri')"))?;
            let r = Request::get(u(uri, "uri"))
                .accept("application/json")
                .anonymous()
                .json_response()
                .data()
                .await?;
            if let (Some(o), Some(b)) = (i.as_object_mut(), r.get("body")) {
                o.insert("triggerInformation".into(), b.clone());
            }
        }
    }
    if cx.str("output").as_deref() != Some("json") {
        for i in items.iter_mut() {
            let p = i.get("properties").cloned().ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'startTime')")
            })?;
            if let Some(o) = i.as_object_mut() {
                for k in ["startTime", "status"] {
                    match p.get(k) {
                        Some(v) => o.insert(k.into(), v.clone()),
                        None => o.shift_remove(k),
                    };
                }
            }
        }
    }
    cx.log(Value::Array(items))
}

pub async fn run_resubmit(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Resubmitting run {} of Microsoft Flow {}...",
        cx.u("name"),
        cx.u("flowName")
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to resubmit the flow with run {}?",
            cx.u("name")
        ),
    )? {
        return Ok(());
    }
    let base = format!(
        "{FLOW}/environments/{}/flows/{}",
        eqp(&cx.u("environmentName")),
        eqp(&cx.u("flowName"))
    );
    let r = Request::get(format!("{base}/triggers?api-version=2016-11-01"))
        .accept("application/json")
        .json_response()
        .data()
        .await?;
    let trig = r
        .get("value")
        .and_then(|v| v.get(0))
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'name')"))?;
    let name = u(trig, "name");
    cx.debug_log(&format!("Retrieved trigger: {name}"));
    Request::post(format!(
        "{base}/triggers/{}/histories/{}/resubmit?api-version=2016-11-01",
        eqp(&name),
        eqp(&cx.u("name"))
    ))
    .accept("application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

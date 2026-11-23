//! `graph *`.

use m365_core::prelude::*;
use m365_core::ut::{entra, entra_app};

const VERSIONS: [&str; 2] = ["beta", "v1.0"];
const SERVICES: [&str; 27] = [
    "Applications",
    "Calendar",
    "Change notifications",
    "Cloud communications",
    "Compliance",
    "Cross-device experiences",
    "Customer booking",
    "Device and app management",
    "Education",
    "Files",
    "Financials",
    "Groups",
    "Identity and access",
    "Mail",
    "Notes",
    "Notifications",
    "People and workplace intelligence",
    "Personal contacts",
    "Reports",
    "Search",
    "Security",
    "Sites and lists",
    "Tasks and plans",
    "Teamwork",
    "To-do tasks",
    "Users",
    "Workbooks and charts",
];

pub fn changelog_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let (k, allowed): (&str, &[&str]) = if i == 0 {
        ("versions", &VERSIONS)
    } else {
        ("services", &SERVICES)
    };
    let Some(v) = o.get(k).filter(|v| truthy(v)).map(to_js_string) else {
        return Ok(true);
    };
    Ok(v.to_lowercase()
        .split(',')
        .all(|x| allowed.iter().any(|a| a.to_lowercase() == x)))
}

/// The text of each `<tag>` element (CDATA and entities decoded).
fn elements<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    let mut rest = xml;
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    while let Some(i) = rest.find(&open) {
        let after = &rest[i + open.len()..];
        // `<titleX` is another element
        if !after.starts_with(['>', ' ', '/', '\t', '\n', '\r']) {
            rest = after;
            continue;
        }
        let Some(gt) = after.find('>') else { break };
        if after[..gt].ends_with('/') {
            out.push("");
            rest = &after[gt + 1..];
            continue;
        }
        let body = &after[gt + 1..];
        let Some(end) = body.find(&close) else { break };
        out.push(&body[..end]);
        rest = &body[end + close.len()..];
    }
    out
}

fn text(raw: &str) -> String {
    let t = raw.trim_start();
    if let Some(c) = t.strip_prefix("<![CDATA[")
        && let Some(e) = c.find("]]>")
    {
        return c[..e].to_string();
    }
    raw.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

pub async fn changelog_list(cx: &mut Cx) -> Result<()> {
    let q = cx
        .s("changeType")
        .map(|c| format!("/?filterBy={c}"))
        .unwrap_or_default();
    let res = Request::get(format!(
        "https://developer.microsoft.com/en-us/graph/changelog/rss{q}"
    ))
    .accept("text/xml")
    .anonymous()
    .data()
    .await?;
    let xml = to_js_string(&res);
    if !xml.contains('<') {
        // what @xmldom/xmldom reports for a document without elements
        cx.stderr("[xmldom fatalError]\tmissing root element \n@#[line:0,col:undefined]");
        return Err(Error::js("missing root element"));
    }
    if !xml.contains("<channel") {
        return Err(Error::js(
            "Cannot read properties of null (reading 'getElementsByTagName')",
        ));
    }
    let trim = cx.str("output").as_deref() == Some("text");
    let mut items: Vec<(String, String, String, String, f64)> = Vec::new();
    for item in elements(&xml, "item") {
        let first = |t: &str| {
            elements(item, t)
                .first()
                .map(|x| text(x))
                .unwrap_or_default()
        };
        let mut description = first("description");
        if trim {
            description = m365_core::help::md2plain(&description);
            if json::js_len(&description) > 50 {
                let head = String::from_utf16_lossy(
                    &description.encode_utf16().take(47).collect::<Vec<u16>>(),
                );
                description = format!("{head}...");
            }
        }
        let category = elements(item, "category")
            .get(1)
            .map(|x| text(x))
            .unwrap_or_default();
        let pub_date = m365_core::time::parse_js_date(&first("pubDate"))
            .map(|d| d.timestamp_millis() as f64)
            .unwrap_or(f64::NAN);
        items.push((
            first("guid"),
            category,
            first("title"),
            description,
            pub_date,
        ));
    }
    if let Some(s) = cx.s("services") {
        let wanted: Vec<String> = s.to_lowercase().split(',').map(String::from).collect();
        let matched: Vec<&str> = SERVICES
            .iter()
            .copied()
            .filter(|a| wanted.contains(&a.to_lowercase()))
            .collect();
        items.retain(|i| matched.contains(&i.2.as_str()));
    }
    if let Some(v) = cx.s("versions") {
        let wanted: Vec<String> = v.to_lowercase().split(',').map(String::from).collect();
        let matched: Vec<&str> = VERSIONS
            .iter()
            .copied()
            .filter(|a| wanted.contains(&a.to_lowercase()))
            .collect();
        items.retain(|i| matched.contains(&i.1.as_str()));
    }
    let date = |k: &str| {
        cx.s(k)
            .and_then(|d| m365_core::time::parse_js_date(&d))
            .map(|d| d.timestamp_millis() as f64)
            .unwrap_or(f64::NAN)
    };
    if cx.s("startDate").is_some() {
        let s = date("startDate");
        items.retain(|i| i.4 >= s);
    }
    if cx.s("endDate").is_some() {
        let e = date("endDate");
        items.retain(|i| i.4 <= e);
    }
    // unique by guid: a later item replaces the earlier one in its position
    let mut unique: Vec<(String, String, String, String, f64)> = Vec::new();
    for i in items {
        match unique.iter_mut().find(|u| u.0 == i.0) {
            Some(u) => *u = i,
            None => unique.push(i),
        }
    }
    unique.sort_by(|a, b| {
        let d = b.4 - a.4;
        if d.is_nan() || d == 0.0 {
            std::cmp::Ordering::Equal
        } else if d < 0.0 {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let out: Vec<Value> = unique
        .into_iter()
        .map(|(guid, category, title, description, d)| {
            let pub_date = if d.is_nan() {
                Value::Null
            } else {
                Value::from(chrono::DateTime::from_timestamp_millis(d as i64).map(m365_core::time::to_iso).unwrap_or_default())
            };
            obj! { "guid": guid, "category": category, "title": title, "description": description, "pubDate": pub_date }
        })
        .collect();
    cx.log(Value::Array(out))
}

// ---- directoryextension --------------------------------------------------------

async fn app_object_id(cx: &Cx) -> Result<String> {
    if let Some(o) = cx.s("appObjectId") {
        return Ok(o);
    }
    if let Some(a) = cx.s("appId") {
        return Ok(u(&entra_app::get_by_app_id(&a, Some(&["id"])).await?, "id"));
    }
    Ok(u(
        &entra_app::get_by_app_name(&cx.u("appName"), Some(&["id"])).await?,
        "id",
    ))
}

pub async fn directoryextension_add(cx: &mut Cx) -> Result<()> {
    let oid = app_object_id(cx).await?;
    cx.verbose_log(&format!(
        "Adding directory extension to the app with id '{oid}'..."
    ));
    let r = Request::post(format!("{GRAPH}/v1.0/applications/{oid}/extensionProperties"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! {
            "name": ?cx.val("name").cloned(),
            "dataType": ?cx.val("dataType").cloned(),
            "targetObjects": ?cx.str("targetObjects").map(|t| t.split(',').map(String::from).collect::<Vec<_>>()),
            "isMultiValued": ?cx.val("isMultiValued").cloned(),
        })
        .data()
        .await?;
    cx.log(r)
}

async fn extension_id(cx: &Cx, oid: &str) -> Result<String> {
    match cx.s("name") {
        Some(n) => Ok(u(
            &entra::directory_extension_by_name(&n, oid, Some(&["id"])).await?,
            "id",
        )),
        None => Ok(cx.u("id")),
    }
}

pub async fn directoryextension_get(cx: &mut Cx) -> Result<()> {
    let oid = app_object_id(cx).await?;
    let id = extension_id(cx, &oid).await?;
    cx.verbose_log(&format!(
        "Retrieving schema extension with ID {id} from application with ID {oid}..."
    ));
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/applications/{oid}/extensionProperties/{id}"
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn directoryextension_list(cx: &mut Cx) -> Result<()> {
    if cx.s("appId").is_some() || cx.s("appObjectId").is_some() || cx.s("appName").is_some() {
        let oid = app_object_id(cx).await?;
        return cx.log(Value::Array(
            odata::get_all_items(&format!(
                "{GRAPH}/v1.0/applications/{oid}/extensionProperties/"
            ))
            .await?,
        ));
    }
    let r = Request::post(format!(
        "{GRAPH}/v1.0/directoryObjects/getAvailableExtensionProperties"
    ))
    .header("content-type", "application/json;odata.metadata=none")
    .json_response()
    .data()
    .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn directoryextension_remove(cx: &mut Cx) -> Result<()> {
    let what = cx
        .s("id")
        .or_else(|| cx.s("name"))
        .unwrap_or_else(|| "undefined".into());
    let app = cx
        .s("appObjectId")
        .or_else(|| cx.s("appId"))
        .or_else(|| cx.s("appName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove directory extension '{what}' from application '{app}'?"
        ),
    )? {
        return Ok(());
    }
    let oid = app_object_id(cx).await?;
    let id = extension_id(cx, &oid).await?;
    cx.verbose_log(&format!(
        "Removing schema extension with ID {id} from application with ID {oid}..."
    ));
    Request::delete(format!(
        "{GRAPH}/v1.0/applications/{oid}/extensionProperties/{id}"
    ))
    .accept("application/json;odata.metadata=none")
    .send()
    .await?;
    Ok(())
}

// ---- openextension -------------------------------------------------------------

fn resource_url(cx: &Cx) -> String {
    let t = cx.u("resourceType");
    let s = if t == "organization" { "" } else { "s" };
    format!("{GRAPH}/v1.0/{t}{s}/{}/extensions", cx.u("resourceId"))
}

pub async fn openextension_add(cx: &mut Cx) -> Result<()> {
    let mut body = Obj::new();
    body.insert(
        "extensionName".into(),
        cx.val("name").cloned().unwrap_or(Value::Null),
    );
    for (k, v) in m365_core::ut::options::unknown_options(&cx.opts, cx.meta) {
        let parsed = json::parse(&to_js_string(&v))
            .ok()
            .filter(|_| v.is_string());
        body.insert(k, parsed.unwrap_or(v));
    }
    cx.verbose_log(&format!(
        "Adding open extension to the {} with id '{}'...",
        cx.u("resourceType"),
        cx.u("resourceId")
    ));
    let r = Request::post(resource_url(cx))
        .graph_json()
        .header("content-type", "application/json")
        .json(Value::Object(body))
        .data()
        .await?;
    cx.log(r)
}

pub async fn openextension_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving open extension for resource {}...",
        cx.u("resourceId")
    ));
    cx.log(
        Request::get(format!("{}/{}", resource_url(cx), cx.u("name")))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn openextension_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving open extensions for the {} with id '{}'...",
        cx.u("resourceType"),
        cx.u("resourceId")
    ));
    let items = odata::get_all_items_req(
        Request::get(resource_url(cx))
            .graph_json()
            .header("content-type", "application/json"),
    )
    .await?;
    cx.log(Value::Array(items))
}

pub async fn openextension_remove(cx: &mut Cx) -> Result<()> {
    let msg = format!(
        "Are you sure you want to remove open extension '{}' from resource '{}' of type '{}'?",
        cx.u("name"),
        cx.u("resourceId"),
        cx.u("resourceType")
    );
    if !cx.proceed("force", &msg)? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing open extension for resource {}...",
        cx.u("resourceId")
    ));
    Request::delete(format!("{}/{}", resource_url(cx), cx.u("name")))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn openextension_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving open extension for resource {}...",
        cx.u("resourceId")
    ));
    let url = format!("{}/{}", resource_url(cx), cx.u("name"));
    let current = Request::get(url.clone()).graph_json().data().await?;
    let mut body = Obj::new();
    body.insert(
        "@odata.type".into(),
        Value::from("#microsoft.graph.openTypeExtension"),
    );
    let unknown = m365_core::ut::options::unknown_options(&cx.opts, cx.meta);
    for (k, v) in &unknown {
        let val = if v == "" {
            Value::Null
        } else {
            json::parse(&to_js_string(v))
                .ok()
                .filter(|_| v.is_string())
                .unwrap_or_else(|| v.clone())
        };
        body.insert(k.clone(), val);
    }
    let keep = cx.bool("keepUnchangedProperties");
    if let Some(o) = current.as_object() {
        for (k, v) in o {
            if !unknown.contains_key(k) && (keep || k == "id" || k == "extensionName") {
                body.insert(k.clone(), v.clone());
            }
        }
    }
    cx.verbose_log(&format!(
        "Updating open extension of the {} with id '{}'...",
        cx.u("resourceType"),
        cx.u("resourceId")
    ));
    Request::patch(url)
        .graph_json()
        .header("content-type", "application/json")
        .json(Value::Object(body))
        .send()
        .await?;
    Ok(())
}

// ---- schemaextension -----------------------------------------------------------

const PROPERTY_TYPES: [&str; 5] = ["Binary", "Boolean", "DateTime", "Integer", "String"];

fn type_msg(p: &Value) -> String {
    format!(
        "{} is not a valid property type. Valid types are: Binary, Boolean, DateTime, Integer and String",
        p.get("type")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    )
}

fn valid_type(p: &Value) -> bool {
    p.get("type")
        .and_then(Value::as_str)
        .is_some_and(|t| PROPERTY_TYPES.contains(&t))
}

/// The items of a parsed `properties` value (`properties[i]`), or `None`
/// when it has no `length`.
fn indexed(v: &Value) -> Option<Vec<Value>> {
    match v {
        Value::Array(a) => Some(a.clone()),
        Value::String(s) => Some(s.chars().map(|c| Value::from(c.to_string())).collect()),
        _ => None,
    }
}

/// `validateProperties` of `schemaextension add`: later problems overwrite earlier ones.
fn validate_add(s: &str) -> std::result::Result<(), String> {
    let p = json::js_parse(s)?;
    if p.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'length')".into());
    }
    let Some(items) = indexed(&p) else {
        return Err("The specified JSON string is not an array".into());
    };
    let mut result: Option<String> = None;
    for prop in &items {
        if !prop.get("name").is_some_and(truthy) {
            result = Some(format!("Property {} misses name", stringify(prop)));
        }
        if !valid_type(prop) {
            result = Some(type_msg(prop));
        }
    }
    result.map_or(Ok(()), Err)
}

/// `validateProperties` of `schemaextension set`: the first problem wins.
fn validate_set(s: &str) -> std::result::Result<(), String> {
    let p = json::js_parse(s)
        .map_err(|_| "The specified properties is not a valid JSON string".to_string())?;
    if p.is_null() {
        return Err("TypeError: Cannot read properties of null (reading 'length')".into());
    }
    let Some(items) = indexed(&p) else {
        return Err("The specified properties JSON string is not an array".into());
    };
    for prop in &items {
        if !prop.get("name").is_some_and(truthy) {
            return Err(format!("Property {} misses name", stringify(prop)));
        }
        if !valid_type(prop) {
            return Err(type_msg(prop));
        }
    }
    Ok(())
}

pub fn schemaextension_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    validate_add(
        &o.get("properties")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )
    .map(|_| true)
}

pub fn schemaextension_set_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    match o.get("properties").filter(|v| truthy(v)) {
        None => Ok(true),
        Some(p) => validate_set(&to_js_string(p)).map(|_| true),
    }
}

pub async fn schemaextension_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Adding schema extension with id '{}'...",
        cx.u("id")
    ));
    let types: Vec<String> = cx
        .u("targetTypes")
        .split(',')
        .map(|t| t.trim().to_string())
        .collect();
    // parsed outside the command's try/catch
    let props = json::js_parse(&cx.u("properties")).map_err(Error::msg)?;
    let data = obj! { "id": ?cx.val("id").cloned(), "description": ?cx.val("description").cloned(), "owner": ?cx.val("owner").cloned(), "targetTypes": types, "properties": props };
    let r = Request::post(format!("{GRAPH}/v1.0/schemaExtensions"))
        .graph_json()
        .header("content-type", "application/json")
        .json(data)
        .data()
        .await?;
    cx.log(r)
}

pub async fn schemaextension_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!(
        "Gets the properties of the specified schema extension definition with id '{id}'..."
    ));
    cx.log(
        Request::get(format!("{GRAPH}/v1.0/schemaExtensions/{id}"))
            .graph_json()
            .header("content-type", "application/json")
            .data()
            .await?,
    )
}

pub async fn schemaextension_list(cx: &mut Cx) -> Result<()> {
    let mut filters = Vec::new();
    for k in ["status", "owner"] {
        if let Some(v) = cx.str(k) {
            filters.push(format!("{k} eq '{}'", v.replace('\'', "''")));
        }
    }
    let filter = if filters.is_empty() {
        String::new()
    } else {
        format!("$filter={}", filters.join(" and "))
    };
    let mut url = format!(
        "{GRAPH}/v1.0/schemaExtensions?$select=*{}",
        if filter.is_empty() {
            String::new()
        } else {
            format!("&{filter}")
        }
    );
    if let Some(n) = cx.s("pageNumber")
        && json::js_number(&n) > 0.0
    {
        let size = json::js_number(&cx.s("pageSize").unwrap_or_else(|| "10".into()));
        // `Number(pageNumber + 1)` concatenates the string
        let n1 = json::js_number(&format!("{n}1"));
        url.push_str(&format!("&$top={}", json::number_to_string(size * n1)));
    }
    let r = Request::get(url)
        .graph_json()
        .header("content-type", "application/json")
        .data()
        .await?;
    let all = arr(&r, "value");
    // `res.value.slice(-size)`; a NaN size slices from 0
    let size = match cx.s("pageSize") {
        Some(p) => json::parse_int(&p).unwrap_or(0),
        None => all.len() as i64,
    };
    let start = match size {
        s if s > 0 => all.len().saturating_sub(s as usize),
        s => ((-s) as usize).min(all.len()),
    };
    let result: Vec<Value> = all[start..].to_vec();
    if cx.str("output").as_deref() != Some("json") && result.len() > 1 {
        let out: Vec<Value> = result
            .iter()
            .map(|x| {
                let g = |k: &str| x.get(k).cloned();
                obj! { "id": ?g("id"), "description": ?g("description"), "targetTypes": ?g("targetTypes"), "status": ?g("status"), "owner": ?g("owner"), "properties": ?x.get("properties").map(stringify) }
            })
            .collect();
        return cx.log(Value::Array(out));
    }
    cx.log(Value::Array(result))
}

pub async fn schemaextension_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the schema extension with ID {id}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removes specified Microsoft Graph schema extension with id '{id}'..."
    ));
    Request::delete(format!("{GRAPH}/v1.0/schemaExtensions/{id}"))
        .graph_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn schemaextension_set(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Updating schema extension with id '{id}'..."));
    let mut data = Obj::new();
    data.insert(
        "owner".into(),
        cx.val("owner").cloned().unwrap_or(Value::Null),
    );
    if let Some(d) = cx.s("description") {
        cx.debug_log(&format!("Will update description to '{d}'..."));
        data.insert("description".into(), Value::from(d));
    }
    if let Some(s) = cx.s("status") {
        cx.debug_log(&format!("Will update status to '{s}'..."));
        data.insert("status".into(), Value::from(s));
    }
    if let Some(t) = cx.s("targetTypes") {
        cx.debug_log(&format!("Will update targetTypes to '{t}'..."));
        data.insert(
            "targetTypes".into(),
            Value::from(
                t.split(',')
                    .map(|x| x.trim().to_string())
                    .collect::<Vec<_>>(),
            ),
        );
    }
    if let Some(p) = cx.s("properties") {
        let props = json::js_parse(&p).map_err(Error::msg)?;
        if truthy(&props) {
            cx.debug_log(&format!("Will update properties to '{p}'..."));
            data.insert("properties".into(), props);
        }
    }
    Request::patch(format!("{GRAPH}/v1.0/schemaExtensions/{id}"))
        .graph_json()
        .header("content-type", "application/json")
        .json(Value::Object(data))
        .send()
        .await?;
    cx.debug_log("Schema extension successfully updated.");
    Ok(())
}

// ---- subscription --------------------------------------------------------------

pub fn subscription_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let c = o.get("changeTypes").map(to_js_string).unwrap_or_default();
    Ok(c.split(',')
        .all(|x| ["created", "updated", "deleted"].contains(&x.trim())))
}

pub async fn subscription_add(cx: &mut Cx) -> Result<()> {
    let expiration = match cx.s("expirationDateTime") {
        Some(e) => {
            cx.debug_log(&format!("Expiration date time is specified ({e})."));
            e
        }
        None => {
            cx.debug_log(
                "Expiration date time is not specified. Will try to get appropriate maximum value",
            );
            let resource = cx.u("resource");
            let per: [(&str, i64); 8] = [
                ("users", 4230),
                ("groups", 4230),
                ("/messages", 4230),
                ("/events", 4230),
                ("/contacts", 4230),
                ("/conversations", 4230),
                ("/drive/root", 4230),
                ("security/alerts", 43200),
            ];
            let from_now =
                |m: i64| m365_core::time::to_iso(chrono::Utc::now() + chrono::Duration::minutes(m));
            match per.iter().find(|(r, _)| resource.contains(r)) {
                Some((r, d)) => {
                    let iso = from_now(d - 1);
                    cx.debug_log(&format!(
                        "Matching resource in default values '{resource}' => '{r}'"
                    ));
                    cx.debug_log(&format!("Resolved expiration delay: {d} (safe delta: 1)"));
                    cx.debug_log(&format!("Actual expiration date time: {iso}"));
                    cx.verbose_log(&format!("An expiration maximum delay is resolved for the resource '{resource}' : {d} minutes."));
                    iso
                }
                None => {
                    cx.verbose_log(&format!("An expiration maximum delay couldn't be resolved for the resource '{resource}'. Will use generic default value: 4230 minutes."));
                    let iso = from_now(4229);
                    cx.debug_log(&format!("Actual expiration date time: {iso}"));
                    iso
                }
            }
        }
    };
    let g = |k: &str| cx.val(k).cloned();
    let data = obj! {
        "changeType": ?g("changeTypes"),
        "resource": ?g("resource"),
        "notificationUrl": ?g("notificationUrl"),
        "expirationDateTime": expiration,
        "clientState": ?g("clientState"),
        "includeResourceData": ?g("withResourceData"),
        "encryptionCertificate": ?g("encryptionCertificate"),
        "encryptionCertificateId": ?g("encryptionCertificateId"),
        "lifecycleNotificationUrl": ?g("lifecycleNotificationUrl"),
        "notificationUrlAppId": ?g("notificationUrlAppId"),
        "latestSupportedTlsVersion": ?g("latestTLSVersion"),
    };
    let r = Request::post(format!("{GRAPH}/v1.0/subscriptions"))
        .graph_json()
        .header("content-type", "application/json")
        .json(data)
        .data()
        .await?;
    cx.log(r)
}

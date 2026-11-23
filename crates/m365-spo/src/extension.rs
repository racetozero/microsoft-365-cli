//! `spo applicationcustomizer *` and `spo commandset *`.

use crate::customaction::{custom_action_by_id, custom_actions};
use crate::sp::{first, ne};
use m365_core::prelude::*;

const APP: &str = "ClientSideExtension.ApplicationCustomizer";
const CMD: &str = "ClientSideExtension.ListViewCommandSet";

/// `a && x || b && y || c && z` over the identifier options.
fn identifier(cx: &Cx) -> String {
    if let Some(t) = ne(cx, "title") {
        return format!("title '{t}'");
    }
    if let Some(c) = ne(cx, "clientSideComponentId") {
        return format!("ClientSideComponentId '{c}'");
    }
    match cx.val("id") {
        Some(v) if truthy(v) => format!("id '{}'", to_js_string(v)),
        Some(v) => to_js_string(v),
        None => "undefined".into(),
    }
}

fn humanize(v: Option<&Value>) -> Value {
    match v.and_then(Value::as_f64) {
        Some(2.0) => Value::from("Site"),
        Some(3.0) => Value::from("Web"),
        _ => Value::from(v.map(to_js_string).unwrap_or_else(|| "undefined".into())),
    }
}

fn try_parse_json(v: Option<&Value>) -> Option<Value> {
    match v {
        Some(Value::String(s)) => {
            Some(json::js_parse(s).unwrap_or_else(|_| Value::from(s.as_str())))
        }
        other => other.cloned(),
    }
}

/// Resolves one custom action by id/title/clientSideComponentId at `location`.
async fn resolve(
    cx: &Cx,
    scope: Option<&str>,
    location: &str,
    what: &str,
    check_location: bool,
) -> Result<Value> {
    let web = cx.u("webUrl");
    let mut found = Vec::new();
    if let Some(id) = ne(cx, "id") {
        if let Some(a) = custom_action_by_id(&web, &id, scope).await?
            && (!check_location || a.get("Location").and_then(Value::as_str) == Some(location))
        {
            found.push(a);
        }
    } else if let Some(t) = ne(cx, "title") {
        found = custom_actions(
            &web,
            scope,
            Some(&format!(
                "(Title eq '{}') and (startswith(Location,'{location}'))",
                eqp(&t)
            )),
        )
        .await?;
    } else {
        found = custom_actions(
            &web,
            scope,
            Some(&format!(
                "(ClientSideComponentId eq guid'{}') and (startswith(Location,'{location}'))",
                cx.s("clientSideComponentId")
                    .unwrap_or_else(|| "undefined".into())
            )),
        )
        .await?;
    }
    match found.len() {
        0 => Err(Error::msg(format!(
            "No {what} with {} found",
            identifier(cx)
        ))),
        1 => Ok(found.remove(0)),
        _ => {
            let by = match ne(cx, "title") {
                Some(t) => format!("title '{t}'"),
                None => format!(
                    "ClientSideComponentId '{}'",
                    cx.s("clientSideComponentId")
                        .unwrap_or_else(|| "undefined".into())
                ),
            };
            let label = if what == "application customizer" {
                "application customizer"
            } else {
                "user commandsets"
            };
            cx.handle_multiple_results(
                &format!("Multiple {label} with {by} found."),
                &util::array_to_hash_table("Id", &found),
            )
        }
    }
}

fn scope_to_string(a: &Value) -> Result<String> {
    match a.get("Scope") {
        None => Err(Error::js(
            "Cannot read properties of undefined (reading 'toString')",
        )),
        Some(Value::Null) => Err(Error::js(
            "Cannot read properties of null (reading 'toString')",
        )),
        Some(v) => Ok(to_js_string(v)),
    }
}

pub async fn app_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Adding application customizer with title '{}' and clientSideComponentId '{}' to the site",
        cx.u("title"),
        cx.u("clientSideComponentId")
    ));
    let mut b = obj! {
        "Title": ?cx.val("title").cloned(), "Name": ?cx.val("title").cloned(), "Description": ?cx.val("description").cloned(),
        "Location": APP, "ClientSideComponentId": ?cx.val("clientSideComponentId").cloned(),
        "HostProperties": ne(cx, "hostProperties").unwrap_or_default(),
    };
    if let Some(p) = ne(cx, "clientSideComponentProperties") {
        b["ClientSideComponentProperties"] = Value::from(p);
    }
    let scope = ne(cx, "scope").unwrap_or_else(|| "Site".into());
    Request::post(format!("{}/_api/{scope}/UserCustomActions", cx.u("webUrl")))
        .spo_json()
        .json(b)
        .send()
        .await
        .map_err(Error::uncaught)?;
    Ok(())
}

pub async fn app_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let scope = cx.s("scope");
    let a = if let Some(id) = ne(cx, "id") {
        match custom_action_by_id(&web, &id, scope.as_deref()).await? {
            Some(a) if a.get("Location").and_then(Value::as_str) == Some(APP) => a,
            _ => {
                return Err(Error::msg(format!(
                    "No application customizer with id '{id}' found"
                )));
            }
        }
    } else {
        let filter = match ne(cx, "title") {
            Some(t) => format!("Title eq '{}'", eqp(&t)),
            None => format!(
                "ClientSideComponentId eq guid'{}'",
                eqp(&cx.u("clientSideComponentId"))
            ),
        };
        let mut l = custom_actions(
            &web,
            scope.as_deref(),
            Some(&format!("{filter} and Location eq '{APP}'")),
        )
        .await?;
        let ident = match ne(cx, "title") {
            Some(t) => format!("title '{t}'"),
            None => format!(
                "Client Side Component Id '{}'",
                cx.u("clientSideComponentId")
            ),
        };
        match l.len() {
            1 => l.remove(0),
            0 => {
                return Err(Error::msg(format!(
                    "No application customizer with {ident} found"
                )));
            }
            _ => cx.handle_multiple_results(
                &format!("Multiple application customizers with {ident} found."),
                &util::array_to_hash_table("Id", &l),
            )?,
        }
    };
    if !cx.bool("clientSideComponentProperties") {
        let mut a = a;
        let s = humanize(a.get("Scope"));
        if let Some(o) = a.as_object_mut() {
            o.insert("Scope".into(), s);
        }
        cx.log(a)
    } else {
        match try_parse_json(a.get("ClientSideComponentProperties")) {
            Some(v) => cx.log(v),
            None => cx.log_undefined(),
        }
    }
}

pub async fn app_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving application customizers...");
    let l = custom_actions(
        &cx.u("webUrl"),
        cx.s("scope").as_deref(),
        Some(&format!("Location eq '{APP}'")),
    )
    .await
    .map_err(Error::uncaught)?;
    cx.log(Value::Array(l))
}

pub async fn app_remove(cx: &mut Cx) -> Result<()> {
    let who = first(cx, &["clientSideComponentId", "title", "id"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the application customizer '{who}'?"),
    )? {
        return Ok(());
    }
    let scope = ne(cx, "scope").unwrap_or_else(|| "All".into());
    let a = resolve(cx, Some(&scope), APP, "application customizer", false).await?;
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Removing application customizer '{who}' from the site '{web}'..."
    ));
    let s = if scope_to_string(&a)? == "2" {
        "Site"
    } else {
        "Web"
    };
    Request::delete(format!(
        "{web}/_api/{s}/UserCustomActions('{}')",
        u(&a, "Id")
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn app_set(cx: &mut Cx) -> Result<()> {
    let scope = ne(cx, "scope").unwrap_or_else(|| "All".into());
    cx.verbose_log(&format!(
        "Getting application customizer {} to update...",
        first(cx, &["title", "clientSideComponentId", "id"])
    ));
    let a = resolve(cx, Some(&scope), APP, "application customizer", true).await?;
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Updating application customizer with ID '{}' on the site '{web}'...",
        u(&a, "Id")
    ));
    let mut b = obj! { "HostProperties": ?cx.val("hostProperties").cloned() };
    for (k, o) in [
        ("Title", "newTitle"),
        ("Description", "description"),
        (
            "ClientSideComponentProperties",
            "clientSideComponentProperties",
        ),
    ] {
        if let Some(v) = cx.val(o) {
            b[k] = v.clone();
        }
    }
    let s = if scope_to_string(&a)? == "2" {
        "Site"
    } else {
        "Web"
    };
    Request::post(format!(
        "{web}/_api/{s}/UserCustomActions('{}')",
        u(&a, "Id")
    ))
    .spo_json()
    .header("X-HTTP-Method", "MERGE")
    .json(b)
    .send()
    .await?;
    Ok(())
}

fn cmd_location(l: &str) -> &'static str {
    match l {
        "Both" => CMD,
        "ContextMenu" => "ClientSideExtension.ListViewCommandSet.ContextMenu",
        _ => "ClientSideExtension.ListViewCommandSet.CommandBar",
    }
}

fn list_template(l: &str) -> &'static str {
    match l {
        "SitePages" => "119",
        "Library" => "101",
        _ => "100",
    }
}

pub async fn cmd_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding ListView Command Set {} to site '{web}'...",
        cx.u("clientSideComponentId")
    ));
    let scope = ne(cx, "scope").unwrap_or_else(|| "Web".into());
    let mut b = obj! {
        "Title": ?cx.val("title").cloned(), "Description": ?cx.val("description").cloned(),
        "Location": ?ne(cx, "location").map(|l| cmd_location(&l)),
        "ClientSideComponentId": ?cx.val("clientSideComponentId").cloned(),
        "RegistrationId": list_template(&cx.u("listType")), "RegistrationType": 1,
    };
    if let Some(p) = ne(cx, "clientSideComponentProperties") {
        b["ClientSideComponentProperties"] = Value::from(p);
    }
    cx.log(
        Request::post(format!("{web}/_api/{scope}/UserCustomActions"))
            .spo_json()
            .json(b)
            .data()
            .await?,
    )
}

pub async fn cmd_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Attempt to get a specific Command Set by property {}.",
        first(cx, &["title", "id", "clientSideComponentId"])
    ));
    let web = cx.u("webUrl");
    let scope = cx.s("scope");
    let base = format!("startswith(Location,'{CMD}') and");
    let set = if let Some(id) = ne(cx, "id") {
        match custom_action_by_id(&web, &id, scope.as_deref()).await? {
            None => {
                return Err(Error::msg(format!(
                    "Command set with id {id} can't be found."
                )));
            }
            Some(a) => {
                let loc = a.get("Location").and_then(Value::as_str).unwrap_or("\u{0}");
                if ![
                    CMD,
                    "ClientSideExtension.ListViewCommandSet.CommandBar",
                    "ClientSideExtension.ListViewCommandSet.ContextMenu",
                ]
                .contains(&loc)
                {
                    return Err(Error::msg(format!(
                        "Custom action with id {id} is not a command set."
                    )));
                }
                a
            }
        }
    } else if let Some(c) = ne(cx, "clientSideComponentId") {
        let l = custom_actions(
            &web,
            scope.as_deref(),
            Some(&format!("{base} ClientSideComponentId eq guid'{c}'")),
        )
        .await?;
        match l.into_iter().next() {
            Some(a) => a,
            None => {
                return Err(Error::msg(format!(
                    "No command set with clientSideComponentId '{c}' found."
                )));
            }
        }
    } else {
        let t = cx.u("title");
        let mut l = custom_actions(
            &web,
            scope.as_deref(),
            Some(&format!("{base} Title eq '{}'", eqp(&t))),
        )
        .await?;
        match l.len() {
            1 => l.remove(0),
            0 => {
                return Err(Error::msg(format!(
                    "No command set with title '{t}' found."
                )));
            }
            _ => cx.handle_multiple_results(
                &format!("Multiple command sets with title '{t}' found."),
                &util::array_to_hash_table("Id", &l),
            )?,
        }
    };
    if !cx.bool("clientSideComponentProperties") {
        cx.log(set)
    } else {
        match try_parse_json(set.get("ClientSideComponentProperties")) {
            Some(v) => cx.log(v),
            None => cx.log_undefined(),
        }
    }
}

pub async fn cmd_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Attempt to get commandsets...");
    cx.log(Value::Array(
        custom_actions(
            &cx.u("webUrl"),
            cx.s("scope").as_deref(),
            Some(&format!("startswith(Location,'{CMD}')")),
        )
        .await?,
    ))
}

fn web_or_site(a: &Value) -> &'static str {
    if a.get("Scope").and_then(Value::as_f64) == Some(3.0) {
        "Web"
    } else {
        "Site"
    }
}

pub async fn cmd_remove(cx: &mut Cx) -> Result<()> {
    let who = first(cx, &["clientSideComponentId", "title", "id"]);
    cx.verbose_log(&format!(
        "Removing ListView Command Set {who} to site '{}'...",
        cx.u("webUrl")
    ));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove command set '{who}'?"),
    )? {
        return Ok(());
    }
    let scope = ne(cx, "scope").unwrap_or_else(|| "All".into());
    let a = resolve(cx, Some(&scope), CMD, "user commandsets", false).await?;
    Request::delete(format!(
        "{}/_api/{}/UserCustomActions('{}')",
        cx.u("webUrl"),
        web_or_site(&a),
        eqp(&u(&a, "Id"))
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn cmd_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating ListView Command Set {} to site '{}'...",
        first(cx, &["id", "title", "clientSideComponentId"]),
        cx.u("webUrl")
    ));
    let scope = ne(cx, "scope").unwrap_or_else(|| "All".into());
    let mut b = Obj::new();
    if let Some(t) = ne(cx, "newTitle") {
        b.insert("Title".into(), Value::from(t));
    }
    if let Some(d) = cx.val("description") {
        b.insert("Description".into(), d.clone());
    }
    if let Some(l) = ne(cx, "location") {
        b.insert("Location".into(), Value::from(cmd_location(&l)));
    }
    if let Some(l) = ne(cx, "listType") {
        b.insert("RegistrationId".into(), Value::from(list_template(&l)));
    }
    if let Some(p) = ne(cx, "clientSideComponentProperties") {
        b.insert("ClientSideComponentProperties".into(), Value::from(p));
    }
    if let Some(p) = ne(cx, "newClientSideComponentId") {
        b.insert("ClientSideComponentId".into(), Value::from(p));
    }
    let a = resolve(cx, Some(&scope), CMD, "user commandsets", false).await?;
    Request::post(format!(
        "{}/_api/{}/UserCustomActions('{}')",
        cx.u("webUrl"),
        web_or_site(&a),
        eqp(&u(&a, "Id"))
    ))
    .spo_json()
    .header("X-HTTP-Method", "MERGE")
    .json(Value::Object(b))
    .send()
    .await?;
    Ok(())
}

pub fn guid_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(truthy(v) && m365_core::validation::is_valid_guid(&to_js_string(v)))
}

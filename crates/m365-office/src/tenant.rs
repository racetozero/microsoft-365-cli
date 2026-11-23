//! `tenant *`.

use m365_core::prelude::*;

const CUSTOM: [&str; 15] = [
    "customAttribute1",
    "customAttribute2",
    "customAttribute3",
    "customAttribute4",
    "customAttribute5",
    "customAttribute6",
    "customAttribute7",
    "customAttribute8",
    "customAttribute9",
    "customAttribute10",
    "customAttribute11",
    "customAttribute12",
    "customAttribute13",
    "customAttribute14",
    "customAttribute15",
];
const STANDARD: [&str; 6] = [
    "UserPrincipalName",
    "Fax",
    "StreetAddress",
    "PostalCode",
    "StateOrProvince",
    "Alias",
];
const GLOBAL: [&str; 4] = ["query", "output", "debug", "verbose"];

fn property_name(n: &str) -> Option<&'static str> {
    STANDARD
        .iter()
        .chain(CUSTOM.iter())
        .copied()
        .find(|p| p.eq_ignore_ascii_case(n))
}

fn user_domain() -> String {
    let n = auth::jwt::user_name(&auth::default_access_token().unwrap_or_default());
    n.split('@').nth(1).unwrap_or("undefined").to_string()
}

pub async fn id_get(cx: &mut Cx) -> Result<()> {
    let domain = cx.s("domainName").unwrap_or_else(user_domain);
    let r = Request::get(format!(
        "https://login.windows.net/{domain}/.well-known/openid-configuration"
    ))
    .header("content-type", "application/json")
    .accept("application/json")
    .anonymous()
    .json_response()
    .data()
    .await?;
    if r.get("error").is_some_and(truthy) {
        return Err(Error::Thrown(
            r.get("error_description").cloned().unwrap_or(Value::Null),
        ));
    }
    if let Some(t) = r.get("token_endpoint").filter(|t| truthy(t)) {
        let t = to_js_string(t);
        return match t.split('/').nth(3) {
            Some(id) => cx.log(Value::from(id)),
            None => cx.log_undefined(),
        };
    }
    Ok(())
}

pub fn info_get_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    match o.get("tenantId").filter(|v| truthy(v)).map(to_js_string) {
        Some(t) if !m365_core::validation::is_valid_guid(&t) => {
            Err(format!("{t} is not a valid GUID"))
        }
        _ => Ok(true),
    }
}

pub async fn info_get(cx: &mut Cx) -> Result<()> {
    let tid = cx.s("tenantId");
    let domain = match (cx.s("domainName"), &tid) {
        (Some(d), _) => Some(d),
        (None, None) => Some(user_domain()),
        _ => None,
    };
    let url = match tid {
        Some(t) => format!(
            "{GRAPH}/v1.0/tenantRelationships/findTenantInformationByTenantId(tenantId='{}')",
            eqp(&t)
        ),
        None => format!(
            "{GRAPH}/v1.0/tenantRelationships/findTenantInformationByDomainName(domainName='{}')",
            eqp(&domain.unwrap_or_default())
        ),
    };
    cx.log(Request::get(url).graph_json().data().await?)
}

fn unknown_keys(o: &Obj) -> Vec<String> {
    o.keys()
        .filter(|k| !GLOBAL.contains(&k.as_str()) && *k != "name" && *k != "displayName")
        .cloned()
        .collect()
}

pub fn profilecard_add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let custom = o
        .get("name")
        .map(to_js_string)
        .unwrap_or_default()
        .to_lowercase()
        .starts_with("customattribute");
    let unknown = unknown_keys(o);
    match i {
        2 if !custom && !unknown.is_empty() => Err(format!(
            "Unknown options like {} are only supported with customAttributes",
            unknown.join(", ")
        )),
        3 if custom => wrong_format(&unknown),
        _ => Ok(true),
    }
}

fn wrong_format(unknown: &[String]) -> std::result::Result<bool, String> {
    let wrong: Vec<&str> = unknown
        .iter()
        .filter(|k| !k.to_lowercase().starts_with("displayname-"))
        .map(String::as_str)
        .collect();
    if wrong.is_empty() {
        Ok(true)
    } else {
        Err(format!(
            "Wrong option format detected for the following option(s): {}'. When adding localizations for customAttributes, use the format displayName-<languageTag>.",
            wrong.join(", ")
        ))
    }
}

pub fn profilecard_set_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    wrong_format(&unknown_keys(o))
}

/// Flattens `annotations[0]` into `displayName` and `displayName <tag>` keys.
fn flatten(cx: &Cx, mut r: Value) -> Result<Value> {
    let out = cx.str("output");
    if out.is_none() || out.as_deref() == Some("json") {
        return Ok(r);
    }
    let anns = r
        .get("annotations")
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'length')"))?
        .clone();
    let Some(first) = anns.get(0).cloned() else {
        return Ok(r);
    };
    if let Some(o) = r.as_object_mut() {
        o.insert(
            "displayName".into(),
            first.get("displayName").cloned().unwrap_or(Value::Null),
        );
        for l in arr(&first, "localizations") {
            o.insert(
                format!("displayName {}", u(l, "languageTag")),
                l.get("displayName").cloned().unwrap_or(Value::Null),
            );
        }
        o.shift_remove("annotations");
    }
    Ok(r)
}

pub async fn profilecard_add(cx: &mut Cx) -> Result<()> {
    let name = property_name(&cx.u("name")).map(String::from);
    cx.verbose_log(&format!(
        "Adding '{}' as a profile card property...",
        name.clone().unwrap_or_else(|| "undefined".into())
    ));
    let annotations = match cx.s("displayName") {
        None => Vec::new(),
        Some(d) => {
            let unknown = m365_core::ut::options::unknown_options(&cx.opts, cx.meta);
            let locs: Vec<Value> = unknown.iter().map(|(k, v)| obj! { "languageTag": k.replacen("displayName-", "", 1), "displayName": v.clone() }).collect();
            vec![obj! { "displayName": d, "localizations": locs }]
        }
    };
    let r = Request::post(format!("{GRAPH}/v1.0/admin/people/profileCardProperties"))
        .header("content-type", "application/json")
        .graph_json()
        .json(obj! { "directoryPropertyName": ?name, "annotations": annotations })
        .data()
        .await?;
    cx.log(flatten(cx, r)?)
}

pub async fn profilecard_get(cx: &mut Cx) -> Result<()> {
    let n = cx.u("name");
    cx.verbose_log(&format!(
        "Retrieving information about profile card property '{n}'..."
    ));
    let p = property_name(&n).unwrap_or("undefined");
    match Request::get(format!(
        "{GRAPH}/v1.0/admin/people/profileCardProperties/{p}"
    ))
    .graph_json()
    .data()
    .await
    {
        Ok(r) => cx.log(flatten(cx, r)?),
        Err(e) if e.status() == Some(404) => Err(Error::msg(format!(
            "Profile card property '{n}' does not exist."
        ))),
        Err(e) => Err(e),
    }
}

pub async fn profilecard_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Listing all profile card properties...");
    let mut items =
        odata::get_all_items(&format!("{GRAPH}/v1.0/admin/people/profileCardProperties")).await?;
    let out = cx.str("output");
    if out.is_none() || out.as_deref() == Some("json") {
        return cx.log(Value::Array(items));
    }
    let locs = |p: &Value| {
        p.get("annotations")
            .and_then(|a| a.get(0))
            .and_then(|a| a.get("localizations"))
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or(0)
    };
    items.sort_by_key(|p| std::cmp::Reverse(locs(p)));
    let out: Vec<Value> = items
        .iter()
        .map(|p| {
            let first = p.get("annotations").and_then(|a| a.get(0));
            let mut r = Obj::new();
            r.insert(
                "directoryPropertyName".into(),
                p.get("directoryPropertyName")
                    .cloned()
                    .unwrap_or(Value::Null),
            );
            if let Some(d) = first
                .and_then(|f| f.get("displayName"))
                .filter(|d| truthy(d))
            {
                r.insert("displayName".into(), d.clone());
            }
            for l in first.map(|f| arr(f, "localizations")).unwrap_or(&[]) {
                r.insert(
                    format!("displayName {}", u(l, "languageTag")),
                    l.get("displayName").cloned().unwrap_or(Value::Null),
                );
            }
            Value::Object(r)
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn profilecard_remove(cx: &mut Cx) -> Result<()> {
    let p = property_name(&cx.u("name")).unwrap_or("undefined");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the profile card property '{p}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing '{p}' as a profile card property..."));
    Request::delete(format!(
        "{GRAPH}/v1.0/admin/people/profileCardProperties/{p}"
    ))
    .header("content-type", "application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn profilecard_set(cx: &mut Cx) -> Result<()> {
    let n = cx.u("name");
    cx.verbose_log(&format!("Updating profile card property '{n}'..."));
    let p = CUSTOM
        .iter()
        .copied()
        .find(|c| c.eq_ignore_ascii_case(&n))
        .unwrap_or("undefined");
    let unknown = m365_core::ut::options::unknown_options(&cx.opts, cx.meta);
    let locs: Vec<Value> = unknown.iter().map(|(k, v)| obj! { "languageTag": k[k.find('-').map(|i| i + 1).unwrap_or(0)..].to_string(), "displayName": v.clone() }).collect();
    let r = Request::patch(format!("{GRAPH}/v1.0/admin/people/profileCardProperties/{p}"))
        .graph_json()
        .json(obj! { "annotations": [obj! { "displayName": ?cx.val("displayName").cloned(), "localizations": locs }] })
        .data()
        .await?;
    cx.log(flatten(cx, r)?)
}

pub async fn pronouns_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving information about pronouns settings...");
    cx.log(
        Request::get(format!("{GRAPH}/v1.0/admin/people/pronouns"))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn pronouns_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Updating pronouns settings...");
    let r = Request::patch(format!("{GRAPH}/v1.0/admin/people/pronouns"))
        .graph_json()
        .json(obj! { "isEnabledInOrganization": ?cx.val("enabled").cloned() })
        .data()
        .await?;
    cx.log(r)
}

async fn activation_report(cx: &Cx, name: &str) -> Result<()> {
    let r = Request::get(format!("{GRAPH}/v1.0/reports/{name}"))
        .graph_json()
        .data()
        .await?;
    if cx.str("output").is_some_and(|o| o.to_lowercase() == "json") {
        if !r.is_string() {
            let kind = if r.is_array() { "Array" } else { "Object" };
            return Err(Error::js(format!(
                "The \"target\" argument must be an instance of Buffer or Uint8Array. Received an instance of {kind}"
            )));
        }
        return cx.log(Value::Array(util::parse_csv_to_json(
            &to_js_string(&r),
            '"',
            ',',
            true,
        )));
    }
    cx.log(r)
}

pub async fn report_office365activationcounts(cx: &mut Cx) -> Result<()> {
    activation_report(cx, "getOffice365ActivationCounts").await
}

pub async fn report_office365activationsusercounts(cx: &mut Cx) -> Result<()> {
    activation_report(cx, "getOffice365ActivationsUserCounts").await
}

pub async fn report_office365activationsuserdetail(cx: &mut Cx) -> Result<()> {
    activation_report(cx, "getOffice365ActivationsUserDetail").await
}

pub async fn report_settings_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting tenant-level settings for Microsoft 365 reports...");
    cx.log(
        Request::get(format!("{GRAPH}/v1.0/admin/reportSettings"))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn report_settings_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating report setting displayConcealedNames to {}",
        cx.u("displayConcealedNames")
    ));
    Request::patch(format!("{GRAPH}/v1.0/admin/reportSettings"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "displayConcealedNames": ?cx.val("displayConcealedNames").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn security_alerts_list(cx: &mut Cx) -> Result<()> {
    let q = match cx.s("vendor") {
        Some(v) => {
            let name = match v.to_lowercase().as_str() {
                "azure security center" => "ASC".to_string(),
                "microsoft cloud app security" => "MCAS".to_string(),
                "azure active directory identity protection" => "IPC".to_string(),
                _ => v,
            };
            format!("?$filter=vendorInformation/provider eq '{}'", eqp(&name))
        }
        None => String::new(),
    };
    let r = Request::get(format!("{GRAPH}/v1.0/security/alerts{q}"))
        .graph_json()
        .data()
        .await?;
    match r.get("value").filter(|v| truthy(v)) {
        Some(v) => cx.log(v.clone()),
        None => Err(Error::msg("Error fetching security alerts")),
    }
}

fn expand_issues(cx: &Cx) -> &'static str {
    let out = cx.str("output");
    if cx.bool("issues") && (out.is_none() || out.is_some_and(|o| o.to_lowercase() == "json")) {
        "?$expand=issues"
    } else {
        ""
    }
}

pub async fn health_get(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/admin/serviceAnnouncement/healthOverviews/{}{}",
        cx.u("serviceName"),
        expand_issues(cx)
    ))
    .graph_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn health_list(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/admin/serviceAnnouncement/healthOverviews{}",
        expand_issues(cx)
    ))
    .graph_json()
    .data()
    .await?;
    match r.get("value").filter(|v| truthy(v)) {
        Some(v) => cx.log(v.clone()),
        None => Err(Error::msg("Error fetching service health")),
    }
}

pub async fn healthissue_get(cx: &mut Cx) -> Result<()> {
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/admin/serviceAnnouncement/issues/{}",
            eqp(&cx.u("id"))
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn healthissue_list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/admin/serviceAnnouncement/issues");
    if let Some(s) = cx.s("service") {
        url.push_str(&format!("?$filter=service eq '{}'", eqp(&s)));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn message_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving service update message {id}"));
    cx.log(
        Request::get(format!(
            "{GRAPH}/v1.0/admin/serviceAnnouncement/messages/{id}"
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn message_list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/admin/serviceAnnouncement/messages");
    if let Some(s) = cx.s("service") {
        url.push_str(&format!("?$filter=services/any(c:c+eq+'{}')", eqp(&s)));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

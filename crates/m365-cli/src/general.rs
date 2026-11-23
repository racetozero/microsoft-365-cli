//! Top-level commands: version, docs, status, login, logout, request, search.

use m365_core::auth::{self, AuthType, CloudType};
use m365_core::config::{self, names};
use m365_core::http::{Method, Request, ResponseType};
use m365_core::json::{Obj, stringify_pretty, to_js_string};
use m365_core::{Cx, Error, Result, Value, obj};

pub async fn version(cx: &mut Cx) -> Result<()> {
    cx.log(Value::from(format!(
        "v{}",
        m365_core::manifest::UPSTREAM_VERSION
    )))
}

pub async fn docs(cx: &mut Cx) -> Result<()> {
    let home = "https://pnp.github.io/cli-microsoft365/";
    cx.log(Value::from(home))?;
    if config::get_bool(names::AUTO_OPEN_LINKS_IN_BROWSER, false) {
        let _ = m365_core::util::open_browser(home);
    }
    Ok(())
}

fn details_with_tokens(key: &str) -> Value {
    let c = auth::connection();
    let mut d = auth::connection_details(&serde_json::to_value(&c).unwrap_or(Value::Null));
    if cx_debug()
        && let Some(o) = d.as_object_mut()
    {
        o.insert(
            key.into(),
            Value::from(stringify_pretty(
                &Value::Object(c.access_tokens.clone()),
                "  ",
            )),
        );
    }
    d
}

fn cx_debug() -> bool {
    m365_core::log::debug()
}

pub async fn status(cx: &mut Cx) -> Result<()> {
    if auth::is_active() {
        if let Err(e) = auth::ensure_access_token(&auth::default_resource(), false).await {
            if cx.debug() {
                cx.stderr(&e.js_message());
            }
            auth::with_connection(|c| c.deactivate());
            return Err(Error::msg(format!(
                "Your login has expired. Sign in again to continue. {}",
                e.js_message()
            )));
        }
        cx.log(details_with_tokens("accessTokens"))
    } else {
        let message = if auth::all_connections().is_empty() {
            "Logged out"
        } else {
            "Logged out, signed in connections available"
        };
        if cx.verbose() {
            cx.stderr(message);
            Ok(())
        } else {
            cx.log(Value::from(message))
        }
    }
}

pub async fn logout(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Logging out from Microsoft 365...");
    if let Err(e) = auth::clear_connection_info() {
        if cx.debug() {
            cx.stderr(&e.js_message());
        }
        return Err(e);
    }
    auth::with_connection(|c| c.deactivate());
    Ok(())
}

pub fn login_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let at = o.get("authType").and_then(Value::as_str);
    let t = |k: &str| o.get(k).is_some_and(m365_core::json::truthy);
    Ok(match i {
        0 => {
            o.contains_key("appId")
                || config::client_id().is_some()
                || at == Some("identity")
                || at == Some("federatedIdentity")
        }
        1 => at != Some("password") || t("userName"),
        2 => at != Some("password") || t("password"),
        3 => at != Some("certificate") || !(t("certificateFile") && t("certificateBase64Encoded")),
        4 => {
            at != Some("certificate")
                || t("certificateFile")
                || t("certificateBase64Encoded")
                || config::get(names::CLIENT_CERTIFICATE_FILE)
                    .is_some_and(|v| m365_core::json::truthy(&v))
                || config::get(names::CLIENT_CERTIFICATE_BASE64_ENCODED)
                    .is_some_and(|v| m365_core::json::truthy(&v))
        }
        5 => {
            at != Some("secret")
                || t("secret")
                || config::get(names::CLIENT_SECRET).is_some_and(|v| m365_core::json::truthy(&v))
        }
        _ => true,
    })
}

pub fn login_field(idx: u16, v: &Value, _o: &Obj) -> m365_core::schema::FieldCheck {
    m365_core::schema::check({
        match idx {
            // certificateFile exists
            0 => !m365_core::json::truthy(v) || std::path::Path::new(&to_js_string(v)).exists(),
            // connectionName is not in use
            _ => {
                let name = to_js_string(v);
                !auth::all_connections()
                    .iter()
                    .any(|c| c.get("name").and_then(Value::as_str) == Some(name.as_str()))
            }
        }
    })
}

fn should_login(cx: &Cx) -> bool {
    let c = auth::connection();
    if !c.active || !cx.bool("ensure") {
        return true;
    }
    let auth_type = cx
        .s("authType")
        .unwrap_or_else(|| config::get_string(names::AUTH_TYPE, "deviceCode"));
    if auth_type != c.auth_type.as_str() {
        return true;
    }
    if cx.str("cloud").as_deref() != Some(c.cloud_type.as_str()) {
        return true;
    }
    if let Some(a) = cx.s("appId")
        && Some(a.as_str()) != c.app_id.as_deref()
    {
        return true;
    }
    if let Some(t) = cx.s("tenant")
        && t != c.tenant
    {
        return true;
    }
    if auth_type == "password" && cx.s("password").is_some() && cx.str("userName") != c.user_name {
        return true;
    }
    if auth_type == "certificate"
        && let Some(f) = cx.s("certificateFile")
    {
        let b = std::fs::read(&f)
            .map(|b| base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b))
            .unwrap_or_default();
        if c.certificate.as_deref() != Some(b.as_str()) {
            return true;
        }
    }
    if auth_type == "identity" && cx.s("userName").is_some() && cx.str("userName") != c.user_name {
        return true;
    }
    if auth_type == "secret" && cx.s("secret").is_some() && cx.str("secret") != c.secret {
        return true;
    }
    let t = c.token(&auth::default_resource());
    let exp = t
        .and_then(|t| {
            t.expires_on
                .as_str()
                .and_then(m365_core::time::parse_js_date)
        })
        .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());
    exp < chrono::Utc::now()
}

async fn login_ensure_token(cx: &Cx) -> Result<()> {
    match auth::ensure_access_token(&auth::default_resource(), false).await {
        Ok(_) => {
            auth::with_connection(|c| c.active = true);
            Ok(())
        }
        Err(e) => {
            if cx.debug() {
                cx.stderr("Error:");
                cx.stderr(&e.js_message());
                cx.stderr("");
            }
            Err(Error::msg(match &e {
                Error::Command {
                    message: Value::String(s),
                    ..
                } => s.clone(),
                Error::Command { message, .. } => message
                    .get("message")
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into()),
                other => other.js_message(),
            }))
        }
    }
}

pub async fn login(cx: &mut Cx) -> Result<()> {
    if cx.debug() {
        cx.stderr("Logging out from Microsoft 365...");
    }
    if !should_login(cx) {
        return login_ensure_token(cx).await;
    }
    auth::with_connection(|c| c.deactivate());
    cx.verbose_log("Signing in to Microsoft 365...");
    let auth_type = cx
        .s("authType")
        .unwrap_or_else(|| config::get_string(names::AUTH_TYPE, "deviceCode"));
    let app_id = cx.s("appId").or_else(config::client_id);
    let tenant = cx.s("tenant").unwrap_or_else(config::tenant);
    let name = cx.str("connectionName");
    let cert = || -> Option<String> {
        if let Some(f) = cx.s("certificateFile") {
            return std::fs::read(&f)
                .ok()
                .map(|b| base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b));
        }
        if let Some(b) = cx.s("certificateBase64Encoded") {
            return Some(b);
        }
        config::get_opt_string(names::CLIENT_CERTIFICATE_FILE)
            .or_else(|| config::get_opt_string(names::CLIENT_CERTIFICATE_BASE64_ENCODED))
    };
    let certificate = if auth_type == "certificate" {
        cert()
    } else {
        None
    };
    let cloud = cx
        .str("cloud")
        .and_then(|c| CloudType::parse(&c))
        .unwrap_or_default();
    auth::with_connection(|c| {
        c.app_id = app_id;
        c.tenant = tenant;
        c.name = name;
        match auth_type.as_str() {
            "password" => {
                c.auth_type = AuthType::Password;
                c.user_name = cx.str("userName");
                c.password = cx.str("password");
            }
            "certificate" => {
                c.auth_type = AuthType::Certificate;
                c.certificate = certificate;
                c.thumbprint = cx.str("thumbprint");
                c.password = cx
                    .str("password")
                    .or_else(|| config::get_opt_string(names::CLIENT_CERTIFICATE_PASSWORD));
            }
            "identity" => {
                c.auth_type = AuthType::Identity;
                c.user_name = cx.str("userName");
            }
            "federatedIdentity" => c.auth_type = AuthType::FederatedIdentity,
            "browser" => c.auth_type = AuthType::Browser,
            "secret" => {
                c.auth_type = AuthType::Secret;
                c.secret = cx
                    .s("secret")
                    .or_else(|| config::get_opt_string(names::CLIENT_SECRET));
            }
            _ => {}
        }
        c.cloud_type = cloud;
    });
    login_ensure_token(cx).await?;
    cx.log(details_with_tokens("accessToken"))
}

fn resolve_url_tokens(url: &str) -> Result<String> {
    if let Some(r) = url.strip_prefix("@graphbeta") {
        return Ok(format!("https://graph.microsoft.com/beta{r}"));
    }
    if let Some(r) = url.strip_prefix("@graph") {
        return Ok(format!("https://graph.microsoft.com/v1.0{r}"));
    }
    if let Some(r) = url.strip_prefix("@spo") {
        return match auth::connection().spo_url {
            Some(s) => Ok(format!("{s}{r}")),
            None => Err(Error::msg(
                "SharePoint root site URL is unknown. Please set your SharePoint URL using command 'spo set'.",
            )),
        };
    }
    Ok(url.to_string())
}

pub fn request_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(m365_core::json::truthy);
    Ok(match i {
        0 => !t("body") || o.get("method").and_then(Value::as_str) != Some("get"),
        1 => !t("body") || t("content-type"),
        2 => {
            !t("filePath") || {
                let p = to_js_string(&o["filePath"]);
                let dir = std::path::Path::new(&p).parent().map(|d| {
                    if d.as_os_str().is_empty() {
                        std::path::Path::new(".")
                    } else {
                        d
                    }
                });
                dir.is_some_and(|d| d.exists())
            }
        }
        _ => true,
    })
}

pub async fn request(cx: &mut Cx) -> Result<()> {
    if cx.debug() {
        cx.stderr("Preparing request...");
    }
    let r = async {
        let url = resolve_url_tokens(&cx.str("url").unwrap_or_default())?;
        let method = cx
            .str("method")
            .unwrap_or_else(|| "get".into())
            .to_uppercase();
        let unknown = m365_core::ut::options::unknown_options(&cx.opts, cx.meta);
        let mut req = Request::new(Method::parse(&method).unwrap_or(Method::Get), url);
        let mut has_accept = false;
        for (k, v) in &unknown {
            if k == "accept" {
                has_accept = true;
            }
            req = req.header(k, to_js_string(v));
        }
        if !has_accept {
            req = req.header("accept", "application/json");
        }
        let accept = unknown
            .get("accept")
            .map(to_js_string)
            .unwrap_or_else(|| "application/json".into());
        if let Some(r) = cx.s("resource") {
            req = req.resource(&r);
        }
        if let Some(b) = cx.str("body") {
            req = req.text(b);
        }
        if accept.starts_with("application/json") {
            req.response_type = ResponseType::Json;
        }
        cx.verbose_log("Executing request...");
        if let Some(path) = cx.s("filePath") {
            req.response_type = ResponseType::Bytes;
            let res = req.send().await?;
            std::fs::write(&path, &res.bytes)?;
            cx.verbose_log(&format!("File saved to path {path}"));
            Ok(())
        } else {
            let v = req.data().await?;
            cx.log(v)
        }
    }
    .await;
    r.map_err(Error::plain)
}

pub fn search_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let scopes: Vec<String> = o
        .get("scopes")
        .map(to_js_string)
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    let t = |k: &str| o.get(k).is_some_and(m365_core::json::truthy);
    Ok(match i {
        0 => !t("sortBy") || !scopes.iter().any(|s| s == "message" || s == "event"),
        1 => {
            !t("enableTopResults")
                || match scopes.len() {
                    1 => scopes[0] == "message" || scopes[0] == "chatMessage",
                    2 => {
                        scopes.iter().any(|s| s == "message")
                            && scopes.iter().any(|s| s == "chatMessage")
                    }
                    _ => false,
                }
        }
        _ => true,
    })
}

pub async fn search(cx: &mut Cx) -> Result<()> {
    let mut all = cx.bool("allResults");
    let mut start = cx.num("startIndex").filter(|n| *n != 0.0).unwrap_or(0.0);
    let page = cx.num("pageSize").filter(|n| *n != 0.0).unwrap_or(25.0);
    let scopes: Vec<Value> = cx
        .str("scopes")
        .unwrap_or_default()
        .split(',')
        .map(|s| Value::from(s.trim()))
        .collect();
    let fields: Option<Vec<Value>> = cx
        .s("select")
        .map(|s| s.split(',').map(|p| Value::from(p.trim())).collect());
    let sort: Option<Vec<Value>> = cx.s("sortBy").map(|s| {
        s.split(',')
            .map(|p| {
                let parts: Vec<&str> = p.trim().split(':').collect();
                let desc = parts.len() == 2 && parts[1].trim() == "desc";
                obj! { "name": parts[0], "isDescending": desc }
            })
            .collect()
    });
    let mut hits: Vec<Value> = Vec::new();
    let mut result;
    loop {
        let body = obj! { "requests": vec![obj! {
            "entityTypes": scopes.clone(),
            "query": obj! { "queryString": cx.str("queryText").unwrap_or_else(|| "*".into()) },
            "enableTopResults": ?cx.opt_bool("enableTopResults"),
            "from": m365_core::json::num_value(start),
            "size": m365_core::json::num_value(page),
            "fields": ?fields.clone(),
            "sortProperties": ?sort.clone(),
            "queryAlterationOptions": obj! {
                "enableModification": ?cx.opt_bool("enableSpellingModification"),
                "enableSuggestion": ?cx.opt_bool("enableSpellingSuggestion")
            }
        }] };
        let res = Request::post("https://graph.microsoft.com/v1.0/search/query")
            .graph_json()
            .json(body)
            .data()
            .await?;
        // `response.value[0].hitsContainers[0]` throws a TypeError when a
        // level is missing, which also ends the `allResults` loop
        let reading = |p: &str| {
            Error::js(format!(
                "Cannot read properties of undefined (reading '{p}')"
            ))
        };
        let values = res.get("value").ok_or_else(|| reading("0"))?;
        result = values
            .get(0)
            .cloned()
            .ok_or_else(|| reading("hitsContainers"))?;
        let containers = result
            .get("hitsContainers")
            .filter(|h| m365_core::json::truthy(h));
        let missing = if all { "moreResultsAvailable" } else { "hits" };
        let container = containers
            .map(|cs| cs.get(0).cloned().ok_or_else(|| reading(missing)))
            .transpose()?;
        if all && let Some(c) = &container {
            all = c
                .get("moreResultsAvailable")
                .is_some_and(m365_core::json::truthy);
        }
        if all {
            start += page;
        }
        if let Some(h) = container
            .as_ref()
            .and_then(|c| c.get("hits"))
            .and_then(Value::as_array)
        {
            hits.extend(h.iter().cloned());
        }
        if !all {
            break;
        }
    }
    if cx.bool("resultsOnly") {
        return cx.log(Value::Array(hits));
    }
    if let Some(c) = result.get_mut("hitsContainers").and_then(|h| h.get_mut(0))
        && c.get("hits").is_some()
        && let Some(o) = c.as_object_mut()
    {
        o.insert("hits".into(), Value::Array(hits));
    }
    cx.log(result)
}

pub fn search_field(idx: u16, v: &Value, _o: &Obj) -> m365_core::schema::FieldCheck {
    m365_core::schema::check({
        const ALLOWED: &[&str] = &[
            "chatMessage",
            "message",
            "event",
            "drive",
            "driveItem",
            "list",
            "listItem",
            "site",
            "bookmark",
            "acronym",
            "person",
        ];
        let n = v.as_f64().unwrap_or(f64::NAN);
        match idx {
            0 => to_js_string(v)
                .split(',')
                .map(str::trim)
                .all(|s| ALLOWED.contains(&s)),
            1 => n >= 0.0,
            _ => (1.0..=500.0).contains(&n),
        }
    })
}

//! `src/utils/spo.ts`: SharePoint helpers (form digests, CSOM, URLs).

use crate::prelude::*;

/// `config.applicationName`, sent in every CSOM request.
pub fn application_name() -> String {
    format!(
        "CLI for Microsoft 365 v{}",
        crate::manifest::UPSTREAM_VERSION
    )
}

#[derive(Clone, Debug)]
pub struct FormDigest {
    pub value: String,
    pub timeout_seconds: i64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub web_full_url: Option<String>,
}

/// `spo.getRequestDigest(siteUrl)`: POST `/_api/contextinfo`.
pub async fn get_request_digest(site_url: &str) -> Result<Value> {
    Request::post(format!("{site_url}/_api/contextinfo"))
        .spo_json()
        .data()
        .await
}

/// `spo.ensureFormDigest`.
pub async fn ensure_form_digest(
    site_url: &str,
    context: Option<&FormDigest>,
    debug: bool,
) -> Result<FormDigest> {
    if let Some(c) = context
        && !c.value.is_empty()
        && chrono::Utc::now() < c.expires_at
    {
        if debug {
            crate::log::log_stderr("Existing form digest still valid");
        }
        return Ok(c.clone());
    }
    let res = get_request_digest(site_url).await?;
    let timeout = res
        .get("FormDigestTimeoutSeconds")
        .and_then(Value::as_i64)
        .unwrap_or(0);
    Ok(FormDigest {
        value: s(&res, "FormDigestValue").to_string(),
        timeout_seconds: timeout,
        expires_at: chrono::Utc::now() + chrono::Duration::seconds(timeout - 5),
        web_full_url: res
            .get("WebFullUrl")
            .and_then(Value::as_str)
            .map(str::to_string),
    })
}

/// The `<Request ...>` envelope of a CSOM call.
pub fn csom(actions: &str, object_paths: &str) -> String {
    format!(
        r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions>{actions}</Actions><ObjectPaths>{object_paths}</ObjectPaths></Request>"#,
        application_name()
    )
}

/// POSTs a CSOM body to `/_vti_bin/client.svc/ProcessQuery` and returns
/// the raw text response (as upstream, which parses it itself).
pub async fn process_query(site_url: &str, digest: &str, body: String) -> Result<String> {
    let v = Request::post(format!("{site_url}/_vti_bin/client.svc/ProcessQuery"))
        .header("X-RequestDigest", digest)
        .text(body)
        .data()
        .await?;
    Ok(match v {
        Value::String(s) => s,
        other => to_js_string(&other),
    })
}

/// Like [`process_query`] with an extra `accept` header, as some calls send.
pub async fn process_query_accept(site_url: &str, digest: &str, body: String) -> Result<String> {
    let v = Request::post(format!("{site_url}/_vti_bin/client.svc/ProcessQuery"))
        .header("X-RequestDigest", digest)
        .accept("application/json;odata=nometadata")
        .text(body)
        .data()
        .await?;
    Ok(match v {
        Value::String(s) => s,
        other => to_js_string(&other),
    })
}

/// Parses a CSOM response (`JSON.parse(res)`).
pub fn parse_csom(text: &str) -> Result<Vec<Value>> {
    match json::js_parse(text) {
        Ok(Value::Array(a)) => Ok(a),
        Ok(other) => Ok(vec![other]),
        Err(e) => Err(Error::Js(e.trim_start_matches("SyntaxError: ").to_string()).with_syntax()),
    }
}

/// The `ErrorInfo.ErrorMessage` of the first CSOM response element, if any.
pub fn csom_error(json: &[Value]) -> Option<Value> {
    let ei = json.first()?.get("ErrorInfo")?;
    if ei.is_null() {
        return None;
    }
    Some(ei.get("ErrorMessage").cloned().unwrap_or(Value::Null))
}

/// `spo.getSpoUrl`.
pub async fn get_spo_url(debug: bool) -> Result<String> {
    if let Some(u) = auth::connection().spo_url {
        if debug {
            crate::log::log_stderr(&format!("SPO URL previously retrieved {u}. Returning..."));
        }
        return Ok(u);
    }
    if debug {
        crate::log::log_stderr("No SPO URL available. Retrieving from MS Graph...");
    }
    let res = Request::get(format!("{GRAPH}/v1.0/sites/root?$select=webUrl"))
        .graph_json()
        .data()
        .await?;
    let url = s(&res, "webUrl").to_string();
    auth::with_connection(|c| c.spo_url = Some(url.clone()));
    if auth::store_connection_info().is_err() && debug {
        crate::log::log_stderr("Error while storing connection info");
    }
    Ok(url)
}

/// `spo.getSpoAdminUrl`.
pub async fn get_spo_admin_url(debug: bool) -> Result<String> {
    let u = get_spo_url(debug).await?;
    Ok(admin_url(&u))
}

/// `https://contoso.sharepoint.com` -> `https://contoso-admin.sharepoint.com`.
pub fn admin_url(u: &str) -> String {
    static RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"(https://)([^.]+)(.*)").unwrap());
    RE.replace(u, "${1}${2}-admin${3}").into_owned()
}

/// `spo.getTenantId`.
pub async fn get_tenant_id(debug: bool) -> Result<String> {
    if let Some(t) = auth::connection().spo_tenant_id {
        if debug {
            crate::log::log_stderr(&format!(
                "SPO Tenant ID previously retrieved {t}. Returning..."
            ));
        }
        return Ok(t);
    }
    if debug {
        crate::log::log_stderr("No SPO Tenant ID available. Retrieving...");
    }
    let admin = get_spo_admin_url(debug).await?;
    let ctx = get_request_digest(&admin).await?;
    let body = csom(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        r#"<Constructor Id="3" TypeId="{268004ae-ef6b-4e9b-8425-127220d84719}" />"#,
    );
    let res = process_query_accept(&admin, s(&ctx, "FormDigestValue"), body).await?;
    let json = parse_csom(&res)?;
    let id = json
        .last()
        .map(|l| s(l, "_ObjectIdentity_").replacen('\n', "&#xA;", 1))
        .unwrap_or_default();
    auth::with_connection(|c| c.spo_tenant_id = Some(id.clone()));
    if auth::store_connection_info().is_err() && debug {
        crate::log::log_stderr("Error while storing connection info");
    }
    Ok(id)
}

/// `spo.waitUntilFinished`.
pub async fn wait_until_finished(
    operation_id: &str,
    site_url: &str,
    context: Option<FormDigest>,
    debug: bool,
    verbose: bool,
) -> Result<()> {
    let mut op = operation_id.to_string();
    let mut ctx = context;
    loop {
        let d = ensure_form_digest(site_url, ctx.as_ref(), debug).await?;
        ctx = Some(d.clone());
        if debug {
            crate::log::log_stderr(&format!("Checking if operation {op} completed..."));
        }
        let name = op.replace("\\n", "&#xA;").replace('"', "");
        let body = csom(
            r#"<Query Id="188" ObjectPathId="184"><Query SelectAllProperties="false"><Properties><Property Name="IsComplete" ScalarProperty="true" /><Property Name="PollingInterval" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(r#"<Identity Id="184" Name="{name}" />"#),
        );
        let res = process_query(site_url, &d.value, body).await?;
        let json = parse_csom(&res)?;
        if let Some(e) = csom_error(&json) {
            return Err(Error::Js(to_js_string(&e)));
        }
        let last = json.last().cloned().unwrap_or(Value::Null);
        if last.get("IsComplete").is_some_and(truthy) {
            if !debug && verbose {
                use std::io::Write;
                let _ = std::io::stdout().write_all(b"\n");
            }
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        op = stringify(last.get("_ObjectIdentity_").unwrap_or(&Value::Null));
    }
}

/// `spo.getSpoGraphSiteId(webUrl)`.
pub async fn graph_site_id(web_url: &str) -> Result<String> {
    let url = url::Url::parse(web_url).map_err(|_| Error::js("Invalid URL"))?;
    let r = Request::get(format!(
        "https://graph.microsoft.com/v1.0/sites/{}:{}?$select=id",
        url.host_str().unwrap_or(""),
        url.path()
    ))
    .graph_json()
    .data()
    .await?;
    Ok(u(&r, "id"))
}

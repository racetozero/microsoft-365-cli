//! The local HTTP server of the browser login (`src/AuthServer.ts`).

use crate::auth::{Connection, login_endpoint};
use crate::error::{Error, Result};
use crate::log;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Opens the sign-in page and waits for the redirect with the code.
/// Returns (code, redirect URI).
pub async fn retrieve_code(conn: &Connection, resource: &str) -> Result<(String, String)> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| Error::msg(e.to_string()))?;
    let port = listener
        .local_addr()
        .map_err(|e| Error::msg(e.to_string()))?
        .port();
    let redirect = format!("http://localhost:{port}");
    let state: String = uuid::Uuid::new_v4().simple().to_string()[..20].to_string();
    let url = format!(
        "{}/{}/oauth2/authorize?response_type=code&client_id={}&redirect_uri={redirect}&state={state}&resource={resource}&prompt=select_account",
        login_endpoint(conn.cloud_type),
        conn.tenant,
        conn.app_id.as_deref().unwrap_or("")
    );
    if log::debug() {
        log::log_stderr("Redirect URL:");
        log::log_stderr(&url);
        log::log_stderr("");
    }
    if crate::util::open_browser(&url).is_err() {
        return Err(Error::value(serde_json::json!({
            "error": "Can't open the default browser",
            "errorDescription": "Was not able to open a browser instance. Try again later or use a different authentication method."
        })));
    }
    log::log_stderr(
        "To sign in, use the web browser that just has been opened. Please sign-in there.",
    );
    let (mut sock, _) = listener
        .accept()
        .await
        .map_err(|e| Error::msg(e.to_string()))?;
    let mut buf = vec![0u8; 16 * 1024];
    let n = sock.read(&mut buf).await.unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]).to_string();
    let path = req.split_whitespace().nth(1).unwrap_or("/").to_string();
    if log::debug() {
        log::log_stderr("Response:");
        log::log_stderr(&path);
        log::log_stderr("");
    }
    let parsed = url::Url::parse(&format!("http://localhost{path}")).ok();
    let q = |k: &str| {
        parsed.as_ref().and_then(|u| {
            u.query_pairs()
                .find(|(n, _)| n == k)
                .map(|(_, v)| v.into_owned())
        })
    };
    let (body, result) = if let Some(code) = q("code") {
        (
            "<script type=\"text/JavaScript\">setTimeout(function(){ window.location = \"https://pnp.github.io/cli-microsoft365/\"; },10000);</script><p><b>You have logged into CLI for Microsoft 365!</b></p><p>You can close this window, or we will redirect you to the <a href=\"https://pnp.github.io/cli-microsoft365/\">CLI for Microsoft 365</a> documentation in 10 seconds.</p>".to_string(),
            Ok((code, redirect)),
        )
    } else if let Some(err) = q("error") {
        let desc = q("error_description");
        let mut b =
            format!("<p>Oops! Microsoft Entra ID replied with an error message.</p><p>{err}</p>");
        if let Some(d) = &desc {
            b.push_str(&format!("<p>{d}</p>"));
        }
        (
            b,
            Err(Error::value(
                serde_json::json!({ "error": err, "errorDescription": desc }),
            )),
        )
    } else {
        (
            "<p>Oops! This is an invalid request.</p><p>invalid request</p><p>An invalid request has been received by the HTTP server</p>".to_string(),
            Err(Error::value(serde_json::json!({ "error": "invalid request", "errorDescription": "An invalid request has been received by the HTTP server" }))),
        )
    };
    let html = format!(
        "<html><head><title>CLI for Microsoft 365</title></head><body>{body}</body></html>"
    );
    let resp = format!(
        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{html}",
        html.len()
    );
    let _ = sock.write_all(resp.as_bytes()).await;
    result
}

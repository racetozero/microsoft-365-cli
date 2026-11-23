//! Parity-checker transport (feature `parity`): instead of the network, each
//! request is described to the mock server named by `M365_PARITY_MOCK`, which
//! records it and returns a canned response. The upstream harness sends the
//! same description, so both CLIs see identical responses.

use crate::http::{Request, Response, body_text};
use crate::json::Obj;
use serde_json::Value;

pub async fn send(
    mock: &str,
    req: &Request,
    headers: &[(String, String)],
) -> Result<Response, String> {
    let mut h = Obj::new();
    for (k, v) in headers {
        h.insert(k.to_ascii_lowercase(), Value::from(v.as_str()));
    }
    let desc = serde_json::json!({
        "method": req.method.as_str(),
        "url": req.url,
        "headers": h,
        "body": body_text(&req.body),
    });
    let res = reqwest::Client::new()
        .post(format!("{mock}/__m365"))
        .body(crate::json::stringify(&desc))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let v: Value = res.json().await.map_err(|e| e.to_string())?;
    let mut headers = Obj::new();
    if let Some(o) = v.get("headers").and_then(Value::as_object) {
        for (k, val) in o {
            headers.insert(k.to_ascii_lowercase(), val.clone());
        }
    }
    let status = v.get("status").and_then(Value::as_u64).unwrap_or(200) as u16;
    let body = v
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or("")
        .as_bytes()
        .to_vec();
    Ok(Response {
        status,
        status_text: v
            .get("statusText")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        headers,
        bytes: body,
        response_type: req.response_type,
    })
}

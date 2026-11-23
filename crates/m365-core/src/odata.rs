//! `src/utils/odata.ts`: follows `@odata.nextLink` / `nextLink` pages.

use crate::error::Result;
use crate::http::Request;
use serde_json::Value;

/// `odata.getAllItems(url)` with `odata.metadata=none`.
pub async fn get_all_items(url: &str) -> Result<Vec<Value>> {
    get_all_items_meta(url, "none").await
}

/// `odata.getAllItems(url, metadata)`.
pub async fn get_all_items_meta(url: &str, metadata: &str) -> Result<Vec<Value>> {
    let req = Request::get(url)
        .accept(&format!("application/json;odata.metadata={metadata}"))
        .header("odata-version", "4.0")
        .json_response();
    get_all_items_req(req).await
}

/// `odata.getAllItems(requestOptions)`.
pub async fn get_all_items_req(req: Request) -> Result<Vec<Value>> {
    let mut items = Vec::new();
    let mut req = req;
    loop {
        let res = req.clone().data().await?;
        if let Some(Value::Array(v)) = res.get("value") {
            items.extend(v.iter().cloned());
        }
        let next = res
            .get("@odata.nextLink")
            .or_else(|| res.get("nextLink"))
            .and_then(Value::as_str)
            .map(str::to_string);
        match next {
            Some(n) if !n.is_empty() => req.url = n,
            _ => return Ok(items),
        }
    }
}

//! `drive.ts` and `spo.getSiteIdByMSGraph`.

use crate::prelude::*;

/// `spo.getSiteIdByMSGraph(webUrl)`.
pub async fn site_id_by_ms_graph(web_url: &str, log: Option<&Cx>) -> Result<String> {
    if let Some(cx) = log {
        cx.verbose_log(&format!("Getting site id for URL: {web_url}..."));
    }
    let url = parse(web_url)?;
    let r = Request::get(format!(
        "{GRAPH}/v1.0/sites/{}:{}?$select=id",
        eqp(&host(&url)),
        url.path()
    ))
    .graph_json()
    .data()
    .await?;
    Ok(u(&r, "id"))
}

/// `new URL(s)`.
pub fn parse(s: &str) -> Result<url::Url> {
    url::Url::parse(s).map_err(|_| Error::js("Invalid URL"))
}

/// `url.host` (with the port, when not the default).
pub fn host(url: &url::Url) -> String {
    match url.port() {
        Some(p) => format!("{}:{p}", url.host_str().unwrap_or("")),
        None => url.host_str().unwrap_or("").to_string(),
    }
}

/// The drive whose web URL is the longest prefix of `url`.
pub fn find_drive(drives: &[Value], href: &str) -> Option<Value> {
    let lower = href.to_lowercase();
    let mut sorted: Vec<&Value> = drives.iter().collect();
    sorted.sort_by(|a, b| util::locale_compare(s(b, "webUrl"), s(a, "webUrl")));
    sorted
        .into_iter()
        .find(|d| {
            let du = s(d, "webUrl").to_lowercase();
            lower.starts_with(&du)
                && (du.len() == lower.len() || lower.as_bytes().get(du.len()) == Some(&b'/'))
        })
        .cloned()
}

pub async fn drives(site_id: &str) -> Result<Vec<Value>> {
    let r = Request::get(format!(
        "{GRAPH}/v1.0/sites/{site_id}/drives?$select=webUrl,id"
    ))
    .graph_json()
    .data()
    .await?;
    Ok(arr(&r, "value").to_vec())
}

/// `drive.getDriveByUrl`.
pub async fn drive_by_url(site_id: &str, url: &url::Url, log: Option<&Cx>) -> Result<Value> {
    if let Some(cx) = log {
        cx.verbose_log(&format!(
            "Retrieving drive information for URL: {}",
            url.as_str()
        ));
    }
    let d = drives(site_id).await?;
    find_drive(&d, url.as_str())
        .ok_or_else(|| Error::js(format!("Drive '{}' not found", url.as_str())))
}

/// `href.replace(new RegExp(webUrl, 'i'), '')`.
pub fn strip_drive_url(href: &str, web_url: &str) -> String {
    match fancy_regex::Regex::new(&format!("(?i){web_url}")) {
        Ok(re) => re.replace(href, "").into_owned(),
        Err(_) => href.to_string(),
    }
}

/// `drive.getDriveItemId`.
pub async fn drive_item_id(drive: &Value, item_url: &url::Url, log: Option<&Cx>) -> Result<String> {
    let rel = strip_drive_url(item_url.as_str(), s(drive, "webUrl"))
        .trim_end_matches('/')
        .to_string();
    if let Some(cx) = log {
        cx.verbose_log(&format!("Retrieving drive item ID for URL: {rel}"));
    }
    let r = Request::get(format!(
        "{GRAPH}/v1.0/drives/{}/root{}?$select=id",
        u(drive, "id"),
        if rel.is_empty() {
            String::new()
        } else {
            format!(":{rel}")
        }
    ))
    .graph_json()
    .data()
    .await?;
    Ok(u(&r, "id"))
}

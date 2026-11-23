//! `file *`: OneDrive/SharePoint files through Microsoft Graph.

use m365_core::prelude::*;
use m365_core::ut::{drive, url as urlutil};

/// `getGraphSiteInfo`: walks the path chunks while they resolve to sites.
/// `stop_at_end` is `file add`'s variant, which stops at the last chunk;
/// `file convert pdf` keeps going until a lookup fails.
async fn graph_site_info(
    host: &str,
    chunks: &[String],
    mut current: usize,
    mut last_site_id: String,
    stop_at_end: bool,
) -> Result<(String, String)> {
    loop {
        let end = (current + 1).min(chunks.len());
        let mut path = chunks[..end].join("/");
        if path.ends_with("/sites") || path.ends_with("/teams") || path.ends_with("/personal") {
            current += 1;
            last_site_id = String::new();
            continue;
        }
        if !path.starts_with('/') {
            path = format!("/{path}");
        }
        match Request::get(format!("{GRAPH}/v1.0/sites/{host}:{path}?$select=id"))
            .graph_json()
            .data()
            .await
        {
            Ok(r) => {
                if stop_at_end && current == chunks.len().saturating_sub(1) {
                    return Ok((u(&r, "id"), path));
                }
                current += 1;
                last_site_id = u(&r, "id");
            }
            Err(e) => {
                if last_site_id.is_empty() {
                    return Err(e);
                }
                let mut sr = chunks[..current.min(chunks.len())].join("/");
                if !sr.starts_with('/') {
                    sr = format!("/{sr}");
                }
                return Ok((last_site_id, sr));
            }
        }
    }
}

/// `getGraphFileUrl` of `file add` and `file convert pdf`.
async fn graph_file_url(
    cx: &Cx,
    file_web_url: &str,
    site_url: Option<&str>,
    stop_at_end: bool,
) -> Result<String> {
    cx.debug_log(&format!(
        "Resolving Graph drive item URL for {file_web_url}"
    ));
    let file = drive::parse(file_web_url)?;
    let site = drive::parse(site_url.unwrap_or(file_web_url))?;
    let chunks: Vec<String> = site.path().split('/').map(String::from).collect();
    let start = if site_url.is_some() {
        chunks.len() - 1
    } else {
        0
    };
    let host = if stop_at_end {
        drive::host(&site)
    } else {
        site.host_str().unwrap_or("").to_string()
    };
    let (site_id, server_relative) =
        graph_site_info(&host, &chunks, start, String::new(), stop_at_end).await?;
    let mut rel = file.path().replacen(&server_relative, "", 1);
    if !rel.starts_with('/') {
        rel = format!("/{rel}");
    }
    let parts: Vec<&str> = rel.split('/').collect();
    let drive_rel = format!(
        "/{}",
        parts.get(2..).map(|p| p.join("/")).unwrap_or_default()
    );
    let lib = parts.get(1).copied().unwrap_or("undefined");
    let r = Request::get(format!(
        "{GRAPH}/v1.0/sites/{site_id}/drives?$select=webUrl,id"
    ))
    .graph_json()
    .data()
    .await?;
    cx.debug_log(&format!(
        "Searching for drive with a URL ending with /{lib}..."
    ));
    let Some(d) = arr(&r, "value")
        .iter()
        .find(|d| s(d, "webUrl").ends_with(&format!("/{lib}")))
    else {
        return Err(Error::msg("Drive not found"));
    };
    let url = format!(
        "{GRAPH}/v1.0/sites/{site_id}/drives/{}/root:{drive_rel}",
        u(d, "id")
    );
    cx.debug_log(&format!("Resolved URL {url}"));
    Ok(url)
}

async fn upload_session(bytes: Vec<u8>, target: &str) -> Result<Value> {
    let res = Request::post(format!("{target}:/createUploadSession"))
        .graph_json()
        .data()
        .await?;
    let len = bytes.len();
    Request::put(u(&res, "uploadUrl"))
        .anonymous()
        .accept("application/json;odata.metadata=none")
        .header("Content-Length", len.to_string())
        .header("Content-Range", format!("bytes 0-{}/{len}", len as i64 - 1))
        .bytes(bytes)
        .json_response()
        .data()
        .await
}

fn read(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|e| Error::js(util::node_fs_error(&e, "open", path)))
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let folder = cx.u("folderUrl");
    let folder = folder.strip_suffix('/').unwrap_or(&folder);
    let fp = cx.u("filePath");
    let base = std::path::Path::new(&fp)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_default();
    let site = cx.str("siteUrl");
    let url = graph_file_url(cx, &format!("{folder}/{base}"), site.as_deref(), true).await?;
    let bytes = read(&fp)?;
    if bytes.is_empty() {
        Request::put(format!("{url}:/content"))
            .graph_json()
            .send()
            .await?;
    } else {
        upload_session(bytes, &url).await?;
    }
    Ok(())
}

pub async fn convert_pdf(cx: &mut Cx) -> Result<()> {
    let token = auth::default_access_token().unwrap_or_default();
    let Some(app) = auth::jwt::is_app_only(&token) else {
        return Err(Error::msg("Unable to determine authentication type"));
    };
    let source = cx.u("sourceFile");
    let target = cx.u("targetFile");
    let source_local = !source.to_lowercase().starts_with("https://");
    let target_local = !target.to_lowercase().starts_with("https://");
    let local_target = if target_local {
        target.clone()
    } else {
        let p = std::env::temp_dir()
            .join(util::new_guid())
            .to_string_lossy()
            .into_owned();
        cx.debug_log(&format!(
            "Target set to a URL. Will store the temporary converted file at {p}"
        ));
        p
    };
    let mut source_graph: Option<String> = None;
    let mut source_url = String::new();
    let r: Result<()> = async {
        source_url = if !source_local {
            source.clone()
        } else {
            cx.verbose_log("Uploading local file temporarily for conversion...");
            let drive_url = format!(
                "{GRAPH}/v1.0/{}",
                if app { "drive/root" } else { "me/drive/root" }
            );
            let ext = std::path::Path::new(&source)
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default();
            let g = format!("{drive_url}:/{}{ext}", util::new_guid());
            cx.debug_log(&format!("Source is a local file. Uploading to {g}..."));
            source_graph = Some(g.clone());
            let res = Request::post(format!("{g}:/createUploadSession"))
                .graph_json()
                .data()
                .await?;
            let bytes = read(&source)?;
            let len = bytes.len();
            let put = Request::put(u(&res, "uploadUrl"))
                .anonymous()
                .accept("application/json;odata.metadata=none")
                .header("Content-Length", len.to_string())
                .header("Content-Range", format!("bytes 0-{}/{len}", len as i64 - 1))
                .bytes(bytes)
                .json_response()
                .data()
                .await?;
            u(&put, "webUrl")
        };
        let gurl = resolve(cx, &source_url, source_graph.as_deref()).await?;
        cx.verbose_log("Converting file...");
        let file = Request::get(format!("{gurl}:/content?format=pdf"))
            .accept("application/json;odata.metadata=none")
            .bytes_response()
            .send()
            .await?;
        cx.verbose_log(&format!("Writing converted PDF file to {local_target}..."));
        std::fs::write(&local_target, &file.bytes)
            .map_err(|e| Error::js(util::node_fs_error(&e, "open", &local_target)))?;
        if target_local {
            cx.debug_log("Specified target is a local file. Not uploading.");
        } else {
            cx.verbose_log(&format!("Uploading converted PDF file to {target}..."));
            let t = resolve(cx, &target, None).await?;
            upload_session(read(&local_target)?, &t).await?;
        }
        Ok(())
    }
    .await;
    if source_local {
        cx.verbose_log(&format!("Deleting the temporary file at {source_url}..."));
        let g = resolve(cx, &source_url, source_graph.as_deref()).await?;
        Request::delete(g)
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    } else {
        cx.debug_log("Source file was URL. Not removing.");
    }
    if !target_local {
        cx.verbose_log(&format!(
            "Deleting the temporary PDF file at {local_target}..."
        ));
        std::fs::remove_file(&local_target)
            .map_err(|e| Error::js(util::node_fs_error(&e, "unlink", &local_target)))?;
    } else {
        cx.debug_log("Target is a local path. Not deleting");
    }
    r
}

async fn resolve(cx: &Cx, web_url: &str, graph_url: Option<&str>) -> Result<String> {
    if let Some(g) = graph_url {
        cx.debug_log(&format!("Resolving Graph drive item URL for {web_url}"));
        cx.debug_log(&format!(
            "Returning previously resolved Graph drive item URL {g}"
        ));
        return Ok(g.to_string());
    }
    graph_file_url(cx, web_url, None, false).await
}

fn absolute(web_url: &str, url: &str) -> String {
    if url.starts_with("https://") {
        url.to_string()
    } else {
        urlutil::get_absolute_url(web_url, url)
    }
}

/// The new name keeps the source file's extension.
fn new_name(source_path: &str, name: &str) -> String {
    let file = &source_path[source_path.rfind('/').map(|i| i + 1).unwrap_or(0)..];
    let ext = file.rfind('.').map(|i| &file[i..]).unwrap_or("");
    match name.rfind('.') {
        Some(i) => format!("{}{ext}", name.replacen(&name[i..], "", 1)),
        None => format!("{name}{ext}"),
    }
}

async fn library(
    cx: &Cx,
    site_id: &str,
    folder: &url::Url,
    from_user: &str,
    log_found: bool,
) -> Result<Value> {
    cx.verbose_log("Getting document library...");
    let d = drive::drives(site_id).await?;
    let Some(lib) = drive::find_drive(&d, folder.as_str()) else {
        return Err(Error::msg(format!(
            "Document library '{from_user}' not found"
        )));
    };
    if log_found {
        cx.verbose_log(&format!(
            "Document library: {}, {}",
            u(&lib, "webUrl"),
            u(&lib, "id")
        ));
    }
    Ok(lib)
}

async fn starting_folder(cx: &Cx, lib: &Value, folder: &url::Url, trim: bool) -> Result<String> {
    cx.verbose_log("Getting starting folder id...");
    let mut rel = drive::strip_drive_url(folder.as_str(), s(lib, "webUrl"));
    if trim {
        rel = rel.trim_end_matches('/').to_string();
    }
    let r = Request::get(format!(
        "{GRAPH}/v1.0/drives/{}/root{}?$select=id",
        u(lib, "id"),
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

fn conflict_query(cx: &Cx) -> String {
    match cx.s("nameConflictBehavior").filter(|b| b != "fail") {
        Some(b) => format!("?@microsoft.graph.conflictBehavior={b}"),
        None => String::new(),
    }
}

pub async fn copy(cx: &mut Cx) -> Result<()> {
    let (web, src, tgt) = (cx.u("webUrl"), cx.u("sourceUrl"), cx.u("targetUrl"));
    let source_path = absolute(&web, &src);
    let dest = absolute(&web, &tgt);
    cx.verbose_log(&format!("Copying file '{source_path}' to '{dest}'..."));
    let folder = drive::parse(&source_path)?;
    let site_id = drive::site_id_by_ms_graph(&web, Some(cx)).await?;
    let lib = library(cx, &site_id, &folder, &src, false).await?;
    let item = starting_folder(cx, &lib, &folder, true).await?;
    let copy_url = format!(
        "{GRAPH}/v1.0/sites/{site_id}/drives/{}/items/{item}/copy{}",
        u(&lib, "id"),
        conflict_query(cx)
    );
    let tsite = urlutil::get_target_site_absolute_url(&web, &tgt);
    let tsite_id = drive::site_id_by_ms_graph(&tsite, Some(cx)).await?;
    let tfolder = drive::parse(&absolute(&tsite, &tgt))?;
    let tlib = library(cx, &tsite_id, &tfolder, &tgt, false).await?;
    let titem = starting_folder(cx, &tlib, &tfolder, true).await?;
    let mut data =
        obj! { "parentReference": obj! { "driveId": ?tlib.get("id").cloned(), "id": titem } };
    if let Some(n) = cx.s("newName") {
        data.as_object_mut()
            .map(|o| o.insert("name".into(), Value::from(new_name(&source_path, &n))));
    }
    Request::post(copy_url)
        .graph_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let mut web = cx.u("webUrl");
    if !web.ends_with('/') {
        web.push('/');
    }
    let fv = cx.u("folderUrl");
    let fv = fv.strip_suffix('/').unwrap_or(&fv);
    let folder =
        drive::parse(&web).and_then(|b| b.join(fv).map_err(|_| Error::js("Invalid URL")))?;
    cx.verbose_log("Getting site id...");
    let site = drive::parse(&cx.u("webUrl"))?;
    let r = Request::get(format!(
        "{GRAPH}/v1.0/sites/{}:{}?$select=id",
        eqp(&drive::host(&site)),
        site.path()
    ))
    .graph_json()
    .data()
    .await?;
    let site_id = u(&r, "id");
    cx.verbose_log(&format!("Site id: {site_id}"));
    let lib = library(cx, &site_id, &folder, &cx.u("folderUrl"), true).await?;
    let drive_id = u(&lib, "id");
    let folder_id = starting_folder(cx, &lib, &folder, false).await?;
    cx.verbose_log(&format!("Starting folder id: {folder_id}"));
    cx.verbose_log("Loading folders to get files from...");
    let mut folders = vec![folder_id.clone()];
    if cx.bool("recursive") {
        // breadth-first, as the concurrent upstream crawl appends level by level
        let mut queue = vec![folder_id];
        while let Some(f) = queue.first().cloned() {
            queue.remove(0);
            let r = Request::get(format!("{GRAPH}/v1.0/drives/{drive_id}/items('{f}')/children?$filter=folder ne null&$select=id")).graph_json().data().await?;
            let ids: Vec<String> = arr(&r, "value").iter().map(|v| u(v, "id")).collect();
            folders.extend(ids.iter().cloned());
            queue.extend(ids);
        }
    }
    cx.debug_log(&format!(
        "Folders to get files from: {}",
        folders.join(", ")
    ));
    cx.verbose_log("Loading files from folders...");
    let mut files = Vec::new();
    for f in &folders {
        files.extend(
            odata::get_all_items(&format!(
                "{GRAPH}/v1.0/drives/{drive_id}/items/{f}/children"
            ))
            .await?,
        );
    }
    let mut files: Vec<Value> = files
        .into_iter()
        .filter(|i| i.get("file").is_some_and(truthy))
        .collect();
    for f in files.iter_mut() {
        let n = f
            .get("lastModifiedBy")
            .and_then(|l| l.get("user"))
            .and_then(|u| u.get("displayName"))
            .cloned();
        if let Some(o) = f.as_object_mut() {
            match n {
                Some(n) => o.insert("lastModifiedByUser".into(), n),
                None => o.shift_remove("lastModifiedByUser"),
            };
        }
    }
    cx.log(Value::Array(files))
}

async fn drive_and_item(cx: &Cx, web: &str, folder: &str) -> Result<(String, String)> {
    let site_id = drive::site_id_by_ms_graph(web, Some(cx)).await?;
    let url = drive::parse(folder)?;
    let d = drive::drive_by_url(&site_id, &url, Some(cx)).await?;
    let item = drive::drive_item_id(&d, &url, Some(cx)).await?;
    Ok((u(&d, "id"), item))
}

pub async fn r#move(cx: &mut Cx) -> Result<()> {
    let (web, src, tgt) = (cx.u("webUrl"), cx.u("sourceUrl"), cx.u("targetUrl"));
    let source_path = absolute(&web, &src);
    let dest = absolute(&web, &tgt);
    let (drive_id, item_id) = drive_and_item(cx, &web, &source_path).await?;
    let tsite = urlutil::get_target_site_absolute_url(&web, &tgt);
    let tfolder = absolute(&tsite, &tgt);
    let (tdrive, titem) = drive_and_item(cx, &tsite, &tfolder).await?;
    let same = drive_id == tdrive;
    let api = if same {
        format!("{GRAPH}/v1.0/drives/{drive_id}/items/{item_id}")
    } else {
        format!("{GRAPH}/v1.0/drives/{drive_id}/items/{item_id}/copy")
    };
    let url = format!("{api}{}", conflict_query(cx));
    let mut data = obj! { "parentReference": obj! { "driveId": tdrive, "id": titem } };
    if let Some(n) = cx.s("newName") {
        data.as_object_mut()
            .map(|o| o.insert("name".into(), Value::from(new_name(&source_path, &n))));
    }
    cx.verbose_log(&format!("Moving file '{source_path}' to '{dest}'..."));
    if same {
        Request::patch(url).graph_json().json(data).send().await?;
        return Ok(());
    }
    let res = Request::post(url).graph_json().json(data).send().await?;
    let monitor = res.header("location").unwrap_or("undefined").to_string();
    loop {
        let r = Request::get(monitor.clone()).json_response().data().await?;
        match s(&r, "status") {
            "completed" => {
                cx.verbose_log("Copy operation completed succesfully. Returning...");
                break;
            }
            "failed" => {
                return Err(Error::Thrown(
                    r.get("error")
                        .and_then(|e| e.get("message"))
                        .cloned()
                        .unwrap_or(Value::Null),
                ));
            }
            _ => {
                cx.verbose_log("Still copying. Retrying in 10 seconds...");
                util::sleep_ms(10_000).await;
            }
        }
    }
    Request::delete(format!("{GRAPH}/v1.0/drives/{drive_id}/items/{item_id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

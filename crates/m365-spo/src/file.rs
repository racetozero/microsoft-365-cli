//! `spo file *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::url as urlutil;

pub(crate) fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

pub(crate) fn opt(cx: &Cx, k: &str) -> String {
    cx.val(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

pub(crate) fn first(cx: &Cx, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|k| ne(cx, k))
        .unwrap_or_else(|| "undefined".into())
}

fn folder_files(web: &str, folder: &str, file: &str) -> String {
    format!(
        "{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')/Files('{}')",
        eqp(folder),
        eqp(file)
    )
}

/// `${err}` of a rejected value.
fn js_display(e: &Error) -> String {
    to_js_string(&e.clone().uncaught().command_message())
}

const CHUNK: u64 = 250 * 1024 * 1024;

pub async fn add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let folder = urlutil::get_server_relative_path(&web, &cx.u("folder"));
    let full = util::resolve(&cx.u("path"));
    let name = cx.s("fileName").unwrap_or_else(|| {
        std::path::Path::new(&full)
            .file_name()
            .map(|f| f.to_string_lossy().into_owned())
            .unwrap_or_default()
    });
    cx.verbose_log(&format!("file name: {name}..."));
    cx.verbose_log(&format!("folder path: {folder}..."));
    let mut checked_out = false;
    let r = add_inner(cx, &web, &folder, &full, &name, &mut checked_out).await;
    if let Err(e) = r {
        if checked_out
            && let Err(e2) = Request::post(format!(
                "{}/UndoCheckOut()",
                folder_files(&web, &folder, &name)
            ))
            .spo_json()
            .send()
            .await
        {
            cx.verbose_log("Could not rollback file checkout");
            if cx.verbose() {
                m365_core::log::log_stderr_value(&e2.command_message());
            }
        }
        return Err(e);
    }
    Ok(())
}

async fn add_inner(
    cx: &Cx,
    web: &str,
    folder: &str,
    full: &str,
    name: &str,
    checked_out: &mut bool,
) -> Result<()> {
    if !cx.bool("overwrite") {
        let exists = Request::get(format!(
            "{web}/_api/Web/GetFileByServerRelativePath(DecodedUrl='{}')?$select=Exists",
            eqp(&format!("{folder}/{name}"))
        ))
        .spo_json()
        .send()
        .await;
        if exists.is_ok() {
            return Err(Error::msg(format!(
                "File '{name}' already exists in folder '{folder}'. To overwrite the file, use the --overwrite option."
            )));
        }
        cx.verbose_log(&format!(
            "File '{name}' does not exist in folder '{folder}'. Proceeding with upload."
        ));
    }
    cx.verbose_log("Check if the specified folder exists.");
    if Request::get(format!(
        "{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')",
        eqp(folder)
    ))
    .spo_json()
    .send()
    .await
    .is_err()
    {
        sp::ensure_folder(cx, web, folder).await?;
    }
    if cx.bool("checkOut") {
        Request::get(folder_files(web, folder, name))
            .spo_json()
            .send()
            .await?;
        Request::post(format!("{}/CheckOut()", folder_files(web, folder, name)))
            .spo_json()
            .send()
            .await?;
        *checked_out = true;
    }
    cx.verbose_log(&format!("Upload file to site {web}..."));
    let size = std::fs::metadata(full)
        .map(|m| m.len())
        .map_err(|_| Error::js(format!("ENOENT: no such file or directory, stat '{full}'")))?;
    cx.verbose_log(&format!("File size is {size} bytes"));
    let result = if size > CHUNK {
        let chunks = size.div_ceil(CHUNK);
        cx.verbose_log(&format!("Uploading {size} bytes in {chunks} chunks..."));
        let id = util::new_guid();
        Request::post(format!("{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')/Files/GetByPathOrAddStub(DecodedUrl='{}')/StartUpload(uploadId=guid'{id}')", eqp(folder), eqp(name))).spo_json().send().await?;
        match upload_chunks(cx, web, folder, name, full, &id, size).await {
            Ok(r) => {
                cx.verbose_log(&format!(
                    "Finished uploading {size} bytes in {chunks} chunks"
                ));
                r
            }
            Err(err) => {
                cx.verbose_log("Cancelling upload session due to error...");
                if let Err(e) = Request::post(format!(
                    "{}/cancelupload(uploadId=guid'{id}')",
                    folder_files(web, folder, name)
                ))
                .spo_json()
                .send()
                .await
                {
                    cx.verbose_log(&format!(
                        "Failed to cancel upload session: {}",
                        js_display(&e)
                    ));
                    return Err(e);
                }
                cx.verbose_log(&format!(
                    "Failed to cancel upload session: {}",
                    js_display(&err)
                ));
                return Err(err);
            }
        }
    } else {
        let body = std::fs::read(full)
            .map_err(|_| Error::js(format!("ENOENT: no such file or directory, open '{full}'")))?;
        let len = body.len();
        Request::post(format!("{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')/Files/Add(url='{}', overwrite=true)", eqp(folder), eqp(name)))
            .spo_json()
            .header("content-length", len.to_string())
            .bytes(body)
            .data()
            .await?
    };
    let ct = ne(cx, "contentType");
    let mut settings = Value::Null;
    if ct.is_some() || cx.bool("publish") || cx.bool("approve") {
        cx.verbose_log(
            "Getting list details in order to get its available content types afterwards...",
        );
        settings = Request::get(format!("{}/ListItemAllFields/ParentList?$Select=Id,EnableModeration,EnableVersioning,EnableMinorVersions", folder_files(web, folder, name))).spo_json().data().await?;
        if let Some(c) = &ct {
            cx.verbose_log("Getting list of available content types ...");
            let r = Request::get(format!(
                "{web}/_api/web/lists('{}')/contenttypes?$select=Name,Id",
                u(&settings, "Id")
            ))
            .spo_json()
            .data()
            .await?;
            let mut found = false;
            for t in need_arr(&r, "value", "iterator")? {
                let id = t.get("Id").ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'StringValue')")
                })?;
                if id.get("StringValue").and_then(Value::as_str) == Some(c)
                    || t.get("Name").and_then(Value::as_str) == Some(c)
                {
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(Error::msg(format!(
                    "Specified content type '{c}' doesn't exist on the target list"
                )));
            }
        }
    }
    const EXCLUDE: [&str; 22] = [
        "webUrl",
        "folder",
        "path",
        "contentType",
        "checkOut",
        "checkInComment",
        "approve",
        "approveComment",
        "publish",
        "publishComment",
        "overwrite",
        "fileName",
        "debug",
        "verbose",
        "output",
        "query",
        "_",
        "u",
        "p",
        "f",
        "o",
        "c",
    ];
    let mut fields: Vec<Value> = cx
        .opts
        .iter()
        .filter(|(k, _)| !EXCLUDE.contains(&k.as_str()))
        .map(|(k, v)| obj! { "FieldName": k.as_str(), "FieldValue": to_js_string(v) })
        .collect();
    if let Some(c) = &ct {
        fields.push(obj! { "FieldName": "ContentType", "FieldValue": c.as_str() });
    }
    let comment = cx.s("checkInComment").unwrap_or_default();
    if !fields.is_empty() {
        cx.verbose_log(&format!(
            "Validate and update list item values for file {name}"
        ));
        cx.verbose_log("ValidateUpdateListItem will perform the check-in ...");
        let r = Request::post(format!("{}/ListItemAllFields/ValidateUpdateListItem()", folder_files(web, folder, name)))
            .spo_json()
            .json(obj! { "formValues": fields, "bNewDocumentUpdate": true, "checkInComment": comment.as_str() })
            .data()
            .await?;
        let vals = need_arr(&r, "value", "iterator")?;
        if vals
            .iter()
            .any(|f| f.get("HasException").is_some_and(truthy))
        {
            return Err(Error::msg(format!(
                "Update field value error: {}",
                json::stringify(&Value::Array(vals.to_vec()))
            )));
        }
    } else if *checked_out {
        Request::post(format!(
            "{}/CheckIn(comment='{}',checkintype=0)",
            folder_files(web, folder, name),
            eqp(&comment)
        ))
        .spo_json()
        .send()
        .await?;
    }
    if cx.bool("approve") {
        cx.verbose_log(&format!("Approve file {name}"));
        Request::post(format!(
            "{}/approve(comment='{}')",
            folder_files(web, folder, name),
            eqp(&cx.s("approveComment").unwrap_or_default())
        ))
        .spo_json()
        .send()
        .await?;
    } else if cx.bool("publish") {
        if settings.get("EnableModeration").is_some_and(truthy)
            && settings.get("EnableMinorVersions").is_some_and(truthy)
        {
            return Err(Error::msg(
                "The file cannot be published without approval. Moderation for this list is enabled. Use the --approve option instead of --publish to approve and publish the file",
            ));
        }
        cx.verbose_log(&format!("Publish file {name}"));
        Request::post(format!(
            "{}/publish(comment='{}')",
            folder_files(web, folder, name),
            eqp(&cx.s("publishComment").unwrap_or_default())
        ))
        .spo_json()
        .send()
        .await?;
    }
    cx.log(result)
}

async fn upload_chunks(
    cx: &Cx,
    web: &str,
    folder: &str,
    name: &str,
    path: &str,
    id: &str,
    size: u64,
) -> Result<Value> {
    use std::io::{Read, Seek, SeekFrom};
    let mut retries = 5;
    let mut pos: u64 = 0;
    loop {
        let read = (|| -> std::io::Result<Vec<u8>> {
            let mut f = std::fs::File::open(path)?;
            f.seek(SeekFrom::Start(pos))?;
            let mut buf = Vec::with_capacity(CHUNK as usize);
            f.take(CHUNK).read_to_end(&mut buf)?;
            Ok(buf)
        })();
        let buf = match read {
            Ok(b) => b,
            Err(e) => {
                retries -= 1;
                if retries > 0 {
                    cx.verbose_log(&format!("Retrying to read chunk due to error: Error: {e}"));
                    continue;
                }
                return Err(Error::js(e.to_string()));
            }
        };
        let offset = pos;
        let n = buf.len() as u64;
        pos += n;
        let last = pos >= size;
        let r = Request::post(format!(
            "{}/{}Upload(uploadId=guid'{id}',fileOffset={offset})",
            folder_files(web, folder, name),
            if last { "Finish" } else { "Continue" }
        ))
        .spo_json()
        .header("content-length", n.to_string())
        .bytes(buf)
        .data()
        .await;
        match r {
            Ok(v) => {
                cx.verbose_log(&format!(
                    "Uploaded {pos} of {size} bytes ({}%)",
                    (100.0 * pos as f64 / size as f64).round()
                ));
                if last {
                    return Ok(v);
                }
            }
            Err(e) => {
                retries -= 1;
                if retries > 0 {
                    cx.verbose_log(&format!(
                        "Retrying to upload chunk due to error: {}",
                        js_display(&e)
                    ));
                    pos -= n;
                } else {
                    return Err(e);
                }
            }
        }
    }
}

fn by_id_or_url(cx: &Cx, id: &str, url: &str) -> String {
    let web = cx.u("webUrl");
    match ne(cx, id) {
        Some(i) => format!("{web}/_api/web/GetFileById('{}')", eqp(&i)),
        None => format!(
            "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(
                &web,
                &cx.s(url).unwrap_or_default()
            ))
        ),
    }
}

async fn archive(cx: &Cx, verb: &str, method: &str) -> Result<()> {
    let web = cx.u("webUrl");
    if cx.val("verbose").is_some_and(truthy) {
        let who = first(cx, &["url", "id"]);
        cx.stderr(&if verb == "Archiving" {
            format!("{verb} file {who} at site {web}...")
        } else {
            format!("{verb} file '{who}' at site '{web}'...")
        });
    }
    let info = Request::get(format!(
        "{}?$select=ListId,ListItemAllFields/Id&$expand=ListItemAllFields",
        by_id_or_url(cx, "id", "url")
    ))
    .spo_json()
    .data()
    .await?;
    let item = dig(&info, &["ListItemAllFields", "Id"])?
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    Request::post(format!(
        "{web}/_api/Lists(guid'{}')/items({item})/{method}",
        u(&info, "ListId")
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn archive_cmd(cx: &mut Cx) -> Result<()> {
    if !cx.bool("force") && !cx.confirm("Are you sure you would like to archive this file? You will be able to reactivate it instantly for the first 7 days. After that, it will take up to 24 hours to reactivate.")? {
        return Ok(());
    }
    archive(cx, "Archiving", "Archive").await
}

pub async fn checkin(cx: &mut Cx) -> Result<()> {
    let ty = match cx.s("type").map(|t| t.to_lowercase()).as_deref() {
        Some("minor") => 0,
        Some("overwrite") => 2,
        _ => 1,
    };
    let comment = ne(cx, "comment").map(|c| eqp(&c)).unwrap_or_default();
    let web = cx.u("webUrl");
    let mut url = String::new();
    if let Some(i) = ne(cx, "id") {
        url = format!(
            "{web}/_api/web/GetFileById('{}')/checkin(comment='{comment}',checkintype={ty})",
            eqp(&i)
        );
    }
    if let Some(u) = ne(cx, "url") {
        url = format!(
            "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/checkin(comment='{comment}',checkintype={ty})",
            eqp(&urlutil::get_server_relative_path(&web, &u))
        );
    }
    Request::post(url).spo_json().send().await?;
    Ok(())
}

pub async fn checkout(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let mut url = String::new();
    if let Some(i) = ne(cx, "id") {
        url = format!("{web}/_api/web/GetFileById('{}')/checkout", eqp(&i));
    }
    if let Some(u) = ne(cx, "url") {
        url = format!(
            "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/checkout",
            eqp(&urlutil::get_server_relative_path(&web, &u))
        );
    }
    Request::post(url).spo_json().send().await?;
    Ok(())
}

pub async fn checkout_undo(cx: &mut Cx) -> Result<()> {
    let who = first(cx, &["fileId", "fileUrl"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to undo the checkout for file {who}?"),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Undoing checkout for file {who} on web {web}"));
    let mut url = format!("{web}/_api/web/");
    if let Some(i) = ne(cx, "fileId") {
        url.push_str(&format!("getFileById('{i}')"));
    } else if let Some(f) = ne(cx, "fileUrl") {
        url.push_str(&format!(
            "getFileByServerRelativePath(DecodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(&web, &f))
        ));
    }
    Request::post(format!("{url}/undocheckout"))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

fn absolute(web: &str, u: &str) -> String {
    let r = if u.starts_with("https://") {
        u.to_string()
    } else {
        urlutil::get_absolute_url(web, u)
    };
    urlutil::remove_trailing_slashes(&r)
}

async fn copy_or_move(cx: &Cx, operation: &str) -> Result<()> {
    let web = cx.u("webUrl");
    let src_rel = match ne(cx, "sourceUrl") {
        Some(s) => urlutil::get_server_relative_path(&web, &s),
        None => {
            let id = opt(cx, "sourceId");
            cx.verbose_log(&format!(
                "Retrieving server-relative path for file with ID '{id}'..."
            ));
            let r = Request::get(format!(
                "{web}/_api/Web/GetFileById('{id}')/ServerRelativePath"
            ))
            .spo_json()
            .data()
            .await?;
            match r.get("DecodedUrl") {
                Some(v) => to_js_string(v),
                None => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading 'startsWith')",
                    ));
                }
            }
        }
    };
    let src = absolute(&web, &src_rel);
    let target = cx.u("targetUrl");
    let dest = absolute(&web, &target);
    let (verb, noun) = if operation == "move" {
        ("Moving", "move")
    } else {
        ("Copying", "copy")
    };
    cx.verbose_log(&format!("{verb} file '{src_rel}' to '{target}'..."));
    let mut new_name = ne(cx, "newName");
    if let Some(n) = &mut new_name
        && !n.contains('.')
    {
        let ext = src_rel
            .rfind('.')
            .map(|i| &src_rel[i..])
            .unwrap_or(&src_rel);
        n.push_str(ext);
    }
    let ncb = match cx
        .s("nameConflictBehavior")
        .map(|s| s.to_lowercase())
        .as_deref()
    {
        Some("replace") => 1,
        Some("rename") => 2,
        _ => 0,
    };
    let options = obj! {
        "NameConflictBehavior": ncb, "AllowSchemaMismatch": true,
        "BypassSharedLock": cx.bool("bypassSharedLock"),
        "IgnoreVersionHistory": cx.bool("ignoreVersionHistory"),
        "IncludeItemPermissions": cx.bool("withItemPermissions"),
        "CustomizedItemName": ?new_name.map(|n| vec![n]),
        "SameWebCopyMoveOptimization": true, "IsMoveMode": operation == "move",
    };
    let job = sp::create_copy_job(&web, &src, &dest, options).await?;
    if cx.bool("skipWait") {
        return Ok(());
    }
    cx.verbose_log(&format!("Waiting for the {noun} job to complete..."));
    let res = sp::copy_job_result(&web, &job).await?;
    cx.verbose_log("Getting information about the destination file...");
    let rel = match res.get("TargetObjectSiteRelativeUrl") {
        Some(Value::String(s)) => s.clone(),
        _ => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'substring')",
            ));
        }
    };
    let folder = format!("/{}", rel.rfind('/').map(|i| &rel[..i]).unwrap_or(""));
    let abs_web = match dest.to_lowercase().rfind(&folder.to_lowercase()) {
        Some(i) => dest[..i].to_string(),
        None => String::new(),
    };
    cx.log(
        Request::get(format!(
            "{abs_web}/_api/Web/GetFileById('{}')",
            u(&res, "TargetObjectUniqueId")
        ))
        .spo_json()
        .data()
        .await?,
    )
}

pub async fn copy(cx: &mut Cx) -> Result<()> {
    copy_or_move(cx, "copy").await
}

pub async fn move_cmd(cx: &mut Cx) -> Result<()> {
    copy_or_move(cx, "move").await
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Retrieving file from site {web}..."));
    let mut url = String::new();
    if let Some(i) = ne(cx, "id") {
        url = format!("{web}/_api/web/GetFileById('{}')", eqp(&i));
    } else if ne(cx, "url").is_some() {
        url = format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl=@f)");
    }
    let (as_file, as_string, as_item) = (
        cx.bool("asFile"),
        cx.bool("asString"),
        cx.bool("asListItem"),
    );
    let mut opts = if as_item {
        "?$expand=ListItemAllFields".to_string()
    } else if as_file || as_string {
        "/$value".to_string()
    } else {
        String::new()
    };
    if let Some(u) = ne(cx, "url") {
        opts.push(if opts.contains('?') { '&' } else { '?' });
        opts.push_str(&format!(
            "@f='{}'",
            eqp(&urlutil::get_server_relative_path(&web, &u))
        ));
    }
    let mut req = Request::get(format!("{url}{opts}")).accept("application/json;odata=nometadata");
    req = if as_file {
        req.bytes_response()
    } else if as_string {
        req
    } else {
        req.json_response()
    };
    let res = req.send().await?;
    if as_file && let Some(p) = ne(cx, "path") {
        std::fs::write(&p, &res.bytes).map_err(|e| Error::js(e.to_string()))?;
        cx.verbose_log(&format!("File saved to path {p}"));
        return Ok(());
    }
    if as_string {
        return cx.log(Value::from(res.text()));
    }
    let file = res.data();
    let mut props = file.clone();
    if cx.bool("withPermissions") {
        let r = Request::get(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/ListItemAllFields/RoleAssignments?$expand=Member,RoleDefinitionBindings", u(&file, "ServerRelativeUrl")))
            .accept("application/json;odata=nometadata")
            .json_response()
            .data()
            .await?;
        let mut v = need_arr(&r, "value", "forEach")?.to_vec();
        for ra in v.iter_mut() {
            let mut b = match ra.get("RoleDefinitionBindings") {
                Some(Value::Array(a)) => a.clone(),
                _ => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading 'forEach')",
                    ));
                }
            };
            sp::set_friendly_permissions(&mut b)?;
            ra["RoleDefinitionBindings"] = Value::Array(b);
        }
        if let Some(o) = props.as_object_mut() {
            o.insert("RoleAssignments".into(), Value::Array(v.clone()));
        }
        if as_item {
            match props
                .get_mut("ListItemAllFields")
                .and_then(Value::as_object_mut)
            {
                Some(li) => {
                    li.insert("RoleAssignments".into(), Value::Array(v));
                }
                None => {
                    return Err(Error::js(
                        "Cannot set properties of undefined (setting 'RoleAssignments')",
                    ));
                }
            }
        }
    }
    if as_item {
        match props.get_mut("ListItemAllFields") {
            Some(Value::Object(li)) => {
                li.shift_remove("ID");
            }
            _ => return Err(Error::js("Cannot convert undefined or null to object")),
        }
        return match props.get("ListItemAllFields") {
            Some(v) => cx.log(v.clone()),
            None => cx.log_undefined(),
        };
    }
    cx.log(props)
}

fn files<'a>(
    cx: &'a Cx,
    folder: String,
    sel: &'a (Vec<String>, Vec<String>),
    skip: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Value>>> + Send + 'a>> {
    Box::pin(async move {
        if cx.verbose() {
            let page = (skip as f64 / 5000.0).ceil() as usize + 1;
            cx.stderr(&format!(
                "Retrieving files in folder '{folder}'{}...",
                if page > 1 {
                    format!(", page {page}")
                } else {
                    String::new()
                }
            ));
        }
        let web = cx.u("webUrl");
        let srv = urlutil::get_server_relative_path(&web, &folder);
        let mut q = vec![format!("$skip={skip}"), "$top=5000".to_string()];
        if !sel.1.is_empty() {
            q.push(format!("$expand={}", sel.1.join(",")));
        }
        if !sel.0.is_empty() {
            q.push(format!("$select={}", sel.0.join(",")));
        }
        if let Some(f) = ne(cx, "filter") {
            q.push(format!("$filter={f}"));
        }
        let r = Request::get(format!(
            "{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')/Files?{}",
            eqp(&srv),
            q.join("&")
        ))
        .spo_json()
        .data()
        .await?;
        let mut out = need_arr(&r, "value", "forEach")?.to_vec();
        if out.len() == 5000 {
            out.extend(files(cx, folder, sel, skip + 5000).await?);
        }
        Ok(out)
    })
}

fn subfolders(
    cx: &Cx,
    folder: String,
    skip: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>>> + Send + '_>> {
    Box::pin(async move {
        if cx.verbose() {
            let page = (skip as f64 / 5000.0).ceil() as usize + 1;
            cx.stderr(&format!(
                "Retrieving folders in folder '{folder}'{}...",
                if page > 1 {
                    format!(", page {page}")
                } else {
                    String::new()
                }
            ));
        }
        let web = cx.u("webUrl");
        let srv = urlutil::get_server_relative_path(&web, &folder);
        let r = Request::get(format!("{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')/Folders?$skip={skip}&$top=5000&$select=ServerRelativeUrl", eqp(&srv))).spo_json().data().await?;
        let items = need_arr(&r, "value", "iterator")?.to_vec();
        let mut out = Vec::new();
        for f in &items {
            let s = u(f, "ServerRelativeUrl");
            out.push(s.clone());
            out.extend(subfolders(cx, s, 0).await?);
        }
        if items.len() == 5000 {
            out.extend(subfolders(cx, folder, skip + 5000).await?);
        }
        Ok(out)
    })
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let folder = cx.u("folderUrl");
    cx.verbose_log(&format!(
        "Retrieving all files in folder '{folder}' at site '{}'{}...",
        cx.u("webUrl"),
        if cx.bool("recursive") {
            " (recursive)"
        } else {
            ""
        }
    ));
    let fields = ne(cx, "fields");
    let (mut s, mut e): (Vec<String>, Vec<String>) = (Vec::new(), Vec::new());
    if cx.str("output").as_deref() == Some("text") && fields.is_none() {
        s = vec!["UniqueId".into(), "Name".into(), "ServerRelativeUrl".into()];
    }
    if let Some(f) = fields {
        for x in f.split(',') {
            let parts: Vec<&str> = x.trim().split('/').collect();
            if parts.len() > 1 {
                e.push(parts[0].to_string());
            }
            s.push(x.trim().to_string());
        }
    }
    let dedupe = |v: Vec<String>| {
        v.into_iter().fold(Vec::new(), |mut acc: Vec<String>, x| {
            if !acc.contains(&x) {
                acc.push(x);
            }
            acc
        })
    };
    let sel = (dedupe(s), dedupe(e));
    let folders = if cx.bool("recursive") {
        let mut f = subfolders(cx, folder.clone(), 0).await?;
        f.push(folder);
        f
    } else {
        vec![folder]
    };
    let mut all = Vec::new();
    for f in folders {
        all.extend(files(cx, f, &sel, 0).await?);
    }
    if sel
        .0
        .iter()
        .any(|p| p.to_lowercase().contains("listitemallfields"))
    {
        for f in all.iter_mut() {
            if let Some(o) = f
                .get_mut("ListItemAllFields")
                .and_then(Value::as_object_mut)
            {
                o.shift_remove("ID");
            }
        }
    }
    cx.log(Value::Array(all))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let permanent = cx.bool("permanent");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to {} the file {} located in site {web}?",
            if permanent {
                "permanently remove"
            } else {
                "recycle"
            },
            first(cx, &["id", "url"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing file in site at {web}..."));
    let mut url = match ne(cx, "id") {
        Some(i) => format!("{web}/_api/web/GetFileById(guid'{}')", eqp(&i)),
        None => format!(
            "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(
                &web,
                &cx.s("url").unwrap_or_default()
            ))
        ),
    };
    if !permanent {
        url.push_str("/recycle()");
    }
    let mut r = Request::post(url)
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json();
    if cx.bool("bypassSharedLock") {
        r = r.header("Prefer", "bypass-shared-lock");
    }
    r.send().await?;
    Ok(())
}

pub async fn rename(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let source = cx.u("sourceUrl");
    let target = cx.u("targetFileName");
    let srv = urlutil::get_server_relative_path(&web, &source);
    Request::get(format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')?$select=UniqueId",
        eqp(&srv)
    ))
    .spo_json()
    .send()
    .await?;
    if cx.bool("force") {
        let dir = source.rfind('/').map(|i| &source[..i]).unwrap_or("");
        let o = obj! { "webUrl": web.as_str(), "url": format!("{}/{target}", urlutil::get_server_relative_path(&web, dir)), "force": true, "debug": cx.debug(), "verbose": cx.verbose() };
        if let Err(e) = m365_core::runner::run_nested(
            "spo file remove",
            o.as_object().cloned().unwrap_or_default(),
            false,
        )
        .await
            && !e.js_message().contains("does not exist")
        {
            return Err(e);
        }
    }
    let r = Request::post(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/ListItemAllFields/ValidateUpdateListItem()", eqp(&srv)))
        .spo_json()
        .json(obj! { "formValues": vec![obj! { "FieldName": "FileLeafRef", "FieldValue": target }] })
        .data()
        .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

async fn item_info(cx: &Cx) -> Result<(String, String)> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving list and item information for file '{}' in site at {web}...",
        first(cx, &["fileId", "fileUrl"])
    ));
    let seg = match ne(cx, "fileId") {
        Some(i) => format!("GetFileById('{i}')"),
        None => format!(
            "GetFileByServerRelativePath(DecodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(
                &web,
                &cx.s("fileUrl").unwrap_or_default()
            ))
        ),
    };
    let r = Request::get(format!(
        "{web}/_api/web/{seg}?$expand=ListItemAllFields,ListItemAllFields/ParentList/RootFolder&$select=ServerRelativeUrl,ListItemAllFields/ParentList/RootFolder/ServerRelativeUrl,ListItemAllFields/Id"
    ))
    .spo_json()
    .data()
    .await?;
    let id = dig(&r, &["ListItemAllFields", "Id"])?
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let list = dig(
        &r,
        &[
            "ListItemAllFields",
            "ParentList",
            "RootFolder",
            "ServerRelativeUrl",
        ],
    )?
    .map(to_js_string)
    .unwrap_or_else(|| "undefined".into());
    Ok((id, list))
}

async fn label(cx: &Cx, name: &str) -> Result<()> {
    let web = cx.u("webUrl");
    let (id, list) = item_info(cx).await?;
    let url = url::Url::parse(&web).map_err(|_| Error::js("Invalid URL"))?;
    let abs = urlutil::url_combine(
        &format!("{}://{}", url.scheme(), url.host_str().unwrap_or("")),
        &list,
    );
    if let Some(a) = ne(cx, "assetId").filter(|_| !name.is_empty()) {
        Request::post(format!(
            "{web}/_api/web/GetList(@listUrl)/items({id})/ValidateUpdateListItem()?@listUrl='{}'",
            eqp(&list)
        ))
        .spo_json()
        .json(
            obj! { "formValues": vec![obj! { "FieldName": "ComplianceAssetId", "FieldValue": a }] },
        )
        .send()
        .await?;
    }
    let n = json::parse_int(&id).map(Value::from).unwrap_or(Value::Null);
    sp::set_compliance_tag_on_items(
        cx,
        &web,
        name,
        &abs,
        vec![n],
        cx.val("verbose").is_some_and(truthy),
    )
    .await
}

pub async fn retentionlabel_ensure(cx: &mut Cx) -> Result<()> {
    label(cx, &opt(cx, "name")).await
}

pub async fn retentionlabel_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the retentionlabel from file {} located in site {}?",
            first(cx, &["fileId", "fileUrl"]),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    label(cx, "").await
}

pub async fn unarchive(cx: &mut Cx) -> Result<()> {
    if !cx.bool("force") && !cx.confirm("Reactivation could take up to 24 hours. Files that are reactivated cannot be archived again for a period of time. Are you sure you would like to unarchive this file?")? {
        return Ok(());
    }
    archive(cx, "Unarchiving", "UnArchive").await
}

// ---- role assignments ----------------------------------------------------------

async fn file_srv(cx: &Cx) -> Result<String> {
    let web = cx.u("webUrl");
    match ne(cx, "fileUrl") {
        Some(f) => Ok(urlutil::get_server_relative_path(&web, &f)),
        None => Ok(u(
            &sp::file_by_id(cx, &web, &opt(cx, "fileId")).await?,
            "ServerRelativeUrl",
        )),
    }
}

fn item_url(web: &str, srv: &str) -> String {
    format!(
        "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')/ListItemAllFields",
        eqp(srv)
    )
}

async fn entra_principal(cx: &Cx) -> Result<String> {
    cx.verbose_log("Retrieving group information...");
    let g = match ne(cx, "entraGroupId") {
        Some(i) => m365_core::ut::entra::group::by_id(&i, None).await?,
        None => {
            m365_core::ut::entra::group::by_display_name(&opt(cx, "entraGroupName"), None).await?
        }
    };
    Ok(u(&sp::ensure_entra_group(&cx.u("webUrl"), &g).await?, "Id"))
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding role assignment to file in site at {web}..."
    ));
    let srv = file_srv(cx).await?;
    let role = match ne(cx, "roleDefinitionName") {
        Some(n) => u(&sp::role_definition_by_name(cx, &web, &n).await?, "Id"),
        None => opt(cx, "roleDefinitionId"),
    };
    let p = if let Some(upn) = ne(cx, "upn") {
        u(&sp::user_by_email(cx, &web, &upn).await?, "Id")
    } else if let Some(g) = ne(cx, "groupName") {
        u(&sp::group_by_name(cx, &web, &g).await?, "Id")
    } else if ne(cx, "entraGroupId").is_some() || ne(cx, "entraGroupName").is_some() {
        entra_principal(cx).await?
    } else {
        opt(cx, "principalId")
    };
    Request::post(format!(
        "{}/roleassignments/addroleassignment(principalid='{p}',roledefid='{role}')",
        item_url(&web, &srv)
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleassignment_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let file = first(cx, &["fileUrl", "fileId"]);
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove role assignment from file {file} from site {web}?"
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing role assignment for {} from file {file}",
        first(cx, &["groupName", "upn"])
    ));
    let srv = file_srv(cx).await?;
    let p = if let Some(g) = ne(cx, "groupName") {
        u(&sp::group_by_name(cx, &web, &g).await?, "Id")
    } else if let Some(upn) = ne(cx, "upn") {
        u(&sp::user_by_email(cx, &web, &upn).await?, "Id")
    } else if ne(cx, "entraGroupId").is_some() || ne(cx, "entraGroupName").is_some() {
        entra_principal(cx).await?
    } else {
        opt(cx, "principalId")
    };
    let srv = urlutil::get_server_relative_path(&web, &srv);
    Request::post(format!(
        "{}/roleassignments/removeroleassignment(principalid='{p}')",
        item_url(&web, &srv)
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_break(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to break the role inheritance of file {} located in site {}?",
            first(cx, &["fileUrl", "fileId"]),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Breaking role inheritance for file {}",
        first(cx, &["fileId", "fileUrl"])
    ));
    let srv = file_srv(cx).await?;
    Request::post(format!(
        "{}/breakroleinheritance({})",
        item_url(&cx.u("webUrl"), &srv),
        !cx.bool("clearExistingPermissions")
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_reset(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to reset the role inheritance of file {} located in site {}?",
            first(cx, &["fileUrl", "fileId"]),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Resetting role inheritance for file {}",
        first(cx, &["fileId", "fileUrl"])
    ));
    let srv = file_srv(cx).await?;
    Request::post(format!(
        "{}/resetroleinheritance",
        item_url(&cx.u("webUrl"), &srv)
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

// ---- sharing -------------------------------------------------------------------

fn principal_type(v: Option<&Value>) -> Option<Value> {
    let n = json::parse_int(&v.map(to_js_string).unwrap_or_default())?;
    Some(Value::from(match n {
        0 => "None",
        1 => "User",
        2 => "DistributionList",
        3 => "SecurityGroup",
        8 => "SharePointGroup",
        15 => "All",
        _ => return None,
    }))
}

pub async fn sharinginfo_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving sharing information report for the file {}",
        first(cx, &["fileId", "fileUrl"])
    ));
    let url = match ne(cx, "fileId") {
        Some(i) => format!(
            "{web}/_api/web/GetFileById('{i}')/?$select=ListItemAllFields/Id,ListItemAllFields/ParentList/Title&$expand=ListItemAllFields/ParentList"
        ),
        None => format!(
            "{web}/_api/web/GetFileByServerRelativePath(decodedUrl='{}')?$select=ListItemAllFields/Id,ListItemAllFields/ParentList/Title&$expand=ListItemAllFields/ParentList",
            eqp(&urlutil::get_server_relative_path(
                &web,
                &cx.s("fileUrl").unwrap_or_default()
            ))
        ),
    };
    let r = Request::get(url).spo_json().data().await?;
    let li = r
        .get("ListItemAllFields")
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'Id')"))?;
    let id = json::parse_int(&u(li, "Id"))
        .map(|n| n.to_string())
        .unwrap_or_else(|| "NaN".into());
    let lib = dig(li, &["ParentList", "Title"])?
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Retrieving sharing information report for the file with item Id  {id}"
    ));
    let res = Request::post(format!("{web}/_api/web/lists/getbytitle('{}')/items({id})/GetSharingInformation?$select=permissionsInformation&$Expand=permissionsInformation", eqp(&lib)))
        .spo_json()
        .data()
        .await?;
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(res);
    }
    let mut out = Vec::new();
    let pi = res
        .get("permissionsInformation")
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'links')"))?;
    let row = |p: &Value| obj! { "SharedWith": ?p.get("name").cloned(), "IsActive": ?p.get("isActive").cloned(), "IsExternal": ?p.get("isExternal").cloned(), "PrincipalType": ?principal_type(p.get("principalType")) };
    for link in need_arr(pi, "links", "forEach")? {
        let ld = link.get("linkDetails").ok_or_else(|| {
            Error::js("Cannot read properties of undefined (reading 'Invitations')")
        })?;
        for inv in need_arr(ld, "Invitations", "forEach")? {
            let p = inv
                .get("invitee")
                .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'name')"))?;
            out.push(row(p));
        }
    }
    for pr in need_arr(pi, "principals", "forEach")? {
        let p = pr
            .get("principal")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'name')"))?;
        out.push(row(p));
    }
    cx.log(Value::Array(out))
}

async fn vroom(cx: &Cx) -> Result<String> {
    let web = cx.u("webUrl");
    let mut url = format!("{web}/_api/web/");
    match ne(cx, "fileUrl") {
        Some(f) => url.push_str(&format!(
            "GetFileByServerRelativePath(decodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(&web, &f))
        )),
        None => url.push_str(&format!("GetFileById('{}')", opt(cx, "fileId"))),
    }
    let d = Request::get(format!("{url}?$select=SiteId,VroomItemId,VroomDriveId"))
        .spo_json()
        .data()
        .await?;
    Ok(format!(
        "https://graph.microsoft.com/v1.0/sites/{}/drives/{}/items/{}",
        u(&d, "SiteId"),
        u(&d, "VroomDriveID"),
        u(&d, "VroomItemID")
    ))
}

pub async fn sharinglink_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Creating a sharing link for file {}...",
        first(cx, &["fileId", "fileUrl"])
    ));
    let base = vroom(cx).await?;
    let b = obj! { "type": ?cx.val("type").cloned(), "expirationDateTime": ?cx.val("expirationDateTime").cloned(), "scope": ?cx.val("scope").cloned() };
    cx.log(
        Request::post(format!("{base}/createLink"))
            .graph_json()
            .json(b)
            .data()
            .await?,
    )
}

pub async fn sharinglink_clear(cx: &mut Cx) -> Result<()> {
    let scope = ne(cx, "scope")
        .map(|s| format!(" with scope {s}"))
        .unwrap_or_default();
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to clear the sharing links of file {}{scope}?",
            first(cx, &["fileUrl", "fileId"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Clearing sharing links for file {}{scope}",
        first(cx, &["fileUrl", "fileId"])
    ));
    let base = vroom(cx).await?;
    let o = obj! { "webUrl": cx.u("webUrl"), "fileId": ?cx.val("fileId").cloned(), "fileUrl": ?cx.val("fileUrl").cloned(), "scope": ?cx.val("scope").cloned(), "debug": cx.debug(), "verbose": cx.verbose() };
    let (out, _) = m365_core::runner::run_nested(
        "spo file sharinglink list",
        o.as_object().cloned().unwrap_or_default(),
        true,
    )
    .await?;
    let links = json::jparse(&out)?;
    for l in links.as_array().cloned().unwrap_or_default() {
        Request::delete(format!("{base}/permissions/{}", u(&l, "id")))
            .graph_json()
            .send()
            .await?;
    }
    Ok(())
}

pub async fn sharinglink_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving sharing link for file {} with id {}...",
        first(cx, &["fileUrl", "fileId"]),
        opt(cx, "id")
    ));
    let base = vroom(cx).await?;
    cx.log(
        Request::get(format!("{base}/permissions/{}", opt(cx, "id")))
            .graph_json()
            .data()
            .await?,
    )
}

pub async fn sharinglink_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving sharing links for file {}...",
        first(cx, &["fileId", "fileUrl"])
    ));
    let base = vroom(cx).await?;
    let mut url = format!("{base}/permissions?$filter=Link ne null");
    if let Some(s) = ne(cx, "scope") {
        url.push_str(&format!(" and Link/Scope eq '{s}'"));
    }
    let links = odata::get_all_items(&url).await?;
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(links));
    }
    let mut out = Vec::new();
    for l in &links {
        let roles = need_arr(l, "roles", "join")?
            .iter()
            .map(|r| {
                if r.is_null() {
                    String::new()
                } else {
                    to_js_string(r)
                }
            })
            .collect::<Vec<_>>()
            .join(",");
        let link = l
            .get("link")
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'webUrl')"))?;
        out.push(obj! { "id": ?l.get("id").cloned(), "roles": roles, "link": ?link.get("webUrl").cloned(), "scope": ?link.get("scope").cloned() });
    }
    cx.log(Value::Array(out))
}

pub async fn sharinglink_remove(cx: &mut Cx) -> Result<()> {
    let id = opt(cx, "id");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove sharing link {id} of file {}?",
            first(cx, &["fileUrl", "fileId"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing sharing link of file {} with id {id}...",
        first(cx, &["fileUrl", "fileId"])
    ));
    let base = vroom(cx).await?;
    Request::delete(format!("{base}/permissions/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn sharinglink_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating sharing link of file {}...",
        first(cx, &["fileId", "fileUrl"])
    ));
    let base = vroom(cx).await?;
    cx.log(
        Request::patch(format!("{base}/permissions/{}", opt(cx, "id")))
            .graph_json()
            .json(obj! { "expirationDateTime": ?cx.val("expirationDateTime").cloned() })
            .data()
            .await?,
    )
}

// ---- versions ------------------------------------------------------------------

fn file_api(cx: &Cx) -> String {
    let web = cx.u("webUrl");
    match ne(cx, "fileUrl") {
        Some(f) => format!(
            "{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(&web, &f))
        ),
        None => format!("{web}/_api/web/GetFileById('{}')", opt(cx, "fileId")),
    }
}

pub async fn version_clear(cx: &mut Cx) -> Result<()> {
    let file = first(cx, &["fileId", "fileUrl"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to delete all version history for file {file}'?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Deletes all version history of the file {} at site {}...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    Request::post(format!("{}/versions/DeleteAll()", file_api(cx)))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn version_get(cx: &mut Cx) -> Result<()> {
    let label = opt(cx, "label");
    cx.verbose_log(&format!(
        "Retrieving version {label} of the file {} at site {}...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    let r = Request::get(format!(
        "{}/versions/?$filter=VersionLabel eq '{label}'&$select=*,ExpirationDate",
        file_api(cx)
    ))
    .spo_json()
    .data()
    .await?;
    match need_arr(&r, "value", "0")?.first() {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn version_keep(cx: &mut Cx) -> Result<()> {
    let label = opt(cx, "label");
    cx.verbose_log(&format!(
        "Ensuring version '{label}' of file '{}' at site '{}' will never expire...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    let base = file_api(cx);
    let r = odata::get_all_items(&format!(
        "{base}/versions?$filter=VersionLabel eq '{}'&$select=ID",
        eqp(&label)
    ))
    .await?;
    let Some(v) = r.first() else {
        return Err(Error::msg(format!(
            "Version with label '{label}' not found."
        )));
    };
    Request::post(format!(
        "{base}/versions({})/SetExpirationDate()",
        u(v, "ID")
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn version_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving all versions of file {} at site {}...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{}/versions?$select=*,ExpirationDate",
            file_api(cx)
        ))
        .await?,
    ))
}

pub async fn version_remove(cx: &mut Cx) -> Result<()> {
    let label = opt(cx, "label");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the version {label} from file {}'?",
            first(cx, &["fileId", "fileUrl"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removes version {label} of the file {} at site {}...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    Request::delete(format!(
        "{}/versions/DeleteByLabel('{label}')",
        file_api(cx)
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn version_restore(cx: &mut Cx) -> Result<()> {
    let label = opt(cx, "label");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to restore the version {label} from file {}'?",
            first(cx, &["fileId", "fileUrl"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Restores version {label} of the file {} at site {}...",
        first(cx, &["fileUrl", "fileId"]),
        cx.u("webUrl")
    ));
    Request::post(format!(
        "{}/versions/RestoreByLabel('{label}')",
        file_api(cx)
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

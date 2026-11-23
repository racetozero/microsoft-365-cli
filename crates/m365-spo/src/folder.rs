//! `spo folder *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::url as urlutil;
use m365_core::ut::{drive, entra, spo};

fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

fn opt(cx: &Cx, k: &str) -> String {
    cx.val(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn first(cx: &Cx, keys: &[&str]) -> String {
    keys.iter()
        .find_map(|k| ne(cx, k))
        .unwrap_or_else(|| "undefined".into())
}

const COLORS: [(&str, &str); 16] = [
    ("yellow", "0"),
    ("darkRed", "1"),
    ("darkOrange", "2"),
    ("darkGreen", "3"),
    ("darkTeal", "4"),
    ("darkBlue", "5"),
    ("darkPurple", "6"),
    ("darkPink", "7"),
    ("grey", "8"),
    ("lightRed", "9"),
    ("lightOrange", "10"),
    ("lightGreen", "11"),
    ("lightTeal", "12"),
    ("lightBlue", "13"),
    ("lightPurple", "14"),
    ("lightPink", "15"),
];

fn color_hex(c: &str) -> String {
    COLORS
        .iter()
        .find(|(k, _)| *k == c)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| c.to_string())
}

fn by_path(srv: &str) -> String {
    format!("GetFolderByServerRelativePath(DecodedUrl='{}')", eqp(srv))
}

async fn add_folder(cx: &Cx, srv: &str) -> Result<Value> {
    if cx.verbose() {
        let parts: Vec<&str> = srv.split('/').collect();
        cx.stderr(&format!(
            "Adding folder with name '{}' at location '{}'...",
            parts.last().unwrap_or(&""),
            parts[..parts.len() - 1].join("/")
        ));
    }
    let web = cx.u("webUrl");
    match ne(cx, "color") {
        None if cx.val("color").is_none() => {
            Request::post(format!(
                "{web}/_api/web/folders/addUsingPath(decodedUrl='{}')",
                eqp(srv)
            ))
            .spo_json()
            .data()
            .await
        }
        c => {
            let c = c.unwrap_or_default();
            Request::post(format!(
                "{web}/_api/foldercoloring/createfolder(DecodedUrl='{}', overwrite=false)",
                eqp(srv)
            ))
            .spo_json()
            .json(obj! { "coloringInformation": obj! { "ColorHex": color_hex(&c) } })
            .data()
            .await
        }
    }
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let parent = urlutil::get_server_relative_path(&web, &cx.u("parentFolderUrl"));
    let srv = format!("{parent}/{}", opt(cx, "name"));
    if cx.bool("ensureParentFolders") {
        cx.verbose_log("Ensuring parent folders exist...");
        let url = url::Url::parse(&web).map_err(|_| Error::js("Invalid URL"))?;
        let abs = format!("{}{parent}", url.origin().ascii_serialization());
        let rel = abs.replacen(&web, "", 1);
        let parts: Vec<&str> = rel.split('/').filter(|f| !f.is_empty()).collect();
        for i in 1..parts.len() {
            let cur = parts[..=i].join("/");
            cx.verbose_log(&format!("Checking if folder '{cur}' exists..."));
            let r = Request::get(format!("{web}/_api/web/{}?$select=Exists", by_path(&cur)))
                .spo_json()
                .data()
                .await?;
            if !r.get("Exists").is_some_and(truthy) {
                add_folder(cx, &cur).await?;
            }
        }
    }
    cx.log(add_folder(cx, &srv).await?)
}

async fn archive(cx: &Cx, verb: &str, method: &str) -> Result<()> {
    let web = cx.u("webUrl");
    let who = ne(cx, "url")
        .or_else(|| ne(cx, "id"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("{verb} folder {who} at site {web}..."));
    let mut url = format!("{web}/_api/web");
    if let Some(i) = ne(cx, "id") {
        url.push_str(&format!("/GetFolderById('{}')", eqp(&i)));
    } else if let Some(u) = ne(cx, "url") {
        url.push_str(&format!(
            "/{}",
            by_path(&urlutil::get_server_relative_path(&web, &u))
        ));
    }
    url.push_str("?$select=Exists,ListItemAllFields/Id,ListItemAllFields/ParentList/Id&$expand=ListItemAllFields,ListItemAllFields/ParentList");
    let info = Request::get(url).spo_json().data().await?;
    if !info.get("Exists").is_some_and(truthy) {
        return Err(Error::msg(format!("The folder '{who}' does not exist.")));
    }
    let item = info.get("ListItemAllFields").filter(|v| !v.is_null());
    let Some(parent) = item.and_then(|i| i.get("ParentList")).filter(|p| truthy(p)) else {
        let what = if method == "Archive" {
            "archived. Archive"
        } else {
            "unarchived. Unarchive"
        };
        return Err(Error::msg(format!(
            "The folder '{who}' is the root folder of a document library and cannot be {what} a subfolder instead."
        )));
    };
    let item = item.unwrap();
    Request::post(format!(
        "{web}/_api/Lists(guid'{}')/items({})/{method}",
        u(parent, "Id"),
        u(item, "Id")
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn archive_cmd(cx: &mut Cx) -> Result<()> {
    if !cx.bool("force") && !cx.confirm("Are you sure you would like to archive this folder? You will be able to reactivate it instantly for the first 7 days. After that, it will take up to 24 hours to reactivate.")? {
        return Ok(());
    }
    archive(cx, "Archiving", "Archive").await
}

pub async fn unarchive(cx: &mut Cx) -> Result<()> {
    if !cx.bool("force") && !cx.confirm("Reactivation could take up to 24 hours. Folders that are reactivated cannot be archived again for a period of time. Are you sure you would like to unarchive this folder?")? {
        return Ok(());
    }
    archive(cx, "Unarchiving", "UnArchive").await
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
                "Retrieving server-relative path for folder with ID '{id}'..."
            ));
            let r = Request::get(format!(
                "{web}/_api/Web/GetFolderById('{id}')/ServerRelativePath"
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
    let dest = absolute(&web, &cx.u("targetUrl"));
    let (verb, noun) = if operation == "move" {
        ("Moving", "move")
    } else {
        ("Copying", "copy")
    };
    cx.verbose_log(&format!("{verb} folder '{src}' to '{dest}'..."));
    let ncb = if cx
        .s("nameConflictBehavior")
        .map(|s| s.to_lowercase())
        .as_deref()
        == Some("rename")
    {
        2
    } else {
        0
    };
    let options = obj! {
        "NameConflictBehavior": ncb, "AllowSchemaMismatch": true,
        "CustomizedItemName": ?ne(cx, "newName").map(|n| vec![n]),
        "SameWebCopyMoveOptimization": true, "IsMoveMode": operation == "move",
    };
    let job = sp::create_copy_job(&web, &src, &dest, options).await?;
    if cx.bool("skipWait") {
        return Ok(());
    }
    cx.verbose_log(&format!("Waiting for the {noun} job to complete..."));
    let res = sp::copy_job_result(&web, &job).await?;
    cx.verbose_log("Getting information about the destination folder...");
    let rel = match res.get("TargetObjectSiteRelativeUrl") {
        Some(Value::String(s)) => s.clone(),
        _ => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'substring')",
            ));
        }
    };
    let folder = format!("/{}", rel.rfind('/').map(|i| &rel[..i]).unwrap_or(""));
    let lower = dest.to_lowercase();
    let abs_web = match lower.rfind(&folder.to_lowercase()) {
        Some(i) => dest[..i].to_string(),
        None => String::new(),
    };
    let r = Request::get(format!(
        "{abs_web}/_api/Web/GetFolderById('{}')",
        u(&res, "TargetObjectUniqueId")
    ))
    .spo_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn copy(cx: &mut Cx) -> Result<()> {
    copy_or_move(cx, "copy").await
}

pub async fn move_cmd(cx: &mut Cx) -> Result<()> {
    copy_or_move(cx, "move").await
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Retrieving folder from site {web}..."));
    let mut url = format!("{web}/_api/web");
    if let Some(i) = ne(cx, "id") {
        url.push_str(&format!("/GetFolderById('{}')", eqp(&i)));
    } else if let Some(u) = ne(cx, "url") {
        url.push_str(&format!(
            "/{}",
            by_path(&urlutil::get_server_relative_path(&web, &u))
        ));
    }
    let with = cx.bool("withPermissions");
    if with {
        url.push_str("?$expand=ListItemAllFields/HasUniqueRoleAssignments,ListItemAllFields/RoleAssignments/Member,ListItemAllFields/RoleAssignments/RoleDefinitionBindings");
    }
    let mut f = Request::get(url).spo_json().data().await?;
    if with {
        let Some(li) = f.get_mut("ListItemAllFields").filter(|v| truthy(v)) else {
            return Err(Error::js(
                "Please ensure the specified folder URL or folder Id does not refer to a root folder. Use 'spo list get' with withPermissions instead.",
            ));
        };
        let ras = match li.get_mut("RoleAssignments") {
            Some(Value::Array(a)) => a,
            _ => {
                return Err(Error::js(
                    "Cannot read properties of undefined (reading 'forEach')",
                ));
            }
        };
        for r in ras.iter_mut() {
            let m = r.get_mut("Member").ok_or_else(|| {
                Error::js("Cannot read properties of undefined (reading 'PrincipalType')")
            })?;
            let t = match m.get("PrincipalType").and_then(Value::as_f64) {
                Some(0.0) => Some("None"),
                Some(1.0) => Some("User"),
                Some(2.0) => Some("DistributionList"),
                Some(4.0) => Some("SecurityGroup"),
                Some(8.0) => Some("SharePointGroup"),
                Some(15.0) => Some("All"),
                _ => None,
            };
            if let (Some(o), Some(t)) = (m.as_object_mut(), t) {
                o.insert("PrincipalTypeString".into(), Value::from(t));
            }
            let mut b = match r.get("RoleDefinitionBindings") {
                Some(Value::Array(a)) => a.clone(),
                _ => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading 'forEach')",
                    ));
                }
            };
            sp::set_friendly_permissions(&mut b)?;
            r["RoleDefinitionBindings"] = Value::Array(b);
        }
    }
    cx.log(f)
}

fn select_expand(fields: Option<String>) -> (Vec<String>, Vec<String>) {
    let (mut s, mut e) = (Vec::new(), Vec::new());
    if let Some(f) = fields {
        for x in f.split(',') {
            let parts: Vec<&str> = x.trim().split('/').collect();
            if parts.len() > 1 && !e.contains(&parts[0].to_string()) {
                e.push(parts[0].to_string());
            }
            if !s.contains(&x.trim().to_string()) {
                s.push(x.trim().to_string());
            }
        }
    }
    (s, e)
}

fn folders<'a>(
    cx: &'a Cx,
    parent: String,
    sel: &'a (Vec<String>, Vec<String>),
    skip: usize,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Value>>> + Send + 'a>> {
    Box::pin(async move {
        if cx.verbose() {
            let page = (skip as f64 / 5000.0).ceil() as usize + 1;
            cx.stderr(&format!(
                "Retrieving folders in folder '{parent}'{}...",
                if page > 1 {
                    format!(", page {page}")
                } else {
                    String::new()
                }
            ));
        }
        let web = cx.u("webUrl");
        let srv = urlutil::get_server_relative_path(&web, &parent);
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
            "{web}/_api/web/{}/Folders?{}",
            by_path(&srv),
            q.join("&")
        ))
        .spo_json()
        .data()
        .await?;
        let items = need_arr(&r, "value", "length")?.to_vec();
        let mut out = Vec::new();
        for f in &items {
            out.push(f.clone());
            if cx.bool("recursive") {
                out.extend(folders(cx, u(f, "ServerRelativeUrl"), sel, 0).await?);
            }
        }
        if items.len() == 5000 {
            out.extend(folders(cx, parent, sel, skip + 5000).await?);
        }
        Ok(out)
    })
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let parent = cx.u("parentFolderUrl");
    cx.verbose_log(&format!(
        "Retrieving all folders in folder '{parent}' at site '{}'{}...",
        cx.u("webUrl"),
        if cx.bool("recursive") {
            " (recursive)"
        } else {
            ""
        }
    ));
    let sel = select_expand(ne(cx, "fields"));
    let mut all = folders(cx, parent, &sel, 0).await?;
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
            "Are you sure you want to {} the folder {} located in site {web}?",
            if permanent {
                "permanently remove"
            } else {
                "recycle"
            },
            opt(cx, "url")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing folder in site at {web}..."));
    let srv = urlutil::get_server_relative_path(&web, &cx.u("url"));
    let url = format!(
        "{web}/_api/web/{}{}",
        by_path(&srv),
        if permanent { "" } else { "/recycle()" }
    );
    Request::post(url)
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json()
        .send()
        .await?;
    Ok(())
}

async fn folder_props(cx: &Cx) -> Result<Value> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving list and item information for folder '{}' in site at {web}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let seg = match ne(cx, "folderId") {
        Some(i) => format!("GetFolderById('{i}')"),
        None => by_path(&urlutil::get_server_relative_path(&web, &cx.u("folderUrl"))),
    };
    Request::get(format!(
        "{web}/_api/web/{seg}?$expand=ListItemAllFields,ListItemAllFields/ParentList/RootFolder&$select=ServerRelativeUrl,ListItemAllFields/ParentList/RootFolder/ServerRelativeUrl,ListItemAllFields/Id"
    ))
    .spo_json()
    .data()
    .await
}

async fn folder_label(cx: &Cx, name: &str) -> Result<()> {
    let web = cx.u("webUrl");
    let props = folder_props(cx).await?;
    let verbose = cx.val("verbose").is_some_and(truthy);
    match props.get("ListItemAllFields").filter(|v| truthy(v)) {
        Some(li) => {
            let url = url::Url::parse(&web).map_err(|_| Error::js("Invalid URL"))?;
            let tenant = format!("{}://{}", url.scheme(), url.host_str().unwrap_or(""));
            let root = match dig(li, &["ParentList", "RootFolder", "ServerRelativeUrl"])? {
                Some(v) => to_js_string(v),
                None => "undefined".into(),
            };
            let abs = urlutil::url_combine(&tenant, &root);
            let id = json::parse_int(&u(li, "Id"))
                .map(Value::from)
                .unwrap_or(Value::Null);
            sp::set_compliance_tag_on_items(cx, &web, name, &abs, vec![id], verbose).await
        }
        None => {
            let abs = urlutil::get_absolute_url(&web, &u(&props, "ServerRelativeUrl"));
            if verbose {
                if name.is_empty() {
                    cx.stderr(&format!(
                        "Removing the default retention label from the list '{abs}'..."
                    ));
                } else {
                    cx.stderr(&format!(
                        "Applying default retention label '{name}' to the list '{abs}'..."
                    ));
                }
            }
            Request::post(format!("{web}/_api/SP_CompliancePolicy_SPPolicyStoreProxy_SetListComplianceTag"))
                .spo_json()
                .json(obj! { "listUrl": abs, "complianceTagValue": name, "blockDelete": false, "blockEdit": false, "syncToItems": false })
                .send()
                .await?;
            Ok(())
        }
    }
}

pub async fn retentionlabel_ensure(cx: &mut Cx) -> Result<()> {
    folder_label(cx, &opt(cx, "name")).await
}

pub async fn retentionlabel_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the retentionlabel from folder {} located in site {}?",
            first(cx, &["folderId", "folderUrl"]),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    folder_label(cx, "").await
}

fn role_url(cx: &Cx, list_for_root: bool) -> String {
    let web = cx.u("webUrl");
    let folder = cx.u("folderUrl");
    let srv = urlutil::get_server_relative_path(&web, &folder);
    let rel = urlutil::get_web_relative_path(&web, &folder);
    if list_for_root && rel.split('/').count() == 2 {
        format!("{web}/_api/web/GetList('{}')", eqp(&srv))
    } else {
        format!("{web}/_api/web/{}/ListItemAllFields", by_path(&srv))
    }
}

async fn principal(cx: &Cx) -> Result<String> {
    let web = cx.u("webUrl");
    if let Some(upn) = ne(cx, "upn") {
        return Ok(u(&sp::user_by_email(cx, &web, &upn).await?, "Id"));
    }
    if let Some(g) = ne(cx, "groupName") {
        return Ok(u(&sp::group_by_name(cx, &web, &g).await?, "Id"));
    }
    if ne(cx, "entraGroupId").is_some() || ne(cx, "entraGroupName").is_some() {
        cx.verbose_log("Retrieving group information...");
        let g = match ne(cx, "entraGroupId") {
            Some(i) => entra::group::by_id(&i, None).await?,
            None => entra::group::by_display_name(&opt(cx, "entraGroupName"), None).await?,
        };
        return Ok(u(&sp::ensure_entra_group(&web, &g).await?, "Id"));
    }
    Ok(opt(cx, "principalId"))
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding role assignment to folder in site at {web}..."
    ));
    let url = role_url(cx, true);
    let role = match ne(cx, "roleDefinitionName") {
        Some(n) => u(
            &sp::role_definition_by_name_strict(cx, &web, &n).await?,
            "Id",
        ),
        None => opt(cx, "roleDefinitionId"),
    };
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}/roleassignments/addroleassignment(principalid='{p}',roledefid='{role}')"
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleassignment_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove a role assignment from the folder with url '{}'?",
            opt(cx, "folderUrl")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing role assignment from folder in site at {}...",
        cx.u("webUrl")
    ));
    let url = role_url(cx, false);
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}/roleassignments/removeroleassignment(principalid='{p}')"
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_break(cx: &mut Cx) -> Result<()> {
    let keep = !cx.bool("clearExistingPermissions");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to break the role inheritance of folder {} located in site {}?",
            opt(cx, "folderUrl"),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    Request::post(format!(
        "{}/breakroleinheritance({keep})",
        role_url(cx, true)
    ))
    .accept("application/json")
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_reset(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to reset the role inheritance of folder {} located in site {}?",
            opt(cx, "folderUrl"),
            cx.u("webUrl")
        ),
    )? {
        return Ok(());
    }
    Request::post(format!("{}/resetroleinheritance", role_url(cx, true)))
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Updating folder '{}'...", opt(cx, "name")));
    let srv = urlutil::get_server_relative_path(&web, &cx.u("url"));
    match ne(cx, "color") {
        None => {
            let r = Request::patch(format!(
                "{web}/_api/Web/{}/ListItemAllFields",
                by_path(&srv)
            ))
            .spo_json()
            .header("if-match", "*")
            .json(
                obj! { "FileLeafRef": ?cx.val("name").cloned(), "Title": ?cx.val("name").cloned() },
            )
            .data()
            .await?;
            if r.get("odata.null") == Some(&Value::Bool(true)) {
                return Err(Error::msg("Folder not found."));
            }
        }
        Some(c) => {
            let m = if ne(cx, "name").is_some() {
                "renamefolder"
            } else {
                "stampcolor"
            };
            Request::post(format!("{web}/_api/foldercoloring/{m}(DecodedUrl='{}')", eqp(&srv)))
                .spo_json()
                .json(obj! { "coloringInformation": obj! { "ColorHex": color_hex(&c) }, "newName": ?cx.val("name").cloned() })
                .send()
                .await?;
        }
    }
    Ok(())
}

// ---- sharing links -------------------------------------------------------------

async fn item_target(cx: &Cx, log: bool) -> Result<(String, String)> {
    let web = cx.u("webUrl");
    let rel = if log {
        sp::folder_server_relative_url(
            cx,
            &web,
            ne(cx, "folderUrl").as_deref(),
            ne(cx, "folderId").as_deref(),
        )
        .await?
    } else {
        let seg = match ne(cx, "folderUrl") {
            Some(f) => format!(
                "GetFolderByServerRelativePath(decodedUrl='{}')",
                eqp(&urlutil::get_server_relative_path(&web, &f))
            ),
            None => format!("GetFolderById('{}')", opt(cx, "folderId")),
        };
        u(
            &Request::get(format!("{web}/_api/web/{seg}?$select=ServerRelativeUrl"))
                .spo_json()
                .data()
                .await?,
            "ServerRelativeUrl",
        )
    };
    let abs = urlutil::get_absolute_url(&web, &rel);
    let url = url::Url::parse(&abs).map_err(|_| Error::js("Invalid URL"))?;
    let site = spo::graph_site_id(&web).await?;
    let lg = if log { Some(cx) } else { None };
    let d = drive::drive_by_url(&site, &url, lg).await?;
    let item = drive::drive_item_id(&d, &url, lg).await?;
    Ok((u(&d, "id"), item))
}

const GRAPH: &str = "https://graph.microsoft.com/v1.0";

pub async fn sharinglink_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Creating a sharing link to a folder {}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let (d, i) = item_target(cx, true).await?;
    let mut b = obj! {
        "type": ?cx.val("type").cloned(), "expirationDateTime": ?cx.val("expirationDateTime").cloned(),
        "scope": ?cx.val("scope").cloned(), "retainInheritedPermissions": cx.bool("retainInheritedPermissions"),
    };
    if cx.str("scope").as_deref() == Some("users")
        && let Some(r) = ne(cx, "recipients")
    {
        b["recipients"] = Value::from(
            util::split_and_trim(&r)
                .into_iter()
                .map(|e| obj! { "email": e })
                .collect::<Vec<_>>(),
        );
    }
    let mut r = Request::post(format!("{GRAPH}/drives/{d}/items/{i}/createLink"))
        .graph_json()
        .json(b)
        .data()
        .await?;
    if r.get("grantedToIdentities").is_some_and(truthy)
        && let Some(o) = r.as_object_mut()
    {
        o.shift_remove("grantedToIdentities");
    }
    cx.log(r)
}

fn scope_label(cx: &Cx) -> String {
    match ne(cx, "scope") {
        Some(s) => format!("{s} scope"),
        None => "all scopes".into(),
    }
}

fn perm_url(d: &str, i: &str, cx: &Cx) -> String {
    let mut u = format!("{GRAPH}/drives/{d}/items/{i}/permissions?$filter=Link ne null");
    if let Some(s) = ne(cx, "scope") {
        u.push_str(&format!(" and Link/Scope eq '{s}'"));
    }
    u
}

pub async fn sharinglink_clear(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to clear sharing links from folder {}? for {}",
            first(cx, &["folderUrl", "folderId"]),
            scope_label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Clearing sharing links from folder {} for {}",
        first(cx, &["folderId", "folderUrl"]),
        scope_label(cx)
    ));
    let (d, i) = item_target(cx, true).await?;
    for l in odata::get_all_items(&perm_url(&d, &i, cx)).await? {
        Request::delete(format!(
            "{GRAPH}/drives/{d}/items/{i}/permissions/{}",
            u(&l, "id")
        ))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    }
    Ok(())
}

pub async fn sharinglink_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving sharing link on folder {}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let (d, i) = item_target(cx, false).await?;
    cx.log(
        Request::get(format!(
            "{GRAPH}/drives/{d}/items/{i}/permissions/{}",
            opt(cx, "id")
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn sharinglink_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving sharing links for folder {}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let (d, i) = item_target(cx, true).await?;
    let links: Vec<Value> = odata::get_all_items(&perm_url(&d, &i, cx))
        .await?
        .into_iter()
        .map(|mut l| {
            if let Some(o) = l.as_object_mut() {
                o.shift_remove("grantedToIdentities");
            }
            l
        })
        .collect();
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
            "Are you sure you want to remove sharing link {id} of folder {}?",
            first(cx, &["folderUrl", "folderId"])
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing sharing link of folder {} with id {id}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let (d, i) = item_target(cx, true).await?;
    Request::delete(format!("{GRAPH}/drives/{d}/items/{i}/permissions/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn sharinglink_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Updating sharing link to a folder {}...",
        first(cx, &["folderId", "folderUrl"])
    ));
    let (d, i) = item_target(cx, true).await?;
    let r = Request::patch(format!(
        "{GRAPH}/drives/{d}/items/{i}/permissions/{}",
        opt(cx, "id")
    ))
    .graph_json()
    .header("content-type", "application/json")
    .json(obj! { "expirationDateTime": ?cx.val("expirationDateTime").cloned() })
    .data()
    .await?;
    cx.log(r)
}

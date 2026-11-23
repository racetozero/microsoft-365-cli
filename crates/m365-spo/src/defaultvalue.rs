//! `spo list defaultvalue *`: default column values stored in the library's
//! `Forms/client_LocationBasedDefaults.html`.

use crate::sp::{self, first, ne};
use m365_core::prelude::*;
use m365_core::ut::url as urlutil;
use m365_core::xml::{self, Element, Node};

fn label(cx: &Cx) -> String {
    first(cx, &sp::LIST)
}

/// `getServerRelativeListUrl`; `strict` is the clear/remove/set variant
/// (encoded list URL and its own "not a document library" message).
async fn list_url(cx: &Cx, strict: bool) -> Result<String> {
    let web = cx.u("webUrl");
    let mut url = format!("{web}/_api/Web");
    if let Some(l) = ne(cx, "listUrl") {
        let srv = urlutil::get_server_relative_path(&web, &l);
        url.push_str(&format!(
            "/GetList('{}')",
            if strict { eqp(&srv) } else { srv }
        ));
    } else if let Some(i) = ne(cx, "listId") {
        url.push_str(&format!("/Lists('{i}')"));
    } else if let Some(t) = ne(cx, "listTitle") {
        url.push_str(&format!("/Lists/GetByTitle('{}')", eqp(&t)));
    }
    url.push_str("?$expand=RootFolder&$select=RootFolder/ServerRelativeUrl,BaseTemplate");
    match Request::get(url).spo_json().data().await {
        Ok(r) => {
            if r.get("BaseTemplate").and_then(Value::as_f64) != Some(101.0) {
                return Err(Error::msg(if strict {
                    "The specified list is not a document library.".to_string()
                } else {
                    format!("List '{}' is not a document library.", label(cx))
                }));
            }
            Ok(match dig(&r, &["RootFolder", "ServerRelativeUrl"])? {
                Some(v) => to_js_string(v),
                None => "undefined".into(),
            })
        }
        Err(e) if e.status() == Some(404) => {
            Err(Error::msg(format!("List '{}' was not found.", label(cx))))
        }
        Err(e) => Err(e),
    }
}

fn file_url(web: &str, list: &str) -> String {
    format!(
        "{web}/_api/Web/GetFileByServerRelativePath(decodedUrl='{}')/$value",
        eqp(&format!("{list}/Forms/client_LocationBasedDefaults.html"))
    )
}

/// `request.get` of the defaults file; `None` when it does not exist.
async fn defaults(web: &str, list: &str, json: bool) -> Result<Option<String>> {
    let mut r = Request::get(file_url(web, list)).accept("application/json;odata=nometadata");
    if json {
        r = r.json_response();
    }
    match r.data().await {
        Ok(Value::String(s)) => Ok(Some(s)),
        Ok(v) => Ok(Some(to_js_string(&v))),
        Err(e) if e.status() == Some(404) => Ok(None),
        Err(e) => Err(e),
    }
}

async fn upload(web: &str, list: &str, xml: String, if_match: bool) -> Result<()> {
    let mut r = Request::put(file_url(web, list)).spo_json().text(xml);
    r = if if_match {
        r.header("If-Match", "*")
    } else {
        r.header("content-type", "text/plain")
    };
    r.send().await?;
    Ok(())
}

fn to_json(text: &str) -> Vec<Value> {
    let doc = xml::parse(text);
    let mut out = Vec::new();
    if let Some(root) = &doc.root {
        let mut links = root.find_all("a");
        if root.name == "a" {
            links.insert(0, Vec::new());
        }
        for p in links {
            let a = root.at(&p).unwrap();
            let folder = util::dec(a.attr("href").unwrap_or("null"));
            for dp in a.find_all("DefaultValue") {
                let d = a.at(&dp).unwrap();
                out.push(obj! { "fieldName": d.attr("FieldName").map(Value::from).unwrap_or(Value::Null), "fieldValue": d.text(), "folderUrl": folder.as_str() });
            }
        }
    }
    out
}

fn lower(v: &Value, k: &str) -> Result<String> {
    match v.get(k) {
        Some(Value::String(s)) => Ok(s.to_lowercase()),
        _ => Err(Error::js(
            "Cannot read properties of null (reading 'toLowerCase')",
        )),
    }
}

async fn values_for(cx: &Cx, json: bool) -> Result<(String, Vec<Value>)> {
    cx.verbose_log("Retrieving list information...");
    let list = list_url(cx, false).await?;
    cx.verbose_log("Retrieving default column values...");
    let web = cx.u("webUrl");
    let vals = match defaults(&web, &list, json).await? {
        Some(x) => to_json(&x),
        None => Vec::new(),
    };
    Ok((list, vals))
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let field = cx.u("fieldName");
    cx.verbose_log(&format!(
        "Retrieving default column value for field '{field}' in list '{}'...",
        label(cx)
    ));
    let (list, vals) = values_for(cx, false).await?;
    let mut keep = Vec::new();
    for d in vals {
        if lower(&d, "fieldName")? == field.to_lowercase() {
            keep.push(d);
        }
    }
    let folder = match ne(cx, "folderUrl") {
        Some(f) => urlutil::remove_trailing_slashes(&urlutil::get_server_relative_path(
            &cx.u("webUrl"),
            &f,
        )),
        None => list,
    };
    let mut found = Vec::new();
    for d in keep {
        if lower(&d, "folderUrl")? == folder.to_lowercase() {
            found.push(d);
        }
    }
    match found.into_iter().next() {
        Some(d) => cx.log(d),
        None => Err(Error::msg(format!(
            "No default column value found for field '{field}'{}.",
            ne(cx, "folderUrl")
                .map(|f| format!(" in folder '{f}'"))
                .unwrap_or_default()
        ))),
    }
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving default column values for list '{}'...",
        label(cx)
    ));
    let (_, mut vals) = values_for(cx, true).await?;
    if let Some(f) = ne(cx, "folderUrl") {
        let folder = urlutil::remove_trailing_slashes(&urlutil::get_server_relative_path(
            &cx.u("webUrl"),
            &f,
        ))
        .to_lowercase();
        let mut keep = Vec::new();
        for d in vals {
            if lower(&d, "folderUrl")? == folder {
                keep.push(d);
            }
        }
        vals = keep;
    }
    cx.log(Value::Array(vals))
}

fn child_count(e: &Element) -> usize {
    e.children.len()
}

pub async fn clear(cx: &mut Cx) -> Result<()> {
    let field = ne(cx, "fieldName");
    let folder = ne(cx, "folderUrl");
    if !cx.bool("force") {
        let what = match (&field, &folder) {
            (Some(f), _) => format!(" for field '{f}'"),
            (None, Some(f)) => format!(" for folder {f}"),
            _ => String::new(),
        };
        if !cx.confirm(&format!(
            "Are you sure you want to clear all default values{what}?"
        ))? {
            return Ok(());
        }
    }
    let what = match (&field, &folder) {
        (Some(f), _) => format!(" for field {f}"),
        (None, Some(f)) => format!("for folder '{f}'"),
        _ => String::new(),
    };
    cx.verbose_log(&format!("Clearing all default column values{what}..."));
    cx.verbose_log("Getting server-relative URL of the list...");
    let list = list_url(cx, true).await?;
    cx.verbose_log(&format!("List server-relative URL: {list}"));
    cx.verbose_log("Getting default column values...");
    let web = cx.u("webUrl");
    let Some(text) = defaults(&web, &list, true).await? else {
        cx.verbose_log("No default column values found.");
        return Ok(());
    };
    let out = if field.is_none() && folder.is_none() {
        "<MetadataDefaults />".to_string()
    } else {
        let target = folder.as_ref().map(|f| {
            urlutil::remove_trailing_slashes(&urlutil::get_server_relative_path(&web, f))
                .to_lowercase()
        });
        let mut doc = xml::parse(&text);
        if let Some(root) = doc.root.as_mut() {
            // a live NodeList: removing a node shifts the remaining ones
            let mut i = 0;
            loop {
                let links = root.find_all("a");
                let Some(p) = links.get(i).cloned() else {
                    break;
                };
                let a = root.at(&p).unwrap().clone();
                let href = a.attr("href").unwrap_or("null").to_string();
                if target
                    .as_deref()
                    .is_some_and(|t| t == util::dec(&href).to_lowercase())
                {
                    root.remove_at(&p);
                    break;
                } else if let Some(f) = &field {
                    for dp in a.find_all("DefaultValue") {
                        let d = a.at(&dp).unwrap();
                        if d.attr("FieldName").unwrap_or("").to_lowercase() == f.to_lowercase() {
                            if child_count(&a) == 1 {
                                root.remove_at(&p);
                            } else {
                                let mut full = p.clone();
                                full.extend(dp.iter());
                                root.remove_at(&full);
                            }
                            break;
                        }
                    }
                }
                i += 1;
            }
        }
        xml::serialize(&doc)
    };
    upload(&web, &list, out, true).await
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let field = cx.u("fieldName");
    let where_ = match ne(cx, "folderUrl") {
        Some(f) => format!("'{f}'"),
        None => "the root of the list".into(),
    };
    if !cx.bool("force")
        && !cx.confirm(&format!(
            "Are you sure you want to remove default column value '{field}' from {where_}?"
        ))?
    {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing default column value '{field}' from {where_}."
    ));
    cx.verbose_log("Getting server-relative URL of the list...");
    let list = list_url(cx, true).await?;
    let web = cx.u("webUrl");
    let folder = match ne(cx, "folderUrl") {
        Some(f) => urlutil::get_server_relative_path(&web, &urlutil::remove_trailing_slashes(&f)),
        None => list.clone(),
    };
    cx.verbose_log("Getting default column values...");
    let text = defaults(&web, &list, true).await?;
    let mut result = None;
    if let Some(text) = text {
        let encoded = folder.replace(' ', "%20").to_lowercase();
        let mut doc = xml::parse(&text);
        if let Some(root) = doc.root.as_mut() {
            'outer: for p in root.find_all("a") {
                let a = root.at(&p).unwrap().clone();
                if a.attr("href").unwrap_or("null").to_lowercase() != encoded {
                    continue;
                }
                for dp in a.find_all("DefaultValue") {
                    if a.at(&dp).unwrap().attr("FieldName") != Some(field.as_str()) {
                        continue;
                    }
                    if child_count(&a) == 1 {
                        root.remove_at(&p);
                    } else {
                        let mut full = p.clone();
                        full.extend(dp.iter());
                        root.remove_at(&full);
                    }
                    result = Some(());
                    break 'outer;
                }
            }
        }
        if result.is_some() {
            cx.verbose_log("Uploading default column values to list...");
            return upload(&web, &list, xml::serialize(&doc), false).await;
        }
    }
    Err(Error::msg(format!(
        "Default column value '{field}' was not found."
    )))
}

pub fn set_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let s = to_js_string(v);
    match i {
        0 => check(!s.is_empty()),
        _ => check(!s.contains('#') && !s.contains('%')),
    }
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let (field, value) = (cx.u("fieldName"), cx.u("fieldValue"));
    cx.verbose_log(&format!(
        "Setting default column value '{value}' for field '{field}'..."
    ));
    cx.verbose_log("Getting server-relative URL of the list...");
    let list = list_url(cx, true).await?;
    let web = cx.u("webUrl");
    let mut folder = list.clone();
    if let Some(f) = ne(cx, "folderUrl") {
        cx.verbose_log(&format!("Getting server-relative URL of folder '{f}'..."));
        let srv = urlutil::get_server_relative_path(&web, &urlutil::remove_trailing_slashes(&f));
        let r = Request::get(format!("{web}/_api/Web/GetFolderByServerRelativePath(decodedUrl='{}')/ListItemAllFields?$select=FileRef", eqp(&srv))).spo_json().data().await?;
        folder = match r.get("FileRef").filter(|v| truthy(v)) {
            Some(v) => to_js_string(v),
            None => return Err(Error::msg(format!("Folder '{srv}' was not found."))),
        };
    }
    cx.verbose_log("Getting default column values...");
    let text = match defaults(&web, &list, true).await? {
        Some(t) => t,
        None => {
            Request::post(format!("{web}/_api/Web/GetFolderByServerRelativePath(decodedUrl='{}')/Files/Add(url='client_LocationBasedDefaults.html', overwrite=false)", eqp(&format!("{list}/Forms"))))
                .spo_json()
                .header("content-type", "text/plain")
                .text("<MetadataDefaults />")
                .send()
                .await?;
            "<MetadataDefaults />".into()
        }
    };
    cx.verbose_log("Modifying default column values...");
    let encoded = folder.replace(' ', "%20");
    let mut doc = xml::parse(&text);
    let mut new_dv = Element::new("DefaultValue");
    new_dv.set_attr("FieldName", &field);
    new_dv.set_text(&value);
    let mut done = false;
    if let Some(root) = doc.root.as_mut() {
        for p in root.find_all("a") {
            let a = root.at_mut(&p).unwrap();
            if a.attr("href") != Some(encoded.as_str()) {
                continue;
            }
            let hit = a
                .find_all("DefaultValue")
                .into_iter()
                .find(|dp| a.at(dp).unwrap().attr("FieldName") == Some(field.as_str()));
            match hit {
                Some(dp) => a.at_mut(&dp).unwrap().set_text(&value),
                None => a.children.push(Node::Element(new_dv.clone())),
            }
            done = true;
            break;
        }
        if !done {
            let mut link = Element::new("a");
            link.set_attr("href", &encoded);
            link.children.push(Node::Element(new_dv));
            root.children.push(Node::Element(link));
        }
    }
    cx.verbose_log("Uploading default column values to list...");
    upload(&web, &list, xml::serialize(&doc), false).await
}

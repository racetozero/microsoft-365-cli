//! `spo listitem *`.

use crate::sp;
use crate::sp::{first, ne, opt};
use m365_core::prelude::*;
use m365_core::term::EOL;
use m365_core::ut::entra;
use m365_core::ut::url as urlutil;

/// `${webUrl}/_api/web` + `/lists(...)` | `/lists/getByTitle(...)` | `/GetList(...)`.
fn list_api(cx: &Cx) -> String {
    list_url(cx, "getByTitle")
}

fn list_label(cx: &Cx) -> String {
    first(cx, &sp::LIST)
}

fn undef(k: &str) -> Error {
    Error::js(format!(
        "Cannot read properties of undefined (reading '{k}')"
    ))
}

fn delete_id(v: &mut Value) {
    if let Some(o) = v.as_object_mut() {
        o.shift_remove("ID");
    }
}

async fn content_type_name(cx: &Cx, api: &str) -> Result<String> {
    let ct = cx.u("contentType");
    let r = Request::get(format!("{api}/contenttypes?$select=Name,Id"))
        .spo_json()
        .data()
        .await?;
    if cx.debug() && cx.has("id") {
        cx.stderr("content type lookup response...");
        m365_core::log::log_stderr_value(&r);
    }
    let mut found = Vec::new();
    for c in need_arr(&r, "value", "filter")? {
        let id = c.get("Id").ok_or_else(|| undef("StringValue"))?;
        let m = id.get("StringValue").and_then(Value::as_str) == Some(ct.as_str())
            || c.get("Name").and_then(Value::as_str) == Some(ct.as_str());
        if cx.debug() {
            cx.stderr(&format!(
                "Checking content type value [{}]: {m}",
                u(c, "Name")
            ));
        }
        if m {
            found.push(c.clone());
        }
    }
    if cx.debug() {
        cx.stderr("content type filter output...");
        m365_core::log::log_stderr_value(&Value::Array(found.clone()));
    }
    let name = found
        .first()
        .and_then(|f| f.get("Name"))
        .filter(|v| truthy(v))
        .map(to_js_string)
        .unwrap_or_default();
    if name.is_empty() {
        return Err(Error::msg(format!(
            "Specified content type '{ct}' doesn't exist on the target list"
        )));
    }
    cx.debug_log(&format!("using content type name: {name}"));
    Ok(name)
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let api = list_api(cx);
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Getting content types for list {}...",
        list_label(cx)
    ));
    let r = Request::get(format!("{api}/contenttypes?$select=Name,Id"))
        .spo_json()
        .data()
        .await?;
    let mut ct_name = String::new();
    if let Some(ct) = ne(cx, "contentType") {
        let mut found = Vec::new();
        for c in need_arr(&r, "value", "filter")? {
            let id = c.get("Id").ok_or_else(|| undef("StringValue"))?;
            let m = id.get("StringValue").and_then(Value::as_str) == Some(ct.as_str())
                || c.get("Name").and_then(Value::as_str) == Some(ct.as_str());
            if cx.debug() {
                cx.stderr(&format!(
                    "Checking content type value [{}]: {m}",
                    u(c, "Name")
                ));
            }
            if m {
                found.push(c.clone());
            }
        }
        if cx.debug() {
            cx.stderr("content type filter output...");
            m365_core::log::log_stderr_value(&Value::Array(found.clone()));
        }
        ct_name = found
            .first()
            .and_then(|f| f.get("Name"))
            .filter(|v| truthy(v))
            .map(to_js_string)
            .unwrap_or_default();
        if ct_name.is_empty() {
            return Err(Error::msg(format!(
                "Specified content type '{ct}' doesn't exist on the target list"
            )));
        }
        cx.debug_log(&format!("using content type name: {ct_name}"));
    }
    let mut folder = String::new();
    if let Some(f) = ne(cx, "folder") {
        cx.debug_log("setting up folder lookup response ...");
        let root = Request::get(format!("{api}/rootFolder"))
            .spo_json()
            .data()
            .await?;
        folder = urlutil::get_server_relative_path(&u(&root, "ServerRelativeUrl"), &f);
        sp::ensure_folder(cx, &web, &folder).await?;
    }
    cx.verbose_log(&format!(
        "Creating item in list {} in site {web}...",
        list_label(cx)
    ));
    const EXCLUDE: [&str; 10] = [
        "listTitle",
        "listId",
        "listUrl",
        "webUrl",
        "contentType",
        "folder",
        "debug",
        "verbose",
        "output",
        "_",
    ];
    let mut values: Vec<Value> = cx
        .opts
        .iter()
        .filter(|(k, _)| !EXCLUDE.contains(&k.as_str()))
        .map(|(k, v)| obj! { "FieldName": k.as_str(), "FieldValue": to_js_string(v) })
        .collect();
    if !ct_name.is_empty() {
        cx.debug_log(&format!(
            "Specifying content type name [{ct_name}] in request body"
        ));
        values.push(obj! { "FieldName": "ContentType", "FieldValue": ct_name.as_str() });
    }
    let mut body = obj! { "formValues": values };
    if ne(cx, "folder").is_some() {
        body["listItemCreateInfo"] = obj! { "FolderPath": obj! { "DecodedUrl": folder } };
    }
    let r = Request::post(format!("{api}/AddValidateUpdateItemUsingPath()"))
        .spo_json()
        .json(body)
        .data()
        .await?;
    let fields = need_arr(&r, "value", "some")?;
    let bad: Vec<String> = fields
        .iter()
        .filter(|f| f.get("HasException").is_some_and(truthy))
        .map(|f| format!("- {} - {}", u(f, "FieldName"), u(f, "ErrorMessage")))
        .collect();
    if !bad.is_empty() {
        return Err(Error::msg(format!(
            "Creating the item failed with the following errors: {EOL}{}",
            bad.join(EOL)
        )));
    }
    let id_field = fields
        .iter()
        .find(|f| f.get("FieldName").and_then(Value::as_str) == Some("Id"));
    if cx.debug() {
        cx.stderr("Field values returned:");
        m365_core::log::log_stderr_value(&Value::Array(fields.to_vec()));
        let Some(f) = id_field else {
            return Err(undef("FieldValue"));
        };
        cx.stderr(&format!(
            "Id returned by AddValidateUpdateItemUsingPath: {}",
            u(f, "FieldValue")
        ));
    }
    let Some(f) = id_field else {
        return Err(undef("FieldValue"));
    };
    let mut item = Request::get(format!("{api}/items({})", u(f, "FieldValue")))
        .spo_json()
        .data()
        .await?;
    delete_id(&mut item);
    cx.log(item)
}

fn file_name_for(path: &str, name: Option<String>, strip_dir: bool) -> String {
    let ext = path.rsplit('.').next().unwrap_or(path).to_string();
    match name {
        None if strip_dir => path.rsplit(['/', '\\']).next().unwrap_or(path).to_string(),
        n => {
            let mut n = n.unwrap_or_else(|| "undefined".into());
            if !n.ends_with(&format!(".{ext}")) {
                n.push('.');
                n.push_str(&ext);
            }
            n
        }
    }
}

fn read(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path)
        .map_err(|_| Error::js(format!("ENOENT: no such file or directory, open '{path}'")))
}

pub async fn attachment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding an attachment to list item with id {} on list {} on web {web}.",
        opt(cx, "listItemId"),
        list_label(cx)
    ));
    let path = cx.u("filePath");
    let name = file_name_for(&path, ne(cx, "fileName"), true);
    let body = read(&path)?;
    let url = format!(
        "{}/items({})/AttachmentFiles/add(FileName='{name}')",
        list_api(cx),
        opt(cx, "listItemId")
    );
    match Request::post(url).spo_json().bytes(body).data().await {
        Ok(r) => cx.log(r),
        Err(Error::Http(h)) => {
            let v = h
                .error
                .as_ref()
                .and_then(|e| e.get("odata.error"))
                .and_then(|e| e.get("message"))
                .and_then(|m| m.get("value"))
                .and_then(Value::as_str)
                .map(String::from);
            match v {
                Some(m) if m.contains("The document or folder name was not changed.") => {
                    Err(Error::msg(m.split('\n').next().unwrap_or("").to_string()))
                }
                _ => Err(Error::Http(h)),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn attachment_get(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{}/items({})/AttachmentFiles('{}')",
        list_api(cx),
        opt(cx, "listItemId"),
        opt(cx, "fileName")
    ))
    .spo_json()
    .data()
    .await?;
    cx.log(r)
}

pub async fn attachment_list(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!(
        "{}/items({})?$select=AttachmentFiles&$expand=AttachmentFiles",
        list_api(cx),
        opt(cx, "listItemId")
    ))
    .spo_json()
    .data()
    .await?;
    match r.get("AttachmentFiles") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn attachment_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let (name, id) = (opt(cx, "fileName"), opt(cx, "listItemId"));
    if !cx.proceed("force", &format!("Are you sure you want to remove the attachment {name} of item with id {id} from the list {} in site {web}?", list_label(cx)))? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing attachment {name} of item with id {id} from list {} in site at {web}...",
        list_label(cx)
    ));
    Request::post(format!(
        "{}/items({id})/AttachmentFiles('{name}')",
        list_api(cx)
    ))
    .header("X-HTTP-Method", "DELETE")
    .header("If-Match", "*")
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn attachment_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let path = cx.u("filePath");
    cx.verbose_log(&format!(
        "Updating attachment {} at path {path} for list item with id {} on list {} on web {web}.",
        opt(cx, "fileName"),
        opt(cx, "listItemId"),
        list_label(cx)
    ));
    let name = file_name_for(&path, cx.s("fileName"), false);
    let body = read(&path)?;
    Request::put(format!(
        "{}/items({})/AttachmentFiles('{name}')/$value",
        list_api(cx),
        opt(cx, "listItemId")
    ))
    .spo_json()
    .bytes(body)
    .send()
    .await?;
    Ok(())
}

// ---- batch ---------------------------------------------------------------------

fn csv_rows(cx: &Cx) -> Result<Vec<Value>> {
    let text = match ne(cx, "filePath") {
        Some(p) => String::from_utf8_lossy(&read(&p)?).into_owned(),
        None => cx.s("csvContent").unwrap_or_default(),
    };
    Ok(util::parse_csv_to_json(&text, '"', ',', false))
}

fn batch_head(batch: &str, changeset: &str) -> String {
    format!(
        "--batch_{batch}\nContent-Type: multipart/mixed; boundary=\"changeset_{changeset}\"\n\nContent-Transfer-Encoding: binary\n\n"
    )
}

fn batch_tail(batch: &str, changeset: &str) -> String {
    format!("\n\n--changeset_{changeset}--\n\n--batch_{batch}--\n")
}

async fn post_batch(web: &str, batch: &str, body: String) -> Result<String> {
    Ok(Request::post(format!("{web}/_api/$batch"))
        .header(
            "Content-Type",
            format!("multipart/mixed; boundary=batch_{batch}"),
        )
        .header("Accept", "application/json;odata=verbose")
        .text(body)
        .send()
        .await?
        .text())
}

pub async fn batch_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Starting to create batch items from csv {}",
        match ne(cx, "filePath") {
            Some(p) => format!("at path {p}"),
            None => "from content".into(),
        }
    ));
    let rows = csv_rows(cx)?;
    let url = format!("{}/AddValidateUpdateItemUsingPath", list_api(cx));
    let total = rows.len();
    let mut chunk: Vec<Vec<(String, String)>> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        let item: Vec<(String, String)> = row
            .as_object()
            .map(|o| {
                o.iter()
                    .map(|(k, v)| (k.clone(), to_js_string(v)))
                    .collect()
            })
            .unwrap_or_default();
        chunk.push(item);
        if chunk.len() == 100 {
            cx.verbose_log(&format!(
                "Writing away batch of items, currently at: {}/{total}.",
                i + 1
            ));
            send_add_batch(&web, &url, &chunk).await?;
            chunk.clear();
        }
    }
    if !chunk.is_empty() {
        cx.verbose_log(&format!("Writing away {} items.", chunk.len()));
        send_add_batch(&web, &url, &chunk).await?;
    }
    Ok(())
}

async fn send_add_batch(web: &str, url: &str, items: &[Vec<(String, String)>]) -> Result<()> {
    let (batch, cs) = (util::new_guid(), util::new_guid());
    let mut body = batch_head(&batch, &cs);
    for item in items {
        let fv: Vec<Value> = item.iter().map(|(k, v)| obj! { "FieldName": k.replace("\\\"", ""), "FieldValue": v.replace("\\\"", "") }).collect();
        body.push_str(&format!(
            "--changeset_{cs}\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\nPOST {url} HTTP/1.1\nAccept: application/json;odata=nometadata\nContent-Type: application/json;odata=verbose\nIf-Match: *\n\n{{\n\"formValues\": {}\n}}",
            json::stringify(&Value::Array(fv))
        ));
    }
    body.push_str(&batch_tail(&batch, &cs));
    let res = post_batch(web, &batch, body).await?;
    let mut results: Vec<(usize, Value)> = Vec::new();
    for (i, line) in res.split("\r\n").filter(|l| l.starts_with('{')).enumerate() {
        let p = json::jparse(line)?;
        if let Some(e) = p.get("error").filter(|e| truthy(e)) {
            let m = e.get("message").ok_or_else(|| undef("value"))?;
            return Err(Error::Thrown(
                m.get("value").cloned().unwrap_or(Value::Null),
            ));
        }
        for f in need_arr(&p, "value", "forEach")? {
            results.push((i + 2, f.clone()));
        }
    }
    let bad: Vec<String> = results
        .iter()
        .filter(|(_, f)| f.get("HasException").is_some_and(truthy))
        .map(|(n, f)| {
            format!(
                "- Line {n}: {} - {}",
                u(f, "FieldName"),
                u(f, "ErrorMessage")
            )
        })
        .collect();
    if !bad.is_empty() {
        return Err(Error::msg(format!(
            "Creating some items failed with the following errors: {EOL}{}",
            bad.join(EOL)
        )));
    }
    Ok(())
}

fn csv_id_key(rows: &[Value]) -> Result<Option<String>> {
    let first = rows
        .first()
        .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?;
    Ok(first
        .as_object()
        .and_then(|o| o.keys().find(|k| k.to_lowercase() == "id").cloned()))
}

pub fn batch_remove_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let s = to_js_string(v);
    if i == 0 {
        if !std::path::Path::new(&s).exists() {
            return Err(Some(format!("File with path {s} does not exist.")));
        }
        let text = std::fs::read_to_string(&s).unwrap_or_default();
        let rows = util::parse_csv_to_json(&text, '"', ',', false);
        let Ok(Some(key)) = csv_id_key(&rows) else {
            return Err(Some(
                "The file does not contain the required header row with the column name 'ID'."
                    .into(),
            ));
        };
        let ids: Vec<String> = rows
            .iter()
            .map(|r| {
                r.get(&key)
                    .map(to_js_string)
                    .unwrap_or_default()
                    .trim()
                    .to_string()
            })
            .collect();
        return match m365_core::validation::is_valid_positive_integer_array(&ids.join(",")) {
            m365_core::validation::Valid::Yes => Ok(()),
            m365_core::validation::Valid::No => Err(None),
            m365_core::validation::Valid::Msg(m) => Err(Some(format!(
                "The file contains one or more invalid IDs: '{m}'."
            ))),
        };
    }
    match m365_core::validation::is_valid_positive_integer_array(&s) {
        m365_core::validation::Valid::Yes => Ok(()),
        m365_core::validation::Valid::No => Err(None),
        m365_core::validation::Valid::Msg(m) => Err(Some(format!(
            "Option 'ids' contains one or more invalid IDs: '{m}'."
        ))),
    }
}

pub async fn batch_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let permanent = cx.bool("permanent");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to {} the list items from list {} located in site {web}?",
            if permanent {
                "permanently remove"
            } else {
                "recycle"
            },
            list_label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log("Removing the listitems from SharePoint...");
    let ids: Vec<String> = match ne(cx, "filePath") {
        Some(_) => {
            let rows = csv_rows(cx)?;
            let key = csv_id_key(&rows)?.unwrap_or_else(|| "undefined".into());
            let mut v = Vec::new();
            for r in &rows {
                match r.get(&key) {
                    Some(x) => v.push(to_js_string(x).trim().to_string()),
                    None => return Err(undef("toString")),
                }
            }
            v
        }
        None => util::split_and_trim(&cx.u("ids")),
    };
    let seg = sp::list_segment(cx, &web, sp::LIST, "getByTitle").unwrap_or_default();
    let chunks: Vec<&[String]> = ids.chunks(10).collect();
    for (i, chunk) in chunks.iter().enumerate() {
        cx.verbose_log(&format!(
            "Processing chunk {} of {}...",
            i + 1,
            chunks.len()
        ));
        let (batch, cs) = (util::new_guid(), util::new_guid());
        let mut body = batch_head(&batch, &cs);
        for item in chunk.iter() {
            let action = format!(
                "{seg}/items({item}){}",
                if permanent { "" } else { "/recycle()" }
            );
            body.push_str(&format!("--changeset_{cs}\nContent-Type: application/http\nContent-Transfer-Encoding: binary\n\nDELETE {action} HTTP/1.1\nAccept: application/json;odata=nometadata\nIf-Match: *\n\n"));
        }
        body.push_str(&batch_tail(&batch, &cs));
        let res = post_batch(&web, &batch, body).await?;
        let mut errors = Vec::new();
        for (j, line) in res.split("\r\n").filter(|l| l.starts_with('{')).enumerate() {
            let p = json::jparse(line)?;
            if let Some(e) = p.get("error").filter(|e| truthy(e)) {
                let m = e.get("message").ok_or_else(|| undef("value"))?;
                errors.push(format!(
                    "Item ID {}: {}",
                    chunk.get(j).cloned().unwrap_or_else(|| "undefined".into()),
                    m.get("value")
                        .map(to_js_string)
                        .unwrap_or_else(|| "undefined".into())
                ));
            }
        }
        if !errors.is_empty() {
            return Err(Error::msg(format!(
                "Creating some items failed with the following errors: {EOL}{}",
                errors
                    .iter()
                    .map(|e| format!("- {e}"))
                    .collect::<Vec<_>>()
                    .join(EOL)
            )));
        }
    }
    Ok(())
}

pub async fn batch_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let path = cx.u("filePath");
    cx.verbose_log(&format!(
        "Starting to create batch items from csv at path {path}"
    ));
    let mut rows =
        util::parse_csv_to_json(&String::from_utf8_lossy(&read(&path)?), '"', ',', false);
    let total = rows.len();
    let id_col = ne(cx, "idColumn").unwrap_or_else(|| "ID".into());
    let first = rows
        .first()
        .ok_or_else(|| Error::js("Cannot convert undefined or null to object"))?
        .clone();
    if !first.as_object().is_some_and(|o| o.contains_key(&id_col)) {
        return Err(Error::msg(format!(
            "The specified value for idColumn does not exist in the array. Specified idColumn is '{id_col}'. Please specify the correct value."
        )));
    }
    let list_id = match ne(cx, "listId") {
        Some(l) => l,
        None => {
            sp::list_id(
                cx,
                &web,
                ne(cx, "listTitle").as_deref(),
                ne(cx, "listUrl").as_deref(),
            )
            .await?
        }
    };
    cx.verbose_log(&format!("Retrieving fields for list with id {list_id}"));
    let keys: Vec<String> = first
        .as_object()
        .map(|o| o.keys().filter(|k| **k != id_col).cloned().collect())
        .unwrap_or_default();
    let filter: Vec<String> = keys
        .iter()
        .map(|k| format!("InternalName eq '{k}'"))
        .collect();
    let fields = odata::get_all_items(&format!(
        "{web}/_api/web/lists(guid'{}')/fields?$select=InternalName,TypeAsString&$filter={}",
        eqp(&list_id),
        filter.join(" or ")
    ))
    .await?;
    if fields.len() != keys.len() {
        let missing: Vec<&str> = keys
            .iter()
            .filter(|k| {
                !fields
                    .iter()
                    .any(|f| f.get("InternalName").and_then(Value::as_str) == Some(k.as_str()))
            })
            .map(String::as_str)
            .collect();
        return Err(Error::msg(format!(
            "Following fields specified in the csv do not exist on the list: {}",
            missing.join(", ")
        )));
    }
    let ty = |f: &Value| {
        f.get("TypeAsString")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    };
    let name = |f: &Value| {
        f.get("InternalName")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    };
    let user_fields: Vec<&Value> = fields
        .iter()
        .filter(|f| ty(f) == "UserMulti" || ty(f) == "User")
        .collect();
    let mut users: Vec<(String, Value)> = Vec::new();
    if !user_fields.is_empty() {
        let mut emails: Vec<String> = Vec::new();
        for uf in &user_fields {
            for row in &rows {
                if let Some(v) = row.get(name(uf)).filter(|v| v.as_str() != Some("")) {
                    for e in to_js_string(v).split(';') {
                        if !emails.iter().any(|x| x == e) {
                            emails.push(e.to_string());
                        }
                    }
                }
            }
        }
        for e in emails {
            let r = Request::post(format!("{web}/_api/web/ensureUser('{e}')?$select=Id"))
                .accept("application/json")
                .header("content-type", "application/json")
                .json_response()
                .data()
                .await?;
            users.push((e, r.get("Id").cloned().unwrap_or(Value::Null)));
        }
    }
    let d = sp::digest(&web).await?;
    let identity = sp::web_identity(&web, &d).await?.object_identity;
    let user_id = |email: &str| -> Result<String> {
        users
            .iter()
            .find(|(e, _)| e == email)
            .map(|(_, id)| {
                if id.is_null() {
                    "undefined".into()
                } else {
                    to_js_string(id)
                }
            })
            .ok_or_else(|| undef("id"))
    };
    let system = cx.bool("systemUpdate");
    let mut counter = 0;
    while !rows.is_empty() {
        let take = rows.len().min(50);
        let entries: Vec<Value> = rows.drain(..take).collect();
        let (mut paths, mut actions) = (String::new(), String::new());
        let mut index = 1;
        for row in &entries {
            counter += 1;
            paths.push_str(&format!(
                r#"<Identity Id="{index}" Name="{identity}:list:{list_id}:item:{},1" />"#,
                row.get(&id_col)
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into())
            ));
            let oid = index;
            for f in &fields {
                let n = name(f);
                let val = row.get(&n);
                index += 1;
                match val.filter(|v| v.as_str() != Some("")) {
                    None => actions.push_str(&format!(r#"<Method Name="ParseAndSetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter Type="String"></Parameter></Parameters></Method>"#)),
                    Some(v) => {
                        let s = to_js_string(v);
                        match ty(f).as_str() {
                            "User" => actions.push_str(&format!(r#"<Method Name="ParseAndSetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#, user_id(&s)?)),
                            "UserMulti" => {
                                let mut objs = String::new();
                                for e in s.split(';') {
                                    objs.push_str(&format!(r#"<Object TypeId="{{c956ab54-16bd-4c18-89d2-996f57282a6f}}"><Property Name="Email" Type="Null" /><Property Name="LookupId" Type="Int32">{}</Property><Property Name="LookupValue" Type="Null" /></Object>"#, user_id(e)?));
                                }
                                actions.push_str(&format!(r#"<Method Name="SetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter Type="Array">{objs}</Parameter></Parameters></Method>"#));
                            }
                            "Lookup" => actions.push_str(&format!(r#"<Method Name="SetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter TypeId="{{f1d34cc0-9b50-4a78-be78-d5facfcccfb7}}"><Property Name="LookupId" Type="Int32">{s}</Property><Property Name="LookupValue" Type="Null"/></Parameter></Parameters></Method>"#)),
                            "LookupMulti" => {
                                let objs: String = s.split(';').map(|e| format!(r#"<Object TypeId="{{f1d34cc0-9b50-4a78-be78-d5facfcccfb7}}"><Property Name="LookupId" Type="Int32">{e}</Property><Property Name="LookupValue" Type="Null" /></Object>"#)).collect();
                                actions.push_str(&format!(r#"<Method Name="SetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter Type="Array">{objs}</Parameter></Parameters></Method>"#));
                            }
                            _ => actions.push_str(&format!(r#"<Method Name="ParseAndSetFieldValue" Id="{index}" ObjectPathId="{oid}"><Parameters><Parameter Type="String">{n}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#, util::escape_xml_value(Some(&Value::from(s))))),
                        }
                    }
                }
            }
            index += 1;
            actions.push_str(&format!(
                r#"<Method Name="{}Update" Id="{index}" ObjectPathId="{oid}"/>"#,
                if system { "System" } else { "" }
            ));
        }
        cx.verbose_log(&format!(
            "Writing away batch of items, currently at: {counter}/{total}."
        ));
        let text = Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
            .header("Content-Type", "text/xml")
            .text(sp::request(&actions, &paths))
            .send()
            .await?
            .text();
        let json = sp::parse(&text)?;
        let f = json.first().ok_or_else(|| undef("ErrorInfo"))?;
        if let Some(e) = f.get("ErrorInfo").filter(|e| truthy(e)) {
            return Err(Error::msg(format!(
                "{} - {}",
                u(e, "ErrorMessage"),
                u(e, "ErrorValue")
            )));
        }
    }
    Ok(())
}

// ---- get / list ----------------------------------------------------------------

fn expand_fields(fields: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for f in fields.iter().filter(|f| f.contains('/')) {
        let p = f.split('/').next().unwrap_or("").to_string();
        if !out.contains(&p) {
            out.push(p);
        }
    }
    out
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let mut url = list_api(cx);
    let fields: Vec<String> = ne(cx, "fields")
        .map(|f| f.split(',').map(String::from).collect())
        .unwrap_or_default();
    let exp = expand_fields(&fields);
    let fexp = if exp.is_empty() {
        String::new()
    } else {
        format!("&$expand={}", exp.join(","))
    };
    match cx.val("id").filter(|v| truthy(v)) {
        Some(i) => url.push_str(&format!("/items({})", to_js_string(i))),
        None => url.push_str(&format!(
            "/GetItemByUniqueId(guid'{}')",
            opt(cx, "uniqueId")
        )),
    }
    let mut item = Request::get(format!("{url}?$select={}{fexp}", eqp(&fields.join(","))))
        .spo_json()
        .data()
        .await?;
    if cx.bool("withPermissions") {
        let r = Request::get(format!(
            "{url}/RoleAssignments?$expand=Member,RoleDefinitionBindings"
        ))
        .spo_json()
        .data()
        .await?;
        let mut v = need_arr(&r, "value", "forEach")?.to_vec();
        for ra in v.iter_mut() {
            let mut b = match ra.get("RoleDefinitionBindings") {
                Some(Value::Array(a)) => a.clone(),
                _ => return Err(undef("forEach")),
            };
            sp::set_friendly_permissions(&mut b)?;
            ra["RoleDefinitionBindings"] = Value::Array(b);
        }
        if let Some(o) = item.as_object_mut() {
            o.insert("RoleAssignments".into(), Value::Array(v));
        }
    }
    delete_id(&mut item);
    cx.log(item)
}

pub async fn isrecord(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let api = list_api(cx);
    cx.debug_log(&format!(
        "Retrieving access token for {}...",
        auth::resource_from_url(&web)
    ));
    let list_id = match cx.val("listId") {
        Some(l) => {
            cx.verbose_log("List Id passed in as an argument.");
            to_js_string(l)
        }
        None => {
            cx.verbose_log(&format!(
                "Getting list id for list {}",
                ne(cx, "listTitle")
                    .or_else(|| cx.s("listUrl"))
                    .unwrap_or_else(|| "undefined".into())
            ));
            u(
                &Request::get(format!("{api}?$select=Id"))
                    .spo_json()
                    .data()
                    .await?,
                "Id",
            )
        }
    };
    cx.debug_log("Getting request digest for request");
    let d = sp::digest(&web).await?;
    let identity = sp::web_identity(&web, &d).await?.object_identity;
    cx.verbose_log(&format!(
        "Checking if list item is a record in list {} in site {web}...",
        list_label(cx)
    ));
    let body = format!(
        "<Request AddExpandoFieldTypeSuffix=\"true\" SchemaVersion=\"15.0.0.0\" LibraryVersion=\"16.0.0.0\" ApplicationName=\"{}\" xmlns=\"http://schemas.microsoft.com/sharepoint/clientquery/2009\">\n            <Actions>\n              <StaticMethod TypeId=\"{{ea8e1356-5910-4e69-bc05-d0c30ed657fc}}\" Name=\"IsRecord\" Id=\"1\"><Parameters><Parameter ObjectPathId=\"14\" /></Parameters></StaticMethod>\n            </Actions>\n            <ObjectPaths>\n              <Identity Id=\"14\" Name=\"{identity}:list:{list_id}:item:{},1\" />\n            </ObjectPaths>\n          </Request>",
        m365_core::ut::spo::application_name(),
        opt(cx, "id")
    );
    let json = record_post(&web, &d, body).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

async fn record_post(web: &str, d: &str, body: String) -> Result<Vec<Value>> {
    let text = Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
        .header("Content-Type", "text/xml")
        .header("X-RequestDigest", d)
        .text(body)
        .send()
        .await?
        .text();
    sp::parse(&text)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let api = list_api(cx);
    let mut items = if ne(cx, "camlQuery").is_some() {
        items_by_caml(cx, &api).await?
    } else {
        items(cx, &api).await?
    };
    items.iter_mut().for_each(delete_id);
    cx.log(Value::Array(items))
}

async fn items(cx: &Cx, api: &str) -> Result<Vec<Value>> {
    cx.verbose_log("Getting list items");
    let fields: Vec<String> = match ne(cx, "fields") {
        Some(f) => f.split(',').map(String::from).collect(),
        None if cx.str("output").as_deref() == Some("text") => vec!["Title".into(), "Id".into()],
        None => Vec::new(),
    };
    let exp = expand_fields(&fields);
    let page_size = cx.num("pageSize").filter(|n| *n != 0.0);
    let page = cx.num("pageNumber").filter(|n| *n != 0.0);
    let skip = match page.filter(|p| *p > 0.0) {
        None => None,
        Some(p) => {
            cx.verbose_log(&format!(
                "Getting skipToken Id for page {}",
                json::number_to_string(p)
            ));
            let top = page_size.unwrap_or(f64::NAN) * p;
            let filter = ne(cx, "filter")
                .map(|f| format!("$filter={}", enc(&f)))
                .unwrap_or_default();
            let r = Request::get(format!(
                "{api}/items?$select=Id&$top={}&{filter}",
                json::number_to_string(top)
            ))
            .spo_json()
            .data()
            .await?;
            let v = match r.get("value") {
                Some(Value::Array(a)) => a,
                _ => return Err(undef("length")),
            };
            v.last().and_then(|l| l.get("Id")).cloned()
        }
    };
    let mut q = vec![format!(
        "$top={}",
        page_size
            .map(json::number_to_string)
            .unwrap_or_else(|| "5000".into())
    )];
    if let Some(f) = ne(cx, "filter") {
        q.push(format!("$filter={}", enc(&f)));
    }
    if !exp.is_empty() {
        q.push(format!("$expand={}", exp.join(",")));
    }
    if !fields.is_empty() {
        q.push(format!("$select={}", eqp(&fields.join(","))));
    }
    if let Some(s) = &skip {
        q.push(format!("$skiptoken=Paged=TRUE%26p_ID={}", to_js_string(s)));
    }
    if page.is_some_and(|p| p > 0.0) && skip.is_none() {
        return Ok(Vec::new());
    }
    let url = format!("{api}/items?{}", q.join("&"));
    if page_size.is_none() {
        odata::get_all_items(&url).await
    } else {
        let r = Request::get(url).spo_json().data().await?;
        Ok(r.get("value")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default())
    }
}

async fn items_by_caml(cx: &Cx, api: &str) -> Result<Vec<Value>> {
    let d = sp::digest(&cx.u("webUrl")).await?;
    cx.verbose_log("Getting list items using CAML query");
    let mut out = Vec::new();
    let mut skip: Option<Value> = None;
    loop {
        let mut q =
            obj! { "ViewXml": ?cx.val("camlQuery").cloned(), "AllowIncrementalResults": true };
        if let Some(s) = &skip {
            q["ListItemCollectionPosition"] =
                obj! { "PagingInfo": format!("Paged=TRUE&p_ID={}", to_js_string(s)) };
        }
        let r = Request::post(format!("{api}/GetItems"))
            .spo_json()
            .header("X-RequestDigest", d.clone())
            .json(obj! { "query": q })
            .data()
            .await?;
        let v = need_arr(&r, "value", "length")?;
        skip = v
            .last()
            .map(|l| l.get("Id").cloned().unwrap_or(Value::Null))
            .filter(|x| !x.is_null());
        out.extend(v.iter().cloned());
        if skip.is_none() {
            break;
        }
    }
    Ok(out)
}

// ---- records -------------------------------------------------------------------

async fn list_id_for_record(cx: &Cx) -> Result<String> {
    match ne(cx, "listId") {
        Some(l) => Ok(l),
        None => Ok(u(
            &Request::get(format!("{}?$select=Id", list_api(cx)))
                .spo_json()
                .data()
                .await?,
            "Id",
        )),
    }
}

pub async fn record_declare(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let list_id = list_id_for_record(cx).await?;
    let d = sp::digest(&web).await?;
    let identity = sp::web_identity(&web, &d).await?.object_identity;
    let id = opt(cx, "listItemId");
    let date = cx.s("date").unwrap_or_default();
    let body = if date.encode_utf16().count() == 10 {
        sp::request(
            &format!(
                r#"<StaticMethod TypeId="{{ea8e1356-5910-4e69-bc05-d0c30ed657fc}}" Name="DeclareItemAsRecordWithDeclarationDate" Id="48"><Parameters><Parameter ObjectPathId="21" /><Parameter Type="DateTime">{date}</Parameter></Parameters></StaticMethod>"#
            ),
            &format!(r#"<Identity Id="21" Name="{identity}:list:{list_id}:item:{id},1" />"#),
        )
    } else {
        sp::request(
            r#"<StaticMethod TypeId="{ea8e1356-5910-4e69-bc05-d0c30ed657fc}" Name="DeclareItemAsRecord" Id="37"><Parameters><Parameter ObjectPathId="12" /></Parameters></StaticMethod>"#,
            &format!(r#"<Identity Id="12" Name="{identity}:list:{list_id}:item:{id},1" />"#),
        )
    };
    let json = record_post(&web, &d, body).await?;
    sp::check_first(&json)?;
    cx.log(json.last().cloned().unwrap_or(Value::Null))
}

pub async fn record_undeclare(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let list_id = list_id_for_record(cx).await?;
    cx.debug_log("getting request digest for request");
    let d = sp::digest(&web).await?;
    let identity = sp::web_identity(&web, &d).await?.object_identity;
    cx.verbose_log(&format!(
        "Undeclare list item as a record in list {} in site {web}...",
        list_label(cx)
    ));
    let body = sp::request(
        r#"<StaticMethod TypeId="{ea8e1356-5910-4e69-bc05-d0c30ed657fc}" Name="UndeclareItemAsRecord" Id="53"><Parameters><Parameter ObjectPathId="49" /></Parameters></StaticMethod>"#,
        &format!(
            r#"<Identity Id="49" Name="{identity}:list:{list_id}:item:{},1" />"#,
            opt(cx, "listItemId")
        ),
    );
    Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
        .header("Content-Type", "text/xml")
        .header("X-RequestDigest", d)
        .text(body)
        .send()
        .await?;
    Ok(())
}

async fn lock(cx: &Cx, verb: &str, method: &str) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "{verb} the list item record {} in site at {web}...",
        list_label(cx)
    ));
    let srv = match ne(cx, "listUrl") {
        Some(l) => urlutil::get_server_relative_path(&web, &l),
        None => {
            let seg = match ne(cx, "listId") {
                Some(i) => format!("lists(guid'{}')/", eqp(&i)),
                None => format!(
                    "lists/getByTitle('{}')/",
                    eqp(&cx.s("listTitle").unwrap_or_else(|| "undefined".into()))
                ),
            };
            let r = Request::get(format!(
                "{web}/_api/web/{seg}?$expand=RootFolder&$select=RootFolder"
            ))
            .spo_json()
            .data()
            .await?;
            match dig(&r, &["RootFolder", "ServerRelativeUrl"])? {
                Some(v) => to_js_string(v),
                None => "undefined".into(),
            }
        }
    };
    let abs = urlutil::get_absolute_url(&web, &srv);
    Request::post(format!(
        "{web}/_api/SP.CompliancePolicy.SPPolicyStoreProxy.{method}()"
    ))
    .spo_json()
    .json(obj! { "listUrl": abs, "itemId": ?cx.val("listItemId").cloned() })
    .send()
    .await?;
    Ok(())
}

pub async fn record_lock(cx: &mut Cx) -> Result<()> {
    lock(cx, "Locking", "LockRecordItem").await
}

pub async fn record_unlock(cx: &mut Cx) -> Result<()> {
    lock(cx, "Unlocking", "UnlockRecordItem").await
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let permanent = cx.bool("permanent");
    let id = opt(cx, "id");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to {} the list item {id} from list {} located in site {web}?",
            if permanent {
                "permanently remove"
            } else {
                "recycle"
            },
            list_label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing list item {id} from list {} in site at {web}...",
        list_label(cx)
    ));
    let url = format!(
        "{}/items({id}){}",
        list_api(cx),
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

// ---- retention labels ----------------------------------------------------------

async fn list_absolute_url(cx: &Cx) -> Result<String> {
    let web = cx.u("webUrl");
    let parsed = url::Url::parse(&web).map_err(|_| Error::js("Invalid URL"))?;
    let tenant = format!(
        "{}//{}",
        parsed.scheme().to_string() + ":",
        parsed.host_str().unwrap_or("")
    );
    if let Some(l) = ne(cx, "listUrl") {
        return Ok(urlutil::url_combine(
            &tenant,
            &urlutil::get_server_relative_path(&web, &l),
        ));
    }
    cx.verbose_log("Retrieving list absolute URL...");
    let r = Request::get(format!(
        "{}?$expand=RootFolder&$select=RootFolder/ServerRelativeUrl",
        list_api(cx)
    ))
    .spo_json()
    .data()
    .await?;
    let srv = match dig(&r, &["RootFolder", "ServerRelativeUrl"])? {
        Some(v) => to_js_string(v),
        None => "undefined".into(),
    };
    let abs = urlutil::url_combine(&tenant, &urlutil::get_server_relative_path(&web, &srv));
    cx.verbose_log(&format!("List absolute URL found: '{abs}'"));
    Ok(abs)
}

pub async fn retentionlabel_ensure(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let abs = list_absolute_url(cx).await?;
    let label = match ne(cx, "name") {
        Some(n) => n,
        None => {
            let id = cx.s("id");
            cx.verbose_log(&format!(
                "Retrieving the name of the retention label based on the Id '{}'...",
                id.clone().unwrap_or_else(|| "undefined".into())
            ));
            let labels = odata::get_all_items(&format!("{web}/_api/SP.CompliancePolicy.SPPolicyStoreProxy.GetAvailableTagsForSite(siteUrl=@a1)?@a1='{}'", eqp(&web))).await?;
            let Some(l) = labels
                .iter()
                .find(|l| l.get("TagId").and_then(Value::as_str) == id.as_deref())
            else {
                return Err(Error::js(
                    "The specified retention label does not exist or is not published to this SharePoint site. Use the name of the label if you want to apply an unpublished label.",
                ));
            };
            cx.verbose_log(&format!(
                "Retention label found in the list of available labels: '{}' / '{}'...",
                u(l, "TagName"),
                u(l, "TagId")
            ));
            u(l, "TagName")
        }
    };
    if let Some(asset) = ne(cx, "assetId") {
        cx.verbose_log(&format!("Applying the asset Id {asset}..."));
        let id = opt(cx, "listItemId");
        let url = if let Some(l) = ne(cx, "listId") {
            format!(
                "{web}/_api/web/lists(guid'{}')/items({id})/ValidateUpdateListItem()",
                eqp(&l)
            )
        } else if let Some(t) = ne(cx, "listTitle") {
            format!(
                "{web}/_api/web/lists/getByTitle('{}')/items({id})/ValidateUpdateListItem()",
                eqp(&t)
            )
        } else {
            format!(
                "{web}/_api/web/GetList(@listUrl)/items({id})/ValidateUpdateListItem()?@listUrl='{}'",
                eqp(&urlutil::get_server_relative_path(&web, &cx.u("listUrl")))
            )
        };
        Request::post(url).spo_json().json(obj! { "formValues": vec![obj! { "FieldName": "ComplianceAssetId", "FieldValue": asset }] }).send().await?;
    }
    let ids = vec![cx.val("listItemId").cloned().unwrap_or(Value::Null)];
    sp::set_compliance_tag_on_items(
        cx,
        &web,
        &label,
        &abs,
        ids,
        cx.val("verbose").is_some_and(truthy),
    )
    .await
}

pub async fn retentionlabel_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    if !cx.proceed("force", &format!("Are you sure you want to remove the retentionlabel from list item {} from list '{}' located in site {web}?", opt(cx, "listItemId"), list_label(cx)))? {
        return Ok(());
    }
    let abs = list_absolute_url(cx).await?;
    let ids = vec![cx.val("listItemId").cloned().unwrap_or(Value::Null)];
    sp::set_compliance_tag_on_items(
        cx,
        &web,
        "",
        &abs,
        ids,
        cx.val("verbose").is_some_and(truthy),
    )
    .await
}

// ---- role assignments ----------------------------------------------------------

fn item_rest(cx: &Cx) -> String {
    let web = cx.u("webUrl");
    format!(
        "{web}/_api/web/{}items({})/",
        sp::list_segment(cx, &web, sp::LIST, "getByTitle")
            .map(|s| format!("{s}/"))
            .unwrap_or_default(),
        opt(cx, "listItemId")
    )
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
            None => entra::group::by_display_name(&cx.u("entraGroupName"), None).await?,
        };
        return Ok(u(&sp::ensure_entra_group(&web, &g).await?, "Id"));
    }
    Ok(opt(cx, "principalId"))
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding role assignment to listitem in site at {web}..."
    ));
    let url = item_rest(cx);
    let role = match ne(cx, "roleDefinitionName") {
        Some(n) => u(&sp::role_definition_by_name(cx, &web, &n).await?, "Id"),
        None => opt(cx, "roleDefinitionId"),
    };
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}roleassignments/addroleassignment(principalid='{p}',roledefid='{role}')"
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleassignment_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let list = ne(cx, "listId")
        .or_else(|| ne(cx, "listTitle"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed("force", &format!("Are you sure you want to remove role assignment from listitem {} from list {list} from site {web}?", opt(cx, "listItemId")))? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing role assignment from listitem in site at {web}..."
    ));
    let url = item_rest(cx);
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}roleassignments/removeroleassignment(principalid='{p}')"
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

/// `${webUrl}/_api/web`, followed by the list segment when a list is named.
fn list_url(cx: &Cx, by_title: &str) -> String {
    let web = cx.u("webUrl");
    match sp::list_segment(cx, &web, sp::LIST, by_title) {
        Some(seg) => format!("{web}/_api/web/{seg}"),
        None => format!("{web}/_api/web"),
    }
}

fn nullish_list(cx: &Cx) -> String {
    cx.val("listId")
        .or(cx.val("listTitle"))
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

pub async fn roleinheritance_break(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Breaking role inheritance of list item in site at {}...",
        cx.u("webUrl")
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to break the role inheritance of {} in list {}?",
            opt(cx, "listItemId"),
            nullish_list(cx)
        ),
    )? {
        return Ok(());
    }
    let keep = !cx.bool("clearExistingPermissions");
    Request::post(format!(
        "{}/items({})/breakroleinheritance({keep})",
        list_url(cx, "getbytitle"),
        opt(cx, "listItemId")
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn roleinheritance_reset(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Restore role inheritance of list item in site at {}...",
        cx.u("webUrl")
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to reset the role inheritance of {} in list {}?",
            opt(cx, "listItemId"),
            nullish_list(cx)
        ),
    )? {
        return Ok(());
    }
    Request::post(format!(
        "{}/items({})/resetroleinheritance",
        list_url(cx, "getbytitle"),
        opt(cx, "listItemId")
    ))
    .spo_json()
    .header("content-type", "application/json")
    .send()
    .await?;
    Ok(())
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let api = list_api(cx);
    let mut ct_name = String::new();
    if ne(cx, "contentType").is_some() {
        cx.verbose_log("Getting content types for list...");
        ct_name = content_type_name(cx, &api).await?;
    }
    const EXCLUDE: [&str; 16] = [
        "listTitle",
        "listId",
        "listUrl",
        "webUrl",
        "id",
        "contentType",
        "systemUpdate",
        "debug",
        "verbose",
        "output",
        "s",
        "i",
        "o",
        "u",
        "t",
        "_",
    ];
    let props: Obj = cx
        .opts
        .iter()
        .filter(|(k, _)| !EXCLUDE.contains(&k.as_str()))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let id = cx.u("id");
    let ct = Some(ct_name.as_str()).filter(|c| !c.is_empty());
    let mut item = if cx.bool("systemUpdate") {
        let n = json::js_number(&id);
        sp::system_update_list_item(cx, &api, &json::number_to_string(n), Some(&props), ct).await?
    } else {
        sp::update_list_item(&api, &id, Some(&props), ct).await?
    };
    delete_id(&mut item);
    cx.log(item)
}

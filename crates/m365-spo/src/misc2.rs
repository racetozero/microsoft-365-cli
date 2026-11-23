//! `spo externaluser list`, `spo get`, `spo set`, `spo search`, `spo agent add`,
//! `spo apppage *` and `spo brandcenter *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::spo;
use m365_core::ut::url as urlutil;

fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

fn undef(k: &str) -> Error {
    Error::js(format!(
        "Cannot read properties of undefined (reading '{k}')"
    ))
}

pub async fn externaluser_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log("Retrieving information about external users...");
    let int = |k: &str, def: &str| {
        json::parse_int(&ne(cx, k).unwrap_or_else(|| def.into()))
            .map(|n| n.to_string())
            .unwrap_or_else(|| "NaN".into())
    };
    let (pos, size) = (int("position", "0"), int("pageSize", "10"));
    let order = if cx.str("sortOrder").as_deref() == Some("desc") {
        1
    } else {
        0
    };
    let filter = util::escape_xml(&ne(cx, "filter").unwrap_or_default());
    let props = r#"<Query SelectAllProperties="false"><Properties><Property Name="TotalUserCount" ScalarProperty="true" /><Property Name="UserCollectionPosition" ScalarProperty="true" /><Property Name="ExternalUserCollection"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="false"><Properties><Property Name="DisplayName" ScalarProperty="true" /><Property Name="InvitedAs" ScalarProperty="true" /><Property Name="UniqueId" ScalarProperty="true" /><Property Name="AcceptedAs" ScalarProperty="true" /><Property Name="WhenCreated" ScalarProperty="true" /><Property Name="InvitedBy" ScalarProperty="true" /></Properties></ChildItemQuery></Property></Properties></Query>"#;
    let body = match ne(cx, "siteUrl") {
        Some(site) => sp::request(
            &format!(
                r#"<ObjectPath Id="135" ObjectPathId="134" /><Query Id="136" ObjectPathId="134">{props}</Query>"#
            ),
            &format!(
                r#"<Method Id="134" ParentId="131" Name="GetExternalUsersForSite"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Int32">{pos}</Parameter><Parameter Type="Int32">{size}</Parameter><Parameter Type="String">{filter}</Parameter><Parameter Type="Enum">{order}</Parameter></Parameters></Method><Constructor Id="131" TypeId="{{e45fd516-a408-4ca4-b6dc-268e2f1f0f83}}" />"#,
                util::escape_xml(&site)
            ),
        ),
        None => sp::request(
            &format!(
                r#"<ObjectPath Id="109" ObjectPathId="108" /><Query Id="110" ObjectPathId="108">{props}</Query>"#
            ),
            &format!(
                r#"<Method Id="108" ParentId="105" Name="GetExternalUsers"><Parameters><Parameter Type="Int32">{pos}</Parameter><Parameter Type="Int32">{size}</Parameter><Parameter Type="String">{filter}</Parameter><Parameter Type="Enum">{order}</Parameter></Parameters></Method><Constructor Id="105" TypeId="{{e45fd516-a408-4ca4-b6dc-268e2f1f0f83}}" />"#
            ),
        ),
    };
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let last = json.last().cloned().unwrap_or(Value::Null);
    let items = match dig(&last, &["ExternalUserCollection", "_Child_Items_"])? {
        Some(Value::Array(a)) => a.clone(),
        _ => return Err(undef("map")),
    };
    let out: Vec<Value> = items
        .into_iter()
        .map(|mut e| {
            if let Some(o) = e.as_object_mut() {
                o.shift_remove("_ObjectType_");
                if let Some(Value::String(w)) = o.get("WhenCreated") {
                    let d = util::extract_csom_date(w);
                    o.insert("WhenCreated".into(), Value::from(d));
                }
            }
            e
        })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    cx.log(obj! { "SpoUrl": auth::connection().spo_url.unwrap_or_default() })
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let url = urlutil::remove_trailing_slashes(&cx.u("url"));
    auth::with_connection(|c| c.spo_url = Some(url));
    auth::store_connection_info()
}

// ---- search --------------------------------------------------------------------

fn select_props(cx: &Cx) -> Vec<String> {
    // upstream's `filter(...).push('DocId')` for allResults has no effect
    match ne(cx, "selectProperties") {
        Some(s) => s.split(',').map(String::from).collect(),
        None => vec!["Title".into(), "OriginalPath".into()],
    }
}

fn relevant(r: &Value) -> Result<&Value> {
    dig(r, &["PrimaryQueryResult", "RelevantResults"])?.ok_or_else(|| undef("RowCount"))
}

fn search_url(cx: &Cx, web: &str, last_doc: &str) -> String {
    let all = cx.bool("allResults");
    let opt = |k: &str| cx.val(k).filter(|v| truthy(v)).map(to_js_string);
    let def = |k: &str| cx.val(k).map(to_js_string);
    let mut sort = Vec::new();
    if all {
        sort.push(eqp("[DocId]:ascending"));
    }
    if let Some(s) = ne(cx, "sortList") {
        sort.extend(s.split(',').map(eqp));
    }
    let sort = sort.join(",");
    let mut props = ne(cx, "properties").unwrap_or_default();
    if let Some(sn) = ne(cx, "sourceName") {
        if !props.is_empty() && !props.ends_with(',') {
            props.push(',');
        }
        props.push_str(&format!("SourceName:{sn},SourceLevel:SPSite"));
    }
    let mut u = format!(
        "{web}/_api/search/query?querytext='{}{}'",
        cx.u("queryText"),
        if all {
            format!(" IndexDocId>{last_doc}")
        } else {
            String::new()
        }
    );
    u.push_str(&format!(
        "&selectproperties='{}'",
        eqp(&select_props(cx).join(","))
    ));
    u.push_str(&format!(
        "&startrow={}",
        opt("startRow").unwrap_or_else(|| "0".into())
    ));
    if let Some(r) = opt("rowLimit") {
        u.push_str(&format!("&rowlimit={r}"));
    } else if all {
        u.push_str("&rowlimit=500");
    }
    let q = |u: &mut String, k: &str, name: &str| {
        if let Some(v) = opt(k) {
            u.push_str(&format!("&{name}='{v}'"));
        }
    };
    q(&mut u, "sourceId", "sourceid");
    u.push_str(&format!(
        "&trimduplicates={}",
        opt("trimDuplicates").unwrap_or_else(|| "false".into())
    ));
    u.push_str(&format!(
        "&enablestemming={}",
        def("enableStemming").unwrap_or_else(|| "true".into())
    ));
    if let Some(c) = opt("culture") {
        u.push_str(&format!("&culture={c}"));
    }
    q(&mut u, "refinementFilters", "refinementfilters");
    q(&mut u, "queryTemplate", "querytemplate");
    if !sort.is_empty() {
        u.push_str(&format!("&sortList='{sort}'"));
    }
    q(&mut u, "rankingModelId", "rankingmodelid");
    if !props.is_empty() {
        u.push_str(&format!("&properties='{props}'"));
    }
    q(&mut u, "refiners", "refiners");
    q(&mut u, "hiddenConstraints", "hiddenconstraints");
    q(&mut u, "clientType", "clienttype");
    for (k, n) in [
        ("enablePhonetic", "enablephonetic"),
        ("processBestBets", "processbestbets"),
        ("enableQueryRules", "enablequeryrules"),
        ("processPersonalFavorites", "processpersonalfavorites"),
    ] {
        if let Some(v) = def(k) {
            u.push_str(&format!("&{n}={v}"));
        }
    }
    u
}

pub async fn search(cx: &mut Cx) -> Result<()> {
    let web = match ne(cx, "webUrl") {
        Some(w) => w,
        None => spo::get_spo_url(cx.debug()).await?,
    };
    cx.verbose_log(&format!(
        "Executing search query '{}' on site at {web}...",
        cx.u("queryText")
    ));
    let mut results: Vec<Value> = Vec::new();
    let mut last_doc = "0".to_string();
    let row_limit = cx.num("rowLimit").filter(|n| *n != 0.0).unwrap_or(500.0);
    loop {
        let url = search_url(cx, &web, &last_doc);
        cx.debug_log(&format!("RequestURL: {url}"));
        let r = Request::get(url).spo_json().data().await?;
        results.push(r.clone());
        if !cx.bool("allResults") {
            break;
        }
        let rel = relevant(&r)?;
        let count = rel.get("RowCount").and_then(Value::as_f64);
        if count != Some(row_limit) {
            break;
        }
        let total = relevant(&results[0])?
            .get("TotalRows")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        cx.verbose_log(&format!(
            "Processing search query, retrieved {} of {total} items...",
            json::number_to_string(results.len() as f64 * row_limit)
        ));
        let rows = dig(rel, &["Table", "Rows"])?
            .ok_or_else(|| undef(&format!("{}", row_limit as i64 - 1)))?;
        let row = rows
            .as_array()
            .and_then(|a| a.get(row_limit as usize - 1))
            .ok_or_else(|| undef("Cells"))?;
        let cells = need_arr(row, "Cells", "filter")?;
        let doc = cells
            .iter()
            .find(|c| c.get("Key").and_then(Value::as_str) == Some("DocId"))
            .ok_or_else(|| undef("Value"))?;
        last_doc = doc
            .get("Value")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
    }
    if cx.bool("rawOutput") {
        cx.log(Value::Array(results.clone()))?;
    } else {
        let sel: Vec<String> = select_props(cx).iter().map(|s| s.to_uppercase()).collect();
        let mut out = Vec::new();
        for r in &results {
            let rows = dig(
                r,
                &["PrimaryQueryResult", "RelevantResults", "Table", "Rows"],
            )?;
            let rows = match rows {
                Some(Value::Array(a)) => a.clone(),
                _ => {
                    return Err(Error::js(
                        "Spread syntax requires ...iterable[Symbol.iterator] to be a function",
                    ));
                }
            };
            for row in rows {
                let mut o = Obj::new();
                for c in need_arr(&row, "Cells", "map")? {
                    let k = match c.get("Key") {
                        Some(Value::String(k)) => k.clone(),
                        _ => return Err(undef("toUpperCase")),
                    };
                    if sel.contains(&k.to_uppercase()) {
                        o.insert(k, c.get("Value").cloned().unwrap_or(Value::Null));
                    }
                }
                out.push(Value::Object(o));
            }
        }
        cx.log(Value::Array(out))?;
    }
    if cx.str("output").as_deref() == Some("text") {
        let last = results.last().unwrap();
        let rel = relevant(last)?;
        let f = |k: &str| {
            rel.get(k)
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into())
        };
        cx.log(Value::from(format!("# Rows: {}", f("TotalRows"))))?;
        cx.log(Value::from(format!(
            "# Rows (Including duplicates): {}",
            f("TotalRowsIncludingDuplicates")
        )))?;
        let mut total = 0.0;
        for r in &results {
            total += r
                .get("ElapsedTime")
                .and_then(Value::as_f64)
                .unwrap_or(f64::NAN);
        }
        cx.log(Value::from(format!(
            "Elapsed Time: {}",
            json::number_to_string(total)
        )))?;
    }
    Ok(())
}

// ---- agent ---------------------------------------------------------------------

const ICON: &str = include_str!("agent-icon.txt");

fn cell(row: &Value, key: &str) -> Result<String> {
    let cells = need_arr(row, "Cells", "find")?;
    Ok(cells
        .iter()
        .find(|c| c.get("Key").and_then(Value::as_str) == Some(key))
        .and_then(|c| c.get("Value"))
        .filter(|v| truthy(v))
        .map(to_js_string)
        .unwrap_or_default())
}

pub async fn agent_add(cx: &mut Cx) -> Result<()> {
    let (web, name) = (cx.u("webUrl"), cx.u("name"));
    cx.verbose_log(&format!(
        "Adding SharePoint agent '{name}' to site '{web}'..."
    ));
    cx.verbose_log(&format!("Ensuring Site Assets library exists at {web}..."));
    Request::post(format!("{web}/_api/web/lists/EnsureSiteAssetsLibrary()"))
        .header("Accept", "application/json;odata=nometadata")
        .json_response()
        .send()
        .await?;
    sp::ensure_folder(cx, &web, "SiteAssets/Copilots").await?;
    let (mut files, mut urls) = (Vec::new(), Vec::new());
    for src in cx.u("sourceUrls").split(',') {
        cx.verbose_log(&format!("Resolving source URL: {src}"));
        let data = obj! { "request": obj! {
            "QueryTemplate": "({searchterms}) (contentclass:STS_Web OR contentclass:STS_Site OR contentclass:STS_ListItem_DocumentLibrary OR contentclass:STS_List_DocumentLibrary)",
            "Querytext": format!("Path=\"{src}\""),
            "SelectProperties": ["contentclass", "Title", "Path", "SiteName", "SiteTitle", "ListID", "ListItemID", "SiteID", "WebId", "UniqueID", "IsDocument", "IsContainer"],
            "RowLimit": 1,
            "TrimDuplicates": false,
        } };
        let r = Request::post(format!("{web}/_api/search/postquery"))
            .header("Accept", "application/json;odata=nometadata")
            .json_response()
            .json(data)
            .data()
            .await?;
        let rows = dig(
            &r,
            &["PrimaryQueryResult", "RelevantResults", "Table", "Rows"],
        )?;
        let rows = match rows {
            Some(Value::Array(a)) => a,
            Some(_) => return Err(undef("0")),
            None => return Err(undef("length")),
        };
        if rows.is_empty() {
            cx.stderr(&format!(
                "{src} has been skipped because no results were found."
            ));
            continue;
        }
        let row = &rows[0];
        let is_container = cell(row, "IsContainer")?;
        let mut unique = cell(row, "UniqueID")?;
        if unique.starts_with('{') && unique.ends_with('}') && unique.len() >= 2 {
            unique = unique[1..unique.len() - 1].to_string();
        }
        let cc = cell(row, "contentclass")?;
        let ty = if is_container == "true" && cc == "STS_ListItem_DocumentLibrary" {
            Some("Folder")
        } else {
            match cc.as_str() {
                "STS_ListItem_DocumentLibrary" => Some("File"),
                "STS_List_DocumentLibrary" => Some("List"),
                "STS_Web" | "STS_Site" => Some("Site"),
                _ => None,
            }
        };
        let item = obj! {
            "url": src, "name": cell(row, "Title")?, "site_id": cell(row, "SiteID")?, "web_id": cell(row, "WebId")?,
            "list_id": cell(row, "ListID")?, "unique_id": unique, "type": ?ty,
        };
        if ty == Some("File") {
            files.push(item)
        } else {
            urls.push(item)
        }
    }
    let starters: Vec<Value> = ne(cx, "conversationStarters")
        .map(|s| s.split(',').map(|t| obj! { "text": t }).collect())
        .unwrap_or_default();
    let body = obj! {
        "schemaVersion": "0.2.0",
        "customCopilotConfig": obj! {
            "conversationStarters": obj! { "conversationStarterList": starters, "welcomeMessage": obj! { "text": ?cx.val("welcomeMessage").cloned() } },
            "gptDefinition": obj! {
                "name": name.as_str(), "description": ?cx.val("description").cloned(), "instructions": ?cx.val("agentInstructions").cloned(),
                "capabilities": vec![obj! { "name": "OneDriveAndSharePoint", "items_by_sharepoint_ids": files, "items_by_url": urls }],
            },
            "icon": ne(cx, "icon").unwrap_or_else(|| ICON.to_string()),
        },
    };
    let path = urlutil::get_server_relative_path(&web, "/SiteAssets/Copilots/");
    let r = Request::post(format!("{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{path}')/Files/AddUsingPath(DecodedUrl='{}.agent',EnsureUniqueFileName=true,AutoCheckoutOnInvalidData=true)", eqp(&name)))
        .header("Accept", "application/json;odata=nometadata")
        .header("Content-Type", "application/json;odata=nometadata")
        .json_response()
        .json(body)
        .data()
        .await?;
    cx.verbose_log(&format!("Agent '{name}' has been successfully created."));
    cx.log(r)
}

// ---- apppage -------------------------------------------------------------------

pub async fn apppage_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let page = Request::post(format!("{web}/_api/sitepages/Pages/CreateAppPage"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(obj! { "webPartDataAsJson": ?cx.val("webPartData").cloned() })
        .data()
        .await?;
    let page_url = page
        .get("value")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let file = Request::get(format!("{web}/_api/web/GetFileByServerRelativePath(DecodedUrl='{}/{page_url}')?$expand=ListItemAllFields", urlutil::get_server_relative_site_url(&web)))
        .header("content-type", "application/json;charset=utf-8")
        .spo_json()
        .data()
        .await?;
    let id = dig(&file, &["ListItemAllFields", "Id"])?.cloned();
    let r = Request::post(format!("{web}/_api/sitepages/Pages/UpdateAppPage"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(obj! { "pageId": ?id, "webPartDataAsJson": ?cx.val("webPartData").cloned(), "title": ?cx.val("title").cloned(), "includeInNavigation": ?cx.val("addToQuickLaunch").cloned() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn apppage_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let srv = format!(
        "{}/{}",
        urlutil::get_server_relative_path(&web, "SitePages"),
        cx.u("name")
    );
    Request::post(format!("{web}/_api/sitepages/Pages/UpdateFullPageApp"))
        .header("content-type", "application/json;odata=nometadata")
        .spo_json()
        .json(
            obj! { "serverRelativeUrl": srv, "webPartDataAsJson": ?cx.val("webPartData").cloned() },
        )
        .send()
        .await?;
    Ok(())
}

// ---- brandcenter ---------------------------------------------------------------

pub async fn brandcenter_colors_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving brand center colors...");
    let spo_url = spo::get_spo_url(cx.verbose()).await?;
    let c = Request::get(format!("{spo_url}/_api/Brandcenter/Configuration"))
        .spo_json()
        .data()
        .await?;
    let Some(list_url) = c.get("BrandColorsListUrl").filter(|v| truthy(v)) else {
        cx.verbose_log("Brand colors list not found.");
        return cx.log(Value::Array(Vec::new()));
    };
    cx.verbose_log(&format!(
        "Brand colors list found at '{}'.",
        list_url
            .get("DecodedUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    ));
    let f = |k: &str| {
        c.get(k)
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    };
    let items = odata::get_all_items(&format!("{}/_api/web/lists(guid'{}')/items?$select=OData__SPColorTitle,OData__SPColorCode,OData__SPColorVisible", f("SiteUrl"), f("BrandColorsListId"))).await?;
    let out: Vec<Value> = items
        .iter()
        .map(|i| obj! { "Title": ?i.get("OData__SPColorTitle").cloned(), "ColorCode": ?i.get("OData__SPColorCode").cloned(), "IsVisible": ?i.get("OData__SPColorVisible").cloned() })
        .collect();
    cx.log(Value::Array(out))
}

pub async fn brandcenter_settings_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving brand center configuration...");
    let spo_url = spo::get_spo_url(cx.verbose()).await?;
    cx.log(
        Request::get(format!("{spo_url}/_api/Brandcenter/Configuration"))
            .spo_json()
            .data()
            .await?,
    )
}

//! `spo app *`.

use m365_core::prelude::*;
use m365_core::ut::spo;
use m365_core::ut::url as urlutil;

fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

fn opt(cx: &Cx, k: &str) -> String {
    cx.val(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn scope(cx: &Cx) -> String {
    ne(cx, "appCatalogScope")
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "tenant".into())
}

fn trim_slash(s: &str) -> String {
    s.strip_suffix('/').unwrap_or(s).to_string()
}

/// `SpoAppBaseCommand.getAppCatalogSiteUrl`.
async fn catalog_url(cx: &Cx, spo_url: &str) -> Result<String> {
    if cx.str("appCatalogScope").as_deref() == Some("sitecollection") {
        let url = trim_slash(&opt(cx, "appCatalogUrl"));
        if url.to_lowercase().ends_with("/appcatalog") && url.split('/').count() != 5 {
            return Ok(url[..url.rfind('/').unwrap()].to_string());
        }
    }
    if let Some(u) = ne(cx, "appCatalogUrl") {
        return Ok(trim_slash(&u));
    }
    cx.verbose_log("Getting tenant app catalog url...");
    let r = Request::get(format!("{spo_url}/_api/SP_TenantSettings_Current"))
        .spo_json()
        .data()
        .await?;
    match r.get("CorporateCatalogUrl").filter(|v| truthy(v)) {
        Some(c) => Ok(to_js_string(c)),
        None => Err(Error::js("Tenant app catalog is not configured.")),
    }
}

async fn catalog(cx: &Cx) -> Result<String> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    catalog_url(cx, &spo_url).await
}

pub fn add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let full = util::resolve(&to_js_string(v));
    let p = std::path::Path::new(&full);
    match i {
        0 if !p.exists() => Err(Some(format!("File '{full}' not found"))),
        1 if p.is_dir() => Err(Some(format!("Path '{full}' points to a directory"))),
        _ => Ok(()),
    }
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let sc = scope(cx);
    let overwrite = cx.bool("overwrite");
    let url = catalog(cx).await?;
    let full = util::resolve(&cx.u("filePath"));
    cx.verbose_log(&format!("Adding app '{full}' to app catalog..."));
    let name = std::path::Path::new(&full)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_default();
    let body = std::fs::read(&full)
        .map_err(|_| Error::js(format!("ENOENT: no such file or directory, open '{full}'")))?;
    let res = Request::post(format!(
        "{url}/_api/web/{sc}appcatalog/Add(overwrite={overwrite}, url='{name}')"
    ))
    .accept("application/json;odata=nometadata")
    .header("binaryStringRequestBody", "true")
    .bytes(body)
    .send()
    .await?
    .text();
    let json = json::jparse(&res)?;
    if cx.str("output").as_deref() != Some("text") {
        cx.log(json)
    } else {
        match json.get("UniqueId") {
            Some(v) => cx.log(v.clone()),
            None => cx.log_undefined(),
        }
    }
}

async fn app_id_by_name(cx: &Cx, url: &str) -> Result<Option<Value>> {
    cx.verbose_log(&format!(
        "Looking up app id for app named {}...",
        opt(cx, "name")
    ));
    let r = Request::get(format!("{url}/_api/web/GetFolderByServerRelativePath(DecodedUrl='AppCatalog')/files('{}')?$select=UniqueId", opt(cx, "name"))).spo_json().data().await?;
    Ok(r.get("UniqueId").cloned())
}

pub async fn deploy(cx: &mut Cx) -> Result<()> {
    let sc = scope(cx);
    let url = catalog(cx).await?;
    let id = match ne(cx, "id") {
        Some(i) => {
            cx.verbose_log(&format!("Using the specified app id {i}"));
            i
        }
        None => app_id_by_name(cx, &url)
            .await?
            .map(|v| to_js_string(&v))
            .unwrap_or_else(|| "undefined".into()),
    };
    cx.verbose_log("Deploying app...");
    Request::post(format!(
        "{url}/_api/web/{sc}appcatalog/AvailableApps/GetById('{id}')/deploy"
    ))
    .spo_json()
    .header(
        "content-type",
        "application/json;odata=nometadata;charset=utf-8",
    )
    .json(obj! { "skipFeatureDeployment": cx.bool("skipFeatureDeployment") })
    .send()
    .await?;
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let sc = scope(cx);
    let url = catalog(cx).await?;
    let id = match ne(cx, "id") {
        Some(i) => i,
        None => app_id_by_name(cx, &url)
            .await?
            .map(|v| to_js_string(&v))
            .unwrap_or_else(|| "undefined".into()),
    };
    cx.verbose_log(&format!("Retrieving information for app {id}..."));
    cx.log(
        Request::get(format!(
            "{url}/_api/web/{sc}appcatalog/AvailableApps/GetById('{}')",
            eqp(&id)
        ))
        .spo_json()
        .data()
        .await?,
    )
}

async fn site_action(cx: &Cx, action: &str) -> Result<()> {
    let sc = scope(cx);
    Request::post(format!(
        "{}/_api/web/{sc}appcatalog/AvailableApps/GetById('{}')/{action}",
        cx.u("siteUrl"),
        eqp(&cx.u("id"))
    ))
    .accept("application/json;odata=nometadata")
    .send()
    .await?;
    Ok(())
}

pub async fn install(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Installing app '{}' in site '{}'...",
        opt(cx, "id"),
        opt(cx, "siteUrl")
    ));
    site_action(cx, "install").await
}

pub async fn instance_list(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    cx.verbose_log(&format!("Retrieving installed apps in site at {site}..."));
    cx.log(Value::Array(
        odata::get_all_items(&format!("{site}/_api/web/AppTiles?$filter=AppType eq 3")).await?,
    ))
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let sc = scope(cx);
    let url = catalog(cx).await?;
    cx.verbose_log("Retrieving apps...");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{url}/_api/web/{sc}appcatalog/AvailableApps")).await?,
    ))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let id = opt(cx, "id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the app {id} from the app catalog?"),
    )? {
        return Ok(());
    }
    let sc = scope(cx);
    let url = catalog(cx).await?;
    cx.debug_log(&format!(
        "Retrieved app catalog URL {url}. Removing app from the app catalog..."
    ));
    Request::post(format!(
        "{url}/_api/web/{sc}appcatalog/AvailableApps/GetById('{}')/remove",
        eqp(&id)
    ))
    .accept("application/json;odata=nometadata")
    .send()
    .await?;
    Ok(())
}

pub async fn retract(cx: &mut Cx) -> Result<()> {
    let id = opt(cx, "id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to retract the app {id} from the app catalog?"),
    )? {
        return Ok(());
    }
    let sc = scope(cx);
    let url = catalog(cx).await?;
    cx.verbose_log("Retracting app...");
    Request::post(format!(
        "{url}/_api/web/{sc}appcatalog/AvailableApps/GetById('{}')/retract",
        eqp(&id)
    ))
    .accept("application/json;odata=nometadata")
    .send()
    .await?;
    Ok(())
}

fn package_name(file: &str) -> String {
    let base = std::path::Path::new(file)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_default();
    let stem = match base.rfind('.') {
        Some(i) if i > 0 => &base[..i],
        _ => &base,
    };
    format!("{stem}.zip")
}

pub async fn teamspackage_download(cx: &mut Cx) -> Result<()> {
    let mut catalog_url_v = ne(cx, "appCatalogUrl");
    let mut id: Option<Value> = cx.val("appItemId").cloned();
    let mut file = cx.s("fileName");
    let info = |id: &Option<Value>, file: &Option<String>| {
        json::stringify(&obj! { "id": ?id.clone(), "packageFileName": ?file.clone() })
    };
    cx.debug_log(&format!("appInfo: {}", info(&id, &file)));
    if !(id.as_ref().is_some_and(truthy) && file.as_deref().is_some_and(|f| !f.is_empty())) {
        if let Some(n) = ne(cx, "appName")
            && file.as_deref().is_none_or(str::is_empty)
        {
            file = Some(package_name(&n));
        }
        if catalog_url_v.is_none() {
            catalog_url_v = Some(catalog(cx).await?);
        }
        let cat = catalog_url_v.clone().unwrap();
        let list = format!("{}/AppCatalog", urlutil::get_server_relative_site_url(&cat));
        let mut url = format!("{cat}/_api/web/");
        if let Some(u) = ne(cx, "appItemUniqueId") {
            url.push_str(&format!(
                "GetList('{list}')/GetItemByUniqueId('{u}')?$expand=File&$select=Id,File/Name"
            ));
        } else if cx.val("appItemId").is_some_and(truthy) {
            url.push_str(&format!(
                "GetList('{list}')/GetItemById({})?$expand=File&$select=File/Name",
                opt(cx, "appItemId")
            ));
        } else if let Some(n) = ne(cx, "appName") {
            url.push_str(&format!("GetFolderByServerRelativePath(DecodedUrl='AppCatalog')/files('{}')/ListItemAllFields?$select=Id", eqp(&n)));
        }
        let r = Request::get(url).spo_json().data().await?;
        let file_name = || -> Result<String> {
            match r.get("File") {
                Some(f) => Ok(f
                    .get("Name")
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into())),
                None => Err(Error::js(
                    "Cannot read properties of undefined (reading 'Name')",
                )),
            }
        };
        let parse_id = || {
            json::parse_int(&u(&r, "Id"))
                .map(Value::from)
                .unwrap_or(Value::Null)
        };
        if ne(cx, "appItemUniqueId").is_some() {
            id = Some(parse_id());
            if file.as_deref().is_none_or(str::is_empty) {
                file = Some(package_name(&file_name()?));
            }
        } else if cx.val("appItemId").is_some_and(truthy) {
            if file.as_deref().is_none_or(str::is_empty) {
                file = Some(package_name(&file_name()?));
            }
        } else {
            id = Some(parse_id());
        }
    }
    cx.debug_log(&format!("ensureAppInfo: {}", info(&id, &file)));
    let cat = match catalog_url_v {
        Some(c) => c,
        None => catalog(cx).await?,
    };
    let idv = match &id {
        Some(Value::Null) => "NaN".to_string(),
        Some(v) => to_js_string(v),
        None => "undefined".into(),
    };
    let res = Request::get(format!(
        "{cat}/_api/web/tenantappcatalog/downloadteamssolution({idv})/$value"
    ))
    .accept("application/json;odata=nometadata")
    .bytes_response()
    .send()
    .await?;
    let path = file.unwrap_or_else(|| "undefined".into());
    std::fs::write(&path, &res.bytes).map_err(|e| Error::js(e.to_string()))?;
    cx.verbose_log(&format!("Package saved to {path}"));
    Ok(())
}

pub async fn uninstall(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to uninstall the app {} from site {}?",
            opt(cx, "id"),
            opt(cx, "siteUrl")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Uninstalling app '{}' from the site '{}'...",
        opt(cx, "id"),
        opt(cx, "siteUrl")
    ));
    site_action(cx, "uninstall").await
}

pub async fn upgrade(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Upgrading app '{}' in site '{}'...",
        opt(cx, "id"),
        opt(cx, "siteUrl")
    ));
    site_action(cx, "upgrade").await
}

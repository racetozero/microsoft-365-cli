//! `spo tenant *`.

use crate::sp;
use crate::sp::{first, ne, opt};
use m365_core::prelude::*;
use m365_core::runner::run_nested;
use m365_core::ut::spo;
use m365_core::ut::url as urlutil;

const TENANT: &str = "{268004ae-ef6b-4e9b-8425-127220d84719}";

fn o(v: Value) -> Obj {
    v.as_object().cloned().unwrap_or_default()
}

pub async fn appcatalogurl_get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let text = Request::get(format!("{spo_url}/_api/SP_TenantSettings_Current"))
        .accept("application/json;odata=nometadata")
        .send()
        .await?
        .text();
    let json = json::jparse(&text)?;
    match json.get("CorporateCatalogUrl").filter(|v| truthy(v)) {
        Some(u) => cx.log(u.clone()),
        None => {
            cx.verbose_log("Tenant app catalog is not configured.");
            Ok(())
        }
    }
}

async fn ensure_no_site(cx: &Cx, url: &str, force: bool) -> Result<()> {
    cx.verbose_log(&format!("Checking if site {url} exists..."));
    let r = run_nested(
        "spo site get",
        o(obj! { "url": url, "verbose": cx.verbose(), "debug": cx.debug() }),
        true,
    )
    .await;
    let res = match r {
        Ok(_) => {
            cx.verbose_log(&format!("Found site {url}"));
            if !force {
                Err(Error::msg(format!("Another site exists at {url}")))
            } else {
                cx.verbose_log(&format!("Deleting site {url}..."));
                run_nested("spo site remove", o(obj! { "url": url, "permanent": true, "wait": true, "force": true, "verbose": cx.verbose(), "debug": cx.debug() }), false).await.map(|_| ())
            }
        }
        Err(e) => Err(e),
    };
    match res {
        Ok(()) => Ok(()),
        Err(e) => {
            let m = e.js_message();
            if m != "File not Found" && m != "404 FILE NOT FOUND" {
                return Err(e);
            }
            cx.verbose_log(&format!("No site found at {url}"));
            Ok(())
        }
    }
}

pub async fn appcatalog_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Checking for existing app catalog URL...");
    let (out, _) = run_nested(
        "spo tenant appcatalogurl get",
        o(obj! { "output": "text" }),
        true,
    )
    .await?;
    let force = cx.bool("force");
    if out.is_empty() {
        cx.verbose_log("No app catalog URL found");
    } else {
        cx.verbose_log(&format!("Found app catalog URL {out}"));
        ensure_no_site(cx, &out, force).await?;
    }
    let url = cx.u("url");
    ensure_no_site(cx, &url, force).await?;
    cx.verbose_log(&format!("Creating app catalog at {url}..."));
    run_nested(
        "spo site add",
        o(obj! {
            "webTemplate": "APPCATALOG#0", "title": "App catalog", "type": "ClassicSite", "url": url.as_str(),
            "timeZone": ?cx.val("timeZone").cloned(), "owners": ?cx.val("owner").cloned(), "wait": ?cx.val("wait").cloned(),
            "verbose": cx.verbose(), "debug": cx.debug(), "removeDeletedSite": false,
        }),
        false,
    )
    .await?;
    Ok(())
}

// ---- tenant-wide extensions ----------------------------------------------------

async fn catalog_via_command(cx: &Cx, what: &str) -> Result<String> {
    let (out, err) = run_nested(
        "spo tenant appcatalogurl get",
        o(obj! { "output": "text" }),
        true,
    )
    .await?;
    if cx.debug() {
        cx.stderr(&err);
    }
    if out.is_empty() {
        return Err(Error::msg(format!(
            "Cannot add tenant-wide {what} as app catalog cannot be found"
        )));
    }
    cx.verbose_log(&format!("Got tenant app catalog url: {out}"));
    Ok(out)
}

async fn list_items_caml(cx: &Cx, catalog: &str, list: &str, caml: String) -> Result<Vec<Value>> {
    let (out, err) = run_nested(
        "spo listitem list",
        o(obj! { "webUrl": catalog, "listUrl": format!("{}/{list}", urlutil::get_server_relative_site_url(catalog)), "camlQuery": caml, "verbose": cx.verbose(), "debug": cx.debug(), "output": "json" }),
        true,
    )
    .await?;
    if cx.verbose() {
        cx.stderr(&err);
    }
    let v = json::jparse(&out)?;
    Ok(v.as_array().cloned().unwrap_or_default())
}

async fn check_component(cx: &Cx, catalog: &str, component: &str, ext_type: &str) -> Result<()> {
    cx.verbose_log("Retrieving component manifest item from the ComponentManifests list on the app catalog site so that we get the solution id");
    let items = list_items_caml(
        cx,
        catalog,
        "Lists/ComponentManifests",
        format!("<View><ViewFields><FieldRef Name='ClientComponentId'></FieldRef><FieldRef Name='SolutionId'></FieldRef><FieldRef Name='ClientComponentManifest'></FieldRef></ViewFields><Query><Where><Eq><FieldRef Name='ClientComponentId' /><Value Type='Guid'>{component}</Value></Eq></Where></Query></View>"),
    )
    .await?;
    let Some(manifest) = items.first() else {
        return Err(Error::msg(
            "No component found with the specified clientSideComponentId found in the component manifest list. Make sure that the application is added to the application catalog",
        ));
    };
    let ccm = json::jparse(
        &manifest
            .get("ClientComponentManifest")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    )?;
    if ccm.get("extensionType").and_then(Value::as_str) != Some(ext_type) {
        return Err(Error::msg(format!(
            "The extension type of this component is not of type '{ext_type}' but of type '{}'",
            ccm.get("extensionType")
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into())
        )));
    }
    let sol = manifest
        .get("SolutionId")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Retrieving solution with id {sol} from the application catalog"
    ));
    let items = list_items_caml(
        cx,
        catalog,
        "AppCatalog",
        format!("<View><ViewFields><FieldRef Name='SkipFeatureDeployment'></FieldRef><FieldRef Name='ContainsTenantWideExtension'></FieldRef></ViewFields><Query><Where><Eq><FieldRef Name='AppProductID' /><Value Type='Guid'>{sol}</Value></Eq></Where></Query></View>"),
    )
    .await?;
    let Some(s) = items.first() else {
        return Err(Error::msg(format!(
            "No component found with the solution id {sol}. Make sure that the solution is available in the app catalog"
        )));
    };
    if !s.get("ContainsTenantWideExtension").is_some_and(truthy) {
        return Err(Error::msg(
            "The solution does not contain an extension that can be deployed to all sites. Make sure that you've entered the correct component Id.",
        ));
    }
    if !s.get("SkipFeatureDeployment").is_some_and(truthy) {
        return Err(Error::msg(
            "The solution has not been deployed to all sites. Make sure to deploy this solution to all sites.",
        ));
    }
    Ok(())
}

fn passthrough_output(cx: &Cx) -> Option<Value> {
    cx.str("output")
        .filter(|o| ["text", "csv", "json", "md", "none"].contains(&o.as_str()))
        .map(Value::from)
}

async fn add_extension(cx: &Cx, catalog: &str, fields: Obj) -> Result<()> {
    cx.verbose_log(
        "Pre-checks finished. Adding tenant wide extension to the TenantWideExtensions list",
    );
    let mut opts = o(
        obj! { "webUrl": catalog, "listUrl": format!("{}/Lists/TenantWideExtensions", urlutil::get_server_relative_site_url(catalog)) },
    );
    opts.extend(fields);
    opts.insert("verbose".into(), Value::Bool(cx.verbose()));
    opts.insert("debug".into(), Value::Bool(cx.debug()));
    if let Some(out) = passthrough_output(cx) {
        opts.insert("output".into(), out);
    }
    run_nested("spo listitem add", opts, false).await?;
    Ok(())
}

pub async fn applicationcustomizer_add(cx: &mut Cx) -> Result<()> {
    let catalog = catalog_via_command(cx, "application customizer").await?;
    let comp = cx.u("clientSideComponentId");
    check_component(cx, &catalog, &comp, "ApplicationCustomizer").await?;
    add_extension(
        cx,
        &catalog,
        o(obj! {
            "Title": ?cx.val("title").cloned(), "TenantWideExtensionComponentId": comp.as_str(), "TenantWideExtensionLocation": "ClientSideExtension.ApplicationCustomizer",
            "TenantWideExtensionSequence": 0, "TenantWideExtensionListTemplate": 0,
            "TenantWideExtensionComponentProperties": ne(cx, "clientSideComponentProperties").unwrap_or_default(),
            "TenantWideExtensionHostProperties": ne(cx, "hostProperties").unwrap_or_default(),
            "TenantWideExtensionWebTemplate": ne(cx, "webTemplate").unwrap_or_default(),
            "TenantWideExtensionDisabled": false,
        }),
    )
    .await
}

fn cmd_location(l: Option<&str>) -> &'static str {
    match l {
        Some("Both") => "ClientSideExtension.ListViewCommandSet",
        Some("ContextMenu") => "ClientSideExtension.ListViewCommandSet.ContextMenu",
        _ => "ClientSideExtension.ListViewCommandSet.CommandBar",
    }
}

fn list_template(l: Option<&str>) -> &'static str {
    match l {
        Some("SitePages") => "119",
        Some("Library") => "101",
        _ => "100",
    }
}

pub async fn commandset_add(cx: &mut Cx) -> Result<()> {
    let catalog = catalog_via_command(cx, "ListView Command Set").await?;
    let comp = cx.u("clientSideComponentId");
    check_component(cx, &catalog, &comp, "ListViewCommandSet").await?;
    add_extension(
        cx,
        &catalog,
        o(obj! {
            "Title": ?cx.val("title").cloned(), "TenantWideExtensionComponentId": comp.as_str(),
            "TenantWideExtensionLocation": cmd_location(cx.str("location").as_deref()),
            "TenantWideExtensionSequence": 0, "TenantWideExtensionListTemplate": list_template(cx.str("listType").as_deref()),
            "TenantWideExtensionComponentProperties": ne(cx, "clientSideComponentProperties").unwrap_or_default(),
            "TenantWideExtensionWebTemplate": ne(cx, "webTemplate").unwrap_or_default(),
            "TenantWideExtensionDisabled": false,
        }),
    )
    .await
}

async fn tenant_catalog(cx: &Cx) -> Result<Option<String>> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let r = Request::get(format!("{spo_url}/_api/SP_TenantSettings_Current"))
        .spo_json()
        .data()
        .await?;
    Ok(r.get("CorporateCatalogUrl")
        .filter(|v| truthy(v))
        .map(to_js_string))
}

fn ext_list(catalog: &str) -> String {
    format!(
        "{catalog}/_api/web/GetList('{}')",
        eqp(&urlutil::get_server_relative_path(
            catalog,
            "/lists/TenantWideExtensions"
        ))
    )
}

fn try_parse(v: Option<&Value>) -> Option<Value> {
    match v {
        Some(Value::String(s)) => {
            Some(json::js_parse(s).unwrap_or_else(|_| Value::from(s.as_str())))
        }
        other => other.cloned(),
    }
}

fn strip_id(items: &mut [Value]) {
    for i in items {
        if let Some(o) = i.as_object_mut() {
            o.shift_remove("ID");
        }
    }
}

async fn log_one(cx: &Cx, mut items: Vec<Value>, label: &str, props_flag: &str) -> Result<()> {
    strip_id(&mut items);
    let item = if items.len() > 1 {
        cx.handle_multiple_results(
            &format!(
                "Multiple {label} with {} were found.",
                first(cx, &["title", "clientSideComponentId"])
            ),
            &util::array_to_hash_table("Id", &items),
        )?
    } else {
        items.remove(0)
    };
    if !cx.bool(props_flag) {
        cx.log(item)
    } else {
        match try_parse(item.get("TenantWideExtensionComponentProperties")) {
            Some(v) => cx.log(v),
            None => cx.log_undefined(),
        }
    }
}

pub async fn applicationcustomizer_get(cx: &mut Cx) -> Result<()> {
    let Some(catalog) = tenant_catalog(cx).await? else {
        return Err(Error::msg("No app catalog URL found"));
    };
    let filter = if let Some(t) = ne(cx, "title") {
        format!("Title eq '{t}'")
    } else if let Some(i) = ne(cx, "id") {
        format!("Id eq '{i}'")
    } else {
        format!(
            "TenantWideExtensionComponentId eq '{}'",
            opt(cx, "clientSideComponentId")
        )
    };
    let items = odata::get_all_items(&format!("{}/items?$filter=TenantWideExtensionLocation eq 'ClientSideExtension.ApplicationCustomizer' and {filter}", ext_list(&catalog))).await?;
    if items.is_empty() {
        return Err(Error::msg(
            "The specified application customizer was not found",
        ));
    }
    log_one(
        cx,
        items,
        "application customizers",
        "tenantWideExtensionComponentProperties",
    )
    .await
}

pub async fn applicationcustomizer_list(cx: &mut Cx) -> Result<()> {
    let catalog = tenant_catalog(cx)
        .await
        .map_err(Error::uncaught)?
        .ok_or_else(|| Error::msg("No app catalog URL found"))?;
    let mut items = odata::get_all_items(&format!("{}/items?$filter=TenantWideExtensionLocation eq 'ClientSideExtension.ApplicationCustomizer'", ext_list(&catalog))).await?;
    strip_id(&mut items);
    cx.log(Value::Array(items))
}

async fn ext_id(
    cx: &Cx,
    base: &str,
    loc_filter: &str,
    label: &str,
    id_quoted: bool,
) -> Result<String> {
    let filter = if let Some(t) = ne(cx, "title") {
        format!("Title eq '{t}'")
    } else if let Some(i) = ne(cx, "id") {
        if id_quoted {
            format!("Id eq '{i}'")
        } else {
            format!("Id eq {i}")
        }
    } else {
        format!(
            "TenantWideExtensionComponentId eq '{}'",
            opt(cx, "clientSideComponentId")
        )
    };
    let mut items = odata::get_all_items(&format!(
        "{base}/items?$filter={loc_filter} and {filter}{}",
        if id_quoted { "&$select=Id" } else { "" }
    ))
    .await?;
    if items.is_empty() {
        return Err(Error::msg(format!("The specified {label} was not found")));
    }
    let item = if items.len() > 1 {
        let plural = if label == "application customizer" {
            "application customizers"
        } else {
            "command sets"
        };
        cx.handle_multiple_results(
            &format!(
                "Multiple {plural} with {} were found.",
                first(cx, &["title", "clientSideComponentId"])
            ),
            &util::array_to_hash_table("Id", &items),
        )?
    } else {
        items.remove(0)
    };
    Ok(u(&item, "Id"))
}

pub async fn applicationcustomizer_remove(cx: &mut Cx) -> Result<()> {
    let who = first(cx, &["id", "title", "clientSideComponentId"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the tenant applicationcustomizer {who}?"),
    )? {
        return Ok(());
    }
    let Some(catalog) = tenant_catalog(cx).await? else {
        return Err(Error::msg("No app catalog URL found"));
    };
    let base = ext_list(&catalog);
    cx.verbose_log(&format!("Getting the tenant application customizer {who}"));
    let id = ext_id(
        cx,
        &base,
        "TenantWideExtensionLocation eq 'ClientSideExtension.ApplicationCustomizer'",
        "application customizer",
        true,
    )
    .await?;
    cx.verbose_log(&format!("Removing tenant application customizer {who}"));
    Request::post(format!("{base}/items({id})"))
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json()
        .send()
        .await?;
    Ok(())
}

pub async fn applicationcustomizer_set(cx: &mut Cx) -> Result<()> {
    let Some(catalog) = tenant_catalog(cx).await? else {
        return Err(Error::msg("No app catalog URL found"));
    };
    if let Some(c) = ne(cx, "newClientSideComponentId") {
        check_component(cx, &catalog, &c, "ApplicationCustomizer").await?;
    }
    let base = ext_list(&catalog);
    let who = first(cx, &["title", "id", "clientSideComponentId"]);
    cx.verbose_log(&format!(
        "Getting tenant-wide application customizer: \"{who}\"..."
    ));
    let filter = if let Some(t) = ne(cx, "title") {
        format!("Title eq '{t}'")
    } else if let Some(i) = ne(cx, "id") {
        format!("Id eq '{i}'")
    } else {
        format!(
            "TenantWideExtensionComponentId eq '{}'",
            opt(cx, "clientSideComponentId")
        )
    };
    let mut items = odata::get_all_items(&format!("{base}/items?$filter=TenantWideExtensionLocation eq 'ClientSideExtension.ApplicationCustomizer' and {filter}")).await?;
    if items.is_empty() {
        return Err(Error::msg(
            "The specified application customizer was not found",
        ));
    }
    let id = if items.len() > 1 {
        let by = match ne(cx, "title") {
            Some(t) => format!("title '{t}'"),
            None => format!(
                "ClientSideComponentId '{}'",
                opt(cx, "clientSideComponentId")
            ),
        };
        u(
            &cx.handle_multiple_results(
                &format!("Multiple application customizers with {by} found."),
                &util::array_to_hash_table("Id", &items),
            )?,
            "Id",
        )
    } else {
        u(&items.remove(0), "Id")
    };
    cx.verbose_log(&format!(
        "Updating tenant-wide application customizer: \"{who}\"..."
    ));
    let mut fv = Vec::new();
    for (k, f) in [
        ("newTitle", "Title"),
        ("newClientSideComponentId", "TenantWideExtensionComponentId"),
        (
            "clientSideComponentProperties",
            "TenantWideExtensionComponentProperties",
        ),
        ("hostProperties", "TenantWideExtensionHostProperties"),
        ("webTemplate", "TenantWideExtensionWebTemplate"),
    ] {
        if let Some(v) = cx.val(k) {
            fv.push(obj! { "FieldName": f, "FieldValue": v.clone() });
        }
    }
    Request::post(format!("{base}/Items({id})/ValidateUpdateListItem()"))
        .spo_json()
        .json(obj! { "formValues": fv })
        .send()
        .await?;
    Ok(())
}

pub async fn commandset_get(cx: &mut Cx) -> Result<()> {
    let catalog = tenant_catalog(cx)
        .await
        .map_err(Error::uncaught)?
        .ok_or_else(|| Error::msg("No app catalog URL found"))?;
    let mut filter =
        "startswith(TenantWideExtensionLocation,'ClientSideExtension.ListViewCommandSet')"
            .to_string();
    if let Some(t) = ne(cx, "title") {
        filter.push_str(&format!(" and Title eq '{t}'"));
    } else if let Some(i) = ne(cx, "id") {
        filter.push_str(&format!(" and Id eq {i}"));
    } else if let Some(c) = ne(cx, "clientSideComponentId") {
        filter.push_str(&format!(" and TenantWideExtensionComponentId eq '{c}'"));
    }
    let r = Request::get(format!("{}/items?$filter={filter}", ext_list(&catalog)))
        .spo_json()
        .data()
        .await?;
    let items = r
        .get("value")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if r.get("value").is_none() && !r.is_null() {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'length')",
        ));
    }
    if items.is_empty() {
        return Err(Error::msg(
            "The specified ListView Command Set was not found",
        ));
    }
    log_one(
        cx,
        items,
        "ListView Command Sets",
        "tenantWideExtensionComponentProperties",
    )
    .await
}

pub async fn commandset_list(cx: &mut Cx) -> Result<()> {
    let catalog = tenant_catalog(cx)
        .await
        .map_err(Error::uncaught)?
        .ok_or_else(|| Error::msg("No app catalog URL found"))?;
    cx.verbose_log("Retrieving a list of ListView Command Sets that are installed tenant-wide");
    let mut items = odata::get_all_items(&format!("{}/items?$filter=startswith(TenantWideExtensionLocation, 'ClientSideExtension.ListViewCommandSet')", ext_list(&catalog))).await?;
    strip_id(&mut items);
    cx.log(Value::Array(items))
}

pub async fn commandset_remove(cx: &mut Cx) -> Result<()> {
    let who = first(cx, &["id", "title", "clientSideComponentId"]);
    let r = async {
        // a failed prompt rejects with inquirer's ExitPromptError
        match cx.proceed(
            "force",
            &format!("Are you sure you want to remove the tenant commandset {who}?"),
        ) {
            Ok(true) => {}
            Ok(false) => return Ok(()),
            Err(e) => {
                return Err(Error::Thrown(obj! { "name": "ExitPromptError" }))
                    .inspect_err(|_| drop(e));
            }
        }
        let Some(catalog) = tenant_catalog(cx).await? else {
            return Err(Error::msg("No app catalog URL found"));
        };
        let base = ext_list(&catalog);
        cx.verbose_log(&format!("Getting the tenant command set {who}"));
        let id = ext_id(
            cx,
            &base,
            "startswith(TenantWideExtensionLocation,'ClientSideExtension.ListViewCommandSet')",
            "command set",
            false,
        )
        .await?;
        cx.verbose_log(&format!("Removing tenant command set {who}"));
        Request::post(format!("{base}/items({id})"))
            .header("X-HTTP-Method", "DELETE")
            .header("If-Match", "*")
            .spo_json()
            .send()
            .await?;
        Ok(())
    }
    .await;
    if let Err(e) = &r {
        // upstream logs the raw error before handling it
        cx.log(match e {
            Error::Thrown(v) => v.clone(),
            other => other.command_message(),
        })?;
    }
    r
}

pub async fn commandset_set(cx: &mut Cx) -> Result<()> {
    let Some(catalog) = tenant_catalog(cx).await? else {
        return Err(Error::msg("No app catalog URL found"));
    };
    let base = ext_list(&catalog);
    let id = opt(cx, "id");
    cx.verbose_log(&format!("Getting the list item by id {id}"));
    let item = Request::get(format!("{base}/Items({id})"))
        .spo_json()
        .data()
        .await?;
    match item.get("TenantWideExtensionLocation") {
        Some(Value::String(s)) if s.contains("ClientSideExtension.ListViewCommandSet") => {}
        Some(Value::String(_)) => return Err(Error::msg("The item is not a ListViewCommandSet")),
        _ => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'indexOf')",
            ));
        }
    }
    cx.verbose_log("Updating tenant wide extension to the TenantWideExtensions list");
    let mut fv = Vec::new();
    if let Some(v) = cx.val("newTitle") {
        fv.push(obj! { "FieldName": "Title", "FieldValue": v.clone() });
    }
    if let Some(v) = cx.val("clientSideComponentId") {
        fv.push(obj! { "FieldName": "TenantWideExtensionComponentId", "FieldValue": v.clone() });
    }
    if cx.has("location") {
        fv.push(obj! { "FieldName": "TenantWideExtensionLocation", "FieldValue": cmd_location(cx.str("location").as_deref()) });
    }
    if cx.has("listType") {
        fv.push(obj! { "FieldName": "TenantWideExtensionListTemplate", "FieldValue": list_template(cx.str("listType").as_deref()) });
    }
    for (k, f) in [
        (
            "clientSideComponentProperties",
            "TenantWideExtensionComponentProperties",
        ),
        ("webTemplate", "TenantWideExtensionWebTemplate"),
    ] {
        if let Some(v) = cx.val(k) {
            fv.push(obj! { "FieldName": f, "FieldValue": v.clone() });
        }
    }
    Request::post(format!("{base}/Items({id})/ValidateUpdateListItem()"))
        .spo_json()
        .json(obj! { "formValues": fv })
        .send()
        .await?;
    Ok(())
}

// ---- recycle bin ---------------------------------------------------------------

fn csom_date(s: &str) -> Value {
    let parts: Vec<f64> = s
        .replacen("/Date(", "", 1)
        .replacen(")/", "", 1)
        .split(',')
        .map(json::js_number)
        .collect();
    let g = |i: usize| parts.get(i).copied().unwrap_or(f64::NAN);
    use chrono::TimeZone;
    let (y, mo, d, h, mi, se, ms) = (g(0), g(1), g(2), g(3), g(4), g(5), g(6));
    if [y, mo, d, h, mi, se, ms].iter().any(|x| !x.is_finite()) {
        return Value::Null;
    }
    let base = chrono::NaiveDate::from_ymd_opt(y as i32, 1, 1).and_then(|n| n.and_hms_opt(0, 0, 0));
    let Some(base) = base else { return Value::Null };
    let dt = base + chrono::Duration::days(0);
    let dt = if mo >= 0.0 {
        dt.checked_add_months(chrono::Months::new(mo as u32))
            .unwrap_or(dt)
    } else {
        dt.checked_sub_months(chrono::Months::new((-mo) as u32))
            .unwrap_or(dt)
    };
    let dt = dt
        + chrono::Duration::days(d as i64 - 1)
        + chrono::Duration::hours(h as i64)
        + chrono::Duration::minutes(mi as i64)
        + chrono::Duration::seconds(se as i64)
        + chrono::Duration::milliseconds(ms as i64);
    match chrono::Local.from_local_datetime(&dt).earliest() {
        Some(l) => Value::from(m365_core::time::to_iso(l.with_timezone(&chrono::Utc))),
        None => Value::Null,
    }
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let body = sp::request(
        r#"<ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties><Property Name="Url" ScalarProperty="true" /><Property Name="SiteId" ScalarProperty="true" /><Property Name="DaysRemaining" ScalarProperty="true" /><Property Name="Status" ScalarProperty="true" /></Properties></ChildItemQuery></Query>"#,
        &format!(
            r#"<Constructor Id="1" TypeId="{TENANT}" /><Method Id="3" ParentId="1" Name="GetDeletedSitePropertiesFromSharePoint"><Parameters><Parameter Type="String">0</Parameter></Parameters></Method>"#
        ),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let last = json.last().cloned().unwrap_or(Value::Null);
    let mut items = match dig(&last, &["_Child_Items_"])? {
        Some(Value::Array(a)) => a.clone(),
        _ if cx.str("output").as_deref() != Some("json") => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'forEach')",
            ));
        }
        _ => return cx.log_undefined(),
    };
    if cx.str("output").as_deref() != Some("json") {
        for s in items.iter_mut() {
            let days = s
                .get("DaysRemaining")
                .map(|v| json::num_value(json::js_number(&to_js_string(v))))
                .unwrap_or(Value::Null);
            let del = match s.get("DeletionTime") {
                Some(Value::String(t)) => csom_date(t),
                _ => {
                    return Err(Error::js(
                        "Cannot read properties of undefined (reading 'replace')",
                    ));
                }
            };
            if let Some(o) = s.as_object_mut() {
                o.insert("DaysRemaining".into(), days);
                o.insert("DeletionTime".into(), del);
            }
        }
    }
    cx.log(Value::Array(items))
}

pub async fn recyclebinitem_remove(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    if !cx.proceed("force", &format!("Are you sure you want to remove the deleted site collection {site} from tenant recycle bin?"))? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let ctx = spo::ensure_form_digest(&admin, None, cx.debug()).await?;
    cx.verbose_log(&format!("Removing deleted site collection {site}..."));
    let body = sp::request(
        r#"<ObjectPath Id="16" ObjectPathId="15" /><Query Id="17" ObjectPathId="15"><Query SelectAllProperties="false"><Properties><Property Name="PollingInterval" ScalarProperty="true" /><Property Name="IsComplete" ScalarProperty="true" /></Properties></Query></Query>"#,
        &format!(
            r#"<Method Id="15" ParentId="1" Name="RemoveDeletedSite"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="1" TypeId="{TENANT}" />"#,
            util::escape_xml(&site)
        ),
    );
    let json = sp::post(&admin, &ctx.value, body).await?;
    sp::check_first(&json)?;
    let op = json.last().cloned().unwrap_or(Value::Null);
    if !cx.bool("wait") || op.get("IsComplete").is_some_and(truthy) {
        return Ok(());
    }
    // `this.context` is never set, so the wait starts without a digest
    crate::site::wait_op(cx, &admin, &op, None).await
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let raw = cx.u("siteUrl");
    cx.verbose_log(&format!(
        "Restoring site collection '{raw}' from recycle bin."
    ));
    let site = urlutil::remove_trailing_slashes(&raw);
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    Request::post(format!("{admin}/_api/SPO.Tenant/RestoreDeletedSite"))
        .spo_json()
        .header("content-type", "application/json;charset=utf-8")
        .json(obj! { "siteUrl": site.as_str() })
        .send()
        .await?;
    let sites = odata::get_all_items(&format!("{admin}/_api/web/lists/GetByTitle('DO_NOT_DELETE_SPLIST_TENANTADMIN_AGGREGATED_SITECOLLECTIONS')/items?$filter=SiteUrl eq '{}'&$select=GroupId", eqp(&site))).await?;
    let first = sites
        .first()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'GroupId')"))?;
    if let Some(g) = first
        .get("GroupId")
        .filter(|g| truthy(g))
        .map(to_js_string)
        .filter(|g| g != "00000000-0000-0000-0000-000000000000")
    {
        cx.verbose_log(&format!(
            "Restoring Microsoft 365 group with ID '{g}' from recycle bin."
        ));
        Request::post(format!(
            "https://graph.microsoft.com/v1.0/directory/deletedItems/{g}/restore"
        ))
        .graph_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    }
    Ok(())
}

// ---- settings ------------------------------------------------------------------

const ENUMS: [(&str, &[&str]); 11] = [
    (
        "SharingCapability",
        &[
            "Disabled",
            "ExternalUserSharingOnly",
            "ExternalUserAndGuestSharing",
            "ExistingExternalUserSharingOnly",
        ],
    ),
    (
        "SharingDomainRestrictionMode",
        &["None", "AllowList", "BlockList"],
    ),
    ("ODBMembersCanShare", &["Unspecified", "On", "Off"]),
    ("ODBAccessRequests", &["Unspecified", "On", "Off"]),
    (
        "DefaultSharingLinkType",
        &["None", "Direct", "Internal", "AnonymousAccess"],
    ),
    ("FileAnonymousLinkType", &["None", "View", "Edit"]),
    ("FolderAnonymousLinkType", &["None", "View", "Edit"]),
    ("DefaultLinkPermission", &["None", "View", "Edit"]),
    (
        "ConditionalAccessPolicy",
        &["AllowFullAccess", "AllowLimitedAccess", "BlockAccess"],
    ),
    (
        "SpecialCharactersStateInFileFolderNames",
        &["NoPreference", "Allowed", "Disallowed"],
    ),
    (
        "LimitedAccessFileType",
        &["OfficeOnlineFilesOnly", "WebPreviewableFiles", "OtherFiles"],
    ),
];

pub async fn settings_list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let body = sp::request(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><Query Id="5" ObjectPathId="3"><Query SelectAllProperties="true"><Properties><Property Name="HideDefaultThemes" ScalarProperty="true" /></Properties></Query></Query>"#,
        &format!(r#"<Constructor Id="3" TypeId="{TENANT}" />"#),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let mut r = match json.get(4) {
        Some(v) if v.is_object() => v.clone(),
        Some(Value::Null) | None => {
            return Err(Error::js("Cannot convert undefined or null to object"));
        }
        Some(v) => v.clone(),
    };
    if let Some(o) = r.as_object_mut() {
        o.shift_remove("_ObjectIdentity_");
        o.shift_remove("_ObjectType_");
        for (k, vals) in ENUMS {
            let mapped = o
                .get(k)
                .map(to_js_string)
                .and_then(|i| i.parse::<usize>().ok())
                .and_then(|i| vals.get(i))
                .map(|s| Value::from(*s));
            match mapped {
                Some(m) => {
                    o.insert(k.into(), m);
                }
                // `arr[x]` is undefined: the key stays but is not serialized
                None => {
                    o.shift_remove(k);
                }
            }
        }
    }
    cx.log(r)
}

pub fn settings_set_refine(i: usize, opts: &Obj) -> std::result::Result<bool, String> {
    if i != 0 {
        return Ok(true);
    }
    Ok(opts
        .keys()
        .any(|k| !["output", "o", "debug", "verbose", "_", "query"].contains(&k.as_str())))
}

pub async fn settings_set(cx: &mut Cx) -> Result<()> {
    let tenant = spo::get_tenant_id(cx.debug()).await?;
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let _digest = spo::get_request_digest(&admin).await?;
    let mut props = String::new();
    let mut id = 42;
    for (k, v) in &cx.opts {
        if ["output", "o", "debug", "verbose", "_", "query"].contains(&k.as_str()) {
            continue;
        }
        let val = match ENUMS.iter().find(|(n, _)| n == k) {
            Some((_, vals)) => (vals
                .iter()
                .position(|x| Some(*x) == v.as_str())
                .map(|i| i as i64)
                .unwrap_or(-1))
            .to_string(),
            None => to_js_string(v),
        };
        if k == "AllowedDomainListForSyncClient" || k == "DisabledWebPartIds" {
            let xml: String = val
                .split(',')
                .map(|x| {
                    format!(
                        r#"<Object Type="Guid">{{{}}}</Object>"#,
                        util::escape_xml_value(Some(&Value::from(x)))
                    )
                })
                .collect();
            props.push_str(&format!(r#"<SetProperty Id="{id}" ObjectPathId="7" Name="{k}"><Parameter Type="Array">{xml}</Parameter></SetProperty><Method Name="Update" Id="{}" ObjectPathId="7" />"#, id + 1));
            id += 2;
        } else if k == "ExcludedFileExtensionsForSyncClient" {
            let xml: String = val
                .split(',')
                .map(|x| format!(r#"<Object Type="String">{x}</Object>"#))
                .collect();
            props.push_str(&format!(r#"<SetProperty Id="{id}" ObjectPathId="7" Name="{k}"><Parameter Type="Array">{xml}</Parameter></SetProperty><Method Name="Update" Id="{}" ObjectPathId="7" />"#, id + 1));
            id += 2;
        } else {
            props.push_str(&format!(r#"<SetProperty Id="{id}" ObjectPathId="7" Name="{k}"><Parameter Type="String">{val}</Parameter></SetProperty>"#));
            id += 1;
        }
    }
    // upstream passes the whole context info object as the digest header
    let text = Request::post(format!("{admin}/_vti_bin/client.svc/ProcessQuery"))
        .header("X-RequestDigest", "[object Object]")
        .text(sp::request(
            &props,
            &format!(r#"<Identity Id="7" Name="{tenant}" />"#),
        ))
        .send()
        .await?
        .text();
    sp::check_first(&sp::parse(&text)?)?;
    if cx.val("EnableAzureADB2BIntegration") == Some(&Value::Bool(true)) {
        cx.warn("WARNING: Make sure to also enable the Microsoft Entra one-time passcode authentication preview. If it is not enabled then SharePoint will not use Microsoft Entra B2B even if EnableAzureADB2BIntegration is set to true. Learn more at http://aka.ms/spo-b2b-integration.");
    }
    Ok(())
}

// ---- site get ------------------------------------------------------------------

pub async fn site_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving tenant site information for site '{}'...",
        first(cx, &["url", "id", "title"])
    ));
    let url = if let Some(u) = ne(cx, "url") {
        u
    } else if let Some(id) = ne(cx, "id") {
        cx.verbose_log(&format!("Retrieving tenant site URL for site '{id}'..."));
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let r = Request::get(format!("{admin}/_api/SPO.Tenant/sites('{id}')?$select=Url"))
            .spo_json()
            .data()
            .await?;
        cx.verbose_log(&format!("Retrieved tenant site URL for site '{id}'..."));
        u(&r, "Url")
    } else {
        let title = opt(cx, "title");
        cx.verbose_log(&format!("Retrieving tenant site URL for site '{title}'..."));
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let view = format!(
            r#"<View><Query><Where><And><IsNull><FieldRef Name="TimeDeleted"/></IsNull><Eq><FieldRef Name="Title"/><Value Type='Text'>{}</Value></Eq></And></Where></Query><ViewFields><FieldRef Name="Title"/><FieldRef Name="SiteUrl"/><FieldRef Name="SiteId"/></ViewFields></View>"#,
            util::escape_xml(&title)
        );
        let r = Request::post(format!("{admin}/_api/web/lists/GetByTitle('DO_NOT_DELETE_SPLIST_TENANTADMIN_AGGREGATED_SITECOLLECTIONS')/RenderListDataAsStream"))
            .spo_json()
            .json(obj! { "parameters": obj! { "ViewXml": view, "DatesInUtc": true } })
            .data()
            .await?;
        let rows = need_arr(&r, "Row", "length")?;
        let url = match rows.len() {
            0 => {
                return Err(Error::msg(format!(
                    "The specified site '{title}' does not exist."
                )));
            }
            1 => u(&rows[0], "SiteUrl"),
            _ => {
                let mut table = Obj::new();
                for row in rows {
                    table.insert(
                        u(row, "SiteUrl"),
                        obj! { "url": ?row.get("SiteUrl").cloned() },
                    );
                }
                u(
                    &cx.handle_multiple_results(
                        &format!("Multiple sites with title '{title}' found."),
                        &table,
                    )?,
                    "url",
                )
            }
        };
        cx.verbose_log(&format!("Retrieved tenant site URL for site '{title}'..."));
        url
    };
    let site = sp::site_admin_properties(cx, &url, false).await?;
    cx.log(site)
}

fn finite_number(v: &Value) -> bool {
    let s = to_js_string(v);
    !s.trim().is_empty() && json::js_number(&s).is_finite()
}

fn json_or_empty(v: &Value) -> bool {
    let s = to_js_string(v);
    s.is_empty() || json::js_parse(&s).is_ok()
}

pub fn number_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(finite_number(v))
}

pub fn json_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(json_or_empty(v))
}

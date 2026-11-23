//! `spo serviceprincipal *` and `spo hubsite *`.

use crate::sp;
use crate::sp::{ex, ne};
use m365_core::prelude::*;
use m365_core::ut::spo;

const SP_TYPE: &str = "{104e8f06-1e00-4675-99c6-1b9b504ed8d8}";
const TENANT: &str = "{268004ae-ef6b-4e9b-8425-127220d84719}";

async fn admin_digest(cx: &Cx) -> Result<(String, String)> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    cx.verbose_log("Retrieving request digest...");
    let d = sp::digest(&admin).await?;
    Ok((admin, d))
}

fn last_without_type(json: &[Value]) -> Result<Value> {
    let mut r = json.last().cloned().unwrap_or(Value::Null);
    if r.is_null() {
        return Err(Error::js("Cannot convert undefined or null to object"));
    }
    if let Some(o) = r.as_object_mut() {
        o.shift_remove("_ObjectType_");
    }
    Ok(r)
}

/// `s.<field>.<method>()` on a JS value with its TypeErrors.
fn js_str<'a>(v: &'a Value, path: &str, method: &str) -> Result<&'a str> {
    let field = path.rsplit('.').next().unwrap_or(path);
    match v.get(field) {
        Some(Value::String(s)) => Ok(s),
        Some(Value::Null) => Err(Error::js(format!(
            "Cannot read properties of null (reading '{method}')"
        ))),
        Some(_) => Err(Error::js(format!("{path}.{method} is not a function"))),
        None => Err(Error::js(format!(
            "Cannot read properties of undefined (reading '{method}')"
        ))),
    }
}

pub async fn grant_add(cx: &mut Cx) -> Result<()> {
    let (admin, d) = admin_digest(cx).await?;
    let body = sp::request(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectPath Id="6" ObjectPathId="5" /><ObjectPath Id="8" ObjectPathId="7" /><Query Id="9" ObjectPathId="7"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        &format!(
            r#"<Constructor Id="3" TypeId="{SP_TYPE}" /><Property Id="5" ParentId="3" Name="PermissionRequests" /><Method Id="7" ParentId="5" Name="Approve"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "resource"),
            ex(cx, "scope")
        ),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    cx.log(last_without_type(&json)?)
}

pub async fn grant_list(cx: &mut Cx) -> Result<()> {
    const NAME: &str = "SharePoint Online Web Client Extensibility";
    cx.verbose_log(&format!(
        "Retrieving permissions granted to the service principal '{NAME}'..."
    ));
    let r = Request::get(format!("https://graph.microsoft.com/v1.0/servicePrincipals?$filter=displayName eq '{NAME}'&$select=id")).graph_json().data().await?;
    let v = need_arr(&r, "value", "length")?;
    let Some(first) = v.first() else {
        return Err(Error::msg(format!("Service principal '{NAME}' not found")));
    };
    let r = Request::get(format!(
        "https://graph.microsoft.com/v1.0/servicePrincipals/{}/oauth2PermissionGrants",
        u(first, "id")
    ))
    .graph_json()
    .data()
    .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn grant_revoke(cx: &mut Cx) -> Result<()> {
    let (admin, d) = admin_digest(cx).await?;
    if let Some(scope) = ne(cx, "scope") {
        let body = sp::request(
            r#"<ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><ObjectPath Id="6" ObjectPathId="5" /><Query Id="7" ObjectPathId="5"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<Constructor Id="1" TypeId="{SP_TYPE}" /><Property Id="3" ParentId="1" Name="PermissionGrants" /><Method Id="5" ParentId="3" Name="GetByObjectId"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                ex(cx, "id")
            ),
        );
        let json = sp::post(&admin, &d, body).await?;
        sp::check_first(&json)?;
        let grant = json.last().cloned().unwrap_or(Value::Null);
        if grant.is_null() {
            return Err(Error::js(
                "Cannot read properties of null (reading 'ClientId')",
            ));
        }
        let body = sp::request(
            &format!(
                r#"<ObjectPath Id="9" ObjectPathId="8" /><Method Name="Remove" Id="10" ObjectPathId="8"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">Microsoft Graph</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                u(&grant, "ClientId"),
                util::escape_xml(&scope)
            ),
            &format!(
                r#"<Property Id="8" ParentId="1" Name="GrantManager" /><Constructor Id="1" TypeId="{SP_TYPE}" />"#
            ),
        );
        sp::check_first(&sp::post(&admin, &d, body).await?)
    } else {
        let body = sp::request(
            r#"<ObjectPath Id="10" ObjectPathId="9" /><ObjectPath Id="12" ObjectPathId="11" /><ObjectPath Id="14" ObjectPathId="13" /><Method Name="DeleteObject" Id="15" ObjectPathId="13" /><Query Id="16" ObjectPathId="13"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<Constructor Id="9" TypeId="{SP_TYPE}" /><Property Id="11" ParentId="9" Name="PermissionGrants" /><Method Id="13" ParentId="11" Name="GetByObjectId"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                ex(cx, "id")
            ),
        );
        sp::check_first(&sp::post(&admin, &d, body).await?)
    }
}

pub fn approve_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    if i != 1 {
        return Ok(true);
    }
    match o.get("id").filter(|v| truthy(v)) {
        Some(id) if !m365_core::validation::is_valid_guid(&to_js_string(id)) => Err(format!(
            "id: The value '{}' is not a valid GUID.",
            to_js_string(id)
        )),
        _ => Ok(true),
    }
}

pub async fn permissionrequest_approve(cx: &mut Cx) -> Result<()> {
    let (admin, _) = (spo::get_spo_admin_url(cx.debug()).await?, ());
    cx.verbose_log("Retrieving request digest...");
    let ids: Vec<String> = match ne(cx, "id") {
        Some(i) => vec![i],
        None => {
            let o = obj! { "debug": cx.debug(), "verbose": cx.verbose() };
            let (out, _) = m365_core::runner::run_nested(
                "spo serviceprincipal permissionrequest list",
                o.as_object().cloned().unwrap_or_default(),
                true,
            )
            .await?;
            let list = json::jparse(&out)?;
            let items = list.as_array().cloned().unwrap_or_default();
            items
                .iter()
                .filter(|x| match ne(cx, "resource") {
                    Some(r) => x.get("Resource").and_then(Value::as_str) == Some(r.as_str()),
                    None => true,
                })
                .map(|x| u(x, "Id"))
                .collect()
        }
    };
    let d = sp::digest(&admin).await?;
    let mut out = Vec::new();
    for id in ids {
        let body = sp::request(
            r#"<ObjectPath Id="16" ObjectPathId="15" /><ObjectPath Id="18" ObjectPathId="17" /><ObjectPath Id="20" ObjectPathId="19" /><ObjectPath Id="22" ObjectPathId="21" /><Query Id="23" ObjectPathId="21"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<Constructor Id="15" TypeId="{SP_TYPE}" /><Property Id="17" ParentId="15" Name="PermissionRequests" /><Method Id="19" ParentId="17" Name="GetById"><Parameters><Parameter Type="Guid">{{{}}}</Parameter></Parameters></Method><Method Id="21" ParentId="19" Name="Approve" />"#,
                util::escape_xml(&id)
            ),
        );
        let json = sp::post(&admin, &d, body).await?;
        sp::check_first(&json)?;
        out.push(last_without_type(&json)?);
    }
    cx.log(if out.len() == 1 {
        out.remove(0)
    } else {
        Value::Array(out)
    })
}

pub async fn permissionrequest_deny(cx: &mut Cx) -> Result<()> {
    let (admin, d) = admin_digest(cx).await?;
    let body = sp::request(
        r#"<ObjectPath Id="160" ObjectPathId="159" /><ObjectPath Id="162" ObjectPathId="161" /><ObjectPath Id="164" ObjectPathId="163" /><Method Name="Deny" Id="165" ObjectPathId="163" />"#,
        &format!(
            r#"<Constructor Id="159" TypeId="{SP_TYPE}" /><Property Id="161" ParentId="159" Name="PermissionRequests" /><Method Id="163" ParentId="161" Name="GetById"><Parameters><Parameter Type="Guid">{{{}}}</Parameter></Parameters></Method>"#,
            ex(cx, "id")
        ),
    );
    sp::check_first(&sp::post(&admin, &d, body).await?)
}

pub async fn permissionrequest_list(cx: &mut Cx) -> Result<()> {
    let (admin, d) = admin_digest(cx).await?;
    let body = sp::request(
        r#"<ObjectPath Id="10" ObjectPathId="9" /><ObjectPath Id="12" ObjectPathId="11" /><Query Id="13" ObjectPathId="11"><Query SelectAllProperties="true"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties /></ChildItemQuery></Query>"#,
        &format!(
            r#"<Constructor Id="9" TypeId="{SP_TYPE}" /><Property Id="11" ParentId="9" Name="PermissionRequests" />"#
        ),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let last = json.last().cloned().unwrap_or(Value::Null);
    let items = match dig(&last, &["_Child_Items_"])? {
        Some(Value::Array(a)) => a.clone(),
        _ => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'length')",
            ));
        }
    };
    let mut result = Vec::new();
    if !items.is_empty() {
        let r = Request::get("https://graph.microsoft.com/v1.0/servicePrincipals/?$filter=displayName eq 'SharePoint Online Web Client Extensibility'").graph_json().data().await?;
        let id = r
            .get("value")
            .and_then(Value::as_array)
            .and_then(|v| v.first())
            .map(|f| u(f, "id"));
        if let Some(id) = id {
            let g = Request::get(format!("https://graph.microsoft.com/v1.0/oAuth2Permissiongrants/?$filter=clientId eq '{id}' and consentType eq 'AllPrincipals'")).graph_json().data().await?;
            if let Some(grants) = g
                .get("value")
                .and_then(Value::as_array)
                .filter(|v| !v.is_empty())
            {
                let mut scopes: Vec<String> = Vec::new();
                for gr in grants {
                    if let Some(s) = gr.get("scope").filter(|s| truthy(s)) {
                        scopes.extend(
                            to_js_string(s)
                                .split(' ')
                                .map(str::trim)
                                .filter(|p| !p.is_empty())
                                .map(String::from),
                        );
                    }
                }
                result = items
                    .into_iter()
                    .filter(|x| {
                        !x.get("Scope")
                            .and_then(Value::as_str)
                            .is_some_and(|s| scopes.iter().any(|p| p == s))
                    })
                    .collect();
            }
        }
    }
    let mut out = Vec::new();
    for r in result {
        let id = js_str(&r, "r.Id", "replace")?
            .replacen("/Guid(", "", 1)
            .replacen(")/", "", 1);
        out.push(obj! { "Id": id, "Resource": ?r.get("Resource").cloned(), "ResourceId": ?r.get("ResourceId").cloned(), "Scope": ?r.get("Scope").cloned() });
    }
    cx.log(Value::Array(out))
}

pub async fn sp_set(cx: &mut Cx) -> Result<()> {
    let enabled = cx.val("enabled").is_some_and(truthy);
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to {} the service principal?",
            if enabled { "enable" } else { "disable" }
        ),
    )? {
        return Ok(());
    }
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log(&format!(
        "{} service principal...",
        if enabled { "Enabling" } else { "Disabling" }
    ));
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="28" ObjectPathId="27" /><SetProperty Id="29" ObjectPathId="27" Name="AccountEnabled"><Parameter Type="Boolean">{}</Parameter></SetProperty><Method Name="Update" Id="30" ObjectPathId="27" /><Query Id="31" ObjectPathId="27"><Query SelectAllProperties="true"><Properties><Property Name="AccountEnabled" ScalarProperty="true" /></Properties></Query></Query>"#,
            cx.val("enabled")
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into())
        ),
        &format!(r#"<Constructor Id="27" TypeId="{SP_TYPE}" />"#),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    cx.log(last_without_type(&json)?)
}

// ---- hubsite -------------------------------------------------------------------

fn lower(v: &Value, field: &str) -> Result<String> {
    Ok(js_str(v, &format!("site.{field}"), "toLowerCase")?.to_lowercase())
}

async fn all_hubs(admin: &str) -> Result<Vec<Value>> {
    let r = Request::get(format!(
        "{admin}/_api/HubSites?$select=ID,Title,SiteUrl&$top=5000"
    ))
    .accept("application/json;odata=minimalmetadata")
    .json_response()
    .data()
    .await?;
    Ok(r.get("value")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default())
}

fn pick_hub(
    hubs: &[Value],
    id: Option<String>,
    title: Option<String>,
    url: Option<String>,
) -> Result<Value> {
    let mut found = Vec::new();
    let (field, want) = if let Some(i) = &id {
        ("ID", i.to_lowercase())
    } else if let Some(t) = &title {
        ("Title", t.to_lowercase())
    } else if let Some(u) = &url {
        ("SiteUrl", u.to_lowercase())
    } else {
        ("", String::new())
    };
    if !field.is_empty() {
        for h in hubs {
            if lower(h, field)? == want {
                found.push(h.clone());
            }
        }
    }
    m365_core::runner::pick_one(
        &found,
        "ID",
        Error::msg(format!(
            "The specified hub site '{}' does not exist.",
            id.or(title.clone())
                .or(url)
                .unwrap_or_else(|| "undefined".into())
        )),
        &format!(
            "Multiple hub sites with name '{}' found.",
            title.unwrap_or_else(|| "undefined".into())
        ),
    )
}

pub async fn connect(cx: &mut Cx) -> Result<()> {
    let first = |ks: &[&str]| {
        ks.iter()
            .find_map(|k| ne(cx, k))
            .unwrap_or_else(|| "undefined".into())
    };
    cx.verbose_log(&format!(
        "Connecting hub site '{}' to hub site '{}'...",
        first(&["id", "title", "url"]),
        first(&["parentId", "parentTitle", "parentUrl"])
    ));
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let hubs = all_hubs(&admin).await?;
    let hub = pick_hub(&hubs, ne(cx, "id"), ne(cx, "title"), ne(cx, "url"))?;
    let parent = pick_hub(
        &hubs,
        ne(cx, "parentId"),
        ne(cx, "parentTitle"),
        ne(cx, "parentUrl"),
    )?;
    let mut r = Request::patch(format!(
        "{admin}/_api/HubSites/GetById('{}')",
        u(&hub, "ID")
    ))
    .spo_json();
    if let Some(e) = hub.get("odata.etag") {
        r = r.header("if-match", to_js_string(e));
    }
    r.json(obj! { "ParentHubSiteId": ?parent.get("ID").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn data_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving hub site data...");
    let web = cx.u("webUrl");
    let r = Request::get(format!(
        "{web}/_api/web/HubSiteData({})",
        cx.val("forceRefresh") == Some(&Value::Bool(true))
    ))
    .spo_json()
    .data()
    .await?;
    if r.get("odata.null") != Some(&Value::Bool(true)) {
        let v = match r.get("value") {
            Some(Value::String(s)) => s.clone(),
            Some(v) => to_js_string(v),
            None => "undefined".into(),
        };
        cx.log(json::jparse(&v)?)
    } else {
        cx.verbose_log(&format!(
            "{web} is not connected to a hub site and is not a hub site itself"
        ));
        Ok(())
    }
}

pub async fn disconnect(cx: &mut Cx) -> Result<()> {
    let who = ["id", "title", "url"]
        .iter()
        .find_map(|k| ne(cx, k))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want disconnect hub site '{who}' from its parent hub site?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Disconnecting hub site '{who}' from its parent hub site..."
    ));
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let hub = match ne(cx, "id") {
        Some(id) => {
            Request::get(format!("{admin}/_api/HubSites/GetById('{id}')?$select=ID"))
                .accept("application/json;odata=minimalmetadata")
                .json_response()
                .data()
                .await?
        }
        None => {
            let hubs = all_hubs(&admin).await?;
            let mut found = Vec::new();
            let (field, want) = match ne(cx, "title") {
                Some(t) => ("Title", Some(t.to_lowercase())),
                None => ("SiteUrl", ne(cx, "url").map(|u| u.to_lowercase())),
            };
            if let Some(w) = want {
                for h in &hubs {
                    if lower(h, field)? == w {
                        found.push(h.clone());
                    }
                }
            }
            match found.len() {
                0 => {
                    return Err(Error::msg(format!(
                        "The specified hub site '{}' does not exist.",
                        ne(cx, "title")
                            .or_else(|| ne(cx, "url"))
                            .unwrap_or_else(|| "undefined".into())
                    )));
                }
                1 => found.remove(0),
                _ => cx.handle_multiple_results(
                    &format!(
                        "Multiple hub sites with name '{}' found.",
                        cx.s("title").unwrap_or_else(|| "undefined".into())
                    ),
                    &util::array_to_hash_table("ID", &found),
                )?,
            }
        }
    };
    let mut r = Request::patch(format!(
        "{admin}/_api/HubSites/GetById('{}')",
        u(&hub, "ID")
    ))
    .spo_json();
    if let Some(e) = hub.get("odata.etag") {
        r = r.header("if-match", to_js_string(e));
    }
    r.json(obj! { "ParentHubSiteId": "00000000-0000-0000-0000-000000000000" })
        .send()
        .await?;
    Ok(())
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let mut hub = match ne(cx, "id") {
        Some(id) => {
            Request::get(format!("{spo_url}/_api/hubsites/getbyid('{id}')"))
                .spo_json()
                .data()
                .await?
        }
        None => {
            let r = Request::get(format!("{spo_url}/_api/hubsites"))
                .spo_json()
                .data()
                .await?;
            let mut hubs = need_arr(&r, "value", "filter")?.to_vec();
            let (field, want) = match ne(cx, "title") {
                Some(t) => ("Title", Some(t)),
                None => ("SiteUrl", ne(cx, "url")),
            };
            if let Some(w) = want {
                let w = w.to_lowercase();
                let mut f = Vec::new();
                for h in hubs {
                    if js_str(&h, &format!("site.{field}"), "toLocaleLowerCase")?.to_lowercase()
                        == w
                    {
                        f.push(h);
                    }
                }
                hubs = f;
            }
            let label = ne(cx, "title")
                .or_else(|| ne(cx, "url"))
                .unwrap_or_else(|| "undefined".into());
            match hubs.len() {
                0 => {
                    return Err(Error::msg(format!(
                        "The specified hub site {label} does not exist"
                    )));
                }
                1 => hubs.remove(0),
                _ => cx.handle_multiple_results(
                    &format!("Multiple hub sites with {label} found."),
                    &util::array_to_hash_table("ID", &hubs),
                )?,
            }
        }
    };
    let output = cx.str("output");
    let with = cx.bool("withAssociatedSites");
    if with
        && output
            .as_deref()
            .is_some_and(|o| !o.is_empty() && o != "json")
    {
        return Err(Error::msg(
            "withAssociatedSites option is only allowed with json output mode",
        ));
    }
    if cx.val("withAssociatedSites") == Some(&Value::Bool(true))
        && output
            .as_deref()
            .is_some_and(|o| !o.is_empty() && o != "text")
    {
        let admin = spo::get_spo_admin_url(cx.debug()).await?;
        let site_opt = hub.get("SiteId").cloned();
        let site_id = site_opt.clone().unwrap_or(Value::Null);
        let o = obj! {
            "output": "json", "debug": ?cx.val("debug").cloned(), "verbose": ?cx.val("verbose").cloned(),
            "listTitle": "DO_NOT_DELETE_SPLIST_TENANTADMIN_AGGREGATED_SITECOLLECTIONS", "webUrl": admin,
            "filter": format!("HubSiteId eq '{}'", if site_opt.is_none() { "undefined".to_string() } else { to_js_string(&site_id) }),
            "fields": "Title,SiteUrl,SiteId",
        };
        let (out, _) = m365_core::runner::run_nested(
            "spo listitem list",
            o.as_object().cloned().unwrap_or_default(),
            true,
        )
        .await?;
        let sites = json::jparse(&out)?;
        let filtered: Vec<Value> = sites
            .as_array()
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter(|s| s.get("SiteId") != site_opt.as_ref())
            .collect();
        if let Some(o) = hub.as_object_mut() {
            o.insert("AssociatedSites".into(), Value::Array(filtered));
        }
    }
    cx.log(hub)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let mut hubs = odata::get_all_items(&format!("{admin}/_api/hubsites")).await?;
    let output = cx.str("output");
    if cx.val("withAssociatedSites") == Some(&Value::Bool(true))
        && !output
            .as_deref()
            .is_some_and(|o| !o.is_empty() && o != "json")
    {
        cx.debug_log("Retrieving associated sites...");
        cx.debug_log("");
        const BATCH: u32 = 30;
        let base = format!(
            "{admin}/_api/web/lists/GetByTitle('DO_NOT_DELETE_SPLIST_TENANTADMIN_AGGREGATED_SITECOLLECTIONS')/RenderListDataAsStream"
        );
        let data = obj! { "parameters": obj! {
            "ViewXml": format!("<View><Query><Where><And><And><IsNull><FieldRef Name=\"TimeDeleted\"/></IsNull><Neq><FieldRef Name=\"State\"/><Value Type='Integer'>0</Value></Neq></And><Neq><FieldRef Name=\"HubSiteId\"/><Value Type='Text'>{{00000000-0000-0000-0000-000000000000}}</Value></Neq></And></Where><OrderBy><FieldRef Name='Title' Ascending='true' /></OrderBy></Query><ViewFields><FieldRef Name=\"Title\"/><FieldRef Name=\"SiteUrl\"/><FieldRef Name=\"SiteId\"/><FieldRef Name=\"HubSiteId\"/></ViewFields><RowLimit Paged=\"TRUE\">{BATCH}</RowLimit></View>"),
            "DatesInUtc": true,
        } };
        cx.debug_log(&format!(
            "Will retrieve associated sites (including the hub sites) in batches of {BATCH}"
        ));
        let mut url = base.clone();
        let mut sites: Vec<Value> = Vec::new();
        let mut batch = 0;
        loop {
            let r = Request::post(&url)
                .spo_json()
                .json(data.clone())
                .data()
                .await?;
            batch += 1;
            let rows = need_arr(&r, "Row", "length")?;
            sites.extend(rows.iter().cloned());
            if cx.debug() {
                m365_core::log::log_stderr_value(&r);
                cx.stderr(&format!("Retrieved {} sites in batch {batch}", rows.len()));
            }
            match r.get("NextHref").filter(|n| truthy(n)) {
                Some(n) => {
                    url = format!("{base}{}", to_js_string(n));
                    cx.debug_log(&format!("Url for next batch of sites: {url}"));
                }
                None => {
                    cx.debug_log(&format!("Retrieved {} sites in total", sites.len()));
                    break;
                }
            }
        }
        for h in hubs.iter_mut() {
            let id = js_str(h, "h.ID", "toUpperCase")?.to_uppercase();
            let mut assoc = Vec::new();
            for f in &sites {
                if f.get("SiteId") == f.get("HubSiteId") {
                    continue;
                }
                if js_str(f, "f.HubSiteId", "toUpperCase")?.to_uppercase() == format!("{{{id}}}") {
                    assoc.push(obj! { "Title": ?f.get("Title").cloned(), "SiteUrl": ?f.get("SiteUrl").cloned() });
                }
            }
            if let Some(o) = h.as_object_mut() {
                o.insert("AssociatedSites".into(), Value::Array(assoc));
            }
        }
    }
    cx.log(Value::Array(hubs))
}

pub async fn register(cx: &mut Cx) -> Result<()> {
    let site = cx.u("siteUrl");
    let d = sp::digest(&site).await?;
    cx.log(
        Request::post(format!("{site}/_api/site/RegisterHubSite"))
            .header("X-RequestDigest", d)
            .spo_json()
            .data()
            .await?,
    )
}

fn principals(cx: &Cx) -> String {
    cx.u("principals")
        .split(',')
        .map(|p| {
            format!(
                r#"<Object Type="String">{}</Object>"#,
                util::escape_xml(p.trim())
            )
        })
        .collect()
}

pub async fn rights_grant(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log(&format!(
        "Granting permissions to join the hub site {} to principals {}...",
        cx.u("hubSiteUrl"),
        cx.u("principals")
    ));
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="37" ObjectPathId="36" /><Method Name="GrantHubSiteRights" Id="38" ObjectPathId="36"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Array">{}</Parameter><Parameter Type="Enum">1</Parameter></Parameters></Method>"#,
            ex(cx, "hubSiteUrl"),
            principals(cx)
        ),
        &format!(r#"<Constructor Id="36" TypeId="{TENANT}" />"#),
    );
    sp::check_first(&sp::post(&admin, &d, body).await?)
}

pub async fn rights_revoke(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", &format!("Are you sure you want to revoke rights to join sites to the hub site {} from the specified users?", cx.u("hubSiteUrl")))? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Revoking rights for {} from {}...",
        cx.u("principals"),
        cx.u("hubSiteUrl")
    ));
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="10" ObjectPathId="9" /><Method Name="RevokeHubSiteRights" Id="11" ObjectPathId="9"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Array">{}</Parameter></Parameters></Method>"#,
            ex(cx, "hubSiteUrl"),
            principals(cx)
        ),
        &format!(r#"<Constructor Id="9" TypeId="{TENANT}" />"#),
    );
    sp::check_first(&sp::post(&admin, &d, body).await?)
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let admin = spo::get_spo_admin_url(cx.debug()).await?;
    let d = sp::digest(&admin).await?;
    cx.verbose_log(&format!("Updating hub site {}...", cx.u("id")));
    let sp_ = |id: u8, name: &str, k: &str| match cx.val(k) {
        Some(Value::String(s)) => format!(
            r#"<SetProperty Id="{id}" ObjectPathId="10" Name="{name}"><Parameter Type="String">{}</Parameter></SetProperty>"#,
            util::escape_xml(s)
        ),
        _ => String::new(),
    };
    let body = sp::request(
        &format!(
            r#"<ObjectPath Id="9" ObjectPathId="8" /><ObjectPath Id="11" ObjectPathId="10" /><Query Id="12" ObjectPathId="10"><Query SelectAllProperties="true"><Properties /></Query></Query>{}{}{}<Method Name="Update" Id="16" ObjectPathId="10" />"#,
            sp_(13, "Title", "title"),
            sp_(14, "LogoUrl", "logoUrl"),
            sp_(15, "Description", "description")
        ),
        &format!(
            r#"<Constructor Id="8" TypeId="{TENANT}" /><Method Id="10" ParentId="8" Name="GetHubSitePropertiesById"><Parameters><Parameter Type="Guid">{}</Parameter></Parameters></Method>"#,
            ex(cx, "id")
        ),
    );
    let json = sp::post(&admin, &d, body).await?;
    sp::check_first(&json)?;
    let mut hub = json.last().cloned().unwrap_or(Value::Null);
    let id = js_str(&hub, "hubSite.ID", "replace")?
        .replacen("/Guid(", "", 1)
        .replacen(")/", "", 1);
    let site = js_str(&hub, "hubSite.SiteId", "replace")?
        .replacen("/Guid(", "", 1)
        .replacen(")/", "", 1);
    if let Some(o) = hub.as_object_mut() {
        o.shift_remove("_ObjectType_");
        o.insert("ID".into(), Value::from(id));
        o.insert("SiteId".into(), Value::from(site));
    }
    cx.log(hub)
}

pub async fn unregister(cx: &mut Cx) -> Result<()> {
    let url = cx.u("url");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to unregister the site collection {url} as a hub site?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Unregistering site collection {url} as a hub site..."
    ));
    let d = sp::digest(&url).await?;
    Request::post(format!("{url}/_api/site/UnregisterHubSite"))
        .header("X-RequestDigest", d)
        .spo_json()
        .send()
        .await?;
    Ok(())
}

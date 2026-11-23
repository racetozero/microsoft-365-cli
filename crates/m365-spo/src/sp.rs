//! SharePoint helpers shared by the `spo` commands: the parts of
//! `src/utils/spo.ts`, `base-permissions.ts` and `propertybag-base.ts` that
//! only SharePoint commands use.

use m365_core::prelude::*;
use m365_core::ut::spo::application_name;
use m365_core::ut::url as urlutil;

/// The `<Request ...>` envelope every CSOM call uses.
pub fn request(actions: &str, object_paths: &str) -> String {
    format!(
        r#"<Request AddExpandoFieldTypeSuffix="true" SchemaVersion="15.0.0.0" LibraryVersion="16.0.0.0" ApplicationName="{}" xmlns="http://schemas.microsoft.com/sharepoint/clientquery/2009"><Actions>{actions}</Actions><ObjectPaths>{object_paths}</ObjectPaths></Request>"#,
        application_name()
    )
}

/// POSTs a CSOM body to `ProcessQuery` and parses the response (`JSON.parse(res)`).
pub async fn post(web: &str, digest: &str, body: String) -> Result<Vec<Value>> {
    let text = m365_core::ut::spo::process_query(web, digest, body).await?;
    parse(&text)
}

pub fn parse(text: &str) -> Result<Vec<Value>> {
    match json::jparse(text)? {
        Value::Array(a) => Ok(a),
        _ => Err(Error::js("json.find is not a function")),
    }
}

/// `json.find(x => x.ErrorInfo)`: throws `ErrorMessage || 'ClientSvc unknown error'`.
pub fn check(json: &[Value]) -> Result<()> {
    if let Some(e) = json
        .iter()
        .find_map(|x| x.get("ErrorInfo").filter(|e| truthy(e)))
    {
        let m = e
            .get("ErrorMessage")
            .filter(|m| truthy(m))
            .cloned()
            .unwrap_or(Value::from("ClientSvc unknown error"));
        return Err(Error::Thrown(m));
    }
    Ok(())
}

/// `json[0].ErrorInfo` (most commands only look at the first entry).
pub fn check_first(json: &[Value]) -> Result<()> {
    let first = json
        .first()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'ErrorInfo')"))?;
    if let Some(e) = first.get("ErrorInfo").filter(|e| truthy(e)) {
        return Err(Error::Thrown(
            e.get("ErrorMessage").cloned().unwrap_or(Value::Null),
        ));
    }
    Ok(())
}

/// The first entry with a truthy `key`.
pub fn find<'a>(json: &'a [Value], key: &str) -> Option<&'a Value> {
    json.iter().find(|x| x.get(key).is_some_and(truthy))
}

#[derive(Clone, Debug)]
pub struct Identity {
    pub object_identity: String,
    pub server_relative_url: String,
}

/// `spo.getCurrentWebIdentity`.
pub async fn web_identity(web: &str, digest: &str) -> Result<Identity> {
    let body = request(
        r#"<Query Id="1" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="ServerRelativeUrl" ScalarProperty="true" /></Properties></Query></Query>"#,
        r#"<Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" />"#,
    );
    let json = post(web, digest, body).await?;
    check(&json)?;
    match find(&json, "_ObjectIdentity_") {
        Some(o) => Ok(Identity {
            object_identity: u(o, "_ObjectIdentity_"),
            server_relative_url: u(o, "ServerRelativeUrl"),
        }),
        None => Err(Error::msg("Cannot proceed. _ObjectIdentity_ not found")),
    }
}

/// `spo.getEffectiveBasePermissions`.
pub async fn effective_base_permissions(
    cx: &Cx,
    identity: &str,
    web: &str,
    digest: &str,
) -> Result<BasePermissions> {
    let body = request(
        r#"<Query Id="11" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="EffectiveBasePermissions" ScalarProperty="true" /></Properties></Query></Query>"#,
        &format!(r#"<Identity Id="5" Name="{identity}" />"#),
    );
    let text = m365_core::ut::spo::process_query(web, digest, body).await?;
    if cx.debug() {
        cx.log(Value::from(
            "Attempt to get the web EffectiveBasePermissions",
        ))?;
    }
    let json = parse(&text)?;
    check(&json)?;
    match find(&json, "EffectiveBasePermissions") {
        Some(p) => {
            let p = &p["EffectiveBasePermissions"];
            Ok(BasePermissions::new(p.get("High"), p.get("Low")))
        }
        None => Err(Error::msg(
            "Cannot proceed. EffectiveBasePermissions not found",
        )),
    }
}

/// `spo.getFolderIdentity`.
pub async fn folder_identity(
    identity: &str,
    web: &str,
    site_relative: &str,
    digest: &str,
) -> Result<Identity> {
    let path = urlutil::get_server_relative_path(web, site_relative);
    let body = request(
        r#"<ObjectPath Id="10" ObjectPathId="9" /><ObjectIdentityQuery Id="11" ObjectPathId="9" /><Query Id="12" ObjectPathId="9"><Query SelectAllProperties="false"><Properties><Property Name="Properties" SelectAll="true"><Query SelectAllProperties="false"><Properties /></Query></Property></Properties></Query></Query>"#,
        &format!(
            r#"<Method Id="9" ParentId="5" Name="GetFolderByServerRelativeUrl"><Parameters><Parameter Type="String">{path}</Parameter></Parameters></Method><Identity Id="5" Name="{identity}" />"#
        ),
    );
    let json = post(web, digest, body).await?;
    check(&json)?;
    match find(&json, "_ObjectIdentity_") {
        Some(o) => Ok(Identity {
            object_identity: u(o, "_ObjectIdentity_"),
            server_relative_url: path,
        }),
        None => Err(Error::msg(
            "Cannot proceed. Folder _ObjectIdentity_ not found",
        )),
    }
}

/// `SpoPropertyBagBaseCommand.setProperty`.
pub async fn set_property(
    name: &str,
    value: &str,
    web: &str,
    digest: &str,
    identity: &Identity,
    folder: Option<&str>,
) -> Result<Vec<Value>> {
    let object_type = if folder.is_some() {
        "Properties"
    } else {
        "AllProperties"
    };
    let body = request(
        &format!(
            r#"<Method Name="SetFieldValue" Id="206" ObjectPathId="205"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method><Method Name="Update" Id="207" ObjectPathId="198" />"#,
            util::escape_xml(name),
            util::escape_xml(value)
        ),
        &format!(
            r#"<Property Id="205" ParentId="198" Name="{object_type}" /><Identity Id="198" Name="{}" />"#,
            identity.object_identity
        ),
    );
    let json = post(web, digest, body).await?;
    check(&json)?;
    Ok(json)
}

/// `SpoPropertyBagBaseCommand.isNoScriptSite`.
pub async fn is_no_script_site(
    cx: &Cx,
    web: &str,
    digest: &str,
    identity: &Identity,
) -> Result<bool> {
    Ok(
        !effective_base_permissions(cx, &identity.object_identity, web, digest)
            .await?
            .has(ADD_AND_CUSTOMIZE_PAGES),
    )
}

// ---- permissions ---------------------------------------------------------------

pub const ADD_AND_CUSTOMIZE_PAGES: i32 = 19;
const FULL_MASK: i32 = 65;

/// `PermissionKind`, in declaration order.
pub const PERMISSION_KINDS: [(&str, i32); 38] = [
    ("EmptyMask", 0),
    ("ViewListItems", 1),
    ("AddListItems", 2),
    ("EditListItems", 3),
    ("DeleteListItems", 4),
    ("ApproveItems", 5),
    ("OpenItems", 6),
    ("ViewVersions", 7),
    ("DeleteVersions", 8),
    ("CancelCheckout", 9),
    ("ManagePersonalViews", 10),
    ("ManageLists", 12),
    ("ViewFormPages", 13),
    ("AnonymousSearchAccessList", 14),
    ("Open", 17),
    ("ViewPages", 18),
    ("AddAndCustomizePages", 19),
    ("ApplyThemeAndBorder", 20),
    ("ApplyStyleSheets", 21),
    ("ViewUsageData", 22),
    ("CreateSSCSite", 23),
    ("ManageSubwebs", 24),
    ("CreateGroups", 25),
    ("ManagePermissions", 26),
    ("BrowseDirectories", 27),
    ("BrowseUserInfo", 28),
    ("AddDelPrivateWebParts", 29),
    ("UpdatePersonalWebParts", 30),
    ("ManageWeb", 31),
    ("AnonymousSearchAccessWebLists", 32),
    ("UseClientIntegration", 37),
    ("UseRemoteAPIs", 38),
    ("ManageAlerts", 39),
    ("CreateAlerts", 40),
    ("EditMyUserInfo", 41),
    ("EnumeratePermissions", 63),
    ("FullMask", 65),
    ("", -1),
];

pub const ROLE_TYPES: [&str; 9] = [
    "None",
    "Guest",
    "Reader",
    "Contributor",
    "WebDesigner",
    "Administrator",
    "Editor",
    "Reviewer",
    "System",
];

#[derive(Clone, Copy, Debug, Default)]
pub struct BasePermissions {
    pub high: i64,
    pub low: i64,
}

fn num(v: Option<&Value>) -> i64 {
    v.map(|x| json::js_number(&to_js_string(x)))
        .filter(|n| n.is_finite())
        .map(|n| n as i64)
        .unwrap_or(0)
}

impl BasePermissions {
    pub fn new(high: Option<&Value>, low: Option<&Value>) -> BasePermissions {
        BasePermissions {
            high: num(high),
            low: num(low),
        }
    }

    /// `BasePermissions.has` with JavaScript's 32-bit bitwise operators.
    pub fn has(&self, perm: i32) -> bool {
        let mut r = false;
        if perm == FULL_MASK {
            r = (self.high as i32 & 32767) == 32767 && self.low == 65535;
        }
        let a = perm - 1;
        if (0..32).contains(&a) {
            r = (self.low as i32) & (1i32 << a) != 0;
        } else if (32..64).contains(&a) {
            r = (self.high as i32) & (1i32 << (a - 32)) != 0;
        }
        r
    }

    /// `BasePermissions.parse`: the names of the permissions held.
    pub fn parse(&self) -> Vec<Value> {
        PERMISSION_KINDS
            .iter()
            .filter(|(n, _)| !n.is_empty())
            .filter(|(_, k)| self.has(*k))
            .map(|(n, _)| Value::from(*n))
            .collect()
    }

    /// `BasePermissions.set`.
    pub fn set(&mut self, perm: i32) {
        if perm == FULL_MASK {
            self.low = 65535;
            self.high = 32767;
        } else if perm == 0 {
            self.low = 0;
            self.high = 0;
        } else {
            let n = perm - 1;
            if (0..32).contains(&n) {
                self.low = ((self.low as i32) | (1i32 << n)) as i64;
            } else if (32..64).contains(&n) {
                self.high = ((self.high as i32) | (1i32 << (n - 32))) as i64;
            }
        }
    }
}

/// Adds `BasePermissionsValue` and `RoleTypeKindValue` to a role definition.
pub fn friendly_role(r: &mut Value) -> Result<()> {
    let bp = r
        .get("BasePermissions")
        .cloned()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'High')"))?;
    let perms = BasePermissions::new(bp.get("High"), bp.get("Low"));
    let kind = r
        .get("RoleTypeKind")
        .and_then(Value::as_u64)
        .and_then(|k| ROLE_TYPES.get(k as usize))
        .map(|k| Value::from(*k));
    if let Some(o) = r.as_object_mut() {
        o.insert("BasePermissionsValue".into(), Value::Array(perms.parse()));
        match kind {
            Some(k) => o.insert("RoleTypeKindValue".into(), k),
            None => o.shift_remove("RoleTypeKindValue"),
        };
    }
    Ok(())
}

/// `formatting.setFriendlyPermissions`.
pub fn set_friendly_permissions(items: &mut [Value]) -> Result<()> {
    for r in items.iter_mut() {
        friendly_role(r)?;
    }
    Ok(())
}

// ---- lookups -------------------------------------------------------------------

pub async fn role_definition_by_name(cx: &Cx, web: &str, name: &str) -> Result<Value> {
    cx.verbose_log(&format!("Retrieving the role definitions for {name}"));
    let items = odata::get_all_items(&format!("{web}/_api/web/roledefinitions")).await?;
    let mut r = items
        .into_iter()
        .find(|r| s(r, "Name") == name)
        .ok_or_else(|| {
            Error::Thrown(Value::from(format!(
                "No roledefinition is found for {name}"
            )))
        })?;
    friendly_role(&mut r)?;
    Ok(r)
}

/// `spo.getRoleDefintionByName` (the misspelled variant `spo folder` uses).
pub async fn role_definition_by_name_strict(cx: &Cx, web: &str, name: &str) -> Result<Value> {
    cx.verbose_log(&format!("Retrieving the role definition by name {name}"));
    let items = odata::get_all_items(&format!("{web}/_api/web/roledefinitions")).await?;
    let mut r = items
        .into_iter()
        .find(|r| s(r, "Name") == name)
        .ok_or_else(|| {
            Error::js(format!(
                "The specified role definition name '{name}' does not exist."
            ))
        })?;
    friendly_role(&mut r)?;
    Ok(r)
}

pub async fn group_by_name(cx: &Cx, web: &str, name: &str) -> Result<Value> {
    cx.verbose_log(&format!("Retrieving the group by name {name}"));
    Request::get(format!(
        "{web}/_api/web/sitegroups/GetByName('{}')",
        eqp(name)
    ))
    .spo_json()
    .data()
    .await
}

pub async fn user_by_email(cx: &Cx, web: &str, email: &str) -> Result<Value> {
    cx.verbose_log(&format!("Retrieving the spo user by email {email}"));
    Request::get(format!(
        "{web}/_api/web/siteusers/GetByEmail('{}')",
        eqp(email)
    ))
    .spo_json()
    .data()
    .await
}

pub async fn ensure_user(web: &str, logon: &str) -> Result<Value> {
    Request::post(format!("{web}/_api/web/EnsureUser"))
        .spo_json()
        .json(obj! { "logonName": logon })
        .data()
        .await
}

/// `spo.ensureEntraGroup`.
pub async fn ensure_entra_group(web: &str, group: &Value) -> Result<Value> {
    if !group.get("securityEnabled").is_some_and(truthy) {
        return Err(Error::js(
            "Cannot ensure a Microsoft Entra ID group that is not security enabled.",
        ));
    }
    let id = u(group, "id");
    let logon = if group.get("mailEnabled").is_some_and(truthy) {
        format!("c:0o.c|federateddirectoryclaimprovider|{id}")
    } else {
        format!("c:0t.c|tenant|{id}")
    };
    ensure_user(web, &logon).await
}

/// `spo.getWeb`.
pub async fn get_web(cx: &Cx, url: &str) -> Result<Value> {
    cx.verbose_log(&format!("Getting the web properties for {url}"));
    Request::get(format!("{url}/_api/web"))
        .spo_json()
        .data()
        .await
}

/// `spo.getSiteIdBySPApi`.
pub async fn site_id_by_sp_api(cx: &Cx, web: &str) -> Result<String> {
    cx.verbose_log(&format!("Getting site id for URL: {web}..."));
    let r = Request::get(format!("{web}/_api/site?$select=Id"))
        .spo_json()
        .data()
        .await?;
    Ok(u(&r, "Id"))
}

/// `spo.getWebId`.
pub async fn web_id(cx: &Cx, web: &str) -> Result<String> {
    cx.verbose_log(&format!("Getting web id for URL: {web}..."));
    let r = Request::get(format!("{web}/_api/web?$select=Id"))
        .spo_json()
        .data()
        .await?;
    Ok(u(&r, "Id"))
}

/// `spo.getListId(webUrl, listTitle?, listUrl?)`.
pub async fn list_id(cx: &Cx, web: &str, title: Option<&str>, url: Option<&str>) -> Result<String> {
    cx.verbose_log("Retrieving list id...");
    let seg = match (
        title.filter(|t| !t.is_empty()),
        url.filter(|u| !u.is_empty()),
    ) {
        (Some(t), _) => format!("lists/getByTitle('{}')", eqp(t)),
        (None, Some(l)) => format!(
            "GetList('{}')",
            eqp(&urlutil::get_server_relative_path(web, l))
        ),
        _ => return Err(Error::js("Either listTitle or listUrl must be provided.")),
    };
    let r = Request::get(format!("{web}/_api/web/{seg}?$select=Id"))
        .spo_json()
        .data()
        .await?;
    Ok(u(&r, "Id"))
}

/// `spo.getFolderServerRelativeUrl`.
pub async fn folder_server_relative_url(
    cx: &Cx,
    web: &str,
    folder_url: Option<&str>,
    folder_id: Option<&str>,
) -> Result<String> {
    cx.verbose_log(&format!(
        "Retrieving server-relative URL for folder {}",
        match folder_url {
            Some(f) => format!("URL: {f}"),
            None => format!("ID: {}", folder_id.unwrap_or("undefined")),
        }
    ));
    let seg = match folder_url {
        Some(f) => format!(
            "GetFolderByServerRelativePath(decodedUrl='{}')",
            eqp(&urlutil::get_server_relative_path(web, f))
        ),
        None => format!("GetFolderById('{}')", folder_id.unwrap_or("undefined")),
    };
    let r = Request::get(format!("{web}/_api/web/{seg}?$select=ServerRelativeUrl"))
        .spo_json()
        .data()
        .await?;
    Ok(u(&r, "ServerRelativeUrl"))
}

/// `listId` / `listTitle` / `listUrl` as a REST path segment after `_api/web`.
pub fn list_segment(
    web: &str,
    id: Option<String>,
    title: Option<String>,
    url: Option<String>,
) -> String {
    if let Some(i) = id {
        format!("lists(guid'{}')", eqp(&i))
    } else if let Some(t) = title {
        format!("lists/getByTitle('{}')", eqp(&t))
    } else if let Some(l) = url {
        format!(
            "GetList('{}')",
            eqp(&urlutil::get_server_relative_path(web, &l))
        )
    } else {
        String::new()
    }
}

/// `spo.getRequestDigest(url).FormDigestValue`.
pub async fn digest(url: &str) -> Result<String> {
    Ok(u(
        &m365_core::ut::spo::get_request_digest(url).await?,
        "FormDigestValue",
    ))
}

/// `spo.ensureFolder`: creates the missing folders of `folder` one by one.
pub async fn ensure_folder(cx: &Cx, web: &str, folder: &str) -> Result<()> {
    if url::Url::parse(web).is_err() {
        return Err(Error::js("webFullUrl is not a valid URL"));
    }
    if folder.is_empty() {
        return Err(Error::js("folderToEnsure cannot be empty"));
    }
    let web = if web.len() > 1 && web.ends_with('/') {
        &web[..web.len() - 1]
    } else {
        web
    };
    let folder = urlutil::get_web_relative_path(web, folder);
    if cx.debug() {
        cx.log(Value::from("folderToEnsure"))?;
        cx.log(Value::from(folder.as_str()))?;
        cx.log(Value::from(""))?;
    }
    let folders: Vec<String> = folder
        .get(1..)
        .unwrap_or("")
        .split('/')
        .map(String::from)
        .collect();
    if cx.debug() {
        cx.log(Value::from("folders to process"))?;
        cx.log(Value::from(stringify(&Value::from(folders.clone()))))?;
        cx.log(Value::from(""))?;
    }
    let mut next = String::new();
    for f in &folders {
        let prev = next.clone();
        next.push_str(&format!("/{f}"));
        let rel = urlutil::get_server_relative_path(web, &next);
        let exists = Request::get(format!(
            "{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl='{}')",
            eqp(&rel)
        ))
        .accept("application/json;odata=nometadata")
        .send()
        .await
        .is_ok();
        if exists {
            continue;
        }
        let prev_rel = urlutil::get_server_relative_path(web, &prev);
        let add = Request::post(format!(
            "{web}/_api/web/GetFolderByServerRelativePath(DecodedUrl=@a1)/AddSubFolderUsingPath(DecodedUrl=@a2)?@a1=%27{}%27&@a2=%27{}%27",
            eqp(&prev_rel),
            eqp(f)
        ))
        .spo_json()
        .send()
        .await;
        if let Err(e) = add {
            if cx.debug() {
                cx.log(Value::from(format!("Could not create sub-folder {rel}")))?;
            }
            return Err(e);
        }
    }
    if cx.debug() {
        cx.log(Value::from("All sub-folders exist"))?;
    }
    Ok(())
}

/// `spo.requestObjectIdentity`.
pub async fn request_object_identity(cx: &Cx, web: &str) -> Result<String> {
    let d = digest(web).await?;
    let body = request(
        r#"<Query Id="1" ObjectPathId="5"><Query SelectAllProperties="false"><Properties><Property Name="ServerRelativeUrl" ScalarProperty="true" /></Properties></Query></Query>"#,
        r#"<Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" />"#,
    );
    let text = m365_core::ut::spo::process_query(web, &d, body).await?;
    cx.verbose_log("Attempt to get _ObjectIdentity_ key values");
    let json = parse(&text)?;
    check(&json)?;
    match find(&json, "_ObjectIdentity_") {
        Some(o) => Ok(u(o, "_ObjectIdentity_")),
        None => Err(Error::msg("Cannot proceed. _ObjectIdentity_ not found")),
    }
}

/// `spo.updateListItem`.
pub async fn update_list_item(
    list_url: &str,
    item_id: &str,
    properties: Option<&Obj>,
    content_type: Option<&str>,
) -> Result<Value> {
    let mut values: Vec<Value> = properties
        .map(|p| {
            p.iter()
                .map(|(k, v)| obj! { "FieldName": k.as_str(), "FieldValue": to_js_string(v) })
                .collect()
        })
        .unwrap_or_default();
    if let Some(c) = content_type {
        values.push(obj! { "FieldName": "ContentType", "FieldValue": c });
    }
    let r = Request::post(format!(
        "{list_url}/items({item_id})/ValidateUpdateListItem()"
    ))
    .spo_json()
    .json(obj! { "formValues": values })
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
            "Updating the items has failed with the following errors: {}{}",
            m365_core::term::EOL,
            bad.join(m365_core::term::EOL)
        )));
    }
    Request::get(format!("{list_url}/items({item_id})"))
        .spo_json()
        .data()
        .await
}

/// `spo.getFileById`.
pub async fn file_by_id(cx: &Cx, web: &str, id: &str) -> Result<Value> {
    cx.verbose_log(&format!("Retrieving the file with id {id}"));
    Request::get(format!("{web}/_api/web/GetFileById('{}')", eqp(id)))
        .spo_json()
        .data()
        .await
}

/// `spo.getSiteAdminPropertiesByUrl`.
pub async fn site_admin_properties(cx: &Cx, site: &str, include_detail: bool) -> Result<Value> {
    cx.verbose_log(&format!("Getting site admin properties for URL: {site}..."));
    let admin = m365_core::ut::spo::get_spo_admin_url(cx.verbose()).await?;
    Request::post(format!("{admin}/_api/SPO.Tenant/GetSitePropertiesByUrl"))
        .spo_json()
        .header("content-type", "application/json;charset=utf-8")
        .json(obj! { "url": site, "includeDetail": include_detail })
        .data()
        .await
}

/// A CSOM call against the tenant admin site with a fresh digest.
pub async fn admin_post(cx: &Cx, verbose_flag: bool, body: String) -> Result<Vec<Value>> {
    let admin = m365_core::ut::spo::get_spo_admin_url(if verbose_flag {
        cx.verbose()
    } else {
        cx.debug()
    })
    .await?;
    let d = digest(&admin).await?;
    post(&admin, &d, body).await
}

// ---- navigation menu state -----------------------------------------------------

/// `spo.getMenuState(webUrl, menuNodeKey?)`.
pub async fn menu_state(web: &str, key: Option<&str>) -> Result<Value> {
    Request::post(format!("{web}/_api/navigation/MenuState"))
        .spo_json()
        .json(obj! { "customProperties": Value::Null, "depth": 10, "mapProviderName": Value::Null, "menuNodeKey": key.map(Value::from).unwrap_or(Value::Null) })
        .data()
        .await
}

pub async fn top_navigation_menu_state(web: &str) -> Result<Value> {
    menu_state(web, Some("1002")).await
}

pub async fn quick_launch_menu_state(web: &str) -> Result<Value> {
    menu_state(web, None).await
}

pub async fn save_menu_state(web: &str, state: &Value) -> Result<()> {
    Request::post(format!("{web}/_api/navigation/SaveMenuState"))
        .spo_json()
        .json(obj! { "menuState": state.clone() })
        .send()
        .await?;
    Ok(())
}

/// `spo.findMenuStateNode`: the path of child indexes to the node.
pub fn find_menu_node(nodes: &[Value], key: &str) -> Result<Option<Vec<usize>>> {
    for (i, n) in nodes.iter().enumerate() {
        if n.get("Key").map(to_js_string).as_deref() == Some(key) {
            return Ok(Some(vec![i]));
        }
        if let Some(mut p) = find_menu_node(need_arr(n, "Nodes", "iterator")?, key)? {
            p.insert(0, i);
            return Ok(Some(p));
        }
    }
    Ok(None)
}

/// The node at `path` (from [`find_menu_node`]).
pub fn menu_node_mut<'a>(state: &'a mut Value, path: &[usize]) -> Option<&'a mut Value> {
    let mut cur = state.get_mut("Nodes")?.get_mut(path[0])?;
    for &i in &path[1..] {
        cur = cur.get_mut("Nodes")?.get_mut(i)?;
    }
    Some(cur)
}

/// `spo.getMenuStateNodeByKey`: the menu state and the path of the node.
pub async fn menu_node_by_key(
    web: &str,
    key: &str,
    location: Option<&str>,
) -> Result<(Value, Vec<usize>)> {
    let locs: Vec<&str> = match location {
        Some(l) => vec![l],
        None => vec!["QuickLaunch", "TopNavigationBar"],
    };
    for l in locs {
        let state = if l == "TopNavigationBar" {
            top_navigation_menu_state(web).await?
        } else {
            quick_launch_menu_state(web).await?
        };
        if let Some(p) = find_menu_node(need_arr(&state, "Nodes", "iterator")?, key)? {
            return Ok((state, p));
        }
    }
    Err(Error::msg(match location {
        Some(l) => format!("Navigation node with id '{key}' not found in {l}."),
        None => format!("Navigation node with id '{key}' not found."),
    }))
}

/// `spo.systemUpdateListItem`.
pub async fn system_update_list_item(
    cx: &Cx,
    list_url: &str,
    item_id: &str,
    properties: Option<&Obj>,
    content_type: Option<&str>,
) -> Result<Value> {
    let content_type = content_type.filter(|c| !c.is_empty());
    if properties.is_none() && content_type.is_none() {
        return Err(Error::msg(
            "Either properties or contentTypeName must be provided for systemUpdateListItem.",
        ));
    }
    let web = &list_url[..list_url.find("/_api/").unwrap_or(list_url.len())];
    cx.verbose_log("Getting list id...");
    let list = Request::get(format!("{list_url}?$select=Id"))
        .spo_json()
        .data()
        .await?;
    let list_id = u(&list, "Id");
    cx.verbose_log("Getting request digest for systemUpdate request");
    let d = digest(web).await?;
    let identity = request_object_identity(cx, web).await?;
    let mut index = 0;
    let mut opts = String::new();
    for (k, v) in properties.into_iter().flatten() {
        index += 1;
        opts.push_str(&format!(
            "\n    <Method Name=\"ParseAndSetFieldValue\" Id=\"{index}\" ObjectPathId=\"147\">\n      <Parameters>\n        <Parameter Type=\"String\">{k}</Parameter>\n        <Parameter Type=\"String\">{}</Parameter>\n      </Parameters>\n    </Method>",
            to_js_string(v)
        ));
    }
    if let Some(c) = content_type {
        index += 1;
        opts.push_str(&format!(
            "\n    <Method Name=\"ParseAndSetFieldValue\" Id=\"{index}\" ObjectPathId=\"147\">\n      <Parameters>\n        <Parameter Type=\"String\">ContentType</Parameter>\n        <Parameter Type=\"String\">{c}</Parameter>\n      </Parameters>\n    </Method>"
        ));
    }
    index += 1;
    let body = format!(
        "<Request AddExpandoFieldTypeSuffix=\"true\" SchemaVersion=\"15.0.0.0\" LibraryVersion=\"16.0.0.0\" ApplicationName=\"{}\" xmlns=\"http://schemas.microsoft.com/sharepoint/clientquery/2009\">\n      <Actions>\n        {opts}\n        <Method Name=\"SystemUpdate\" Id=\"{index}\" ObjectPathId=\"147\" />\n      </Actions>\n      <ObjectPaths>\n        <Identity Id=\"147\" Name=\"{identity}:list:{list_id}:item:{item_id},1\" />\n      </ObjectPaths>\n    </Request>",
        application_name()
    );
    let r = Request::post(format!("{web}/_vti_bin/client.svc/ProcessQuery"))
        .header("Content-Type", "text/xml")
        .header("X-RequestDigest", d)
        .text(body)
        .send()
        .await?
        .text();
    if r.contains("ErrorMessage") {
        return Err(Error::msg(format!(
            "Error occurred in systemUpdate operation - {r}"
        )));
    }
    Request::get(format!("{list_url}/items({item_id})"))
        .spo_json()
        .data()
        .await
}

/// `spo.applyRetentionLabelToListItems` (`name` empty removes the label).
pub async fn set_compliance_tag_on_items(
    cx: &Cx,
    web: &str,
    name: &str,
    list_abs: &str,
    ids: Vec<Value>,
    verbose: bool,
) -> Result<()> {
    if verbose {
        if name.is_empty() {
            cx.stderr(&format!(
                "Removing retention label from item(s) in list '{list_abs}'..."
            ));
        } else {
            cx.stderr(&format!(
                "Applying retention label '{name}' to item(s) in list '{list_abs}'..."
            ));
        }
    }
    Request::post(format!(
        "{web}/_api/SP_CompliancePolicy_SPPolicyStoreProxy_SetComplianceTagOnBulkItems"
    ))
    .spo_json()
    .json(obj! { "listUrl": list_abs, "complianceTagValue": name, "itemIds": ids })
    .send()
    .await?;
    Ok(())
}

/// `spo.createFolderCopyJob` / `spo.createFileCopyJob`: the first job info.
pub async fn create_copy_job(web: &str, source: &str, dest: &str, options: Value) -> Result<Value> {
    let r = Request::post(format!("{web}/_api/Site/CreateCopyJobs"))
        .spo_json()
        .json(obj! { "destinationUri": dest, "exportObjectUris": vec![source], "options": options })
        .data()
        .await?;
    match r.get("value") {
        Some(Value::Array(a)) => Ok(a.first().cloned().unwrap_or(Value::Null)),
        _ => Err(Error::js(
            "Cannot read properties of undefined (reading '0')",
        )),
    }
}

fn copy_logs(p: &Value) -> Result<Option<Vec<Value>>> {
    match p.get("Logs") {
        Some(Value::Array(a)) => a
            .iter()
            .map(|l| json::jparse(&to_js_string(l)))
            .collect::<Result<Vec<_>>>()
            .map(Some),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(Error::js("progress.Logs?.map is not a function")),
    }
}

/// `spo.getCopyJobResult`: polls until the job finishes.
pub async fn copy_job_result(web: &str, info: &Value) -> Result<Value> {
    let req = || {
        Request::post(format!("{web}/_api/Site/GetCopyJobProgress"))
            .spo_json()
            .json(obj! { "copyJobInfo": info.clone() })
            .data()
    };
    let mut logs = Vec::new();
    let mut progress = req().await?;
    logs.extend(copy_logs(&progress)?.unwrap_or_default());
    while progress.get("JobState").and_then(Value::as_f64) != Some(0.0) {
        util::sleep_ms(3000).await;
        progress = req().await?;
        logs.extend(copy_logs(&progress)?.unwrap_or_default());
    }
    if let Some(e) = logs
        .iter()
        .find(|l| l.get("Event").and_then(Value::as_str) == Some("JobError"))
    {
        return Err(Error::js(u(e, "Message")));
    }
    if let Some(o) = logs
        .iter()
        .find(|l| l.get("Event").and_then(Value::as_str) == Some("JobFinishedObjectInfo"))
    {
        return Ok(o.clone());
    }
    progress = req().await?;
    let Some(newer) = copy_logs(&progress)? else {
        return Err(Error::js(
            "Cannot read properties of undefined (reading 'find')",
        ));
    };
    Ok(newer
        .into_iter()
        .find(|l| l.get("Event").and_then(Value::as_str) == Some("JobFinishedObjectInfo"))
        .unwrap_or(Value::Null))
}

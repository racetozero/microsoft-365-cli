//! `spo list *` (except views, webhooks and default values).

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::entra;
use m365_core::ut::url as urlutil;
use m365_core::validation::{self as v, Valid};

fn ne(cx: &Cx, k: &str) -> Option<String> {
    cx.s(k).filter(|s| !s.is_empty())
}

fn opt(cx: &Cx, k: &str) -> String {
    cx.val(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn so(o: &Obj, k: &str) -> Option<String> {
    o.get(k).filter(|v| truthy(v)).map(to_js_string)
}

fn sp_url(o: &Obj) -> std::result::Result<(), String> {
    match v::is_valid_sharepoint_url(
        &o.get("webUrl")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
    ) {
        Valid::Msg(m) => Err(m),
        Valid::No => Err("false".into()),
        Valid::Yes => Ok(()),
    }
}

fn guid_check(o: &Obj, k: &str, msg: impl Fn(&str) -> String) -> std::result::Result<(), String> {
    match so(o, k) {
        Some(g) if !v::is_valid_guid(&g) => Err(msg(&g)),
        _ => Ok(()),
    }
}

/// `webUrl` must be valid and `key` (when set) a GUID: `${id} is not a valid GUID`.
pub fn validate_url_guid(o: &Obj, key: &str) -> std::result::Result<(), String> {
    sp_url(o)?;
    guid_check(o, key, |g| format!("{g} is not a valid GUID"))
}

pub fn list_id_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "listId")
}

pub fn id_validate(o: &Obj) -> std::result::Result<(), String> {
    validate_url_guid(o, "id")
}

pub fn url_validate(o: &Obj) -> std::result::Result<(), String> {
    sp_url(o)
}

/// Validators that check the GUID first and the URL last.
pub fn guid_then_url_validate(o: &Obj) -> std::result::Result<(), String> {
    guid_check(o, "listId", |g| format!("{g} is not a valid GUID"))?;
    sp_url(o)
}

pub fn sensitivity_validate(o: &Obj) -> std::result::Result<(), String> {
    guid_check(o, "id", |g| format!("{g} is not a valid GUID"))?;
    guid_check(o, "listId", |g| format!("{g} is not a valid GUID"))?;
    sp_url(o)
}

fn role_common(o: &Obj) -> std::result::Result<(), String> {
    sp_url(o)?;
    guid_check(o, "listId", |g| {
        format!("'{g}' is not a valid GUID for option listId.")
    })?;
    if let Some(u) = so(o, "upn")
        && !v::is_valid_user_principal_name(&u)
    {
        return Err(format!(
            "'{u}' is not a valid user principal name for option upn."
        ));
    }
    Ok(())
}

pub fn roleassignment_add_validate(o: &Obj) -> std::result::Result<(), String> {
    role_common(o)?;
    if let Some(p) = so(o, "principalId")
        && !v::is_valid_positive_integer(&p)
    {
        return Err(format!(
            "Specified principalId '{p}' is not a valid number."
        ));
    }
    guid_check(o, "entraGroupId", |g| {
        format!("'{g}' is not a valid GUID for option entraGroupId.")
    })?;
    if let Some(p) = so(o, "roleDefinitionId")
        && !v::is_valid_positive_integer(&p)
    {
        return Err(format!(
            "Specified roleDefinitionId '{p}' is not a valid number."
        ));
    }
    Ok(())
}

pub fn roleassignment_remove_validate(o: &Obj) -> std::result::Result<(), String> {
    role_common(o)?;
    if let Some(p) = so(o, "principalId")
        && !v::is_valid_positive_integer(&p)
    {
        return Err(format!(
            "'{p}' is not a valid number for option principalId."
        ));
    }
    guid_check(o, "entraGroupId", |g| {
        format!("'{g}' is not a valid GUID for option entraGroupId.")
    })
}

// ---- enums ---------------------------------------------------------------------

const TEMPLATES: [(&str, i64); 19] = [
    ("GenericList", 100),
    ("DocumentLibrary", 101),
    ("Survey", 102),
    ("Links", 103),
    ("Announcements", 104),
    ("Contacts", 105),
    ("Events", 106),
    ("Tasks", 107),
    ("DiscussionBoard", 108),
    ("PictureLibrary", 109),
    ("DataSources", 110),
    ("XmlForm", 115),
    ("NoCodeWorkflows", 117),
    ("WorkflowProcess", 118),
    ("WebPageLibrary", 119),
    ("CustomGrid", 120),
    ("WorkflowHistory", 140),
    ("GanttTasks", 150),
    ("IssuesTracking", 1100),
];
const DRAFT: [(&str, i64); 3] = [("Reader", 0), ("Author", 1), ("Approver", 2)];
const EXPERIENCE: [(&str, i64); 3] = [("Auto", 0), ("NewExperience", 1), ("ClassicExperience", 2)];

/// `Enum[key]` of a numeric TypeScript enum: the number for a name, the
/// name for a numeric key, `None` (undefined) otherwise.
fn enum_get(e: &[(&str, i64)], key: &str) -> Option<Value> {
    if let Some((_, n)) = e.iter().find(|(k, _)| *k == key) {
        return Some(Value::from(*n));
    }
    e.iter()
        .find(|(_, n)| n.to_string() == key)
        .map(|(k, _)| Value::from(*k))
}

pub fn add_validate(o: &Obj) -> std::result::Result<(), String> {
    sp_url(o)?;
    if let Some(t) = so(o, "baseTemplate")
        && !enum_get(&TEMPLATES, t.trim()).is_some_and(|v| truthy(&v))
    {
        return Err(format!("{t} is not a valid baseTemplate value"));
    }
    guid_check(o, "templateFeatureId", |g| {
        format!("{g} in option templateFeatureId is not a valid GUID")
    })?;
    guid_check(o, "defaultContentApprovalWorkflowId", |g| {
        format!("{g} in option defaultContentApprovalWorkflowId is not a valid GUID")
    })?;
    if let Some(d) = so(o, "direction")
        && !["NONE", "LTR", "RTL"].contains(&d.as_str())
    {
        return Err(format!(
            "{d} is not a valid direction value. Allowed values are NONE|LTR|RTL"
        ));
    }
    if let Some(d) = so(o, "draftVersionVisibility")
        && enum_get(&DRAFT, d.trim()).is_none()
    {
        return Err(format!("{d} is not a valid draftVisibilityType value"));
    }
    let is_true = |k: &str| o.get(k) == Some(&Value::Bool(true));
    if so(o, "emailAlias").is_some() && !is_true("enableAssignToEmail") {
        return Err("emailAlias could not be set if enableAssignToEmail is not set to true. Please set enableAssignToEmail.".into());
    }
    if let Some(e) = so(o, "listExperienceOptions")
        && !enum_get(&EXPERIENCE, e.trim()).is_some_and(|v| truthy(&v))
    {
        return Err(format!("{e} is not a valid listExperienceOptions value"));
    }
    if so(o, "majorVersionLimit").is_some() && !is_true("enableVersioning") {
        return Err(
            "majorVersionLimit option is only valid in combination with enableVersioning.".into(),
        );
    }
    if so(o, "majorWithMinorVersionsLimit").is_some()
        && !is_true("enableMinorVersions")
        && !is_true("enableModeration")
    {
        return Err("majorWithMinorVersionsLimit option is only valid in combination with enableMinorVersions or enableModeration.".into());
    }
    let num_in = |k: &str, allowed: &[f64]| {
        o.get(k).filter(|v| truthy(v)).map(|v| {
            (
                to_js_string(v),
                v.as_f64().is_some_and(|n| allowed.contains(&n)),
            )
        })
    };
    if let Some((r, false)) = num_in("readSecurity", &[1.0, 2.0]) {
        return Err(format!(
            "{r} is not a valid readSecurity value. Allowed values are 1|2"
        ));
    }
    if let Some((w, false)) = num_in("writeSecurity", &[1.0, 2.0, 4.0]) {
        return Err(format!(
            "{w} is not a valid writeSecurity value. Allowed values are 1|2|4"
        ));
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum K {
    /// `if (options.x)`
    T,
    /// `if (options.x !== undefined)`
    D,
    Draft,
    Experience,
    /// list set: `majorVersionLimit` also turns versioning on
    MajorVersion,
}

const PROPS: [(&str, &str, K); 60] = [
    ("description", "Description", K::T),
    ("templateFeatureId", "TemplateFeatureId", K::T),
    ("allowDeletion", "AllowDeletion", K::D),
    ("allowEveryoneViewItems", "AllowEveryoneViewItems", K::D),
    ("allowMultiResponses", "AllowMultiResponses", K::D),
    ("contentTypesEnabled", "ContentTypesEnabled", K::D),
    ("crawlNonDefaultViews", "CrawlNonDefaultViews", K::D),
    (
        "defaultContentApprovalWorkflowId",
        "DefaultContentApprovalWorkflowId",
        K::T,
    ),
    ("defaultDisplayFormUrl", "DefaultDisplayFormUrl", K::T),
    ("defaultEditFormUrl", "DefaultEditFormUrl", K::T),
    ("direction", "Direction", K::T),
    ("disableCommenting", "DisableCommenting", K::D),
    ("disableGridEditing", "DisableGridEditing", K::D),
    ("draftVersionVisibility", "DraftVersionVisibility", K::Draft),
    ("emailAlias", "EmailAlias", K::T),
    ("enableAssignToEmail", "EnableAssignToEmail", K::D),
    ("enableAttachments", "EnableAttachments", K::D),
    (
        "enableDeployWithDependentList",
        "EnableDeployWithDependentList",
        K::D,
    ),
    ("enableFolderCreation", "EnableFolderCreation", K::D),
    ("enableMinorVersions", "EnableMinorVersions", K::D),
    ("enableModeration", "EnableModeration", K::D),
    ("enablePeopleSelector", "EnablePeopleSelector", K::D),
    ("enableResourceSelector", "EnableResourceSelector", K::D),
    ("enableSchemaCaching", "EnableSchemaCaching", K::D),
    ("enableSyndication", "EnableSyndication", K::D),
    ("enableThrottling", "EnableThrottling", K::D),
    ("enableVersioning", "EnableVersioning", K::D),
    ("enforceDataValidation", "EnforceDataValidation", K::D),
    ("excludeFromOfflineClient", "ExcludeFromOfflineClient", K::D),
    (
        "fetchPropertyBagForListView",
        "FetchPropertyBagForListView",
        K::D,
    ),
    ("followable", "Followable", K::D),
    ("forceCheckout", "ForceCheckout", K::D),
    ("forceDefaultContentType", "ForceDefaultContentType", K::D),
    ("hidden", "Hidden", K::D),
    ("includedInMyFilesScope", "IncludedInMyFilesScope", K::D),
    ("irmEnabled", "IrmEnabled", K::D),
    ("irmExpire", "IrmExpire", K::D),
    ("irmReject", "IrmReject", K::D),
    ("isApplicationList", "IsApplicationList", K::D),
    (
        "listExperienceOptions",
        "ListExperienceOptions",
        K::Experience,
    ),
    ("majorVersionLimit", "MajorVersionLimit", K::MajorVersion),
    (
        "majorWithMinorVersionsLimit",
        "MajorWithMinorVersionsLimit",
        K::T,
    ),
    ("multipleDataList", "MultipleDataList", K::D),
    ("navigateForFormsPages", "NavigateForFormsPages", K::D),
    ("needUpdateSiteClientTag", "NeedUpdateSiteClientTag", K::D),
    ("noCrawl", "NoCrawl", K::D),
    ("onQuickLaunch", "OnQuickLaunch", K::D),
    ("ordered", "Ordered", K::D),
    ("parserDisabled", "ParserDisabled", K::D),
    ("readOnlyUI", "ReadOnlyUI", K::D),
    ("readSecurity", "ReadSecurity", K::T),
    ("requestAccessEnabled", "RequestAccessEnabled", K::D),
    ("restrictUserUpdates", "RestrictUserUpdates", K::D),
    ("sendToLocationName", "SendToLocationName", K::T),
    ("sendToLocationUrl", "SendToLocationUrl", K::T),
    ("showUser", "ShowUser", K::D),
    ("useFormsForDisplay", "UseFormsForDisplay", K::D),
    ("validationFormula", "ValidationFormula", K::T),
    ("validationMessage", "ValidationMessage", K::T),
    ("writeSecurity", "WriteSecurity", K::T),
];

fn map_props(cx: &Cx, body: &mut Obj, set: bool) {
    for (o, p, k) in PROPS {
        let val = cx.val(o);
        match k {
            K::D => {
                if let Some(v) = val {
                    body.insert(p.into(), v.clone());
                }
            }
            K::T => {
                if let Some(v) = val.filter(|v| truthy(v)) {
                    body.insert(p.into(), v.clone());
                }
            }
            K::MajorVersion => {
                if let Some(v) = val.filter(|v| truthy(v)) {
                    body.insert(p.into(), v.clone());
                    if set {
                        body.insert("EnableVersioning".into(), Value::Bool(true));
                    }
                }
            }
            K::Draft | K::Experience => {
                if let Some(v) = val.filter(|v| truthy(v)) {
                    let e: &[(&str, i64)] = if matches!(k, K::Draft) {
                        &DRAFT
                    } else {
                        &EXPERIENCE
                    };
                    // `requestBody.X = undefined` is dropped by JSON.stringify
                    if let Some(x) = enum_get(e, to_js_string(v).trim()) {
                        body.insert(p.into(), x);
                    }
                }
            }
        }
    }
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Creating list in site at {web}..."));
    let mut body = Obj::new();
    body.insert(
        "Title".into(),
        cx.val("title").cloned().unwrap_or(Value::Null),
    );
    let bt = match ne(cx, "baseTemplate") {
        Some(t) => enum_get(&TEMPLATES, t.trim()).unwrap_or(Value::Null),
        None => Value::from(100),
    };
    body.insert("BaseTemplate".into(), bt);
    if cx.val("title").is_none() {
        body.shift_remove("Title");
    }
    map_props(cx, &mut body, false);
    cx.log(
        Request::post(format!("{web}/_api/web/lists"))
            .spo_json()
            .json(Value::Object(body))
            .data()
            .await?,
    )
}

fn list_seg(cx: &Cx, id: &str, title: &str, url: &str) -> String {
    let web = cx.u("webUrl");
    if let Some(i) = ne(cx, id) {
        format!("lists(guid'{}')", eqp(&i))
    } else if let Some(t) = ne(cx, title) {
        format!("lists/getByTitle('{}')", eqp(&t))
    } else if let Some(l) = ne(cx, url) {
        format!(
            "GetList('{}')",
            eqp(&urlutil::get_server_relative_path(&web, &l))
        )
    } else {
        String::new()
    }
}

fn base(cx: &Cx) -> String {
    format!(
        "{}/_api/web/{}",
        cx.u("webUrl"),
        list_seg(cx, "listId", "listTitle", "listUrl")
    )
}

fn label(cx: &Cx) -> String {
    ["listId", "listTitle", "listUrl"]
        .iter()
        .find_map(|k| ne(cx, k))
        .unwrap_or_else(|| "undefined".into())
}

pub async fn contenttype_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Adding content type {} to list {} in site at {}...",
        opt(cx, "id"),
        label(cx),
        cx.u("webUrl")
    ));
    let r = Request::post(format!("{}/ContentTypes/AddAvailableContentType", base(cx)))
        .spo_json()
        .json(obj! { "contentTypeId": ?cx.val("id").cloned() })
        .data()
        .await?;
    cx.log(r)
}

fn upper(v: &Value, what: &str) -> Result<String> {
    match v.get("StringValue") {
        Some(Value::String(s)) => Ok(s.to_uppercase()),
        None => Err(Error::js(
            "Cannot read properties of undefined (reading 'toUpperCase')",
        )),
        _ => Err(Error::js(format!("{what}.toUpperCase is not a function"))),
    }
}

pub async fn contenttype_default_set(cx: &mut Cx) -> Result<()> {
    let b = base(cx);
    let ct = cx.u("contentTypeId");
    cx.verbose_log("Retrieving content type order...");
    let r = Request::get(format!(
        "{b}/RootFolder?$select=ContentTypeOrder,UniqueContentTypeOrder"
    ))
    .spo_json()
    .data()
    .await?;
    let mut order = match r.get("UniqueContentTypeOrder") {
        Some(Value::Null) => {
            cx.debug_log("Unique content type order not defined. Using content type order");
            r.get("ContentTypeOrder").cloned()
        }
        other => {
            cx.debug_log("Using unique content type order");
            other.cloned()
        }
    };
    let arr = match &mut order {
        Some(Value::Array(a)) => a,
        _ => {
            return Err(Error::js(
                "Cannot read properties of undefined (reading 'findIndex')",
            ));
        }
    };
    let mut idx = None;
    for (i, c) in arr.iter().enumerate() {
        if upper(c, "ct.StringValue")? == ct.to_uppercase() {
            idx = Some(i);
            break;
        }
    }
    let post = |order: Vec<Value>| {
        Request::post(format!("{b}/RootFolder"))
            .spo_json()
            .header("content-type", "application/json;odata=nometadata")
            .header("x-http-method", "MERGE")
            .json(obj! { "UniqueContentTypeOrder": order })
            .send()
    };
    match idx {
        Some(i) => {
            cx.debug_log(&format!("Content type {ct} is visible in the list"));
            if i == 0 {
                cx.verbose_log(&format!("Content type {ct} is already set as default"));
            } else {
                cx.verbose_log(&format!("Setting content type {ct} as default..."));
                arr.remove(i);
                arr.insert(0, obj! { "StringValue": ct.as_str() });
                post(arr.clone()).await?;
            }
        }
        None => {
            cx.debug_log(&format!("Content type {ct} is not visible in the list"));
            cx.verbose_log("Retrieving list content types...");
            let cts = Request::get(format!("{b}/ContentTypes?$select=Id"))
                .spo_json()
                .data()
                .await?;
            let mut ids = Vec::new();
            for c in need_arr(&cts, "value", "map")? {
                let id = c.get("Id").ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'StringValue')")
                })?;
                ids.push(id.get("StringValue").cloned());
            }
            let mut found = false;
            for i in &ids {
                match i {
                    Some(Value::String(s)) if s.to_uppercase() == ct.to_uppercase() => {
                        found = true;
                        break;
                    }
                    Some(Value::String(_)) => {}
                    None => {
                        return Err(Error::js(
                            "Cannot read properties of undefined (reading 'toUpperCase')",
                        ));
                    }
                    _ => return Err(Error::js("ct.toUpperCase is not a function")),
                }
            }
            if !found {
                return Err(Error::msg(format!(
                    "Content type {ct} missing in the list. Add the content type to the list first and try again."
                )));
            }
            cx.verbose_log(&format!("Setting content type {ct} as default..."));
            arr.insert(0, obj! { "StringValue": ct.as_str() });
            post(arr.clone()).await?;
        }
    }
    Ok(())
}

pub async fn contenttype_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving content types information for list {} in site at {}...",
        label(cx),
        cx.u("webUrl")
    ));
    cx.log(Value::Array(
        odata::get_all_items(&format!("{}/ContentTypes?$expand=Parent", base(cx))).await?,
    ))
}

pub async fn contenttype_remove(cx: &mut Cx) -> Result<()> {
    let (id, web) = (opt(cx, "id"), cx.u("webUrl"));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the content type {id} from the list {} in site {web}?",
            label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing content type {id} from list {} in site at {web}...",
        label(cx)
    ));
    Request::post(format!("{}/ContentTypes('{}')", base(cx), eqp(&id)))
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json()
        .send()
        .await?;
    Ok(())
}

// ---- get / list / remove -------------------------------------------------------

const VERSION_TEMPLATES: [i64; 16] = [
    101, 109, 110, 111, 113, 114, 115, 116, 117, 119, 121, 122, 123, 126, 130, 175,
];

fn trim_mode(v: Option<&Value>) -> Option<Value> {
    match v.and_then(Value::as_f64) {
        Some(0.0) => Some(Value::from("NoExpiration")),
        Some(1.0) => Some(Value::from("ExpireAfter")),
        Some(2.0) => Some(Value::from("AutoExpiration")),
        _ => None,
    }
}

fn set_trim(vp: &mut Value) {
    let t = trim_mode(vp.get("DefaultTrimMode"));
    if let Some(o) = vp.as_object_mut() {
        match t {
            Some(t) => {
                o.insert("DefaultTrimModeValue".into(), t);
            }
            None => {
                o.shift_remove("DefaultTrimModeValue");
            }
        }
    }
}

fn dedupe(v: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for x in v {
        if !out.contains(&x) {
            out.push(x);
        }
    }
    out
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Retrieving information for list in site at {web}..."
    ));
    let mut url = format!("{web}/_api/web/");
    if let Some(i) = ne(cx, "id") {
        url.push_str(&format!("lists(guid'{}')", eqp(&i)));
    } else if let Some(t) = ne(cx, "title") {
        url.push_str(&format!("lists/GetByTitle('{}')", eqp(&t)));
    } else if let Some(l) = ne(cx, "url") {
        url.push_str(&format!(
            "GetList('{}')",
            eqp(&urlutil::get_server_relative_path(&web, &l))
        ));
    } else if cx.bool("default") {
        url.push_str("DefaultDocumentLibrary");
    }
    let with = cx.bool("withPermissions");
    let mut select = Vec::new();
    let mut expand: Vec<String> = if with {
        [
            "HasUniqueRoleAssignments",
            "RoleAssignments/Member",
            "RoleAssignments/RoleDefinitionBindings",
            "VersionPolicies",
        ]
        .map(String::from)
        .to_vec()
    } else {
        Vec::new()
    };
    if let Some(p) = ne(cx, "properties") {
        for prop in p.split(',') {
            let parts: Vec<&str> = prop.trim().split('/').collect();
            if parts.len() > 1 {
                expand.push(parts[0].to_string());
            }
            select.push(prop.trim().to_string());
        }
    }
    let (select, expand) = (dedupe(select), dedupe(expand));
    let mut q = Vec::new();
    if !select.is_empty() {
        q.push(format!("$select={}", select.join(",")));
    }
    if !expand.is_empty() {
        q.push(format!("$expand={}", expand.join(",")));
    }
    let full = if q.is_empty() {
        url.clone()
    } else {
        format!("{url}?{}", q.join("&"))
    };
    let mut list = Request::get(full).spo_json().data().await?;
    if with {
        let ras = match list.get_mut("RoleAssignments") {
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
        }
    }
    if list
        .get("BaseTemplate")
        .and_then(Value::as_f64)
        .is_some_and(|b| VERSION_TEMPLATES.contains(&(b as i64)) && b.fract() == 0.0)
    {
        let r = Request::get(format!(
            "{url}?$select=VersionPolicies&$expand=VersionPolicies"
        ))
        .spo_json()
        .data()
        .await?;
        let mut vp = r.get("VersionPolicies").cloned().ok_or_else(|| {
            Error::js("Cannot read properties of undefined (reading 'DefaultTrimMode')")
        })?;
        set_trim(&mut vp);
        if let Some(o) = list.as_object_mut() {
            o.insert("VersionPolicies".into(), vp);
        }
    }
    if let Some(vp) = list.get_mut("VersionPolicies").filter(|v| truthy(v)) {
        set_trim(vp);
    }
    cx.log(list)
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Retrieving all lists in site at {web}..."));
    let mut select = vec!["RootFolder/ServerRelativeUrl".to_string()];
    let mut expand = vec!["RootFolder".to_string()];
    match ne(cx, "properties") {
        None => select.push("*".into()),
        Some(p) => {
            for f in p.split(',') {
                let parts: Vec<&str> = f.trim().split('/').collect();
                if parts.len() > 1 {
                    expand.push(parts[0].to_string());
                }
                select.push(f.trim().to_string());
            }
        }
    }
    let mut q = vec![
        format!("$expand={}", dedupe(expand).join(",")),
        format!("$select={}", dedupe(select).join(",")),
    ];
    if let Some(f) = ne(cx, "filter") {
        q.push(format!("$filter={f}"));
    }
    cx.log(Value::Array(
        odata::get_all_items(&format!("{web}/_api/web/lists?{}", q.join("&"))).await?,
    ))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let who = ne(cx, "id")
        .or_else(|| ne(cx, "title"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the list {who} from site {web}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!("Removing list in site at {web}..."));
    let mut url = match ne(cx, "id") {
        Some(i) => format!("{web}/_api/web/lists(guid'{}')", eqp(&i)),
        None => format!(
            "{web}/_api/web/lists/GetByTitle('{}')",
            eqp(&opt(cx, "title"))
        ),
    };
    if !cx.bool("permanent") {
        url.push_str("/recycle");
    }
    Request::post(url)
        .header("X-HTTP-Method", "DELETE")
        .header("If-Match", "*")
        .spo_json()
        .send()
        .await?;
    Ok(())
}

// ---- retention / sensitivity labels -------------------------------------------

async fn root_folder_url(url: String) -> Result<String> {
    let r = Request::get(url).spo_json().data().await?;
    Ok(match dig(&r, &["RootFolder", "ServerRelativeUrl"])? {
        Some(v) => to_js_string(v),
        None => "undefined".into(),
    })
}

async fn list_server_relative(cx: &Cx) -> Result<String> {
    cx.verbose_log("Getting the list server relative URL");
    let web = cx.u("webUrl");
    if let Some(l) = ne(cx, "listUrl") {
        return Ok(urlutil::get_server_relative_path(&web, &l));
    }
    let seg = match ne(cx, "listId") {
        Some(i) => format!("lists(guid'{}')/", eqp(&i)),
        None => format!("lists/getByTitle('{}')/", eqp(&opt(cx, "listTitle"))),
    };
    root_folder_url(format!(
        "{web}/_api/web/{seg}?$expand=RootFolder&$select=RootFolder/ServerRelativeUrl"
    ))
    .await
}

async fn set_list_tag(cx: &Cx, abs: &str, name: &str, sync: bool) -> Result<()> {
    let web = cx.u("webUrl");
    if cx.val("verbose").is_some_and(truthy) {
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
        .json(obj! { "listUrl": abs, "complianceTagValue": name, "blockDelete": false, "blockEdit": false, "syncToItems": sync })
        .send()
        .await?;
    Ok(())
}

pub async fn retentionlabel_ensure(cx: &mut Cx) -> Result<()> {
    let srv = list_server_relative(cx).await?;
    let abs = urlutil::get_absolute_url(&cx.u("webUrl"), &srv);
    set_list_tag(
        cx,
        &abs,
        &opt(cx, "name"),
        cx.val("syncToItems").is_some_and(truthy),
    )
    .await
}

pub async fn retentionlabel_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Getting label set on the list {} in site at {web}...",
        label(cx)
    ));
    let srv = match ne(cx, "listUrl") {
        Some(l) => {
            cx.debug_log(&format!("Retrieving List from URL '{l}'..."));
            urlutil::get_server_relative_path(&web, &l)
        }
        None => {
            let mut url = format!("{web}/_api/web/");
            if let Some(i) = ne(cx, "listId") {
                cx.debug_log(&format!("Retrieving List from Id '{i}'..."));
                url.push_str(&format!(
                    "lists(guid'{}')?$expand=RootFolder&$select=RootFolder",
                    eqp(&i)
                ));
            } else if let Some(t) = ne(cx, "listTitle") {
                cx.debug_log(&format!("Retrieving List from Title '{t}'..."));
                url.push_str(&format!(
                    "lists/GetByTitle('{}')?$expand=RootFolder&$select=RootFolder",
                    eqp(&t)
                ));
            }
            root_folder_url(url).await?
        }
    };
    let abs = urlutil::get_absolute_url(&web, &srv);
    let r = Request::post(format!(
        "{web}/_api/SP_CompliancePolicy_SPPolicyStoreProxy_GetListComplianceTag"
    ))
    .spo_json()
    .header("content-type", "application/json;odata=nometadata")
    .json(obj! { "listUrl": abs })
    .data()
    .await?;
    if r.get("odata.null") != Some(&Value::Bool(true)) {
        cx.log(r)?;
    }
    Ok(())
}

pub async fn retentionlabel_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the retention label from list '{}'?",
            label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Clears the retention label from list {} in site at {}...",
        label(cx),
        cx.u("webUrl")
    ));
    let srv = list_server_relative(cx).await?;
    let abs = urlutil::get_absolute_url(&cx.u("webUrl"), &srv);
    set_list_tag(cx, &abs, "", false).await
}

fn rest_base(cx: &Cx) -> String {
    format!("{}/_api/web{}", cx.u("webUrl"), {
        let s = list_seg(cx, "listId", "listTitle", "listUrl");
        if s.is_empty() { s } else { format!("/{s}") }
    })
}

pub async fn sensitivitylabel_ensure(cx: &mut Cx) -> Result<()> {
    let id = match ne(cx, "id") {
        Some(i) => i,
        None => {
            let n = opt(cx, "name");
            cx.verbose_log(&format!("Retrieving sensitivity label id of {n}..."));
            let r = Request::get(format!("https://graph.microsoft.com/beta/security/informationProtection/sensitivityLabels?$filter=name eq '{}'&$select=id", eqp(&n))).graph_json().data().await?;
            let v = need_arr(&r, "value", "0")?;
            match v.first().filter(|x| truthy(x)) {
                Some(l) => opt_val(l, "id"),
                None => return Err(Error::js("The specified sensitivity label does not exist")),
            }
        }
    };
    cx.verbose_log(&format!(
        "Applying a sensitivity label {id} to the document library..."
    ));
    Request::patch(rest_base(cx))
        .spo_json()
        .header("content-type", "application/json;odata=nometadata")
        .header("if-match", "*")
        .json(obj! { "DefaultSensitivityLabelForLibrary": id })
        .send()
        .await?;
    Ok(())
}

fn opt_val(v: &Value, k: &str) -> String {
    v.get(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

pub async fn sensitivitylabel_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the sensitivity label from list '{}'?",
            label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing the sensitivity label from list '{}' in site at {}...",
        label(cx),
        cx.u("webUrl")
    ));
    Request::patch(rest_base(cx))
        .spo_json()
        .header("content-type", "application/json;odata=nometadata")
        .header("if-match", "*")
        .json(obj! { "DefaultSensitivityLabelForLibrary": "" })
        .send()
        .await?;
    Ok(())
}

// ---- role assignments / inheritance -------------------------------------------

fn role_base(cx: &Cx) -> String {
    let s = list_seg(cx, "listId", "listTitle", "listUrl");
    format!(
        "{}/_api/web/{}",
        cx.u("webUrl"),
        if s.is_empty() { s } else { format!("{s}/") }
    )
}

async fn principal(cx: &Cx) -> Result<String> {
    let web = cx.u("webUrl");
    if let Some(upn) = ne(cx, "upn") {
        return Ok(opt_val(&sp::ensure_user(&web, &upn).await?, "Id"));
    }
    if let Some(g) = ne(cx, "groupName") {
        return Ok(opt_val(&sp::group_by_name(cx, &web, &g).await?, "Id"));
    }
    if ne(cx, "entraGroupId").is_some() || ne(cx, "entraGroupName").is_some() {
        cx.verbose_log("Retrieving group information...");
        let g = match ne(cx, "entraGroupId") {
            Some(i) => entra::group::by_id(&i, None).await?,
            None => entra::group::by_display_name(&opt(cx, "entraGroupName"), None).await?,
        };
        return Ok(opt_val(&sp::ensure_entra_group(&web, &g).await?, "Id"));
    }
    Ok(opt(cx, "principalId"))
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!(
        "Adding role assignment to list in site at {web}..."
    ));
    let url = role_base(cx);
    let role = match cx.val("roleDefinitionId").filter(|v| truthy(v)) {
        Some(r) => to_js_string(r),
        None => opt_val(
            &sp::role_definition_by_name(cx, &web, &opt(cx, "roleDefinitionName")).await?,
            "Id",
        ),
    };
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}roleassignments/addroleassignment(principalid='{p}',roledefid='{role}')"
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

pub async fn roleassignment_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove role assignment from the specified user of list '{}'?",
            label(cx)
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Removing role assignment from list '{}' of site {}...",
        label(cx),
        cx.u("webUrl")
    ));
    let url = role_base(cx);
    let p = principal(cx).await?;
    Request::post(format!(
        "{url}roleassignments/removeroleassignment(principalid='{p}')"
    ))
    .spo_json()
    .send()
    .await?;
    Ok(())
}

fn nullish(cx: &Cx) -> String {
    cx.val("listId")
        .or(cx.val("listTitle"))
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

pub async fn roleinheritance_break(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Breaking role inheritance of list in site at {}...",
        cx.u("webUrl")
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to break the role inheritance of {}?",
            nullish(cx)
        ),
    )? {
        return Ok(());
    }
    let keep = !cx.bool("clearExistingPermissions");
    Request::post(format!("{}breakroleinheritance({keep})", role_base(cx)))
        .spo_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn roleinheritance_reset(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Restore role inheritance of list in site at {}...",
        cx.u("webUrl")
    ));
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to reset the role inheritance of {}?",
            nullish(cx)
        ),
    )? {
        return Ok(());
    }
    Request::post(format!("{}resetroleinheritance", role_base(cx)))
        .spo_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    Ok(())
}

// ---- set -----------------------------------------------------------------------

pub fn set_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    if i != 4 {
        return Ok(true);
    }
    const SKIP: [&str; 8] = [
        "webUrl", "id", "title", "url", "output", "query", "debug", "verbose",
    ];
    Ok(o.iter().any(|(k, _)| !SKIP.contains(&k.as_str())))
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.verbose_log(&format!("Updating list in site at {web}..."));
    let mut body = Obj::new();
    if let Some(t) = cx.val("newTitle").filter(|v| truthy(v)) {
        body.insert("Title".into(), t.clone());
    }
    map_props(cx, &mut body, true);
    let url = format!("{web}/_api/web/{}/", list_seg(cx, "id", "title", "url"));
    let url = if url.ends_with("//") {
        url[..url.len() - 1].to_string()
    } else {
        url
    };
    if !body.is_empty() {
        Request::post(url)
            .header("X-HTTP-Method", "MERGE")
            .header("If-Match", "*")
            .spo_json()
            .json(Value::Object(body))
            .send()
            .await?;
    }
    if cx.has("versionExpireAfterDays") || cx.has("versionAutoExpireTrim") {
        let d = sp::digest(&web).await?;
        let mut paths = String::from(
            r#"<StaticProperty Id="0" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" />"#,
        );
        let mut actions = String::from(r#"<ObjectPath Id="1" ObjectPathId="0" />"#);
        paths.push_str(r#"<Property Id="2" ParentId="0" Name="Web" />"#);
        actions.push_str(r#"<ObjectPath Id="3" ObjectPathId="2" />"#);
        let list_oid;
        if let Some(u) = ne(cx, "url") {
            paths.push_str(&format!(r#"<Method Id="4" ParentId="2" Name="GetList"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#, util::escape_xml_value(Some(&Value::from(urlutil::get_server_relative_path(&web, &u))))));
            list_oid = 4;
            actions.push_str(r#"<ObjectPath Id="5" ObjectPathId="4" />"#);
        } else if let Some(i) = ne(cx, "id") {
            paths.push_str(r#"<Property Id="4" ParentId="2" Name="Lists" />"#);
            actions.push_str(r#"<ObjectPath Id="5" ObjectPathId="4" />"#);
            paths.push_str(&format!(r#"<Method Id="6" ParentId="4" Name="GetById"><Parameters><Parameter Type="Guid">{{{}}}</Parameter></Parameters></Method>"#, util::escape_xml(&i)));
            list_oid = 6;
            actions.push_str(r#"<ObjectPath Id="7" ObjectPathId="6" />"#);
        } else {
            let t = cx.val("newTitle").or(cx.val("title"));
            paths.push_str(r#"<Property Id="4" ParentId="2" Name="Lists" />"#);
            actions.push_str(r#"<ObjectPath Id="5" ObjectPathId="4" />"#);
            paths.push_str(&format!(r#"<Method Id="6" ParentId="4" Name="GetByTitle"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#, util::escape_xml_value(t)));
            list_oid = 6;
            actions.push_str(r#"<ObjectPath Id="7" ObjectPathId="6" />"#);
        }
        let vp = list_oid + 10;
        paths.push_str(&format!(
            r#"<Property Id="{vp}" ParentId="{list_oid}" Name="VersionPolicies" />"#
        ));
        actions.push_str(&format!(
            r#"<ObjectPath Id="{}" ObjectPathId="{vp}" />"#,
            vp + 1
        ));
        let mut next = vp + 2;
        if let Some(days) = cx.val("versionExpireAfterDays") {
            actions.push_str(&format!(r#"<SetProperty Id="{next}" ObjectPathId="{vp}" Name="DefaultTrimMode"><Parameter Type="Int32">1</Parameter></SetProperty>"#));
            next += 1;
            actions.push_str(&format!(r#"<SetProperty Id="{next}" ObjectPathId="{vp}" Name="DefaultExpireAfterDays"><Parameter Type="Int32">{}</Parameter></SetProperty>"#, to_js_string(days)));
            next += 1;
        } else if cx.val("versionAutoExpireTrim") == Some(&Value::Bool(true)) {
            actions.push_str(&format!(r#"<SetProperty Id="{next}" ObjectPathId="{vp}" Name="DefaultTrimMode"><Parameter Type="Int32">2</Parameter></SetProperty>"#));
            next += 1;
        } else if cx.val("versionAutoExpireTrim") == Some(&Value::Bool(false)) {
            actions.push_str(&format!(r#"<SetProperty Id="{next}" ObjectPathId="{vp}" Name="DefaultTrimMode"><Parameter Type="Int32">0</Parameter></SetProperty>"#));
            next += 1;
        }
        actions.push_str(&format!(
            r#"<Method Name="Update" Id="{next}" ObjectPathId="{list_oid}" />"#
        ));
        let json = sp::post(&web, &d, sp::request(&actions, &paths)).await?;
        sp::check_first(&json)?;
    }
    Ok(())
}

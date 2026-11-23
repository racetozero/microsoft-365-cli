//! `spo customaction *` and `spo field *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::schema::FieldCheck;
use m365_core::ut::url as urlutil;

// ---- shared custom-action helpers (spo.getCustomActions / getCustomActionById) ----

fn bad_scope(scope: Option<&str>) -> Result<()> {
    match scope {
        Some(s) if !["All", "Site", "Web"].contains(&s) => Err(Error::msg(format!(
            "Invalid scope '{s}'. Allowed values are 'Site', 'Web' or 'All'."
        ))),
        _ => Ok(()),
    }
}

pub async fn custom_actions(
    web: &str,
    scope: Option<&str>,
    filter: Option<&str>,
) -> Result<Vec<Value>> {
    bad_scope(scope)?;
    let q = filter.map(|f| format!("?$filter={f}")).unwrap_or_default();
    match scope {
        Some(s) if s != "All" => {
            odata::get_all_items(&format!("{web}/_api/{s}/UserCustomActions{q}")).await
        }
        _ => {
            let mut v =
                odata::get_all_items(&format!("{web}/_api/Site/UserCustomActions{q}")).await?;
            v.extend(odata::get_all_items(&format!("{web}/_api/Web/UserCustomActions{q}")).await?);
            Ok(v)
        }
    }
}

pub async fn custom_action_by_id(
    web: &str,
    id: &str,
    scope: Option<&str>,
) -> Result<Option<Value>> {
    bad_scope(scope)?;
    async fn by_id(web: &str, id: &str, scope: &str) -> Result<Option<Value>> {
        let r = Request::get(format!("{web}/_api/{scope}/UserCustomActions(guid'{id}')"))
            .spo_json()
            .data()
            .await?;
        Ok(if r.get("odata.null") == Some(&Value::Bool(true)) {
            None
        } else {
            Some(r)
        })
    }
    match scope {
        Some(s) if s != "All" => by_id(web, id, s).await,
        _ => match by_id(web, id, "Web").await? {
            Some(r) => Ok(Some(r)),
            None => by_id(web, id, "Site").await,
        },
    }
}

fn humanize_scope(v: Option<&Value>) -> Value {
    match v.and_then(Value::as_f64) {
        Some(2.0) => Value::from("Site"),
        Some(3.0) => Value::from("Web"),
        _ => Value::from(v.map(to_js_string).unwrap_or_else(|| "undefined".into())),
    }
}

fn summary(a: &Value, scope: Value) -> Value {
    let g = |k: &str| a.get(k).cloned();
    obj! {
        "ClientSideComponentId": ?g("ClientSideComponentId"),
        "ClientSideComponentProperties": ?g("ClientSideComponentProperties"),
        "CommandUIExtension": ?g("CommandUIExtension"),
        "Description": ?g("Description"),
        "Group": ?g("Group"),
        "Id": ?g("Id"),
        "ImageUrl": ?g("ImageUrl"),
        "Location": ?g("Location"),
        "Name": ?g("Name"),
        "RegistrationId": ?g("RegistrationId"),
        "RegistrationType": ?g("RegistrationType"),
        "Rights": ?a.get("Rights").map(|r| Value::from(json::stringify(r))),
        "Scope": scope,
        "ScriptBlock": ?g("ScriptBlock"),
        "ScriptSrc": ?g("ScriptSrc"),
        "Sequence": ?g("Sequence"),
        "Title": ?g("Title"),
        "Url": ?g("Url"),
        "VersionOfUserCustomAction": ?g("VersionOfUserCustomAction"),
    }
}

fn kind_set() -> impl Iterator<Item = &'static str> {
    sp::PERMISSION_KINDS
        .iter()
        .filter(|(n, _)| !n.is_empty())
        .map(|(n, _)| *n)
}

pub fn rights_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    if i != 1 {
        return Ok(());
    }
    let s = to_js_string(v);
    match s
        .split(',')
        .map(str::trim)
        .find(|x| !kind_set().any(|k| k == *x))
    {
        None => Ok(()),
        Some(bad) => Err(Some(format!(
            "Rights option '{bad}' is not recognized as valid PermissionKind choice. Please note it is case sensitive"
        ))),
    }
}

fn registration_type(t: &str) -> i32 {
    match t.to_lowercase().as_str() {
        "list" => 1,
        "contenttype" => 2,
        "progid" => 3,
        "filetype" => 4,
        _ => 0,
    }
}

fn body(cx: &Cx, base: Obj) -> Value {
    let mut b = base;
    for (k, o) in [
        ("Location", "location"),
        ("Name", "name"),
        ("Title", "title"),
        ("Group", "group"),
        ("Description", "description"),
    ] {
        if !b.contains_key(k)
            && let Some(v) = cx.val(o).filter(|v| truthy(v))
        {
            b.insert(k.into(), v.clone());
        }
    }
    if let Some(s) = cx.val("sequence") {
        b.insert("Sequence".into(), s.clone());
    }
    if let Some(t) = cx.s("registrationType").filter(|s| !s.is_empty()) {
        b.insert(
            "RegistrationType".into(),
            Value::from(registration_type(&t)),
        );
    }
    if let Some(r) = cx.val("registrationId").filter(|v| truthy(v)) {
        b.insert("RegistrationId".into(), Value::from(to_js_string(r)));
    }
    for (k, o) in [
        ("Url", "actionUrl"),
        ("ImageUrl", "imageUrl"),
        ("ClientSideComponentId", "clientSideComponentId"),
        (
            "ClientSideComponentProperties",
            "clientSideComponentProperties",
        ),
        ("ScriptBlock", "scriptBlock"),
        ("ScriptSrc", "scriptSrc"),
    ] {
        if let Some(v) = cx.val(o).filter(|v| truthy(v)) {
            b.insert(k.into(), v.clone());
        }
    }
    if let Some(c) = cx.val("commandUIExtension").filter(|v| truthy(v)) {
        b.insert("CommandUIExtension".into(), Value::from(to_js_string(c)));
    }
    if let Some(r) = cx.s("rights").filter(|s| !s.is_empty()) {
        let mut p = sp::BasePermissions::default();
        for item in r.split(',') {
            if let Some((_, k)) = sp::PERMISSION_KINDS
                .iter()
                .find(|(n, _)| !n.is_empty() && *n == item.trim())
            {
                p.set(*k);
            }
        }
        b.insert(
            "Rights".into(),
            obj! { "High": p.high.to_string(), "Low": p.low.to_string() },
        );
    }
    Value::Object(b)
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let scope = cx
        .s("scope")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Web".into());
    let mut base = Obj::new();
    base.insert(
        "Title".into(),
        cx.val("title").cloned().unwrap_or(Value::Null),
    );
    base.insert(
        "Name".into(),
        cx.val("name").cloned().unwrap_or(Value::Null),
    );
    base.insert(
        "Location".into(),
        cx.val("location").cloned().unwrap_or(Value::Null),
    );
    let data = body(cx, base);
    let r = Request::post(format!("{}/_api/{scope}/UserCustomActions", cx.u("webUrl")))
        .spo_json()
        .json(data)
        .data()
        .await?;
    if cx.verbose() {
        m365_core::log::log_stderr_value(&summary(&r, Value::from(scope)));
    }
    Ok(())
}

pub async fn clear(cx: &mut Cx) -> Result<()> {
    let scope = cx.s("scope").filter(|s| !s.is_empty());
    let shown = scope.clone().unwrap_or_else(|| "All".into());
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to clear all the user custom actions with scope {}?",
            m365_core::term::yellow(&shown)
        ),
    )? {
        return Ok(());
    }
    let web = cx.u("webUrl");
    let scopes: Vec<String> = match scope {
        Some(s) if s.to_lowercase() != "all" => vec![s],
        _ => vec!["Web".into(), "Site".into()],
    };
    for s in scopes {
        Request::post(format!("{web}/_api/{s}/UserCustomActions/clear"))
            .spo_json()
            .send()
            .await?;
    }
    Ok(())
}

async fn pick(cx: &Cx, list: Vec<Value>, none: String, many: String) -> Result<Value> {
    match list.len() {
        0 => Err(Error::msg(none)),
        1 => Ok(list.into_iter().next().unwrap()),
        _ => cx.handle_multiple_results(&many, &util::array_to_hash_table("Id", &list)),
    }
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let scope = cx.s("scope");
    let a = if let Some(id) = cx.s("id").filter(|s| !s.is_empty()) {
        match custom_action_by_id(&web, &id, scope.as_deref()).await? {
            Some(a) => a,
            None => {
                return Err(Error::msg(format!(
                    "No user custom action with id '{id}' found"
                )));
            }
        }
    } else if let Some(t) = cx.s("title").filter(|s| !s.is_empty()) {
        let l = custom_actions(
            &web,
            scope.as_deref(),
            Some(&format!("Title eq '{}'", eqp(&t))),
        )
        .await?;
        pick(
            cx,
            l,
            format!("No user custom action with title '{t}' found"),
            format!("Multiple user custom actions with title '{t}' found."),
        )
        .await?
    } else {
        let c = cx.u("clientSideComponentId");
        let l = custom_actions(
            &web,
            scope.as_deref(),
            Some(&format!("ClientSideComponentId eq guid'{c}'")),
        )
        .await?;
        pick(
            cx,
            l,
            format!("No user custom action with ClientSideComponentId '{c}' found"),
            format!("Multiple user custom actions with ClientSideComponentId '{c}' found."),
        )
        .await?
    };
    if truthy(&a) {
        cx.log(summary(&a, humanize_scope(a.get("Scope"))))?;
    }
    Ok(())
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let scope = cx
        .s("scope")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "All".into());
    if cx.debug() {
        cx.stderr(&format!(
            "Attempt to get custom actions list with scope: {scope}"
        ));
        cx.stderr("");
    }
    let mut l = custom_actions(&cx.u("webUrl"), cx.s("scope").as_deref(), None).await?;
    if cx.str("output").as_deref() != Some("json") {
        for a in l.iter_mut() {
            let s = humanize_scope(a.get("Scope"));
            if let Some(o) = a.as_object_mut() {
                o.insert("Scope".into(), s);
            }
        }
    }
    cx.log(Value::Array(l))
}

async fn remove_scoped(cx: &Cx, scope: &str) -> Result<Value> {
    let web = cx.u("webUrl");
    let id = match cx.s("id").filter(|s| !s.is_empty()) {
        Some(i) => i,
        None => {
            let t = cx.u("title");
            let l =
                custom_actions(&web, Some(scope), Some(&format!("Title eq '{}'", eqp(&t)))).await?;
            u(
                &pick(
                    cx,
                    l,
                    format!("No user custom action with title '{t}' found"),
                    format!("Multiple user custom actions with title '{t}' found."),
                )
                .await?,
                "Id",
            )
        }
    };
    Request::post(format!(
        "{web}/_api/{scope}/UserCustomActions('{}')')",
        eqp(&id)
    ))
    .spo_json()
    .header("X-HTTP-Method", "DELETE")
    .data()
    .await
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the {} user custom action?",
            cx.s("id").unwrap_or_else(|| "undefined".into())
        ),
    )? {
        return Ok(());
    }
    let r = match cx
        .s("scope")
        .filter(|s| !s.is_empty() && s.to_lowercase() != "all")
    {
        Some(s) => remove_scoped(cx, &s).await?,
        None => {
            let w = remove_scoped(cx, "Web").await?;
            if !truthy(&w) {
                w
            } else {
                remove_scoped(cx, "Site").await?
            }
        }
    };
    if cx.verbose() && r.get("odata.null") == Some(&Value::Bool(true)) {
        cx.stderr(&format!(
            "Custom action with id {} not found",
            cx.s("id").unwrap_or_else(|| "undefined".into())
        ));
    }
    Ok(())
}

async fn update(cx: &Cx, scope: &str) -> Result<Value> {
    Request::post(format!(
        "{}/_api/{scope}/UserCustomActions('{}')",
        cx.u("webUrl"),
        eqp(&cx.u("id"))
    ))
    .spo_json()
    .header("X-HTTP-Method", "MERGE")
    .json(body(cx, Obj::new()))
    .data()
    .await
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let scope = cx
        .s("scope")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "All".into());
    let r = if scope.to_lowercase() != "all" {
        update(cx, &scope).await?
    } else {
        let w = update(cx, "Web").await?;
        if w.get("odata.null") != Some(&Value::Bool(true)) {
            w
        } else {
            update(cx, "Site").await?
        }
    };
    if cx.verbose() && r.get("odata.null") == Some(&Value::Bool(true)) {
        cx.stderr(&format!("Custom action with id {} not found", cx.u("id")));
    }
    Ok(())
}

// ---- field ---------------------------------------------------------------------

const FIELD_OPTIONS: [(&str, i64); 7] = [
    ("DefaultValue", 0),
    ("AddToDefaultContentType", 1),
    ("AddToNoContentType", 2),
    ("AddToAllContentTypes", 4),
    ("AddFieldInternalNameHint", 8),
    ("AddFieldToDefaultView", 16),
    ("AddFieldCheckDisplayName", 32),
];

pub fn field_add_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    if i != 0 {
        return Ok(());
    }
    let s = to_js_string(v);
    match s
        .split(',')
        .map(str::trim)
        .find(|x| !FIELD_OPTIONS.iter().any(|(n, _)| n == x))
    {
        None => Ok(()),
        Some(bad) => Err(Some(format!(
            "{bad} is not a valid value for the options argument. Allowed values are {}",
            FIELD_OPTIONS
                .iter()
                .map(|(n, _)| *n)
                .collect::<Vec<_>>()
                .join("|")
        ))),
    }
}

fn list_rest(cx: &Cx, web: &str) -> String {
    if let Some(i) = cx.s("listId").filter(|s| !s.is_empty()) {
        format!("lists(guid'{}')/", eqp(&i))
    } else if let Some(t) = cx.s("listTitle").filter(|s| !s.is_empty()) {
        format!("lists/getByTitle('{}')/", eqp(&t))
    } else if let Some(l) = cx.s("listUrl").filter(|s| !s.is_empty()) {
        format!(
            "GetList('{}')/",
            eqp(&urlutil::get_server_relative_path(web, &l))
        )
    } else {
        String::new()
    }
}

pub async fn field_add(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let lr = list_rest(cx, &web);
    let url = format!(
        "{web}/_api/web{}/fields/CreateFieldAsXml",
        if lr.is_empty() {
            String::new()
        } else {
            format!("/{}", &lr[..lr.len() - 1])
        }
    );
    let d = sp::digest(&web).await?;
    let opts = cx
        .s("options")
        .filter(|s| !s.is_empty())
        .map(|o| {
            o.split(',')
                .map(|x| {
                    FIELD_OPTIONS
                        .iter()
                        .find(|(n, _)| *n == x.trim())
                        .map_or(0, |(_, v)| *v)
                })
                .sum::<i64>()
        })
        .unwrap_or(0);
    let data =
        obj! { "parameters": obj! { "SchemaXml": ?cx.val("xml").cloned(), "Options": opts } };
    cx.log(
        Request::post(url)
            .header("X-RequestDigest", d)
            .spo_json()
            .json(data)
            .data()
            .await?,
    )
}

fn field_rest(cx: &Cx) -> String {
    match cx.s("id").filter(|s| !s.is_empty()) {
        Some(i) => format!("/getbyid('{}')", eqp(&i)),
        None => format!(
            "/getbyinternalnameortitle('{}')",
            eqp(&cx
                .s("title")
                .filter(|s| !s.is_empty())
                .or_else(|| cx.s("internalName"))
                .unwrap_or_else(|| "undefined".into()))
        ),
    }
}

pub async fn field_get(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let url = format!(
        "{web}/_api/web/{}fields{}",
        list_rest(cx, &web),
        field_rest(cx)
    );
    cx.log(Request::get(url).spo_json().data().await?)
}

pub async fn field_list(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    cx.log(Value::Array(
        odata::get_all_items(&format!("{web}/_api/web/{}fields", list_rest(cx, &web))).await?,
    ))
}

pub async fn field_remove(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let end = match cx
        .s("listId")
        .filter(|s| !s.is_empty())
        .or_else(|| cx.s("listTitle").filter(|s| !s.is_empty()))
    {
        Some(l) => format!("in list {l}"),
        None => format!("in site {web}"),
    };
    let group = cx.s("group");
    if !cx.bool("force") {
        let what = ["id", "title", "internalName"]
            .iter()
            .find_map(|k| cx.s(k).filter(|s| !s.is_empty()))
            .unwrap_or_else(|| {
                format!(
                    "from group {}",
                    group.clone().unwrap_or_else(|| "undefined".into())
                )
            });
        let msg = format!(
            "Are you sure you want to remove the {} {what} {end}?",
            if group.as_deref().is_some_and(|g| !g.is_empty()) {
                "fields"
            } else {
                "field"
            }
        );
        if !cx.confirm(&msg)? {
            return Ok(());
        }
    }
    let lr = list_rest(cx, &web);
    let remove = |id: Option<String>| {
        let label = id
            .clone()
            .or_else(|| {
                ["title", "internalName"]
                    .iter()
                    .find_map(|k| cx.s(k).filter(|s| !s.is_empty()))
            })
            .unwrap_or_else(|| "undefined".into());
        cx.verbose_log(&format!("Removing field {label} {end}..."));
        let fr = match &id {
            Some(i) => format!("/getbyid('{}')", eqp(i)),
            None => format!(
                "/getbyinternalnameortitle('{}')",
                eqp(&cx
                    .s("title")
                    .filter(|s| !s.is_empty())
                    .or_else(|| cx.s("internalName"))
                    .unwrap_or_else(|| "undefined".into()))
            ),
        };
        Request::post(format!("{web}/_api/web/{lr}fields{fr}"))
            .header("X-HTTP-Method", "DELETE")
            .header("If-Match", "*")
            .spo_json()
            .send()
    };
    if let Some(g) = group.filter(|s| !s.is_empty()) {
        cx.verbose_log(&format!("Retrieving fields assigned to group {g}..."));
        let r = Request::get(format!("{web}/_api/web/{lr}fields"))
            .spo_json()
            .data()
            .await?;
        let matches: Vec<Value> = need_arr(&r, "value", "filter")?
            .iter()
            .filter(|f| f.get("Group").and_then(Value::as_str) == Some(g.as_str()))
            .cloned()
            .collect();
        cx.verbose_log(&format!("{} matches found...", matches.len()));
        for f in &matches {
            remove(Some(
                f.get("Id")
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into()),
            ))
            .await?;
        }
    } else {
        remove(cx.s("id").filter(|s| !s.is_empty())).await?;
    }
    Ok(())
}

pub async fn field_set(cx: &mut Cx) -> Result<()> {
    let web = cx.u("webUrl");
    let d = sp::digest(&web).await?;
    let mut parent = r#"<Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}" Name="Current" />"#.to_string();
    let ex = |k: &str| util::escape_xml_value(cx.val(k));
    let list_body = if cx.s("listId").filter(|s| !s.is_empty()).is_some() {
        Some(sp::request(
            r#"<ObjectPath Id="664" ObjectPathId="663" /><Query Id="665" ObjectPathId="663"><Query SelectAllProperties="false"><Properties /></Query></Query>"#,
            &format!(
                r#"<Method Id="663" ParentId="7" Name="GetById"><Parameters><Parameter Type="Guid">{}</Parameter></Parameters></Method><Property Id="7" ParentId="5" Name="Lists" /><Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}}" Name="Current" />"#,
                ex("listId")
            ),
        ))
    } else if cx.s("listTitle").filter(|s| !s.is_empty()).is_some() {
        Some(sp::request(
            r#"<ObjectPath Id="664" ObjectPathId="663" /><Query Id="665" ObjectPathId="663"><Query SelectAllProperties="false"><Properties /></Query></Query>"#,
            &format!(
                r#"<Method Id="663" ParentId="7" Name="GetByTitle"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Property Id="7" ParentId="5" Name="Lists" /><Property Id="5" ParentId="3" Name="Web" /><StaticProperty Id="3" TypeId="{{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}}" Name="Current" />"#,
                ex("listTitle")
            ),
        ))
    } else {
        cx.s("listUrl").filter(|s| !s.is_empty()).map(|l| sp::request(
            r#"<ObjectPath Id="2" ObjectPathId="1" /><ObjectPath Id="4" ObjectPathId="3" /><ObjectPath Id="6" ObjectPathId="5" /><Query Id="7" ObjectPathId="5"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(r#"<StaticProperty Id="1" TypeId="{{3747adcd-a3c3-41b9-bfab-4a64dd2f1e0a}}" Name="Current" /><Property Id="3" ParentId="1" Name="Web" /><Method Id="5" ParentId="3" Name="GetList"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#, urlutil::get_server_relative_path(&web, &l)),
        ))
    };
    let ident = |json: &[Value]| match json.last() {
        Some(Value::Null) => Err(Error::js(
            "Cannot read properties of null (reading '_ObjectIdentity_')",
        )),
        Some(v) => Ok(v
            .get("_ObjectIdentity_")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())),
        None => Err(Error::js(
            "Cannot read properties of undefined (reading '_ObjectIdentity_')",
        )),
    };
    if let Some(b) = list_body {
        let json = sp::post(&web, &d, b).await?;
        sp::check_first(&json)?;
        parent = format!(r#"<Identity Id="5" Name="{}" />"#, ident(&json)?);
    }
    let fq = match cx.s("id").filter(|s| !s.is_empty()) {
        Some(_) => format!(
            r#"<Method Id="663" ParentId="7" Name="GetById"><Parameters><Parameter Type="Guid">{}</Parameter></Parameters></Method>"#,
            ex("id")
        ),
        None => {
            let v = cx
                .val("title")
                .filter(|v| truthy(v))
                .or(cx.val("internalName"));
            format!(
                r#"<Method Id="663" ParentId="7" Name="GetByInternalNameOrTitle"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                util::escape_xml_value(v)
            )
        }
    };
    let json = sp::post(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="664" ObjectPathId="663" /><Query Id="665" ObjectPathId="663"><Query SelectAllProperties="false"><Properties /></Query></Query>"#,
            &format!(r#"{fq}<Property Id="7" ParentId="5" Name="Fields" />{parent}"#),
        ),
    )
    .await?;
    sp::check_first(&json)?;
    let field_id = ident(&json)?;
    const EXCLUDE: [&str; 11] = [
        "webUrl",
        "listId",
        "listTitle",
        "listUrl",
        "id",
        "title",
        "internalName",
        "updateExistingLists",
        "debug",
        "verbose",
        "output",
    ];
    let mut i = 667;
    let mut payload = String::new();
    for (k, v) in &cx.opts {
        if EXCLUDE.contains(&k.as_str()) {
            continue;
        }
        payload.push_str(&format!(r#"<SetProperty Id="{i}" ObjectPathId="663" Name="{k}"><Parameter Type="String">{}</Parameter></SetProperty>"#, util::escape_xml_value(Some(v))));
        i += 1;
    }
    let json = sp::post(
        &web,
        &d,
        sp::request(
            &format!(r#"{payload}<Method Name="UpdateAndPushChanges" Id="9000" ObjectPathId="663"><Parameters><Parameter Type="Boolean">{}</Parameter></Parameters></Method>"#, cx.bool("updateExistingLists")),
            &format!(r#"<Identity Id="663" Name="{field_id}" />"#),
        ),
    )
    .await?;
    sp::check_first(&json)
}

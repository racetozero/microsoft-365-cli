//! `spo term *` and `spo sitescript *`.

use crate::sp;
use m365_core::prelude::*;
use m365_core::ut::spo;

const SESSION: &str = "{981cbc68-9edc-4f8d-872f-71146fcbb84f}";

fn undef(what: &str) -> Error {
    Error::js(format!(
        "Cannot read properties of undefined (reading '{what}')"
    ))
}

/// `formatting.escapeXml(opt)` in a template literal.
fn ex(cx: &Cx, k: &str) -> String {
    util::escape_xml_value(cx.val(k))
}

/// `${v}` for a property that may be missing.
fn prop(v: &Value, k: &str) -> String {
    v.get(k)
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into())
}

fn strip<'a>(v: Option<&'a Value>, what: &str) -> Result<&'a str> {
    match v {
        Some(Value::String(s)) => Ok(s),
        Some(Value::Null) => Err(Error::js(format!(
            "Cannot read properties of null (reading '{what}')"
        ))),
        Some(_) => Err(Error::js(format!("{what} is not a function"))),
        None => Err(undef(what)),
    }
}

/// `new Date(Number(s.replace('/Date(', '').replace(')/', ''))).toISOString()`.
fn csom_date(v: Option<&Value>) -> Result<Value> {
    let s = strip(v, "replace")?
        .replacen("/Date(", "", 1)
        .replacen(")/", "", 1);
    let n = json::js_number(&s);
    if !n.is_finite() || n.abs() > 8.64e15 {
        return Err(Error::js("Invalid time value"));
    }
    let d = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(n.trunc() as i64)
        .ok_or_else(|| Error::js("Invalid time value"))?;
    Ok(Value::from(m365_core::time::to_iso(d)))
}

fn csom_id(v: Option<&Value>) -> Result<Value> {
    Ok(Value::from(
        strip(v, "replace")?
            .replacen("/Guid(", "", 1)
            .replacen(")/", "", 1),
    ))
}

/// Deletes `_ObjectIdentity_`/`_ObjectType_` and converts the dates and id.
fn fix(term: &mut Value, delete: bool) -> Result<()> {
    if !term.is_object() {
        if term.is_null() {
            return Err(Error::js(
                "Cannot read properties of null (reading 'replace')",
            ));
        }
        return Err(undef("replace"));
    }
    let o = term.as_object_mut().unwrap();
    if delete {
        o.shift_remove("_ObjectIdentity_");
        o.shift_remove("_ObjectType_");
    }
    let c = csom_date(o.get("CreatedDate"))?;
    o.insert("CreatedDate".into(), c);
    let id = csom_id(o.get("Id"))?;
    o.insert("Id".into(), id);
    let l = csom_date(o.get("LastModifiedDate"))?;
    o.insert("LastModifiedDate".into(), l);
    Ok(())
}

async fn web_and_digest(cx: &Cx) -> Result<(String, String)> {
    let web = match cx.s("webUrl").filter(|w| !w.is_empty()) {
        Some(w) => w,
        None => spo::get_spo_admin_url(cx.debug()).await?,
    };
    let d = sp::digest(&web).await?;
    Ok((web, d))
}

async fn last(web: &str, d: &str, body: String) -> Result<Value> {
    let json = sp::post(web, d, body).await?;
    sp::check_first(&json)?;
    Ok(json.last().cloned().unwrap_or(Value::Null))
}

fn child_items(v: &Value) -> Result<&Vec<Value>> {
    match v.get("_Child_Items_") {
        Some(Value::Array(a)) => Ok(a),
        _ if v.is_null() => Err(Error::js(
            "Cannot read properties of null (reading '_Child_Items_')",
        )),
        _ => Err(undef("forEach")),
    }
}

fn custom_props(raw: &str, method: &str, i: &mut u32, out: &mut Vec<String>) -> Result<Value> {
    let v = json::jparse(raw)?;
    if let Some(o) = v.as_object() {
        for (k, val) in o {
            out.push(format!(
                r#"<Method Name="{method}" Id="{i}" ObjectPathId="117"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
                util::escape_xml(k),
                util::escape_xml_value(Some(val))
            ));
            *i += 1;
        }
    }
    Ok(v)
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Adding taxonomy term...");
    let group_q = match cx.s("termGroupId").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="11" ParentId="9" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="11" ParentId="9" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termGroupName")
        ),
    };
    let parent_q = if let Some(p) = cx.s("parentTermId").filter(|s| !s.is_empty()) {
        format!(
            r#"<Method Id="16" ParentId="6" Name="GetTerm"><Parameters><Parameter Type="Guid">{{{p}}}</Parameter></Parameters></Method>"#
        )
    } else if let Some(s) = cx.s("termSetId").filter(|s| !s.is_empty()) {
        format!(
            r#"<Method Id="16" ParentId="14" Name="GetById"><Parameters><Parameter Type="Guid">{{{s}}}</Parameter></Parameters></Method>"#
        )
    } else {
        format!(
            r#"<Method Id="16" ParentId="14" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termSetName")
        )
    };
    let id = cx
        .s("id")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(util::new_guid);
    let body = sp::request(
        r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectIdentityQuery Id="5" ObjectPathId="3" /><ObjectPath Id="7" ObjectPathId="6" /><ObjectIdentityQuery Id="8" ObjectPathId="6" /><ObjectPath Id="10" ObjectPathId="9" /><ObjectPath Id="12" ObjectPathId="11" /><ObjectIdentityQuery Id="13" ObjectPathId="11" /><ObjectPath Id="15" ObjectPathId="14" /><ObjectPath Id="17" ObjectPathId="16" /><ObjectIdentityQuery Id="18" ObjectPathId="16" /><ObjectPath Id="20" ObjectPathId="19" /><ObjectIdentityQuery Id="21" ObjectPathId="19" /><Query Id="22" ObjectPathId="19"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        &format!(
            r#"<StaticMethod Id="3" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="6" ParentId="3" Name="GetDefaultSiteCollectionTermStore" /><Property Id="9" ParentId="6" Name="Groups" />{group_q}<Property Id="14" ParentId="11" Name="TermSets" />{parent_q}<Method Id="19" ParentId="16" Name="CreateTerm"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Int32">1033</Parameter><Parameter Type="Guid">{{{id}}}</Parameter></Parameters></Method>"#,
            ex(cx, "name")
        ),
    );
    let json = sp::post(&web, &d, body).await?;
    sp::check_first(&json)?;
    let mut term = json.last().cloned().unwrap_or(Value::Null);
    let (desc, cp, lcp) = (
        cx.s("description").filter(|s| !s.is_empty()),
        cx.s("customProperties").filter(|s| !s.is_empty()),
        cx.s("localCustomProperties").filter(|s| !s.is_empty()),
    );
    if desc.is_some() || cp.is_some() || lcp.is_some() {
        cx.verbose_log("Setting term properties...");
        let mut props = Vec::new();
        let mut i = 127;
        let set = |t: &mut Value, k: &str, v: Value| -> Result<()> {
            match t.as_object_mut() {
                Some(o) => {
                    o.insert(k.into(), v);
                    Ok(())
                }
                None => Err(Error::js(format!(
                    "Cannot set properties of {} (setting '{k}')",
                    if t.is_null() { "null" } else { "undefined" }
                ))),
            }
        };
        if let Some(dsc) = &desc {
            props.push(format!(r#"<Method Name="SetDescription" Id="{i}" ObjectPathId="117"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Int32">1033</Parameter></Parameters></Method>"#, util::escape_xml(dsc)));
            i += 1;
            set(&mut term, "Description", Value::from(dsc.as_str()))?;
        }
        if let Some(c) = &cp {
            let v = custom_props(c, "SetCustomProperty", &mut i, &mut props)?;
            set(&mut term, "CustomProperties", v)?;
        }
        if let Some(c) = &lcp {
            let v = custom_props(c, "SetLocalCustomProperty", &mut i, &mut props)?;
            set(&mut term, "LocalCustomProperties", v)?;
        }
        let store = match json.iter().position(|x| x.as_f64() == Some(8.0)) {
            Some(p) => match json.get(p + 1) {
                Some(n) => prop(n, "_ObjectIdentity_"),
                None => return Err(undef("_ObjectIdentity_")),
            },
            None => String::new(),
        };
        let body = sp::request(
            &format!(
                r#"{}<Method Name="CommitAll" Id="131" ObjectPathId="109" />"#,
                props.join("")
            ),
            &format!(
                r#"<Identity Id="117" Name="{}" /><Identity Id="109" Name="{store}" />"#,
                prop(&term, "_ObjectIdentity_")
            ),
        );
        let r = m365_core::ut::spo::process_query(&web, &d, body).await?;
        if !r.is_empty() {
            sp::check_first(&sp::parse(&r)?)?;
        }
    }
    fix(&mut term, true)?;
    cx.log(term)
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term...");
    let body = if let Some(id) = cx.s("id").filter(|s| !s.is_empty()) {
        sp::request(
            r#"<ObjectPath Id="14" ObjectPathId="13" /><ObjectIdentityQuery Id="15" ObjectPathId="13" /><Query Id="16" ObjectPathId="13"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(
                r#"<StaticMethod Id="6" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="7" ParentId="6" Name="GetDefaultSiteCollectionTermStore" /><Method Id="13" ParentId="7" Name="GetTerm"><Parameters><Parameter Type="Guid">{{{id}}}</Parameter></Parameters></Method>"#
            ),
        )
    } else {
        let or = |name: &str, id: &str| {
            let e = ex(cx, name);
            if cx.val(name).is_some_and(truthy) {
                e
            } else {
                cx.val(id)
                    .filter(|v| truthy(v))
                    .map(to_js_string)
                    .unwrap_or(if cx.val(name).is_some() {
                        e
                    } else {
                        prop(&Value::Object(cx.opts.clone()), id)
                    })
            }
        };
        let gm = if cx.val("termGroupName").is_none() {
            "GetById"
        } else {
            "GetByName"
        };
        let sm = if cx.val("termSetName").is_none() {
            "GetById"
        } else {
            "GetByName"
        };
        sp::request(
            &format!(
                r#"<ObjectPath Id="2" ObjectPathId="1" /><ObjectIdentityQuery Id="3" ObjectPathId="1" /><ObjectPath Id="5" ObjectPathId="4" /><ObjectIdentityQuery Id="6" ObjectPathId="4" /><ObjectPath Id="8" ObjectPathId="7" /><ObjectPath Id="10" ObjectPathId="9" /><ObjectIdentityQuery Id="11" ObjectPathId="9" /><ObjectPath Id="13" ObjectPathId="12" /><ObjectPath Id="15" ObjectPathId="14" /><ObjectIdentityQuery Id="16" ObjectPathId="14" /><ObjectPath Id="18" ObjectPathId="17" /><SetProperty Id="19" ObjectPathId="17" Name="TrimUnavailable"><Parameter Type="Boolean">true</Parameter></SetProperty><SetProperty Id="20" ObjectPathId="17" Name="TermLabel"><Parameter Type="String">{}</Parameter></SetProperty><ObjectPath Id="22" ObjectPathId="21" /><Query Id="23" ObjectPathId="21"><Query SelectAllProperties="true"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties /></ChildItemQuery></Query>"#,
                ex(cx, "name")
            ),
            &format!(
                r#"<StaticMethod Id="1" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="4" ParentId="1" Name="GetDefaultSiteCollectionTermStore" /><Property Id="7" ParentId="4" Name="Groups" /><Method Id="9" ParentId="7" Name="{gm}"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Property Id="12" ParentId="9" Name="TermSets" /><Method Id="14" ParentId="12" Name="{sm}"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method><Constructor Id="17" TypeId="{{61a1d689-2744-4ea3-a88b-c95bee9803aa}}" /><Method Id="21" ParentId="14" Name="GetTerms"><Parameters><Parameter ObjectPathId="17" /></Parameters></Method>"#,
                or("termGroupName", "termGroupId"),
                or("termSetName", "termSetId")
            ),
        )
    };
    let r = last(&web, &d, body).await?;
    let mut term = if r.is_null() {
        return Err(Error::msg(format!(
            "Term with id '{}' could not be found.",
            prop(&Value::Object(cx.opts.clone()), "id")
        )));
    } else if let Some(items) = r.get("_Child_Items_") {
        let items = match items {
            Value::Array(a) => a,
            _ => return Err(undef("length")),
        };
        if items.is_empty() {
            return Err(Error::msg(format!(
                "Term with name '{}' could not be found.",
                prop(&Value::Object(cx.opts.clone()), "name")
            )));
        }
        if items.len() > 1 {
            cx.handle_multiple_results(
                "Multiple terms with the specific term name found.",
                &util::array_to_hash_table("Id", items),
            )?
        } else {
            items[0].clone()
        }
    } else {
        r
    };
    fix(&mut term, true)?;
    cx.log(term)
}

pub async fn group_add(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Getting taxonomy term store...");
    let store = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectIdentityQuery Id="5" ObjectPathId="3" /><ObjectPath Id="7" ObjectPathId="6" /><ObjectIdentityQuery Id="8" ObjectPathId="6" /><Query Id="9" ObjectPathId="6"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
            &format!(r#"<StaticMethod Id="3" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="6" ParentId="3" Name="GetDefaultSiteCollectionTermStore" />"#),
        ),
    )
    .await?;
    let id = cx
        .s("id")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(util::new_guid);
    cx.verbose_log("Adding taxonomy term group...");
    if store.is_null() {
        return Err(Error::js(
            "Cannot read properties of null (reading '_ObjectIdentity_')",
        ));
    }
    let mut group = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="14" ObjectPathId="13" /><ObjectIdentityQuery Id="15" ObjectPathId="13" /><Query Id="16" ObjectPathId="13"><Query SelectAllProperties="false"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /><Property Name="Description" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(
                r#"<Method Id="13" ParentId="6" Name="CreateGroup"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Guid">{{{id}}}</Parameter></Parameters></Method><Identity Id="6" Name="{}" />"#,
                ex(cx, "name"),
                prop(&store, "_ObjectIdentity_")
            ),
        ),
    )
    .await?;
    if let Some(desc) = cx.s("description").filter(|s| !s.is_empty()) {
        cx.verbose_log("Setting taxonomy term group description...");
        if group.is_null() {
            return Err(Error::js(
                "Cannot read properties of null (reading '_ObjectIdentity_')",
            ));
        }
        let body = sp::request(
            &format!(
                r#"<SetProperty Id="51" ObjectPathId="45" Name="Description"><Parameter Type="String">{}</Parameter></SetProperty>"#,
                util::escape_xml(&desc)
            ),
            &format!(
                r#"<Identity Id="45" Name="{}" />"#,
                prop(&group, "_ObjectIdentity_")
            ),
        );
        let r = m365_core::ut::spo::process_query(&web, &d, body).await?;
        if !r.is_empty() {
            sp::check_first(&sp::parse(&r)?)?;
        }
    }
    let Some(o) = group.as_object_mut() else {
        return Err(Error::js(
            "Cannot convert undefined or null to object".to_string(),
        ));
    };
    o.shift_remove("_ObjectIdentity_");
    o.shift_remove("_ObjectType_");
    let id = csom_id(o.get("Id"))?;
    o.insert("Id".into(), id);
    o.insert(
        "Description".into(),
        Value::from(cx.s("description").unwrap_or_default()),
    );
    cx.log(group)
}

pub async fn group_get(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term groups...");
    let q = match cx.s("id").filter(|s| !s.is_empty()) {
        Some(id) => format!(
            r#"<Method Id="32" ParentId="30" Name="GetById"><Parameters><Parameter Type="Guid">{{{}}}</Parameter></Parameters></Method>"#,
            util::escape_xml(&id)
        ),
        None => format!(
            r#"<Method Id="32" ParentId="30" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "name")
        ),
    };
    let mut g = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="25" ObjectPathId="24" /><ObjectIdentityQuery Id="26" ObjectPathId="24" /><ObjectPath Id="28" ObjectPathId="27" /><ObjectIdentityQuery Id="29" ObjectPathId="27" /><ObjectPath Id="31" ObjectPathId="30" /><ObjectPath Id="33" ObjectPathId="32" /><ObjectIdentityQuery Id="34" ObjectPathId="32" /><Query Id="35" ObjectPathId="32"><Query SelectAllProperties="true"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(r#"<StaticMethod Id="24" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="27" ParentId="24" Name="GetDefaultSiteCollectionTermStore" /><Property Id="30" ParentId="27" Name="Groups" />{q}"#),
        ),
    )
    .await?;
    fix(&mut g, true)?;
    cx.log(g)
}

/// The `_Child_Items_` of a collection, with dates and ids converted.
fn fix_children(r: &Value) -> Result<Vec<Value>> {
    let mut items = child_items(r)?.clone();
    for t in items.iter_mut() {
        let o = t.as_object_mut().ok_or_else(|| undef("replace"))?;
        let c = csom_date(o.get("CreatedDate"))?;
        o.insert("CreatedDate".into(), c);
        let id = match o.get("Id") {
            Some(Value::String(s)) => util::extract_csom_guid(s),
            _ => return Err(undef("replace")),
        };
        o.insert("Id".into(), Value::from(id));
        let l = csom_date(o.get("LastModifiedDate"))?;
        o.insert("LastModifiedDate".into(), l);
    }
    Ok(items)
}

pub async fn group_list(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term groups...");
    let r = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="4" ObjectPathId="3" /><ObjectIdentityQuery Id="5" ObjectPathId="3" /><ObjectPath Id="7" ObjectPathId="6" /><ObjectIdentityQuery Id="8" ObjectPathId="6" /><ObjectPath Id="10" ObjectPathId="9" /><Query Id="11" ObjectPathId="9"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /></Properties></ChildItemQuery></Query>"#,
            &format!(r#"<StaticMethod Id="3" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="6" ParentId="3" Name="GetDefaultSiteCollectionTermStore" /><Property Id="9" ParentId="6" Name="Groups" />"#),
        ),
    )
    .await?;
    cx.log(Value::Array(fix_children(&r)?))
}

fn set_term_details(t: &mut Value) -> Result<()> {
    fix(t, false)
}

fn child_terms<'a>(
    web: &'a str,
    d: &'a str,
    parent: &'a mut Value,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(child_terms_impl(web, d, parent))
}

async fn child_terms_impl(web: &str, d: &str, parent: &mut Value) -> Result<()> {
    let ident = prop(parent, "_ObjectIdentity_");
    let body = sp::request(
        r#"<ObjectPath Id="20" ObjectPathId="19" /><Query Id="21" ObjectPathId="19"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties><Property Name="CustomSortOrder" ScalarProperty="true" /><Property Name="CustomProperties" ScalarProperty="true" /><Property Name="LocalCustomProperties" ScalarProperty="true" /></Properties></ChildItemQuery></Query>"#,
        &format!(
            r#"<Property Id="19" ParentId="16" Name="Terms" /><Identity Id="16" Name="{ident}" />"#
        ),
    );
    let r = last(web, d, body).await?;
    let mut children = Vec::new();
    if let Some(Value::Array(items)) = r.get("_Child_Items_") {
        for mut t in items.clone() {
            set_term_details(&mut t)?;
            if t.get("TermsCount")
                .and_then(Value::as_f64)
                .is_some_and(|c| c > 0.0)
            {
                child_terms(web, d, &mut t).await?;
            }
            children.push(t);
        }
    }
    parent["Children"] = Value::Array(children);
    Ok(())
}

fn friendly(term: &Value, out: &mut Vec<Value>) {
    if let Some(Value::Array(ch)) = term.get("Children") {
        for c in ch {
            out.push(obj! { "Id": ?c.get("Id").cloned(), "Name": ?c.get("Name").cloned(), "ParentTermId": ?term.get("Id").cloned() });
            friendly(c, out);
        }
    }
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term sets...");
    let gq = match cx.s("termGroupId").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="77" ParentId="75" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="77" ParentId="75" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termGroupName")
        ),
    };
    let sq = match cx.s("termSetId").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="82" ParentId="80" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="82" ParentId="80" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termSetName")
        ),
    };
    let body = sp::request(
        r#"<ObjectPath Id="70" ObjectPathId="69" /><ObjectIdentityQuery Id="71" ObjectPathId="69" /><ObjectPath Id="73" ObjectPathId="72" /><ObjectIdentityQuery Id="74" ObjectPathId="72" /><ObjectPath Id="76" ObjectPathId="75" /><ObjectPath Id="78" ObjectPathId="77" /><ObjectIdentityQuery Id="79" ObjectPathId="77" /><ObjectPath Id="81" ObjectPathId="80" /><ObjectPath Id="83" ObjectPathId="82" /><ObjectIdentityQuery Id="84" ObjectPathId="82" /><ObjectPath Id="86" ObjectPathId="85" /><Query Id="87" ObjectPathId="85"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /></Properties></ChildItemQuery></Query>"#,
        &format!(
            r#"<StaticMethod Id="69" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="72" ParentId="69" Name="GetDefaultSiteCollectionTermStore" /><Property Id="75" ParentId="72" Name="Groups" />{gq}<Property Id="80" ParentId="77" Name="TermSets" />{sq}<Property Id="85" ParentId="82" Name="Terms" />"#
        ),
    );
    let with_children = cx.bool("withChildTerms");
    let r = last(&web, &d, body).await?;
    if r.is_null() {
        return Err(Error::js(
            "Cannot read properties of null (reading '_Child_Items_')",
        ));
    }
    let mut terms = Vec::new();
    if let Some(Value::Array(items)) = r.get("_Child_Items_") {
        for mut t in items.clone() {
            set_term_details(&mut t)?;
            if with_children
                && t.get("TermsCount")
                    .and_then(Value::as_f64)
                    .is_some_and(|c| c > 0.0)
            {
                child_terms(&web, &d, &mut t).await?;
            }
            terms.push(t);
        }
    }
    if cx.str("output").as_deref() != Some("text") {
        return cx.log(Value::Array(terms));
    }
    if !with_children {
        return cx.log(Value::Array(
            terms
                .iter()
                .map(|i| obj! { "Id": ?i.get("Id").cloned(), "Name": ?i.get("Name").cloned() })
                .collect(),
        ));
    }
    let mut out = Vec::new();
    for t in &terms {
        out.push(obj! { "Id": ?t.get("Id").cloned(), "Name": ?t.get("Name").cloned(), "ParentTermId": "" });
        friendly(t, &mut out);
    }
    cx.log(Value::Array(out))
}

pub async fn set_add(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Adding taxonomy term set...");
    let gq = match cx.s("termGroupName").filter(|s| !s.is_empty()) {
        Some(n) => format!(
            r#"<Method Id="42" ParentId="40" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            util::escape_xml(&n)
        ),
        None => format!(
            r#"<Method Id="42" ParentId="40" Name="GetById"><Parameters><Parameter Type="Guid">{{{}}}</Parameter></Parameters></Method>"#,
            prop(&Value::Object(cx.opts.clone()), "termGroupId")
        ),
    };
    let id = cx
        .s("id")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(util::new_guid);
    let body = sp::request(
        r#"<ObjectPath Id="35" ObjectPathId="34" /><ObjectIdentityQuery Id="36" ObjectPathId="34" /><ObjectPath Id="38" ObjectPathId="37" /><ObjectIdentityQuery Id="39" ObjectPathId="37" /><ObjectPath Id="41" ObjectPathId="40" /><ObjectPath Id="43" ObjectPathId="42" /><ObjectIdentityQuery Id="44" ObjectPathId="42" /><ObjectPath Id="46" ObjectPathId="45" /><ObjectIdentityQuery Id="47" ObjectPathId="45" /><Query Id="48" ObjectPathId="45"><Query SelectAllProperties="true"><Properties /></Query></Query>"#,
        &format!(
            r#"<StaticMethod Id="34" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="37" ParentId="34" Name="GetDefaultSiteCollectionTermStore" /><Property Id="40" ParentId="37" Name="Groups" />{gq}<Method Id="45" ParentId="42" Name="CreateTermSet"><Parameters><Parameter Type="String">{}</Parameter><Parameter Type="Guid">{{{id}}}</Parameter><Parameter Type="Int32">1033</Parameter></Parameters></Method>"#,
            ex(cx, "name")
        ),
    );
    let json = sp::post(&web, &d, body).await?;
    sp::check_first(&json)?;
    let mut set = json.last().cloned().unwrap_or(Value::Null);
    let (desc, cp) = (
        cx.s("description").filter(|s| !s.is_empty()),
        cx.s("customProperties").filter(|s| !s.is_empty()),
    );
    if desc.is_some() || cp.is_some() {
        let store = match json.iter().position(|x| x.as_f64() == Some(39.0)) {
            Some(p) => match json.get(p + 1) {
                Some(n) => prop(n, "_ObjectIdentity_"),
                None => return Err(undef("_ObjectIdentity_")),
            },
            None => String::new(),
        };
        cx.verbose_log("Setting term set properties...");
        let mut props = Vec::new();
        let mut i = 127;
        if let Some(dsc) = &desc {
            props.push(format!(r#"<SetProperty Id="{i}" ObjectPathId="117" Name="Description"><Parameter Type="String">{}</Parameter></SetProperty>"#, util::escape_xml(dsc)));
            i += 1;
            if let Some(o) = set.as_object_mut() {
                o.insert("Description".into(), Value::from(dsc.as_str()));
            }
        }
        if let Some(c) = &cp {
            let v = custom_props(c, "SetCustomProperty", &mut i, &mut props)?;
            if let Some(o) = set.as_object_mut() {
                o.insert("CustomProperties".into(), v);
            }
        }
        let body = sp::request(
            &format!(
                r#"{}<Method Name="CommitAll" Id="131" ObjectPathId="109" />"#,
                props.join("")
            ),
            &format!(
                r#"<Identity Id="117" Name="{}" /><Identity Id="109" Name="{store}" />"#,
                prop(&set, "_ObjectIdentity_")
            ),
        );
        let r = m365_core::ut::spo::process_query(&web, &d, body).await?;
        if !r.is_empty() {
            sp::check_first(&sp::parse(&r)?)?;
        }
    }
    fix(&mut set, true)?;
    cx.log(set)
}

pub async fn set_get(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term set...");
    let gq = match cx.s("termGroupId").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="62" ParentId="60" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="62" ParentId="60" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termGroupName")
        ),
    };
    let sq = match cx.s("id").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="67" ParentId="65" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="67" ParentId="65" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "name")
        ),
    };
    let mut s = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="55" ObjectPathId="54" /><ObjectIdentityQuery Id="56" ObjectPathId="54" /><ObjectPath Id="58" ObjectPathId="57" /><ObjectIdentityQuery Id="59" ObjectPathId="57" /><ObjectPath Id="61" ObjectPathId="60" /><ObjectPath Id="63" ObjectPathId="62" /><ObjectIdentityQuery Id="64" ObjectPathId="62" /><ObjectPath Id="66" ObjectPathId="65" /><ObjectPath Id="68" ObjectPathId="67" /><ObjectIdentityQuery Id="69" ObjectPathId="67" /><Query Id="70" ObjectPathId="67"><Query SelectAllProperties="true"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /></Properties></Query></Query>"#,
            &format!(r#"<StaticMethod Id="54" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="57" ParentId="54" Name="GetDefaultSiteCollectionTermStore" /><Property Id="60" ParentId="57" Name="Groups" />{gq}<Property Id="65" ParentId="62" Name="TermSets" />{sq}"#),
        ),
    )
    .await?;
    fix(&mut s, true)?;
    cx.log(s)
}

pub async fn set_list(cx: &mut Cx) -> Result<()> {
    let (web, d) = web_and_digest(cx).await?;
    cx.verbose_log("Retrieving taxonomy term sets...");
    let q = match cx.s("termGroupId").filter(|s| !s.is_empty()) {
        Some(g) => format!(
            r#"<Method Id="62" ParentId="60" Name="GetById"><Parameters><Parameter Type="Guid">{{{g}}}</Parameter></Parameters></Method>"#
        ),
        None => format!(
            r#"<Method Id="62" ParentId="60" Name="GetByName"><Parameters><Parameter Type="String">{}</Parameter></Parameters></Method>"#,
            ex(cx, "termGroupName")
        ),
    };
    let r = last(
        &web,
        &d,
        sp::request(
            r#"<ObjectPath Id="55" ObjectPathId="54" /><ObjectIdentityQuery Id="56" ObjectPathId="54" /><ObjectPath Id="58" ObjectPathId="57" /><ObjectIdentityQuery Id="59" ObjectPathId="57" /><ObjectPath Id="61" ObjectPathId="60" /><ObjectPath Id="63" ObjectPathId="62" /><ObjectIdentityQuery Id="64" ObjectPathId="62" /><ObjectPath Id="66" ObjectPathId="65" /><Query Id="67" ObjectPathId="65"><Query SelectAllProperties="false"><Properties /></Query><ChildItemQuery SelectAllProperties="true"><Properties><Property Name="Name" ScalarProperty="true" /><Property Name="Id" ScalarProperty="true" /></Properties></ChildItemQuery></Query>"#,
            &format!(r#"<StaticMethod Id="54" Name="GetTaxonomySession" TypeId="{SESSION}" /><Method Id="57" ParentId="54" Name="GetDefaultSiteCollectionTermStore" /><Property Id="60" ParentId="57" Name="Groups" />{q}<Property Id="65" ParentId="62" Name="TermSets" />"#),
        ),
    )
    .await?;
    cx.log(Value::Array(fix_children(&r)?))
}

// ---- sitescript ----------------------------------------------------------------

const SSU: &str = "_api/Microsoft.Sharepoint.Utilities.WebTemplateExtensions.SiteScriptUtility";

async fn site_script(cx: &Cx, method: &str, data: Option<Value>) -> Result<Value> {
    let spo_url = spo::get_spo_url(cx.debug()).await?;
    let d = sp::digest(&spo_url).await?;
    let mut r = Request::post(format!("{spo_url}/{SSU}.{method}")).header("X-RequestDigest", d);
    if let Some(data) = data {
        r = r
            .header("content-type", "application/json;charset=utf-8")
            .json(data);
    }
    r.spo_json().data().await
}

pub async fn sitescript_add(cx: &mut Cx) -> Result<()> {
    let content = json::jparse(&cx.u("content"))?;
    let m = format!(
        "CreateSiteScript(Title=@title, Description=@description)?@title='{}'&@description='{}'",
        eqp(&cx.u("title")),
        eqp(&cx.s("description").unwrap_or_default())
    );
    cx.log(site_script(cx, &m, Some(content)).await?)
}

pub async fn sitescript_get(cx: &mut Cx) -> Result<()> {
    let mut r = site_script(cx, "GetSiteScriptMetadata", Some(obj! { "id": cx.u("id") })).await?;
    let c = match r.get("Content") {
        Some(Value::String(s)) => json::jparse(s)?,
        Some(v) => json::jparse(&to_js_string(v))?,
        None => return Err(Error::Syntax("\"undefined\" is not valid JSON".into())),
    };
    if let Some(o) = r.as_object_mut() {
        o.insert("Content".into(), c.clone());
    }
    cx.log(if cx.bool("content") { c } else { r })
}

pub async fn sitescript_list(cx: &mut Cx) -> Result<()> {
    let r = site_script(cx, "GetSiteScripts'", None).await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn sitescript_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the site script {id}?"),
    )? {
        return Ok(());
    }
    site_script(cx, "DeleteSiteScript", Some(obj! { "id": id })).await?;
    Ok(())
}

pub async fn sitescript_set(cx: &mut Cx) -> Result<()> {
    let mut info = obj! { "Id": cx.u("id") };
    if let Some(t) = cx.s("title").filter(|s| !s.is_empty()) {
        info["Title"] = Value::from(t);
    }
    if let Some(t) = cx.s("description").filter(|s| !s.is_empty()) {
        info["Description"] = Value::from(t);
    }
    if let Some(t) = cx.s("version").filter(|s| !s.is_empty()) {
        info["Version"] = json::parse_int(&t).map(Value::from).unwrap_or(Value::Null);
    }
    if let Some(t) = cx.s("content").filter(|s| !s.is_empty()) {
        info["Content"] = Value::from(t);
    }
    cx.log(site_script(cx, "UpdateSiteScript", Some(obj! { "updateInfo": info })).await?)
}

fn json_ok(o: &Obj, k: &str) -> bool {
    match o.get(k).filter(|v| truthy(v)) {
        Some(v) => json::js_parse(&to_js_string(v)).is_ok(),
        None => true,
    }
}

pub fn add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(match i {
        8 => json_ok(o, "customProperties"),
        9 => json_ok(o, "localCustomProperties"),
        _ => true,
    })
}

pub fn set_add_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(i != 3 || json_ok(o, "customProperties"))
}

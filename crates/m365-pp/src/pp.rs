//! `pp *`: Power Platform (Dataverse, solutions, copilots, websites, ...).

use crate::pa::BAP;
use m365_core::prelude::*;
use m365_core::ut::entra_app;

const PAGES: &str = "https://api.powerplatform.com/powerpages/environments";

/// `powerPlatform.getDynamicsInstanceApiUrl`.
async fn dynamics_url(cx: &Cx) -> Result<String> {
    let env = cx.u("environmentName");
    let scope = if cx.bool("asAdmin") {
        "scopes/admin/"
    } else {
        ""
    };
    let url = format!(
        "{BAP}/providers/Microsoft.BusinessAppPlatform/{scope}environments/{}?api-version=2020-10-01&$select=properties.linkedEnvironmentMetadata.instanceApiUrl",
        eqp(&env)
    );
    let fail = |m: String| {
        Error::js(format!(
            "The environment '{env}' could not be retrieved. See the inner exception for more details: {m}"
        ))
    };
    let r = Request::get(url)
        .graph_json()
        .data()
        .await
        .map_err(|e| fail(e.js_message()))?;
    let meta = r
        .get("properties")
        .and_then(|p| p.get("linkedEnvironmentMetadata"));
    match meta {
        Some(m) => Ok(u(m, "instanceApiUrl")),
        None => Err(fail(format!(
            "Cannot read properties of undefined (reading '{}')",
            if r.get("properties").is_none() {
                "linkedEnvironmentMetadata"
            } else {
                "instanceApiUrl"
            }
        ))),
    }
}

fn pick(items: &[Value], key: &str, multiple: String) -> Result<Value> {
    if items.len() > 1 {
        return m365_core::runner::handle_multiple_results(
            &multiple,
            &util::array_to_hash_table(key, items),
        );
    }
    Ok(items[0].clone())
}

async fn solution_by_name(api: &str, name: &str) -> Result<Value> {
    let r = Request::get(format!("{api}/api/data/v9.0/solutions?$filter=isvisible eq true and uniquename eq '{name}'&$expand=publisherid($select=friendlyname)&$select=solutionid,uniquename,version,publisherid,installedon,solutionpackageversion,friendlyname,versionnumber&api-version=9.1"))
        .graph_json()
        .data()
        .await?;
    let v = arr(&r, "value");
    if v.is_empty() {
        return Err(Error::js(format!(
            "The specified solution '{name}' does not exist."
        )));
    }
    pick(
        v,
        "solutionid",
        format!("Multiple solutions with name '{name}' found."),
    )
}

/// Runs another command (`cli.executeCommandWithOutput`) and returns its parsed JSON output.
async fn nested(cx: &Cx, name: &str) -> Result<Value> {
    let o = obj! { "environmentName": ?cx.val("environmentName").cloned(), "name": ?cx.val("name").cloned(), "output": "json", "debug": cx.debug(), "verbose": cx.verbose() };
    let (out, _) =
        m365_core::runner::run_nested(name, o.as_object().cloned().unwrap_or_default(), true)
            .await?;
    json::jparse(&out)
}

pub async fn aibuildermodel_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving an AI builder model '{}'...",
        cx.first_u(&["id", "name"])
    ));
    let api = dynamics_url(cx).await?;
    let r = match cx.s("id") {
        Some(id) => {
            Request::get(format!(
                "{api}/api/data/v9.1/msdyn_aimodels({id})?$filter=iscustomizable/Value eq true"
            ))
            .graph_json()
            .data()
            .await?
        }
        None => {
            let n = cx.u("name");
            let r = Request::get(format!("{api}/api/data/v9.1/msdyn_aimodels?$filter=msdyn_name eq '{n}' and iscustomizable/Value eq true")).graph_json().data().await?;
            let v = arr(&r, "value");
            if v.is_empty() {
                return Err(Error::msg(format!(
                    "The specified AI builder model '{n}' does not exist."
                )));
            }
            pick(
                v,
                "msdyn_aimodelid",
                format!("Multiple AI builder models with name '{n}' found."),
            )?
        }
    };
    cx.log(r)
}

pub async fn aibuildermodel_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving available AI Builder models");
    let api = dynamics_url(cx).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{api}/api/data/v9.0/msdyn_aimodels?$filter=iscustomizable/Value eq true"
        ))
        .await?,
    ))
}

pub async fn aibuildermodel_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    cx.verbose_log(&format!("Removing AI builder model '{what}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove AI builder model '{what}'?"),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    let id = match cx.s("id") {
        Some(i) => i,
        None => u(
            &nested(cx, "pp aibuildermodel get").await?,
            "msdyn_aimodelid",
        ),
    };
    Request::delete(format!("{api}/api/data/v9.1/msdyn_aimodels({id})"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn copilot_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving copilot '{}'...",
        cx.first_u(&["id", "name"])
    ));
    let api = dynamics_url(cx).await?;
    let r = match cx.s("id") {
        Some(id) => {
            Request::get(format!("{api}/api/data/v9.1/bots({id})"))
                .graph_json()
                .data()
                .await?
        }
        None => {
            let n = cx.u("name");
            let r = Request::get(format!(
                "{api}/api/data/v9.1/bots?$filter=name eq '{}'",
                eqp(&n)
            ))
            .graph_json()
            .data()
            .await?;
            let v = arr(&r, "value");
            if v.len() > 1 {
                return cx.log(m365_core::runner::handle_multiple_results(
                    &format!("Multiple copilots with name '{n}' found."),
                    &util::array_to_hash_table("botid", v),
                )?);
            }
            match v.first() {
                Some(b) => b.clone(),
                None => {
                    return Err(Error::msg(format!(
                        "The specified copilot '{n}' does not exist."
                    )));
                }
            }
        }
    };
    cx.log(r)
}

const BOT_FETCH_XML: &str = "
      <fetch mapping='logical' version='1.0' >
        <entity name='bot'>
          <attribute name='accesscontrolpolicy' alias='accessControlPolicy' />,
          <attribute name='applicationmanifestinformation' alias='applicationManifestInformation' />,
          <attribute name='authenticationmode' alias='authenticationMode' />,
          <attribute name='authenticationtrigger' alias='authenticationTrigger' />,
          <attribute name='authorizedsecuritygroupids' alias='authorizedSecurityGroupIds' />,
          <attribute name='componentidunique' alias='componentIdUnique' />,
          <attribute name='componentstate' alias='componentState' />,
          <attribute name='configuration' alias='configuration' />,
          <attribute name='createdon' alias='createdOn' />,
          <attribute name='importsequencenumber' alias='importSequenceNumber' />,
          <attribute name='ismanaged' alias='isManaged' />,
          <attribute name='language' alias='language' />,
          <attribute name='modifiedon' alias='botModifiedOn' />,
          <attribute name='overriddencreatedon' alias='overriddenCreatedOn' />,
          <attribute name='overwritetime' alias='overwriteTime' />,
          <attribute name='iconbase64' alias='iconBase64' />,
          <attribute name='publishedon' alias='publishedOn' />,
          <attribute name='schemaname' alias='schemaName' />,
          <attribute name='solutionid' alias='solutionId' />,
          <attribute name='statecode' alias='stateCode' />,
          <attribute name='statuscode' alias='statusCode' />,
          <attribute name='timezoneruleversionnumber' alias='timezoneRuleVersionNumber' />,
          <attribute name='utcconversiontimezonecode' alias='utcConversionTimezoneCode' />,
          <attribute name='versionnumber' alias='versionNumber' />,
          <attribute name='name' alias='name' />,
          <attribute name='botid' alias='cdsBotId' />,
          <attribute name='ownerid' alias='ownerId' />,
          <attribute name='synchronizationstatus' alias='synchronizationStatus' />
          <link-entity name='systemuser' to='ownerid' from='systemuserid' link-type='inner' >
            <attribute name='fullname' alias='owner' />
          </link-entity>
          <link-entity name='systemuser' to='modifiedby' from='systemuserid' link-type='inner' >
            <attribute name='fullname' alias='botModifiedBy' />
          </link-entity>
        </entity>
      </fetch>
    ";

pub async fn copilot_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving list of copilots for environment '{}'.",
        cx.u("environmentName")
    ));
    let api = dynamics_url(cx).await?;
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{api}/api/data/v9.1/bots?fetchXml={BOT_FETCH_XML}"
        ))
        .await?,
    ))
}

pub async fn copilot_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    cx.verbose_log(&format!("Removing copilot '{what}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove copilot '{what}'?"),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    let id = match cx.s("id") {
        Some(i) => i,
        None => u(&nested(cx, "pp copilot get").await?, "botid"),
    };
    Request::post(format!("{api}/api/data/v9.1/bots({id})/Microsoft.Dynamics.CRM.PvaDeleteBot?tag=deprovisionbotondelete"))
        .accept("application/json")
        .header("content-type", "application/json")
        .json_response()
        .send()
        .await?;
    Ok(())
}

const TABLE_SELECT: &str = "MetadataId,IsCustomEntity,IsManaged,SchemaName,IconVectorName,LogicalName,EntitySetName,IsActivity,DataProviderId,IsRenameable,IsCustomizable,CanCreateForms,CanCreateViews,CanCreateCharts,CanCreateAttributes,CanChangeTrackingBeEnabled,CanModifyAdditionalSettings,CanChangeHierarchicalRelationship,CanEnableSyncToExternalSearchIndex";

pub async fn dataverse_table_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving a table for which the user is an admin...");
    let api = dynamics_url(cx).await?;
    cx.log(Request::get(format!("{api}/api/data/v9.0/EntityDefinitions(LogicalName='{}')?$select={TABLE_SELECT}&api-version=9.1", cx.u("name"))).graph_json().data().await?)
}

pub async fn dataverse_table_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of tables for which the user is an admin...");
    let api = dynamics_url(cx).await?;
    let filter = "(IsIntersect eq false and IsLogicalEntity eq false and%0APrimaryNameAttribute ne null and PrimaryNameAttribute ne %27%27 and ObjectTypeCode gt 0 and%0AObjectTypeCode ne 4712 and ObjectTypeCode ne 4724 and ObjectTypeCode ne 9933 and ObjectTypeCode ne 9934 and%0AObjectTypeCode ne 9935 and ObjectTypeCode ne 9947 and ObjectTypeCode ne 9945 and ObjectTypeCode ne 9944 and%0AObjectTypeCode ne 9942 and ObjectTypeCode ne 9951 and ObjectTypeCode ne 2016 and ObjectTypeCode ne 9949 and%0AObjectTypeCode ne 9866 and ObjectTypeCode ne 9867 and ObjectTypeCode ne 9868) and (IsCustomizable/Value eq true or IsCustomEntity eq true or IsManaged eq false or IsMappable/Value eq true or IsRenameable/Value eq true)";
    cx.log(Value::Array(odata::get_all_items(&format!("{api}/api/data/v9.0/EntityDefinitions?$select={TABLE_SELECT}&$filter={filter}&api-version=9.1")).await?))
}

pub async fn dataverse_table_remove(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Removing a table for which the user is an admin...");
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the dataverse table {}?",
            cx.u("name")
        ),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    Request::delete(format!(
        "{api}/api/data/v9.0/EntityDefinitions(LogicalName='{}')",
        cx.u("name")
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

async fn entity_set(cx: &Cx, api: &str) -> Result<String> {
    if let Some(e) = cx.s("entitySetName") {
        return Ok(e);
    }
    let r = Request::get(format!(
        "{api}/api/data/v9.0/EntityDefinitions(LogicalName='{}')?$select=EntitySetName",
        cx.u("tableName")
    ))
    .graph_json()
    .data()
    .await?;
    Ok(u(&r, "EntitySetName"))
}

pub async fn dataverse_table_row_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of table rows");
    let api = dynamics_url(cx).await?;
    let set = entity_set(cx, &api).await?;
    cx.verbose_log(&format!("Entity set name is: {set}"));
    cx.log(Value::Array(
        odata::get_all_items(&format!("{api}/api/data/v9.0/{set}")).await?,
    ))
}

pub async fn dataverse_table_row_remove(cx: &mut Cx) -> Result<()> {
    let table = cx.first_u(&["tableName", "entitySetName"]);
    let id = cx.u("id");
    cx.verbose_log(&format!("Removing row '{id}' from table '{table}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove row '{id}' from table '{table}'?"),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    let set = entity_set(cx, &api).await?;
    cx.verbose_log(&format!("Entity set name is: {set}"));
    Request::delete(format!("{api}/api/data/v9.1/{set}({id})"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn environment_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving environment: {}",
        cx.s("name").unwrap_or_else(|| "default".into())
    ));
    let scope = if cx.bool("asAdmin") {
        "/scopes/admin"
    } else {
        ""
    };
    let env = if cx.bool("default") {
        "~Default".to_string()
    } else {
        eqp(&cx.u("name"))
    };
    // requested outside a try/catch
    let r = Request::get(format!("{BAP}/providers/Microsoft.BusinessAppPlatform{scope}/environments/{env}?api-version=2020-10-01")).graph_json().data().await.map_err(Error::uncaught)?;
    cx.log(r)
}

pub async fn environment_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of Microsoft Power Platform environments...");
    let scope = if cx.bool("asAdmin") {
        "scopes/admin/"
    } else {
        ""
    };
    let r = Request::get(format!(
        "{BAP}/providers/Microsoft.BusinessAppPlatform/{scope}environments?api-version=2020-10-01"
    ))
    .accept("application/json")
    .json_response()
    .data()
    .await?;
    match r.get("value") {
        Some(Value::Array(a)) => {
            let mut items = a.clone();
            crate::pa::copy_display_name(&mut items)?;
            cx.log(Value::Array(items))
        }
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn gateway_get(cx: &mut Cx) -> Result<()> {
    cx.log(
        Request::get(format!(
            "https://api.powerbi.com/v1.0/myorg/gateways/{}",
            eqp(&cx.u("id"))
        ))
        .graph_json()
        .data()
        .await?,
    )
}

pub async fn gateway_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of gateways for which the user is an admin...");
    let r = Request::get("https://api.powerbi.com/v1.0/myorg/gateways")
        .graph_json()
        .data()
        .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn managementapp_add(cx: &mut Cx) -> Result<()> {
    let app = if let Some(a) = cx.s("appId") {
        a
    } else if let Some(o) = cx.s("objectId") {
        u(
            &entra_app::get_by_object_id(&o, Some(&["appId"])).await?,
            "appId",
        )
    } else {
        u(
            &entra_app::get_by_app_name(&cx.u("name"), Some(&["appId"])).await?,
            "appId",
        )
    };
    cx.log(Request::put(format!("{BAP}/providers/Microsoft.BusinessAppPlatform/adminApplications/{app}?api-version=2020-06-01")).graph_json().data().await?)
}

pub async fn managementapp_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{BAP}/providers/Microsoft.BusinessAppPlatform/adminApplications?api-version=2020-06-01"
        ))
        .await?,
    ))
}

fn trimmed_solution(s: &Value) -> Result<Value> {
    let p = s
        .get("publisherid")
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'friendlyname')"))?;
    Ok(
        obj! { "uniquename": ?s.get("uniquename").cloned(), "version": ?s.get("version").cloned(), "publisher": ?p.get("friendlyname").cloned() },
    )
}

fn trim_output(cx: &Cx) -> bool {
    cx.str("output").as_deref() == Some("text")
}

pub async fn solution_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving a specific solution '{}'...",
        cx.first_u(&["id", "name"])
    ));
    let api = dynamics_url(cx).await?;
    let r = match cx.s("name") {
        Some(n) => solution_by_name(&api, &n).await?,
        None => Request::get(format!("{api}/api/data/v9.0/solutions({})?$expand=publisherid($select=friendlyname)&$select=solutionid,uniquename,version,publisherid,installedon,solutionpackageversion,friendlyname,versionnumber&api-version=9.1", cx.u("id")))
            .graph_json()
            .data()
            .await?,
    };
    cx.log(if trim_output(cx) {
        trimmed_solution(&r)?
    } else {
        r
    })
}

pub async fn solution_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of solutions for which the user is an admin...");
    let api = dynamics_url(cx).await?;
    let items = odata::get_all_items(&format!("{api}/api/data/v9.0/solutions?$filter=isvisible eq true&$expand=publisherid($select=friendlyname)&$select=solutionid,uniquename,version,publisherid,installedon,solutionpackageversion,friendlyname,versionnumber&api-version=9.1")).await?;
    if !trim_output(cx) {
        return cx.log(Value::Array(items));
    }
    let out: Result<Vec<Value>> = items.iter().map(trimmed_solution).collect();
    cx.log(Value::Array(out?))
}

pub async fn solution_publish(cx: &mut Cx) -> Result<()> {
    let api = dynamics_url(cx).await?;
    let sid = match cx.s("id") {
        Some(i) => i,
        None => {
            cx.verbose_log("Retrieving solutionId...");
            u(&solution_by_name(&api, &cx.u("name")).await?, "solutionid")
        }
    };
    cx.verbose_log("Retrieving solution components");
    let r = Request::get(format!("{api}/api/data/v9.0/msdyn_solutioncomponentsummaries?$filter=(msdyn_solutionid eq {sid})&$select=msdyn_componentlogicalname,msdyn_name&$orderby=msdyn_componentlogicalname asc&api-version=9.1"))
        .graph_json()
        .data()
        .await?;
    let comps = r
        .get("value")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'reduce')"))?;
    cx.verbose_log("Building the XML request object...");
    let mut groups = Obj::new();
    for c in &comps {
        let logical = c
            .get("msdyn_componentlogicalname")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'slice')"))?
            .to_string();
        let key = match logical.strip_suffix('y') {
            Some(stem) => format!("{stem}ies"),
            None => format!("{logical}s"),
        };
        let mut item = Obj::new();
        item.insert(
            logical.clone(),
            c.get("msdyn_name").cloned().unwrap_or(Value::Null),
        );
        match groups.get_mut(&key) {
            Some(Value::Array(a)) => a.push(Value::Object(item)),
            _ => {
                groups.insert(key, Value::Array(vec![Value::Object(item)]));
            }
        }
    }
    let xml = format!(
        "<importexportxml>{}</importexportxml>",
        util::object_to_xml(&Value::Object(groups))
    );
    cx.verbose_log(&format!(
        "Publishing the solution '{}'...",
        cx.first_u(&["id", "name"])
    ));
    let req = Request::post(format!("{api}/api/data/v9.0/PublishXml"))
        .graph_json()
        .json(obj! { "ParameterXml": xml });
    if cx.bool("wait") {
        req.send().await?;
    } else {
        // fire and forget: the CLI exits without waiting for the response
        let _ = req.send().await;
    }
    Ok(())
}

pub async fn solution_publisher_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!("Adding new publisher '{}'...", cx.u("name")));
    let api = dynamics_url(cx).await?;
    let data = obj! {
        "uniquename": ?cx.val("name").cloned(),
        "friendlyname": ?cx.val("displayName").cloned(),
        "customizationprefix": ?cx.val("prefix").cloned(),
        "customizationoptionvalueprefix": json::num_value(json::js_number(&cx.u("choiceValuePrefix"))),
    };
    Request::post(format!("{api}/api/data/v9.0/publishers"))
        .graph_json()
        .json(data)
        .send()
        .await?;
    Ok(())
}

const PUBLISHER_SELECT: &str = "publisherid,uniquename,friendlyname,versionnumber,isreadonly,description,customizationprefix,customizationoptionvalueprefix";

pub async fn solution_publisher_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving a specific publisher '{}'...",
        cx.first_u(&["id", "name"])
    ));
    let api = dynamics_url(cx).await?;
    let r = match cx.s("id") {
        Some(id) => {
            Request::get(format!(
                "{api}/api/data/v9.0/publishers({id})?$select={PUBLISHER_SELECT}&api-version=9.1"
            ))
            .graph_json()
            .data()
            .await?
        }
        None => {
            let n = cx.u("name");
            let r = Request::get(format!("{api}/api/data/v9.0/publishers?$filter=friendlyname eq '{n}'&$select={PUBLISHER_SELECT}&api-version=9.1")).graph_json().data().await?;
            match arr(&r, "value").first() {
                Some(p) => p.clone(),
                None => {
                    return Err(Error::msg(format!(
                        "The specified publisher '{n}' does not exist."
                    )));
                }
            }
        }
    };
    cx.log(r)
}

pub async fn solution_publisher_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of publishers...");
    let api = dynamics_url(cx).await?;
    let filter = if cx.bool("withMicrosoftPublishers") {
        ""
    } else {
        "&$filter=publisherid ne 'd21aab70-79e7-11dd-8874-00188b01e34f'"
    };
    let r = Request::get(format!(
        "{api}/api/data/v9.0/publishers?$select={PUBLISHER_SELECT}{filter}&api-version=9.1"
    ))
    .graph_json()
    .data()
    .await?;
    match r.get("value") {
        Some(v) => cx.log(v.clone()),
        None => cx.log_undefined(),
    }
}

pub async fn solution_publisher_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    cx.verbose_log(&format!("Removes a publisher '{what}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove publisher '{what}'?"),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    let id = match cx.s("id") {
        Some(i) => i,
        None => u(
            &nested(cx, "pp solution publisher get").await?,
            "publisherid",
        ),
    };
    Request::delete(format!("{api}/api/data/v9.1/publishers({id})"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn solution_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    cx.verbose_log(&format!("Removing solution '{what}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove solution '{what}'?"),
    )? {
        return Ok(());
    }
    let api = dynamics_url(cx).await?;
    let id = match cx.s("id") {
        Some(i) => i,
        None => u(&solution_by_name(&api, &cx.u("name")).await?, "solutionid"),
    };
    Request::delete(format!("{api}/api/data/v9.1/solutions({id})"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn tenant_settings_list(cx: &mut Cx) -> Result<()> {
    cx.log(Request::post(format!("{BAP}/providers/Microsoft.BusinessAppPlatform/listtenantsettings?api-version=2020-10-01")).accept("application/json").json_response().data().await?)
}

pub async fn tenant_settings_set(cx: &mut Cx) -> Result<()> {
    let g = |k: &str| cx.val(k).cloned();
    let n = |k: &str| cx.str(k).map(|v| json::num_value(json::js_number(&v)));
    let data = obj! {
        "walkMeOptOut": ?g("walkMeOptOut"),
        "disableNPSCommentsReachout": ?g("disableNPSCommentsReachout"),
        "disableNewsletterSendout": ?g("disableNewsletterSendout"),
        "disableEnvironmentCreationByNonAdminUsers": ?g("disableEnvironmentCreationByNonAdminUsers"),
        "disablePortalsCreationByNonAdminUsers": ?g("disablePortalsCreationByNonAdminUsers"),
        "disableSurveyFeedback": ?g("disableSurveyFeedback"),
        "disableTrialEnvironmentCreationByNonAdminUsers": ?g("disableTrialEnvironmentCreationByNonAdminUsers"),
        "disableCapacityAllocationByEnvironmentAdmins": ?g("disableCapacityAllocationByEnvironmentAdmins"),
        "disableSupportTicketsVisibleByAllUsers": ?g("disableSupportTicketsVisibleByAllUsers"),
        "powerPlatform": obj! {
            "search": obj! { "disableDocsSearch": ?g("disableDocsSearch"), "disableCommunitySearch": ?g("disableCommunitySearch"), "disableBingVideoSearch": ?g("disableBingVideoSearch") },
            "teamsIntegration": obj! { "shareWithColleaguesUserLimit": ?n("shareWithColleaguesUserLimit") },
            "powerApps": obj! { "disableShareWithEveryone": ?g("disableShareWithEveryone"), "enableGuestsToMake": ?g("enableGuestsToMake"), "disableMembersIndicator": ?g("disableMembersIndicator"), "disableMakerMatch": ?g("disableMakerMatch") },
            "environments": obj! { "disablePreferredDataLocationForTeamsEnvironment": ?g("disablePreferredDataLocationForTeamsEnvironment") },
            "governance": obj! { "disableAdminDigest": ?g("disableAdminDigest"), "disableDeveloperEnvironmentCreationByNonAdminUsers": ?g("disableDeveloperEnvironmentCreationByNonAdminUsers") },
            "licensing": obj! { "disableBillingPolicyCreationByNonAdminUsers": ?g("disableBillingPolicyCreationByNonAdminUsers"), "storageCapacityConsumptionWarningThreshold": ?n("storageCapacityConsumptionWarningThreshold") },
            "champions": obj! { "disableChampionsInvitationReachout": ?g("disableChampionsInvitationReachout"), "disableSkillsMatchInvitationReachout": ?g("disableSkillsMatchInvitationReachout") },
            "intelligence": obj! { "disableCopilot": ?g("disableCopilot"), "enableOpenAiBotPublishing": ?g("enableOpenAiBotPublishing") },
            "modelExperimentation": obj! { "enableModelDataSharing": ?g("enableModelDataSharing") },
        },
    };
    cx.log(Request::post(format!("{BAP}/providers/Microsoft.BusinessAppPlatform/scopes/admin/updateTenantSettings?api-version=2020-10-01")).accept("application/json").json_response().json(data).data().await?)
}

async fn website_by_id(env: &str, id: &str) -> Result<Value> {
    Request::get(format!(
        "{PAGES}/{env}/websites/{id}?api-version=2022-03-01-preview"
    ))
    .graph_json()
    .data()
    .await
    .map_err(|_| {
        Error::js(format!(
            "The specified Power Page website with id '{id}' does not exist."
        ))
    })
}

async fn websites(env: &str) -> Result<Vec<Value>> {
    odata::get_all_items(&format!(
        "{PAGES}/{env}/websites?api-version=2022-03-01-preview"
    ))
    .await
}

async fn website_by_name(env: &str, name: &str) -> Result<Value> {
    let items: Vec<Value> = websites(env)
        .await?
        .into_iter()
        .filter(|w| s(w, "name") == name)
        .collect();
    if items.is_empty() {
        return Err(Error::js(format!(
            "The specified Power Page website '{name}' does not exist."
        )));
    }
    pick(
        &items,
        "websiteUrl",
        format!("Multiple Power Page websites with name '{name}' found."),
    )
}

async fn website_by_url(env: &str, url: &str) -> Result<Value> {
    websites(env)
        .await?
        .into_iter()
        .find(|w| s(w, "websiteUrl") == url)
        .ok_or_else(|| {
            Error::js(format!(
                "The specified Power Page website with url '{url}' does not exist."
            ))
        })
}

pub async fn website_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving the website...");
    let env = cx.u("environmentName");
    let r = if let Some(i) = cx.s("id") {
        website_by_id(&env, &i).await?
    } else if let Some(n) = cx.s("name") {
        website_by_name(&env, &n).await?
    } else if let Some(u) = cx.s("url") {
        website_by_url(&env, &u).await?
    } else {
        Value::Null
    };
    cx.log(r)
}

pub async fn website_remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name", "url"]);
    cx.verbose_log(&format!("Removing website '{what}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove website '{what}'?"),
    )? {
        return Ok(());
    }
    let env = cx.u("environmentName");
    let id = if let Some(i) = cx.s("id") {
        i
    } else if let Some(n) = cx.s("name") {
        u(&website_by_name(&env, &n).await?, "id")
    } else {
        u(&website_by_url(&env, &cx.u("url")).await?, "id")
    };
    Request::delete(format!(
        "{PAGES}/{env}/websites/{id}?api-version=2022-03-01-preview"
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn website_webrole_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Retrieving the webroles for '{}'...",
        cx.s("websiteId")
            .or_else(|| cx.s("websiteName"))
            .unwrap_or_else(|| "undefined".into())
    ));
    let api = dynamics_url(cx).await?;
    let rid = match cx.s("websiteId") {
        Some(i) => u(
            &website_by_id(&cx.u("environmentName"), &i).await?,
            "websiteRecordId",
        ),
        None => {
            let n = cx.u("websiteName");
            let r = Request::get(format!(
                "{api}/api/data/v9.2/powerpagesites?$filter=name eq '{n}'"
            ))
            .graph_json()
            .data()
            .await?;
            match arr(&r, "value").first() {
                Some(s) => u(s, "powerpagesiteid"),
                None => {
                    return Err(Error::js(format!(
                        "The specified website '{n}' does not exist."
                    )));
                }
            }
        }
    };
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{api}/api/data/v9.2/mspp_webroles?$filter=_mspp_websiteid_value eq {rid}"
        ))
        .await?,
    ))
}

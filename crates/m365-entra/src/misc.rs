//! `entra multitenant`, `oauth2grant`, `organization`, `policy`,
//! `resourcenamespace`, `roleassignment`, `rolepermission` and
//! `siteclassification` commands.

use m365_core::prelude::*;
use m365_core::ut::{entra, entra_app, entra_sp};
use m365_core::validation as val;

const MTO: &str = "https://graph.microsoft.com/v1.0/tenantRelationships/multiTenantOrganization";

pub async fn multitenant_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Creating multitenant organization...");
    let r = Request::put(MTO)
        .graph_json()
        .json(obj! { "description": ?cx.str("description"), "displayName": ?cx.str("displayName") })
        .data()
        .await?;
    cx.log(r)
}

pub async fn multitenant_get(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving multitenant organization...");
    let r = Request::get(MTO).graph_json().data().await?;
    cx.log(r)
}

async fn tenant_ids() -> Result<Vec<String>> {
    let t = odata::get_all_items_req(
        Request::get(format!("{MTO}/tenants?$select=tenantId")).graph_json(),
    )
    .await?;
    Ok(t.iter().map(|x| u(x, "tenantId")).collect())
}

async fn remove_tenant(cx: &Cx, id: &str) -> Result<()> {
    cx.verbose_log(&format!("Removing tenant: {id}"));
    Request::delete(format!("{MTO}/tenants/{id}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn multitenant_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to remove multitenant organization?",
    )? {
        return Ok(());
    }
    let orgs = odata::get_all_items_req(
        Request::get(format!("{GRAPH}/v1.0/organization?$select=id")).graph_json(),
    )
    .await?;
    let tenant_id = orgs
        .first()
        .map(|o| u(o, "id"))
        .unwrap_or_else(|| "undefined".into());
    let mut ids = tenant_ids().await?;
    let count = ids.len();
    if count > 0 {
        for t in ids.iter().filter(|x| **x != tenant_id) {
            remove_tenant(cx, t).await?;
        }
        loop {
            cx.verbose_log("Waiting 30 seconds...");
            util::sleep_ms(30_000).await;
            cx.verbose_log("Checking all tenants were removed...");
            ids = tenant_ids().await?;
            cx.verbose_log(&format!(
                "Number of removed tenants: {}",
                count as i64 - ids.len() as i64
            ));
            if ids.len() == 1 {
                break;
            }
        }
        remove_tenant(cx, &tenant_id).await?;
        cx.stderr("Your Multi-Tenant organization is being removed. This can take up to 2 hours.");
    }
    Ok(())
}

pub async fn multitenant_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Updating multitenant organization...");
    Request::patch(MTO)
        .graph_json()
        .json(obj! { "description": ?cx.str("description"), "displayName": ?cx.str("displayName") })
        .send()
        .await?;
    Ok(())
}

pub async fn oauth2grant_add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Granting the service principal specified permissions...");
    Request::post(format!("{GRAPH}/v1.0/oauth2PermissionGrants"))
        .header("content-type", "application/json;odata.metadata=none")
        .json(obj! { "clientId": ?cx.str("clientId"), "consentType": "AllPrincipals", "principalId": Value::Null, "resourceId": ?cx.str("resourceId"), "scope": ?cx.str("scope") })
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub async fn oauth2grant_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving list of OAuth grants for the service principal...");
    let r = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/oauth2PermissionGrants?$filter=clientId eq '{}'",
        eqp(&cx.u("spObjectId"))
    ))
    .await?;
    cx.log(Value::Array(r))
}

pub async fn oauth2grant_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        &format!(
            "Are you sure you want to remove the OAuth2 permissions for {}?",
            cx.u("grantId")
        ),
    )? {
        return Ok(());
    }
    cx.verbose_log("Removing OAuth2 permissions...");
    Request::delete(format!(
        "{GRAPH}/v1.0/oauth2PermissionGrants/{}",
        eqp(&cx.u("grantId"))
    ))
    .graph_json()
    .send()
    .await?;
    Ok(())
}

pub async fn oauth2grant_set(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Updating OAuth2 permissions...");
    Request::patch(format!(
        "{GRAPH}/v1.0/oauth2PermissionGrants/{}",
        eqp(&cx.u("grantId"))
    ))
    .header("content-type", "application/json")
    .json(obj! { "scope": ?cx.str("scope") })
    .json_response()
    .send()
    .await?;
    Ok(())
}

pub async fn organization_list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/organization");
    if let Some(p) = cx.s("properties") {
        url.push_str(&format!("?$select={p}"));
    }
    if cx.bool("verbose") {
        cx.stderr("Retrieving organizations...");
    }
    let r = odata::get_all_items_req(
        Request::get(url)
            .graph_json()
            .header("content-type", "application/json"),
    )
    .await?;
    cx.log(Value::Array(r))
}

pub async fn organization_set(cx: &mut Cx) -> Result<()> {
    let id = match cx.s("displayName") {
        Some(dn) => {
            let orgs =
                odata::get_all_items(&format!("{GRAPH}/v1.0/organization?$select=id,displayName"))
                    .await?;
            let first = orgs.first().cloned().unwrap_or(Value::Null);
            if first.get("displayName").map(to_js_string) != Some(dn.clone()) {
                return Err(Error::msg(format!(
                    "The specified organization '{dn}' does not exist."
                )));
            }
            u(&first, "id")
        }
        None => cx.u("id"),
    };
    if cx.bool("verbose") {
        cx.stderr(&format!("Updating organization with ID {id}..."));
    }
    let split = |k: &str| {
        cx.str(k)
            .map(|v| Value::Array(v.split(',').map(Value::from).collect()))
    };
    let mut data = obj! {
        "marketingNotificationEmails": ?split("marketingNotificationEmails"),
        "securityComplianceNotificationMails": ?split("securityComplianceNotificationMails"),
        "securityComplianceNotificationPhones": ?split("securityComplianceNotificationPhones"),
        "technicalNotificationMails": ?split("technicalNotificationMails")
    };
    if cx.s("contactEmail").is_some() || cx.s("statementUrl").is_some() {
        data.as_object_mut().unwrap().insert(
            "privacyProfile".into(),
            obj! { "contactEmail": ?cx.s("contactEmail"), "statementUrl": ?cx.s("statementUrl") },
        );
    }
    Request::patch(format!("{GRAPH}/v1.0/organization/{id}"))
        .graph_json()
        .header("content-type", "application/json")
        .json(data)
        .send()
        .await?;
    Ok(())
}

const POLICIES: &[(&str, &str)] = &[
    ("activitybasedtimeout", "activityBasedTimeoutPolicies"),
    ("adminconsentrequest", "adminConsentRequestPolicy"),
    ("appmanagement", "appManagementPolicies"),
    ("authenticationflows", "authenticationFlowsPolicy"),
    ("authenticationmethods", "authenticationMethodsPolicy"),
    ("authenticationstrength", "authenticationStrengthPolicies"),
    ("authorization", "authorizationPolicy"),
    ("claimsmapping", "claimsMappingPolicies"),
    ("conditionalaccess", "conditionalAccessPolicies"),
    ("crosstenantaccess", "crossTenantAccessPolicy"),
    ("defaultappmanagement", "defaultAppManagementPolicy"),
    ("deviceregistration", "deviceRegistrationPolicy"),
    ("featurerolloutpolicy", "featureRolloutPolicies"),
    ("homerealmdiscovery", "homeRealmDiscoveryPolicies"),
    (
        "identitysecuritydefaultsenforcement",
        "identitySecurityDefaultsEnforcementPolicy",
    ),
    ("permissiongrant", "permissionGrantPolicies"),
    ("rolemanagement", "roleManagementPolicies"),
    ("tokenissuance", "tokenIssuancePolicies"),
    ("tokenlifetime", "tokenLifetimePolicies"),
];

async fn policies(t: &str) -> Result<Value> {
    let ep = POLICIES
        .iter()
        .find(|(k, _)| *k == t)
        .map(|(_, v)| *v)
        .unwrap_or("undefined");
    let mut url = format!("{GRAPH}/v1.0/policies/{ep}");
    if ep == "roleManagementPolicies" {
        url.push_str("?$filter=scopeId eq '/' and scopeType eq 'DirectoryRole'");
    }
    let r = Request::get(url).graph_json().data().await?;
    let single = [
        "adminConsentRequestPolicy",
        "authenticationFlowsPolicy",
        "authenticationMethodsPolicy",
        "authorizationPolicy",
        "crossTenantAccessPolicy",
        "defaultAppManagementPolicy",
        "deviceRegistrationPolicy",
        "identitySecurityDefaultsEnforcementPolicy",
    ];
    Ok(if single.contains(&ep) {
        r
    } else {
        r.get("value").cloned().unwrap_or(Value::Null)
    })
}

pub async fn policy_list(cx: &mut Cx) -> Result<()> {
    let t = cx
        .s("type")
        .map(|t| t.to_lowercase())
        .unwrap_or_else(|| "all".into());
    if t != "all" {
        let p = policies(&t).await?;
        return cx.log(p);
    }
    let mut all = Vec::new();
    for (k, _) in POLICIES {
        match policies(k).await? {
            Value::Array(a) => all.extend(a),
            other => all.push(other),
        }
    }
    cx.log(Value::Array(all))
}

pub async fn resourcenamespace_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting a list of the RBAC resource namespaces and their properties...");
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/beta/roleManagement/directory/resourceNamespaces"
        ))
        .await?,
    ))
}

pub fn roleassignment_add_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let p = to_js_string(v);
    check(
        val::is_valid_guid(&p)
            || val::is_valid_user_principal_name(&p)
            || val::is_valid_mail_nickname(&p),
    )
}

pub fn roleassignment_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let keys = [
        "userId",
        "userName",
        "administrativeUnitId",
        "administrativeUnitName",
        "applicationId",
        "applicationObjectId",
        "applicationName",
        "servicePrincipalId",
        "servicePrincipalName",
        "groupId",
        "groupName",
    ];
    Ok(keys.iter().filter(|k| o.contains_key(**k)).count() < 2)
}

pub async fn roleassignment_add(cx: &mut Cx) -> Result<()> {
    let role = match cx.s("roleDefinitionId") {
        Some(r) => r,
        None => {
            let n = cx.u("roleDefinitionName");
            cx.verbose_log(&format!("Retrieving role definition by its name '{n}'"));
            u(
                &entra::role_definition_by_display_name(&n, None).await?,
                "id",
            )
        }
    };
    let principal = cx.u("principal");
    let principal_id = if val::is_valid_guid(&principal) {
        principal
    } else if val::is_valid_user_principal_name(&principal) {
        cx.verbose_log(&format!("Retrieving user id by UPN '{principal}'"));
        entra::user::id_by_upn(&principal).await?
    } else {
        cx.verbose_log(&format!(
            "Retrieving group id by mail nickname '{principal}'"
        ));
        entra::group::id_by_mail_nickname(&principal).await?
    };
    let mut prefix = "/";
    let mut resource = String::new();
    if cx.s("userId").is_some() || cx.s("userName").is_some() {
        resource = match cx.s("userId") {
            Some(i) => i,
            None => entra::user::id_by_upn(&cx.u("userName")).await?,
        };
    } else if cx.s("administrativeUnitId").is_some() || cx.s("administrativeUnitName").is_some() {
        prefix = "/administrativeUnits/";
        resource = match cx.s("administrativeUnitId") {
            Some(i) => i,
            None => u(
                &entra::administrative_unit_by_display_name(
                    &cx.u("administrativeUnitName"),
                    Some("id"),
                )
                .await?,
                "id",
            ),
        };
    } else if cx.s("applicationId").is_some()
        || cx.s("applicationObjectId").is_some()
        || cx.s("applicationName").is_some()
    {
        resource = cx.u("applicationObjectId");
        if let Some(a) = cx.s("applicationId") {
            resource = u(&entra_app::get_by_app_id(&a, Some(&["id"])).await?, "id");
        } else if let Some(n) = cx.s("applicationName") {
            resource = u(&entra_app::get_by_app_name(&n, Some(&["id"])).await?, "id");
        }
    } else if cx.s("servicePrincipalId").is_some() || cx.s("servicePrincipalName").is_some() {
        resource = match cx.s("servicePrincipalId") {
            Some(i) => i,
            None => u(
                &entra_sp::get_by_app_name(&cx.u("servicePrincipalName"), Some("id")).await?,
                "id",
            ),
        };
    } else if cx.s("groupId").is_some() || cx.s("groupName").is_some() {
        resource = match cx.s("groupId") {
            Some(i) => i,
            None => entra::group::id_by_display_name(&cx.u("groupName")).await?,
        };
    }
    let r = Request::post(format!("{GRAPH}/v1.0/roleManagement/directory/roleAssignments"))
        .graph_json()
        .json(obj! { "roleDefinitionId": role, "principalId": principal_id, "directoryScopeId": format!("{prefix}{resource}") })
        .data()
        .await?;
    cx.log(r)
}

pub async fn rolepermission_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Getting Microsoft Entra ID role permissions...");
    let q = if cx.bool("privileged") {
        "?$filter=isPrivileged eq true"
    } else {
        ""
    };
    let r = odata::get_all_items(&format!(
        "{GRAPH}/beta/roleManagement/directory/resourceNamespaces/{}/resourceActions{q}",
        cx.u("resourceNamespace")
    ))
    .await?;
    cx.log(Value::Array(r))
}

async fn group_settings() -> Result<Vec<Value>> {
    let r = Request::get(format!("{GRAPH}/v1.0/groupSettings"))
        .graph_json()
        .data()
        .await?;
    Ok(arr(&r, "value").to_vec())
}

fn unified(v: &[Value]) -> Vec<Value> {
    v.iter()
        .filter(|d| s(d, "displayName") == "Group.Unified")
        .cloned()
        .collect()
}

pub async fn siteclassification_disable(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to disable site classification?",
    )? {
        return Ok(());
    }
    let all = group_settings().await?;
    if all.is_empty() {
        return Err(Error::msg("Site classification is not enabled."));
    }
    let u0 = unified(&all);
    let Some(first) = u0.first() else {
        return Err(Error::msg(
            "Missing DirectorySettingTemplate for \"Group.Unified\"",
        ));
    };
    let id = s(first, "id");
    if id.is_empty() {
        return Err(Error::msg("Missing UnifiedGroupSettting id"));
    }
    Request::delete(format!("{GRAPH}/v1.0/groupSettings/{id}"))
        .graph_json()
        .header("content-type", "application/json")
        .send()
        .await?;
    Ok(())
}

pub async fn siteclassification_enable(cx: &mut Cx) -> Result<()> {
    let r = Request::get(format!("{GRAPH}/v1.0/groupSettingTemplates"))
        .graph_json()
        .data()
        .await?;
    let u0 = unified(arr(&r, "value"));
    let Some(t) = u0.first() else {
        return Err(Error::msg(
            "Missing DirectorySettingTemplate for \"Group.Unified\"",
        ));
    };
    let mut values = Vec::new();
    for d in arr(t, "values") {
        let name = d.get("name").cloned().unwrap_or(Value::Null);
        let default = d.get("defaultValue").cloned();
        let v = match s(d, "name") {
            "ClassificationList" => cx.str("classifications").map(Value::from),
            "DefaultClassification" => cx.str("defaultClassification").map(Value::from),
            "UsageGuidelinesUrl" => cx.s("usageGuidelinesUrl").map(Value::from).or(default),
            "GuestUsageGuidelinesUrl" => {
                cx.s("guestUsageGuidelinesUrl").map(Value::from).or(default)
            }
            _ => default,
        };
        values.push(obj! { "name": name, "value": ?v });
    }
    Request::post(format!("{GRAPH}/v1.0/groupSettings"))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "values": values, "templateId": ?t.get("id").cloned() })
        .send()
        .await?;
    Ok(())
}

pub async fn siteclassification_get(cx: &mut Cx) -> Result<()> {
    let all = group_settings().await?;
    if all.is_empty() {
        return Err(Error::msg("Site classification is not enabled."));
    }
    let u0 = unified(&all);
    let Some(first) = u0.first() else {
        return Err(Error::msg(
            "Missing DirectorySettingTemplate for \"Group.Unified\"",
        ));
    };
    let find = |n: &str| {
        arr(first, "values")
            .iter()
            .find(|v| s(v, "name") == n)
            .cloned()
    };
    let classifications: Vec<Value> = find("ClassificationList")
        .map(|v| u(&v, "value").split(',').map(Value::from).collect())
        .unwrap_or_default();
    let value_of = |n: &str| find(n).and_then(|v| v.get("value").cloned());
    cx.log(obj! {
        "UsageGuidelinesUrl": ?value_of("UsageGuidelinesUrl").or_else(|| find("UsageGuidelinesUrl").is_none().then(|| Value::from(""))),
        "GuestUsageGuidelinesUrl": ?value_of("GuestUsageGuidelinesUrl").or_else(|| find("GuestUsageGuidelinesUrl").is_none().then(|| Value::from(""))),
        "Classifications": classifications,
        "DefaultClassification": ?value_of("DefaultClassification").or_else(|| find("DefaultClassification").is_none().then(|| Value::from("")))
    })
}

pub async fn siteclassification_set(cx: &mut Cx) -> Result<()> {
    let all = group_settings().await?;
    let u0 = unified(&all);
    let Some(first) = u0.first() else {
        return Err(Error::msg(
            "There is no previous defined site classification which can updated.",
        ));
    };
    let mut values = Vec::new();
    for d in arr(first, "values") {
        let current = d.get("value").cloned();
        let opt = match s(d, "name") {
            "ClassificationList" => cx.s("classifications"),
            "DefaultClassification" => cx.s("defaultClassification"),
            "UsageGuidelinesUrl" => cx.s("usageGuidelinesUrl"),
            "GuestUsageGuidelinesUrl" => cx.s("guestUsageGuidelinesUrl"),
            _ => None,
        };
        values.push(
            obj! { "name": ?d.get("name").cloned(), "value": ?opt.map(Value::from).or(current) },
        );
    }
    Request::patch(format!("{GRAPH}/v1.0/groupSettings/{}", u(first, "id")))
        .graph_json()
        .header("content-type", "application/json")
        .json(obj! { "values": values })
        .send()
        .await?;
    Ok(())
}

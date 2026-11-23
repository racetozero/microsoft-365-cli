//! `entra user *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

fn rand_index(n: usize) -> usize {
    let b = uuid::Uuid::new_v4();
    let x = u64::from_le_bytes(b.as_bytes()[..8].try_into().unwrap());
    (x % n as u64) as usize
}

fn generate_password() -> String {
    let sets = [
        "0123456789",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "-_@%$#*&",
    ];
    let all: String = sets.concat();
    let mut chars: Vec<char> = Vec::with_capacity(15);
    for i in 0..15 {
        // one character of each set first, then any
        let set: Vec<char> = sets
            .get(i)
            .filter(|_| i < 4)
            .copied()
            .unwrap_or(all.as_str())
            .chars()
            .collect();
        chars.push(set[rand_index(set.len())]);
    }
    for i in (1..chars.len()).rev() {
        let j = rand_index(i + 1);
        chars.swap(i, j);
    }
    chars.into_iter().collect()
}

pub async fn add(cx: &mut Cx) -> Result<()> {
    cx.verbose_log(&format!(
        "Adding user to AAD with displayName {} and userPrincipalName {}",
        cx.u("displayName"),
        cx.u("userName")
    ));
    let un = cx.u("userName");
    let password = cx.str("password").unwrap_or_else(generate_password);
    let mut body = obj! {
        "accountEnabled": cx.opt_bool("accountEnabled").unwrap_or(true),
        "displayName": ?cx.str("displayName"),
        "userPrincipalName": ?cx.str("userName"),
        "mailNickName": cx.str("mailNickname").unwrap_or_else(|| un.split('@').next().unwrap_or("").to_string()),
        "passwordProfile": obj! {
            "forceChangePasswordNextSignIn": cx.bool("forceChangePasswordNextSignIn"),
            "forceChangePasswordNextSignInWithMfa": cx.bool("forceChangePasswordNextSignInWithMfa"),
            "password": password.as_str()
        },
        "givenName": ?cx.str("firstName"),
        "surName": ?cx.str("lastName"),
        "usageLocation": ?cx.str("usageLocation"),
        "officeLocation": ?cx.str("officeLocation"),
        "jobTitle": ?cx.str("jobTitle"),
        "companyName": ?cx.str("companyName"),
        "department": ?cx.str("department"),
        "preferredLanguage": ?cx.str("preferredLanguage")
    };
    cx.add_unknown_options(&mut body);
    let pw = body
        .get("passwordProfile")
        .and_then(|p| p.get("password"))
        .cloned();
    let mut user = Request::post(format!("{GRAPH}/v1.0/users"))
        .graph_json()
        .json(body)
        .data()
        .await?;
    if let (Some(o), Some(pw)) = (user.as_object_mut(), pw) {
        o.insert("password".into(), pw);
    }
    if let Some(m) = cx.s("managerUserId").or_else(|| cx.s("managerUserName")) {
        Request::put(format!(
            "{GRAPH}/v1.0/users/{}/manager/$ref",
            u(&user, "id")
        ))
        .accept("application/json;odata.metadata=none")
        .json(obj! { "@odata.id": format!("{GRAPH}/v1.0/users/{m}") })
        .send()
        .await?;
    }
    cx.log(user)
}

pub async fn get(cx: &mut Cx) -> Result<()> {
    let mut id = cx.u("id");
    if let Some(n) = cx.s("userName") {
        id = eqp(&n);
    } else if let Some(e) = cx.s("email") {
        id = entra::user::id_by_email(&e).await?;
    }
    let mut q: Vec<String> = entra::select_query(cx.str("properties").as_deref())
        .into_iter()
        .collect();
    if cx.bool("withManager") {
        q.push("$expand=manager($select=displayName,userPrincipalName,id,mail)".into());
    }
    let q = qs(&q);
    let url = if id.starts_with("%24") {
        format!("{GRAPH}/v1.0/users('{id}'){q}")
    } else {
        format!("{GRAPH}/v1.0/users/{id}{q}")
    };
    let r = Request::get(url).graph_json().data().await?;
    cx.log(r)
}

pub async fn groupmembership_list(cx: &mut Cx) -> Result<()> {
    let mut id = cx.u("userId");
    if let Some(n) = cx.s("userName") {
        id = entra::user::id_by_upn(&n).await?;
    } else if let Some(e) = cx.s("userEmail") {
        id = entra::user::id_by_email(&e).await?;
    }
    let r = Request::post(format!("{GRAPH}/v1.0/users/{id}/getMemberGroups"))
        .graph_json()
        .json(obj! { "securityEnabledOnly": cx.bool("securityEnabledOnly") })
        .data()
        .await?;
    let groups: Vec<Value> = arr(&r, "value")
        .iter()
        .map(|x| obj! { "groupId": x.clone() })
        .collect();
    cx.log(Value::Array(groups))
}

pub async fn guest_add(cx: &mut Cx) -> Result<()> {
    let cc = cx
        .s("ccRecipients")
        .map(|c| vec![obj! { "emailAddress": obj! { "address": c.trim() } }]);
    let body = obj! {
        "invitedUserEmailAddress": ?cx.str("emailAddress"),
        "inviteRedirectUrl": cx.s("inviteRedirectUrl").unwrap_or_else(|| "https://myapplications.microsoft.com".into()),
        "invitedUserDisplayName": ?cx.str("displayName"),
        "sendInvitationMessage": ?cx.opt_bool("sendInvitationMessage"),
        "invitedUserMessageInfo": obj! {
            "customizedMessageBody": ?cx.str("welcomeMessage"),
            "messageLanguage": cx.s("messageLanguage").unwrap_or_else(|| "en-US".into()),
            "ccRecipients": ?cc
        }
    };
    let r = Request::post(format!("{GRAPH}/v1.0/invitations"))
        .graph_json()
        .json(body)
        .data()
        .await?;
    cx.log(r)
}

pub async fn hibp(cx: &mut Cx) -> Result<()> {
    let domain = cx
        .s("domain")
        .map(|d| format!("?domain={}", eqp(&d)))
        .unwrap_or_default();
    let r = Request::get(format!(
        "https://haveibeenpwned.com/api/v3/breachedaccount/{}{domain}",
        eqp(&cx.u("userName"))
    ))
    .accept("application/json")
    .header("hibp-api-key", cx.u("apiKey"))
    .anonymous()
    .json_response()
    .data()
    .await;
    match r {
        Ok(v) => cx.log(v),
        Err(e) if e.status() == Some(404) && (cx.debug() || cx.verbose()) => {
            cx.log(Value::from("No pwnage found"))
        }
        Err(e) => Err(e),
    }
}

pub fn license_ids_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(
        !to_js_string(v)
            .split(',')
            .any(|e| !m365_core::validation::is_valid_guid(e)),
    )
}

pub fn license_list_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    Ok(!o.get("userId").is_some_and(truthy) || !o.get("userName").is_some_and(truthy))
}

pub async fn license_add(cx: &mut Cx) -> Result<()> {
    let add: Vec<Value> = cx
        .u("ids")
        .split(',')
        .map(|x| obj! { "disabledPlans": Vec::<Value>::new(), "skuId": x })
        .collect();
    let who = cx
        .str("userId")
        .or_else(|| cx.str("userName"))
        .unwrap_or_else(|| "undefined".into());
    let r = Request::post(format!("{GRAPH}/v1.0/users/{}/assignLicense", eqp(&who)))
        .graph_json()
        .json(obj! { "addLicenses": add, "removeLicenses": Vec::<Value>::new() })
        .data()
        .await?;
    cx.log(r)
}

pub async fn license_list(cx: &mut Cx) -> Result<()> {
    let who = cx.s("userId").or_else(|| cx.s("userName"));
    cx.verbose_log(&format!(
        "Retrieving licenses from user: {}.",
        who.clone().unwrap_or_else(|| "current user".into())
    ));
    let url = match who {
        Some(w) => format!("{GRAPH}/v1.0/users/{}/licenseDetails", eqp(&w)),
        None => format!("{GRAPH}/v1.0/me/licenseDetails"),
    };
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn license_remove(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("userId")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Removing the licenses for the user '{who}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the licenses for the user '{who}'?"),
    )? {
        return Ok(());
    }
    let rm: Vec<Value> = cx.u("ids").split(',').map(Value::from).collect();
    Request::post(format!("{GRAPH}/v1.0/users/{}/assignLicense", eqp(&who)))
        .graph_json()
        .json(obj! { "addLicenses": Vec::<Value>::new(), "removeLicenses": rm })
        .send()
        .await?;
    Ok(())
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/users");
    if let Some(p) = cx.s("properties") {
        let all: Vec<&str> = p.split(',').collect();
        let exp: Vec<String> = all
            .iter()
            .filter(|i| i.contains('/'))
            .map(|x| {
                let parts: Vec<&str> = x.split('/').collect();
                format!("{}($select={})", parts[0], parts[1])
            })
            .collect();
        let exp = if exp.is_empty() {
            String::new()
        } else {
            format!("&$expand={}", exp.join(","))
        };
        let sel: Vec<&str> = all.iter().copied().filter(|i| !i.contains('/')).collect();
        url.push_str(&format!("?$select={}{exp}", sel.join(",")));
    }
    let mut filters = Vec::new();
    for (k, v) in ut::options::unknown_options(&cx.opts, cx.meta) {
        if v.is_boolean() {
            return Err(Error::msg(format!("Specify value for the {k} property")));
        }
        filters.push(format!("startsWith({k}, '{}')", eqp(&to_js_string(&v))));
    }
    if let Some(t) = cx.s("type") {
        filters.push(format!("userType eq '{t}'"));
    }
    if !filters.is_empty() {
        url.push_str(&format!(
            "{}$filter={}",
            if cx.s("properties").is_some() {
                "&"
            } else {
                "?"
            },
            filters.join(" and ")
        ));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub async fn password_validate(cx: &mut Cx) -> Result<()> {
    let r = Request::post(format!("{GRAPH}/beta/users/validatePassword"))
        .graph_json()
        .json(obj! { "password": ?cx.str("password") })
        .data()
        .await?;
    cx.log(r)
}

pub async fn recyclebinitem_clear(cx: &mut Cx) -> Result<()> {
    if !cx.proceed(
        "force",
        "Are you sure you want to permanently delete all deleted users?",
    )? {
        return Ok(());
    }
    let users = odata::get_all_items(&format!(
        "{GRAPH}/v1.0/directory/deletedItems/microsoft.graph.user?$select=id"
    ))
    .await?;
    cx.verbose_log(&format!(
        "Amount of users to permanently delete: {}",
        users.len()
    ));
    let reqs: Vec<Value> = users.iter().enumerate().map(|(i, x)| obj! { "id": i, "method": "DELETE", "url": format!("/directory/deletedItems/{}", u(x, "id")) }).collect();
    for (n, chunk) in reqs.chunks(20).enumerate() {
        cx.verbose_log(&format!(
            "Deleting users: {}/{}",
            n * 20 + chunk.len(),
            users.len()
        ));
        Request::post(format!("{GRAPH}/v1.0/$batch"))
            .accept("application/json")
            .header("content-type", "application/json")
            .json(obj! { "requests": chunk.to_vec() })
            .json_response()
            .send()
            .await?;
    }
    Ok(())
}

pub async fn recyclebinitem_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving users from the recycle bin...");
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/directory/deletedItems/microsoft.graph.user"
        ))
        .await?,
    ))
}

pub async fn recyclebinitem_remove(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to permanently delete the user with id {id}?"),
    )? {
        return Ok(());
    }
    cx.verbose_log(&format!(
        "Permanently deleting user with id {id} from Microsoft Entra ID"
    ));
    Request::delete(format!("{GRAPH}/v1.0/directory/deletedItems/{id}"))
        .send()
        .await?;
    Ok(())
}

pub async fn recyclebinitem_restore(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!(
        "Restoring user with id {id} from the recycle bin."
    ));
    let r = Request::post(format!("{GRAPH}/v1.0/directory/deletedItems/{id}/restore"))
        .header("content-type", "application/json;odata.metadata=none")
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

const AUTH_METHODS: &[&str] = &[
    "push",
    "oath",
    "voiceMobile",
    "voiceAlternateMobile",
    "voiceOffice",
    "sms",
    "none",
];
const METHODS_REGISTERED: &[&str] = &[
    "mobilePhone",
    "email",
    "fido2",
    "microsoftAuthenticatorPush",
    "softwareOneTimePasscode",
];

pub fn registrationdetails_field(i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    let allowed = if i == 2 {
        METHODS_REGISTERED
    } else {
        AUTH_METHODS
    };
    check(
        to_js_string(v)
            .split(',')
            .map(str::trim)
            .all(|m| allowed.contains(&m)),
    )
}

pub async fn registrationdetails_list(cx: &mut Cx) -> Result<()> {
    let mut upns: Vec<String> = Vec::new();
    if let Some(ids) = cx.s("userIds") {
        for id in ids.split(',').map(str::trim) {
            upns.push(entra::user::upn_by_id(id, false).await?);
        }
    }
    let mut q = Vec::new();
    if let Some(p) = cx.s("properties") {
        q.push(format!("$select={p}"));
    }
    let mut f: Vec<String> = Vec::new();
    for k in [
        "isAdmin",
        "isMfaCapable",
        "isMfaRegistered",
        "isPasswordlessCapable",
        "isSelfServicePasswordResetCapable",
        "isSelfServicePasswordResetEnabled",
        "isSelfServicePasswordResetRegistered",
        "isSystemPreferredAuthenticationMethodEnabled",
    ] {
        if let Some(b) = cx.opts.get(k) {
            f.push(format!("{k} eq {}", to_js_string(b)));
        }
    }
    if let Some(m) = cx.str("methodsRegistered").filter(|x| !x.is_empty()) {
        f.push(format!(
            "({})",
            m.split(',')
                .map(|x| format!("methodsRegistered/any(m:m eq '{}')", x.trim()))
                .collect::<Vec<_>>()
                .join(" or ")
        ));
    }
    if let Some(m) = cx
        .str("systemPreferredAuthenticationMethods")
        .filter(|x| !x.is_empty())
    {
        f.push(format!(
            "({})",
            m.split(',')
                .map(|x| format!(
                    "systemPreferredAuthenticationMethods/any(m:m eq '{}')",
                    x.trim()
                ))
                .collect::<Vec<_>>()
                .join(" or ")
        ));
    }
    let mut ups: Vec<String> = upns
        .iter()
        .map(|p| format!("userPrincipalName eq '{}'", eqp(p)))
        .collect();
    if let Some(p) = cx.s("userPrincipalNames") {
        ups.extend(
            p.split(',')
                .map(|x| format!("userPrincipalName eq '{}'", eqp(x.trim()))),
        );
    }
    if !ups.is_empty() {
        f.push(format!("({})", ups.join(" or ")));
    }
    if let Some(m) = cx
        .str("userPreferredMethodForSecondaryAuthentication")
        .filter(|x| !x.is_empty())
    {
        f.push(format!(
            "({})",
            m.split(',')
                .map(|x| format!(
                    "userPreferredMethodForSecondaryAuthentication eq '{}'",
                    x.trim()
                ))
                .collect::<Vec<_>>()
                .join(" or ")
        ));
    }
    if let Some(t) = cx.s("userType") {
        f.push(format!("userType eq '{t}'"));
    }
    if !f.is_empty() {
        q.push(format!("$filter={}", f.join(" and ")));
    }
    cx.log(Value::Array(
        odata::get_all_items(&format!(
            "{GRAPH}/v1.0/reports/authenticationMethods/userRegistrationDetails{}",
            qs(&q)
        ))
        .await?,
    ))
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("id")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Removing user '{who}'..."));
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove user '{who}'?"),
    )? {
        return Ok(());
    }
    Request::delete(format!("{GRAPH}/v1.0/users/{who}"))
        .graph_json()
        .send()
        .await?;
    Ok(())
}

pub async fn session_revoke(cx: &mut Cx) -> Result<()> {
    let who = cx
        .s("userId")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    if !cx.proceed(
        "force",
        &format!("This will revoke all sessions for the user '{who}', requiring the user to re-sign in from all devices. Are you sure?"),
    )? {
        return Ok(());
    }
    let ident = cx
        .str("userId")
        .or_else(|| cx.str("userName"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!(
        "Invalidating all the refresh tokens for user {ident}..."
    ));
    Request::post(format!(
        "{GRAPH}/v1.0/users('{}')/revokeSignInSessions",
        eqp(&ident)
    ))
    .graph_json()
    .json(obj! {})
    .send()
    .await?;
    Ok(())
}

pub fn set_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(truthy);
    Ok(match i {
        1 => {
            if !t("managerUserId") && !t("managerUserName") && !t("removeManager") {
                true
            } else {
                ["managerUserId", "managerUserName", "removeManager"]
                    .iter()
                    .filter(|k| o.contains_key(**k))
                    .count()
                    == 1
            }
        }
        2 => {
            !(!t("resetPassword")
                && ((t("currentPassword") && !t("newPassword"))
                    || (t("newPassword") && !t("currentPassword"))))
        }
        4 => !(t("resetPassword") && !t("newPassword")),
        5 => !(t("forceChangePasswordNextSignIn") && !t("resetPassword")),
        6 => !(t("forceChangePasswordNextSignInWithMfa") && !t("resetPassword")),
        _ => true,
    })
}

pub async fn set(cx: &mut Cx) -> Result<()> {
    let token = auth::default_access_token().unwrap_or_default();
    if cx.s("currentPassword").is_some() {
        if let Some(id) = cx.s("id")
            && id != auth::jwt::user_id(&token)
        {
            return Err(Error::msg(
                "You can only change your own password. Please use --id @meId to reference to your own userId",
            ));
        } else if let Some(n) = cx.s("userName")
            && cx.s("id").is_none()
            && n.to_lowercase() != auth::jwt::user_name(&token).to_lowercase()
        {
            return Err(Error::msg(
                "You can only change your own password. Please use --userName @meUserName to reference to your own user principal name",
            ));
        }
    }
    let who = cx
        .s("userName")
        .or_else(|| cx.s("id"))
        .unwrap_or_else(|| "undefined".into());
    cx.verbose_log(&format!("Updating user {who}"));
    let ident = cx.s("id").unwrap_or_else(|| cx.u("userName"));
    let url = format!("{GRAPH}/v1.0/users/{}", eqp(&ident));
    let mut body = obj! {
        "displayName": ?cx.str("displayName"),
        "givenName": ?cx.str("firstName"),
        "surname": ?cx.str("lastName"),
        "usageLocation": ?cx.str("usageLocation"),
        "officeLocation": ?cx.str("officeLocation"),
        "jobTitle": ?cx.str("jobTitle"),
        "companyName": ?cx.str("companyName"),
        "department": ?cx.str("department"),
        "preferredLanguage": ?cx.str("preferredLanguage"),
        "accountEnabled": ?cx.opt_bool("accountEnabled")
    };
    cx.add_unknown_options(&mut body);
    if cx.bool("resetPassword") {
        body.as_object_mut().unwrap().insert(
            "passwordProfile".into(),
            obj! { "forceChangePasswordNextSignIn": cx.bool("forceChangePasswordNextSignIn"), "forceChangePasswordNextSignInWithMfa": cx.bool("forceChangePasswordNextSignInWithMfa"), "password": ?cx.str("newPassword") },
        );
    }
    for (_, v) in body.as_object_mut().unwrap().iter_mut() {
        if let Value::String(s) = v
            && s.trim().is_empty()
        {
            *v = Value::Null;
        }
    }
    if !body.as_object().unwrap().is_empty() {
        cx.verbose_log(&format!("Setting the updated properties for user {who}"));
        Request::patch(url.clone())
            .accept("application/json")
            .json(body)
            .json_response()
            .send()
            .await?;
    }
    if cx.s("currentPassword").is_some() {
        cx.verbose_log(&format!("Changing password for user {who}"));
        Request::post(format!("{url}/changePassword"))
            .graph_json()
            .json(obj! { "currentPassword": ?cx.str("currentPassword"), "newPassword": ?cx.str("newPassword") })
            .send()
            .await?;
    }
    let user = cx
        .s("id")
        .or_else(|| cx.s("userName"))
        .unwrap_or_else(|| "undefined".into());
    if let Some(m) = cx.s("managerUserId").or_else(|| cx.s("managerUserName")) {
        cx.verbose_log(&format!("Updating the manager to {m}"));
        Request::put(format!("{GRAPH}/v1.0/users/{user}/manager/$ref"))
            .accept("application/json;odata.metadata=none")
            .json(obj! { "@odata.id": format!("{GRAPH}/v1.0/users/{m}") })
            .send()
            .await?;
    } else if cx.bool("removeManager") {
        cx.verbose_log("Removing the manager");
        Request::delete(format!("{GRAPH}/v1.0/users/{user}/manager/$ref"))
            .accept("application/json;odata.metadata=none")
            .send()
            .await?;
    }
    Ok(())
}

pub async fn signin_list(cx: &mut Cx) -> Result<()> {
    let mut filter = String::new();
    if cx.s("userName").is_some() || cx.s("userId").is_some() {
        filter = match cx.s("userId") {
            Some(i) => format!("?$filter=userId eq '{}'", eqp(&i)),
            None => format!("?$filter=userPrincipalName eq '{}'", eqp(&cx.u("userName"))),
        };
    }
    if cx.s("appId").is_some() || cx.s("appDisplayName").is_some() {
        filter.push_str(if filter.is_empty() {
            "?$filter="
        } else {
            " and "
        });
        filter.push_str(&match cx.s("appId") {
            Some(a) => format!("appId eq '{}'", eqp(&a)),
            None => format!("appDisplayName eq '{}'", eqp(&cx.u("appDisplayName"))),
        });
    }
    cx.log(Value::Array(
        odata::get_all_items(&format!("{GRAPH}/v1.0/auditLogs/signIns{filter}")).await?,
    ))
}

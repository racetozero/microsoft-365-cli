//! Connections and access tokens: a port of upstream's `src/Auth.ts`.
//!
//! The connection files have upstream's names and shapes
//! (`~/.cli-m365-connection.json`, `~/.cli-m365-all-connections.json`), so
//! the Node and Rust CLIs share a login. Tokens are requested from the
//! Microsoft identity platform directly; the refresh tokens of public-client
//! logins are kept in an MSAL-format cache (`~/.cli-m365-msal-cache.json`).

use crate::config::{self, names};
use crate::error::{Error, Result};
use crate::http::Request;
use crate::json::{Obj, stringify};
use crate::log;
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CloudType {
    #[default]
    Public,
    USGov,
    USGovHigh,
    USGovDoD,
    China,
}

impl CloudType {
    pub fn as_str(self) -> &'static str {
        match self {
            CloudType::Public => "Public",
            CloudType::USGov => "USGov",
            CloudType::USGovHigh => "USGovHigh",
            CloudType::USGovDoD => "USGovDoD",
            CloudType::China => "China",
        }
    }
    pub fn parse(s: &str) -> Option<CloudType> {
        Some(match s {
            "Public" => CloudType::Public,
            "USGov" => CloudType::USGov,
            "USGovHigh" => CloudType::USGovHigh,
            "USGovDoD" => CloudType::USGovDoD,
            "China" => CloudType::China,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AuthType {
    #[default]
    DeviceCode,
    Password,
    Certificate,
    Identity,
    FederatedIdentity,
    Browser,
    Secret,
}

impl AuthType {
    pub fn as_str(self) -> &'static str {
        match self {
            AuthType::DeviceCode => "deviceCode",
            AuthType::Password => "password",
            AuthType::Certificate => "certificate",
            AuthType::Identity => "identity",
            AuthType::FederatedIdentity => "federatedIdentity",
            AuthType::Browser => "browser",
            AuthType::Secret => "secret",
        }
    }
    pub fn parse(s: &str) -> Option<AuthType> {
        Some(match s {
            "deviceCode" => AuthType::DeviceCode,
            "password" => AuthType::Password,
            "certificate" => AuthType::Certificate,
            "identity" => AuthType::Identity,
            "federatedIdentity" => AuthType::FederatedIdentity,
            "browser" => AuthType::Browser,
            "secret" => AuthType::Secret,
            _ => return None,
        })
    }
    fn is_app(self) -> bool {
        matches!(
            self,
            AuthType::Certificate
                | AuthType::Secret
                | AuthType::Identity
                | AuthType::FederatedIdentity
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    #[serde(default)]
    pub expires_on: Value,
    #[serde(default)]
    pub access_token: String,
}

/// Mirrors upstream's `Connection` class; field order is the class's
/// declaration order, which `JSON.stringify` follows.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    #[serde(default)]
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub identity_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub identity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub identity_tenant_id: Option<String>,
    #[serde(default)]
    pub auth_type: AuthType,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub secret: Option<String>,
    #[serde(default)]
    pub certificate_type: u8,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub thumbprint: Option<String>,
    #[serde(default)]
    pub access_tokens: Obj,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub spo_tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub app_id: Option<String>,
    #[serde(default = "common")]
    pub tenant: String,
    #[serde(default)]
    pub cloud_type: CloudType,
}

fn common() -> String {
    "common".into()
}

pub const CERT_UNKNOWN: u8 = 0;
pub const CERT_BASE64: u8 = 1;
pub const CERT_BINARY: u8 = 2;

impl Default for Connection {
    fn default() -> Connection {
        Connection {
            active: false,
            name: None,
            identity_name: None,
            identity_id: None,
            identity_tenant_id: None,
            auth_type: AuthType::DeviceCode,
            user_name: None,
            password: None,
            secret: None,
            certificate_type: CERT_UNKNOWN,
            certificate: None,
            thumbprint: None,
            access_tokens: Obj::new(),
            spo_url: None,
            spo_tenant_id: None,
            app_id: None,
            tenant: "common".into(),
            cloud_type: CloudType::Public,
        }
    }
}

impl Connection {
    pub fn deactivate(&mut self) {
        *self = Connection {
            app_id: config::client_id(),
            tenant: config::tenant(),
            ..Connection::default()
        };
    }

    pub fn token(&self, resource: &str) -> Option<AccessToken> {
        self.access_tokens
            .get(resource)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

struct State {
    connection: Connection,
    restored: bool,
    all: Option<Vec<Value>>,
}

fn state() -> &'static Mutex<State> {
    static S: OnceLock<Mutex<State>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(State {
            connection: Connection::default(),
            restored: false,
            all: None,
        })
    })
}

pub fn connection() -> Connection {
    state().lock().unwrap().connection.clone()
}

pub fn with_connection<R>(f: impl FnOnce(&mut Connection) -> R) -> R {
    f(&mut state().lock().unwrap().connection)
}

pub fn cloud_type() -> CloudType {
    state().lock().unwrap().connection.cloud_type
}

pub fn connection_file() -> PathBuf {
    config::home().join(".cli-m365-connection.json")
}

pub fn all_connections_file() -> PathBuf {
    config::home().join(".cli-m365-all-connections.json")
}

pub fn msal_cache_file() -> PathBuf {
    config::home().join(".cli-m365-msal-cache.json")
}

/// `Auth.getEndpointForResource`.
pub fn endpoint_for_resource(resource: &str, cloud: CloudType) -> String {
    let map: &[(&str, &str)] = match cloud {
        CloudType::Public => &[],
        CloudType::USGov => &[
            ("https://graph.microsoft.com", "https://graph.microsoft.com"),
            (
                "https://management.azure.com/",
                "https://management.usgovcloudapi.net/",
            ),
            (
                "https://login.microsoftonline.com",
                "https://login.microsoftonline.com",
            ),
        ],
        CloudType::USGovHigh => &[
            ("https://graph.microsoft.com", "https://graph.microsoft.us"),
            (
                "https://management.azure.com/",
                "https://management.usgovcloudapi.net/",
            ),
            (
                "https://login.microsoftonline.com",
                "https://login.microsoftonline.us",
            ),
        ],
        CloudType::USGovDoD => &[
            (
                "https://graph.microsoft.com",
                "https://dod-graph.microsoft.us",
            ),
            (
                "https://management.azure.com/",
                "https://management.usgovcloudapi.net/",
            ),
            (
                "https://login.microsoftonline.com",
                "https://login.microsoftonline.us",
            ),
        ],
        CloudType::China => &[
            (
                "https://graph.microsoft.com",
                "https://microsoftgraph.chinacloudapi.cn",
            ),
            (
                "https://management.azure.com/",
                "https://management.chinacloudapi.cn",
            ),
            (
                "https://login.microsoftonline.com",
                "https://login.chinacloudapi.cn",
            ),
        ],
    };
    map.iter()
        .find(|(k, _)| *k == resource)
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| resource.to_string())
}

/// `auth.defaultResource`.
pub fn default_resource() -> String {
    endpoint_for_resource("https://graph.microsoft.com", cloud_type())
}

/// `Auth.getResourceFromUrl`.
pub fn resource_from_url(url: &str) -> String {
    let mut resource = url.to_string();
    if resource.len() > 8
        && let Some(pos) = resource[8..].find('/')
    {
        resource.truncate(pos + 8);
    }
    if resource == "https://api.bap.microsoft.com"
        || resource == "https://api.powerapps.com"
        || resource.ends_with(".api.bap.microsoft.com")
    {
        resource = "https://service.powerapps.com/".into();
    }
    if resource == "https://api.flow.microsoft.com" {
        resource = "https://management.azure.com/".into();
    }
    if resource == "https://api.powerbi.com" {
        resource = "https://analysis.windows.net/powerbi/api".into();
    }
    resource
}

/// `auth.restoreAuth()`.
pub fn restore() {
    let mut s = state().lock().unwrap();
    if s.restored || s.connection.active {
        return;
    }
    s.restored = true;
    if let Ok(text) = std::fs::read_to_string(connection_file())
        && let Ok(v) = serde_json::from_str::<Value>(&text)
        && let Ok(c) = serde_json::from_value::<Connection>(v)
    {
        s.connection = c;
    }
}

pub fn is_active() -> bool {
    state().lock().unwrap().connection.active
}

pub fn all_connections() -> Vec<Value> {
    let mut s = state().lock().unwrap();
    if s.all.is_none() {
        let all = std::fs::read_to_string(all_connections_file())
            .ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .and_then(|v| v.as_array().cloned())
            .unwrap_or_default();
        s.all = Some(all);
    }
    s.all.clone().unwrap_or_default()
}

fn write_private(path: &PathBuf, text: &str) -> std::io::Result<()> {
    std::fs::write(path, text)?;
    config::set_mode(path, 0o600);
    Ok(())
}

/// `auth.storeConnectionInfo()`.
pub fn store_connection_info() -> Result<()> {
    let conn = connection();
    let conn_value = serde_json::to_value(&conn)?;
    write_private(&connection_file(), &stringify(&conn_value))
        .map_err(|e| Error::msg(e.to_string()))?;
    let mut all = all_connections();
    if conn.active {
        all.retain(|c| {
            c.get("identityId").and_then(Value::as_str) != conn.identity_id.as_deref()
                || c.get("appId").and_then(Value::as_str) != conn.app_id.as_deref()
                || c.get("tenant").and_then(Value::as_str) != Some(conn.tenant.as_str())
        });
        for c in all.iter_mut() {
            if let Some(o) = c.as_object_mut() {
                o.insert("active".into(), Value::Bool(false));
            }
        }
        all.insert(0, conn_value);
    }
    state().lock().unwrap().all = Some(all.clone());
    write_private(&all_connections_file(), &stringify(&Value::Array(all)))
        .map_err(|e| Error::msg(e.to_string()))?;
    Ok(())
}

/// `auth.clearConnectionInfo()`.
pub fn clear_connection_info() -> Result<()> {
    for f in [connection_file(), all_connections_file()] {
        if f.exists() {
            std::fs::write(&f, "").map_err(|e| Error::msg(e.to_string()))?;
        }
    }
    let _ = std::fs::remove_file(msal_cache_file());
    let _ = std::fs::remove_file(config::home().join(".cli-m365-msal.json"));
    state().lock().unwrap().all = Some(Vec::new());
    Ok(())
}

/// `auth.removeConnectionInfo(connection)`.
pub fn remove_connection_info(c: &Value) -> Result<()> {
    let name = c.get("name").and_then(Value::as_str).map(str::to_string);
    let current = connection();
    let is_current = current.name == name;
    let mut all = all_connections();
    all.retain(|x| x.get("name").and_then(Value::as_str).map(str::to_string) != name);
    if !current.auth_type.is_app()
        && let Some(id) = c.get("identityId").and_then(Value::as_str)
    {
        msal::remove_account(id);
    }
    state().lock().unwrap().all = Some(all.clone());
    write_private(&all_connections_file(), &stringify(&Value::Array(all)))
        .map_err(|e| Error::msg(e.to_string()))?;
    if is_current {
        let f = connection_file();
        if f.exists() {
            let _ = std::fs::write(&f, "");
        }
        with_connection(|c| c.deactivate());
    }
    Ok(())
}

/// `auth.switchToConnection`.
pub fn switch_to_connection(c: &Value) -> Result<()> {
    let mut conn: Connection = serde_json::from_value(c.clone())?;
    conn.active = true;
    with_connection(|x| *x = conn);
    store_connection_info()
}

/// `auth.updateConnection`.
pub fn update_connection(old_name: &str, new_name: &str) -> Result<()> {
    let mut all = all_connections();
    if all
        .iter()
        .any(|c| c.get("name").and_then(Value::as_str) == Some(new_name))
    {
        return Err(Error::msg(format!(
            "The connection name '{new_name}' is already in use."
        )));
    }
    for c in all.iter_mut() {
        if c.get("name").and_then(Value::as_str) == Some(old_name)
            && let Some(o) = c.as_object_mut()
        {
            o.insert("name".into(), Value::from(new_name));
        }
    }
    state().lock().unwrap().all = Some(all);
    with_connection(|c| {
        if c.name.as_deref() == Some(old_name) {
            c.name = Some(new_name.to_string());
        }
    });
    store_connection_info()
}

/// `auth.getConnection(name)`.
pub fn get_connection(name: &str) -> Result<Value> {
    all_connections()
        .into_iter()
        .find(|c| c.get("name").and_then(Value::as_str) == Some(name))
        .ok_or_else(|| Error::msg(format!("The connection '{name}' cannot be found.")))
}

/// `auth.getConnectionDetails`.
pub fn connection_details(c: &Value) -> Value {
    let mut o = Obj::new();
    o.insert(
        "connectionName".into(),
        c.get("name").cloned().unwrap_or(Value::Null),
    );
    o.insert(
        "connectedAs".into(),
        c.get("identityName").cloned().unwrap_or(Value::Null),
    );
    o.insert(
        "authType".into(),
        c.get("authType")
            .cloned()
            .unwrap_or(Value::from("deviceCode")),
    );
    if let Some(a) = c.get("appId") {
        o.insert("appId".into(), a.clone());
    }
    o.insert(
        "appTenant".into(),
        c.get("tenant").cloned().unwrap_or(Value::from("common")),
    );
    o.insert(
        "cloudType".into(),
        c.get("cloudType").cloned().unwrap_or(Value::from("Public")),
    );
    Value::Object(o)
}

// ---- access tokens ----------------------------------------------------------

pub mod jwt {
    use super::*;

    pub fn payload(token: &str) -> Option<Value> {
        let chunks: Vec<&str> = token.split('.').collect();
        if chunks.len() != 3 {
            return None;
        }
        let p = chunks[1].trim_end_matches('=');
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(p)
            .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(p))
            .ok()?;
        serde_json::from_slice(&bytes).ok()
    }

    pub fn is_app_only(token: &str) -> Option<bool> {
        if token.is_empty() {
            return None;
        }
        payload(token).map(|p| p.get("idtyp").and_then(Value::as_str) == Some("app"))
    }

    pub fn tenant_id(token: &str) -> String {
        payload(token)
            .and_then(|p| p.get("tid").and_then(Value::as_str).map(str::to_string))
            .unwrap_or_default()
    }

    pub fn user_name(token: &str) -> String {
        payload(token)
            .and_then(|p| {
                p.get("upn")
                    .filter(|v| crate::json::truthy(v))
                    .or_else(|| p.get("app_displayname"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
            })
            .unwrap_or_default()
    }

    pub fn user_id(token: &str) -> String {
        payload(token)
            .and_then(|p| p.get("oid").and_then(Value::as_str).map(str::to_string))
            .unwrap_or_default()
    }

    pub fn scopes(token: &str) -> Vec<String> {
        payload(token)
            .and_then(|p| {
                p.get("scp")
                    .and_then(Value::as_str)
                    .map(|s| s.split(' ').map(str::to_string).collect())
            })
            .unwrap_or_default()
    }

    pub fn roles(token: &str) -> Vec<String> {
        payload(token)
            .and_then(|p| {
                p.get("roles").and_then(Value::as_array).map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(str::to_string))
                        .collect()
                })
            })
            .unwrap_or_default()
    }
}

/// The access token for the default (Graph) resource, if any.
pub fn default_access_token() -> Option<String> {
    let c = connection();
    c.token(&default_resource())
        .map(|t| t.access_token)
        .filter(|t| !t.is_empty())
}

/// `accessToken.assertAccessTokenType`.
pub fn assert_access_token_type(delegated: bool) -> Result<()> {
    let Some(token) = default_access_token() else {
        return Err(Error::msg("No access token found."));
    };
    let app = jwt::is_app_only(&token);
    if delegated && app == Some(true) {
        return Err(Error::msg("This command requires delegated permissions."));
    }
    if !delegated && app != Some(true) {
        return Err(Error::msg(
            "This command requires application-only permissions.",
        ));
    }
    Ok(())
}

fn expires_on(t: &AccessToken) -> chrono::DateTime<chrono::Utc> {
    match &t.expires_on {
        Value::String(s) => crate::time::parse_js_date(s).unwrap_or_default(),
        Value::Number(n) => {
            chrono::DateTime::from_timestamp_millis(n.as_i64().unwrap_or(0)).unwrap_or_default()
        }
        _ => chrono::DateTime::from_timestamp(0, 0).unwrap(),
    }
}

pub struct TokenResult {
    pub access_token: String,
    pub expires_on: chrono::DateTime<chrono::Utc>,
}

/// `auth.ensureAccessToken(resource, logger, debug, fetchNew)`.
pub async fn ensure_access_token(resource: &str, fetch_new: bool) -> Result<String> {
    let debug = log::debug();
    let conn = connection();
    let existing = conn.token(resource);
    #[cfg(feature = "parity")]
    let parity = std::env::var_os("M365_PARITY_MOCK").is_some();
    #[cfg(not(feature = "parity"))]
    let parity = false;
    if parity && let Some(t) = &existing {
        return Ok(t.access_token.clone());
    }
    let now = chrono::Utc::now();
    if let Some(t) = &existing
        && !fetch_new
        && expires_on(t) > now
    {
        if debug {
            log::log_stderr(&format!(
                "Existing access token {} still valid. Returning...",
                t.access_token
            ));
        }
        return Ok(t.access_token.clone());
    } else if debug {
        match &existing {
            None => log::log_stderr(&format!("No token found for resource {resource}.")),
            Some(t) => log::log_stderr(&format!(
                "Access token expired. Token: {}, ExpiresAt: {}",
                t.access_token,
                crate::json::to_js_string(&t.expires_on)
            )),
        }
    }

    let result = if parity {
        TokenResult {
            access_token: std::env::var("M365_PARITY_TOKEN")
                .unwrap_or_else(|_| "parity-mock-token".into()),
            expires_on: crate::time::parse_js_date("2099-01-01T00:00:00.000Z").unwrap_or_default(),
        }
    } else if !conn.auth_type.is_app()
        && conn.active
        && conn.identity_id.as_deref().is_some_and(msal::has_account)
    {
        flows::silent(&conn, resource).await?
    } else {
        match conn.auth_type {
            AuthType::DeviceCode => flows::device_code(&conn, resource).await?,
            AuthType::Password => flows::password(&conn, resource).await?,
            AuthType::Certificate => flows::certificate(resource).await?,
            AuthType::Identity => flows::identity(&conn, resource).await?,
            AuthType::FederatedIdentity => flows::federated(resource).await?,
            AuthType::Browser => flows::browser(&conn, resource).await?,
            AuthType::Secret => flows::secret(&conn, resource).await?,
        }
    };
    if debug {
        log::log_stderr("Response");
        log::log_stderr(&format!(
            "{{\n  accessToken: '{}',\n  expiresOn: {}\n}}",
            result.access_token,
            crate::time::to_iso(result.expires_on)
        ));
        log::log_stderr("");
    }
    with_connection(|c| {
        let mut t = Obj::new();
        t.insert(
            "expiresOn".into(),
            Value::from(crate::time::to_iso(result.expires_on)),
        );
        t.insert(
            "accessToken".into(),
            Value::from(result.access_token.as_str()),
        );
        c.access_tokens
            .insert(resource.to_string(), Value::Object(t));
        c.active = true;
        c.identity_name = Some(jwt::user_name(&result.access_token));
        let oid = jwt::user_id(&result.access_token);
        c.identity_id = if oid.is_empty() {
            c.app_id.clone()
        } else {
            Some(oid)
        };
        c.identity_tenant_id = Some(jwt::tenant_id(&result.access_token));
        if c.name.is_none() {
            c.name = c.identity_id.clone();
        }
    });
    if let Err(e) = store_connection_info()
        && debug
    {
        log::log_stderr(&e.js_message());
    }
    Ok(result.access_token)
}

pub fn login_endpoint(cloud: CloudType) -> String {
    endpoint_for_resource("https://login.microsoftonline.com", cloud)
}

pub mod msal {
    //! A minimal MSAL-compatible token cache holding accounts and refresh tokens.
    use super::*;

    fn load() -> Obj {
        std::fs::read_to_string(msal_cache_file())
            .ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default()
    }

    fn save(cache: &Obj) {
        let _ = write_private(
            &msal_cache_file(),
            &stringify(&Value::Object(cache.clone())),
        );
    }

    pub fn has_account(local_id: &str) -> bool {
        account(local_id).is_some()
    }

    pub fn account(local_id: &str) -> Option<Value> {
        let cache = load();
        cache
            .get("Account")
            .and_then(Value::as_object)?
            .values()
            .find(|a| a.get("local_account_id").and_then(Value::as_str) == Some(local_id))
            .cloned()
    }

    pub fn refresh_token(home_account_id: &str, client_id: &str) -> Option<String> {
        let cache = load();
        cache
            .get("RefreshToken")
            .and_then(Value::as_object)?
            .values()
            .find_map(|t| {
                (t.get("home_account_id").and_then(Value::as_str) == Some(home_account_id)
                    && t.get("client_id").and_then(Value::as_str) == Some(client_id))
                .then(|| t.get("secret").and_then(Value::as_str).map(str::to_string))
                .flatten()
            })
    }

    fn environment(cloud: CloudType) -> &'static str {
        match cloud {
            CloudType::Public | CloudType::USGov => "login.windows.net",
            CloudType::USGovHigh | CloudType::USGovDoD => "login.microsoftonline.us",
            CloudType::China => "login.chinacloudapi.cn",
        }
    }

    /// Stores the account and refresh token of a token response.
    pub fn store(conn: &Connection, resp: &Value) {
        let Some(client_id) = conn.app_id.as_deref() else {
            return;
        };
        let env = environment(conn.cloud_type);
        let id_claims = resp
            .get("id_token")
            .and_then(Value::as_str)
            .and_then(jwt::payload)
            .unwrap_or(Value::Null);
        let client_info = resp
            .get("client_info")
            .and_then(Value::as_str)
            .and_then(|ci| {
                base64::engine::general_purpose::URL_SAFE_NO_PAD
                    .decode(ci.trim_end_matches('='))
                    .ok()
            })
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .unwrap_or(Value::Null);
        let oid = id_claims
            .get("oid")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let tid = id_claims
            .get("tid")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let home = match (
            client_info.get("uid").and_then(Value::as_str),
            client_info.get("utid").and_then(Value::as_str),
        ) {
            (Some(u), Some(t)) => format!("{u}.{t}"),
            _ => format!("{oid}.{tid}"),
        };
        let mut cache = load();
        let entry = |cache: &mut Obj, section: &str, key: String, v: Value| {
            let s = cache
                .entry(section.to_string())
                .or_insert_with(|| Value::Object(Obj::new()));
            if let Some(o) = s.as_object_mut() {
                o.insert(key, v);
            }
        };
        if !oid.is_empty() {
            let username = id_claims
                .get("preferred_username")
                .and_then(Value::as_str)
                .unwrap_or("");
            entry(
                &mut cache,
                "Account",
                format!("{home}-{env}-{tid}").to_lowercase(),
                serde_json::json!({
                    "home_account_id": home, "environment": env, "realm": tid, "local_account_id": oid,
                    "username": username, "authority_type": "MSSTS",
                    "name": id_claims.get("name").cloned().unwrap_or(Value::from("")),
                    "client_info": resp.get("client_info").cloned().unwrap_or(Value::from(""))
                }),
            );
        }
        if let Some(rt) = resp.get("refresh_token").and_then(Value::as_str) {
            entry(
                &mut cache,
                "RefreshToken",
                format!("{home}-{env}-refreshtoken-{client_id}--").to_lowercase(),
                serde_json::json!({ "home_account_id": home, "environment": env, "credential_type": "RefreshToken", "client_id": client_id, "secret": rt }),
            );
        }
        entry(
            &mut cache,
            "AppMetadata",
            format!("appmetadata-{env}-{client_id}"),
            serde_json::json!({ "client_id": client_id, "environment": env }),
        );
        save(&cache);
    }

    pub fn remove_account(local_id: &str) {
        let mut cache = load();
        let Some(acc) = account(local_id) else { return };
        let home = acc
            .get("home_account_id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        for section in ["Account", "RefreshToken", "AccessToken", "IdToken"] {
            if let Some(Value::Object(o)) = cache.get_mut(section) {
                o.retain(|_, v| {
                    v.get("home_account_id").and_then(Value::as_str) != Some(home.as_str())
                });
            }
        }
        save(&cache);
    }
}

mod flows {
    use super::*;

    fn token_url(conn: &Connection) -> String {
        format!(
            "{}/{}/oauth2/v2.0/token",
            login_endpoint(conn.cloud_type),
            conn.tenant
        )
    }

    fn form(pairs: &[(&str, &str)]) -> String {
        pairs
            .iter()
            .map(|(k, v)| format!("{}={}", enc(k), enc(v)))
            .collect::<Vec<_>>()
            .join("&")
    }

    fn enc(s: &str) -> String {
        percent_encoding::utf8_percent_encode(s, crate::util::URI_COMPONENT).to_string()
    }

    fn scopes(resource: &str, public: bool) -> String {
        if public {
            format!("{resource}/.default openid profile offline_access")
        } else {
            format!("{resource}/.default")
        }
    }

    async fn post_token(url: &str, body: String) -> Result<Value> {
        let res = Request::post(url)
            .anonymous()
            .header("content-type", "application/x-www-form-urlencoded")
            .accept("application/json")
            .text(body)
            .json_response()
            .send()
            .await;
        match res {
            Ok(r) => Ok(r.data()),
            Err(e) => {
                let body = e.body().cloned();
                match body {
                    Some(b) if b.get("error").is_some() => Err(Error::msg(
                        b.get("error_description")
                            .and_then(Value::as_str)
                            .or_else(|| b.get("error").and_then(Value::as_str))
                            .unwrap_or("")
                            .to_string(),
                    )),
                    _ => Err(e.plain()),
                }
            }
        }
    }

    fn result(resp: &Value) -> Result<TokenResult> {
        let at = resp
            .get("access_token")
            .and_then(Value::as_str)
            .ok_or_else(|| Error::msg("Failed to retrieve an access token. Please try again."))?;
        let expires_in = match resp.get("expires_in") {
            Some(Value::Number(n)) => n.as_i64().unwrap_or(3600),
            Some(Value::String(s)) => s.parse().unwrap_or(3600),
            _ => 3600,
        };
        Ok(TokenResult {
            access_token: at.to_string(),
            expires_on: chrono::Utc::now() + chrono::Duration::seconds(expires_in),
        })
    }

    fn public_tenant(conn: &Connection) -> Connection {
        let mut c = conn.clone();
        if c.auth_type == AuthType::Password && c.tenant == "common" {
            c.tenant = "organizations".into();
            with_connection(|x| x.tenant = "organizations".into());
        }
        c
    }

    fn app_id(conn: &Connection) -> Result<String> {
        conn.app_id.clone().ok_or_else(|| Error::msg("appId is required. TIP: use the \"m365 setup\" command to configure the default appId."))
    }

    pub async fn silent(conn: &Connection, resource: &str) -> Result<TokenResult> {
        if log::debug() {
            log::log_stderr("Retrieving new access token silently");
        }
        let conn = public_tenant(conn);
        let client_id = app_id(&conn)?;
        let acc = msal::account(conn.identity_id.as_deref().unwrap_or(""))
            .ok_or_else(|| Error::msg("account not found"))?;
        let home = acc
            .get("home_account_id")
            .and_then(Value::as_str)
            .unwrap_or("");
        let rt = msal::refresh_token(home, &client_id)
            .ok_or_else(|| Error::msg("No refresh token found. Sign in again."))?;
        let sc = scopes(resource, true);
        let body = form(&[
            ("client_id", &client_id),
            ("grant_type", "refresh_token"),
            ("refresh_token", &rt),
            ("scope", &sc),
            ("client_info", "1"),
        ]);
        let resp = post_token(&token_url(&conn), body).await?;
        msal::store(&conn, &resp);
        result(&resp)
    }

    pub async fn device_code(conn: &Connection, resource: &str) -> Result<TokenResult> {
        if log::debug() {
            log::log_stderr(&format!(
                "Starting Auth.ensureAccessTokenWithDeviceCode. resource: {resource}, debug: true"
            ));
        }
        let client_id = app_id(conn)?;
        let sc = scopes(resource, true);
        let url = format!(
            "{}/{}/oauth2/v2.0/devicecode",
            login_endpoint(conn.cloud_type),
            conn.tenant
        );
        let dc = post_token(&url, form(&[("client_id", &client_id), ("scope", &sc)])).await?;
        if log::debug() {
            log::log_stderr("Response:");
            log::log_stderr(&crate::json::stringify_pretty(&dc, "  "));
            log::log_stderr("");
        }
        if let Some(m) = dc.get("message").and_then(Value::as_str) {
            log::log_stderr(&format!("🌶️  {m}"));
        }
        if config::get_bool(names::AUTO_OPEN_LINKS_IN_BROWSER, false)
            && let Some(u) = dc.get("verification_uri").and_then(Value::as_str)
        {
            let _ = crate::util::open_browser(u);
        }
        if config::get_bool(names::COPY_DEVICE_CODE_TO_CLIPBOARD, false)
            && let Some(code) = dc.get("user_code").and_then(Value::as_str)
        {
            crate::util::copy_to_clipboard(code);
        }
        let device_code = dc
            .get("device_code")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let mut interval = dc.get("interval").and_then(Value::as_u64).unwrap_or(5);
        let expires = std::time::Instant::now()
            + std::time::Duration::from_secs(
                dc.get("expires_in").and_then(Value::as_u64).unwrap_or(900),
            );
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(interval)).await;
            let body = form(&[
                ("client_id", &client_id),
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("device_code", &device_code),
                ("client_info", "1"),
            ]);
            let res = Request::post(token_url(conn))
                .anonymous()
                .header("content-type", "application/x-www-form-urlencoded")
                .text(body)
                .json_response()
                .send()
                .await;
            match res {
                Ok(r) => {
                    let resp = r.data();
                    msal::store(conn, &resp);
                    return result(&resp);
                }
                Err(e) => {
                    let code = e
                        .body()
                        .and_then(|b| b.get("error"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                    match code.as_str() {
                        "authorization_pending" if std::time::Instant::now() < expires => continue,
                        "slow_down" if std::time::Instant::now() < expires => interval += 5,
                        _ => {
                            let d = e
                                .body()
                                .and_then(|b| b.get("error_description"))
                                .and_then(Value::as_str)
                                .map(str::to_string);
                            return Err(Error::msg(d.unwrap_or_else(|| e.js_message())));
                        }
                    }
                }
            }
        }
    }

    pub async fn password(conn: &Connection, resource: &str) -> Result<TokenResult> {
        if log::debug() {
            log::log_stderr("Retrieving new access token using credentials...");
        }
        let conn = public_tenant(conn);
        let client_id = app_id(&conn)?;
        let sc = scopes(resource, true);
        let body = form(&[
            ("client_id", &client_id),
            ("grant_type", "password"),
            ("username", conn.user_name.as_deref().unwrap_or("")),
            ("password", conn.password.as_deref().unwrap_or("")),
            ("scope", &sc),
            ("client_info", "1"),
        ]);
        let resp = post_token(&token_url(&conn), body).await?;
        msal::store(&conn, &resp);
        result(&resp)
    }

    pub async fn browser(conn: &Connection, resource: &str) -> Result<TokenResult> {
        if log::debug() {
            log::log_stderr("Retrieving new access token using interactive browser session...");
        }
        let client_id = app_id(conn)?;
        let (code, redirect) = crate::auth_server::retrieve_code(conn, resource).await?;
        if log::debug() {
            log::log_stderr(&format!("The service returned the code '{code}'"));
        }
        let sc = scopes(resource, true);
        let body = form(&[
            ("client_id", &client_id),
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", &redirect),
            ("scope", &sc),
            ("client_info", "1"),
        ]);
        let resp = post_token(&token_url(conn), body).await?;
        msal::store(conn, &resp);
        result(&resp)
    }

    pub async fn secret(conn: &Connection, resource: &str) -> Result<TokenResult> {
        let client_id = app_id(conn)?;
        let sc = scopes(resource, false);
        let body = form(&[
            ("client_id", &client_id),
            ("grant_type", "client_credentials"),
            ("client_secret", conn.secret.as_deref().unwrap_or("")),
            ("scope", &sc),
        ]);
        result(&post_token(&token_url(conn), body).await?)
    }

    pub async fn certificate(resource: &str) -> Result<TokenResult> {
        if log::debug() {
            log::log_stderr("Retrieving new access token using certificate...");
        }
        let conn = connection();
        let (key_pem, thumbprint, cert_type) = crate::cert::load(&conn)?;
        with_connection(|c| {
            c.thumbprint = Some(thumbprint.clone());
            c.certificate_type = cert_type;
        });
        let conn = connection();
        let client_id = app_id(&conn)?;
        let url = token_url(&conn);
        let assertion = crate::cert::client_assertion(&key_pem, &thumbprint, &client_id, &url)?;
        let sc = scopes(resource, false);
        let body = form(&[
            ("client_id", &client_id),
            ("grant_type", "client_credentials"),
            (
                "client_assertion_type",
                "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
            ),
            ("client_assertion", &assertion),
            ("scope", &sc),
        ]);
        result(&post_token(&url, body).await?)
    }

    pub async fn identity(conn: &Connection, resource: &str) -> Result<TokenResult> {
        let debug = log::debug();
        let user = conn.user_name.clone();
        if debug {
            log::log_stderr("Will try to retrieve access token using identity...");
        }
        let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let res_enc = enc(resource);
        let mut extra_header: Option<String> = None;
        let mut url = if let (Some(ep), Some(h)) =
            (env("IDENTITY_ENDPOINT"), env("IDENTITY_HEADER"))
        {
            if debug {
                log::log_stderr(
                    "IDENTITY_ENDPOINT and IDENTITY_HEADER env variables found it is Azure Function, WebApp...",
                );
            }
            extra_header = Some(h);
            format!("{ep}?resource={res_enc}&api-version=2019-08-01")
        } else if let (Some(ep), Some(s)) = (env("MSI_ENDPOINT"), env("MSI_SECRET")) {
            if debug {
                log::log_stderr(
                    "MSI_ENDPOINT and MSI_SECRET env variables found it is Azure Function or WebApp, but using the old names of the env variables...",
                );
            }
            extra_header = Some(s);
            format!("{ep}?resource={res_enc}&api-version=2019-08-01")
        } else if let Some(ep) = env("IDENTITY_ENDPOINT") {
            if debug {
                log::log_stderr("IDENTITY_ENDPOINT env variable found it is Azure Could Shell...");
            }
            if user.is_some() && env("ACC_CLOUD").is_some() {
                return Err(Error::msg(
                    "Azure Cloud Shell does not support user-managed identity. You can execute the command without the --userName option to login with user identity",
                ));
            }
            format!("{ep}?resource={res_enc}")
        } else if let Some(ep) = env("MSI_ENDPOINT") {
            if debug {
                log::log_stderr(
                    "MSI_ENDPOINT env variable found it is Azure Could Shell, but using the old names of the env variables...",
                );
            }
            if user.is_some() && env("ACC_CLOUD").is_some() {
                return Err(Error::msg(
                    "Azure Cloud Shell does not support user-managed identity. You can execute the command without the --userName option to login with user identity",
                ));
            }
            format!("{ep}?resource={res_enc}")
        } else {
            if debug {
                log::log_stderr(
                    "IDENTITY_ENDPOINT and MSI_ENDPOINT env variables not found. Attempt to get Managed Identity token by using the Azure Virtual Machine API...",
                );
            }
            format!(
                "http://169.254.169.254/metadata/identity/oauth2/token?resource={res_enc}&api-version=2018-02-01"
            )
        };
        if let Some(u) = &user {
            url.push_str(&format!("&client_id={}", enc(u)));
            if debug {
                log::log_stderr("Wil try to get token using client_id param...");
            }
        }
        let build = |url: &str| {
            let mut r = Request::get(url)
                .anonymous()
                .accept("application/json")
                .header("Metadata", "true")
                .json_response();
            if let Some(h) = &extra_header {
                r = r.header("X-IDENTITY-HEADER", h.clone());
            }
            r
        };
        let parse = |v: Value| -> TokenResult {
            let exp = match v.get("expires_on") {
                Some(Value::String(s)) => s.parse::<i64>().unwrap_or(0),
                Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
                _ => 0,
            };
            TokenResult {
                access_token: v
                    .get("access_token")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string(),
                expires_on: chrono::DateTime::from_timestamp(exp, 0).unwrap_or_default(),
            }
        };
        match build(&url).data().await {
            Ok(v) => Ok(parse(v)),
            Err(e) => {
                if user.is_none() {
                    return Err(e.odata_json());
                }
                let body = e.body().cloned().unwrap_or(Value::Null);
                let not_found = if let Some(m) = body.get("Message").and_then(Value::as_str) {
                    m.contains("No Managed Identity found")
                } else {
                    body.get("error_description").and_then(Value::as_str)
                        == Some("Identity not found")
                };
                if !not_found {
                    return Err(e.odata_json());
                }
                if debug {
                    log::log_stderr(
                        "Wil try to get token using principal_id (also known as object_id) param ...",
                    );
                }
                let url = url.replace("&client_id=", "&principal_id=");
                match build(&url).data().await {
                    Ok(v) => Ok(parse(v)),
                    Err(Error::Network(m))
                        if m.contains("EACCES") || m.contains("Permission denied") =>
                    {
                        Err(Error::msg(
                            "Error while logging with Managed Identity. Please check if a Managed Identity is assigned to the current Azure resource.",
                        ))
                    }
                    Err(e) => Err(e.odata_json()),
                }
            }
        }
    }

    pub async fn federated(resource: &str) -> Result<TokenResult> {
        let debug = log::debug();
        if debug {
            log::log_stderr("Trying to retrieve access token using federated identity...");
        }
        let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let token = if let (Some(url), Some(tok)) = (
            env("ACTIONS_ID_TOKEN_REQUEST_URL"),
            env("ACTIONS_ID_TOKEN_REQUEST_TOKEN"),
        ) {
            if debug {
                log::log_stderr(
                    "ACTIONS_ID_TOKEN_REQUEST_URL and ACTIONS_ID_TOKEN_REQUEST_TOKEN env variables found. The context is GitHub Actions...",
                );
                log::log_stderr("Retrieving GitHub federation token...");
            }
            let v = Request::get(format!(
                "{url}&audience={}",
                enc("api://AzureADTokenExchange")
            ))
            .header("Authorization", format!("Bearer {tok}"))
            .header("Accept", "application/json")
            .anonymous()
            .json_response()
            .data()
            .await
            .map_err(Error::odata_json)?;
            v.get("value")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        } else if let Some(uri) = env("SYSTEM_OIDCREQUESTURI") {
            if debug {
                log::log_stderr(
                    "SYSTEM_OIDCREQUESTURI env variable found. The context is Azure DevOps...",
                );
            }
            let Some(sat) = env("SYSTEM_ACCESSTOKEN") else {
                return Err(Error::msg(
                    "The SYSTEM_ACCESSTOKEN environment variable is not available. Please check the Azure DevOps pipeline task configuration. It should contain 'SYSTEM_ACCESSTOKEN: $(System.AccessToken)' in the env section.",
                ));
            };
            let sc_id = env("AZURESUBSCRIPTION_SERVICE_CONNECTION_ID");
            let sc_app = env("AZURESUBSCRIPTION_CLIENT_ID");
            let sc_tenant = env("AZURESUBSCRIPTION_TENANT_ID");
            match (&sc_id, &sc_app, &sc_tenant) {
                (Some(id), Some(app), Some(tenant)) => {
                    if debug {
                        let c = connection();
                        if c.app_id.is_some() || c.tenant != "common" {
                            log::log_stderr(
                                "When using a service connection, the appId and tenant values are updated to the values of the service connection.",
                            );
                        }
                        log::log_stderr(&format!(
                            "Using service connection '{id}' with app Id '{app}' and tenant Id '{tenant}'..."
                        ));
                    }
                    with_connection(|c| {
                        c.app_id = Some(app.clone());
                        c.tenant = tenant.clone();
                    });
                }
                _ => {
                    if debug {
                        log::log_stderr(
                            "Not using a service connection. Run this command in an AzurePowerShell task to be able to use a service connection.",
                        );
                    }
                    let c = connection();
                    if c.app_id.is_none() || c.tenant == "common" {
                        return Err(Error::msg(
                            "The appId and tenant parameters are required when not using a service connection.",
                        ));
                    }
                }
            }
            if debug {
                log::log_stderr("Retrieving Azure DevOps federation token...");
            }
            let suffix = sc_id
                .map(|s| format!("&serviceConnectionId={s}"))
                .unwrap_or_default();
            let v = Request::post(format!("{uri}?api-version=7.1{suffix}"))
                .header("Authorization", format!("Bearer {sat}"))
                .header("Accept", "application/json")
                .header("Content-Type", "application/json")
                .anonymous()
                .json_response()
                .data()
                .await
                .map_err(Error::odata_json)?;
            v.get("oidcToken")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string()
        } else if env("GITLAB_CI").is_some() {
            if debug {
                log::log_stderr("GITLAB_CI env variable found. The context is GitLab CI/CD...");
            }
            let Some(t) = env("GITLAB_OIDC_TOKEN") else {
                return Err(Error::msg(
                    "The `GITLAB_OIDC_TOKEN` environment variable is not available. Please add an id_tokens entry named GITLAB_OIDC_TOKEN with audience `api://AzureADTokenExchange` to the job.",
                ));
            };
            let c = connection();
            if c.app_id.is_none() || c.tenant == "common" {
                return Err(Error::msg(
                    "The appId and tenant parameters are required when using federated identity in GitLab CI/CD.",
                ));
            }
            t
        } else {
            return Err(Error::msg(
                "Federated identity is currently only supported in GitHub Actions, Azure DevOps and GitLab.",
            ));
        };
        if debug {
            log::log_stderr("Retrieving Entra ID Access Token with federated token...");
        }
        let c = connection();
        let body = [
            "grant_type=client_credentials".to_string(),
            format!("scope={}", enc(&format!("{resource}/.default"))),
            format!("client_id={}", c.app_id.clone().unwrap_or_default()),
            format!(
                "client_assertion_type={}",
                enc("urn:ietf:params:oauth:client-assertion-type:jwt-bearer")
            ),
            format!("client_assertion={token}"),
        ]
        .join("&");
        let v = Request::post(format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            c.tenant
        ))
        .accept("application/json")
        .anonymous()
        .header("Content-Type", "application/x-www-form-urlencoded")
        .text(body)
        .json_response()
        .data()
        .await
        .map_err(Error::odata_json)?;
        result(&v)
    }
}

//! The configuration file upstream keeps with `configstore`:
//! `$XDG_CONFIG_HOME/configstore/cli-m365-config.json` (default
//! `~/.config/configstore/cli-m365-config.json`), tab-indented JSON.

use crate::json::{Obj, stringify_pretty};
use serde_json::Value;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub mod names {
    pub const AUTH_TYPE: &str = "authType";
    pub const AUTO_OPEN_LINKS_IN_BROWSER: &str = "autoOpenLinksInBrowser";
    pub const CLIENT_ID: &str = "clientId";
    pub const CLIENT_SECRET: &str = "clientSecret";
    pub const CLIENT_CERTIFICATE_FILE: &str = "clientCertificateFile";
    pub const CLIENT_CERTIFICATE_BASE64_ENCODED: &str = "clientCertificateBase64Encoded";
    pub const CLIENT_CERTIFICATE_PASSWORD: &str = "clientCertificatePassword";
    pub const COPY_DEVICE_CODE_TO_CLIPBOARD: &str = "copyDeviceCodeToClipboard";
    pub const CSV_ESCAPE: &str = "csvEscape";
    pub const CSV_HEADER: &str = "csvHeader";
    pub const CSV_QUOTE: &str = "csvQuote";
    pub const CSV_QUOTED: &str = "csvQuoted";
    pub const CSV_QUOTED_EMPTY: &str = "csvQuotedEmpty";
    pub const DISABLE_TELEMETRY: &str = "disableTelemetry";
    pub const ERROR_OUTPUT: &str = "errorOutput";
    pub const HELP_MODE: &str = "helpMode";
    pub const HELP_TARGET: &str = "helpTarget";
    pub const OUTPUT: &str = "output";
    pub const PRINT_ERRORS_AS_PLAIN_TEXT: &str = "printErrorsAsPlainText";
    pub const PROMPT: &str = "prompt";
    pub const PROMPT_LIST_PAGE_SIZE: &str = "promptListPageSize";
    pub const SHOW_HELP_ON_FAILURE: &str = "showHelpOnFailure";
    pub const TENANT_ID: &str = "tenantId";

    pub const ALL: &[&str] = &[
        AUTH_TYPE,
        AUTO_OPEN_LINKS_IN_BROWSER,
        CLIENT_ID,
        CLIENT_SECRET,
        CLIENT_CERTIFICATE_FILE,
        CLIENT_CERTIFICATE_BASE64_ENCODED,
        CLIENT_CERTIFICATE_PASSWORD,
        COPY_DEVICE_CODE_TO_CLIPBOARD,
        CSV_ESCAPE,
        CSV_HEADER,
        CSV_QUOTE,
        CSV_QUOTED,
        CSV_QUOTED_EMPTY,
        DISABLE_TELEMETRY,
        ERROR_OUTPUT,
        HELP_MODE,
        HELP_TARGET,
        OUTPUT,
        PRINT_ERRORS_AS_PLAIN_TEXT,
        PROMPT,
        PROMPT_LIST_PAGE_SIZE,
        SHOW_HELP_ON_FAILURE,
        TENANT_ID,
    ];
}

pub fn home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

pub fn path() -> PathBuf {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home().join(".config"));
    base.join("configstore").join("cli-m365-config.json")
}

fn store() -> &'static Mutex<Obj> {
    static STORE: OnceLock<Mutex<Obj>> = OnceLock::new();
    STORE.get_or_init(|| {
        let v = std::fs::read_to_string(path())
            .ok()
            .and_then(|s| serde_json::from_str::<Value>(&s).ok())
            .and_then(|v| v.as_object().cloned())
            .unwrap_or_default();
        Mutex::new(v)
    })
}

fn save(map: &Obj) {
    let p = path();
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
        set_mode(dir, 0o700);
    }
    let text = stringify_pretty(&Value::Object(map.clone()), "\t");
    if std::fs::write(&p, text).is_ok() {
        set_mode(&p, 0o600);
    }
}

#[cfg(unix)]
pub fn set_mode(p: &std::path::Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(p, std::fs::Permissions::from_mode(mode));
}

#[cfg(not(unix))]
pub fn set_mode(_p: &std::path::Path, _mode: u32) {}

pub fn get(key: &str) -> Option<Value> {
    store().lock().unwrap().get(key).cloned()
}

pub fn all() -> Obj {
    store().lock().unwrap().clone()
}

pub fn set(key: &str, value: Value) {
    let mut s = store().lock().unwrap();
    s.insert(key.to_string(), value);
    save(&s);
}

pub fn delete(key: &str) {
    let mut s = store().lock().unwrap();
    s.shift_remove(key);
    save(&s);
}

pub fn clear() {
    let mut s = store().lock().unwrap();
    s.clear();
    save(&s);
}

pub fn get_bool(key: &str, default: bool) -> bool {
    match get(key) {
        Some(Value::Bool(b)) => b,
        Some(other) => crate::json::truthy(&other),
        None => default,
    }
}

pub fn get_string(key: &str, default: &str) -> String {
    match get(key) {
        Some(Value::String(s)) => s,
        Some(other) => crate::json::to_js_string(&other),
        None => default.to_string(),
    }
}

pub fn get_opt_string(key: &str) -> Option<String> {
    get(key).map(|v| crate::json::to_js_string(&v))
}

/// `cli.getClientId()`.
pub fn client_id() -> Option<String> {
    get_opt_string(names::CLIENT_ID).or_else(|| std::env::var("CLIMICROSOFT365_ENTRAAPPID").ok())
}

/// `cli.getTenant()`.
pub fn tenant() -> String {
    get_opt_string(names::TENANT_ID)
        .or_else(|| {
            std::env::var("CLIMICROSOFT365_TENANT")
                .ok()
                .filter(|s| !s.is_empty())
        })
        .unwrap_or_else(|| "common".into())
}

pub fn prompt_enabled() -> bool {
    if crate::agent::enabled() {
        return false;
    }
    get_bool(names::PROMPT, true)
}

/// `config.allScopes`.
pub const ALL_SCOPES: &[&str] = &[
    "https://management.azure.com/user_impersonation",
    "https://admin.services.crm.dynamics.com/user_impersonation",
    "https://graph.microsoft.com/AgentIdentityBlueprint.Read.All",
    "https://graph.microsoft.com/AgentIdentityBlueprintPrincipal.Read.All",
    "https://graph.microsoft.com/AppCatalog.ReadWrite.All",
    "https://graph.microsoft.com/AuditLog.Read.All",
    "https://graph.microsoft.com/Bookings.Read.All",
    "https://graph.microsoft.com/Calendars.Read",
    "https://graph.microsoft.com/Calendars.ReadWrite",
    "https://graph.microsoft.com/ChannelMember.ReadWrite.All",
    "https://graph.microsoft.com/ChannelMessage.Read.All",
    "https://graph.microsoft.com/ChannelMessage.ReadWrite",
    "https://graph.microsoft.com/ChannelMessage.Send",
    "https://graph.microsoft.com/ChannelSettings.ReadWrite.All",
    "https://graph.microsoft.com/Chat.ReadWrite",
    "https://graph.microsoft.com/Community.ReadWrite.All",
    "https://graph.microsoft.com/Directory.AccessAsUser.All",
    "https://graph.microsoft.com/Directory.ReadWrite.All",
    "https://graph.microsoft.com/EngagementRole.ReadWrite.All",
    "https://graph.microsoft.com/ExternalConnection.ReadWrite.All",
    "https://graph.microsoft.com/ExternalItem.ReadWrite.All",
    "https://graph.microsoft.com/FileStorageContainer.Selected",
    "https://graph.microsoft.com/FileStorageContainerType.Manage.All",
    "https://graph.microsoft.com/Group.ReadWrite.All",
    "https://graph.microsoft.com/IdentityProvider.ReadWrite.All",
    "https://graph.microsoft.com/InformationProtectionPolicy.Read",
    "https://graph.microsoft.com/Mail.Read.Shared",
    "https://graph.microsoft.com/Mail.ReadWrite",
    "https://graph.microsoft.com/Mail.Send",
    "https://graph.microsoft.com/MailboxSettings.ReadWrite",
    "https://graph.microsoft.com/Notes.ReadWrite.All",
    "https://graph.microsoft.com/OnlineMeetingArtifact.Read.All",
    "https://graph.microsoft.com/OnlineMeetings.ReadWrite",
    "https://graph.microsoft.com/OnlineMeetingTranscript.Read.All",
    "https://graph.microsoft.com/Organization.ReadWrite.All",
    "https://graph.microsoft.com/PeopleSettings.ReadWrite.All",
    "https://graph.microsoft.com/Place.Read.All",
    "https://graph.microsoft.com/Policy.Read.All",
    "https://graph.microsoft.com/Policy.Read.PermissionGrant",
    "https://graph.microsoft.com/RecordsManagement.ReadWrite.All",
    "https://graph.microsoft.com/Reports.Read.All",
    "https://graph.microsoft.com/ReportSettings.ReadWrite.All",
    "https://graph.microsoft.com/RoleAssignmentSchedule.ReadWrite.Directory",
    "https://graph.microsoft.com/RoleEligibilitySchedule.Read.Directory",
    "https://graph.microsoft.com/RoleManagement.Read.Directory",
    "https://graph.microsoft.com/SecurityEvents.Read.All",
    "https://graph.microsoft.com/ServiceHealth.Read.All",
    "https://graph.microsoft.com/ServiceMessage.Read.All",
    "https://graph.microsoft.com/ServiceMessageViewpoint.Write",
    "https://graph.microsoft.com/Sites.Read.All",
    "https://graph.microsoft.com/Tasks.ReadWrite",
    "https://graph.microsoft.com/Team.Create",
    "https://graph.microsoft.com/TeamMember.ReadWrite.All",
    "https://graph.microsoft.com/TeamsAppInstallation.ReadWriteForUser",
    "https://graph.microsoft.com/TeamSettings.ReadWrite.All",
    "https://graph.microsoft.com/TeamsTab.ReadWrite.All",
    "https://graph.microsoft.com/User.Invite.All",
    "https://graph.microsoft.com/User.ReadBasic.All",
    "https://manage.office.com/ActivityFeed.Read",
    "https://manage.office.com/ServiceHealth.Read",
    "https://analysis.windows.net/powerbi/api/Dataset.Read.All",
    "https://api.powerapps.com//User",
    "https://microsoft.sharepoint-df.com/AllSites.FullControl",
    "https://microsoft.sharepoint-df.com/TermStore.ReadWrite.All",
    "https://microsoft.sharepoint-df.com/User.ReadWrite.All",
];

/// `config.minimalScopes`.
pub const MINIMAL_SCOPES: &[&str] = &["https://graph.microsoft.com/User.Read"];

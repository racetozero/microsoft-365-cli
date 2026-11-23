//! `cli *`, `connection *`, `context *`, `util accesstoken get` and `setup`.

use m365_core::auth;
use m365_core::config::{self, ALL_SCOPES, MINIMAL_SCOPES, names};
use m365_core::http::Request;
use m365_core::json::{Obj, stringify_pretty, to_js_string};
use m365_core::ut::entra_app::{self, AppCreationOptions};
use m365_core::util::{CheckStatus, status};
use m365_core::{Cx, Error, Result, Value, obj};

pub async fn consent(cx: &mut Cx) -> Result<()> {
    let scope = if cx.str("service").as_deref() == Some("VivaEngage") {
        "https://api.yammer.com/user_impersonation"
    } else {
        ""
    };
    let c = auth::connection();
    let app_id = if c.active {
        c.app_id.clone()
    } else {
        config::client_id()
    };
    let Some(app_id) = app_id else {
        return Err(Error::msg(
            "Unable to determine the client ID of the Microsoft Entra application used by the CLI. Log in to Microsoft 365 first or store the client ID using 'm365 cli config set --key clientId --value <appId>'.",
        ));
    };
    let tenant = if c.active {
        c.tenant.clone()
    } else {
        config::tenant()
    };
    cx.log(Value::from(format!(
        "To consent permissions for executing {} commands, navigate in your web browser to https://login.microsoftonline.com/{tenant}/oauth2/v2.0/authorize?client_id={app_id}&response_type=code&scope={}",
        cx.str("service").unwrap_or_default(),
        m365_core::util::enc(scope)
    )))
}

fn os_info() -> Value {
    let platform = match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "win32",
        other => other,
    };
    let (version, release) = os_version();
    obj! { "platform": platform, "version": version, "release": release }
}

fn os_version() -> (String, String) {
    #[cfg(unix)]
    {
        let run = |a: &str| {
            std::process::Command::new("uname")
                .arg(a)
                .output()
                .ok()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default()
        };
        (run("-v"), run("-r"))
    }
    #[cfg(not(unix))]
    {
        (String::new(), String::new())
    }
}

pub async fn doctor(cx: &mut Cx) -> Result<()> {
    let c = auth::connection();
    let mut roles: Vec<Value> = Vec::new();
    let mut scopes = Obj::new();
    for (_, t) in c.access_tokens.iter() {
        let tok = t.get("accessToken").and_then(Value::as_str).unwrap_or("");
        for r in auth::jwt::roles(tok) {
            roles.push(Value::from(r));
        }
        if let Some(p) = auth::jwt::payload(tok)
            && let Some(scp) = p
                .get("scp")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
        {
            let aud = p.get("aud").map(to_js_string).unwrap_or_default();
            static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                regex::Regex::new(r"(-my|-admin).sharepoint.com").unwrap()
            });
            let res = RE.replace(&aud, ".sharepoint.com").into_owned();
            scopes.insert(res, Value::Array(scp.split(' ').map(Value::from).collect()));
        }
    }
    let tenant = if m365_core::validation::is_valid_guid(&c.tenant) {
        "single".to_string()
    } else {
        c.tenant.clone()
    };
    cx.log(obj! {
        "os": os_info(),
        "cliVersion": m365_core::manifest::UPSTREAM_VERSION,
        "nodeVersion": format!("rust-{}", env!("CARGO_PKG_VERSION")),
        "cliEntraAppId": ?c.app_id.clone(),
        "cliEntraAppTenant": tenant,
        "authMode": c.auth_type.as_str(),
        "cliEnvironment": std::env::var("CLIMICROSOFT365_ENV").unwrap_or_default(),
        "cliConfig": Value::Object(config::all()),
        "roles": roles,
        "scopes": Value::Object(scopes)
    })
}

pub async fn issue(cx: &mut Cx) -> Result<()> {
    let link = match cx.str("type").as_deref() {
        Some("bug") => "https://aka.ms/cli-m365/bug",
        Some("command") => "https://aka.ms/cli-m365/new-command",
        Some("sample") => "https://aka.ms/cli-m365/new-sample-script",
        _ => "",
    };
    let _ = m365_core::util::open_browser(link);
    cx.log(Value::from(link))
}

pub async fn config_get(cx: &mut Cx) -> Result<()> {
    match config::get(&cx.str("key").unwrap_or_default()) {
        Some(v) => cx.log(v),
        None => cx.log_undefined(),
    }
}

pub async fn config_list(cx: &mut Cx) -> Result<()> {
    cx.log(Value::Object(config::all()))
}

pub async fn config_reset(cx: &mut Cx) -> Result<()> {
    match cx.s("key") {
        Some(k) => config::delete(&k),
        None => config::clear(),
    }
    Ok(())
}

pub fn config_set_refine(i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let key = o.get("key").map(to_js_string).unwrap_or_default();
    let value = o.get("value").map(to_js_string).unwrap_or_default();
    Ok(match i {
        0 => {
            key != names::OUTPUT || ["text", "json", "csv", "md", "none"].contains(&value.as_str())
        }
        1 => key != names::ERROR_OUTPUT || ["stdout", "stderr"].contains(&value.as_str()),
        2 => {
            key != names::PROMPT_LIST_PAGE_SIZE || {
                let n = m365_core::json::js_number(&value);
                !n.is_nan() && n > 0.0
            }
        }
        3 => key != names::HELP_MODE || m365_core::help::HELP_MODES.contains(&value.as_str()),
        4 => key != names::AUTH_TYPE || m365_core::auth::AuthType::parse(&value).is_some(),
        5 => key != names::HELP_TARGET || ["console", "web"].contains(&value.as_str()),
        6 => key != names::CLIENT_ID || m365_core::validation::is_valid_guid(&value),
        7 => {
            key != names::TENANT_ID
                || value == "common"
                || m365_core::validation::is_valid_guid(&value)
        }
        _ => true,
    })
}

pub async fn config_set(cx: &mut Cx) -> Result<()> {
    let key = cx.str("key").unwrap_or_default();
    let raw = cx.opts.get("value").cloned().unwrap_or(Value::Null);
    let value = match key.as_str() {
        names::AUTO_OPEN_LINKS_IN_BROWSER
        | names::COPY_DEVICE_CODE_TO_CLIPBOARD
        | names::CSV_HEADER
        | names::CSV_QUOTED
        | names::CSV_QUOTED_EMPTY
        | names::DISABLE_TELEMETRY
        | names::PRINT_ERRORS_AS_PLAIN_TEXT
        | names::PROMPT
        | names::SHOW_HELP_ON_FAILURE => Value::Bool(raw.as_str() == Some("true")),
        _ => raw,
    };
    config::set(&key, value);
    Ok(())
}

// ---- completion ---------------------------------------------------------------

pub fn completion_dir() -> std::path::PathBuf {
    config::home().join(".cli-m365-completion")
}

fn generate_sh_completion() -> Result<()> {
    let dir = completion_dir();
    std::fs::create_dir_all(&dir)?;
    std::fs::write(
        dir.join("commands.json"),
        m365_core::json::stringify(&Value::Object(m365_core::completion::tree())),
    )?;
    std::fs::write(
        dir.join("Register-CLIM365Completion.ps1"),
        crate::completion::PWSH_SCRIPT,
    )?;
    Ok(())
}

pub async fn completion_sh_setup(cx: &mut Cx) -> Result<()> {
    cx.debug_log("Generating command completion...");
    generate_sh_completion()?;
    cx.debug_log("Registering command completion with the shell...");
    crate::completion::setup_shell_init_file()?;
    // omelette exits the process after writing the init file, so upstream
    // never prints its success message
    std::process::exit(0)
}

pub async fn completion_sh_update(cx: &mut Cx) -> Result<()> {
    cx.debug_log("Generating command completion...");
    generate_sh_completion()
}

pub async fn completion_pwsh_setup(cx: &mut Cx) -> Result<()> {
    cx.debug_log("Generating command completion...");
    generate_sh_completion()?;
    let profile = cx.str("profile").unwrap_or_default();
    cx.debug_log(&format!(
        "Ensuring that the specified profile path {profile} exists..."
    ));
    let p = std::path::Path::new(&profile);
    if p.exists() {
        cx.debug_log("Profile file already exists");
    } else {
        let dir = p.parent().unwrap_or(std::path::Path::new("."));
        if dir.exists() {
            cx.debug_log(&format!("Profile path {} already exists", dir.display()));
        } else {
            cx.debug_log(&format!(
                "Profile path {} doesn't exist. Creating...",
                dir.display()
            ));
            std::fs::create_dir_all(dir)?;
        }
        cx.debug_log(&format!("Creating profile file {profile}..."));
        std::fs::write(p, "")?;
    }
    cx.verbose_log("Adding CLI for Microsoft 365 command completion to PowerShell profile...");
    let script = completion_dir().join("Register-CLIM365Completion.ps1");
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().append(true).open(p)?;
    write!(f, "{}{}", m365_core::term::EOL, script.display())?;
    Ok(())
}

pub async fn completion_clink_update(cx: &mut Cx) -> Result<()> {
    cx.log(Value::from(crate::completion::clink(
        &m365_core::completion::tree(),
    )))
}

// ---- cli app ---------------------------------------------------------------------

pub fn app_add_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let scopes = o.get("scopes").map(to_js_string).unwrap_or_default();
    if !scopes.contains(',') {
        return Ok(scopes == "minimal" || scopes == "all");
    }
    Ok(scopes
        .split(',')
        .map(str::trim)
        .all(|s| s.starts_with("https")))
}

pub fn cli_app_options(name: &str, scopes: &str) -> AppCreationOptions {
    AppCreationOptions {
        allow_public_client_flows: true,
        apis_delegated: Some(scopes.to_string()),
        implicit_flow: false,
        multitenant: false,
        name: Some(name.to_string()),
        platform: Some("publicClient".into()),
        redirect_uris: Some("http://localhost,https://localhost,https://login.microsoftonline.com/common/oauth2/nativeclient".into()),
        ..Default::default()
    }
}

pub async fn create_cli_app(
    name: &str,
    scopes: &str,
    verbose: bool,
    debug: bool,
) -> Result<(String, String)> {
    let options = cli_app_options(name, scopes);
    let apis = entra_app::resolve_apis(&options, None, verbose, debug).await?;
    let info =
        entra_app::create_app_registration(&options, &Obj::new(), &apis, verbose, debug).await?;
    let tenant = auth::jwt::tenant_id(&auth::default_access_token().unwrap_or_default());
    entra_app::grant_admin_consent(&info, true, debug).await?;
    Ok((m365_core::json::s(&info, "appId").to_string(), tenant))
}

pub async fn app_add(cx: &mut Cx) -> Result<()> {
    let scopes = match cx.str("scopes").as_deref() {
        Some("all") => ALL_SCOPES.join(","),
        Some("minimal") | None => MINIMAL_SCOPES.join(","),
        Some(other) => other.to_string(),
    };
    let (app_id, tenant) = create_cli_app(
        &cx.str("name").unwrap_or_default(),
        &scopes,
        cx.verbose(),
        cx.debug(),
    )
    .await?;
    if cx.bool("saveToConfig") {
        config::set("clientId", Value::from(app_id));
        config::set("tenantId", Value::from(tenant));
    }
    Ok(())
}

pub async fn app_reconsent(cx: &mut Cx) -> Result<()> {
    let app_id = auth::connection().app_id.unwrap_or_default();
    cx.verbose_log(&format!("Adding all missing permission scopes used in CLI for Microsoft 365 to application with ID '{app_id}'..."));
    let app = entra_app::get_by_app_id(&app_id, Some(&["requiredResourceAccess", "id"])).await?;
    let had_rra = app.get("requiredResourceAccess").is_some();
    let mut rra: Vec<Value> = m365_core::json::arr(&app, "requiredResourceAccess").to_vec();
    let sps = m365_core::ut::entra_sp::get_all(Some(
        "displayName,appId,oauth2PermissionScopes,servicePrincipalNames",
    ))
    .await?;
    cx.verbose_log(&format!(
        "Verifying if all {} permission scopes are present in the app registration...",
        ALL_SCOPES.len()
    ));
    use m365_core::ut::url::remove_trailing_slashes as rts;
    for scope in ALL_SCOPES {
        let pos = scope.rfind('/').unwrap_or(0);
        let sp_name = rts(&scope[..pos]);
        let scope_name = &scope[pos + 1..];
        let sp = sps.iter().find(|sp| {
            m365_core::json::arr(sp, "servicePrincipalNames")
                .iter()
                .any(|n| rts(&to_js_string(n)).to_lowercase() == sp_name.to_lowercase())
        });
        let Some(sp) = sp else {
            cx.verbose_log(&format!(
                "Service principal with name '{sp_name}' not found. Skipping scope '{scope_name}'."
            ));
            continue;
        };
        let Some(sc) = m365_core::json::arr(sp, "oauth2PermissionScopes")
            .iter()
            .find(|s| m365_core::json::s(s, "value").to_lowercase() == scope_name.to_lowercase())
        else {
            cx.verbose_log(&format!(
                "Scope '{scope_name}' not found in service principal '{sp_name}'. Skipping scope..."
            ));
            continue;
        };
        let sp_app = m365_core::json::s(sp, "appId").to_lowercase();
        let idx = match rra
            .iter()
            .position(|r| m365_core::json::s(r, "resourceAppId").to_lowercase() == sp_app)
        {
            Some(i) => i,
            None => {
                rra.push(obj! { "resourceAppId": m365_core::json::s(sp, "appId"), "resourceAccess": Value::Array(vec![]) });
                rra.len() - 1
            }
        };
        let sid = m365_core::json::s(sc, "id").to_lowercase();
        if let Some(Value::Array(a)) = rra[idx].get_mut("resourceAccess")
            && !a
                .iter()
                .any(|x| m365_core::json::s(x, "id").to_lowercase() == sid)
        {
            a.push(obj! { "id": m365_core::json::s(sc, "id"), "type": "Scope" });
        }
    }
    let id = m365_core::json::s(&app, "id").to_string();
    cx.verbose_log(&format!(
        "Updating permission scopes of application with ID '{id}'..."
    ));
    Request::patch(format!(
        "https://graph.microsoft.com/v1.0/applications/{id}"
    ))
    .graph_json()
    .json(obj! { "requiredResourceAccess": ?(had_rra || !rra.is_empty()).then_some(rra) })
    .send()
    .await?;
    let url = format!(
        "https://login.microsoftonline.com/{}/adminconsent?client_id={app_id}",
        auth::connection().tenant
    );
    cx.log(Value::from(format!("To consent to the new scopes for your Microsoft Entra application registration, please navigate to the following URL: {url}")))?;
    if config::get_bool(names::AUTO_OPEN_LINKS_IN_BROWSER, false) {
        let _ = m365_core::util::open_browser(&url);
    }
    Ok(())
}

// ---- connection -------------------------------------------------------------------

pub async fn connection_list(cx: &mut Cx) -> Result<()> {
    let current = auth::connection().name;
    let mut out: Vec<Value> = auth::all_connections()
        .iter()
        .map(|c| {
            let name = c.get("name").cloned();
            obj! {
                "name": ?name.clone(),
                "connectedAs": ?c.get("identityName").cloned(),
                "authType": ?c.get("authType").cloned(),
                "active": name.as_ref().and_then(Value::as_str).map(str::to_string) == current
            }
        })
        .collect();
    out.sort_by(|a, b| {
        m365_core::util::locale_compare(
            m365_core::json::s(a, "name"),
            m365_core::json::s(b, "name"),
        )
    });
    cx.log(Value::Array(out))
}

fn conn_desc(c: &Value) -> String {
    format!(
        "'{}', appId: {}, tenantId: {}",
        c.get("identityName")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
        c.get("appId")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()),
        c.get("identityTenantId")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    )
}

pub async fn connection_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove the connection?")? {
        return Ok(());
    }
    let c = auth::get_connection(&cx.str("name").unwrap_or_default())?;
    cx.verbose_log(&format!("Removing connection {}...", conn_desc(&c)));
    auth::remove_connection_info(&c)
}

pub fn connection_set_field(idx: u16, v: &Value, _o: &Obj) -> m365_core::schema::FieldCheck {
    m365_core::schema::check({
        let n = to_js_string(v);
        let exists = auth::all_connections()
            .iter()
            .any(|c| c.get("name").and_then(Value::as_str) == Some(n.as_str()));
        if idx == 0 { exists } else { !exists }
    })
}

pub async fn connection_set(cx: &mut Cx) -> Result<()> {
    let c = auth::get_connection(&cx.str("name").unwrap_or_default())?;
    cx.verbose_log(&format!("Updating connection {}...", conn_desc(&c)));
    auth::update_connection(
        m365_core::json::s(&c, "name"),
        &cx.str("newName").unwrap_or_default(),
    )
}

pub async fn connection_use(cx: &mut Cx) -> Result<()> {
    let c = match cx.s("name") {
        Some(n) => auth::get_connection(&n)?,
        None => {
            let mut all = auth::all_connections();
            all.sort_by(|a, b| {
                m365_core::util::locale_compare(
                    m365_core::json::s(a, "name"),
                    m365_core::json::s(b, "name"),
                )
            });
            cx.handle_multiple_results(
                "Please select the connection you want to activate.",
                &m365_core::util::array_to_hash_table("name", &all),
            )?
        }
    };
    cx.verbose_log(&format!("Switching to connection {}...", conn_desc(&c)));
    auth::switch_to_connection(&c)?;
    let conn = auth::connection();
    let mut d = auth::connection_details(&serde_json::to_value(&conn)?);
    if cx.debug()
        && let Some(o) = d.as_object_mut()
    {
        o.insert(
            "accessTokens".into(),
            Value::from(stringify_pretty(
                &Value::Object(conn.access_tokens.clone()),
                "  ",
            )),
        );
    }
    cx.log(d)
}

// ---- context ---------------------------------------------------------------------

const RC: &str = ".m365rc.json";

fn read_rc(on_error: impl Fn(&str) -> String) -> Result<Value> {
    if !std::path::Path::new(RC).exists() {
        return Ok(Value::Object(Obj::new()));
    }
    let text =
        std::fs::read_to_string(RC).map_err(|e| Error::msg(on_error(&format!("Error: {e}"))))?;
    if text.is_empty() {
        return Ok(Value::Object(Obj::new()));
    }
    m365_core::json::parse(&text)
        .map_err(|_| Error::msg(on_error(&m365_core::util::js_json_error(&text))))
}

fn write_rc(v: &Value, on_error: impl Fn(&str) -> String) -> Result<()> {
    std::fs::write(RC, stringify_pretty(v, "  "))
        .map_err(|e| Error::msg(on_error(&format!("Error: {e}"))))
}

/// `ContextCommand.saveContextInfo`.
fn save_context_info(context: Obj) -> Result<()> {
    let mut rc =
        read_rc(|e| format!("Error reading {RC}: {e}. Please add context info to {RC} manually."))?;
    if rc.get("context").is_none() {
        if let Some(o) = rc.as_object_mut() {
            o.insert("context".into(), Value::Object(context));
        }
        write_rc(&rc, |e| {
            format!("Error writing {RC}: {e}. Please add context info to {RC} manually.")
        })?;
    }
    Ok(())
}

pub async fn context_init(_cx: &mut Cx) -> Result<()> {
    save_context_info(Obj::new())
}

pub async fn context_option_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving context options...");
    if std::path::Path::new(RC).exists() {
        cx.verbose_log("Reading context file...");
    }
    let rc = read_rc(|e| {
        format!("Error reading {RC}: {e}. Please retrieve context options from {RC} manually.")
    })?;
    match rc.get("context").filter(|c| m365_core::json::truthy(c)) {
        Some(c) => cx.log(c.clone()),
        None => Err(Error::msg("No context present")),
    }
}

pub async fn context_option_remove(cx: &mut Cx) -> Result<()> {
    let name = cx.str("name").unwrap_or_default();
    cx.verbose_log(&format!("Removing context option '{name}'..."));
    let msg = format!("Are you sure you want to remove the context option {name}?");
    if !cx.proceed("force", &msg)? {
        return Ok(());
    }
    {
        if std::path::Path::new(RC).exists() {
            cx.verbose_log("Reading context file...");
        }
        let mut rc = read_rc(|e| {
            format!(
                "Error reading {RC}: {e}. Please remove context option {name} from {RC} manually."
            )
        })?;
        let present = rc
            .get("context")
            .and_then(|c| c.get(&name))
            .is_some_and(m365_core::json::truthy);
        if !present {
            return Err(Error::msg(format!(
                "There is no option {name} in the context info"
            )));
        }
        cx.verbose_log(&format!(
            "Removing context option {name} from the context file..."
        ));
        if let Some(Value::Object(c)) = rc.get_mut("context") {
            c.shift_remove(&name);
        }
        write_rc(&rc, |e| {
            format!(
                "Error writing {RC}: {e}. Please remove context option {name} from {RC} manually."
            )
        })
    }
}

pub async fn context_option_set(cx: &mut Cx) -> Result<()> {
    let name = cx.str("name").unwrap_or_default();
    let value = cx.str("value").unwrap_or_default();
    cx.verbose_log(&format!(
        "Saving {name} with value {value} to the {RC} file..."
    ));
    if std::path::Path::new(RC).exists() {
        cx.verbose_log(&format!("Reading existing {RC}..."));
    }
    let mut rc =
        read_rc(|e| format!("Error reading {RC}: {e}. Please add {name} to {RC} manually."))?;
    if rc.get("context").is_some_and(m365_core::json::truthy) {
        if let Some(Value::Object(c)) = rc.get_mut("context") {
            c.insert(name.clone(), Value::from(value.as_str()));
        }
        cx.verbose_log(&format!(
            "Creating option {name} with value {value} in existing context..."
        ));
        write_rc(&rc, |e| {
            format!("Error writing {RC}: {e}. Please add {name} to {RC} manually.")
        })
    } else {
        cx.verbose_log(&format!("Context doesn't exist. Initializing the context and creating option {name} with value {value}..."));
        let mut c = Obj::new();
        c.insert(name, Value::from(value));
        save_context_info(c)
    }
}

pub async fn context_remove(cx: &mut Cx) -> Result<()> {
    if !cx.proceed("force", "Are you sure you want to remove the context?")? {
        return Ok(());
    }
    {
        let mut rc = read_rc(|e| {
            format!("Error reading {RC}: {e}. Please remove context info from {RC} manually.")
        })?;
        if rc.get("context").is_none() {
            return Ok(());
        }
        let keys: Vec<String> = rc
            .as_object()
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default();
        if keys.len() == 1 && keys[0] == "context" {
            std::fs::remove_file(RC).map_err(|e| {
                Error::msg(format!(
                    "Error removing {RC}: Error: {e}. Please remove {RC} manually."
                ))
            })
        } else {
            if let Some(o) = rc.as_object_mut() {
                o.shift_remove("context");
            }
            write_rc(&rc, |e| {
                format!("Error writing {RC}: {e}. Please remove context info from {RC} manually.")
            })
        }
    }
}

// ---- util ------------------------------------------------------------------------

pub async fn accesstoken_get(cx: &mut Cx) -> Result<()> {
    let mut resource = cx.str("resource").unwrap_or_default();
    if resource.to_lowercase() == "sharepoint" {
        match auth::connection().spo_url {
            Some(s) => resource = s,
            None => {
                return Err(Error::msg(
                    "SharePoint URL undefined. Use the 'm365 spo set --url https://contoso.sharepoint.com' command to set the URL",
                ));
            }
        }
    } else if resource.to_lowercase() == "graph" {
        resource = auth::default_resource();
    }
    let token = auth::ensure_access_token(&resource, cx.bool("new")).await?;
    if cx.bool("decoded") {
        let chunks: Vec<&str> = token.split('.').collect();
        let dec = |s: &str| -> Value {
            base64::Engine::decode(
                &base64::engine::general_purpose::URL_SAFE_NO_PAD,
                s.trim_end_matches('='),
            )
            .ok()
            .and_then(|b| serde_json::from_slice(&b).ok())
            .unwrap_or(Value::Null)
        };
        let header = dec(chunks.first().copied().unwrap_or(""));
        let payload = dec(chunks.get(1).copied().unwrap_or(""));
        cx.log_raw(&format!(
            "{}.{}.[signature]",
            stringify_pretty(&header, "  "),
            stringify_pretty(&payload, "  ")
        ));
        Ok(())
    } else {
        cx.log(Value::from(token))
    }
}

// ---- setup -----------------------------------------------------------------------

fn interactive_preset() -> Vec<(&'static str, Value)> {
    vec![
        ("autoOpenLinksInBrowser", Value::Bool(true)),
        ("copyDeviceCodeToClipboard", Value::Bool(true)),
        ("output", Value::from("text")),
        ("printErrorsAsPlainText", Value::Bool(true)),
        ("prompt", Value::Bool(true)),
        ("showHelpOnFailure", Value::Bool(true)),
    ]
}

fn scripting_preset() -> Vec<(&'static str, Value)> {
    vec![
        ("autoOpenLinksInBrowser", Value::Bool(false)),
        ("copyDeviceCodeToClipboard", Value::Bool(false)),
        ("output", Value::from("json")),
        ("printErrorsAsPlainText", Value::Bool(false)),
        ("prompt", Value::Bool(false)),
        ("showHelpOnFailure", Value::Bool(false)),
    ]
}

fn assign(settings: &mut Obj, preset: Vec<(&'static str, Value)>) {
    for (k, v) in preset {
        settings.insert(k.to_string(), v);
    }
}

#[derive(Default)]
struct Prefs {
    entra_app: &'static str,
    new_scopes: &'static str,
    client_id: Option<String>,
    tenant_id: Option<String>,
    client_secret: Option<String>,
    cert_file: Option<String>,
    cert_b64: Option<String>,
    cert_password: Option<String>,
    usage: &'static str,
    powershell: Option<bool>,
    experience: &'static str,
}

fn settings_for(p: &Prefs) -> Obj {
    let mut s = Obj::new();
    match p.usage {
        "interactively" => assign(&mut s, interactive_preset()),
        "scripting" => assign(&mut s, scripting_preset()),
        _ => {}
    }
    if p.powershell == Some(true) {
        s.insert("errorOutput".into(), Value::from("stdout"));
    }
    match p.experience {
        "beginner" => {
            s.insert("helpMode".into(), Value::from("full"));
        }
        "proficient" => {
            s.insert("helpMode".into(), Value::from("options"));
        }
        _ => {}
    }
    match p.entra_app {
        "create" => {
            s.insert("authType".into(), Value::from("browser"));
        }
        "useExisting" => {
            let t = if p.client_secret.as_deref().is_some_and(|x| !x.is_empty()) {
                "secret"
            } else if p.cert_file.as_deref().is_some_and(|x| !x.is_empty())
                || p.cert_b64.as_deref().is_some_and(|x| !x.is_empty())
            {
                "certificate"
            } else {
                "browser"
            };
            s.insert("authType".into(), Value::from(t));
        }
        _ => {}
    }
    s
}

pub fn setup_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let t = |k: &str| o.get(k).is_some_and(m365_core::json::truthy);
    Ok(!t("interactive") || !t("scripting"))
}

async fn configure_settings(cx: &Cx, prefs: &Prefs, mut settings: Obj, silent: bool) -> Result<()> {
    match prefs.entra_app {
        "create" => {
            cx.verbose_log("Creating a new Entra app...");
            if !cx.confirm_default(
                "CLI for Microsoft 365 will now sign in to your Microsoft 365 tenant as Microsoft Azure CLI to create a new app registration. Continue?",
            )? {
                return Err(Error::msg("Cancelled"));
            }
            auth::with_connection(|c| {
                c.auth_type = auth::AuthType::Browser;
                c.app_id = Some("04b07795-8ddb-461a-bbee-02f9e1bf7b46".into());
                c.tenant = "common".into();
            });
            auth::ensure_access_token(&auth::default_resource(), false).await?;
            auth::with_connection(|c| c.active = true);
            let scopes = if prefs.new_scopes == "all" {
                ALL_SCOPES.join(",")
            } else {
                MINIMAL_SCOPES.join(",")
            };
            let (app_id, tenant) =
                create_cli_app("CLI for M365", &scopes, cx.verbose(), cx.debug()).await?;
            settings.insert("clientId".into(), Value::from(app_id));
            settings.insert("tenantId".into(), Value::from(tenant));
            for k in [
                names::CLIENT_SECRET,
                names::CLIENT_CERTIFICATE_FILE,
                names::CLIENT_CERTIFICATE_BASE64_ENCODED,
                names::CLIENT_CERTIFICATE_PASSWORD,
            ] {
                config::delete(k);
            }
        }
        "useExisting" => {
            let put = |s: &mut Obj, k: &str, v: &Option<String>| {
                match v {
                    Some(v) => s.insert(k.to_string(), Value::from(v.as_str())),
                    None => s.insert(k.to_string(), Value::Null),
                };
            };
            put(&mut settings, "clientId", &prefs.client_id);
            put(&mut settings, "tenantId", &prefs.tenant_id);
            put(&mut settings, "clientSecret", &prefs.client_secret);
            put(&mut settings, "clientCertificateFile", &prefs.cert_file);
            put(
                &mut settings,
                "clientCertificateBase64Encoded",
                &prefs.cert_b64,
            );
            put(
                &mut settings,
                "clientCertificatePassword",
                &prefs.cert_password,
            );
            settings.retain(|_, v| !v.is_null());
        }
        _ => {}
    }
    if cx.debug() {
        cx.stderr("Configuring settings...");
        cx.stderr(&stringify_pretty(&Value::Object(settings.clone()), "  "));
    }
    for (k, v) in settings {
        if !silent {
            cx.stderr(&status(
                CheckStatus::Success,
                &format!("{k}: {}", to_js_string(&v)),
            ));
        }
        config::set(&k, v);
    }
    Ok(())
}

pub async fn setup(cx: &mut Cx) -> Result<()> {
    if cx.bool("interactive") || cx.bool("scripting") {
        let mut settings = Obj::new();
        if cx.bool("interactive") {
            assign(&mut settings, interactive_preset());
        } else {
            assign(&mut settings, scripting_preset());
        }
        if m365_core::ut::pid::is_powershell() {
            settings.insert("errorOutput".into(), Value::from("stdout"));
        }
        return configure_settings(cx, &Prefs::default(), settings, true).await;
    }
    cx.stderr("Welcome to the CLI for Microsoft 365 setup!");
    cx.stderr(
        "This command will guide you through the process of configuring the CLI for your needs.",
    );
    cx.stderr("Please, answer the following questions and we'll define a set of settings to best match how you intend to use the CLI.");
    cx.stderr("");
    let mut p = Prefs::default();
    if !cx.bool("skipApp") {
        let i = cx.select(
            "CLI for Microsoft 365 requires a Microsoft Entra app. Do you want to create a new app registration or use an existing one?",
            &["Create a new app registration".into(), "Use an existing app registration".into(), "Skip configuring app registration".into()],
        )?;
        p.entra_app = ["create", "useExisting", "skip"][i];
        match p.entra_app {
            "create" => {
                let i = cx.select(
                    "What scopes should the new app registration have?",
                    &[
                        "User.Read (you will need to add the necessary permissions yourself)"
                            .into(),
                        "All (easy way to use all CLI commands)".into(),
                    ],
                )?;
                p.new_scopes = ["minimal", "all"][i];
            }
            "useExisting" => {
                cx.stderr("Please provide the details of the existing app registration.");
                p.client_id = Some(cx.input("Client ID:")?);
                let t = cx.input("Tenant ID (leave common if the app is multitenant):")?;
                p.tenant_id = Some(if t.is_empty() { "common".into() } else { t });
                let secret = cx.input(
                    "Client secret (leave empty if you use a certificate or a public client):",
                )?;
                p.client_secret = Some(secret.clone());
                if secret.is_empty() {
                    let f = cx.input("Path to the client certificate file (leave empty if you want to specify a base64-encoded certificate string):")?;
                    p.cert_file = Some(f.clone());
                    if f.is_empty() {
                        p.cert_b64 = Some(cx.input("Base64-encoded certificate string (leave empty if you don't connect using a certificate):")?);
                    }
                    if !f.is_empty() || p.cert_b64.as_deref().is_some_and(|x| !x.is_empty()) {
                        p.cert_password = Some(cx.input("Password for the client certificate (leave empty if the certificate is not password-protected):")?);
                    }
                }
            }
            _ => {}
        }
    } else {
        p.entra_app = "skip";
    }
    let i = cx.select(
        "How do you plan to use the CLI?",
        &["Interactively".into(), "Scripting".into()],
    )?;
    p.usage = ["interactively", "scripting"][i];
    if p.usage == "scripting" {
        p.powershell = Some(m365_core::prompt::confirm(
            "Are you going to use the CLI in PowerShell?",
            m365_core::ut::pid::is_powershell(),
        )?);
        m365_core::log::cli_error("");
    }
    let i = cx.select(
        "How experienced are you in using the CLI?",
        &["Beginner".into(), "Proficient".into()],
    )?;
    p.experience = ["beginner", "proficient"][i];
    let mut lines =
        vec!["Based on your preferences, we'll configure the following settings:".to_string()];
    match p.entra_app {
        "create" => lines.push(format!(
            "- Entra app: Create a new app registration with {} scopes",
            p.new_scopes
        )),
        "useExisting" => {
            lines.push("- Entra app: use existing".into());
            lines.push(format!(
                "  - Client ID: {}",
                p.client_id.clone().unwrap_or_default()
            ));
            lines.push(format!(
                "  - Tenant ID: {}",
                p.tenant_id.clone().unwrap_or_default()
            ));
            for (label, v) in [
                ("Client secret", &p.client_secret),
                ("Client certificate file", &p.cert_file),
                ("Client certificate base64-encoded", &p.cert_b64),
                ("Client certificate password", &p.cert_password),
            ] {
                if let Some(v) = v.as_deref().filter(|v| !v.is_empty()) {
                    lines.push(format!("  - {label}: {v}"));
                }
            }
        }
        _ => lines.push("- Entra app: skip".into()),
    }
    for (k, v) in settings_for(&p) {
        lines.push(format!("- {k}: {}", to_js_string(&v)));
    }
    lines.push(String::new());
    lines.push("You can change any of these settings later using the `m365 cli config set` command or reset them to default using `m365 cli config reset`.".into());
    lines.push(String::new());
    lines.push("Do you want to apply these settings now?".into());
    if !m365_core::prompt::confirm(&lines.join(m365_core::term::EOL), false)? {
        return Ok(());
    }
    m365_core::log::cli_error("");
    cx.stderr("");
    cx.stderr("Configuring settings...");
    cx.stderr("");
    let settings = settings_for(&p);
    configure_settings(cx, &p, settings, false).await?;
    if !cx.verbose() {
        cx.stderr("");
        cx.stderr(&m365_core::term::green("DONE"));
    }
    Ok(())
}

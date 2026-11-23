//! `teams app *`, `teams cache remove` and `teams callrecord *`.

use m365_core::prelude::*;
use m365_core::ut::entra;

async fn app_id(cx: &Cx, select_id: bool, quoted: bool) -> Result<String> {
    if let Some(i) = cx.s("id") {
        return Ok(i);
    }
    let name = cx.u("name");
    let sel = if select_id { "&$select=id" } else { "" };
    let r = Request::get(format!(
        "{GRAPH}/v1.0/appCatalogs/teamsApps?$filter=displayName eq '{}'{sel}",
        eqp(&name)
    ))
    .graph_json()
    .data()
    .await?;
    let v = arr(&r, "value");
    match v.len() {
        0 => Err(Error::msg("The specified Teams app does not exist")),
        1 => Ok(u(&v[0], "id")),
        _ => {
            let msg = if quoted {
                format!("Multiple Teams apps with name '{name}' found.")
            } else {
                format!("Multiple Teams apps with name {name} found.")
            };
            Ok(u(
                &cx.handle_multiple_results(&msg, &util::array_to_hash_table("id", v))?,
                "id",
            ))
        }
    }
}

pub async fn install(cx: &mut Cx) -> Result<()> {
    if let Some(uid) = cx.s("userId") {
        cx.verbose_log(&format!("Checking if user {uid} exists..."));
        match entra::user::upn_by_id(&uid, cx.verbose()).await {
            Ok(upn) => cx.verbose_log(&upn),
            Err(e) => {
                cx.verbose_log("undefined");
                let m = e
                    .body()
                    .and_then(|b| b.get("message"))
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into());
                return Err(Error::msg(format!(
                    "User with ID {uid} not found. Original error: {m}"
                )));
            }
        }
    }
    let id = app_id(cx, false, false).await?;
    let url = match cx.s("teamId") {
        Some(t) => format!("{GRAPH}/v1.0/teams/{}/installedApps", eqp(&t)),
        None => format!(
            "{GRAPH}/v1.0/users/{}/teamwork/installedApps",
            eqp(&cx
                .str("userId")
                .or_else(|| cx.str("userName"))
                .unwrap_or_else(|| "undefined".into()))
        ),
    };
    Request::post(url)
        .header("content-type", "application/json;odata=nometadata")
        .accept("application/json;odata.metadata=none")
        .json(obj! { "teamsApp@odata.bind": format!("{GRAPH}/v1.0/appCatalogs/teamsApps/{id}") })
        .json_response()
        .send()
        .await?;
    Ok(())
}

pub async fn list(cx: &mut Cx) -> Result<()> {
    let mut url = format!("{GRAPH}/v1.0/appCatalogs/teamsApps");
    if let Some(d) = cx.s("distributionMethod") {
        url.push_str(&format!("?$filter=distributionMethod eq '{d}'"));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

pub fn file_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(util::is_file(&to_js_string(v)))
}

pub async fn publish(cx: &mut Cx) -> Result<()> {
    let full = util::resolve(&cx.u("filePath"));
    cx.verbose_log(&format!("Adding app '{full}' to app catalog..."));
    let r = Request::post(format!("{GRAPH}/v1.0/appCatalogs/teamsApps"))
        .header("content-type", "application/zip")
        .accept("application/json;odata.metadata=none")
        .bytes(std::fs::read(&full)?)
        .json_response()
        .data()
        .await?;
    cx.log(r)
}

pub async fn remove(cx: &mut Cx) -> Result<()> {
    let what = cx.first_u(&["id", "name"]);
    if !cx.proceed(
        "force",
        &format!("Are you sure you want to remove the Teams app {what} from the app catalog?"),
    )? {
        return Ok(());
    }
    if cx.s("id").is_none() {
        cx.verbose_log("Retrieving app Id...");
    }
    let id = app_id(cx, true, true).await?;
    cx.verbose_log(&format!("Removing app with ID {id}"));
    Request::delete(format!("{GRAPH}/v1.0/appCatalogs/teamsApps/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn uninstall(cx: &mut Cx) -> Result<()> {
    let (id, team) = (cx.u("id"), cx.u("teamId"));
    if !cx.proceed("force", &format!("Are you sure you want to uninstall the app with id {id} from the Microsoft Teams team {team}?"))? {
        return Ok(());
    }
    cx.verbose_log(&format!("Uninstalling app with ID {id} in team {team}"));
    Request::delete(format!("{GRAPH}/v1.0/teams/{team}/installedApps/{id}"))
        .accept("application/json;odata.metadata=none")
        .send()
        .await?;
    Ok(())
}

pub async fn update(cx: &mut Cx) -> Result<()> {
    let id = app_id(cx, false, false).await?;
    let full = util::resolve(&cx.u("filePath"));
    cx.verbose_log(&format!(
        "Updating app with id '{id}' and file '{full}' in the app catalog..."
    ));
    Request::put(format!("{GRAPH}/v1.0/appCatalogs/teamsApps/{id}"))
        .header("content-type", "application/zip")
        .bytes(std::fs::read(&full)?)
        .send()
        .await?;
    Ok(())
}

pub fn cache_validate(o: &Obj) -> std::result::Result<(), String> {
    if let Some(c) = o.get("client").map(to_js_string)
        && !c.is_empty()
        && !["new", "classic"].contains(&c.to_lowercase().as_str())
    {
        return Err(format!(
            "'{c}' is not a valid value for option 'client'. Allowed values are new, classic"
        ));
    }
    if std::env::var("CLIMICROSOFT365_ENV").is_ok_and(|v| v == "docker") {
        return Err("Because you're running CLI for Microsoft 365 in a Docker container, we can't clear the cache on your host. Instead run this command on your host using \"npx ...\"".into());
    }
    if !cfg!(windows) && !cfg!(target_os = "macos") {
        return Err(format!(
            "{} platform is unsupported for this command",
            std::env::consts::OS
        ));
    }
    Ok(())
}

fn platform() -> &'static str {
    if cfg!(windows) {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        std::env::consts::OS
    }
}

pub async fn cache_remove(cx: &mut Cx) -> Result<()> {
    let client = cx
        .s("client")
        .map(|c| c.to_lowercase())
        .unwrap_or_else(|| "new".into());
    if !cx.bool("force") {
        cx.stderr("This command will execute the following steps.");
        cx.stderr("- Stop the Microsoft Teams client.");
        cx.stderr("- Clear the Microsoft Teams cached files.");
        if !cx.confirm("Are you sure you want to clear your Microsoft Teams cache?")? {
            return Ok(());
        }
    }
    let p = platform();
    cx.verbose_log(&format!(
        "Getting path of Teams cache folder for platform {p}..."
    ));
    let home = m365_core::config::home().display().to_string();
    let env = |k: &str| std::env::var(k).unwrap_or_else(|_| "undefined".into());
    let paths: Vec<String> = match (p, client.as_str()) {
        ("win32", "classic") => vec![format!("{}\\Microsoft\\Teams", env("APPDATA"))],
        ("win32", _) => vec![format!(
            "{}\\Packages\\MSTeams_8wekyb3d8bbwe\\LocalCache\\Microsoft\\MSTeams",
            env("LOCALAPPDATA")
        )],
        ("darwin", "classic") => vec![format!(
            "{home}/Library/Application Support/Microsoft/Teams"
        )],
        ("darwin", _) => vec![
            format!("{home}/Library/Group Containers/UBF8T346G9.com.microsoft.teams"),
            format!("{home}/Library/Containers/com.microsoft.teams2"),
        ],
        _ => vec![],
    };
    let mut exists = true;
    for f in &paths {
        cx.verbose_log(&format!("Checking if Teams cache folder exists at {f}..."));
        if !std::path::Path::new(f).exists() {
            exists = false;
        }
    }
    if !exists {
        cx.stderr("Cache folder does not exist. Nothing to remove.");
        return Ok(());
    }
    cx.verbose_log("Stopping Teams client...");
    let cmd = match (p, client.as_str()) {
        ("win32", "classic") => "tasklist /FI \"IMAGENAME eq Teams.exe\" /FO csv",
        ("win32", _) => "tasklist /FI \"IMAGENAME eq ms-teams.exe\" /FO csv",
        ("darwin", "classic") => {
            "ps ax | grep MacOS/Teams -m 1 | grep -v grep | awk '{ print $1 }'"
        }
        _ => "ps ax | grep MacOS/MSTeams -m 1 | grep -v grep | awk '{ print $1 }'",
    };
    cx.debug_log(cmd);
    let (out, _) = util::exec(cmd).map_err(Error::msg)?;
    if p == "darwin" && !out.trim().is_empty() {
        let _ = util::exec(&format!("kill {}", out.trim().parse::<i64>().unwrap_or(0)));
    } else if p == "win32" {
        for proc in util::parse_csv_to_json(&out, '"', ',', true) {
            let _ = util::exec(&format!("taskkill /F /PID {}", u(&proc, "PID")));
        }
    }
    cx.verbose_log("Teams client closed");
    cx.verbose_log("Removing Teams cache files...");
    let base = if p == "win32" {
        "rmdir /s /q "
    } else {
        "rm -r "
    };
    for f in &paths {
        let cmd = format!("{base}\"{f}\"");
        cx.debug_log(&cmd);
        if let Err(e) = util::exec(&cmd) {
            if e.contains("Operation not permitted") {
                cx.log(Value::from(
                    "Deleting the folder failed. Please have a look at the following URL to delete the folders manually: https://answers.microsoft.com/en-us/msteams/forum/all/clearing-cache-on-microsoft-teams/35876f6b-eb1a-4b77-bed1-02ce3277091f",
                ))?;
            } else {
                return Err(Error::msg(e));
            }
        }
    }
    cx.stderr("Teams cache cleared!");
    Ok(())
}

pub async fn callrecord_get(cx: &mut Cx) -> Result<()> {
    let id = cx.u("id");
    cx.verbose_log(&format!("Retrieving call record {id}..."));
    let mut p1 = Request::get(format!(
        "{GRAPH}/v1.0/communications/callRecords/{id}?$expand=sessions($expand=segments)"
    ))
    .graph_json()
    .data()
    .await?;
    let p2 = Request::get(format!("{GRAPH}/v1.0/communications/callRecords/{id}?$select=participants_v2&$expand=participants_v2")).graph_json().data().await?;
    if let (Some(a), Some(b)) = (p1.as_object_mut(), p2.as_object()) {
        for (k, v) in b {
            a.insert(k.clone(), v.clone());
        }
    }
    cx.log(p1)
}

fn within_30_days(v: &str) -> bool {
    if !m365_core::validation::is_valid_iso_date_time(v) {
        return false;
    }
    let Some(d) = m365_core::time::parse_js_date(v) else {
        return false;
    };
    let max = chrono::Utc::now();
    let min = max - chrono::Duration::days(30);
    d >= min && d <= max
}

pub fn callrecord_list_field(_i: u16, v: &Value, _o: &Obj) -> FieldCheck {
    check(within_30_days(&to_js_string(v)))
}

pub fn callrecord_list_refine(_i: usize, o: &Obj) -> std::result::Result<bool, String> {
    let both = o.contains_key("startDateTime") && o.contains_key("endDateTime");
    if !both {
        return Ok(true);
    }
    let a = m365_core::time::parse_js_date(&to_js_string(&o["startDateTime"]));
    let b = m365_core::time::parse_js_date(&to_js_string(&o["endDateTime"]));
    Ok(matches!((a, b), (Some(a), Some(b)) if a < b))
}

pub async fn callrecord_list(cx: &mut Cx) -> Result<()> {
    cx.verbose_log("Retrieving call records...");
    let mut f = Vec::new();
    if cx.s("userId").is_some() || cx.s("userName").is_some() {
        let uid = match cx.s("userName") {
            Some(n) => entra::user::id_by_upn(&n).await?,
            None => cx.u("userId"),
        };
        f.push(format!("participants_v2/any(p:p/id eq '{uid}')"));
    }
    let iso = |v: String| {
        m365_core::time::parse_js_date(&v)
            .map(m365_core::time::to_iso)
            .unwrap_or_default()
    };
    if let Some(s) = cx.s("startDateTime") {
        f.push(format!("startDateTime ge {}", iso(s)));
    }
    if let Some(e) = cx.s("endDateTime") {
        f.push(format!("startDateTime lt {}", iso(e)));
    }
    let mut url = format!("{GRAPH}/v1.0/communications/callRecords");
    if !f.is_empty() {
        url.push_str(&format!("?$filter={}", f.join(" and ")));
    }
    cx.log(Value::Array(odata::get_all_items(&url).await?))
}

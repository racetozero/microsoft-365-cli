//! `m365 update` and the update notice (not upstream commands; `main`
//! dispatches them before the command runner, like `m365 skill`).
//!
//! In a terminal, a release build checks GitHub for a newer release at most
//! once every 20 hours, in the background while the command runs, and
//! prints one line on stderr after the command's output. `m365 update` runs
//! the release's own installer (`install.sh` or `install.ps1` of that tag,
//! which downloads the archive and verifies its SHA-256 checksum) into a
//! staging folder and then replaces the running executable. A Homebrew
//! install is upgraded with `brew upgrade`, and `M365_SELF_UPDATE_COMMAND`
//! hands the update to another package manager.

use m365_core::agent::{self, Code};
use m365_core::json::Obj;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::Duration;

const REPOSITORY: &str = "racetozero/microsoft-365-cli";
const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/racetozero/microsoft-365-cli/releases/latest";
const RELEASES_URL: &str = "https://github.com/racetozero/microsoft-365-cli/releases";
const RAW_REPOSITORY_URL: &str = "https://raw.githubusercontent.com/racetozero/microsoft-365-cli";
const HOMEBREW_FORMULA: &str = "racetozero/tap/m365";
const CACHE_MAX_AGE_SECS: u64 = 20 * 60 * 60;
/// Longest wait for the background check after a command finished.
const FINISH_WAIT: Duration = Duration::from_millis(800);

/// `major.minor.patch` with an optional prerelease, compared like SemVer
/// (a prerelease sorts before its release).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Version {
    core: (u64, u64, u64),
    pre: Option<String>,
}

impl Version {
    fn parse(s: &str) -> Option<Version> {
        let s = s.strip_prefix('v').unwrap_or(s);
        let s = s.split('+').next()?;
        let (core, pre) = match s.split_once('-') {
            Some((c, p)) => (c, Some(p.to_string())),
            None => (s, None),
        };
        let mut it = core.split('.').map(|p| p.parse::<u64>().ok());
        let v = (it.next()??, it.next()??, it.next()??);
        if it.next().is_some() {
            return None;
        }
        Some(Version { core: v, pre })
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Version {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.core
            .cmp(&o.core)
            .then_with(|| match (&self.pre, &o.pre) {
                (None, None) => std::cmp::Ordering::Equal,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a), Some(b)) => a.cmp(b),
            })
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b, c) = self.core;
        write!(f, "{a}.{b}.{c}")?;
        if let Some(p) = &self.pre {
            write!(f, "-{p}")?;
        }
        Ok(())
    }
}

fn current_version() -> Version {
    Version::parse(env!("CARGO_PKG_VERSION")).expect("crate version")
}

fn flag(v: &str) -> bool {
    !v.is_empty() && v != "0" && !v.eq_ignore_ascii_case("false")
}

// ---- m365 update ------------------------------------------------------------------

fn ok(v: Value, text: &str) -> i32 {
    if agent::enabled() {
        agent::print_ok_with(v);
    } else {
        println!("{text}");
    }
    0
}

fn fail(message: String) -> i32 {
    if agent::enabled() {
        return agent::print_error(Code::Failed, &Value::from(message), Obj::new());
    }
    eprintln!("Error: {message}");
    1
}

/// `m365 update [--check]`; returns the exit code.
pub fn run(args: &[String]) -> i32 {
    agent::set_command("update");
    let mut check_only = false;
    for a in args {
        match a.as_str() {
            "--check" => check_only = true,
            "--help" | "-h" => {
                println!(
                    "Usage: m365 update [--check] [--agent]\n\nInstalls the latest m365 release in place of this one. With --check, only\nreports whether a newer release exists.\n\nA Homebrew install is upgraded with `brew upgrade {HOMEBREW_FORMULA}`. Set\nM365_SELF_UPDATE_COMMAND to use another package manager, for example\n`mise use -g github:{REPOSITORY}@{{version}}`."
                );
                return 0;
            }
            other => {
                return fail(format!(
                    "unknown option `{other}`; run `m365 update --help`"
                ));
            }
        }
    }
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => return fail(format!("cannot start the update: {e}")),
    };
    match rt.block_on(update(check_only)) {
        Ok((v, text)) => ok(v, &text),
        Err(e) => fail(e),
    }
}

async fn update(check_only: bool) -> Result<(Value, String), String> {
    let current = current_version();
    let latest = fetch_latest_version(&http_client()?).await?;
    let _ = write_cache(&latest);
    if latest <= current {
        return Ok((
            json!({ "version": current.to_string(), "latest": latest.to_string(), "updated": false }),
            format!("m365 v{current} is up to date."),
        ));
    }
    if check_only {
        return Ok((
            json!({ "version": current.to_string(), "latest": latest.to_string(), "updated": false, "update_command": update_hint() }),
            format!(
                "m365 v{latest} is available (you have v{current}). Run `{}`.",
                update_hint()
            ),
        ));
    }
    if cfg!(debug_assertions) {
        return Err("`m365 update` is not available in debug builds; install a release (see the README) to use it".into());
    }
    if let Some(template) = custom_update_command() {
        run_custom_update(&template, &current, &latest).await?;
    } else if installed_by_homebrew() {
        run_homebrew_upgrade(&current, &latest).await?;
    } else {
        eprintln!("Updating m365 from v{current} to v{latest}...");
        let client = http_client()?;
        let staging = tempfile::tempdir()
            .map_err(|e| format!("cannot create the update staging folder: {e}"))?;
        let installer = download_installer(&client, &latest, staging.path()).await?;
        let staged = run_staged_installer(&installer, &latest, staging.path()).await?;
        replace_current_exe(&staged)?;
    }
    Ok((
        json!({ "version": latest.to_string(), "previous": current.to_string(), "updated": true }),
        format!("Updated m365 from v{current} to v{latest}."),
    ))
}

/// The command a user should run to update this install.
fn update_hint() -> String {
    if custom_update_command().is_none() && installed_by_homebrew() {
        format!("brew upgrade {HOMEBREW_FORMULA}")
    } else {
        "m365 update".into()
    }
}

/// `M365_SELF_UPDATE_COMMAND`, split on spaces.
fn custom_update_command() -> Option<Vec<String>> {
    std::env::var("M365_SELF_UPDATE_COMMAND")
        .ok()
        .map(|c| c.split_whitespace().map(str::to_owned).collect::<Vec<_>>())
        .filter(|c| !c.is_empty())
}

/// Fills `{packageName}`, `{version}` and `{installSpec}`.
fn update_command_argv(template: &[String], latest: &Version) -> Vec<String> {
    template
        .iter()
        .map(|a| {
            a.replace("{installSpec}", &format!("m365@{latest}"))
                .replace("{packageName}", "m365")
                .replace("{version}", &latest.to_string())
        })
        .collect()
}

async fn run_status(argv: &[String], what: &str) -> Result<(), String> {
    let shown = argv.join(" ");
    let status = tokio::process::Command::new(&argv[0])
        .args(&argv[1..])
        .status()
        .await
        .map_err(|e| {
            format!(
                "cannot start `{shown}` ({e}); check that `{}` is installed and on PATH{what}",
                argv[0]
            )
        })?;
    if !status.success() {
        return Err(format!("`{shown}` failed ({status}){what}"));
    }
    Ok(())
}

async fn run_custom_update(
    template: &[String],
    current: &Version,
    latest: &Version,
) -> Result<(), String> {
    let argv = update_command_argv(template, latest);
    eprintln!(
        "Updating m365 from v{current} to v{latest} with `{}`...",
        argv.join(" ")
    );
    run_status(
        &argv,
        "; fix M365_SELF_UPDATE_COMMAND, or remove it to use the m365 installer",
    )
    .await
}

async fn run_homebrew_upgrade(current: &Version, latest: &Version) -> Result<(), String> {
    eprintln!("Updating m365 from v{current} to v{latest} with Homebrew...");
    let brew = homebrew_brew().unwrap_or_else(|| PathBuf::from("brew"));
    let argv = vec![
        brew.to_string_lossy().into_owned(),
        "upgrade".into(),
        HOMEBREW_FORMULA.into(),
    ];
    // the tap may not know the release yet; `brew update` refreshes it first
    let _ = tokio::process::Command::new(&argv[0])
        .arg("update")
        .arg("--quiet")
        .status()
        .await;
    run_status(&argv, "").await
}

/// True when the resolved executable is `.../Cellar/m365/<version>/bin/m365`.
fn installed_by_homebrew() -> bool {
    homebrew_prefix().is_some()
}

/// The Homebrew prefix (`.../Cellar/m365/<version>/bin/m365` -> `...`).
fn homebrew_prefix() -> Option<PathBuf> {
    let exe = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .ok()?;
    let rack = exe.ancestors().nth(3)?;
    if !rack.ends_with("Cellar/m365") {
        return None;
    }
    rack.ancestors().nth(2).map(Path::to_path_buf)
}

/// `<prefix>/bin/brew`, so the upgrade uses the Homebrew that installed m365.
fn homebrew_brew() -> Option<PathBuf> {
    Some(homebrew_prefix()?.join("bin").join("brew")).filter(|b| b.is_file())
}

fn http_client() -> Result<reqwest::Client, String> {
    m365_core::http::client_builder()
        .timeout(Duration::from_secs(10))
        .user_agent(concat!("m365/", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|e| format!("cannot build the update HTTP client: {e}"))
}

async fn fetch_latest_version(client: &reqwest::Client) -> Result<Version, String> {
    let url = std::env::var("M365_UPDATE_LATEST_URL").unwrap_or_else(|_| LATEST_RELEASE_URL.into());
    let release: Value = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .map_err(|e| {
            format!(
                "cannot check the latest m365 release on GitHub: {}",
                m365_core::http::error_chain(&e)
            )
        })?
        .error_for_status()
        .map_err(|e| format!("GitHub did not return the latest m365 release: {e}"))?
        .json()
        .await
        .map_err(|e| format!("cannot read the latest m365 release from GitHub: {e}"))?;
    let tag = release
        .get("tag_name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    Version::parse(tag)
        .ok_or_else(|| format!("the latest GitHub release has an invalid tag `{tag}`"))
}

/// The installer of exactly this release (not of `main`).
fn installer_url(version: &Version) -> String {
    let name = if cfg!(windows) {
        "install.ps1"
    } else {
        "install.sh"
    };
    format!("{RAW_REPOSITORY_URL}/v{version}/{name}")
}

async fn download_installer(
    client: &reqwest::Client,
    version: &Version,
    staging: &Path,
) -> Result<PathBuf, String> {
    let url = installer_url(version);
    let bytes = client
        .get(&url)
        .send()
        .await
        .map_err(|e| {
            format!(
                "cannot download the m365 installer from {url}: {}",
                m365_core::http::error_chain(&e)
            )
        })?
        .error_for_status()
        .map_err(|e| format!("the m365 installer is not available at {url}: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("cannot read the m365 installer: {e}"))?;
    let path = staging.join(if cfg!(windows) {
        "install.ps1"
    } else {
        "install.sh"
    });
    std::fs::write(&path, &bytes).map_err(|e| format!("cannot write the m365 installer: {e}"))?;
    Ok(path)
}

/// Runs the installer with its install folder set to `<staging>/bin`.
async fn run_staged_installer(
    installer: &Path,
    version: &Version,
    staging: &Path,
) -> Result<PathBuf, String> {
    let bin = staging.join("bin");
    let mut command = installer_command(installer)?;
    command
        .current_dir(staging)
        .env("M365_VERSION", version.to_string())
        .env("M365_INSTALL_DIR", &bin)
        .env("M365_REPOSITORY", REPOSITORY)
        .env("M365_RELEASES_URL", RELEASES_URL)
        .env_remove("M365_TARGET");
    let output = command
        .output()
        .await
        .map_err(|e| format!("cannot start the m365 release installer: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        let detail = if stderr.is_empty() {
            String::from_utf8_lossy(&output.stdout).trim().to_owned()
        } else {
            stderr
        };
        return Err(format!(
            "the m365 installer failed ({}): {detail}",
            output.status
        ));
    }
    let staged = bin.join(if cfg!(windows) { "m365.exe" } else { "m365" });
    if !staged.is_file() {
        return Err(format!(
            "the m365 installer did not create {}",
            staged.display()
        ));
    }
    Ok(staged)
}

#[cfg(windows)]
fn installer_command(installer: &Path) -> Result<tokio::process::Command, String> {
    let root = std::env::var_os("SystemRoot").ok_or("SystemRoot is not set")?;
    let ps = PathBuf::from(root)
        .join("System32")
        .join("WindowsPowerShell")
        .join("v1.0")
        .join("powershell.exe");
    if !ps.is_file() {
        return Err(format!("PowerShell was not found at {}", ps.display()));
    }
    let mut c = tokio::process::Command::new(ps);
    c.args(["-NoProfile", "-ExecutionPolicy", "Bypass", "-File"])
        .arg(installer);
    Ok(c)
}

#[cfg(not(windows))]
fn installer_command(installer: &Path) -> Result<tokio::process::Command, String> {
    let mut c = tokio::process::Command::new("/bin/sh");
    c.arg(installer);
    Ok(c)
}

/// Puts `staged` where the running executable is. The new file is copied
/// next to the target first, so the final rename stays on one file system;
/// on Windows the running file is moved aside (it cannot be overwritten).
fn replace_current_exe(staged: &Path) -> Result<(), String> {
    let exe = std::env::current_exe()
        .and_then(std::fs::canonicalize)
        .map_err(|e| format!("cannot find the m365 executable: {e}"))?;
    let dir = exe.parent().ok_or("the m365 executable has no folder")?;
    let tmp = dir.join(format!(".m365.update.{}", std::process::id()));
    // a failed step removes the staged copy
    let denied = |e: std::io::Error| {
        let _ = std::fs::remove_file(&tmp);
        format!(
            "cannot write to {} ({e}); rerun the installer with permission to that folder, or set M365_SELF_UPDATE_COMMAND",
            dir.display()
        )
    };
    std::fs::copy(staged, &tmp).map_err(denied)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o755));
    }
    #[cfg(windows)]
    let old = exe.with_extension("exe.old");
    #[cfg(windows)]
    {
        let _ = std::fs::remove_file(&old);
        std::fs::rename(&exe, &old).map_err(denied)?;
    }
    std::fs::rename(&tmp, &exe).map_err(|e| {
        // put the old executable back, so a failed update leaves m365 in place
        #[cfg(windows)]
        let _ = std::fs::rename(&old, &exe);
        denied(e)
    })
}

// ---- the update notice --------------------------------------------------------------

/// `~/.config/m365/update-check.json`.
fn cache_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    Some(
        PathBuf::from(home)
            .join(".config")
            .join("m365")
            .join("update-check.json"),
    )
}

fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}

/// (latest version, seconds since 1970 of the check)
fn read_cache() -> Option<(Version, u64)> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(cache_path()?).ok()?).ok()?;
    Some((
        Version::parse(v.get("latest_version")?.as_str()?)?,
        v.get("last_checked_at")?.as_u64()?,
    ))
}

fn write_cache(latest: &Version) -> Option<()> {
    let path = cache_path()?;
    std::fs::create_dir_all(path.parent()?).ok()?;
    std::fs::write(
        &path,
        serde_json::to_vec_pretty(
            &json!({ "latest_version": latest.to_string(), "last_checked_at": now() }),
        )
        .ok()?,
    )
    .ok()
}

fn needs_refresh(cache: Option<&(Version, u64)>, now: u64) -> bool {
    cache.is_none_or(|(_, t)| t + CACHE_MAX_AGE_SECS < now)
}

/// A pending update check, started before the command runs.
pub struct Check {
    latest: Option<Version>,
    refresh: Option<std::thread::JoinHandle<()>>,
}

/// Whether this run may show a notice: a terminal, no CI or agent mode, no
/// opt-out (`M365_NO_UPDATE_NOTIFIER`, upstream's `CLIMICROSOFT365_NOUPDATE`),
/// and not `update`, `skill`, completion or output-free runs.
fn eligible(args: &[String], terminal: bool, var: impl Fn(&str) -> Option<String>) -> bool {
    let set = |n: &str| var(n).is_some_and(|v| flag(&v));
    if !terminal
        || set("CI")
        || set("M365_NO_UPDATE_NOTIFIER")
        || set("CLIMICROSOFT365_NOUPDATE")
        || set("M365_AGENT")
        || set("M365_PARITY_MOCK")
    {
        return false;
    }
    if args
        .iter()
        .any(|a| a == "--agent" || a == "--completion" || a == "--compgen")
    {
        return false;
    }
    if args
        .windows(2)
        .any(|w| (w[0] == "--output" || w[0] == "-o") && w[1] == "none")
    {
        return false;
    }
    !matches!(
        args.iter()
            .find(|a| !a.starts_with('-'))
            .map(String::as_str),
        None | Some("update" | "skill")
    )
}

/// Starts the check for this run (release builds only).
pub fn start(args: &[String]) -> Option<Check> {
    if cfg!(debug_assertions) {
        return None;
    }
    let terminal = std::io::IsTerminal::is_terminal(&std::io::stderr());
    if !eligible(args, terminal, |n| std::env::var(n).ok()) {
        return None;
    }
    let cache = read_cache();
    let current = current_version();
    let refresh = needs_refresh(cache.as_ref(), now()).then(|| {
        // record the attempt first, so an offline or slow network does not
        // make every following command wait for GitHub again
        let known = cache
            .as_ref()
            .map_or_else(|| current.clone(), |(v, _)| v.clone());
        let _ = write_cache(&known);
        std::thread::spawn(|| {
            let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };
            rt.block_on(async {
                if let Ok(client) = http_client()
                    && let Ok(latest) = fetch_latest_version(&client).await
                {
                    let _ = write_cache(&latest);
                }
            });
        })
    });
    Some(Check {
        latest: cache.map(|(v, _)| v).filter(|v| *v > current),
        refresh,
    })
}

/// Lets a running refresh finish briefly, then prints the notice.
pub fn finish(check: Option<Check>) {
    let Some(check) = check else { return };
    if let Some(refresh) = check.refresh {
        let deadline = std::time::Instant::now() + FINISH_WAIT;
        while !refresh.is_finished() && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    if let Some(latest) = check.latest {
        eprintln!("{}", notice(&current_version(), &latest));
    }
}

fn notice(current: &Version, latest: &Version) -> String {
    format!(
        "Update available: m365 v{current} -> v{latest}. Run `{}`.",
        update_hint()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn compares_versions_like_semver() {
        assert!(v("12.2.0") > v("12.1.9"));
        assert!(v("v12.10.0") > v("12.9.0"));
        assert!(v("12.1.0") > v("12.1.0-beta.1"));
        assert_eq!(v("v1.2.3"), v("1.2.3+build"));
        assert!(Version::parse("release-1").is_none());
        assert!(Version::parse("1.2").is_none());
    }

    #[test]
    fn custom_update_command_gets_the_release_placeholders() {
        let t: Vec<String> = [
            "mise",
            "use",
            "-g",
            "github:racetozero/microsoft-365-cli@{version}",
            "{installSpec}",
            "{packageName}",
        ]
        .map(String::from)
        .into();
        assert_eq!(
            update_command_argv(&t, &v("12.2.0")),
            [
                "mise",
                "use",
                "-g",
                "github:racetozero/microsoft-365-cli@12.2.0",
                "m365@12.2.0",
                "m365"
            ]
        );
    }

    #[test]
    fn pins_the_installer_to_the_release_tag() {
        let url = installer_url(&v("12.2.0"));
        assert!(
            url.starts_with(
                "https://raw.githubusercontent.com/racetozero/microsoft-365-cli/v12.2.0/"
            ),
            "{url}"
        );
    }

    #[test]
    fn refreshes_only_missing_or_old_cache_data() {
        let now = 1_000_000;
        assert!(!needs_refresh(Some(&(v("1.0.0"), now - 19 * 3600)), now));
        assert!(needs_refresh(Some(&(v("1.0.0"), now - 21 * 3600)), now));
        assert!(needs_refresh(None, now));
    }

    #[test]
    fn checks_only_in_a_terminal_without_ci_agents_or_opt_outs() {
        let a = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        let none = |_: &str| None;
        assert!(eligible(&a(&["entra", "user", "list"]), true, none));
        assert!(!eligible(&a(&["entra", "user", "list"]), false, none));
        assert!(!eligible(
            &a(&["entra", "user", "list", "--agent"]),
            true,
            none
        ));
        assert!(!eligible(
            &a(&["entra", "user", "list", "-o", "none"]),
            true,
            none
        ));
        assert!(!eligible(&a(&["update"]), true, none));
        assert!(!eligible(&a(&["skill"]), true, none));
        assert!(!eligible(&a(&[]), true, none));
        let upstream_off = |n: &str| (n == "CLIMICROSOFT365_NOUPDATE").then(|| "1".to_owned());
        assert!(!eligible(&a(&["status"]), true, upstream_off));
        let ci = |n: &str| (n == "CI").then(|| "true".to_owned());
        assert!(!eligible(&a(&["status"]), true, ci));
    }

    #[cfg(not(windows))]
    #[tokio::test]
    async fn runs_the_installer_into_the_staging_bin_folder() {
        let staging = tempfile::tempdir().unwrap();
        let installer = staging.path().join("install.sh");
        std::fs::write(&installer, "#!/bin/sh\nset -eu\nmkdir -p \"$M365_INSTALL_DIR\"\nprintf '%s' \"$M365_VERSION\" > \"$M365_INSTALL_DIR/m365\"\n").unwrap();
        let bin = run_staged_installer(&installer, &v("12.2.0"), staging.path())
            .await
            .unwrap();
        assert_eq!(std::fs::read_to_string(bin).unwrap(), "12.2.0");
    }

    #[cfg(unix)]
    #[test]
    fn detects_homebrew_from_the_resolved_executable() {
        // the test binary is not under a Cellar
        assert!(!installed_by_homebrew());
        assert_eq!(update_hint(), "m365 update");
    }
}

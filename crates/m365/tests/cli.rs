//! End-to-end checks of the `m365` binary: each test runs it in a fresh
//! home folder, without network access or a terminal.

use std::process::{Command, Output, Stdio};

fn m365(args: &[&str]) -> Output {
    let home = tempfile::tempdir().expect("temp home");
    Command::new(env!("CARGO_BIN_EXE_m365"))
        .args(args)
        .current_dir(home.path())
        .env_clear()
        .envs(std::env::vars_os().filter(|(k, _)| {
            k.eq_ignore_ascii_case("PATH") || k.eq_ignore_ascii_case("SystemRoot")
        }))
        .env("HOME", home.path())
        .env("USERPROFILE", home.path())
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("M365_NO_UPDATE_NOTIFIER", "1")
        .stdin(Stdio::null())
        .output()
        .expect("run m365")
}

fn stdout(o: &Output) -> &str {
    std::str::from_utf8(&o.stdout).expect("utf-8 stdout")
}

fn stderr(o: &Output) -> &str {
    std::str::from_utf8(&o.stderr).expect("utf-8 stderr")
}

#[test]
fn prints_the_version() {
    let o = m365(&["--version"]);
    assert_eq!(
        stdout(&o).trim_end(),
        concat!("m365 ", env!("CARGO_PKG_VERSION"))
    );
    let o = m365(&["version"]);
    assert_eq!(
        stdout(&o).trim_end(),
        format!("\"v{}\"", m365_core::manifest::UPSTREAM_VERSION)
    );
    assert!(o.status.success());
}

#[test]
fn help_lists_the_suites_and_the_command_options() {
    let o = m365(&["--help"]);
    assert!(o.status.success());
    assert!(stdout(&o).contains("m365 entra user get"));
    let o = m365(&["spo", "site", "get", "--help"]);
    assert!(o.status.success());
    assert!(stdout(&o).contains("--url"), "{}", stdout(&o));
}

#[test]
fn agent_mode_reports_missing_input_as_json() {
    let o = m365(&["entra", "user", "get", "--agent"]);
    assert_eq!(o.status.code(), Some(3));
    assert_eq!(
        stdout(&o).trim_end(),
        r#"{"ok":false,"command":"entra user get","error":{"code":"input_required","message":"Specify either 'id', 'userName', or 'email'.","input":{"options":["id","userName","email"]}}}"#
    );
    let o = m365(&["spo", "site", "get", "--agent"]);
    assert_eq!(o.status.code(), Some(3));
    assert!(
        stdout(&o).contains(r#""input":{"arg":"url"}"#),
        "{}",
        stdout(&o)
    );
}

#[test]
fn agent_mode_reports_an_unknown_command() {
    let o = m365(&["nope", "--agent"]);
    assert_eq!(o.status.code(), Some(4));
    assert!(stdout(&o).contains(r#""code":"unknown""#), "{}", stdout(&o));
}

#[test]
fn validation_errors_name_the_option_and_point_to_help() {
    let o = m365(&["spo", "site", "get", "--url", "foo"]);
    assert_eq!(o.status.code(), Some(1));
    assert_eq!(
        stderr(&o).lines().collect::<Vec<_>>(),
        [
            "Error: url: Specify a valid SharePoint site URL",
            "Run 'm365 spo site get -h' for help."
        ]
    );
}

#[test]
fn skill_list_includes_every_suite() {
    let o = m365(&["skill", "--list", "--agent"]);
    assert!(o.status.success());
    for name in ["m365", "m365-spo", "m365-entra", "m365-teams", "m365-spfx"] {
        assert!(
            stdout(&o).contains(&format!(r#""name":"{name}""#)),
            "{name}"
        );
    }
}

#[test]
fn completion_replies_follow_the_typed_words() {
    let o = m365(&["--compgen", "3", "m365", "m365 spo site get --output "]);
    assert_eq!(
        stdout(&o).lines().collect::<Vec<_>>(),
        ["csv", "json", "md", "text", "none"]
    );
    let o = m365(&["--compgen", "2", "m365", "m365 entra "]);
    let replies: Vec<&str> = stdout(&o).lines().collect();
    assert!(
        replies.contains(&"user") && replies.contains(&"sp"),
        "{replies:?}"
    );
    let o = m365(&["--compgen", "1", "m365", "m365 "]);
    let roots: Vec<&str> = stdout(&o).lines().collect();
    assert!(
        roots.contains(&"spo") && roots.contains(&"entra"),
        "{roots:?}"
    );
    let o = m365(&["--compgen", "4", "m365", "m365 spo site get --url x --"]);
    let opts: Vec<&str> = stdout(&o).lines().collect();
    assert!(
        !opts.contains(&"--url") && opts.contains(&"--help"),
        "{opts:?}"
    );
}

#[test]
fn completion_prints_the_shell_script() {
    let o = m365(&["--completion"]);
    assert!(o.status.success());
    assert!(stdout(&o).contains("--compgen"), "{}", stdout(&o));
}

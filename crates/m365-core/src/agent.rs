//! Agent mode (`--agent` or `M365_AGENT=1`), modelled on `atl --agent`.
//!
//! Every command prints exactly one compact JSON line on stdout:
//! `{"ok":true,"command":"...","data":...}` on success and
//! `{"ok":false,"command":"...","error":{"code":...,"message":...}}` on
//! failure. Nothing prompts; destructive commands fail with
//! `confirmation_required` until `--force` is added, and missing values fail
//! with `input_required` listing the option and its allowed values.
//!
//! Exit codes: 0 ok, 1 command or API error, 2 authentication, 3 invalid or
//! missing input (including confirmation), 4 unknown command, 5 internal.

use crate::json::{Obj, stringify};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

static ENABLED: AtomicBool = AtomicBool::new(false);

pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

/// Decides agent mode from argv and the environment, and removes `--agent`
/// from the arguments so no command sees it as an option.
pub fn take_flag(args: &mut Vec<String>) -> bool {
    let before = args.len();
    args.retain(|a| a != "--agent");
    let env = std::env::var("M365_AGENT").is_ok_and(|v| !v.is_empty() && v != "0" && v != "false");
    let on = env || args.len() != before;
    set_enabled(on);
    on
}

fn data() -> &'static Mutex<Vec<Value>> {
    static D: OnceLock<Mutex<Vec<Value>>> = OnceLock::new();
    D.get_or_init(|| Mutex::new(Vec::new()))
}

fn command() -> &'static Mutex<String> {
    static C: OnceLock<Mutex<String>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(String::new()))
}

pub fn set_command(name: &str) {
    *command().lock().unwrap() = name.to_string();
}

/// Records one `logger.log` value.
pub fn push(v: Value) {
    data().lock().unwrap().push(v);
}

fn envelope_data() -> Value {
    let mut d = data().lock().unwrap();
    match d.len() {
        0 => Value::Null,
        1 => d.pop().unwrap(),
        _ => Value::Array(std::mem::take(&mut *d)),
    }
}

pub fn print_ok() {
    let mut o = Obj::new();
    o.insert("ok".into(), Value::Bool(true));
    o.insert(
        "command".into(),
        Value::from(command().lock().unwrap().clone()),
    );
    o.insert("data".into(), envelope_data());
    crate::log::stdout(&stringify(&Value::Object(o)));
}

pub fn print_ok_with(data: Value) {
    push(data);
    print_ok();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Code {
    Failed,
    Api,
    Auth,
    Invalid,
    InputRequired,
    ConfirmationRequired,
    Unknown,
    Internal,
}

impl Code {
    pub fn as_str(self) -> &'static str {
        match self {
            Code::Failed => "failed",
            Code::Api => "api",
            Code::Auth => "auth",
            Code::Invalid => "invalid",
            Code::InputRequired => "input_required",
            Code::ConfirmationRequired => "confirmation_required",
            Code::Unknown => "unknown",
            Code::Internal => "internal",
        }
    }

    pub fn exit(self) -> i32 {
        match self {
            Code::Failed | Code::Api => 1,
            Code::Auth => 2,
            Code::Invalid | Code::InputRequired | Code::ConfirmationRequired => 3,
            Code::Unknown => 4,
            Code::Internal => 5,
        }
    }
}

/// Prints the failure envelope and returns the exit code.
pub fn print_error(code: Code, message: &Value, extra: Obj) -> i32 {
    let mut e = Obj::new();
    e.insert("code".into(), Value::from(code.as_str()));
    e.insert(
        "message".into(),
        match message {
            Value::String(_) => message.clone(),
            other => Value::from(crate::json::to_js_string(other)),
        },
    );
    if !message.is_string() && !message.is_null() {
        e.insert("details".into(), message.clone());
    }
    for (k, v) in extra {
        e.insert(k, v);
    }
    let mut o = Obj::new();
    o.insert("ok".into(), Value::Bool(false));
    o.insert(
        "command".into(),
        Value::from(command().lock().unwrap().clone()),
    );
    o.insert("error".into(), Value::Object(e));
    crate::log::stdout(&stringify(&Value::Object(o)));
    code.exit()
}

/// Classifies a command failure by its message and HTTP status.
pub fn classify(message: &str, status: Option<u16>) -> Code {
    let m = message.to_ascii_lowercase();
    if m.contains("log in to microsoft 365 first")
        || m.contains("your login has expired")
        || m.contains("access token")
        || m.contains("aadsts")
        || status == Some(401)
    {
        return Code::Auth;
    }
    if status.is_some() {
        return Code::Api;
    }
    Code::Failed
}

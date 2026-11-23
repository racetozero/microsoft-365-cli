//! Process-wide output, like upstream's `cli.log`, `cli.error` and the
//! `Logger` passed to commands.

use crate::config::{self, names};
use crate::json::Obj;
use crate::manifest::CmdMeta;
use crate::output::{self, OutputOptions};
use serde_json::Value;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Default)]
pub struct RunState {
    pub debug: bool,
    pub verbose: bool,
    pub output: String,
    pub query: Option<String>,
    pub options: Obj,
    pub meta: Option<&'static CmdMeta>,
    pub command_name: String,
    /// Overrides the command's default text-output properties.
    pub default_properties: Option<&'static [&'static str]>,
}

fn state() -> &'static Mutex<RunState> {
    static S: OnceLock<Mutex<RunState>> = OnceLock::new();
    S.get_or_init(|| {
        Mutex::new(RunState {
            output: "json".into(),
            ..Default::default()
        })
    })
}

/// `cli.commandToExecute.defaultProperties = ...` set by a command at run time.
pub fn set_default_properties(props: Vec<String>) {
    let leaked: Vec<&'static str> = props
        .into_iter()
        .map(|p| &*Box::leak(p.into_boxed_str()))
        .collect();
    state().lock().unwrap().default_properties = Some(Box::leak(leaked.into_boxed_slice()));
}

pub fn set_state(s: RunState) {
    *state().lock().unwrap() = s;
}

pub fn get_state() -> RunState {
    state().lock().unwrap().clone()
}

pub fn debug() -> bool {
    state().lock().unwrap().debug
}

pub fn verbose() -> bool {
    let s = state().lock().unwrap();
    s.verbose || s.debug
}

pub fn output_none() -> bool {
    state().lock().unwrap().output == "none"
}

/// Captured output for `executeCommandWithOutput`.
struct Capture {
    stdout: Vec<String>,
    stderr: Vec<String>,
}

fn capture() -> &'static Mutex<Vec<Capture>> {
    static C: OnceLock<Mutex<Vec<Capture>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn begin_capture() {
    capture().lock().unwrap().push(Capture {
        stdout: Vec::new(),
        stderr: Vec::new(),
    });
}

/// Ends the innermost capture and returns (stdout, stderr) joined by EOL.
pub fn end_capture() -> (String, String) {
    let c = capture().lock().unwrap().pop();
    match c {
        Some(c) => (
            c.stdout.join(crate::term::EOL),
            c.stderr.join(crate::term::EOL),
        ),
        None => (String::new(), String::new()),
    }
}

/// Adds `s` to the innermost capture of stdout or stderr, if one is open.
fn capturing(s: &str, stderr: bool) -> bool {
    let mut c = capture().lock().unwrap();
    let Some(cap) = c.last_mut() else {
        return false;
    };
    if stderr {
        &mut cap.stderr
    } else {
        &mut cap.stdout
    }
    .push(s.to_string());
    true
}

/// `console.log(message)`.
pub fn stdout(s: &str) {
    let mut out = std::io::stdout().lock();
    let _ = out.write_all(s.as_bytes());
    let _ = out.write_all(b"\n");
    let _ = out.flush();
}

/// `console.error(message)`.
pub fn raw_stderr(s: &str) {
    let mut out = std::io::stderr().lock();
    let _ = out.write_all(s.as_bytes());
    let _ = out.write_all(b"\n");
    let _ = out.flush();
}

/// `cli.error`: stderr, or stdout when `errorOutput` is `stdout`.
pub fn cli_error(s: &str) {
    if config::get_string(names::ERROR_OUTPUT, "stderr") == "stdout" {
        stdout(s)
    } else {
        raw_stderr(s)
    }
}

/// `logger.log`: formats `v` with the command's output settings.
pub fn log(v: Value) -> Result<(), String> {
    let s = get_state();
    if crate::agent::enabled() && capture().lock().unwrap().last().is_none() {
        let v = match &s.query {
            Some(q) => crate::query::search(&v, q).map_err(|e| format!("JMESPath query error. {e}. See https://jmespath.org/specification.html for more information"))?,
            None => v,
        };
        crate::agent::push(v);
        return Ok(());
    }
    if capture().lock().unwrap().last().is_some() {
        let opts = OutputOptions {
            output: s.output.clone(),
            query: s.query.clone(),
            options: s.options.clone(),
        };
        let text = output::format_output(s.meta, s.default_properties, &s.command_name, v, &opts)?;
        capturing(&text, false);
        return Ok(());
    }
    if s.output == "none" {
        return Ok(());
    }
    let opts = OutputOptions {
        output: s.output.clone(),
        query: s.query.clone(),
        options: s.options.clone(),
    };
    let text = output::format_output(s.meta, s.default_properties, &s.command_name, v, &opts)?;
    stdout(&text);
    Ok(())
}

/// `logger.logRaw`.
pub fn log_raw(text: &str) {
    if capturing(text, false) {
        return;
    }
    if crate::agent::enabled() {
        crate::agent::push(Value::from(text));
        return;
    }
    if output_none() {
        return;
    }
    stdout(text);
}

/// `logger.logToStderr`.
pub fn log_stderr(text: &str) {
    if capturing(text, true) {
        return;
    }
    if output_none() {
        return;
    }
    cli_error(text);
}

/// `logger.logToStderr` with a value printed the way `console.error` prints it.
pub fn log_stderr_value(v: &Value) {
    log_stderr(&inspect(v));
}

/// A reasonable stand-in for Node's `util.inspect` used by `console.error(obj)`.
pub fn inspect(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => crate::json::stringify_pretty(other, "  "),
    }
}

pub fn warn(text: &str) {
    log_stderr(&crate::term::yellow(text));
}

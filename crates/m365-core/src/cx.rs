//! The context a command handler runs with: its parsed options and the
//! logger.

use crate::error::{Error, Result};
use crate::json::{Obj, to_js_string};
use crate::manifest::CmdMeta;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

pub type BoxFut<'a, T = ()> = Pin<Box<dyn Future<Output = Result<T>> + 'a>>;
pub type Handler = for<'a> fn(&'a mut Cx) -> BoxFut<'a>;
/// Extra validation of the parsed options (`Err(message)` fails).
pub type Validate = fn(&Obj) -> std::result::Result<(), String>;

/// A command implementation registered with the runner.
#[derive(Clone, Copy)]
pub struct Spec {
    pub name: &'static str,
    pub run: Handler,
    pub hooks: crate::schema::Hooks,
    /// Validation of legacy (non-zod) commands after the built-in checks,
    /// or extra validation that needs the parsed options.
    pub validate: Option<Validate>,
}

pub struct Cx {
    pub meta: &'static CmdMeta,
    /// Options after validation, in schema order.
    pub opts: Obj,
    /// The name the command was invoked with (an alias or the name).
    pub used_name: String,
    /// `AppCommand.appId`: the Entra app picked from `.m365rc.json`.
    pub app_id: Option<String>,
}

impl Cx {
    pub fn val(&self, k: &str) -> Option<&Value> {
        self.opts.get(k).filter(|v| !v.is_null() || k.is_empty())
    }

    /// Whether the option was specified (`!== undefined`).
    pub fn has(&self, k: &str) -> bool {
        self.opts.contains_key(k)
    }

    /// The option as a string (numbers and booleans converted as JavaScript does).
    pub fn str(&self, k: &str) -> Option<String> {
        self.opts.get(k).map(to_js_string)
    }

    /// The option as a string when it is truthy.
    pub fn s(&self, k: &str) -> Option<String> {
        self.opts
            .get(k)
            .filter(|v| crate::json::truthy(v))
            .map(to_js_string)
    }

    /// The first truthy option of `keys` as a string, or `"undefined"`
    /// (`options.a || options.b` in a template literal).
    pub fn first_u(&self, keys: &[&str]) -> String {
        keys.iter()
            .find_map(|k| self.s(k))
            .unwrap_or_else(|| "undefined".into())
    }

    pub fn bool(&self, k: &str) -> bool {
        self.opts.get(k).is_some_and(crate::json::truthy)
    }

    /// `true`, `false` or unspecified.
    pub fn opt_bool(&self, k: &str) -> Option<bool> {
        match self.opts.get(k)? {
            Value::Bool(b) => Some(*b),
            Value::String(s) => Some(s == "true"),
            other => Some(crate::json::truthy(other)),
        }
    }

    pub fn num(&self, k: &str) -> Option<f64> {
        match self.opts.get(k)? {
            Value::Number(n) => n.as_f64(),
            Value::String(s) => Some(crate::json::js_number(s)),
            _ => None,
        }
    }

    pub fn debug(&self) -> bool {
        crate::log::debug()
    }

    pub fn verbose(&self) -> bool {
        crate::log::verbose()
    }

    /// `logger.log`.
    pub fn log(&self, v: Value) -> Result<()> {
        crate::log::log(v).map_err(Error::msg)
    }

    /// `logger.logRaw`.
    pub fn log_raw(&self, s: &str) {
        crate::log::log_raw(s)
    }

    /// `logger.logToStderr`.
    pub fn stderr(&self, s: &str) {
        crate::log::log_stderr(s)
    }

    /// Writes to stderr when verbose.
    pub fn verbose_log(&self, s: &str) {
        if self.verbose() {
            self.stderr(s);
        }
    }

    /// Writes to stderr when debugging.
    pub fn debug_log(&self, s: &str) {
        if self.debug() {
            self.stderr(s);
        }
    }

    /// `this.warn(logger, message)`.
    pub fn warn(&self, s: &str) {
        crate::log::warn(s)
    }

    /// `cli.promptForConfirmation({ message })`. In agent mode fails with
    /// `confirmation_required`.
    pub fn confirm(&self, message: &str) -> Result<bool> {
        if crate::agent::enabled() {
            return Err(Error::Command {
                message: Value::from(format!("{message} Add --force to confirm.")),
                code: Some(-3),
                status: None,
            });
        }
        let r = crate::prompt::confirm(message, true)?;
        crate::log::cli_error("");
        Ok(r)
    }

    /// The common `if (force) run() else if (await confirm(message)) run()`
    /// shape: true when the command should proceed.
    pub fn proceed(&self, force_option: &str, message: &str) -> Result<bool> {
        if self.bool(force_option) {
            return Ok(true);
        }
        self.confirm(message)
    }

    /// `cli.handleMultipleResultsFound(message, values)`.
    pub fn handle_multiple_results(&self, message: &str, values: &Obj) -> Result<Value> {
        crate::runner::handle_multiple_results(message, values)
    }

    /// `cli.promptForSelection`.
    pub fn select(&self, message: &str, choices: &[String]) -> Result<usize> {
        let r = crate::prompt::select(message, choices)?;
        crate::log::cli_error("");
        Ok(r)
    }

    /// `cli.promptForInput`.
    pub fn input(&self, message: &str) -> Result<String> {
        let r = crate::prompt::input(message)?;
        crate::log::cli_error("");
        Ok(r)
    }
}

/// Wraps an `async fn(&mut Cx) -> Result<()>` into a [`Handler`].
#[macro_export]
macro_rules! handler {
    ($f:path) => {{
        fn h(cx: &mut $crate::cx::Cx) -> $crate::cx::BoxFut<'_> {
            Box::pin($f(cx))
        }
        h as $crate::cx::Handler
    }};
}

/// Declares a command implementation: `spec!("entra user get", run)`,
/// optionally with `field = f`, `refine = f` and `validate = f` hooks.
#[macro_export]
macro_rules! spec {
    ($name:expr, $f:path) => {
        $crate::cx::Spec { name: $name, run: $crate::handler!($f), hooks: $crate::schema::Hooks { field_check: None, refine: None }, validate: None }
    };
    ($name:expr, $f:path, $($k:ident = $v:expr),+ $(,)?) => {{
        #[allow(unused_mut)]
        let mut s = $crate::cx::Spec { name: $name, run: $crate::handler!($f), hooks: $crate::schema::Hooks { field_check: None, refine: None }, validate: None };
        $( $crate::spec!(@set s, $k, $v); )+
        s
    }};
    (@set $s:ident, field, $v:expr) => { $s.hooks.field_check = Some($v) };
    (@set $s:ident, refine, $v:expr) => { $s.hooks.refine = Some($v) };
    (@set $s:ident, validate, $v:expr) => { $s.validate = Some($v) };
}

impl Cx {
    /// `logger.log(undefined)`: an empty line.
    pub fn log_undefined(&self) -> Result<()> {
        if crate::agent::enabled() {
            crate::agent::push(Value::Null);
            return Ok(());
        }
        if !crate::log::output_none() {
            crate::log::stdout("");
        }
        Ok(())
    }

    /// A confirmation prompt that defaults to "no" (fails in agent mode).
    pub fn confirm_default(&self, message: &str) -> Result<bool> {
        if crate::agent::enabled() {
            return self.confirm(message);
        }
        let r = crate::prompt::confirm(message, false)?;
        crate::log::cli_error("");
        Ok(r)
    }
}

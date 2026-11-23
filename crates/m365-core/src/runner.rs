//! The command runner: a port of `cli.execute` (src/cli/cli.ts) together
//! with `Command.action`, `Command.validate` and the base-class checks.

use crate::agent::{self, Code};
use crate::args::{self, Flags};
use crate::auth;
use crate::config::{self, names};
use crate::cx::{Cx, Spec};
use crate::error::Error;
use crate::help;
use crate::json::{Obj, stringify, to_js_string};
use crate::log::{self, RunState};
use crate::manifest::{self, CmdMeta, OptType, RefineCode, base};
use crate::schema;
use crate::term::{self, EOL};
use serde_json::Value;
use std::sync::OnceLock;
use std::time::Instant;

static SPECS: OnceLock<&'static [Spec]> = OnceLock::new();

pub fn specs() -> &'static [Spec] {
    SPECS.get().copied().unwrap_or(&[])
}

pub fn spec(name: &str) -> Option<&'static Spec> {
    specs().iter().find(|s| s.name == name)
}

/// Process exit after printing an error (`cli.closeWithError`).
struct Exit(i32);

#[derive(Default)]
struct Timings {
    api: f64,
    core: f64,
    command: f64,
    options: f64,
    total: f64,
    validation: f64,
}

/// Entry point for the binary: runs one invocation and returns the exit code.
pub fn main(specs: &'static [Spec]) -> i32 {
    let _ = SPECS.set(specs);
    #[cfg(feature = "parity")]
    if std::env::var_os("M365_PARITY_LIST").is_some() {
        list_specs();
        return 0;
    }
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    agent::take_flag(&mut args);
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    match rt.block_on(execute(args)) {
        Ok(code) => code,
        Err(Exit(code)) => code,
    }
}

#[cfg(feature = "parity")]
fn list_specs() {
    let mut out = Vec::new();
    for s in specs() {
        let m = manifest::find(s.name);
        let needs_field = m.is_some_and(|m| {
            m.fields.iter().any(|f| {
                f.checks
                    .iter()
                    .any(|c| matches!(c, manifest::Check::Custom(..)))
            })
        });
        let needs_refine = m.is_some_and(|m| m.refines.iter().any(|r| r.rule.is_none()));
        let needs_validate = m
            .and_then(|m| m.legacy.as_ref())
            .is_some_and(|l| l.custom_validators > 0);
        out.push(serde_json::json!({
            "name": s.name, "field": s.hooks.field_check.is_some(), "refine": s.hooks.refine.is_some(), "validate": s.validate.is_some(),
            "needsField": needs_field, "needsRefine": needs_refine, "needsValidate": needs_validate
        }));
    }
    log::stdout(&stringify(&Value::Array(out)));
}

fn command_flags(meta: &CmdMeta, raw: &[String]) -> (Flags, Vec<&'static str>) {
    let mut f = Flags::default();
    let mut booleans: Vec<&'static str> = Vec::new();
    match &meta.legacy {
        None => {
            for o in meta.options {
                match o.ty {
                    OptType::String => {
                        f.strings.insert(o.name.to_string());
                    }
                    OptType::Boolean => {
                        f.bools.insert(o.name.to_string());
                        booleans.push(o.name);
                    }
                    OptType::Number => {
                        f.numbers.insert(o.name.to_string());
                    }
                    OptType::Unknown => {}
                }
            }
        }
        Some(l) => {
            for s in l.strings {
                f.strings.insert(s.to_string());
            }
            for b in l.booleans {
                if raw.iter().any(|a| {
                    a.strip_prefix("--")
                        .or_else(|| a.strip_prefix('-'))
                        .is_some_and(|f| f == *b)
                }) {
                    f.bools.insert(b.to_string());
                }
                booleans.push(b);
            }
        }
    }
    for o in meta.options {
        if let (Some(s), Some(l)) = (o.short, o.long) {
            f.aliases.insert(l.to_string(), s.to_string());
        }
    }
    (f, booleans)
}

/// The issues as one message, a line each.
fn render(issues: &[schema::Issue], missing_as_required: bool) -> String {
    issues
        .iter()
        .map(|i| i.render(missing_as_required))
        .collect::<Vec<_>>()
        .join(EOL)
}

fn positional_name(p: &[Value]) -> String {
    p.iter().map(to_js_string).collect::<Vec<_>>().join(" ")
}

/// `cli.closeWithError`.
fn close_with_error(
    error: &Error,
    output: Option<&str>,
    meta: Option<&CmdMeta>,
    show_help: bool,
) -> Exit {
    let mut exit = 1;
    if agent::enabled() {
        return Exit(agent_error(error, meta));
    }
    if output == Some("none") {
        return Exit(1);
    }
    // failures the command did not translate go through the helper its
    // commandAction uses; without one, upstream prints `${error}`
    use manifest::ErrStyle;
    let converted = match (error, meta.map_or(ErrStyle::None, |m| m.errors)) {
        (Error::Command { .. }, _) => None,
        (_, ErrStyle::Json) => Some(error.clone().odata_json()),
        (_, ErrStyle::Viva) => Some(error.clone().viva()),
        (_, ErrStyle::OData) => Some(error.clone().odata()),
        (_, ErrStyle::Plain) => Some(error.clone().plain()),
        (Error::Thrown(v), ErrStyle::None) => Some(Error::value(v.clone())),
        (Error::Js(m), ErrStyle::None) => Some(Error::msg(format!("Error: {m}"))),
        (Error::Syntax(m), ErrStyle::None) => Some(Error::msg(format!("SyntaxError: {m}"))),
        (_, ErrStyle::None) => None,
    };
    let error = converted.as_ref().unwrap_or(error);
    let message: Value = match error {
        Error::Command { message, code, .. } => {
            if let Some(c) = code
                && *c > 0
            {
                exit = *c;
            }
            message.clone()
        }
        Error::Http(h) => Value::from(format!("AxiosError: {}", h.message())),
        Error::Network(m) | Error::Js(m) | Error::Syntax(m) => Value::from(m.clone()),
        Error::Thrown(v) => v.clone(),
    };
    let text = if (output.is_none() || output == Some("json"))
        && !config::get_bool(names::PRINT_ERRORS_AS_PLAIN_TEXT, true)
    {
        let mut o = Obj::new();
        o.insert("error".into(), message);
        stringify(&Value::Object(o))
    } else {
        term::red(&format!("Error: {}", to_js_string(&message)))
    };
    log::cli_error(&text);
    if show_help
        && config::get_bool(names::SHOW_HELP_ON_FAILURE, show_help)
        && let Some(m) = meta
    {
        log::cli_error(&format!("Run 'm365 {} -h' for help.", m.name));
    }
    Exit(exit)
}

fn agent_error(error: &Error, _meta: Option<&CmdMeta>) -> i32 {
    match error {
        Error::Command {
            message,
            code: Some(-3),
            ..
        } => agent::print_error(Code::ConfirmationRequired, message, Obj::new()),
        Error::Command {
            message,
            code: Some(-4),
            ..
        } => agent::print_error(Code::Invalid, message, Obj::new()),
        Error::Command {
            message, status, ..
        } => {
            let code = agent::classify(&to_js_string(message), *status);
            let mut extra = Obj::new();
            if let Some(s) = status {
                extra.insert("status".into(), Value::from(*s));
            }
            agent::print_error(code, message, extra)
        }
        Error::Http(h) => {
            let mut extra = Obj::new();
            extra.insert("status".into(), Value::from(h.status));
            let msg = Error::Http(h.clone()).odata_json();
            agent::print_error(
                agent::classify(&msg.js_message(), Some(h.status)),
                &Value::from(msg.js_message()),
                extra,
            )
        }
        Error::Network(m) | Error::Js(m) | Error::Syntax(m) => {
            agent::print_error(Code::Failed, &Value::from(m.clone()), Obj::new())
        }
        Error::Thrown(v) => agent::print_error(
            Code::Failed,
            &Error::Thrown(v.clone()).odata_json().command_message(),
            Obj::new(),
        ),
    }
}

fn invalid(msg: impl Into<String>) -> Error {
    Error::Command {
        message: Value::from(msg.into()),
        code: Some(-4),
        status: None,
    }
}

/// `getHelpMode`.
fn help_mode(first: &Obj) -> Result<String, Exit> {
    let v = first.get("h").or_else(|| first.get("help"));
    // every section unless `helpMode` is set
    let default = || config::get_string(names::HELP_MODE, "full");
    match v {
        None | Some(Value::Bool(_)) | Some(Value::Number(_)) | Some(Value::Null) => Ok(default()),
        Some(Value::String(s)) => {
            let l = s.to_lowercase();
            if help::HELP_MODES.contains(&l.as_str()) {
                Ok(l)
            } else {
                Err(close_with_error(
                    &Error::msg(format!(
                        "Unknown help mode {s}. Allowed values are {} ",
                        help::HELP_MODES.join(", ")
                    )),
                    first.get("output").and_then(Value::as_str),
                    None,
                    false,
                ))
            }
        }
        Some(_) => Ok(default()),
    }
}

fn print_help(meta: Option<&'static CmdMeta>, mode: &str, current: &str) {
    let group = if help::is_group(current) { current } else { "" };
    if agent::enabled() {
        let data = match meta {
            Some(m) => help::command_help_json(m),
            None => help::group_help_json(group),
        };
        agent::set_command(meta.map(|m| m.name).unwrap_or("help"));
        agent::print_ok_with(data);
        return;
    }
    match meta {
        Some(m) => {
            if config::get_string(names::HELP_TARGET, "console") == "web" {
                let _ = crate::util::open_browser(&help::online_url(m));
            } else if let Some(h) = help::command_help(m, mode) {
                log::stdout(&h);
            }
        }
        None => {
            if !current.is_empty() && group.is_empty() {
                log::cli_error(&term::red(&format!(
                    "Command '{current}' was not found. Below you can find the commands and command groups you can use. For detailed information on a command group, use 'm365 [command group] --help'."
                )));
            }
            log::stdout(&help::group_help(group));
        }
    }
}

/// `loadOptionValuesFromFiles`.
fn load_from_files(opts: &mut Obj) {
    for (_, v) in opts.iter_mut() {
        if let Value::String(s) = v
            && let Some(path) = s.strip_prefix('@')
            && std::path::Path::new(path).exists()
            && let Ok(c) = std::fs::read_to_string(path)
        {
            *v = Value::String(c);
        }
    }
}

/// `loadOptionsFromContext`.
fn load_from_context(meta: &CmdMeta, debug: bool) -> Result<Obj, Error> {
    let path = std::path::Path::new(".m365rc.json");
    let mut found = Obj::new();
    if !path.exists() {
        return Ok(found);
    }
    if debug {
        log::cli_error("found .m365rc.json file");
    }
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let rc: Value = if text.is_empty() {
        Value::Object(Obj::new())
    } else {
        serde_json::from_str(&text).map_err(|_| Error::msg("Error parsing .m365rc.json"))?
    };
    let Some(ctx) = rc.get("context").and_then(Value::as_object) else {
        return Ok(found);
    };
    if debug {
        log::cli_error("found context in .m365rc.json file");
    }
    for o in meta.options {
        if let Some(v) = ctx.get(o.name)
            && crate::json::truthy(v)
        {
            found.insert(o.name.to_string(), v.clone());
            if debug {
                log::cli_error(&format!("returning {} option from context", o.name));
            }
        }
    }
    Ok(found)
}

/// `SpoCommand.processOptions`: server-relative URLs become absolute.
fn process_options(meta: &CmdMeta, opts: &mut Obj) -> Result<(), Error> {
    if !meta.is(base::SPO) {
        return Ok(());
    }
    for (k, v) in opts.iter_mut() {
        if !meta.url_options.contains(&k.as_str()) {
            continue;
        }
        let Value::String(s) = v else { continue };
        if !s.starts_with('/') {
            continue;
        }
        auth::restore();
        let Some(spo) = auth::connection().spo_url else {
            return Err(Error::msg(
                "SharePoint URL is not available. Set SharePoint URL using the 'm365 spo set' command or use absolute URLs",
            ));
        };
        *v = Value::String(format!("{spo}{s}"));
    }
    Ok(())
}

fn prompt_value(meta: &CmdMeta, name: &str) -> Result<Value, Error> {
    let info = meta.option(name);
    let answer = match info.filter(|o| !o.autocomplete.is_empty()) {
        Some(o) => {
            let choices: Vec<String> = o.autocomplete.iter().map(|s| s.to_string()).collect();
            let i = crate::prompt::select(&format!("{name}: "), &choices)?;
            choices[i].clone()
        }
        None => crate::prompt::input(&format!("{name}: "))?,
    };
    let (flags, booleans) = command_flags(meta, &[]);
    let raw = vec![format!("--{name}"), answer];
    let raw = args::rewrite_args(&raw, &booleans).map_err(Error::msg)?;
    let p = args::parse(&raw, &flags);
    Ok(p.options.get(name).cloned().unwrap_or(Value::Null))
}

fn prompt_option_set(meta: &CmdMeta, opts: &mut Obj, set: &[&str]) -> Result<(), Error> {
    log::cli_error("🌶️  Please specify one of the following options:");
    let choices: Vec<String> = set.iter().map(|s| s.to_string()).collect();
    let i = crate::prompt::select("Option to use:", &choices)?;
    let name = &choices[i];
    let value = crate::prompt::input(&format!("{name}:"))?;
    let (flags, booleans) = command_flags(meta, &[]);
    let raw = args::rewrite_args(&[format!("--{name}"), value], &booleans).map_err(Error::msg)?;
    let p = args::parse(&raw, &flags);
    for o in set {
        if *o != name {
            opts.shift_remove(*o);
        }
    }
    opts.insert(
        name.clone(),
        p.options.get(name).cloned().unwrap_or(Value::Null),
    );
    log::cli_error("");
    Ok(())
}

fn input_required(issue: &schema::Issue, meta: &CmdMeta) -> Error {
    let mut input = Obj::new();
    let message = issue.render(true);
    if let Some(p) = &issue.path {
        input.insert("arg".into(), Value::from(p.as_str()));
        if let Some(o) = meta.option(p)
            && !o.autocomplete.is_empty()
        {
            input.insert(
                "choices".into(),
                Value::Array(
                    o.autocomplete
                        .iter()
                        .map(|c| serde_json::json!({ "value": c }))
                        .collect(),
                ),
            );
        }
    }
    if !issue.set_options.is_empty() {
        input.insert(
            "options".into(),
            Value::Array(issue.set_options.iter().map(|s| Value::from(*s)).collect()),
        );
    }
    let mut e = Obj::new();
    e.insert("input".into(), Value::Object(input));
    Error::Command {
        message: serde_json::json!({ "__input_required": message, "extra": Value::Object(e) }),
        code: Some(-5),
        status: None,
    }
}

fn agent_input_required(error: &Error) -> Option<i32> {
    if let Error::Command {
        message,
        code: Some(-5),
        ..
    } = error
    {
        let msg = message
            .get("__input_required")
            .cloned()
            .unwrap_or(Value::Null);
        let extra = message
            .get("extra")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        return Some(agent::print_error(Code::InputRequired, &msg, extra));
    }
    None
}

/// Validates zod-based options, prompting when allowed. Returns final options.
fn validate_schema(
    meta: &'static CmdMeta,
    sp: Option<&Spec>,
    opts: &mut Obj,
) -> Result<Obj, (Error, bool)> {
    let hooks = sp.map(|s| s.hooks).unwrap_or_default();
    loop {
        let issues = match schema::validate(meta, opts, &hooks) {
            Ok(v) => return Ok(v),
            Err(i) => i,
        };
        if agent::enabled() {
            if let Some(i) = issues.iter().find(|i| {
                i.missing || i.code == RefineCode::Required || i.code == RefineCode::OptionSet
            }) {
                return Err((input_required(i, meta), false));
            }
            let msg = render(&issues, true);
            return Err((invalid(msg), true));
        }
        if !config::prompt_enabled() {
            let msg = render(&issues, true);
            return Err((Error::msg(msg), true));
        }
        let missing: Vec<&schema::Issue> = issues
            .iter()
            .filter(|i| i.missing || i.code == RefineCode::Required)
            .collect();
        let sets: Vec<&schema::Issue> = issues
            .iter()
            .filter(|i| i.code == RefineCode::OptionSet)
            .collect();
        let others = issues.len() - missing.len() - sets.len();
        if others > 0 {
            let msg = render(&issues, false);
            return Err((Error::msg(msg), true));
        }
        if !missing.is_empty() {
            log::cli_error("🌶️  Provide values for the following parameters:");
            for i in &missing {
                let name = i.path.clone().unwrap_or_default();
                let v = prompt_value(meta, &name).map_err(|e| (e, true))?;
                opts.insert(name, v);
            }
            process_options(meta, opts).map_err(|e| (e, false))?;
            continue;
        }
        for i in &sets {
            prompt_option_set(meta, opts, i.set_options).map_err(|e| (e, true))?;
        }
        process_options(meta, opts).map_err(|e| (e, false))?;
    }
}

/// `Command.validate` for legacy (non-zod) commands.
fn validate_legacy(meta: &'static CmdMeta, sp: Option<&Spec>, opts: &mut Obj) -> Result<(), Error> {
    let outputs = meta.allowed_outputs();
    if let Some(o) = opts
        .get("output")
        .and_then(Value::as_str)
        .filter(|o| !o.is_empty())
        && !outputs.contains(&o)
    {
        return Err(Error::msg(format!(
            "'{o}' is not a valid output type. Allowed output types are {}",
            outputs.join(", ")
        )));
    }
    if !meta.allow_unknown {
        for k in opts.keys() {
            if !meta
                .options
                .iter()
                .any(|o| o.long == Some(k.as_str()) || o.short == Some(k.as_str()))
            {
                return Err(Error::msg(format!("Invalid option: '{k}'{EOL}")));
            }
        }
    }
    let prompt = config::prompt_enabled();
    let mut prompted = false;
    for o in meta.options {
        if !o.required || opts.contains_key(o.name) {
            continue;
        }
        if agent::enabled() {
            let issue = schema::Issue {
                path: Some(o.name.to_string()),
                message: format!("Required option {} not specified", o.name),
                missing: true,
                code: RefineCode::Required,
                set_options: &[],
            };
            return Err(input_required(&issue, meta));
        }
        if !prompt {
            return Err(Error::msg(format!(
                "Required option {} not specified",
                o.name
            )));
        }
        if !prompted {
            prompted = true;
            log::cli_error("🌶️  Provide values for the following parameters:");
        }
        let v = prompt_value(meta, o.name)?;
        opts.insert(o.name.to_string(), v);
    }
    if prompted {
        log::cli_error("");
    }
    process_options(meta, opts)?;
    if let Some(l) = &meta.legacy {
        // `optionSets.sort(opt => opt.runsWhen ? 0 : 1)` with a one-argument
        // comparator keeps the order
        for set in l.option_sets {
            if let Some(rw) = set.runs_when
                && !runs_when(rw, opts)
            {
                continue;
            }
            let common: Vec<String> = opts
                .keys()
                .filter(|k| set.options.contains(&k.as_str()))
                .cloned()
                .collect();
            if common.is_empty() {
                if agent::enabled() || !prompt {
                    let msg = format!(
                        "Specify one of the following options: {}.",
                        set.options.join(", ")
                    );
                    if agent::enabled() {
                        let issue = schema::Issue {
                            path: None,
                            message: msg,
                            missing: false,
                            code: RefineCode::OptionSet,
                            set_options: set.options,
                        };
                        return Err(input_required(&issue, meta));
                    }
                    return Err(Error::msg(msg));
                }
                prompt_option_set(meta, opts, set.options)?;
            }
            if common.len() > 1 {
                if agent::enabled() || !prompt {
                    return Err(Error::msg(format!(
                        "Specify one of the following options: {}, but not multiple.",
                        set.options.join(", ")
                    )));
                }
                log::cli_error(
                    "🌶️  Multiple options for an option set specified. Please specify the correct option that you wish to use.",
                );
                let choices: Vec<String> = common.clone();
                let i = crate::prompt::select("Option to use:", &choices)?;
                for c in &choices {
                    if *c != choices[i] {
                        opts.shift_remove(c);
                    }
                }
                log::cli_error("");
            }
        }
    }
    if let Some(v) = sp.and_then(|s| s.validate) {
        v(opts).map_err(Error::msg)?;
    }
    Ok(())
}

/// Evaluates the `runsWhen` predicates found in legacy option sets.
fn runs_when(src: &str, opts: &Obj) -> bool {
    // shapes: `(args) => args.options.x`, `args => !args.options.x`,
    // `(args) => args.options.x !== undefined`, `... && ...`
    let body = src.split("=>").nth(1).unwrap_or("").trim();
    body.split("&&").all(|part| {
        let p = part.trim().trim_matches(|c| c == '(' || c == ')');
        let (neg, p) = match p.strip_prefix('!') {
            Some(r) => (true, r),
            None => (false, p),
        };
        let (name, cmp) = match p.split_once(" !== undefined") {
            Some((n, _)) => (n, true),
            None => match p.split_once(" === undefined") {
                Some((n, _)) => (n, false),
                None => (p, true),
            },
        };
        let name = name.trim().rsplit('.').next().unwrap_or("");
        let present = if p.contains("undefined") {
            opts.contains_key(name)
        } else {
            opts.get(name).is_some_and(crate::json::truthy)
        };
        let r = if cmp { present } else { !present };
        r != neg
    })
}

/// `Command.loadValuesFromAccessToken`: `@meId` and `@meUserName`.
fn load_values_from_access_token(opts: &mut Obj) -> Result<(), Error> {
    let Some(token) = auth::default_access_token() else {
        return Ok(());
    };
    for (_, v) in opts.iter_mut() {
        let Value::String(s) = v else { continue };
        if s.is_empty() {
            continue;
        }
        let l = s.to_lowercase();
        let l = l.trim();
        if (l == "@meid" || l == "@meusername") && auth::jwt::is_app_only(&token) == Some(true) {
            return Err(Error::msg(format!(
                "It's not possible to use {s} with application permissions"
            )));
        }
        if l == "@meid" {
            *v = Value::from(auth::jwt::user_id(&token));
        } else if l == "@meusername" {
            *v = Value::from(auth::jwt::user_name(&token));
        }
    }
    Ok(())
}

/// Runs the base-class checks of upstream's `Command.action` and the
/// handler.
/// `AppCommand.action`: picks the app from `.m365rc.json` before anything else.
fn m365rc_app_id(opts: &Obj) -> Result<String, Error> {
    let p = ".m365rc.json";
    if !std::path::Path::new(p).exists() {
        return Err(Error::msg(format!("Could not find file: {p}")));
    }
    let text =
        std::fs::read_to_string(p).map_err(|_| Error::msg(format!("Could not parse file: {p}")))?;
    if text.is_empty() {
        return Err(Error::msg(format!("File {p} is empty")));
    }
    let rc: Value = serde_json::from_str(&text)
        .map_err(|_| Error::msg(format!("Could not parse file: {p}")))?;
    let apps = rc
        .get("apps")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    if apps.is_empty() {
        return Err(Error::msg(format!("No Entra apps found in {p}")));
    }
    if let Some(id) = opts.get("appId").map(to_js_string) {
        if !apps
            .iter()
            .any(|a| a.get("appId").map(to_js_string).as_deref() == Some(id.as_str()))
        {
            return Err(Error::msg(format!("App {id} not found in {p}")));
        }
        return Ok(id);
    }
    if apps.len() == 1 {
        return Ok(apps[0]
            .get("appId")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into()));
    }
    let indexed: Vec<Value> = apps
        .into_iter()
        .enumerate()
        .map(|(i, mut a)| {
            if let Some(o) = a.as_object_mut() {
                o.insert("appIdIndex".into(), Value::from(i));
            }
            a
        })
        .collect();
    let r = handle_multiple_results(
        &format!("Multiple Entra apps found in {p}."),
        &crate::util::array_to_hash_table("appId", &indexed),
    )?;
    Ok(r.get("appId")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into()))
}

pub async fn action(
    meta: &'static CmdMeta,
    sp: &Spec,
    opts: Obj,
    used_name: &str,
) -> Result<(), Error> {
    let app_id = if meta.is(base::APP) {
        Some(m365rc_app_id(&opts)?)
    } else {
        None
    };
    let anonymous = meta.is(base::ANONYMOUS);
    // `AnonymousCommand.action` runs initAction without restoring the login
    // (commands that override `action`, such as `cli consent`, restore it)
    if !anonymous || meta.custom_action {
        auth::restore();
        if !meta.custom_action
            && meta.is(base::SPO)
            && auth::is_active()
            && auth::connection().auth_type == auth::AuthType::Secret
        {
            return Err(Error::msg(
                "SharePoint does not support authentication using client ID and secret. Please use a different login type to use SharePoint commands.",
            ));
        }
    }
    // initAction
    let debug = log::debug();
    let c = auth::connection();
    if debug && c.identity_name.is_some() {
        log::log_stderr(&format!(
            "Executing command as '{}', appId: {}, tenantId: {}",
            c.identity_name.clone().unwrap_or_default(),
            c.app_id.clone().unwrap_or_else(|| "undefined".into()),
            c.identity_tenant_id
                .clone()
                .unwrap_or_else(|| "undefined".into())
        ));
    }
    if !anonymous && !meta.custom_action && auth::is_active() {
        let public = c.cloud_type == auth::CloudType::Public;
        let power = [
            (base::POWER_PLATFORM, "Power Platform"),
            (base::POWER_APPS, "Power Apps"),
            (base::POWER_AUTOMATE, "Power Automate"),
        ]
        .into_iter()
        .find(|(b, _)| meta.is(*b));
        if let Some((_, product)) = power {
            if !public {
                return Err(Error::msg(format!(
                    "{product} commands only support the public cloud at the moment. We'll add support for other clouds in the future. Sorry for the inconvenience."
                )));
            }
            auth::assert_access_token_type(true)?;
        } else if meta.is(base::POWER_BI)
            || meta.is(base::VIVA_ENGAGE)
            || meta.is(base::GRAPH_DELEGATED)
        {
            auth::assert_access_token_type(true)?;
        } else if meta.is(base::GRAPH_APPLICATION) {
            auth::assert_access_token_type(false)?;
        }
    }
    let mut opts = opts;
    if !anonymous && !meta.custom_action {
        if !auth::is_active() {
            return Err(Error::msg("Log in to Microsoft 365 first"));
        }
        load_values_from_access_token(&mut opts)?;
    }
    let mut cx = Cx {
        meta,
        opts,
        used_name: used_name.to_string(),
        app_id,
    };
    (sp.run)(&mut cx).await
}

/// The one item of `items`: `not_found` when there is none, and the user's
/// pick (`cli.handleMultipleResultsFound`, keyed by `key`) when there are more.
pub fn pick_one(
    items: &[Value],
    key: &str,
    not_found: Error,
    multiple: &str,
) -> Result<Value, Error> {
    match items {
        [] => Err(not_found),
        [one] => Ok(one.clone()),
        _ => handle_multiple_results(multiple, &crate::util::array_to_hash_table(key, items)),
    }
}

/// `cli.handleMultipleResultsFound`.
pub fn handle_multiple_results(message: &str, values: &Obj) -> Result<Value, Error> {
    let keys: Vec<String> = values.keys().cloned().collect();
    if agent::enabled() || !config::prompt_enabled() {
        return Err(Error::js(format!("{message} Found: {}.", keys.join(", "))));
    }
    log::cli_error(&format!("🌶️  {message} "));
    let i = crate::prompt::select("Please choose one:", &keys)?;
    log::cli_error("");
    Ok(values[&keys[i]].clone())
}

/// `cli.executeCommand` for one command calling another: output goes
/// through the nested command's logger with the given options.
pub async fn run_nested(
    name: &str,
    mut opts: Obj,
    capture: bool,
) -> Result<(String, String), Error> {
    let meta =
        manifest::find(name).ok_or_else(|| Error::msg(format!("Command {name} not found")))?;
    let sp =
        spec(meta.name).ok_or_else(|| Error::msg(format!("Command {name} is not implemented")))?;
    let prev = log::get_state();
    if !opts.contains_key("output") {
        opts.insert("output".into(), Value::from("json"));
    }
    let state = RunState {
        debug: opts.get("debug").is_some_and(crate::json::truthy),
        verbose: opts.get("verbose").is_some_and(crate::json::truthy)
            || opts.get("debug").is_some_and(crate::json::truthy),
        output: opts
            .get("output")
            .map(to_js_string)
            .unwrap_or_else(|| "json".into()),
        query: opts.get("query").map(to_js_string),
        options: opts.clone(),
        meta: Some(meta),
        command_name: meta.name.to_string(),
        default_properties: None,
    };
    if state.debug && (capture || state.output != "none") {
        let mut o = Obj::new();
        o.insert("options".into(), Value::Object(opts.clone()));
        let msg = format!(
            "Executing command {} with options {}",
            meta.name,
            stringify(&Value::Object(o))
        );
        if capture {
            log::begin_capture();
        }
        log::log_stderr(&msg);
    } else if capture {
        log::begin_capture();
    }
    log::set_state(state);
    let r = action(meta, sp, opts, meta.name).await;
    let out = if capture {
        log::end_capture()
    } else {
        (String::new(), String::new())
    };
    log::set_state(prev);
    r.map(|_| out)
}

async fn execute(raw_args: Vec<String>) -> Result<i32, Exit> {
    let start = Instant::now();
    let mut t = Timings::default();
    let mut raw = raw_args;
    let mut show_help = false;
    if raw.first().is_some_and(|a| a == "help") {
        show_help = true;
        raw.remove(0);
    }
    let first = args::parse(&raw, &Flags::default());
    let name = positional_name(&first.positional);
    let meta = if name.is_empty() {
        None
    } else {
        manifest::find(&name)
    };
    let used_name = name.clone();
    agent::set_command(meta.map(|m| m.name).unwrap_or(&name));

    let mut opts = match meta {
        Some(m) => {
            let (flags, booleans) = command_flags(m, &raw);
            let rewritten = match args::rewrite_args(&raw, &booleans) {
                Ok(r) => r,
                Err(e) => {
                    return Err(close_with_error(
                        &invalid(e),
                        first.options.get("output").and_then(Value::as_str),
                        Some(m),
                        false,
                    ));
                }
            };
            args::parse(&rewritten, &flags).options
        }
        None => first.options.clone(),
    };

    let wants_help = show_help
        || first.options.get("h").is_some_and(crate::json::truthy)
        || first.options.get("help").is_some_and(crate::json::truthy);
    if meta.is_none() || wants_help {
        if meta.is_none() && agent::enabled() && !name.is_empty() && !wants_help {
            let msg = format!(
                "Command '{name}' was not found. Run `m365 --help --agent` or `m365 {} --help --agent` to list commands.",
                name.split(' ').next().unwrap_or("")
            );
            return Ok(agent::print_error(
                Code::Unknown,
                &Value::from(msg),
                Obj::new(),
            ));
        }
        if first.options.get("output").and_then(Value::as_str) != Some("none") {
            let mode = help_mode(&first.options)?;
            print_help(meta, &mode, &name);
        }
        return Ok(0);
    }
    let meta = meta.unwrap();
    let sp = spec(meta.name);

    load_from_files(&mut opts);
    let debug_arg = opts.get("debug").is_some_and(crate::json::truthy);
    let opt_start = Instant::now();
    match load_from_context(meta, debug_arg) {
        Ok(ctx) => {
            let mut merged = ctx;
            for (k, v) in std::mem::take(&mut opts) {
                merged.insert(k, v);
            }
            opts = merged;
        }
        Err(e) => return Err(close_with_error(&e, None, Some(meta), false)),
    }
    if let Err(e) = process_options(meta, &mut opts) {
        return Err(close_with_error(
            &e,
            opts.get("output").and_then(Value::as_str),
            Some(meta),
            false,
        ));
    }
    t.options += opt_start.elapsed().as_secs_f64() * 1e3;
    if agent::enabled() {
        opts.insert("output".into(), Value::from("json"));
    } else if !opts.contains_key("output") {
        opts.insert(
            "output".into(),
            config::get(names::OUTPUT).unwrap_or(Value::from("json")),
        );
    }

    let val_start = Instant::now();
    let final_args = if meta.legacy.is_none() {
        match validate_schema(meta, sp, &mut opts) {
            Ok(v) => v,
            Err((e, help)) => {
                if agent::enabled()
                    && let Some(code) = agent_input_required(&e)
                {
                    return Ok(code);
                }
                return Err(close_with_error(
                    &e,
                    opts.get("output").and_then(Value::as_str),
                    Some(meta),
                    help,
                ));
            }
        }
    } else {
        match validate_legacy(meta, sp, &mut opts) {
            Ok(()) => opts.clone(),
            Err(e) => {
                if agent::enabled()
                    && let Some(code) = agent_input_required(&e)
                {
                    return Ok(code);
                }
                return Err(close_with_error(
                    &e,
                    opts.get("output").and_then(Value::as_str),
                    Some(meta),
                    true,
                ));
            }
        }
    };
    t.validation += val_start.elapsed().as_secs_f64() * 1e3;
    t.core += start.elapsed().as_secs_f64() * 1e3;

    let output = final_args
        .get("output")
        .map(to_js_string)
        .unwrap_or_else(|| "json".into());
    let debug = final_args.get("debug").is_some_and(crate::json::truthy)
        || std::env::var("CLIMICROSOFT365_DEBUG").is_ok_and(|v| v == "1");
    let verbose = debug
        || final_args.get("verbose").is_some_and(crate::json::truthy)
        || std::env::var("CLIMICROSOFT365_VERBOSE").is_ok_and(|v| v == "1");
    log::set_state(RunState {
        debug,
        verbose,
        output: output.clone(),
        query: final_args.get("query").map(to_js_string),
        options: final_args.clone(),
        meta: Some(meta),
        command_name: if meta.aliases.contains(&used_name.as_str()) {
            used_name.clone()
        } else {
            meta.name.to_string()
        },
        default_properties: None,
    });
    if final_args.get("debug").is_some_and(crate::json::truthy) && output != "none" {
        let mut o = Obj::new();
        o.insert("options".into(), Value::Object(final_args.clone()));
        log::log_stderr(&format!(
            "Executing command {} with options {}",
            meta.name,
            stringify(&Value::Object(o))
        ));
    }

    let Some(sp) = sp else {
        let e = Error::msg(format!(
            "Command '{}' is not implemented in this port",
            meta.name
        ));
        return Err(close_with_error(&e, Some(&output), Some(meta), false));
    };
    let cmd_start = Instant::now();
    let result = action(meta, sp, final_args, &used_name).await;
    t.command += cmd_start.elapsed().as_secs_f64() * 1e3;
    t.total = start.elapsed().as_secs_f64() * 1e3;
    match result {
        Ok(()) => {
            if verbose {
                log::log_stderr(&term::green("DONE"));
            }
            print_timings(&raw, &t);
            if agent::enabled() {
                agent::print_ok();
            }
            Ok(0)
        }
        Err(e) => {
            print_timings(&raw, &t);
            Err(close_with_error(&e, Some(&output), Some(meta), false))
        }
    }
}

fn print_timings(raw: &[String], t: &Timings) {
    if !raw.iter().any(|a| a == "--debug") {
        return;
    }
    log::cli_error("");
    log::cli_error("Timings:");
    for (k, v) in [
        ("api", t.api),
        ("core", t.core),
        ("command", t.command),
        ("options", t.options),
        ("total", t.total),
        ("validation", t.validation),
    ] {
        log::cli_error(&format!("{k}: {}ms", crate::json::number_to_string(v)));
    }
}

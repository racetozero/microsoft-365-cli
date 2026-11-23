//! `--help`: command, group and root help in the style of `atl` (clap's long
//! help): a summary, a usage line, options with their values, the option
//! rules, examples, notes and permissions. With `--agent` the same content is
//! returned as JSON. The text of each command lives in `help/data/`.

mod data;

use crate::manifest::{self, CmdMeta, FType, Lit, Msg, OptMeta, OptType, Rule};
pub use crate::md::md2plain;
use serde_json::{Value, json};

/// The values of `-h <mode>` and the `helpMode` setting (upstream's names).
pub const HELP_MODES: &[&str] = &[
    "options",
    "examples",
    "remarks",
    "permissions",
    "response",
    "full",
];

/// The width text is wrapped to.
const WIDTH: usize = 100;

/// Options every command accepts; they are listed once under Global options.
const GLOBAL: &[&str] = &["query", "output", "debug", "verbose"];

/// One line per suite for root help and `m365 <suite> --help`.
const SUITES: &[(&str, &str)] = &[
    ("adaptivecard", "Adaptive Cards: send a card to a webhook"),
    (
        "app",
        "Microsoft Entra apps in .m365rc.json: get, open, permissions",
    ),
    ("booking", "Microsoft Bookings: businesses"),
    (
        "cli",
        "The CLI itself: settings, consent, completion, doctor, issue",
    ),
    ("connection", "Saved logins: list, use, set, remove"),
    (
        "context",
        "Default option values for the current folder (.m365rc.json)",
    ),
    (
        "entra",
        "Microsoft Entra ID: users, groups, apps, licenses, roles, policies",
    ),
    ("exo", "Exchange Online: app role assignments"),
    ("external", "Microsoft Search connectors and their items"),
    (
        "file",
        "Files in document libraries: list, add, copy, move, convert",
    ),
    (
        "flow",
        "Power Automate flows, runs, owners and environments",
    ),
    (
        "graph",
        "Microsoft Graph extensions, subscriptions and the changelog",
    ),
    ("onedrive", "OneDrive sites and usage reports"),
    ("onenote", "OneNote notebooks and pages"),
    (
        "outlook",
        "Outlook mail, calendars, events, rooms and reports",
    ),
    ("pa", "Power Apps: apps, owners, permissions, connectors"),
    ("planner", "Planner plans, buckets, tasks and rosters"),
    (
        "pp",
        "Power Platform: environments, solutions, Dataverse, Copilot, gateways",
    ),
    (
        "purview",
        "Purview: retention, sensitivity labels, audit log, threat assessments",
    ),
    (
        "search",
        "Microsoft Search external connections (alias of external)",
    ),
    ("spe", "SharePoint Embedded container types and containers"),
    (
        "spfx",
        "SharePoint Framework: doctor, project upgrade, pipelines, packages",
    ),
    (
        "spo",
        "SharePoint Online: sites, lists, items, files, pages, apps, tenant",
    ),
    ("spp", "SharePoint Premium: content centers and models"),
    (
        "teams",
        "Microsoft Teams: teams, channels, messages, chats, meetings, apps",
    ),
    (
        "tenant",
        "The tenant: info, service health, people settings, reports",
    ),
    ("todo", "Microsoft To Do lists and tasks"),
    ("util", "Utilities: access tokens"),
    ("viva", "Viva Connections and Viva Engage"),
];

/// The port's own commands, which are not in the command tables.
const OWN_COMMANDS: &[(&str, &str)] = &[
    (
        "skill",
        "Print an agent skill, list them, or install them all",
    ),
    (
        "update",
        "Install the latest m365 release (or check for one)",
    ),
];

fn help_of(meta: &CmdMeta) -> Option<&'static data::Help> {
    data::find(meta.name)
}

/// `--webUrl` -> `<WEB_URL>`.
fn placeholder(name: &str) -> String {
    let mut out = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_ascii_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_ascii_uppercase());
    }
    format!("<{out}>")
}

fn takes_value(o: &OptMeta) -> bool {
    o.ty != OptType::Boolean
}

fn flags(o: &OptMeta) -> String {
    let long = o.long.unwrap_or(o.name);
    let mut s = match o.short {
        Some(sh) => format!("-{sh}, --{long}"),
        None => format!("    --{long}"),
    };
    if takes_value(o) {
        s.push(' ');
        s.push_str(&placeholder(long));
    }
    s
}

/// Allowed values of an option: the field's enum, else upstream's completions.
fn values(meta: &CmdMeta, o: &OptMeta) -> Vec<&'static str> {
    match meta.field(o.name) {
        Some(f) if !f.values.is_empty() => f.values.to_vec(),
        _ if o.ty == OptType::Boolean => Vec::new(),
        _ => o.autocomplete.to_vec(),
    }
}

fn default_of(meta: &CmdMeta, o: &OptMeta) -> Option<String> {
    match meta.field(o.name).and_then(|f| f.default) {
        Some(Lit::Bool(b)) => Some(b.to_string()),
        Some(Lit::Str(s)) => Some(s.to_string()),
        Some(Lit::Num(n)) => Some(crate::json::number_to_string(n)),
        None => None,
    }
}

fn description(meta: &CmdMeta, o: &OptMeta) -> &'static str {
    help_of(meta)
        .and_then(|h| {
            h.options
                .iter()
                .find(|(n, _)| n.eq_ignore_ascii_case(o.long.unwrap_or(o.name)))
        })
        .map(|(_, d)| *d)
        .unwrap_or("")
}

/// Wraps `text` at `WIDTH`, every line starting with `indent`; lines that
/// already start with spaces (code) are kept as they are.
fn wrap(text: &str, indent: usize) -> String {
    let pad = " ".repeat(indent);
    let mut out = Vec::new();
    for para in text.split('\n') {
        if para.trim().is_empty() {
            out.push(String::new());
            continue;
        }
        if para.starts_with("    ") {
            out.push(format!("{pad}{para}"));
            continue;
        }
        let (bullet, body) = match para.strip_prefix("- ") {
            Some(b) => ("- ", b),
            None => match para.strip_prefix("  - ") {
                Some(b) => ("  - ", b),
                None => ("", para),
            },
        };
        let hang = " ".repeat(bullet.len());
        let mut line = format!("{pad}{bullet}");
        let mut empty = true;
        for word in body.split_whitespace() {
            if !empty && line.len() + 1 + word.len() > WIDTH {
                out.push(std::mem::take(&mut line));
                line = format!("{pad}{hang}");
                empty = true;
            }
            if !empty {
                line.push(' ');
            }
            line.push_str(word);
            empty = false;
        }
        out.push(line);
    }
    out.join("\n")
}

fn opt_list(names: &[&str]) -> String {
    names
        .iter()
        .map(|n| format!("--{n}"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The option rules as sentences, for example `Use exactly one of: --id, --userName.`
fn rules(meta: &CmdMeta) -> Vec<String> {
    let mut out = Vec::new();
    for r in meta.refines {
        let line = match r.rule {
            Some(Rule::ExactlyOne(o)) | Some(Rule::ExactlyOneTruthy(o)) => {
                format!("Use exactly one of: {}.", opt_list(o))
            }
            Some(Rule::AtMostOne(o)) | Some(Rule::AtMostOneTruthy(o)) => {
                format!("Use at most one of: {}.", opt_list(o))
            }
            Some(Rule::AtLeastOne(o)) | Some(Rule::AtLeastOneTruthy(o)) => {
                format!("Use at least one of: {}.", opt_list(o))
            }
            Some(Rule::NotAllTruthy(o)) => format!("Do not use all of: {}.", opt_list(o)),
            Some(Rule::Implies(o)) if o.len() >= 2 => format!("--{} needs --{}.", o[0], o[1]),
            _ => match r.msg {
                Msg::Text(t) if !t.contains("{input}") => t.to_string(),
                _ => continue,
            },
        };
        if !out.contains(&line) {
            out.push(line);
        }
    }
    if let Some(l) = &meta.legacy {
        for s in l.option_sets {
            let line = match s.runs_when {
                Some(when) => format!("When {when}, use exactly one of: {}.", opt_list(s.options)),
                None => format!("Use exactly one of: {}.", opt_list(s.options)),
            };
            if !out.contains(&line) {
                out.push(line);
            }
        }
    }
    out
}

fn own_options(meta: &CmdMeta) -> impl Iterator<Item = &'static OptMeta> {
    meta.options.iter().filter(|o| !GLOBAL.contains(&o.name))
}

/// `Usage: m365 spo web get --url <URL> [options]`.
fn usage(meta: &CmdMeta) -> String {
    let mut s = format!("Usage: m365 {}", meta.name);
    for o in own_options(meta).filter(|o| o.required) {
        let long = o.long.unwrap_or(o.name);
        s.push_str(&format!(" --{long}"));
        if takes_value(o) {
            s.push(' ');
            s.push_str(&placeholder(long));
        }
    }
    s.push_str(" [options]");
    s
}

fn wants(mode: &str, section: &str) -> bool {
    match mode {
        "full" | "response" => true,
        "options" => section == "options",
        m => section == "options" || section == m,
    }
}

/// The help of a command as text, for `-h <mode>`.
pub fn command_help(meta: &CmdMeta, mode: &str) -> Option<String> {
    let h = help_of(meta);
    let mut out = Vec::new();
    out.push(wrap(
        h.map(|h| h.summary)
            .filter(|s| !s.is_empty())
            .unwrap_or(meta.description),
        0,
    ));
    out.push(String::new());
    out.push(usage(meta));
    if !meta.aliases.is_empty() {
        out.push(format!(
            "Alias: {}",
            meta.aliases
                .iter()
                .map(|a| format!("m365 {a}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let own: Vec<&OptMeta> = own_options(meta).collect();
    if !own.is_empty() {
        out.push(String::new());
        out.push("Options:".into());
        for o in &own {
            out.push(format!("  {}", flags(o)));
            let mut text = description(meta, o).to_string();
            let mut extra = Vec::new();
            if o.required {
                extra.push("required".to_string());
            }
            let v = values(meta, o);
            if !v.is_empty() {
                extra.push(format!("possible values: {}", v.join(", ")));
            }
            if let Some(d) = default_of(meta, o) {
                extra.push(format!("default: {d}"));
            }
            if !extra.is_empty() {
                if !text.is_empty() {
                    text.push_str("\n\n");
                }
                text.push_str(&format!("[{}]", extra.join("; ")));
            }
            if !text.is_empty() {
                out.push(wrap(&text, 10));
            }
            out.push(String::new());
        }
        out.pop();
    }
    if meta.allow_unknown || meta.loose {
        out.push(String::new());
        out.push(wrap(
            "Other options are passed through, for example list field values as --FieldName value.",
            0,
        ));
    }
    let r = rules(meta);
    if !r.is_empty() {
        out.push(String::new());
        out.push("Rules:".into());
        for line in r {
            out.push(wrap(&format!("- {line}"), 2));
        }
    }
    out.push(String::new());
    out.push("Global options:".into());
    let all = meta.allowed_outputs();
    let mut outputs: Vec<String> = all
        .iter()
        .filter(|o| **o == "json")
        .map(|_| "json (default)".to_string())
        .collect();
    outputs.extend(all.iter().filter(|o| **o != "json").map(|o| o.to_string()));
    let outputs = outputs.join(", ");
    for (flag, text) in [
        ("-o, --output <FORMAT>", format!("Output format: {outputs}")),
        (
            "    --query <JMESPATH>",
            "Keep only part of the output (a JMESPath query)".to_string(),
        ),
        (
            "    --agent",
            "Print one JSON line and never prompt (or set M365_AGENT=1)".to_string(),
        ),
        (
            "    --debug",
            "Log HTTP requests and responses on stderr".to_string(),
        ),
        ("    --verbose", "Log progress on stderr".to_string()),
        (
            "-h, --help [<MODE>]",
            "Help sections: full (default), options, examples, remarks, permissions".to_string(),
        ),
    ] {
        out.push(format!("  {flag:<24} {text}"));
    }
    if let Some(h) = h {
        if wants(mode, "examples") && !h.examples.is_empty() {
            out.push(String::new());
            out.push("Examples:".into());
            for (caption, cmd) in h.examples {
                if !caption.is_empty() {
                    out.push(wrap(caption, 2));
                }
                for line in cmd.lines() {
                    out.push(format!("    {line}"));
                }
                out.push(String::new());
            }
            out.pop();
        }
        if wants(mode, "remarks") && !h.notes.is_empty() {
            out.push(String::new());
            out.push("Notes:".into());
            for n in h.notes {
                out.push(wrap(n, 2));
            }
        }
        if wants(mode, "permissions") && !h.permissions.is_empty() {
            out.push(String::new());
            out.push("Permissions:".into());
            let w = h
                .permissions
                .iter()
                .map(|(k, _, _)| k.len() + 1)
                .max()
                .unwrap_or(0);
            for (kind, resource, perms) in h.permissions {
                let label = if kind.is_empty() {
                    String::new()
                } else {
                    format!("{:<w$} ", format!("{kind}:"))
                };
                let text = wrap(&format!("{resource}: {perms}"), 2 + label.len());
                out.push(format!("  {label}{}", text.trim_start()));
            }
        }
        if (mode == "full" || mode == "response") && !h.links.is_empty() {
            out.push(String::new());
            out.push("Learn more:".into());
            for (text, url) in h.links {
                out.push(if text == url {
                    format!("  {url}")
                } else {
                    wrap(&format!("{text}: {url}"), 2)
                });
            }
        }
    }
    Some(out.join("\n"))
}

fn option_json(meta: &CmdMeta, o: &OptMeta) -> Value {
    let ty = match o.ty {
        OptType::Boolean => "boolean",
        OptType::Number => "number",
        _ => match meta.field(o.name).map(|f| f.t) {
            Some(FType::Array) => "array",
            Some(FType::Number) => "number",
            _ => "string",
        },
    };
    let mut v = json!({
        "name": o.long.unwrap_or(o.name),
        "type": ty,
        "required": o.required,
        "description": description(meta, o),
    });
    if let Some(s) = o.short {
        v["short"] = Value::from(s);
    }
    let vals = values(meta, o);
    if !vals.is_empty() {
        v["values"] = Value::from(vals);
    }
    if let Some(d) = default_of(meta, o) {
        v["default"] = Value::from(d);
    }
    v
}

/// The help of a command for `--help --agent`.
pub fn command_help_json(meta: &CmdMeta) -> Value {
    let h = help_of(meta);
    let mut v = json!({
        "name": meta.name,
        "summary": h.map(|h| h.summary).filter(|s| !s.is_empty()).unwrap_or(meta.description),
        "usage": usage(meta).trim_start_matches("Usage: ").to_string(),
        "options": own_options(meta).map(|o| option_json(meta, o)).collect::<Vec<_>>(),
        "rules": rules(meta),
        "accepts_other_options": meta.allow_unknown || meta.loose,
        "outputs": meta.allowed_outputs(),
    });
    if !meta.aliases.is_empty() {
        v["aliases"] = Value::from(meta.aliases.to_vec());
    }
    if let Some(h) = h {
        v["examples"] = h
            .examples
            .iter()
            .map(|(d, c)| json!({ "description": d, "command": c }))
            .collect();
        v["notes"] = Value::from(h.notes.to_vec());
        v["permissions"] = h
            .permissions
            .iter()
            .map(|(k, r, p)| json!({ "type": k, "resource": r, "permissions": p }))
            .collect();
        v["links"] = h
            .links
            .iter()
            .map(|(t, u)| json!({ "text": t, "url": u }))
            .collect();
    }
    v
}

fn summary(meta: &CmdMeta) -> &'static str {
    help_of(meta)
        .map(|h| h.summary)
        .filter(|s| !s.is_empty())
        .unwrap_or(meta.description)
}

/// (next word, command when it is one, words below it) under `prefix`.
fn children(prefix: &str) -> Vec<(String, Option<&'static CmdMeta>, Vec<String>)> {
    let depth = if prefix.is_empty() {
        0
    } else {
        prefix.split(' ').count()
    };
    let mut out: Vec<(String, Option<&'static CmdMeta>, Vec<String>)> = Vec::new();
    for c in manifest::COMMANDS {
        let words: Vec<&str> = c.name.split(' ').collect();
        if words.len() <= depth || (depth > 0 && words[..depth].join(" ") != prefix) {
            continue;
        }
        let next = words[depth].to_string();
        let i = match out.iter().position(|(n, _, _)| *n == next) {
            Some(i) => i,
            None => {
                out.push((next, None, Vec::new()));
                out.len() - 1
            }
        };
        if words.len() == depth + 1 {
            out[i].1 = Some(c);
        } else if !out[i].2.contains(&words[depth + 1].to_string()) {
            out[i].2.push(words[depth + 1].to_string());
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn suite_description(name: &str) -> Option<&'static str> {
    SUITES.iter().find(|(n, _)| *n == name).map(|(_, d)| *d)
}

/// Lines `  name   description`, aligned.
fn table(rows: &[(String, String)]) -> Vec<String> {
    let w = rows.iter().map(|(n, _)| n.len()).max().unwrap_or(0) + 2;
    rows.iter()
        .map(|(n, d)| {
            let text = wrap(d, 2 + w);
            format!("  {n}{}{}", " ".repeat(w - n.len()), text.trim_start())
        })
        .collect()
}

/// Help for `m365 --help` (empty prefix) or a group such as `m365 entra user`.
pub fn group_help(prefix: &str) -> String {
    if prefix.is_empty() {
        return root_help();
    }
    let kids = children(prefix);
    let first = prefix.split(' ').next().unwrap_or(prefix);
    let mut out = Vec::new();
    let names: Vec<&str> = kids.iter().map(|(n, _, _)| n.as_str()).collect();
    let head = if prefix.contains(' ') {
        format!("m365 {prefix}")
    } else {
        suite_description(first)
            .map(|d| d.split(':').next().unwrap_or(d).to_string())
            .unwrap_or_else(|| format!("m365 {prefix}"))
    };
    out.push(wrap(&format!("{head}: {}", names.join(", ")), 0));
    out.push(String::new());
    out.push(format!("Usage: m365 {prefix} <command> [options]"));
    out.push(String::new());
    out.push("Commands:".into());
    // below a suite, every command with its summary; at a suite, its resources
    let below = format!("{prefix} ");
    let rows: Vec<(String, String)> = if prefix.contains(' ') {
        manifest::COMMANDS
            .iter()
            .filter(|c| c.name.starts_with(&below))
            .map(|c| {
                (
                    c.name[prefix.len() + 1..].to_string(),
                    summary(c).to_string(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>()
            .into_iter()
            .collect()
    } else {
        kids.iter()
            .map(|(n, c, sub)| {
                let d = match (c, sub.is_empty()) {
                    (Some(c), true) => summary(c).to_string(),
                    (Some(c), false) => format!("{} (also: {})", summary(c), sub.join(", ")),
                    (None, _) => sub.join(", "),
                };
                (n.clone(), d)
            })
            .collect()
    };
    out.extend(table(&rows));
    out.push(String::new());
    out.push(format!(
        "Run `m365 {prefix} <command> --help` for a command's options and examples."
    ));
    out.join("\n")
}

fn root_help() -> String {
    let mut out = vec![
        format!(
            "m365 {}: Microsoft 365 from the command line (Entra ID, SharePoint, Teams, Outlook,",
            env!("CARGO_PKG_VERSION")
        ),
        "Planner, OneDrive, Power Platform, Purview, Viva and SharePoint Framework).".to_string(),
        String::new(),
        "Everyday tasks, as m365 <suite> <resource> <verb>:".to_string(),
        "  m365 entra user get|list|add|set, m365 entra group member add".to_string(),
        "  m365 spo site list, m365 spo listitem list|add|set, m365 spo file get|add".to_string(),
        "  m365 teams message send, m365 outlook mail send, m365 planner task add".to_string(),
        "  m365 spfx project upgrade|doctor".to_string(),
        "Any Microsoft Graph or SharePoint REST call:".to_string(),
        "  m365 request --url <URL> [--method get|post|patch|put|delete] [--body JSON]".to_string(),
        String::new(),
        "Start with `m365 login`. Agents: add --agent (or set M365_AGENT=1) and read `m365 skill`."
            .to_string(),
        "Output: JSON by default; --output text|csv|md|none; --query filters with JMESPath."
            .to_string(),
        "Exit codes with --agent: 0 ok, 1 command or API error, 2 auth, 3 invalid input, 4 unknown"
            .to_string(),
        "command, 5 internal.".to_string(),
        String::new(),
        "Usage: m365 <suite|command> ... [options]".to_string(),
        String::new(),
        "Suites:".to_string(),
    ];
    let kids = children("");
    let suites: Vec<(String, String)> = kids
        .iter()
        .filter(|(_, c, sub)| c.is_none() && !sub.is_empty())
        .map(|(n, _, _)| {
            let count = manifest::COMMANDS
                .iter()
                .filter(|c| c.name.split(' ').next() == Some(n.as_str()))
                .count();
            (
                n.clone(),
                format!("{} ({count})", suite_description(n).unwrap_or("")),
            )
        })
        .collect();
    out.extend(table(&suites));
    out.push(String::new());
    out.push("Commands:".into());
    let mut cmds: Vec<(String, String)> = kids
        .iter()
        .filter_map(|(n, c, _)| c.map(|c| (n.clone(), summary(c).to_string())))
        .collect();
    cmds.extend(
        OWN_COMMANDS
            .iter()
            .map(|(n, d)| (n.to_string(), d.to_string())),
    );
    cmds.sort();
    out.extend(table(&cmds));
    out.push(String::new());
    out.push("Run `m365 <suite> --help` for its commands, `m365 <command> --help` for options and examples.".into());
    out.join("\n")
}

/// Group or root help for `--help --agent`: every command below `prefix`.
pub fn group_help_json(prefix: &str) -> Value {
    let below = format!("{prefix} ");
    let commands: Vec<Value> = manifest::COMMANDS
        .iter()
        .filter(|c| prefix.is_empty() || c.name == prefix || c.name.starts_with(&below))
        .map(|c| json!({ "name": c.name, "summary": summary(c) }))
        .collect();
    let mut v = json!({ "group": prefix, "commands": commands });
    if prefix.is_empty() {
        v["suites"] = SUITES
            .iter()
            .map(|(n, d)| json!({ "name": n, "description": d }))
            .collect();
        v["own_commands"] = OWN_COMMANDS
            .iter()
            .map(|(n, d)| json!({ "name": n, "summary": d }))
            .collect();
    } else if let Some(d) = suite_description(prefix) {
        v["description"] = Value::from(d);
    }
    v
}

/// Whether `prefix` names a group (some command starts with it).
pub fn is_group(prefix: &str) -> bool {
    let below = format!("{prefix} ");
    prefix.is_empty()
        || manifest::COMMANDS
            .iter()
            .any(|c| c.name.starts_with(&below))
}

/// The upstream docs page of a command (`helpTarget` set to `web`).
pub fn online_url(meta: &CmdMeta) -> String {
    let p = meta.help.replace('\\', "/");
    let p = match p.rfind('.') {
        Some(i) if !p[i..].contains('/') => p[..i].to_string(),
        _ => p,
    };
    format!("https://pnp.github.io/cli-microsoft365/cmd/{p}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_command_has_help_with_a_summary_and_examples() {
        let mut bad = Vec::new();
        for c in manifest::COMMANDS {
            match help_of(c) {
                None => bad.push(format!("{}: no help", c.name)),
                Some(h) => {
                    if h.summary.is_empty() {
                        bad.push(format!("{}: no summary", c.name));
                    }
                    if h.examples.is_empty() {
                        bad.push(format!("{}: no examples", c.name));
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }

    #[test]
    fn every_option_has_a_description() {
        let mut bad = Vec::new();
        for c in manifest::COMMANDS {
            for o in own_options(c) {
                if description(c, o).is_empty() {
                    bad.push(format!("{} --{}", c.name, o.long.unwrap_or(o.name)));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "options without a description:\n{}",
            bad.join("\n")
        );
    }

    #[test]
    fn help_describes_only_real_options() {
        let mut bad = Vec::new();
        for c in manifest::COMMANDS {
            let Some(h) = help_of(c) else { continue };
            for (n, _) in h.options {
                if !c
                    .options
                    .iter()
                    .any(|o| o.long.unwrap_or(o.name).eq_ignore_ascii_case(n))
                {
                    bad.push(format!("{} --{n}", c.name));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "help for options the command does not have:\n{}",
            bad.join("\n")
        );
    }

    #[test]
    fn examples_use_their_command_and_its_options() {
        let mut bad = Vec::new();
        for c in manifest::COMMANDS {
            let Some(h) = help_of(c) else { continue };
            let names: Vec<String> = std::iter::once(c.name.to_string())
                .chain(c.aliases.iter().map(|a| a.to_string()))
                .collect();
            for (_, cmd) in h.examples {
                // multi-line examples are scripts; check the lines that run m365
                for line in cmd.lines().filter(|l| l.trim_start().starts_with("m365 ")) {
                    let rest = line.trim_start().trim_start_matches("m365 ");
                    let Some(name) = names
                        .iter()
                        .find(|n| rest == n.as_str() || rest.starts_with(&format!("{n} ")))
                    else {
                        continue; // another command used in the script
                    };
                    if c.allow_unknown || c.loose {
                        continue;
                    }
                    for word in rest[name.len()..].split_whitespace() {
                        let Some(opt) = word.strip_prefix("--") else {
                            continue;
                        };
                        let opt = opt.split('=').next().unwrap_or(opt);
                        let global = GLOBAL.contains(&opt) || opt == "help";
                        if !global
                            && !c.options.iter().any(|o| o.long.unwrap_or(o.name) == opt)
                            && !opt.starts_with("no-")
                        {
                            bad.push(format!("{}: example uses --{opt}", c.name));
                        }
                    }
                }
            }
        }
        // upstream's own docs contain a few; keep the count from growing
        assert!(
            bad.len() <= 20,
            "{} examples use unknown options:\n{}",
            bad.len(),
            bad.join("\n")
        );
    }

    #[test]
    fn renders_user_get_help() {
        let meta = manifest::find("entra user get").unwrap();
        let h = command_help(meta, "full").unwrap();
        assert!(h.starts_with("Gets information about the specified user\n\nUsage: m365 entra user get [options]\n"), "{h}");
        assert!(
            h.contains(
                "\n  -i, --id <ID>\n          The ID of the user to retrieve information for.\n"
            ),
            "{h}"
        );
        assert!(
            h.contains("- Use exactly one of: --id, --userName, --email."),
            "{h}"
        );
        assert!(h.contains("\nExamples:\n  Get information about the user by id.\n    m365 entra user get --id "), "{h}");
        assert!(h.contains("\nPermissions:\n  Delegated:   Microsoft Graph: User.Read.All\n  Application: Microsoft Graph: User.Read.All"), "{h}");
        let j = command_help_json(meta);
        assert_eq!(j["options"][0]["name"], "id");
        assert_eq!(
            j["rules"][0],
            "Use exactly one of: --id, --userName, --email."
        );
    }

    #[test]
    fn usage_shows_required_options() {
        let meta = manifest::find("spo web get").unwrap();
        assert_eq!(usage(meta), "Usage: m365 spo web get --url <URL> [options]");
    }

    #[test]
    fn wraps_at_the_width() {
        let text = wrap(&"word ".repeat(60), 10);
        assert!(text.lines().all(|l| l.len() <= WIDTH), "{text}");
        assert!(
            text.lines().all(|l| l.starts_with("          word")),
            "{text}"
        );
    }
}

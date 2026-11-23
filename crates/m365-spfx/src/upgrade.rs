//! `spfx project upgrade`.

use crate::project::{self, Project};
use crate::report::{self, EOL, Item, Packages, pm, shell_cmd};
use crate::rules::{self, CATALOG, Finding};
use m365_core::prelude::*;

pub const SUPPORTED: [&str; 52] = [
    "1.0.0", "1.0.1", "1.0.2", "1.1.0", "1.1.1", "1.1.3", "1.2.0", "1.3.0", "1.3.1", "1.3.2",
    "1.3.4", "1.4.0", "1.4.1", "1.5.0", "1.5.1", "1.6.0", "1.7.0", "1.7.1", "1.8.0", "1.8.1",
    "1.8.2", "1.9.1", "1.10.0", "1.11.0", "1.12.0", "1.12.1", "1.13.0", "1.13.1", "1.14.0",
    "1.15.0", "1.15.2", "1.16.0", "1.16.1", "1.17.0", "1.17.1", "1.17.2", "1.17.3", "1.17.4",
    "1.18.0", "1.18.1", "1.18.2", "1.19.0", "1.20.0", "1.21.0", "1.21.1", "1.22.0", "1.22.1",
    "1.22.2", "1.23.0", "1.23.2", "1.24.0", "1.24.1",
];

/// `getProjectRoot(process.cwd())` or the command's error 1.
pub fn root() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().map_err(|e| Error::msg(e.to_string()))?;
    project::project_root(&cwd)
        .ok_or_else(|| Error::with_code("Couldn't find project root folder", 1))
}

pub async fn run(cx: &mut Cx) -> Result<()> {
    let root = root()?;
    let to = cx
        .s("toVersion")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| SUPPORTED[SUPPORTED.len() - 1].to_string());
    let manager = cx
        .s("packageManager")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "npm".into());
    let shell = cx
        .s("shell")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "powershell".into());
    let Some(pos_to) = SUPPORTED.iter().position(|v| *v == to) else {
        return Err(Error::with_code(
            format!(
                "CLI for Microsoft 365 doesn't support upgrading SharePoint Framework projects to version {to}. Supported versions are {}",
                SUPPORTED.join(", ")
            ),
            2,
        ));
    };
    let Some(version) = project::project_version(&root).filter(|v| !v.is_empty()) else {
        return Err(Error::with_code(
            "Unable to determine the version of the current SharePoint Framework project",
            3,
        ));
    };
    let Some(pos) = SUPPORTED.iter().position(|v| *v == version) else {
        return Err(Error::with_code(
            format!(
                "CLI for Microsoft 365 doesn't support upgrading projects built using SharePoint Framework v{version}"
            ),
            4,
        ));
    };
    if pos > pos_to {
        return Err(Error::with_code("You cannot downgrade a project", 5));
    }
    if pos == pos_to {
        return cx.log(Value::from("Project doesn't need to be upgraded"));
    }
    cx.verbose_log("Collecting project...");
    let project = Project::load(&root)?;
    if cx.debug() {
        cx.stderr("Collected project");
        cx.stderr(&project.inspect());
    }
    let mut all: Vec<Finding> = Vec::new();
    for v in SUPPORTED[pos + 1..=pos_to].iter().rev() {
        for &i in &CATALOG.upgrade[*v] {
            rules::visit(&CATALOG.rules[i], &project, &mut all)
                .map_err(|e| Error::msg(e.js_message()))?;
        }
    }
    if manager == "npm" {
        crate::custom::npm_dedupe(&CATALOG.rules[CATALOG.npm_dedupe], &mut all);
    }
    // dedupe by id, keeping the first
    let mut findings: Vec<Finding> = all
        .iter()
        .enumerate()
        .filter(|(i, f)| all.iter().position(|x| x.id == f.id) == Some(*i))
        .map(|(_, f)| f.clone())
        .collect();
    report::remove_superseded(&mut findings);
    // remove findings without title (`forEach` while splicing skips the next one)
    let n = findings.len();
    let mut k = 0;
    while k < n && k < findings.len() {
        if findings[k].title.as_deref().is_none_or(str::is_empty) {
            let id = findings[k].id.clone();
            if let Some(i) = findings.iter().position(|f| f.id == id) {
                findings.remove(i);
            }
        }
        k += 1;
    }
    let mut items = report::flatten(&findings);
    for f in items.iter_mut() {
        replace_tokens(f, &manager, &shell);
    }
    let ctx = Ctx {
        manager: &manager,
        shell: &shell,
        to: &to,
    };
    match cx.s("output").as_deref() {
        Some("text") => cx.log(Value::from(text_report(&items, &ctx))),
        Some("tour") => report::write_tour(
            &root,
            "upgrade.tour",
            &tour_report(&items, &project, &root, &ctx)?,
        ),
        Some("md") => {
            let s = md_report(&items, &root, &ctx)?;
            cx.log_raw(&s);
            Ok(())
        }
        _ => cx.log(Value::Array(items.iter().map(Item::to_value).collect())),
    }
}

struct Ctx<'a> {
    manager: &'a str,
    shell: &'a str,
    to: &'a str,
}

/// Replaces package operation and shell tokens in a resolution.
fn replace_tokens(f: &mut Item, manager: &str, shell: &str) {
    let r = &mut f.resolution;
    for k in [
        "removeOverride",
        "override",
        "uninstallDev",
        "installDev",
        "uninstall",
        "install",
    ] {
        if r.starts_with(k) {
            *r = r.replacen(k, pm(k, manager), 1);
            return;
        }
    }
    if r.starts_with("copy_cmd") {
        *r = r
            .replacen("copy_cmd", shell_cmd("copyCommand", shell), 1)
            .replacen(
                "DestinationParam",
                shell_cmd("copyDestinationParam", shell),
                1,
            );
        return;
    }
    if r.starts_with("create_dir_cmd") {
        *r = r
            .replacen(
                "create_dir_cmd",
                shell_cmd("createDirectoryCommand", shell),
                1,
            )
            .replacen("NameParam", shell_cmd("createDirectoryNameParam", shell), 1)
            .replacen("PathParam", shell_cmd("createDirectoryPathParam", shell), 1)
            .replacen(
                "ItemTypeParam",
                shell_cmd("createDirectoryItemTypeParam", shell),
                1,
            );
        return;
    }
    if r.starts_with("add_cmd") {
        let between = |a: &str, b: &str| -> String {
            let start = r.find(a).map(|i| i + a.len()).unwrap_or(a.len() - 1);
            let end = r.find(b).unwrap_or(0);
            js_substring(r, start, end)
        };
        let path = between("[BEFOREPATH]", "[AFTERPATH]");
        let content = between("[BEFORECONTENT]", "[AFTERCONTENT]");
        *r = shell_cmd("addFileCommand", shell)
            .replacen("[FILECONTENT]", &content, 1)
            .replacen("[FILEPATH]", &path, 1)
            .replacen("[BEFOREPATH]", " ", 1)
            .replacen("[AFTERPATH]", " ", 1)
            .replacen("[BEFORECONTENT]", " ", 1)
            .replacen("[AFTERCONTENT]", " ", 1);
        return;
    }
    if r.starts_with("remove_cmd") {
        *r = r.replacen("remove_cmd", shell_cmd("removeFileCommand", shell), 1);
    }
}

/// `String#substring(a, b)` on byte offsets (swapped when `a > b`).
fn js_substring(s: &str, a: usize, b: usize) -> String {
    let (a, b) = if a > b { (b, a) } else { (a, b) };
    s.get(a.min(s.len())..b.min(s.len()))
        .unwrap_or("")
        .to_string()
}

struct ReportData {
    commands: Vec<String>,
    pm_commands: Vec<String>,
    per_file: Vec<(String, Vec<(String, String)>)>,
    type_per_file: Vec<(String, String)>,
}

fn report_data(items: &[Item], ctx: &Ctx) -> ReportData {
    let mut commands = Vec::new();
    let mut per_file: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let mut type_per_file: Vec<(String, String)> = Vec::new();
    let mut pk = Packages::default();
    for f in items {
        if f.resolution_type == "cmd" {
            if f.resolution.contains("npm") {
                pk.map(&f.resolution, ctx.manager);
            } else {
                commands.push(f.resolution.clone());
            }
        } else {
            match per_file.iter_mut().find(|(k, _)| *k == f.file) {
                Some((_, v)) => v.push((f.description.clone(), f.resolution.clone())),
                None => per_file.push((
                    f.file.clone(),
                    vec![(f.description.clone(), f.resolution.clone())],
                )),
            }
            if !type_per_file
                .iter()
                .any(|(k, t)| *k == f.file && !t.is_empty())
            {
                type_per_file.retain(|(k, _)| *k != f.file);
                type_per_file.push((f.file.clone(), f.resolution_type.clone()));
            }
        }
    }
    let mut pm_commands = pk.reduce(ctx.manager);
    if ctx.manager == "npm"
        && let Some(d) = items.iter().find(|f| f.id == "FN017001")
    {
        pm_commands.push(d.resolution.clone());
    }
    ReportData {
        commands,
        pm_commands,
        per_file,
        type_per_file,
    }
}

/// The package manager commands followed by the other commands that do not
/// contain one.
fn script(d: &ReportData, manager: &str) -> String {
    let pms = [
        "install",
        "installDev",
        "uninstall",
        "uninstallDev",
        "override",
        "removeOverride",
    ]
    .map(|k| pm(k, manager));
    let mut all = d.pm_commands.clone();
    all.extend(
        d.commands
            .iter()
            .filter(|c| pms.iter().all(|p| !c.contains(p)))
            .cloned(),
    );
    all.join(EOL)
}

fn text_report(items: &[Item], ctx: &Ctx) -> String {
    let d = report_data(items, ctx);
    let files: Vec<String> = d
        .per_file
        .iter()
        .map(|(file, mods)| {
            let m: Vec<String> = mods
                .iter()
                .map(|(desc, m)| format!("{desc}:{EOL}{m}{EOL}"))
                .collect();
            format!(
                "{file}{EOL}{}{EOL}{}{EOL}",
                "-".repeat(report::js_len(file)),
                m.join(EOL)
            )
        })
        .collect();
    let s = format!(
        "Execute in {}{EOL}-----------------------{EOL}{}{EOL}{EOL}{}{EOL}",
        ctx.shell,
        script(&d, ctx.manager),
        files.join(EOL)
    );
    s.trim().to_string()
}

fn position_suffix(f: &Item) -> String {
    f.position
        .map(|(l, c)| format!(":{l}:{c}"))
        .unwrap_or_default()
}

fn md_report(items: &[Item], root: &std::path::Path, ctx: &Ctx) -> Result<String> {
    let name = report::project_name(root)?;
    let d = report_data(items, ctx);
    let mut findings = String::new();
    for f in items {
        let resolution = match f.resolution_type.as_str() {
            "cmd" => format!(
                "Execute the following command:\n\n```sh\n{}\n```\n",
                f.resolution
            ),
            "json" | "js" | "ts" | "scss" => {
                format!("```{}\n{}\n```\n", f.resolution_type, f.resolution)
            }
            _ => String::new(),
        };
        findings.push_str(&format!(
            "### {} {} | {}{EOL}{EOL}{}{EOL}{EOL}{resolution}{EOL}File: [{}{}]({}){EOL}{EOL}",
            f.id,
            f.title(),
            f.severity,
            f.description,
            f.file,
            position_suffix(f),
            f.file
        ));
    }
    let files: Vec<String> = d
        .per_file
        .iter()
        .map(|(file, mods)| {
            let ty = d
                .type_per_file
                .iter()
                .find(|(k, _)| k == file)
                .map(|(_, t)| t.as_str())
                .unwrap_or("undefined");
            let m: Vec<String> = mods
                .iter()
                .map(|(desc, m)| format!("{desc}:{EOL}{EOL}```{ty}{EOL}{m}{EOL}```"))
                .collect();
            format!(
                "#### [{file}]({file}){EOL}{EOL}{}{EOL}",
                m.join(&format!("{EOL}{EOL}"))
            )
        })
        .collect();
    let s = format!(
        "# Upgrade project {name} to v{to}{EOL}{EOL}Date: {date}{EOL}{EOL}## Findings{EOL}{EOL}Following is the list of steps required to upgrade your project to SharePoint Framework version {to}. [Summary](#Summary) of the modifications is included at the end of the report.{EOL}{EOL}{findings}## Summary{EOL}{EOL}### Execute script{EOL}{EOL}```sh{EOL}{script}{EOL}```{EOL}{EOL}### Modify files{EOL}{EOL}{files}{EOL}",
        to = ctx.to,
        date = m365_core::time::locale_date_string(chrono::Local::now()),
        script = script(&d, ctx.manager),
        files = files.join(EOL),
    );
    Ok(s.trim().to_string())
}

fn tour_report(
    items: &[Item],
    project: &Project,
    root: &std::path::Path,
    ctx: &Ctx,
) -> Result<String> {
    let name = report::project_name(root)?;
    let mut steps = Vec::new();
    for f in items {
        let line = f.position.map(|(l, _)| l).filter(|l| *l != 0).unwrap_or(1);
        let resolution = match f.resolution_type.as_str() {
            "cmd" => format!(
                "Execute the following command:\r\n\r\n[`{0}`](command:codetour.sendTextToTerminal?[\"{0}\"])",
                f.resolution
            ),
            "json" | "js" | "ts" | "scss" => {
                format!("\r\n```{}\r\n{}\r\n```", f.resolution_type, f.resolution)
            }
            _ => String::new(),
        };
        let sev = f.severity.to_uppercase();
        let exists = std::path::Path::new(&project.path).join(&f.file).exists();
        let mut step = obj! {
            "title": format!("{sev}: {} ({})", f.title(), f.id),
            "description": format!("### {sev}\r\n\r\n{}\r\n\r\n{resolution}", f.description),
            "line": line,
        };
        if exists {
            step["file"] = Value::from(f.file.replace("./", "").replace('\\', "/"));
        } else {
            step["directory"] = Value::from("");
        }
        steps.push(step);
    }
    steps.push(obj! {
        "file": ".tours/upgrade.tour",
        "title": "RECOMMENDED: Delete tour",
        "description": "### THAT'S IT!!!\r\nOnce you have tested that your upgrade is successful, you can delete the `.tour` folder and its contents. Otherwise, you'll be prompted to launch this CodeTour every time you open this project.",
    });
    let tour = obj! { "title": format!("Upgrade project {name} to v{}", ctx.to), "steps": Value::Array(steps) };
    Ok(json::stringify_pretty(&tour, "  "))
}

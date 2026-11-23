//! `spfx project doctor`.

use crate::project::{self, Project};
use crate::report::{self, EOL, Item, Packages, pm};
use crate::rules::{self, CATALOG, Finding};
use crate::upgrade::SUPPORTED;
use m365_core::prelude::*;

pub async fn run(cx: &mut Cx) -> Result<()> {
    let root = crate::upgrade::root()?;
    let manager = cx
        .s("packageManager")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "npm".into());
    cx.verbose_log("Collecting project...");
    let mut project = Project::load(&root)?;
    if cx.debug() {
        cx.stderr("Collected project");
        cx.stderr(&project.inspect());
    }
    let Some(version) = project::project_version(&root).filter(|v| !v.is_empty()) else {
        return Err(Error::with_code(
            "Unable to determine the version of the current SharePoint Framework project",
            3,
        ));
    };
    if !SUPPORTED.contains(&version.as_str()) {
        return Err(Error::with_code(
            format!(
                "CLI for Microsoft 365 doesn't support validating projects built using SharePoint Framework v{version}"
            ),
            4,
        ));
    }
    cx.verbose_log(&format!("Project built using SPFx v{version}"));
    project.version = Some(version.clone());
    let mut all: Vec<Finding> = Vec::new();
    let ids = CATALOG.doctor["generic"].iter().chain(
        CATALOG
            .doctor
            .get(&version)
            .map(Vec::as_slice)
            .unwrap_or(&[]),
    );
    for &i in ids {
        rules::visit(&CATALOG.rules[i], &project, &mut all)?;
    }
    if manager == "npm" {
        crate::custom::npm_dedupe(&CATALOG.rules[CATALOG.npm_dedupe], &mut all);
    }
    report::remove_superseded(&mut all);
    let mut items = report::flatten(&all);
    for f in items.iter_mut() {
        for k in ["uninstallDev", "installDev", "uninstall", "install"] {
            if f.resolution.starts_with(k) {
                f.resolution = f.resolution.replacen(k, pm(k, &manager), 1);
                break;
            }
        }
    }
    match cx.s("output").as_deref() {
        Some("text") => cx.log(Value::from(text_report(&items, &manager))),
        Some("tour") => report::write_tour(&root, "validation.tour", &tour_report(&items, &root)?),
        Some("md") => {
            let s = md_report(&items, &root, &manager)?;
            cx.log_raw(&s);
            Ok(())
        }
        _ => cx.log(Value::Array(items.iter().map(Item::to_value).collect())),
    }
}

fn pm_commands(items: &[Item], manager: &str) -> Vec<String> {
    let mut pk = Packages::default();
    for f in items {
        pk.map(&f.resolution, manager);
    }
    let mut out = pk.reduce(manager);
    if manager == "npm"
        && let Some(d) = items.iter().find(|f| f.id == "FN017001")
    {
        out.push(d.resolution.clone());
    }
    out
}

fn text_report(items: &[Item], manager: &str) -> String {
    if items.is_empty() {
        return "✅ CLI for Microsoft 365 has found no issues in your project".into();
    }
    format!(
        "Execute in command line{EOL}-----------------------{EOL}{}{EOL}{EOL}",
        pm_commands(items, manager).join(EOL)
    )
    .trim()
    .to_string()
}

fn md_report(items: &[Item], root: &std::path::Path, manager: &str) -> Result<String> {
    let name = report::project_name(root)?;
    let mut findings = String::new();
    for f in items {
        let resolution = if f.resolution_type == "cmd" {
            format!(
                "Execute the following command:\n\n```sh\n{}\n```\n",
                f.resolution
            )
        } else {
            String::new()
        };
        let pos = f
            .position
            .map(|(l, c)| format!(":{l}:{c}"))
            .unwrap_or_default();
        findings.push_str(&format!(
            "### {} {} | {}{EOL}{EOL}{}{EOL}{EOL}{resolution}{EOL}File: [{}{pos}]({}){EOL}{EOL}",
            f.id,
            f.title(),
            f.severity,
            f.description,
            f.file,
            f.file
        ));
    }
    let mut s = format!(
        "# Validate project {name}{EOL}{EOL}Date: {}{EOL}{EOL}## Findings{EOL}{EOL}",
        m365_core::time::locale_date_string(chrono::Local::now())
    );
    if findings.is_empty() {
        s.push_str(&format!(
            "✅ CLI for Microsoft 365 has found no issues in your project{EOL}"
        ));
    } else {
        s.push_str(&format!(
            "Following is the list of issues found in your project. [Summary](#Summary) of the recommended fixes is included at the end of the report.{EOL}{EOL}{findings}## Summary{EOL}{EOL}### Execute script{EOL}{EOL}```sh{EOL}{}{EOL}```{EOL}{EOL}",
            pm_commands(items, manager).join(EOL)
        ));
    }
    Ok(s.trim().to_string())
}

fn tour_report(items: &[Item], root: &std::path::Path) -> Result<String> {
    let name = report::project_name(root)?;
    let mut steps: Vec<Value> = items
        .iter()
        .map(|f| {
            let line = f.position.map(|(l, _)| l).filter(|l| *l != 0).unwrap_or(1);
            let resolution = if f.resolution_type == "cmd" { format!("Execute the following command:\r\n\r\n[`{0}`](command:codetour.sendTextToTerminal?[\"{0}\"])", f.resolution) } else { String::new() };
            let sev = f.severity.to_uppercase();
            obj! {
                "file": f.file.replace("./", "").replace('\\', "/"),
                "title": format!("{sev}: {} ({})", f.title(), f.id),
                "description": format!("### {sev}\r\n\r\n{}\r\n\r\n{resolution}", f.description),
                "line": line,
            }
        })
        .collect();
    steps.push(obj! {
        "file": ".tours/validation.tour",
        "title": "RECOMMENDED: Delete tour",
        "description": "### THAT'S IT!!!\r\nOnce you have tested that your project has no more issues, you can delete the `.tour` folder and its contents. Otherwise, you'll be prompted to launch this CodeTour every time you open this project.",
    });
    Ok(json::stringify_pretty(
        &obj! { "title": format!("Validate project {name}"), "steps": Value::Array(steps) },
        "  ",
    ))
}

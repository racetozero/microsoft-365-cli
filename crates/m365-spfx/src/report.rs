//! Report pieces shared by `spfx project upgrade` and `spfx project doctor`:
//! `packageManager` (utils/packageManager.ts), the shell commands of
//! `fsUtil`, flattening findings and the tour file.

use crate::rules::Finding;
use m365_core::prelude::*;

/// `os.EOL`.
pub const EOL: &str = m365_core::term::EOL;

/// `packageManager.getPackageManagerCommand`.
pub fn pm(command: &str, manager: &str) -> &'static str {
    match (manager, command) {
        ("npm", "install") => "npm i -SE",
        ("npm", "installDev") => "npm i -DE",
        ("npm", "installLockFile") => "npm i",
        ("npm", "uninstall") => "npm un -S",
        ("npm", "uninstallDev") => "npm un -D",
        ("npm", "override") => "npm pkg set",
        ("npm", "removeOverride") => "npm pkg delete",
        ("pnpm", "install") => "pnpm i -E",
        ("pnpm", "installDev") => "pnpm i -DE",
        ("pnpm", "installLockFile") => "pnpm i",
        ("pnpm", "uninstall") => "pnpm un",
        ("pnpm", "uninstallDev") => "pnpm un",
        ("pnpm", "override") => "pnpm pkg set",
        ("pnpm", "removeOverride") => "pnpm pkg delete",
        ("yarn", "install") => "yarn add -E",
        ("yarn", "installDev") => "yarn add -DE",
        ("yarn", "uninstall") => "yarn remove",
        ("yarn", "uninstallDev") => "yarn remove",
        _ => "undefined",
    }
}

/// Packages collected by `mapPackageManagerCommand`.
#[derive(Default)]
pub struct Packages {
    dep_exact: Vec<String>,
    dev_exact: Vec<String>,
    dep_un: Vec<String>,
    dev_un: Vec<String>,
    overrides: Vec<String>,
    override_remove: Vec<String>,
}

impl Packages {
    /// `packageManager.mapPackageManagerCommand`.
    pub fn map(&mut self, command: &str, manager: &str) {
        type List = fn(&mut Packages) -> &mut Vec<String>;
        let order: [(&str, List); 6] = [
            ("removeOverride", |p| &mut p.override_remove),
            ("override", |p| &mut p.overrides),
            ("uninstallDev", |p| &mut p.dev_un),
            ("installDev", |p| &mut p.dev_exact),
            ("uninstall", |p| &mut p.dep_un),
            ("install", |p| &mut p.dep_exact),
        ];
        for (k, list) in order {
            let c = pm(k, manager);
            if command.starts_with(&format!("{c} ")) {
                list(self).push(command.replacen(c, "", 1).trim().to_string());
                return;
            }
        }
    }

    /// `packageManager.reducePackageManagerCommand`.
    pub fn reduce(&self, manager: &str) -> Vec<String> {
        let mut out = Vec::new();
        if !self.override_remove.is_empty() {
            out.push(format!(
                "{} {}",
                pm("removeOverride", manager),
                self.override_remove.join(" ")
            ));
            if self.dep_un.is_empty()
                && self.dev_un.is_empty()
                && self.dep_exact.is_empty()
                && self.dev_exact.is_empty()
                && self.overrides.is_empty()
            {
                out.push(pm("installLockFile", manager).into());
            }
        }
        for (k, l) in [
            ("uninstall", &self.dep_un),
            ("uninstallDev", &self.dev_un),
            ("install", &self.dep_exact),
            ("installDev", &self.dev_exact),
        ] {
            if !l.is_empty() {
                out.push(format!("{} {}", pm(k, manager), l.join(" ")));
            }
        }
        if !self.overrides.is_empty() {
            out.push(format!(
                "{} {}",
                pm("override", manager),
                self.overrides.join(" ")
            ));
            out.push(pm("installLockFile", manager).into());
        }
        out
    }
}

/// `fsUtil.getCopyCommand` & co.
pub fn shell_cmd(kind: &str, shell: &str) -> &'static str {
    match (shell, kind) {
        ("bash", "copyCommand") => "cp",
        ("bash", "copyDestinationParam") => " ",
        ("powershell", "copyCommand") => "Copy-Item",
        ("powershell", "copyDestinationParam") => " -Destination ",
        ("cmd", "copyCommand") => "copy",
        ("cmd", "copyDestinationParam") => " ",
        ("bash", "createDirectoryCommand") => "mkdir",
        ("bash", "createDirectoryPathParam") => " \"",
        ("bash", "createDirectoryNameParam") => "/",
        ("bash", "createDirectoryItemTypeParam") => "\"",
        ("powershell", "createDirectoryCommand") => "New-Item",
        ("powershell", "createDirectoryPathParam") => " -Path \"",
        ("powershell", "createDirectoryNameParam") => "\" -Name \"",
        ("powershell", "createDirectoryItemTypeParam") => "\" -ItemType \"directory\"",
        ("cmd", "createDirectoryCommand") => "mkdir",
        ("cmd", "createDirectoryPathParam") => " \"",
        ("cmd", "createDirectoryNameParam") => "\\",
        ("cmd", "createDirectoryItemTypeParam") => "\"",
        ("bash", "addFileCommand") => "cat > [FILEPATH] << EOF [FILECONTENT]EOF",
        ("powershell", "addFileCommand") => "@'[FILECONTENT]'@ | Out-File -FilePath [FILEPATH]",
        ("cmd", "addFileCommand") => "echo [FILECONTENT] > [FILEPATH]",
        ("bash", "removeFileCommand") => "rm",
        ("powershell", "removeFileCommand") => "Remove-Item",
        ("cmd", "removeFileCommand") => "del",
        _ => "undefined",
    }
}

/// A flattened finding (`FindingToReport`).
#[derive(Clone, Debug)]
pub struct Item {
    pub description: String,
    pub id: String,
    pub file: String,
    pub position: Option<(usize, usize)>,
    pub resolution: String,
    pub resolution_type: String,
    pub severity: String,
    pub title: Option<String>,
}

impl Item {
    pub fn title(&self) -> String {
        self.title.clone().unwrap_or_else(|| "undefined".into())
    }

    pub fn to_value(&self) -> Value {
        obj! {
            "description": self.description.as_str(),
            "id": self.id.as_str(),
            "file": self.file.as_str(),
            "position": ?self.position.map(|(l, c)| obj! { "line": l, "character": c }),
            "resolution": self.resolution.as_str(),
            "resolutionType": self.resolution_type.as_str(),
            "severity": self.severity.as_str(),
            "title": ?self.title.clone(),
        }
    }
}

/// Flattens findings into one item per occurrence.
pub fn flatten(findings: &[Finding]) -> Vec<Item> {
    findings
        .iter()
        .flat_map(|f| {
            f.occurrences.iter().map(|o| Item {
                description: f.description.clone(),
                id: f.id.clone(),
                file: o.file.clone(),
                position: o.position,
                resolution: o.resolution.clone(),
                resolution_type: f.resolution_type.clone(),
                severity: f.severity.clone(),
                title: f.title.clone(),
            })
        })
        .collect()
}

/// Removes findings superseded by others (`findings.splice(findIndex(...))`).
pub fn remove_superseded(findings: &mut Vec<Finding>) {
    let superseding: Vec<Vec<String>> = findings
        .iter()
        .filter(|f| !f.supersedes.is_empty())
        .map(|f| f.supersedes.clone())
        .collect();
    for list in superseding {
        for s in list {
            if let Some(i) = findings.iter().position(|f| f.id == s) {
                findings.remove(i);
            }
        }
    }
}

/// Writes `.tours/<name>` in the project.
pub fn write_tour(root: &std::path::Path, name: &str, content: &str) -> Result<()> {
    let dir = root.join(".tours");
    if !dir.exists() {
        std::fs::create_dir(&dir)
            .map_err(|e| Error::js(util::node_fs_error(&e, "mkdir", &dir.to_string_lossy())))?;
    }
    let file = dir.join(name);
    std::fs::write(&file, content)
        .map_err(|e| Error::js(util::node_fs_error(&e, "open", &file.to_string_lossy())))
}

/// `packageSolutionJson?.solution?.name` of a freshly loaded project.
pub fn project_name(root: &std::path::Path) -> Result<String> {
    let p = crate::project::Project::load(root)?;
    Ok(p.package_solution_json
        .as_ref()
        .and_then(|j| j.get("solution"))
        .and_then(|s| s.get("name"))
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into()))
}

/// JavaScript `String#length` (UTF-16 code units).
pub fn js_len(s: &str) -> usize {
    s.encode_utf16().count()
}

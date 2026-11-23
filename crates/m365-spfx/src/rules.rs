//! The upgrade/doctor rule engine. Rule instances are data extracted from
//! upstream (`spfx-rules.json`, see `scripts/spfx-rules.mjs`): the class,
//! the constructor fields and the evaluated getters. The generic bases
//! (`DependencyRule`, `FileAddRemoveRule`, `PackageRule`, `ResolutionRule`)
//! are implemented here; classes with their own `visit` or
//! `customCondition` live in `custom.rs`.

use crate::project::Project;
use crate::semver;
use m365_core::prelude::*;
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct Rule {
    pub class: String,
    pub fields: Value,
    pub props: Value,
}

impl Rule {
    fn p(&self, k: &str) -> String {
        // `os.EOL` in a getter (see `scripts/spfx-rules.mjs`)
        self.props
            .get(k)
            .map(|v| to_js_string(v).replace("\u{0}EOL\u{0}", m365_core::term::EOL))
            .unwrap_or_else(|| "undefined".into())
    }
    pub fn id(&self) -> String {
        self.p("id")
    }
    /// `undefined` when the getter returns nothing.
    pub fn title(&self) -> Option<String> {
        self.props.get("title").map(to_js_string)
    }
    pub fn description(&self) -> String {
        self.p("description")
    }
    pub fn resolution(&self) -> String {
        self.p("resolution")
    }
    pub fn file(&self) -> String {
        self.p("file")
    }
    /// A constructor field (`this.packageName`).
    pub fn f(&self, k: &str) -> Option<&Value> {
        self.fields.get(k)
    }
    pub fn fs(&self, k: &str) -> String {
        self.f(k)
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into())
    }
    pub fn fb(&self, k: &str) -> bool {
        self.f(k).is_some_and(truthy)
    }

    /// `addFindingWithCustomInfo`.
    pub fn add_custom(
        &self,
        title: Option<String>,
        description: String,
        occurrences: Vec<Occurrence>,
        findings: &mut Vec<Finding>,
    ) {
        findings.push(Finding {
            id: self.id(),
            title,
            description,
            occurrences,
            resolution_type: self.p("resolutionType"),
            severity: self.p("severity"),
            supersedes: self
                .props
                .get("supersedes")
                .and_then(Value::as_array)
                .map(|a| a.iter().map(to_js_string).collect())
                .unwrap_or_default(),
        });
    }
    /// `addFindingWithOccurrences`.
    pub fn add_occurrences(&self, occurrences: Vec<Occurrence>, findings: &mut Vec<Finding>) {
        self.add_custom(self.title(), self.description(), occurrences, findings);
    }
    /// `addFinding`: one occurrence in `this.file` without a position.
    pub fn add(&self, findings: &mut Vec<Finding>) {
        self.add_occurrences(
            vec![Occurrence {
                file: self.file(),
                resolution: self.resolution(),
                position: None,
            }],
            findings,
        );
    }
    /// `addFindingWithPosition(findings, node)`.
    pub fn add_at(&self, findings: &mut Vec<Finding>, pos: (usize, usize)) {
        self.add_occurrences(
            vec![Occurrence {
                file: self.file(),
                resolution: self.resolution(),
                position: Some(pos),
            }],
            findings,
        );
    }
}

#[derive(Clone, Debug)]
pub struct Occurrence {
    pub file: String,
    pub resolution: String,
    pub position: Option<(usize, usize)>,
}

#[derive(Clone, Debug)]
pub struct Finding {
    pub id: String,
    pub title: Option<String>,
    pub description: String,
    pub occurrences: Vec<Occurrence>,
    pub resolution_type: String,
    pub severity: String,
    pub supersedes: Vec<String>,
}

pub struct ClassInfo {
    pub visit: Option<String>,
    pub custom: Option<String>,
}

pub struct Catalog {
    pub upgrade: HashMap<String, Vec<usize>>,
    pub doctor: HashMap<String, Vec<usize>>,
    pub npm_dedupe: usize,
    pub classes: HashMap<String, ClassInfo>,
    pub rules: Vec<Rule>,
}

pub static CATALOG: LazyLock<Catalog> = LazyLock::new(|| {
    let v: Value = serde_json::from_str(include_str!("spfx-rules.json")).expect("spfx-rules.json");
    let sets = |k: &str| -> HashMap<String, Vec<usize>> {
        v[k].as_object()
            .expect("sets")
            .iter()
            .map(|(ver, ids)| {
                (
                    ver.clone(),
                    ids.as_array()
                        .expect("ids")
                        .iter()
                        .map(|i| i.as_u64().expect("id") as usize)
                        .collect(),
                )
            })
            .collect()
    };
    Catalog {
        upgrade: sets("upgrade"),
        doctor: sets("doctor"),
        npm_dedupe: v["npmDedupe"].as_u64().expect("npmDedupe") as usize,
        classes: v["classes"]
            .as_object()
            .expect("classes")
            .iter()
            .map(|(k, c)| {
                (
                    k.clone(),
                    ClassInfo {
                        visit: c.get("visit").and_then(Value::as_str).map(String::from),
                        custom: c.get("custom").and_then(Value::as_str).map(String::from),
                    },
                )
            })
            .collect(),
        rules: v["rules"]
            .as_array()
            .expect("rules")
            .iter()
            .map(|r| Rule {
                class: s(r, "class").to_string(),
                fields: r["fields"].clone(),
                props: r["props"].clone(),
            })
            .collect(),
    }
});

/// `rule.visit(project, findings)`.
pub fn visit(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let info = CATALOG.classes.get(&rule.class);
    match info.and_then(|i| i.visit.as_deref()) {
        Some("DependencyRule") => dependency_visit(rule, project, findings),
        Some("FileAddRemoveRule") => file_add_remove(rule, project, findings),
        Some("PackageRule") => package(rule, project, findings),
        Some("ResolutionRule") => resolution(rule, project, findings),
        Some("DependencyRule@doctor") => doctor_dependency(rule, project, findings),
        _ => crate::custom::visit(rule, project, findings),
    }
}

/// `customCondition(project)` (false unless the class overrides it).
fn custom_condition(rule: &Rule, project: &Project) -> bool {
    match CATALOG
        .classes
        .get(&rule.class)
        .and_then(|i| i.custom.as_deref())
    {
        Some(c) if c == rule.class => crate::custom::condition(rule, project),
        _ => false,
    }
}

/// `project-doctor/rules/DependencyRule.visit`.
fn doctor_dependency(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let Some(pkg) = &project.package_json else {
        return Ok(());
    };
    let custom = match CATALOG
        .classes
        .get(&rule.class)
        .and_then(|i| i.custom.as_deref())
    {
        Some(c) if c == rule.class => crate::custom::condition(rule, project),
        _ => true,
    };
    if !custom {
        return Ok(());
    }
    let is_dev = rule.fb("isDevDep");
    let section = if is_dev {
        "devDependencies"
    } else {
        "dependencies"
    };
    let name = rule.fs("packageName");
    let range = rule.fs("supportedRange");
    let deps = pkg.get(section).filter(|d| truthy(d));
    let min = match deps {
        None => None,
        Some(d) => match d.get(&name).filter(|v| truthy(v)) {
            None => None,
            Some(Value::String(v)) => semver::coerce(v),
            Some(Value::Number(n)) => {
                semver::coerce(&json::number_to_string(n.as_f64().unwrap_or(0.0)))
            }
            Some(_) => None,
        },
    };
    let Some(min) = min else {
        rule.add_custom(
            rule.title(),
            format!("Install missing package {name}"),
            vec![Occurrence {
                file: rule.file(),
                resolution: rule.resolution(),
                position: None,
            }],
            findings,
        );
        return Ok(());
    };
    if semver::satisfies(&min.version, &range) {
        return Ok(());
    }
    let pos = project.position(pkg, &format!("{section}.{name}"))?;
    rule.add_custom(
        rule.title(),
        format!("Install supported version of the {name} package"),
        vec![Occurrence {
            file: rule.file(),
            resolution: rule.resolution(),
            position: Some(pos),
        }],
        findings,
    );
    Ok(())
}

/// `DependencyRule#needsUpdate`.
fn needs_update(rule_version: &str, current: &str) -> bool {
    (|| -> Result<bool, String> {
        let rule_min = semver::min_version(rule_version)?;
        let current_min = semver::min_version(current)?;
        if semver::intersects(rule_version, current)? {
            return Ok(matches!((&rule_min, &current_min), (Some(r), Some(c)) if semver::gt(r, c)));
        }
        let current_max = max_version(current)?;
        Ok(matches!((&rule_min, &current_max), (Some(r), Some(c)) if semver::gt(r, c)))
    })()
    .unwrap_or(false)
}

/// `DependencyRule#getMaxVersion`.
fn max_version(range: &str) -> Result<Option<semver::SemVer>, String> {
    let r = semver::Range::new(range)?;
    if semver::valid(range).is_some() {
        return Ok(semver::SemVer::parse(range));
    }
    let mut max: Option<semver::SemVer> = None;
    for set in &r.set {
        for c in set {
            if (c.operator == "<" || c.operator == "<=")
                && let Some(v) = &c.semver
                && max.as_ref().is_none_or(|m| semver::gt(v, m))
            {
                max = Some(v.clone());
            }
        }
    }
    match max {
        Some(m) => Ok(Some(m)),
        None => semver::min_version(range),
    }
}

/// `DependencyRule.visit`.
pub fn dependency_visit(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let Some(pkg) = &project.package_json else {
        return Ok(());
    };
    let (is_override, is_dev, add) = (rule.fb("isOverride"), rule.fb("isDevDep"), rule.fb("add"));
    let name = rule.fs("packageName");
    let section = if is_override {
        "overrides"
    } else if is_dev {
        "devDependencies"
    } else {
        "dependencies"
    };
    let deps = pkg.get(section);
    let version = match deps {
        Some(d) if truthy(d) => d.get(&name).cloned(),
        _ => Some(Value::from("")),
    };
    let version = version.filter(truthy);
    if add {
        match version {
            Some(v) => {
                if needs_update(&rule.fs("packageVersion"), &to_js_string(&v)) {
                    let pos = project.position(pkg, &format!("{section}.{name}"))?;
                    rule.add_at(findings, pos);
                }
            }
            None => {
                if !rule.fb("isOptional") || custom_condition(rule, project) {
                    let pos = project.position(pkg, section)?;
                    rule.add_custom(
                        Some(name.clone()),
                        rule.description().replacen("Upgrade", "Install", 1),
                        vec![Occurrence {
                            file: rule.file(),
                            resolution: rule.resolution(),
                            position: Some(pos),
                        }],
                        findings,
                    );
                }
            }
        }
    } else if version.is_some() {
        let pos = project.position(pkg, &format!("{section}.{name}"))?;
        rule.add_at(findings, pos);
    }
    Ok(())
}

/// `FileAddRemoveRule.visit`.
fn file_add_remove(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let target = std::path::Path::new(&project.path).join(rule.fs("filePath"));
    let add = rule.fb("add");
    let exists = target.exists();
    if (!add && exists) || (add && !exists) {
        rule.add(findings);
        return Ok(());
    }
    if add && rule.fb("contents") {
        let content = std::fs::read_to_string(&target)
            .map_err(|e| Error::js(util::node_fs_error(&e, "open", &target.to_string_lossy())))?;
        if content != rule.fs("contents") {
            rule.add(findings);
        }
    }
    Ok(())
}

/// `PackageRule.visit`.
fn package(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let Some(pkg) = &project.package_json else {
        return Ok(());
    };
    let name = rule.fs("propertyName");
    let has = pkg.get(&name).is_some_and(truthy);
    let add = rule.fb("add");
    if (add && !has) || (!add && has) {
        let pos = project.position(pkg, &name)?;
        rule.add_at(findings, pos);
    }
    Ok(())
}

/// `ResolutionRule.visit`.
fn resolution(rule: &Rule, project: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let Some(pkg) = &project.package_json else {
        return Ok(());
    };
    let name = rule.fs("packageName");
    let current = pkg
        .get("resolutions")
        .filter(|r| truthy(r))
        .and_then(|r| r.get(&name));
    if custom_condition(rule, project)
        && current.is_none_or(|c| {
            c.as_str() != Some(rule.fs("packageVersion").as_str())
                || rule.f("packageVersion").is_none()
        })
    {
        let pos = project.position(pkg, &format!("resolutions.{name}"))?;
        rule.add_at(findings, pos);
    }
    Ok(())
}

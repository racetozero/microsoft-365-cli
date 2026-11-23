//! Rule classes with their own `visit` or `customCondition`, by class name.
//! Each arm is a port of `project-upgrade/rules/<Class>.ts` or
//! `project-doctor/rules/<Class>.ts`.

use crate::project::{Project, SourceFile};
use crate::rules::{CATALOG, Finding, Occurrence, Rule};
use crate::semver;
use crate::tslex::{self, Kind, Token};
use m365_core::prelude::*;
use std::path::Path;
use std::sync::LazyLock;

/// JavaScript `a === b` for JSON values that may be `undefined`.
fn js_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(Value::Number(x)), Some(Value::Number(y))) => x.as_f64() == y.as_f64(),
        (Some(x @ (Value::String(_) | Value::Bool(_) | Value::Null)), Some(y)) => x == y,
        _ => false,
    }
}

/// `v?.a?.b` (null and undefined both stop the chain).
fn at<'a>(v: Option<&'a Value>, path: &[&str]) -> Option<&'a Value> {
    let mut cur = v;
    for k in path {
        cur = cur.filter(|c| !c.is_null()).and_then(|c| c.get(*k));
    }
    cur
}

fn t(v: Option<&Value>) -> bool {
    v.is_some_and(truthy)
}

/// `Object.keys(o)` (`[]` for non-objects).
fn keys(v: Option<&Value>) -> Vec<String> {
    match v {
        Some(Value::Object(o)) => o.keys().cloned().collect(),
        Some(Value::Array(a)) => (0..a.len()).map(|i| i.to_string()).collect(),
        Some(Value::String(s)) => (0..s.encode_utf16().count())
            .map(|i| i.to_string())
            .collect(),
        _ => Vec::new(),
    }
}

/// `x.indexOf(needle)` for arrays (strict equality) and strings.
fn index_of(v: &Value, needle: &str) -> i64 {
    match v {
        Value::Array(a) => a
            .iter()
            .position(|x| x.as_str() == Some(needle))
            .map_or(-1, |i| i as i64),
        Value::String(s) => s
            .find(needle)
            .map_or(-1, |i| s[..i].encode_utf16().count() as i64),
        _ => -1,
    }
}

fn pretty(v: &Value) -> String {
    json::stringify_pretty(v, "  ")
}

/// `spfx.isReactProject`.
fn is_react(p: &Project) -> bool {
    let g = at(p.yo_rc_json.as_ref(), &["@microsoft/generator-sharepoint"]);
    (g.is_some()
        && (at(g, &["framework"]).and_then(Value::as_str) == Some("react")
            || at(g, &["template"]).and_then(Value::as_str) == Some("react")))
        || at(p.package_json.as_ref(), &["dependencies", "react"]).is_some()
}

/// `spfx.isKnockoutProject`.
fn is_knockout(p: &Project) -> bool {
    let g = at(p.yo_rc_json.as_ref(), &["@microsoft/generator-sharepoint"]);
    (g.is_some() && at(g, &["framework"]).and_then(Value::as_str) == Some("knockout"))
        || at(p.package_json.as_ref(), &["dependencies", "knockout"]).is_some()
}

pub fn condition(rule: &Rule, p: &Project) -> bool {
    match rule.class.trim_end_matches("@doctor") {
        "FN001019_DEP_knockout" | "FN001020_DEP_types_knockout" => is_knockout(p),
        "FN001021_DEP_microsoft_sp_property_pane" => t(at(
            p.package_json.as_ref(),
            &["dependencies", "@microsoft/sp-webpart-base"],
        )),
        _ => is_react(p),
    }
}

/// `FN017001_MISC_npm_dedupe.visit`: suggested when any occurrence installs
/// or uninstalls a package.
pub fn npm_dedupe(rule: &Rule, findings: &mut Vec<Finding>) {
    if findings.iter().any(|f| {
        f.occurrences
            .iter()
            .any(|o| o.resolution.starts_with("install") || o.resolution.starts_with("uninstall"))
    }) {
        rule.add(findings);
    }
}

/// Which project JSON file a table rule reads.
#[derive(Clone, Copy)]
enum F {
    Config,
    CopyAssets,
    DeployAzure,
    PackageSolution,
    Serve,
    TsLint,
    WriteManifests,
    YoRc,
    TsConfig,
    TsLintRoot,
    Sass,
}

fn file(p: &Project, f: F) -> Option<&Value> {
    match f {
        F::Config => p.config_json.as_ref(),
        F::CopyAssets => p.copy_assets_json.as_ref(),
        F::DeployAzure => p.deploy_azure_storage_json.as_ref(),
        F::PackageSolution => p.package_solution_json.as_ref(),
        F::Serve => p.serve_json.as_ref(),
        F::TsLint => p.ts_lint_json.as_ref(),
        F::WriteManifests => p.write_manifests_json.as_ref(),
        F::YoRc => p.yo_rc_json.as_ref(),
        F::TsConfig => p.ts_config_json.as_ref(),
        F::TsLintRoot => p.ts_lint_json_root.as_ref(),
        F::Sass => p.sass_json.as_ref(),
    }
}

const GEN: &str = "@microsoft/generator-sharepoint";

/// `if (!file || !file.<guard>) return; if (file.<value> !== this.<field>)
/// addFindingWithPosition(<ast>)`.
struct Neq {
    class: &'static str,
    file: F,
    guard: &'static [&'static str],
    value: &'static [&'static str],
    field: &'static str,
    ast: &'static str,
}

const NEQ: &[Neq] = &[
    Neq {
        class: "FN003001_CFG_schema",
        file: F::Config,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN003002_CFG_version",
        file: F::Config,
        guard: &[],
        value: &["version"],
        field: "version",
        ast: "version",
    },
    Neq {
        class: "FN004001_CFG_CA_schema",
        file: F::CopyAssets,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN004002_CFG_CA_deployCdnPath",
        file: F::CopyAssets,
        guard: &[],
        value: &["deployCdnPath"],
        field: "deployCdnPath",
        ast: "deployCdnPath",
    },
    Neq {
        class: "FN005001_CFG_DAS_schema",
        file: F::DeployAzure,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN005002_CFG_DAS_workingDir",
        file: F::DeployAzure,
        guard: &[],
        value: &["workingDir"],
        field: "workingDir",
        ast: "workingDir",
    },
    Neq {
        class: "FN006001_CFG_PS_schema",
        file: F::PackageSolution,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN006002_CFG_PS_includeClientSideAssets",
        file: F::PackageSolution,
        guard: &["solution"],
        value: &["solution", "includeClientSideAssets"],
        field: "includeClientSideAssets",
        ast: "solution.includeClientSideAssets",
    },
    Neq {
        class: "FN006003_CFG_PS_isDomainIsolated",
        file: F::PackageSolution,
        guard: &["solution"],
        value: &["solution", "isDomainIsolated"],
        field: "isDomainIsolated",
        ast: "solution.isDomainIsolated",
    },
    Neq {
        class: "FN007001_CFG_S_schema",
        file: F::Serve,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN007002_CFG_S_initialPage",
        file: F::Serve,
        guard: &[],
        value: &["initialPage"],
        field: "initialPage",
        ast: "initialPage",
    },
    Neq {
        class: "FN008001_CFG_TSL_schema",
        file: F::TsLint,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN009001_CFG_WM_schema",
        file: F::WriteManifests,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN010001_YORC_version",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "version"],
        field: "version",
        ast: "@microsoft/generator-sharepoint.version",
    },
    Neq {
        class: "FN010002_YORC_isCreatingSolution",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "isCreatingSolution"],
        field: "value",
        ast: "@microsoft/generator-sharepoint.isCreatingSolution",
    },
    Neq {
        class: "FN010003_YORC_packageManager",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "packageManager"],
        field: "packageManager",
        ast: "@microsoft/generator-sharepoint.packageManager",
    },
    Neq {
        class: "FN010005_YORC_environment",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "environment"],
        field: "environment",
        ast: "@microsoft/generator-sharepoint.environment",
    },
    Neq {
        class: "FN010007_YORC_isDomainIsolated",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "isDomainIsolated"],
        field: "value",
        ast: "@microsoft/generator-sharepoint.isDomainIsolated",
    },
    Neq {
        class: "FN010011_YORC_useGulp",
        file: F::YoRc,
        guard: &[],
        value: &[GEN, "useGulp"],
        field: "useGulp",
        ast: "@microsoft/generator-sharepoint.useGulp",
    },
    Neq {
        class: "FN012001_TSC_module",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "module"],
        field: "module",
        ast: "compilerOptions.module",
    },
    Neq {
        class: "FN012002_TSC_moduleResolution",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "moduleResolution"],
        field: "moduleResolution",
        ast: "compilerOptions.moduleResolution",
    },
    Neq {
        class: "FN012003_TSC_skipLibCheck",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "skipLibCheck"],
        field: "skipLibCheck",
        ast: "compilerOptions.skipLibCheck",
    },
    Neq {
        class: "FN012011_TSC_outDir",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "outDir"],
        field: "outDir",
        ast: "compilerOptions.outDir",
    },
    Neq {
        class: "FN012014_TSC_inlineSources",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "inlineSources"],
        field: "inlineSources",
        ast: "compilerOptions.inlineSources",
    },
    Neq {
        class: "FN012015_TSC_strictNullChecks",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "strictNullChecks"],
        field: "strictNullChecks",
        ast: "compilerOptions.strictNullChecks",
    },
    Neq {
        class: "FN012016_TSC_noUnusedLocals",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "noUnusedLocals"],
        field: "noUnusedLocals",
        ast: "compilerOptions.noUnusedLocals",
    },
    Neq {
        class: "FN012020_TSC_noImplicitAny",
        file: F::TsConfig,
        guard: &["compilerOptions"],
        value: &["compilerOptions", "noImplicitAny"],
        field: "noImplicitAny",
        ast: "compilerOptions.noImplicitAny",
    },
    Neq {
        class: "FN019002_TSL_extends",
        file: F::TsLintRoot,
        guard: &[],
        value: &["extends"],
        field: "_extends",
        ast: "extends",
    },
    Neq {
        class: "FN026001_CFG_SASS_schema",
        file: F::Sass,
        guard: &[],
        value: &["$schema"],
        field: "schema",
        ast: "$schema",
    },
    Neq {
        class: "FN026002_CFG_SASS_extends",
        file: F::Sass,
        guard: &[],
        value: &["extends"],
        field: "_extends",
        ast: "extends",
    },
];

/// `if (!file.<guard>... || <list>.indexOf(<item>) < 0)` rules on tsconfig
/// `compilerOptions.<list>`.
const TS_LIST: &[(&str, &str, &str)] = &[
    (
        "FN012004_TSC_typeRoots_types",
        "typeRoots",
        "./node_modules/@types",
    ),
    (
        "FN012005_TSC_typeRoots_microsoft",
        "typeRoots",
        "./node_modules/@microsoft",
    ),
    ("FN012007_TSC_lib_es5", "lib", "es5"),
    ("FN012008_TSC_lib_dom", "lib", "dom"),
    (
        "FN012009_TSC_lib_es2015_collection",
        "lib",
        "es2015.collection",
    ),
    ("FN012018_TSC_lib_es2015_promise", "lib", "es2015.promise"),
];

/// `.gitignore` must contain the line.
const GITIGNORE: &[(&str, &str)] = &[
    ("FN023001_GITIGNORE_release", "release"),
    ("FN023002_GITIGNORE_heft", ".heft"),
    ("FN023003_GITIGNORE_libdts", "lib-dts"),
    ("FN023004_GITIGNORE_libcommonjs", "lib-commonjs"),
    ("FN023005_GITIGNORE_libesm", "lib-esm"),
    ("FN023006_GITIGNORE_jestoutput", "jest-output"),
];

/// `package.json` `scripts.<name>` must equal `this.script`.
const SCRIPTS: &[(&str, &str)] = &[
    ("FN021004_PKG_scripts_build", "build"),
    ("FN021006_PKG_scripts_clean", "clean"),
    ("FN021007_PKG_scripts_start", "start"),
    ("FN021008_PKG_scripts_eject_webpack", "eject-webpack"),
];

const SPFX_DEPS: [&str; 18] = [
    "@microsoft/sp-core-library",
    "@microsoft/sp-lodash-subset",
    "@microsoft/sp-office-ui-fabric-core",
    "@microsoft/sp-webpart-base",
    "@microsoft/sp-dialog",
    "@microsoft/sp-application-base",
    "@microsoft/decorators",
    "@microsoft/sp-listview-extensibility",
    "@microsoft/sp-property-pane",
    "@microsoft/sp-component-base",
    "@microsoft/sp-diagnostics",
    "@microsoft/sp-dynamic-data",
    "@microsoft/sp-extension-base",
    "@microsoft/sp-http",
    "@microsoft/sp-list-subscription",
    "@microsoft/sp-loader",
    "@microsoft/sp-odata-types",
    "@microsoft/sp-page-context",
];
const SPFX_DEV_DEPS: [&str; 5] = [
    "@microsoft/eslint-config-spfx",
    "@microsoft/eslint-plugin-spfx",
    "@microsoft/sp-build-web",
    "@microsoft/sp-module-interfaces",
    "@microsoft/sp-tslint-rules",
];

pub fn visit(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let class = rule.class.trim_end_matches("@doctor");
    if let Some(n) = NEQ.iter().find(|n| n.class == class) {
        let Some(f) = file(p, n.file) else {
            return Ok(());
        };
        if !n.guard.is_empty() && !t(at(Some(f), n.guard)) {
            return Ok(());
        }
        if !js_eq(at(Some(f), n.value), rule.f(n.field)) {
            rule.add_at(findings, p.position(f, n.ast)?);
        }
        return Ok(());
    }
    if let Some((_, list, item)) = TS_LIST.iter().find(|(c, _, _)| *c == class) {
        let Some(f) = p
            .ts_config_json
            .as_ref()
            .filter(|f| t(f.get("compilerOptions")))
        else {
            return Ok(());
        };
        let l = &f["compilerOptions"].get(*list);
        if !t(*l) || index_of(l.expect("list"), item) < 0 {
            rule.add_at(findings, p.position(f, &format!("compilerOptions.{list}"))?);
        }
        return Ok(());
    }
    if let Some((_, line)) = GITIGNORE.iter().find(|(c, _)| *c == class) {
        if let Some(g) = &p.gitignore
            && !g
                .split('\n')
                .any(|l| l.strip_suffix('\r').unwrap_or(l) == *line || l == *line)
        {
            rule.add(findings);
        }
        return Ok(());
    }
    if let Some((_, name)) = SCRIPTS.iter().find(|(c, _)| *c == class) {
        let Some(pkg) = &p.package_json else {
            return Ok(());
        };
        let scripts = pkg.get("scripts");
        let v = scripts.filter(|s| s.is_object()).and_then(|s| s.get(*name));
        if !t(scripts)
            || !scripts.is_some_and(|s| s.is_object() || s.is_array())
            || !t(v)
            || !js_eq(v, rule.f("script"))
        {
            rule.add_at(findings, p.position(pkg, &format!("scripts.{name}"))?);
        }
        return Ok(());
    }
    match class {
        "FN002019_DEVDEP_microsoft_rush_stack_compiler" => rush_stack_compiler(rule, p, findings),
        "FN003003_CFG_bundles" => {
            let Some(c) = &p.config_json else {
                return Ok(());
            };
            if c.get("bundles").is_some() {
                return Ok(());
            }
            let Some(entries) = c.get("entries").filter(|e| truthy(e)) else {
                return Ok(());
            };
            let mut bundles = Obj::new();
            for e in entries.as_array().map(Vec::as_slice).unwrap_or(&[]) {
                let entry = e
                    .get("entry")
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into());
                bundles.insert(bundle_name(&entry), obj! { "components": vec![obj! { "entrypoint": ?e.get("entry").cloned(), "manifest": ?e.get("manifest").cloned() }] });
            }
            let pos = p.position(c, "bundles")?;
            rule.add_occurrences(
                vec![occ(
                    rule.file(),
                    pretty(&obj! { "bundles": Value::Object(bundles) }),
                    Some(pos),
                )],
                findings,
            );
            Ok(())
        }
        "FN003004_CFG_entries" => {
            let Some(c) = &p.config_json else {
                return Ok(());
            };
            if let Some(e) = c.get("entries") {
                let pos = p.position(c, "entries")?;
                rule.add_occurrences(
                    vec![occ(
                        rule.file(),
                        pretty(&obj! { "entries": e.clone() }),
                        Some(pos),
                    )],
                    findings,
                );
            }
            Ok(())
        }
        "FN003005_CFG_localizedResource_pathLib" => {
            let Some(c) = p
                .config_json
                .as_ref()
                .filter(|c| t(c.get("localizedResources")))
            else {
                return Ok(());
            };
            let mut occurrences = Vec::new();
            for k in keys(c.get("localizedResources")) {
                let path = c["localizedResources"]
                    .get(&k)
                    .map(to_js_string)
                    .unwrap_or_default();
                if !path.starts_with("lib/") {
                    let mut res = Obj::new();
                    res.insert(k.clone(), Value::from(format!("lib/{path}")));
                    let pos = p.position(c, &format!("localizedResources.{k}"))?;
                    occurrences.push(occ(
                        rule.file(),
                        pretty(&obj! { "localizedResources": Value::Object(res) }),
                        Some(pos),
                    ));
                }
            }
            if !occurrences.is_empty() {
                rule.add_occurrences(occurrences, findings);
            }
            Ok(())
        }
        "FN006004_CFG_PS_developer" => {
            let Some(ps) = p
                .package_solution_json
                .as_ref()
                .filter(|ps| t(ps.get("solution")))
            else {
                return Ok(());
            };
            if !t(ps["solution"].get("developer")) {
                rule.add_at(findings, p.position(ps, "solution")?);
            }
            Ok(())
        }
        "FN006005_CFG_PS_metadata" => {
            let Some(ps) = p
                .package_solution_json
                .as_ref()
                .filter(|ps| t(ps.get("solution")))
            else {
                return Ok(());
            };
            if !t(ps["solution"].get("metadata")) {
                let d = format!(
                    "{} description",
                    jsu(at(p.package_json.as_ref(), &["name"]))
                );
                let res = obj! { "solution": obj! { "metadata": obj! {
                    "shortDescription": obj! { "default": d.as_str() }, "longDescription": obj! { "default": d.as_str() },
                    "screenshotPaths": Value::Array(vec![]), "videoUrl": "", "categories": Value::Array(vec![]),
                } } };
                let pos = p.position(ps, "solution")?;
                rule.add_occurrences(vec![occ(rule.file(), pretty(&res), Some(pos))], findings);
            }
            Ok(())
        }
        "FN006006_CFG_PS_features" => {
            let Some(ps) = p
                .package_solution_json
                .as_ref()
                .filter(|ps| t(ps.get("solution")))
            else {
                return Ok(());
            };
            let features = ps["solution"].get("features");
            if (t(features) && length(features.expect("features")) > 0.0) || p.manifests.is_empty()
            {
                return Ok(());
            }
            let name = jsu(at(p.package_json.as_ref(), &["name"]));
            let mut occurrences = Vec::new();
            for m in &p.manifests {
                let alias = jsu(m.get("alias"));
                let res = obj! { "solution": obj! { "features": vec![obj! {
                    "title": format!("{name} {alias} Feature"),
                    "description": format!("The feature that activates {alias} from the {name} solution."),
                    "id": ?m.get("id").cloned(),
                    "version": ?ps["solution"].get("version").cloned(),
                    "componentIds": vec![m.get("id").cloned().unwrap_or(Value::Null)],
                }] } };
                let pos = p.position(ps, "solution")?;
                occurrences.push(occ(rule.file(), pretty(&res), Some(pos)));
            }
            rule.add_occurrences(occurrences, findings);
            Ok(())
        }
        "FN007003_CFG_S_api" => {
            if let Some(s) = &p.serve_json
                && t(s.get("api"))
            {
                rule.add_at(findings, p.position(s, "api")?);
            }
            Ok(())
        }
        "FN008002_CFG_TSL_removeRule" => {
            let Some(l) = p
                .ts_lint_json
                .as_ref()
                .filter(|l| t(at(Some(l), &["lintConfig", "rules"])) && t(l.get("lintConfig")))
            else {
                return Ok(());
            };
            let r = rule.fs("rule");
            if l["lintConfig"]["rules"].get(&r).is_some() {
                rule.add_at(findings, p.position(l, &format!("lintConfig.rules.{r}"))?);
            }
            Ok(())
        }
        "FN008003_CFG_TSL_preferConst" => {
            if let Some(l) = &p.ts_lint_json
                && at(Some(l), &["lintConfig", "rules", "prefer-const"]) == Some(&Value::Bool(true))
            {
                rule.add_at(findings, p.position(l, "lintConfig.rules.prefer-const")?);
            }
            Ok(())
        }
        "FN010004_YORC_componentType" => {
            let Some(y) = &p.yo_rc_json else {
                return Ok(());
            };
            let mut ct = None;
            for m in &p.manifests {
                match m.get("componentType").and_then(Value::as_str) {
                    Some("WebPart") => {
                        ct = Some("webpart");
                        break;
                    }
                    Some("Extension") => {
                        ct = Some("extension");
                        break;
                    }
                    _ => {}
                }
            }
            let ct = ct.unwrap_or("webpart");
            if at(Some(y), &[GEN, "componentType"]).and_then(Value::as_str) != Some(ct) {
                let pos = p.position(y, "@microsoft/generator-sharepoint.componentType")?;
                rule.add_occurrences(vec![occ(rule.file(), pretty(&obj! { "@microsoft/generator-sharepoint": obj! { "componentType": ct } }), Some(pos))], findings);
            }
            Ok(())
        }
        "FN010006_YORC_framework" => {
            let Some(y) = &p.yo_rc_json else {
                return Ok(());
            };
            let fw = at(Some(y), &[GEN, "framework"]);
            if (rule.fb("add") && !js_eq(fw, rule.f("framework"))) || (!rule.fb("add") && t(fw)) {
                rule.add_at(
                    findings,
                    p.position(y, "@microsoft/generator-sharepoint.framework")?,
                );
            }
            Ok(())
        }
        "FN010008_YORC_nodeVersion" => {
            let Some(y) = &p.yo_rc_json else {
                return Ok(());
            };
            let node = node_version();
            if at(Some(y), &[GEN, "nodeVersion"]).and_then(Value::as_str) != Some(node.as_str()) {
                // the resolution getter reads `process.version` too, so it is
                // built here rather than taken from the extracted data
                let res = format!(
                    "{{\n  \"@microsoft/generator-sharepoint\": {{\n    \"nodeVersion\": \"{node}\"\n  }}\n}}"
                );
                let pos = p.position(y, "@microsoft/generator-sharepoint.nodeVersion")?;
                rule.add_occurrences(vec![occ(rule.file(), res, Some(pos))], findings);
            }
            Ok(())
        }
        "FN010009_YORC_sdkVersions_microsoft_graph_client" => {
            sdk_version(rule, p, findings, "@microsoft/microsoft-graph-client")
        }
        "FN010010_YORC_sdkVersions_teams_js" => {
            sdk_version(rule, p, findings, "@microsoft/teams-js")
        }
        c if c.starts_with("FN011") => manifest_rule(rule, p, findings),
        "FN012006_TSC_types_es6_collections" => ts_types(rule, p, findings, "es6-collections"),
        "FN012019_TSC_types_es6_promise" => ts_types(rule, p, findings, "es6-promise"),
        "FN012010_TSC_experimentalDecorators" => {
            let Some(f) = p
                .ts_config_json
                .as_ref()
                .filter(|f| t(f.get("compilerOptions")))
            else {
                return Ok(());
            };
            if !t(f["compilerOptions"].get("experimentalDecorators")) {
                rule.add_at(
                    findings,
                    p.position(f, "compilerOptions.experimentalDecorators")?,
                );
            }
            Ok(())
        }
        "FN012012_TSC_include" | "FN012013_TSC_exclude" => {
            let Some(f) = &p.ts_config_json else {
                return Ok(());
            };
            let key = if class == "FN012012_TSC_include" {
                "include"
            } else {
                "exclude"
            };
            let add = class == "FN012012_TSC_include" || rule.fb("add");
            let have = f.get(key);
            if add {
                let wanted = rule
                    .f(key)
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default();
                let missing = t(have)
                    && wanted
                        .iter()
                        .any(|w| index_of(have.expect("list"), &to_js_string(w)) < 0);
                if !t(have) || missing {
                    rule.add_at(findings, p.position(f, key)?);
                }
            } else if t(have) {
                rule.add_at(findings, p.position(f, key)?);
            }
            Ok(())
        }
        "FN012017_TSC_extends" => {
            let Some(f) = &p.ts_config_json else {
                return Ok(());
            };
            if !t(f.get("extends")) || !js_eq(f.get("extends"), rule.f("_extends")) {
                rule.add_at(findings, p.position(f, "extends")?);
            }
            Ok(())
        }
        "FN012021_TSC_excludeDirectories" => {
            let Some(f) = &p.ts_config_json else {
                return Ok(());
            };
            let d = at(Some(f), &["watchOptions", "excludeDirectories"]);
            if !t(d)
                || stringify(d.expect("dirs"))
                    != rule.f("directories").map(stringify).unwrap_or_default()
            {
                rule.add_at(findings, p.position(f, "watchOptions.excludeDirectories")?);
            }
            Ok(())
        }
        "FN013001_GULP_msGridSassSuppression" => {
            if let Some(g) = &p.gulpfile_js
                && !g.contains(&rule.resolution())
            {
                rule.add(findings);
            }
            Ok(())
        }
        "FN013002_GULP_serveTask" => {
            if let Some(g) = &p.gulpfile_js
                && !g.contains("result.set('serve', result.get('serve-deprecated'));")
            {
                rule.add(findings);
            }
            Ok(())
        }
        c if c.starts_with("FN014") => vscode_rule(rule, p, findings),
        "FN015008_FILE_eslintrc_js" => eslintrc_file(rule, p, findings),
        c if c.starts_with("FN016") || c.starts_with("FN025") => ts_rule(rule, p, findings),
        c if c.starts_with("FN018") => teams_rule(rule, p, findings),
        "FN019001_TSL_rulesDirectory" => {
            if let Some(l) = &p.ts_lint_json_root
                && t(l.get("rulesDirectory"))
            {
                rule.add_at(findings, p.position(l, "rulesDirectory")?);
            }
            Ok(())
        }
        c if c.starts_with("FN021") => package_rule(rule, p, findings),
        "FN022001_SCSS_remove_fabric_react" | "FN022002_SCSS_add_fabric_react" => {
            if !is_react(p) || p.scss_files.is_empty() {
                return Ok(());
            }
            let import = rule.fs("importValue");
            let mut occurrences = Vec::new();
            for f in &p.scss_files {
                let src = f.source.as_deref().ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'indexOf')")
                })?;
                let hit = if class == "FN022001_SCSS_remove_fabric_react" {
                    src.contains(&import)
                } else {
                    !src.contains(&import)
                        && (!rule.fb("addIfContains") || src.contains(&rule.fs("addIfContains")))
                };
                if hit {
                    occurrences.push(occ(p.relative(&f.path), rule.resolution(), None));
                }
            }
            if !occurrences.is_empty() {
                rule.add_occurrences(occurrences, findings);
            }
            Ok(())
        }
        "FN024001_NPMIGNORE_file" => {
            if p.npmignore.is_none() {
                rule.add(findings);
            }
            Ok(())
        }
        "FN027001_OVERRIDES_rushstack_heft"
        | "FN027002_OVERRIDES_qs"
        | "FN027003_OVERRIDES_uuid" => {
            let name = rule.fs("packageName");
            if let Some(pkg) = &p.package_json
                && let Some(v) = at(Some(pkg), &["overrides", &name]).filter(|v| truthy(v))
                && !js_eq(Some(v), rule.f("packageVersion"))
            {
                let pos = p.position(pkg, &format!("overrides.{name}"))?;
                findings.push(remove_override(rule, &name, pos));
            }
            crate::rules::dependency_visit(rule, p, findings)
        }
        "FN027004_OVERRIDES_serialize_javascript" => {
            let name = rule.fs("packageName");
            let mut updates = Vec::new();
            crate::rules::dependency_visit(rule, p, &mut updates)?;
            if let Some(pkg) = &p.package_json
                && t(at(Some(pkg), &["overrides", &name]))
                && !updates.is_empty()
            {
                let pos = p.position(pkg, &format!("overrides.{name}"))?;
                findings.push(remove_override(rule, &name, pos));
            }
            findings.extend(updates);
            Ok(())
        }
        _ => Err(Error::msg(format!("spfx rule {class} is not ported"))),
    }
}

fn remove_override(rule: &Rule, name: &str, pos: (usize, usize)) -> Finding {
    Finding {
        id: format!("{}_REMOVE", rule.id()),
        title: Some(name.to_string()),
        description: format!(
            "Remove existing SharePoint Framework override dependency package {name}"
        ),
        occurrences: vec![occ(
            rule.file(),
            format!("removeOverride overrides.{name}"),
            Some(pos),
        )],
        resolution_type: "cmd".into(),
        severity: "Required".into(),
        supersedes: vec![],
    }
}

fn occ(file: String, resolution: String, position: Option<(usize, usize)>) -> Occurrence {
    Occurrence {
        file,
        resolution,
        position,
    }
}

fn jsu(v: Option<&Value>) -> String {
    v.map(to_js_string).unwrap_or_else(|| "undefined".into())
}

/// `x.length` of an array or string.
fn length(v: &Value) -> f64 {
    match v {
        Value::Array(a) => a.len() as f64,
        Value::String(s) => s.encode_utf16().count() as f64,
        _ => f64::NAN,
    }
}

/// `FN003003.tryGetBundleName`.
fn bundle_name(entry: &str) -> String {
    let name = entry
        .rsplit('/')
        .next()
        .unwrap_or(entry)
        .replacen(".js", "", 1);
    // `/([a-z](?=[A-Z]))/g` -> '$1-': a dash between a lower and an upper
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        out.push(*c);
        if c.is_ascii_lowercase() && chars.get(i + 1).is_some_and(char::is_ascii_uppercase) {
            out.push('-');
        }
    }
    out.to_lowercase()
}

static NODE_VERSION: LazyLock<String> = LazyLock::new(|| {
    std::process::Command::new("node")
        .arg("--version")
        .output()
        .ok()
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .trim()
                .trim_start_matches('v')
                .to_string()
        })
        .unwrap_or_default()
});

/// `process.version.substring(1)`: the Node.js on PATH.
pub fn node_version_public() -> String {
    node_version()
}

fn node_version() -> String {
    NODE_VERSION.clone()
}

fn sdk_version(rule: &Rule, p: &Project, findings: &mut Vec<Finding>, pkg: &str) -> Result<()> {
    let Some(y) = &p.yo_rc_json else {
        return Ok(());
    };
    if !js_eq(at(Some(y), &[GEN, "sdkVersions", pkg]), rule.f("version")) {
        let mut path = GEN.to_string();
        if t(at(Some(y), &[GEN, "sdkVersions"])) {
            path.push_str(".sdkVersions");
            if t(at(Some(y), &[GEN, "sdkVersions", pkg])) {
                path.push('.');
                path.push_str(pkg);
            }
        }
        rule.add_at(findings, p.position(y, &path)?);
    }
    Ok(())
}

fn ts_types(rule: &Rule, p: &Project, findings: &mut Vec<Finding>, item: &str) -> Result<()> {
    let Some(f) = p
        .ts_config_json
        .as_ref()
        .filter(|f| t(f.get("compilerOptions")))
    else {
        return Ok(());
    };
    let types = f["compilerOptions"].get("types");
    if rule.fb("add") {
        if !t(types) || index_of(types.expect("types"), item) < 0 {
            rule.add_at(findings, p.position(f, "compilerOptions.types")?);
        }
    } else if t(types) && index_of(types.expect("types"), item) > -1 {
        rule.add_at(
            findings,
            p.position(f, &format!("compilerOptions.types[{item}]"))?,
        );
    }
    Ok(())
}

fn rush_stack_compiler(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let Some(pkg) = &p.package_json else {
        return Ok(());
    };
    let mut deps = Vec::new();
    if t(pkg.get("dependencies")) {
        deps.extend(keys(pkg.get("dependencies")));
    }
    let mut dev = Vec::new();
    if t(pkg.get("devDependencies")) {
        dev = keys(pkg.get("devDependencies"));
        deps.extend(dev.iter().cloned());
    }
    let supported: Vec<String> = rule
        .f("supportedVersions")
        .and_then(Value::as_array)
        .map(|a| a.iter().map(to_js_string).collect())
        .unwrap_or_default();
    let mut occurrences = Vec::new();
    for rs in deps
        .iter()
        .filter(|d| d.starts_with("@microsoft/rush-stack-compiler"))
    {
        let version = rs.replacen("@microsoft/rush-stack-compiler-", "", 1);
        if supported.contains(&version) {
            continue;
        }
        let section = if dev.contains(rs) {
            "devDependencies"
        } else {
            "dependencies"
        };
        let pos = p.position(pkg, &format!("{section}.{rs}"))?;
        occurrences.push(occ(rule.file(), format!("uninstallDev {rs}"), Some(pos)));
    }
    if occurrences.is_empty() {
        return Ok(());
    }
    let d = format!(
        "Uninstall unsupported version{} of @microsoft/rush-stack-compiler",
        if occurrences.len() == 1 { "" } else { "s" }
    );
    rule.add_custom(rule.title(), d, occurrences, findings);
    Ok(())
}

fn manifest_rule(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    if p.manifests.is_empty() {
        return Ok(());
    }
    let class = rule.class.trim_end_matches("@doctor");
    let mut occurrences = Vec::new();
    let s = |m: &Value, k: &str| m.get(k).and_then(Value::as_str).map(String::from);
    for (mi, m) in p.manifests.iter().enumerate() {
        let path = p.relative(&jsu(m.get("path")));
        // `manifest.commands`, as changed by earlier rules
        let commands = p
            .commands
            .borrow()
            .get(&mi)
            .cloned()
            .or_else(|| m.get("commands").cloned());
        let ct = s(m, "componentType");
        let et = s(m, "extensionType");
        let web_part = ct.as_deref() == Some("WebPart");
        let ext = |t: &str| ct.as_deref() == Some("Extension") && et.as_deref() == Some(t);
        let schema_differs = || !js_eq(m.get("$schema"), rule.f("schema"));
        let mut add = |prop: &str, res: String| -> Result<()> {
            let pos = p.position(m, prop)?;
            occurrences.push(occ(path.clone(), res, Some(pos)));
            Ok(())
        };
        match class {
            "FN011001_MAN_webpart_schema" if web_part && schema_differs() => {
                add("$schema", rule.resolution())?
            }
            "FN011002_MAN_applicationCustomizer_schema"
                if ext("ApplicationCustomizer") && schema_differs() =>
            {
                add("$schema", rule.resolution())?
            }
            "FN011003_MAN_listViewCommandSet_schema"
                if ext("ListViewCommandSet") && schema_differs() =>
            {
                add("$schema", rule.resolution())?
            }
            "FN011004_MAN_fieldCustomizer_schema" if ext("FieldCustomizer") && schema_differs() => {
                add("$schema", rule.resolution())?
            }
            "FN011005_MAN_webpart_defaultGroup" if web_part && t(m.get("preconfiguredEntries")) => {
                for (i, e) in m["preconfiguredEntries"]
                    .as_array()
                    .map(Vec::as_slice)
                    .unwrap_or(&[])
                    .iter()
                    .enumerate()
                {
                    if t(e.get("group"))
                        && js_eq(
                            at(Some(e), &["group", "default"]),
                            rule.f("oldDefaultGroup"),
                        )
                    {
                        add(
                            &format!("preconfiguredEntries[{i}].group.default"),
                            rule.resolution(),
                        )?;
                    }
                }
            }
            "FN011006_MAN_listViewCommandSet_items"
                if ext("ListViewCommandSet") && !t(m.get("items")) =>
            {
                // `items: commandSetManifest.commands || {}` aliases the manifest's object
                let aliased = commands.as_ref().is_some_and(truthy);
                let mut items = commands
                    .clone()
                    .filter(truthy)
                    .unwrap_or(Value::Object(Obj::new()));
                if let Some(o) = items.as_object_mut() {
                    for (_, v) in o.iter_mut() {
                        if let Some(c) = v.as_object_mut() {
                            let title = c.get("title").cloned();
                            c.insert("title".into(), obj! { "default": ?title });
                            c.insert("type".into(), Value::from("command"));
                        }
                    }
                }
                if aliased {
                    p.commands.borrow_mut().insert(mi, items.clone());
                }
                add("commands", pretty(&obj! { "items": items }))?;
            }
            "FN011007_MAN_listViewCommandSet_removeCommands"
                if ext("ListViewCommandSet") && t(commands.as_ref()) =>
            {
                add(
                    "commands",
                    pretty(&obj! { "commands": commands.clone().unwrap_or(Value::Null) }),
                )?;
            }
            "FN011008_MAN_requiresCustomScript"
                if m.get("safeWithCustomScriptDisabled").is_some() =>
            {
                add(
                    "safeWithCustomScriptDisabled",
                    format!(
                        "{{\n  \"requiresCustomScript\": {}\n}}",
                        !t(m.get("safeWithCustomScriptDisabled"))
                    ),
                )?;
            }
            "FN011009_MAN_webpart_safeWithCustomScriptDisabled"
            | "FN011011_MAN_webpart_supportedHosts"
                if web_part =>
            {
                let key = if class == "FN011009_MAN_webpart_safeWithCustomScriptDisabled" {
                    "safeWithCustomScriptDisabled"
                } else {
                    "supportedHosts"
                };
                let has = m.get(key).is_some();
                if (rule.fb("add") && !has) || (!rule.fb("add") && has) {
                    add(key, rule.resolution())?;
                }
            }
            "FN011010_MAN_webpart_version"
                if web_part && m.get("version").and_then(Value::as_str) != Some("*") =>
            {
                add("version", rule.resolution())?
            }
            "FN011001_MAN_webpart_schema"
            | "FN011002_MAN_applicationCustomizer_schema"
            | "FN011003_MAN_listViewCommandSet_schema"
            | "FN011004_MAN_fieldCustomizer_schema"
            | "FN011005_MAN_webpart_defaultGroup"
            | "FN011006_MAN_listViewCommandSet_items"
            | "FN011007_MAN_listViewCommandSet_removeCommands"
            | "FN011008_MAN_requiresCustomScript"
            | "FN011009_MAN_webpart_safeWithCustomScriptDisabled"
            | "FN011010_MAN_webpart_version"
            | "FN011011_MAN_webpart_supportedHosts" => {}
            _ => return Err(Error::msg(format!("spfx rule {class} is not ported"))),
        }
    }
    if !occurrences.is_empty() {
        rule.add_occurrences(occurrences, findings);
    }
    Ok(())
}

fn vscode_rule(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let v = &p.vs_code;
    let class = rule.class.trim_end_matches("@doctor");
    match class {
        "FN014001_CODE_settings_jsonSchemas" => {
            let Some(s) = &v.settings_json else {
                return Ok(());
            };
            let has = t(s.get("json.schemas"));
            if rule.fb("add") != has {
                rule.add_at(findings, p.position(s, "json.schemas")?);
            }
        }
        "FN014002_CODE_extensions" => {
            if v.extensions_json.is_none() {
                rule.add(findings);
            }
        }
        "FN014003_CODE_launch" => {
            let norm = |s: &str| s.replace("\r\n", "\n");
            let differs = match &v.launch_json {
                None => true,
                Some(l) => {
                    l.get("source").map(|s| norm(&to_js_string(s)))
                        != Some(norm(&rule.fs("contents")))
                }
            };
            if differs {
                rule.add(findings);
            }
        }
        "FN014004_CODE_settings_jsonSchemas_configJson_url" => {
            let Some(s) = v
                .settings_json
                .as_ref()
                .filter(|s| t(s.get("json.schemas")))
            else {
                return Ok(());
            };
            let schemas = s["json.schemas"]
                .as_array()
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            for (i, sc) in schemas.iter().enumerate() {
                let fm = sc.get("fileMatch").ok_or_else(|| {
                    Error::js("Cannot read properties of undefined (reading 'indexOf')")
                })?;
                if index_of(fm, "/config/config.json") == -1 {
                    continue;
                }
                if !js_eq(sc.get("url"), rule.f("url")) {
                    rule.add_at(findings, p.position(s, &format!("json;#schemas[{i}]"))?);
                }
                return Ok(());
            }
        }
        "FN014005_CODE_settingsfile" => {
            if !Path::new(&p.path).join(rule.file()).exists() {
                rule.add(findings);
            }
        }
        "FN014006_CODE_launch_sourceMapPathOverrides"
        | "FN014007_CODE_launch_localWorkbench"
        | "FN014008_CODE_launch_hostedWorkbench_type"
        | "FN014009_CODE_launch_hostedWorkbench_url" => {
            let Some(l) = v
                .launch_json
                .as_ref()
                .filter(|l| t(l.get("configurations")))
            else {
                return Ok(());
            };
            let configs = l["configurations"].as_array().ok_or_else(|| {
                Error::js("project.vsCode.launchJson.configurations.forEach is not a function")
            })?;
            let mut occurrences = Vec::new();
            for (i, c) in configs.iter().enumerate() {
                let (hit, prop) = match class {
                    "FN014006_CODE_launch_sourceMapPathOverrides" => (
                        t(c.get("sourceMapPathOverrides"))
                            && !t(c["sourceMapPathOverrides"].get(rule.fs("overrideKey"))),
                        "sourceMapPathOverrides",
                    ),
                    "FN014007_CODE_launch_localWorkbench" => (
                        t(c.get("url")) && index_of(&c["url"], "/temp/workbench.html") > -1,
                        "url",
                    ),
                    "FN014008_CODE_launch_hostedWorkbench_type" => (
                        c.get("name").and_then(Value::as_str) == Some("Hosted workbench")
                            && !js_eq(c.get("type"), rule.f("type")),
                        "url",
                    ),
                    _ => (
                        c.get("name").and_then(Value::as_str) == Some("Hosted workbench")
                            && !js_eq(c.get("url"), rule.f("url")),
                        "url",
                    ),
                };
                if hit {
                    let pos = p.position(l, &format!("configurations[{i}].{prop}"))?;
                    occurrences.push(occ(
                        crate::project::relative(&p.path, &abs(&rule.file())),
                        rule.resolution(),
                        Some(pos),
                    ));
                }
            }
            if !occurrences.is_empty() {
                rule.add_occurrences(occurrences, findings);
            }
        }
        "FN014010_CODE_settings_filesexclude_jest" => {
            let Some(s) = v
                .settings_json
                .as_ref()
                .filter(|s| t(s.get("files.exclude")))
            else {
                return Ok(());
            };
            if s["files.exclude"].get("**/jest-output") == Some(&Value::Bool(true)) {
                return Ok(());
            }
            rule.add_at(findings, p.position(s, "files;#exclude")?);
        }
        _ => return Err(Error::msg(format!("spfx rule {class} is not ported"))),
    }
    Ok(())
}

/// `path.resolve(p)` relative to the current directory.
fn abs(p: &str) -> String {
    let cwd = std::env::current_dir()
        .map(|d| d.to_string_lossy().into_owned())
        .unwrap_or_default();
    let joined = if p.starts_with('/') {
        p.to_string()
    } else {
        format!("{cwd}/{p}")
    };
    let mut parts: Vec<&str> = Vec::new();
    for s in joined.split('/') {
        match s {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    format!("/{}", parts.join("/"))
}

fn eslintrc_file(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    // `this.contents` is replaced in place (and stays replaced), so the
    // resolution follows the current contents
    let mut contents = rule.f("contents").map(to_js_string);
    if is_react(p)
        && let Some(c) = contents.as_mut()
        && !c.is_empty()
    {
        *c = c.replacen(
            "@microsoft/eslint-config-spfx/lib/profiles/default",
            "@microsoft/eslint-config-spfx/lib/profiles/react",
            1,
        );
    }
    let file_path = rule.fs("filePath");
    let add = rule.fb("add");
    let resolution = if add {
        format!(
            "add_cmd[BEFOREPATH]\"{file_path}\"[AFTERPATH][BEFORECONTENT]\n{}\n[AFTERCONTENT]",
            contents.clone().unwrap_or_else(|| "undefined".into())
        )
    } else {
        format!("remove_cmd \"{file_path}\"")
    };
    let finding = |findings: &mut Vec<Finding>| {
        rule.add_occurrences(vec![occ(rule.file(), resolution.clone(), None)], findings)
    };
    let target = Path::new(&p.path).join(&file_path);
    let exists = target.exists();
    if (!add && exists) || (add && !exists) {
        finding(findings);
        return Ok(());
    }
    if add && let Some(c) = contents.filter(|c| !c.is_empty()) {
        let content = std::fs::read_to_string(&target)
            .map_err(|e| Error::js(util::node_fs_error(&e, "open", &target.to_string_lossy())))?;
        if content != c {
            finding(findings);
        }
    }
    Ok(())
}

fn teams_rule(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let class = rule.class.trim_end_matches("@doctor");
    if class == "FN018005_TEAMS_deprecated_manifest" {
        let Some(fn018002) = CATALOG
            .rules
            .iter()
            .find(|r| r.class == "FN018002_TEAMS_manifest")
        else {
            return Ok(());
        };
        let mut sub = Vec::new();
        teams_rule(fn018002, p, &mut sub)?;
        if !sub.is_empty() {
            rule.add(findings);
        }
        return Ok(());
    }
    let web_parts: Vec<&Value> = p
        .manifests
        .iter()
        .filter(|m| m.get("componentType").and_then(Value::as_str) == Some("WebPart"))
        .collect();
    if web_parts.is_empty() {
        return Ok(());
    }
    let teams = Path::new(&p.path)
        .join("teams")
        .to_string_lossy()
        .into_owned();
    match class {
        "FN018001_TEAMS_folder" => {
            if !Path::new(&teams).exists() {
                rule.add_custom(
                    rule.title(),
                    rule.description(),
                    vec![occ(
                        p.relative(&teams),
                        format!(
                            "create_dir_cmdPathParam{}NameParamteamsItemTypeParam",
                            p.path
                        ),
                        None,
                    )],
                    findings,
                );
            }
        }
        "FN018002_TEAMS_manifest" => {
            let mut occurrences = Vec::new();
            for m in web_parts {
                let mpath = jsu(m.get("path"));
                let folder = Path::new(&mpath)
                    .parent()
                    .and_then(|d| d.file_name())
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let manifest = Path::new(&teams)
                    .join(format!("manifest_{folder}.json"))
                    .to_string_lossy()
                    .into_owned();
                if Path::new(&manifest).exists() {
                    continue;
                }
                let entry = m
                    .get("preconfiguredEntries")
                    .and_then(Value::as_array)
                    .and_then(|a| a.first());
                let title = at(entry, &["title", "default"])
                    .filter(|v| truthy(v))
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into());
                let description = at(entry, &["description", "default"])
                    .filter(|v| truthy(v))
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into());
                let id = m
                    .get("id")
                    .filter(|v| truthy(v))
                    .map(to_js_string)
                    .unwrap_or_else(|| "undefined".into());
                let res = rule
                    .resolution()
                    .replace("__filePath__", &manifest)
                    .replace("__packageName__", &title)
                    .replace("__id__", &id)
                    .replace("__name__", &title)
                    .replace("__description__", &description);
                occurrences.push(occ(p.relative(&manifest), res, None));
            }
            if !occurrences.is_empty() {
                rule.add_occurrences(occurrences, findings);
            }
        }
        "FN018003_TEAMS_tab20x20_png" | "FN018004_TEAMS_tab96x96_png" => {
            let (asset, suffix) = if class == "FN018003_TEAMS_tab20x20_png" {
                ("tab20x20.png", "outline")
            } else {
                ("tab96x96.png", "color")
            };
            let mut occurrences = Vec::new();
            for m in web_parts {
                let Some(id) = m.get("id").filter(|v| truthy(v)) else {
                    continue;
                };
                let name = rule
                    .f("fixedFileName")
                    .filter(|v| truthy(v))
                    .map(to_js_string)
                    .unwrap_or_else(|| format!("{}_{suffix}.png", to_js_string(id)));
                let icon = Path::new(&teams).join(&name).to_string_lossy().into_owned();
                if !Path::new(&icon).exists() {
                    occurrences.push(occ(
                        p.relative(&icon),
                        format!(
                            "copy_cmd \"{}\"DestinationParam\"{icon}\"",
                            asset_path(asset)
                        ),
                        None,
                    ));
                }
            }
            if !occurrences.is_empty() {
                rule.add_occurrences(occurrences, findings);
            }
        }
        _ => return Err(Error::msg(format!("spfx rule {class} is not ported"))),
    }
    Ok(())
}

/// Upstream points at its own install folder for the Teams icons; the
/// port writes the embedded icons to a cache folder and points there.
fn asset_path(name: &str) -> String {
    let dir = std::env::temp_dir().join("m365-spfx-assets");
    let file = dir.join(name);
    if !file.exists() {
        let bytes: &[u8] = if name == "tab20x20.png" {
            include_bytes!("assets/tab20x20.png")
        } else {
            include_bytes!("assets/tab96x96.png")
        };
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(&file, bytes);
    }
    file.to_string_lossy().into_owned()
}

fn package_rule(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let class = rule.class.trim_end_matches("@doctor");
    let Some(pkg) = &p.package_json else {
        return Ok(());
    };
    let deps = pkg.get("dependencies");
    let dev = pkg.get("devDependencies");
    let version = p.version.clone();
    let custom = |title: String,
                  description: String,
                  resolution: String,
                  pos: (usize, usize),
                  findings: &mut Vec<Finding>| {
        rule.add_custom(
            Some(title),
            description,
            vec![occ(rule.file(), resolution, Some(pos))],
            findings,
        );
    };
    match class {
        "FN021001_PKG_spfx_deps_versions_match_project_version"
        | "FN021002_PKG_spfx_deps_use_exact_version" => {
            let Some(v) = version.filter(|v| !v.is_empty()) else {
                return Ok(());
            };
            let all: Vec<&str> = SPFX_DEPS
                .iter()
                .chain(SPFX_DEV_DEPS.iter())
                .copied()
                .collect();
            let include_dev =
                class == "FN021002_PKG_spfx_deps_use_exact_version" || rule.fb("includeDevDeps");
            for (list, is_dev) in [(deps, false), (if include_dev { dev } else { None }, true)] {
                if !t(list) {
                    continue;
                }
                for dep in keys(list) {
                    let dv = jsu(list.and_then(|l| l.get(&dep)));
                    let skip = !all.contains(&dep.as_str())
                        || if class == "FN021001_PKG_spfx_deps_versions_match_project_version" {
                            semver::satisfies(&v, &dv)
                        } else {
                            semver::valid(&dv).is_some()
                        };
                    if skip {
                        continue;
                    }
                    let section = if is_dev {
                        "devDependencies"
                    } else {
                        "dependencies"
                    };
                    let pos = p.position(pkg, &format!("{section}.{dep}"))?;
                    let op = if is_dev { "installDev" } else { "install" };
                    if class == "FN021001_PKG_spfx_deps_versions_match_project_version" {
                        custom(
                            format!("{dep} doesn't match project version"),
                            format!("{dep}@{dv} doesn't match the project version {v}"),
                            format!("{op} {dep}@{v}"),
                            pos,
                            findings,
                        );
                    } else {
                        custom(
                            format!("{dep} is not using exact version"),
                            format!(
                                "{dep} is referenced using a range {dv}. Install the exact version matching the project {dep}@{v}"
                            ),
                            format!("{op} {dep}@{v}"),
                            pos,
                            findings,
                        );
                    }
                }
            }
        }
        "FN021003_PKG_engines_node" => {
            let e = pkg.get("engines");
            let node = e
                .filter(|e| e.is_object() || e.is_array())
                .and_then(|e| e.get("node"));
            if !t(e)
                || !e.is_some_and(|e| e.is_object() || e.is_array())
                || !t(node)
                || !js_eq(node, rule.f("version"))
            {
                rule.add_at(findings, p.position(pkg, "engines.node")?);
            }
        }
        "FN021003_PKG_spfx_deps_installed_as_deps" => {
            let (Some(v), true) = (version.filter(|v| !v.is_empty()), t(dev)) else {
                return Ok(());
            };
            for dep in keys(dev)
                .into_iter()
                .filter(|d| SPFX_DEPS.contains(&d.as_str()))
            {
                let pos = p.position(pkg, &format!("devDependencies.{dep}"))?;
                custom(
                    format!("{dep} installed as devDependency"),
                    format!(
                        "Package {dep} is installed as a devDependency. Install it as a dependency instead"
                    ),
                    format!("install {dep}@{v}"),
                    pos,
                    findings,
                );
            }
        }
        "FN021004_PKG_spfx_devdeps_installed_as_devdeps" => {
            let (Some(v), true) = (version.filter(|v| !v.is_empty()), t(deps)) else {
                return Ok(());
            };
            for dep in keys(deps)
                .into_iter()
                .filter(|d| SPFX_DEV_DEPS.contains(&d.as_str()))
            {
                let pos = p.position(pkg, &format!("dependencies.{dep}"))?;
                custom(
                    format!("{dep} installed as a dependency"),
                    format!(
                        "Package {dep} is installed as a dependency. Install it as a devDependency instead"
                    ),
                    format!("installDev {dep}@{v}"),
                    pos,
                    findings,
                );
            }
        }
        "FN021005_PKG_scripts_test" => {
            let scripts = pkg.get("scripts");
            let test = scripts
                .filter(|s| s.is_object())
                .and_then(|s| s.get("test"));
            let hit = if rule.fb("add") {
                !t(scripts)
                    || !scripts.is_some_and(|s| s.is_object() || s.is_array())
                    || !t(test)
                    || !js_eq(test, rule.f("script"))
            } else {
                js_eq(at(scripts, &["test"]), rule.f("script"))
            };
            if hit {
                rule.add_at(findings, p.position(pkg, "scripts.test")?);
            }
        }
        "FN021005_PKG_types_installed_as_devdep"
        | "FN021006_PKG_rush_stack_compiler_installed_as_devdep" => {
            if version.as_deref().is_none_or(str::is_empty) || !t(deps) {
                return Ok(());
            }
            let needle = if class == "FN021005_PKG_types_installed_as_devdep" {
                "@types/"
            } else {
                "@microsoft/rush-stack-compiler"
            };
            for dep in keys(deps).into_iter().filter(|d| d.contains(needle)) {
                let pos = p.position(pkg, &format!("dependencies.{dep}"))?;
                custom(
                    format!("{dep} installed as a dependency"),
                    format!(
                        "Package {dep} is installed as a dependency. Install it as a devDependency instead"
                    ),
                    format!("installDev {dep}@{}", jsu(deps.and_then(|d| d.get(&dep)))),
                    pos,
                    findings,
                );
            }
        }
        "FN021007_PKG_only_one_rush_stack_compiler_installed" => {
            if !t(deps) {
                return Ok(());
            }
            let mut all = keys(deps);
            let mut devs = Vec::new();
            if t(dev) {
                devs = keys(dev);
                all.extend(devs.iter().cloned());
            }
            let stacks: Vec<String> = all
                .into_iter()
                .filter(|d| d.starts_with("@microsoft/rush-stack-compiler"))
                .collect();
            if stacks.len() <= 1 {
                return Ok(());
            }
            static RE: LazyLock<regex::Regex> = LazyLock::new(|| {
                regex::Regex::new("@microsoft/rush-stack-compiler[^/]+").expect("re")
            });
            let ext = at(p.ts_config_json.as_ref(), &["extends"])
                .filter(|v| truthy(v))
                .map(to_js_string);
            let keep = ext
                .and_then(|e| RE.find(&e).map(|m| m.as_str().to_string()))
                .unwrap_or_else(|| stacks[0].clone());
            for rs in stacks.iter().filter(|s| **s != keep) {
                let is_dev = devs.contains(rs);
                let section = if is_dev {
                    "devDependencies"
                } else {
                    "dependencies"
                };
                let pos = p.position(pkg, &format!("{section}.{rs}"))?;
                custom(
                    format!("Obsolete {rs} found in {section}"),
                    format!("Multiple rush-stack-compiler packages found. Uninstall obsolete {rs}"),
                    format!("{} {rs}", if is_dev { "uninstallDev" } else { "uninstall" }),
                    pos,
                    findings,
                );
            }
        }
        "FN021008_PKG_no_duplicate_deps" => {
            if version.as_deref().is_none_or(str::is_empty) || !t(deps) || !t(dev) {
                return Ok(());
            }
            let devs = keys(dev);
            for dep in keys(deps).into_iter().filter(|d| devs.contains(d)) {
                let is_dev = dep.starts_with("@types/") || SPFX_DEV_DEPS.contains(&dep.as_str());
                let pos = p.position(
                    pkg,
                    &format!(
                        "{}.{dep}",
                        if is_dev {
                            "dependencies"
                        } else {
                            "devDependencies"
                        }
                    ),
                )?;
                let ver = jsu(if is_dev { dev } else { deps }.and_then(|l| l.get(&dep)));
                custom(
                    format!("Duplicate {dep} installed in the project"),
                    format!(
                        "Duplicate {dep} installed in the project. Install it only as a {}",
                        if is_dev {
                            "devDependency"
                        } else {
                            "dependency"
                        }
                    ),
                    format!(
                        "{} {dep}@{ver}",
                        if is_dev { "installDev" } else { "install" }
                    ),
                    pos,
                    findings,
                );
            }
        }
        "FN021009_PKG_no_duplicate_oui_deps" | "FN021012_PKG_no_duplicate_pnpjs_deps" => {
            let (old, new) = if class == "FN021009_PKG_no_duplicate_oui_deps" {
                ("office-ui-fabric-react", "@fluentui/react")
            } else {
                ("sp-pnp-js", "@pnp/sp")
            };
            if class == "FN021009_PKG_no_duplicate_oui_deps" {
                // `coerce(v) < new SemVer('7.199.0')` compares the version strings
                let v = at(Some(pkg), &["dependencies", old]);
                let c = match v {
                    Some(Value::String(s)) => semver::coerce(s),
                    Some(Value::Number(n)) => {
                        semver::coerce(&json::number_to_string(n.as_f64().unwrap_or(0.0)))
                    }
                    _ => None,
                };
                if c.is_none_or(|c| c.version.as_str() >= "7.199.0") {
                    return Ok(());
                }
            }
            let mut d = Vec::new();
            if t(deps) {
                d = keys(deps);
            }
            let mut dv = Vec::new();
            if t(dev) {
                dv = keys(dev);
            }
            let all: Vec<&String> = d.iter().chain(dv.iter()).collect();
            if all.is_empty() || !all.iter().any(|x| *x == old) || !all.iter().any(|x| *x == new) {
                return Ok(());
            }
            let target = if class == "FN021009_PKG_no_duplicate_oui_deps" {
                new
            } else {
                old
            };
            let section = if d.iter().any(|x| x == target) {
                "dependencies"
            } else {
                "devDependencies"
            };
            let pos = p.position(pkg, &format!("{section}.{target}"))?;
            let op = if section == "dependencies" {
                "uninstall"
            } else {
                "uninstallDev"
            };
            rule.add_custom(
                rule.title(),
                rule.description(),
                vec![occ(rule.file(), format!("{op} {target}"), Some(pos))],
                findings,
            );
        }
        "FN021010_PKG_gulp_installed_as_devdep" | "FN021011_PKG_ajv_installed_as_devdep" => {
            let name = if class == "FN021010_PKG_gulp_installed_as_devdep" {
                "gulp"
            } else {
                "ajv"
            };
            if !t(deps) || !keys(deps).iter().any(|d| d == name) {
                return Ok(());
            }
            let pos = p.position(pkg, &format!("dependencies.{name}"))?;
            rule.add_custom(
                rule.title(),
                rule.description(),
                vec![occ(
                    rule.file(),
                    format!("installDev {name}@{}", jsu(deps.and_then(|d| d.get(name)))),
                    Some(pos),
                )],
                findings,
            );
        }
        "FN021013_PKG_spfx_devdeps_match_version" => {
            if !t(dev) {
                return Ok(());
            }
            let rv = rule.fs("version");
            let pv = version.clone().unwrap_or_else(|| "undefined".into());
            for dep in keys(dev) {
                let dv = jsu(dev.and_then(|d| d.get(&dep)));
                if !SPFX_DEV_DEPS.contains(&dep.as_str()) || semver::satisfies(&rv, &dv) {
                    continue;
                }
                let pos = p.position(pkg, "devDependencies")?;
                custom(
                    format!("{dep} doesn't match project version"),
                    format!("{dep}@{dv} doesn't match the project version {pv}"),
                    format!("installDev {dep}@{pv}"),
                    pos,
                    findings,
                );
            }
        }
        _ => return Err(Error::msg(format!("spfx rule {class} is not ported"))),
    }
    Ok(())
}

// ---- TypeScript rules ------------------------------------------------------------

fn ts_occ(rule_res: String, file: &SourceFile, p: &Project, offset: usize) -> Occurrence {
    let src = file.source.as_deref().unwrap_or("");
    let (line, character) = tslex::line_char(src, offset);
    occ(p.relative(&file.path), rule_res, Some((line, character)))
}

/// An `import ...;` statement: token range and the module specifier.
struct Import {
    start: usize,
    end: usize,
    first: usize,
    specifier: Option<usize>,
    names: Vec<String>,
}

fn imports(src: &str, toks: &[Token]) -> Vec<Import> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        let tk = &toks[i];
        let prev = i.checked_sub(1).map(|j| toks[j].text(src));
        let next = toks.get(i + 1).map(|n| n.text(src));
        if tk.kind == Kind::Ident
            && tk.text(src) == "import"
            && !matches!(next, Some("(" | "."))
            && !matches!(prev, Some("." | "?."))
        {
            let first = i;
            let mut j = i + 1;
            let mut names = Vec::new();
            let mut in_braces = false;
            let mut last_ident: Option<String> = None;
            let mut specifier = None;
            while j < toks.len() {
                let x = toks[j].text(src);
                match x {
                    "{" => in_braces = true,
                    "}" => {
                        if let Some(n) = last_ident.take() {
                            names.push(n);
                        }
                        in_braces = false;
                    }
                    "," if in_braces => {
                        if let Some(n) = last_ident.take() {
                            names.push(n);
                        }
                    }
                    _ if in_braces && toks[j].kind == Kind::Ident => {
                        last_ident = Some(x.to_string())
                    }
                    _ => {}
                }
                if !in_braces && toks[j].kind == Kind::Str {
                    specifier = Some(j);
                    break;
                }
                j += 1;
            }
            let mut end_tok = j.min(toks.len().saturating_sub(1));
            if toks.get(j + 1).is_some_and(|t| t.text(src) == ";") {
                end_tok = j + 1;
            }
            out.push(Import {
                start: toks[first].start,
                end: toks[end_tok].end,
                first,
                specifier,
                names,
            });
            i = end_tok + 1;
            continue;
        }
        i += 1;
    }
    out
}

/// The variable declaration name an expression starting at token `k`
/// initializes (`const x: T = <expr>`), as a byte offset.
fn var_decl_start(src: &str, toks: &[Token], k: usize) -> Option<usize> {
    if k == 0 || toks[k - 1].text(src) != "=" {
        return None;
    }
    let mut j = k - 1;
    let mut depth = 0i32;
    while j > 0 {
        j -= 1;
        let x = toks[j].text(src);
        match x {
            ">" | ")" | "]" => depth += 1,
            "<" | "(" | "[" => depth -= 1,
            ";" | "{" | "}" | "=" if depth == 0 => return None,
            _ => {}
        }
        if depth == 0
            && toks[j].kind == Kind::Ident
            && j > 0
            && matches!(toks[j - 1].text(src), "const" | "let" | "var" | ",")
        {
            return Some(toks[j].start);
        }
    }
    None
}

/// Argument token ranges of the call whose `(` is token `open`.
fn call_args(src: &str, toks: &[Token], open: usize) -> Vec<(usize, usize)> {
    let mut args = Vec::new();
    let mut depth = 0;
    let mut start = open + 1;
    let mut j = open;
    while j < toks.len() {
        let x = toks[j].text(src);
        match x {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth -= 1;
                if depth == 0 {
                    if j > start {
                        args.push((start, j - 1));
                    }
                    break;
                }
            }
            "," if depth == 1 => {
                args.push((start, j - 1));
                start = j + 1;
            }
            _ => {}
        }
        j += 1;
    }
    args
}

fn ts_rule(rule: &Rule, p: &Project, findings: &mut Vec<Finding>) -> Result<()> {
    let class = rule.class.trim_end_matches("@doctor");
    let mut occurrences = Vec::new();
    if class.starts_with("FN025") {
        let Some(f) = &p.es_lint_rc_js else {
            return Ok(());
        };
        // `TsFile.sourceFile` is undefined for an empty file
        let Some(src) = f.source.as_deref().filter(|s| !s.is_empty()) else {
            return Ok(());
        };
        let toks = tslex::tokenize(src);
        let text_of = |want: &str| {
            toks.iter().find(|tk| {
                matches!(tk.kind, Kind::Ident | Kind::Str | Kind::Num) && tk.value(src) == want
            })
        };
        let has_literal = |lit: &str| {
            toks.iter()
                .any(|tk| tk.kind == Kind::Str && tk.text(src) == lit)
        };
        let node = match class {
            "FN025001_ESLINTRCJS_overrides" => {
                if toks
                    .iter()
                    .any(|tk| tk.kind == Kind::Ident && tk.text(src) == "overrides")
                {
                    return Ok(());
                }
                text_of("module")
            }
            "FN025002_ESLINTRCJS_rushstack_import_requires_chunk_name"
            | "FN025003_ESLINTRCJS_rushstack_pair_react_dom_render_unmount" => {
                let lit = if class.starts_with("FN025002") {
                    "'@rushstack/import-requires-chunk-name'"
                } else {
                    "'@rushstack/pair-react-dom-render-unmount'"
                };
                if has_literal(lit) {
                    return Ok(());
                }
                text_of("rules")
            }
            _ => {
                let lit = if class.starts_with("FN025004") {
                    "'@microsoft/spfx/import-requires-chunk-name'"
                } else {
                    "'@microsoft/spfx/pair-react-dom-render-unmount'"
                };
                toks.iter()
                    .find(|tk| tk.kind == Kind::Str && tk.text(src) == lit)
            }
        };
        let Some(node) = node else { return Ok(()) };
        occurrences.push(ts_occ(rule.resolution(), f, p, node.start));
    } else {
        for f in &p.ts_files {
            let Some(src) = f.source.as_deref().filter(|s| !s.is_empty()) else {
                continue;
            };
            let toks = tslex::tokenize(src);
            match class {
                "FN016001_TS_msgraphclient_packageName" => {
                    let pkg = rule.fs("packageName");
                    for imp in imports(src, &toks) {
                        let Some(sp) = imp.specifier else { continue };
                        for _ in imp.names.iter().filter(|n| *n == "MSGraphClient") {
                            let spec = toks[sp].text(src);
                            if spec != format!("\"{pkg}\"") && spec != format!("'{pkg}'") {
                                let res = src[imp.start..imp.end].replacen(
                                    spec,
                                    &format!("\"{pkg}\""),
                                    1,
                                );
                                occurrences.push(ts_occ(res, f, p, imp.start));
                            }
                        }
                    }
                }
                "FN016002_TS_msgraphclient_instance" => {
                    for (k, tk) in toks.iter().enumerate() {
                        if tk.kind != Kind::Ident
                            || tk.text(src) != "consume"
                            || k < 3
                            || toks[k - 1].text(src) != "."
                            || toks[k - 2].text(src) != "serviceScope"
                            || toks[k - 3].text(src) != "."
                        {
                            continue;
                        }
                        let Some(open) = toks
                            .get(k + 1)
                            .filter(|o| o.text(src) == "(")
                            .map(|_| k + 1)
                        else {
                            continue;
                        };
                        let args = call_args(src, &toks, open);
                        let Some(&(a, b)) = args.first() else {
                            continue;
                        };
                        if b != a + 2
                            || toks[a].text(src) != "MSGraphClient"
                            || toks[a + 1].text(src) != "."
                            || toks[a + 2].text(src) != "serviceKey"
                        {
                            continue;
                        }
                        // the start of the `a.b.serviceScope.consume` chain
                        let mut s = k - 2;
                        while s >= 2
                            && toks[s - 1].text(src) == "."
                            && toks[s - 2].kind == Kind::Ident
                        {
                            s -= 2;
                        }
                        if let Some(start) = var_decl_start(src, &toks, s) {
                            occurrences.push(ts_occ(rule.resolution(), f, p, start));
                        }
                    }
                }
                "FN016003_TS_aadhttpclient_instance" => {
                    for (k, tk) in toks.iter().enumerate() {
                        if tk.kind != Kind::Ident
                            || tk.text(src) != "new"
                            || toks
                                .get(k + 1)
                                .is_none_or(|n| n.text(src) != "AadHttpClient")
                        {
                            continue;
                        }
                        let args = toks
                            .get(k + 2)
                            .filter(|o| o.text(src) == "(")
                            .map(|_| call_args(src, &toks, k + 2))
                            .unwrap_or_default();
                        let resource = if args.len() == 2 {
                            src[toks[args[1].0].start..toks[args[1].1].end].to_string()
                        } else {
                            "/* your resource */".to_string()
                        };
                        if let Some(start) = var_decl_start(src, &toks, k) {
                            let res = format!(
                                "this.context.aadHttpClientFactory\n  .getClient({resource})\n  .then((client: AadHttpClient): void => {{\n    // use AadHttpClient here\n  }});"
                            );
                            occurrences.push(ts_occ(res, f, p, start));
                        }
                    }
                }
                "FN016004_TS_property_pane_property_import" => {
                    for imp in imports(src, &toks) {
                        let text = &src[imp.start..imp.end];
                        if text
                            .find("@microsoft/sp-webpart-base")
                            .is_none_or(|i| i == 0)
                        {
                            continue;
                        }
                        let _ = imp.first;
                        let brace = text.find('{').map_or(-1i64, |i| i as i64);
                        let stripped: String =
                            text.chars().filter(|c| !c.is_whitespace()).collect();
                        let from = if brace < 0 {
                            0
                        } else {
                            (brace as usize).min(stripped.len())
                        };
                        let rest = &stripped[from..];
                        let names: Vec<&str> = rest[..rest.find('}').unwrap_or(rest.len())]
                            .split(',')
                            .collect();
                        let moved: Vec<&str> = names
                            .iter()
                            .copied()
                            .filter(|n| PROPERTY_PANE.contains(n))
                            .collect();
                        let stay: Vec<&str> = names
                            .iter()
                            .copied()
                            .filter(|n| !PROPERTY_PANE.contains(n))
                            .collect();
                        if moved.is_empty() {
                            continue;
                        }
                        let mut res = format!(
                            "import {{ {} }} from \"@microsoft/sp-property-pane\";",
                            moved.join(", ")
                        );
                        if !stay.is_empty() {
                            res = format!(
                                "import {{ {} }} from \"@microsoft/sp-webpart-base\";\n{res}",
                                stay.join(", ")
                            );
                        }
                        occurrences.push(ts_occ(res, f, p, imp.start));
                    }
                }
                _ => return Err(Error::msg(format!("spfx rule {class} is not ported"))),
            }
        }
    }
    if !occurrences.is_empty() {
        rule.add_occurrences(occurrences, findings);
    }
    Ok(())
}

const PROPERTY_PANE: [&str; 58] = [
    "PropertyPaneLifeCycleEvent",
    "ILifeCycleEventCallback",
    "_PropertyPaneController",
    "PropertyPaneAction",
    "IPropertyPaneData",
    "IPropertyPaneConfiguration",
    "IPropertyPanePage",
    "IPropertyPanePageHeader",
    "IPropertyPaneGroup",
    "IPropertyPaneField",
    "PropertyPaneFieldType",
    "IPropertyPaneCustomFieldProps",
    "PropertyPaneCustomField",
    "IPropertyPaneButtonProps",
    "PropertyPaneButtonType",
    "PropertyPaneButton",
    "IPropertyPaneCheckboxProps",
    "PropertyPaneCheckbox",
    "IPropertyPaneChoiceGroupOptionIconProps",
    "IPropertyPaneChoiceGroupProps",
    "IPropertyPaneChoiceGroupOption",
    "_IPropertyPaneChoiceGroupOptionInternal",
    "PropertyPaneChoiceGroup",
    "IPropertyPaneDropdownProps",
    "IPropertyPaneDropdownOption",
    "PropertyPaneDropdownOptionType",
    "IPropertyPaneDropdownCalloutProps",
    "PropertyPaneDropdown",
    "IPropertyPaneDynamicFieldFilters",
    "IPropertyPaneDynamicFieldProps",
    "PropertyPaneDynamicField",
    "PropertyPaneDynamicFieldSet",
    "IPropertyPaneDynamicFieldSetProps",
    "PropertyPaneHorizontalRule",
    "IPropertyPaneLabelProps",
    "PropertyPaneLabel",
    "IPropertyPaneLinkProps",
    "PropertyPaneLink",
    "IPropertyPaneSliderProps",
    "PropertyPaneSlider",
    "IPropertyPaneTextFieldProps",
    "PropertyPaneTextField",
    "IPropertyPaneDynamicTextFieldProps",
    "IConfiguredDynamicTextFieldProps",
    "PropertyPaneDynamicTextField",
    "IPropertyPaneToggleProps",
    "PropertyPaneToggle",
    "IPropertyPaneSpinButtonProps",
    "PropertyPaneSpinButton",
    "IPropertyPaneConsumer",
    "IDynamicConfiguration",
    "_IDynamicConfiguration",
    "IDynamicDataSharedSourceConfiguration",
    "IDynamicDataSharedPropertyConfiguration",
    "DynamicDataSharedDepth",
    "IDynamicDataSharedSourceFilters",
    "IDynamicDataSharedPropertyFilters",
    "IPropertyPaneConditionalGroup",
];

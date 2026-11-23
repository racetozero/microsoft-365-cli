//! `spfx doctor`: checks the machine against the SharePoint Framework
//! compatibility matrix (Node, yo, gulp-cli, heft, typescript), asking
//! `npm ls` like upstream does.

use crate::matrix;
use crate::project;
use crate::semver;
use m365_core::prelude::*;
use m365_core::util::CheckStatus;

struct Doctor<'a> {
    cx: &'a Cx,
    output: String,
    results: Vec<Value>,
}

#[derive(Clone, Copy, PartialEq)]
enum Search {
    LocalOnly,
    GlobalOnly,
    LocalAndGlobal,
}

/// Why a version lookup failed: the rejection upstream propagates.
enum Fail {
    /// `reject(string)` from `npm ls` (its message or `Package not found`).
    Reject(String),
    /// `throw new Error()` (`HandlePromise.Fail` without a version).
    Empty,
}

impl Doctor<'_> {
    fn msg(&self, m: &str) -> Result<()> {
        if self.output == "json" {
            self.cx.stderr(m);
            Ok(())
        } else {
            self.cx.log(Value::from(m))
        }
    }

    fn status(&self, s: CheckStatus, m: &str) -> Result<()> {
        self.msg(&util::status(s, m))
    }

    fn push(
        &mut self,
        check: &str,
        passed: bool,
        message: &str,
        version: Option<&str>,
        fix: Option<&str>,
    ) {
        // key order follows each upstream object literal
        let mut o = Obj::new();
        o.insert("check".into(), Value::from(check));
        o.insert("passed".into(), Value::Bool(passed));
        if let Some(v) = version {
            o.insert("version".into(), Value::from(v));
        }
        o.insert("message".into(), Value::from(message));
        if let Some(f) = fix {
            o.insert("fix".into(), Value::from(f));
        }
        self.results.push(Value::Object(o));
    }

    /// `getPackageVersionFromNpm`.
    fn npm(&self, args: &[&str]) -> Result<String, String> {
        self.cx
            .debug_log(&format!("Executing npm: {}...", args.join(" ")));
        let cmd = format!("npm {}", args.join(" "));
        let (out, _) = util::exec(&cmd).map_err(|e| format!("Command failed: {cmd}\n{e}"))?;
        let v: Value =
            json::js_parse(&out).map_err(|e| e.trim_start_matches("SyntaxError: ").to_string())?;
        v.get("dependencies")
            .filter(|d| truthy(d))
            .and_then(|d| d.get(args[1]))
            .filter(|p| truthy(p))
            .and_then(|p| p.get("version"))
            .map(to_js_string)
            .ok_or_else(|| "Package not found".to_string())
    }

    /// `getPackageVersion`; `Ok("")` for `HandlePromise.Continue` misses.
    fn package(&self, name: &str, mode: Search, fail: bool) -> Result<String, Fail> {
        let mut args = vec!["ls", name, "--depth=0", "--json"];
        if mode == Search::GlobalOnly {
            args.push("-g");
        }
        let version = match self.npm(&args) {
            Ok(v) => v,
            Err(e) => {
                if mode == Search::LocalAndGlobal {
                    args.push("-g");
                    self.npm(&args).map_err(Fail::Reject)?
                } else {
                    let _ = e;
                    String::new()
                }
            }
        };
        if !version.is_empty() {
            Ok(version)
        } else if fail {
            Err(Fail::Empty)
        } else {
            Ok(String::new())
        }
    }

    fn version_from_yo_rc(&self) -> Option<String> {
        let root = project::project_root(&std::env::current_dir().ok()?)?;
        let v = project::project_version(&root).filter(|v| !v.is_empty())?;
        self.cx.debug_log(&format!(
            "SPFx version retrieved from .yo-rc.json file. Retrieved version: {v}"
        ));
        Some(v)
    }

    /// `getSharePointFrameworkVersion`.
    fn spfx_version(&self) -> Result<String> {
        if let Some(v) = self.version_from_yo_rc() {
            return Ok(v);
        }
        if let Ok(v) = self.package("@microsoft/sp-core-library", Search::LocalOnly, true) {
            self.cx
                .debug_log(&format!("Found @microsoft/sp-core-library@{v}"));
            return Ok(v);
        }
        self.cx.debug_log("@microsoft/sp-core-library not found. Search for @microsoft/generator-sharepoint local or global...");
        match self.package(
            "@microsoft/generator-sharepoint",
            Search::LocalAndGlobal,
            true,
        ) {
            Ok(v) => Ok(v),
            Err(e) => {
                self.cx
                    .debug_log("@microsoft/generator-sharepoint not found");
                match e {
                    Fail::Reject(m) if m.contains("ENOENT") => {
                        Err(Error::Thrown(Value::from("npm not found")))
                    }
                    Fail::Reject(_) => Ok(String::new()),
                    Fail::Empty => Err(Error::js("error.indexOf is not a function")),
                }
            }
        }
    }

    /// `checkStatus`.
    fn check(&mut self, what: &str, found: &str, range: &str, fix: &str) -> Result<()> {
        if found.is_empty() {
            return Ok(());
        }
        if semver::satisfies(found, range) {
            let m = format!("{what} v{found}");
            self.results.push(
                obj! { "check": what, "passed": true, "message": m.as_str(), "version": found },
            );
            self.status(CheckStatus::Success, &m)
        } else {
            let m = format!("{what} v{found} found, v{range} required");
            self.push(what, false, &m, Some(found), Some(fix));
            self.status(CheckStatus::Failure, &m)
        }
    }

    fn check_global(&mut self, name: &str, check: &str, req: Option<&Value>) -> Result<()> {
        let Some(req) = req.filter(|r| truthy(r)) else {
            return Ok(());
        };
        let v = self
            .package(name, Search::GlobalOnly, false)
            .unwrap_or_default();
        let (range, fix) = (s(req, "range").to_string(), s(req, "fix").to_string());
        if v.is_empty() {
            let m = format!("{check} not found");
            self.push(check, false, &m, None, Some(&fix));
            return self.status(CheckStatus::Failure, &m);
        }
        self.check(check, &v, &range, &fix)
    }

    fn fixes(&self) -> Result<()> {
        let fixes: Vec<String> = self
            .results
            .iter()
            .filter_map(|r| r.get("fix").map(to_js_string))
            .collect();
        if !fixes.is_empty() {
            self.msg("Recommended fixes:")?;
            self.msg(" ")?;
            for f in fixes {
                self.msg(&format!("- {f}"))?;
            }
            self.msg(" ")?;
        }
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        let cx = self.cx;
        let requested = cx.s("spfxVersion").filter(|s| !s.is_empty());
        let version = match &requested {
            Some(v) => v.clone(),
            None => self.spfx_version()?,
        };
        if version.is_empty() {
            self.status(CheckStatus::Failure, "SharePoint Framework")?;
            self.push(
                "SharePoint Framework",
                false,
                "SharePoint Framework not found",
                None,
                None,
            );
            return Err(Error::Thrown(Value::from("SharePoint Framework not found")));
        }
        let Some(pre) = matrix::version(&version) else {
            let m = format!("spfx doctor doesn't support SPFx v{version} at this moment");
            self.push("SharePoint Framework", true, &m, Some(&version), None);
            self.status(
                CheckStatus::Failure,
                &format!("SharePoint Framework v{version}"),
            )?;
            return Err(Error::Thrown(Value::from(m)));
        };
        self.push(
            "SharePoint Framework",
            true,
            &format!("SharePoint Framework v{version} valid."),
            Some(&version),
            None,
        );
        if let Some(req) = &requested {
            let detected = match self.version_from_yo_rc() {
                Some(v) => v,
                None => self
                    .package("@microsoft/generator-sharepoint", Search::GlobalOnly, false)
                    .unwrap_or_default(),
            };
            let fix = format!("npm i -g @microsoft/generator-sharepoint@{req}");
            if !detected.is_empty() {
                self.check("SharePoint Framework", &detected, req, &fix)?;
            } else {
                let m = format!("SharePoint Framework v{req} not found");
                self.push("SharePoint Framework", false, &m, Some(req), Some(&fix));
                self.status(CheckStatus::Failure, &m)?;
            }
        } else {
            let m = format!("SharePoint Framework v{version}");
            self.push("SharePoint Framework", true, &m, Some(&version), None);
            self.status(CheckStatus::Success, &m)?;
        }
        if let Some(env) = cx.s("env").filter(|s| !s.is_empty()) {
            let sp = matrix::sharepoint_version(&env.to_uppercase())
                .and_then(Value::as_f64)
                .unwrap_or(f64::NAN) as i64;
            let name = matrix::sharepoint_version(&sp.to_string())
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into());
            let have = pre.get("sp").and_then(Value::as_i64).unwrap_or(0);
            if (have & sp) == sp {
                let m = format!("Supported in {name}");
                self.push("env", true, &m, Some(&env), None);
                // `{ check, passed, message, version }`
                if let Some(Value::Object(o)) = self.results.last_mut() {
                    let v = o.shift_remove("version");
                    if let Some(v) = v {
                        o.insert("version".into(), v);
                    }
                }
                self.status(CheckStatus::Success, &m)?;
            } else {
                let fix = format!(
                    "Use SharePoint Framework v{}",
                    if sp == 1 { "1.1" } else { "1.4.1" }
                );
                let m = format!("Not supported in {name}");
                let mut o = Obj::new();
                o.insert("check".into(), Value::from("env"));
                o.insert("passed".into(), Value::Bool(false));
                o.insert("fix".into(), Value::from(fix));
                o.insert("message".into(), Value::from(m.as_str()));
                o.insert("version".into(), Value::from(env.as_str()));
                self.results.push(Value::Object(o));
                self.status(CheckStatus::Failure, &m)?;
                return Err(Error::Thrown(Value::from(format!(
                    "SharePoint Framework v{version} is not supported in {name}"
                ))));
            }
        }
        let node = crate::custom::node_version_public();
        self.check(
            "Node",
            &node,
            s(&pre["node"], "range"),
            s(&pre["node"], "fix"),
        )?;
        self.check_global("yo", "yo", pre.get("yo"))?;
        let gulp = self
            .package("gulp", Search::GlobalOnly, false)
            .unwrap_or_default();
        if !gulp.is_empty() {
            self.push(
                "gulp",
                false,
                "gulp should be removed",
                Some(&gulp),
                Some("npm un -g gulp"),
            );
            self.status(CheckStatus::Failure, "gulp should be removed")?;
        }
        self.check_global("gulp-cli", "gulp-cli", pre.get("gulpCli"))?;
        self.check_global("@rushstack/heft", "@rushstack/heft", pre.get("heft"))?;
        let ts = self
            .package("typescript", Search::LocalOnly, false)
            .unwrap_or_default();
        if !ts.is_empty() {
            let m = format!("typescript v{ts} installed in the project");
            self.push("typescript", false, &m, None, Some("npm un typescript"));
            self.status(CheckStatus::Failure, &m)?;
        } else {
            self.push("typescript", true, "bundled typescript used", None, None);
            self.status(CheckStatus::Success, "bundled typescript used")?;
        }
        self.fixes()
    }
}

pub async fn run(cx: &mut Cx) -> Result<()> {
    let output = cx
        .s("output")
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "text".into());
    let mut d = Doctor {
        cx,
        output: output.clone(),
        results: Vec::new(),
    };
    d.msg(" ")?;
    d.msg("CLI for Microsoft 365 SharePoint Framework doctor")?;
    d.msg("Verifying configuration of your system for working with the SharePoint Framework")?;
    d.msg(" ")?;
    let r = d.run().await;
    let mut err = None;
    if let Err(e) = r {
        d.msg(" ")?;
        d.fixes()?;
        if output == "text" {
            err = Some(e);
        }
    }
    if cx.s("output").as_deref() == Some("json") && !d.results.is_empty() {
        cx.log(Value::Array(d.results))?;
    }
    match err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

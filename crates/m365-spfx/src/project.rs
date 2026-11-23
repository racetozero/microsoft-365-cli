//! `BaseProjectCommand`: the SharePoint Framework project model the rules
//! inspect. JSON files are parsed values with their (comment-stripped)
//! source attached as a `source` property, as upstream does.

use crate::jsonast::{self, Node};
use m365_core::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// A text file read lazily (`TsFile`, `ScssFile`).
#[derive(Debug, Default)]
pub struct SourceFile {
    pub path: String,
    pub source: Option<String>,
}

impl SourceFile {
    fn read(path: &Path) -> SourceFile {
        SourceFile {
            path: path.to_string_lossy().into_owned(),
            source: std::fs::read_to_string(path).ok().filter(|s| !s.is_empty()),
        }
    }
}

#[derive(Debug, Default)]
pub struct VsCode {
    pub settings_json: Option<Value>,
    pub extensions_json: Option<Value>,
    pub launch_json: Option<Value>,
}

#[derive(Debug, Default)]
pub struct Project {
    pub path: String,
    /// Set by `spfx project doctor` (`project.version`).
    pub version: Option<String>,
    pub gitignore: Option<String>,
    pub npmignore: Option<String>,
    pub config_json: Option<Value>,
    pub copy_assets_json: Option<Value>,
    pub deploy_azure_storage_json: Option<Value>,
    pub package_json: Option<Value>,
    pub package_solution_json: Option<Value>,
    pub serve_json: Option<Value>,
    pub ts_config_json: Option<Value>,
    pub ts_lint_json: Option<Value>,
    pub ts_lint_json_root: Option<Value>,
    pub write_manifests_json: Option<Value>,
    pub yo_rc_json: Option<Value>,
    pub sass_json: Option<Value>,
    pub gulpfile_js: Option<String>,
    pub es_lint_rc_js: Option<SourceFile>,
    pub vs_code: VsCode,
    pub manifests: Vec<Value>,
    pub ts_files: Vec<SourceFile>,
    pub scss_files: Vec<SourceFile>,
    asts: RefCell<HashMap<String, Rc<Option<Node>>>>,
    /// Manifest `commands` objects changed in place by a rule (FN011006
    /// rewrites the titles of the object it also reports), by manifest index.
    pub commands: RefCell<HashMap<usize, Value>>,
}

/// `fsUtil.readdirR`: every file below `dir`, in libuv's (byte) order.
pub fn readdir_r(dir: &Path) -> Result<Vec<PathBuf>> {
    let meta = std::fs::metadata(dir)
        .map_err(|e| Error::js(util::node_fs_error(&e, "stat", &dir.to_string_lossy())))?;
    if !meta.is_dir() {
        return Ok(vec![dir.to_path_buf()]);
    }
    let mut names: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| Error::js(util::node_fs_error(&e, "scandir", &dir.to_string_lossy())))?
        .filter_map(|e| e.ok())
        .map(|e| e.file_name())
        .collect();
    names.sort_by(|a, b| a.as_encoded_bytes().cmp(b.as_encoded_bytes()));
    let mut out = Vec::new();
    for n in names {
        out.extend(readdir_r(&dir.join(n))?);
    }
    Ok(out)
}

/// `readAndParseJsonFile`.
fn read_json(path: &Path) -> Result<Option<Value>> {
    if !path.exists() {
        return Ok(None);
    }
    let raw = std::fs::read(path).map_err(|e| Error::msg(e.to_string()))?;
    let source = util::remove_single_line_comments(&String::from_utf8_lossy(&raw));
    match json::js_parse(&source) {
        Ok(mut v) => {
            if let Some(o) = v.as_object_mut() {
                o.insert("source".into(), Value::from(source));
            }
            Ok(Some(v))
        }
        Err(e) => Err(Error::msg(format!(
            "The file {} is not a valid JSON file or is not utf-8 encoded. Error: {e}",
            path.display()
        ))),
    }
}

/// `getProjectRoot`: the nearest folder (upwards) with a `package.json`.
pub fn project_root(folder: &Path) -> Option<PathBuf> {
    let mut p = folder.to_path_buf();
    loop {
        if p.join("package.json").exists() {
            return Some(p);
        }
        if !p.pop() {
            return None;
        }
    }
}

/// `getProjectVersion`.
pub fn project_version(root: &Path) -> Option<String> {
    if let Ok(s) = std::fs::read_to_string(root.join(".yo-rc.json"))
        && let Ok(yo) = serde_json::from_str::<Value>(&s)
        && let Some(g) = yo
            .get("@microsoft/generator-sharepoint")
            .filter(|g| truthy(g))
        && let Some(v) = g.get("version").filter(|v| truthy(v))
    {
        return Some(match g.get("environment").and_then(Value::as_str) {
            Some("onprem19") => "1.4.1".into(),
            Some("onprem") => "1.1.0".into(),
            _ => to_js_string(v),
        });
    }
    let pkg: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("package.json")).ok()?).ok()?;
    let v = pkg
        .get("dependencies")?
        .get("@microsoft/sp-core-library")
        .filter(|v| truthy(v))?;
    Some(
        to_js_string(v)
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect(),
    )
}

impl Project {
    /// `getProject`.
    pub fn load(root: &Path) -> Result<Project> {
        let j = |p: &[&str]| -> Result<Option<Value>> {
            read_json(&p.iter().fold(root.to_path_buf(), |acc, s| acc.join(s)))
        };
        let text = |name: &str| std::fs::read_to_string(root.join(name)).ok();
        let mut p = Project {
            path: root.to_string_lossy().into_owned(),
            gitignore: text(".gitignore"),
            npmignore: text(".npmignore"),
            ..Default::default()
        };
        p.config_json = j(&["config", "config.json"])?;
        p.copy_assets_json = j(&["config", "copy-assets.json"])?;
        p.deploy_azure_storage_json = j(&["config", "deploy-azure-storage.json"])?;
        p.package_json = j(&["package.json"])?;
        p.package_solution_json = j(&["config", "package-solution.json"])?;
        p.serve_json = j(&["config", "serve.json"])?;
        p.ts_config_json = j(&["tsconfig.json"])?;
        p.ts_lint_json = j(&["config", "tslint.json"])?;
        p.ts_lint_json_root = j(&["tslint.json"])?;
        p.write_manifests_json = j(&["config", "write-manifests.json"])?;
        p.yo_rc_json = j(&[".yo-rc.json"])?;
        p.sass_json = j(&["config", "sass.json"])?;
        p.gulpfile_js = text("gulpfile.js");
        let eslintrc = root.join(".eslintrc.js");
        if eslintrc.exists() {
            p.es_lint_rc_js = Some(SourceFile::read(&eslintrc));
        }
        p.vs_code.settings_json = j(&[".vscode", "settings.json"])?;
        p.vs_code.extensions_json = j(&[".vscode", "extensions.json"])?;
        p.vs_code.launch_json = j(&[".vscode", "launch.json"])?;
        let src = readdir_r(&root.join("src"))?;
        for f in src
            .iter()
            .filter(|f| f.to_string_lossy().ends_with(".manifest.json"))
        {
            let raw = std::fs::read_to_string(f).map_err(|e| Error::msg(e.to_string()))?;
            let source = util::remove_single_line_comments(&raw);
            let mut m = json::jparse(source.strip_prefix('\u{feff}').unwrap_or(&source))?;
            if let Some(o) = m.as_object_mut() {
                o.insert("path".into(), Value::from(f.to_string_lossy().as_ref()));
                o.insert("source".into(), Value::from(source));
            }
            p.manifests.push(m);
        }
        for f in &src {
            let s = f.to_string_lossy();
            if s.ends_with(".ts") || s.ends_with(".tsx") {
                p.ts_files.push(SourceFile::read(f));
            } else if s.ends_with(".scss") {
                p.scss_files.push(SourceFile::read(f));
            }
        }
        Ok(p)
    }

    /// json-to-ast of a JSON file's `source`, parsed once per source.
    pub fn ast(&self, file: &Value) -> Rc<Option<Node>> {
        let Some(src) = file
            .get("source")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
        else {
            return Rc::new(None);
        };
        if let Some(n) = self.asts.borrow().get(src) {
            return n.clone();
        }
        let n = Rc::new(jsonast::parse(src).ok());
        self.asts.borrow_mut().insert(src.to_string(), n.clone());
        n
    }

    /// `getAstNodeFromFile(file, property)` as a `{ line, character }`.
    pub fn position(&self, file: &Value, property: &str) -> Result<(usize, usize)> {
        let ast = self.ast(file);
        match ast.as_ref() {
            None => Ok((1, 1)),
            Some(root) => Ok(jsonast::position(
                jsonast::node_for_property(root, property).map_err(Error::js)?,
            )),
        }
    }

    /// A rough `util.inspect(project)` for `--debug`.
    pub fn inspect(&self) -> String {
        format!("{{ path: '{}' }}", self.path)
    }

    /// `path.relative(project.path, file)`.
    pub fn relative(&self, file: &str) -> String {
        relative(&self.path, file)
    }
}

/// Node's `path.relative(from, to)` for absolute paths, with the platform's
/// separator.
pub fn relative(from: &str, to: &str) -> String {
    use std::path::Component;
    let parts = |p: &str| -> Vec<String> {
        Path::new(p)
            .components()
            .filter(|c| matches!(c, Component::Normal(_)))
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .collect()
    };
    let (a, b) = (parts(from), parts(to));
    let common = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let mut out = PathBuf::new();
    for _ in common..a.len() {
        out.push("..");
    }
    for p in &b[common..] {
        out.push(p);
    }
    out.to_string_lossy().into_owned()
}

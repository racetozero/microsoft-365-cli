//! `spfx project rename`, `externalize`, `permissions grant`,
//! `github workflow add` and `azuredevops pipeline add`.

use crate::matrix;
use crate::project::{self, Project};
use crate::upgrade::root;
use crate::yaml;
use m365_core::prelude::*;
use std::path::Path;

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)
        .map_err(|e| Error::js(util::node_fs_error(&e, "open", &path.to_string_lossy())))
}

fn write(path: &Path, s: &str) -> Result<()> {
    std::fs::write(path, s)
        .map_err(|e| Error::js(util::node_fs_error(&e, "open", &path.to_string_lossy())))
}

/// `JSON.parse`, failing like upstream's `throw new CommandError(error)`.
fn parse(s: &str) -> Result<Value> {
    json::jparse(s)
}

// ---- rename ------------------------------------------------------------------------

/// `s.replace(new RegExp(pattern, 'g'), with)`.
fn replace_all_regex(s: &str, pattern: &str, with: &str) -> String {
    match regex::Regex::new(pattern) {
        Ok(re) => re.replace_all(s, regex::NoExpand(with)).into_owned(),
        Err(_) => s.to_string(),
    }
}

fn rewrite(path: &Path, cx: &Cx, f: impl FnOnce(&mut Value)) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let existing = read(path)?;
    let mut v = parse(&existing)?;
    f(&mut v);
    let updated = json::stringify_pretty(&v, "  ");
    if updated != existing {
        write(path, &updated)?;
        cx.debug_log(&format!(
            "Updated {}",
            path.file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default()
        ));
    }
    Ok(())
}

pub async fn rename(cx: &mut Cx) -> Result<()> {
    let root = root()?;
    let pkg = Project::load(&root)?.package_json;
    let project_name = pkg
        .as_ref()
        .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'name')"))?
        .get("name")
        .map(to_js_string)
        .unwrap_or_else(|| "undefined".into());
    let new_name = cx.u("newName");
    let gen_id = cx.bool("generateNewId");
    let mut new_id = String::new();
    if gen_id {
        new_id = util::new_guid();
        cx.debug_log("Created new solution id");
        cx.debug_log(&new_id);
    }
    cx.debug_log(&format!(
        "Renaming SharePoint Framework project to '{new_name}'"
    ));
    let r = (|| -> Result<()> {
        rewrite(&root.join("package.json"), cx, |v| {
            if v.get("name").is_some_and(truthy) {
                v["name"] = Value::from(new_name.as_str());
            }
        })?;
        rewrite(&root.join(".yo-rc.json"), cx, |v| {
            let Some(g) = v
                .get_mut("@microsoft/generator-sharepoint")
                .filter(|g| truthy(g))
            else {
                return;
            };
            for k in ["libraryName", "solutionName"] {
                if g.get(k).is_some_and(truthy) {
                    g[k] = Value::from(new_name.as_str());
                }
            }
            if g.get("libraryId").is_some_and(truthy) && gen_id {
                g["libraryId"] = Value::from(new_id.as_str());
            }
        })?;
        rewrite(
            &root.join("config").join("package-solution.json"),
            cx,
            |v| {
                if let Some(s) = v.get_mut("solution").filter(|s| truthy(s)) {
                    if let Some(n) = s.get("name").filter(|n| truthy(n)).map(to_js_string) {
                        s["name"] = Value::from(replace_all_regex(&n, &project_name, &new_name));
                    }
                    if s.get("id").is_some_and(truthy) && gen_id {
                        s["id"] = Value::from(new_id.as_str());
                    }
                }
                if let Some(p) = v.get_mut("paths").filter(|p| truthy(p))
                    && let Some(z) = p
                        .get("zippedPackage")
                        .filter(|z| truthy(z))
                        .map(to_js_string)
                {
                    p["zippedPackage"] =
                        Value::from(replace_all_regex(&z, &project_name, &new_name));
                }
            },
        )?;
        rewrite(
            &root.join("config").join("deploy-azure-storage.json"),
            cx,
            |v| {
                if v.get("container").is_some_and(truthy) {
                    v["container"] = Value::from(new_name.as_str());
                }
            },
        )?;
        let readme = root.join("README.md");
        if readme.exists() {
            let existing = read(&readme)?;
            let updated = replace_all_regex(&existing, &project_name, &new_name);
            if updated != existing {
                write(&readme, &updated)?;
                cx.debug_log("Updated README.md");
            }
        }
        Ok(())
    })();
    r.map_err(|e| match e {
        Error::Syntax(m) => Error::msg(format!("SyntaxError: {m}")),
        Error::Js(m) => Error::msg(format!("Error: {m}")),
        other => other,
    })
}

// ---- externalize ----------------------------------------------------------------------

const EXTERNALIZE_VERSIONS: [&str; 22] = [
    "1.0.0", "1.0.1", "1.0.2", "1.1.0", "1.1.1", "1.1.3", "1.2.0", "1.3.0", "1.3.1", "1.3.2",
    "1.3.4", "1.4.0", "1.4.1", "1.5.0", "1.5.1", "1.6.0", "1.7.0", "1.7.1", "1.8.0", "1.8.1",
    "1.8.2", "1.9.1",
];

struct Entry {
    key: String,
    path: String,
    global_name: Option<String>,
    global_dependencies: Option<Vec<String>>,
}

struct Edit {
    path: String,
    target: String,
    action: &'static str,
}

/// `PnPJsRule.pnpModules`: key, globalName, globalDependencies, shadowRequire.
const PNP: [(&str, &str, &[&str], Option<&str>); 5] = [
    (
        "@pnp/odata",
        "pnp.odata",
        &["@pnp/common", "@pnp/logging", "tslib"],
        Some("require(\"@pnp/odata\");"),
    ),
    (
        "@pnp/common",
        "pnp.common",
        &[],
        Some("require(\"@pnp/common\");"),
    ),
    (
        "@pnp/logging",
        "pnp.logging",
        &["tslib"],
        Some("require(\"@pnp/logging\");"),
    ),
    (
        "@pnp/sp",
        "pnp.sp",
        &["@pnp/logging", "@pnp/common", "@pnp/odata", "tslib"],
        None,
    ),
    ("@pnp/pnpjs", "pnp", &[], None),
];

fn pnp_rule(p: &Project) -> (Vec<Entry>, Vec<Edit>) {
    let mut found: Vec<Entry> = Vec::new();
    for (key, global, deps, _) in PNP {
        // the recursion into `@${dep.replace('/', '.')}` never matches a module
        if let Some(v) = p
            .package_json
            .as_ref()
            .and_then(|j| j.get("dependencies"))
            .filter(|d| !d.is_null())
            .and_then(|d| d.get(key))
            .filter(|v| truthy(v))
        {
            let bundle = if key == "@pnp/common" { ".bundle" } else { "" };
            found.push(Entry {
                key: key.into(),
                path: format!(
                    "https://unpkg.com/{key}@{}/dist/{}.es5.umd{bundle}.min.js",
                    to_js_string(v),
                    key.replacen("@pnp/", "", 1)
                ),
                global_name: Some(global.into()),
                global_dependencies: if deps.is_empty() {
                    None
                } else {
                    Some(deps.iter().map(|d| d.to_string()).collect())
                },
            });
        }
    }
    let files: Vec<String> = p
        .manifests
        .iter()
        .map(|m| {
            m.get("path")
                .map(to_js_string)
                .unwrap_or_default()
                .replacen(".manifest.json", ".ts", 1)
        })
        .collect();
    let mut edits = Vec::new();
    for (key, _, _, shadow) in PNP {
        if let Some(s) = shadow
            && found.iter().any(|f| f.key == key)
        {
            edits.extend(files.iter().map(|f| Edit {
                action: "add",
                path: f.clone(),
                target: s.into(),
            }));
        }
    }
    if found.iter().any(|f| f.key != "@pnp/pnpjs") {
        found.push(Entry {
            key: "tslib".into(),
            path: "https://unpkg.com/tslib@^1.10.0/tslib.js".into(),
            global_name: Some("tslib".into()),
            global_dependencies: None,
        });
        edits.extend(files.iter().map(|f| Edit {
            action: "add",
            path: f.clone(),
            target: "require(\"tslib\");".into(),
        }));
    }
    (found, edits)
}

const SUFFIXES: [&str; 4] = [".min", ".bundle", "-min", ".bundle.min"];

fn min_suffix(path: &str) -> bool {
    SUFFIXES.iter().any(|s| path.contains(s))
}

/// `DynamicRule`: entries for dependencies whose `node_modules` package
/// points at a browser-loadable file on unpkg.
async fn dynamic_rule(p: &Project) -> Vec<Entry> {
    let Some(deps) = p
        .package_json
        .as_ref()
        .and_then(|j| j.get("dependencies"))
        .filter(|d| truthy(d))
        .and_then(Value::as_object)
    else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (name, version) in deps {
        if name.contains("@types/")
            || name.contains("@microsoft/")
            || [
                "react",
                "react-dom",
                "@pnp/sp-clientsvc",
                "@pnp/sp-taxonomy",
            ]
            .contains(&name.as_str())
        {
            continue;
        }
        let version = to_js_string(version);
        let paths: Vec<String> = package_files(name)
            .into_iter()
            .map(|x| x.replacen("./", "", 1))
            .collect();
        if version.is_empty() || paths.is_empty() {
            continue;
        }
        let mut candidates = paths.clone();
        for x in &paths {
            for s in SUFFIXES {
                candidates.push(if x.contains(s) {
                    x.clone()
                } else {
                    x.replacen(".js", &format!("{s}.js"), 1)
                });
            }
        }
        let mut found = Vec::new();
        for path in candidates {
            if let Some(e) = entry_for_file(name, &version, &path).await {
                found.push(e);
            }
        }
        let pick = |g: bool, m: bool| {
            found
                .iter()
                .position(|e| e.global_name.is_some() == g && min_suffix(&e.path) == m)
        };
        let order = [
            pick(false, true),
            pick(true, true),
            pick(true, false),
            pick(false, false),
        ];
        if let Some(i) = order.into_iter().flatten().next() {
            out.push(found.swap_remove(i));
        }
    }
    out
}

fn package_files(name: &str) -> Vec<String> {
    let Ok(s) = std::fs::read_to_string(format!("node_modules/{name}/package.json")) else {
        return Vec::new();
    };
    let Ok(j) = serde_json::from_str::<Value>(&s) else {
        return Vec::new();
    };
    let mut files: Vec<String> = Vec::new();
    let push = |files: &mut Vec<String>, v: Option<&Value>| {
        if let Some(v) = v.filter(|v| truthy(v)) {
            files.push(to_js_string(v));
        }
    };
    push(&mut files, j.get("module"));
    push(&mut files, j.get("main"));
    push(&mut files, j.get("es2015"));
    if let Some(jspm) = j.get("jspm").filter(|v| truthy(v)) {
        push(&mut files, jspm.get("main"));
        if let Some(f) = jspm.get("files").and_then(Value::as_array) {
            files.extend(f.iter().map(to_js_string));
        }
    }
    if let Some(spm) = j.get("spm").filter(|v| truthy(v)) {
        push(&mut files, spm.get("main"));
    }
    let mut out: Vec<String> = Vec::new();
    for f in files {
        if !out.contains(&f) {
            out.push(f);
        }
    }
    out.into_iter()
        .map(|x| {
            if x.ends_with(".js") {
                x
            } else {
                format!("{x}.js")
            }
        })
        .collect()
}

async fn entry_for_file(name: &str, version: &str, path: &str) -> Option<Entry> {
    let url = format!("https://unpkg.com/{name}@{version}/{path}");
    Request::head(url.clone())
        .anonymous()
        .header("x-anonymous", "true")
        .send()
        .await
        .ok()?;
    let info = Request::post("https://scriptcheck-weu-fn.azurewebsites.net/api/v2/script-check")
        .anonymous()
        .header("content-type", "application/json")
        .accept("application/json")
        .header("x-anonymous", "true")
        .json_response()
        .json(obj! { "url": url.as_str() })
        .data()
        .await
        .unwrap_or_else(|_| obj! { "scriptType": "non-module", "exports": Value::Array(vec![]) });
    match info.get("scriptType").and_then(Value::as_str) {
        Some("CommonJs") => None,
        Some("ES2015" | "AMD") => Some(Entry {
            key: name.into(),
            path: url,
            global_name: None,
            global_dependencies: None,
        }),
        _ => {
            let g = info
                .get("exports")
                .and_then(Value::as_array)
                .and_then(|e| e.first())
                .map(to_js_string)
                .unwrap_or_else(|| name.into());
            Some(Entry {
                key: name.into(),
                path: url,
                global_name: Some(g),
                global_dependencies: None,
            })
        }
    }
}

fn externals(entries: &[Entry]) -> Value {
    let mut o = Obj::new();
    for e in entries {
        let v = match &e.global_name {
            Some(g) if !g.is_empty() => {
                obj! { "path": e.path.as_str(), "globalName": g.as_str(), "globalDependencies": ?e.global_dependencies.clone() }
            }
            _ => Value::from(e.path.as_str()),
        };
        o.insert(e.key.clone(), v);
    }
    obj! { "externals": Value::Object(o) }
}

fn edits_value(edits: &[Edit]) -> Value {
    Value::Array(edits.iter().map(|e| obj! { "action": e.action, "path": e.path.as_str(), "targetValue": e.target.as_str() }).collect())
}

pub async fn externalize(cx: &mut Cx) -> Result<()> {
    let e = crate::report::EOL;
    let output = cx.s("output");
    if output.as_deref() != Some("json") || cx.verbose() {
        cx.stderr(&format!("This command is currently in preview. Feedback welcome at https://github.com/pnp/cli-microsoft365/issues{e}"));
    }
    let root = root()?;
    let Some(version) = project::project_version(&root).filter(|v| !v.is_empty()) else {
        return Err(Error::with_code(
            "Unable to determine the version of the current SharePoint Framework project",
            3,
        ));
    };
    if !EXTERNALIZE_VERSIONS.contains(&version.as_str()) {
        return Err(Error::with_code(
            format!(
                "CLI for Microsoft 365 doesn't support externalizing dependencies of SharePoint Framework projects of version {version}. Supported versions are {}",
                EXTERNALIZE_VERSIONS.join(", ")
            ),
            2,
        ));
    }
    cx.verbose_log("Collecting project...");
    let p = Project::load(&root)?;
    if cx.debug() {
        cx.stderr("Collected project");
        cx.stderr(&p.inspect());
    }
    let (mut entries, edits) = pnp_rule(&p);
    entries.extend(dynamic_rule(&p).await);
    let mut unique: Vec<Entry> = Vec::new();
    for e in entries {
        if !unique.iter().any(|u| u.key == e.key) {
            unique.push(e);
        }
    }
    let ext = externals(&unique);
    match output.as_deref() {
        Some("json") => cx.log(obj! { "externalConfiguration": ext, "edits": edits_value(&edits) }),
        Some("md") => {
            let name = root
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let mut s = format!(
                "# Externalizing dependencies of project {name}{e}{e}Date: {}{e}{e}## Findings{e}{e}### Modify files{e}{e}#### [config.json](config/config.json){e}{e}Replace the externals property (or add if not defined) with{e}{e}```json{e}{}{e}```{e}",
                m365_core::time::locale_date_string(chrono::Local::now()),
                json::stringify_pretty(&ext, "  ")
            );
            for action in ["add", "remove"] {
                let matching: Vec<&Edit> = edits.iter().filter(|e| e.action == action).collect();
                let mut paths: Vec<&str> = Vec::new();
                for e in &matching {
                    if !paths.contains(&e.path.as_str()) {
                        paths.push(&e.path);
                    }
                }
                for path in paths {
                    s.push_str(&format!(
                        "#### [{path}]({path}){e}{action}{e}```JavaScript{e}"
                    ));
                    for edit in matching.iter().filter(|x| x.path == path) {
                        s.push_str(&edit.target);
                        s.push_str(e);
                    }
                    s.push_str(&format!("```{e}"));
                }
            }
            cx.log_raw(&s);
            Ok(())
        }
        _ => {
            let body = json::stringify_pretty(
                &obj! { "externalConfiguration": ext, "edits": edits_value(&edits) },
                "  ",
            );
            cx.log(Value::from(
                format!(
                    "In the config/config.json file update the externals property to:{e}{e}{body}"
                )
                .trim()
                .to_string(),
            ))
        }
    }
}

// ---- permissions grant ----------------------------------------------------------------

pub async fn permissions_grant(cx: &mut Cx) -> Result<()> {
    let root = root()?;
    cx.debug_log("Granting API permissions defined in the current SPFx project");
    let file = root.join("config").join("package-solution.json");
    if !file.exists() {
        return Err(Error::msg(
            "The package-solution.json file could not be found",
        ));
    }
    let content =
        parse(&read(&file)?).map_err(|e| Error::msg(format!("SyntaxError: {}", e.js_message())))?;
    let requests = match content.get("solution") {
        None | Some(Value::Null) => {
            return Err(Error::msg(format!(
                "TypeError: Cannot read properties of {} (reading 'webApiPermissionRequests')",
                if content.get("solution").is_some() {
                    "null"
                } else {
                    "undefined"
                }
            )));
        }
        Some(s) => s.get("webApiPermissionRequests").cloned(),
    };
    let Some(Value::Array(list)) = requests else {
        return Err(Error::msg(
            "TypeError: webApiPermissionsRequest is not iterable",
        ));
    };
    for p in list {
        let mut o = Obj::new();
        o.insert(
            "resource".into(),
            p.get("resource").cloned().unwrap_or(Value::Null),
        );
        o.insert(
            "scope".into(),
            p.get("scope").cloned().unwrap_or(Value::Null),
        );
        o.insert("output".into(), Value::from("json"));
        o.insert("debug".into(), Value::Bool(cx.debug()));
        o.insert("verbose".into(), Value::Bool(cx.verbose()));
        match m365_core::runner::run_nested("spo serviceprincipal grant add", o, true).await {
            Ok((out, _)) => {
                let v = json::jparse(&out)
                    .map_err(|e| Error::msg(format!("SyntaxError: {}", e.js_message())))?;
                cx.log(v)?;
            }
            Err(e) => {
                let msg = e.command_message();
                if to_js_string(&msg).contains("already exists") {
                    cx.warn(&to_js_string(&msg));
                    continue;
                }
                return Err(e);
            }
        }
    }
    Ok(())
}

// ---- github workflow / azure devops pipeline ----------------------------------------------

fn version_requirements(root: &Path) -> Result<&'static Value> {
    let Some(v) = project::project_version(root).filter(|v| !v.is_empty()) else {
        return Err(Error::Thrown(Value::from(
            "Unable to determine the version of the current SharePoint Framework project. Could not find the correct version based on the version property in the .yo-rc.json file.",
        )));
    };
    matrix::version(&v).ok_or_else(|| {
        Error::Thrown(Value::from(format!(
            "Could not find Node version for version '{v}' of SharePoint Framework."
        )))
    })
}

fn read_json_file(path: &Path) -> Result<Option<Value>> {
    if !path.exists() {
        return Ok(None);
    }
    let source = util::remove_single_line_comments(&read(path)?);
    json::js_parse(&source).map(Some).map_err(|e| {
        Error::msg(format!(
            "The file {} is not a valid JSON file or is not utf-8 encoded. Error: {e}",
            path.display()
        ))
    })
}

fn ensure_dir(p: &Path) -> Result<()> {
    if !p.exists() {
        std::fs::create_dir_all(p)
            .map_err(|e| Error::js(util::node_fs_error(&e, "mkdir", &p.to_string_lossy())))?;
    }
    Ok(())
}

const WORKFLOW: &str = r#"{"name":"Deploy Solution {{ name }}","on":{"push":{"branches":["main"]}},"jobs":{"build-and-deploy":{"runs-on":"ubuntu-latest","env":{"NodeVersion":""},"steps":[{"name":"Checkout","uses":"actions/checkout@v6"},{"name":"Use Node.js","uses":"actions/setup-node@v6","with":{"node-version":"${{ env.NodeVersion }}"}},{"name":"Run npm ci","run":"npm ci"},{"name":"Build & Package","run":"npm run build"},{"name":"CLI for Microsoft 365 Login","uses":"pnp/action-cli-login@v4","with":{"CERTIFICATE_ENCODED":"${{ secrets.CERTIFICATE_ENCODED }}","CERTIFICATE_PASSWORD":"${{ secrets.CERTIFICATE_PASSWORD }}","APP_ID":"${{ secrets.APP_ID }}","TENANT":"${{ secrets.TENANT_ID }}"}},{"name":"CLI for Microsoft 365 Deploy App","uses":"pnp/action-cli-deploy@v6","with":{"APP_FILE_PATH":"sharepoint/{{ sppkgPath }}","SKIP_FEATURE_DEPLOYMENT":false,"OVERWRITE":true}}]}}}"#;

pub async fn github_workflow_add(cx: &mut Cx) -> Result<()> {
    let root = root()?;
    let r = (|| -> Result<Value> {
        let pkg = read_json_file(&root.join("package.json"))?;
        let ps = read_json_file(&root.join("config").join("package-solution.json"))?;
        let name = pkg
            .as_ref()
            .ok_or_else(|| Error::js("Cannot read properties of undefined (reading 'name')"))?
            .get("name")
            .map(to_js_string)
            .unwrap_or_else(|| "undefined".into());
        let sppkg = ps
            .as_ref()
            .and_then(|p| p.get("paths"))
            .filter(|v| !v.is_null())
            .and_then(|p| p.get("zippedPackage"))
            .cloned();
        cx.debug_log("Adding GitHub workflow in the current SPFx project");
        let mut w: Value = serde_json::from_str(WORKFLOW).expect("workflow");
        w["name"] = Value::from(
            cx.s("name")
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| s(&w, "name").replacen("{{ name }}", &name, 1)),
        );
        if let Some(b) = cx.s("branchName").filter(|s| !s.is_empty()) {
            w["on"]["push"]["branches"][0] = Value::from(b);
        }
        let req = version_requirements(&root)?;
        let node = matrix::highest_node_version(&to_js_string(&req["node"]["range"]))?;
        w["jobs"]["build-and-deploy"]["env"]["NodeVersion"] = Value::from(node);
        if cx.bool("manuallyTrigger") {
            w["on"]["workflow_dispatch"] = Value::Null;
        }
        fn steps(w: &mut Value) -> &mut Vec<Value> {
            w["jobs"]["build-and-deploy"]["steps"]
                .as_array_mut()
                .expect("steps")
        }
        fn find(w: &mut Value, uses: &str) -> usize {
            steps(w)
                .iter()
                .position(|st| {
                    st.get("uses")
                        .and_then(Value::as_str)
                        .is_some_and(|u| u.contains(uses))
                })
                .expect("step")
        }
        if cx.bool("skipFeatureDeployment") {
            let i = find(&mut w, "action-cli-deploy");
            steps(&mut w)[i]["with"]["SKIP_FEATURE_DEPLOYMENT"] = Value::Bool(true);
        }
        if cx.s("loginMethod").as_deref() == Some("user") {
            let i = find(&mut w, "action-cli-login");
            steps(&mut w)[i]["with"] = obj! { "ADMIN_USERNAME": "${{ secrets.ADMIN_USERNAME }}", "ADMIN_PASSWORD": "${{ secrets.ADMIN_PASSWORD }}" };
        }
        if cx.s("scope").as_deref() == Some("sitecollection") {
            let i = find(&mut w, "action-cli-deploy");
            let with = &mut steps(&mut w)[i]["with"];
            with["SCOPE"] = Value::from("sitecollection");
            match cx.val("siteUrl") {
                Some(u) => with["SITE_COLLECTION_URL"] = u.clone(),
                None => with["SITE_COLLECTION_URL"] = Value::from(json::UNDEFINED),
            }
        }
        if let Some(p) = sppkg.filter(truthy) {
            let i = find(&mut w, "action-cli-deploy");
            let with = &mut steps(&mut w)[i]["with"];
            with["APP_FILE_PATH"] = Value::from(s(with, "APP_FILE_PATH").replacen(
                "{{ sppkgPath }}",
                &to_js_string(&p),
                1,
            ));
        }
        if req.get("heft").is_none() {
            let st = steps(&mut w)
                .iter_mut()
                .find(|st| {
                    st.get("run").is_some_and(truthy)
                        && st.get("name").and_then(Value::as_str) == Some("Build & Package")
                })
                .expect("build step");
            st["run"] = Value::from("gulp bundle --ship\ngulp package-solution --ship\n");
            st["name"] = Value::from("Bundle & Package");
        }
        Ok(w)
    })();
    let mut w = r?;
    strip_undefined(&mut w);
    let dir = root.join(".github");
    ensure_dir(&dir)?;
    let dir = dir.join("workflows");
    ensure_dir(&dir)?;
    write(
        &dir.join("deploy-spfx-solution.yml"),
        &yaml::stringify_doc(&w),
    )
}

fn strip_undefined(v: &mut Value) {
    match v {
        Value::Object(o) => {
            o.retain(|_, x| x.as_str() != Some(json::UNDEFINED));
            o.values_mut().for_each(strip_undefined);
        }
        Value::Array(a) => a.iter_mut().for_each(strip_undefined),
        _ => {}
    }
}

const PIPELINE: &str = r#"{"name":"Deploy Solution","trigger":{"branches":{"include":["main"]}},"pool":{"vmImage":"ubuntu-latest"},"variables":[{"name":"CertificateBase64Encoded","value":""},{"name":"CertificateSecureFileId","value":""},{"name":"CertificatePassword","value":""},{"name":"EntraAppId","value":""},{"name":"UserName","value":""},{"name":"Password","value":""},{"name":"TenantId","value":""},{"name":"SharePointBaseUrl","value":""},{"name":"PackageName","value":""},{"name":"SppkgPath","value":""},{"name":"SiteAppCatalogUrl","value":""},{"name":"NodeVersion","value":""}],"stages":[{"stage":"Build_and_Deploy","jobs":[{"job":"Build_and_Deploy","steps":[{"task":"NodeTool@0","displayName":"Use Node.js","inputs":{"versionSpec":"$(NodeVersion)"}},{"task":"Npm@1","displayName":"Run npm install","inputs":{"command":"install"}},{"task":"Npm@1","displayName":"Build and package","inputs":{"command":"custom","verbose":false,"customCommand":"run build"}},{"task":"Npm@1","displayName":"Install CLI for Microsoft 365","inputs":{"command":"custom","verbose":false,"customCommand":"install -g @pnp/cli-microsoft365"}},{"script":"\n{{login}} \nm365 spo set --url '$(SharePointBaseUrl)' \n{{addApp}} \n{{deploy}}\n","displayName":"CLI for Microsoft 365 Deploy App"}]}]}]}"#;

pub async fn azuredevops_pipeline_add(cx: &mut Cx) -> Result<()> {
    let root = root()?;
    cx.debug_log("Adding Azure DevOps pipeline in the current SPFx project");
    let mut p: Value = serde_json::from_str(PIPELINE).expect("pipeline");
    let ps = read_json_file(&root.join("config").join("package-solution.json"))?;
    let sppkg = ps
        .as_ref()
        .and_then(|p| p.get("paths"))
        .filter(|v| !v.is_null())
        .and_then(|p| p.get("zippedPackage"))
        .cloned();
    match cx.s("name").filter(|s| !s.is_empty()) {
        Some(n) => p["name"] = Value::from(n),
        None => {
            p.as_object_mut().expect("pipeline").shift_remove("name");
        }
    }
    if let Some(b) = cx.s("branchName").filter(|s| !s.is_empty()) {
        p["trigger"]["branches"]["include"][0] = Value::from(b);
    }
    let req = version_requirements(&root)?;
    let node = matrix::highest_node_version(&to_js_string(&req["node"]["range"]))?;
    let assign = |p: &mut Value, name: &str, value: Value| {
        if let Some(v) = p["variables"]
            .as_array_mut()
            .expect("variables")
            .iter_mut()
            .find(|v| v.get("name").and_then(Value::as_str) == Some(name))
        {
            v["value"] = value;
        }
    };
    let drop_vars = |p: &mut Value, names: &[&str]| {
        p["variables"]
            .as_array_mut()
            .expect("variables")
            .retain(|v| !names.contains(&s(v, "name")))
    };
    assign(&mut p, "NodeVersion", Value::from(node));
    fn steps(p: &mut Value) -> &mut Vec<Value> {
        p["stages"][0]["jobs"][0]["steps"]
            .as_array_mut()
            .expect("steps")
    }
    let si = steps(&mut p)
        .iter()
        .position(|st| st.get("script").is_some_and(truthy))
        .expect("script");
    let mut script = s(&steps(&mut p)[si], "script").to_string();
    if cx.s("loginMethod").as_deref() == Some("user") {
        script = script.replacen(
            "{{login}}",
            "m365 login --authType password --userName '$(UserName)' --password '$(Password)'",
            1,
        );
        drop_vars(
            &mut p,
            &[
                "CertificateBase64Encoded",
                "CertificateSecureFileId",
                "CertificatePassword",
                "EntraAppId",
                "TenantId",
            ],
        );
    } else {
        script = script.replacen("{{login}}", "m365 login --authType certificate --certificateBase64Encoded '$(CertificateBase64Encoded)' --password '$(CertificatePassword)' --appId '$(EntraAppId)' --tenant '$(TenantId)'", 1);
        drop_vars(&mut p, &["UserName", "Password"]);
    }
    if cx.s("scope").as_deref() == Some("sitecollection") {
        script = script.replacen("{{deploy}}", "m365 spo app deploy --name '$(PackageName)' --appCatalogScope sitecollection --appCatalogUrl '$(SiteAppCatalogUrl)'", 1);
        script = script.replacen("{{addApp}}", "m365 spo app add --filePath '$(Build.SourcesDirectory)/sharepoint/$(SppkgPath)' --appCatalogScope sitecollection --appCatalogUrl '$(SiteAppCatalogUrl)' --overwrite", 1);
        assign(
            &mut p,
            "SiteAppCatalogUrl",
            cx.val("siteUrl")
                .cloned()
                .unwrap_or(Value::from(json::UNDEFINED)),
        );
    } else {
        script = script.replacen(
            "{{deploy}}",
            "m365 spo app deploy --name '$(PackageName)' --appCatalogScope 'tenant'",
            1,
        );
        script = script.replacen("{{addApp}}", "m365 spo app add --filePath '$(Build.SourcesDirectory)/sharepoint/$(SppkgPath)' --overwrite", 1);
        drop_vars(&mut p, &["SiteAppCatalogUrl"]);
    }
    if let Some(sp) = sppkg.filter(truthy) {
        let sp = to_js_string(&sp);
        assign(&mut p, "SppkgPath", Value::from(sp.as_str()));
        assign(
            &mut p,
            "PackageName",
            Value::from(sp.rsplit('/').next().unwrap_or(&sp)),
        );
    }
    if cx.bool("skipFeatureDeployment") {
        script = script.replacen(
            "m365 spo app deploy ",
            "m365 spo app deploy --skipFeatureDeployment ",
            1,
        );
    }
    steps(&mut p)[si]["script"] = Value::from(script);
    if req.get("heft").is_none() {
        steps(&mut p).retain(|st| {
            !(st.get("task").is_some_and(truthy)
                && st.get("displayName").and_then(Value::as_str) == Some("Build and package"))
        });
        let i = steps(&mut p).iter().position(|st| {
            st.get("task").and_then(Value::as_str) == Some("Npm@1")
                && st.get("displayName").and_then(Value::as_str)
                    == Some("Install CLI for Microsoft 365")
        });
        let bundle = obj! { "task": "Gulp@0", "displayName": "Gulp bundle", "inputs": obj! { "gulpFile": "./gulpfile.js", "targets": "bundle", "arguments": "--ship" } };
        let package = obj! { "task": "Gulp@0", "displayName": "Gulp package", "inputs": obj! { "targets": "package-solution", "arguments": "--ship" } };
        let st = steps(&mut p);
        // `splice(-1, 0, ...)` inserts before the last step
        let at = match i {
            Some(i) => i,
            None => st.len().saturating_sub(1),
        };
        st.insert(at, package);
        st.insert(at, bundle);
    }
    strip_undefined(&mut p);
    let dir = root.join(".azuredevops");
    ensure_dir(&dir)?;
    let dir = dir.join("pipelines");
    ensure_dir(&dir)?;
    write(
        &dir.join("deploy-spfx-solution.yml"),
        &yaml::stringify_doc(&p),
    )
}

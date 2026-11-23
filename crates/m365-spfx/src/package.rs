//! `spfx package generate`: a `.sppkg` wrapping an HTML snippet in a
//! client-side web part, from upstream's asset template.

use m365_core::prelude::*;
use std::path::{Path, PathBuf};

/// The template folder (`package-generate/assets`), relative paths with
/// `$token$` placeholders.
const ASSETS: [(&str, &[u8]); 14] = [
    (
        "$webPartId$/WebPart_$webPartId$.xml",
        include_bytes!("package-assets/$webPartId$/WebPart_$webPartId$.xml"),
    ),
    (
        "AppManifest.xml",
        include_bytes!("package-assets/AppManifest.xml"),
    ),
    (
        "ClientSideAssets.xml",
        include_bytes!("package-assets/ClientSideAssets.xml"),
    ),
    (
        "ClientSideAssets.xml.config.xml",
        include_bytes!("package-assets/ClientSideAssets.xml.config.xml"),
    ),
    (
        "ClientSideAssets/$webPartId$_color.png",
        include_bytes!("package-assets/ClientSideAssets/$webPartId$_color.png"),
    ),
    (
        "ClientSideAssets/$webPartId$_outline.png",
        include_bytes!("package-assets/ClientSideAssets/$webPartId$_outline.png"),
    ),
    (
        "ClientSideAssets/$webPartModule$_b69cad1599a6969840f0.js",
        include_bytes!("package-assets/ClientSideAssets/$webPartModule$_b69cad1599a6969840f0.js"),
    ),
    (
        "[Content_Types].xml",
        include_bytes!("package-assets/[Content_Types].xml"),
    ),
    ("_rels/.rels", include_bytes!("package-assets/_rels/.rels")),
    (
        "_rels/AppManifest.xml.rels",
        include_bytes!("package-assets/_rels/AppManifest.xml.rels"),
    ),
    (
        "_rels/ClientSideAssets.xml.rels",
        include_bytes!("package-assets/_rels/ClientSideAssets.xml.rels"),
    ),
    (
        "_rels/feature_$webPartId$.xml.rels",
        include_bytes!("package-assets/_rels/feature_$webPartId$.xml.rels"),
    ),
    (
        "feature_$webPartId$.xml",
        include_bytes!("package-assets/feature_$webPartId$.xml"),
    ),
    (
        "feature_$webPartId$.xml.config.xml",
        include_bytes!("package-assets/feature_$webPartId$.xml.config.xml"),
    ),
];

fn replace_tokens(s: &str, tokens: &Obj) -> String {
    // `/\$([^$]+)\$/g`: unknown or falsy tokens stay as they are
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find('$') {
        let after = &rest[i + 1..];
        match after.find('$') {
            Some(j) if j > 0 => {
                let token = &after[..j];
                out.push_str(&rest[..i]);
                match tokens.get(token).filter(|v| truthy(v)) {
                    Some(v) => out.push_str(&to_js_string(v)),
                    None => out.push_str(&rest[i..i + j + 2]),
                }
                rest = &after[j + 1..];
            }
            _ => {
                out.push_str(&rest[..i + 1]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

fn safe_package_name(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect()
}

/// `fs.mkdtempSync(path.join(os.tmpdir(), 'cli-spfx'))`.
fn mkdtemp() -> Result<PathBuf> {
    let base = std::env::temp_dir();
    for _ in 0..100 {
        let suffix: String = util::new_guid().replace('-', "").chars().take(6).collect();
        let p = base.join(format!("cli-spfx{suffix}"));
        if std::fs::create_dir(&p).is_ok() {
            return Ok(p);
        }
    }
    Err(Error::msg("Unable to create a temp folder"))
}

pub async fn generate(cx: &mut Cx) -> Result<()> {
    let title = cx.u("webPartTitle");
    let teams = cx.s("enableForTeams");
    let mut hosts = vec!["SharePointWebPart"];
    if matches!(teams.as_deref(), Some("tab" | "all")) {
        hosts.push("TeamsTab");
    }
    if matches!(teams.as_deref(), Some("personalApp" | "all")) {
        hosts.push("TeamsPersonalApp");
    }
    let or = |k: &str, d: &str| {
        cx.s(k)
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| d.to_string())
    };
    let alias: String = title.chars().filter(char::is_ascii_alphanumeric).collect();
    let tokens = obj! {
        "clientSideAssetsFeatureId": util::new_guid(),
        "developerName": or("developerName", "Contoso"),
        "developerWebsiteUrl": or("developerWebsiteUrl", "https://contoso.com/my-app"),
        "developerPrivacyUrl": or("developerPrivacyUrl", "https://contoso.com/privacy"),
        "developerTermsOfUseUrl": or("developerTermsOfUseUrl", "https://contoso.com/terms-of-use"),
        "developerMpnId": or("developerMpnId", "000000"),
        "exposePageContextGlobally": if cx.bool("exposePageContextGlobally") { "!0" } else { "!1" },
        "exposeTeamsContextGlobally": if cx.bool("exposeTeamsContextGlobally") { "!0" } else { "!1" },
        "html": cx.u("html").replace('"', "\\\"").replace("\r\n", " ").replace('\n', " "),
        "packageName": safe_package_name(&title),
        "productId": util::new_guid(),
        "skipFeatureDeployment": (cx.val("allowTenantWideDeployment") == Some(&Value::Bool(true))).to_string(),
        "supportedHosts": stringify(&Value::from(hosts.clone())).replace('"', "&quot;"),
        "webPartId": util::new_guid(),
        "webPartFeatureName": format!("{title} Feature"),
        "webPartFeatureDescription": format!("A feature which activates the Client-Side WebPart named {title}"),
        "webPartAlias": format!("AutoWP{}", alias.chars().take(40).collect::<String>()),
        "webPartName": title.as_str(),
        "webPartSafeName": title.replace(' ', "-"),
        "webPartDescription": cx.u("webPartDescription"),
        "webPartModule": safe_package_name(&title),
    };
    let tokens = tokens.as_object().cloned().unwrap_or_default();
    let log = |s: &str| cx.log(Value::from(s));
    if cx.verbose() {
        log("Creating temp folder...")?;
    }
    let tmp = mkdtemp()?;
    let result = (|| -> Result<()> {
        let tmp_s = tmp.to_string_lossy().into_owned();
        if cx.debug() {
            log(&format!("Temp folder created at {tmp_s}"))?;
        }
        if cx.verbose() {
            log("Copying files...")?;
        }
        for (rel, bytes) in ASSETS {
            let dest = replace_tokens(&format!("{tmp_s}/{rel}"), &tokens);
            let dest = Path::new(&dest);
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).map_err(|e| Error::msg(e.to_string()))?;
            }
            std::fs::write(dest, bytes).map_err(|e| Error::msg(e.to_string()))?;
        }
        let files = crate::project::readdir_r(&tmp)?;
        if cx.verbose() {
            log("Processing files...")?;
        }
        for f in &files {
            let fs = f.to_string_lossy();
            if cx.debug() {
                log(&format!("Processing {fs}..."))?;
            }
            if fs.ends_with(".png") {
                if cx.verbose() {
                    log("Binary file. Skipping replacing tokens in contents")?;
                }
                continue;
            }
            if cx.verbose() {
                log("Replacing tokens...")?;
            }
            let contents = std::fs::read_to_string(f).map_err(|e| Error::msg(e.to_string()))?;
            if cx.debug() {
                log("Before:")?;
                log(&contents)?;
            }
            let contents = replace_tokens(&contents, &tokens);
            if cx.debug() {
                log("After:")?;
                log(&contents)?;
            }
            std::fs::write(f, contents).map_err(|e| Error::msg(e.to_string()))?;
        }
        if cx.verbose() {
            log("Creating .sppkg file...")?;
        }
        let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
        for f in crate::project::readdir_r(&tmp)? {
            if cx.debug() {
                log(&format!("Adding {} to archive...", f.to_string_lossy()))?;
            }
            let rel = crate::project::relative(&tmp_s, &f.to_string_lossy());
            entries.push((
                rel,
                std::fs::read(&f).map_err(|e| Error::msg(e.to_string()))?,
            ));
        }
        if cx.debug() {
            log("Writing archive...")?;
        }
        // adm-zip orders entries by `localeCompare` of the lowercased names
        entries.sort_by(|a, b| util::locale_compare(&a.0.to_lowercase(), &b.0.to_lowercase()));
        let refs: Vec<(&str, &[u8])> = entries
            .iter()
            .map(|(n, b)| (n.as_str(), b.as_slice()))
            .collect();
        let zip = m365_core::zip::write(&refs);
        let name = format!("{}.sppkg", cx.u("name"));
        std::fs::write(&name, zip).map_err(|e| Error::msg(e.to_string()))?;
        Ok(())
    })();
    if cx.verbose() {
        log(&format!(
            "Deleting temp folder at {}...",
            tmp.to_string_lossy()
        ))?;
    }
    // upstream calls `fs.rmdirSync(tmpDir, { recursive: true })`; Node.js 25+
    // dropped `recursive` there, so the non-empty folder stays and the
    // command fails after writing the package
    if std::fs::remove_dir(&tmp).is_err() && result.is_ok() {
        return Err(Error::Thrown(Value::from(format!(
            "An error has occurred while removing the temp folder at {}. Please remove it manually.",
            tmp.to_string_lossy()
        ))));
    }
    result
}

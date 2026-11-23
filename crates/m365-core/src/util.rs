//! Shared helpers ported from upstream `src/utils/formatting.ts` and friends.

use crate::json::{Obj, keys, to_js_string};
use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
use serde_json::Value;

/// Characters `encodeURIComponent` leaves alone: `A-Z a-z 0-9 - _ . ! ~ * ' ( )`.
pub const URI_COMPONENT: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'!')
    .remove(b'~')
    .remove(b'*')
    .remove(b'\'')
    .remove(b'(')
    .remove(b')');

/// Characters `encodeURI` leaves alone (also `; , / ? : @ & = + $ #`).
pub const URI: &AsciiSet = &URI_COMPONENT
    .remove(b';')
    .remove(b',')
    .remove(b'/')
    .remove(b'?')
    .remove(b':')
    .remove(b'@')
    .remove(b'&')
    .remove(b'=')
    .remove(b'+')
    .remove(b'$')
    .remove(b'#');

/// `encodeURIComponent`.
pub fn enc(s: &str) -> String {
    utf8_percent_encode(s, URI_COMPONENT).to_string()
}

/// `encodeURI`.
pub fn enc_uri(s: &str) -> String {
    utf8_percent_encode(s, URI).to_string()
}

/// `decodeURIComponent` (lossy on invalid sequences).
pub fn dec(s: &str) -> String {
    percent_encoding::percent_decode_str(s)
        .decode_utf8_lossy()
        .into_owned()
}

/// `formatting.encodeQueryParameter`: `encodeURIComponent` with `'` doubled.
pub fn encode_query_parameter(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    enc(s).replace('\'', "''")
}

/// `formatting.escapeXml`.
pub fn escape_xml(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            c => out.push(c),
        }
    }
    out
}

/// `formatting.escapeXml` on any value (`undefined`/falsy values pass through).
pub fn escape_xml_value(v: Option<&Value>) -> String {
    match v {
        None => "undefined".into(),
        Some(v) if !crate::json::truthy(v) => to_js_string(v),
        Some(v) => escape_xml(&to_js_string(v)),
    }
}

/// `formatting.filterObject`.
pub fn filter_object(obj: &Obj, props: &[&str]) -> Obj {
    let mut out = Obj::new();
    for p in props {
        if let Some(v) = obj.get(*p) {
            out.insert((*p).to_string(), v.clone());
        }
    }
    out
}

/// `formatting.splitAndTrim`.
pub fn split_and_trim(s: &str) -> Vec<String> {
    s.split(',').map(|c| c.trim().to_string()).collect()
}

/// `formatting.openTypesEncoder`.
pub fn open_types_encoder(s: &str) -> String {
    s.replace('%', "%25")
        .replace('.', "%2E")
        .replace(':', "%3A")
        .replace('@', "%40")
        .replace('#', "%23")
}

/// `formatting.rewriteBooleanValue`.
pub fn rewrite_boolean_value(s: &str) -> String {
    crate::args::rewrite_boolean(s)
}

/// `formatting.extractCsomGuid`.
pub fn extract_csom_guid(s: &str) -> String {
    static RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"(?i)/Guid\(([0-9a-f-]+)\)/").unwrap());
    RE.captures(s)
        .map(|c| c[1].to_string())
        .unwrap_or_else(|| s.to_string())
}

/// `formatting.extractCsomDate`.
pub fn extract_csom_date(s: &str) -> String {
    static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?i)^\\?/Date\((\d+(?:,\d+){2,6})\)\\?/$").unwrap()
    });
    let Some(c) = RE.captures(s.trim()) else {
        return s.to_string();
    };
    let p: Vec<i64> = c[1].split(',').map(|x| x.parse().unwrap_or(0)).collect();
    let get = |i: usize| p.get(i).copied().unwrap_or(0);
    let base = chrono::NaiveDate::from_ymd_opt(get(0) as i32, 1, 1)
        .unwrap_or_default()
        .and_hms_opt(0, 0, 0)
        .unwrap_or_default();
    let base = chrono::TimeZone::from_utc_datetime(&chrono::Utc, &base);
    let d = crate::time::add_months(base, get(1) as i32)
        + chrono::Duration::days(get(2) - 1)
        + chrono::Duration::hours(get(3))
        + chrono::Duration::minutes(get(4))
        + chrono::Duration::seconds(get(5))
        + chrono::Duration::milliseconds(get(6));
    crate::time::to_iso(d)
}

/// `formatting.objectToXml`.
pub fn object_to_xml(v: &Value) -> String {
    let mut xml = String::new();
    if let Value::Object(o) = v {
        for k in keys(o) {
            xml.push_str(&format!("<{k}>"));
            match &o[k.as_str()] {
                Value::Array(a) => {
                    for item in a {
                        xml.push_str(&object_to_xml(item));
                    }
                }
                other => xml.push_str(&to_js_string(other)),
            }
            xml.push_str(&format!("</{k}>"));
        }
    }
    static RE: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new(r"</?[0-9]{1,}>").unwrap());
    RE.replace_all(&xml, "").into_owned()
}

/// `formatting.convertArrayToHashTable`.
pub fn array_to_hash_table(key: &str, arr: &[Value]) -> Obj {
    let mut o = Obj::new();
    for v in arr {
        o.insert(
            v.get(key)
                .map(to_js_string)
                .unwrap_or_else(|| "undefined".into()),
            v.clone(),
        );
    }
    o
}

/// `stripJsonComments` (comments become spaces, as the package does).
pub fn remove_single_line_comments(s: &str) -> String {
    let b: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let mut in_str = false;
    while i < b.len() {
        let c = b[i];
        if in_str {
            out.push(c);
            if c == '\\' && i + 1 < b.len() {
                out.push(b[i + 1]);
                i += 2;
                continue;
            }
            if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_str = true;
            out.push(c);
            i += 1;
        } else if c == '/' && b.get(i + 1) == Some(&'/') {
            while i < b.len() && b[i] != '\n' && !(b[i] == '\r' && b.get(i + 1) == Some(&'\n')) {
                out.push(' ');
                i += 1;
            }
        } else if c == '/' && b.get(i + 1) == Some(&'*') {
            out.push_str("  ");
            i += 2;
            while i < b.len() && !(b[i] == '*' && b.get(i + 1) == Some(&'/')) {
                out.push(if b[i].is_whitespace() { b[i] } else { ' ' });
                i += 1;
            }
            if i < b.len() {
                out.push_str("  ");
                i += 2;
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

/// `formatting.parseCsvToJson` (csv-parse with columns, trimming leading
/// whitespace and casting numbers).
pub fn parse_csv_to_json(s: &str, quote: char, delimiter: char, cast: bool) -> Vec<Value> {
    let s = s.strip_prefix('\u{feff}').unwrap_or(s);
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut in_q = false;
    let mut at_start = true;
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if in_q {
            if c == quote {
                if chars.get(i + 1) == Some(&quote) {
                    field.push(quote);
                    i += 2;
                    continue;
                }
                in_q = false;
            } else {
                field.push(c);
            }
            i += 1;
            continue;
        }
        if at_start && (c == ' ' || c == '\t') {
            i += 1;
            continue;
        }
        if c == quote && at_start {
            in_q = true;
            at_start = false;
        } else if c == delimiter {
            row.push(std::mem::take(&mut field));
            at_start = true;
        } else if c == '\n' || c == '\r' {
            if c == '\r' && chars.get(i + 1) == Some(&'\n') {
                i += 1;
            }
            row.push(std::mem::take(&mut field));
            if !(row.len() == 1 && row[0].is_empty()) {
                rows.push(std::mem::take(&mut row));
            } else {
                row.clear();
            }
            at_start = true;
        } else {
            field.push(c);
            at_start = false;
        }
        i += 1;
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        if !(row.len() == 1 && row[0].is_empty()) {
            rows.push(row);
        }
    }
    let Some(header) = rows.first().cloned() else {
        return Vec::new();
    };
    rows[1..]
        .iter()
        .map(|r| {
            let mut o = Obj::new();
            for (i, h) in header.iter().enumerate() {
                let v = r.get(i).cloned().unwrap_or_default();
                let val = if cast && !v.is_empty() && crate::args::looks_like_number(&v) {
                    crate::json::num_value(crate::json::js_number(&v))
                } else {
                    Value::String(v)
                };
                o.insert(h.clone(), val);
            }
            Value::Object(o)
        })
        .collect()
}

#[derive(Clone, Copy)]
pub enum CheckStatus {
    Success,
    Failure,
    Information,
    Warning,
}

/// `formatting.getStatus`.
pub fn status(result: CheckStatus, message: &str) -> String {
    let primary = !cfg!(windows)
        || std::env::var("CI").is_ok_and(|v| v == "true")
        || std::env::var("TERM").is_ok_and(|v| v == "xterm-256color");
    let (success, failure) = if primary {
        ("✔", "✖")
    } else {
        ("√", "×")
    };
    match result {
        CheckStatus::Success => format!("{} {message}", crate::term::green(success)),
        CheckStatus::Failure => format!("{} {message}", crate::term::red(failure)),
        CheckStatus::Information => format!("{} {message}", crate::term::blue("i")),
        CheckStatus::Warning => format!("{} {message}", crate::term::yellow("!")),
    }
}

/// Opening URLs from WSL in the Windows browser.
#[cfg_attr(not(all(unix, not(target_os = "macos"))), allow(dead_code))]
mod wsl {
    use base64::Engine;
    use std::path::Path;

    /// WSL is the Linux kernel whose release names Microsoft (`is-wsl`).
    pub fn is_wsl_kernel(release: &str) -> bool {
        release.to_lowercase().contains("microsoft")
    }

    /// Inside WSL, not in a container on it, and not over SSH, where the
    /// Windows desktop may not be the user's.
    pub fn active() -> bool {
        let read = |p: &str| std::fs::read_to_string(p).unwrap_or_default();
        let kernel = is_wsl_kernel(&read("/proc/sys/kernel/osrelease"))
            || is_wsl_kernel(&read("/proc/version"));
        let container = Path::new("/.dockerenv").exists()
            || Path::new("/run/.containerenv").exists()
            || read("/proc/self/cgroup").contains("docker");
        let ssh = std::env::var_os("SSH_CONNECTION").is_some()
            || std::env::var_os("SSH_CLIENT").is_some();
        kernel && !container && !ssh
    }

    /// `Start '<url>'` for `powershell -EncodedCommand` (Base64 of UTF-16LE),
    /// so `&` and quotes in sign-in URLs need no shell quoting.
    pub fn encoded_start(url: &str) -> String {
        let script = format!(
            "$ProgressPreference = 'SilentlyContinue'; Start '{}'",
            url.replace('\'', "''")
        );
        let bytes: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    /// Runs PowerShell from PATH (WSL interop) or the default C: mount; None
    /// when neither starts.
    pub fn open(url: &str) -> Option<std::process::ExitStatus> {
        let encoded = encoded_start(url);
        let fallback = "/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe";
        for exe in ["powershell.exe", fallback] {
            let mut cmd = std::process::Command::new(exe);
            cmd.args([
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
            ])
            .args(["-EncodedCommand", &encoded])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
            // a Linux working directory becomes a \\wsl.localhost path that
            // Windows may not be able to enter; the Windows drive always works
            if Path::new("/mnt/c").is_dir() {
                cmd.current_dir("/mnt/c");
            }
            if let Ok(status) = cmd.status() {
                return Some(status);
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn detects_wsl_kernels() {
            assert!(is_wsl_kernel("5.15.167.4-microsoft-standard-WSL2"));
            assert!(is_wsl_kernel("4.4.0-19041-Microsoft"));
            assert!(!is_wsl_kernel("6.8.0-45-generic"));
        }

        #[test]
        fn encodes_the_url_for_powershell() {
            let url = "https://login.microsoftonline.com/x?a=1&b='2'";
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded_start(url))
                .unwrap();
            let units: Vec<u16> = bytes
                .chunks(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            assert_eq!(
                String::from_utf16(&units).unwrap(),
                "$ProgressPreference = 'SilentlyContinue'; Start 'https://login.microsoftonline.com/x?a=1&b=''2'''"
            );
        }
    }
}

/// `browserUtil.open`.
pub fn open_browser(url: &str) -> std::io::Result<()> {
    // parity runs (and anyone setting M365_NO_BROWSER) never launch a browser
    if cfg!(feature = "parity") && std::env::var_os("M365_PARITY_MOCK").is_some()
        || std::env::var_os("M365_NO_BROWSER").is_some()
    {
        let _ = url;
        return Ok(());
    }
    #[cfg(target_os = "macos")]
    let mut cmd = {
        let mut c = std::process::Command::new("open");
        c.arg(url);
        c
    };
    #[cfg(windows)]
    let mut cmd = {
        let mut c = std::process::Command::new("cmd");
        c.args(["/c", "start", "", url]);
        c
    };
    // WSL: the Windows browser, as the `open` package does (xdg-open is
    // rarely installed there); fall back to xdg-open when PowerShell fails
    #[cfg(all(unix, not(target_os = "macos")))]
    if wsl::active() && wsl::open(url).is_some_and(|s| s.success()) {
        return Ok(());
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut cmd = {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(url);
        c
    };
    cmd.stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("failed to open the browser"))
    }
}

/// `clipboardy.writeSync`.
pub fn copy_to_clipboard(text: &str) {
    if cfg!(feature = "parity") && std::env::var_os("M365_PARITY_MOCK").is_some() {
        return;
    }
    use std::io::Write;
    let candidates: &[(&str, &[&str])] = if cfg!(target_os = "macos") {
        &[("pbcopy", &[])]
    } else if cfg!(windows) {
        &[("clip", &[])]
    } else {
        &[
            ("wl-copy", &[]),
            ("xclip", &["-selection", "clipboard"]),
            ("xsel", &["--clipboard", "--input"]),
        ]
    };
    for (cmd, args) in candidates {
        if let Ok(mut child) = std::process::Command::new(cmd)
            .args(*args)
            .stdin(std::process::Stdio::piped())
            .spawn()
        {
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
            return;
        }
    }
}

/// A new random GUID.
pub fn new_guid() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helpers() {
        assert_eq!(enc("a b/c'"), "a%20b%2Fc'");
        assert_eq!(encode_query_parameter("o'neil"), "o''neil");
        assert_eq!(
            extract_csom_date("/Date(2026,8,21,12,24,45,997)/"),
            "2026-09-21T12:24:45.997Z"
        );
        assert_eq!(
            extract_csom_guid("/Guid(eae15efb-ac09-49b9-8906-e579efd622e4)/"),
            "eae15efb-ac09-49b9-8906-e579efd622e4"
        );
        let rows = parse_csv_to_json("a,b\n1, x\n", '"', ',', true);
        assert_eq!(rows[0]["a"], serde_json::json!(1));
        assert_eq!(rows[0]["b"], serde_json::json!("x"));
    }
}

/// Node's message for a failed `fs` call, e.g.
/// `ENOENT: no such file or directory, open 'x'`.
pub fn node_fs_error(e: &std::io::Error, syscall: &str, path: &str) -> String {
    let (code, text) = match e.kind() {
        std::io::ErrorKind::NotFound => ("ENOENT", "no such file or directory"),
        std::io::ErrorKind::PermissionDenied => ("EACCES", "permission denied"),
        std::io::ErrorKind::AlreadyExists => ("EEXIST", "file already exists"),
        std::io::ErrorKind::IsADirectory => ("EISDIR", "illegal operation on a directory"),
        std::io::ErrorKind::NotADirectory => ("ENOTDIR", "not a directory"),
        _ => ("EIO", "i/o error"),
    };
    format!("{code}: {text}, {syscall} '{path}'")
}

/// `String.prototype.localeCompare` with the default (ICU root) collation,
/// approximated: case-insensitive first, lowercase before uppercase.
pub fn locale_compare(a: &str, b: &str) -> std::cmp::Ordering {
    // ICU root collation, primary level: whitespace, punctuation and
    // symbols (in this order), digits, then letters, case-insensitively
    const PUNCT: &str = "_-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$";
    let weight = |c: char| -> u32 {
        if c.is_whitespace() {
            0
        } else if let Some(i) = PUNCT.find(c) {
            1 + i as u32
        } else if c.is_ascii_digit() {
            100 + c as u32
        } else if c.is_ascii_lowercase() {
            300 + c as u32
        } else {
            1000 + c as u32
        }
    };
    let la = a.to_lowercase();
    let lb = b.to_lowercase();
    la.chars()
        .map(weight)
        .cmp(lb.chars().map(weight))
        .then_with(|| b.cmp(a))
}

/// The message of a `JSON.parse` failure, close to V8's wording.
pub fn js_json_error(_e: &serde_json::Error, text: &str) -> String {
    format!("SyntaxError: {}", crate::json::v8_parse_error(text))
}

/// `await setTimeout(ms)`.
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}

/// `path.resolve(p)` exists and is not a directory.
pub fn is_file(p: &str) -> bool {
    let full = std::path::absolute(p).unwrap_or_default();
    full.exists() && !full.is_dir()
}

/// `path.resolve(p)` as a string.
/// `path.resolve(p)`: absolute and normalized (`.` and `..` removed).
pub fn resolve(p: &str) -> String {
    use std::path::Component;
    let abs = std::path::absolute(p).unwrap_or_else(|_| std::path::PathBuf::from(p));
    let mut out = std::path::PathBuf::new();
    for c in abs.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    let s = out.display().to_string();
    // a trailing separator is dropped, except for the root
    if s.len() > 1 && s.ends_with(std::path::MAIN_SEPARATOR) {
        s[..s.len() - 1].to_string()
    } else {
        s
    }
}

/// Runs a command line through the shell (`child_process.exec`), returning
/// (stdout, stderr) or an error with the stderr text.
pub fn exec(cmd: &str) -> std::result::Result<(String, String), String> {
    #[cfg(windows)]
    let out = std::process::Command::new("cmd").args(["/C", cmd]).output();
    #[cfg(not(windows))]
    let out = std::process::Command::new("sh").args(["-c", cmd]).output();
    match out {
        Ok(o) => {
            let so = String::from_utf8_lossy(&o.stdout).into_owned();
            let se = String::from_utf8_lossy(&o.stderr).into_owned();
            if o.status.success() {
                Ok((so, se))
            } else {
                Err(se)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod locale_tests {
    /// Node's `localeCompare` order for ASCII punctuation, digits and letters.
    #[test]
    fn locale_compare_matches_icu_root() {
        let mut v: Vec<String> = "~ $ a _ 0 [ . / - A b B 9 ,"
            .split(' ')
            .map(String::from)
            .collect();
        v.sort_by(|a, b| super::locale_compare(a, b));
        assert_eq!(v.join(" "), "_ - , . [ / ~ $ 0 9 a A b B");
    }
}

//! `src/utils/urlUtil.ts`.

fn origin(url: &str) -> String {
    match ::url::Url::parse(url) {
        Ok(u) => u.origin().ascii_serialization(),
        Err(_) => String::new(),
    }
}

/// JavaScript `String.prototype.replace` with a string pattern (first match only).
fn replace_first(s: &str, from: &str, to: &str) -> String {
    s.replacen(from, to, 1)
}

fn lower_index_of(hay: &str, needle: &str) -> Option<usize> {
    hay.to_lowercase().find(&needle.to_lowercase())
}

pub fn get_url_relative_path(url: &str) -> String {
    if url.contains("://") {
        let o = origin(url);
        return url.get(o.len()..).unwrap_or("").to_string();
    }
    url.to_string()
}

pub fn get_server_relative_path(web_url: &str, folder_relative_path: &str) -> String {
    let mut web_rel = get_url_relative_path(web_url);
    let mut to_remove = web_rel.clone();
    if !web_rel.starts_with('/') {
        web_rel = format!("/{web_rel}");
    } else {
        to_remove = web_rel[1..].to_string();
    }
    if web_rel.len() > 1 && web_rel.ends_with('/') {
        web_rel.pop();
    }
    let mut folder = folder_relative_path.to_string();
    if let Some(idx) = lower_index_of(&folder, &to_remove) {
        let pos = idx + to_remove.len();
        folder = folder.get(pos..).unwrap_or("").to_string();
    }
    if !folder.is_empty() {
        if !folder.starts_with('/') {
            folder = format!("/{folder}");
        }
        if folder.ends_with('/') {
            folder.pop();
        }
        if web_rel == "/" && !folder.is_empty() {
            web_rel = folder;
        } else {
            web_rel = format!("{web_rel}{folder}");
        }
    }
    replace_first(&web_rel, "//", "/")
}

pub fn get_server_relative_site_url(web_url: &str) -> String {
    let s = get_server_relative_path(web_url, "");
    if s == "/" { String::new() } else { s }
}

pub fn get_web_relative_path(web_url: &str, folder_url: &str) -> String {
    let web_rel = get_url_relative_path(web_url);
    let to_remove = if let Some(r) = web_rel.strip_prefix('/') {
        r.to_string()
    } else {
        web_rel.clone()
    };
    let mut f = match lower_index_of(folder_url, &to_remove) {
        Some(idx) => folder_url
            .get(idx + to_remove.len()..)
            .unwrap_or("")
            .to_string(),
        None => folder_url.to_string(),
    };
    if !f.starts_with('/') {
        f = format!("/{f}");
    }
    if f.len() > 1 && f.ends_with('/') {
        f.pop();
    }
    replace_first(&f, "//", "/")
}

pub fn get_absolute_url(web_url: &str, server_relative_url: &str) -> String {
    let s = if server_relative_url.starts_with('/') {
        server_relative_url.to_string()
    } else {
        format!("/{server_relative_url}")
    };
    format!("{}{s}", origin(web_url))
}

pub fn url_combine(base: &str, relative: &str) -> String {
    let mut b = base.to_string();
    if b.ends_with('/') {
        b.pop();
    }
    let mut r = relative.strip_prefix('/').unwrap_or(relative).to_string();
    if r.ends_with('/') {
        r.pop();
    }
    format!("{b}/{r}")
}

pub fn get_target_site_absolute_url(web_url: &str, url: &str) -> String {
    let full = if url.starts_with("https://") {
        url.to_string()
    } else {
        get_absolute_url(web_url, url)
    };
    static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"(?i)^https://[\w-]+\.sharepoint\.com/(teams|sites|personal)/[^/?#]+")
            .unwrap()
    });
    match RE.find(&full) {
        Some(m) => m.as_str().to_string(),
        None => origin(&full),
    }
}

pub fn remove_leading_slashes(url: &str) -> String {
    url.trim_start_matches('/').to_string()
}

pub fn remove_trailing_slashes(url: &str) -> String {
    url.trim_end_matches('/').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths() {
        assert_eq!(
            get_server_relative_path("https://contoso.sharepoint.com/sites/team1", ""),
            "/sites/team1"
        );
        assert_eq!(
            get_server_relative_path(
                "https://contoso.sharepoint.com/sites/team1",
                "/Shared Documents"
            ),
            "/sites/team1/Shared Documents"
        );
        assert_eq!(
            get_server_relative_path("/sites/team1/", "/Shared Documents"),
            "/sites/team1/Shared Documents"
        );
        assert_eq!(
            get_server_relative_site_url("https://contoso.sharepoint.com"),
            ""
        );
        assert_eq!(
            get_web_relative_path(
                "https://contoso.sharepoint.com/sites/team1/",
                "/sites/team1/Lists/MyList"
            ),
            "/Lists/MyList"
        );
        assert_eq!(url_combine("https://a/", "/b/"), "https://a/b");
    }
}

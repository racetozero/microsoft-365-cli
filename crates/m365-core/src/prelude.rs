//! Everything a command implementation usually needs.

pub use crate::auth;
pub use crate::config;
pub use crate::cx::Cx;
pub use crate::error::{Error, Result};
pub use crate::http::{Method, Request, ResponseType};
pub use crate::json::{self, Obj, arr, dig, need_arr, s, stringify, to_js_string, truthy, u};
pub use crate::odata;
pub use crate::schema::{FieldCheck, check};
pub use crate::ut;
pub use crate::util::{self, enc, encode_query_parameter as eqp};
pub use crate::{Value, obj};

pub const GRAPH: &str = "https://graph.microsoft.com";

/// `$select`/`$expand` from a `--properties` value: properties with a `/`
/// (`manager/displayName`) become `$expand=manager($select=displayName)`.
pub fn select_expand(properties: Option<&str>) -> Vec<String> {
    let mut q = Vec::new();
    let Some(p) = properties.filter(|p| !p.is_empty()) else {
        return q;
    };
    let all: Vec<&str> = p.split(',').collect();
    let sel: Vec<&str> = all.iter().copied().filter(|x| !x.contains('/')).collect();
    let exp: Vec<&str> = all.iter().copied().filter(|x| x.contains('/')).collect();
    if !sel.is_empty() {
        q.push(format!("$select={}", sel.join(",")));
    }
    if !exp.is_empty() {
        let e: Vec<String> = exp
            .iter()
            .map(|x| {
                let parts: Vec<&str> = x.split('/').collect();
                format!(
                    "{}($select={})",
                    parts[0],
                    parts.get(1).copied().unwrap_or("undefined")
                )
            })
            .collect();
        q.push(format!("$expand={}", e.join(",")));
    }
    q
}

/// `?a&b` or an empty string.
pub fn qs(parts: &[String]) -> String {
    if parts.is_empty() {
        String::new()
    } else {
        format!("?{}", parts.join("&"))
    }
}

/// Moves `@odata.type` (`#microsoft.graph.user`) into `type` (`user`).
pub fn odata_type_to_type(v: &mut Value) {
    if let Some(o) = v.as_object_mut() {
        let t = o
            .get("@odata.type")
            .and_then(Value::as_str)
            .map(|t| t.replacen("#microsoft.graph.", "", 1));
        if let Some(t) = t {
            o.insert("type".into(), Value::from(t));
        }
        o.shift_remove("@odata.type");
    }
}

impl Cx {
    /// `addUnknownOptionsToPayloadZod` / `addUnknownOptionsToPayload`.
    pub fn add_unknown_options(&self, payload: &mut Value) {
        let unknown = crate::ut::options::unknown_options(&self.opts, self.meta);
        if let Some(o) = payload.as_object_mut() {
            crate::ut::options::add_unknown_options_to_payload(o, &unknown);
        }
    }

    /// The option as a string, or `"undefined"` (template literal of a missing value).
    pub fn u(&self, k: &str) -> String {
        self.str(k).unwrap_or_else(|| "undefined".into())
    }
}

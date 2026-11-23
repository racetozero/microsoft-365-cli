//! HTTP requests: a port of upstream's `src/request.ts` (axios with
//! throttling retries, cloud endpoint rewriting and access tokens).

use crate::auth;
use crate::error::{Error, HttpError, Result};
use crate::json::{Obj, stringify, stringify_pretty};
use crate::log;
use serde_json::Value;
use std::sync::OnceLock;
use std::time::Duration;

pub fn user_agent() -> String {
    format!(
        "NONISV|SharePointPnP|CLIMicrosoft365/{}",
        crate::manifest::UPSTREAM_VERSION
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Head,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Patch => "PATCH",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
        }
    }

    pub fn parse(s: &str) -> Option<Method> {
        Some(match s.to_ascii_uppercase().as_str() {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PATCH" => Method::Patch,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            "HEAD" => Method::Head,
            _ => return None,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Body {
    None,
    Json(Value),
    Text(String),
    Bytes(Vec<u8>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseType {
    /// axios `responseType: 'text'`, the CLI default
    Text,
    /// axios `responseType: 'json'`: parsed when the body is JSON
    Json,
    /// raw bytes (`arraybuffer` / `stream`)
    Bytes,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Body,
    pub response_type: ResponseType,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub headers: Obj,
    pub bytes: Vec<u8>,
    pub response_type: ResponseType,
}

impl Response {
    /// `res.data` as axios returns it.
    pub fn data(&self) -> Value {
        data_of(&self.bytes, self.response_type)
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(&name.to_ascii_lowercase())
            .and_then(Value::as_str)
    }
}

fn data_of(bytes: &[u8], rt: ResponseType) -> Value {
    let text = String::from_utf8_lossy(bytes);
    match rt {
        ResponseType::Json => {
            if text.is_empty() {
                Value::String(String::new())
            } else {
                serde_json::from_str(&text).unwrap_or_else(|_| Value::String(text.into_owned()))
            }
        }
        _ => Value::String(text.into_owned()),
    }
}

impl Request {
    pub fn new(method: Method, url: impl Into<String>) -> Request {
        Request {
            method,
            url: url.into(),
            headers: Vec::new(),
            body: Body::None,
            response_type: ResponseType::Text,
        }
    }
    pub fn get(url: impl Into<String>) -> Request {
        Request::new(Method::Get, url)
    }
    pub fn post(url: impl Into<String>) -> Request {
        Request::new(Method::Post, url)
    }
    pub fn patch(url: impl Into<String>) -> Request {
        Request::new(Method::Patch, url)
    }
    pub fn put(url: impl Into<String>) -> Request {
        Request::new(Method::Put, url)
    }
    pub fn delete(url: impl Into<String>) -> Request {
        Request::new(Method::Delete, url)
    }
    pub fn head(url: impl Into<String>) -> Request {
        Request::new(Method::Head, url)
    }
    pub fn header(mut self, k: &str, v: impl Into<String>) -> Request {
        let v = v.into();
        match self
            .headers
            .iter_mut()
            .find(|(n, _)| n.eq_ignore_ascii_case(k))
        {
            Some(e) => e.1 = v,
            None => self.headers.push((k.to_string(), v)),
        }
        self
    }
    pub fn accept(self, v: &str) -> Request {
        self.header("accept", v)
    }
    /// `accept: application/json;odata.metadata=none` + `responseType: 'json'`.
    pub fn graph_json(self) -> Request {
        self.accept("application/json;odata.metadata=none")
            .json_response()
    }
    /// `accept: application/json;odata=nometadata` + `responseType: 'json'`.
    pub fn spo_json(self) -> Request {
        self.accept("application/json;odata=nometadata")
            .json_response()
    }
    pub fn json_response(mut self) -> Request {
        self.response_type = ResponseType::Json;
        self
    }
    pub fn bytes_response(mut self) -> Request {
        self.response_type = ResponseType::Bytes;
        self
    }
    pub fn json(mut self, v: Value) -> Request {
        self.body = Body::Json(v);
        self
    }
    pub fn text(mut self, s: impl Into<String>) -> Request {
        self.body = Body::Text(s.into());
        self
    }
    pub fn bytes(mut self, b: Vec<u8>) -> Request {
        self.body = Body::Bytes(b);
        self
    }
    pub fn anonymous(self) -> Request {
        self.header("x-anonymous", "true")
    }
    pub fn resource(self, r: &str) -> Request {
        self.header("x-resource", r)
    }
    fn get_header(&self, k: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(k))
            .map(|(_, v)| v.as_str())
    }

    pub async fn send(self) -> Result<Response> {
        execute(self).await
    }

    /// Sends the request and returns `res.data`.
    pub async fn data(self) -> Result<Value> {
        Ok(execute(self).await?.data())
    }
}

/// The builder for every HTTPS client. It trusts the OS certificate store
/// (through rustls-native-certs, never a bundled root list) and the CA
/// certificates in `NODE_EXTRA_CA_CERTS`, which Node.js also reads.
pub fn client_builder() -> reqwest::ClientBuilder {
    static EXTRA: OnceLock<Vec<reqwest::Certificate>> = OnceLock::new();
    let extra = EXTRA.get_or_init(|| {
        let Some(path) = std::env::var_os("NODE_EXTRA_CA_CERTS").filter(|p| !p.is_empty()) else {
            return Vec::new();
        };
        let loaded = std::fs::read(&path)
            .map_err(|e| e.to_string())
            .and_then(|pem| reqwest::Certificate::from_pem_bundle(&pem).map_err(|e| e.to_string()));
        loaded.unwrap_or_else(|e| {
            eprintln!(
                "Warning: Ignoring extra certs from `{}`, load failed: {e}. Set NODE_EXTRA_CA_CERTS to a readable PEM file of CA certificates, or unset it.",
                std::path::Path::new(&path).display()
            );
            Vec::new()
        })
    });
    extra.iter().cloned().fold(
        reqwest::Client::builder().tls_built_in_native_certs(true),
        reqwest::ClientBuilder::add_root_certificate,
    )
}

/// The error and its causes. reqwest's own message hides the cause, such as
/// an untrusted certificate.
pub fn error_chain(e: &dyn std::error::Error) -> String {
    let mut msg = e.to_string();
    let mut cause = e.source();
    while let Some(c) = cause {
        msg = format!("{msg}: {c}");
        cause = c.source();
    }
    msg
}

fn client() -> &'static reqwest::Client {
    static C: OnceLock<reqwest::Client> = OnceLock::new();
    C.get_or_init(|| {
        client_builder()
            .gzip(true)
            .deflate(true)
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("HTTP client")
    })
}

fn update_for_cloud(url: &str) -> String {
    let Ok(u) = url::Url::parse(url) else {
        return url.to_string();
    };
    let host = format!("{}://{}", u.scheme(), u.host_str().unwrap_or(""));
    let cloud = auth::endpoint_for_resource(&host, auth::cloud_type());
    if cloud == host {
        url.to_string()
    } else {
        url.replacen(&host, &cloud, 1)
    }
}

/// `removeDoubleSlashes`: the first `//` after the scheme becomes `/`.
fn remove_double_slashes(url: &str) -> String {
    if url.len() <= 8 || !url.is_char_boundary(8) {
        return url.to_string();
    }
    let (a, b) = url.split_at(8);
    format!("{a}{}", b.replacen("//", "/", 1))
}

fn body_for_debug(b: &Body) -> Option<Value> {
    match b {
        Body::None => None,
        Body::Json(v) => Some(v.clone()),
        Body::Text(s) => Some(Value::String(s.clone())),
        Body::Bytes(_) => Some(Value::String("<binary>".into())),
    }
}

/// Executes a request with upstream's semantics.
pub async fn execute(mut req: Request) -> Result<Response> {
    // axios rejects relative or malformed URLs before sending (`new URL`)
    if url::Url::parse(&req.url).is_err() {
        return Err(Error::js("Invalid URL"));
    }
    req.url = update_for_cloud(&req.url);
    req.url = remove_double_slashes(&req.url);
    let debug = log::debug();
    loop {
        let anonymous = req.get_header("x-anonymous").is_some();
        let token = if anonymous {
            String::new()
        } else {
            let url = req.get_header("x-resource").unwrap_or(&req.url).to_string();
            let resource = auth::resource_from_url(&url);
            Box::pin(auth::ensure_access_token(&resource, false)).await?
        };
        let mut headers = req.headers.clone();
        headers.retain(|(n, _)| {
            !n.eq_ignore_ascii_case("x-anonymous") && !n.eq_ignore_ascii_case("x-resource")
        });
        if !token.is_empty() {
            headers.retain(|(n, _)| !n.eq_ignore_ascii_case("authorization"));
            headers.push(("authorization".into(), format!("Bearer {token}")));
        }
        if debug {
            let mut h = Obj::new();
            h.insert("user-agent".into(), Value::from(user_agent()));
            h.insert("accept-encoding".into(), Value::from("gzip, deflate"));
            h.insert(
                "X-ClientService-ClientTag".into(),
                Value::from(format!("M365CLI:{}", crate::manifest::UPSTREAM_VERSION)),
            );
            for (k, v) in &headers {
                h.insert(k.clone(), Value::from(v.as_str()));
            }
            let mut o = Obj::new();
            o.insert("url".into(), Value::from(req.url.as_str()));
            o.insert(
                "method".into(),
                Value::from(req.method.as_str().to_lowercase()),
            );
            o.insert("headers".into(), Value::Object(h));
            o.insert(
                "responseType".into(),
                Value::from(match req.response_type {
                    ResponseType::Json => "json",
                    ResponseType::Text => "text",
                    ResponseType::Bytes => "stream",
                }),
            );
            o.insert("decompress".into(), Value::Bool(true));
            if req.response_type != ResponseType::Bytes
                && let Some(b) = body_for_debug(&req.body)
            {
                o.insert("data".into(), b);
            }
            log::log_stderr("Request:");
            log::log_stderr(&stringify_pretty(&Value::Object(o), "  "));
        }
        let res = transport(&req, &headers).await;
        let res = match res {
            Ok(r) => r,
            Err(e) => {
                if debug {
                    log::log_stderr("Request error:");
                    log::log_stderr(&format!("{{\n  \"url\": \"{}\"\n}}", req.url));
                }
                return Err(Error::Network(e));
            }
        };
        if (200..300).contains(&res.status) {
            if debug {
                let mut o = Obj::new();
                o.insert("url".into(), Value::from(req.url.as_str()));
                o.insert("status".into(), Value::from(res.status));
                o.insert("statusText".into(), Value::from(res.status_text.as_str()));
                o.insert("headers".into(), Value::Object(res.headers.clone()));
                if res
                    .header("content-type")
                    .is_some_and(|c| c.contains("json"))
                {
                    o.insert("data".into(), res.data());
                }
                log::log_stderr("Response:");
                log::log_stderr(&stringify_pretty(&Value::Object(o), "  "));
            }
            return Ok(res);
        }
        if res.status == 429 || res.status == 503 {
            let retry = res
                .header("retry-after")
                .and_then(crate::json::parse_int)
                .unwrap_or(10)
                .max(0) as u64;
            if debug {
                let _ = log::log(Value::from(format!(
                    "Request throttled. Waiting {retry} sec before retrying..."
                )));
            }
            tokio::time::sleep(Duration::from_secs(retry)).await;
            continue;
        }
        // axios moves `response.data` to `error`; streams are left out
        let error = if res.bytes.is_empty() || res.response_type == ResponseType::Bytes {
            None
        } else {
            Some(data_of(&res.bytes, res.response_type))
        };
        if debug {
            let mut o = Obj::new();
            o.insert("url".into(), Value::from(req.url.as_str()));
            o.insert("status".into(), Value::from(res.status));
            o.insert("statusText".into(), Value::from(res.status_text.as_str()));
            o.insert("headers".into(), Value::Object(res.headers.clone()));
            if let Some(e) = &error {
                o.insert("error".into(), e.clone());
            }
            log::log_stderr("Request error:");
            log::log_stderr(&stringify_pretty(&Value::Object(o), "  "));
        }
        return Err(Error::Http(Box::new(HttpError {
            status: res.status,
            status_text: res.status_text,
            headers: res.headers,
            error,
            url: req.url.clone(),
        })));
    }
}

async fn transport(
    req: &Request,
    headers: &[(String, String)],
) -> std::result::Result<Response, String> {
    #[cfg(feature = "parity")]
    if let Ok(mock) = std::env::var("M365_PARITY_MOCK") {
        return crate::parity::send(&mock, req, headers).await;
    }
    let method = reqwest::Method::from_bytes(req.method.as_str().as_bytes()).unwrap();
    let mut b = client().request(method, &req.url);
    b = b.header("user-agent", user_agent());
    b = b.header(
        "X-ClientService-ClientTag",
        format!("M365CLI:{}", crate::manifest::UPSTREAM_VERSION),
    );
    let has = |k: &str| headers.iter().any(|(n, _)| n.eq_ignore_ascii_case(k));
    if !has("accept") {
        b = b.header("accept", "application/json, text/plain, */*");
    }
    for (k, v) in headers {
        b = b.header(k.as_str(), v.as_str());
    }
    match &req.body {
        Body::None => {}
        Body::Json(v) => {
            if !has("content-type") {
                b = b.header("content-type", "application/json");
            }
            b = b.body(stringify(v));
        }
        Body::Text(s) => {
            if !has("content-type") {
                b = b.header("content-type", "application/x-www-form-urlencoded");
            }
            b = b.body(s.clone());
        }
        Body::Bytes(bytes) => {
            b = b.body(bytes.clone());
        }
    }
    let res = b.send().await.map_err(|e| error_chain(&e))?;
    let status = res.status();
    let mut h = Obj::new();
    for (k, v) in res.headers() {
        h.insert(
            k.as_str().to_string(),
            Value::from(v.to_str().unwrap_or("")),
        );
    }
    let bytes = res.bytes().await.map_err(|e| e.to_string())?.to_vec();
    Ok(Response {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or("").to_string(),
        headers: h,
        bytes,
        response_type: req.response_type,
    })
}

/// The serialized body for traces and mocks.
pub fn body_text(b: &Body) -> Option<String> {
    match b {
        Body::None => None,
        Body::Json(v) => Some(stringify(v)),
        Body::Text(s) => Some(s.clone()),
        Body::Bytes(b) => Some(format!("<{} bytes>", b.len())),
    }
}

//! Errors and upstream's error-message extraction helpers
//! (`handleRejectedODataJsonPromise`, `handleRejectedODataPromise`,
//! `handleError`).

use crate::json::{Obj, to_js_string};
use serde_json::Value;

/// A failed HTTP request, shaped like the axios error upstream inspects:
/// `error` is the response body (parsed JSON when it parses, else text).
#[derive(Debug, Clone)]
pub struct HttpError {
    pub status: u16,
    pub status_text: String,
    pub headers: Obj,
    pub error: Option<Value>,
    pub url: String,
}

impl HttpError {
    /// axios' message, e.g. `Request failed with status code 404`.
    pub fn message(&self) -> String {
        format!("Request failed with status code {}", self.status)
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    /// `CommandError(message, code)`; the message may be any value.
    /// `status` keeps the HTTP status when the error came from a response.
    Command {
        message: Value,
        code: Option<i32>,
        status: Option<u16>,
    },
    /// A failed request that the command did not translate.
    Http(Box<HttpError>),
    /// A network or I/O failure (axios error without a response).
    Network(String),
    /// A JavaScript `Error` thrown by upstream code (`throw new Error(m)`):
    /// printed as `Error: Error: m` when no `handleRejected*` translates it.
    Js(String),
    /// A JavaScript `SyntaxError` (a failed `JSON.parse`).
    Syntax(String),
    /// `throw value` of a non-Error value (an object such as a batch
    /// response body).
    Thrown(Value),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
    pub fn msg(m: impl Into<String>) -> Error {
        Error::Command {
            message: Value::String(m.into()),
            code: None,
            status: None,
        }
    }

    pub fn value(v: Value) -> Error {
        Error::Command {
            message: v,
            code: None,
            status: None,
        }
    }

    pub fn with_code(m: impl Into<String>, code: i32) -> Error {
        Error::Command {
            message: Value::String(m.into()),
            code: Some(code),
            status: None,
        }
    }

    /// The message of a `CommandError`, or the JavaScript message otherwise.
    pub fn command_message(&self) -> Value {
        match self {
            Error::Command { message, .. } => message.clone(),
            other => Value::from(other.js_message()),
        }
    }

    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Http(h) => Some(h.status),
            Error::Command { status, .. } => *status,
            _ => None,
        }
    }

    fn with_status(self, s: Option<u16>) -> Error {
        match self {
            Error::Command { message, code, .. } => Error::Command {
                message,
                code,
                status: s,
            },
            other => other,
        }
    }

    pub fn http(&self) -> Option<&HttpError> {
        match self {
            Error::Http(h) => Some(h),
            _ => None,
        }
    }

    /// The body of a failed request (axios `err.error`).
    pub fn body(&self) -> Option<&Value> {
        self.http().and_then(|h| h.error.as_ref())
    }

    /// `err.message` of the JavaScript error.
    pub fn js_message(&self) -> String {
        match self {
            Error::Command { message, .. } => to_js_string(message),
            Error::Http(h) => h.message(),
            Error::Network(m) | Error::Js(m) | Error::Syntax(m) => m.clone(),
            Error::Thrown(v) => to_js_string(v),
        }
    }

    /// `throw new Error(m)`.
    pub fn js(m: impl Into<String>) -> Error {
        Error::Js(m.into())
    }

    /// Marks a `JSON.parse` failure, whose `${err}` reads `SyntaxError: ...`.
    pub fn with_syntax(self) -> Error {
        match self {
            Error::Js(m) => Error::Syntax(m),
            other => other,
        }
    }

    /// The pieces of a rejected value that upstream's handlers look at:
    /// `response.error`, whether it is an `Error` instance, its message and
    /// the value itself (for `CommandError(response)`).
    fn view(&self) -> Option<(Option<Value>, bool, String, Value)> {
        match self {
            Error::Http(h) => Some((
                h.error.clone(),
                true,
                h.message(),
                Value::from(format!("AxiosError: {}", h.message())),
            )),
            Error::Thrown(v) => Some((v.get("error").cloned(), false, String::new(), v.clone())),
            _ => None,
        }
    }

    /// `handleRejectedODataJsonPromise`.
    pub fn odata_json(self) -> Error {
        let st = self.status();
        let Some((err, is_error, message, raw)) = self.view() else {
            return match self {
                Error::Network(m) | Error::Js(m) | Error::Syntax(m) => Error::msg(m),
                other => other,
            };
        };
        let r = (|| {
            if let Some(e) = &err
                && let Some(m) = e.get("odata.error").and_then(|e| e.get("message"))
                && truthy(Some(m))
            {
                return Error::value(m.get("value").cloned().unwrap_or(Value::Null));
            }
            let Some(e) = err.filter(crate::json::truthy) else {
                return if is_error {
                    Error::msg(message)
                } else {
                    Error::value(raw)
                };
            };
            if let Some(m) = e.get("error").and_then(|x| x.get("message"))
                && truthy(Some(m))
            {
                return Error::value(m.clone());
            }
            if let Some(m) = e.get("message")
                && truthy(Some(m))
            {
                return Error::value(m.clone());
            }
            if let Some(m) = e.get("error_description")
                && truthy(Some(m))
            {
                return Error::value(m.clone());
            }
            if let Value::String(s) = &e
                && let Ok(parsed) = serde_json::from_str::<Value>(s)
                && let Some(m) = parsed.get("error").and_then(|x| x.get("message"))
                && truthy(Some(m))
            {
                return Error::value(m.clone());
            }
            Error::value(e)
        })();
        r.with_status(st)
    }

    /// An error raised outside the command's try/catch: the CLI prints
    /// `${err}` (`Error: m`, `AxiosError: ...`) as the message.
    pub fn uncaught(self) -> Error {
        match self {
            Error::Js(m) | Error::Network(m) => Error::msg(format!("Error: {m}")),
            Error::Syntax(m) => Error::msg(format!("SyntaxError: {m}")),
            Error::Http(h) => Error::msg(format!("AxiosError: {}", h.message())),
            Error::Thrown(v) => Error::value(v),
            c @ Error::Command { .. } => c,
        }
    }

    /// `VivaEngageCommand.handleRejectedODataJsonPromise`: `error.base` or
    /// the rejected value itself.
    pub fn viva(self) -> Error {
        let st = self.status();
        let r = match &self {
            Error::Http(h) => match h
                .error
                .as_ref()
                .and_then(|e| e.get("base"))
                .filter(|b| truthy(Some(b)))
            {
                Some(b) => Error::value(b.clone()),
                None => Error::msg(format!("AxiosError: {}", h.message())),
            },
            Error::Thrown(v) => match v
                .get("error")
                .and_then(|e| e.get("base"))
                .filter(|b| truthy(Some(b)))
            {
                Some(b) => Error::value(b.clone()),
                None => Error::value(v.clone()),
            },
            Error::Network(m) | Error::Js(m) => Error::msg(format!("Error: {m}")),
            Error::Syntax(m) => Error::msg(format!("SyntaxError: {m}")),
            Error::Command { .. } => return self,
        };
        r.with_status(st)
    }

    /// `handleRejectedODataPromise`.
    pub fn odata(self) -> Error {
        let st = self.status();
        let Some((err, is_error, message, raw)) = self.view() else {
            return match self {
                Error::Network(m) | Error::Js(m) | Error::Syntax(m) => Error::msg(m),
                other => other,
            };
        };
        let r = (|| {
            let Some(e) = err.filter(crate::json::truthy) else {
                return if is_error {
                    Error::msg(message)
                } else {
                    Error::value(raw)
                };
            };
            if let Value::String(s) = &e
                && let Ok(parsed) = serde_json::from_str::<Value>(s)
                && let Some(v) = parsed
                    .get("odata.error")
                    .and_then(|x| x.get("message"))
                    .and_then(|m| m.get("value"))
            {
                return Error::value(v.clone());
            }
            if let Some(inner) = e.get("error") {
                let code = inner.get("code").filter(|c| crate::json::truthy(c));
                let msg = inner.get("message").cloned().unwrap_or(Value::Null);
                return match code {
                    Some(c) => Error::msg(format!("{} - {}", to_js_string(c), to_js_string(&msg))),
                    None => Error::value(msg),
                };
            }
            Error::value(e)
        })();
        r.with_status(st)
    }

    /// `handleError` / `handleRejectedPromise`.
    pub fn plain(self) -> Error {
        let st = self.status();
        match self {
            Error::Http(h) => Error::msg(h.message()).with_status(st),
            Error::Network(m) | Error::Js(m) | Error::Syntax(m) => Error::msg(m),
            Error::Thrown(v) => Error::value(v),
            other => other,
        }
    }
}

fn truthy(v: Option<&Value>) -> bool {
    crate::json::truthy_opt(v)
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::msg(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Error {
        Error::msg(s)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::msg(e.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::msg(e.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.js_message())
    }
}

/// `bail!("...")` returns a `CommandError` with a formatted message.
#[macro_export]
macro_rules! bail {
    ($($t:tt)*) => {
        return Err($crate::error::Error::msg(format!($($t)*)))
    };
}

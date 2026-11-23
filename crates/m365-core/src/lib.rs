//! Runtime of the Rust port of the CLI for Microsoft 365.
//!
//! The command tables are `src/generated/manifest.rs` and the help text is
//! `src/help/data/`;
//! command implementations live in the suite crates and are registered with
//! [`runner::main`].

pub mod agent;
pub mod args;
pub mod auth;
pub mod auth_server;
pub mod cert;
pub mod completion;
pub mod config;
pub mod cx;
pub mod error;
pub mod expr;
pub mod help;
pub mod http;
pub mod json;
pub mod log;
pub mod manifest;
pub mod md;
pub mod odata;
pub mod output;
#[cfg(feature = "parity")]
pub mod parity;
pub mod prelude;
pub mod prompt;
pub mod query;
pub mod report;
pub mod runner;
pub mod schema;
pub mod term;
pub mod time;
pub mod ut;
pub mod util;
pub mod validation;
pub mod xml;
pub mod zip;

pub use cx::{Cx, Spec};
pub use error::{Error, Result};
pub use http::{Method, Request};
pub use json::{Obj, json};
pub use serde_json::Value;

/// SharePoint CSOM property types (`src/data/csom.json`).
pub fn csom_defs() -> &'static Value {
    static C: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    C.get_or_init(|| serde_json::from_str(include_str!("data/csom.json")).unwrap_or(Value::Null))
}

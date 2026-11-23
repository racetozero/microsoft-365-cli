//! Terminal helpers: line endings and `chalk`-style colors.
//!
//! `chalk` decides from stdout whether to color, also for text written to
//! stderr, so these helpers do the same.

use std::io::IsTerminal;
use std::sync::OnceLock;

#[cfg(windows)]
pub const EOL: &str = "\r\n";
#[cfg(not(windows))]
pub const EOL: &str = "\n";

pub fn color_enabled() -> bool {
    static C: OnceLock<bool> = OnceLock::new();
    *C.get_or_init(|| {
        if let Ok(f) = std::env::var("FORCE_COLOR") {
            return !(f == "0" || f == "false");
        }
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }
        if std::env::var("TERM").is_ok_and(|t| t == "dumb") {
            return false;
        }
        std::io::stdout().is_terminal()
    })
}

fn wrap(s: &str, open: &str, close: &str) -> String {
    if color_enabled() {
        format!("\u{1b}[{open}m{s}\u{1b}[{close}m")
    } else {
        s.to_string()
    }
}

pub fn bold(s: &str) -> String {
    wrap(s, "1", "22")
}
pub fn red(s: &str) -> String {
    wrap(s, "31", "39")
}
pub fn green(s: &str) -> String {
    wrap(s, "32", "39")
}
pub fn yellow(s: &str) -> String {
    wrap(s, "33", "39")
}
pub fn blue(s: &str) -> String {
    wrap(s, "34", "39")
}
pub fn cyan(s: &str) -> String {
    wrap(s, "36", "39")
}
pub fn gray(s: &str) -> String {
    wrap(s, "90", "39")
}

pub fn stdin_is_tty() -> bool {
    std::io::stdin().is_terminal()
}

pub fn stdout_is_tty() -> bool {
    std::io::stdout().is_terminal()
}

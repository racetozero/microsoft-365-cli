//! Interactive prompts (`src/utils/prompt.ts`), on stderr.

use crate::config::{self, names};
use crate::error::{Error, Result};

fn theme() -> dialoguer::theme::ColorfulTheme {
    dialoguer::theme::ColorfulTheme::default()
}

fn interactive() -> Result<()> {
    if crate::agent::enabled() {
        return Err(Error::msg("prompting is disabled in agent mode"));
    }
    Ok(())
}

pub fn input(message: &str) -> Result<String> {
    interactive()?;
    dialoguer::Input::<String>::with_theme(&theme())
        .with_prompt(message.trim_end_matches(':').trim_end())
        .allow_empty(true)
        .interact_text()
        .map_err(|e| Error::with_code(e.to_string(), 1))
}

pub fn confirm(message: &str, default: bool) -> Result<bool> {
    interactive()?;
    dialoguer::Confirm::with_theme(&theme())
        .with_prompt(message)
        .default(default)
        .interact()
        .map_err(|e| Error::with_code(e.to_string(), 1))
}

pub fn select(message: &str, choices: &[String]) -> Result<usize> {
    interactive()?;
    let page = config::get(names::PROMPT_LIST_PAGE_SIZE)
        .and_then(|v| v.as_u64())
        .unwrap_or(7) as usize;
    dialoguer::Select::with_theme(&theme())
        .with_prompt(message)
        .items(choices)
        .default(0)
        .max_length(page)
        .interact()
        .map_err(|e| Error::with_code(e.to_string(), 1))
}

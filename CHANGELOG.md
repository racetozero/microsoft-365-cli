# Changelog

## Unreleased

First release of the Rust port of CLI for Microsoft 365 v12.1.0.

- All 888 commands, with upstream's options, validation, output formats, `--query` and requests, checked by the parity harness in `scripts/parity`.
- Same configuration and login files as the Node.js CLI.
- `--agent` / `M365_AGENT=1`: one JSON line per command, no prompts, `input_required` and `confirmation_required` errors.
- `m365 skill`: nine agent skills (`m365` and one per suite), also in `.agents/skills`.
- `m365 update` installs the latest release (the release's own installer, or `brew upgrade` for Homebrew installs, or `M365_SELF_UPDATE_COMMAND`), and a release build prints a one-line notice when a newer release exists. `M365_NO_UPDATE_NOTIFIER=1` or upstream's `CLIMICROSOFT365_NOUPDATE=1` turn the notice off.
- Help in the style of `atl`: a summary, the usage with required options, options with their values and allowed values, the rules between options, examples, notes and permissions; `--help --agent` returns it as JSON. The text lives in `crates/m365-core/src/help/data/`.
- `m365 --version` prints the version.
- Profile-guided release builds for every platform except Windows ARM64 (`scripts/build_m365_pgo.py`, `just pgo-bench`): SharePoint Framework checks run up to 8% faster and the binary is 9% smaller.
- In WSL, browser sign-in and `m365 docs` open the Windows browser.
- No telemetry.

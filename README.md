# Microsoft 365 Toolkit - `m365`

[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://github.com/racetozero/microsoft-365-cli/blob/main/rust-toolchain.toml)
[![Release](https://img.shields.io/github/v/release/racetozero/microsoft-365-cli?sort=semver)](https://github.com/racetozero/microsoft-365-cli/releases)
[![Downloads](https://img.shields.io/github/downloads/racetozero/microsoft-365-cli/total)](https://github.com/racetozero/microsoft-365-cli/releases)

A [very fast](#performance) command-line tool for Microsoft 365 that you can
use from your terminal or let your AI agent use for you.

`m365` works with Entra ID, SharePoint, Teams, Outlook, Planner, To Do,
OneDrive, Power Platform, Power Automate, Purview, Viva Engage and SharePoint
Framework projects. It has the same 888 commands as
[CLI for Microsoft 365](https://pnp.github.io/cli-microsoft365/), so scripts
written for that tool keep working.

Claude Code, Codex, Cursor and other agents can find users, update SharePoint
lists and post to Teams with the same commands you type, through a JSON mode
that never stops to ask a question, and installable skills.

```sh
m365 entra user get --userName jane@contoso.com
```

That command shows one user. Most of what you do in `m365` reads like that.

## Why m365

- **Start quickly.** Commands start in about 3 ms, where the Node.js tool
  takes about 155 ms, and use about a tenth of the memory. See [Performance](#performance).
- **Agent ready.** Add `--agent` and every command prints one JSON line.
  Nothing prompts. When a value is missing, the error names the option to add.
  Commands that remove things wait for `--force`.
- **Skills for your agent.** One core skill and one per area teach the
  commands. Install them with `npx skills add racetozero/microsoft-365-cli`
  or `m365 skill --install`.
- **The commands you already know.** All 888 commands of CLI for Microsoft 365,
  with the same options, output and errors. A test harness compares the two
  tools on every command.
- **Help that reads well.** Every command's `--help` shows its usage, options
  with their values, the rules between options, examples, notes and
  permissions. With `--agent` it is JSON.
- **Every API, not only the common ones.** `m365 request` calls any Microsoft
  Graph or SharePoint REST endpoint with your normal login.
- **SharePoint Framework helpers.** Upgrade an SPFx project to a newer version,
  check it for problems, and add a deployment pipeline.
- **Every sign-in your company uses.** Device code, browser, certificates,
  client secrets, managed identity and federated credentials.
- **One file, kept current.** A single native binary with no Node.js to set
  up. `m365 update` installs new releases.
- **Private by design.** `m365` collects no telemetry.

## Install

macOS, Linux and WSL:

```sh
curl -LsSf https://raw.githubusercontent.com/racetozero/microsoft-365-cli/main/install.sh | sh
```

Windows, including ARM64:

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://raw.githubusercontent.com/racetozero/microsoft-365-cli/main/install.ps1 | iex"
```

The installer selects the correct release, verifies its SHA-256 checksum, and
installs `m365` in `~/.local/bin` (`%USERPROFILE%\.local\bin` on Windows).

On Linux, if the glibc release needs a newer `GLIBC_*` version than your
system provides, the installer uses the matching musl release. Set
`M365_LIBC=musl` to choose it yourself.

On macOS and Linux, the installer adds `~/.local/bin` to your shell's startup
file (bash, zsh, fish, or sh). Open a new terminal after installation, or run
`~/.local/bin/m365` immediately. On Windows, the installer prints the folder
to add to `PATH` if it is not there yet.

In WSL, run the Linux installer inside your distribution. Browser sign-in
(`m365 login --authType browser`) and `m365 docs` open the Windows browser.

`M365_VERSION` installs a specific release and `M365_INSTALL_DIR` another
folder, on every platform.

Update later with:

```sh
m365 update
```

`m365 update --check` only reports whether a newer release exists. In a
terminal, `m365` checks for a new release at most once every 20 hours, in
the background, and prints one line after a command when there is one. Set
`M365_NO_UPDATE_NOTIFIER=1` to turn the notice off.

Homebrew on macOS and Linux:

```sh
brew install racetozero/tap/m365
```

Homebrew owns this binary, so `m365 update` runs
`brew upgrade racetozero/tap/m365` for you. To use another package manager,
set `M365_SELF_UPDATE_COMMAND`, for example
`mise use -g github:racetozero/microsoft-365-cli@{version}`.

## Start in two commands

Sign in, then run a command:

```sh
m365 login
m365 entra user get --userName jane@contoso.com
```

`m365 login` shows a code and a web address. Open the address in any browser,
enter the code and sign in. This works over SSH too. `m365 status` shows who
you're signed in as.

To see every command, run `m365 --help`, then add `--help` to a group or a
command (`m365 spo --help`, `m365 spo web get --help`). `-h examples`,
`-h options` and `-h permissions` show one part of a command's help.

## A few examples

**Find people and groups.** Look up a user, then see the groups they're in:

```sh
m365 entra user get --userName jane@contoso.com
m365 entra user list --department Sales --query "[].userPrincipalName"
m365 entra group member list --groupName "Sales" --role Owner
```

**Work with SharePoint lists.** Add an item, then change it. Field values are
options named after the field:

```sh
m365 spo listitem add --webUrl https://contoso.sharepoint.com/sites/sales --listTitle Projects --Title "New site" --Status Open
m365 spo listitem set --webUrl https://contoso.sharepoint.com/sites/sales --listTitle Projects --id 3 --Status Done
```

**Upload and find files:**

```sh
m365 spo file add --webUrl https://contoso.sharepoint.com/sites/sales --folder "Shared Documents" --path ./report.pdf
m365 spo search --queryText "quarterly report" --selectProperties Title,Path
```

**Post to Teams:**

```sh
m365 teams message send --teamId TEAM_ID --channelId CHANNEL_ID --message "Deployed v2.3"
m365 teams chat message send --userEmails jane@contoso.com --message "Done"
```

**Send mail and plan work:**

```sh
m365 outlook mail send --to jane@contoso.com --subject "Report" --bodyContents "See attached" --attachment ./report.pdf
m365 planner task add --title "Call client" --planTitle "Q1" --ownerGroupName "Sales" --bucketName "To do"
```

**Upgrade an SPFx project.** Run it in the project folder to get every step as
a Markdown report:

```sh
m365 spfx project upgrade --output md > upgrade.md
```

## When there's no command for it

The commands cover common work. For everything else, `m365 request` sends an
authenticated request to Microsoft Graph or SharePoint. You use your normal
login, and you don't write any HTTP code.

```sh
m365 request --url "https://graph.microsoft.com/v1.0/me/onlineMeetings"
m365 request --url "https://contoso.sharepoint.com/sites/sales/_api/web" --method get
```

## Sign in

`m365` supports the ways companies usually sign in to Microsoft 365:

- **People:** device code (the default), the system browser, or a user name
  and password.
- **Apps and automation:** certificates, client secrets, Azure managed
  identity and federated credentials such as GitHub Actions OIDC.

Logins are saved in your home folder (`~/.cli-m365-connection.json`). You can
keep several and switch between them with `m365 connection use`.

## For scripts and AI agents

Every command prints JSON by default, and `--output text`, `csv` or `md`
change that. `--query` keeps only what you need. With `--agent`, every command
prints one short JSON line, and `m365` never stops to ask a question.

```json
{"ok":true,"command":"entra user get","data":{"displayName":"Jane Doe","id":"..."}}
{"ok":false,"command":"entra user get","error":{"code":"input_required","message":"Specify either 'id', 'userName', or 'email'.","input":{"options":["id","userName","email"]}}}
```

If an agent leaves out a value, the error lists the options it can use, so the
agent can retry once and succeed. A command that removes something fails with
`confirmation_required` until the agent adds `--force`.
`m365 <command> --help --agent` describes a command's options, rules and
examples as JSON.

Agent skills teach your agent the commands. There is one core skill and one
for each area: Entra ID, SharePoint, Teams, Outlook and Planner, files, Power
Platform, SharePoint Framework and tenant administration. Install them with
the [skills](https://github.com/vercel-labs/skills) CLI, which works with
Claude Code, Codex, Cursor and other agents, or with `m365` itself:

```sh
npx skills add racetozero/microsoft-365-cli     # or: bunx skills add racetozero/microsoft-365-cli
m365 skill --install                                # writes them to .claude/skills/
```

The skills are also in [`.agents/skills`](.agents/skills).

## Performance

`m365` benchmarks local work, not Microsoft's response time, which is most of
what a command that calls Microsoft 365 waits for. The results below were
measured on 2026-11-22 with `m365` 12.1.0 and CLI for Microsoft 365 12.1.0.

### Startup and memory

| Command                                  | CLI for Microsoft 365 (Node.js) |   m365 | Faster |
| ---------------------------------------- | ------------------------------: | -----: | -----: |
| `m365 version`                           |                          155 ms | 3.1 ms |    50x |
| `m365 --help`                            |                          155 ms | 3.5 ms |    44x |
| `m365 spo web get --help`                |                          156 ms | 3.3 ms |    47x |
| `m365 entra user get` (validation error) |                          155 ms | 3.3 ms |    47x |
| `m365 cli config list`                   |                          154 ms | 3.1 ms |    50x |

| Maximum memory (RSS)                      | CLI for Microsoft 365 (Node.js) |     m365 |
| ----------------------------------------- | ------------------------------: | -------: |
| `m365 --version`                          |                        92.4 MiB |  7.7 MiB |
| `m365 --help`                             |                        92.5 MiB |  8.8 MiB |
| `m365 spfx project doctor --output json`  |                       156.4 MiB | 23.3 MiB |
| `m365 spfx project upgrade --output json` |                       173.3 MiB | 25.6 MiB |

A command that calls Microsoft 365 starts its first request about 150 ms
sooner, which adds up in scripts and agent sessions that run many commands.

### SharePoint Framework

| Command                                   | CLI for Microsoft 365 (Node.js) |    m365 | Faster |
| ----------------------------------------- | ------------------------------: | ------: | -----: |
| `m365 spfx project doctor --output json`  |                          274 ms |  9.4 ms |    29x |
| `m365 spfx project upgrade --output json` |                          306 ms | 88.2 ms |   3.5x |

`spfx project upgrade` runs `node --version` for one of its checks, and that
process is most of its time.

### Profile-guided release builds

Release builds are optimized with profiles from real runs of `m365`
([profile-guided optimization](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)).
The release workflow trains each platform's build on offline workloads: help,
validation errors, configuration, output formats and the SharePoint Framework
rule engine. Windows ARM64 builds are not profiled, because Rust has no
profiling runtime for that platform. Apart from plain `--version` and
`--help`, the comparison below uses commands that were not part of the
training.

| Measure                                 | Standard build | Optimized build |        Change |
| --------------------------------------- | -------------: | --------------: | ------------: |
| `m365 --version`                        |       4.114 ms |        4.140 ms |  0.63% slower |
| `m365 --help`                           |       4.666 ms |        4.574 ms |  1.98% faster |
| `m365 outlook mail send --help`         |       4.381 ms |        4.330 ms |  1.17% faster |
| Validation error                        |       4.614 ms |        4.533 ms |  1.75% faster |
| `m365 spfx project doctor`              |      13.231 ms |       12.224 ms |  7.61% faster |
| `m365 spfx project upgrade --toVersion` |      30.732 ms |       29.645 ms |  3.54% faster |
| Geometric mean latency                  |         1.000x |          0.974x |  2.61% faster |
| Executable size                         |      13.56 MiB |       12.37 MiB | 8.81% smaller |
| gzip size                               |       5.34 MiB |        4.83 MiB | 9.43% smaller |

Start-up is the same within 2%, because it is mostly the operating system
loading the program. The gain is in the SharePoint Framework rule engine and
in size.

### Method

The tests ran on macOS with an Apple M4 and Rust 1.100.0, with an isolated
home folder and no network. Lower is better.

- **Start-up and SPFx:** the profile-guided release build against the Node.js
  CLI, the median of 21 runs of each command. Other programs were running on
  the machine, so each time is the lower median of two runs. The SPFx project
  is a SharePoint Framework 1.10 React web part.
- **Memory:** the maximum resident set size of one run.
- **Profile-guided builds:** the median of five trials of 50 runs (20 for
  SPFx) after five warm-up runs, alternating the two binaries. The SPFx
  commands run on a generated 1.9.1 project. These runs are timed from
  Python, which adds about 1 ms to each.

```sh
just bench       # m365 against the Node.js CLI (needs a built checkout in /tmp/cli-microsoft365)
just pgo-bench   # ordinary and profile-guided builds on held-out workloads
```

## Compatibility

`m365` tracks
[CLI for Microsoft 365 v12.1.0](https://github.com/pnp/cli-microsoft365/releases/tag/v12.1.0).
A test harness ([`scripts/parity`](scripts/parity)) runs both tools side by
side against a mock server and compares, for all 888 commands, the validation
errors, the output, the exit code and every HTTP request. For SharePoint
Framework commands it also compares the reports and files on 386 sample
projects. It also checks that every command's help is complete. All 888
commands match.

Help is the one intended difference: `m365` has its own help layout (see
[Start in two commands](#start-in-two-commands)).

## Development

Use Rust stable, cargo-nextest and [just](https://github.com/casey/just):

```bash
cargo nextest run --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
just pgo-test
just pgo-bench
just parity      # every command against the Node.js CLI
```

The command tables are generated code in
`crates/m365-core/src/generated/manifest.rs` and the help text is in
`crates/m365-core/src/help/data/`. To move to a new CLI for Microsoft 365
release, build its checkout in `/tmp/cli-microsoft365` and regenerate both:

```bash
node scripts/extract-upstream.mjs /tmp/cli-microsoft365 > target/commands.json
cargo run -q -p m365-codegen -- target/commands.json
node scripts/gen-help.mjs /tmp/cli-microsoft365 target/commands.json
```

## Release

1. Set `[workspace.package].version` in `Cargo.toml`.
2. Add release notes to `CHANGELOG.md`.
3. Run `cargo check --workspace` to update `Cargo.lock`.
4. Run `just release-check VERSION`.
5. Commit the version, lock file, and changelog.
6. Run `just release VERSION` from a clean `main` branch.

The release command runs all checks, creates the tag, and starts the GitHub
release workflow. The workflow builds profile-guided binaries for macOS,
Linux (glibc and musl) and Windows, publishes the installers, and updates the
Homebrew formula in `racetozero/homebrew-tap`.

## License

MIT

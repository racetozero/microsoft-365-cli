---
name: m365
description: Use the `m365` CLI (a native port of CLI for Microsoft 365) to read and change Microsoft 365 data from a shell. Use for any Microsoft 365 task (Entra ID, SharePoint, Teams, Outlook, Planner, OneDrive, Power Platform, SharePoint Framework), for raw Microsoft Graph or SharePoint REST calls, and to learn the agent mode, errors and login that the suite skills (m365-entra, m365-spo, m365-teams, m365-outlook, m365-files, m365-power-platform, m365-spfx, m365-admin) build on.
---

# m365

Commands read `m365 <suite> <resource> <verb>` with camelCase options, the same as CLI for Microsoft 365 (`m365 entra user get --userName jane@contoso.com`). Always add `--agent` (or set `M365_AGENT=1`).

## Output

With `--agent`, every command prints one JSON line on stdout and never prompts:

```json
{"ok":true,"command":"entra user get","data":{"id":"...","displayName":"Jane Doe"}}
{"ok":false,"command":"spo web get","error":{"code":"input_required","message":"url: Required option not specified","input":{"arg":"url"}}}
```

- `error.code` is one of `failed`, `api`, `auth`, `invalid`, `input_required`, `confirmation_required`, `unknown`, `internal`. HTTP failures add `error.status`.
- Exit codes: 0 ok, 1 command or API error, 2 auth, 3 invalid or missing input, 4 unknown command, 5 internal.
- `--query 'JMESPath'` filters the data before it is printed: `m365 entra user list --query "[].userPrincipalName" --agent`. Use `--properties` (where a command has it) to fetch fewer fields.
- Without `--agent`, `--output json|text|csv|md|none` picks the format (`text` keeps each command's default properties).

## Errors

- `input_required`: run the command again with the option named in `error.input.arg`, one of `error.input.options` (exactly one of them is needed, for example `--id`, `--userName` or `--email`), or a value from `error.input.choices[].value`.
- `confirmation_required`: the command removes or overwrites something. Run it again with `--force` only if the user asked for that change.
- `auth`: ask the user to run `m365 login` (see below). `m365 status --agent` shows the active connection.
- `invalid`: an option value is wrong; the message says which.
- `--debug` prints the HTTP requests and responses on stderr.

## Login

- `m365 login` uses the device code flow; the user opens the URL shown and enters the code. Other `--authType` values: `certificate` (`--certificateFile` or `--certificateBase64Encoded`, `--appId`, `--tenant`), `secret` (`--appId`, `--secret`, `--tenant`), `password` (`--userName`, `--password`), `identity` (managed identity), `federatedIdentity`, `browser`.
- `m365 status --agent` · `m365 connection list --agent` · `m365 connection use --name NAME --agent` · `m365 logout`
- Login files are shared with the Node.js CLI (`~/.cli-m365-connection.json`), so a login made with either works for both.

## Any API call

When no command fits, `m365 request` sends an authenticated request to Microsoft Graph or SharePoint:

- `m365 request --url "https://graph.microsoft.com/v1.0/me" --agent`
- `m365 request --url "https://contoso.sharepoint.com/sites/x/_api/web" --method get --agent`
- `m365 request --url "https://graph.microsoft.com/v1.0/me/events" --method post --body '{"subject":"Sync"}' --agent` (`--body @file.json` reads a file; `--filePath` saves a binary response).

## Suites

Load the suite skill for its commands: `m365-entra` (users, groups, apps), `m365-spo` (SharePoint sites, lists, files, pages), `m365-teams`, `m365-outlook` (mail, calendar, Planner, To Do, Bookings, OneNote), `m365-files` (OneDrive, files, SharePoint Embedded), `m365-power-platform` (Power Platform, Power Apps, Power Automate), `m365-spfx` (SharePoint Framework projects), `m365-admin` (tenant, Purview, Viva Engage, Graph connectors and extensions). `m365 <suite> --help` lists the commands of a suite and `m365 <command> --help` prints the options, rules (such as "Use exactly one of: --id, --userName"), examples and permissions. `m365 <command> --help --agent` returns the same as JSON: `options` (name, type, required, values, description), `rules`, `examples` and `permissions`.

## Settings, updates and skills

- `m365 cli config set --key output --value json` sets a default; `m365 cli config list --agent` shows them.
- `m365 update --check --agent` reports whether a newer release exists; `m365 update` installs it (Homebrew installs are upgraded with `brew upgrade`).
- `m365 skill` prints this skill, `m365 skill m365-spo` a suite skill, `m365 skill --list` lists them and `m365 skill --install [DIR]` writes them all as `DIR/<name>/SKILL.md` (default `.claude/skills`).

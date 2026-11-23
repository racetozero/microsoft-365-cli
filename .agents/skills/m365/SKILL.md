---
name: m365
description: Use the `m365` CLI (a native port of CLI for Microsoft 365 with all 888 commands) to read and change Microsoft 365 data from a shell. Use for any Microsoft 365 task (Entra ID, SharePoint, Teams, Outlook, Planner, To Do, OneDrive, SharePoint Embedded, Power Platform, Purview, Viva, SharePoint Framework), for raw Microsoft Graph or SharePoint REST calls, and to learn the agent mode, errors, login, settings and folder context that the suite skills build on.
---

# m365

Commands read `m365 <suite> <resource> <verb>` with camelCase options, the same as CLI for Microsoft 365 v12.1.0 (`m365 entra user get --userName jane@contoso.com`). Always add `--agent` (or set `M365_AGENT=1`).

## Nuances

- Login needs an Entra app ID. A fresh install fails with `input_required` on `appId` until the user runs `m365 setup` once, or passes `--appId` and `--tenant` to `m365 login`, or sets `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT`.
- Device code and browser logins need a person. Ask the user to run `m365 login` themselves; do not start it from an agent and wait.
- Options are validated before the login check, so a wrong or missing option fails fast even when signed out. The login check runs before `confirmation_required`.
- Names are looked up for you: `--userName`, `--groupName`, `--listTitle` and similar options cost one extra request each. Pass IDs in loops.
- An option that takes a GUID rejects other text with `invalid`, so do not pass a name to `--id`.
- Any option value `@meId` or `@meUserName` becomes the signed-in user's ID or user name (`--userId @meId`). It fails with an app login.
- When `accepts_other_options` is true in a command's JSON help, unknown options pass through as data: SharePoint field values on `m365 spo listitem add`, user properties on `m365 entra user add`.
- `--force` skips the confirmation of a remove, overwrite or reset. Add it only when the user asked for that change.
- `--output text` keeps a list command's default columns; JSON returns every property. Prefer `--query` or `--properties` over large JSON.
- A `.m365rc.json` in the working folder can supply option values (see Context). It can make a command act on a site you did not name.

## Output

With `--agent`, every command prints one JSON line on stdout and never prompts:

```json
{"ok":true,"command":"entra user get","data":{"id":"...","displayName":"Jane Doe"}}
{"ok":false,"command":"spo web get","error":{"code":"input_required","message":"url: Required option not specified","input":{"arg":"url"}}}
```

- `data` is `null` when a command prints nothing and an array when it prints more than one value.
- `error.code` is one of `failed`, `api`, `auth`, `invalid`, `input_required`, `confirmation_required`, `unknown`, `internal`. HTTP failures add `error.status`.
- Exit codes: 0 ok, 1 command or API error, 2 auth, 3 invalid or missing input (also confirmation), 4 unknown command, 5 internal.
- `--query 'JMESPath'` filters `data` before it is printed: `m365 entra user list --query "[].userPrincipalName" --agent`.
- `--agent` always prints JSON and ignores `--output`. Without `--agent`, `--output json|text|csv|md|none` picks the format and `m365 cli config set --key output --value text` changes the default.
- `--debug` prints HTTP requests and responses on stderr; `--verbose` prints progress.

## Errors

- `input_required`: run again with the option in `error.input.arg` (pick a value from `error.input.choices[].value` when present), or with exactly one of `error.input.options` (for example `--id`, `--userName` or `--email`).
- `confirmation_required`: the command removes or overwrites something. Run it again with `--force` only if the user asked for that change.
- `auth`: ask the user to sign in (see Login). `m365 status --agent` shows the active connection.
- `invalid`: an option value is wrong; the message names the option and the allowed values.
- `unknown`: the command does not exist. Run `m365` with `--help --agent` to list every command with a summary.
- `api`: Microsoft 365 refused the request. Read `error.status` and the message; a 403 usually means the signed-in app or user lacks the permission listed in the command's help.

## Find a command

- `--help --agent` on `m365` alone lists all 888 commands with a summary; `m365 spo --help --agent` lists one suite.
- `m365 spo listitem add --help --agent` returns `usage`, `options` (name, type, required, values, description), `rules` (such as "Use exactly one of: --listId, --listTitle, --listUrl"), `accepts_other_options`, `outputs`, `examples`, `notes` and `permissions`.
- Without `--agent`, `-h options`, `-h examples`, `-h remarks` and `-h permissions` print one part of the help.

## Login

- `m365 setup` is an interactive wizard: it creates or reuses an Entra app, saves its ID as `clientId`, and sets presets. `m365 setup --scripting` (or `--interactive`) applies that preset without the preset questions; `--skipApp` skips the app step.
- `m365 cli app add --scopes minimal --saveToConfig` creates the app registration without the wizard (`--scopes all` or a list of scope URLs for more). `m365 cli app reconsent` adds scopes that a newer CLI needs.
- `m365 login` uses the device code flow. Other `--authType` values: `browser`, `password` (`--userName`, `--password`), `certificate` (`--certificateFile` or `--certificateBase64Encoded`, optional `--password` and `--thumbprint`), `secret` (`--secret`), `identity` (Azure managed identity), `federatedIdentity` (GitHub Actions, Azure DevOps and GitLab CI/CD OIDC).
- `certificate`, `secret` and `federatedIdentity` logins need `--appId` and a real `--tenant` (not `common`). App logins act as the app, so commands that default to the signed-in user need `--userName` or `--userId`.
- `--cloud USGov|USGovHigh|USGovDoD|China` signs in to a sovereign cloud; it needs an app registered in that cloud.
- `m365 login --ensure` signs in only when there is no valid login. `--connectionName work` names the connection.
- `m365 status --agent` · `m365 connection list --agent` · `m365 connection use --name work --agent` · `m365 connection set --name OLD --newName work --agent` · `m365 connection remove --name work --force --agent` · `m365 logout`
- Logins are saved in `~/.cli-m365-connection.json` and shared with the Node.js CLI.
- Viva Engage commands need an extra permission: `m365 cli consent --service VivaEngage` prints a consent URL for the user.

## Any API call

When no command fits, `m365 request` sends an authenticated request to Microsoft Graph, SharePoint or another Entra-protected API:

- `m365 request --url "@graph/me" --agent` (`@graph` is `https://graph.microsoft.com/v1.0`, `@graphbeta` the beta endpoint, `@spo` the SharePoint root URL)
- `m365 request --url "https://contoso.sharepoint.com/sites/x/_api/web" --method get --agent`
- `m365 request --url "@graph/me/events" --method post --content-type "application/json" --body '{"subject":"Sync"}' --agent`
- A body needs `--method` and `--content-type`; `--body @file.json` reads a file and `--filePath ./out.bin` saves a binary response.
- Extra headers are options: `--if-match "*"`, `--accept "application/json;odata=nometadata"`.
- `--resource https://management.azure.com` picks the token audience when the URL does not imply it.
- Quote URLs that contain `$filter` or `$select` in single quotes so the shell does not expand `$`.
- `m365 util accesstoken get --resource graph --agent` returns a token for another tool (`sharepoint` or any resource URI; `--new` forces a fresh one, `--decoded` shows the claims).
- `m365 spo set --url https://contoso.sharepoint.com --agent` sets the SharePoint root URL (needed for multi-geo); `m365 spo get --agent` reads it.

## Context and project apps

A `.m365rc.json` file in the current folder holds default option values and Entra apps for the project.

- `m365 context init --agent` creates it. `m365 context option set --name webUrl --value https://contoso.sharepoint.com/sites/sales --agent` makes every command that takes `--webUrl` use that site when the option is left out.
- `m365 context option list --agent` · `m365 context option remove --name webUrl --force --agent` · `m365 context remove --force --agent`
- `m365 entra app add --name "My app" --save --agent` stores the new app in `.m365rc.json`. Then `m365 app get --agent`, `m365 app permission list --agent` and `m365 app permission add --delegatedPermissions "https://graph.microsoft.com/Mail.Read" --grantAdminConsent --agent` act on it (`--appId` picks one when the file has several; without it the command prompts). `m365 app open --agent` returns its Entra portal URL.

## Settings and diagnostics

- `m365 cli config set --key output --value json` · `m365 cli config get --key output --agent` · `m365 cli config list --agent` · `m365 cli config reset --key output` (no `--key` resets all).
- Useful keys: `output`, `errorOutput` (`stdout` helps in PowerShell), `prompt` (false stops prompts without `--agent`), `helpMode`, `authType`, `clientId`, `tenantId`, `autoOpenLinksInBrowser`, `copyDeviceCodeToClipboard`, `showHelpOnFailure`, `csvHeader`, `csvQuoted`.
- `m365 cli doctor --agent` reports the OS, CLI version, login type, app ID, tenant, roles, scopes and settings without secrets. Attach it to bug reports.
- `m365 cli issue --type bug` returns the URL to open an issue. `m365 docs` returns the documentation URL. `m365 version --agent` prints the version.
- Shell completion: `m365 cli completion sh setup` (Zsh, Bash, Fish), `m365 cli completion pwsh setup --profile $PROFILE`, `m365 cli completion clink update` (cmder). Run the `update` form again after an upgrade.
- Environment variables: `M365_AGENT`, `CLIMICROSOFT365_ENTRAAPPID`, `CLIMICROSOFT365_TENANT`, `M365_NO_BROWSER` (never open a browser), `M365_NO_UPDATE_NOTIFIER` (no update notice), `M365_SELF_UPDATE_COMMAND` (how `m365 update` upgrades).

## Updates and skills

- `m365 update --check --agent` reports whether a newer release exists; `m365 update` installs it (Homebrew installs are upgraded with `brew upgrade`).
- `m365 skill` prints this skill, `m365 skill m365-spo` a suite skill, `m365 skill --list` lists them and `m365 skill --install [DIR]` writes them all as `DIR/<name>/SKILL.md` (default `.claude/skills`).

## Suites

Load the suite skill for its commands, nuances and full command map:

- `m365-entra`: users, groups, Microsoft 365 groups, apps and permissions, licenses, roles, PIM, administrative units, policies, Exchange app roles.
- `m365-spo`: SharePoint sites, webs, lists, items, views, fields, content types, files, folders, pages, navigation, permissions, sharing links, retention labels, search.
- `m365-spo-admin`: SharePoint tenant settings, app catalogs and apps, hub and home sites, site designs, themes, CDN, term store, API access, SharePoint Premium.
- `m365-teams`: teams, channels, tabs, messages, chats, meetings, transcripts, call records, apps, team settings.
- `m365-outlook`: mail, calendars, events, rooms, Planner, To Do, Bookings, OneNote.
- `m365-files`: files through Graph, OneDrive sites, SharePoint Embedded.
- `m365-power-platform`: environments, solutions, Dataverse, Copilot Studio, Power Pages, Power Apps, Power Automate.
- `m365-spfx`: SharePoint Framework project doctor, upgrade, pipelines, packages.
- `m365-admin`: tenant info, service health, usage reports for every service, Purview, Viva Engage and Connections, Search connectors, Graph extensions and subscriptions, Adaptive Cards.

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `login`: (itself)
- `logout`: (itself)
- `status`: (itself)
- `setup`: (itself)
- `request`: (itself)
- `version`: (itself)
- `docs`: (itself)
- `connection`: list, remove, set, use
- `context`: init, remove
- `context option`: list, remove, set
- `app`: get, open — the Entra apps in `.m365rc.json`
- `app permission`: add, list
- `util accesstoken`: get
- `cli`: consent, doctor, issue
- `cli app`: add, reconsent
- `cli config`: get, list, reset, set
- `cli completion sh`: setup, update
- `cli completion pwsh`: setup, update
- `cli completion clink`: update

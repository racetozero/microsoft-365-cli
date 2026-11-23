---
name: m365-power-platform
description: Work with Power Platform, Power Apps and Power Automate through the `m365` CLI. Covers environments, solutions and publishers, Dataverse tables and rows, Copilot Studio copilots, AI Builder models, gateways, Power Pages websites, tenant settings, canvas apps with their owners, permissions and consent, custom connectors, and flows with their runs, owners and recycle bin. Builds on the `m365` skill.
---

# Power Platform with m365

Add `--agent` to every command (see the `m365` skill). Most commands need `--environmentName`, the environment ID (for example `Default-<tenantId>`), not its display name. Get it from `m365 pp environment list --agent`. Apps, flows and runs are addressed by their GUID `--name`, not the display name.

## Nuances

- `--asAdmin` acts on environments, apps and flows you have no permission to. Without it, commands see only your own items.
- `m365 pa app list` needs `--environmentName` and `--asAdmin` together, or neither (then it lists only your own apps). `m365 pa app permission ensure` has the same pairing.
- `m365 flow list` takes `--sharingStatus all|personal|ownedByMe|sharedWithMe` (default `ownedByMe`) or `--asAdmin`, not both. Add `--withSolutions` to include flows in solutions.
- `m365 pp environment list` and `get` use an admin API that needs a registered management app. Register one once with `m365 pp managementapp add --appId APP_ID`; the first time, sign in with the Azure PowerShell app (`m365 login --appId 1950a258-227b-4e31-a9cf-717495945fc2`).
- A missing environment gives `Access to the environment 'xyz' is denied.`; a missing flow gives a `does not have permission for connection ... 'shared_logicflows'` error. Check the IDs before you assume a permission problem.
- `m365 flow export` writes a `.zip` package (default) or `--format json`; the `--package*` options work only with zip. `m365 pa app export` writes a zip to `--path` or the current folder. `m365 pa connector export --outputFolder` must already exist; the command then creates a subfolder named after the connector and fails if it exists.
- `m365 flow run list --withTrigger` sends one extra request per run and needs JSON output.
- A flow restored with `m365 flow recyclebinitem restore` is disabled; enable it with `m365 flow enable`. Restore needs a Global or Power Platform admin, as does `m365 pa app owner set`.
- Dataverse commands take exactly one of `--tableName` (logical name, for example `account`) or `--entitySetName` (for example `accounts`).
- `m365 pp solution publish` takes the solution's unique `--name`, not its display name; add `--wait` to wait for the job.
- Sharing an app with the whole tenant (`--tenant`) allows only `--roleName CanView`. Group sharing works only with security groups.
- Many commands use preview APIs; output fields can change.

## Environments and solutions

- `m365 pp environment list --agent [--asAdmin]` · `m365 pp environment get --name ENV --agent` (or `--default`)
- `m365 pp solution list --environmentName ENV --agent` · `m365 pp solution get --environmentName ENV --name SOLUTION --agent` · `m365 pp solution publish --environmentName ENV --name SOLUTION --wait --agent`
- `m365 pp solution publisher add --environmentName ENV --name contoso --displayName Contoso --prefix con --choiceValuePrefix 10000 --agent`
- `m365 pp tenant settings list --agent` · `m365 pp tenant settings set --disableTrialEnvironmentCreationByNonAdminUsers true --agent`

## Dataverse, copilots and more

- `m365 pp dataverse table list --environmentName ENV --agent` · `m365 pp dataverse table get --environmentName ENV --name account --agent`
- `m365 pp dataverse table row list --environmentName ENV --tableName account --agent` · `m365 pp dataverse table row remove --environmentName ENV --tableName account --id ROW_ID --force --agent`
- `m365 pp copilot list --environmentName ENV --agent` · `m365 pp aibuildermodel list --environmentName ENV --agent` · `m365 pp gateway list --agent`
- `m365 pp website get --environmentName ENV --name Demo --agent` · `m365 pp website webrole list --environmentName ENV --websiteName Demo --agent`

## Power Apps

- `m365 pa app list --agent` (your apps) · `m365 pa app list --environmentName ENV --asAdmin --agent` (all apps) · `m365 pa app get --name APP_ID --agent`
- `m365 pa app export --name APP_ID --environmentName ENV --agent [--path ./export]`
- `m365 pa app owner set --environmentName ENV --appName APP_ID --userName jane@contoso.com --agent [--roleForOldAppOwner CanEdit]`
- `m365 pa app permission ensure --appName APP_ID --userName jane@contoso.com --roleName CanView --agent [--sendInvitationMail]` · `m365 pa app permission list --appName APP_ID --agent`
- `m365 pa app consent set --environmentName ENV --name APP_ID --bypass true --force --agent` (canvas apps only)
- `m365 pa connector list --environmentName ENV --agent` · `m365 pa connector export --environmentName ENV --name CONNECTOR_NAME --agent [--outputFolder ./connectors]`

## Power Automate

- `m365 flow list --environmentName ENV --agent [--sharingStatus all]` (or `--asAdmin`) · `m365 flow get --environmentName ENV --name FLOW_ID --agent`
- `m365 flow enable --environmentName ENV --name FLOW_ID --agent` · `m365 flow disable --environmentName ENV --name FLOW_ID --agent`
- `m365 flow run list --environmentName ENV --flowName FLOW_ID --agent [--status Failed] [--triggerStartTime 2026-01-01T00:00:00Z]`
- `m365 flow run get --environmentName ENV --flowName FLOW_ID --name RUN_ID --agent` · `m365 flow run resubmit --environmentName ENV --flowName FLOW_ID --name RUN_ID --force --agent` · `m365 flow run cancel --environmentName ENV --flowName FLOW_ID --name RUN_ID --force --agent`
- `m365 flow export --environmentName ENV --name FLOW_ID --agent [--format zip --path ./flow.zip]`
- `m365 flow owner list --environmentName ENV --flowName FLOW_ID --agent` · `m365 flow owner ensure --environmentName ENV --flowName FLOW_ID --userName jane@contoso.com --roleName CanEdit --agent`
- `m365 flow recyclebinitem list --environmentName ENV --agent` · `m365 flow recyclebinitem restore --environmentName ENV --flowName FLOW_ID --agent`

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

**Power Platform (`pp`)**

- `pp environment`: get, list
- `pp managementapp`: add, list
- `pp tenant settings`: list, set
- `pp solution`: get, list, publish, remove
- `pp solution publisher`: add, get, list, remove
- `pp dataverse table`: get, list, remove
- `pp dataverse table row`: list, remove
- `pp copilot`: get, list, remove — Copilot Studio copilots
- `pp aibuildermodel`: get, list, remove
- `pp gateway`: get, list — on-premises data gateways
- `pp website`: get, remove — Power Pages
- `pp website webrole`: list

**Power Apps (`pa`)**

- `pa environment`: get, list
- `pa app`: export, get, list, remove
- `pa app owner`: set
- `pa app permission`: ensure, list, remove
- `pa app consent`: set
- `pa connector`: export, list — custom connectors

**Power Automate (`flow`)**

- `flow environment`: get, list
- `flow`: disable, enable, export, get, list, remove
- `flow run`: cancel, get, list, resubmit
- `flow owner`: ensure, list, remove
- `flow recyclebinitem`: list, restore

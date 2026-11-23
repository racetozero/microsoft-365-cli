---
name: m365-power-platform
description: Work with Power Platform, Power Apps and Power Automate through the `m365` CLI. Covers environments, solutions, Dataverse tables and rows, Copilot Studio bots, AI Builder models, gateways, canvas apps and their owners and permissions, connectors, and flows with their runs and owners. Builds on the `m365` skill.
---

# Power Platform with m365

Add `--agent` to every command (see the `m365` skill). Most commands need `--environmentName` (the environment ID, for example `Default-<tenantId>`); `--asAdmin` acts across the tenant.

## Environments and solutions

- `m365 pp environment list --agent [--asAdmin]` · `m365 pp environment get --name ENV --agent`
- `m365 pp solution list --environmentName ENV --agent` · `m365 pp solution get --environmentName ENV --name SOLUTION --agent` · `m365 pp solution publish --environmentName ENV --name SOLUTION --agent`
- `m365 pp dataverse table list --environmentName ENV --agent` · `m365 pp dataverse table row list --environmentName ENV --tableName account --agent`
- `m365 pp copilot list --environmentName ENV --agent` · `m365 pp aibuildermodel list --environmentName ENV --agent` · `m365 pp gateway list --agent`

## Power Apps

- `m365 pa app list --agent [--environmentName ENV] [--asAdmin]` · `m365 pa app get --name APP_ID --agent`
- `m365 pa app export --name APP_ID --environmentName ENV --agent [--path ./app.zip]`
- `m365 pa app owner set --environmentName ENV --appName APP_ID --userName jane@contoso.com --agent`
- `m365 pa connector list --environmentName ENV --agent`

## Power Automate

- `m365 flow list --environmentName ENV --agent [--sharingStatus ownedByMe] [--asAdmin]` · `m365 flow get --environmentName ENV --name FLOW_ID --agent`
- `m365 flow enable --environmentName ENV --name FLOW_ID --agent` · `m365 flow disable --environmentName ENV --name FLOW_ID --agent`
- `m365 flow run list --environmentName ENV --flowName FLOW_ID --agent [--status Failed]` · `m365 flow run resubmit --environmentName ENV --flowName FLOW_ID --name RUN_ID --agent`
- `m365 flow export --environmentName ENV --name FLOW_ID --agent [--format zip --path ./flow.zip]`
- `m365 flow owner list --environmentName ENV --flowName FLOW_ID --agent`

---
name: m365-admin
description: Administer a Microsoft 365 tenant with the `m365` CLI. Covers tenant information, service health and messages, usage reports, Purview retention and sensitivity labels and audit logs, Viva Engage communities and messages, Microsoft Search connectors, Graph extensions and subscriptions, and Adaptive Card webhooks. Builds on the `m365` skill.
---

# Tenant administration with m365

Add `--agent` to every command (see the `m365` skill). Most of these need an admin role.

## Tenant and service health

- `m365 tenant info get --domainName contoso.com --agent` · `m365 tenant id get --agent`
- `m365 tenant serviceannouncement health list --agent [--issues]` · `m365 tenant serviceannouncement message list --agent [--service "SharePoint Online"]`
- Usage reports take `--period D7|D30|D90|D180`: `m365 tenant report activeusercounts --period D30 --agent` (also `outlook report`, `onedrive report`, `teams report`, `spo report`, `viva engage report`).

## Purview

- `m365 purview retentionlabel list --agent` · `m365 purview sensitivitylabel list --agent`
- `m365 purview auditlog list --contentType SharePoint --startTime 2026-01-01T00:00:00Z --agent`
- `m365 purview retentionevent list --agent` · `m365 purview threatassessment list --agent`

## Viva Engage

- `m365 viva engage community list --agent` · `m365 viva engage message list --agent`
- `m365 viva engage message add --body "Welcome" --groupId GROUP_ID --agent`

## Search connectors and Graph

- `m365 external connection list --agent` · `m365 external connection add --id contosohr --name "HR" --description "HR documents" --agent`
- `m365 graph subscription add --resource "me/mailFolders('inbox')/messages" --changeTypes created --notificationUrl https://contoso.com/hook --agent`
- `m365 graph schemaextension list --agent` · `m365 graph openextension list --resourceId USER_ID --resourceType user --agent`
- `m365 graph changelog list --agent [--services "Microsoft Teams"]`

## Other

- `m365 adaptivecard send --url WEBHOOK_URL --title "Deployed" --description "v2 is live" --agent`
- `m365 search --queryText "budget" --scopes driveItem --agent` searches Microsoft 365 content (Microsoft Search).

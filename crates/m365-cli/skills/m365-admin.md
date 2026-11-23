---
name: m365-admin
description: Administer a Microsoft 365 tenant with the `m365` CLI. Covers tenant information, service health, security alerts, profile card and pronoun settings, every usage report (tenant, Outlook, OneDrive, SharePoint, Teams, Viva Engage, Microsoft 365 groups), Purview retention, sensitivity labels, audit logs and threat assessments, Viva Engage and Viva Connections, Microsoft Search and Copilot connectors, Graph extensions, subscriptions and changelog, Microsoft Search queries and Adaptive Card webhooks. Builds on the `m365` skill.
---

# Tenant administration with m365

Add `--agent` to every command (see the `m365` skill). Most of these commands need an admin role (Global, Reports, Compliance or SharePoint Administrator).

## Nuances

- Usage reports accept only `json` and `csv` output. With `--agent` you get an array of row objects whose values are all strings (for example `"Report Refresh Date"`). The column names come from the Microsoft report.
- Report options differ by report. Most `*counts`, `*storage` and `*pages` reports need `--period D7|D30|D90|D180`. The `*detail` reports take `--period` or `--date YYYY-MM-DD` (one, not both); a date must be in the last 28 days. `tenant report office365activation*` take no options. `teams report pstncalls` and `directroutingcalls` take `--fromDateTime` (and optional `--toDateTime`, at most 90 days) and work only with an app-only login that has `CallRecords.Read.All`.
- Reports show concealed (hashed) user, group and site names when the tenant hides them. `m365 tenant report settings get --agent` shows the setting; `m365 tenant report settings set --displayConcealedNames false --agent` turns it off.
- Viva Engage message, network, search and user commands need the Viva Engage API permission. Run `m365 cli consent --service VivaEngage` once and open the URL it prints. They do not support application permissions. Community and role commands use Microsoft Graph and need no consent.
- `m365 viva engage search --show` works only with `--output text`, so do not use it with `--agent`; filter with `--query` instead.
- `m365 purview auditlog list` needs unified audit logging. The first call for a content type creates a subscription, and data can take up to 12 hours to appear. The range defaults to the last 24 hours; `--startTime` (ISO 8601) cannot be more than 7 days in the past, so older entries are not available here. Large tenants are slow; keep the range short.
- `m365 purview threatassessment add` supports only delegated logins.
- Profile card changes can take up to 24 hours to show. `customAttribute1` to `customAttribute15` need `--displayName`.
- `m365 external connection add --id` must be 3 to 32 alphanumeric characters and must not start with `Microsoft`. Add a schema before you add items. `m365 external connection doctor --id ID --ux copilot` checks that a connection works with Copilot.
- `m365 external item add` maps extra options to item properties (`--title "..."`). Separate multi-values with `;#` and give the type, for example `--authors@odata.type "Collection(String)"`. `--acls` uses `accessType,type,value;...`, for example `grant,everyone,everyone`.
- `m365 graph subscription add --notificationUrl` must be `https://`, `EventHub:https://` or `EventGrid:`. `--changeTypes` takes `created`, `updated` and `deleted`.
- `m365 graph openextension add` and `set` take the extension's data as extra options (`--theme dark`).
- `m365 search` (Microsoft Search) needs `--scopes` and takes KQL in `--queryText`. For SharePoint search with managed properties use `m365 spo search` (`m365-spo` skill).

## Tenant and service health

- `m365 tenant id get --agent` · `m365 tenant info get --agent` (your tenant) or `m365 tenant info get --domainName contoso.com --agent`
- `m365 tenant serviceannouncement health list --agent [--issues]` · `m365 tenant serviceannouncement healthissue list --service "SharePoint Online" --agent`
- `m365 tenant serviceannouncement message list --service "Microsoft Teams" --agent` · `m365 tenant serviceannouncement message get --id MC123456 --agent`
- `m365 tenant security alerts list --agent [--vendor "Azure Sentinel"]`
- `m365 tenant people profilecardproperty add --name customAttribute1 --displayName "Cost center" --agent` · `m365 tenant people pronouns set --enabled true --agent`

## Usage reports

- `m365 tenant report activeusercounts --period D30 --agent` · `m365 tenant report activeuserdetail --period D7 --agent` (or `--date` with a day in the last 28 days)
- `m365 teams report useractivityuserdetail --period D7 --agent` · `m365 outlook report mailboxusagedetail --period D30 --agent`
- `m365 spo report siteusagedetail --period D30 --agent` · `m365 onedrive report usageaccountdetail --period D90 --agent`
- `m365 tenant report office365activationsuserdetail --agent` · `m365 teams report pstncalls --fromDateTime 2026-01-01 --toDateTime 2026-01-31 --agent`
- Save a report as a file with `m365 spo report siteusagedetail --period D30 --output csv > sites.csv` (no `--agent`).

## Purview

- `m365 purview retentionlabel list --agent` · `m365 purview retentionlabel add --displayName "Contracts" --behaviorDuringRetentionPeriod retain --actionAfterRetentionPeriod delete --retentionDuration 2555 --agent [--retentionTrigger dateCreated]`
- Event-based retention: `m365 purview retentioneventtype add --displayName "Contract end" --agent`, a label with `--retentionTrigger dateOfEvent --eventTypeName "Contract end"`, then `m365 purview retentionevent add --displayName "Contoso contract ended" --eventTypeName "Contract end" --assetIds "ContractId:1234" --agent`
- `m365 purview sensitivitylabel list --agent [--userName jane@contoso.com]` · `m365 purview sensitivitylabel policysettings list --agent`
- `m365 purview auditlog list --contentType SharePoint --agent` (last 24 hours; also `AzureActiveDirectory`, `Exchange`, `General`, `DLP`). For another range, add `--startTime` and `--endTime` within the last 7 days.
- `m365 purview threatassessment add --type url --expectedAssessment block --category phishing --url https://contoso.example --agent` · `m365 purview threatassessment list --agent`

## Viva

- `m365 viva engage network list --agent` · `m365 viva engage community list --agent` · `m365 viva engage community add --displayName "Engineers" --description "All engineers" --privacy public --wait --agent`
- `m365 viva engage community user add --communityDisplayName "Engineers" --userNames jane@contoso.com --role Member --agent`
- `m365 viva engage message list --feedType Following --limit 20 --agent` · `m365 viva engage message add --body "Welcome" --groupId GROUP_ID --agent` (or `--repliedToId`, `--directToUserIds`)
- `m365 viva engage search --queryText "launch" --agent` · `m365 viva engage role member add --roleName "Verified Admin" --userName jane@contoso.com --agent`
- `m365 viva connections app create --portalUrl https://contoso.sharepoint.com --name Contoso --description "Contoso app" --longDescription "Stay up to date" --companyName Contoso --companyWebsiteUrl https://contoso.com --coloredIconPath color.png --outlineIconPath outline.png --agent` writes a Teams app package; the portal must be a communication site. Upload it with `m365 teams app publish`.

## Connectors, Graph and messages

- `m365 external connection add --id contosohr --name "HR" --description "HR documents" --agent` · `m365 external connection schema add --externalConnectionId contosohr --schema @schema.json --wait --agent`
- `m365 external item add --id doc1 --externalConnectionId contosohr --content "Leave policy" --acls "grant,everyone,everyone" --title "Leave policy" --agent`
- `m365 graph subscription add --resource "me/mailFolders('inbox')/messages" --changeTypes created --notificationUrl https://contoso.com/hook --agent [--clientState SECRET]` (without `--expirationDateTime` the CLI asks for the longest expiry the resource allows)
- `m365 graph schemaextension list --agent` · `m365 graph directoryextension add --name costCenter --appName ContosoApp --targetObjects User --dataType String --agent`
- `m365 graph openextension add --resourceId jane@contoso.com --resourceType user --name com.contoso.settings --theme dark --agent`
- `m365 graph changelog list --services "Users,Groups" --startDate 2026-01-01 --agent`
- `m365 search --scopes driveItem --queryText "budget" --agent [--select name,webUrl] [--allResults]`
- `m365 adaptivecard send --url WEBHOOK_URL --title "Deployed" --description "v2 is live" --agent` (or `--card @card.json --cardData @data.json`)

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

**Tenant**

- `tenant id`: get
- `tenant info`: get
- `tenant serviceannouncement health`: get, list
- `tenant serviceannouncement healthissue`: get, list
- `tenant serviceannouncement message`: get, list
- `tenant security alerts`: list
- `tenant people profilecardproperty`: add, get, list, remove, set
- `tenant people pronouns`: get, set

**Usage reports**

- `tenant report`: activeusercounts, activeuserdetail, office365activationcounts, office365activationsusercounts, office365activationsuserdetail, servicesusercounts
- `tenant report settings`: get, set
- `outlook report`: mailactivitycounts, mailactivityusercounts, mailactivityuserdetail, mailappusageappsusercounts, mailappusageusercounts, mailappusageuserdetail, mailappusageversionsusercounts, mailboxusagedetail, mailboxusagemailboxcount, mailboxusagequotastatusmailboxcounts, mailboxusagestorage
- `onedrive report`: activityfilecounts, activityusercounts, activityuserdetail, usageaccountcounts, usageaccountdetail, usagefilecounts, usagestorage
- `spo report`: activityfilecounts, activitypages, activityusercounts, activityuserdetail, siteusagedetail, siteusagefilecounts, siteusagepages, siteusagesitecounts, siteusagestorage
- `teams report`: deviceusagedistributionusercounts, deviceusageusercounts, deviceusageuserdetail, directroutingcalls, pstncalls, useractivitycounts, useractivityusercounts, useractivityuserdetail
- `viva engage report`: activitycounts, activityusercounts, activityuserdetail, deviceusagedistributionusercounts, deviceusageusercounts, deviceusageuserdetail, groupsactivitycounts, groupsactivitydetail, groupsactivitygroupcounts
- `entra m365group report`: activitycounts, activitydetail, activityfilecounts, activitygroupcounts, activitystorage

**Purview**

- `purview auditlog`: list
- `purview retentionlabel`: add, get, list, remove, set
- `purview retentioneventtype`: add, get, list, remove, set
- `purview retentionevent`: add, get, list, remove
- `purview sensitivitylabel`: get, list
- `purview sensitivitylabel policysettings`: list
- `purview threatassessment`: add, get, list

**Viva**

- `viva engage network`: list
- `viva engage community`: add, get, list, remove, set
- `viva engage community user`: add, list, remove
- `viva engage message`: add, get, list, remove
- `viva engage message like`: set
- `viva engage role`: list
- `viva engage role member`: add, list, remove
- `viva engage user`: get, list
- `viva engage`: search
- `viva connections app`: create

**Connectors, Graph and messages**

- `external connection`: add, doctor, get, list, remove
- `external connection schema`: add
- `external connection urltoitemresolver`: add
- `external item`: add
- `graph subscription`: add
- `graph schemaextension`: add, get, list, remove, set
- `graph directoryextension`: add, get, list, remove
- `graph openextension`: add, get, list, remove, set
- `graph changelog`: list
- `search`: (itself) — Microsoft Search across Microsoft 365
- `adaptivecard`: send

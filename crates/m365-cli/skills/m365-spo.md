---
name: m365-spo
description: Work with SharePoint Online through the `m365` CLI. Covers sites and webs, lists and list items, files and folders, pages, fields and content types, users, groups and permissions, the app catalog, search, property bags, custom actions and tenant settings. Builds on the `m365` skill.
---

# SharePoint with m365

Add `--agent` to every command (see the `m365` skill). Sites are addressed by absolute URL (`--webUrl https://contoso.sharepoint.com/sites/sales`); lists by `--listTitle`, `--listId` or `--listUrl`.

## Sites

- `m365 spo site list --agent [--type TeamSite|CommunicationSite] [--filter "Url -like 'sales'"]`
- `m365 spo site get --url https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo web get --url URL --agent [--withPermissions]`
- `m365 spo site add --type CommunicationSite --title "Sales" --url https://contoso.sharepoint.com/sites/sales --agent` (`--type TeamSite --alias sales` creates a group-connected site)
- `m365 spo site sharingpermission set --siteUrl URL --capability ownersOnly --agent`

## Lists and items

- `m365 spo list list --webUrl URL --agent` · `m365 spo list get --webUrl URL --title "Projects" --agent`
- `m365 spo listitem list --webUrl URL --listTitle "Projects" --agent [--fields Title,Status] [--filter "Status eq 'Open'"] [--camlQuery "<View>...</View>"]`
- `m365 spo listitem get --webUrl URL --listTitle "Projects" --id 3 --agent`
- Field values are extra options named after the internal field name: `m365 spo listitem add --webUrl URL --listTitle "Projects" --Title "New" --Status Open --agent` · `m365 spo listitem set --webUrl URL --listTitle "Projects" --id 3 --Status Done --agent [--systemUpdate]`
- `m365 spo listitem remove --webUrl URL --listTitle "Projects" --id 3 --force --agent`
- `m365 spo field list --webUrl URL --listTitle "Projects" --agent` gives internal names.

## Files and folders

- `m365 spo folder list --webUrl URL --parentFolderUrl "/Shared Documents" --agent [--recursive]` · `m365 spo folder add --webUrl URL --parentFolderUrl "/Shared Documents" --name "Reports" --agent`
- `m365 spo file list --webUrl URL --folderUrl "/Shared Documents" --agent [--recursive]`
- `m365 spo file get --webUrl URL --url "/sites/sales/Shared Documents/a.docx" --agent` (`--asString` returns the content, `--asFile --path out.docx` downloads it)
- `m365 spo file add --webUrl URL --folder "Shared Documents" --path ./report.pdf --agent [--overwrite]`

## Pages

- `m365 spo page list --webUrl URL --agent` · `m365 spo page get --webUrl URL --name home.aspx --agent`
- `m365 spo page add --webUrl URL --name news.aspx --title "News" --agent [--layoutType Article] [--promoteAs NewsPage] [--publish]`
- `m365 spo page text add --webUrl URL --pageName news.aspx --text "Hello" --agent` · `m365 spo page clientsidewebpart add --webUrl URL --pageName news.aspx --standardWebPart BingMap --agent`

## Permissions and users

- `m365 spo user list --webUrl URL --agent` · `m365 spo group member add --webUrl URL --groupName "Sales Members" --userNames jane@contoso.com --agent`
- `m365 spo web roleassignment add --webUrl URL --upn jane@contoso.com --roleDefinitionName "Read" --agent`

## Apps, search and settings

- `m365 spo app list --agent [--appCatalogScope sitecollection --appCatalogUrl URL]` · `m365 spo app add --filePath ./solution.sppkg --overwrite --agent` · `m365 spo app deploy --name solution.sppkg --agent [--skipFeatureDeployment]`
- `m365 spo search --queryText "contract" --agent [--selectProperties Title,Path] [--rowLimit 20] [--allResults]`
- `m365 spo propertybag get --webUrl URL --key KEY --agent` · `m365 spo customaction list --webUrl URL --agent`
- `m365 spo tenant settings list --agent` (needs a SharePoint admin)
- `m365 spo --help` lists every SharePoint command.

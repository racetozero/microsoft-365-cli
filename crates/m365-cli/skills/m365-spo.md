---
name: m365-spo
description: Work with SharePoint Online content through the `m365` CLI. Covers sites and subsites, lists, views and list items (including CSV batches, attachments and records), files and folders (versions, check-in, sharing links, archive), pages and web parts, fields and content types, users, groups, permissions and role inheritance, navigation, search, property bags, SPFx extensions on a site, retention and sensitivity labels and the site recycle bin. Builds on the `m365` skill; tenant-wide SharePoint is in `m365-spo-admin`.
---

# SharePoint with m365

Add `--agent` to every command (see the `m365` skill). Address a site by its absolute URL (`--webUrl https://contoso.sharepoint.com/sites/sales`; some commands name it `--url` or `--siteUrl`). Tenant-wide SharePoint (app catalog, hub sites, site designs, themes, term store, tenant settings) is in `m365-spo-admin`; SharePoint usage reports are in `m365-admin`.

## Nuances

- A list is one of `--listId`, `--listTitle` or `--listUrl` (exactly one). `spo field add` and `spo contenttype add` take none of them for a site column or site content type.
- File, folder and list URLs can be server-relative (`/sites/sales/Shared Documents/a.docx`) or site-relative (`/Shared Documents/a.docx`). Many file commands take `--url` or `--id` (exactly one); version, sharing link and role commands call them `--fileUrl` or `--fileId`.
- Field values are extra options named after the field's internal name (`--Status Open`), on `spo listitem add`, `spo listitem set` and `spo file add`. Get internal names with `spo field list`. Write dates as `yyyy-MM-dd HH:mm:ss`, managed metadata as `"Label|TERM_GUID;"`, and multiple choices as `"A;#B"`.
- `spo listitem list` returns every item, also past 5,000. `--pageNumber` is 0-based and needs `--pageSize`. `--camlQuery` cannot be combined with `--fields`, `--pageSize` or `--pageNumber`.
- Commands that remove, archive or overwrite fail with `confirmation_required` until you add `--force`. A removed file, item or site goes to the recycle bin; `--permanent` (on `spo file remove`, `spo listitem remove`, `spo site remove`) deletes it. `spo site recyclebinitem list|restore` gets items back.
- `roleinheritance break` keeps the copied permissions unless you add `--clearExistingPermissions`. A role assignment needs one principal (`--principalId`, `--upn`, `--groupName`, `--entraGroupId` or `--entraGroupName`) and one role (`--roleDefinitionName` such as `Read`, `Contribute`, `Edit`, `Full Control`, or `--roleDefinitionId`).
- `spo site add` without `--type` makes a group-connected team site and needs `--alias`; `CommunicationSite` and `ClassicSite` need `--url`. Each type accepts only its own options (the error lists them); `--wait` is for `ClassicSite` only.
- `spo site set`, `spo site rename` and `spo site ensure` run in the background; add `--wait` when the next step needs the result. `spo file copy|move` wait unless you add `--skipWait`.
- `spo site set --owners` adds owners and never removes them; `--isPublic` works only on group-connected sites.
- Page promotion (`--promoteAs`) and comments show only after the page is published. `spo page add` fails if the page name exists.
- JSON and XML option values can come from a file: `--webPartProperties @props.json`.
- `spo site remove`, `spo site archive`, `spo site rename` and `spo site membership list` need a SharePoint or Global administrator. An archived file reactivates at once within 7 days, then in up to 24 hours.
- The CLI finds your SharePoint URL itself. `spo get` shows it; `spo set --url` changes it (multi-geo tenants). `m365 context option set --name webUrl --value URL` makes `--webUrl` a default in the current folder.

## Sites and subsites

- `m365 spo site list --agent [--type TeamSite|CommunicationSite] [--filter "Url -like 'sales'"] [--state archived] [--withOneDriveSites]`
- `m365 spo site get --url https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo web get --url URL --agent [--withGroups] [--withPermissions]`
- `m365 spo site add --type CommunicationSite --title "Sales" --url https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo site add --alias sales --title "Sales" --owners jane@contoso.com --agent` (group-connected team site)
- `m365 spo site ensure --url https://contoso.sharepoint.com/sites/comms --title Comms --type CommunicationSite --agent` creates the site or updates it.
- `m365 spo site set --url URL --sharingCapability ExternalUserSharingOnly --lockState ReadOnly --agent` · `m365 spo site rename --url URL --newUrl https://contoso.sharepoint.com/sites/sales-eu --wait --agent`
- `m365 spo site groupify --url URL --alias sales --displayName "Sales" --agent` connects a site to a new Microsoft 365 group.
- `m365 spo site admin add --siteUrl URL --userName jane@contoso.com --agent [--primary]` · `m365 spo site membership list --siteUrl URL --role Owner --agent`
- `m365 spo site chrome set --siteUrl URL --headerLayout Compact --agent` · `m365 spo site sharingpermission set --siteUrl URL --capability ownersOnly --agent`
- `m365 spo web add --title Docs --url docs --webTemplate STS#3 --parentWebUrl URL --agent [--breakInheritance]` · `m365 spo web list --url URL --agent`

## Lists, views and items

- `m365 spo list list --webUrl URL --agent` · `m365 spo list add --webUrl URL --title "Projects" --baseTemplate GenericList --agent [--enableVersioning true]`
- `m365 spo list set --webUrl URL --title "Projects" --enableAttachments false --agent` · `m365 spo list view add --webUrl URL --listTitle "Projects" --title "Open" --fields "LinkTitle,Status" --viewQuery "<Where><Eq><FieldRef Name='Status'/><Value Type='Text'>Open</Value></Eq></Where>" --agent`
- `m365 spo listitem list --webUrl URL --listTitle "Projects" --agent [--fields Title,Status] [--filter "Status eq 'Open'"] [--pageSize 100 --pageNumber 0]`
- `m365 spo listitem add --webUrl URL --listTitle "Projects" --Title "New" --Status Open --agent` · `m365 spo listitem set --webUrl URL --listTitle "Projects" --id 3 --Status Done --agent [--systemUpdate]`
- `m365 spo listitem batch add --webUrl URL --listTitle "Projects" --filePath ./items.csv --agent` (first CSV row holds internal names) · `m365 spo listitem batch set --webUrl URL --listTitle "Projects" --filePath ./items.csv --agent [--idColumn ID]`
- `m365 spo listitem remove --webUrl URL --listTitle "Projects" --id 3 --force --agent [--permanent]`
- `m365 spo listitem attachment add --webUrl URL --listTitle "Projects" --listItemId 3 --filePath ./spec.pdf --agent`
- `m365 spo listitem record declare --webUrl URL --listTitle "Contracts" --listItemId 3 --agent` · `m365 spo list defaultvalue set --webUrl URL --listTitle Documents --fieldName Company --fieldValue Contoso --agent [--folderUrl ...]`
- `m365 spo list webhook add --webUrl URL --listTitle Documents --notificationUrl https://contoso.com/hook --agent [--expirationDateTime 2027-03-01]` (the expiry must be in the future and at most 6 months away)

## Columns and content types

- `m365 spo field list --webUrl URL --listTitle "Projects" --agent` · `m365 spo field add --webUrl URL --listTitle "Projects" --xml '<Field Type="Text" DisplayName="Code" Name="Code" />' --agent`
- `m365 spo contenttype add --webUrl URL --name "Contract" --id 0x0101009D1CB255DA76424F860D91F20E6C4118 --group "Contoso" --agent` (the ID starts with the parent's ID)
- `m365 spo list contenttype add --webUrl URL --listTitle Documents --id 0x0120 --agent`

## Files and folders

- `m365 spo folder list --webUrl URL --parentFolderUrl "/Shared Documents" --agent [--recursive]` · `m365 spo folder add --webUrl URL --parentFolderUrl "/Shared Documents" --name "Reports" --agent [--ensureParentFolders]`
- `m365 spo file list --webUrl URL --folderUrl "/Shared Documents" --agent [--recursive]`
- `m365 spo file get --webUrl URL --url "/sites/sales/Shared Documents/a.docx" --agent` (`--asString` returns the content, `--asFile --path out.docx` downloads it, `--asListItem` gives the fields)
- `m365 spo file add --webUrl URL --folder "Shared Documents" --path ./report.pdf --agent [--overwrite] [--Department Sales]`
- `m365 spo file copy --webUrl URL --sourceUrl "/sites/sales/Shared Documents/a.docx" --targetUrl "/sites/archive/Shared Documents" --nameConflictBehavior rename --agent` · `spo file move` takes the same options plus `--withItemPermissions`.
- `m365 spo file rename --webUrl URL --sourceUrl "/Shared Documents/a.docx" --targetFileName b.docx --agent`
- `m365 spo file checkout --webUrl URL --url "/Shared Documents/a.docx" --agent` · `m365 spo file checkin --webUrl URL --url "/Shared Documents/a.docx" --type Major --comment "Final" --agent`
- `m365 spo file version list --webUrl URL --fileUrl "/Shared Documents/a.docx" --agent` · `m365 spo file version restore --webUrl URL --fileUrl "/Shared Documents/a.docx" --label "2.0" --force --agent`
- `m365 spo file sharinglink add --webUrl URL --fileUrl "/Shared Documents/a.docx" --type view --scope organization --agent` · `m365 spo file sharinginfo get --webUrl URL --fileUrl "/Shared Documents/a.docx" --agent`
- `m365 spo file archive --webUrl URL --url "/Shared Documents/old.docx" --force --agent`

## Pages

- `m365 spo page list --webUrl URL --agent` · `m365 spo page get --webUrl URL --name home.aspx --agent`
- `m365 spo page add --webUrl URL --name news.aspx --title "News" --agent [--layoutType Article] [--promoteAs NewsPage] [--publish]`
- `m365 spo page section add --webUrl URL --pageName news.aspx --sectionTemplate TwoColumn --agent` · `m365 spo page text add --webUrl URL --pageName news.aspx --text "Hello" --section 1 --column 1 --agent`
- `m365 spo page clientsidewebpart add --webUrl URL --pageName news.aspx --standardWebPart BingMap --agent` (or `--webPartId` with `--webPartProperties @props.json`)
- `m365 spo page header set --webUrl URL --pageName news.aspx --type Custom --imageUrl "/sites/sales/SiteAssets/hero.jpg" --agent` · `m365 spo page publish --webUrl URL --name news.aspx --agent`
- `m365 spo page copy --webUrl URL --sourceName home.aspx --targetUrl home-copy.aspx --agent`

## Permissions, users and groups

- `m365 spo user ensure --webUrl URL --userName jane@contoso.com --agent` · `m365 spo user list --webUrl URL --agent`
- `m365 spo group list --webUrl URL --agent` · `m365 spo group member add --webUrl URL --groupName "Sales Members" --userNames jane@contoso.com --agent`
- `m365 spo web roleassignment add --webUrl URL --upn jane@contoso.com --roleDefinitionName "Read" --agent` · `m365 spo roledefinition list --webUrl URL --agent`
- `m365 spo list roleinheritance break --webUrl URL --listTitle "HR" --clearExistingPermissions --force --agent` · `m365 spo list roleassignment add --webUrl URL --listTitle "HR" --groupName "HR Owners" --roleDefinitionName "Full Control" --agent`

## Navigation, search, settings and extensions

- `m365 spo navigation node list --webUrl URL --location QuickLaunch --agent` · `m365 spo navigation node add --webUrl URL --location TopNavigationBar --title About --url /sites/sales/SitePages/about.aspx --agent`
- `m365 spo search --queryText "contract" --agent [--selectProperties Title,Path] [--rowLimit 20] [--allResults] [--webUrl URL]`
- `m365 spo propertybag get --webUrl URL --key KEY --agent` · `m365 spo propertybag set --webUrl URL --key KEY --value VALUE --agent`
- `m365 spo feature list --webUrl URL --scope Site --agent` · `m365 spo customaction list --webUrl URL --agent`
- `m365 spo applicationcustomizer add --webUrl URL --title "Banner" --clientSideComponentId GUID --agent` (site scope; tenant-wide extensions are in `m365-spo-admin`)

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

**Sites and webs**
- `spo`: get, search, set
- `spo site`: add, archive, ensure, get, groupify, list, remove, rename, set, unarchive
- `spo site accessrequest setting`: set
- `spo site admin`: add, list, remove
- `spo site chrome`: set
- `spo site commsite`: enable
- `spo site hubsite`: connect, disconnect
- `spo site hubsite theme`: sync
- `spo site inplacerecordsmanagement`: set
- `spo site membership`: list
- `spo site recyclebinitem`: clear, list, move, remove, restore
- `spo site sharingpermission`: set
- `spo site versionpolicy`: get
- `spo web`: add, get, list, reindex, remove, set
- `spo web clientsidewebpart`: list
- `spo web installedlanguage`: list
- `spo web retentionlabel`: list
- `spo web roleassignment`: add, remove
- `spo web roleinheritance`: break, reset
- `spo web rule`: get, list, remove

**Lists and items**
- `spo list`: add, get, list, remove, set
- `spo list contenttype`: add, list, remove
- `spo list contenttype default`: set
- `spo list defaultvalue`: clear, get, list, remove, set
- `spo list retentionlabel`: ensure, get, remove
- `spo list roleassignment`: add, remove
- `spo list roleinheritance`: break, reset
- `spo list sensitivitylabel`: ensure, remove
- `spo list sitescript`: get
- `spo list view`: add, get, list, remove, set
- `spo list view field`: add, remove, set
- `spo list webhook`: add, get, list, remove, set
- `spo listitem`: add, get, isrecord, list, remove, set
- `spo listitem attachment`: add, get, list, remove, set
- `spo listitem batch`: add, remove, set
- `spo listitem record`: declare, lock, undeclare, unlock
- `spo listitem retentionlabel`: ensure, remove
- `spo listitem roleassignment`: add, remove
- `spo listitem roleinheritance`: break, reset

**Columns, content types and events**
- `spo field`: add, get, list, remove, set
- `spo contenttype`: add, get, list, remove, set, sync
- `spo contenttype field`: list, remove, set
- `spo eventreceiver`: get, list, remove

**Files and folders**
- `spo file`: add, archive, checkin, checkout, copy, get, list, move, remove, rename, unarchive
- `spo file checkout`: undo
- `spo file retentionlabel`: ensure, remove
- `spo file roleassignment`: add, remove
- `spo file roleinheritance`: break, reset
- `spo file sharinginfo`: get
- `spo file sharinglink`: add, clear, get, list, remove, set
- `spo file version`: clear, get, keep, list, remove, restore
- `spo folder`: add, archive, copy, get, list, move, remove, set, unarchive
- `spo folder retentionlabel`: ensure, remove
- `spo folder roleassignment`: add, remove
- `spo folder roleinheritance`: break, reset
- `spo folder sharinglink`: add, clear, get, list, remove, set

**Pages**
- `spo page`: add, copy, get, list, publish, remove, set, unpublish
- `spo page clientsidewebpart`: add
- `spo page column`: get, list
- `spo page control`: get, list, remove, set
- `spo page header`: set
- `spo page section`: add, get, list, remove
- `spo page template`: list
- `spo page text`: add
- `spo apppage`: add, set

**Users, groups and permissions**
- `spo user`: ensure, get, list, remove
- `spo group`: add, get, list, remove, set
- `spo group member`: add, list, remove
- `spo roledefinition`: add, get, list, remove

**Navigation, settings and extensions**
- `spo navigation node`: add, get, list, remove, set
- `spo propertybag`: get, list, remove, set
- `spo feature`: disable, enable, list
- `spo customaction`: add, clear, get, list, remove, set
- `spo applicationcustomizer`: add, get, list, remove, set
- `spo commandset`: add, get, list, remove, set

---
name: m365-files
description: Work with files in Microsoft 365 document libraries, OneDrive and SharePoint Embedded through the `m365` CLI. Covers listing, uploading, copying, moving and converting files with Microsoft Graph, finding OneDrive sites, and SharePoint Embedded container types, containers, permissions and recycle bins. Builds on the `m365` skill.
---

# Files with m365

Add `--agent` to every command (see the `m365` skill). The `file` commands use Microsoft Graph and take site and folder URLs. For SharePoint-only file features (check-in, versions, sharing links, retention labels, role assignments) see the `m365-spo` skill. Usage reports for OneDrive are in the `m365-admin` skill.

## Nuances

- `m365 file list` is the Graph version of `spo file list` and handles large folders. Use it when a folder has many files.
- `m365 file add` takes an absolute `--folderUrl` (library or subfolder, in any site, OneDrive included). It finds the site by walking the URL with Graph calls; add `--siteUrl` to skip that lookup.
- `m365 file move` keeps version history only inside one library. Across libraries it copies and deletes, and the history is lost.
- `--nameConflictBehavior fail|replace|rename` controls what `copy` and `move` do when the target name exists.
- `m365 file convert pdf` takes a local path or a full URL for both `--sourceFile` and `--targetFile`. A local source is uploaded to a library for a short time and removed after. With app-only login it goes to the default library of the root site.
- OneDrive files are SharePoint files: address them by the personal site URL, for example `https://contoso-my.sharepoint.com/personal/jane_contoso_com`.
- SharePoint Embedded container type commands (`spe containertype *`) need a SharePoint Administrator or Global Administrator and do not support application permissions. The tenant must own the app that owns the container type; one app has one container type, a tenant has at most five, and only one trial type.
- Containers are addressed by a `b!...` drive ID. Commands that take a container `--name` also need `--containerTypeId` or `--containerTypeName`. With application permissions, use the ID forms (`--containerTypeId`, `--containerId`).
- Container roles are `reader`, `writer`, `manager` and `owner` (comma-separated in `--roles`).
- `m365 spe container remove` moves a container to the recycle bin; add `--permanent` to delete it. Removes need `--force` in agent mode.

## Files in libraries (Microsoft Graph)

- `m365 file list --webUrl https://contoso.sharepoint.com/sites/sales --folderUrl "Shared Documents" --agent [--recursive]`
- `m365 file add --filePath ./report.pdf --folderUrl "https://contoso.sharepoint.com/sites/sales/Shared Documents/Reports" --agent [--siteUrl https://contoso.sharepoint.com/sites/sales]`
- `m365 file copy --webUrl https://contoso.sharepoint.com/sites/sales --sourceUrl "/sites/sales/Shared Documents/a.docx" --targetUrl "/sites/archive/Shared Documents" --agent [--newName b.docx] [--nameConflictBehavior rename]`
- `m365 file move --webUrl https://contoso.sharepoint.com/sites/sales --sourceUrl "/sites/sales/Shared Documents/a.docx" --targetUrl "/sites/sales/Shared Documents/Archive" --agent`
- `m365 file convert pdf --sourceFile "https://contoso.sharepoint.com/sites/sales/Shared Documents/a.docx" --targetFile ./a.pdf --agent` (or a local `--sourceFile ./a.docx`, or a URL as `--targetFile`)

## OneDrive

- `m365 onedrive list --agent` lists the OneDrive sites in the tenant (it reads the tenant site list, so use a SharePoint admin account).
- `m365 spo file list --webUrl https://contoso-my.sharepoint.com/personal/jane_contoso_com --folderUrl Documents --agent` lists a user's files.

## SharePoint Embedded

- Container types: `m365 spe containertype list --agent` · `m365 spe containertype get --name "Projects" --agent` · `m365 spe containertype add --name "Projects" --appId APP_ID --billingType trial --agent`
- Containers: `m365 spe container list --containerTypeName "Projects" --agent` · `m365 spe container get --id CONTAINER_ID --agent` (or `--name "Project X" --containerTypeName "Projects"`)
- `m365 spe container add --name "Project X" --containerTypeName "Projects" --agent [--description "..."] [--ocrEnabled true]`
- `m365 spe container activate --id CONTAINER_ID --agent` · `m365 spe container set --id CONTAINER_ID --newName "Project Y" --agent`
- `m365 spe container permission list --containerId CONTAINER_ID --agent` · `m365 spe container permission add --containerId CONTAINER_ID --userName jane@contoso.com --roles reader,writer --agent` · `m365 spe container permission set --containerId CONTAINER_ID --id PERMISSION_ID --roles owner --agent`
- `m365 spe container remove --id CONTAINER_ID --force --agent [--permanent]` · `m365 spe container recyclebinitem list --containerTypeName "Projects" --agent` · `m365 spe container recyclebinitem restore --id CONTAINER_ID --agent`

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `file`: add, copy, list, move — Graph file operations in any library
- `file convert`: pdf
- `onedrive`: list
- `spe containertype`: add, get, list, remove
- `spe container`: activate, add, get, list, remove, set
- `spe container permission`: add, list, remove, set
- `spe container recyclebinitem`: list, remove, restore

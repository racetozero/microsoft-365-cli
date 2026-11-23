---
name: m365-files
description: Work with OneDrive, files in Microsoft 365 document libraries and SharePoint Embedded containers through the `m365` CLI. Covers listing, uploading, copying, moving and converting files, and SharePoint Embedded container types, containers, permissions and recycle bins. Builds on the `m365` skill.
---

# Files with m365

Add `--agent` to every command (see the `m365` skill). For SharePoint-specific file commands (check-in, versions, sharing links, retention labels) see the `m365-spo` skill.

## Files in libraries (Microsoft Graph)

- `m365 file list --webUrl https://contoso.sharepoint.com/sites/sales --folderUrl "Shared Documents" --agent [--recursive]`
- `m365 file add --filePath ./report.pdf --folderUrl "https://contoso.sharepoint.com/sites/sales/Shared Documents" --agent`
- `m365 file copy --webUrl URL --sourceUrl "Shared Documents/a.docx" --targetUrl "Shared Documents/Archive" --agent [--nameConflictBehavior rename]`
- `m365 file move --webUrl URL --sourceUrl "Shared Documents/a.docx" --targetUrl "Shared Documents/Archive" --agent`
- `m365 file convert pdf --sourceFile "https://contoso.sharepoint.com/sites/sales/Shared Documents/a.docx" --targetFile ./a.pdf --agent`

## OneDrive

- `m365 onedrive list --agent` lists the OneDrive sites in the tenant (admin).
- OneDrive files are SharePoint files: use `m365 spo file list --webUrl https://contoso-my.sharepoint.com/personal/jane_contoso_com --folderUrl Documents --agent`.

## SharePoint Embedded

- `m365 spe containertype list --agent` · `m365 spe container list --containerTypeName "Projects" --agent`
- `m365 spe container add --name "Project X" --containerTypeName "Projects" --agent` · `m365 spe container get --id ID --agent`
- `m365 spe container permission list --containerId ID --agent` · `m365 spe container permission add --containerId ID --userName jane@contoso.com --roles reader --agent`
- `m365 spe container recyclebinitem list --containerTypeName "Projects" --agent`

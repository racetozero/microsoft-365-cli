---
name: m365-spo-admin
description: Administer SharePoint Online across the tenant with the `m365` CLI. Covers tenant settings, the tenant and site collection app catalogs and SPFx app deployment, tenant-wide extensions, hub sites, home sites, site designs and site scripts, themes and the brand center, the Microsoft 365 CDN and organization assets, the term store, API access for SPFx (service principal grants), Entra app access to sites, tenant properties, external users, user profiles, the tenant recycle bin, SharePoint agents and SharePoint Premium models. Builds on the `m365` skill.
---

# SharePoint tenant administration with m365

Add `--agent` to every command (see the `m365` skill). Most commands here need the SharePoint Administrator or Global Administrator role; the CLI finds the tenant admin site itself. Site content (lists, files, pages, permissions) is in `m365-spo`.

## Nuances

- `spo app` commands use the tenant app catalog by default and find its URL. For a site collection app catalog add `--appCatalogScope sitecollection --appCatalogUrl URL` (`spo app install|upgrade|uninstall` take only `--appCatalogScope`).
- An SPFx package goes in three steps: `spo app add` uploads it, `spo app deploy` makes it available (`--skipFeatureDeployment` deploys it to all sites when the package allows that), `spo app install --siteUrl` adds it to one site. `spo app add` fails if the package exists, unless you add `--overwrite`. `~` in `--filePath` is not expanded.
- `spo tenant appcatalog add --force` deletes the existing app catalog and any site at the URL, bypassing the recycle bin. Add `--wait` to wait for the new site.
- After you deploy a package that requests API permissions, approve them with `spo serviceprincipal permissionrequest approve` (`--id`, `--resource` or `--all`) or grant a scope directly with `spo serviceprincipal grant add`.
- `spo site apppermission add` gives an Entra app (Sites.Selected) access to one site: `--permission read|write|manage|fullcontrol` and `--appId` or `--appDisplayName`.
- Term store commands without `--webUrl` need tenant admin rights. With `--webUrl` they work with site permissions and also see the site collection term group.
- `spo sitescript add` and `spo sitedesign add` always create a new item with a new ID; list first to avoid duplicates. `spo sitedesign apply --asTask` is needed for large designs.
- `spo theme set` checks the theme JSON and needs every palette key; pass it from a file with `--theme @theme.json`. `spo theme apply --sharePointTheme` applies a built-in theme (Blue, Orange, Red, Purple, Green, Gray, Dark Yellow, Dark Blue).
- `spo tenant settings set` takes the setting names as options (PascalCase, such as `--SharingCapability`) and needs at least one. `spo tenant settings list` shows the current values.
- Tenant properties (`spo storageentity`) live in the app catalog site; a URL of another site gives access denied.
- `spo hubsite connect` links a hub to a parent hub; `spo site hubsite connect` (in `m365-spo`) links a regular site to a hub. `spo orgassetslibrary add` needs the CDN on (`spo cdn set`).
- Removals ask for `--force`. `spo tenant recyclebinitem restore --siteUrl URL` brings back a deleted site collection.

## Apps and the app catalog

- `m365 spo tenant appcatalogurl get --agent` · `m365 spo tenant appcatalog add --url https://contoso.sharepoint.com/sites/apps --owner admin@contoso.com --timeZone 4 --wait --agent`
- `m365 spo site appcatalog add --siteUrl https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo site appcatalog list --agent`
- `m365 spo app list --agent [--appCatalogScope sitecollection --appCatalogUrl URL]` · `m365 spo app get --name solution.sppkg --agent`
- `m365 spo app add --filePath ./sharepoint/solution/solution.sppkg --overwrite --agent` · `m365 spo app deploy --name solution.sppkg --agent [--skipFeatureDeployment]`
- `m365 spo app install --id APP_ID --siteUrl https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo app upgrade --id APP_ID --siteUrl URL --agent` · `m365 spo app instance list --siteUrl URL --agent`
- `m365 spo app retract --id APP_ID --force --agent` · `m365 spo app remove --id APP_ID --force --agent`
- `m365 spo app teamspackage download --appName solution.sppkg --fileName solution.zip --agent` gives the Teams package of a tenant-deployed SPFx solution.

## API access and app permissions

- `m365 spo serviceprincipal permissionrequest list --agent` · `m365 spo serviceprincipal permissionrequest approve --all --agent` · `m365 spo serviceprincipal permissionrequest deny --id ID --agent`
- `m365 spo serviceprincipal grant list --agent` · `m365 spo serviceprincipal grant add --resource "Microsoft Graph" --scope "Mail.Read" --agent` · `m365 spo serviceprincipal grant revoke --id GRANT_ID --agent`
- `m365 spo site apppermission add --siteUrl URL --permission write --appId APP_ID --appDisplayName "Contoso sync" --agent` · `m365 spo site apppermission list --siteUrl URL --agent`

## Tenant-wide extensions and properties

- `m365 spo tenant applicationcustomizer add --title "Banner" --clientSideComponentId GUID --clientSideComponentProperties '{"message":"Hi"}' --agent [--webTemplate SITEPAGEPUBLISHING#0]`
- `m365 spo tenant commandset add --title "Export" --listType Library --clientSideComponentId GUID --location ContextMenu --agent`
- `m365 spo storageentity list --agent` · `m365 spo storageentity set --key AnalyticsId --value 123 --description "Web analytics ID" --agent`

## Hub sites, home sites and news

- `m365 spo hubsite register --siteUrl https://contoso.sharepoint.com/sites/sales --agent` · `m365 spo hubsite list --agent` · `m365 spo hubsite set --id HUB_ID --title "Sales hub" --agent`
- `m365 spo hubsite rights grant --hubSiteUrl URL --principals "PattiF,AdeleV" --rights Join --agent` · `m365 spo hubsite connect --url URL --parentUrl https://contoso.sharepoint.com/sites/corp --agent`
- `m365 spo homesite add --url https://contoso.sharepoint.com/sites/intranet --agent [--audienceNames "All staff"] [--vivaConnectionsDefaultStart true]` · `m365 spo homesite list --agent`
- `m365 spo orgnewssite set --url https://contoso.sharepoint.com/sites/news --agent` · `m365 spo knowledgehub set --siteUrl URL --agent`

## Site designs, themes and branding

- `m365 spo sitescript add --title "Contoso lists" --content @script.json --agent` · `m365 spo sitedesign add --title "Contoso team" --webTemplate TeamSite --siteScripts SCRIPT_ID --agent [--isDefault]`
- `m365 spo sitedesign apply --id DESIGN_ID --webUrl URL --agent [--asTask]` · `m365 spo sitedesign run list --webUrl URL --agent` · `m365 spo sitedesign rights grant --siteDesignId DESIGN_ID --principals PattiF --rights View --agent`
- `m365 spo theme set --name Contoso-Blue --theme @theme.json --agent` · `m365 spo theme apply --name Contoso-Blue --webUrl URL --agent` · `m365 spo hidedefaultthemes set --hideDefaultThemes true --agent`
- `m365 spo brandcenter settings list --agent` · `m365 spo brandcenter colors list --agent`

## CDN and organization assets

- `m365 spo cdn get --type Private --agent` · `m365 spo cdn set --type Public --enabled true --agent` · `m365 spo cdn origin add --type Public --origin "*/CDN" --agent`
- `m365 spo orgassetslibrary add --libraryUrl https://contoso.sharepoint.com/sites/brand/Logos --orgAssetType ImageDocumentLibrary --agent`

## Term store

- `m365 spo term group list --agent` · `m365 spo term set list --termGroupName People --agent` · `m365 spo term list --termSetName Department --termGroupName People --agent`
- `m365 spo term set add --name Department --termGroupName People --agent` · `m365 spo term add --name IT --termSetName Department --termGroupName People --agent`

## Tenant settings, users and the recycle bin

- `m365 spo tenant settings list --agent` · `m365 spo tenant settings set --SharingCapability ExternalUserSharingOnly --DefaultSharingLinkType Internal --agent`
- `m365 spo tenant site get --url https://contoso.sharepoint.com/sites/sales --agent`
- `m365 spo externaluser list --pageSize 50 --position 0 --agent [--siteUrl URL] [--filter Vesa]`
- `m365 spo userprofile get --userName jane@contoso.com --agent` · `m365 spo userprofile set --userName jane@contoso.com --propertyName AboutMe --propertyValue "Developer" --agent`
- `m365 spo tenant recyclebinitem list --agent` · `m365 spo tenant recyclebinitem restore --siteUrl URL --agent` · `m365 spo tenant recyclebinitem remove --siteUrl URL --force --agent`

## SharePoint agents and SharePoint Premium

- `m365 spo agent add --webUrl URL --name "Project Assistant" --description "Answers project questions" --agentInstructions "Be concise." --welcomeMessage "Hi" --sourceUrls "https://contoso.sharepoint.com/sites/projects/Shared Documents" --agent`
- `m365 spp contentcenter list --agent` · `m365 spp model list --siteUrl https://contoso.sharepoint.com/sites/ContentCenter --agent`
- `m365 spp model apply --contentCenterUrl https://contoso.sharepoint.com/sites/ContentCenter --webUrl URL --title "Invoices" --listTitle Documents --agent`
- `m365 spp autofillcolumn set --siteUrl URL --listTitle Documents --columnTitle Summary --prompt "Write a 2-line summary of the document" --agent` (`--prompt` is needed the first time)

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

**Apps, API access and extensions**
- `spo app`: add, deploy, get, install, list, remove, retract, uninstall, upgrade
- `spo app instance`: list
- `spo app teamspackage`: download
- `spo tenant appcatalog`: add
- `spo tenant appcatalogurl`: get
- `spo site appcatalog`: add, list, remove
- `spo site apppermission`: add, get, list, remove, set
- `spo serviceprincipal`: set
- `spo serviceprincipal grant`: add, list, revoke
- `spo serviceprincipal permissionrequest`: approve, deny, list
- `spo tenant applicationcustomizer`: add, get, list, remove, set
- `spo tenant commandset`: add, get, list, remove, set
- `spo storageentity`: get, list, remove, set

**Hubs, home sites and news**
- `spo hubsite`: connect, disconnect, get, list, register, set, unregister
- `spo hubsite data`: get
- `spo hubsite rights`: grant, revoke
- `spo homesite`: add, get, list, remove, set
- `spo orgnewssite`: list, remove, set
- `spo knowledgehub`: get, remove, set

**Designs, themes, branding and CDN**
- `spo sitedesign`: add, apply, get, list, remove, set
- `spo sitedesign rights`: grant, list, revoke
- `spo sitedesign run`: list
- `spo sitedesign run status`: get
- `spo sitedesign task`: get, list, remove
- `spo sitescript`: add, get, list, remove, set
- `spo theme`: apply, get, list, remove, set
- `spo hidedefaultthemes`: get, set
- `spo brandcenter colors`: list
- `spo brandcenter settings`: list
- `spo cdn`: get, set
- `spo cdn origin`: add, list, remove
- `spo cdn policy`: list, set
- `spo orgassetslibrary`: add, list, remove

**Term store and content types**
- `spo term`: add, get, list
- `spo term group`: add, get, list
- `spo term set`: add, get, list
- `spo contenttypehub`: get

**Tenant, users and recycle bin**
- `spo tenant settings`: list, set
- `spo tenant site`: get
- `spo tenant recyclebinitem`: list, remove, restore
- `spo externaluser`: list
- `spo userprofile`: get, set

**Agents and SharePoint Premium**
- `spo agent`: add
- `spp contentcenter`: list
- `spp model`: apply, get, list, remove
- `spp autofillcolumn`: set

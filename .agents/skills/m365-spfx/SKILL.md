---
name: m365-spfx
description: Maintain SharePoint Framework (SPFx) projects with the `m365` CLI. Covers checking the development environment, validating a project, upgrading it to a newer or preview SPFx version, externalizing dependencies, renaming, adding GitHub or Azure DevOps deployment pipelines, granting API permissions and generating a package from an HTML snippet. Builds on the `m365` skill.
---

# SharePoint Framework with m365

Run the project commands inside the SPFx project folder (the one with `package.json`, `.yo-rc.json` and `src`). Most of them work offline and need no login.

## Nuances

- `spfx project doctor`, `upgrade` and `externalize` never change files. They print findings; you apply them. Only `rename`, `github workflow add`, `azuredevops pipeline add`, `package generate` and `--output tour` write files. `permissions grant` changes the tenant and needs a login.
- With `--agent` the output is always JSON (an `--output` value is ignored). Each finding has `id` (for example `FN001001`), `title`, `description`, `file`, `position` (`line`, `character`), `severity` (`Required`, `Recommended`, `Optional`), `resolutionType` (`cmd` for a shell command, or `json`, `js`, `ts`, `scss`, `text` for content to put in `file`) and `resolution`.
- For a report a person reads, leave out `--agent`: `--output md` gives Markdown with a summary script, `--output text` only the summary script, and `--output tour` writes `.tours/upgrade.tour` for the VS Code CodeTour extension.
- The commands in a resolution use `--packageManager npm` and `--shell powershell` by default. Set `--shell bash` on macOS and Linux, and `--packageManager pnpm` (upgrade) or `npm|pnpm|yarn` (doctor) to match the project.
- `spfx project upgrade` targets the newest supported version (1.24.1 in this build) unless you give `--toVersion` or `--preview`. When the project is current, `data` is the string `Project doesn't need to be upgraded`.
- `spfx doctor` checks Node.js, yo, gulp-cli and TypeScript against the project's SPFx version, or `--spfxVersion` outside a project. `--env sp2016|sp2019|spo` checks support on SharePoint Server. With `--agent` each check is `{check, passed, version, message, fix}`; progress text goes to stderr.
- Errors: `Couldn't find project root folder` means no `package.json` was found; an `ENOENT ... src` error means the folder is not a full SPFx project.
- `spfx project externalize` supports projects up to SPFx 1.9.1 only.
- `spfx project rename` updates `package.json`, `.yo-rc.json`, `package-solution.json`, `deploy-azure-storage.json` and `README.md`. Add `--generateNewId` for a new solution ID so the copy can be deployed next to the original.
- The pipeline commands do not register an Entra app or a certificate. With `--loginMethod application` (the default), create them first and add the secrets the help lists (`APP_ID`, `CERTIFICATE_ENCODED`, `CERTIFICATE_PASSWORD` for GitHub). `--scope sitecollection` needs `--siteUrl`.

## Check and validate

- `m365 spfx doctor --agent` · `m365 spfx doctor --spfxVersion 1.21.1 --agent` · `m365 spfx doctor --env sp2019 --output text`
- `m365 spfx project doctor --agent [--packageManager pnpm]` lists problems (wrong dependency versions, duplicates, misplaced packages) with the fix for each.

## Upgrade

- `m365 spfx project upgrade --shell bash --agent` lists every step to the newest version. `m365 spfx project upgrade --toVersion 1.20.0 --shell bash --agent` targets another version; `--preview` targets the newest preview.
- Apply the findings in order: run each `cmd` resolution and merge each file resolution into `file`. Then build the project and run `m365 spfx project upgrade --agent` again until it returns `Project doesn't need to be upgraded`. Run `m365 spfx project doctor --agent` after that.
- `m365 spfx project upgrade --output md > upgrade.md` writes a report for a person; `m365 spfx project upgrade --output tour` writes a CodeTour.

## Other project tasks

- `m365 spfx project externalize --agent` suggests externals for PnPjs and other libraries.
- `m365 spfx project rename --newName contoso-webparts --generateNewId`
- `m365 spfx project github workflow add --branchName main --manuallyTrigger --agent [--loginMethod user] [--scope sitecollection --siteUrl https://contoso.sharepoint.com/sites/apps] [--skipFeatureDeployment]` writes `.github/workflows/deploy-spfx-solution.yml`.
- `m365 spfx project azuredevops pipeline add --branchName main --agent` writes `.azuredevops/pipelines/deploy-spfx-solution.yml` with the same login and scope options. Create the pipeline in Azure DevOps from that file.
- `m365 spfx project permissions grant --agent` grants the API permissions in `config/package-solution.json` (needs an admin login).
- `m365 spfx package generate --webPartTitle "Weather" --webPartDescription "Shows weather" --name weather --html @weather.html --enableForTeams all --agent [--allowTenantWideDeployment]` builds `weather.sppkg` with a no-framework web part. Deploy it with `m365 spo app add` (`m365-spo-admin` skill).

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `spfx`: doctor — checks the development environment
- `spfx project`: doctor, externalize, rename, upgrade
- `spfx project github workflow`: add
- `spfx project azuredevops pipeline`: add
- `spfx project permissions`: grant
- `spfx package`: generate

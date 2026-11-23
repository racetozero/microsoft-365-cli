---
name: m365-spfx
description: Maintain SharePoint Framework (SPFx) projects with the `m365` CLI. Covers checking the development environment, validating a project, upgrading it to a newer SPFx version, externalizing dependencies, renaming, adding GitHub or Azure DevOps deployment pipelines, granting API permissions and generating a package from HTML. Builds on the `m365` skill.
---

# SharePoint Framework with m365

Run the project commands inside the SPFx project folder (the one with `package.json`). They read the project and print findings; only `rename`, the pipeline commands and `--output tour` change files.

## Check and validate

- `m365 spfx doctor --output text` checks Node.js, yo, gulp-cli, heft and TypeScript against the project's SPFx version (`--spfxVersion 1.21.1` for another version, `--env spo|sp2019|sp2016` for on-premises support).
- `m365 spfx project doctor --output json` lists problems in the project (wrong dependency versions, duplicates, misplaced packages) with the command that fixes each.

## Upgrade

- `m365 spfx project upgrade --output json` lists every step to upgrade to the latest supported version (`--toVersion 1.20.0` for another). Each finding has `id`, `title`, `file`, `position` and a `resolution` (a command, or JSON/TS to put in the file).
- `--output md` gives a readable report with a summary script; `--output text` only the summary; `--output tour` writes `.tours/upgrade.tour` for the VS Code CodeTour extension.
- `--packageManager npm|pnpm` and `--shell bash|powershell|cmd` choose how the commands are written.
- Apply the findings in order, then run the build. Run `m365 spfx project upgrade` again: it prints `Project doesn't need to be upgraded` when done.

## Other project tasks

- `m365 spfx project externalize --output json` suggests externals for PnPjs and other libraries (SPFx up to 1.9.1).
- `m365 spfx project rename --newName contoso-webparts [--generateNewId]`
- `m365 spfx project github workflow add [--branchName main] [--loginMethod application|user] [--scope tenant|sitecollection --siteUrl URL]` writes `.github/workflows/deploy-spfx-solution.yml`; `m365 spfx project azuredevops pipeline add` writes `.azuredevops/pipelines/deploy-spfx-solution.yml` with the same options.
- `m365 spfx project permissions grant --agent` grants the API permissions in `config/package-solution.json`.
- `m365 spfx package generate --webPartTitle "Weather" --webPartDescription "Shows weather" --name weather --html @weather.html --enableForTeams all` builds `weather.sppkg` from an HTML snippet.

---
name: m365-entra
description: Manage Microsoft Entra ID (Azure AD) with the `m365` CLI. Covers users, groups and Microsoft 365 groups, memberships, app registrations, enterprise apps, permissions and consent, licenses, sign-ins, roles, PIM and administrative units. Builds on the `m365` skill.
---

# Entra ID with m365

Add `--agent` to every command (see the `m365` skill for output and errors). Most commands identify an object by `--id` or a name option; give exactly one.

## Users

- `m365 entra user get --userName jane@contoso.com --agent` (or `--id`, `--email`; `--properties displayName,jobTitle`, `--withManager`)
- `m365 entra user list --agent [--type Member|Guest] [--properties displayName,mail]`; filter with `--query "[?department=='Sales']"` or extra options such as `--department Sales`.
- `m365 entra user add --displayName "Jane Doe" --userName jane@contoso.com --password "..." --agent [--forceChangePasswordNextSignIn] [--jobTitle T] [--managerUserName boss@contoso.com]`
- `m365 entra user set --userName jane@contoso.com --jobTitle "Lead" --agent` · `--accountEnabled false` disables the account · `--resetPassword --newPassword "..."`
- `m365 entra user remove --userName jane@contoso.com --force --agent`
- `m365 entra user license add --userName jane@contoso.com --ids SKU_ID --agent` · `m365 entra license list --agent` lists the SKUs
- `m365 entra user signin list --userName jane@contoso.com --agent`

## Groups

- `m365 entra group list --agent [--type microsoft365|security|distribution|mailEnabledSecurity]` · `m365 entra group get --displayName "Sales" --agent`
- `m365 entra group add --displayName "Sales" --type security --agent [--ownerUserNames a@contoso.com] [--memberUserNames b@contoso.com,c@contoso.com]`
- `m365 entra group member list --groupName "Sales" --agent [--role Owner|Member]`
- `m365 entra group member add --groupName "Sales" --userNames b@contoso.com --role Member --agent`
- Microsoft 365 groups: `m365 entra m365group list --agent [--displayName Sales] [--withSiteUrl]` · `m365 entra m365group add --displayName "Sales" --mailNickname sales --agent` · `m365 entra m365group user add --groupName "Sales" --userNames b@contoso.com --role member --agent`

## Apps

- `m365 entra app list --agent` · `m365 entra app get --name "My app" --agent` (or `--appId`, `--objectId`)
- `m365 entra app add --name "My app" --agent [--multitenant] [--redirectUris https://localhost --platform spa] [--withSecret] [--apisDelegated "https://graph.microsoft.com/User.Read"] [--grantAdminConsent]`
- `m365 entra app permission add --appName "My app" --delegatedPermissions "https://graph.microsoft.com/Mail.Read" --grantAdminConsent --agent`
- Enterprise apps (service principals): `m365 entra enterpriseapp list --agent [--displayName X]` · app role assignments: `m365 entra approleassignment list --appDisplayName "My app" --agent`

## Directory

- `m365 entra roledefinition list --agent` · `m365 entra pim role assignment add --roleDefinitionName "Global Reader" --userName jane@contoso.com --duration PT8H --justification "audit" --agent`
- `m365 entra administrativeunit list --agent` · `m365 entra policy list --agent [--type authorization]`
- `m365 entra --help` lists every Entra command.

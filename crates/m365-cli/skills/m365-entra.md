---
name: m365-entra
description: Manage Microsoft Entra ID (Azure AD) with the `m365` CLI. Covers users, guests, licenses, sign-ins and MFA registration, security and Microsoft 365 groups, group settings and site classification, app registrations, enterprise apps, API permissions, consent grants, directory and PIM roles, administrative units, multitenant organizations, policies, agent identities and Exchange Online app role assignments. Builds on the `m365` skill.
---

# Entra ID with m365

Add `--agent` to every command (see the `m365` skill for output and errors). Most commands identify an object by `--id` or a name option; give exactly one. Usage reports for Microsoft 365 groups are in the `m365-admin` skill.

## Nuances

- Users are addressed by `--id` or `--userName` (the UPN); `entra user get` also takes `--email`. Sub-commands often name them `--userId`/`--userName`, and `entra user groupmembership list` adds `--userEmail`.
- `entra user list`, `entra user add` and `entra user set` accept any Graph user property as an extra option: `--department Sales` filters a list (values match from the start, several filters are combined with `and`), and `--employeeId 123` sets a property on add.
- `--properties manager/displayName` adds `$expand=manager` on user and member lists. Some properties (`aboutMe`, `skills`, `mailboxSettings`, ...) work only on `entra user get`, not on `list`.
- A license needs a `usageLocation` on the user first: set `--usageLocation US` on add or set. `entra license list` gives the SKU IDs for `--ids`.
- Removing a user or Microsoft 365 group moves it to the recycle bin (it can take a minute); restore it with `recyclebinitem restore`. `entra m365group remove --permanent` deletes the group and its site and then purges it.
- Security groups use `entra group`; Microsoft 365 groups and their Teams use `entra m365group`. `entra group add --type microsoft365` needs `--visibility`. `HiddenMembership` can only be set when the group is created.
- Member roles are `Owner|Member` on `entra group member` and lowercase `owner|member` on `entra m365group user`. `entra m365group user add` also takes `--teamId`/`--teamName`.
- App commands accept `--appId`, `--objectId` (or `--appObjectId`) or `--name`. `--objectId` is fastest and never ambiguous; a name can match several apps.
- Permission lists differ by command: `entra app add --apisDelegated` is comma-separated, `entra app permission add --delegatedPermissions` is space-separated. Both take fully qualified scopes such as `https://graph.microsoft.com/Mail.Read`.
- Removing a permission does not revoke admin consent; add `--revokeAdminConsent`. An app registered with `m365` or Graph gets no enterprise app (service principal) until `entra enterpriseapp add` creates one.
- PIM durations are ISO 8601 (`PT8H`); give one of `--duration`, `--endDateTime` or `--noExpiration`. Without a user or group option, the assignment is for the signed-in user.
- Most write commands need an admin role (User, Groups, Application, Privileged Role or Global Administrator). `--help` lists each command's permissions.

## Users and guests

- `m365 entra user get --userName jane@contoso.com --agent [--properties displayName,jobTitle,manager/displayName] [--withManager]`
- `m365 entra user list --type Guest --agent` · `m365 entra user list --department Sales --properties displayName,mail --agent` · `m365 entra user list --query "[?!accountEnabled].userPrincipalName" --agent`
- `m365 entra user add --displayName "Jane Doe" --userName jane@contoso.com --usageLocation US --agent [--password "..." --forceChangePasswordNextSignIn] [--jobTitle Lead] [--managerUserName boss@contoso.com]` (no password generates one)
- `m365 entra user set --userName jane@contoso.com --jobTitle "Lead" --department Sales --agent` · `--accountEnabled false` disables the account · `--resetPassword --newPassword "..."` · `--removeManager`
- `m365 entra user guest add --emailAddress partner@fabrikam.com --displayName "Pat Partner" --sendInvitationMessage --agent [--inviteRedirectUrl https://contoso.sharepoint.com]`
- `m365 entra user session revoke --userName jane@contoso.com --force --agent` signs the user out everywhere.
- `m365 entra user remove --userName jane@contoso.com --force --agent` · `m365 entra user recyclebinitem list --agent` · `m365 entra user recyclebinitem restore --id USER_ID --agent`
- `m365 entra user groupmembership list --userName jane@contoso.com --agent [--securityEnabledOnly]`

## Licenses, sign-ins and MFA

- `m365 entra license list --agent` · `m365 entra user license list --userName jane@contoso.com --agent`
- `m365 entra user license add --userName jane@contoso.com --ids SKU_ID --agent` · `m365 entra user license remove --userName jane@contoso.com --ids SKU_ID --force --agent`
- `m365 entra user signin list --userName jane@contoso.com --agent [--appDisplayName "Microsoft Teams"]`
- `m365 entra user registrationdetails list --isMfaRegistered false --userType member --agent` finds users without MFA.

## Groups

- `m365 entra group list --type security --agent` · `m365 entra group get --displayName "Sales" --properties id,mail --agent`
- `m365 entra group add --displayName "Sales" --type security --agent [--ownerUserNames a@contoso.com] [--memberUserNames b@contoso.com,c@contoso.com]`
- `m365 entra group member list --groupName "Sales" --role Owner --agent` · `m365 entra group member add --groupName "Sales" --userNames b@contoso.com --role Member --agent` (`--subgroupNames` nests groups)
- `m365 entra group member remove --groupName "Sales" --userNames b@contoso.com --suppressNotFound --force --agent`
- `m365 entra m365group list --displayName Sales --agent [--withSiteUrl] [--orphaned]` · `m365 entra m365group get --displayName "Sales" --withSiteUrl --agent`
- `m365 entra m365group add --displayName "Sales" --mailNickname sales --visibility Private --agent [--owners a@contoso.com] [--members b@contoso.com]`
- `m365 entra m365group user add --groupName "Sales" --userNames b@contoso.com --role owner --agent` · `m365 entra m365group user list --groupDisplayName "Sales" --role Owner --agent`
- `m365 entra m365group teamify --displayName "Sales" --agent` adds a team to the group. `m365 entra m365group renew --displayName "Sales" --agent` resets its expiration.
- Group settings: `m365 entra groupsettingtemplate list --agent`, then `m365 entra groupsetting add --templateId TEMPLATE_ID --ClassificationList "HBI, MBI, LBI" --DefaultClassification MBI --agent` (template properties are extra options).

## Apps and consent

- `m365 entra app list --properties appId,displayName --agent` · `m365 entra app get --appId APP_ID --agent [--save]`
- `m365 entra app add --name "My app" --agent [--multitenant] [--platform spa --redirectUris https://localhost] [--withSecret] [--apisDelegated https://graph.microsoft.com/User.Read] [--grantAdminConsent]` (`--manifest @manifest.json` creates it from a manifest)
- `m365 entra app set --objectId OBJECT_ID --platform web --redirectUris https://contoso.com/auth --agent` · `--certificateFile ./cert.cer` adds a certificate.
- `m365 entra app permission add --appName "My app" --delegatedPermissions "https://graph.microsoft.com/Mail.Read https://graph.microsoft.com/User.Read" --grantAdminConsent --agent` · `m365 entra app permission list --appName "My app" --type application --agent`
- `m365 entra app role add --appName "My app" --name Managers --description "Managers" --allowedMembers usersGroups --claim managers --agent`
- Enterprise apps (service principals): `m365 entra enterpriseapp list --displayName "My app" --agent` · `m365 entra enterpriseapp add --id APP_ID --agent`
- Application permissions on a service principal or managed identity: `m365 entra approleassignment add --appDisplayName "My app" --resource "Microsoft Graph" --scopes "Mail.Read,Mail.Send" --agent` · `m365 entra approleassignment list --appDisplayName "My app" --agent`
- Delegated grants: `m365 entra oauth2grant list --spObjectId SP_OBJECT_ID --agent` · `m365 entra oauth2grant add --clientId SP_OBJECT_ID --resourceId RESOURCE_SP_OBJECT_ID --scope "Calendars.Read Mail.Read" --agent`
- Exchange Online RBAC for apps: `m365 exo approleassignment add --roleDefinitionName "Application Mail.Read" --principalName "My app" --scope group --groupName Sales --agent`

## Roles and directory

- `m365 entra roledefinition list --agent` · `m365 entra roleassignment add --roleDefinitionName "SharePoint Administrator" --principal jane@contoso.com --agent`
- `m365 entra pim role assignment add --roleDefinitionName "Global Reader" --userName jane@contoso.com --duration PT8H --justification "audit" --agent` · `m365 entra pim role assignment eligibility list --userName jane@contoso.com --agent` · `m365 entra pim role request list --agent`
- `m365 entra administrativeunit add --displayName "EMEA" --agent` · `m365 entra administrativeunit member add --administrativeUnitName "EMEA" --userName jane@contoso.com --agent` · `m365 entra administrativeunit roleassignment add --administrativeUnitName "EMEA" --roleDefinitionName "User Administrator" --userName boss@contoso.com --agent`
- `m365 entra policy list --type authorization --agent` · `m365 entra organization list --agent` · `m365 entra siteclassification get --agent`

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `entra user`: add, get, hibp, list, remove, set — `hibp` checks Have I Been Pwned with your `--apiKey`
- `entra user guest`: add
- `entra user groupmembership`: list
- `entra user license`: add, list, remove
- `entra user password`: validate
- `entra user recyclebinitem`: clear, list, remove, restore
- `entra user registrationdetails`: list
- `entra user session`: revoke
- `entra user signin`: list
- `entra license`: list
- `entra group`: add, get, list, remove, set
- `entra group member`: add, list, remove, set
- `entra groupsetting`: add, get, list, remove, set
- `entra groupsettingtemplate`: get, list
- `entra siteclassification`: disable, enable, get, set
- `entra m365group`: add, get, list, remove, renew, set, teamify
- `entra m365group user`: add, list, remove, set
- `entra m365group conversation`: list
- `entra m365group conversation post`: list
- `entra m365group recyclebinitem`: clear, list, remove, restore
- `entra app`: add, get, list, remove, set
- `entra app permission`: add, list, remove
- `entra app role`: add, list, remove
- `entra enterpriseapp`: add, get, list, remove
- `entra approleassignment`: add, list, remove
- `entra oauth2grant`: add, list, remove, set
- `entra resourcenamespace`: list
- `entra rolepermission`: list
- `entra roledefinition`: add, get, list, remove, set
- `entra roleassignment`: add
- `entra pim role assignment`: add, list, remove
- `entra pim role assignment eligibility`: list
- `entra pim role request`: list
- `entra administrativeunit`: add, get, list, remove
- `entra administrativeunit member`: add, get, list, remove
- `entra administrativeunit roleassignment`: add
- `entra organization`: list, set
- `entra multitenant`: add, get, remove, set
- `entra policy`: list
- `entra agent`: list — agent identities
- `entra agent blueprint`: list
- `entra agent blueprintprincipal`: list
- `exo approleassignment`: add — Exchange Online RBAC for apps

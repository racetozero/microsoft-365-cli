---
name: m365-teams
description: Work with Microsoft Teams through the `m365` CLI. Covers teams (create, clone, archive), channels and their members, tabs, channel messages and replies, chats, chat members and messages, online meetings, transcripts and attendance reports, call records, the Teams app catalog and per-user apps, team settings and clearing the local Teams client cache. Builds on the `m365` skill.
---

# Teams with m365

Add `--agent` to every command (see the `m365` skill). Teams are addressed by `--teamId` or `--teamName` (`--id`/`--name` on `teams team`), channels by `--channelId` or `--channelName`. Teams usage reports are in the `m365-admin` skill.

## Nuances

- Several commands take IDs only, not names: `teams message list|get|send`, `teams message reply list`, `teams tab add|list|remove`, `teams user list` and the `*settings` commands. Get the IDs first with `teams team get --name` and `teams channel get --name`.
- Channel IDs look like `19:...@thread.skype` or `19:...@thread.tacv2`; quote them in shells. `teams channel get --teamName Sales --primary` returns the General channel.
- You see and post to messages, tabs and channels only in teams you are a member of. Admins can archive, clone and unarchive teams they are not in.
- `teams team add` without `--wait` returns while the team is still being made; add `--wait` before you add channels or members. Use `entra m365group teamify` (see `m365-entra`) to add a team to an existing group.
- Private and shared channels need `--owner` on create, and only team members can be added to a private channel.
- `teams chat message send --userEmails` reuses the chat with exactly those people or creates one. `teams chat message send`, `teams message remove|restore` and `teams app publish|update|remove` need a delegated (user) login.
- With an application login, `teams chat list` needs `--userId` or `--userName`. Meeting, transcript and attendance commands need an application access policy for the app; `teams callrecord` works only with application permissions and only for the last 30 days.
- `teams meeting add` makes a standalone online meeting that does not show in the calendar; use `outlook event add --isOnlineMeeting` (see `m365-outlook`) for a calendar invite.
- App IDs differ by command: `teams app install`, `teams app remove` and `teams user app add` take the catalog ID from `teams app list`, not the manifest ID. `teams app uninstall` and `teams user app remove` take the installation ID.
- Channel member roles are lowercase (`owner|member`); `teams user list --role` uses `Owner|Member|Guest`. Removed channels and teams stay in a recycle bin for 30 days; restore a team's group with `entra m365group recyclebinitem restore`.
- Dates are ISO 8601 (`2026-01-01T00:00:00Z`). `teams message list --since` returns changes from the last 8 months only.

## Teams

- `m365 teams team list --joined --agent` · `m365 teams team get --name "Sales" --agent`
- `m365 teams team add --name "Sales" --description "Sales team" --ownerUserNames a@contoso.com --memberUserNames b@contoso.com,c@contoso.com --wait --agent` (or `--template`)
- `m365 teams team set --name "Sales" --visibility Private --agent` · `m365 teams team clone --id TEAM_ID --name "Sales EMEA" --partsToClone "apps,tabs,settings,channels" --agent`
- `m365 teams team archive --name "Old project" --shouldSetSpoSiteReadOnlyForMembers --agent` · `m365 teams team unarchive --name "Old project" --agent`
- `m365 teams user list --teamId TEAM_ID --role Owner --agent` (team members are managed with `entra m365group user add --teamName`)

## Channels and tabs

- `m365 teams channel list --teamName "Sales" --agent [--type private]` · `m365 teams channel get --teamName "Sales" --name "Deals" --agent`
- `m365 teams channel add --teamName "Sales" --name "Deals" --description "Open deals" --agent [--type private --owner a@contoso.com]`
- `m365 teams channel set --teamName "Sales" --name "Deals" --newName "Pipeline" --agent` · `m365 teams channel remove --teamName "Sales" --name "Pipeline" --force --agent`
- `m365 teams channel member add --teamName "Sales" --channelName "Deals" --userIds jane@contoso.com --agent [--owner]` · `m365 teams channel member list --teamName "Sales" --channelName "Deals" --role owner --agent`
- `m365 teams channel member set --teamName "Sales" --channelName "Deals" --userName jane@contoso.com --role owner --agent`
- `m365 teams tab list --teamId TEAM_ID --channelId CHANNEL_ID --agent` · `m365 teams tab add --teamId TEAM_ID --channelId CHANNEL_ID --appId APP_ID --appName "Website" --contentUrl https://contoso.com --agent`

## Channel messages

- `m365 teams message list --teamId TEAM_ID --channelId CHANNEL_ID --agent [--since 2026-01-01T00:00:00Z]`
- `m365 teams message send --teamId TEAM_ID --channelId CHANNEL_ID --message "Deployed v2.3" --agent`
- `m365 teams message get --teamId TEAM_ID --channelId CHANNEL_ID --id MESSAGE_ID --agent` · `m365 teams message reply list --teamId TEAM_ID --channelId CHANNEL_ID --messageId MESSAGE_ID --agent`
- `m365 teams message remove --teamName "Sales" --channelName "Deals" --id MESSAGE_ID --force --agent` (your own messages; `teams message restore` undoes it)

## Chats

- `m365 teams chat list --type group --agent` · `m365 teams chat get --participants jane@contoso.com,joe@contoso.com --agent`
- `m365 teams chat message send --userEmails jane@contoso.com --message "Hi" --agent` (or `--chatId`, `--chatName`; `--contentType html`)
- `m365 teams chat message list --chatId CHAT_ID --agent [--createdEndDateTime 2026-02-01T00:00:00Z]`
- `m365 teams chat member add --chatId CHAT_ID --userName jane@contoso.com --withAllHistory --agent` · `m365 teams chat member list --chatId CHAT_ID --agent`

## Meetings and calls

- `m365 teams meeting list --startDateTime 2026-01-01T00:00:00Z --endDateTime 2026-01-31T00:00:00Z --agent [--isOrganizer] [--userName jane@contoso.com]`
- `m365 teams meeting add --subject "Sync" --startTime 2026-01-10T09:00:00Z --endTime 2026-01-10T09:30:00Z --participantUserNames jane@contoso.com --agent`
- `m365 teams meeting get --joinUrl JOIN_URL --agent` gives the meeting ID for the next commands.
- `m365 teams meeting transcript list --meetingId MEETING_ID --agent` · `m365 teams meeting transcript get --meetingId MEETING_ID --id TRANSCRIPT_ID --outputFile ./transcript.vtt --agent`
- `m365 teams meeting attendancereport list --meetingId MEETING_ID --agent` · `m365 teams meeting attendancereport get --meetingId MEETING_ID --id REPORT_ID --agent`
- `m365 teams callrecord list --userName jane@contoso.com --startDateTime 2026-01-01T00:00:00Z --agent` (application login)

## Apps and settings

- `m365 teams app list --distributionMethod organization --agent` · `m365 teams app publish --filePath ./app.zip --agent` · `m365 teams app update --name "My app" --filePath ./app.zip --agent`
- `m365 teams app install --id CATALOG_APP_ID --teamId TEAM_ID --agent` · `m365 teams team app list --teamName "Sales" --agent`
- `m365 teams user app list --userName jane@contoso.com --agent` · `m365 teams user app add --name "My app" --userName jane@contoso.com --agent`
- `m365 teams messagingsettings set --teamId TEAM_ID --allowUserDeleteMessages false --agent` (also `membersettings`, `guestsettings`, `funsettings`, each with `list`)
- `m365 teams cache remove --client new --force --agent` stops the local Teams client and clears its cache.

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `teams team`: add, archive, clone, get, list, remove, set, unarchive
- `teams team app`: list
- `teams user`: list — team members
- `teams channel`: add, get, list, remove, set
- `teams channel member`: add, list, remove, set
- `teams tab`: add, get, list, remove
- `teams message`: get, list, remove, restore, send
- `teams message reply`: list
- `teams chat`: get, list
- `teams chat member`: add, list, remove
- `teams chat message`: list, send
- `teams meeting`: add, get, list
- `teams meeting attendancereport`: get, list
- `teams meeting transcript`: get, list
- `teams callrecord`: get, list
- `teams app`: install, list, publish, remove, uninstall, update
- `teams user app`: add, list, remove, upgrade
- `teams funsettings`: list, set
- `teams guestsettings`: list, set
- `teams membersettings`: list, set
- `teams messagingsettings`: list, set
- `teams cache`: remove — local Teams client

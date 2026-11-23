---
name: m365-teams
description: Work with Microsoft Teams through the `m365` CLI. Covers teams, channels and their members, channel messages and replies, chats and chat messages, meetings, transcripts and attendance reports, apps and team settings. Builds on the `m365` skill.
---

# Teams with m365

Add `--agent` to every command (see the `m365` skill). Teams are addressed by `--teamId` or `--teamName`, channels by `--channelId` or `--channelName`.

## Teams and channels

- `m365 teams team list --agent [--joined]` · `m365 teams team get --name "Sales" --agent`
- `m365 teams team add --name "Sales" --description "Sales team" --agent [--ownerUserNames a@contoso.com] [--memberUserNames b@contoso.com] [--wait]`
- `m365 teams channel list --teamName "Sales" --agent [--type standard|private|shared]`
- `m365 teams channel add --teamName "Sales" --name "Deals" --agent [--type private --owner a@contoso.com]`
- `m365 teams channel member add --teamName "Sales" --channelName "Deals" --userIds USER_ID --agent [--owner]`
- `m365 teams user list --teamId TEAM_ID --agent [--role Owner|Member|Guest]`

## Messages and chats

- `m365 teams message list --teamId TEAM_ID --channelId CHANNEL_ID --agent [--since 2026-01-01T00:00:00Z]`
- `m365 teams message send --teamId TEAM_ID --channelId CHANNEL_ID --message "Hello" --agent`
- `m365 teams chat list --agent [--type oneOnOne|group|meeting]` · `m365 teams chat message list --chatId CHAT_ID --agent`
- `m365 teams chat message send --userEmails jane@contoso.com --message "Hi" --agent` (creates or reuses the chat; or `--chatId`, `--chatName`; `--contentType html`)

## Meetings

- `m365 teams meeting list --startDateTime 2026-01-01T00:00:00Z --agent [--endDateTime ...] [--userName jane@contoso.com]`
- `m365 teams meeting get --joinUrl URL --agent` · `m365 teams meeting transcript list --meetingId ID --agent` · `m365 teams meeting attendancereport list --meetingId ID --agent`

## Apps and settings

- `m365 teams app list --agent [--distributionMethod organization]` · `m365 teams app publish --filePath ./app.zip --agent` · `m365 teams app install --id APP_ID --teamId TEAM_ID --agent`
- `m365 teams user app list --userName jane@contoso.com --agent`
- `m365 teams messagingsettings list --teamId TEAM_ID --agent` (also `membersettings`, `guestsettings`, `funsettings`, each with `set`)
- `m365 teams --help` lists every Teams command.

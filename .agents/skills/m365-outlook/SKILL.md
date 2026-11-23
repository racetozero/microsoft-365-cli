---
name: m365-outlook
description: Work with Outlook mail and calendars, Planner, Microsoft To Do, Bookings and OneNote through the `m365` CLI. Covers sending mail, reading and moving messages, events and calendars, rooms, Planner plans, buckets and tasks, To Do lists and tasks. Builds on the `m365` skill.
---

# Outlook, Planner and To Do with m365

Add `--agent` to every command (see the `m365` skill). Commands act on the signed-in user unless `--userId` or `--userName` is given (application logins need one).

## Mail

- `m365 outlook mail send --to jane@contoso.com --subject "Report" --bodyContents "See attached" --agent [--bodyContentType HTML] [--cc ...] [--attachment ./report.pdf] [--importance high]`
- `m365 outlook message list --folderName inbox --agent [--startTime 2026-01-01T00:00:00Z]` · `m365 outlook message get --id ID --agent`
- `m365 outlook message move --id ID --sourceFolderName inbox --targetFolderName archive --agent`
- `m365 outlook mailbox settings get --agent`

## Calendar

- `m365 outlook event list --agent [--startDateTime 2026-01-01T00:00:00Z --endDateTime 2026-01-31T00:00:00Z] [--calendarName Calendar]`
- `m365 outlook event add --subject "Sync" --start 2026-01-10T09:00:00 --end 2026-01-10T09:30:00 --timeZone "W. Europe Standard Time" --agent [--requiredAttendees jane@contoso.com] [--isOnlineMeeting] [--location "Room 1"]`
- `m365 outlook event cancel --id ID --agent` · `m365 outlook calendar list --agent` · `m365 outlook room list --agent`

## Planner

- `m365 planner plan list --ownerGroupName "Sales" --agent` · `m365 planner bucket list --planTitle "Q1" --ownerGroupName "Sales" --agent`
- `m365 planner task list --planTitle "Q1" --ownerGroupName "Sales" --agent [--bucketName "To do"]`
- `m365 planner task add --title "Call client" --planTitle "Q1" --ownerGroupName "Sales" --bucketName "To do" --agent [--dueDateTime 2026-02-01] [--assignedToUserNames jane@contoso.com] [--description "..."]`
- `m365 planner task set --id TASK_ID --percentComplete 100 --agent`

## To Do, Bookings, OneNote

- `m365 todo list list --agent` · `m365 todo task list --listName Tasks --agent`
- `m365 todo task add --title "Renew license" --listName Tasks --agent [--dueDateTime 2026-02-01] [--importance high]` · `m365 todo task set --id ID --listName Tasks --status completed --agent`
- `m365 booking business list --agent` · `m365 onenote notebook list --agent` · `m365 onenote page list --agent`

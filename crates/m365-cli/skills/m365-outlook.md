---
name: m365-outlook
description: Work with Outlook mail and calendars, Planner, Microsoft To Do, Bookings and OneNote through the `m365` CLI. Covers sending mail (also as or on behalf of another mailbox), reading, moving and deleting messages, search folders, mailbox settings and automatic replies, calendars, calendar groups, events and online meetings, rooms, Planner plans, buckets, tasks, checklists, rosters and tenant settings, To Do lists and tasks, Bookings businesses and OneNote notebooks and pages. Builds on the `m365` skill.
---

# Outlook, Planner and To Do with m365

Add `--agent` to every command (see the `m365` skill). Most commands act on the signed-in user unless `--userId` or `--userName` is given. Outlook usage reports are in the `m365-admin` skill.

## Nuances

- `outlook calendar get|list` and `outlook event get|list` always need `--userId` or `--userName`. Use `--userId @meId` (or `--userName @meUserName`) for the signed-in user.
- With an application login, give `--userId` or `--userName` on every mail and calendar command, and `outlook mail send` needs `--sender`. OneNote commands and `planner tenant settings set` do not work with application permissions.
- `outlook mail send --mailbox shared@contoso.com` sends on behalf of a shared mailbox, group or distribution list (you need "Send on behalf of"); `--sender` sends as another user (needs Full Access and `Mail.Send.Shared`).
- Mail attachments: repeat `--attachment` for each file; all files together must be 3 MB or less. `--bodyContentType` is `Text` by default for mail and `HTML` for events. `--bodyContents @file.html` reads an event body from a file.
- `outlook message remove` deletes permanently. To put a message in the trash, use `outlook message move --targetFolderName deleteditems`. `outlook event remove` and `outlook calendar remove` use the recycle bin unless `--permanent`.
- Event times are `YYYY-MM-DDTHH:mm:ss` and are read as UTC unless you add `--timeZone "W. Europe Standard Time"` (Windows or IANA names). Only the organizer can `outlook event cancel`, which also notifies attendees; online meeting settings cannot change after creation.
- A calendar group must be empty before `outlook calendargroup remove`.
- Planner plans belong to a Microsoft 365 group or a roster. When you use `--planTitle`, also give `--ownerGroupName` or `--ownerGroupId`; `--planId` needs neither. `planner task list` without options lists the signed-in user's tasks.
- Planner `--percentComplete` 0 is Not started, 1 to 99 In progress, 100 Completed. `--priority` is `Urgent|Important|Medium|Low` or 0 to 10. Dates are ISO 8601 (`2026-02-01T00:00:00Z`).
- Planner rosters are a preview API. A roster holds one plan; it is deleted 24 hours after creation if it has no plan, and when its last member leaves.
- To Do `--dueDateTime` uses only the date part, in UTC. Tasks are addressed by `--id` plus `--listName` or `--listId`.

## Mail

- `m365 outlook mail send --to jane@contoso.com --subject "Report" --bodyContents "See attached" --agent [--bodyContentType HTML] [--cc a@contoso.com] [--attachment ./report.pdf --attachment ./data.xlsx] [--importance high] [--saveToSentItems false]`
- `m365 outlook mail send --to team@contoso.com --subject "Update" --bodyContents "..." --mailbox support@contoso.com --agent`
- `m365 outlook message list --folderName inbox --startTime 2026-01-01T00:00:00Z --agent [--endTime 2026-02-01T00:00:00Z] [--userName jane@contoso.com]`
- `m365 outlook message get --id MESSAGE_ID --agent` · `m365 outlook message move --id MESSAGE_ID --sourceFolderName inbox --targetFolderName archive --agent`
- `m365 outlook message remove --id MESSAGE_ID --force --agent` (permanent)
- `m365 outlook mail searchfolder add --folderName "From Jane" --messageFilter "from/emailAddress/address eq 'jane@contoso.com'" --sourceFoldersIds FOLDER_ID --agent [--includeNestedFolders]`

## Mailbox settings

- `m365 outlook mailbox settings get --agent [--userName jane@contoso.com]`
- `m365 outlook mailbox settings set --timeZone "W. Europe Standard Time" --workingDays monday,tuesday,wednesday,thursday,friday --workingHoursStartTime 09:00 --workingHoursEndTime 17:00 --agent`
- `m365 outlook mailbox settings set --autoReplyStatus scheduled --autoReplyStartDateTime 2026-07-01T00:00:00 --autoReplyEndDateTime 2026-07-15T00:00:00 --autoReplyInternalMessage "Out until 15 July" --autoReplyExternalAudience all --autoReplyExternalMessage "Out until 15 July" --agent`

## Calendar

- `m365 outlook calendar list --userId @meId --agent` · `m365 outlook calendar add --name "Holidays" --color lightGreen --agent` · `m365 outlook calendargroup list --agent`
- `m365 outlook event list --userId @meId --startDateTime 2026-01-01T00:00:00Z --endDateTime 2026-01-31T00:00:00Z --agent [--calendarName Calendar] [--timeZone "W. Europe Standard Time"] [--filter "categories/any(c:c eq 'Red')"]`
- `m365 outlook event add --subject "Sync" --start 2026-01-10T09:00:00 --end 2026-01-10T09:30:00 --timeZone "W. Europe Standard Time" --requiredAttendees jane@contoso.com --isOnlineMeeting true --onlineMeetingProvider teamsForBusiness --agent [--location "Room 1"] [--recurrence @recurrence.json]`
- `m365 outlook event get --id EVENT_ID --userId @meId --agent` · `m365 outlook event cancel --id EVENT_ID --comment "Moved to next week" --force --agent`
- `m365 outlook room list --agent [--roomlistEmail bldg2@contoso.com]` · `m365 outlook roomlist list --agent`

## Planner

- `m365 planner plan list --ownerGroupName "Sales" --agent` · `m365 planner plan add --title "Q1" --ownerGroupName "Sales" --agent`
- `m365 planner bucket list --planTitle "Q1" --ownerGroupName "Sales" --agent` · `m365 planner bucket add --name "To do" --planTitle "Q1" --ownerGroupName "Sales" --agent`
- `m365 planner task list --planTitle "Q1" --ownerGroupName "Sales" --agent [--bucketName "To do"]`
- `m365 planner task add --title "Call client" --planTitle "Q1" --ownerGroupName "Sales" --bucketName "To do" --agent [--dueDateTime 2026-02-01T00:00:00Z] [--assignedToUserNames jane@contoso.com] [--priority Important] [--description "..."]`
- `m365 planner task set --id TASK_ID --percentComplete 100 --agent` · `--bucketName Done --planId PLAN_ID` moves the task.
- `m365 planner task checklistitem add --taskId TASK_ID --title "Send quote" --agent` · `m365 planner task reference add --taskId TASK_ID --url https://contoso.com/spec.docx --alias "Spec" --type Word --agent`
- Rosters (plans without a group): `m365 planner roster add --agent`, then `m365 planner roster member add --rosterId ROSTER_ID --userName jane@contoso.com --agent` and `m365 planner plan add --title "Event" --rosterId ROSTER_ID --agent`

## To Do, Bookings and OneNote

- `m365 todo list list --agent` · `m365 todo list add --name "Errands" --agent` · `m365 todo task list --listName Tasks --agent`
- `m365 todo task add --title "Renew license" --listName Tasks --agent [--dueDateTime 2026-02-01] [--importance high] [--bodyContent "notes"] [--reminderDateTime 2026-01-31T09:00:00Z]`
- `m365 todo task set --id TASK_ID --listName Tasks --status completed --agent`
- `m365 booking business list --agent` · `m365 booking business get --name "Contoso Advisors" --agent`
- `m365 onenote notebook list --agent [--groupName "Sales"] [--webUrl https://contoso.sharepoint.com/sites/sales]` · `m365 onenote notebook add --name "Meeting notes" --agent` · `m365 onenote page list --agent`

## Command map

`m365 <command> --help --agent` gives each command's options, rules and examples. Each line is a resource and its verbs.

- `outlook mail`: send
- `outlook mail searchfolder`: add
- `outlook message`: get, list, move, remove
- `outlook mailbox settings`: get, set — time zone, working hours, automatic replies
- `outlook calendar`: add, get, list, remove, set
- `outlook calendargroup`: add, get, list, remove, set
- `outlook event`: add, cancel, get, list, remove
- `outlook room`: list
- `outlook roomlist`: list
- `planner plan`: add, get, list, remove, set
- `planner bucket`: add, get, list, remove, set
- `planner task`: add, get, list, remove, set
- `planner task checklistitem`: add, list, remove
- `planner task reference`: add, list, remove
- `planner roster`: add, get, remove
- `planner roster member`: add, get, list, remove
- `planner roster plan`: list
- `planner tenant settings`: list, set — global administrator
- `todo list`: add, get, list, remove, set
- `todo task`: add, get, list, remove, set
- `booking business`: get, list
- `onenote notebook`: add, list
- `onenote page`: list

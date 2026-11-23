//! Help for the `m365 outlook ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "outlook calendar add",
        summary: "Creates a new calendar for a user",
        options: &[
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both.",
            ),
            ("name", "Name of the calendar."),
            (
                "calendarGroupId",
                "Id of the group where the calendar will belong. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "calendarGroupName",
                "Name of the group where the calendar will belong. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "color",
                "The color of the calendar in UI. Allowed values are `auto`, `lightBlue`, `lightGreen`, `lightOrange`, `lightGray`, `lightYellow`, `lightTeal`, `lightPink`, `lightBrown`, `lightRed`, `maxColor`. Defaults to `auto`.",
            ),
            (
                "defaultOnlineMeetingProvider",
                "The default online meeting provider for meetings sent from the calendar. Allowed values are `none`, `teamsForBusiness`. Defaults to `teamsForBusiness`.",
            ),
            (
                "default",
                "Specify whether the calendar will be the default calendar for new events.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Create a new calendar for a user in a default calendar's group",
                "m365 outlook calendar add --userId '@meId' --name 'Holidays'",
            ),
            (
                "Create a new calendar for a user specified by email in a specific calendar group and defined color",
                "m365 outlook calendar add --userName 'john.doe@contoso.com' --name 'Interviews' --calendarGroupId 'AAMkADY1YmE3N2FhLWEwMz' --color 'lightBlue'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendar get",
        summary: "Retrieves the calendar of a user or a group.",
        options: &[
            ("id", "ID of the calendar."),
            ("name", "Name of the calendar."),
            ("userId", "ID of the user."),
            ("userName", "UPN of the user."),
            (
                "calendarGroupId",
                "ID of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "calendarGroupName",
                "Name of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.Read"),
            ("Application", "Microsoft Graph", "Calendars.Read"),
        ],
        examples: &[
            (
                "Get the calendar for the current signed-in user by id.",
                "m365 outlook calendar get --userId \"@meId\" --id \"AAMkAGI2TGuLAAA=\"",
            ),
            (
                "Get the calendar from a specific calendar group for the current signed-in user by name.",
                "m365 outlook calendar get --userId \"@meId\" --calendarGroupName \"Colleague calendars\" --name \"Calendar\"",
            ),
            (
                "Get the calendar from a specific calendar group for a specific user by name.",
                "m365 outlook calendar get --userId b743445a-112c-4fda-9afd-05943f9c7b36 --calendarGroupId \"AAMkADIxYjJiYmIzLTFmNjYtNGNhMy0YOkcEEh3vhfAAAGgdFjAAA=\"  --name \"Calendar\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendar list",
        summary: "Retrieves a list of all calendars of a user or a group.",
        options: &[
            ("userId", "ID of the user."),
            ("userName", "UPN of the user."),
            (
                "calendarGroupId",
                "ID of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "calendarGroupName",
                "Name of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.Read"),
            ("Application", "Microsoft Graph", "Calendars.Read"),
        ],
        examples: &[
            (
                "List all calendars for the current signed-in user.",
                "m365 outlook calendar list --userId \"@meId\"",
            ),
            (
                "List all calendars in a specific calendar group for the current signed-in user.",
                "m365 outlook calendar list --userId \"@meId\" --calendarGroupName \"Colleague calendars\"",
            ),
            (
                "List all calendars for a specific user.",
                "m365 outlook calendar list --userName \"john.doe@contoso.com\"",
            ),
            (
                "List all calendars from a specific calendar group for a specific user.",
                "m365 outlook calendar list --userId b743445a-112c-4fda-9afd-05943f9c7b36 --calendarGroupId \"AAMkADIxYjJiYmIzLTFmNjYtNGNhMy0YOkcEEh3vhfAAAGgdFjAAA=\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendar remove",
        summary: "Removes the calendar of a user.",
        options: &[
            ("id", "ID of the calendar."),
            ("name", "Name of the calendar."),
            ("userId", "ID of the user."),
            ("userName", "UPN of the user."),
            (
                "calendarGroupId",
                "ID of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "calendarGroupName",
                "Name of the calendar group. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "permanent",
                "Permanently remove the calendar, don't send it to the recycle bin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Remove the calendar for the current signed-in user by id.",
                "m365 outlook calendar remove --userId \"@meId\" --id \"AAMkAGI2TGuLAAA=\"",
            ),
            (
                "Permanently remove the calendar from a specific calendar group for the current signed-in user by name.",
                "m365 outlook calendar remove --userId \"@meId\" --calendarGroupName \"Colleague calendars\" --name \"Calendar\"  --permanent",
            ),
            (
                "Remove the calendar from a specific calendar group for a specific user by name.",
                "m365 outlook calendar remove --userId b743445a-112c-4fda-9afd-05943f9c7b36 --calendarGroupId \"AAMkADIxYjJiYmIzLTFmNjYtNGNhMy0YOkcEEh3vhfAAAGgdFjAAA=\"  --name \"Calendar\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendar set",
        summary: "Updates a calendar for a user.",
        options: &[
            ("id", "ID of the calendar."),
            ("name", "New name of the calendar."),
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "calendarGroupId",
                "ID of the calendar group where the calendar belongs. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "calendarGroupName",
                "Name of the calendar group where the calendar belongs. Specify either `calendarGroupId` or `calendarGroupName`, but not both.",
            ),
            (
                "color",
                "The color of the calendar in the UI. Allowed values: `auto`, `lightBlue`, `lightGreen`, `lightOrange`, `lightGray`, `lightYellow`, `lightTeal`, `lightPink`, `lightBrown`, `lightRed`, `maxColor`.",
            ),
            (
                "isDefault",
                "Mark whether the calendar is the user's default calendar. Allowed values: `true`, `false`.",
            ),
        ],
        notes: &[
            "When using delegated permissions, specifying `userId` or `userName` for a different user requires the `Calendars.ReadWrite.Shared` scope. When the specified user matches the signed-in user, no shared scope is needed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadWrite, Calendars.ReadWrite.Shared",
            ),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Update the name of a calendar for the signed-in user.",
                "m365 outlook calendar set --id 'AAMkAGI2TQpZAAA=' --name 'Team planning'",
            ),
            (
                "Change the color of a specific calendar for a user by UPN.",
                "m365 outlook calendar set --id 'AAMkAGI2TQpZAAA=' --userName 'john.doe@contoso.com' --color 'lightGreen'",
            ),
            (
                "Mark a calendar as the default calendar for a user.",
                "m365 outlook calendar set --id 'AAMkAGI2TQpZAAA=' --userId 'b743445a-112c-4fda-9afd-05943f9c7b36' --isDefault true",
            ),
            (
                "Update a calendar within a specific calendar group by name.",
                "m365 outlook calendar set --id 'AAMkAGI2TQpZAAA=' --name 'Team planning' --calendarGroupName 'My Calendars'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendargroup add",
        summary: "Creates a new calendar group for a user.",
        options: &[
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both.",
            ),
            ("name", "Name of the calendar group."),
        ],
        notes: &[
            "When using delegated permissions, specifying `userId` or `userName` for a different user requires the `Calendars.ReadWrite.Shared` scope. When the specified user matches the signed-in user, no shared scope is needed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadWrite, Calendars.ReadWrite.Shared",
            ),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Create a new calendar group for the current user",
                "m365 outlook calendargroup add --name \"Personal Events\" --userId \"@meId\"",
            ),
            (
                "Create a new calendar group for a user",
                "m365 outlook calendargroup add --name \"Personal Events\" --userId \"44288f7d-7710-4293-8c8e-36f310ed2e6a\"",
            ),
            (
                "Create a new calendar group for a user specified by email",
                "m365 outlook calendargroup add --name \"Personal Events\" --userName \"john.doe@contoso.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendargroup get",
        summary: "Retrieves a calendar group for a user",
        options: &[
            (
                "id",
                "ID of the calendar group. Specify either `id` or `name`, but not both.",
            ),
            (
                "name",
                "Name of the calendar group. Specify either `id` or `name`, but not both.",
            ),
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
        ],
        notes: &[
            "When using delegated permissions, specifying `userId` or `userName` for a different user requires the `Calendars.Read.Shared` or `Calendars.ReadWrite.Shared` scope. When the specified user matches the signed-in user, no shared scope is needed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read, Calendars.Read.Shared",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
        ],
        examples: &[
            (
                "Get the calendar group specified by name for the signed-in user.",
                "m365 outlook calendargroup get --name \"Personal Events\"",
            ),
            (
                "Get the calendar group specified by name for a user using application permissions.",
                "m365 outlook calendargroup get --name \"Personal Events\" --userId \"44288f7d-7710-4293-8c8e-36f310ed2e6a\"",
            ),
            (
                "Get the calendar group specified by id for a user using application permissions.",
                "m365 outlook calendargroup get --id \"AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAAABuC34AAA=\" --userId \"44288f7d-7710-4293-8c8e-36f310ed2e6a\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendargroup list",
        summary: "Retrieves calendar groups for a user.",
        options: &[
            (
                "userId",
                "ID of the user whose calendar groups to retrieve. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user whose calendar groups to retrieve. Specify either `userId` or `userName`, but not both.",
            ),
        ],
        notes: &[
            "When using delegated permissions, specifying `userId` or `userName` for a different user requires the `Calendars.Read.Shared` or `Calendars.ReadWrite.Shared` scope. When the specified user matches the signed-in user, no shared scope is needed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read, Calendars.Read.Shared, Calendars.ReadWrite.Shared",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
        ],
        examples: &[
            (
                "Retrieve all calendar groups for the signed-in user.",
                "m365 outlook calendargroup list",
            ),
            (
                "Retrieve all calendar groups for a specific user by email.",
                "m365 outlook calendargroup list --userName \"john.doe@contoso.com\"",
            ),
            (
                "Retrieve all calendar groups for a specific user by ID.",
                "m365 outlook calendargroup list --userId b743445a-112c-4fda-9afd-05943f9c7b36",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendargroup remove",
        summary: "Removes a calendar group.",
        options: &[
            ("id", "ID of the calendar group to remove."),
            ("name", "Name of the calendar group to remove."),
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "The calendar group must be empty before it can be removed. Make sure to delete all calendars in the group first.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Remove a calendar group specified by id for the signed-in user.",
                "m365 outlook calendargroup remove --id \"AAMkAGE0MGM1Y2M5LWEzMmUtNGVlNy05MjRlLTk0YmYyY2I5NTM3ZAAuAAAAAAC_0WfqSjt_SqLtNkuO-bj1AQAbfYq5lmBxQ6a4t1fGbeYAAAAAAEOAAA=\"",
            ),
            (
                "Remove a calendar group specified by name for the signed-in user.",
                "m365 outlook calendargroup remove --name \"Personal Events\"",
            ),
            (
                "Remove a calendar group specified by id for a user specified by id.",
                "m365 outlook calendargroup remove --id \"AAMkADIxYjJiYm\" --userId \"44288f7d-7710-4293-8c8e-36f310ed2e6a\"",
            ),
            (
                "Remove a calendar group specified by name for a user specified by UPN without prompting for confirmation.",
                "m365 outlook calendargroup remove --name \"Personal Events\" --userName \"john.doe@contoso.com\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook calendargroup set",
        summary: "Updates a calendar group for a user.",
        options: &[
            (
                "id",
                "ID of the calendar group. Specify either `id` or `name`, but not both.",
            ),
            (
                "name",
                "Name of the calendar group. Specify either `id` or `name`, but not both.",
            ),
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both.",
            ),
            ("newName", "New name of the calendar group."),
        ],
        notes: &[
            "When using delegated permissions, specifying `userId` or `userName` for a different user requires the `Calendars.ReadWrite.Shared` scope. When the specified user matches the signed-in user, no shared scope is needed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadWrite, Calendars.ReadWrite.Shared",
            ),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Update the calendar group specified by name for the current user.",
                "m365 outlook calendargroup set --name \"Personal Evts\" --newName \"Personal Events\"",
            ),
            (
                "Update the calendar group specified by id for a user.",
                "m365 outlook calendargroup set --id \"AAMkADIxYjJiYm\" --newName \"Personal Events\" --userId \"44288f7d-7710-4293-8c8e-36f310ed2e6a\"",
            ),
            (
                "Update the calendar group specified by name for a user specified by email.",
                "m365 outlook calendargroup set --name \"Personal Evts\" --newName \"Personal Events\" --userName \"john.doe@contoso.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook event add",
        summary: "Create an event in the default calendar or a specific calendar of a user",
        options: &[
            ("subject", "Subject of the event."),
            (
                "start",
                "Start date and time in format `{date}T{time}` (for example `2026-02-10T09:00:00`).",
            ),
            (
                "end",
                "End date and time in format `{date}T{time}` (for example `2026-02-10T09:30:00`).",
            ),
            (
                "userId",
                "ID of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "calendarId",
                "ID of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "calendarName",
                "Name of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "allowNewTimeProposals",
                "`true` if the organizer allows invitees to propose a new time when responding; otherwise, `false`. Default is `true`.",
            ),
            (
                "bodyContents",
                "Body content of the event. You can also load from a file using `@filePath`.",
            ),
            (
                "bodyContentType",
                "Type of the body content. Available options: `Text`, `HTML`. Default is `HTML`.",
            ),
            ("categories", "Comma-separated list of categories."),
            (
                "hideAttendees",
                "When set to `true`, each attendee only sees themselves in the meeting request and meeting tracking list. Default is `false`.",
            ),
            (
                "importance",
                "Importance of the event. Available options: `low`, `normal`, `high`.",
            ),
            ("isAllDay", "Set to `true` if the event lasts all day."),
            (
                "isOnlineMeeting",
                "Set to `true` to create the event as an online meeting. Default is `false`.",
            ),
            (
                "isReminderOn",
                "Set to `false` if an alert is set to not remind the user of the event. Default is `true`.",
            ),
            (
                "location",
                "Location display name of the event (maps to `location.displayName`). Specify either `location` or `locations`, but not both.",
            ),
            (
                "locationEmailAddress",
                "Optional email address of the location (maps to `location.locationEmailAddress`). Requires `location`.",
            ),
            (
                "locations",
                "Comma-separated list of location display names (maps to `locations[].displayName`). Specify either `location` or `locations`, but not both.",
            ),
            (
                "onlineMeetingProvider",
                "Online meeting provider. Available options: `teamsForBusiness`, `skypeForBusiness`, `skypeForConsumer`, `unknown`. Default is `unknown`.",
            ),
            (
                "optionalAttendees",
                "Comma-separated list of optional attendee email addresses.",
            ),
            (
                "recurrence",
                "Recurrence definition as JSON string. You can also load from a file using `@filePath`.",
            ),
            (
                "reminderMinutesBeforeStart",
                "The number of minutes before the event start time that the reminder alert occurs.",
            ),
            (
                "requiredAttendees",
                "Comma-separated list of required attendee email addresses.",
            ),
            (
                "resources",
                "Comma-separated list of resource attendee email addresses (added as `resource` attendees).",
            ),
            (
                "responseRequested",
                "Default is true, which represents the organizer would like an invitee to send a response to the event.",
            ),
            (
                "sensitivity",
                "Sensitivity of the event. Available options: `normal`, `personal`, `private`, `confidential`.",
            ),
            (
                "showAs",
                "Status to show. Available options: `free`, `tentative`, `busy`, `oof`, `workingElsewhere`, `unknown`. Default is `busy`.",
            ),
            (
                "timeZone",
                "Time zone used for the event start and end. If not specified, the start and end times are in UTC.",
            ),
            (
                "transactionId",
                "Custom identifier to help avoid redundant POST operations in case of client retries.",
            ),
        ],
        notes: &[
            "- `--timeZone` is used to set the `start.timeZone` and `end.timeZone` values in the request. If you pass only `start` and `end` without `timeZone`, the default is UTC.",
            "- For `--isAllDay true`, set `start` and `end` to midnight values and keep them in the same time zone.",
            "- When using application permissions, specify either `--userId` or `--userName`.",
            "- After creating an online meeting by setting `isOnlineMeeting` and `onlineMeetingProvider`, you can’t change those values later.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Create an event in the default calendar of the current signed-in user.",
                "m365 outlook event add --subject \"Weekly sync\" --start \"2026-02-10T09:00:00\" --end \"2026-02-10T09:30:00\"",
            ),
            (
                "Create an event for a specific user using their UPN.",
                "m365 outlook event add --userName \"john.doe@contoso.com\" --subject \"Sprint review\" --start \"2026-02-10T14:00:00\" --end \"2026-02-10T15:00:00\" --timeZone \"Pacific Standard Time\"",
            ),
            (
                "Create an event in a specific calendar of a user.",
                "m365 outlook event add --userName \"john.doe@contoso.com\" --calendarId \"AAMkAGRkZ\" --subject \"Release planning\" --start \"2026-02-12T10:00:00\" --end \"2026-02-12T11:30:00\"",
            ),
            (
                "Create an event with required and optional attendees, categories, and a reminder.",
                "m365 outlook event add --subject \"Project kickoff\" --start \"2026-02-15T09:00:00\" --end \"2026-02-15T10:00:00\" --requiredAttendees \"alex@contoso.com,dana@contoso.com\" --optionalAttendees \"sam@contoso.com\" --categories \"Preset0,Preset6\" --isReminderOn true --reminderMinutesBeforeStart 15",
            ),
            (
                "Create an all-day event with a physical location.",
                "m365 outlook event add --subject \"Company offsite\" --start \"2026-02-20T00:00:00\" --end \"2026-02-21T00:00:00\" --isAllDay true --location \"Head Office\"",
            ),
            (
                "Create an online meeting using Microsoft Teams and disallow new time proposals.",
                "m365 outlook event add --subject \"Customer demo\" --start \"2026-02-18T13:00:00\" --end \"2026-02-18T14:00:00\" --isOnlineMeeting true --onlineMeetingProvider teamsForBusiness --allowNewTimeProposals false",
            ),
            (
                "Create a recurring weekly event using a recurrence definition loaded from file.",
                "m365 outlook event add --subject \"Weekly status meeting\" --start \"2026-02-16T10:00:00\" --end \"2026-02-16T10:30:00\" --recurrence @recurrence.json",
            ),
            (
                "Create an event with HTML body content loaded from a file and a custom transaction ID.",
                "m365 outlook event add --subject \"Town hall\" --start \"2026-02-25T16:00:00\" --end \"2026-02-25T17:30:00\" --bodyContents @invite.html --bodyContentType HTML --transactionId \"townhall-2026-02\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook event cancel",
        summary: "Cancels a calendar event",
        options: &[
            ("id", "ID of the event."),
            (
                "userId",
                "ID of the user that owns the calendar. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "UPN of the user that owns the calendar. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "comment",
                "A comment about the cancellation sent to all the attendees.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &["This action is only available to the organizer of the event."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Cancel a calendar event from the current logged-in user without a comment",
                "m365 outlook event cancel --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
            (
                "Cancel a calendar event from a specific user with a comment",
                "m365 outlook event cancel --userName \"john.doe@contoso.com\" --comment \"Cancelling for this week due to all hands\" --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
            (
                "Cancel a calendar event from a specific user specified by user ID",
                "m365 outlook event cancel --userId 6799fd1a-723b-4eb7-8e52-41ae530274ca --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook event get",
        summary: "Retrieve an event from a specific calendar of a user",
        options: &[
            ("id", "ID of the event."),
            ("userId", "ID of the user."),
            ("userName", "UPN of the user."),
            (
                "calendarId",
                "ID of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "calendarName",
                "Name of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "timeZone",
                "The time zone for the event start and end times. If not specified, the start and end times are in UTC.",
            ),
        ],
        notes: &[
            "When you specify a value for timeZone, consider the options of the time zone list, or additional time zone list.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
        ],
        examples: &[
            (
                "Get an event for the current signed-in user from a calendar specified by id.",
                "m365 outlook event get --id \"AAMkAGVmMDEzMTM4L\" --userId \"@meId\" --calendarId \"AAMkAGRkZ\"",
            ),
            (
                "Get an event for the user from a calendar specified by id and return event times in Pacific Standard Time time zone.",
                "m365 outlook event get --id \"AAMkAGVmMDEzMTM4L\" --userName \"john.doe@contoso.com\" --calendarId \"AAMkAGRkZ\" --timeZone 'Pacific Standard Time'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook event list",
        summary: "Retrieves a list of events from a specific calendar of a user.",
        options: &[
            ("userId", "ID of the user."),
            ("userName", "UPN of the user."),
            (
                "calendarId",
                "ID of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "calendarName",
                "Name of the calendar. Specify either `calendarId` or `calendarName`, but not both.",
            ),
            (
                "startDateTime",
                "Time indicating the inclusive start of a time range when the event starts.",
            ),
            (
                "endDateTime",
                "Time indicating the exclusive end of a time range when the event starts.",
            ),
            (
                "timeZone",
                "The time zone for the event start and end times.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            (
                "filter",
                "OData filter to apply when retrieving the events.",
            ),
        ],
        notes: &[
            "When you specify a value for `timeZone`, consider the options of the time zone list, or additional time zone list",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Calendars.ReadBasic, Calendars.Read",
            ),
        ],
        examples: &[
            (
                "List all events for the current signed-in user from a calendar specified by id.",
                "m365 outlook event list --userId \"@meId\" --calendarId \"AAMkAGRkZ\"",
            ),
            (
                "List all events for the current signed-in user from a calendar specified by id and return event times in Pacific Standard Time time zone.",
                "m365 outlook event list --userId \"@meId\" --calendarId \"AAMkAGRkZ\" --timeZone 'Pacific Standard Time'",
            ),
            (
                "List only id, subject, start time and end time of all events for a specific user and specific calendar",
                "m365 outlook event list --userName \"john.doe@contoso.com\" --calendarName \"Calendar\" --properties \"id,subject,start,end\"",
            ),
            (
                "Filter events for the current signed-in user",
                "m365 outlook event list --userId \"@meId\" --calendarId \"AAMkAGRkZ\" --filter \"contains(subject, 'contoso')\"",
            ),
            (
                "List all events from specific date range",
                "m365 outlook event list --userId \"@meId\" --calendarId \"AAMkAGRkZ\" --startDateTime '2026-01-01' --endDateTime '2026-01-31'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook event remove",
        summary: "Removes an event from a calendar",
        options: &[
            ("id", "ID of the event."),
            (
                "userId",
                "ID of the user that owns the calendar. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "UPN of the user that owns the calendar. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "permanent",
                "Permanently remove the event, don't send it to the recycle bin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "When using the `--permanent` option, the event will be permanently deleted and cannot be recovered.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Calendars.ReadWrite"),
            ("Application", "Microsoft Graph", "Calendars.ReadWrite"),
        ],
        examples: &[
            (
                "Remove a calendar event from the current logged-in user",
                "m365 outlook event remove --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
            (
                "Permanently remove a calendar event from a specific user",
                "m365 outlook event remove --userName \"john.doe@contoso.com\" --permanent --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
            (
                "Remove a calendar event from a specific user specified by user ID",
                "m365 outlook event remove --userId 6799fd1a-723b-4eb7-8e52-41ae530274ca --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAENAAAiIsqMbYjsT5e-T7KzowPTAAAa_WKzAAA=",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook mail searchfolder add",
        summary: "Creates a new mail search folder in the user's mailbox",
        options: &[
            (
                "userId",
                "The id of the user in whose mailbox the search folder should be created. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "The UPN of the user in whose mailbox the search folder should be created. Specify either `userId` or `userName`, but not both.",
            ),
            ("folderName", "The name of the mail search folder."),
            (
                "includeNestedFolders",
                "The nested mail folders will be searched if specified.",
            ),
            ("messageFilter", "The OData query to filter the messages."),
            (
                "sourceFoldersIds",
                "Comma-separated list of mail folders that should be searched.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.ReadWrite"),
            ("Application", "Microsoft Graph", "Mail.ReadWrite"),
        ],
        examples: &[
            (
                "Create a mail search folder in the user's mailbox specified by id for messages from the inbox that contain specific subject",
                "m365 outlook mail searchfolder add --userId 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --folderName 'CLI m365' --sourceFoldersIds 'AQMkADYAAAIBDAAAAA==' --messageFilter \"contains(subject, 'CLI for Microsoft 365')\"",
            ),
            (
                "Create a mail search folder in the user's mailbox specified by UPN for incoming and outgoing messages from a specific year that contain specific text in a message body, search for messages inside all subfolders",
                "m365 outlook mail searchfolder add --userName john.doe@contoso.com --folderName 'Power Platform Community' --sourceFoldersIds 'AQMkADYAAAIBDAAAAA==,AQMkADYAAAIBDBBBBB==' --includeNestedFolders --messageFilter \"contains(body/content,'Power Platform') AND receivedDateTime ge 2024-01-01 AND receivedDateTime le 2024-12-31\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook mail send",
        summary: "Sends an email",
        options: &[
            ("subject", "Email subject."),
            (
                "to",
                "Comma-separated list of emails to send the message to.",
            ),
            (
                "cc",
                "Comma-separated list of CC recipients for the message.",
            ),
            (
                "bcc",
                "Comma-separated list of BCC recipients for the message.",
            ),
            (
                "sender",
                "Optional upn or user id to specify what account to send the message from. Also see the remarks section.",
            ),
            (
                "mailbox",
                "Specify this option to send the email on behalf of another mailbox, for example a shared mailbox, group or distribution list. The sender needs to be a delegate on the specified mailbox. Also see the remarks section.",
            ),
            (
                "bodyContents",
                "String containing the body of the email to send.",
            ),
            (
                "bodyContentType",
                "Type of the body content. Available options: `Text`, `HTML`. Default is `Text`.",
            ),
            (
                "importance",
                "The importance of the message. Available options: `low`, `normal`, or `high`. Default is `normal`.",
            ),
            (
                "attachment",
                "Path to the file that will be added as attachment to the email. This option can be used multiple times to attach multiple attachments.",
            ),
            (
                "saveToSentItems",
                "Save email in the sent items folder. Default `true`.",
            ),
        ],
        notes: &[
            "Attachments:",
            "When using the `attachment` option, note that the total size of all attachment files cannot exceed 3 MB.",
            "If you are connected using app only authentication:",
            "- Always specify a user id or upn in the `--sender` option. The email will be sent as if it came from the specified user, and can optionally be saved in the sent folder of that user account.",
            "- You can optionally also specify the `--mailbox` option to send mail on behalf of a shared mailbox, group or distribution list. The account used in the `--sender` option, needs to have 'Send on behalf of' permissions on the mailbox in question.",
            "You need `Mail.Send` application permissions on the Microsoft Graph to be able to send mails using an application identity.",
            "If you are connected with a regular user account:",
            "- Specify the `--mailbox` option if you want to send an email on behalf of another mailbox. This can be a shared mailbox, group or distribution list. It will be visible in the email that the email was sent by you. You need to be assigned `Send on behalf of` permissions on the mailbox in question.",
            "- You can specify the `--sender` option if you want to send an email as if you were the other user.",
            "The sent email can optionally be saved in the sent folder of that user account. You'll need `Read and manage (Full Access)` permissions on the mailbox of the other user. You can combine the `--sender` and `--mailbox` options to let the other user send a mail on behalf of the specified mailbox.",
            "You need at least `Mail.Send` delegated permissions on the Microsoft Graph to be able to send emails. When specifying another user as sender, you'll need `Mail.Send.Shared` permissions.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.Send"),
            ("Application", "Microsoft Graph", "Mail.Send"),
        ],
        examples: &[
            (
                "Send a text email to the specified email address",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\"",
            ),
            (
                "Send an HTML email to the specified email addresses",
                "m365 outlook mail send --to \"chris@contoso.com,brian@contoso.com\" --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the <a href='https://contoso.sharepoint.com/sites/marketing'>team site</a>\" --bodyContentType HTML",
            ),
            (
                "Send an HTML email to the specified email address loading email contents from a file on disk",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents @email.html --bodyContentType HTML",
            ),
            (
                "Send a text email to the specified email address. Don't store the email in sent items",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\" --saveToSentItems false",
            ),
            (
                "Send an email on behalf of a shared mailbox using the signed in user",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\" --mailbox sales@contoso.com",
            ),
            (
                "Send an email as another user",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\" --sender svc_project@contoso.com",
            ),
            (
                "Send an email as another user, on behalf of a shared mailbox",
                "m365 outlook mail send --to chris@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\" --sender svc_project@contoso.com --mailbox sales@contoso.com",
            ),
            (
                "Send an email with cc and bcc recipients marked as important",
                "m365 outlook mail send --to chris@contoso.com --cc claire@contoso.com --bcc randy@contoso.com --subject \"DG2000 Data Sheets\" --bodyContents \"The latest data sheets are in the team site\" --importance high",
            ),
            (
                "Send an email with multiple attachments",
                "m365 outlook mail send --to chris@contoso.com --subject \"Monthly reports\" --bodyContents \"Here are the reports of this month.\" --attachment \"C:/Reports/File1.jpg\" --attachment \"C:/Reports/File2.docx\" --attachment \"C:/Reports/File3.xlsx\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook mailbox settings get",
        summary: "Gets the user's mailbox settings",
        options: &[
            (
                "userId",
                "The ID of the Microsoft Entra user for which you want to get mailbox settings. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "The UPN of the Microsoft Entra user for which you want to get mailbox settings. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "MailboxSettings.Read"),
            ("Application", "Microsoft Graph", "MailboxSettings.Read"),
        ],
        examples: &[
            (
                "Get mailbox settings of the signed-in user",
                "m365 outlook mailbox settings get",
            ),
            (
                "Get mailbox settings of a user specified by id",
                "m365 outlook mailbox settings get --userId 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook mailbox settings set",
        summary: "Updates user mailbox settings",
        options: &[
            (
                "userId",
                "The ID of the Microsoft Entra user to update mailbox settings for. Either `userId` or `userName` is required when using application permissions.",
            ),
            (
                "userName",
                "The UPN of the Microsoft Entra user to update mailbox settings for. Either `userId` or `userName` is required when using application permissions.",
            ),
            (
                "dateFormat",
                "The date format for the user's mailbox. Example: `dd.MM.yyyy`.",
            ),
            (
                "timeFormat",
                "The time format for the user's mailbox. Example: `H:mm`.",
            ),
            (
                "timeZone",
                "The default time zone for the user's mailbox. Should follow Windows time zone name or IANA time zone identifier. Example: `Central Europe Standard Time`.",
            ),
            (
                "language",
                "The preferred language for the user. Should follow ISO 639-1 Code. Example: `en-US`.",
            ),
            (
                "delegateMeetingMessageDeliveryOptions",
                "Specifies who can receive meeting messages and meeting responses. Allowed values are `sendToDelegateOnly`, `sendToDelegateAndPrincipal`, or `sendToDelegateAndInformationToPrincipal`.",
            ),
            (
                "workingDays",
                "The days of the week on which the user works, separated by a comma. Allowed values are `monday`, `tuesday`, `wednesday`, `thursday`, `friday`, `saturday`, or `sunday`.",
            ),
            (
                "workingHoursStartTime",
                "The time of the day that the user starts working. Example: `09:00`.",
            ),
            (
                "workingHoursEndTime",
                "The time of the day that the user stops working. Example: `17:00`.",
            ),
            (
                "workingHoursTimeZone",
                "The name of a time zone to which the working hours apply. Should follow Windows time zone name or IANA time zone identifier. Example: `Central Europe Standard Time`.",
            ),
            (
                "autoReplyExternalAudience",
                "Specifies external audience who will receive reply message. Allowed values are `none`, `contactsOnly`, or `all`.",
            ),
            (
                "autoReplyExternalMessage",
                "The reply message for the external audience.",
            ),
            (
                "autoReplyInternalMessage",
                "The reply message for the audience from the signed-in user's organization.",
            ),
            (
                "autoReplyStartDateTime",
                "The date and time that automatic replies are set to begin. Example: `2025-01-06T11:00:00.0000000`.",
            ),
            (
                "autoReplyStartTimeZone",
                "The time zone that automatic replies are set to begin.",
            ),
            (
                "autoReplyEndDateTime",
                "The date and time that automatic replies are set to end. Example: `2025-01-07T11:00:00.0000000`.",
            ),
            (
                "autoReplyEndTimeZone",
                "The time zone that automatic replies are set to end.",
            ),
            (
                "autoReplyStatus",
                "The status for automatic replies. Allowed values are `disabled`, `alwaysEnabled`, or `scheduled`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "MailboxSettings.ReadWrite"),
            (
                "Application",
                "Microsoft Graph",
                "MailboxSettings.ReadWrite",
            ),
        ],
        examples: &[
            (
                "Update date, time format and time zone of the signed-in user.",
                "m365 outlook mailbox settings set --dateFormat 'dd.MM.yyyy' --timeFormat 'H:mm' --timeZone 'Central Europe Standard Time' --language 'en-US'",
            ),
            (
                "Update working hours of a user specified by id",
                "m365 outlook mailbox settings set --userId 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --workingDays 'monday,tuesday,thursday,friday' --workingHoursStartTime '08:00' --workingHoursEndTime '16:30' --workingHoursTimeZone 'Central Europe Standard Time'",
            ),
            (
                "Set scheduled automatic replies for the internal audience of a user specified by UPN",
                "m365 outlook mailbox settings set --userName john.doe@contoso.com --autoReplyExternalAudience none --autoReplyInternalMessage 'On vacation' --autoReplyStartDateTime '2024-08-05T08:00:00.0000000' --autoReplyStartTimeZone 'Central Europe Standard Time' --autoReplyEndDateTime  '2024-08-09T16:00:00.0000000' --autoReplyEndTimeZone 'Central Europe Standard Time' --autoReplyStatus scheduled",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook message get",
        summary: "Retrieves specified message",
        options: &[
            ("id", "ID of the message."),
            (
                "userId",
                "ID of the user from which to retrieve the message. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "UPN of the user from which to retrieve the message Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.ReadBasic"),
            ("Application", "Microsoft Graph", "Mail.ReadBasic.All"),
        ],
        examples: &[
            (
                "Get a specific message using delegated permissions.",
                "m365 outlook message get --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA=",
            ),
            (
                "Get a specific message using delegated permissions from a shared mailbox.",
                "m365 outlook message get --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userName sharedmailbox@tenant.com",
            ),
            (
                "Get a specific message from a specific user retrieved by user ID using application permissions.",
                "m365 outlook message get --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userId 6799fd1a-723b-4eb7-8e52-41ae530274ca",
            ),
            (
                "Get a specific message from a specific user retrieved by user principal name using application permissions.",
                "m365 outlook message get --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userName user@tenant.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook message list",
        summary: "Gets all mail messages from the specified folder",
        options: &[
            (
                "folderName",
                "Name of the folder from which to list messages.",
            ),
            ("folderId", "ID of the folder from which to list messages."),
            (
                "startTime",
                "Time indicating the inclusive start of a time range when the message was received. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z).",
            ),
            (
                "endTime",
                "Time indicating the exclusive end of a time range when the message was received. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z).",
            ),
            (
                "userId",
                "The Microsoft Entra user ID. Specify either `userId` or `userName`, not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "User principal name of the user. Specify either `userId` or `userName`, not both. This option is required when using application permissions.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.ReadBasic"),
            ("Application", "Microsoft Graph", "Mail.ReadBasic.All"),
        ],
        examples: &[
            (
                "List all messages from a folder with the specified name received within a specific time frame.",
                "m365 outlook message list --folderName Archive --startTime 2023-12-16T18:28:48.6964197Z --endTime 2024-02-01",
            ),
            (
                "List all messages from a specific folder of a user specified by ID.",
                "m365 outlook message list --folderId AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OAAuAAAAAAAiQ8W967B7TKBjgx9rVEURAQAiIsqMbYjsT5e-T7KzowPTAAAAAAFNAAA= --userId 48d31887-5fad-4d73-a9f5-3c356e68a038",
            ),
            (
                "List all messages from a folder of a user specified by UPN.",
                "m365 outlook message list --folderName inbox --userName john@contoso.com",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/mailfolder?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/mailfolder?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "outlook message move",
        summary: "Moves message to the specified folder",
        options: &[
            ("id", "ID of the message to move"),
            (
                "sourceFolderName",
                "Name of the folder to move the message from",
            ),
            (
                "sourceFolderId",
                "ID of the folder to move the message from",
            ),
            (
                "targetFolderName",
                "Name of the folder to move the message to",
            ),
            ("targetFolderId", "ID of the folder to move the message to"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.ReadWrite"),
            ("Application", "Microsoft Graph", "Mail.ReadWrite"),
        ],
        examples: &[
            (
                "Move the specified message to another folder specified by ID",
                "m365 outlook message move --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALdyzhHAAA= --sourceFolderId AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OAAuAAAAAAAiQ8W967B7TKBjgx9rVEURAQAiIsqMbYjsT5e-T7KzowPTAAAAAAEKAAA= --targetFolderId AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OAAuAAAAAAAiQ8W967B7TKBjgx9rVEURAQAiIsqMbYjsT5e-T7KzowPTAAAeUO-fAAA=",
            ),
            (
                "Move the specified message to another folder specified by name",
                "m365 outlook message move --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALdyzhHAAA= --sourceFolderName Inbox --targetFolderName \"Project X\"",
            ),
            (
                "Move the specified message to another folder specified by its well-known name",
                "m365 outlook message move --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALdyzhHAAA= --sourceFolderName inbox --targetFolderName archive",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/mailfolder?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/mailfolder?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "outlook message remove",
        summary: "Permanently removes a specific message from a mailbox",
        options: &[
            ("id", "ID of the message."),
            (
                "userId",
                "ID of the user that owns the mailbox. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            (
                "userName",
                "User principal name of the user that owns the mailbox. Specify either `userId` or `userName`, but not both. This option is required when using application permissions.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command will permanently delete the Outlook message. To move a message to the trash bin, use outlook message move instead.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Mail.ReadWrite"),
            ("Application", "Microsoft Graph", "Mail.ReadWrite"),
        ],
        examples: &[
            (
                "Remove a specific message for the currently signed-in user",
                "m365 outlook message remove --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA=",
            ),
            (
                "Remove a specific message in a shared mailbox",
                "m365 outlook message remove --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userName sharedmailbox@contoso.com",
            ),
            (
                "Remove a specific message from a specific mailbox specified by user ID using application permissions",
                "m365 outlook message remove --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userId 6799fd1a-723b-4eb7-8e52-41ae530274ca",
            ),
            (
                "Remove a specific message from a specific mailbox specified by user UPN using application permissions",
                "m365 outlook message remove --id AAMkAGVmMDEzMTM4LTZmYWUtNDdkNC1hMDZiLTU1OGY5OTZhYmY4OABGAAAAAAAiQ8W967B7TKBjgx9rVEURBwAiIsqMbYjsT5e-T7KzowPTAAAAAAEMAAAiIsqMbYjsT5e-T7KzowPTAALvuv07AAA= --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailactivitycounts",
        summary: "Enables you to understand the trends of email activity (like how many were sent, read, and received) in your organization",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trends of email activity (like how many were sent, read, and received) in your organization for the last week",
                "m365 outlook report mailactivitycounts --period D7",
            ),
            (
                "Gets the trends of email activity (like how many were sent, read, and received) in your organization for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailactivitycounts --period D7 --output text > \"mailactivitycounts.txt\"",
            ),
            (
                "Gets the trends of email activity (like how many were sent, read, and received) in your organization for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailactivitycounts --period D7 --output json > \"mailactivitycounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailactivityusercounts",
        summary: "Enables you to understand trends on the number of unique users who are performing email activities like send, read, and receive",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trends on the number of unique users who are performing email activities like send, read, and receive for the last week",
                "m365 outlook report mailactivityusercounts --period D7",
            ),
            (
                "Gets the trends on the number of unique users who are performing email activities like send, read, and receive for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailactivityusercounts --period D7 --output text > \"mailactivityusercounts.txt\"",
            ),
            (
                "Gets the trends on the number of unique users who are performing email activities like send, read, and receive for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailactivityusercounts --period D7 --output json > \"mailactivityusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailactivityuserdetail",
        summary: "Gets details about email activity users have performed",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is YYYY-MM-DD. Specify the date or period, but not both",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about email activity users have performed for the last week",
                "m365 outlook report mailactivityuserdetail --period D7",
            ),
            (
                "Gets details about email activity users have performed for May 1st, 2019",
                "m365 outlook report mailactivityuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about email activity users have performed for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailactivityuserdetail --period D7 --output text > \"mailactivityuserdetail.txt\"",
            ),
            (
                "Gets details about email activity users have performed for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailactivityuserdetail --period D7 --output json > \"mailactivityuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailappusageappsusercounts",
        summary: "Gets the count of unique users per email app",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of unique users per email app for the last week",
                "m365 outlook report mailappusageappsusercounts --period D7",
            ),
            (
                "Gets the count of unique users per email app for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailappusageappsusercounts --period D7 --output text > \"mailappusageappsusercounts.txt\"",
            ),
            (
                "Gets the count of unique users per email app for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailappusageappsusercounts --period D7 --output json > \"mailappusageappsusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailappusageusercounts",
        summary: "Gets the count of unique users that connected to Exchange Online using any email app",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of unique users that connected to Exchange Online using any email app for the last week",
                "m365 outlook report mailappusageusercounts --period D7",
            ),
            (
                "Gets the count of unique users that connected to Exchange Online using any email app for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailappusageusercounts --period D7 --output text > \"mailappusageusercounts.txt\"",
            ),
            (
                "Gets the count of unique users that connected to Exchange Online using any email app for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailappusageusercounts --period D7 --output json > \"mailappusageusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailappusageuserdetail",
        summary: "Gets details about which activities users performed on the various email apps",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is YYYY-MM-DD. Specify the date or period, but not both",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about which activities users performed on the various email apps for the last week",
                "m365 outlook report mailappusageuserdetail --period D7",
            ),
            (
                "Gets details about which activities users performed on the various email apps for May 1st, 2019",
                "m365 outlook report mailappusageuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about which activities users performed on the various email apps for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailappusageuserdetail --period D7 --output text > \"mailappusageuserdetail.txt\"",
            ),
            (
                "Gets details about which activities users performed on the various email apps for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailappusageuserdetail --period D7 --output json > \"mailappusageuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailappusageversionsusercounts",
        summary: "Gets the count of unique users by Outlook desktop version.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of unique users by Outlook desktop version for the last week",
                "m365 outlook report mailappusageversionsusercounts --period D7",
            ),
            (
                "Gets the count of unique users by Outlook desktop version for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailappusageversionsusercounts --period D7 --output text > \"mailappusageversionsusercounts.txt\"",
            ),
            (
                "Gets the count of unique users by Outlook desktop version for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailappusageversionsusercounts --period D7 --output json > \"mailappusageversionsusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailboxusagedetail",
        summary: "Gets details about mailbox usage",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about mailbox usage for the last week",
                "m365 outlook report mailboxusagedetail --period D7",
            ),
            (
                "Gets details about mailbox usage for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailboxusagedetail --period D7 --output text > \"mailboxusagedetail.txt\"",
            ),
            (
                "Gets details about mailbox usage for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailboxusagedetail --period D7 --output json > \"mailboxusagedetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailboxusagemailboxcount",
        summary: "Gets the total number of user mailboxes in your organization and how many are active each day of the reporting period.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "A mailbox is considered active if the user sent or read any email.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the total number of user mailboxes in your organization and how many are active each day for the last week.",
                "m365 outlook report mailboxusagemailboxcount --period D7",
            ),
            (
                "Gets the total number of user mailboxes in your organization and how many are active each day for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailboxusagemailboxcount --period D7 --output text > \"mailboxusagemailboxcount.txt\"",
            ),
            (
                "Gets the total number of user mailboxes in your organization and how many are active each day for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailboxusagemailboxcount --period D7 --output json > \"mailboxusagemailboxcount.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailboxusagequotastatusmailboxcounts",
        summary: "Gets the count of user mailboxes in each quota category",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of user mailboxes in each quota category for the last week",
                "m365 outlook report mailboxusagequotastatusmailboxcounts --period D7",
            ),
            (
                "Gets the count of user mailboxes in each quota category for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailboxusagequotastatusmailboxcounts --period D7 --output text > \"mailboxusagequotastatusmailboxcounts.txt\"",
            ),
            (
                "Gets the count of user mailboxes in each quota category for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailboxusagequotastatusmailboxcounts --period D7 --output json > \"mailboxusagequotastatusmailboxcounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook report mailboxusagestorage",
        summary: "Gets the amount of mailbox storage used in your organization",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the amount of mailbox storage used in your organization for the last week",
                "m365 outlook report mailboxusagestorage --period D7",
            ),
            (
                "Gets the amount of mailbox storage used in your organization for the last week and exports the report data in the specified path in text format",
                "m365 outlook report mailboxusagestorage --period D7 --output text > \"mailboxusagestorage.txt\"",
            ),
            (
                "Gets the amount of mailbox storage used in your organization for the last week and exports the report data in the specified path in json format",
                "m365 outlook report mailboxusagestorage --period D7 --output json > \"mailboxusagestorage.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook room list",
        summary: "Gets a collection of all available rooms",
        options: &[(
            "roomlistEmail",
            "Use to filter returned rooms by their roomlist email (eg. bldg2@contoso.com).",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Place.Read.All"),
            ("Application", "Microsoft Graph", "Place.Read.All"),
        ],
        examples: &[
            ("Get all the rooms.", "m365 outlook room list"),
            (
                "Get all the rooms of specified roomlist e-mail address.",
                "m365 outlook room list --roomlistEmail \"bldg2@contoso.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "outlook roomlist list",
        summary: "Gets a collection of available roomlists",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Place.Read.All"),
            ("Application", "Microsoft Graph", "Place.Read.All"),
        ],
        examples: &[(
            "Get all roomlists in your tenant.",
            "m365 outlook roomlist list",
        )],
        links: &[],
    },
];

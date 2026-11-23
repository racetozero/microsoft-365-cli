//! Help for the `m365 todo ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "todo list add",
        summary: "Adds a new Microsoft To Do task list",
        options: &[("name", "The name of the task list to add.")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[(
            "Add a task list with the name My task list",
            "m365 todo list add --name \"My task list\"",
        )],
        links: &[],
    },
    Help {
        name: "todo list get",
        summary: "Returns a specific Microsoft To Do task list",
        options: &[
            ("id", "The id of the list."),
            ("name", "The name of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[
            (
                "Get a specific Microsoft To Do task list based on id.",
                "m365 todo list get --id \"AAMkADY3NmM5ZjhiLTc3M2ItNDg5ZC1iNGRiLTAyM2FmMjVjZmUzOQAuAAAAAAAZ1T9YqZrvS66KkevskFAXAQBEMhhN5VK7RaaKpIc1KhMKAAAZ3e1AAAA=\"",
            ),
            (
                "Get a specific Microsoft To Do task list based on name.",
                "m365 todo list get --name \"Task list\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo list list",
        summary: "Returns a list of Microsoft To Do task lists",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[(
            "Get the list of Microsoft To Do task lists",
            "m365 todo list list",
        )],
        links: &[],
    },
    Help {
        name: "todo list remove",
        summary: "Removes a Microsoft To Do task list",
        options: &[
            ("name", "The name of the task list to remove."),
            ("id", "The ID of the task list to remove."),
            (
                "force",
                "Don't prompt for confirming removing the task list.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a task list with specific name",
                "m365 todo list remove --name \"My task list\"",
            ),
            (
                "Remove a task list with the ID without confirmation prompt",
                "m365 todo list remove --id \"AAMkAGI3NDhlZmQzLWQxYjAtNGJjNy04NmYwLWQ0M2IzZTNlMDUwNAAuAAAAAACQ1l2jfH6VSZraktP8Z7auAQCbV93BagWITZhL3J6BMqhjAAD9pHIhAAA=\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo list set",
        summary: "Updates a Microsoft To Do task list",
        options: &[
            ("id", "The ID of the list to update."),
            ("name", "The display name of the list to update."),
            ("newName", "The new name for the task list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Rename the list with a specific ID",
                "m365 todo list set --id \"AAMkAGI3NDhlZmQzLWQxYjAtNGJjNy04NmYwLWQ0M2IzZTNlMDUwNAAuAAAAAACQ1l2jfH6VSZraktP8Z7auAQCbV93BagWITZhL3J6BMqhjAAD9pHIhAAA=\" --newName \"My updated task list\"",
            ),
            (
                "Rename a list with a specific title",
                "m365 todo list set --name \"My Task list\" --newName \"My updated task list\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo task add",
        summary: "Adds a task to a Microsoft To Do list",
        options: &[
            ("title", "The title of the task."),
            (
                "listName",
                "The name of the list in which to create the task.",
            ),
            ("listId", "The id of the list in which to create the task."),
            (
                "bodyContent",
                "The body content of the task. In the UI this is called 'notes'.",
            ),
            (
                "bodyContentType",
                "The type of the body content. Allowed values: `text` and `html`. Defaults to `text`.",
            ),
            (
                "dueDateTime",
                "The date when the task is due. This should be defined as a valid ISO 8601 string in the UTC time zone. Only date value is needed, time value is always ignored.",
            ),
            (
                "importance",
                "The importance of the task. Allowed values: `low`, `normal`, `high`. Defaults to `normal`.",
            ),
            (
                "reminderDateTime",
                "The date and time for a reminder alert of the task to occur. This should be defined as a valid ISO 8601 string in the UTC time zone.",
            ),
            (
                "categories",
                "Comma-separated list of categories associated with the task.",
            ),
            (
                "completedDateTime",
                "The date and time when the task was finished. This should be defined as a valid ISO 8601 string. `2021-12-16T18:28:48.6964197Z`. This option can only be used when the `status` is set to `completed`.",
            ),
            (
                "startDateTime",
                "The date and time when the task is scheduled to start. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`.",
            ),
            (
                "status",
                "Indicates the state or progress of the task. Allowed values: `notStarted`, `inProgress`, `completed`, `waitingOnOthers`, `deferred`.",
            ),
        ],
        notes: &[
            "When you specify the values for `categories`, each category can correspond to the displayName property of an outlookCategory. It is permissible to use distinct names.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add a task to Microsoft To Do tasks list with with a specific name.",
                "m365 todo task add --title \"New task\" --listName \"My task list\"",
            ),
            (
                "Add a task to a Microsoft To Do tasks list with a specific id.",
                "m365 todo task add --title \"New task\" --listId \"AQMkADlhMTRkOGEzLWQ1M2QtNGVkNS04NjdmLWU0NzJhMjZmZWNmMwAuAAADKvwNgAMNPE_zFNRJXVrU1wEAhHKQZHItDEOVCn8U3xuA2AABmQeVPwAAAA==\"",
            ),
            (
                "Create a new task with bodyContent and reminder and flag it as important.",
                "m365 todo task add --title \"New task\" --listName \"My task list\" --bodyContent \"I should not forget this\" --reminderDateTime 2023-01-01T12:00:00Z --importance high",
            ),
            (
                "Create a new task with a specific due date.",
                "m365 todo task add --title \"New task\" --listId \"AQMkADlhMTRkOGEzLWQ1M2QtNGVkNS04NjdmLWU0NzJhMjZmZWNmMwAuAAADKvwNgAMNPE_zFNRJXVrU1wEAhHKQZHItDEOVCn8U3xuA2AABmQeVPwAAAA==\" --dueDateTime 2023-01-01",
            ),
            (
                "Create a new task with categories, a completedDateTime, a startDateTime and a status.",
                "m365 todo task add --title \"New task\" --listName \"My task list\" --categories \"Red category,Important\" --completedDateTime 2023-12-01 --startDateTime 2023-12-01 --status \"notStarted\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo task get",
        summary: "Gets a specific task from a Microsoft To Do task list",
        options: &[
            ("id", "The ID of the task in the list."),
            (
                "listName",
                "The name of the task list to return tasks from.",
            ),
            ("listId", "The id of the task list to return tasks from."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[
            (
                "Gets a specific task from a Microsoft To Do tasks list based on the name of the list and the task id.",
                "m365 todo task get --listName \"My task list\" --id \"AAMkAGYzNjMxYTU4LTJjZjYtNDlhMi1iMzQ2LWVmMTU3YmUzOGM5MABGAAAAAAAw3-tXgryDSr5p162KnUPKBwDEwEFouXWWT50CfwqSN9cpAAEX8ECDAADEwEFouXWWT50CfwqSN9cpAAEX8GuPAAA=\"",
            ),
            (
                "Gets a specific task from a Microsoft To Do tasks list based on the id of the list and the task id.",
                "m365 todo task get --listId \"AQMkAGYzNjMxYTU4LTJjZjYtNDlhMi1iMzQ2LWVmMTU3YmUzOGM5MAAuAAADMN-7V4K8g0q_adetip1DygEAxMBBaLl1lk_dAn8KkjfXKQABF-BAgwAAAA==\" --id \"AAMkAGYzNjMxYTU4LTJjZjYtNDlhMi1iMzQ2LWVmMTU3YmUzOGM5MABGAAAAAAAw3-tXgryDSr5p162KnUPKBwDEwEFouXWWT50CfwqSN9cpAAEX8ECDAADEwEFouXWWT50CfwqSN9cpAAEX8GuPAAA=\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo task list",
        summary: "Lists tasks from a Microsoft To Do task list",
        options: &[
            (
                "listName",
                "The name of the task list to return tasks from.",
            ),
            ("listId", "The id of the task list to return tasks from."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[
            (
                "List tasks from Microsoft To Do tasks list with the name My task list",
                "m365 todo task list --listName \"My task list\"",
            ),
            (
                "List tasks from Microsoft To Do tasks list with the id AQMkAGYzNjMxYTU4LTJjZjYtNDlhMi1iMzQ2LWVmMTU3YmUzOGM5MAAuAAADMN-7V4K8g0q_adetip1DygEAxMBBaLl1lk_dAn8KkjfXKQABF-BAgwAAAA==",
                "m365 todo task list --listId \"AQMkAGYzNjMxYTU4LTJjZjYtNDlhMi1iMzQ2LWVmMTU3YmUzOGM5MAAuAAADMN-7V4K8g0q_adetip1DygEAxMBBaLl1lk_dAn8KkjfXKQABF-BAgwAAAA==\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo task remove",
        summary: "Removes the specified Microsoft To Do task",
        options: &[
            ("id", "The id of the task to remove."),
            (
                "listName",
                "The name of the task list in which the task exists.",
            ),
            (
                "listId",
                "The id of the task list in which the task exists.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes Microsoft To Do task with the specified id in a list with the specified name",
                "m365 todo task remove --id \"BBMkAGI3NDhlZmQzLWQxYjAtNGJjNy04NmYwLWQ0M2IzZTNlMDUwNAAuAAAAAACQ1l2jfH6VSZraktP8Z7auAQCbV93BagWITZhL3J6BMqhjAAD9pHIhBBB=\" --listName \"Tasks\"",
            ),
            (
                "Removes Microsoft To Do task with the specified id in a list with the specified id",
                "m365 todo task remove --id \"BBMkAGI3NDhlZmQzLWQxYjAtNGJjNy04NmYwLWQ0M2IzZTNlMDUwNAAuAAAAAACQ1l2jfH6VSZraktP8Z7auAQCbV93BagWITZhL3J6BMqhjAAD9pHIhBBB=\" --listId \"AAMkAGI3NDhlZmQzLWQxYjAtNGJjNy04NmYwLWQ0M2IzZTNlMDUwNAAuAAAAAACQ1l2jfH6VSZraktP8Z7auAQCbV93BagWITZhL3J6BMqhjAAD9pHIhAAA=\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "todo task set",
        summary: "Updates a task in a Microsoft To Do task list",
        options: &[
            ("id", "The id of the task to update."),
            ("title", "Sets the task title."),
            (
                "status",
                "Sets the status of the task. Allowed values are `notStarted`, `inProgress`, `completed`, `waitingOnOthers`, `deferred`.",
            ),
            (
                "listName",
                "The name of the task list in which the task exists.",
            ),
            (
                "listId",
                "The id of the task list in which the task exists.",
            ),
            (
                "bodyContent",
                "The body content of the task. In the UI this is called 'notes'.",
            ),
            (
                "bodyContentType",
                "The type of the body content. Allowed values: `text` and `html`. Defaults to `text`.",
            ),
            (
                "importance",
                "The importance of the task. Allowed values: `low`, `normal`, `high`.",
            ),
            (
                "dueDateTime",
                "The date and time when the task is due. This should be defined as a valid ISO 8601 string in the UTC time zone. Only date value is needed, time value is always ignored.",
            ),
            (
                "reminderDateTime",
                "The date and time for a reminder alert of the task to occur. This should be defined as a valid ISO 8601 string in the UTC time zone.",
            ),
            (
                "categories",
                "Comma-separated list of categories associated with the task.",
            ),
            (
                "completedDateTime",
                "The date and time when the task was finished. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`.",
            ),
            (
                "startDateTime",
                "The date and time when the task is scheduled to start. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`.",
            ),
        ],
        notes: &[
            "When you specify the values for `categories`, each category can correspond to the displayName property of an outlookCategory. It is permissible to use distinct names.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update a task with title New task to Update doco in Microsoft To Do tasks list with a specific name.",
                "m365 todo task set --id \"AAMkADU3Y2E0OTMxLTllYTQtNGFlZS1hZGM0LWI1NjZjY2FhM2RhMABGAAAAAADhr7P77n9xS6PdtDemRwpHBwCin1tvQMXzRKN1hQDz2S3VAAAXXsleAACin1tvQMXzRKN1hQDz2S3VAAAXXzr9AAA=\" --title \"Update doco\" --listName \"My task list\"",
            ),
            (
                "Update a task with status from notStarted to inProgress in Microsoft To Do tasks list with a specific name.",
                "m365 todo task set --id \"AAMkADU3Y2E0OTMxLTllYTQtNGFlZS1hZGM0LWI1NjZjY2FhM2RhMABGAAAAAADhr7P77n9xS6PdtDemRwpHBwCin1tvQMXzRKN1hQDz2S3VAAAXXsleAACin1tvQMXzRKN1hQDz2S3VAAAXXzr9AAA=\" --status \"inProgress\" --listName \"My task list\"",
            ),
            (
                "Update a task with bodyContent and reminder and flag it as important in Microsoft To Do tasks list with a specific name.",
                "m365 todo task set --id \"AAMkADU3Y2E0OTMxLTllYTQtNGFlZS1hZGM0LWI1NjZjY2FhM2RhMABGAAAAAADhr7P77n9xS6PdtDemRwpHBwCin1tvQMXzRKN1hQDz2S3VAAAXXsleAACin1tvQMXzRKN1hQDz2S3VAAAXXzr9AAA=\" --listName \"My task list\" --bodyContent \"I should not forget this\" --reminderDateTime 2023-01-01T12:00:00Z --importance high",
            ),
            (
                "Update a task with due date in Microsoft To Do tasks list with list id.",
                "m365 todo task set --id \"AAMkADU3Y2E0OTMxLTllYTQtNGFlZS1hZGM0LWI1NjZjY2FhM2RhMABGAAAAAADhr7P77n9xS6PdtDemRwpHBwCin1tvQMXzRKN1hQDz2S3VAAAXXsleAACin1tvQMXzRKN1hQDz2S3VAAAXXzr9AAA=\" --listId \"AQMkADlhMTRkOGEzLWQ1M2QtNGVkNS04NjdmLWU0NzJhMjZmZWNmMwAuAAADKvwNgAMNPE_zFNRJXVrU1wEAhHKQZHItDEOVCn8U3xuA2AABmQeVPwAAAA==\" --dueDateTime 2023-01-01",
            ),
            (
                "Update a task to Microsoft To Do with several categories.",
                "m365 todo task set --title \"New task\" --listName \"My task list\" --categories \"Red category,Important\"",
            ),
            (
                "Update a task to Microsoft To Do with a start date and set a reminder for the task.",
                "m365 todo task set --title \"New task\" --listName \"My task list\" --startDateTime \"2023-12-16T18:28:48.6964197Z\"",
            ),
        ],
        links: &[],
    },
];

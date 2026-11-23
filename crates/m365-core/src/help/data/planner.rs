//! Help for the `m365 planner ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "planner bucket add",
        summary: "Adds a new Microsoft Planner bucket",
        options: &[
            ("name", "Name of the bucket to add."),
            ("planId", "ID of the plan to which the bucket belongs."),
            (
                "planTitle",
                "Title of the plan to which the bucket belongs.",
            ),
            ("rosterId", "ID of the Planner Roster."),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "orderHint",
                "Hint used to order items of this type in a list view. The format is defined as outlined here (https://learn.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0).",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Adds a Microsoft Planner bucket for a plan based on its ID with an order hint",
                "m365 planner bucket add --name \"My Planner Bucket\" --planId \"xqQg5FS2LkCp935s-FIFm2QAFkHM\" --orderHint \" !\"",
            ),
            (
                "Adds a Microsoft Planner bucket for a plan based on its title and owner group name",
                "m365 planner bucket add --name \"My Planner Bucket\" --planTitle \"My Planner Plan\" --ownerGroupName \"My Planner Group\"",
            ),
            (
                "Adds a Microsoft Planner bucket for a plan based on its Planner Roster ID",
                "m365 planner bucket add --name \"My Planner Bucket\" --rosterId \"RuY-PSpdw02drevnYDTCJpgAEfoI\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner bucket get",
        summary: "Gets the Microsoft Planner bucket in a plan",
        options: &[
            ("id", "ID of the bucket to retrieve details."),
            ("name", "Name of the bucket to retrieve details."),
            (
                "planId",
                "ID of the plan to which the bucket belongs. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "planTitle",
                "Title of the plan to which the bucket belongs. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Gets the specified Microsoft Planner bucket",
                "m365 planner bucket get --id \"5h1uuYFk4kKQ0hfoTUkRLpgALtYi\"",
            ),
            (
                "Gets the Microsoft Planner bucket based on its plan ID",
                "m365 planner bucket get --name \"Planner Bucket A\" --planId \"xqQg5FS2LkCp935s-FIFm2QAFkHM\"",
            ),
            (
                "Gets the Microsoft Planner bucket based on its plan title and owner group name",
                "m365 planner bucket get --name \"Planner Bucket A\" --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "Gets the Microsoft Planner bucket based on its plan title and owner group ID",
                "m365 planner bucket get --name \"Planner Bucket A\" --planTitle \"My Plan\" --ownerGroupId \"ee0f40fc-b2f7-45c7-b62d-11b90dd2ea8e\"",
            ),
            (
                "Gets the Microsoft Planner bucket based on its Roster ID",
                "m365 planner bucket get --name \"Planner Bucket A\" --rosterId \"RuY-PSpdw02drevnYDTCJpgAEfoI\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner bucket list",
        summary: "Lists the Microsoft Planner buckets in a plan",
        options: &[
            ("planId", "ID of the plan to list the buckets of."),
            ("planTitle", "Title of the plan to list the buckets of."),
            ("rosterId", "ID of the Planner Roster."),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Lists the Microsoft Planner buckets based on its plan ID",
                "m365 planner bucket list --planId \"xqQg5FS2LkCp935s-FIFm2QAFkHM\"",
            ),
            (
                "Lists the Microsoft Planner buckets based on its plan title and owner group name",
                "m365 planner bucket list --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "Lists the Microsoft Planner buckets based on its Roster ID",
                "m365 planner bucket list --planTitle \"My Plan\" --rosterId \"RuY-PSpdw02drevnYDTCJpgAEfoI\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner bucket remove",
        summary: "Removes the Microsoft Planner bucket from a plan",
        options: &[
            ("id", "ID of the bucket to remove."),
            ("name", "Name of the bucket to remove."),
            (
                "planId",
                "ID of the plan to which the bucket to remove belongs. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "planTitle",
                "Title of the plan to which the bucket to remove belongs. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Removes the Microsoft Planner bucket by ID",
                "m365 planner bucket remove --id \"vncYUXCRBke28qMLB-d4xJcACtNz\"",
            ),
            (
                "Removes the Microsoft Planner bucket by ID without confirmation",
                "m365 planner bucket remove --id \"vncYUXCRBke28qMLB-d4xJcACtNz\" --force",
            ),
            (
                "Removes the Microsoft Planner bucket by its name based on the plan ID",
                "m365 planner bucket remove --name \"My Bucket\" --planId \"oUHpnKBFekqfGE_PS6GGUZcAFY7b\"",
            ),
            (
                "Removes the Microsoft Planner bucket by its name based on the plan title and owner group name",
                "m365 planner bucket remove --name \"My Bucket\" --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "Removes the Microsoft Planner bucket by its name based on the Roster ID",
                "m365 planner bucket remove --name \"My Bucket\" --rosterId \"RuY-PSpdw02drevnYDTCJpgAEfoI\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner bucket set",
        summary: "Updates a Microsoft Planner bucket",
        options: &[
            ("id", "ID of the bucket."),
            ("name", "Name of the bucket."),
            (
                "planId",
                "ID of the plan to update the bucket of. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "planTitle",
                "Title of the plan to update the bucket of. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`. Always use in combination with either `ownerGroupId` or `ownerGroupName`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` but not multiple when using `name`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            ("newName", "New name of the bucket."),
            (
                "orderHint",
                "Hint used to order items of this type in a list view. The format is defined as outlined here (https://learn.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0).",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Updates the Microsoft Planner bucket by its ID",
                "m365 planner bucket set --id \"vncYUXCRBke28qMLB-d4xJcACtNz\" --newName \"New bucket name\"",
            ),
            (
                "Updates the Microsoft Planner bucket with a given name based on its plan title and owner group name",
                "m365 planner bucket set --name \"My Bucket\" --planTitle \"My Plan\" --ownerGroupName \"My Group\" --newName \"New bucket name\"",
            ),
            (
                "Updates the Microsoft Planner bucket with a given name based on its plan title and owner group ID",
                "m365 planner bucket set --name \"My Bucket\" --planTitle \"My Plan\" --ownerGroupId 00000000-0000-0000-0000-000000000000 --newName \"New bucket name\"",
            ),
            (
                "Updates the Microsoft Planner bucket with a given name based on its Roster ID",
                "m365 planner bucket set --name \"My Bucket\" --rosterId \"RuY-PSpdw02drevnYDTCJpgAEfoI\" --newName \"New bucket name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner plan add",
        summary: "Adds a new Microsoft Planner plan",
        options: &[
            ("title", "Title of the plan to add."),
            (
                "ownerGroupId",
                "ID of the Group that owns the plan. A valid group must exist before this option can be set.",
            ),
            (
                "ownerGroupName",
                "Name of the Group that owns the plan. A valid group must exist before this option can be set.",
            ),
            ("rosterId", "ID of the Planner Roster."),
            (
                "shareWithUserIds",
                "The comma-separated IDs of the users with whom you want to share the plan. Specify either `shareWithUserIds` or `shareWithUserNames` but not both.",
            ),
            (
                "shareWithUserNames",
                "The comma-separated UPNs of the users with whom you want to share the plan. Specify either `shareWithUserIds` or `shareWithUserNames` but not both.",
            ),
        ],
        notes: &[
            "- Related to the options `--shareWithUserIds` and `--shareWithUserNames`. If you are leveraging Microsoft 365 groups, use the `entra m365group user` commands to manage group membership to share the group's plan. You can also add existing members of the group to this collection though it is not required for them to access the plan owned by the group.",
            "- Hint: Unlike for groups, a Planner Roster can contain only 1 plan.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Adds a Microsoft Planner plan with a Group by id.",
                "m365 planner plan add --title 'My Planner Plan' --ownerGroupId '233e43d0-dc6a-482e-9b4e-0de7a7bce9b4'",
            ),
            (
                "Adds a Microsoft Planner plan with with a Group by name.",
                "m365 planner plan add --title 'My Planner Plan' --ownerGroupName 'My Planner Group'",
            ),
            (
                "Adds a Microsoft Planner plan with a Roster by id.",
                "m365 planner plan add --title 'My Planner Plan' --rosterId '8bc07d47-c06f-41e1-8f00-1c113c8f6067'",
            ),
            (
                "Adds a Microsoft Planner plan with with a Group by name and share it with the given users.",
                "m365 planner plan add --title 'My Planner Plan' --ownerGroupName 'My Planner Group' --shareWithUserNames 'Allan.Carroll@contoso.com,Ida.Stevens@contoso.com'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner plan get",
        summary: "Retrieves information about the specified plan",
        options: &[
            ("id", "ID of the plan."),
            ("title", "Title of the plan."),
            ("rosterId", "ID of the Planner Roster."),
            (
                "ownerGroupId",
                "ID of the Group that owns the plan. Specify either `ownerGroupId` or `ownerGroupName` when using `title` but not both.",
            ),
            (
                "ownerGroupName",
                "Name of the Group that owns the plan. Specify either `ownerGroupId` or `ownerGroupName` when using `title` but not both.",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Returns the Microsoft Planner plan by id.",
                "m365 planner plan get --id \"gndWOTSK60GfPQfiDDj43JgACDCb\"",
            ),
            (
                "Returns the Microsoft Planner plan by title and owner group id.",
                "m365 planner plan get --title \"MyPlan\" --ownerGroupId \"233e43d0-dc6a-482e-9b4e-0de7a7bce9b4\"",
            ),
            (
                "Returns the Microsoft Planner plan by title and owner group name.",
                "m365 planner plan get --title \"MyPlan\" --ownerGroupName \"My Planner Group\"",
            ),
            (
                "Returns the Microsoft Planner plan by roster id.",
                "m365 planner plan get --rosterId \"FeMZFDoK8k2oWmuGE-XFHZcAEwtn\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner plan list",
        summary: "Returns a list of plans associated with a specified group",
        options: &[
            ("ownerGroupId", "ID of the Group that owns the plan."),
            ("ownerGroupName", "Name of the Group that owns the plan."),
            ("rosterId", "ID of the Planner Roster."),
        ],
        notes: &[
            "When using rosterId, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Returns a list of Microsoft Planner plans for the specified Group by id.",
                "m365 planner plan list --ownerGroupId \"233e43d0-dc6a-482e-9b4e-0de7a7bce9b4\"",
            ),
            (
                "Returns a list of Microsoft Planner plans for the specified Group by name.",
                "m365 planner plan list --ownerGroupName \"My Planner Group\"",
            ),
            (
                "Returns a list of Microsoft Planner plans for the specified Roster.",
                "m365 planner plan list --rosterId \"FeMZFDoK8k2oWmuGE-XFHZcAEwtn\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner plan remove",
        summary: "Removes the Microsoft Planner plan",
        options: &[
            ("id", "ID of the plan to remove."),
            ("title", "Title of the plan to remove."),
            (
                "ownerGroupId",
                "ID of the Group that owns the plan. Specify either `ownerGroupId` or `ownerGroupName` when using `title`.",
            ),
            (
                "ownerGroupName",
                "Name of the Group that owns the plan. Specify either `ownerGroupId` or `ownerGroupName` when using `title`.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "If you wish to delete a Planner plan contained within a Planner Roster, you'll have to remove the roster using planner roster remove.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Removes the Microsoft Planner plan by ID",
                "m365 planner plan remove --id gndWOTSK60GfPQfiDDj43JgACDCb",
            ),
            (
                "Removes the Microsoft Planner plan with title My Plan in group with specific ID",
                "m365 planner plan remove --title \"My Plan\" --ownerGroupId 00000000-0000-0000-0000-000000000000",
            ),
            (
                "Removes the Microsoft Planner plan with title My Plan in group with name My Planner Group without confirmation prompt",
                "m365 planner plan remove --title \"My Plan\" --ownerGroupName \"My Planner Group\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner plan set",
        summary: "Updates a Microsoft Planner plan",
        options: &[
            ("id", "ID of the plan."),
            ("title", "Title of the plan."),
            ("rosterId", "ID of the Planner Roster."),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `title` but not both.",
            ),
            (
                "ownerGroupName",
                "Name of the Group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `title` but not both.",
            ),
            ("newTitle", "New title of the plan."),
            (
                "shareWithUserIds",
                "The comma-separated IDs of the users with whom you want to share the plan. Specify either `shareWithUserIds` or `shareWithUserNames` but not both.",
            ),
            (
                "shareWithUserNames",
                "The comma-separated UPNs of the users with whom you want to share the plan. Specify either `shareWithUserIds` or `shareWithUserNames` but not both.",
            ),
        ],
        notes: &[
            "This command allows using unknown options.",
            "    `--category1 [category1]`\n    : New label for a category. Define the category key within your option to update the related label. Category 1 to 25 are available. E.g., `--category4`, `--category12`.",
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Updates a Microsoft Planner plan title.",
                "m365 planner plan set --id 'gndWOTSK60GfPQfiDDj43JgACDCb' --newTitle 'New Title'",
            ),
            (
                "Share a Microsoft Planner plan owned by the group, with the users.",
                "m365 planner plan set --title 'Plan Title' --ownerGroupName 'My Group' --shareWithUserNames 'user1@contoso.com,user2@contoso.com'",
            ),
            (
                "Updates a Microsoft Planner plan category labels.",
                "m365 planner plan set --id 'gndWOTSK60GfPQfiDDj43JgACDCb' --category21 'ToDo' --category25 'Urgent'",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/graph/api/plannerplan-update?view=graph-rest-1.0&tabs=http",
                "https://learn.microsoft.com/graph/api/plannerplan-update?view=graph-rest-1.0&tabs=http",
            ),
            (
                "https://learn.microsoft.com/graph/api/resources/plannerplandetails?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/resources/plannerplandetails?view=graph-rest-1.0",
            ),
            (
                "https://learn.microsoft.com/graph/api/resources/plannercategorydescriptions?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/resources/plannercategorydescriptions?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "planner roster add",
        summary: "Creates a new Microsoft Planner Roster",
        options: &[],
        notes: &[
            "The Roster will be automatically deleted when it doesn't contain a plan 24 hours after its creation. Membership information will be completely erased within 30 days of this deletion.",
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "To be able to create a new Roster, the Planner Roster creation tenant setting should be enabled. Use the planner tenant settings list command to check if this setting is enabled for your tenant.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[(
            "Creates a new Microsoft Planner Roster",
            "m365 planner roster add",
        )],
        links: &[],
    },
    Help {
        name: "planner roster get",
        summary: "Gets information about the specific Microsoft Planner Roster",
        options: &[("id", "ID of the Planner Roster.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[(
            "Gets information about a specific Planner Roster.",
            "m365 planner roster get --id 8bc07d47-c06f-41e1-8f00-1c113c8f6067",
        )],
        links: &[],
    },
    Help {
        name: "planner roster member add",
        summary: "Adds a user to a Microsoft Planner Roster",
        options: &[
            ("rosterId", "ID of the Planner Roster."),
            ("userId", "User's Microsoft Entra ID."),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com).",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Adds a specific user by user name as member of the Planner Roster.",
                "m365 planner roster member add --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userName john.doe@contoso.com",
            ),
            (
                "Adds a specific user by ID as member of the Planner Roster.",
                "m365 planner roster member add --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userId d049a857-f1c3-4fb3-a629-d8cfb3bd7275",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner roster member get",
        summary: "Gets a member of the specified Microsoft Planner Roster",
        options: &[
            ("rosterId", "ID of the Planner Roster."),
            ("userId", "User's Microsoft Entra ID."),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com).",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[
            (
                "Gets a specific user by user name as member of the Planner Roster.",
                "m365 planner roster member get --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userName john.doe@contoso.com",
            ),
            (
                "Gets a specific user by ID as member of the Planner Roster.",
                "m365 planner roster member get --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userId d049a857-f1c3-4fb3-a629-d8cfb3bd7275",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner roster member list",
        summary: "Lists members of the specified Microsoft Planner Roster",
        options: &[("rosterId", "ID of the Planner Roster.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[(
            "Lists members of the specified Microsoft Planner Roster.",
            "m365 planner roster member list --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067",
        )],
        links: &[],
    },
    Help {
        name: "planner roster member remove",
        summary: "Removes a member from a Microsoft Planner Roster",
        options: &[
            ("rosterId", "ID of the Planner Roster."),
            ("userId", "User's Microsoft Entra ID."),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com).",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "The Planner Roster will be deleted when it doesn't have any users remaining in the membership list because the last user removed themselves. Roster, its plan and all contained tasks will be deleted within 30 days of this operation. We will show an extra prompt when this happens. This prompt is also suppressed when specifying `-f, --force`.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a Roster member by its Microsoft Entra ID.",
                "m365 planner roster member remove --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userId 126878e5-d8f9-4db2-951d-d25486488d38",
            ),
            (
                "Remove a Roster member by its UPN.",
                "m365 planner roster member remove --rosterId 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner roster plan list",
        summary: "Lists all Microsoft Planner Roster plans for a specific user",
        options: &[
            (
                "userId",
                "User's Microsoft Entra ID. Specify either `userId` or `userName` but not both. Specify this option only when using application permissions.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Specify either `userId` or `userName` but not both. Specify this option only when using application permissions.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[
            (
                "List all Planner plans contained in a Roster where the current logged in user is member of.",
                "m365 planner roster plan list",
            ),
            (
                "List all Planner plans contained in a Roster where specific user is member of by its UPN.",
                "m365 planner roster plan list --userName john.doe@contoso.com",
            ),
            (
                "List all Planner plans contained in a Roster where specific user is member of by its Id.",
                "m365 planner roster plan list --userId 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner roster remove",
        summary: "Removes a Microsoft Planner Roster",
        options: &[
            ("id", "ID of the Planner Roster."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "Deleting a Planner Roster will also delete the plan within the Roster.",
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a Planner Roster",
                "m365 planner roster remove --id 8bc07d47-c06f-41e1-8f00-1c113c8f6067",
            ),
            (
                "Removes a Planner Roster without confirmation prompt",
                "m365 planner roster remove --id 8bc07d47-c06f-41e1-8f00-1c113c8f6067 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task add",
        summary: "Adds a new Microsoft Planner task",
        options: &[
            ("title", "Title of the task to add."),
            ("planId", "ID of the plan to which the task belongs."),
            ("planTitle", "Title of the plan to which the task belongs."),
            ("rosterId", "ID of the Planner Roster."),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "bucketId",
                "ID of the bucket to which the task belongs. The bucket needs to exist in the selected plan.",
            ),
            (
                "bucketName",
                "Name of the bucket to which the task belongs. The bucket needs to exist in the selected plan.",
            ),
            (
                "startDateTime",
                "The date and time when the task started. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`",
            ),
            (
                "dueDateTime",
                "The date and time when the task is due. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`",
            ),
            (
                "percentComplete",
                "Percentage of task completion. Number between 0 and 100.",
            ),
            (
                "assignedToUserIds",
                "The comma-separated IDs of the assignees the task is assigned to. Specify either `assignedToUserIds` or `assignedToUserNames` but not both.",
            ),
            (
                "assignedToUserNames",
                "The comma-separated UPNs of the assignees the task is assigned to. Specify either `assignedToUserIds` or `assignedToUserNames` but not both.",
            ),
            (
                "assigneePriority",
                "Hint used to order items of this type in a list view. The format is defined as outlined here (https://docs.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0).",
            ),
            ("description", "Description of the task."),
            (
                "appliedCategories",
                "Comma-separated categories that should be added to the task. The possible options are: `category1`, `category2`, `category3`, `category4`, `category5` and/or `category6`. Additional info defined here (https://docs.microsoft.com/graph/api/resources/plannerappliedcategories?view=graph-rest-1.0).",
            ),
            (
                "previewType",
                "This sets the type of preview that shows up on the task. Allowed values: `automatic`, `noPreview`, `checklist`, `description`, `reference`. When set to automatic the displayed preview is chosen by the app viewing the task. Defaults to `automatic`.",
            ),
            (
                "orderHint",
                "Hint used to order items of this type in a list view. The format is defined as outlined here (https://docs.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0).",
            ),
            (
                "priority",
                "Priority of the task. Allowed values: `Urgent`, `Important`, `Medium`, `Low`. Or an integer between 0 and 10 (check remarks section for more info). Defaults to `Medium`.",
            ),
        ],
        notes: &[
            "When you specify the value for `percentComplete`, consider the following:",
            "- when set to 0, the task is considered Not started",
            "- when set between 1 and 99, the task is considered In progress",
            "- when set to 100, the task is considered Completed",
            "When you specify an integer value for `priority`, consider the following:",
            "- values 0 and 1 are interpreted as Urgent",
            "- values 2, 3 and 4 are interpreted as Important",
            "- values 5, 6 and 7 are interpreted as Medium",
            "- values 8, 9 and 10 are interpreted as Low",
            "When using `description` with a multiple lines value, use the new line character of the shell you are using to indicate line breaks. For PowerShell this is `` `n ``. For Zsh or Bash use `\\n` with a `$` in front. E.g. `$\"Line 1\\nLine 2\"`.",
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All, User.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Adds a Microsoft Planner task with the name for plan with the specified ID and specified bucket with the ID.",
                "m365 planner task add --title \"My Planner Task\" --planId \"8QZEH7b3wkSbGQobscsM5gADCBa\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\"",
            ),
            (
                "Adds a Completed Microsoft Planner task with the name for plan with the specified title owned by specified group and the specified bucket with the ID.",
                "m365 planner task add --title \"My Planner task\" --planTitle \"My Planner Plan\" --ownerGroupName \"My Planner Group\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\" --percentComplete 100",
            ),
            (
                "Adds a Microsoft Planner task with the specified name for the plan with the specified ID and bucket with the ID. The new task will be assigned to the specified users and receive a specified due date.",
                "m365 planner task add --title \"My Planner Task\" --planId \"8QZEH7b3wkSbGQobscsM5gADCBa\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\" --assignedToUserNames \"Allan.Carroll@contoso.com,Ida.Stevens@contoso.com\" --dueDateTime \"2021-12-16\"",
            ),
            (
                "Adds a Microsoft Planner task with the specified name for the plan with the specified ID and bucket with the ID. The new task will be assigned to the specified users who will appear first with the asssignee priority ' !'",
                "m365 planner task add --title \"My Planner Task\" --planId \"8QZEH7b3wkSbGQobscsM5gADCBa\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\" --assignedToUserNames \"Allan.Carroll@contoso.com,Ida.Stevens@contoso.com\" --assigneePriority ' !'",
            ),
            (
                "Adds a Microsoft Planner task with the specified name for the plan with the specified ID and the bucket with the ID. The new task will receive the specified categories and get a specified preview with the type.",
                "m365 planner task add --title \"My Planner Task\" --planId \"8QZEH7b3wkSbGQobscsM5gADCBa\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\" --appliedCategories \"category1,category3\" --previewType \"noPreview\"",
            ),
            (
                "Adds a Microsoft Planner task with the specified name for the plan with the specified rosterId and bucket with the ID.",
                "m365 planner task add --title \"My Planner Task\" --rosterId \"8bc07d47-c06f-41e1-8f00-1c113c8f6067\" --bucketId \"IK8tuFTwQEa5vTonM7ZMRZgAKdna\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task checklistitem add",
        summary: "Adds a new checklist item to a Planner task",
        options: &[
            ("taskId", "ID of the task."),
            ("title", "Title of the checklist item."),
            ("isChecked", "Mark the checklist item as checked."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Adds an unchecked checklist item with the specified title to a Microsoft Planner task with the specified ID.",
                "m365 planner task checklistitem add --taskId 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2 --title \"My checklist item\"",
            ),
            (
                "Adds a checked checklist item with the specified title to a Microsoft Planner task with the specified ID.",
                "m365 planner task checklistitem add --taskId 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2 --title \"My checklist item\" --isChecked",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task checklistitem list",
        summary: "Lists the checklist items of a Planner task.",
        options: &[("taskId", "ID of the task.")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[(
            "Lists the checklist items of a Planner task.",
            "m365 planner task checklistitem list --taskId 'vzCcZoOv-U27PwydxHB8opcADJo-'",
        )],
        links: &[],
    },
    Help {
        name: "planner task checklistitem remove",
        summary: "Removes the checklist item from the Planner task.",
        options: &[
            ("id", "ID of the checklist item."),
            ("taskId", "ID of the task."),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a checklist item with the id 40012 from the Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task checklistitem remove --id \"40012\" --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\"",
            ),
            (
                "Removes a checklist item with the id 40012 from the Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2 without prompt",
                "m365 planner task checklistitem remove --id \"40012\" --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task get",
        summary: "Retrieves the specified planner task",
        options: &[
            (
                "id",
                "ID of the task. When you specify the task ID, you no longer need to provide the information for `bucket`, `plan`, and `ownerGroup`.",
            ),
            ("title", "Title of the task."),
            (
                "bucketId",
                "ID of the bucket to which the task belongs. Specify `bucketId` or `bucketName` when using `title`.",
            ),
            (
                "bucketName",
                "Name of the bucket to which the task belongs. Specify `bucketId` or `bucketName` when using `title`.",
            ),
            (
                "planId",
                "ID of the plan to which the task belongs. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "planTitle",
                "Title of the plan to which the task belongs. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Returns the Microsoft Planner task by id.",
                "m365 planner task get --id \"vzCcZoOv-U27PwydxHB8opcADJo-\"",
            ),
            (
                "Retrieve the Microsoft Planner task by title from the specified bucket. Based on the specified plan with the title owned by the specified group.",
                "m365 planner task get --title \"My Planner Task\" --bucketName \"My Planner Bucket\" --planTitle \"My Planner Plan\" --ownerGroupName \"My Planner Group\"",
            ),
            (
                "Returns the Microsoft Planner task by rosterId from the specified bucket.",
                "m365 planner task get --title \"New Task\" --bucketName \"To do\" --rosterId \"8bc07d47-c06f-41e1-8f00-1c113c8f6067\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task list",
        summary: "Lists planner tasks in a bucket, plan, or tasks for the currently logged in user",
        options: &[
            (
                "bucketId",
                "ID of the bucket to list the tasks of. To retrieve tasks from a bucket, specify `bucketId` or `bucketName`, but not both.",
            ),
            (
                "bucketName",
                "Name of the bucket to list the tasks of. To retrieve tasks from a bucket, specify `bucketId` or `bucketName`, but not both.",
            ),
            (
                "planId",
                "ID of a plan to list the tasks of. To retrieve all tasks from a plan, Specify either `planId`, `planTitle`, or `rosterId` but not multiple. Use in combination with `bucketName` to retrieve tasks from a specific bucket.",
            ),
            (
                "planTitle",
                "Title of a plan to list the tasks of. To retrieve all tasks from a plan, Specify either `planId`, `planTitle`, or `rosterId` but not multiple. Always use in combination with either `ownerGroupId` or `ownerGroupName`. Use in combination with `bucketName` to retrieve tasks from a specific bucket.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` but not multiple.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
        ],
        notes: &[
            "This command uses API that is currently in preview to enrich the results with the `priority` field. Keep in mind that this preview API is subject to change once the API reached general availability.",
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.Read, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.Read.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "List tasks for the currently logged in user.",
                "m365 planner task list",
            ),
            (
                "List the Microsoft Planner tasks in the plan by id.",
                "m365 planner task list --planId \"iVPMIgdku0uFlou-KLNg6MkAE1O2\"`",
            ),
            (
                "List the Microsoft Planner tasks in the specified plan by title from the specified group by name.",
                "m365 planner task list --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "List the Microsoft Planner tasks in the bucket by id.",
                "m365 planner task list --bucketId \"FtzysDykv0-9s9toWiZhdskAD67z\"",
            ),
            (
                "List the Microsoft Planner tasks in the bucket by name belonging to plan by id.",
                "m365 planner task list --bucketName \"My Bucket\" --planId \"iVPMIgdku0uFlou-KLNg6MkAE1O2\"",
            ),
            (
                "List the Microsoft Planner tasks in the bucket by name belonging to plan by title in the specified group.",
                "m365 planner task list --bucketName \"My Bucket\" --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "List the Microsoft Planner tasks by rosterId from the specified bucket.",
                "m365 planner task list --bucketName \"My Bucket\" --rosterId \"8bc07d47-c06f-41e1-8f00-1c113c8f6067\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task reference add",
        summary: "Adds a new reference to a Planner task.",
        options: &[
            ("taskId", "ID of the task."),
            ("url", "URL location of the reference."),
            ("alias", "A name alias to describe the reference."),
            (
                "type",
                "Used to describe the type of the reference. Types include: `PowerPoint`, `Word`, `Excel`, `Other`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add a new reference with the url https://www.microsoft.com to a Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task reference add --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --url \"https://www.microsoft.com\"",
            ),
            (
                "Add a new reference with the url https://www.microsoft.com and with the alias Parker to a Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task reference add --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --url \"https://www.microsoft.com\" --alias \"Parker\"",
            ),
            (
                "Add a new reference with the url https://www.microsoft.com and with the type Excel to a Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task reference add --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --url \"https://www.microsoft.com\" --type \"Excel\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task reference list",
        summary: "Retrieves the references associated to a Planner task",
        options: &[("taskId", "ID of the task to retrieve references from")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.Read"),
            ("Application", "Microsoft Graph", "Tasks.Read.All"),
        ],
        examples: &[(
            "Retrieve the references of the specified planner task",
            "m365 planner task reference list --taskId uBk5fK_MHkeyuPYlCo4OFpcAM",
        )],
        links: &[],
    },
    Help {
        name: "planner task reference remove",
        summary: "Removes the reference from the Planner task.",
        options: &[
            ("url", "URL location of the reference."),
            ("alias", "The alias of the reference."),
            ("taskId", "ID of the task."),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Tasks.ReadWrite"),
            ("Application", "Microsoft Graph", "Tasks.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a reference with the url https://www.microsoft.com from the Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task reference remove --url \"https://www.microsoft.com\" --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\"",
            ),
            (
                "Removes a reference with the alias Parker from the Planner task with the id 2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2",
                "m365 planner task reference remove --alias \"Parker\" --taskId \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task remove",
        summary: "Removes the Microsoft Planner task from a plan",
        options: &[
            ("id", "ID of the task to remove."),
            ("title", "Title of the task to remove."),
            (
                "bucketId",
                "ID of the bucket to which the task to remove belongs. Specify either `bucketId` or `bucketName` but not both.",
            ),
            (
                "bucketName",
                "Name of the bucket to which the task to remove belongs. Specify either `bucketId` or `bucketName` but not both.",
            ),
            (
                "planId",
                "ID of the plan to which the task to remove belongs. Specify either `planId`, `planTitle`, or `rosterId` when using `title`.",
            ),
            (
                "planTitle",
                "Title of the plan to which the task to remove belongs. Specify either `planId`, `planTitle`, or `rosterId` when using `title`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster. Specify either `planId`, `planTitle`, or `rosterId` when using `title`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify either `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Removes the Microsoft Planner task by ID.",
                "m365 planner task remove --id \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\"",
            ),
            (
                "Removes the Microsoft Planner task by ID without confirmation.",
                "m365 planner task remove --id \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --force",
            ),
            (
                "Removes the Microsoft Planner task with the specified title in the bucket by ID.",
                "m365 planner task remove --title \"My Task\" --bucketId \"vncYUXCRBke28qMLB-d4xJcACtNz\"",
            ),
            (
                "Removes the Microsoft Planner task with the specified title the bucket by name in the Plan with the specified ID.",
                "m365 planner task remove --title \"My Task\" --bucketName \"My Bucket\" --planId \"oUHpnKBFekqfGE_PS6GGUZcAFY7b\"",
            ),
            (
                "Removes the Microsoft Planner task with the specified title in the bucket with name in the Plan owned by group with the specified name.",
                "m365 planner task remove --title \"My Task\" --bucketName \"My Bucket\" --planTitle \"My Plan\" --ownerGroupName \"My Group\"",
            ),
            (
                "Removes the Microsoft Planner task with the specified title in the bucket with name in the Plan owned by group with the specified ID.",
                "m365 planner task remove --title \"My Task\" --bucketName \"My Bucket\" --planTitle \"My Plan\" --ownerGroupId \"00000000-0000-0000-0000-000000000000\"",
            ),
            (
                "Removes the Microsoft Planner task by rosterId from the specified bucket.",
                "m365 planner task remove --title \"My Task\" --bucketName \"My Bucket\" --rosterId \"8bc07d47-c06f-41e1-8f00-1c113c8f6067\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "planner task set",
        summary: "Updates a Microsoft Planner task",
        options: &[
            ("id", "ID of the task."),
            ("title", "New title of the task."),
            (
                "bucketId",
                "ID of the bucket to move the task to. Specify either `bucketId` or `bucketName` when using `title`.",
            ),
            (
                "bucketName",
                "Name of the bucket to move the task to. The bucket needs to exist in the selected plan. Specify either `bucketId` or `bucketName` when using `title`.",
            ),
            (
                "planId",
                "ID of the plan to which the bucket belongs to. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "planTitle",
                "Title of the plan to which the bucket belongs to. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "rosterId",
                "ID of the Planner Roster to which the bucket belongs to. Specify either `planId`, `planTitle`, or `rosterId` when using `bucketName`.",
            ),
            (
                "ownerGroupId",
                "ID of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "ownerGroupName",
                "Name of the group to which the plan belongs. Specify `ownerGroupId` or `ownerGroupName` when using `planTitle`.",
            ),
            (
                "startDateTime",
                "The date and time when the task started. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`",
            ),
            (
                "dueDateTime",
                "The date and time when the task is due. This should be defined as a valid ISO 8601 string. e.g. `2021-12-16T18:28:48.6964197Z`",
            ),
            (
                "percentComplete",
                "Percentage of task completion. Number between 0 and 100.",
            ),
            (
                "assignedToUserIds",
                "Comma-separated IDs of the assignees that should be added to the task assignment. Specify either `assignedToUserIds` or `assignedToUserNames` but not both.",
            ),
            (
                "assignedToUserNames",
                "Comma-separated UPNs of the assignees that should be added to the task assignment. Specify either `assignedToUserIds` or `assignedToUserNames` but not both.",
            ),
            ("description", "Description of the task."),
            (
                "orderHint",
                "Hint used to order items of this type in a list view.",
            ),
            (
                "assigneePriority",
                "Hint used to order items of this type in a list view.",
            ),
            (
                "appliedCategories",
                "Comma-separated categories that should be added to the task.",
            ),
            (
                "priority",
                "Priority of the task. Allowed values: `Urgent`, `Important`, `Medium`, `Low`. Or an integer between 0 and 10 (check remarks section for more info).",
            ),
        ],
        notes: &[
            "When you specify the value for `percentComplete`, consider the following:",
            "- when set to 0, the task is considered Not started",
            "- when set between 1 and 99, the task is considered In progress",
            "- when set to 100, the task is considered Completed",
            "When you specify an integer value for `priority`, consider the following:",
            "- values 0 and 1 are interpreted as Urgent",
            "- values 2, 3 and 4 are interpreted as Important",
            "- values 5, 6 and 7 are interpreted as Medium",
            "- values 8, 9 and 10 are interpreted as Low",
            "You can add up to 6 categories to the task. An example to add category1 and category3 would be `category1,category3`.",
            "When using `description` with a multiple lines value, use the new line character of the shell you are using to indicate line breaks. For PowerShell this is `` `n ``. For Zsh or Bash use `\\n` with a `$` in front. E.g. `$\"Line 1\\nLine 2\"`.",
            "When using `rosterId`, the command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Tasks.ReadWrite, GroupMember.Read.All, User.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Tasks.ReadWrite.All, GroupMember.Read.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Updates a Microsoft Planner task name of the specified task by ID.",
                "m365 planner task set --id \"Z-RLQGfppU6H3663DBzfs5gAMD3o\" --title \"My Planner Task\"",
            ),
            (
                "Moves a Microsoft Planner task with the specified ID to the bucket by name. Based on the plan with the specified rosterId.",
                "m365 planner task set --id \"Z-RLQGfppU6H3663DBzfs5gAMD3o\" --bucketName \"My Planner Bucket\" --rosterId \"8bc07d47-c06f-41e1-8f00-1c113c8f6067\"",
            ),
            (
                "Moves a Microsoft Planner task with the specified ID to the bucket by name. Based on the plan with the specified title owned by the specified group by name.",
                "m365 planner task set --id \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --bucketName \"My Planner Bucket\" --planTitle \"My Planner Plan\" --ownerGroupName \"My Planner Group\"",
            ),
            (
                "Marks a Microsoft Planner task with the specified ID as 50% complete and assigned to categories 1 and 3.",
                "m365 planner task set --id \"2Vf8JHgsBUiIf-nuvBtv-ZgAAYw2\" --percentComplete 50 --appliedCategories \"category1,category3\"",
            ),
        ],
        links: &[
            (
                "https://docs.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0",
                "https://docs.microsoft.com/graph/api/resources/planner-order-hint-format?view=graph-rest-1.0",
            ),
            (
                "https://docs.microsoft.com/graph/api/resources/plannerappliedcategories?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/resources/plannerappliedcategories?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "planner tenant settings list",
        summary: "Lists the Microsoft Planner configuration of the tenant",
        options: &[],
        notes: &[
            "After executing the command `planner tenant settings set`, it can take some time for all changes to propagate across the tenant. Because of this, executing this command right away can return some unexpected results.",
            "To use this command you must be a global administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "email")],
        examples: &[(
            "Lists the Microsoft Planner settings of the tenant.",
            "m365 planner tenant settings list",
        )],
        links: &[],
    },
    Help {
        name: "planner tenant settings set",
        summary: "Sets Microsoft Planner configuration of the tenant",
        options: &[
            (
                "isPlannerAllowed",
                "Configure whether Planner should be enabled on the tenant.",
            ),
            (
                "allowCalendarSharing",
                "Configure whether Outlook calendar sync is enabled.",
            ),
            (
                "allowTenantMoveWithDataLoss",
                "Configure whether a tenant move into a new region is authorized.",
            ),
            (
                "allowTenantMoveWithDataMigration",
                "Configure whether a tenant move with data migration is authorized.",
            ),
            (
                "allowRosterCreation",
                "Configure whether Planner roster creation is allowed.",
            ),
            (
                "allowPlannerMobilePushNotifications",
                "Configure whether push notifications are enabled in the mobile app.",
            ),
        ],
        notes: &[
            "To use this command you must be a global administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "email")],
        examples: &[
            (
                "Disable Microsoft Planner in the tenant.",
                "m365 planner tenant settings set --isPlannerAllowed false",
            ),
            (
                "Disable Outlook calendar sync and mobile push notifications.",
                "m365 planner tenant settings set --allowCalendarSharing false --allowPlannerMobilePushNotifications false",
            ),
            (
                "Enable Microsoft Planner but disallow roster plans to be created.",
                "m365 planner tenant settings set --isPlannerAllowed true --allowRosterCreation false",
            ),
        ],
        links: &[],
    },
];

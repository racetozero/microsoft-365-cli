//! Help for the `m365 flow ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "flow disable",
        summary: "Disables specified Power Automate flow",
        options: &[
            ("name", "The name of the Power Automate flow to disable."),
            (
                "environmentName",
                "The name of the environment for which to disable Power Automate flow.",
            ),
            ("asAdmin", "Set, to disable the flow as admin."),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "By default, the command will try to disable Power Automate flows you own. If you want to disable Power Automate flows owned by another user, use the `asAdmin` flag.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Power Automate flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error. If you try to disable a non-existing flow as admin, you will get the `Could not find flow 'xyz'.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Disables Power Automate flow owned by the currently signed-in user",
                "m365 flow disable --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Disables Power Automate flow owned by another user",
                "m365 flow disable --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow enable",
        summary: "Enables specified Power Automate flow",
        options: &[
            ("name", "The name of the Power Automate flow to enable"),
            (
                "environmentName",
                "The name of the environment for which to enable Power Automate",
            ),
            ("asAdmin", "Set, to enable the Power Automate flow as admin"),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "By default, the command will try to enable Power Automate flows you own. If you want to enable flows owned by another user, use the `asAdmin` flag.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Power Automate flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error. If you try to enable a non-existing flow as admin, you will get the `Could not find flow 'xyz'.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Enables Power Automate flow owned by the currently signed-in user",
                "m365 flow enable --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Enables Power Automate flow owned by another user",
                "m365 flow enable --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow environment get",
        summary: "Gets information about the specified Microsoft Flow environment",
        options: &[
            (
                "name",
                "The name of the environment to get information about. When not specified, the default environment is retrieved. Specify either `name` or `default`, but not both.",
            ),
            (
                "default",
                "Indicates that the default environment should be retrieved. Specify either `name` or `default`, but not both.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about a Microsoft Flow environment specified by name.",
                "m365 flow environment get --name Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Get information about the default Microsoft Flow environment.",
                "m365 flow environment get --default",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow environment list",
        summary: "Lists Microsoft Flow environments in the current tenant",
        options: &[],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[(
            "List Microsoft Flow environments in the current tenant.",
            "m365 flow environment list",
        )],
        links: &[],
    },
    Help {
        name: "flow export",
        summary: "Exports the specified Power Automate flow",
        options: &[
            (
                "name",
                "The name (GUID, not the display name) of the Power Automate flow to export",
            ),
            (
                "environmentName",
                "The name of the environment for which to export the flow",
            ),
            (
                "packageDisplayName",
                "The display name to use in the exported package",
            ),
            (
                "packageDescription",
                "The description to use in the exported package",
            ),
            (
                "packageCreatedBy",
                "The name of the person to be used as the creator of the exported package",
            ),
            (
                "packageSourceEnvironment",
                "The name of the source environment from which the exported package was taken",
            ),
            ("format", "Export format type. `json,zip`. Default `zip`"),
            ("path", "The path to save the exported package to"),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Power Automate flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Export the specified Power Automate flow as a ZIP file",
                "m365 flow export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Export the specified Power Automate flow as a JSON file",
                "m365 flow export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --format json",
            ),
            (
                "Export the specified Power Automate flow as a ZIP file, specifying a Display Name of 'My flow name' to be embedded into the package",
                "m365 flow export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --packageDisplayName 'My flow name'",
            ),
            (
                "Export the specified Power Automate flow as a ZIP file with the filename 'MyFlow.zip' saved to the current directory",
                "m365 flow export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --path './MyFlow.zip'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow get",
        summary: "Gets information about the specified Power Automate flow",
        options: &[
            (
                "name",
                "The name of the Power Automate flow to get information about",
            ),
            (
                "environmentName",
                "The name of the environment for which to retrieve available flows",
            ),
            ("asAdmin", "Set, to retrieve the Flow as admin"),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "By default, the command will try to retrieve Power Automate flows you own. If you want to retrieve a flow owned by another user, use the `asAdmin` flag.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Power Automate flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error. If you try to retrieve a non-existing flow as admin, you will get the `Could not find flow 'xyz'.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the specified Power Automate flow owned by the currently signed-in user",
                "m365 flow get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Get information about the specified Power Automate flow owned by another user",
                "m365 flow get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow list",
        summary: "Lists Power Automate flows in the given environment",
        options: &[
            (
                "environmentName",
                "The name of the environment for which to retrieve available flows.",
            ),
            (
                "sharingStatus",
                "List a specific type of flows. Valid values: `ownedByMe`, `personal`, `sharedWithMe`, `all`. Defaults to `ownedByMe`. Don't specify this option when using asAdmin.",
            ),
            ("withSolutions", "Include flows from solutions."),
            (
                "asAdmin",
                "Set, to list all Flows as admin. Otherwise will return only your own flows.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "When you specify a value for `sharingStatus`, consider the following:",
            "- `ownedByMe` returns all flows created by you regardless of whether they are shared or not.",
            "- `personal` returns all flows created by you which are not shared.",
            "- `sharedWithMe` returns flows that are shared with you, or created by you and shared with someone.",
            "- `all` combines `personal` and `sharedWithMe`.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all your flows in the given environment.",
                "m365 flow list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "List all flows in the given environment.",
                "m365 flow list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --asAdmin",
            ),
            (
                "List all flows that are shared with the current user.",
                "m365 flow list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --sharingStatus sharedWithMe",
            ),
            (
                "List all personal flows and include flows from solutions.",
                "m365 flow list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --sharingStatus personal --withSolutions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow owner ensure",
        summary: "Assigns/updates permissions to a Power Automate flow",
        options: &[
            ("environmentName", "The name of the environment."),
            ("flowName", "The name of the Power Automate flow."),
            ("userId", "The ID of the user."),
            ("userName", "User principal name of the user."),
            ("groupId", "The ID of the group."),
            ("groupName", "The name of the group."),
            (
                "roleName",
                "Access level for the user on the flow. Valid options are `CanView` or `CanEdit`.",
            ),
            ("asAdmin", "Run the command as admin."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Assign owner permissions to a specific Power Automate flow for a user with ID",
                "m365 flow owner ensure --userId 5c241023-2ba5-4ea8-a516-a2481a3e6c51 --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --roleName CanEdit",
            ),
            (
                "Assign owner permissions to a specific Power Automate flow for a user with UPN",
                "m365 flow owner ensure --userName \"john.doe@contoso.com\" --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --roleName CanEdit",
            ),
            (
                "Assign owner permissions to a specific Power Automate flow for a group",
                "m365 flow owner ensure --groupId 8d4d9f32-1ab0-4f81-9054-efbb1759e8e6 --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --roleName CanEdit",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow owner list",
        summary: "Lists all owners of a Power Automate flow",
        options: &[
            ("environmentName", "The name of the environment."),
            ("flowName", "The name of the Power Automate flow."),
            ("asAdmin", "Run the command as admin."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Gets the owners by the name of the Power Automate flow within a specified environment.",
                "m365 flow owner list --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72",
            ),
            (
                "Gets the owners by the name of the Power Automate flow within a specified environment with admin privileges.",
                "m365 flow owner list --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow owner remove",
        summary: "Removes owner permissions to a Power Automate flow",
        options: &[
            ("environmentName", "The name of the environment."),
            ("flowName", "The name of the Power Automate flow."),
            ("userId", "The ID of the user."),
            ("userName", "User principal name of the user."),
            ("groupId", "The ID of the group."),
            ("groupName", "The name of the group."),
            ("asAdmin", "Run the command as admin."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove owner permissions from a specific Power Automate flow for a user by ID without prompting for confirmation",
                "m365 flow owner remove --userId 5c241023-2ba5-4ea8-a516-a2481a3e6c51 --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --force",
            ),
            (
                "Remove owner permissions from a specific Power Automate flow for a user by UPN as admin",
                "m365 flow owner remove --userName john.doe@contoso.com --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --asAdmin",
            ),
            (
                "Remove owner permissions from a specific Power Automate flow for a group by ID",
                "m365 flow owner remove --groupId 5c241023-2ba5-4ea8-a516-a2481a3e6c51 --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72",
            ),
            (
                "Remove owner permissions from a specific Power Automate flow for a group by name as admin",
                "m365 flow owner remove --groupName \"Test group\" --environmentName Default-c5a5d746-3520-453f-8a69-780f8e44917e --flowName 72f2be4a-78c1-4220-a048-dbf557296a72 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow recyclebinitem list",
        summary: "Lists all soft-deleted Power Automate flows within an environment",
        options: &[("environmentName", "The name of the environment.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
            "To use this command, you must be a Global or Power Platform administrator.",
            "A Power Automate flow is soft-deleted when:",
            "- It's a non-solution flow.",
            "- It's been deleted less than 21 days ago.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
        ],
        permissions: &[],
        examples: &[(
            "List all soft-deleted flows within a specific environment",
            "m365 flow recyclebinitem list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
        )],
        links: &[],
    },
    Help {
        name: "flow recyclebinitem restore",
        summary: "Restores a soft-deleted Power Automate flow",
        options: &[
            (
                "environmentName",
                "The name of the environment where the flow is located.",
            ),
            ("flowName", "The name of the Power Automate flow."),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
            "To use this command, you must be a Global or Power Platform administrator.",
            "When a Power Automate flow is restored, it will be automatically disabled. To make it operational again, you must enable it.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
        ],
        permissions: &[],
        examples: &[(
            "Restores a soft-deleted flow within a specific environment",
            "m365 flow recyclebinitem restore --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a",
        )],
        links: &[],
    },
    Help {
        name: "flow remove",
        summary: "Removes the specified Power Automate flow",
        options: &[
            ("name", "The name of the Power Automate flow to remove"),
            (
                "environmentName",
                "The name of the environment to which the flow belongs",
            ),
            ("asAdmin", "Set, to remove the flow as admin"),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "By default, the command will try to remove a Power Automate flow you own. If you want to remove a Flow owned by another user, use the `asAdmin` flag.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Power Automate flow with the name you specified doesn't exist, you will get the `Error: Resource 'abc' does not exist in environment 'xyz'` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the specified Power Automate flow owned by the currently signed-in user",
                "m365 flow remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Removes the specified Power Automate flow owned by the currently signed-in user without confirmation",
                "m365 flow remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --force",
            ),
            (
                "Removes the specified Power Automate flow owned by another user",
                "m365 flow remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --asAdmin",
            ),
            (
                "Removes the specified Power Automate flow owned by another user without confirmation",
                "m365 flow remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --asAdmin --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow run cancel",
        summary: "Cancels a specific run for the specified flow",
        options: &[
            ("name", "The name of the run to cancel"),
            ("flowName", "The name of the flow to cancel the run for"),
            (
                "environmentName",
                "The name of the environment where the flow is located",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error.",
            "If the run with the name you specified doesn't exist, you will get the `The workflow 'xyz' run 'abc' could not be found.` error.",
        ],
        permissions: &[],
        examples: &[(
            "Cancel the given run of the specified flow",
            "m365 flow run cancel --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62",
        )],
        links: &[],
    },
    Help {
        name: "flow run get",
        summary: "Gets information about a specific run of the specified Power Automate flow",
        options: &[
            ("name", "The name of the run to get information about."),
            (
                "flowName",
                "The name of the Power Automate flow for which to retrieve information.",
            ),
            (
                "environmentName",
                "The name of the environment where the flow is located.",
            ),
            (
                "withTrigger",
                "If specified, include information about the trigger details.",
            ),
            (
                "withActions",
                "If specified, include information about all actions when no action names are specified, or provide a specified list of actions separated by commas to include only the relevant action details.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Microsoft Flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error.",
            "If the run with the name you specified doesn't exist, you will get the `The provided workflow run name is not valid.` error.",
            "If the option `withTrigger` is specified, but the trigger does not contain an outputsLink such as for example with a `Recurrence` trigger, this option will be ignored.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the given run of the specified Power Automate flow.",
                "m365 flow run get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62",
            ),
            (
                "Get information about the given run of the specified Power Automate flow including trigger information.",
                "m365 flow run get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62 --withTrigger",
            ),
            (
                "Get information about the given run of the specified Power Automate flow including details about all associated actions.",
                "m365 flow run get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62 --withActions",
            ),
            (
                "Get information about the given run of the specified Power Automate flow including exclusively the details of 'Action_Internal_Name1' and 'Action_Internal_Name2' actions.",
                "m365 flow run get --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62 --withActions \"Action_Internal_Name1,Action_Internal_Name2\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow run list",
        summary: "Lists runs of the specified Microsoft Flow",
        options: &[
            (
                "flowName",
                "The name of the Microsoft Flow to retrieve the runs for.",
            ),
            (
                "environmentName",
                "The name of the environment to which the flow belongs.",
            ),
            (
                "status",
                "Filter the results to only flow runs with a given status: `Succeeded`, `Running`, `Failed` or `Cancelled`. By default all flow runs are listed.",
            ),
            (
                "triggerStartTime",
                "Time indicating the inclusive start of a time range of flow runs to return. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z).",
            ),
            (
                "triggerEndTime",
                "Time indicating the exclusive end of a time range of flow runs to return. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z).",
            ),
            (
                "withTrigger",
                "If specified, include information about the trigger details for each run.",
            ),
            (
                "asAdmin",
                "Run the command as admin. Use this flag when targeting flows you don't own.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Microsoft Flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error.",
            "When using `--withTrigger`, extra requests will get fired for each retrieved flow run. This has an impact on the time the command takes to run.",
        ],
        permissions: &[],
        examples: &[
            (
                "List runs of the specified Microsoft Flow.",
                "m365 flow run list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a",
            ),
            (
                "List runs of the specified Microsoft Flow with a specific status",
                "m365 flow run list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --status Running",
            ),
            (
                "List runs of the specified Microsoft Flow between a specific time range.",
                "m365 flow run list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --triggerStartTime 2023-01-21T18:19:00Z --triggerEndTime 2023-01-22T00:00:00Z",
            ),
            (
                "List runs of the specified Microsoft Flow owned by another user.",
                "m365 flow run list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --asAdmin",
            ),
            (
                "List runs of the specified Microsoft Flow with the trigger information.",
                "m365 flow run list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --withTrigger",
            ),
        ],
        links: &[],
    },
    Help {
        name: "flow run resubmit",
        summary: "Resubmits a specific flow run for the specified Microsoft Flow",
        options: &[
            ("name", "The name of the run to resubmit"),
            ("flowName", "The name of the flow to resubmit the run for"),
            (
                "environmentName",
                "The name of the environment where the Flow is located",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "If the Microsoft Flow with the name you specified doesn't exist, you will get the `The caller with object id 'abc' does not have permission for connection 'xyz' under Api 'shared_logicflows'.` error.",
            "If the run with the name you specified doesn't exist, you will get the `The workflow 'xyz' run 'abc' could not be found.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Resubmits a specific flow run for the specified Microsoft Flow",
                "m365 flow run resubmit --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62",
            ),
            (
                "Resubmits a specific flow run for the specified Microsoft Flow without prompting for confirmation",
                "m365 flow run resubmit --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --flowName 5923cb07-ce1a-4a5c-ab81-257ce820109a --name 08586653536760200319026785874CU62 --force",
            ),
        ],
        links: &[],
    },
];

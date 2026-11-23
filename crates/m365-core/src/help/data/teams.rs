//! Help for the `m365 teams ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "teams app install",
        summary: "Installs a Microsoft Teams team app from the catalog in the specified team or for the specified user",
        options: &[
            (
                "id",
                "ID of the app to update. Specify either id or name, but not both",
            ),
            (
                "name",
                "The display name of the app to update. Specify either id or name, but not both",
            ),
            (
                "teamId",
                "The ID of the Microsoft Teams team to which to install the app",
            ),
            ("userId", "The ID of the user for who to install the app."),
            (
                "userName",
                "The UPN of the user for who to install the app.",
            ),
        ],
        notes: &[
            "The `id` has to be the ID of the app from the Microsoft Teams App Catalog. Do not use the ID from the manifest of the zip app package. Use the teams app list command to get this ID instead.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AppCatalog.Submit, TeamsAppInstallation.ReadWriteForTeam, TeamsAppInstallation.ReadWriteForUser, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AppCatalog.Read.All, TeamsAppInstallation.ReadWriteForTeam.All, TeamsAppInstallation.ReadWriteForUser.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Install an app from the catalog in a Microsoft Teams team",
                "m365 teams app install --id 4440558e-8c73-4597-abc7-3644a64c4bce --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Install a personal app for the user specified using their user name",
                "m365 teams app install --id 4440558e-8c73-4597-abc7-3644a64c4bce --userName steve@contoso.com",
            ),
            (
                "Install a personal app for the user specified using their ID",
                "m365 teams app install --id 4440558e-8c73-4597-abc7-3644a64c4bce --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Install an app with name Test app from the catalog in a Microsoft Teams team",
                "m365 teams app install --name \"Test app\" --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams app list",
        summary: "Lists apps from the Microsoft Teams app catalog",
        options: &[(
            "distributionMethod",
            "The distribution method. Allowed values `store`, `organization`, `sideloaded`.",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "AppCatalog.Submit"),
            ("Application", "Microsoft Graph", "AppCatalog.Read.All"),
        ],
        examples: &[
            (
                "List all apps from the Microsoft Teams app catalog",
                "m365 teams app list",
            ),
            (
                "List all apps from the Microsoft Teams app catalog according to a given distribution method",
                "m365 teams app list --distributionMethod 'store'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams app publish",
        summary: "Publishes Teams app to the organization's app catalog",
        options: &[(
            "filePath",
            "Absolute or relative path to the Teams manifest zip file to add to the app catalog.",
        )],
        notes: &[
            "To use this command you must be a Global administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "AppCatalog.ReadWrite.All")],
        examples: &[(
            "Add the teams-manifest.zip file to the organization's app catalog",
            "m365 teams app publish --filePath ./teams-manifest.zip",
        )],
        links: &[],
    },
    Help {
        name: "teams app remove",
        summary: "Removes a Teams app from the organization's app catalog",
        options: &[
            (
                "id",
                "ID of the Teams app to remove. Needs to be available in your organization\\'s app catalog.",
            ),
            (
                "name",
                "Name of the Teams app to remove. Needs to be available in your organization\\'s app catalog.",
            ),
            ("force", "Don't prompt for confirming removing the app."),
        ],
        notes: &["This command does not support application permissions."],
        permissions: &[("Delegated", "Microsoft Graph", "AppCatalog.ReadWrite.All")],
        examples: &[
            (
                "Remove the Teams app by ID from the organization's app catalog. Will prompt for confirmation before actually removing the app.",
                "m365 teams app remove --id 83cece1e-938d-44a1-8b86-918cf6151957",
            ),
            (
                "Remove the Teams app by name from the organization's app catalog. Don't prompt for confirmation.",
                "m365 teams app remove --name HelloWorld --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams app uninstall",
        summary: "Uninstalls an app from a Microsoft Team team",
        options: &[
            (
                "id",
                "The unique id of the app instance installed in the Microsoft Teams team",
            ),
            (
                "teamId",
                "The ID of the Microsoft Teams team from which to uninstall the app",
            ),
            (
                "force",
                "Don't prompt for confirmation when uninstalling the app",
            ),
        ],
        notes: &[
            "The `id` has to be the id the app instance installed in the Microsoft Teams team. Do not use the ID from the manifest of the zip app package or the id from the Microsoft Teams App Catalog.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "TeamsAppInstallation.ReadWriteForTeam",
            ),
            (
                "Application",
                "Microsoft Graph",
                "TeamsAppInstallation.ReadWriteForTeam.All",
            ),
        ],
        examples: &[(
            "Uninstall an app from a Microsoft Teams team",
            "m365 teams app uninstall --id YzUyN2E0NzAtYTg4Mi00ODFjLTk4MWMtZWU2ZWZhYmE4NWM3IyM0ZDFlYTA0Ny1mMTk2LTQ1MGQtYjJlOS0wZDI4NTViYTA1YTY= --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
        )],
        links: &[],
    },
    Help {
        name: "teams app update",
        summary: "Updates Teams app in the organization's app catalog",
        options: &[
            (
                "id",
                "ID of the app to update. Specify either id or name, but not both",
            ),
            (
                "name",
                "The display name of the app to update. Specify either id or name, but not both",
            ),
            (
                "filePath",
                "Absolute or relative path to the Teams manifest zip file to update in the app catalog",
            ),
        ],
        notes: &[
            "You can only update a Teams app as a global administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "AppCatalog.ReadWrite.All")],
        examples: &[
            (
                "Update the Teams app with ID 83cece1e-938d-44a1-8b86-918cf6151957 from file teams-manifest.zip",
                "m365 teams app update --id 83cece1e-938d-44a1-8b86-918cf6151957 --filePath ./teams-manifest.zip",
            ),
            (
                "Update the Teams app with name Test app from file teams-manifest.zip",
                "m365 teams app update --name \"Test app\" --filePath ./teams-manifest.zip",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams cache remove",
        summary: "Removes the Microsoft Teams client cache",
        options: &[
            (
                "client",
                "Teams client to target. Possible values are: `new` or `classic`. Default value is `new`.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "This command will execute the following steps.",
            "- Stop the Microsoft Teams client. This will kill all the running `Teams.exe` tasks.",
            "- Clear the Microsoft Teams cached files. For Windows it will delete all files and folders in the %appdata%\\Microsoft\\Teams directory. For macOS it will delete all files and folders in the ~/Library/Application Support/Microsoft/Teams directory.",
            "- You won't lose any user data by clearing the cache.",
            "- Restarting Teams after you clear the cache might take longer than usual because the Teams cache files have to be rebuilt.",
            "If you run the command in the CLI Docker container, you will get the following error message:",
            "> Because you're running CLI for Microsoft 365 in a Docker container, we can't clear the cache on your host. Instead run this command on your host using 'npx ...'.",
            "The command works only on Windows and macOS. If you run it on a different operating system, you will get the `'abc' platform is unsupported for this command` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the Microsoft Teams client cache for the new client.",
                "m365 teams cache remove",
            ),
            (
                "Removes the Microsoft Teams client cache for the classic client.",
                "m365 teams cache remove --client classic",
            ),
        ],
        links: &[(
            "Clear the Teams client cache guidance",
            "https://docs.microsoft.com/microsoftteams/troubleshoot/teams-administration/clear-teams-cache",
        )],
    },
    Help {
        name: "teams callrecord get",
        summary: "Gets a specific Teams call",
        options: &[("id", "ID of the call record.")],
        notes: &[
            "It is only possible to retrieve call records from the past 30 days.",
            "This command does not support delegated permissions.",
        ],
        permissions: &[("Application", "Microsoft Graph", "CallRecords.Read.All")],
        examples: &[(
            "Get a specific call record",
            "m365 teams callrecord get --id e523d2ed-2966-4b6b-925b-754a88034cc5",
        )],
        links: &[],
    },
    Help {
        name: "teams callrecord list",
        summary: "Lists all Teams calls within the tenant",
        options: &[
            (
                "userId",
                "Filters call records to only include those where the user participated. Use the Entra user ID of the user.",
            ),
            (
                "userName",
                "Filters call records to only include those where the user participated. Use the UPN of the user.",
            ),
            (
                "startDateTime",
                "Only show calls that started after this time. Defaults to 30 days ago.",
            ),
            (
                "endDateTime",
                "Only show calls that started before this time (exclusive). Defaults to now.",
            ),
        ],
        notes: &[
            "It is only possible to retrieve call records from the past 30 days.",
            "This command does not support delegated permissions.",
        ],
        permissions: &[("Application", "Microsoft Graph", "CallRecords.Read.All")],
        examples: &[
            (
                "Get all call records from the past 30 days.",
                "m365 teams callrecord list",
            ),
            (
                "Get all call records where a specific user participated in.",
                "m365 teams callrecord list --userName \"john.doe@contoso.com\"",
            ),
            (
                "Get all call records that started within a specific timeframe.",
                "m365 teams callrecord list --startDateTime \"2025-05-1T00:00:00Z\" --endDateTime \"2025-05-14T00:00:00Z\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel add",
        summary: "Adds a channel to the specified Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team to add the channel to."),
            (
                "teamName",
                "The display name of the team to add the channel to.",
            ),
            ("name", "The name of the channel to add"),
            ("description", "The description of the channel to add"),
            (
                "type",
                "Type of channel to create: `standard`, `private`, `shared`. Default `standard`.",
            ),
            (
                "owner",
                "User with this ID or UPN will be added as owner of the channel. This option is required when type is `private` or `shared`.",
            ),
        ],
        notes: &["You can only add a channel to the Microsoft Teams team you are a member of."],
        permissions: &[],
        examples: &[
            (
                "Add channel to the specified Microsoft Teams team with id 6703ac8a-c49b-4fd4-8223-28f0ac3a6402",
                "m365 teams channel add --teamId 6703ac8a-c49b-4fd4-8223-28f0ac3a6402 --name climicrosoft365 --description development",
            ),
            (
                "Add channel to the specified Microsoft Teams team with name Team Name",
                "m365 teams channel add --teamName \"Team Name\" --name climicrosoft365 --description development",
            ),
            (
                "Add private channel to the specified Microsoft Teams team with owner UPN",
                "m365 teams channel add --teamName \"Team Name\" --name climicrosoft365 --type private --owner john.doe@contoso.com",
            ),
            (
                "Add shared channel to the specified Microsoft Teams team with owner ID",
                "m365 teams channel add --teamId 6703ac8a-c49b-4fd4-8223-28f0ac3a6402 --name climicrosoft365 --type shared --owner cc693a7d-4833-4911-a89a-f0fe6e49bf69",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel get",
        summary: "Gets information about the specific Microsoft Teams team channel",
        options: &[
            (
                "teamId",
                "The ID of the team to which the channel belongs to.",
            ),
            (
                "teamName",
                "The display name of the team to which the channel belongs to.",
            ),
            (
                "id",
                "The ID of the channel for which to retrieve more information.",
            ),
            (
                "name",
                "The display name of the channel for which to retrieve more information.",
            ),
            (
                "primary",
                "Gets the default channel, General, of a team. If specified, id or name are not needed.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Get information about Microsoft Teams team channel with id 19:493665404ebd4a18adb8a980a31b4986@thread.skype",
                "m365 teams channel get --teamId 00000000-0000-0000-0000-000000000000 --id '19:493665404ebd4a18adb8a980a31b4986@thread.skype'",
            ),
            (
                "Get information about Microsoft Teams team channel with name Channel Name",
                "m365 teams channel get --teamName \"Team Name\" --name \"Channel Name\"",
            ),
            (
                "Get information about Microsoft Teams team primary channel , i.e. General",
                "m365 teams channel get --teamName \"Team Name\" --primary",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel list",
        summary: "Lists channels in the specified Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team to list the channels of."),
            (
                "teamName",
                "The display name of the team to list the channels of.",
            ),
            (
                "type",
                "Filter the results to only channels of a given type: `standard`, `private`, `shared`. By default all channels are listed.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all channels in a specified Microsoft Teams team with id 00000000-0000-0000-0000-000000000000",
                "m365 teams channel list --teamId 00000000-0000-0000-0000-000000000000",
            ),
            (
                "List all channels in a specified Microsoft Teams team with name Team Name",
                "m365 teams channel list --teamName \"Team Name\"",
            ),
            (
                "List private channels in a specified Microsoft Teams team with id 00000000-0000-0000-0000-000000000000",
                "m365 teams channel list --teamId 00000000-0000-0000-0000-000000000000 --type private",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel member add",
        summary: "Adds a specified member in the specified Microsoft Teams private or shared team channel",
        options: &[
            ("teamId", "The ID of the team where the channel is located."),
            (
                "teamName",
                "The name of the team where the channel is located.",
            ),
            ("channelId", "The Id of the Microsoft Teams team channel."),
            (
                "channelName",
                "The display name of the Microsoft Teams team channel.",
            ),
            (
                "userIds",
                "The user's ID or principal name. You can also pass a comma-separated list of userIds.",
            ),
            (
                "userDisplayNames",
                "The display name of a user. You can also pass a comma-separated list of display names.",
            ),
            (
                "owner",
                "Assign the user the owner role. Defaults to member permissions.",
            ),
        ],
        notes: &[
            "At least one owner must be assigned to a private or shared channel.",
            "You can only add members and owners of a team to a private channel.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add members to a channel based on their id or user principal name",
                "m365 teams channel member add --teamId 47d6625d-a540-4b59-a4ab-19b787e40593 --channelId 19:586a8b9e36c4479bbbd378e439a96df2@thread.skype --userIds \"85a50aa1-e5b8-48ac-b8ce-8e338033c366,john.doe@contoso.com\"",
            ),
            (
                "Add owners to a channel based on their display names",
                "m365 teams channel member add --teamName \"Human Resources\" --channelName \"Private Channel\" --userDisplayNames \"Anne Matthews,John Doe\" --owner",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel member list",
        summary: "Lists members of the specified Microsoft Teams team channel",
        options: &[
            ("teamId", "The Id of the Microsoft Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            ("channelId", "The Id of the Microsoft Teams team channel."),
            (
                "channelName",
                "The display name of the Microsoft Teams team channel.",
            ),
            (
                "role",
                "Filter the results to only users with the given role: owner, member, guest",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List the members of a specified Microsoft Teams team with the specified id and channel with the specified id.",
                "m365 teams channel member list --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype",
            ),
            (
                "List the members of a specified Microsoft Teams team with the specified name and channel with the specified name.",
                "m365 teams channel member list --teamName \"Team Name\" --channelName \"Channel Name\"",
            ),
            (
                "List all owners of the specified Microsoft Teams team with the specified id and channel the specified id.",
                "m365 teams channel member list --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --role owner",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel member remove",
        summary: "Removes the specified member from the specified Microsoft Teams private or shared team channel",
        options: &[
            ("teamId", "The Id of the Microsoft Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            ("channelId", "The Id of the Microsoft Teams team channel."),
            (
                "channelName",
                "The display name of the Microsoft Teams team channel.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Specify either userName, userId or id but not multiple.",
            ),
            (
                "userId",
                "User's Microsoft Entra ID. Specify either userName, userId or id but not multiple.",
            ),
            (
                "id",
                "Channel member Id of a user. Specify either userName, userId or id but not multiple.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove the user johndoe@example.com from the Microsoft Teams team with id 00000000-0000-0000-0000-000000000000 and channel id 19:00000000000000000000000000000000@thread.skype",
                "m365 teams channel member remove --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --userName \"johndoe@example.com\"",
            ),
            (
                "Remove the user with id 00000000-0000-0000-0000-000000000000 from the Microsoft Teams team with name Team Name and channel with name Channel Name",
                "m365 teams channel member remove --teamName \"Team Name\" --channelName \"Channel Name\" --userId 00000000-0000-0000-0000-000000000000",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel member set",
        summary: "Updates the role of the specified member in the specified Microsoft Teams private or shared team channel",
        options: &[
            ("teamId", "The Id of the Microsoft Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            ("channelId", "The Id of the Microsoft Teams team channel."),
            (
                "channelName",
                "The display name of the Microsoft Teams team channel.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Specify either userName, userId or id but not multiple.",
            ),
            (
                "userId",
                "User's Microsoft Entra ID. Specify either userName, userId or id but not multiple.",
            ),
            (
                "id",
                "Channel member Id of a user. Specify either userName, userId or id but not multiple.",
            ),
            (
                "role",
                "The role to be assigned to the user: owner, member.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Updates the role of the user to owner in the Microsoft Teams team with id 00000000-0000-0000-0000-000000000000 and channel id 19:00000000000000000000000000000000@thread.skype",
                "m365 teams channel member set --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --userName \"johndoe@example.com\" --role owner",
            ),
            (
                "Updates the role of the user with id 00000000-0000-0000-0000-000000000000 to member in the Microsoft Teams team with name Team Name and channel with name Channel Name",
                "m365 teams channel member set --teamName \"Team Name\" --channelName \"Channel Name\" --userId 00000000-0000-0000-0000-000000000000 --role member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams channel remove",
        summary: "Removes the specified channel in the Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the team to which the channel to remove belongs.",
            ),
            (
                "teamName",
                "The display name of the team to which the channel to remove belongs.",
            ),
            ("id", "The ID of the channel to remove."),
            ("name", "The name of the channel to remove."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "When deleted, Microsoft Teams channels are moved to a recycle bin and can be restored within 30 days. After that time, they are permanently deleted.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove the specified Microsoft Teams channel by Id",
                "m365 teams channel remove --id 19:f3dcbb1674574677abcae89cb626f1e6@thread.skype --teamId d66b8110-fcad-49e8-8159-0d488ddb7656",
            ),
            (
                "Remove the specified Microsoft Teams channel by Id without confirmation",
                "m365 teams channel remove --id 19:f3dcbb1674574677abcae89cb626f1e6@thread.skype --teamId d66b8110-fcad-49e8-8159-0d488ddb7656 --force",
            ),
            (
                "Remove the specified Microsoft Teams channel by Name",
                "m365 teams channel remove --name 'name' --teamName \"Team Name\"",
            ),
            (
                "Remove the specified Microsoft Teams channel by Name without confirmation",
                "m365 teams channel remove --name 'name' --teamName \"Team Name\" --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/directory?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/directory?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "teams channel set",
        summary: "Updates properties of the specified channel in the given Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the team where the channel to update is located.",
            ),
            (
                "teamName",
                "The display name of the team where the channel to update is located.",
            ),
            ("id", "The ID of the channel to update."),
            ("name", "The name of the channel to update."),
            ("newName", "The new name of the channel."),
            ("description", "The description of the channel."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Set new description and display name for the specified channel in the given Microsoft Teams team",
                "m365 teams channel set --teamId \"00000000-0000-0000-0000-000000000000\" --name Reviews --newName Projects --description \"Channel for new projects\"",
            ),
            (
                "Set new display name for the specified channel in the given Microsoft Teams team",
                "m365 teams channel set --teamId \"00000000-0000-0000-0000-000000000000\" --name Reviews --newName Projects",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat get",
        summary: "Gets a Microsoft Teams chat conversation by id, participants or chat name.",
        options: &[
            ("id", "The ID of the chat conversation."),
            ("name", "The display name of the chat conversation."),
            (
                "participants",
                "A comma-separated list of one or more e-mail addresses.",
            ),
        ],
        notes: &[
            "The output will not include the chat conversation members or messages. It will just retrieve the conversation details. When using the `participants` option, the signed-in user will automatically be included as a participant. There's no need to add it to the list manually.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get a Microsoft Teams chat conversation by id.",
                "m365 teams chat get --id 19:2da4c29f6d7041eca70b638b43d45437@thread.v2",
            ),
            (
                "Get a Microsoft Teams one on one chat conversation, finding it by participant.",
                "m365 teams chat get --participants alexw@contoso.com",
            ),
            (
                "Get a Microsoft Teams group chat conversation, finding it by participants.",
                "m365 teams chat get --participants alexw@contoso.com,meganb@contoso.com",
            ),
            (
                "Get a Microsoft Teams chat conversation, finding it by display name.",
                "m365 teams chat get --name \"Just a conversation\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat list",
        summary: "Lists all Microsoft Teams chat conversations for the current or a specific user.",
        options: &[
            (
                "type",
                "The chat type to optionally filter chat conversations by type. The value can be `oneOnOne`, `group` or `meeting`.",
            ),
            (
                "userId",
                "ID of the user. Has to be specified when using application permissions. Specify either `userId` or `userName`, but not both.",
            ),
            (
                "userName",
                "UPN of the user. Has to be specified when using application permissions. Specify either `userId` or `userName`, but not both.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all the Microsoft Teams chat conversations of the current user.",
                "m365 teams chat list",
            ),
            (
                "List only the one on one Microsoft Teams chat conversations of a specific user retrieved by id.",
                "m365 teams chat list --userId e6296ed0-4b7d-4ace-aed4-f6b7371ce060 --type oneOnOne",
            ),
            (
                "List only the group Microsoft Teams chat conversations of a specific user retrieved by mail.",
                "m365 teams chat list --userName 'john@contoso.com' --type group",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat member add",
        summary: "Adds a member to a Microsoft Teams chat conversation.",
        options: &[
            ("chatId", "The ID of the chat conversation."),
            ("userId", "The ID of the user."),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com).",
            ),
            (
                "role",
                "The role for the user to add. Valid values are: `owner`, `guest`. Defaults to `owner`.",
            ),
            (
                "visibleHistoryStartDateTime",
                "Include chat history until a certain point in time. Specify either `visibleHistoryStartDateTime`, `withAllHistory` or neither.",
            ),
            (
                "withAllHistory",
                "Include all chat history. Specify either `visibleHistoryStartDateTime`, `withAllHistory` or neither.",
            ),
        ],
        notes: &[
            "When you specify a value for `role`, consider the following:",
            "- `owner` can only be used for internal users.",
            "- `guest` can only be used for guest users.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add a member by ID to a Teams chat without sharing any chat history.",
                "m365 teams chat member add --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --userId bd94e214-7852-48b0-a326-5a34b2a02183",
            ),
            (
                "Add a member by UPN to a Teams chat and share chat history until a specific point in time.",
                "m365 teams chat member add --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --userName john.doe@contoso.com --visibleHistoryStartDateTime 2023-05-03T12:00:00Z",
            ),
            (
                "Add a guest member by UPN to a Teams chat and share all chat history.",
                "m365 teams chat member add --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --userName nelson.wilke@fabrikam.com --role guest --withAllHistory",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat member list",
        summary: "Lists all members from a Microsoft Teams chat conversation.",
        options: &[("chatId", "The ID of the chat conversation.")],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List the members from a Microsoft Teams chat conversation.",
            "m365 teams chat member list --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces",
        )],
        links: &[],
    },
    Help {
        name: "teams chat member remove",
        summary: "Removes a member from a Microsoft Teams chat conversation",
        options: &[
            ("chatId", "The ID of the chat conversation."),
            ("id", "The ID of the chat member."),
            ("userId", "The ID of the Microsoft Entra user."),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com).",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove a member from a Teams chat by its member ID without prompting for confirmation.",
                "m365 teams chat member remove --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --id MCMjMjQzMmI1N2ItMGFiZC00M2RiLWFhN2ItMTZlYWRkMTE1ZDM0IyMxOTpiZDlkYTQ2MzIzYWY0MjUzOTZkMGZhNjcyMDAyODk4NEB0aHJlYWQudjIjIzQ4YmY5ZDUyLWRjYTctNGE1Zi04Mzk4LTM3Yjk1Y2M3YmQ4Mw== --force",
            ),
            (
                "Remove a member from a Teams chat by its Microsoft Entra ID.",
                "m365 teams chat member remove --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --userId bd94e214-7852-48b0-a326-5a34b2a02183",
            ),
            (
                "Remove a member from a Teams chat by its Microsoft Entra UPN.",
                "m365 teams chat member remove --chatId 19:8b081ef6-4792-4def-b2c9-c363a1bf41d5_5031bb31-22c0-4f6f-9f73-91d34ab2b32d@unq.gbl.spaces --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat message list",
        summary: "Lists all messages from a Microsoft Teams chat conversation.",
        options: &[
            ("chatId", "The ID of the chat conversation."),
            (
                "createdEndDateTime",
                "Time indicating the exclusive end of a time range when the message was created.",
            ),
            (
                "endDateTime",
                "(deprecated. Use `createdEndDateTime` instead) Time indicating the exclusive end of a time range when the message was created.",
            ),
            (
                "modifiedStartDateTime",
                "Time indicating the inclusive start of a time range when the message was last modified. Cannot be combined with `createdEndDateTime`.",
            ),
            (
                "modifiedEndDateTime",
                "Time indicating the exclusive end of a time range when the message was last modified. Cannot be combined with `createdEndDateTime`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ChatMessage.Read"),
            ("Application", "Microsoft Graph", "ChatMessage.Read.All"),
        ],
        examples: &[
            (
                "List the messages from a Microsoft Teams chat conversation",
                "m365 teams chat message list --chatId 19:2da4c29f6d7041eca70b638b43d45437@thread.v2",
            ),
            (
                "List messages from a Microsoft Teams chat conversation created before November 1, 2022",
                "m365 teams chat message list --chatId 19:2da4c29f6d7041eca70b638b43d45437@thread.v2 --createdEndDateTime 2022-11-01T00:00:00Z",
            ),
            (
                "List messages from a Microsoft Teams chat conversation modified after October 1, 2025",
                "m365 teams chat message list --chatId 19:2da4c29f6d7041eca70b638b43d45437@thread.v2 --modifiedStartDateTime 2025-10-01T00:00:00Z",
            ),
            (
                "List messages from a Microsoft Teams chat conversation modified between October 1, 2025 and November 1, 2025",
                "m365 teams chat message list --chatId 19:2da4c29f6d7041eca70b638b43d45437@thread.v2 --modifiedStartDateTime 2025-10-01T00:00:00Z --modifiedEndDateTime 2025-11-01T00:00:00Z",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams chat message send",
        summary: "Sends a chat message to a Microsoft Teams chat conversation.",
        options: &[
            ("chatId", "The ID of the chat conversation."),
            ("chatName", "The display name of the chat conversation."),
            (
                "userEmails",
                "A comma-separated list of one or more e-mail addresses.",
            ),
            ("message", "The message to send"),
            (
                "contentType",
                "The content type of the message. Allowed values are `text` and `html`. Default is `text`.",
            ),
        ],
        notes: &[
            "A new chat conversation will be created if no existing conversation with the participants specified with emails is found.",
            "This command does not support application permissions.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "Chat.Read, ChatMessage.Send",
        )],
        examples: &[
            (
                "Send a message to a Microsoft Teams chat conversation by id",
                "m365 teams chat message send --chatId 19:2da4c29f6d7041eca70b638b43d45437@thread.v2 --message \"<b>Welcome</b> to Teams\" --contentType html",
            ),
            (
                "Send a message to a single person",
                "m365 teams chat message send --userEmails alexw@contoso.com --message \"Welcome to Teams\"",
            ),
            (
                "Send a message to a group of people",
                "m365 teams chat message send --userEmails alexw@contoso.com,meganb@contoso.com --message \"Welcome to Teams\"",
            ),
            (
                "Send a message to a chat conversation finding it by display name",
                "m365 teams chat message send --chatName \"Just a conversation\" --message \"Welcome to Teams\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams funsettings list",
        summary: "Lists fun settings for the specified Microsoft Teams team",
        options: &[(
            "teamId",
            "The ID of the team for which to list fun settings",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List fun settings of a Microsoft Teams team",
            "m365 teams funsettings list --teamId 83cece1e-938d-44a1-8b86-918cf6151957",
        )],
        links: &[],
    },
    Help {
        name: "teams funsettings set",
        summary: "Updates fun settings of a Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the Teams team for which to update settings",
            ),
            (
                "allowGiphy",
                "Set to `true` to allow giphy and to `false` to disable it",
            ),
            (
                "giphyContentRating",
                "Settings to set content rating for giphy. Allowed values `Strict,Moderate`",
            ),
            (
                "allowStickersAndMemes",
                "Set to `true` to allow stickers and memes and to `false` to disable them",
            ),
            (
                "allowCustomMemes",
                "Set to `true` to allow custom memes and to `false` to disable them",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Allow giphy usage within a given Microsoft Teams team, setting the content rating for giphy to Moderate",
                "m365 teams funsettings set --teamId 83cece1e-938d-44a1-8b86-918cf6151957 --allowGiphy true --giphyContentRating Moderate",
            ),
            (
                "Disable usage of giphy within a given Microsoft Teams team",
                "m365 teams funsettings set --teamId 83cece1e-938d-44a1-8b86-918cf6151957 --allowGiphy false",
            ),
            (
                "Allow usage of stickers and memes within a given Microsoft Teams team",
                "m365 teams funsettings set --teamId 83cece1e-938d-44a1-8b86-918cf6151957 --allowStickersAndMemes true",
            ),
            (
                "Disable usage custom memes within a given Microsoft Teams team",
                "m365 teams funsettings set --teamId 83cece1e-938d-44a1-8b86-918cf6151957 --allowCustomMemes false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams guestsettings list",
        summary: "Lists guest settings for a Microsoft Teams team",
        options: &[(
            "teamId",
            "The ID of the team for which to get the guest settings",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get guest settings for a Microsoft Teams team",
            "m365 teams guestsettings list --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
        )],
        links: &[],
    },
    Help {
        name: "teams guestsettings set",
        summary: "Updates guest settings of a Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the Teams team for which to update settings",
            ),
            (
                "allowCreateUpdateChannels",
                "Set to `true` to allow guests to create and update channels and to `false` to disallow it",
            ),
            (
                "allowDeleteChannels",
                "Set to `true` to allow guests to create and update channels and to `false` to disallow it",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Allow guests to create and edit channels",
                "m365 teams guestsettings set --teamId '00000000-0000-0000-0000-000000000000' --allowCreateUpdateChannels true",
            ),
            (
                "Disallow guests to delete channels",
                "m365 teams guestsettings set --teamId '00000000-0000-0000-0000-000000000000' --allowDeleteChannels false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting add",
        summary: "Creates a new online meeting",
        options: &[
            (
                "startTime",
                "The start time of the meeting. If not specified, the start time will be set to the current time.",
            ),
            (
                "endTime",
                "The end time of the meeting. If not specified, the end time will be set to one hour after the start time.",
            ),
            ("subject", "The subject of the meeting."),
            (
                "participantUserNames",
                "A comma-separated list of participant UPNs.",
            ),
            ("organizerEmail", "The organizer's email address."),
            (
                "recordAutomatically",
                "When using this flag, the meeting will be recorded automatically.",
            ),
        ],
        notes: &[
            "This command creates a standalone meeting that is not associated with any event on the user's calendar; therefore, meetings created via this command will not show on the user's calendar.",
            "To create an online meeting for a specific organizer, use the `organizerEmail` option along with app-only permissions. The application should have the OnlineMeetings.ReadWrite.All permissions, and a special policy should be assigned to the user specified in the `organizerEmail` option. You can find more information on how to assign this policy to a user here (https://learn.microsoft.com/en-us/graph/cloud-communication-online-meeting-application-access-policy).",
        ],
        permissions: &[],
        examples: &[
            (
                "Create a new online meeting for the currently logged-in user, starting immediately and ending after one hour.",
                "m365 teams meeting add",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with a specified start date and a duration of one hour.",
                "m365 teams meeting add --startTime \"2025-09-21T13:30:00Z\"",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with specified end date the current date as the start date.",
                "m365 teams meeting add --endTime \"2025-09-21T23:55:00Z\"",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with specified start and end dates.",
                "m365 teams meeting add --startTime \"2025-09-21T13:30:00Z\" --endTime \"2025-09-21T23:55:00Z\"",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with a specified subject.",
                "m365 teams meeting add --startTime \"2025-09-21T13:30:00Z\" --endTime \"2025-09-21T23:55:00Z\" --subject \"Test Subject\"",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with a specified subject and a list of participantUserNames.",
                "m365 teams meeting add --subject \"Test Subject\" --participantUserNames \"john.doe@contoso.com,olga.manager@contoso.com\"",
            ),
            (
                "Create a new online meeting for the currently logged-in user, with a specified subject, a list of participantUserNames, and automatic meeting recording.",
                "m365 teams meeting add --subject \"Test Subject\" --participantUserNames \"john.doe@contoso.com,olga.manager@contoso.com\" --recordAutomatically",
            ),
            (
                "Create a new online meeting for a selected organizer, with a specified subject, a list of participantUserNames, and automatic meeting recording. This option is available for app-only permissions.",
                "m365 teams meeting add --organizerEmail \"john.doe@contoso.com\" --subject \"Test Subject\" --participantUserNames \"olga.manager@contoso.com\" --recordAutomatically",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting attendancereport get",
        summary: "Gets attendance report for a given meeting",
        options: &[
            (
                "userId",
                "The id of the user, omit to get attendance report for the current signed in user.",
            ),
            (
                "userName",
                "The name of the user, omit to get attendance report for the current signed in user.",
            ),
            (
                "email",
                "The email of the user, omit to get attendance report for the current signed in user.",
            ),
            ("meetingId", "The Id of the meeting."),
            ("id", "The Id of the attendance report."),
        ],
        notes: &[
            "To run this command with application permissions, tenant administrators must create an application access policy and grant it to a user. This authorizes the app configured in the policy to fetch online meetings and/or online meeting artifacts on behalf of that user. For more details, click here (https://learn.microsoft.com/graph/cloud-communication-online-meeting-application-access-policy).",
            "This method doesn't support channel meetings.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the specified attendance report made for the current signed in user and Microsoft Teams meeting with given id.",
                "m365 teams meeting attendancereport get --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ --id a8634e64-3147-4a56-9b19-cc822e9c7972",
            ),
            (
                "Gets the specified attendance report made for the garthf@contoso.com and Microsoft Teams meeting with given id.",
                "m365 teams meeting attendancereport list --userName garthf@contoso.com --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ --id a8634e64-3147-4a56-9b19-cc822e9c7972",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting attendancereport list",
        summary: "Lists all attendance reports for a given meeting",
        options: &[
            (
                "userId",
                "The id of the user, omit to list attendance reports for current signed in user. Use either `id`, `userName` or `email`, but not multiple.",
            ),
            (
                "userName",
                "The name of the user, omit to list attendance reports for current signed in user. Use either `id`, `userName` or `email`, but not multiple.",
            ),
            (
                "email",
                "The email of the user, omit to list attendance reports for current signed in user. Use either `id`, `userName` or `email`, but not multiple.",
            ),
            ("meetingId", "The Id of the meeting."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Lists all attendance reports made for the current signed in user and Microsoft Teams meeting with given id",
                "m365 teams meeting attendancereport list --meetingId MSo4NmY0MWZhMi1kYjUxLTQyMTUtYjE3Zi1lYzY3NGQ2MjQzNjYqMCoqMTk6bWVldGluZ19OamMwWW1Zd05UVXROMlJoTlMwME0yRmhMV0V6T1RndE9HVmxNbVl4TkdVd05tUTNAdGhyZWFkLnYy",
            ),
            (
                "Lists all attendance reports made for the garthf@contoso.com and Microsoft Teams meeting with given id",
                "m365 teams meeting attendancereport list --userName garthf@contoso.com --meetingId MSo4NmY0MWZhMi1kYjUxLTQyMTUtYjE3Zi1lYzY3NGQ2MjQzNjYqMCoqMTk6bWVldGluZ19OamMwWW1Zd05UVXROMlJoTlMwME0yRmhMV0V6T1RndE9HVmxNbVl4TkdVd05tUTNAdGhyZWFkLnYy",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting get",
        summary: "Gets specified meeting details",
        options: &[
            (
                "userId",
                "The id of the user, omit to retrieve meetings for current signed in user. Use either `id`, `userName`, or `email`, but not multiple.",
            ),
            (
                "userName",
                "The name of the user, omit to retrieve meetings for current signed in user. Use either `id`, `userName`, or `email`, but not multiple.",
            ),
            (
                "email",
                "The email of the user, omit to retrieve meetings for current signed in user. Use either `id`, `userName`, or `email`, but not multiple.",
            ),
            ("joinUrl", "The join URL from a calendar invite."),
        ],
        notes: &[
            "When you connect with application permissions, you should add an application access policy for the command to work.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieve meeting details for given meeting with userId and joinurl",
                "m365 teams meeting get --userId 00000000-0000-0000-0000-000000000000 --joinUrl 'https://teams.microsoft.com/l/meetup-join/19%3ameeting_MGQ4MDQyNTEtNTQ2NS00YjQxLTlkM2EtZWVkODYxODYzMmY2%40thread.v2/0?context=%7b%22Tid%22%3a%22909c6581-5130-43e9-88f3-fcb3582cde37%22%2c%22Oid%22%3a%22dc17674c-81d9-4adb-bfb2-8f6a442e4622%22%7d'",
            ),
            (
                "Retrieve meeting details for given meeting with userName and joinurl",
                "m365 teams meeting get --userName 'user@tenant.com' --joinUrl 'https://teams.microsoft.com/l/meetup-join/19%3ameeting_MGQ4MDQyNTEtNTQ2NS00YjQxLTlkM2EtZWVkODYxODYzMmY2%40thread.v2/0?context=%7b%22Tid%22%3a%22909c6581-5130-43e9-88f3-fcb3582cde37%22%2c%22Oid%22%3a%22dc17674c-81d9-4adb-bfb2-8f6a442e4622%22%7d'",
            ),
            (
                "Retrieve meeting details for given meeting with email and joinurl",
                "m365 teams meeting get --email 'user@tenant.com' --joinUrl 'https://teams.microsoft.com/l/meetup-join/19%3ameeting_MGQ4MDQyNTEtNTQ2NS00YjQxLTlkM2EtZWVkODYxODYzMmY2%40thread.v2/0?context=%7b%22Tid%22%3a%22909c6581-5130-43e9-88f3-fcb3582cde37%22%2c%22Oid%22%3a%22dc17674c-81d9-4adb-bfb2-8f6a442e4622%22%7d'",
            ),
            (
                "Retrieve meeting details of the currently logged in user with joinurl",
                "m365 teams meeting get --joinUrl 'https://teams.microsoft.com/l/meetup-join/19%3ameeting_MGQ4MDQyNTEtNTQ2NS00YjQxLTlkM2EtZWVkODYxODYzMmY2%40thread.v2/0?context=%7b%22Tid%22%3a%22909c6581-5130-43e9-88f3-fcb3582cde37%22%2c%22Oid%22%3a%22dc17674c-81d9-4adb-bfb2-8f6a442e4622%22%7d'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting list",
        summary: "Retrieves all online meetings for a given user or shared mailbox",
        options: &[
            (
                "startDateTime",
                "Time indicating the inclusive start of a time range of content to return.",
            ),
            (
                "endDateTime",
                "Time indicating the exclusive end of a time range of content to return.",
            ),
            (
                "isOrganizer",
                "Set to retrieve only meetings the user is organizer for.",
            ),
            (
                "userId",
                "The id of the user or shared mailbox, omit to retrieve meetings for current signed in user. Use either `userId`, `userName` or `email`, but not multiple. Use only when using application permissions.",
            ),
            (
                "userName",
                "The name of the user or shared mailbox, omit to retrieve meetings for current signed in user. Use either `userId`, `userName` or `email`, but not multiple. Use only when using application permissions.",
            ),
            (
                "email",
                "The email of the user or shared mailbox, omit to retrieve meetings for current signed in user. Use either `userId`, `userName` or `email`, but not multiple. Use only when using application permissions.",
            ),
        ],
        notes: &[
            "To use application permission for this command, an application access policy must be created and assigned to a user. This authorizes the app configured in the policy to retrieve online meetings on behalf of that user.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all meetings of the currently logged in user between two specific dates",
                "m365 teams meeting list --startDateTime '2022-01-01T10:00:00Z' --endDateTime '2022-03-31T23:59:59Z'",
            ),
            (
                "List all meetings for a specific user retrieved by email of which the user is an organizer using application permissions",
                "m365 teams meeting list --startDateTime '2022-01-01T10:00:00Z' --email user@tenant.com --isOrganizer",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting transcript get",
        summary: "Downloads a transcript for a given meeting",
        options: &[
            (
                "userId",
                "The id of the user, omit to get meeting transcript for the currently signed-in user.",
            ),
            (
                "userName",
                "The name of the user, omit to get the meeting transcript for the currently signed-in user.",
            ),
            (
                "email",
                "The email of the user, omit to get the meeting transcript for the currently signed-in user.",
            ),
            ("meetingId", "The Id of the meeting."),
            ("id", "The Id of the transcript."),
            ("outputFile", "Destination path for the report file."),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
            "To run this command with application permissions, tenant administrators must create an application access policy and grant it to a user. This authorizes the app configured in the policy to fetch online meetings and/or online meeting artifacts on behalf of that user. For more details, click here (https://learn.microsoft.com/graph/cloud-communication-online-meeting-application-access-policy).",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the specified transcript made for the current signed in user and Microsoft Teams meeting with given id.",
                "m365 teams meeting transcript get --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ --id MSMjMCMjNzU3ODc2ZDYtOTcwMi00MDhkLWFkNDItOTE2ZDNmZjkwZGY4",
            ),
            (
                "Saves the specified transcript made for a given user and Microsoft Teams meeting with the given id.",
                "m365 teams meeting transcript get --userName garthf@contoso.com --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ --id MSMjMCMjNzU3ODc2ZDYtOTcwMi00MDhkLWFkNDItOTE2ZDNmZjkwZGY4 --outputFile c:/Transcript.vtt",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams meeting transcript list",
        summary: "Lists all transcripts for a given meeting",
        options: &[
            (
                "userId",
                "The id of the user, omit to list meeting transcripts list for current signed in user.",
            ),
            (
                "userName",
                "The upn of the user, omit to list meeting transcripts list for current signed in user.",
            ),
            (
                "email",
                "The email of the user, omit to list meeting transcripts list for current signed in user.",
            ),
            ("meetingId", "The id of the meeting."),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
            "To run this command with application permissions, tenant administrators must create an application access policy and grant it to a user. This authorizes the app configured in the policy to fetch online meetings and/or online meeting artifacts on behalf of that user. For more details, click here (https://learn.microsoft.com/graph/cloud-communication-online-meeting-application-access-policy).",
        ],
        permissions: &[],
        examples: &[
            (
                "Lists all transcripts made for the current signed in user and Microsoft Teams meeting with given id",
                "m365 teams meeting transcript list --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ",
            ),
            (
                "Lists all transcripts for a meeting of a specific user",
                "m365 teams meeting transcript list --userName garthf@contoso.com --meetingId MSo1N2Y5ZGFjYy03MWJmLTQ3NDMtYjQxMy01M2EdFGkdRWHJlQ",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams membersettings list",
        summary: "Lists member settings for a Microsoft Teams team",
        options: &[(
            "teamId",
            "The ID of the team for which to get the member settings",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get member settings for a Microsoft Teams team",
            "m365 teams membersettings list --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
        )],
        links: &[],
    },
    Help {
        name: "teams membersettings set",
        summary: "Updates member settings of a Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the Teams team for which to update settings",
            ),
            (
                "allowAddRemoveApps",
                "Set to `true` to allow members to add and remove apps and to `false` to disallow it",
            ),
            (
                "allowCreateUpdateChannels",
                "Set to `true` to allow members to create and update channels and to `false` to disallow it",
            ),
            (
                "allowCreateUpdateRemoveConnectors",
                "Set to `true` to allow members to create, update and remove connectors and to `false` to disallow it",
            ),
            (
                "allowCreateUpdateRemoveTabs",
                "Set to `true` to allow members to create, update and remove tabs and to `false` to disallow it",
            ),
            (
                "allowDeleteChannels",
                "Set to `true` to allow members to create and update channels and to `false` to disallow it",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Allow members to create and edit channels",
                "m365 teams membersettings set --teamId '00000000-0000-0000-0000-000000000000' --allowCreateUpdateChannels true",
            ),
            (
                "Disallow members to add and remove apps",
                "m365 teams membersettings set --teamId '00000000-0000-0000-0000-000000000000' --allowAddRemoveApps false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams message get",
        summary: "Retrieves a message from a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team where the channel is located."),
            (
                "channelId",
                "The ID of the channel that contains the message.",
            ),
            ("id", "The ID of the message to retrieve."),
        ],
        notes: &[
            "You can only retrieve a message from a Microsoft Teams team if you are a member of that team.",
        ],
        permissions: &[],
        examples: &[(
            "Retrieve the specified message from a channel of the Microsoft Teams team.",
            "m365 teams message get --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:88f7e66a8dfe42be92db19505ae912a8@thread.skype --id 1540747442203",
        )],
        links: &[],
    },
    Help {
        name: "teams message list",
        summary: "Lists all messages from a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team where the channel is located."),
            (
                "channelId",
                "The ID of the channel for which to list messages.",
            ),
            (
                "since",
                "Date (ISO standard, dash separator) to get delta of messages from (in last 8 months).",
            ),
        ],
        notes: &[
            "You can only retrieve a message from a Microsoft Teams team if you are a member of that team.",
        ],
        permissions: &[],
        examples: &[
            (
                "List the messages from a channel of the Microsoft Teams team.",
                "m365 teams message list --teamId fce9e580-8bba-4638-ab5c-ab40016651e3 --channelId 19:eb30973b42a847a2a1df92d91e37c76a@thread.skype",
            ),
            (
                "List the messages from a channel of the Microsoft Teams team that have been created or modified since the date specified by the `--since` parameter (WARNING: only captures the last 8 months of data).",
                "m365 teams message list --teamId fce9e580-8bba-4638-ab5c-ab40016651e3 --channelId 19:eb30973b42a847a2a1df92d91e37c76a@thread.skype --since 2019-12-31T14:00:00Z",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams message remove",
        summary: "Removes a message from a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "ID of the Microsoft Teams team."),
            ("teamName", "Name of the Microsoft Teams team."),
            ("channelId", "Channel ID of the Microsoft Teams team."),
            ("channelName", "Channel name of the Microsoft Teams team."),
            ("id", "The ID of the Teams message."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command does only support delegated permissions.",
            "You can only remove Microsoft Teams messages that you created yourself.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove a message by using IDs",
                "m365 teams message remove --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:4a95f7d8db4c4e7fae857bcebe0623e6@thread.tacv2 --id 1540747442203",
            ),
            (
                "Remove a message by using display names",
                "m365 teams message remove --teamName Marketing --channelName Branding --id 1540747442203",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams message reply list",
        summary: "Retrieves replies to a message from a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team where the channel is located."),
            (
                "channelId",
                "The ID of the channel that contains the message.",
            ),
            (
                "messageId",
                "The ID of the message to retrieve replies for.",
            ),
        ],
        notes: &[
            "You can only retrieve replies to a message from a Microsoft Teams team if you are a member of that team.",
        ],
        permissions: &[],
        examples: &[(
            "Retrieve the replies from a specified message from a channel of the Microsoft Teams team.",
            "m365 teams message reply list --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:88f7e66a8dfe42be92db19505ae912a8@thread.skype --messageId 1540747442203",
        )],
        links: &[],
    },
    Help {
        name: "teams message restore",
        summary: "Restores a deleted message from a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "ID of the Microsoft Teams team."),
            ("teamName", "Name of the Microsoft Teams team."),
            ("channelId", "Channel ID of the Microsoft Teams team."),
            ("channelName", "Channel name of the Microsoft Teams team."),
            ("id", "The ID of the Teams message."),
        ],
        notes: &[
            "This command does only support delegated permissions.",
            "You can only restore Microsoft Teams messages that you created yourself.",
        ],
        permissions: &[],
        examples: &[
            (
                "Restore a deleted message by using IDs",
                "m365 teams message restore --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:4a95f7d8db4c4e7fae857bcebe0623e6@thread.tacv2 --id 1540747442203",
            ),
            (
                "Restore a deleted message by using display names",
                "m365 teams message restore --teamName Marketing --channelName Branding --id 1540747442203",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams message send",
        summary: "Sends a message to a channel in a Microsoft Teams team",
        options: &[
            ("teamId", "The ID of the team where the channel is located."),
            ("channelId", "The ID of the channel."),
            ("message", "The message to send."),
        ],
        notes: &[
            "You can only send a message to a channel in a Microsoft Teams team if you are a member of that team or channel.",
        ],
        permissions: &[],
        examples: &[
            (
                "Send a message to a specified channel in a Microsoft Teams team.",
                "m365 teams message send --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:88f7e66a8dfe42be92db19505ae912a8@thread.skype --message \"Hello World!\"",
            ),
            (
                "Send an HTML formatted message to a specified channel in a Microsoft Teams team.",
                "m365 teams message send --teamId 5f5d7b71-1161-44d8-bcc1-3da710eb4171 --channelId 19:88f7e66a8dfe42be92db19505ae912a8@thread.skype --message \"Hello <b>World</b>!\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams messagingsettings list",
        summary: "Lists messaging settings for a Microsoft Teams team",
        options: &[(
            "teamId",
            "The ID of the team for which to get the messaging settings.",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get messaging settings for a Microsoft Teams team.",
            "m365 teams messagingsettings list --teamId 2609af39-7775-4f94-a3dc-0dd67657e900",
        )],
        links: &[],
    },
    Help {
        name: "teams messagingsettings set",
        summary: "Updates messaging settings of a Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the Microsoft Teams team for which to update messaging settings.",
            ),
            (
                "allowUserEditMessages",
                "Set to `true` to allow users to edit messages and to `false` to disallow it.",
            ),
            (
                "allowUserDeleteMessages",
                "Set to `true` to allow users to delete messages and to `false` to disallow it.",
            ),
            (
                "allowOwnerDeleteMessages",
                "Set to `true` to allow owner to delete messages and to `false` to disallow it.",
            ),
            (
                "allowTeamMentions",
                "Set to `true` to allow @team or @[team name] mentions and to `false` to disallow it.",
            ),
            (
                "allowChannelMentions",
                "Set to `true` to allow @channel or @[channel name] mentions and to `false` to disallow it.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Allow users to edit messages in channels.",
                "m365 teams messagingsettings set --teamId '00000000-0000-0000-0000-000000000000' --allowUserEditMessages true",
            ),
            (
                "Disallow users to delete messages in channels.",
                "m365 teams messagingsettings set --teamId '00000000-0000-0000-0000-000000000000' --allowUserDeleteMessages false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report deviceusagedistributionusercounts",
        summary: "Gets the number of Microsoft Teams unique users by device type.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of Microsoft Teams unique users by device type for the last week.",
                "m365 teams report deviceusagedistributionusercounts --period D7",
            ),
            (
                "Gets the number of Microsoft Teams unique users by device type for the last week and exports the report data in the specified path in text format.",
                "m365 teams report deviceusagedistributionusercounts --period D7 --output text > \"deviceusagedistributionusercounts.txt\"",
            ),
            (
                "Gets the number of Microsoft Teams unique users by device type for the last week and exports the report data in the specified path in json format.",
                "m365 teams report deviceusagedistributionusercounts --period D7 --output json > \"deviceusagedistributionusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report deviceusageusercounts",
        summary: "Gets the number of Microsoft Teams daily unique users by device type",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of Microsoft Teams daily unique users by device type for the last week.",
                "m365 teams report deviceusageusercounts --period D7",
            ),
            (
                "Gets the number of Microsoft Teams daily unique users by device type for the last week and exports the report data in the specified path in text format.",
                "m365 teams report deviceusageusercounts --period D7 --output text > \"deviceusageusercounts.txt\"",
            ),
            (
                "Gets the number of Microsoft Teams daily unique users by device type for the last week and exports the report data in the specified path in json format.",
                "m365 teams report deviceusageusercounts --period D7 --output json > \"deviceusageusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report deviceusageuserdetail",
        summary: "Gets detail about Microsoft Teams device usage by user",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`.",
            ),
        ],
        notes: &[
            "As this report is only available for the past 28 days, date parameter value should be a date from that range.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets information about Microsoft Teams device usage by user for the last week.",
                "m365 teams report deviceusageuserdetail --period D7",
            ),
            (
                "Gets information about Microsoft Teams device usage by user for July 1, 2019.",
                "m365 teams report deviceusageuserdetail --date 2019-07-01",
            ),
            (
                "Gets information about Microsoft Teams device usage by user for the last week and exports the report data in the specified path in text format.",
                "m365 teams report deviceusageuserdetail --period D7 --output text > \"deviceusageuserdetail.txt\"",
            ),
            (
                "Gets information about Microsoft Teams device usage by user for the last week and exports the report data in the specified path in json format.",
                "m365 teams report deviceusageuserdetail --period D7 --output json > \"deviceusageuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report directroutingcalls",
        summary: "Gets details about direct routing calls made within a given time period",
        options: &[
            (
                "fromDateTime",
                "The start of time range to query. UTC, inclusive.",
            ),
            (
                "toDateTime",
                "The end time range to query. UTC, inclusive. Defaults to today if omitted.",
            ),
        ],
        notes: &[
            "This command only works with app-only permissions. You will need to create your own Microsoft Entra app with `CallRecords.Read.All` permission assigned. Instructions on how to create your own Microsoft Entra app can be found at Using your own Microsoft Entra identity",
            "The difference between `fromDateTime` and `toDateTime` cannot exceed a period of 90 days.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get details about direct routing calls made between 2020-10-31 and today.",
                "m365 teams report directroutingcalls --fromDateTime 2020-10-31",
            ),
            (
                "Get details about direct routing calls made between 2020-10-31 and 2020-12-31 and exports the report data in the specified path in text format.",
                "m365 teams report directroutingcalls --fromDateTime 2020-10-31 --toDateTime 2020-12-31 --output text > \"directroutingcalls.txt\"",
            ),
            (
                "Get details about direct routing calls made between 2020-10-31 and 2020-12-31 and exports the report data in the specified path in json format.",
                "m365 teams report directroutingcalls --fromDateTime 2020-10-31 --toDateTime 2020-12-31 --output json > \"directroutingcalls.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report pstncalls",
        summary: "Gets details about PSTN calls made within a given time period",
        options: &[
            (
                "fromDateTime",
                "The start of time range to query. UTC, inclusive.",
            ),
            (
                "toDateTime",
                "The end time range to query. UTC, inclusive. Defaults to today if omitted.",
            ),
        ],
        notes: &[
            "This command only works with app-only permissions. You will need to create your own Microsoft Entra app with `CallRecords.Read.All` permission assigned. Instructions on how to create your own Microsoft Entra app can be found at Using your own Microsoft Entra identity",
            "The difference between `fromDateTime` and `toDateTime` cannot exceed a period of 90 days.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get details about PSTN calls made between 2020-10-31 and today.",
                "m365 teams report pstncalls --fromDateTime 2020-10-31",
            ),
            (
                "Get details about PSTN calls made between 2020-10-31 and 2020-12-31 and exports the report data in the specified path in text format.",
                "m365 teams report pstncalls --fromDateTime 2020-10-31 --toDateTime 2020-12-31 --output text > \"pstncalls.txt\"",
            ),
            (
                "Get details about PSTN calls made between 2020-10-31 and 2020-12-31 and exports the report data in the specified path in json format.",
                "m365 teams report pstncalls --fromDateTime 2020-10-31 --toDateTime 2020-12-31 --output json > \"pstncalls.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report useractivitycounts",
        summary: "Gets the number of Microsoft Teams activities by activity type. The activity types are team chat messages, private chat messages, calls, and meetings",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of Microsoft Teams activities by activity type for the last week.",
                "m365 teams report useractivitycounts --period D7",
            ),
            (
                "Gets the number of Microsoft Teams activities by activity type for the last week and exports the report data in the specified path in text format.",
                "m365 teams report useractivitycounts --period D7 --output text > \"useractivitycounts.txt\"",
            ),
            (
                "Gets the number of Microsoft Teams activities by activity type for the last week and exports the report data in the specified path in json format.",
                "m365 teams report useractivitycounts --period D7 --output json > \"useractivitycounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report useractivityusercounts",
        summary: "Gets the number of Microsoft Teams users by activity type. The activity types are number of teams chat messages, private chat messages, calls, or meetings",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of Microsoft Teams users by activity type for the last week.",
                "m365 teams report useractivityusercounts --period D7",
            ),
            (
                "Gets the number of Microsoft Teams users by activity type for the last week and exports the report data in the specified path in text format.",
                "m365 teams report useractivityusercounts --period D7 --output text > \"useractivityusercounts.txt\"",
            ),
            (
                "Gets the number of Microsoft Teams users by activity type for the last week and exports the report data in the specified path in json format.",
                "m365 teams report useractivityusercounts --period D7 --output json > \"useractivityusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams report useractivityuserdetail",
        summary: "Gets details about Microsoft Teams user activity by user",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`.",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about Microsoft Teams user activity by user for the last week.",
                "m365 teams report useractivityuserdetail --period D7",
            ),
            (
                "Gets details about Microsoft Teams user activity by user for July 13, 2019.",
                "m365 teams report useractivityuserdetail --date 2019-07-13",
            ),
            (
                "Gets details about Microsoft Teams user activity by user for the last week and exports the report data in the specified path in text format.",
                "m365 teams report useractivityuserdetail --period D7 --output text > \"useractivityuserdetail.txt\"",
            ),
            (
                "Gets details about Microsoft Teams user activity by user for the last week and exports the report data in the specified path in json format.",
                "m365 teams report useractivityuserdetail --period D7 --output json > \"useractivityuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams tab add",
        summary: "Adds a tab to the specified channel",
        options: &[
            ("teamId", "The ID of the team to where the channel exists."),
            ("channelId", "The ID of the channel to add a tab to."),
            ("appId", "The ID of the Teams app that contains the Tab."),
            (
                "appName",
                "The name of the Teams app that contains the Tab.",
            ),
            ("contentUrl", "The URL used for rendering Tab contents."),
            ("entityId", "A unique identifier for the Tab."),
            ("removeUrl", "The URL displayed when a Tab is removed."),
            (
                "websiteUrl",
                "The URL for showing tab contents outside of Teams.",
            ),
        ],
        notes: &["The corresponding app must already be installed in the team."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "TeamsTab.Create"),
            ("Application", "Microsoft Graph", "TeamsTab.Create"),
        ],
        examples: &[
            (
                "Add teams tab for website.",
                "m365 teams tab add --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --appId 06805b9e-77e3-4b93-ac81-525eb87513b8 --appName 'My Contoso Tab' --contentUrl 'https://www.contoso.com/Orders/2DCA2E6C7A10415CAF6B8AB6661B3154/tabView'",
            ),
            (
                "Add teams tab for website with additional configuration which is unknown.",
                "m365 teams tab add --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --appId 06805b9e-77e3-4b93-ac81-525eb87513b8 --appName 'My Contoso Tab' --contentUrl 'https://www.contoso.com/Orders/2DCA2E6C7A10415CAF6B8AB6661B3154/tabView' --test1 'value for test1'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams tab get",
        summary: "Gets information about the specified Microsoft Teams tab",
        options: &[
            (
                "teamId",
                "The ID of the Microsoft Teams team where the tab is located. Specify either teamId or teamName but not both.",
            ),
            (
                "teamName",
                "The display name of the Microsoft Teams team where the tab is located. Specify either teamId or teamName but not both.",
            ),
            (
                "channelId",
                "The ID of the Microsoft Teams channel where the tab is located. Specify either channelId or channelName but not both.",
            ),
            (
                "channelName",
                "The display name of the Microsoft Teams channel where the tab is located. Specify either channelId or channelName but not both.",
            ),
            ("id", "The ID of the Microsoft Teams tab."),
            ("name", "The display name of the Microsoft Teams tab."),
        ],
        notes: &["You can only retrieve tabs for teams of which you are a member."],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Channel.ReadBasic.All, Team.ReadBasic.All, TeamsTab.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Channel.ReadBasic.All, Team.ReadBasic.All, TeamsTab.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a Microsoft Teams Tab with the specified ID.",
                "m365 teams tab get --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --id 1432c9da-8b9c-4602-9248-e0800f3e3f07",
            ),
            (
                "Get a Microsoft Teams Tab with the specified name.",
                "m365 teams tab get --teamName \"Team Name\" --channelName \"Channel Name\" --name \"Tab Name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams tab list",
        summary: "Lists tabs in the specified Microsoft Teams channel",
        options: &[
            (
                "teamId",
                "The ID of the Microsoft Teams team where the channel is located.",
            ),
            ("channelId", "The ID of the channel for which to list tabs."),
        ],
        notes: &[
            "You can only retrieve tabs for teams of which you are a member.",
            "Tabs Conversations and Files are present in every team and therefore not included in the list of available tabs.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "TeamsTab.Read.All"),
            ("Application", "Microsoft Graph", "TeamsTab.Read.All"),
        ],
        examples: &[
            (
                "List all tabs in a Microsoft Teams channel.",
                "m365 teams tab list --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype",
            ),
            (
                "Include all the values from the tab configuration and associated teams app.",
                "m365 teams tab list --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --output json",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams tab remove",
        summary: "Removes a tab from the specified channel",
        options: &[
            ("teamId", "The ID of the team where the tab exists"),
            ("channelId", "The ID of the channel to remove the tab from"),
            ("id", "The ID of the tab to remove"),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "TeamsTab.ReadWriteForTeam"),
            (
                "Application",
                "Microsoft Graph",
                "TeamsTab.ReadWriteForTeam.All",
            ),
        ],
        examples: &[
            (
                "Removes a tab from the specified channel. Will prompt for confirmation",
                "m365 teams tab remove --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --id 06805b9e-77e3-4b93-ac81-525eb87513b8",
            ),
            (
                "Removes a tab from the specified channel without prompting for confirmation",
                "m365 teams tab remove --teamId 00000000-0000-0000-0000-000000000000 --channelId 19:00000000000000000000000000000000@thread.skype --id 06805b9e-77e3-4b93-ac81-525eb87513b8 --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/channel-delete-tabs",
            "https://learn.microsoft.com/graph/api/channel-delete-tabs",
        )],
    },
    Help {
        name: "teams team add",
        summary: "Adds a new Microsoft Teams team",
        options: &[
            (
                "name",
                "Display name for the Microsoft Teams team. Required if `template` not supplied.",
            ),
            (
                "description",
                "Description for the Microsoft Teams team. Required if `template` not supplied.",
            ),
            (
                "template",
                "Template to use to create the team. If `name` or `description` are supplied, these take precedence over the template values.",
            ),
            (
                "wait",
                "Wait for the team to be provisioned before completing the command.",
            ),
            (
                "ownerUserNames",
                "User principal names of the owners to set, separated by a `,`.",
            ),
            ("ownerIds", "Ids of the owners to set, separated by a `,`."),
            (
                "ownerEmails",
                "Email addresses of the owners to set, separated by a `,`.",
            ),
            (
                "memberUserNames",
                "User principal names of the members to set, separated by a `,`.",
            ),
            (
                "memberIds",
                "Ids of the members to set, separated by a `,`.",
            ),
            (
                "memberEmails",
                "Email addresses of the members to set, separated by a `,`.",
            ),
        ],
        notes: &[
            "If you want to add a Team to an existing Microsoft 365 Group use the entra m365group teamify command instead.",
            "This command will return different responses based on the presence of the `--wait` option. If present, the command will return a `group` resource in the response. If not present, the command will return a `teamsAsyncOperation` resource in the response.",
            "When using application permissions, you have to specify atleast one owner using the `--ownerUserNames`, `--ownerIds` or `--ownerEmails` options. This will also result in the `--wait` option being enforced if more than one owner is being set.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add a new Microsoft Teams team with a specific owner.",
                "m365 teams team add --name \"Architecture\" --description \"Architecture Discussion\" --ownerIds \"d7a3d3e5-3e3d-4f3e-8a3d-3e3d3f3e3d3f\"",
            ),
            (
                "Add a new Microsoft Teams team using a template from a file.",
                "m365 teams team add --name \"Architecture\" --description \"Architecture Discussion\" --template @template.json",
            ),
            (
                "Add a new Microsoft Teams team using a template and wait for the team to be provisioned with multiple members.",
                "m365 teams team add --name \"Architecture\" --description \"Architecture Discussion\" --template @template.json --wait --memberUserNames \"john@contoso.com,doe@contoso.com\"",
            ),
        ],
        links: &[
            (
                "https://docs.microsoft.com/MicrosoftTeams/get-started-with-teams-templates",
                "https://docs.microsoft.com/MicrosoftTeams/get-started-with-teams-templates",
            ),
            (
                "https://docs.microsoft.com/graph/api/resources/group?view=graph-rest-1.0",
                "https://docs.microsoft.com/graph/api/resources/group?view=graph-rest-1.0",
            ),
            (
                "https://docs.microsoft.com/graph/api/resources/teamsasyncoperation?view=graph-rest-1.0",
                "https://docs.microsoft.com/graph/api/resources/teamsasyncoperation?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "teams team app list",
        summary: "Lists apps installed in the specified team",
        options: &[
            ("teamId", "The id of the Microsoft Teams team."),
            ("teamName", "The name of the Microsoft Teams team."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List applications installed in the specified Microsoft Teams team by id.",
                "m365 teams team app list --teamId 2eaf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "List applications installed in the specified Microsoft Teams team by name.",
                "m365 teams team app list --teamName \"Team Name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team archive",
        summary: "Archives specified Microsoft Teams team",
        options: &[
            ("id", "The ID of the Microsoft Teams team to archive."),
            (
                "name",
                "The display name of the Microsoft Teams team to archive.",
            ),
            (
                "shouldSetSpoSiteReadOnlyForMembers",
                "Sets the permissions for team members to read-only on the SharePoint Online site associated with the team.",
            ),
        ],
        notes: &[
            "Using this command, global admins and Microsoft Teams service admins can access teams that they are not a member of.",
            "If the command finds multiple Microsoft Teams teams with the specified name, it will prompt you to disambiguate which team it should use, listing the discovered team IDs.",
            "When a team is archived, users can no longer send or like messages on any channel in the team, edit the team's name, description, or other settings, or in general make most changes to the team. Membership changes to the team continue to be allowed.",
        ],
        permissions: &[],
        examples: &[
            (
                "Archive the specified Microsoft Teams team by id.",
                "m365 teams team archive --id 6f6fd3f7-9ba5-4488-bbe6-a789004d0d55",
            ),
            (
                "Archive the specified Microsoft Teams team by name.",
                "m365 teams team archive --name \"Team Name\"",
            ),
            (
                "Archive the specified Microsoft Teams team and set permissions for team members to read-only on the SharePoint Online site associated with the team",
                "m365 teams team archive --id 6f6fd3f7-9ba5-4488-bbe6-a789004d0d55 --shouldSetSpoSiteReadOnlyForMembers",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team clone",
        summary: "Creates a clone of a Microsoft Teams team",
        options: &[
            ("id", "The ID of the Microsoft Teams team to clone."),
            ("name", "The display name for the new Microsoft Teams team."),
            (
                "partsToClone",
                "A comma-separated list of the parts to clone. Allowed values are `apps`, `channels`, `members`, `settings`, `tabs`.",
            ),
            (
                "description",
                "The description for the new Microsoft Teams Team.",
            ),
            (
                "classification",
                "The classification for the new Microsoft Teams Team. If not specified, will be copied from the original Microsoft Teams Team.",
            ),
            (
                "visibility",
                "Specify the visibility of the new Microsoft Teams Team. Allowed values are `Private`, `Public`.",
            ),
        ],
        notes: &[
            "Using this command, global admins and Microsoft Teams service admins can access teams that they are not a member of.",
            "When tabs are cloned, they are put into an unconfigured state. The first time you open them, you'll go through the configuration screen. If the person opening the tab does not have permission to configure apps, they will see a message explaining that the tab hasn't been configured.",
        ],
        permissions: &[],
        examples: &[
            (
                "Creates a clone of a Microsoft Teams team with mandatory parameters.",
                "m365 teams team clone --id 15d7a78e-fd77-4599-97a5-dbb6372846c5 --name \"Library Assist\" --partsToClone \"apps,tabs,settings,channels,members\"",
            ),
            (
                "Creates a clone of a Microsoft Teams team with mandatory and optional parameters.",
                "m365 teams team clone --id 15d7a78e-fd77-4599-97a5-dbb6372846c5 --name \"Library Assist\" --partsToClone \"apps,tabs,settings,channels,members\" --description \"Self help community for library\" --classification \"Library\" --visibility \"public\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team get",
        summary: "Gets information about the specified Microsoft Teams team",
        options: &[
            (
                "id",
                "The ID of the Microsoft Teams team to retrieve information for.",
            ),
            (
                "name",
                "The display name of the Microsoft Teams team to retrieve information for.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Get information about the Microsoft Teams team by id.",
                "m365 teams team get --id 2eaf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Get information about Microsoft Teams team by name.",
                "m365 teams team get --name \"Team Name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team list",
        summary: "Lists Microsoft Teams teams in the current tenant",
        options: &[
            (
                "joined",
                "Show only teams that the user is a direct member of.",
            ),
            (
                "associated",
                "Show only teams that the user is associated with. This includes teams with direct membership and shared channels.",
            ),
            (
                "userId",
                "The ID of the user. Specify only when using `joined` or `associated` options.",
            ),
            (
                "userName",
                "The user principal name of the user. Specify only when using `joined` or `associated` options.",
            ),
        ],
        notes: &[
            "To list `joined` or `associated` teams of another user, you have to be a Teams Administrator or use application permissions.",
            "You can only see the details or archived status of the Microsoft Teams you are a member of.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all Microsoft Teams in the tenant",
                "m365 teams team list",
            ),
            (
                "List all Microsoft Teams teams you are a direct member of",
                "m365 teams team list --joined",
            ),
            (
                "List all Microsoft Teams teams you are a direct member of or are associated with via a shared channel",
                "m365 teams team list --associated",
            ),
            (
                "List all Microsoft Teams teams another user is member of",
                "m365 teams team list --joined --userName john.doe@contoso.com",
            ),
            (
                "List all Microsoft Teams teams where another user is a direct member of or is associated with via a shared channel",
                "m365 teams team list --associated --userId e3aca2a8-625a-449b-bb86-cfa84c5d08de",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team remove",
        summary: "Removes the specified Microsoft Teams team",
        options: &[
            ("id", "The ID of the Microsoft Teams team to remove."),
            (
                "name",
                "The display name of the Microsoft Teams team to remove.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the specified team.",
            ),
        ],
        notes: &[
            "When deleted, Microsoft 365 groups are moved to a temporary container and can be restored within 30 days. After that time, they are permanently deleted. This applies only to Microsoft 365 groups.",
            "If the command finds multiple Microsoft Teams teams with the specified name, it will prompt you to disambiguate which team it should use, listing the discovered team IDs.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the specified Microsoft Teams team by id.",
                "m365 teams team remove --id 00000000-0000-0000-0000-000000000000",
            ),
            (
                "Removes the specified Microsoft Teams team by name.",
                "m365 teams team remove --name \"Team Name\"",
            ),
            (
                "Removes the specified team without confirmation.",
                "m365 teams team remove --id 00000000-0000-0000-0000-000000000000 --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/directory?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/directory?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "teams team set",
        summary: "Updates settings of a Microsoft Teams team",
        options: &[
            (
                "id",
                "The ID of the Microsoft Teams team for which to update settings.",
            ),
            (
                "name",
                "The display name for the Microsoft Teams team for which to update settings.",
            ),
            (
                "description",
                "The description for the Microsoft Teams team.",
            ),
            (
                "mailNickName",
                "The mail alias for the Microsoft Teams team.",
            ),
            (
                "classification",
                "The classification for the Microsoft Teams team.",
            ),
            (
                "visibility",
                "The visibility of the Microsoft Teams team. Valid values `Private`, `Public`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Set Microsoft Teams team visibility.",
                "m365 teams team set --id \"00000000-0000-0000-0000-000000000000\" --visibility Private",
            ),
            (
                "Set Microsoft Teams team classification.",
                "m365 teams team set --id \"00000000-0000-0000-0000-000000000000\" --classification MBI",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams team unarchive",
        summary: "Restores an archived Microsoft Teams team",
        options: &[
            ("id", "The ID of the Microsoft Teams team to restore."),
            (
                "name",
                "The display name of the Microsoft Teams team to restore.",
            ),
        ],
        notes: &[
            "This command supports admin permissions. Global admins and Microsoft Teams service admins can restore teams that they are not a member of.",
            "If the command finds multiple Microsoft Teams teams with the specified name, it will prompt you to disambiguate which team it should use, listing the discovered team IDs.",
            "This command restores users' ability to send messages and edit the team, abiding by tenant and team settings.",
        ],
        permissions: &[],
        examples: &[
            (
                "Restore an archived Microsoft Teams team by id.",
                "m365 teams team unarchive --id 6f6fd3f7-9ba5-4488-bbe6-a789004d0d55",
            ),
            (
                "Restore an archived Microsoft Teams team by name.",
                "m365 teams team unarchive --name \"Team Name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams user app add",
        summary: "Installs an app in the personal scope of the specified user",
        options: &[
            ("id", "The ID of the app to install."),
            ("name", "Name of the app to install."),
            ("userId", "The ID of the user to install the app for."),
            ("userName", "The UPN of the user to install the app for."),
        ],
        notes: &[
            "The `id` has to be the ID of the app from the Microsoft Teams App Catalog. Do not use the ID from the manifest of the zip app package. Use the teams app list command to get this ID.",
        ],
        permissions: &[],
        examples: &[
            (
                "Install an app by id from the catalog for the specified user by id.",
                "m365 teams user app add --id 4440558e-8c73-4597-abc7-3644a64c4bce --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Install an app by name from the catalog for the specified user by name.",
                "m365 teams user app add --name HelloWorld --userName admin@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams user app list",
        summary: "Lists the apps installed in the personal scope of the specified user",
        options: &[
            ("userId", "The ID of the user to get the apps from."),
            ("userName", "The UPN of the user to get the apps from."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List the apps installed in the personal scope of the specified user using its ID.",
                "m365 teams user app list --userId 4440558e-8c73-4597-abc7-3644a64c4bce",
            ),
            (
                "List the apps installed in the personal scope of the specified user using its UPN.",
                "m365 teams user app list --userName admin@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams user app remove",
        summary: "Uninstalls an app from the personal scope of the specified user",
        options: &[
            (
                "id",
                "The unique id of the app instance installed for the user.",
            ),
            ("name", "Name of the app instance installed for the user."),
            ("userId", "The ID of the user to uninstall the app for."),
            ("userName", "The UPN of the user to uninstall the app for."),
            ("force", "Confirm removal of app for user."),
        ],
        notes: &[
            "The `id` has to be the id of the app instance installed for the user. Do not use the ID from the manifest of the zip app package or the id from the Microsoft Teams App Catalog.",
        ],
        permissions: &[],
        examples: &[
            (
                "Uninstall an app by id for the specified user.",
                "m365 teams user app remove --id YzUyN2E0NzAtYTg4Mi00ODFjLTk4MWMtZWU2ZWZhYmE4NWM3IyM0ZDFlYTA0Ny1mMTk2LTQ1MGQtYjJlOS0wZDI4NTViYTA1YTY= --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Uninstall an app by id for the specified user using its UPN.",
                "m365 teams user app remove --id YzUyN2E0NzAtYTg4Mi00ODFjLTk4MWMtZWU2ZWZhYmE4NWM3IyM0ZDFlYTA0Ny1mMTk2LTQ1MGQtYjJlOS0wZDI4NTViYTA1YTY= --userName admin@contoso.com",
            ),
            (
                "Uninstall an app by name for the specified user.",
                "m365 teams user app remove --name HelloWorld --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams user app upgrade",
        summary: "Upgrades an app in the personal scope of the specified user",
        options: &[
            (
                "id",
                "The unique id of the app instance installed for the user.",
            ),
            ("name", "Name of the app instance installed for the user."),
            ("userId", "The ID of the user to upgrade the app for."),
            ("userName", "The UPN of the user to upgrade the app for."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Upgrade an app by name for the specified user using its UPN.",
                "m365 teams user app upgrade --name HelloWorld --userName admin@contoso.com",
            ),
            (
                "Upgrade an app by name for the specified user using its id.",
                "m365 teams user app upgrade --name HelloWorld --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Upgrade an app by id for the specified user using its UPN.",
                "m365 teams user app upgrade --id YzUyN2E0NzAtYTg4Mi00ODFjLTk4MWMtZWU2ZWZhYmE4NWM3IyM0ZDFlYTA0Ny1mMTk2LTQ1MGQtYjJlOS0wZDI4NTViYTA1YTY= --userName admin@contoso.com",
            ),
            (
                "Upgrade an app by id for the specified user using its id.",
                "m365 teams user app upgrade --id YzUyN2E0NzAtYTg4Mi00ODFjLTk4MWMtZWU2ZWZhYmE4NWM3IyM0ZDFlYTA0Ny1mMTk2LTQ1MGQtYjJlOS0wZDI4NTViYTA1YTY= --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
        ],
        links: &[],
    },
    Help {
        name: "teams user list",
        summary: "Lists users for the specified Microsoft Teams team",
        options: &[
            (
                "teamId",
                "The ID of the Microsoft Teams team for which to list users.",
            ),
            (
                "role",
                "Filter the results to only users with the given role: `Owner`, `Member`, `Guest`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all users and their role in the specified Microsoft teams team.",
                "m365 teams user list --teamId '00000000-0000-0000-0000-000000000000'",
            ),
            (
                "List all owners and their role in the specified Microsoft teams team.",
                "m365 teams user list --teamId '00000000-0000-0000-0000-000000000000' --role Owner",
            ),
        ],
        links: &[],
    },
];

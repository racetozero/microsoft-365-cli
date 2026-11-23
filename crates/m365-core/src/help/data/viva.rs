//! Help for the `m365 viva ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "viva connections app create",
        summary: "Creates a Viva Connections desktop app package to upload to Microsoft Teams",
        options: &[
            (
                "portalUrl",
                "The URL of the site to pin in Microsoft Teams. Must be a Communication site",
            ),
            (
                "name",
                "Name of the app to create, eg. `Contoso`. No longer than 30 characters",
            ),
            (
                "description",
                "Short description of the app. Displayed in the app's About dialog. No longer than 80 characters",
            ),
            (
                "longDescription",
                "Long description of the app. Displayed in the app's About dialog. No longer than 4000 characters",
            ),
            (
                "privacyPolicyUrl",
                "URL to your organization's privacy policy. Displayed in the app's About dialog. Defaults to `https://privacy.microsoft.com/en-us/privacystatement` if not specified",
            ),
            (
                "termsOfUseUrl",
                "URL to your organization's terms of use. Displayed in the app's About dialog. Defaults to `https://go.microsoft.com/fwlink/?linkid=2039674` if not specified",
            ),
            (
                "companyName",
                "Your organization's name. Displayed in the app's About dialog",
            ),
            (
                "companyWebsiteUrl",
                "Your organization's website URL. Displayed in the app's About dialog",
            ),
            (
                "coloredIconPath",
                "Absolute or relative path to the color icon for your app",
            ),
            (
                "outlineIconPath",
                "Absolute or relative path to the outline icon for your app",
            ),
            (
                "accentColor",
                "A HEX color to use in conjunction with and as a background for your outline icon. Defaults to `#40497E` if not specified",
            ),
            (
                "force",
                "Specify, to overwrite the existing package file on disk",
            ),
        ],
        notes: &[
            "If the specified portal URL doesn't exist, the command will return `404 - FILE NOT FOUND` error.",
            "The specified portal URL must point to a valid Communication site. To get the list of Communication sites in your tenant, execute: `m365 spo site list --type CommunicationSite`.",
            "The command generates a Microsoft Teams app package. App packages must meet specific requirements to be uploaded to Microsoft Teams. Specified attributes must not exceed their maximum length and the specified color and outline icons must be respectively 192x192px and 32x32px. For the latest list of requirements, see the links in the More information section at the end of this page. The generated app package will be written in the current working folder.",
            "After creating the Viva Connections desktop app package, you need to upload it to your Microsoft Teams app catalog. You can do it either manually, or using the CLI by executing `m365 teams app publish --filePath ./contoso.zip`.",
        ],
        permissions: &[],
        examples: &[(
            "Create a Viva Connections desktop app package",
            "m365 viva connections app create --portalUrl https://contoso.sharepoint.com --name Contoso --description \"Contoso company app\" --longDescription \"Stay on top of what's happening at Contoso\" --companyName Contoso --companyWebsiteUrl https://contoso.com --coloredIconPath icon-color.png --outlineIconPath icon-outline.png",
        )],
        links: &[
            (
                "https://docs.microsoft.com/viva/connections/add-viva-connections-app",
                "https://docs.microsoft.com/viva/connections/add-viva-connections-app",
            ),
            (
                "https://docs.microsoft.com/microsoftteams/platform/concepts/deploy-and-publish/appsource/prepare/app-manifest-checklist",
                "https://docs.microsoft.com/microsoftteams/platform/concepts/deploy-and-publish/appsource/prepare/app-manifest-checklist?WT.mc_id=m365-15896-cxa",
            ),
            (
                "https://docs.microsoft.com/microsoftteams/platform/resources/schema/manifest-schema",
                "https://docs.microsoft.com/microsoftteams/platform/resources/schema/manifest-schema?WT.mc_id=m365-15896-cxa",
            ),
        ],
    },
    Help {
        name: "viva engage community add",
        summary: "Creates a new community in Viva Engage",
        options: &[
            (
                "displayName",
                "The name of the community. The maximum length is 255 characters.",
            ),
            (
                "description",
                "The description of the community. The maximum length is 1024 characters.",
            ),
            (
                "privacy",
                "Defines the privacy level of the community. The possible values are: `public`, `private`.",
            ),
            (
                "adminEntraIds",
                "Comma-separated list of Microsoft Entra IDs, assigning admin privileges to the designated individuals. Required when using application permissions. Specify either `adminEntraIds` or `adminEntraUserNames`, but not both.",
            ),
            (
                "adminEntraUserNames",
                "Comma-separated list of Microsoft Entra UPNs, assigning admin privileges to the designated individuals. Required when using application permissions. Specify either `adminEntraIds` or `adminEntraUserNames`, but not both.",
            ),
            ("wait", "Wait for the operation to finish."),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
            "Creating a community is limited to networks in Native mode only - legacy and external Yammer networks will not be able to use this API for community creation.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Community.ReadWrite.All, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Community.ReadWrite.All, User.ReadBasic.All",
            ),
        ],
        examples: &[
            (
                "Create a public community and wait for the community to be created",
                "m365 viva engage community add --displayName \"Software engineers\" --description \"A community for all software engineers\" --privacy public --wait",
            ),
            (
                "Create a private community",
                "m365 viva engage community add --displayName \"Software engineers\" --description \"A community for all software engineers\" --privacy private",
            ),
            (
                "Create a private community with owners",
                "m365 viva engage community add --displayName \"Software engineers\" --description \"A community for all software engineers\" --privacy private --adminEntraUserNames \"john.doe@contoso.onmicrosoft.com,jane.doe@contoso.onmicrosoft.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage community get",
        summary: "Gets information of a Viva Engage community",
        options: &[("id", "The unique identifier of the community.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Community.Read.All"),
            ("Application", "Microsoft Graph", "Community.Read.All"),
        ],
        examples: &[(
            "Get a specific community by ID.",
            "m365 viva engage community get --id eyJfdHlwZSI6Ikdyb3VwIiwiaWQiOiI0Mjg1NzkwNjE3NyJ9",
        )],
        links: &[],
    },
    Help {
        name: "viva engage community list",
        summary: "Lists all Viva Engage communities",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Community.Read.All"),
            ("Application", "Microsoft Graph", "Community.Read.All"),
        ],
        examples: &[(
            "Lists all Viva Engage communities",
            "m365 viva engage community list",
        )],
        links: &[],
    },
    Help {
        name: "viva engage community remove",
        summary: "Removes a Viva Engage community",
        options: &[
            ("id", "The id of the community."),
            ("displayName", "The name of the community."),
            (
                "entraGroupId",
                "The id of the Microsoft 365 group associated with the community.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "When the Viva Engage community is removed, all the associated Microsoft 365 content, including the M365 group, the document library, OneNote notebook, and Planner plan is deleted.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Community.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Community.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a community specified by id without prompting",
                "m365 viva engage community remove --id eyJfdHlwZSI6Ikdyb3VwIiwiaWQiOiI0NzY5MTM1ODIwOSJ9 --force",
            ),
            (
                "Remove a community specified by name and prompt for confirmation",
                "m365 viva engage community remove --displayName 'Software Engineers'",
            ),
            (
                "Remove a community specified by Entra group id and prompt for confirmation",
                "m365 viva engage community remove --entraGroupId '0bed8b86-5026-4a93-ac7d-56750cc099f1'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage community set",
        summary: "Updates an existing Viva Engage community",
        options: &[
            ("id", "The id of the community."),
            ("displayName", "The name of the community."),
            (
                "entraGroupId",
                "The id of the Microsoft Entra group associated with the community.",
            ),
            (
                "newDisplayName",
                "New name for the community. The maximum length is 255 characters.",
            ),
            (
                "description",
                "The description of the community. The maximum length is 1024 characters.",
            ),
            (
                "privacy",
                "Defines the privacy level of the community. The possible values are: `public`, and `private`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Community.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Community.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update info about the community specified by id",
                "m365 viva engage community set --id eyJfdHlwZSI6Ikdyb3VwIiwiaWQiOiI0NzY5MTM1ODIwOSJ9 --newDisplayName 'Developers' --description 'Community for all devs' --privacy public",
            ),
            (
                "Update info about the community specified by name",
                "m365 viva engage community set --displayName 'Developrs' --newDisplayName 'Developers'",
            ),
            (
                "Update info about the community specified by Entra group id",
                "m365 viva engage community set --entraGroupId '0bed8b86-5026-4a93-ac7d-56750cc099f1' --newDisplayName 'Developers'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage community user add",
        summary: "Adds a user to a specific Microsoft 365 Viva Engage community",
        options: &[
            ("communityId", "The ID of the Viva Engage community."),
            (
                "communityDisplayName",
                "The display name of the Viva Engage community.",
            ),
            ("entraGroupId", "The ID of the Microsoft 365 group."),
            (
                "ids",
                "Microsoft Entra IDs of users. You can pass a comma-separated list of multiple IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can pass a comma-separated list of multiple UPNs.",
            ),
            (
                "role",
                "The role to be assigned to the new users. Valid values: `Admin`, `Member`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupMember.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "GroupMember.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Add a single user specified by ID as a member to a community specified by display name.",
                "m365 viva engage community user add --communityDisplayName \"All company\" --ids 098b9f52-f48c-4401-819f-29c33794c3f5 --role Member",
            ),
            (
                "Add multiple users specified by ID as members to a community specified by ID.",
                "m365 viva engage community user add --communityId eyJfdHlwZSI6Ikdyb3VwIiwiaWQiOiIzNjAyMDAxMTAwOSJ9 --ids \"098b9f52-f48c-4401-819f-29c33794c3f5,f1e06e31-3abf-4746-83c2-1513d71f38b8\" --role Member",
            ),
            (
                "Add a single user specified by UPN as an admin to a community specified by display name.",
                "m365 viva engage community user add --communityDisplayName \"All company\" --userNames john.doe@contoso.com --role Admin",
            ),
            (
                "Adds multiple users specified by UPN as admins to a community specified by its group ID.",
                "m365 viva engage community user add --entraGroupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userNames \"john.doe@contoso.com,adele.vance@contoso.com\" --role Admin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage community user list",
        summary: "Lists all users within a specified Microsoft 365 Viva Engage community",
        options: &[
            ("communityId", "The ID of the Viva Engage community."),
            (
                "communityDisplayName",
                "The display name of the Viva Engage community.",
            ),
            ("entraGroupId", "The ID of the Microsoft 365 group."),
            (
                "role",
                "Filter the results to only users with the given role: `Admin`, `Member`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all users from a community specified by ID.",
                "m365 viva engage community user list --communityId eyJfdHlwZSI6Ikdyb3VwIiwiaWQiOiIzNjAyMDAxMTAwOSJ9",
            ),
            (
                "List all admins from a community specified by display name.",
                "m365 viva engage community user list --communityDisplayName \"All company\" --role Admin",
            ),
            (
                "List all members from a community specified by group ID.",
                "m365 viva engage community user list --entraGroupId b6c35b51-ebca-445c-885a-63a67d24cb53 --role Member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage community user remove",
        summary: "Removes a specified user from a Microsoft 365 Viva Engage community",
        options: &[
            ("communityId", "The ID of the Viva Engage community."),
            (
                "communityDisplayName",
                "The display name of the Viva Engage community.",
            ),
            ("entraGroupId", "The ID of the Microsoft 365 group."),
            ("id", "Microsoft Entra ID of the user."),
            ("userName", "The user principal name of the user."),
            (
                "force",
                "Don't prompt for confirming removing the user from the specified Viva Engage community.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupMember.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "GroupMember.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove a user specified by ID as a member from a community specified by display name.",
                "m365 viva engage community user remove --communityDisplayName \"All company\" --id 098b9f52-f48c-4401-819f-29c33794c3f5",
            ),
            (
                "Remove a user specified by UPN from a community specified by its group ID without confirmation.",
                "m365 viva engage community user remove --entraGroupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userName john.doe@contoso.com --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage message add",
        summary: "Posts a Viva Engage network message on behalf of the current user",
        options: &[
            ("body", "The text of the message body."),
            (
                "groupId",
                "Post the message to this group, specified by ID. If this is set then the networkId is inferred from it. You must either specify `groupId`, `repliedToId`, or `directToUserIds` to send the message.",
            ),
            (
                "repliedToId",
                "The message ID this message is in reply to. If this is set then groupId and networkId are inferred from it. You must either specify `groupId`, `repliedToId`, or `directToUserIds` to send the message.",
            ),
            (
                "directToUserIds",
                "Send a private message to one or more users, specified by ID. Alternatively, you can use the Viva Engage network e-mail addresses instead of the IDs. You must either specify `groupId`, `repliedToId`, or `directToUserIds` to send the message.",
            ),
            ("networkId", "Specify the network to post a message."),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Yammer", "user_impersonation")],
        examples: &[
            (
                "Replies to a message with the ID 1231231231.",
                "m365 viva engage message add --body \"Hello everyone!\" --repliedToId 1231231231",
            ),
            (
                "Sends a private conversation to the user with the ID 1231231231.",
                "m365 viva engage message add --body \"Hello everyone!\" --directToUserIds 1231231231",
            ),
            (
                "Sends a private conversation to multiple users by ID.",
                "m365 viva engage message add --body \"Hello everyone!\" --directToUserIds \"1231231231,1121312\"",
            ),
            (
                "Sends a private conversation to the user with several e-mail addresses.",
                "m365 viva engage message add --body \"Hello everyone!\" --directToUserIds \"pl@nubo.eu,sc@nubo.eu\"",
            ),
            (
                "Posts a message to the group with the ID 12312312312.",
                "m365 viva engage message add --body \"Hello everyone!\" --groupId 12312312312",
            ),
            (
                "Posts a message to the group with the ID 12312312312 in the network 11112312.",
                "m365 viva engage message add --body \"Hello everyone!\" --groupId 12312312312 --networkId 11112312",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage message get",
        summary: "Returns a Viva Engage message",
        options: &[("id", "The id of the Viva Engage message.")],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Yammer", "user_impersonation")],
        examples: &[
            (
                "Returns the Viva Engage message with the id 1239871123.",
                "m365 viva engage message get --id 1239871123",
            ),
            (
                "Returns the Viva Engage message with the id 1239871123 in JSON format.",
                "m365 viva engage message get --id 1239871123 --output json",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage message like set",
        summary: "Likes or unlikes a Viva Engage message",
        options: &[
            ("messageId", "The id of the Viva Engage message."),
            (
                "enable",
                "Set to `true` to like a message. Set to `false` to unlike it. Default `true`.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Likes the message with the ID `5611239081`",
                "m365 viva engage message like set --messageId 5611239081",
            ),
            (
                "Unlike the message with the ID `5611239081`",
                "m365 viva engage message like set --messageId 5611239081 --enable false",
            ),
            (
                "Unlike the message with the ID `5611239081` without asking for confirmation",
                "m365 viva engage message like set --messageId 5611239081 --enable false --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage message list",
        summary: "Returns all accessible messages from the user's Viva Engage network",
        options: &[
            (
                "olderThanId",
                "Returns messages older than the message ID specified as a numeric string",
            ),
            (
                "threaded",
                "Will only return the thread starter (first message) for each thread. This parameter is intended for apps which need to display message threads collapsed",
            ),
            (
                "feedType",
                "Returns messages from a specific feed. Available options: `All`, `Top`, `My`, `Following`, `Sent`, `Private`, `Received`. Default `All`",
            ),
            ("groupId", "Returns the messages from a specific group"),
            ("threadId", "Returns the messages from a specific thread"),
            ("limit", "Limits the messages returned"),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "Feed types",
            "- All: Corresponds to “All” conversations in the Viva Engage web interface",
            "- Top: The algorithmic feed for the user that corresponds to \"Top\" conversations. The Top conversations feed is the feed currently shown in the Viva Engage mobile apps",
            "- My: The user’s feed, based on the selection they have made between \"Following\" and \"Top\" conversations",
            "- Following: The \"Following\" feed which is conversations involving people and topics that the user is following",
            "- Sent: All messages sent by the user",
            "- Private: Private messages received by the user",
            "- Received: All messages received by the user",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Yammer", "user_impersonation")],
        examples: &[
            (
                "Returns all Viva Engage network messages",
                "m365 viva engage message list",
            ),
            (
                "Returns all Viva Engage network messages older than the message ID 5611239081",
                "m365 viva engage message list --olderThanId 5611239081",
            ),
            (
                "Returns all Viva Engage network thread starter (first message) for each thread",
                "m365 viva engage message list --threaded",
            ),
            (
                "Returns the first 10 Viva Engage network messages",
                "m365 viva engage message list --limit 10",
            ),
            (
                "Returns the first 10 Viva Engage network messages from the Viva Engage group 312891231",
                "m365 viva engage message list --groupId 312891231 --limit 10",
            ),
            (
                "Returns the first 10 Viva Engage network messages from thread 5611239081",
                "m365 viva engage message list --threadId 5611239081 --limit 10",
            ),
            (
                "Returns the first 20 Viva Engage message from the sent feed of the user",
                "m365 viva engage message list --feedType Sent --limit 20",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage message remove",
        summary: "Removes a Viva Engage message",
        options: &[
            ("id", "The id of the Viva Engage message."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "To remove a message, you must either:",
            "- have posted the message yourself",
            "- be an administrator of the group the message was posted to or",
            "- be an admin of the network the message is in",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Yammer", "user_impersonation")],
        examples: &[
            (
                "Removes the Viva Engage message with the id 1239871123",
                "m365 viva engage message remove --id 1239871123",
            ),
            (
                "Removes the Viva Engage message with the id 1239871123 without prompting for confirmation.",
                "m365 viva engage message remove --id 1239871123 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage network list",
        summary: "Returns a list of networks to which the current user has access",
        options: &[(
            "withSuspended",
            "Include the networks in which the user is suspended.",
        )],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Returns the current user's networks.",
                "m365 viva engage network list",
            ),
            (
                "Returns the current user's networks including the networks in which the user is suspended.",
                "m365 viva engage network list --withSuspended",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report activitycounts",
        summary: "Gets the trends on the amount of Viva Engage activity in your organization by how many messages were posted, read, and liked",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trends on the amount of Viva Engage activity in your organization by how many messages were posted, read, and liked for the last week",
                "m365 viva engage report activitycounts --period D7",
            ),
            (
                "Gets the trends on the amount of Viva Engage activity in your organization by how many messages were posted, read, and liked for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report activitycounts --period D7 --output text > \"activitycounts.txt\"",
            ),
            (
                "Gets the trends on the amount of Viva Engage activity in your organization by how many messages were posted, read, and liked for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report activitycounts --period D7 --output json > \"activitycounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report activityusercounts",
        summary: "Gets the trends on the number of unique users who posted, read, and liked Viva Engage messages",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trends on the number of unique users who posted, read, and liked Viva Engage messages for the last week",
                "m365 viva engage report activityusercounts --period D7",
            ),
            (
                "Gets the trends on the number of unique users who posted, read, and liked Viva Engage messages for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report activityusercounts --period D7 --output text > \"activityusercounts.txt\"",
            ),
            (
                "Gets the trends on the number of unique users who posted, read, and liked Viva Engage messages for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report activityusercounts --period D7 --output json > \"activityusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report activityuserdetail",
        summary: "Gets details about Viva Engage activity by user",
        options: &[
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`. Specify the date or period, but not both.",
            ),
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
        ],
        notes: &[
            "As this report is only available for the past 28 days, date parameter value should be a date from that range.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets details about Viva Engage activity by user for the last week",
                "m365 viva engage report activityuserdetail --period D7",
            ),
            (
                "Gets details about Viva Engage activity by user for May 1, 2019",
                "m365 viva engage report activityuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about Viva Engage activity by user for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report activityuserdetail --period D7 --output text > \"activityuserdetail.txt\"",
            ),
            (
                "Gets details about Viva Engage activity by user for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report activityuserdetail --period D7 --output json > \"activityuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report deviceusagedistributionusercounts",
        summary: "Gets the number of users by device type",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of users by device type for the last week",
                "m365 viva engage report deviceusagedistributionusercounts --period D7",
            ),
            (
                "Gets the number of users by device type for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report deviceusagedistributionusercounts --period D7 --output text > \"deviceusagedistributionusercounts.txt\"",
            ),
            (
                "Gets the number of users by device type for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report deviceusagedistributionusercounts --period D7 --output json > \"deviceusagedistributionusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report deviceusageusercounts",
        summary: "Gets the number of daily users by device type",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of daily users by device type for the last week",
                "m365 viva engage report deviceusageusercounts --period D7",
            ),
            (
                "Gets the number of daily users by device type for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report deviceusageusercounts --period D7 --output text > \"deviceusageusercounts.txt\"",
            ),
            (
                "Gets the number of daily users by device type for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report deviceusageusercounts --period D7 --output json > \"deviceusageusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report deviceusageuserdetail",
        summary: "Gets details about viva engage device usage by user",
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
                "Gets details about Viva Engage device usage by user for the last week",
                "m365 viva engage report deviceusageuserdetail --period D7",
            ),
            (
                "Gets details about Viva Engage device usage by user for July 1, 2019",
                "m365 viva engage report deviceusageuserdetail --date 2019-07-01",
            ),
            (
                "Gets details about Viva Engage device usage by user for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report deviceusageuserdetail --period D7 --output text > \"deviceusageuserdetail.txt\"",
            ),
            (
                "Gets details about Viva Engage device usage by user for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report deviceusageuserdetail --period D7 --output json > \"deviceusageuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report groupsactivitycounts",
        summary: "Gets the number of Viva Engage messages posted, read, and liked in groups",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of Viva Engage messages posted, read, and liked in groups for the last week",
                "m365 viva engage report groupsactivitycounts --period D7",
            ),
            (
                "Gets the number of Viva Engage messages posted, read, and liked in groups for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report groupsactivitycounts --period D7 --output text > \"groupsactivitycounts.txt\"",
            ),
            (
                "Gets the number of Viva Engage messages posted, read, and liked in groups for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report groupsactivitycounts --period D7 --output json > \"groupsactivitycounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report groupsactivitydetail",
        summary: "Gets details about Viva Engage groups activity by group",
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
                "Gets details about Viva Engage groups activity by group for the last week",
                "m365 viva engage report groupsactivitydetail --period D7",
            ),
            (
                "Gets details about Viva Engage groups activity by group for July 1, 2019",
                "m365 viva engage report groupsactivitydetail --date 2019-07-01",
            ),
            (
                "Gets details about Viva Engage groups activity by group for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report groupsactivitydetail --period D7 --output text > \"groupsactivitydetail.txt\"",
            ),
            (
                "Gets details about Viva Engage groups activity by group for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report groupsactivitydetail --period D7 --output json > \"groupsactivitydetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage report groupsactivitygroupcounts",
        summary: "Gets the total number of groups that existed and how many included group conversation activity",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the total number of groups that existed and how many included group conversation activity for the last week",
                "m365 viva engage report groupsactivitygroupcounts --period D7",
            ),
            (
                "Gets the total number of groups that existed and how many included group conversation activity for the last week and exports the report data in the specified path in text format",
                "m365 viva engage report groupsactivitygroupcounts --period D7 --output text > \"groupsactivitygroupcounts.txt\"",
            ),
            (
                "Gets the total number of groups that existed and how many included group conversation activity for the last week and exports the report data in the specified path in json format",
                "m365 viva engage report groupsactivitygroupcounts --period D7 --output json > \"groupsactivitygroupcounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage role list",
        summary: "Lists all Viva Engage roles",
        options: &[],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "EngagementRole.Read.All"),
            ("Application", "Microsoft Graph", "EngagementRole.Read.All"),
        ],
        examples: &[("List all Viva Engage roles", "m365 viva engage role list")],
        links: &[],
    },
    Help {
        name: "viva engage role member add",
        summary: "Assigns a Viva Engage role to a user",
        options: &[
            ("roleId", "The id of the Viva Engage role."),
            ("roleName", "The name of the Viva Engage role."),
            ("userId", "The id of the user to be assigned to the role."),
            (
                "userName",
                "The UPN of the user to be assigned to the role.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reaches general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "EngagementRole.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "EngagementRole.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Assign a user specified by id to the role specified by name.",
                "m365 viva engage role member add --userId 7a2ca997-9461-402e-9882-58088a370889 --roleName 'Verified Admin'",
            ),
            (
                "Assign a user specified by UPN to the role specified by id.",
                "m365 viva engage role member add --userName john.doe@contoso.com --roleId 77aa47ad-96fe-4ecc-8024-fd1ac5e28f17",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage role member list",
        summary: "Lists all users assigned to a Viva Engage role",
        options: &[
            ("roleId", "The id of the Viva Engage role."),
            ("roleName", "The name of the Viva Engage role."),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "EngagementRole.Read.All"),
            ("Application", "Microsoft Graph", "EngagementRole.Read.All"),
        ],
        examples: &[
            (
                "Get all users assigned to a Viva Engage role specified by id",
                "m365 viva engage role member list --roleId 77aa47ad-96fe-4ecc-8024-fd1ac5e28f17",
            ),
            (
                "Get all users assigned to a Viva Engage role specified by name",
                "m365 viva engage role member list --roleName 'Verified Admin'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage role member remove",
        summary: "Removes a user from a Viva Engage role",
        options: &[
            ("roleId", "The id of the Viva Engage role."),
            ("roleName", "The name of the Viva Engage role."),
            (
                "userId",
                "The Microsoft Entra ID of the user to remove from the specified Viva Engage role.",
            ),
            (
                "userName",
                "The UPN of the user to remove from the specified Viva Engage role.",
            ),
            ("force", "Do not prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "EngagementRole.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "EngagementRole.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Revoke a user specified by id from a Viva Engage role specified by a name",
                "m365 viva engage role member remove --userId 7a2ca997-9461-402e-9882-58088a370889 --roleName 'Verified Admin'",
            ),
            (
                "Revoke a user specified by UPN from a Viva Engage role specified by id and do not prompt for confirmation",
                "m365 viva engage role member remove --userName john.doe@contoso.com --roleId 77aa47ad-96fe-4ecc-8024-fd1ac5e28f17 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage search",
        summary: "Returns a list of messages, users, topics and groups that match the specified query.",
        options: &[
            ("queryText", "The query for the search"),
            (
                "show",
                "Specifies the type of data to return when using --output text. Allowed values `summary`, `messages`, `users`, `topics`, `groups`.",
            ),
            (
                "limit",
                "Limits the results returned for each item category.",
            ),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "Using the `--show` option in JSON output is not supported. To filter JSON results, use either a JMESPath query or filter the data yourself after retrieving them.",
        ],
        permissions: &[],
        examples: &[
            (
                "Returns search result for the query `community`",
                "m365 viva engage search --queryText \"community\"",
            ),
            (
                "Returns groups that match `community`",
                "m365 viva engage search --queryText \"community\" --show \"groups\"",
            ),
            (
                "Returns topics that match `community`",
                "m365 viva engage search --queryText \"community\" --show \"topics\"",
            ),
            (
                "Returns the first 50 users who match the search query `nuborocks.onmicrosoft.com`",
                "m365 viva engage search --queryText \"nuborocks.onmicrosoft.com\" --show \"users\" --limit 50",
            ),
            (
                "Returns all search results for the query `community`. Stops at 1000 results.",
                "m365 viva engage search --queryText \"community\" --output json",
            ),
            (
                "Returns the search results for the query `community` and limits the results to the first 50 entries for each result category.",
                "m365 viva engage search --queryText \"community\" --output json --limit 50",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage user get",
        summary: "Retrieves the current user or searches for a user by ID or e-mail",
        options: &[
            ("id", "Retrieve a user by ID"),
            ("email", "Retrieve a user by e-mail"),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
            "All operations return a single user object. Operations executed with the `email` parameter return an array of user objects.",
        ],
        permissions: &[],
        examples: &[
            ("Returns the current user", "m365 viva engage user get"),
            (
                "Returns the user with the ID 1496550697",
                "m365 viva engage user get --id 1496550697",
            ),
            (
                "Returns an array of users matching the e-mail",
                "m365 viva engage user get --email john.smith@contoso.com",
            ),
            (
                "Returns an array of users matching the e-mail in JSON. The JSON output returns a full user object",
                "m365 viva engage user get --email john.smith@contoso.com --output json",
            ),
        ],
        links: &[],
    },
    Help {
        name: "viva engage user list",
        summary: "Returns users from the current network",
        options: &[
            ("groupId", "Returns users within a given group"),
            (
                "letter",
                "Returns users with usernames beginning with the given character",
            ),
            ("reverse", "Returns users in reverse sorting order"),
            ("limit", "Limits the users returned"),
            (
                "sortBy",
                "Returns users sorted by a number of messages or followers, instead of the default behavior of sorting alphabetically. Allowed values are `messages,followers`",
            ),
        ],
        notes: &[
            "In order to use this command, you need to grant the Microsoft Entra application used by the CLI for Microsoft 365 the permission to the Viva Engage API. To do this, execute the `cli consent --service VivaEngage` command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Returns all Viva Engage network users",
                "m365 viva engage user list",
            ),
            (
                "Returns all Viva Engage network users with usernames beginning with \"a\"",
                "m365 viva engage user list --letter a",
            ),
            (
                "Returns all Viva Engage network users sorted alphabetically in descending order",
                "m365 viva engage user list --reverse",
            ),
            (
                "Returns the first 10 Viva Engage network users within the group 5785177",
                "m365 user list --groupId 5785177 --limit 10",
            ),
        ],
        links: &[],
    },
];

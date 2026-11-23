//! Help for the `m365 pa ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "pa app consent set",
        summary: "Configures if users can bypass the API Consent window for the selected canvas app",
        options: &[
            ("environmentName", "The name of the environment."),
            ("name", "The name of the Power App to update"),
            (
                "bypass",
                "Set to `true` to allow users to bypass the API Consent window. Set to `false` to disable the bypass.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &["This command only works for canvas apps."],
        permissions: &[],
        examples: &[
            (
                "Enables the bypass for the specified canvas app",
                "m365 pa app consent set --environmentName 4be50206-9576-4237-8b17-38d8aadfaa36 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --bypass true",
            ),
            (
                "Disables the bypass consent for the specified canvas app",
                "m365 pa app consent set --environmentName 4be50206-9576-4237-8b17-38d8aadfaa36 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --bypass false --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app export",
        summary: "Exports the specified Power App",
        options: &[
            ("name", "The name (GUID) of the Power Apps app to export."),
            (
                "environmentName",
                "The name of the environment for which to export the app.",
            ),
            (
                "packageDisplayName",
                "The display name to use in the exported package.",
            ),
            (
                "packageDescription",
                "The description to use in the exported package.",
            ),
            (
                "packageCreatedBy",
                "The name of the person to be used as the creator of the exported package.",
            ),
            (
                "packageSourceEnvironment",
                "The name of the source environment from which the exported package was taken.",
            ),
            (
                "path",
                "The path to save the exported package to. If not specified the app will be exported in the current working directory.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Export the specified Power App as a ZIP file",
                "m365 pa app export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Export the specified Power App as a ZIP file with a custom package name",
                "m365 pa app export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --packageDisplayName \"Assets app\"",
            ),
            (
                "Export the specified Power App as a ZIP file with the package displayname, package description, the one who created it, the package source environment and the path",
                "m365 pa app export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --packageDisplayName \"Assets app\" --packageDescription \"App to track assets of people\" --packageCreatedBy \"John Doe\" --packageSourceEnvironment \"Contoso\" --path \"C:/Users/John/Documents\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app get",
        summary: "Gets information about the specified Microsoft Power App",
        options: &[
            (
                "name",
                "The name of the Microsoft Power App to get information about.",
            ),
            (
                "displayName",
                "The display name of the Microsoft Power App to get information about.",
            ),
            (
                "asAdmin",
                "Run the command as admin for apps you don't have permission to.",
            ),
            (
                "environmentName",
                "The name of the environment. Required when using `asAdmin`.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
            "As a maker, you are able to retrieve the Power Apps you have permission to. As an administrator, you are also able to retrieve Power Apps from other users you don't have permission to. To get the app from another user, use the `asAdmin` option and make sure to specify the `environment` option as well.",
            "If you try to retrieve a non-existing Microsoft Power App, you will get the `Request failed with status code 404` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the specified Microsoft Power App by the app's name.",
                "m365 pa app get --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Get information about the specified Microsoft Power App by the app's display name.",
                "m365 pa app get --displayName App",
            ),
            (
                "Get information about the specific app you do not have access to.",
                "m365 pa app get --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app list",
        summary: "Lists all Power Apps apps",
        options: &[
            (
                "environmentName",
                "The name of the environment for which to retrieve available apps.",
            ),
            (
                "asAdmin",
                "Set, to list all Power Apps as admin. Otherwise will return only your own apps.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
            "If the environment with the name you specified doesn't exist, you will get the `Access to the environment 'xyz' is denied.` error.",
            "By default, the `app list` command returns only your apps. To list all apps, use the `asAdmin` option and make sure to specify the `environment` option. You cannot specify only one of the options, when specifying the `environment` option the `asAdmin` option has to be present as well.",
        ],
        permissions: &[],
        examples: &[
            ("List all your apps.", "m365 pa app list"),
            (
                "List all apps in a given environment.",
                "m365 pa app list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app owner set",
        summary: "Sets a new owner for a Power Apps app",
        options: &[
            ("environmentName", "The name of the environment."),
            ("appName", "The name (GUID) of the Microsoft Power App."),
            ("userId", "The Microsoft Entra ID of the new owner."),
            ("userName", "The user principal name of the new owner."),
            (
                "roleForOldAppOwner",
                "Grant permissions for the previous owner. Allowed values: `CanView`, `CanEdit`. Don't specify a value to remove the previous owner from the app.",
            ),
        ],
        notes: &["To use this command you must be a Global or Power Platform admin."],
        permissions: &[],
        examples: &[
            (
                "Set a new owner for a Power Apps app and make the previous owner co-owner",
                "m365 pa app owner set --environmentName Default-716eb9fb-bf79-4be9-b0c7-900824fcbe25 --appName 7ab97923-4a4d-4467-b030-12071d2b810b --userId c6ee6ed4-0172-4fdc-87a3-cfd9c324de1d --roleForOldAppOwner CanEdit",
            ),
            (
                "Set a new owner for a Power Apps app and remove the previous owner",
                "m365 pa app owner set --environmentName Default-716eb9fb-bf79-4be9-b0c7-900824fcbe25 --appName 7ab97923-4a4d-4467-b030-12071d2b810b --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app permission ensure",
        summary: "Assigns/updates permissions to a Power Apps app",
        options: &[
            ("appName", "The name (GUID) of the Microsoft Power App."),
            (
                "roleName",
                "Permission level given to the app. Valid values: `CanEdit`, `CanView`. Sharing with the entire tenant is only supported with `CanView` role.",
            ),
            ("userId", "The ID of the Microsoft Entra user."),
            ("userName", "The user principal name of the user."),
            ("groupId", "The ID of the Microsoft Entra group."),
            ("groupName", "The name of the Microsoft Entra group."),
            (
                "tenant",
                "Share the app with the entire tenant. When specifying this, only `CanView` role is supported.",
            ),
            (
                "sendInvitationMail",
                "When set, an invitation mail will be sent.",
            ),
            (
                "asAdmin",
                "Run the command as admin for apps you don't own.",
            ),
            (
                "environmentName",
                "The name of the environment. Specify only a value when running the command as admin.",
            ),
        ],
        notes: &[
            "When specifying a value for `groupId` or `groupName`, note that you can only grant permissions to security groups.",
        ],
        permissions: &[],
        examples: &[
            (
                "Share a Power App app with another user with run-only permissions",
                "m365 pa app permission ensure --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --userName john.doe@contoso.com --roleName CanView --sendInvitationMail",
            ),
            (
                "Add co-owner permissions for a security group to a Power Apps app",
                "m365 pa app permission ensure --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --groupName Developers --roleName CanEdit",
            ),
            (
                "Add permissions as admin to a Power App app that you do not own",
                "m365 pa app permission ensure --environmentName Default-e535e809-c6a1-47b5-8951-d0898a64bf8a --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --userId 42a28ec6-972f-4fdd-b483-50ee4b237f91 --roleName CanEdit --asAdmin",
            ),
            (
                "Share a Power Apps app with everyone in the tenant",
                "m365 pa app permission ensure --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --roleName CanView --tenant",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app permission list",
        summary: "Lists all permissions of a Power Apps app",
        options: &[
            ("appName", "The name (GUID) of the Microsoft Power App."),
            (
                "roleName",
                "Filter the results to only a given role: `Owner`, `CanEdit`, `CanView`.",
            ),
            (
                "asAdmin",
                "Run the command as admin for apps you don't own.",
            ),
            (
                "environmentName",
                "The name of the environment. Required when using `asAdmin`.",
            ),
        ],
        notes: &[
            "When you specify a value for `roleName`, consider the following:",
            "- `Owner` shows all the owners of the application.",
            "- `CanEdit` shows all the co-owners of the application.",
            "- `CanView` shows all the users with run only permissions.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all permissions for an app you have access to",
                "m365 pa app permission list --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0",
            ),
            (
                "List all permissions as admin for an app you don't have access to",
                "m365 pa app permission list --environmentName \"Default-e448e34a-40cc-441a-94c6-ec701a9a9ec2\" --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --asAdmin",
            ),
            (
                "List all co-owners for an app you have access to",
                "m365 pa app permission list --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --roleName CanEdit",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app permission remove",
        summary: "Removes permissions to a Power Apps app",
        options: &[
            ("appName", "The name (GUID) of the Microsoft Power App."),
            ("userId", "The ID of the Microsoft Entra user."),
            ("userName", "The user principal name of the user."),
            ("groupId", "The ID of the Microsoft Entra group."),
            ("groupName", "The name of the Microsoft Entra group."),
            (
                "tenant",
                "When specifying this, tenant permissions (everyone in the tenant) will be removed.",
            ),
            (
                "asAdmin",
                "Run the command as admin for apps you don't own.",
            ),
            (
                "environmentName",
                "The name of the environment. Specify only a value when running the command as admin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove permissions of a specific user for a specific app",
                "m365 pa app permission remove --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --userName john.doe@contoso.com",
            ),
            (
                "Remove permissions of a specific security group for a specific app",
                "m365 pa app permission remove --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --groupName Developers",
            ),
            (
                "Removes permissions as administrator for an app which you do not own",
                "m365 pa app permission remove --environmentName Default-e535e809-c6a1-47b5-8951-d0898a64bf8a --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --userId 42a28ec6-972f-4fdd-b483-50ee4b237f91 --asAdmin",
            ),
            (
                "Unshare a Power Apps app with the entire tenant",
                "m365 pa app permission remove --appName bc9f0a7e-53df-46af-b669-5888bb2f63d0 --tenant",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa app remove",
        summary: "Removes the specified Power App",
        options: &[
            ("name", "The name of the Power App to remove"),
            ("force", "Don't prompt for confirmation"),
            (
                "asAdmin",
                "Run the command as admin for apps you don't own.",
            ),
            (
                "environmentName",
                "The name of the environment. Required when using `asAdmin`.",
            ),
        ],
        notes: &[
            "By default, the command will try to remove a Power App. As maker, you are able to delete the Power Apps you own. As administrator, you are also able to delete Power Apps from other users.",
            "To remove an app you do not own, use the `asAdmin` option and make sure to specify the `environmentName` option as well.",
            "To remove a model-driven Power App you need administrator permissions.",
            "If the Power App with the name you specified doesn't exist, you will get the `Error: App 'abc' does not exist` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the specified Power App",
                "m365 pa app remove --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d",
            ),
            (
                "Removes the specified Power App without confirmation",
                "m365 pa app remove --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --force",
            ),
            (
                "Removes the specified Power App you don't own",
                "m365 pa app remove --name 3989cb59-ce1a-4a5c-bb78-257c5c39381d --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa connector export",
        summary: "Exports the specified Power Automate or Power Apps custom connector",
        options: &[
            (
                "environmentName",
                "The name of the environment where the custom connector to export is located",
            ),
            ("name", "The name of the custom connector to export"),
            (
                "outputFolder",
                "Path where the folder with connector's files should be saved. If not specified, will create the connector's folder in the current folder.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "If no output folder has been specified, the `pa connector export` command will create a folder named after the connector in the current folder. If the output folder has been specified, the folder named after the connector will be created in that folder.",
        ],
        permissions: &[],
        examples: &[
            (
                "Export the specified custom connector",
                "m365 pa connector export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name shared_connector-201-5f20a1f2d8d6777a75-5fa602f410652f4dfa",
            ),
            (
                "Export the specified custom connector to the specific directory",
                "m365 pa connector export --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name shared_connector-201-5f20a1f2d8d6777a75-5fa602f410652f4dfa --outputFolder connector",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa connector list",
        summary: "Lists custom connectors in the given environment",
        options: &[(
            "environmentName",
            "The name of the environment for which to retrieve custom connectors.",
        )],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[(
            "List all custom connectors in the given environment.",
            "m365 pa connector list --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
        )],
        links: &[],
    },
    Help {
        name: "pa environment get",
        summary: "Gets information about the specified Power Apps environment",
        options: &[
            (
                "name",
                "The name of the environment. When not specified, the default environment is retrieved. Specify either `name` or `default`, but not both.",
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
                "Get information about the default Power Apps environment.",
                "m365 pa environment get --default",
            ),
            (
                "Get information about the Power Apps environment with the specified name.",
                "m365 pa environment get --name Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pa environment list",
        summary: "Lists Microsoft Power Apps environments in the current tenant",
        options: &[],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[(
            "List Microsoft Power Apps environments in the current tenant.",
            "m365 pa environment list",
        )],
        links: &[],
    },
];

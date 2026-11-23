//! Help for the `m365 app ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "app get",
        summary: "Retrieves information about the current Microsoft Entra app",
        options: &[(
            "appId",
            "Client ID of the Microsoft Entra app registered in the .m365rc.json file to retrieve information for.",
        )],
        notes: &[
            "Use this command to quickly look up information for the Microsoft Entra application registration registered in the .m365rc.json file in your current project (folder).",
            "If you have multiple apps registered in your .m365rc.json file, you can specify the app for which you'd like to retrieve permissions using the `--appId` option. If you don't specify the app using the `--appId` option, you'll be prompted to select one of the applications from your .m365rc.json file.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieve information about your current Microsoft Entra app.",
                "m365 app get",
            ),
            (
                "Retrieve information about the Microsoft Entra app with client ID specified in the .m365rc.json file.",
                "m365 app get --appId e23d235c-fcdf-45d1-ac5f-24ab2ee0695d",
            ),
        ],
        links: &[],
    },
    Help {
        name: "app open",
        summary: "Opens Microsoft Entra app in the Microsoft Entra ID portal",
        options: &[
            (
                "appId",
                "Optional Application (client) ID of the Microsoft Entra application registration to open. Uses the app from the `.m365rc.json` file corresponding to the `appId`. If multiple apps are available, this will evade the prompt to choose an app. If the `appId` is not available in the list of apps, an error is thrown.",
            ),
            (
                "preview",
                "Use to open the url of the Microsoft Entra ID preview portal.",
            ),
        ],
        notes: &[
            "If config setting `autoOpenLinksInBrowser` is configured to true, the command will automatically open the link to the Azure Portal in the browser.",
            "Gets the app from the `.m365rc.json` file in the current directory. If the `--appId` option is not used and multiple apps are available, it will prompt the user to choose one.",
        ],
        permissions: &[],
        examples: &[
            (
                "Prints the URL to the Microsoft Entra application registration management page on the Azure Portal.",
                "m365 app open",
            ),
            (
                "Prints the url of the Microsoft Entra application registration management page on the preview Azure Portal.",
                "m365 app open --preview",
            ),
            (
                "Prints the URL to the Microsoft Entra application registration management page on the Azure Portal, evading a possible choice prompt in the case of multiple saved apps in the `.m365rc.json` file.",
                "m365 app open --appId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
        ],
        links: &[],
    },
    Help {
        name: "app permission add",
        summary: "Adds the specified application and/or delegated permissions to the current Microsoft Entra app API permissions",
        options: &[
            (
                "appId",
                "Client ID of the Microsoft Entra app registered in the .m365rc.json file to retrieve API permissions for.",
            ),
            (
                "applicationPermissions",
                "Space-separated list of application permissions to add.",
            ),
            (
                "delegatedPermissions",
                "Space-separated list of delegated permissions to add.",
            ),
            (
                "grantAdminConsent",
                "If specified, admin consent to the newly added permissions will be granted.",
            ),
        ],
        notes: &[
            "If you have multiple apps registered in your .m365rc.json file, you can specify the app for which you'd like to retrieve permissions using the `--appId` option. If you don't specify the app using the `--appId` option, you'll be prompted to select one of the applications from your .m365rc.json file.",
        ],
        permissions: &[],
        examples: &[
            (
                "Adds the specified application permissions to the default app registered in the .m365rc.json file while granting admin consent.",
                "m365 app permission add --applicationPermissions 'https://graph.microsoft.com/User.ReadWrite.All https://graph.microsoft.com/User.Read.All' --grantAdminConsent",
            ),
            (
                "Adds the specified delegated permissions to the default app registered in the .m365rc.json file without granting admin consent.",
                "m365 app permission add --delegatedPermissions 'https://graph.microsoft.com/offline_access'",
            ),
            (
                "Adds the specified application and delegated permissions to a specific app registered in the .m365rc.json file while granting admin consent.",
                "m365 app permission add --appId '1663767b-4172-4519-bfd1-28e6ff19055b' --applicationPermissions 'https://graph.microsoft.com/User.ReadWrite.All https://graph.microsoft.com/User.Read.All' --delegatedPermissions 'https://graph.microsoft.com/offline_access' --grantAdminConsent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "app permission list",
        summary: "Lists API permissions for the current Microsoft Entra app",
        options: &[(
            "appId",
            "Client ID of the Microsoft Entra app registered in the .m365rc.json file to retrieve API permissions for.",
        )],
        notes: &[
            "Use this command to quickly look up API permissions for the Microsoft Entra application registration registered in the .m365rc.json file in your current project (folder).",
            "If you have multiple apps registered in your .m365rc.json file, you can specify the app for which you'd like to retrieve permissions using the `--appId` option. If you don't specify the app using the `--appId` option, you'll be prompted to select one of the applications from your .m365rc.json file.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieve API permissions for your current Microsoft Entra app.",
                "m365 app permission list",
            ),
            (
                "Retrieve API permissions for the Microsoft Entra app with the client ID specified in the .m365rc.json file.",
                "m365 app permission list --appId e23d235c-fcdf-45d1-ac5f-24ab2ee0695d",
            ),
        ],
        links: &[],
    },
];

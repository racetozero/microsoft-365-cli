//! Help for the top-level commands (login, status, request, ...).

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "docs",
        summary: "Returns the CLI for Microsoft 365 docs webpage URL",
        options: &[],
        notes: &[
            "Configure `autoOpenLinksInBrowser` using cli config set to automatically open the webpage in the default browser.",
        ],
        permissions: &[],
        examples: &[(
            "Returns the CLI for Microsoft 365 docs webpage URL",
            "m365 docs",
        )],
        links: &[],
    },
    Help {
        name: "login",
        summary: "Logs in to Microsoft 365",
        options: &[
            (
                "appId",
                "App ID of the Microsoft Entra application to use for authentication. This option is crucial and should be specified if it isn't defined elsewhere.",
            ),
            (
                "tenant",
                "ID of the tenant from which accounts should authenticate. Use `common` or `organization` for multitenant apps. Defaults to `common` if not specified and if the config value `tenantId` and the environment variable `CLIMICROSOFT365_TENANT` are also not set.",
            ),
            (
                "authType",
                "The type of authentication to use. Allowed values `certificate`, `deviceCode`, `password`, `identity`, `federatedIdentity`, `browser`, `secret`. Default `deviceCode`",
            ),
            (
                "userName",
                "Name of the user to authenticate. Required when `authType` is set to `password`",
            ),
            (
                "password",
                "Password for the user or the certificate. Required when `authType` is set to `password`, or when `authType` is set to `certificate` and the provided certificate requires a password to open",
            ),
            (
                "certificateFile",
                "Path to the file with certificate private key. When `authType` is set to `certificate`, specify either `certificateFile` or `certificateBase64Encoded`",
            ),
            (
                "certificateBase64Encoded",
                "Base64-encoded string with certificate private key. When `authType` is set to `certificate`, specify either `certificateFile` or `certificateBase64Encoded`",
            ),
            (
                "thumbprint",
                "Certificate thumbprint. If not specified, and `authType` is set to `certificate`, it will be automatically calculated based on the specified certificate",
            ),
            (
                "secret",
                "Client Secret of the Microsoft Entra application to use for authentication. Required when `authType` is set to `secret`.",
            ),
            (
                "cloud",
                "Cloud to connect to. Allowed values `Public`, `USGov`, `USGovHigh`, `USGovDoD` and `China`. Default `Public`",
            ),
            (
                "connectionName",
                "Specify an optional name to make switching between connections easier.",
            ),
            (
                "ensure",
                "Ensures that the user is signed in. if the user isn't signed in, it initiates the login flow",
            ),
        ],
        notes: &[
            "The CLI determines the `appId` and `tenant` values in the following order of precedence:",
            "- The `appId` or `tenant` option specified during login.",
            "- The `clientId` or `tenantId` value set in the configuration.",
            "- The `CLIMICROSOFT365_ENTRAAPPID` or `CLIMICROSOFT365_TENANT` environment variable.",
            "For `appId`, it is required that at least one of the options is specified during login.",
            "For `tenant`, if none are specified, the CLI will default to `common`.",
            "Using the `login` command you can log in to Microsoft 365.",
            "By default, the `login` command uses device code OAuth flow to log in to Microsoft 365. Alternatively, you can authenticate using a user name and password or certificate, which are convenient for CI/CD scenarios, but which come with their own limitations. The default `authType` can be configured using `m365 cli config set`. This means you'll be able to run `m365 login` without specifying the `--authType` option.",
            "When logging in to Microsoft 365 using the user name and password, next to the access and refresh token, the CLI for Microsoft 365 will store the user credentials so that it can automatically re-authenticate if necessary. Similarly to the tokens, the credentials are removed by re-authenticating using the device code or by calling the logout command.",
            "When logging in to Microsoft 365 using a certificate, the CLI for Microsoft 365 will store the contents of the certificate so that it can automatically re-authenticate if necessary. The contents of the certificate are removed by re-authenticating using the device code or by calling the logout command.",
            "Federated Identity is currently supported in GitHub Actions, Azure DevOps and GitLab CI/CD. To use this you need to set `authType` to `federatedIdentity` and specify `appId` and `tenant` to the values of the app registration you've configured a federated credential on.",
            "To use Federated Identity in GitLab CI/CD, the job must request an ID token named `GITLAB_OIDC_TOKEN` with audience `api://AzureADTokenExchange`, for example:",
            "    job:\n      id_tokens:\n        GITLAB_OIDC_TOKEN:\n          aud: \"api://AzureADTokenExchange\"\n      script:\n        - m365 login --authType federatedIdentity --appId eb96cfd0-abfa-4477-8d1d-c6dfde3efb19 --tenant 9a1a1bbd-e158-4cf5-967d-a53b8e85c628",
            "The corresponding federated credential on the Microsoft Entra app registration must have issuer `https://gitlab.com` (or your self-managed GitLab instance URL) and a subject matching the GitLab OIDC claim for the job, for example `project_path:/:ref_type:branch:ref:`.",
            "To log in to Microsoft 365 using a certificate or secret, you will typically create a custom Microsoft Entra application. To use this application with the CLI for Microsoft 365, you will set the `CLIMICROSOFT365_ENTRAAPPID` environment variable to the application's ID and the `CLIMICROSOFT365_TENANT` environment variable to the ID of the Microsoft Entra tenant, where you created the Microsoft Entra application. Also, please make sure to read about the caveats when using the certificate login option.",
            "Managed identity in Azure Cloud Shell is the identity of the user. It is neither system- nor user-assigned and it can't be configured. To log in to Microsoft 365 using managed identity in Azure Cloud Shell, set `authType` to `identity` and don't specify the `userName` option.",
            "When connecting to clouds other than `Public`, you'll need to use a Microsoft Entra application registered in a directory provisioned in that cloud. If you try to login using the default Microsoft Entra application, login will fail.",
            "When signing in with multiple identities, every signin will be saved as a connection. You can list available connections using m365 connection list and switch between connections using m365 connection use",
        ],
        permissions: &[],
        examples: &[
            (
                "Log in to Microsoft 365 using the device code.",
                "m365 login --appId 31359c7f-bd7e-475c-86db-fdb8c937548e",
            ),
            (
                "Log in to Microsoft 365 using the device code and set the connection name.",
                "m365 login --connectionName 'myworkaccount' --appId 31359c7f-bd7e-475c-86db-fdb8c937548e",
            ),
            (
                "Log in to Microsoft 365 using your own Microsoft Entra application that's restricted only to allow accounts from the specific tenant.",
                "m365 login --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a",
            ),
            (
                "Log in to Microsoft 365 using your own Microsoft Entra application and a personal information exchange (.pfx) file.",
                "m365 login --authType certificate --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --certificateFile /Users/user/dev/localhost.pfx --password 'pass@word1'",
            ),
            (
                "Log in to Microsoft 365 using a user name and password by providing the application and tenant identifiers explicitly.",
                "m365 login --authType password --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --userName user@contoso.com --password pass@word1",
            ),
            (
                "Log in to Microsoft 365 using a PEM certificate with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pem",
            ),
            (
                "Log in to Microsoft 365 using a PEM certificate and a specific thumbprint with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pem --thumbprint 47C4885736C624E90491F32B98855AA8A7562AF1",
            ),
            (
                "Log in to Microsoft 365 using a PEM certificate by providing the application and tenant identifiers explicitly.",
                "m365 login --authType certificate --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --certificateFile /Users/user/dev/localhost.pem",
            ),
            (
                "Log in to Microsoft 365 using a personal information exchange (.pfx) file with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pfx --password 'pass@word1'",
            ),
            (
                "Log in to Microsoft 365 using a personal information exchange (.pfx) file protected with an empty password with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pfx --password",
            ),
            (
                "Log in to Microsoft 365 using a personal information exchange (.pfx) file not protected with a password with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pfx",
            ),
            (
                "Log in to Microsoft 365 using a personal information exchange (.pfx) file and a specific thumbprint with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateFile /Users/user/dev/localhost.pfx --thumbprint 47C4885736C624E90491F32B98855AA8A7562AF1 --password 'pass@word1'",
            ),
            (
                "Log in to Microsoft 365 using a personal information exchange (.pfx) file by providing the application and tenant identifiers explicitly.",
                "m365 login --authType certificate --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --certificateFile /Users/user/dev/localhost.pfx --password 'pass@word1'",
            ),
            (
                "Log in to Microsoft 365 using a certificate from a base64-encoded string with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType certificate --certificateBase64Encoded MIII2QIBAzCCCJ8GCSqGSIb3DQEHAaCCCJAEg...eX1N5AgIIAA== --thumbprint D0C9B442DE249F55D10CDA1A2418952DC7D407A3",
            ),
            (
                "Log in to Microsoft 365 using a certificate from a base64-encoded string by providing the application and tenant identifiers explicitly.",
                "m365 login --authType certificate --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --certificateBase64Encoded MIII2QIBAzCCCJ8GCSqGSIb3DQEHAaCCCJAEg...eX1N5AgIIAA== --thumbprint D0C9B442DE249F55D10CDA1A2418952DC7D407A3",
            ),
            (
                "Log in to Microsoft 365 using a system assigned managed identity with `clientId` set in the configuration. Applies to Azure resources with managed identity enabled, such as Azure Virtual Machines, Azure App Service or Azure Functions.",
                "m365 login --authType identity",
            ),
            (
                "Log in to Microsoft 365 using managed identity in Azure Cloud Shell with `clientId` set in the configuration. Uses the identity of the current user.",
                "m365 login --authType identity",
            ),
            (
                "Log in to Microsoft 365 using a user-assigned managed identity with `clientId` set in the configuration. Client id or principal id also known as object id value can be specified in the `userName` option. Applies to Azure resources with managed identity enabled, such as Azure Virtual Machines, Azure App Service or Azure Functions.",
                "m365 login --authType identity --userName ac9fbed5-804c-4362-a369-21a4ec51109e",
            ),
            (
                "Log in to Microsoft 365 using a federated identity. Can currently only be used in GitHub Actions, Azure DevOps and GitLab CI/CD.",
                "m365 login --authType federatedIdentity --appId eb96cfd0-abfa-4477-8d1d-c6dfde3efb19 --tenant 9a1a1bbd-e158-4cf5-967d-a53b8e85c628",
            ),
            (
                "Log in to Microsoft 365 using the interactive browser authentication with `clientId` set in the configuration. Uses the identity of the current user.",
                "m365 login --authType browser",
            ),
            (
                "Log in to Microsoft 365 using a client secret with `CLIMICROSOFT365_ENTRAAPPID` and `CLIMICROSOFT365_TENANT` environment variables set.",
                "m365 login --authType secret --secret topSeCr3t@007",
            ),
            (
                "Log in to Microsoft 365 using a client secret by providing the application and tenant identifiers explicitly.",
                "m365 login --authType secret --appId 31359c7f-bd7e-475c-86db-fdb8c937548c --tenant 31359c7f-bd7e-475c-86db-fdb8c937548a --secret topSeCr3t@007",
            ),
            (
                "Ensures that the user is signed in, initiates the login flow if the user isn't signed in",
                "m365 login --ensure",
            ),
        ],
        links: &[],
    },
    Help {
        name: "logout",
        summary: "Logs out from Microsoft 365",
        options: &[],
        notes: &[
            "The `logout` command logs out from Microsoft 365 and removes any access and refresh tokens from memory",
        ],
        permissions: &[],
        examples: &[
            ("Log out from Microsoft 365", "m365 logout"),
            (
                "Log out from Microsoft 365 in debug mode including detailed debug information in the console output",
                "m365 logout --debug",
            ),
        ],
        links: &[],
    },
    Help {
        name: "request",
        summary: "Executes the specified web request using CLI for Microsoft 365",
        options: &[
            ("url", "The request URL."),
            (
                "method",
                "The HTTP request method. Accepted values are `get`, `post`, `put`, `patch`, `delete`, `head`, `options`. The default value is `get`.",
            ),
            (
                "resource",
                "The resource uri for which the CLI should acquire a token from Microsoft Entra ID in order to access the service.",
            ),
            (
                "body",
                "The request body. Optionally use `@example.json` to load the body from a file.",
            ),
            (
                "filePath",
                "The file path to save the response to. This option can be used when downloading files.",
            ),
        ],
        notes: &[
            "When executing a request, CLI will take care of the very basic configuration, and you'll need to specify all additional information, such as headers, method and body. CLI will take care for you of:",
            "- applying compression and handling throttling,",
            "- setting the `accept` to `application/json` if you don't specify it yourself,",
            "- setting the `authorization` header to the bearer token obtained for the resource determined from the request URL",
            "If you specify the `resource` option, the CLI will try to retrieve a valid token for the resource instead of determining the resource based on the url. The value doesn't have to be a URL. It can be also a URI like `app://`.",
            "Specify additional headers by typing them as options, for example: `--content-type \"application/json\"`, `--if-match \"*\"`, `--x-requestdigest \"somedigest\"`.",
            "You can simplify the request by using tokens for the `url` option. Following tokens are available:",
            "- `@graph` resolves to `https://graph.microsoft.com/v1.0`",
            "- `@graphbeta` resolves to `https://graph.microsoft.com/beta`",
            "- `@spo` resolves to the current SharePoint URL if available. You can set a SharePoint URL using the spo set command.",
            "When building the request, depending on the shell you use, you might need to escape all `$` characters in the URL, request headers, and the body. If you don't do it, the shell will treat it as a variable and will remove the following word from the request, breaking the request.",
        ],
        permissions: &[],
        examples: &[
            (
                "Call the SharePoint Rest API using a GET request with a constructed URL containing expands, filters and selects.",
                "m365 request --url \"https://contoso.sharepoint.com/sites/project-x/_api/web/siteusers?\\$filter=IsShareByEmailGuestUser eq true&\\$expand=Groups&\\$select=Title,LoginName,Email,Groups/LoginName\" --accept \"application/json;odata=nometadata\"",
            ),
            (
                "Call the Microsoft Graph beta endpoint using a GET request.",
                "m365 request --url \"https://graph.microsoft.com/beta/me\"",
            ),
            (
                "Call the SharePoint API to retrieve a form digest.",
                "m365 request --method post --url \"https://contoso.sharepoint.com/sites/project-x/_api/contextinfo\"",
            ),
            (
                "Call the SharePoint API to update a site title using the @spo token.",
                "m365 request --method post --url \"@spo/sites/project-x/_api/web\" --body '{ \"Title\": \"New title\" }' --content-type \"application/json\" --x-http-method \"PATCH\"",
            ),
            (
                "Call the Microsoft Graph to get a profile photo using the @graphbeta token.",
                "m365 request --url '@graphbeta/me/photo/$value' --filePath ./profile-pic.jpg",
            ),
        ],
        links: &[],
    },
    Help {
        name: "search",
        summary: "Uses the Microsoft Search to query Microsoft 365 data",
        options: &[
            (
                "queryText",
                "Query to be executed in Keyword Query Language (KQL) format.",
            ),
            (
                "scopes",
                "Comma-separated list of scopes for the search request. Allowed values are `chatMessage`, `message`, `event`, `drive`, `driveItem`, `list`, `listItem`, `site`, `bookmark`, `acronym`, or `person`.",
            ),
            (
                "startIndex",
                "Set the number that indicates the 0-based starting point to list search results on the page. The default value is 0.",
            ),
            (
                "pageSize",
                "Set the number that indicates the number of results to be returned for a page. The default is 25 results. The maximum is 500 results.",
            ),
            (
                "allResults",
                "Set to get all search results from all pages.",
            ),
            (
                "resultsOnly",
                "Set to get only the results form the query response.",
            ),
            (
                "enableTopResults",
                "Use to get the first three messages in the response are sorted by relevance. Use this option only when scopes is set to message type.",
            ),
            (
                "select",
                "Comma-separated list of properties to return in the search results.",
            ),
            (
                "sortBy",
                "Comma-separated list of properties to sort search results.",
            ),
            (
                "enableSpellingSuggestion",
                "Set to get the suggested spelling correction information for typos in the user query.",
            ),
            (
                "enableSpellingModification",
                "Set to get the search results for the corrected query when there are no results for the original query with typos, and get the corresponding correction information.",
            ),
        ],
        notes: &[
            "The `queryText` option supports Keyword Query Language (KQL).",
            "The `sortBy` option allows to specify the order. By default, the order is ascending. If you want to apply descending order, add `:desc` to the property name, like `createdDateTime:desc`.",
        ],
        permissions: &[],
        examples: &[
            (
                "Queries messages in the current user mailbox that contain the string \"contoso\" in any part of the message (the sender name, subject, message body, or any attachments). The query returns the first 100 results.",
                "m365 search --scopes 'message' --queryText 'contoso' --pageSize 100",
            ),
            (
                "Queries list items and sites that contain the string \"accountt\" and requests a spelling modification for the query.",
                "m365 search --scopes 'listItem, site' --queryText 'accountt' --enableSpellingSuggestion --enableSpellingModification",
            ),
            (
                "Search for file report.xlsl in specific folder of user personal OneDrive",
                "m365 search --scopes 'driveItem' --queryText 'filename:report.xlsl AND path:\\\"https://contoso-my.sharepoint.com/personal/john.doe_contoso_com/Documents/Reports/2024\\\"'",
            ),
            (
                "Search for all sites",
                "m365 search --scopes 'site' --queryText 'site:\\\"https://contoso-my.sharepoint.com/personal/*\\\"'",
            ),
        ],
        links: &[(
            "Use the Microsoft Search API to query data",
            "https://learn.microsoft.com/graph/api/resources/search-api-overview",
        )],
    },
    Help {
        name: "setup",
        summary: "Sets up CLI for Microsoft 365 based on your preferences",
        options: &[
            (
                "interactive",
                "Configure CLI for Microsoft 365 for interactive use without prompting for additional information.",
            ),
            (
                "scripting",
                "Configure CLI for Microsoft 365 for use in scripts without prompting for additional information.",
            ),
            (
                "skipApp",
                "Skip configuring an Entra app for use with CLI for Microsoft 365.",
            ),
        ],
        notes: &[
            "The `m365 setup` command is a wizard that helps you configure the CLI for Microsoft 365 for your needs. It will ask you a series of questions and based on your answers, it will configure the CLI for Microsoft 365 for you.",
            "The command will ask you the following questions:",
            "- CLI for Microsoft 365 requires a Microsoft Entra app. Do you want to create a new app registration or use an existing one?",
            "You can choose between using an existing Entra app or creating a new one. If you choose to create a new app, the CLI will ask you to choose between a minimal and a full set of permissions. It then signs in as Azure CLI to your tenant, creates a new app registration, and stores its information in the CLI configuration.",
            "- How do you plan to use the CLI?",
            "You can choose between interactive and scripting use. In interactive mode, the CLI for Microsoft 365 will prompt you for additional information when needed, automatically open links browser, automatically show help on errors. In scripting mode, the CLI will not use interactivity to prevent blocking your scripts.",
            "- Are you going to use the CLI in PowerShell? (asked only when you chose to configure CLI for scripting)",
            "To simplify using CLI in PowerShell, you can configure the CLI to output errors to stdout, instead of the default stderr which is tedious to handle in PowerShell.",
            "- How experienced are you in using the CLI?",
            "You can choose between beginner and proficient. If you're just starting working with the CLI, it will show you full help information. For more experienced users, it will only show information about commands' options. No matter how you configure this setting, you can always invoke full help by using `--help full`.",
            "After you answer these questions, the command will build a preset and ask you to confirm using it to configure the CLI.",
            "If you want to configure the CLI on a non-interactive system, or want to configure CLI without answering the questions, you can use the `--interactive` or `--scripting` options. When using these options, CLI will apply the correct presets. Additionally, it will detect if it's used in PowerShell and configure CLI accordingly.",
            "The `m365 setup` command uses the following presets:",
            "- interactive use:",
            "  - autoOpenLinksInBrowser: true,",
            "  - copyDeviceCodeToClipboard: true,",
            "  - output: 'text',",
            "  - printErrorsAsPlainText: true,",
            "  - prompt: true,",
            "  - showHelpOnFailure: true,",
            "- scripting use:",
            "  - autoOpenLinksInBrowser: false,",
            "  - copyDeviceCodeToClipboard: false,",
            "  - output: 'json',",
            "  - printErrorsAsPlainText: false,",
            "  - prompt: false,",
            "  - showHelpOnFailure: false,",
            "- use in PowerShell:",
            "  - errorOutput: 'stdout'",
            "- beginner:",
            "  - helpMode: 'full'",
            "- proficient:",
            "  - helpMode: 'options'",
        ],
        permissions: &[],
        examples: &[
            (
                "Configure CLI for Microsoft 365 based on your preferences interactively",
                "m365 setup",
            ),
            (
                "Configure CLI for Microsoft 365 for interactive use without prompting for additional information",
                "m365 setup --interactive",
            ),
            (
                "Configure CLI for Microsoft 365 for use in scripts without prompting for additional information",
                "m365 setup --scripting",
            ),
            (
                "Configure CLI for Microsoft 365 without setting up an Entra app",
                "m365 setup --skipApp",
            ),
        ],
        links: &[],
    },
    Help {
        name: "status",
        summary: "Shows Microsoft 365 login status",
        options: &[],
        notes: &[
            "If you are logged in to Microsoft 365, the `status` command will show you information about the user or application name used to sign in and the details about the stored refresh and access tokens and their expiration date and time when run in debug mode.",
        ],
        permissions: &[],
        examples: &[(
            "Show the information about the current login to the Microsoft 365",
            "m365 status",
        )],
        links: &[],
    },
    Help {
        name: "version",
        summary: "Shows CLI for Microsoft 365 version",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[("Show the version of CLI for Microsoft 365", "m365 version")],
        links: &[],
    },
];

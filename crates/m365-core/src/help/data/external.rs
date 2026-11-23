//! Help for the `m365 external ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "external connection add",
        summary: "Adds a new external connection",
        options: &[
            (
                "id",
                "Developer-provided unique ID of the connection within the Microsoft Entra tenant.",
            ),
            (
                "name",
                "The display name of the connection to be displayed in the Microsoft 365 admin center. Maximum length of 128 characters.",
            ),
            (
                "description",
                "Description of the connection displayed in the Microsoft 365 admin center.",
            ),
            (
                "authorizedAppIds",
                "Comma-separated collection of application IDs for registered Microsoft Entra apps that are allowed to manage the external connection and to index content in the external connection.",
            ),
        ],
        notes: &[
            "The `id` must be at least 3 and no more than 32 characters long. It can contain only alphanumeric characters, can't begin with Microsoft and can be any of the following values: None, Directory, Exchange, ExchangeArchive, LinkedIn, Mailbox, OneDriveBusiness, SharePoint, Teams, Yammer, Connectors, TaskFabric, PowerBI, Assistant, TopicEngine, MSFT_All_Connectors.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Adds a new external connection with name and description of test app.",
                "m365 external connection add --id MyApp --name \"Test\" --description \"Test\"",
            ),
            (
                "Adds a new external connection with a limited number of authorized apps.",
                "m365 external connection add --id MyApp --name \"Test\" --description \"Test\" --authorizedAppIds  \"00000000-0000-0000-0000-000000000000,00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000002\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "external connection doctor",
        summary: "Checks if the external connection is correctly configured for use with a specified user experience in Microsoft 365",
        options: &[
            ("id", "The ID of the external connection to check."),
            (
                "ux",
                "Microsoft 365 experience for which to check compatibility. Allowed values `copilot`, `search`, `all` (default)",
            ),
        ],
        notes: &[
            "The `external connection doctor` command runs several automated checks to verify if an external connection is correctly configured for use with a user experience in Microsoft 365.",
            "Check | ID | UX | Type | Description",
            "Required semantic labels|`copilotRequiredSemanticLabels`|Copilot|Required|Checks if the external connection schema has the required semantic labels configured: `title`, `url` and `iconUrl`",
            "Searchable properties | Copilot | `searchableProperties` | Required | Checks if the external connection schema has at least one searchable property",
            "Items have content ingested | `contentIngested` | Copilot | Required | Checks if external items have content ingested",
            "Connection configured for inline results | `enabledForInlineResults` | Copilot | Required (manual) | Check if the external connection is configured for inline results",
            "Items have activities recorded | `itemsHaveActivities` | Copilot | Recommended (manual) | Check if external items have activities recorded",
            "Meaningful connection name and description | `meaningfulNameAndDescription` | Copilot | Required (manual) | Check if the external connection has a meaningful name and description",
            "Semantic labels | `semanticLabels` | Search | Recommended | Checks if the external connection schema uses semantic labels",
            "Searchable properties | `searchableProperties` | Search | Recommended | Checks if the external connection schema has at least one searchable property",
            "Result types | `resultType` | Search | Recommended | Checks if the external connection has a result type configured",
            "Items have content ingested | `contentIngested` | Search | Recommended | Checks if external items have content ingested",
            "Items have activities recorded | `itemsHaveActivities` | Search | Recommended | Check if external items have activities recorded",
            "urlToItemResolver configured | `urlToItemResolver` | All | Recommended | Checks if the external connection has at least one urlToItemResolver configured",
            "Required checks must pass for the external connection to be compatible with the specified user experience. Recommended checks are optional, but recommended for optimal user experience.",
            "Some checks must be done manually, because there are no APIs available to verify the configuration automatically.",
            "When you check the compatibility with all UXs, and there are multiple checks with the same ID, the command will use the first matching check, following the order listed above.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy, ExternalItem.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy, ExternalItem.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[(
            "Checks if the external connection with the specified ID is correctly configured for use with Copilot for Microsoft 365.",
            "m365 external connection doctor --id contosoproducts --ux copilot",
        )],
        links: &[(
            "https://learn.microsoft.com/graph/connecting-external-content-experiences",
            "https://learn.microsoft.com/graph/connecting-external-content-experiences",
        )],
    },
    Help {
        name: "external connection get",
        summary: "Allows the administrator to get a specific external connection",
        options: &[
            ("id", "ID of the External Connection to get."),
            ("name", "Name of the External Connection to get."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Get the External Connection by its id.",
                "m365 external connection get --id \"MyApp\"",
            ),
            (
                "Get the External Connection by its name.",
                "m365 external connection get --name \"Test\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "external connection list",
        summary: "Lists external connections",
        options: &[],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[(
            "List external connections.",
            "m365 external connection list",
        )],
        links: &[],
    },
    Help {
        name: "external connection remove",
        summary: "Allows the administrator to remove a specific external connection",
        options: &[
            ("id", "ID of the External Connection to remove."),
            ("name", "Name of the External Connection to remove."),
            (
                "force",
                "Don't prompt for confirming removing the connection.",
            ),
        ],
        notes: &[
            "If the command finds multiple external connections with the specified name, it will prompt you to disambiguate which external connection it should remove, listing the discovered IDs.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Removes external connection by its id.",
                "m365 external connection remove --id \"MyApp\"",
            ),
            (
                "Removes external connection by its name. Will NOT prompt for confirmation before removing.",
                "m365 external connection remove --name \"Test\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "external connection schema add",
        summary: "Allows the administrator to add a schema to a specific external connection",
        options: &[
            ("externalConnectionId", "ID of the External Connection."),
            ("schema", "The schema object to be added."),
            (
                "wait",
                "Wait for the schema to be added before completing the command.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Adds a new schema to a specific external connection.",
                "m365 external connection schema add --externalConnectionId 'CliConnectionId' --schema '{\"baseType\":\"microsoft.graph.externalItem\",\"properties\":[{\"name\":\"ticketTitle\",\"type\":\"String\",\"isSearchable\":\"true\",\"isRetrievable\":\"true\",\"labels\":[\"title\"]},{\"name\":\"priority\",\"type\":\"String\",\"isQueryable\":\"true\",\"isRetrievable\":\"true\",\"isSearchable\":\"false\"},{\"name\":\"assignee\",\"type\":\"String\",\"isRetrievable\":\"true\"}]}'",
            ),
            (
                "Adds a new schema to a specific external connection and wait for its provisioning to complete.",
                "m365 external connection schema add --externalConnectionId 'CliConnectionId' --schema '{\"baseType\":\"microsoft.graph.externalItem\",\"properties\":[{\"name\":\"ticketTitle\",\"type\":\"String\",\"isSearchable\":\"true\",\"isRetrievable\":\"true\",\"labels\":[\"title\"]},{\"name\":\"priority\",\"type\":\"String\",\"isQueryable\":\"true\",\"isRetrievable\":\"true\",\"isSearchable\":\"false\"},{\"name\":\"assignee\",\"type\":\"String\",\"isRetrievable\":\"true\"}]}' --wait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "external connection urltoitemresolver add",
        summary: "Adds a URL to item resolver to an external connection",
        options: &[
            (
                "externalConnectionId",
                "The ID of the external connection to which to add the resolver",
            ),
            ("baseUrls", "Comma-separated list of base URLs"),
            ("urlPattern", "Regex pattern to match URLs"),
            (
                "itemId",
                "Pattern that matches captured matches from the URL with the external item ID",
            ),
            ("priority", "Integer that defines the resolution order"),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalConnection.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[(
            "Adds a URL to item resolver to an existing connection",
            "m365 external connection urltoitemresolver add --externalConnectionId \"samplesolutiongallery\" --baseUrls \"https://adoption.microsoft.com\" --urlPattern \"/sample-solution-gallery/sample/(?<sampleId>[^/]+)\" --itemId \"{sampleId}\" --priority 1",
        )],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/externalconnectors-activitysettings?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/externalconnectors-activitysettings?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "external item add",
        summary: "Creates external item",
        options: &[
            ("id", "The ID of the item to create"),
            (
                "externalConnectionId",
                "The ID of the external connection on which to create the item",
            ),
            ("content", "External item content"),
            (
                "contentType",
                "Type of content to load. Allowed values: `text` (default), `html`",
            ),
            (
                "acls",
                "List of permissions to set on the item, in the format `accessType1,type1,value1;accessType2,type2,value2`, eg. `grant,everyone,everyone`",
            ),
        ],
        notes: &[
            "When creating an external item, the options you specify will map to the external item properties, eg. `--author \"Steve\"` will set the `author` property of the external item to `Steve`.",
            "If in your schema you have a multi-value property, to specify its value you need two things:",
            "- you need to separate multiple values using `;#`, eg. `--author \"Steve;#Bill\"`",
            "- you need to specify the type of the property as defined in the schema, eg. `--author@odata.type \"Collection(String)\"`",
            "When creating items you can specify one or more permissions that define who can access the external item. The format of the permissions is `accessType1,type1,value1;accessType2,type2,value2`, eg. `grant,everyone,everyone`.",
            "You can use the following access types:",
            "- `grant` - grants access to the specified users or groups",
            "- `deny` - denies access to the specified users or groups",
            "You can use the following types:",
            "- `everyone` - everyone",
            "- `group` - an Entra group",
            "- `user` - an Entra user",
            "- `everyoneExceptGuests` - everyone except external users",
            "- `externalGroup` - a group defined in the external system from which you're importing the content",
            "For more information about using these options, see the Microsoft Graph API documentation and documentation about Microsoft Graph connectors.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ExternalItem.ReadWrite.OwnedBy",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ExternalItem.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Creates an external item with simple properties that everyone is allowed to access",
                "m365 external item add --id \"pnp-ensure-siteassets-library\" --externalConnectionId \"samplesolutiongallery\" --content \"Ensure that the Site Assets library is created.\" --title \"Ensure the Site Assets Library is created\" --description \"Ensure that the Site Assets library is created.\" --authors \"Phil Harding\" --acls \"grant,everyone,everyone\"",
            ),
            (
                "Creates an external item with multi-value properties accessible only to users from the specified Entra group",
                "m365 external item add --id \"pnp-ensure-siteassets-library\" --externalConnectionId \"samplesolutiongallery\" --content \"Ensure that the Site Assets library is created.\" --title \"Ensure the Site Assets Library is created\" --description \"Ensure that the Site Assets library is created.\" --authors@odata.type \"Collection(String)\" --authors \"Phil Harding;#Steve Smith\" --acls \"grant,group,Super users\"",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/graph/connecting-external-content-manage-items#access-control-list",
                "https://learn.microsoft.com/graph/connecting-external-content-manage-items#access-control-list",
            ),
            (
                "https://learn.microsoft.com/graph/connecting-external-content-manage-items",
                "https://learn.microsoft.com/graph/connecting-external-content-manage-items",
            ),
            (
                "https://learn.microsoft.com/graph/connecting-external-content-external-groups",
                "https://learn.microsoft.com/graph/connecting-external-content-external-groups",
            ),
            (
                "https://learn.microsoft.com/graph/api/externalconnectors-externalconnection-put-items?view=graph-rest-1.0&tabs=http",
                "https://learn.microsoft.com/graph/api/externalconnectors-externalconnection-put-items?view=graph-rest-1.0&tabs=http",
            ),
        ],
    },
];

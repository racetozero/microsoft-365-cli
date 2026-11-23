//! Help for the `m365 graph ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "graph changelog list",
        summary: "Gets an overview of specific API-level changes in Microsoft Graph v1.0 and beta",
        options: &[
            (
                "versions",
                "Comma-separated list of versions to show changes for. `Beta`, `v1.0`. When no version is selected all versions are returned.",
            ),
            (
                "changeType",
                "Change type to show changes for. `Addition`, `Change`, `Deletion`. When no changeType is selected all change types are returned.",
            ),
            (
                "services",
                "Comma-separated list of services to show changes for. `Applications`, `Calendar`, `Change notifications`, `Cloud communications`, `Compliance`, `Cross-device experiences`, `Customer booking`, `Device and app management`, `Education`, `Files`, `Financials`, `Groups`, `Identity and access`, `Mail`, `Notes`, `Notifications`, `People and workplace intelligence`, `Personal contacts`, `Reports`, `Search`, `Security`, `Sites and lists`, `Tasks and plans`, `Teamwork`, `To-do tasks`, `Users`, `Workbooks and charts`. When no service is selected all services are returned.",
            ),
            (
                "startDate",
                "The startdate used to query for changes. Supported date format is `YYYY-MM-DD`. When no date is specified all changes are returned.",
            ),
            (
                "endDate",
                "The enddate used to query for changes. Supported date format is `YYYY-MM-DD`. When no date is specified all changes are returned.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get all changes within Microsoft Graph.",
                "m365 graph changelog list",
            ),
            (
                "Get all changes within Microsoft Graph for the services Groups and Users.",
                "m365 graph changelog list --services 'Groups,Users'",
            ),
            (
                "Get all changes within Microsoft Graph that happend between 2021-01-01 and 2021-05-01.",
                "m365 graph changelog list --startDate '2021-01-01' --endDate '2021-05-01'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "graph directoryextension add",
        summary: "Creates a new directory extension",
        options: &[
            ("name", "The name of the directory extension."),
            (
                "appId",
                "Application (client) ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appName",
                "The name of Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "dataType",
                "The data type of the value the extension property can hold. Possible values are: `Binary`, `Boolean`, `DateTime`, `Integer`, `LargeInteger` and `String`.",
            ),
            (
                "targetObjects",
                "Comma-separated list of Microsoft Graph resources that can use the extension. Possible values are: `User`, `Group`, `Application`, `AdministrativeUnit`, `Device` and `Organization`.",
            ),
            (
                "isMultiValued",
                "Defines the directory extension as a multi-valued property.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Create a new directory extension of string type defined for user resource.",
                "m365 graph directoryextension add --name gitHubWorkAccountName --appName ContosoApp --targetObjects User --dataType String",
            ),
            (
                "Create a new multi-valued directory extension of integer type defined for device and application resource",
                "m365 graph directoryextension add --name departmentIds --appId 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --targetObjects 'Application,Device' --dataType Integer --isMultiValued",
            ),
        ],
        links: &[(
            "Directory extensions",
            "https://learn.microsoft.com/graph/extensibility-overview#directory-microsoft-entra-id-extensions",
        )],
    },
    Help {
        name: "graph directoryextension get",
        summary: "Retrieves the definition of a directory extension",
        options: &[
            ("id", "The ID of the directory extension."),
            ("name", "The name of the directory extension."),
            (
                "appId",
                "Application (client) ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appName",
                "The name of Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Get directory extension by id registered for an application specified by app id.",
                "m365 graph directoryextension get --id 1f0f15e3-925d-40f0-8fc8-9d3ad135bce0 --appId fd918e4b-c821-4efb-b50a-5eddd23afc6f",
            ),
            (
                "Get directory extension by name registered for an application specified by name.",
                "m365 graph directoryextension get --name extension_105be60b603845fea385e58772d9d630_GitHubWorkAccount --appName ContosoApp",
            ),
        ],
        links: &[(
            "Directory extensions",
            "https://learn.microsoft.com/graph/extensibility-overview#directory-microsoft-entra-id-extensions",
        )],
    },
    Help {
        name: "graph directoryextension list",
        summary: "Retrieves a list of directory extensions",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application where the directory extensions are registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application where the directory extensions are registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appName",
                "The name of Entra application where the directory extensions are registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
        ],
        notes: &[
            "When neither `appId`, `appObjectId` nor `appName` is specified, the command will return all available directory extensions including those registered for multi-tenant apps.",
            "https://learn.microsoft.com/en-us/graph/api/directoryobject-getavailableextensionproperties?view=graph-rest-1.0&tabs=http",
            "Otherwise, it will return directory extensions for a specific application.",
            "https://learn.microsoft.com/en-us/graph/api/application-list-extensionproperty?view=graph-rest-1.0&tabs=http",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Get all available directory extensions including those registered for multi-tenant apps",
                "m365 graph directoryextension list",
            ),
            (
                "Get all directory extensions registered for an application specified by app id.",
                "m365 graph directoryextension list --appId fd918e4b-c821-4efb-b50a-5eddd23afc6f",
            ),
            (
                "Get all directory extensions registered for an application specified by app object id.",
                "m365 graph directoryextension list --appObjectId 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Get all directory extensions registered for an application specified by name.",
                "m365 graph directoryextension list --appName ContosoApp",
            ),
        ],
        links: &[(
            "Directory extensions",
            "https://learn.microsoft.com/graph/extensibility-overview#directory-microsoft-entra-id-extensions",
        )],
    },
    Help {
        name: "graph directoryextension remove",
        summary: "Removes a directory extension",
        options: &[
            ("id", "The ID of the directory extension to remove."),
            ("name", "The name of the directory extension to remove."),
            (
                "appId",
                "Application (client) ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "appName",
                "The name of Entra application where the directory extension is registered. Specify either `appId`, `appObjectId` or `appName`, but not multiple.",
            ),
            (
                "force",
                "Don't prompt for confirmation before removing the directory extension",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.OwnedBy",
            ),
        ],
        examples: &[
            (
                "Remove a directory extension specified by name from an application specified by app object id and don't prompt for confirmation",
                "m365 graph directoryextension remove --name extension_105be60b603845fea385e58772d9d630_githubworkaccount --appObjectId 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --force",
            ),
            (
                "Remove a directory extension specified by id from an application specified by app name and don't prompt for confirmation",
                "m365 graph directoryextension remove --id 522817ae-5c95-4243-96c1-f85231fcbc1f --appName ContosoApp",
            ),
        ],
        links: &[(
            "Directory extensions",
            "https://learn.microsoft.com/graph/extensibility-overview#directory-microsoft-entra-id-extensions",
        )],
    },
    Help {
        name: "graph openextension add",
        summary: "Adds an open extension to a resource",
        options: &[
            ("name", "The name of the open extension."),
            (
                "resourceId",
                "The Id of the resource for which the extension is created.",
            ),
            (
                "resourceType",
                "The resource type for which the extension is created. Allowed values are `user`, `group`, `device`, `organization`.",
            ),
        ],
        notes: &[
            "This command allows using unknown options to add custom data to the open extension.",
            "When adding an open extension to a user, it's possible to use the UPN as the resourceId.",
            "When creating open extensions it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Directory.AccessAsUser.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Device.ReadWrite.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Create a new open extension for a user specified by id. Extension properties are specified by unknown options.",
                "m365 graph openextension add --resourceId eb77fbcf-6fe8-458b-985d-1747284793bc --resourceType user --name 'com.contoso.roamingSettings' --theme dark --color red --language English",
            ),
            (
                "Create a new open extension for a user specified by userName. Extension properties are specified by unknown options.",
                "m365 graph openextension add --resourceId adelev@contoso.com --resourceType user --name 'com.contoso.roamingSettings' --theme dark --color red --language English",
            ),
            (
                "Create a new open extension for a group, one of the property represents a JSON object",
                "m365 graph openextension add --resourceId eb77fbcf-6fe8-458b-985d-1747284793bc --resourceType group --name 'com.contoso.roamingSettings' --settings '{\"theme\": \"dark\", \"color\": \"red\", \"language\": \"English\"}' --supportedSystem 'Linux'",
            ),
        ],
        links: &[(
            "Open extensions",
            "https://learn.microsoft.com/graph/api/opentypeextension-post-opentypeextension",
        )],
    },
    Help {
        name: "graph openextension get",
        summary: "Retrieves a specific open extension for a resource",
        options: &[
            ("name", "The name of the open extension to retrieve."),
            (
                "resourceId",
                "The Id of the resource for which to retrieve the open extension.",
            ),
            (
                "resourceType",
                "The type of resource. Allowed values are `user`, `group`, `device`, `organization`.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Directory.Read.All, Group.Read.All, User.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Device.Read.All, Group.Read.All, Organization.Read.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieve a specified open extension for a user specified by id.",
                "m365 graph openextension get --resourceId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user",
            ),
            (
                "Retrieve a specified open extension for a user specified by UPN.",
                "m365 graph openextension get --resourceId john.doe@contoso.com --name 'com.contoso.roamingSettings' --resourceType user",
            ),
        ],
        links: &[(
            "Open extensions",
            "https://learn.microsoft.com/graph/extensibility-overview?tabs=http#open-extensions",
        )],
    },
    Help {
        name: "graph openextension list",
        summary: "Retrieves all open extensions for a resource",
        options: &[
            (
                "resourceId",
                "The Id of the resource for which to retrieve open extensions.",
            ),
            (
                "resourceType",
                "The type of resource. Allowed values are `user`, `group`, `device`, `organization`.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Directory.Read.All, Group.Read.All, User.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Device.Read.All, Group.Read.All, Organization.Read.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieve open extensions for a user specified by id",
                "m365 graph openextension list --resourceId eb77fbcf-6fe8-458b-985d-1747284793bc --resourceType user",
            ),
            (
                "Retrieve open extensions for a user specified by UPN",
                "m365 graph openextension list --resourceId john.doe@contoso.com --resourceType user",
            ),
            (
                "Retrieve open extensions for a group specified by id",
                "m365 graph openextension list --resourceId 19588303-70bf-44a4-beb1-e2f0f1ed06b3 --resourceType group",
            ),
            (
                "Retrieve open extensions for an organization specified by id",
                "m365 graph openextension list --resourceId c956e711-f074-40c3-8431-fbd69bb67d9c --resourceType organization",
            ),
        ],
        links: &[(
            "Open extensions",
            "https://learn.microsoft.com/graph/extensibility-overview?tabs=http#open-extensions",
        )],
    },
    Help {
        name: "graph openextension remove",
        summary: "Removes a specific open extension for a resource",
        options: &[
            ("name", "The name of the open extension to remove."),
            (
                "resourceId",
                "The Id of the resource for which to remove the open extension.",
            ),
            (
                "resourceType",
                "The type of resource. Allowed values are `user`, `group`, `device`, `organization`.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Directory.AccessAsUser.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Device.ReadWrite.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove a specified open extension for a user specified by id.",
                "m365 graph openextension remove --resourceId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user",
            ),
            (
                "Remove a specified open extension for a user specified by UPN without confirmation prompt.",
                "m365 graph openextension remove --resourceId john.doe@contoso.com --name 'com.contoso.roamingSettings' --resourceType user --force",
            ),
            (
                "Remove a specified open extension for a group specified by id.",
                "m365 graph openextension remove --resourceId c956e711-f074-40c3-8431-fbd69bb67d9c --name 'com.contoso.groupSettings' --resourceType group",
            ),
        ],
        links: &[(
            "Open extensions",
            "https://learn.microsoft.com/graph/extensibility-overview?tabs=http#open-extensions",
        )],
    },
    Help {
        name: "graph openextension set",
        summary: "Updates an open extension for a resource",
        options: &[
            ("name", "The name of the open extension."),
            (
                "resourceId",
                "The Id of the resource for which to update the open extension.",
            ),
            (
                "resourceType",
                "The type of resource. Allowed values are `user`, `group`, `device`, `organization`.",
            ),
            (
                "keepUnchangedProperties",
                "Keeps unspecified properties. Without this flag, any property that is not updated will be removed from the extension.",
            ),
        ],
        notes: &[
            "This command allows using unknown options to update custom data of the open extension.",
            "When updating an open extension to a user, it's possible to use the UPN as the resourceId.",
            "When updating open extensions it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "If a property of the open extension is not specified, the property is removed from the open extension.",
            "If a property of the open extension is not specified and `keepUnchangedProperties` is specified, the property will be kept in the open extension.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Directory.AccessAsUser.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Device.ReadWrite.All, Group.ReadWrite.All, Organization.ReadWrite.All, User.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Updates an open extension for a user specified by id",
                "m365 graph openextension set --userId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user --theme dark --color red --language English",
            ),
            (
                "Updates an open extension for a user specified by UPN",
                "m365 graph openextension set --userName john.doe@contoso.com --name 'com.contoso.roamingSettings' --resourceType user --theme dark --color red --language English",
            ),
            (
                "Updates an open extension for a group specified by id, one of the property represents a JSON object",
                "m365 graph openextension set --resourceId c956e711-f074-40c3-8431-fbd69bb67d9c --name 'com.contoso.roamingSettings' --resourceType group --settings '{\"theme\": \"dark\", \"color\": \"red\" }'",
            ),
            (
                "Updates an open extension, but keeps the properties that are not specified",
                "m365 graph openextension set --userId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user --color red --keepUnchangedProperties",
            ),
            (
                "Updates an open extension, but removes the properties that are not specified",
                "m365 graph openextension set --userId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user --color red",
            ),
            (
                "Clears the value of the property",
                "m365 graph openextension set --userId eb77fbcf-6fe8-458b-985d-1747284793bc --name 'com.contoso.roamingSettings' --resourceType user --theme \"\"",
            ),
        ],
        links: &[(
            "Open extensions",
            "https://learn.microsoft.com/graph/extensibility-overview?tabs=http#open-extensions",
        )],
    },
    Help {
        name: "graph schemaextension add",
        summary: "Creates a Microsoft Graph schema extension",
        options: &[
            (
                "id",
                "The unique identifier for the schema extension definition",
            ),
            ("description", "Description of the schema extension"),
            (
                "owner",
                "The Id of the Microsoft Entra application that is the owner of the schema extension",
            ),
            (
                "targetTypes",
                "Comma-separated list of Microsoft Graph resource types the schema extension targets",
            ),
            (
                "properties",
                "The collection of property names and types that make up the schema extension definition formatted as a JSON string",
            ),
        ],
        notes: &[
            "To create a schema extension, you have to specify a unique ID for the schema extension. You can assign a value in one of two ways:",
            "- concatenate the name of one of your verified domains with a name for the schema extension to form a unique string in this format, `{domainName}_{schemaName}`, eg. `contoso_mySchema`.",
            "    NOTE: Only verified domains under the following top-level domains are supported: .com, .net, .gov, .edu or .org.",
            "- Provide a schema name, and let Microsoft Graph use that schema name to complete the id assignment in this format: `ext{8-random-alphanumeric-chars}_{schema-name}`, eg. `extkvbmkofy_mySchema`.",
            "The schema extension ID cannot be changed after creation.",
            "The schema extension owner is the ID of the Microsoft Entra application that is the owner of the schema extension. Once set, this property is read-only and cannot be changed.",
            "The target types are the set of Microsoft Graph resource types (that support schema extensions) that this schema extension definition can be applied to. This option is specified as a comma-separated list",
            "When using the `--properties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All, Directory.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Create a schema extension",
                "m365 graph schemaextension add --id MySchemaExtension --description \"My Schema Extension\" --targetTypes Group --owner 62375ab9-6b52-47ed-826b-58e47e0e304b --properties '[{\"name\":\"myProp1\",\"type\":\"Integer\"},{\"name\":\"myProp2\",\"type\":\"String\"}]'",
            ),
            (
                "Create a schema extension with a verified domain",
                "m365 graph schemaextension add --id contoso_MySchemaExtension --description \"My Schema Extension\" --targetTypes Group --owner 62375ab9-6b52-47ed-826b-58e47e0e304b --properties '[{\"name\":\"myProp1\",\"type\":\"Integer\"},{\"name\":\"myProp2\",\"type\":\"String\"}]'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "graph schemaextension get",
        summary: "Gets the properties of the specified schema extension definition",
        options: &[(
            "id",
            "The unique identifier for the schema extension definition",
        )],
        notes: &[
            "To get properties of a schema extension definition, you have to pass the ID of the schema extension.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[(
            "Gets properties of a schema extension definition with ID domain_myExtension",
            "m365 graph schemaextension get --id domain_myExtension",
        )],
        links: &[],
    },
    Help {
        name: "graph schemaextension list",
        summary: "Gets a list of schemaExtension objects created in the current tenant, that can be InDevelopment, Available, or Deprecated.",
        options: &[
            (
                "status",
                "The status to filter on. Available values are Available, InDevelopment, Deprecated",
            ),
            ("owner", "The id of the owner to filter on"),
            ("pageSize", "Number of objects to return"),
            (
                "pageNumber",
                "Page number to return if pageSize is specified (first page is indexed as value of 0)",
            ),
        ],
        notes: &[
            "pageNumber is specified as a 0-based index. A value of 2 returns the third page of items.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Get a list of schemaExtension objects created in the current tenant, that can be InDevelopment, Available, or Deprecated.",
                "m365 graph schemaextension list",
            ),
            (
                "Get a list of schemaExtension objects created in the current tenant, with the specified owner.",
                "m365 graph schemaextension list --owner 617720dc-85fc-45d7-a187-cee75eaf239e",
            ),
            (
                "Get a list of schemaExtension objects created in the current tenant, with the specified owner and return the third page of results of 10.",
                "m365 graph schemaextension list --owner 617720dc-85fc-45d7-a187-cee75eaf239e --pageNumber 2 --pageSize 10",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/schemaextension-list",
            "https://learn.microsoft.com/graph/api/schemaextension-list",
        )],
    },
    Help {
        name: "graph schemaextension remove",
        summary: "Removes specified Microsoft Graph schema extension",
        options: &[
            (
                "id",
                "The unique identifier for the schema extension definition",
            ),
            (
                "force",
                "Don't prompt for confirming removing the specified schema extension",
            ),
        ],
        notes: &[
            "To remove specified schema extension definition, you have to pass the ID of the schema extension.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All, Directory.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Removes specified Microsoft Graph schema extension with ID domain_myExtension. Will prompt for confirmation",
                "m365 graph schemaextension remove --id domain_myExtension",
            ),
            (
                "Removes specified Microsoft Graph schema extension with ID domain_myExtension without prompt for confirmation",
                "m365 graph schemaextension remove --id domain_myExtension --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "graph schemaextension set",
        summary: "Updates a Microsoft Graph schema extension",
        options: &[
            (
                "id",
                "The unique identifier for the schema extension definition",
            ),
            (
                "owner",
                "The ID of the Microsoft Entra application that is the owner of the schema extension",
            ),
            ("description", "Description of the schema extension"),
            (
                "status",
                "The lifecycle state of the schema extension. Accepted values are `Available` or `Deprecated`",
            ),
            (
                "targetTypes",
                "Comma-separated list of Microsoft Graph resource types the schema extension targets",
            ),
            (
                "properties",
                "The collection of property names and types that make up the schema extension definition formatted as a JSON string",
            ),
        ],
        notes: &[
            "The lifecycle state of the schema extension. The initial state upon creation is `InDevelopment`. Possible states transitions are from `InDevelopment` to `Available` and `Available` to `Deprecated`. The target types are the set of Microsoft Graph resource types (that support schema extensions) that this schema extension definition can be applied to. This option is specified as a comma-separated list.",
            "When using the `--properties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All, Directory.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Update the description of a schema extension.",
                "m365 graph schemaextension set --id MySchemaExtension --owner 62375ab9-6b52-47ed-826b-58e47e0e304b --description \"My schema extension\"",
            ),
            (
                "Update the target types and properties of a schema extension.",
                "m365 graph schemaextension set --id contoso_MySchemaExtension --owner 62375ab9-6b52-47ed-826b-58e47e0e304b --targetTypes \"Group,User\" --properties '[{\"name\":\"myProp1\",\"type\":\"Integer\"},{\"name\":\"myProp2\",\"type\":\"String\"}]'",
            ),
            (
                "Change the status of a schema extension to 'Available'.",
                "m365 graph schemaextension set --id contoso_MySchemaExtension --owner 62375ab9-6b52-47ed-826b-58e47e0e304b --status Available",
            ),
        ],
        links: &[],
    },
    Help {
        name: "graph subscription add",
        summary: "Creates a Microsoft Graph subscription",
        options: &[
            (
                "resource",
                "The resource that will be monitored for changes.",
            ),
            (
                "changeTypes",
                "The type of change in the subscribed resource that will raise a notification. The supported values are: `created`, `updated`, `deleted`. Multiple values can be combined using a comma-separated list.",
            ),
            (
                "notificationUrl",
                "The URL of the endpoint that will receive the notifications. This URL must use the HTTPS protocol.",
            ),
            (
                "expirationDateTime",
                "The date and time when the webhook subscription expires. The time is in UTC, and can be an amount of time from subscription creation that varies for the resource subscribed to. If not specified, the maximum allowed expiration for the specified resource will be used.",
            ),
            (
                "clientState",
                "The value of the clientState property sent by the service in each notification. The maximum length is 128 characters.",
            ),
            (
                "lifecycleNotificationUrl",
                "The URL of the endpoint that will receive the lifecycle notifications. This URL must use the HTTPS protocol.",
            ),
            (
                "notificationUrlAppId",
                "The app ID that the subscription service can use to generate the validation token. The value allows the client to validate the authenticity of the notification received.",
            ),
            (
                "latestTLSVersion",
                "The latest version of TLS that the notification endpoint supports. Allowed values are `v1_0`, `v1_1`, `v1_2`, `v1_3`. Default is `v1_2`.",
            ),
            (
                "withResourceData",
                "When set, the change notifications will include the changed resource data.",
            ),
            (
                "encryptionCertificate",
                "A base64-encoded representation of a certificate with a public key used to encrypt resource data in change notifications. Required when using `withResourceData`.",
            ),
            (
                "encryptionCertificateId",
                "A custom app-provided identifier to help identify the certificate needed to decrypt resource data. Required when using `withResourceData`.",
            ),
        ],
        notes: &[
            "On personal OneDrive, you can subscribe to the root folder or any subfolder in that drive. On OneDrive for Business, you can subscribe to only the root folder.",
            "Notifications are sent for the requested types of changes on the subscribed folder, or any file, folder, or other `driveItem` instances in its hierarchy. You cannot subscribe to `drive` or `driveItem` instances that are not folders, such as individual files.",
            "In Outlook, delegated permission supports subscribing to items in folders in only the signed-in user's mailbox. That means, for example, you cannot use the delegated permission Calendars.Read to subscribe to events in another user’s mailbox.",
            "To subscribe to change notifications of Outlook contacts, events, or messages in shared or delegated folders:",
            "- Use the corresponding application permission to subscribe to changes of items in a folder or mailbox of any user in the tenant.",
            "- Do not use the Outlook sharing permissions (Contacts.Read.Shared, Calendars.Read.Shared, Mail.Read.Shared, and their read/write counterparts), as they do not support subscribing to change notifications on items in shared or delegated folders.",
            "For subscribers whose notification endpoint supports a version lower than the currently recommended version (TLS 1.2), specifying this property by a set timeline allows them to temporarily use their deprecated version of TLS before completing their upgrade to TLS 1.2. For these subscribers, not setting this property per the timeline would result in subscription operations failing.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AiEnterpriseInteraction.Read, Calendars.Read, Channel.ReadBasic.All, ChannelMessage.Read.All, ChannelSettings.Read.All, Chat.Read, Chat.ReadBasic, Chat.ReadWrite, ChatMember.Read, ChatMember.ReadWrite, Contacts.Read, Files.Read.All, Group.Read.All, Mail.Read, Mail.ReadBasic, OnlineMeetingRecording.Read.All, OnlineMeetingTranscript.Read.All, Presence.Read.All, Schedule.Read.All, Schedule.ReadWrite.All, SecurityEvents.ReadWrite.All, Sites.Read.All, Tasks.ReadWrite, Team.ReadBasic.All, TeamMember.Read.All, TeamSettings.Read.All, User.Read.All, VirtualEvent.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AiEnterpriseInteraction.Read.All, AiEnterpriseInteraction.Read.User, Calendars.Read, CallRecords.Read.All, Channel.ReadBasic.All, ChannelMessage.Read.All, ChannelMessage.Read.Group, ChannelMember.Read.All, ChannelSettings.Read.All, Chat.Manage.Chat, Chat.Read.All, Chat.Read.WhereInstalled, Chat.ReadBasic.All, Chat.ReadBasic.WhereInstalled, Chat.ReadWrite.All, Chat.ReadWrite.WhereInstalled, ChatMember.Read.All, ChatMember.Read.Chat, ChatMember.Read.WhereInstalled, ChatMember.ReadWrite.All, ChatMember.ReadWrite.WhereInstalled, ChatSettings.Read.Chat, ChatSettings.ReadWrite.Chat, Contacts.Read, Files.Read.All, Group.Read.All, Mail.Read, OnlineMeetingRecording.Read.All, OnlineMeetingTranscript.Read.All, Presence.Read.All, PrintTaskDefinition.ReadWrite.All, Printer.Read.All, Printer.ReadWrite.All, Schedule.Read.All, Schedule.ReadWrite.All, SecurityEvents.ReadWrite.All, Sites.Read.All, Team.ReadBasic.All, TeamMember.Read.All, TeamSettings.Read.All, User.Read.All, VirtualEvent.Read.All",
            ),
        ],
        examples: &[
            (
                "Create a subscription.",
                "m365 graph subscription add --resource \"me/mailFolders('Inbox')/messages\" --changeTypes \"updated\" --notificationUrl \"https://webhook.azurewebsites.net/api/send/myNotifyClient\" --expirationDateTime \"2016-11-20T18:23:45.935Z\" --clientState \"secretClientState\"",
            ),
            (
                "Create a subscription on multiple change types and include resource data.",
                "m365 graph subscription add --resource \"me/messages\" --changeTypes \"updated,deleted\" --withResourceData --encryptionCertificate 'Q0xJIGZvciBNaWNyb3NvZnQgMzY1' --encryptionCertificateId 'myCert' --notificationUrl \"https://webhook.azurewebsites.net/api/send/myNotifyClient\" --expirationDateTime \"2016-11-20T18:23:45.935Z\" --clientState \"secretClientState\"",
            ),
            (
                "Create a subscription using the maximum allowed expiration for Group resources.",
                "m365 graph subscription add --resource groups --changeTypes \"updated\" --notificationUrl \"https://webhook.azurewebsites.net/api/send/myNotifyClient\" --lifecycleNotificationUrl \"https://webhook.azurewebsites.net/api/send/lifecycleNotifications\"",
            ),
            (
                "Create a subscription for Event Hub location if you are using Key Vault.",
                "m365 graph subscription add --resource user --changeTypes \"created,updated,deleted\" --notificationUrl \"EventHub:https://azureKeyVaultName.vault.azure.net/secrets/secretName?tenantId=contoso.com\"",
            ),
            (
                "Create a subscription for Event Hub location if you are using role-based access control.",
                "m365 graph subscription add --resource user --changeTypes \"created,updated,deleted\" --notificationUrl \"EventHub:https://eventHubNamespace.servicebus.windows.net/eventhubname/eventHubName?tenantId=contoso.com\"",
            ),
            (
                "Create a subscription for Event Grid Partner Topic and use TLS 1.3.",
                "m365 graph subscription add --resource user --changeTypes \"created,updated,deleted\" --latestTLSVersion 'v1_3' --notificationUrl \"EventGrid:?azuresubscriptionid=8A8A8A8A-4B4B-4C4C-4D4D-12E12E12E12E&resourcegroup=resourceGroupName&partnertopic=partnerTopicName&location=partnerTopicAzureRegionName\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/en-us/graph/api/subscription-post-subscriptions",
            "https://learn.microsoft.com/en-us/graph/api/subscription-post-subscriptions",
        )],
    },
];

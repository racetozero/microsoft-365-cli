//! Help for the `m365 entra ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "entra administrativeunit add",
        summary: "Creates a new administrative unit",
        options: &[
            ("displayName", "Display name for the administrative unit."),
            ("description", "Description for the administrative unit."),
            (
                "hiddenMembership",
                "Indicates whether the administrative unit and its members are hidden.",
            ),
        ],
        notes: &[
            "To use this command you must be either Global Administrator or Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Create an administrative unit with a specific display name",
                "m365 entra administrativeunit add --displayName 'Marketing Division'",
            ),
            (
                "Create an administrative unit with a specific display name and description",
                "m365 entra administrativeunit add --displayName 'Marketing Division' --description 'Marketing department administration'",
            ),
            (
                "Create a hidden administrative unit with a specific display name",
                "m365 entra administrativeunit add --displayName 'Marketing Division' --hiddenMembership",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit get",
        summary: "Gets information about a specific administrative unit",
        options: &[
            ("id", "The id of the administrative unit."),
            (
                "displayName",
                "The display name of the administrative unit.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of administrative unit properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
        ],
        examples: &[
            (
                "Get information about the administrative unit by its id.",
                "m365 entra administrativeunit get --id 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Get information about the administrative unit by its display name with specified properties.",
                "m365 entra administrativeunit get --displayName \"Marketing Division\" --properties \"id,displayName\"",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit list",
        summary: "Retrieves a list of administrative units",
        options: &[(
            "properties",
            "Comma-separated list of properties to retrieve.",
        )],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of administrative unit properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieve a list of administrative units.",
                "m365 entra administrativeunit list",
            ),
            (
                "Retrieve a list of administrative units with specified properties.",
                "m365 entra administrativeunit list --properties \"id,displayName\"",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit member add",
        summary: "Adds a member (user, group, or device) to an administrative unit",
        options: &[
            ("administrativeUnitId", "The id of the administrative unit."),
            (
                "administrativeUnitName",
                "The name of the administrative unit.",
            ),
            ("userId", "The id of the user to be added."),
            (
                "userName",
                "The user principal name (UPN) of the user to be added.",
            ),
            ("groupId", "The id of the group to be added."),
            ("groupName", "The name of the group to be added."),
            ("deviceId", "The id of the device to be added."),
            ("deviceName", "The name of the device to be added."),
        ],
        notes: &[
            "To use this command you must be either Global Administrator or Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Add a single user specified by id to an administrative unit specified by id",
                "m365 entra administrativeunit member add --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --userId 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Add a single user specified by user principal name to an administrative unit specified by name",
                "m365 entra administrativeunit member add --administrativeUnitName 'Marketing Division' --userName john.doe@contoso.com",
            ),
            (
                "Add a single group specified by id to an administrative unit specified by id",
                "m365 entra administrativeunit member add --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --groupId b2307a39-e878-458b-bc90-03bc578531d6",
            ),
            (
                "Add a single group specified by name to an administrative unit specified by name",
                "m365 entra administrativeunit member add --administrativeUnitName 'Marketing Division' --groupName 'Marketing Group'",
            ),
            (
                "Add a single device specified by id to an administrative unit specified by id",
                "m365 entra administrativeunit member add --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --deviceId 810c84a8-4a9e-49e6-bf7d-12d183f40d01",
            ),
            (
                "Add a single device specified by name to an administrative unit specified by name",
                "m365 entra administrativeunit member add --administrativeUnitName 'Marketing Division' --deviceName 'JohnDoe-PC'",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit member get",
        summary: "Retrieves info about a specific member of an administrative unit",
        options: &[
            ("id", "The id of a member to be retrieved."),
            ("administrativeUnitId", "The id of the administrative unit."),
            (
                "administrativeUnitName",
                "The name of the administrative unit.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "To get the member of a hidden membership in an administrative unit, the `Member.Read.Hidden` permission is required.",
            "When the `properties` option includes values with a `/`, for example: `manager/displayName`, an additional `$expand` query parameter will be included on manager.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
        ],
        examples: &[
            (
                "Get information about a member specified by id from an administrative unit specified by id",
                "m365 entra administrativeunit member get --id 64131a70-beb9-4ccb-b590-4401e58446ec --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Get information about a member specified by id from an administrative unit specified by name",
                "m365 entra administrativeunit member get --id 64131a70-beb9-4ccb-b590-4401e58446ec --administrativeUnitName 'Marketing Division'",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit member list",
        summary: "Retrieves members (users, groups, or devices) of an administrative unit",
        options: &[
            ("administrativeUnitId", "The id of the administrative unit."),
            (
                "administrativeUnitName",
                "The name of the administrative unit.",
            ),
            (
                "type",
                "When specified, the command returns only objects of specified type. Allowed values are `user`, `group` and `device`.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            (
                "filter",
                "OData filter to use to query the list of members with. Applied only when `type` is specified.",
            ),
        ],
        notes: &[
            "To list the members of a hidden membership in an administrative unit, the `Member.Read.Hidden` permission is required.",
            "When the `properties` option includes values with a `/`, for example: `manager/displayName`, an additional `$expand` query parameter will be included on manager.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.Read.All",
            ),
        ],
        examples: &[
            (
                "List members of an administrative unit specified by id",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "List members of an administrative unit specified by id. For each one return the `id` and the `displayName`",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --properties 'id,displayName'",
            ),
            (
                "List members of an administrative unit specified by name",
                "m365 entra administrativeunit member list --administrativeUnitName 'Marketing Division'",
            ),
            (
                "List users of an administrative unit specified by id",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type user",
            ),
            (
                "List users of an administrative unit specified by id that are guest users",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type user --filter \"userType eq 'Guest'\"",
            ),
            (
                "List groups of an administrative unit specified by id",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type group",
            ),
            (
                "List groups of an administrative unit specified by id and include owners names, for each member return the `id` and the `displayName`",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type group --properties 'id,displayName,owners/displayName'",
            ),
            (
                "List devices of an administrative unit specified by id",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type device",
            ),
            (
                "List devices with linux operating system of an administrative unit specified by id",
                "m365 entra administrativeunit member list --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7 --type device --filter \"operatingSystem eq 'linux'\"",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit member remove",
        summary: "Removes a member (user, group, or device) from an administrative unit",
        options: &[
            ("id", "The id of the member to be removed."),
            ("userId", "The id of the user to be removed."),
            (
                "userName",
                "The user principal name (UPN) of the user to be removed.",
            ),
            ("groupId", "The id of the group to be removed."),
            ("groupName", "The name of the group to be removed."),
            ("deviceId", "The id of the device to be removed."),
            ("deviceName", "The name of the device to be removed."),
            ("administrativeUnitId", "The id of the administrative unit."),
            (
                "administrativeUnitName",
                "The name of the administrative unit.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either Global Administrator or Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove a member specified by id from an administrative unit specified by id",
                "m365 entra administrativeunit member remove --id 64131a70-beb9-4ccb-b590-4401e58446ec --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove a user specified by id from an administrative unit specified by id",
                "m365 entra administrativeunit member remove --userId 64131a70-beb9-4ccb-b590-4401e58446ec --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove a group specified by id from an administrative unit specified by id",
                "m365 entra administrativeunit member remove --groupId b2307a39-e878-458b-bc90-03bc578531d6 --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove a device specified by id from an administrative unit specified by id",
                "m365 entra administrativeunit member remove --deviceId 810c84a8-4a9e-49e6-bf7d-12d183f40d01 --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove a user specified by UPN from an administrative unit specified by name",
                "m365 entra administrativeunit member remove --userName john.doe@contoso.com --administrativeUnitName 'Marketing Division'",
            ),
            (
                "Remove a group specified by name from an administrative unit specified by id",
                "m365 entra administrativeunit member remove --groupName 'Marketing Group' --administrativeUnitId 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove a device specified by name from an administrative unit specified by name",
                "m365 entra administrativeunit member remove --deviceName 'JohnDoe-PC' --administrativeUnitName 'Marketing Division'",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit remove",
        summary: "Removes an administrative unit",
        options: &[
            ("id", "The id of the administrative unit."),
            (
                "displayName",
                "The display name of the administrative unit.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either Global Administrator or Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AdministrativeUnit.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove an administrative unit by its id",
                "m365 entra administrativeunit remove --id 03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7",
            ),
            (
                "Remove an administrative unit by it displayName",
                "m365 entra administrativeunit remove --displayName 'Marketing Division'",
            ),
        ],
        links: &[(
            "Administrative units",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/administrative-units",
        )],
    },
    Help {
        name: "entra administrativeunit roleassignment add",
        summary: "Assigns a Microsoft Entra role with administrative unit scope to a user",
        options: &[
            ("administrativeUnitId", "The id of the administrative unit."),
            (
                "administrativeUnitName",
                "The name of the administrative unit.",
            ),
            (
                "roleDefinitionId",
                "The id of the role definition that the member is in.",
            ),
            (
                "roleDefinitionName",
                "The name of the role definition that the member is in.",
            ),
            (
                "userId",
                "The id of the user that is a member of the scoped role.",
            ),
            (
                "userName",
                "The name of the user that is a member of the scoped role.",
            ),
        ],
        notes: &[
            "To use this command you must be either Global Administrator or Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Assign a role definition specified by id to a user specified by id for an administrative unit specified by id",
                "m365 entra administrativeunit roleassignment add --administrativeUnitId 81bb36e4-f4c6-4984-8e56-d4f8feae9e09 --roleDefinitionId 4d6ac14f-3453-41d0-bef9-a3e0c569773a --userId 5f91f951-7305-4a27-9b63-7b00906de09f",
            ),
            (
                "Assign a role definition specified by name to a user specified by name for an administrative unit specified by name",
                "m365 entra administrativeunit roleassignment add --administrativeUnitName 'Marketing Division' --roleDefinitionName 'License Administrator' --userName 'john.doe@contoso.com'",
            ),
        ],
        links: &[(
            "Roles with administrative unit scope",
            "https://learn.microsoft.com/entra/identity/role-based-access-control/admin-units-assign-roles#roles-that-can-be-assigned-with-administrative-unit-scope",
        )],
    },
    Help {
        name: "entra agent blueprint list",
        summary: "Retrieves a list of templates defining the agent identity type",
        options: &[(
            "properties",
            "Comma-separated list of properties to retrieve.",
        )],
        notes: &[
            "Using the --properties option, you can specify a comma-separated list of agent blueprint properties to retrieve from Microsoft Graph. If you don't specify any properties, Microsoft Graph returns its default property set, as shown in the response examples. The CLI's default properties are used only for text output.",
            "A principal who creates a blueprint or blueprint principal is assigned as the owner. Owners can create agent identities and modify only the resources they own, even if they aren't assigned an Agent ID role.",
            "For nonowners to call this API in delegated scenarios using work or school accounts, the admin must be assigned a supported Microsoft Entra role. This operation supports the following built-in roles, which provide only the least privilege necessary:",
            "- Agent ID Administrator.",
            "- Agent ID Developer - Create agent identity blueprints and blueprint principals.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AgentIdentityBlueprint.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AgentIdentityBlueprint.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieve all agent blueprint templates",
                "m365 entra agent blueprint list",
            ),
            (
                "Retrieve id and name of agent blueprint templates",
                "m365 entra agent blueprint list --properties 'id,displayName'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra agent blueprintprincipal list",
        summary: "Retrieves a list of agent blueprint principals",
        options: &[(
            "properties",
            "Comma-separated list of properties to retrieve.",
        )],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of agent blueprint principal properties to retrieve from Microsoft Graph. If you don't specify any properties, Microsoft Graph returns its default property set. The CLI's default properties (`id`, `appDisplayName`, and `appId`) are used only for text output.",
            "A principal who creates a blueprint or blueprint principal is assigned as the owner. Owners can create agent identities and modify only the resources they own, even if they aren't assigned an Agent ID role.",
            "For nonowners to call this API in delegated scenarios using work or school accounts, the admin must be assigned a supported Microsoft Entra role. This operation supports the following built-in roles, which provide only the least privilege necessary:",
            "- Agent ID Administrator.",
            "- Agent ID Developer - Create agent identity blueprints and blueprint principals.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AgentIdentityBlueprintPrincipal.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AgentIdentityBlueprintPrincipal.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieve all agent blueprint principals",
                "m365 entra agent blueprintprincipal list",
            ),
            (
                "Retrieve id and name of agent blueprint principals",
                "m365 entra agent blueprintprincipal list --properties 'id,appDisplayName'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra agent list",
        summary: "Retrieves a list of agents",
        options: &[(
            "properties",
            "Comma-separated list of properties to be returned.",
        )],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of agent properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.ReadBasic.All"),
            ("Application", "Microsoft Graph", "User.ReadBasic.All"),
        ],
        examples: &[
            ("Retrieve all agents.", "m365 entra agent list"),
            (
                "Retrieve id and name of all agents",
                "m365 entra agent list --properties 'id,displayName'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app add",
        summary: "Creates new Entra app registration",
        options: &[
            ("name", "Name of the app."),
            (
                "multitenant",
                "Specify, to make the app available to other tenants.",
            ),
            (
                "redirectUris",
                "Comma-separated list of redirect URIs. Requires `platform` to be specified.",
            ),
            (
                "platform",
                "Platform for which the app should be configured. Allowed values `spa`, `web`, `publicClient`, `apple`, `android`.",
            ),
            (
                "implicitFlow",
                "Specify, to indicate that the authorization endpoint should return ID and access tokens.",
            ),
            (
                "withSecret",
                "When specified, will create a secret named `Default` and set it to expire 1 year in the future.",
            ),
            (
                "apisDelegated",
                "Comma-separated list of delegated permissions to register with the app.",
            ),
            (
                "apisApplication",
                "Comma-separated list of application permissions to register with the app.",
            ),
            ("uri", "Application ID URI."),
            (
                "scopeName",
                "Name of the scope to add. Requires `uri` to be specified.",
            ),
            (
                "scopeConsentBy",
                "Specifies if the scope can be consented only by admins or by admins and users. Allowed values `admins`, `adminsAndUsers`. Default `admins`.",
            ),
            (
                "scopeAdminConsentDisplayName",
                "Scope admin consent display name.",
            ),
            (
                "scopeAdminConsentDescription",
                "Scope admin consent description.",
            ),
            (
                "certificateFile",
                "Path to the file with certificate public key. Specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "certificateBase64Encoded",
                "Base64-encoded string with certificate public key. Specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "certificateDisplayName",
                "Display name for the certificate. If not given, the displayName will be set to the certificate subject. When specified, also specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "grantAdminConsent",
                "When specified, grants application & delegated permissions through admin consent.",
            ),
            (
                "manifest",
                "App manifest as retrieved from Entra ID to create the app registration from.",
            ),
            (
                "bundleId",
                "Specify a bundle Id to add the iOS / macOS or Android platform to the application. Required when platform is `apple` or `android`.",
            ),
            (
                "signatureHash",
                "A required hash when specifying a bundle Id and platform `android`.",
            ),
            (
                "save",
                "Use to store the information about the created app in a local file.",
            ),
            (
                "allowPublicClientFlows",
                "Enable the allow public client flows feature on the app registration.",
            ),
        ],
        notes: &[
            "You can use this command to create a new Entra app registration either by specifying the different configuration settings through the corresponding options or by using the manifest.",
            "If you don't use the manifest, you must specify the name of the Entra app registration using the `name` option. If you use the manifest, you can skip the `name` option assuming the manifest contains the `displayName` property.",
            "You can also use the manifest to provision some of the configuration settings of your Entra app. All properties specified in the manifest are optional and will set if specified.",
            "If you specify the manifest along with some options, values specified in the options will override settings from the manifest. One exception is the name specified in the `name` option which will be overridden by the `displayName` property from the manifest if specified.",
            "The following properties specified in the manifest retrieved from Entra ID are not supported by this command:",
            "- accessTokenAcceptedVersion",
            "- disabledByMicrosoftStatus",
            "- errorUrl",
            "- oauth2RequirePostResponse",
            "- oauth2AllowUrlPathMatching",
            "- orgRestrictions",
            "- samlMetadataUrl",
            "When specifying the value for the `uri`, you can use the `_appId_` token, to include the ID of the newly generated Entra app registration in the Application ID URI, eg. URI `api://caf406b91cd4.ngrok.io/_appId_` will become `api://caf406b91cd4.ngrok.io/ab3bd119-faf7-4db5-ba99-eb7e748f778a` where the last portion is the app ID of the created Entra app registration.",
            "When using the `withSecret` option, this command will automatically generate a secret named `Default` and set it to expire 1 year in the future.",
            "After creating the Entra app registration, this command returns the app ID and object ID of the created app registration. If you used the `withSecret` option, it will also return the generated secret.",
            "If you want to store the information about the created Entra app registration, use the `--save` option. This is useful when you build solutions connected to Microsoft 365 and want to easily manage app registrations used with your solution. When you use the `--save` option, after you create the app registration, the command will write its ID and name to the `.m365rc.json` file in the current directory. If the file already exists, it will add the information about the to it, allowing you to track multiple apps. If the file doesn't exist, the command will create it.",
            "When specifying `--grantAdminConsent` option, an enterprise application will be created for the app registration.",
        ],
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
                "Create new Entra app registration with the specified name",
                "m365 entra app add --name 'My Entra app'",
            ),
            (
                "Create new Entra app registration from the manifest retrieved from the Azure Portal stored in a local file named manifest.json",
                "m365 entra app add --manifest @manifest.json",
            ),
            (
                "Create new multitenant Entra app registration",
                "m365 entra app add --name 'My Entra app' --multitenant",
            ),
            (
                "Create new Entra app registration for a web app with the specified redirect URIs",
                "m365 entra app add --name 'My Entra app' --redirectUris 'https://myapp.azurewebsites.net,http://localhost:4000' --platform web",
            ),
            (
                "Create new Entra app registration for a desktop app",
                "m365 entra app add --name 'My Entra app' --redirectUris 'https://login.microsoftonline.com/common/oauth2/nativeclient' --platform publicClient",
            ),
            (
                "Create new Entra app registration with an auto-generated secret (secret returned in the command output)",
                "m365 entra app add --name 'My Entra app' --withSecret",
            ),
            (
                "Create new Entra app registration for a daemon app with specified Microsoft Graph application permissions",
                "m365 entra app add --name 'My Entra app' --withSecret --apisApplication 'https://graph.microsoft.com/Group.ReadWrite.All,https://graph.microsoft.com/Directory.Read.All'",
            ),
            (
                "Create new Entra app registration for a single-page app with specified Microsoft Graph delegated permissions",
                "m365 entra app add --name 'My Entra app' --platform spa --redirectUris 'https://myspa.azurewebsites.net,http://localhost:8080' --apisDelegated 'https://graph.microsoft.com/Calendars.Read,https://graph.microsoft.com/Directory.Read.All' --implicitFlow",
            ),
            (
                "Create new Entra app registration with Application ID URI set to a fixed value",
                "m365 entra app add --name 'My Entra app' --uri https://contoso.onmicrosoft.com/myapp",
            ),
            (
                "Create new Entra app registration with Application ID URI set to a value that contains the ID of the app registration, designated with the `_appId_` token and a custom API scope that can be consented by both admins and users",
                "m365 entra app add --name 'My Entra app' --uri api://caf406b91cd4.ngrok.io/_appId_ --scopeName access_as_user --scopeAdminConsentDescription 'Access as a user' --scopeAdminConsentDisplayName 'Access as a user' --scopeConsentBy adminsAndUsers",
            ),
            (
                "Create new Entra app registration for a daemon app with specified Microsoft Graph application permissions, including admin consent",
                "m365 entra app add --name 'My Entra app' --apisApplication 'https://graph.microsoft.com/Group.ReadWrite.All' --grantAdminConsent",
            ),
            (
                "Create new Entra app registration with the specified name. Store information about the created app registration in the .m365rc.json file in the current directory.",
                "m365 entra app add --name 'My Entra app' --save",
            ),
            (
                "Create new Entra app registration with a certificate",
                "m365 entra app add --name 'My Entra app' --certificateDisplayName \"Some certificate name\" --certificateFile \"c:\\temp\\some-certificate.cer\"",
            ),
            (
                "Create a new Entra app registration with the allow public client flows feature enabled.",
                "m365 entra app add --name 'My Entra app' --allowPublicClientFlows",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app get",
        summary: "Gets an Entra app registration",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration to get.",
            ),
            (
                "objectId",
                "Object ID of the Entra application registration to get.",
            ),
            ("name", "Name of the Entra application registration to get."),
            (
                "save",
                "Use to store the information about the created app in a local file.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "For best performance use the `objectId` option to reference the Entra application registration to get. If you use `appId` or `name`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
            "If you want to store the information about the Entra app registration, use the `--save` option. This is useful when you build solutions connected to Microsoft 365 and want to easily manage app registrations used with your solution. When you use the `--save` option, after you get the app registration, the command will write its ID and name to the `.m365rc.json` file in the current directory. If the file already exists, it will add the information about the App registration to it if it's not already present, allowing you to track multiple apps. If the file doesn't exist, the command will create it.",
            "Using the `--properties` option, you can specify a comma-separated list of app properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Get the Entra application registration by its app (client) ID.",
                "m365 entra app get --appId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Get the Entra application registration by its object ID.",
                "m365 entra app get --objectId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Get the Entra application registration by its name with specified properties.",
                "m365 entra app get --name \"My app\" --properties \"appId,displayName\"",
            ),
            (
                "Get the Entra application registration by its name. Store information about the retrieved app registration in the .m365rc.json file in the current directory.",
                "m365 entra app get --name \"My app\" --save",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app list",
        summary: "Retrieves a list of Entra app registrations",
        options: &[(
            "properties",
            "Comma-separated list of properties to retrieve.",
        )],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of app properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Retrieve a list of Entra app registrations.",
                "m365 entra app list",
            ),
            (
                "Retrieve a list of Entra app registrations with specified properties.",
                "m365 entra app list --properties \"appId,displayName\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app permission add",
        summary: "Adds the specified application and/or delegated permissions to a Microsoft Entra app",
        options: &[
            (
                "appId",
                "Client ID of the Microsoft Entra app to add the API permissions to.",
            ),
            (
                "appName",
                "Display name of the Microsoft Entra app to add the API permissions to.",
            ),
            (
                "appObjectId",
                "Object ID of the Microsoft Entra app to add the API permissions to.",
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
                "When specified, grants application & delegated permissions through admin consent.",
            ),
        ],
        notes: &[
            "Scopes/Roles to grant must be fully-qualified so that we can disambiguate them between the different resources.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Grant multiple delegated API permissions to a Microsoft Entra app specified by client id.",
                "m365 entra app permission add --appId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --delegatedPermissions 'https://management.azure.com/user_impersonation https://service.flow.microsoft.com/Flows.Read.All https://graph.microsoft.com/Agreement.Read.All'",
            ),
            (
                "Grant multiple delegated API permissions to a Microsoft Entra app specified by object id.",
                "m365 entra app permission add --appObjectId 'e0306bb2-bf0b-4cc5-a845-a0b2cf11f690' --delegatedPermissions 'https://management.azure.com/user_impersonation https://service.flow.microsoft.com/Flows.Read.All https://graph.microsoft.com/Agreement.Read.All'",
            ),
            (
                "Grant multiple app-only permissions to a Microsoft Entra app specified by app display name and grant admin consent.",
                "m365 entra app permission add --appName 'Contoso App' --applicationPermissions 'https://graph.microsoft.com/Sites.FullControl.All https://microsoft.sharepoint-df.com/Sites.FullControl.All' --grantAdminConsent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app permission list",
        summary: "Lists the application and delegated permissions for a specified Entra Application Registration",
        options: &[
            (
                "appId",
                "Client ID of the Entra application registration to retrieve the permissions for.",
            ),
            (
                "appName",
                "Display name of the Entra application registration to add the API permissions to.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application registration to retrieve the permissions for.",
            ),
            (
                "type",
                "The type of permissions to retrieve. Allowed values: `delegated`, `application`, `all`. Defaults to `all`.",
            ),
        ],
        notes: &[
            "For best performance use the `objectId` option to reference the Entra application registration to get. If you use `appId`, this command will first need to find the corresponding object ID for that application.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Retrieves all permissions for an Entra application registration specified by client id.",
                "m365 entra app permission list --appId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690'",
            ),
            (
                "Retrieves all permissions for an Entra application registration by specified by app display name.",
                "m365 entra app permission list --appName 'Contoso App'",
            ),
            (
                "Retrieves all delegated permissions for an Entra application registration specified by object id.",
                "m365 entra app permission list --appObjectId '64381cda-d40d-4fb0-bce2-bece391546a2' --type delegated",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app permission remove",
        summary: "Removes the specified application and/or delegated permissions from a specified Entra app.",
        options: &[
            (
                "appId",
                "Client ID of the Microsoft Entra app to remove the API permissions from.",
            ),
            (
                "appObjectId",
                "Object ID of the Microsoft Entra app to remove the API permissions from.",
            ),
            (
                "appName",
                "Display name of the Entra app to remove the API permissions from.",
            ),
            (
                "applicationPermissions",
                "Space-separated list of application permissions to remove.",
            ),
            (
                "delegatedPermissions",
                "Space-separated list of delegated permissions to remove.",
            ),
            (
                "revokeAdminConsent",
                "When specified, revokes the admin consent for the specified permissions as well.",
            ),
            (
                "force",
                "Don't prompt for confirmation to remove the permissions.",
            ),
        ],
        notes: &[
            "Removing permissions on App Registrations does not immediately remove consent given by an administrator. Explicitly instruct the CLI to revoke consent by using the `--revokeAdminConsent` flag.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove multiple delegated API permissions from an Entra app registration",
                "m365 entra app permission remove --appId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --delegatedPermissions 'https://management.azure.com/user_impersonation https://service.flow.microsoft.com/Flows.Read.All https://graph.microsoft.com/Agreement.Read.All'",
            ),
            (
                "Remove multiple app-only permissions from an Entra app registration and revoke admin consent",
                "m365 entra app permission remove --appId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --applicationPermissions 'https://graph.microsoft.com/Sites.FullControl.All https://microsoft.sharepoint-df.com/Sites.FullControl.All' --revokeAdminConsent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app remove",
        summary: "Removes an Entra app registration",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration to remove.",
            ),
            (
                "objectId",
                "Object ID of the Entra application registration to remove.",
            ),
            (
                "name",
                "Name of the Entra application registration to remove.",
            ),
            ("force", "Don't prompt for confirmation to remove the app."),
        ],
        notes: &[
            "For best performance use the `objectId` option to reference the Entra application registration to remove. If you use `appId` or `name`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
        ],
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
                "Remove the Entra application registration by its app (client) ID.",
                "m365 entra app remove --appId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Remove the Entra application registration by its object ID.",
                "m365 entra app remove --objectId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Remove the Entra application registration by its name. Will NOT prompt for confirmation before deleting.",
                "m365 entra app remove --name \"My app\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app role add",
        summary: "Adds role to the specified Entra app registration",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration to which to add the role.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application registration to which to add the role.",
            ),
            (
                "appName",
                "Name of the Entra application registration to which to add the role.",
            ),
            ("name", "Name of the role to add"),
            ("description", "Description of the role to add"),
            (
                "allowedMembers",
                "Types of members that can be added to the group. Allowed values: `usersGroups`, `applications`, `both`",
            ),
            ("claim", "Claim value"),
        ],
        notes: &[
            "For best performance use the `appObjectId` option to reference the Entra application registration for which to add the role. If you use `appId` or `appName`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Add role to the Entra application registration specified by its object ID",
                "m365 entra app role add --appObjectId d75be2e1-0204-4f95-857d-51a37cf40be8 --name Managers --description \"Managers\" --allowedMembers usersGroups --claim managers",
            ),
            (
                "Add role to the Entra application registration specified by its app (client) ID",
                "m365 entra app role add --appId e75be2e1-0204-4f95-857d-51a37cf40be8 --name Managers --description \"Managers\" --allowedMembers usersGroups --claim managers",
            ),
            (
                "Add role to the Entra application registration specified by its name",
                "m365 entra app role add --appName \"My app\" --name Managers --description \"Managers\" --allowedMembers usersGroups --claim managers",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app role list",
        summary: "Gets Entra app registration roles",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration for which to retrieve roles.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application registration for which to retrieve roles.",
            ),
            (
                "appName",
                "Name of the Entra application registration for which to retrieve roles.",
            ),
        ],
        notes: &[
            "For best performance use the `appObjectId` option to reference the Entra application registration for which to retrieve roles. If you use `appId` or `appName`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Get roles for the Entra application registration specified by its object ID",
                "m365 entra app role list --appObjectId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Get roles for the Entra application registration specified by its app (client) ID",
                "m365 entra app role list --appId e75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Get roles for the Entra application registration specified by its name",
                "m365 entra app role list --appName \"My app\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app role remove",
        summary: "Removes role from the specified Entra app registration",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration from which role should be removed.",
            ),
            (
                "appObjectId",
                "Object ID of the Entra application registration from which role should be removed.",
            ),
            (
                "appName",
                "Name of the Entra application registration from which role should be removed.",
            ),
            ("name", "Name of the role to remove."),
            ("id", "Id of the role to remove."),
            ("claim", "Claim value of the role to remove."),
            ("force", "Don't prompt for confirmation to remove the role."),
        ],
        notes: &[
            "For best performance use the `appObjectId` option to reference the Entra application registration from which to remove the role. If you use `appId` or `appName`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
            "If the command finds multiple roles with the specified role name, it will prompt you to disambiguate which role it should use, listing the claim values.",
            "If the role to be removed is 'Enabled', this command will disable the role first and then remove.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove role from a Entra application registration using object ID and role name options. Will prompt for confirmation before deleting the role.",
                "m365 entra app role remove --appObjectId d75be2e1-0204-4f95-857d-51a37cf40be8 --name \"Get Product\"",
            ),
            (
                "Remove role from a Entra application registration using app (client) ID and role claim options. Will prompt for confirmation before deleting the role.",
                "m365 entra app role remove --appId e75be2e1-0204-4f95-857d-51a37cf40be8 --claim \"Product.Get\"",
            ),
            (
                "Remove role from a Entra application registration using app name and role claim options. Will prompt for confirmation before deleting the role.",
                "m365 entra app role remove --appName \"My app\" --claim \"Product.Get\"",
            ),
            (
                "Remove role from a Entra application registration using object ID and role id options. Will NOT prompt for confirmation before deleting the role.",
                "m365 entra app role remove --appObjectId d75be2e1-0204-4f95-857d-51a37cf40be8 --id 15927ce6-1933-4b2f-b029-4dee3d53f4dd --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra app set",
        summary: "Updates Entra app registration",
        options: &[
            (
                "appId",
                "Application (client) ID of the Entra application registration to update.",
            ),
            (
                "objectId",
                "Object ID of the Entra application registration to update.",
            ),
            (
                "name",
                "Name of the Entra application registration to update.",
            ),
            (
                "uris",
                "Comma-separated list of Application ID URIs to update.",
            ),
            (
                "redirectUris",
                "Comma-separated list of redirect URIs to add to the app registration. Requires `platform` to be specified.",
            ),
            (
                "platform",
                "Platform for which the `redirectUri` should be configured. Allowed values `spa`, `web`, `publicClient`.",
            ),
            (
                "redirectUrisToRemove",
                "Comma-separated list of existing redirect URIs to remove. Specify, when you want to replace existing redirect URIs with another.",
            ),
            (
                "certificateFile",
                "Path to the file with certificate public key. Specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "certificateBase64Encoded",
                "Base64-encoded string with certificate public key. Specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "certificateDisplayName",
                "Display name for the certificate. If not given, the displayName will be set to the certificate subject. When specified, also specify either `certificateFile` or `certificateBase64Encoded`.",
            ),
            (
                "allowPublicClientFlows",
                "Set to `true` or `false` to toggle the allow public client flows feature on the app registration.",
            ),
        ],
        notes: &[
            "For best performance use the `objectId` option to reference the Entra application registration to update. If you use `appId` or `name`, this command will first need to find the corresponding object ID for that application.",
            "If the command finds multiple Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
            "When a certificate is specified it will be added to the list of certificates of the app without changing existing certificates.",
        ],
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
                "Update the app URI of the Entra application registration specified by its object ID",
                "m365 entra app set --objectId d75be2e1-0204-4f95-857d-51a37cf40be8 --uris https://contoso.com/e75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Update the app URI of the Entra application registration specified by its app (client) ID",
                "m365 entra app set --appId e75be2e1-0204-4f95-857d-51a37cf40be8 --uris https://contoso.com/e75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Update the app URI of the Entra application registration specified by its name",
                "m365 entra app set --name \"My app\" --uris https://contoso.com/e75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Add a new redirect URI for SPA authentication",
                "m365 entra app set --objectId 95cfe30d-ed44-4f9d-b73d-c66560f72e83 --redirectUris https://contoso.com/auth --platform spa",
            ),
            (
                "Replace one redirect URI with another for SPA authentication",
                "m365 entra app set --objectId 95cfe30d-ed44-4f9d-b73d-c66560f72e83 --redirectUris https://contoso.com/auth --platform spa --redirectUrisToRemove https://contoso.com/old-auth",
            ),
            (
                "Add a certificate to the app",
                "m365 entra app set --appId e75be2e1-0204-4f95-857d-51a37cf40be8 --certificateDisplayName \"Some certificate name\" --certificateFile \"c:\\temp\\some-certificate.cer\"",
            ),
            (
                "Enable the allow public client flows feature on the app registration",
                "m365 entra app set --appId e75be2e1-0204-4f95-857d-51a37cf40be8 --allowPublicClientFlows true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra approleassignment add",
        summary: "Adds service principal permissions also known as scopes and app role assignments for specified Microsoft Entra application registration",
        options: &[
            (
                "appId",
                "Application appId also known as clientId of the App Registration to which the configured scopes (app roles) should be applied.",
            ),
            (
                "appObjectId",
                "Application objectId of the App Registration to which the configured scopes (app roles) should be applied.",
            ),
            (
                "appDisplayName",
                "Application name of the App Registration to which the configured scopes (app roles) should be applied.",
            ),
            (
                "resource",
                "Service principal name, appId or objectId that has the scopes (roles) e.g. `SharePoint`.",
            ),
            (
                "scopes",
                "Permissions known also as scopes and roles to grant the application with. If multiple permissions have to be granted, they have to be comma-separated e.g. `Sites.Read.All`, `Sites.ReadWrite.all`.",
            ),
        ],
        notes: &[
            "This command requires tenant administrator permissions.",
            "Specify either the `appId`, `appObjectId` or `appDisplayName` but not multiple. If you specify more than one option value, the command will fail with an error.",
            "Autocomplete values for the `resource` option do not mean allowed values. The autocomplete will just suggest some known names, but that doesn't restrict you to use the name of your own custom application or other application within your tenant.",
            "This command can also be used to assign permissions to system or user-assigned managed identity.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AppRoleAssignment.ReadWrite.All, Application.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AppRoleAssignment.ReadWrite.All, Application.Read.All",
            ),
        ],
        examples: &[
            (
                "Adds SharePoint Sites.Read.All application permissions to Entra application with app id 57907bf8-73fa-43a6-89a5-1f603e29e451",
                "m365 entra approleassignment add --appId \"57907bf8-73fa-43a6-89a5-1f603e29e451\" --resource \"SharePoint\" --scopes \"Sites.Read.All\"",
            ),
            (
                "Adds multiple Microsoft Graph application permissions to an Entra application with the name _MyAppName__",
                "m365 entra approleassignment add --appDisplayName \"MyAppName\" --resource \"Microsoft Graph\" --scopes \"Mail.Read,Mail.Send\"",
            ),
            (
                "Adds Microsoft Graph Mail.Read application permissions to a system-managed identity app with objectId 57907bf8-73fa-43a6-89a5-1f603e29e451",
                "m365 entra approleassignment add --appObjectId \"57907bf8-73fa-43a6-89a5-1f603e29e451\" --resource \"Microsoft Graph\" --scopes \"Mail.Read\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/permissions-reference",
            "https://learn.microsoft.com/graph/permissions-reference",
        )],
    },
    Help {
        name: "entra approleassignment list",
        summary: "Lists app role assignments for the specified application registration",
        options: &[
            (
                "appId",
                "Application (client) Id of the App Registration for which the configured app roles should be retrieved",
            ),
            (
                "appDisplayName",
                "Display name of the application for which the configured app roles should be retrieved",
            ),
            (
                "appObjectId",
                "ObjectId of the application for which the configured app roles should be retrieved",
            ),
        ],
        notes: &[
            "Specify either the `appId`, `appObjectId` or `appDisplayName`. If you specify more than one option value, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "List app roles assigned to service principal with Application (client) ID b2307a39-e878-458b-bc90-03bc578531d6.",
                "m365 entra approleassignment list --appId b2307a39-e878-458b-bc90-03bc578531d6",
            ),
            (
                "List app roles assigned to service principal with Application display name MyAppName.",
                "m365 entra approleassignment list --appDisplayName 'MyAppName'",
            ),
            (
                "List app roles assigned to service principal with ObjectId b2307a39-e878-458b-bc90-03bc578531dd.",
                "m365 entra approleassignment list --appObjectId b2307a39-e878-458b-bc90-03bc578531dd",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
        )],
    },
    Help {
        name: "entra approleassignment remove",
        summary: "Deletes an app role assignment for the specified Entra application registration",
        options: &[
            (
                "appId",
                "Application appId also known as clientId of the App Registration for which the configured scopes (app roles) should be deleted.",
            ),
            (
                "appObjectId",
                "Application objectId of the App Registration for which the configured scopes (app roles) should be deleted.",
            ),
            (
                "appDisplayName",
                "Application name of the App Registration for which the configured scopes (app roles) should be deleted.",
            ),
            (
                "resource",
                "Service principal name, appId or objectId that has the scopes (roles) e.g. `SharePoint`.",
            ),
            (
                "scopes",
                "Permissions known also as scopes and roles to be deleted from the application. If multiple permissions have to be deleted, they have to be comma-separated e.g. `Sites.Read.All`,`Sites.ReadWrite.All`.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the all role assignment.",
            ),
        ],
        notes: &[
            "This command requires tenant administrator permissions.",
            "Specify either the `appId`, `appObjectId` or `appDisplayName` but not multiple. If you specify more than one option value, the command will fail with an error.",
            "Autocomplete values for the `resource` option do not mean allowed values. The autocomplete will just suggest some known names, but that doesn't restrict you to use name of your own custom application or other application within your tenant.",
            "This command can also be used to assign permissions to system- or user-assigned managed identity.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "AppRoleAssignment.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "AppRoleAssignment.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Deletes SharePoint Sites.Read.All application permissions from Entra application with app id 57907bf8-73fa-43a6-89a5-1f603e29e451",
                "m365 entra approleassignment remove --appId \"57907bf8-73fa-43a6-89a5-1f603e29e451\" --resource \"SharePoint\" --scopes \"Sites.Read.All\"",
            ),
            (
                "Deletes multiple Microsoft Graph application permissions from an Entra application with name MyAppName",
                "m365 entra approleassignment remove --appDisplayName \"MyAppName\" --resource \"Microsoft Graph\" --scopes \"Mail.Read,Mail.Send\"",
            ),
            (
                "Deletes Microsoft Graph Mail.Read application permissions from a system-managed identity app with objectId 57907bf8-73fa-43a6-89a5-1f603e29e451",
                "m365 entra approleassignment remove --appObjectId \"57907bf8-73fa-43a6-89a5-1f603e29e451\" --resource \"Microsoft Graph\" --scopes \"Mail.Read\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/permissions-reference",
            "https://learn.microsoft.com/graph/permissions-reference",
        )],
    },
    Help {
        name: "entra enterpriseapp add",
        summary: "Creates an enterprise application (or service principal) for a registered Entra app",
        options: &[
            (
                "id",
                "ID of the app for which the enterprise application should be created.",
            ),
            (
                "displayName",
                "Display name of the app for which the enterprise application should be created.",
            ),
            (
                "objectId",
                "ObjectId of the app for which the enterprise application should be created.",
            ),
        ],
        notes: &[
            "Specify either the `id`, `displayName` or `objectId`. If you specify more than one option value, the command will fail with an error.",
            "If you register an application in the portal, an application object as well as an enterprise application object are automatically created in your home tenant. If you register an application using CLI for Microsoft 365 or the Microsoft Graph, you'll need to create the enterprise application separately. To register/create an application using the CLI for Microsoft 365, use the m365 entra app add command.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Creates an enterprise application for a registered Entra app with the specified id.",
                "m365 entra enterpriseapp add --id b2307a39-e878-458b-bc90-03bc578531d6",
            ),
            (
                "Creates an enterprise application for a registered Entra app with the specified displayName.",
                "m365 entra enterpriseapp add --displayName \"Microsoft Graph\"",
            ),
            (
                "Creates an enterprise application for a registered Entra app with the specified objectId.",
                "m365 entra enterpriseapp add --objectId b2307a39-e878-458b-bc90-03bc578531d6",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            ),
            (
                "https://learn.microsoft.com/graph/api/serviceprincipal-post-serviceprincipals?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/serviceprincipal-post-serviceprincipals?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "entra enterpriseapp get",
        summary: "Gets information about an enterprise application (or service principal)",
        options: &[
            (
                "id",
                "ID of the application for which the enterprise application should be retrieved.",
            ),
            (
                "displayName",
                "Display name of the application for which the enterprise application should be retrieved.",
            ),
            (
                "objectId",
                "ObjectId of the application for which the enterprise application should be retrieved.",
            ),
        ],
        notes: &[
            "Specify either the `id`, `objectId` or `displayName`. If you specify more than one option value, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Return details about the enterprise application with the specified id.",
                "m365 entra enterpriseapp get --id b2307a39-e878-458b-bc90-03bc578531d6",
            ),
            (
                "Return details about the enterprise application with the specified displayName.",
                "m365 entra enterpriseapp get --displayName \"Microsoft Graph\"",
            ),
            (
                "Return details about the enterprise application with the specified ObjectId.",
                "m365 entra enterpriseapp get --objectId b2307a39-e878-458b-bc90-03bc578531dd",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            ),
            (
                "https://learn.microsoft.com/graph/api/serviceprincipal-get?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/serviceprincipal-get?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "entra enterpriseapp list",
        summary: "Lists the enterprise applications (or service principals) in Entra ID",
        options: &[
            (
                "displayName",
                "Returns only enterprise applications with the specified name.",
            ),
            (
                "tag",
                "Returns only enterprise applications with the specified tag.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.Read.All"),
            ("Application", "Microsoft Graph", "Application.Read.All"),
        ],
        examples: &[
            (
                "Returns a list of all enterprise applications.",
                "m365 entra enterpriseapp list",
            ),
            (
                "Returns a list of all enterprise applications that comply with the specified display name and the tag parameters.",
                "m365 entra enterpriseapp list --displayName \"My custom enterprise application\" --tag \"WindowsAzureActiveDirectoryIntegratedApp\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra enterpriseapp remove",
        summary: "Deletes an enterprise application (or service principal)",
        options: &[
            ("id", "ID of the enterprise application."),
            ("displayName", "Display name of the enterprise application."),
            ("objectId", "ObjectId of the enterprise application."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Application.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Application.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Delete an enterprise application by application ID.",
                "m365 entra enterpriseapp remove --id b2307a39-e878-458b-bc90-03bc578531d6 --force",
            ),
            (
                "Delete an enterprise application by display name.",
                "m365 entra enterpriseapp remove --displayName \"Contoso app\"",
            ),
            (
                "Delete an enterprise application by object ID.",
                "m365 entra enterpriseapp remove --objectId b2307a39-e878-458b-bc90-03bc578531dd",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
        )],
    },
    Help {
        name: "entra group add",
        summary: "Creates a Microsoft Entra group",
        options: &[
            (
                "displayName",
                "The name for the group. The maximum length is 256 characters.",
            ),
            ("description", "The description for the group."),
            (
                "type",
                "The group type. Valid values: `microsoft365` or `security`.",
            ),
            (
                "mailNickname",
                "The mail alias for the group (part before the @). Maximum length is 64 characters.",
            ),
            (
                "ownerIds",
                "Comma-separated list of IDs of Microsoft Entra ID users that will be group owners. Specify either `ownerIds` or `ownerUserNames`, but not both.",
            ),
            (
                "ownerUserNames",
                "Comma-separated list of UPNs of Microsoft Entra ID users that will be group owners. Specify either `ownerIds` or `ownerUserNames`, but not both.",
            ),
            (
                "memberIds",
                "Comma-separated list of IDs of Microsoft Entra ID users that will be group members. Specify either `memberIds` or `memberUserNames`, but not both.",
            ),
            (
                "memberUserNames",
                "Comma-separated list of UPNs of Microsoft Entra ID users that will be group members. Specify either `memberIds` or `memberUserNames`, but not both.",
            ),
            (
                "visibility",
                "Specifies the group join policy and group content visibility for Microsoft 365 groups. Possible values are: `Private`, `Public`, or `HiddenMembership`. Defaults to `Public`. Specify only when creating a group of type `microsoft365`.",
            ),
        ],
        notes: &[
            "The `visibility` option affects the behavior of the group.",
            "With the `Public` visibility:",
            "- Anyone can join the group without needing owner permission.",
            "- Anyone can view the attributes of the group.",
            "- Anyone can see the members of the group.",
            "With the `Private` visibilty:",
            "- Owner permission is needed to join the group.",
            "- Anyone can view the attributes of the group.",
            "- Anyone can see the members of the group.",
            "With the `HiddenMembership` visibility:",
            "- Owner permission is needed to join the group.",
            "- Guest users cannot view the attributes of the group.",
            "- Non-members cannot see the members of the group. This setting doesn't affect visibility of group owners.",
            "- Administrators (global, company, user, and helpdesk) can view the membership of the group.",
            "- The group appears in the global address book (GAL).",
            "The `HiddenMembership` visibility can be set only for Microsoft 365 groups when the groups are created. It can't be updated later.",
            "This command allows using unknown options. For a comprehensive list of group properties, please refer to the Graph documentation page.",
            "If the specified option is not found, you will receive a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Create a security group without any owners or members",
                "m365 entra group add --displayName Developers --type security",
            ),
            (
                "Create a private Microsoft 365 group with owners and members and a custom mail address",
                "m365 entra group add --displayName Developers --type microsoft365 --mailNickname devs --ownerUserNames john.doe@contoso.com --memberUserNames \"john.doe@contoso.com,adele.vance@contoso.com\" --visibility Private",
            ),
            (
                "Create a public Microsoft 365 group without any owners or members",
                "m365 entra group add --displayName Developers --type microsoft365 --description \"This group is for all developers in the company.\" --visibility Public",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group get",
        summary: "Gets information about the specified Entra group",
        options: &[
            ("id", "The object Id of the Entra group."),
            ("displayName", "The display name of the Entra group."),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of group properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.Read.All"),
            ("Application", "Microsoft Graph", "Group.Read.All"),
        ],
        examples: &[
            (
                "Get information about an Entra Group by id.",
                "m365 entra group get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Get information about an Entra Group by its display name with specified properties.",
                "m365 entra group get --displayName Finance --properties \"mail,displayName\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group list",
        summary: "Lists Entra groups",
        options: &[
            (
                "type",
                "Filter the results to only groups of a given type. Allowed values: `microsoft365`, `security`, `distribution`, `mailEnabledSecurity`. By default, all groups are listed.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of group properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.Read.All"),
            ("Application", "Microsoft Graph", "Group.Read.All"),
        ],
        examples: &[
            (
                "Lists all groups defined in Entra ID.",
                "m365 entra group list",
            ),
            (
                "List all security groups defined in Entra ID with specified properties.",
                "m365 entra group list --type security --properties \"mail,displayName\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group member add",
        summary: "Adds members to a Microsoft Entra group",
        options: &[
            ("groupId", "The ID of the Microsoft Entra group."),
            (
                "groupName",
                "The display name of the Microsoft Entra group.",
            ),
            (
                "userIds",
                "Microsoft Entra IDs of users. You can also pass a comma-separated list of IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "subgroupIds",
                "Comma-separated list of Microsoft Entra group IDs to add.",
            ),
            (
                "subgroupNames",
                "Comma-separated list of Microsoft Entra group titles to add.",
            ),
            (
                "role",
                "The role to be assigned to the new users. Valid values: `Owner`, `Member`.",
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
                "Add a single member specified by ID as a member to a group specified by group name.",
                "m365 entra group member add --groupName Developers --userIds 098b9f52-f48c-4401-819f-29c33794c3f5 --role Member",
            ),
            (
                "Add multiple members specified by ID as members to a group specified by ID.",
                "m365 entra group member add --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userIds \"098b9f52-f48c-4401-819f-29c33794c3f5,f1e06e31-3abf-4746-83c2-1513d71f38b8\" --role Member",
            ),
            (
                "Add a single member specified by UPN as an owner to a group specified by group name.",
                "m365 entra group member add --groupName Developers --userNames john.doe@contoso.com --role Owner",
            ),
            (
                "Adds multiple members specified by UPN as owners to a group specified by ID.",
                "m365 entra group member add --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userNames \"john.doe@contoso.com,adele.vance@contoso.com\" --role Owner",
            ),
            (
                "Add multiple members (subgroups) specified by ID as members to a group specified by ID.",
                "m365 entra group member add --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --subgroupIds \"098b9f52-f48c-4401-819f-29c33794c3f5,f1e06e31-3abf-4746-83c2-1513d71f38b8\" --role Member",
            ),
            (
                "Adds multiple members (subgroups) specified by name as members to a group specified by ID.",
                "m365 entra group member add --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --subgroupNames \"Developers,Human Resources\" --role Member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group member list",
        summary: "Lists members of a specific Entra group",
        options: &[
            ("groupId", "The ID of the Entra group."),
            ("groupName", "The display name of the Entra group."),
            (
                "role",
                "Filter the results to only members with the given role: `Owner`, `Member`.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            (
                "filter",
                "OData filter to use to query the list of members with.",
            ),
        ],
        notes: &[
            "When the `properties` option includes values with a `/`, for example: `manager/displayName`, an additional `$expand` query parameter will be included on `manager`.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupMember.Read.All"),
            ("Application", "Microsoft Graph", "GroupMember.Read.All"),
        ],
        examples: &[
            (
                "List all group members from a group specified by ID.",
                "m365 entra group member list --groupId 03cba9da-3974-46c1-afaf-79caa2e45bbe",
            ),
            (
                "List all owners from a group specified by display name.",
                "m365 entra group member list --groupName Developers --role Owner",
            ),
            (
                "List all group members from a group specified by name. For each one return the display name, e-mail address, and manager display name.",
                "m365 entra group member list --groupName Developers --properties \"displayName,mail,manager/displayName\"",
            ),
            (
                "List all group members from a group specified by name. For each one return the display name, e-mail address, and manager information.",
                "m365 entra group member list --groupName Developers --properties \"displayName,mail,manager/*\"",
            ),
            (
                "List all group members that are guest members.",
                "m365 entra group member list --groupName Developers --filter \"userType eq 'Guest'\"",
            ),
        ],
        links: &[(
            "https://pnp.github.io/cli-microsoft365/cmd/entra/user/user-get#more-information",
            "https://pnp.github.io/cli-microsoft365/cmd/entra/user/user-get#more-information",
        )],
    },
    Help {
        name: "entra group member remove",
        summary: "Removes members from a Microsoft Entra group",
        options: &[
            ("groupId", "The ID of the Entra ID group."),
            ("groupName", "The display name of the Entra ID group."),
            (
                "userIds",
                "Microsoft Entra user IDs. You can also pass a comma-separated list of IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "subgroupIds",
                "Microsoft Entra group IDs. You can also pass a comma-separated list of IDs.",
            ),
            (
                "subgroupNames",
                "The display names of Microsoft Entra groups. You can also pass a comma-separated list of group display names.",
            ),
            (
                "role",
                "The role to be removed from the users. Valid values: `Owner`, `Member`. Defaults to both.",
            ),
            (
                "suppressNotFound",
                "Suppress errors when a user was not found in a group.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "When you use the `suppressNotFound` option, the command will not return an error if a user is not found as either an owner or a member of the group. This feature proves useful when you need to remove a user from a group, but you are uncertain whether the user holds the role of a member or an owner within that group. Without using this option, you would need to manually verify the user's role in the group before proceeding with removal.",
        ],
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
                "Remove a single user specified by ID as member from a group specified by display name",
                "m365 entra group member remove --groupName Developers --userIds 098b9f52-f48c-4401-819f-29c33794c3f5 --role Member",
            ),
            (
                "Remove multiple users specified by ID from a group specified by ID",
                "m365 entra group member remove --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userIds \"098b9f52-f48c-4401-819f-29c33794c3f5,f1e06e31-3abf-4746-83c2-1513d71f38b8\"",
            ),
            (
                "Remove a single user specified by UPN as an owner from a group specified by display name",
                "m365 entra group member remove --groupName Developers --userNames john.doe@contoso.com --role Owner",
            ),
            (
                "Remove multiple users specified by UPN from a group specified by ID",
                "m365 entra group member remove --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userNames \"john.doe@contoso.com,adele.vance@contoso.com\"",
            ),
            (
                "Remove a single user specified by ID as owner and member of the group and suppress errors when the user was not found as owner or member",
                "m365 entra group member remove --groupName Developers --userIds 098b9f52-f48c-4401-819f-29c33794c3f5 --suppressNotFound",
            ),
            (
                "Remove 2 nested groups referenced by id from a security group",
                "m365 entra group member remove --groupName Developers --subgroupIds \"b51b6157-839f-4d92-8dab-ac61b53c6c40,1e793f86-8dc6-4df6-8037-649ef9a22330\" --role Member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group member set",
        summary: "Updates role of members in a Microsoft Entra ID group",
        options: &[
            ("groupId", "The ID of the Entra ID group."),
            (
                "groupName",
                "The display name of the Microsoft Entra group.",
            ),
            ("userIds", "Comma-separated list of user IDs."),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "role",
                "The new role to be assigned to the members. Valid values: `Owner`, `Member`.",
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
                "Update a single member specified by ID to a member of a group specified by group name",
                "m365 entra group member set --groupName Developers --userIds 098b9f52-f48c-4401-819f-29c33794c3f5 --role Member",
            ),
            (
                "Update multiple members specified by ID to members of a group specified by ID",
                "m365 entra group member set --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userIds \"098b9f52-f48c-4401-819f-29c33794c3f5,f1e06e31-3abf-4746-83c2-1513d71f38b8\" --role Member",
            ),
            (
                "Update a single member specified by UPN to an owner of a group specified by group name",
                "m365 entra group member set --groupName Developers --userNames john.doe@contoso.com --role Owner",
            ),
            (
                "Update multiple members specified by UPN to owners of a group specified by ID",
                "m365 entra group member set --groupId a03c0c35-ef9a-419b-8cab-f89e0a8d2d2a --userNames \"john.doe@contoso.com,adele.vance@contoso.com\" --role Owner",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group remove",
        summary: "Removes an Entra group",
        options: &[
            ("id", "The ID of the Entra group."),
            ("displayName", "The display name of the Entra group."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove an Entra group by display name",
                "m365 entra group remove --displayName Developers",
            ),
            (
                "Remove an Entra group by ID",
                "m365 entra group remove --id 016bf125-195b-43c5-a1a0-28ccd3192c6d",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra group set",
        summary: "Updates a Microsoft Entra group",
        options: &[
            ("id", "The ID of the Microsoft Entra group to update."),
            (
                "displayName",
                "The display name of the Microsoft Entra group to update.",
            ),
            (
                "newDisplayName",
                "The new display name of the Microsoft Entra group. The maximum length is 256 characters.",
            ),
            ("description", "The new description for the group."),
            (
                "mailNickname",
                "The new mail alias for the group (part before the @). Use only for mail-enabled groups. Maximum length is 64 characters.",
            ),
            (
                "ownerIds",
                "Comma-separated list of IDs of Microsoft Entra users that will be the group owners.",
            ),
            (
                "ownerUserNames",
                "Comma-separated list of UPNs of Microsoft Entra users that will be the group owners.",
            ),
            (
                "memberIds",
                "Comma-separated list of IDs of Microsoft Entra users that will be the group members.",
            ),
            (
                "memberUserNames",
                "Comma-separated list of UPNs of Microsoft Entra users that will be the group members.",
            ),
            (
                "visibility",
                "Specifies the group join policy and group content visibility for Microsoft 365 groups. Possible values are: `Private` or `Public`. Specify only when targeting a Microsoft 365 group.",
            ),
        ],
        notes: &[
            "The `visibility` option affects the behavior of the group.",
            "With the `Public` visibility:",
            "- Anyone can join the group without needing owner approval.",
            "- Anyone can view the attributes of the group.",
            "- Anyone can see the members of the group.",
            "With the `Private` visibilty:",
            "- Owner approval is needed to join the group.",
            "- Anyone can view the attributes of the group.",
            "- Anyone can see the members of the group.",
            "If the specified option is not found, you will receive a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
            "Specifying `memberIds` or `memberUserNames` will make only those users members, removing all others. Similarly, specifying `ownerIds` or `ownerUserNames` will make only those users owners, removing all others.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update the display name of a group specified by the display name",
                "m365 entra group set --displayName Devs --newDisplayName Developers",
            ),
            (
                "Set the owners of a group to the specified people",
                "m365 entra group set --id 57fd6b33-54eb-42b0-9ea0-8a9ac04eab7d --ownerUserNames \"john.doe@contoso.com,adele.vance@contoso.com\"",
            ),
            (
                "Update the description and mail nickname of a group",
                "m365 entra group set --id 57fd6b33-54eb-42b0-9ea0-8a9ac04eab7d --description \"All developers of the company\" --mailNickname developers",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra groupsetting add",
        summary: "Creates a group setting",
        options: &[(
            "templateId",
            "The ID of the group setting template to use to create the group setting",
        )],
        notes: &[
            "To create a group setting, you have to specify the ID of the group setting template that should be used to create the setting. You can retrieve the ID of the template using the entra groupsettingtemplate list command.",
            "To specify values for the different properties specified in the group setting template, include additional options that match the property in the group setting template. For example `--ClassificationList 'HBI, MBI, LBI, GDPR'` will set the list of classifications to use on modern SharePoint sites.",
            "Each group setting template specifies default value for each property. If you don't specify a value for the particular property yourself, the default value from the group setting template will be used. To find out which properties are available for the particular group setting template, use the entra groupsettingtemplate get command.",
            "If the specified templateId doesn't reference a valid group setting template, you will get a Resource 'xyz' does not exist or one of its queried reference-property objects are not present. error.",
            "If you try to add a group setting using a template, for which a setting already exists, you will get a A conflicting object with one or more of the specified property values is present in the directory. error.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Configure classification for modern SharePoint sites",
            "m365 entra groupsetting add --templateId 62375ab9-6b52-47ed-826b-58e47e0e304b --UsageGuidelinesUrl https://contoso.sharepoint.com/sites/compliance --ClassificationList 'HBI, MBI, LBI, GDPR' --DefaultClassification MBI",
        )],
        links: &[],
    },
    Help {
        name: "entra groupsetting get",
        summary: "Gets information about the particular group setting",
        options: &[("id", "The ID of the group setting to retrieve")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupSettings.Read.All"),
            ("Application", "Microsoft Graph", "GroupSettings.Read.All"),
        ],
        examples: &[(
            "Get information about the group setting with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            "m365 entra groupsetting get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
        )],
        links: &[],
    },
    Help {
        name: "entra groupsetting list",
        summary: "Lists Entra group settings",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupSettings.Read.All"),
            ("Application", "Microsoft Graph", "GroupSettings.Read.All"),
        ],
        examples: &[(
            "List all group settings in the tenant",
            "m365 entra groupsetting list",
        )],
        links: &[],
    },
    Help {
        name: "entra groupsetting remove",
        summary: "Removes the particular group setting",
        options: &[
            ("id", "The ID of the group setting to remove"),
            (
                "force",
                "Don't prompt for confirming removing the group setting",
            ),
        ],
        notes: &[
            "If the specified id doesn't refer to an existing group setting, you will get a `Resource does not exist` error.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove group setting with id 28beab62-7540-4db1-a23f-29a6018a3848. Will prompt for confirmation before removing the group setting",
                "m365 entra groupsetting remove --id 28beab62-7540-4db1-a23f-29a6018a3848",
            ),
            (
                "Remove group setting with id 28beab62-7540-4db1-a23f-29a6018a3848 without prompting for confirmation",
                "m365 entra groupsetting remove --id 28beab62-7540-4db1-a23f-29a6018a3848 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra groupsetting set",
        summary: "Updates the particular group setting",
        options: &[("id", "The ID of the group setting to update")],
        notes: &[
            "To update a group setting, you have to specify the ID of the group setting. You can retrieve the ID of the group setting using the entra groupsetting list command.",
            "To update values for the different properties specified in the group setting, include additional options that match the property in the group setting. For example `--ClassificationList 'HBI, MBI, LBI, GDPR'` will set the list of classifications to use on modern SharePoint sites.",
            "If you don't specify a value for the particular property, it will remain unchanged. To find out which properties are available for the particular group setting, use the entra groupsetting get command.",
            "If the specified id doesn't reference a valid group setting, you will get a Resource 'xyz' does not exist or one of its queried reference-property objects are not present. error.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Configure classification for modern SharePoint sites",
            "m365 entra groupsetting set --id c391b57d-5783-4c53-9236-cefb5c6ef323 --UsageGuidelinesUrl https://contoso.sharepoint.com/sites/compliance --ClassificationList 'HBI, MBI, LBI, GDPR' --DefaultClassification MBI",
        )],
        links: &[],
    },
    Help {
        name: "entra groupsettingtemplate get",
        summary: "Gets information about the specified Entra group settings template",
        options: &[
            ("id", "The ID of the settings template to retrieve."),
            (
                "displayName",
                "The display name of the settings template to retrieve.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupSettings.Read.All"),
            ("Application", "Microsoft Graph", "GroupSettings.Read.All"),
        ],
        examples: &[
            (
                "Get information about the group setting template with id 62375ab9-6b52-47ed-826b-58e47e0e304b",
                "m365 entra groupsettingtemplate get --id 62375ab9-6b52-47ed-826b-58e47e0e304b",
            ),
            (
                "Get information about the group setting template with display name Group.Unified",
                "m365 entra groupsettingtemplate get --displayName Group.Unified",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra groupsettingtemplate list",
        summary: "Lists Entra group settings templates",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupSettings.Read.All"),
            ("Application", "Microsoft Graph", "GroupSettings.Read.All"),
        ],
        examples: &[(
            "List all group setting templates in the tenant",
            "m365 entra groupsettingtemplate list",
        )],
        links: &[],
    },
    Help {
        name: "entra license list",
        summary: "Lists commercial subscriptions that an organization has acquired",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "LicenseAssignment.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "LicenseAssignment.Read.All",
            ),
        ],
        examples: &[(
            "List all licenses within the tenant.",
            "m365 entra license list",
        )],
        links: &[],
    },
    Help {
        name: "entra m365group add",
        summary: "Creates a Microsoft 365 Group",
        options: &[
            ("displayName", "Display name for the Microsoft 365 Group."),
            (
                "mailNickname",
                "Name to use in the group e-mail (part before the `@`).",
            ),
            ("description", "Description for the Microsoft 365 Group."),
            (
                "owners",
                "Comma-separated list of Microsoft 365 Group owners.",
            ),
            (
                "members",
                "Comma-separated list of Microsoft 365 Group members.",
            ),
            (
                "visibility",
                "Specifies the group join policy and group content visibility for groups. Allowed values: `Private`, `Public`, or `HiddenMembership`. Defaults to `Public`.",
            ),
            (
                "allowMembersToPost",
                "Set if only group members should be able to post conversations to the group.",
            ),
            (
                "hideGroupInOutlook",
                "Set to hide the group in Outlook experiences.",
            ),
            (
                "subscribeNewGroupMembers",
                "Set to subscribe all new group members to receive group conversation emails when new messages are posted in the group.",
            ),
            (
                "welcomeEmailDisabled",
                "Set to not send welcome emails to new group members.",
            ),
            (
                "logoPath",
                "Local path to the image file to use as group logo.",
            ),
        ],
        notes: &[
            "You cannot change the group type when you choose `HiddenMembership` visibility. HiddenMembership can be set only for Microsoft 365 groups, when the groups are created. It can't be updated later.",
            "When specifying the path to the logo image you can use both relative and absolute paths. Note, that ~ in the path, will not be resolved and will most likely result in an error. If an invalid user is provided in the comma-separated list of Owners or Members, the command operation will fail and the Microsoft 365 Group will not be created.",
            "Group visibility options:",
            "Value\t| Description\nPublic | Anyone can join the group without needing owner permission. Anyone can view the attributes of the group. Anyone can see the members of the group.\nPrivate | Owner permission is needed to join the group. Anyone can view the attributes of the group. Anyone can see the members of the group.\nHiddenMembership | Owner permission is needed to join the group. Guest users cannot view the attributes of the group. Non-members cannot see the members of the group. Administrators (global, company, user, and helpdesk) can view the membership of the group. The group appears in the global address book (GAL).",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.Create"),
        ],
        examples: &[
            (
                "Create a public Microsoft 365 Group.",
                "m365 entra m365group add --displayName Finance --mailNickname finance",
            ),
            (
                "Create a private Microsoft 365 Group.",
                "m365 entra m365group add --displayName Finance --mailNickname finance --visibility Private",
            ),
            (
                "Create a public Microsoft 365 Group with description and set specified users as its owners.",
                "m365 entra m365group add --displayName Finance --description \"This is the Contoso Finance Group. Please come here and check out the latest news, posts, files, and more.\" --mailNickname finance --owners \"DebraB@contoso.onmicrosoft.com,DiegoS@contoso.onmicrosoft.com\"",
            ),
            (
                "Create a public Microsoft 365 Group with description and set specified users as its members.",
                "m365 entra m365group add --displayName Finance --description \"This is the Contoso Finance Group. Please come here and check out the latest news, posts, files, and more.\" --mailNickname finance --members \"DebraB@contoso.onmicrosoft.com,DiegoS@contoso.onmicrosoft.com\"",
            ),
            (
                "Create a public Microsoft 365 Group and allow only group members to be able to post conversations to the group.",
                "m365 entra m365group add --displayName Finance --mailNickname finance --allowMembersToPost",
            ),
            (
                "Create a public Microsoft 365 Group and hide it from the Outlook experiences (web and client).",
                "m365 entra m365group add --displayName Finance --mailNickname finance --hideGroupInOutlook",
            ),
            (
                "Create a public Microsoft 365 Group and subscribe all new group members to receive group conversation emails when new messages are posted in the group.",
                "m365 entra m365group add --displayName Finance --mailNickname finance --subscribeNewGroupMembers",
            ),
            (
                "Create a public Microsoft 365 Group and set to not send welcome emails to new group members.",
                "m365 entra m365group add --displayName Finance --mailNickname finance --welcomeEmailDisabled",
            ),
            (
                "Create a public Microsoft 365 Group and set its logo.",
                "m365 entra m365group add --displayName Finance --mailNickname finance --logoPath images/logo.png",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group conversation list",
        summary: "Lists conversations for the specified Microsoft 365 group",
        options: &[
            ("groupId", "The ID of the Microsoft 365 Group."),
            ("groupName", "Display name of the Microsoft 365 Group."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Group-Conversation.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Group-Conversation.Read.All",
            ),
        ],
        examples: &[
            (
                "Lists conversations for the Microsoft 365 group specified by id.",
                "m365 entra m365group conversation list --groupId '00000000-0000-0000-0000-000000000000'",
            ),
            (
                "Lists conversations for the Microsoft 365 group specified by displayName.",
                "m365 entra m365group conversation list --groupName Finance",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group conversation post list",
        summary: "Lists conversation posts of a Microsoft 365 group",
        options: &[
            (
                "groupId",
                "The Id of the Microsoft 365 Group. You can specify the groupId or groupName, but not both.",
            ),
            (
                "groupName",
                "The Displayname of the Microsoft 365 Group. You can specify the groupId or groupName, but not both.",
            ),
            ("threadId", "The ID of the thread to retrieve details for"),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Group-Conversation.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Group-Conversation.Read.All",
            ),
        ],
        examples: &[
            (
                "Lists the posts of the specific conversation of Microsoft 365 group by groupId",
                "m365 entra m365group conversation post list --groupId '00000000-0000-0000-0000-000000000000' --threadId 'AAQkADkwN2Q2NDg1LWQ3ZGYtNDViZi1iNGRiLTVhYjJmN2Q5NDkxZQAQAOnRAfDf71lIvrdK85FAn5E='",
            ),
            (
                "Lists the posts of the specific conversation of Microsoft 365 group by groupName",
                "m365 entra m365group conversation post list --groupName 'MyGroup' --threadId 'AAQkADkwN2Q2NDg1LWQ3ZGYtNDViZi1iNGRiLTVhYjJmN2Q5NDkxZQAQAOnRAfDf71lIvrdK85FAn5E='",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group get",
        summary: "Gets information about the specified Microsoft 365 Group or Microsoft Teams team",
        options: &[
            (
                "id",
                "The ID of the Microsoft 365 Group or Microsoft Teams team to retrieve information for.",
            ),
            (
                "displayName",
                "Display name of the Microsoft 365 Group or Microsoft Teams team to retrieve information for.",
            ),
            ("withSiteUrl", "Set to retrieve the site URL for the group."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.Read.All"),
            ("Application", "Microsoft Graph", "Group.Read.All"),
        ],
        examples: &[
            (
                "Get information about the Microsoft 365 Group with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844.",
                "m365 entra m365group get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Get information about the Microsoft 365 Group with displayName Finance.",
                "m365 entra m365group get --displayName Finance",
            ),
            (
                "Get information about the Microsoft 365 Group with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 and also retrieve the URL of the corresponding SharePoint site.",
                "m365 entra m365group get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --withSiteUrl",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group list",
        summary: "Lists Microsoft 365 Groups in the current tenant",
        options: &[
            (
                "displayName",
                "Retrieve only groups with displayName starting with the specified value.",
            ),
            (
                "mailNickname",
                "Retrieve only groups with mailNickname starting with the specified value.",
            ),
            (
                "withSiteUrl",
                "Set to retrieve the site URL for each group.",
            ),
            ("orphaned", "Set to only retrieve groups without owners."),
        ],
        notes: &[
            "Using the `--withSiteUrl` option, you can retrieve the URL of the site associated with the particular Microsoft 365 Group. If you however retrieve too many groups and will try to get their site URLs, you will most likely get an error as the command will get throttled, issuing too many requests, too frequently. If you get an error, consider narrowing down the result set using the `--displayName` and `--mailNickname` filters.",
            "Using the `--orphaned` option, you can retrieve Microsoft 365 Groups without owners.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.Read.All"),
            ("Application", "Microsoft Graph", "Group.Read.All"),
        ],
        examples: &[
            (
                "List all Microsoft 365 Groups in the tenant.",
                "m365 entra m365group list",
            ),
            (
                "List Microsoft 365 Groups with display name starting with Project.",
                "m365 entra m365group list --displayName Project",
            ),
            (
                "List Microsoft 365 Groups mail nick name starting with team.",
                "m365 entra m365group list --mailNickname team",
            ),
            (
                "List Microsoft 365 Groups with display name starting with Project including the URL of the corresponding SharePoint site.",
                "m365 entra m365group list --displayName Project --withSiteUrl",
            ),
            (
                "List Microsoft 365 Groups without owners.",
                "m365 entra m365group list --orphaned",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group recyclebinitem clear",
        summary: "Clears Microsoft 365 Groups from the recycle bin in the current tenant",
        options: &[(
            "force",
            "Don't prompt for confirmation to clear the recycle bin items.",
        )],
        notes: &[],
        permissions: &[("Delegated", "Microsoft Graph", "Group.ReadWrite.All")],
        examples: &[
            (
                "Removes all deleted Microsoft 365 Groups in the tenant. Will prompt for confirmation before permanently removing groups.",
                "m365 entra m365group recyclebinitem clear",
            ),
            (
                "Removes all deleted Microsoft 365 Groups in the tenant. Will NOT prompt for confirmation before permanently removing groups.",
                "m365 entra m365group recyclebinitem clear --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group recyclebinitem list",
        summary: "Lists Groups from the recycle bin in the current tenant",
        options: &[
            (
                "groupName",
                "Lists groups with DisplayName starting with the specified value",
            ),
            (
                "groupMailNickname",
                "Lists groups with MailNickname starting with the specified value",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.Read.All"),
            ("Application", "Microsoft Graph", "Group.Read.All"),
        ],
        examples: &[
            (
                "List all deleted Microsoft 365 Groups in the tenant",
                "m365 entra m365group recyclebinitem list",
            ),
            (
                "List deleted Microsoft 365 Groups with display name starting with Project",
                "m365 entra m365group recyclebinitem list --groupName Project",
            ),
            (
                "List deleted Microsoft 365 Groups mail nick name starting with team",
                "m365 entra m365group recyclebinitem list --groupMailNickname team",
            ),
            (
                "List deleted Microsoft 365 Groups mail nick name starting with team and with display name starting with Project",
                "m365 entra m365group recyclebinitem list --groupMailNickname team --groupName Project",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group recyclebinitem remove",
        summary: "Permanently deletes a Microsoft 365 Group from the recycle bin in the current tenant",
        options: &[
            ("id", "The ID of the Microsoft 365 Group to remove."),
            (
                "displayName",
                "Display name for the Microsoft 365 Group to remove.",
            ),
            (
                "mailNickname",
                "Name of the group e-mail (part before the @).",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[("Delegated", "Microsoft Graph", "Group.ReadWrite.All")],
        examples: &[
            (
                "Removes the Microsoft 365 Group with specific ID",
                "m365 entra m365group recyclebinitem remove --id \"00000000-0000-0000-0000-000000000000\"",
            ),
            (
                "Removes the Microsoft 365 Group with specific name",
                "m365 entra m365group recyclebinitem remove --displayName \"My Group\"",
            ),
            (
                "Remove the Microsoft 365 Group with specific mail nickname without confirmation",
                "m365 entra m365group recyclebinitem remove --mailNickname \"Mygroup\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group recyclebinitem restore",
        summary: "Restores a deleted Microsoft 365 Group",
        options: &[
            ("id", "The ID of the Microsoft 365 Group to restore."),
            (
                "displayName",
                "Display name for the Microsoft 365 Group to restore.",
            ),
            (
                "mailNickname",
                "Name of the group e-mail (part before the @).",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Restores the Microsoft 365 Group with specific ID",
                "m365 entra m365group recyclebinitem restore --id 28beab62-7540-4db1-a23f-29a6018a3848",
            ),
            (
                "Restores the Microsoft 365 Group with specific name",
                "m365 entra m365group recyclebinitem restore --displayName \"My Group\"",
            ),
            (
                "Restores the Microsoft 365 Group with specific mail nickname",
                "m365 entra m365group recyclebinitem restore --mailNickname \"Mygroup\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group remove",
        summary: "Removes an Microsoft 365 Group",
        options: &[
            ("id", "The ID of the Microsoft 365 Group to remove."),
            (
                "displayName",
                "Display name of the Microsoft 365 Group to remove.",
            ),
            ("force", "Don't prompt for confirming removing the group"),
            (
                "permanent",
                "Permanently remove the group without leaving it in the recycle bin.",
            ),
        ],
        notes: &[
            "If the specified id doesn't refer to an existing group, you will get a `Resource does not exist` error. Additionally, if you do not have access to the group or the associated group-connected site, you will get an `Access denied` error.",
            "When you use the `--permanent` flag to remove a Microsoft 365 group permanently, the process involves deleting the associated group-connected site and the group, and then checking if the group has been moved to the deleted groups list. In some cases, it may take a few seconds for the group to appear in the deleted groups list.",
            "To ensure a smooth deletion process, the command employs a retry mechanism with the following parameters:",
            "- `maxRetries`: This parameter is set to 10 by default. It specifies the maximum number of times the command will check if the group has been moved to the deleted groups list.",
            "- `intervalInMs`: This parameter is set to 5000 milliseconds (5 seconds) by default. It defines the time interval between each retry attempt.",
            "If the group is successfully moved to the deleted groups list within the specified number of retries, the command will then proceed to permanently remove it from the tenant. If the group cannot be removed after all retries, it will remain in the deleted groups list.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove group with id 28beab62-7540-4db1-a23f-29a6018a3848. Will prompt for confirmation before removing the group",
                "m365 entra m365group remove --id 28beab62-7540-4db1-a23f-29a6018a3848",
            ),
            (
                "Remove group with displayName Finance without prompting for confirmation",
                "m365 entra m365group remove --displayName Finance --force",
            ),
            (
                "Remove group with id 28beab62-7540-4db1-a23f-29a6018a3848 without prompting for confirmation and without moving it to the Recycle Bin",
                "m365 entra m365group remove --id 28beab62-7540-4db1-a23f-29a6018a3848 --force --permanent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group renew",
        summary: "Renews Microsoft 365 group's expiration",
        options: &[
            ("id", "The ID of the Microsoft 365 group to renew."),
            (
                "displayName",
                "Display name of the Microsoft 365 Group to renew.",
            ),
        ],
        notes: &[
            "If the specified id doesn't refer to an existing group, you will get a `The remote server returned an error: (404) Not Found.` error.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Renew the Microsoft 365 group with id 28beab62-7540-4db1-a23f-29a6018a3848",
                "m365 entra m365group renew --id 28beab62-7540-4db1-a23f-29a6018a3848",
            ),
            (
                "Renew the Microsoft 365 group with displayName Finance",
                "m365 entra m365group renew --displayName Finance",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group report activitycounts",
        summary: "Gets the number of group activities across group workloads",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Reports.Read.All"),
            ("Application", "Microsoft Graph", "Reports.Read.All"),
        ],
        examples: &[
            (
                "Get the number of group activities across group workloads for the last week",
                "m365 entra m365group report activitycounts --period D7",
            ),
            (
                "Get the number of group activities across group workloads for the last week and exports the report data in the specified path in text format",
                "m365 entra m365group report activitycounts --period D7 --output text > \"m365groupactivitycounts.txt\"",
            ),
            (
                "Get the number of group activities across group workloads for the last week and exports the report data in the specified path in json format",
                "m365 entra m365group report activitycounts --period D7 --output json > \"m365groupactivitycounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group report activitydetail",
        summary: "Gets details about Microsoft 365 Groups activity by group.",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`. Specify the `period` or `date`, but not both.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`. Specify the `date` or `period`, but not both.",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Reports.Read.All"),
            ("Application", "Microsoft Graph", "Reports.Read.All"),
        ],
        examples: &[
            (
                "Get details about Microsoft 365 Groups activity by group for the last week",
                "m365 entra m365group report activitydetail --period D7",
            ),
            (
                "Get details about Microsoft 365 Groups activity by group for September 09, 2019",
                "m365 entra m365group report activitydetail --date 2019-09-28",
            ),
            (
                "Get details about Microsoft 365 Groups activity by group for the last week and exports the report data in the specified path in text format",
                "m365 entra m365group report activitydetail --period D7 --output text > \"m365groupactivitydetail.txt\"",
            ),
            (
                "Get details about Microsoft 365 Groups activity by group for the last week and exports the report data in the specified path in json format",
                "m365 entra m365group report activitydetail --period D7 --output json > \"m365groupactivitydetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group report activityfilecounts",
        summary: "Gets the total number of files and how many of them were active across all group sites associated with a Microsoft 365 Group",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Reports.Read.All"),
            ("Application", "Microsoft Graph", "Reports.Read.All"),
        ],
        examples: &[
            (
                "Get the total number of files and how many of them were active across all group sites associated with a Microsoft 365 Group for the last week",
                "m365 entra m365group report activityfilecounts --period D7",
            ),
            (
                "Get the total number of files and how many of them were active across all group sites associated with a Microsoft 365 Group for the last week and exports the report data in the specified path in text format",
                "m365 entra m365group report activityfilecounts --period D7 --output text > \"m365groupactivityfilecounts.txt\"",
            ),
            (
                "Get the total number of files and how many of them were active across all group sites associated with a Microsoft 365 Group for the last week and exports the report data in the specified path in json format",
                "m365 entra m365group report activityfilecounts --period D7 --output json > \"m365groupactivityfilecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group report activitygroupcounts",
        summary: "Gets the daily total number of groups and how many of them were active based on email conversations, Viva Engage posts, and SharePoint file activities.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Reports.Read.All"),
            ("Application", "Microsoft Graph", "Reports.Read.All"),
        ],
        examples: &[
            (
                "Get the daily total number of groups and how many of them were active based on activities for the last week",
                "m365 entra m365group report activitygroupcounts --period D7",
            ),
            (
                "Get the daily total number of groups and how many of them were active based on activities for the last week and exports the report data in the specified path in text format",
                "m365 entra m365group report activitygroupcounts --period D7 --output text > \"m365groupactivitygroupcounts.txt\"",
            ),
            (
                "Get the daily total number of groups and how many of them were active based on activities for the last week and exports the report data in the specified path in json format",
                "m365 entra m365group report activitygroupcounts --period D7 --output json > \"m365groupactivitygroupcounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group report activitystorage",
        summary: "Gets the total storage used across all group mailboxes and group sites",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Reports.Read.All"),
            ("Application", "Microsoft Graph", "Reports.Read.All"),
        ],
        examples: &[
            (
                "Get the total storage used across all group mailboxes and group sites for the last week",
                "m365 entra m365group report activitystorage --period D7",
            ),
            (
                "Get the total storage used across all group mailboxes and group sites for the last week and exports the report data in the specified path in text format",
                "m365 entra m365group report activitystorage --period D7 --output text > \"m365groupactivitystorage.txt\"",
            ),
            (
                "Get the total storage used across all group mailboxes and group sites for the last week and exports the report data in the specified path in json format",
                "m365 entra m365group report activitystorage --period D7 --output json > \"m365groupactivitystorage.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group set",
        summary: "Updates Microsoft 365 Group properties",
        options: &[
            ("id", "The ID of the Microsoft 365 Group to update."),
            (
                "displayName",
                "Display name of the Microsoft 365 Group to update.",
            ),
            (
                "newDisplayName",
                "New display name for the Microsoft 365 Group.",
            ),
            ("description", "Description for the Microsoft 365 Group."),
            (
                "ownerIds",
                "Comma-separated list of IDs of Microsoft Entra ID users that will be group owners. Specify either `ownerIds` or `ownerUserNames`, but not both.",
            ),
            (
                "ownerUserNames",
                "Comma-separated list of UPNs of Microsoft Entra ID users that will be group owners. Specify either `ownerIds` or `ownerUserNames`, but not both.",
            ),
            (
                "memberIds",
                "Comma-separated list of IDs of Microsoft Entra ID users that will be group members. Specify either `memberIds` or `memberUserNames`, but not both.",
            ),
            (
                "memberUserNames",
                "Comma-separated list of UPNs of Microsoft Entra ID users that will be group members. Specify either `memberIds` or `memberUserNames`, but not both.",
            ),
            (
                "isPrivate",
                "Set to `true` if the Microsoft 365 Group should be private and `false` if it should be public.",
            ),
            (
                "logoPath",
                "Local path to the image file to use as group logo.",
            ),
            (
                "allowExternalSenders",
                "Indicates if people external to the organization can send messages to the group. Valid values: `true`, `false`.",
            ),
            (
                "autoSubscribeNewMembers",
                "Indicates if new members added to the group will be auto-subscribed to receive email notifications. Valid values: `true`, `false`.",
            ),
            (
                "hideFromAddressLists",
                "Indicates if the group is not displayed in certain parts of the Outlook UI: the Address Book, address lists for selecting message recipients, and the Browse Groups dialog for searching groups. Valid values: `true`, `false`.",
            ),
            (
                "hideFromOutlookClients",
                "Indicates if the group is not displayed in Outlook clients, such as Outlook for Windows and Outlook on the web. Valid values: `true`, `false`.",
            ),
        ],
        notes: &[
            "When updating group's owners and members, the command will remove existing owners/members from the group, and the specified users will be added.",
            "When specifying the path to the logo image you can use both relative and absolute paths. Note, that ~ in the path, will not be resolved and will most likely result in an error.",
            "Options `allowExternalSenders` and `autoSubscribeNewMembers` can only be set using delegated permissions.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Group.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Group.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update Microsoft 365 Group display name.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --newDisplayName Finance",
            ),
            (
                "Change Microsoft 365 Group visibility to public.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --isPrivate `false`",
            ),
            (
                "Updates the list of Microsoft 365 Group owners with a list of users by UPN.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --ownerUserNames \"DebraB@contoso.onmicrosoft.com,DiegoS@contoso.onmicrosoft.com\"",
            ),
            (
                "Updates the list of Microsoft 365 Group owners with a list of users by ID.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --ownerIds \"3527dada-9368-4cdd-a958-5460f5658e0e,e94b2cb8-7c9a-4651-b1af-207d81a010b6\"",
            ),
            (
                "Updates the list of Microsoft 365 Group members with a list of users by UPN.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --memberUserNames \"DebraB@contoso.onmicrosoft.com,DiegoS@contoso.onmicrosoft.com\"",
            ),
            (
                "Updates the list of Microsoft 365 Group members with a list of users by ID.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --memberIds \"3527dada-9368-4cdd-a958-5460f5658e0e,e94b2cb8-7c9a-4651-b1af-207d81a010b6\"",
            ),
            (
                "Update Microsoft 365 Group logo.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --logoPath images/logo.png",
            ),
            (
                "Hide the Microsoft 365 group from the address lists and the Outlook clients.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --hideFromOutlookClients true --hideFromAddressLists true",
            ),
            (
                "Auto-subscribe new members to receive email notifications and do not allow external senders to send messages to the group.",
                "m365 entra m365group set --id 28beab62-7540-4db1-a23f-29a6018a3848 --autoSubscribeNewMembers true --allowExternalSenders false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group teamify",
        summary: "Creates a new Microsoft Teams team under existing Microsoft 365 group",
        options: &[
            (
                "id",
                "The ID of the Microsoft 365 Group to connect to Microsoft Teams.",
            ),
            (
                "displayName",
                "Display name of the Microsoft 365 Group to connect to Microsoft Teams.",
            ),
            (
                "mailNickname",
                "The mail alias of the Microsoft 365 Group to connect to Microsoft Teams.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Creates a new Microsoft Teams team under existing Microsoft 365 group with the specified id.",
                "m365 entra m365group teamify --id e3f60f99-0bad-481f-9e9f-ff0f572fbd03",
            ),
            (
                "Creates a new Microsoft Teams team under existing Microsoft 365 group with the specified displayName.",
                "m365 entra m365group teamify --displayName Finance",
            ),
            (
                "Creates a new Microsoft Teams team under existing Microsoft 365 group with the specified mailNickname._",
                "m365 entra m365group teamify --mailNickname GroupName",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group user add",
        summary: "Adds user to specified Microsoft 365 Group or Microsoft Teams team",
        options: &[
            ("groupId", "The ID of the Microsoft 365 group."),
            ("groupName", "The display name of the Microsoft 365 group."),
            ("teamId", "The ID of the Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            (
                "ids",
                "Microsoft Entra IDs of users. You can also pass a comma-separated list of IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "role",
                "The role to be assigned to the new user. Allowed values: `Owner`, `Member`. Default `Member`.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Add a new member with the userNames parameter to the specified Microsoft 365 Group.",
                "m365 entra m365group user add --groupId '00000000-0000-0000-0000-000000000000' --userNames 'anne.matthews@contoso.onmicrosoft.com'",
            ),
            (
                "Add multiple new owners with the userNames parameter to the specified Microsoft 365 Group.",
                "m365 entra m365group user add --groupName 'Contoso' --userNames 'anne.matthews@contoso.onmicrosoft.com, john.doe@contoso.onmicrosoft.com' --role Owner",
            ),
            (
                "Add a new member with the userNames parameter to the specified Microsoft Teams team.",
                "m365 entra m365group member add --teamId '00000000-0000-0000-0000-000000000000' --userNames 'anne.matthews@contoso.onmicrosoft.com' --role Member",
            ),
            (
                "Add multiple new members with the ids parameter to the specified Microsoft Teams team.",
                "m365 entra m365group user add --teamName 'Engineering' --ids '74a3b772-3122-447b-b9da-10895e238219,dd3d21e4-a142-46b9-8482-bca8fe9596b3' --role Member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group user list",
        summary: "Lists users for the specified Microsoft 365 group",
        options: &[
            ("groupId", "The ID of the Microsoft 365 group."),
            (
                "groupDisplayName",
                "The display name of the Microsoft 365 group.",
            ),
            (
                "role",
                "Filter the results to only users with the given role. Allowed values: `Owner`, `Member`.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            (
                "filter",
                "OData filter to use to query the list of users with.",
            ),
        ],
        notes: &[
            "When the `properties` option includes values with a `/`, for example: `manager/displayName`, an additional `$expand` query parameter will be included on `manager`.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupMember.Read.All, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupMember.Read.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "List all users and their role from Microsoft 365 group specified by ID.",
                "m365 entra m365group user list --groupId '00000000-0000-0000-0000-000000000000'",
            ),
            (
                "List all owners from Microsoft 365 group specified by display name.",
                "m365 entra m365group user list --groupDisplayName Developers --role Owner",
            ),
            (
                "List specific properties for all group users from a group specified by ID.",
                "m365 entra m365group user list --groupId 03cba9da-3974-46c1-afaf-79caa2e45bbe --properties \"id,jobTitle,companyName,accountEnabled\"",
            ),
            (
                "List all group members that are guest users.",
                "m365 entra m365group user list --groupDisplayName Developers --filter \"userType eq 'Guest'\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group user remove",
        summary: "Removes the specified user from specified Microsoft 365 Group or Microsoft Teams team",
        options: &[
            ("groupId", "The ID of the Microsoft 365 group."),
            ("groupName", "The display name of the Microsoft 365 group."),
            ("teamId", "The ID of the Microsoft Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            (
                "ids",
                "Microsoft Entra IDs of users. You can also pass a comma-separated list of IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the user from the specified Microsoft 365 Group or Microsoft Teams team.",
            ),
        ],
        notes: &[
            "You can remove users from a Microsoft 365 Group or Microsoft Teams team if you are owner of that group or team.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Removes user from the specified Microsoft 365 Group.",
                "m365 entra m365group user remove --groupId '5b8e4cb1-ea40-484b-a94e-02a4313fefb4' --userNames 'anne.matthews@contoso.onmicrosoft.com'",
            ),
            (
                "Removes user from the specified Microsoft 365 Team specified by id without confirmation.",
                "m365 entra m365group user remove --teamId '5b8e4cb1-ea40-484b-a94e-02a4313fefb4' --userNames 'anne.matthews@contoso.onmicrosoft.com' --force",
            ),
            (
                "Removes users specified by a comma separated list of user principal names from the specified Microsoft Teams team specified by name",
                "m365 entra m365group user remove --teamName 'Project Team' --userNames 'anne.matthews@contoso.onmicrosoft.com,john@contoso.com'",
            ),
            (
                "Removes users specified by a comma separated list of user ids from the specified Microsoft 365 group specified by name.",
                "m365 entra m365group user remove --groupName 'Project Team' --ids '5b8e4cb1-ea40-484b-a94e-02a4313fefb4,be7a56d8-b045-4938-af35-917ab6e5309f'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra m365group user set",
        summary: "Updates role of the specified users in the specified Microsoft 365 Group or Microsoft Teams team",
        options: &[
            ("groupId", "The ID of the Microsoft 365 group."),
            ("groupName", "The display name of the Microsoft 365 group."),
            ("teamId", "The ID of the Teams team."),
            ("teamName", "The display name of the Microsoft Teams team."),
            (
                "ids",
                "Microsoft Entra IDs of users. You can also pass a comma-separated list of IDs.",
            ),
            (
                "userNames",
                "The user principal names of users. You can also pass a comma-separated list of UPNs.",
            ),
            (
                "role",
                "Role to set for the given user in the specified Microsoft 365 Group or Microsoft Teams team. Allowed values: `Owner`, `Member`.",
            ),
        ],
        notes: &[
            "The command will return an error if the user already has the specified role in the given Microsoft 365 Group or Microsoft Teams team.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Group.ReadWrite.All, User.Read.All",
            ),
        ],
        examples: &[
            (
                "Promote a single user to Owner of the given Microsoft 365 Group",
                "m365 entra m365group user set --groupId '00000000-0000-0000-0000-000000000000' --userNames 'anne.matthews@contoso.onmicrosoft.com' --role Owner",
            ),
            (
                "Promote multiple users specified by the userNames parameter to Owner of the given Microsoft 365 Group",
                "m365 entra m365group user set --groupName 'Contoso' --userNames 'anne.matthews@contoso.onmicrosoft.com,john.doe@contoso.onmicrosoft.com' --role Owner",
            ),
            (
                "Promote multiple users specified by the ids parameter to Owner of the given Microsoft 365 Group",
                "m365 entra m365group user set --groupId '00000000-0000-0000-0000-000000000000' --ids '74a3b772-3122-447b-b9da-10895e238219,dd3d21e4-a142-46b9-8482-bca8fe9596b3' --role Owner",
            ),
            (
                "Demote a single user from Owner to Member in the given Microsoft 365 Group",
                "m365 entra m365group user set --groupId '00000000-0000-0000-0000-000000000000' --userNames 'anne.matthews@contoso.onmicrosoft.com' --role Member",
            ),
            (
                "Demote multiple users specified by the userNames parameter from Owner to Member of the given Microsoft Teams team",
                "m365 teams user set --teamId '00000000-0000-0000-0000-000000000000' --userNames 'anne.matthews@contoso.onmicrosoft.com,john.doe@contoso.onmicrosoft.com' --role Member",
            ),
            (
                "Demote multiple users specified by the ids parameter from Owner to Member in the given Microsoft Teams team",
                "m365 teams user set --teamName 'Engineering' --ids '74a3b772-3122-447b-b9da-10895e238219,dd3d21e4-a142-46b9-8482-bca8fe9596b3' --role Member",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra multitenant add",
        summary: "Creates a new multitenant organization",
        options: &[
            (
                "displayName",
                "Display name for the multitenant organization.",
            ),
            (
                "description",
                "Description for the multitenant organiztion.",
            ),
        ],
        notes: &["To use this command you must be at least Security Administrator."],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Create a new multitenant organization with display name only.",
                "m365 entra multitenant add --displayName 'Contoso organization'",
            ),
            (
                "Create a new multitenant organization with display name and description.",
                "m365 entra multitenant add --displayName 'Contoso organization' --description 'Multitenant organization between Contoso, Fabrikam, and Woodgrove Bank'",
            ),
        ],
        links: &[(
            "Multitenant organization",
            "https://learn.microsoft.com/entra/identity/multi-tenant-organizations/overview",
        )],
    },
    Help {
        name: "entra multitenant get",
        summary: "Gets properties of the multitenant organization",
        options: &[],
        notes: &["To use this command you must be at least Security Administrator."],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "MultiTenantOrganization.Read.All",
            ),
        ],
        examples: &[(
            "Gets properties of the multitenant organization",
            "m365 entra multitenant get",
        )],
        links: &[(
            "Multitenant organization",
            "https://learn.microsoft.com/entra/identity/multi-tenant-organizations/overview",
        )],
    },
    Help {
        name: "entra multitenant remove",
        summary: "Removes a multitenant organization",
        options: &[("force", "Don't prompt for confirmation.")],
        notes: &[
            "To use this command you must be at least Security Administrator.",
            "When removing a Multi-Tenant Organization, all associations with other tenants will be removed too.",
            "The removing process can take up to 2 hours to complete.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove the multitenant organization.",
                "m365 entra multitenant remove",
            ),
            (
                "Remove the multitenant organization without prompting for confirmation.",
                "m365 entra multitenant remove --force",
            ),
        ],
        links: &[(
            "Multitenant organization",
            "https://learn.microsoft.com/entra/identity/multi-tenant-organizations/overview",
        )],
    },
    Help {
        name: "entra multitenant set",
        summary: "Updates the properties of a multitenant organization",
        options: &[
            (
                "displayName",
                "New display name of the multitenant organization. Both options, `displayName` and `description` are optional but at least must be specified.",
            ),
            (
                "description",
                "New description of the multitenant organization. Both options, `displayName` and `description` are optional but at least must be specified.",
            ),
        ],
        notes: &["To use this command you must be at least Security Administrator."],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "MultiTenantOrganization.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Update display name of multitenant organization.",
                "m365 entra multitenant set --displayName 'Fabrikam organization'",
            ),
            (
                "Update display name and description of multitenant organization.",
                "m365 entra multitenant set --displayName 'Fabrikam organization' --description 'Multitenant organization between Fabrikam and Contoso'",
            ),
        ],
        links: &[(
            "Multitenant organization",
            "https://learn.microsoft.com/entra/identity/multi-tenant-organizations/overview",
        )],
    },
    Help {
        name: "entra oauth2grant add",
        summary: "Grants the specified service principal OAuth2 permissions to the specified resource",
        options: &[
            (
                "clientId",
                "`objectId` of the service principal for which permissions should be granted.",
            ),
            (
                "resourceId",
                "`objectId` of the Entra application to which permissions should be granted.",
            ),
            ("scope", "Permissions to grant."),
        ],
        notes: &[
            "Before you can grant service principal OAuth2 permissions, you need its `objectId`. You can retrieve it using the entra enterpriseapp get command.",
            "The resource for which you want to grant permissions is designated using its `objectId`. You can retrieve it using the entra enterpriseapp get command, the same way you would retrieve the `objectId` of the service principal.",
            "When granting OAuth2 permissions, you have to specify which permission scopes you want to grant the service principal. You can get the list of available permission scopes either from the resource documentation or from the `appRoles` property when retrieving information about the service principal using the entra enterpriseapp get command. Multiple permission scopes can be specified separated by a space.",
            "When granting OAuth2 permissions, the values of the `clientId` and `resourceId` properties form a unique key. If a grant for the same `clientId`-`resourceId` pair already exists, running the `entra oauth2grant add` command will fail with an error. If you want to change permissions on an existing OAuth2 grant use the entra oauth2grant set command instead.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Grant the service principal d03a0062-1aa6-43e1-8f49-d73e969c5812 the Calendars.Read OAuth2 permissions to the c2af2474-2c95-423a-b0e5-e4895f22f9e9 resource.",
                "m365 entra oauth2grant add --clientId d03a0062-1aa6-43e1-8f49-d73e969c5812 --resourceId c2af2474-2c95-423a-b0e5-e4895f22f9e9 --scope Calendars.Read",
            ),
            (
                "Grant the service principal d03a0062-1aa6-43e1-8f49-d73e969c5812 the Calendars.Read and Mail.Read OAuth2 permissions to the c2af2474-2c95-423a-b0e5-e4895f22f9e9 resource.",
                "m365 entra oauth2grant add --clientId d03a0062-1aa6-43e1-8f49-d73e969c5812 --resourceId c2af2474-2c95-423a-b0e5-e4895f22f9e9 --scope \"Calendars.Read Mail.Read\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
        )],
    },
    Help {
        name: "entra oauth2grant list",
        summary: "Lists OAuth2 permission grants for the specified service principal",
        options: &[(
            "spObjectId",
            "objectId of the service principal for which the configured OAuth2 permission grants should be retrieved.",
        )],
        notes: &[
            "In order to list existing OAuth2 permissions granted to a service principal, you need its `objectId`. You can retrieve it using the entra enterpriseapp get command.",
            "When using the text output type (default), the command lists only the values of the `objectId`, `resourceId` and `scope` properties of the OAuth grant. When setting the output type to JSON, all available properties are included in the command output.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Directory.Read.All"),
            ("Application", "Microsoft Graph", "Directory.Read.All"),
        ],
        examples: &[(
            "List OAuth2 permissions granted to service principal with `objectId` b2307a39-e878-458b-bc90-03bc578531d6.",
            "m365 entra oauth2grant list --spObjectId b2307a39-e878-458b-bc90-03bc578531d6",
        )],
        links: &[
            (
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            ),
            (
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-list?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-list?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "entra oauth2grant remove",
        summary: "Removes specified service principal OAuth2 permissions",
        options: &[
            (
                "grantId",
                "`objectId` of OAuth2 permission grant to remove.",
            ),
            (
                "force",
                "Do not prompt for confirmation before removing OAuth2 permission grant.",
            ),
        ],
        notes: &[
            "Before you can remove service principal's OAuth2 permissions, you need to get the `objectId` of the permissions grant to remove. You can retrieve it using the entra oauth2grant list command.",
            "If the `objectId` listed when using the entra oauth2grant list command has a minus sign ('-') prefix, you may receive an error indicating `--grantId` is missing. To resolve this issue simply escape the leading '-'.",
            "    m365 entra oauth2grant remove --grantId \\\\-Zc1JRY8REeLxmXz5KtixAYU3Q6noCBPlhwGiX7pxmU",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove the OAuth2 permission grant with the specified ID.",
                "m365 entra oauth2grant remove --grantId YgA60KYa4UOPSdc-lpxYEnQkr8KVLDpCsOXkiV8i-ek",
            ),
            (
                "Remove the OAuth2 permission grant with the specified ID without being asked for confirmation",
                "m365 entra oauth2grant remove --grantId YgA60KYa4UOPSdc-lpxYEnQkr8KVLDpCsOXkiV8i-ek --force",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            ),
            (
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-delete?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-delete?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "entra oauth2grant set",
        summary: "Updates OAuth2 permissions for the service principal",
        options: &[
            (
                "grantId",
                "`objectId` of OAuth2 permission grant to update.",
            ),
            ("scope", "Permissions to grant."),
        ],
        notes: &[
            "Before you can update service principal's OAuth2 permissions, you need to get the `objectId` of the permissions grant to update. You can retrieve it using the entra oauth2grant list command.",
            "If the `objectId` listed when using the entra oauth2grant list command has a minus sign ('-') prefix, you may receive an error indicating `--grantId` is missing. To resolve this issue simply escape the leading '-'.",
            "    m365 entra oauth2grant set --grantId \\\\-Zc1JRY8REeLxmXz5KtixAYU3Q6noCBPlhwGiX7pxmU",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "DelegatedPermissionGrant.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Update the existing OAuth2 permission grant with ID YgA60KYa4UOPSdc-lpxYEnQkr8KVLDpCsOXkiV8i-ek to the Calendars.Read Mail.Read permissions.",
            "m365 entra oauth2grant set --grantId YgA60KYa4UOPSdc-lpxYEnQkr8KVLDpCsOXkiV8i-ek --scope \"Calendars.Read Mail.Read\"",
        )],
        links: &[
            (
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
                "https://learn.microsoft.com/azure/active-directory/develop/active-directory-application-objects",
            ),
            (
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-update?view=graph-rest-1.0",
                "https://learn.microsoft.com/graph/api/oauth2permissiongrant-update?view=graph-rest-1.0",
            ),
        ],
    },
    Help {
        name: "entra organization list",
        summary: "Retrieves a list of organizations",
        options: &[(
            "properties",
            "The comma separated list of properties to be returned.",
        )],
        notes: &[
            "Applications granted the `User.Read` permission are able to read only the id, displayName, and verifiedDomains properties of the organization. All other properties return with null values. To read all properties, use at least `Organization.Read.All`.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "User.Read, Organization.Read.All",
            ),
            ("Application", "Microsoft Graph", "Organization.Read.All"),
        ],
        examples: &[
            ("Retrieve organizations", "m365 entra organization list"),
            (
                "Retrieve specific info",
                "m365 entra organization list --properties 'id,displayName,tenantType'",
            ),
        ],
        links: &[(
            "Organization",
            "https://learn.microsoft.com/graph/api/organization-list",
        )],
    },
    Help {
        name: "entra organization set",
        summary: "Updates info about the organization",
        options: &[
            ("id", "The id of the organization."),
            ("displayName", "The name of the organization."),
            (
                "marketingNotificationEmails",
                "The comma separated list of marketing notification emails.",
            ),
            (
                "securityComplianceNotificationMails",
                "The comma separated list of security compliance notification emails.",
            ),
            (
                "securityComplianceNotificationPhones",
                "The comma separated list of security compliance notification phones.",
            ),
            (
                "technicalNotificationMails",
                "The comma separated list of technical notification emails.",
            ),
            (
                "contactEmail",
                "A valid smtp email address for the privacy statement contact",
            ),
            (
                "statementUrl",
                "The URL that directs to the company's privacy statement",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Organization.ReadWrite.All"),
            (
                "Application",
                "Microsoft Graph",
                "Organization.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Updates properties of an organization specified by id",
                "m365 entra organization set --id 84841066-274d-4ec0-a5c1-276be684bdd3 --marketingNotificationEmails 'marketing@contoso.com'",
            ),
            (
                "Updates properties of an organization specified by displayName",
                "m365 entra organization set --displayName Contoso --marketingNotificationEmails 'marketing@contoso.com'",
            ),
        ],
        links: &[(
            "Organization",
            "https://learn.microsoft.com/graph/api/organization-update",
        )],
    },
    Help {
        name: "entra pim role assignment add",
        summary: "Requests activation of an Entra role assignment for a user or group.",
        options: &[
            (
                "roleDefinitionName",
                "Name of the role definition that should be assigned.",
            ),
            (
                "roleDefinitionId",
                "Id of the role definition that is being assigned.",
            ),
            (
                "userId",
                "Id of the user that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "userName",
                "UPN of the user that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "groupId",
                "Id of the group that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "groupName",
                "Display name of the group that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "administrativeUnitId",
                "Id of the administrative unit representing the scope of the assignment. If not specified, default scope will be tenant-wide.",
            ),
            (
                "applicationId",
                "Object Id of the application representing the scope of the assignment. If not specified, default scope will be tenant-wide.",
            ),
            ("justification", "An optional justification message."),
            (
                "startDateTime",
                "When the assignment should start. If left out, the assignment will start from the current time.",
            ),
            (
                "endDateTime",
                "When the assignment should end. Specify either `duration`, `endDateTime` or `noExpiration`.",
            ),
            (
                "duration",
                "How long the assignment should last. Write in ISO 8601 format for durations: PT3H for 3 hours. Specify either `duration`, `endDateTime` or `noExpiration`.",
            ),
            (
                "noExpiration",
                "If specified, the role assignment will never expire. Specify either `duration`, `endDateTime` or `noExpiration`.",
            ),
            (
                "ticketNumber",
                "Optional ticket number value to communicate with the request.",
            ),
            (
                "ticketSystem",
                "Optional ticket system to communicate with the request.",
            ),
        ],
        notes: &[
            "When activating a role for other users, you must be Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Request activation of the SharePoint Administrator Entra role assignment for the current user.",
                "m365 entra pim role assignment add --roleDefinitionName 'SharePoint Administrator'",
            ),
            (
                "Request activation of an Entra role assignment for the current user.",
                "m365 entra pim role assignment add --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690'",
            ),
            (
                "Request activation of an Entra role assignment for the current user with a justification and max duration of 4 hours.",
                "m365 entra pim role assignment add --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --justification 'Need Global Admin to release application xyz to production' --duration 'PT4H'",
            ),
            (
                "Request activation of an Entra role assignment for a specified user with tenant scope.",
                "m365 entra pim role assignment add --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --userId '3488d6b8-6b2e-41c3-9583-1991205323c2'",
            ),
            (
                "Request activation of the User Administrator Entra role assignment for a specified group with administrative unit scope.",
                "m365 entra pim role assignment add --roleDefinitionName 'User Administrator' --groupId '3488d6b8-6b2e-41c3-9583-1991205323c2' --administrativeUnitId '03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7'",
            ),
            (
                "Request activation of the Application Administrator Entra role assignment for a specified group with scope to a specific application.",
                "m365 entra pim role assignment add --roleDefinitionName 'Application Administrator' --groupName 'Applications admins' --applicationId '03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7'",
            ),
            (
                "Request activation of an Entra role assignment for a specific period of two days.",
                "m365 entra pim role assignment add --roleDefinitionName 'Global Administrator' --userName 'admin-john@contoso.com' --startDateTime '2024-01-10T09:00:00Z' --endDateTime '2024-01-11T17:00:00Z'",
            ),
            (
                "Request activation of an Entra role assignment with no expiration.",
                "m365 entra pim role assignment add --roleDefinitionName 'Global Administrator' --userName 'admin-john@contoso.com' --noExpiration",
            ),
        ],
        links: &[(
            "Role assignment request",
            "https://learn.microsoft.com/graph/api/rbacapplication-post-roleassignmentschedulerequests",
        )],
    },
    Help {
        name: "entra pim role assignment eligibility list",
        summary: "Retrieves a list of eligible roles a user or group can be assigned to",
        options: &[
            (
                "userId",
                "Id of the user for which to list eligible roles. If not specified, all eligible roles will be listed.",
            ),
            (
                "userName",
                "UPN of the user for which to list eligible roles. If not specified, all eligible roles will be listed.",
            ),
            (
                "groupId",
                "Id of the group for which to list eligible roles. If not specified, all eligible roles will be listed.",
            ),
            (
                "groupName",
                "Display name of the group for which to list eligible roles. If not specified, all eligible roles will be listed.",
            ),
            (
                "withPrincipalDetails",
                "An optional flag to include details of the principals that were eligible for a role.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleEligibilitySchedule.Read.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleEligibilitySchedule.Read.Directory",
            ),
        ],
        examples: &[
            (
                "Get a list of eligible roles for any user.",
                "m365 entra pim role assignment eligibility list",
            ),
            (
                "Get a list of eligible roles for the current user.",
                "m365 entra pim role assignment eligibility list --userId '@meID'",
            ),
            (
                "Get a list of eligible roles for any user with principal details.",
                "m365 entra pim role assignment eligibility list --withPrincipalDetails",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra pim role assignment list",
        summary: "Retrieves a list of Entra role assignments for a user or group.",
        options: &[
            (
                "userId",
                "Id of the user to list role assignments for. If not specified, all assigned roles will be listed.",
            ),
            (
                "userName",
                "UPN of the user to list role assignments for. If not specified, all assigned roles will be listed.",
            ),
            (
                "groupId",
                "Id of the group to list role assignments for. If not specified, all assigned roles will be listed.",
            ),
            (
                "groupName",
                "Display name of the group to list role assignments for. If not specified, all assigned roles will be listed.",
            ),
            (
                "startDateTime",
                "An optional ISO 8601 formatted date filter to search from.",
            ),
            (
                "withPrincipalDetails",
                "An optional flag to include details of the principals that were assigned a role.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleAssignmentSchedule.Read.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleAssignmentSchedule.Read.Directory",
            ),
        ],
        examples: &[
            (
                "Get a list of Entra role assignments.",
                "m365 entra pim role assignment list",
            ),
            (
                "Get a list of Entra role assignments for the current user.",
                "m365 entra pim role assignment list --userId '@meID'",
            ),
            (
                "Get a list of Entra role assignments since the first of January 2024.",
                "m365 entra pim role assignment list --startDateTime 2024-01-01T00:00:00Z",
            ),
            (
                "Get a list of Entra role assignments with principal details.",
                "m365 entra pim role assignment list --withPrincipalDetails",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra pim role assignment remove",
        summary: "Requests deactivation of an Entra role assignment for a user or group.",
        options: &[
            (
                "roleDefinitionName",
                "Name of the role definition that should be assigned.",
            ),
            (
                "roleDefinitionId",
                "Id of the role definition that is being assigned.",
            ),
            (
                "userId",
                "Id of the user that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "userName",
                "UPN of the user that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "groupId",
                "Id of the group that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "groupName",
                "Display name of the group that will be granted the assignment. If not specified, the current user will be used.",
            ),
            (
                "administrativeUnitId",
                "Id of the administrative unit representing the scope of the assignment. If not specified, default scope will be tenant-wide.",
            ),
            (
                "applicationId",
                "Object Id of the application representing the scope of the assignment. If not specified, default scope will be tenant-wide.",
            ),
            ("justification", "An optional justification message."),
            (
                "ticketNumber",
                "Optional ticket number value to communicate with the request.",
            ),
            (
                "ticketSystem",
                "Optional ticket system to communicate with the request.",
            ),
        ],
        notes: &[
            "When deactivating a role for other users, you must be Privileged Role Administrator.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Request deactivation of the SharePoint Administrator Entra role assignment for the current user.",
                "m365 entra pim role assignment remove --roleDefinitionName 'SharePoint Administrator'",
            ),
            (
                "Request deactivation of an Entra role assignment for the current user.",
                "m365 entra pim role assignment remove --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690'",
            ),
            (
                "Request deactivation of an Entra role assignment for the current user with a justification",
                "m365 entra pim role assignment remove --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --justification 'Need Global Admin to release application xyz to production'",
            ),
            (
                "Request deactivation of an Entra role assignment for a specified user with tenant scope.",
                "m365 entra pim role assignment remove --roleDefinitionId 'f1417aa3-bf0b-4cc5-a845-a0b2cf11f690' --userId '3488d6b8-6b2e-41c3-9583-1991205323c2'",
            ),
            (
                "Request deactivation of the User Administrator Entra role assignment for a specified group with administrative unit scope.",
                "m365 entra pim role assignment remove --roleDefinitionName 'User Administrator' --groupId '3488d6b8-6b2e-41c3-9583-1991205323c2' --administrativeUnitId '03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7'",
            ),
            (
                "Request deactivation of the Application Administrator Entra role assignment for a specified group with scope to a specific application.",
                "m365 entra pim role assignment remove --roleDefinitionName 'Application Administrator' --groupName 'Applications admins' --applicationId '03c4c9dc-6f0c-4c4f-a4e6-0c9ed80f54c7'",
            ),
        ],
        links: &[(
            "Role assignment request",
            "https://learn.microsoft.com/graph/api/rbacapplication-post-roleassignmentschedulerequests",
        )],
    },
    Help {
        name: "entra pim role request list",
        summary: "Retrieves a list of PIM requests for roles",
        options: &[
            (
                "userId",
                "Id of the user for which to list requests. If not specified, all requests will be listed.",
            ),
            (
                "userName",
                "UPN of the user for which to list requests. If not specified, all requests will be listed.",
            ),
            (
                "groupId",
                "Id of the group for which to list requests. If not specified, all requests will be listed.",
            ),
            (
                "groupName",
                "Display name of the group for which to list requests. If not specified, all requests will be listed.",
            ),
            (
                "createdDateTime",
                "An optional ISO 8601 formatted date filter to search from.",
            ),
            (
                "status",
                "An optional value to filter the list of requests. Allowed values are: `Canceled`, `Denied`, `Failed`, `Granted`, `PendingAdminDecision`, `PendingApproval`, `PendingProvisioning`, `PendingScheduleCreation`, `Provisioned`, `Revoked`, and `ScheduleCreated`.",
            ),
            (
                "withPrincipalDetails",
                "An optional flag to include details of the principals that were assigned a role.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleAssignmentSchedule.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Get a list of PIM requests for roles.",
                "m365 entra pim role request list",
            ),
            (
                "Get a list of PIM requests for the current user.",
                "m365 entra pim role request list --userId '@meID'",
            ),
            (
                "Get a list of PIM requests for a specified user since the first of January 2024.",
                "m365 entra pim role request list --createdDateTime 2024-01-01T00:00:00Z",
            ),
            (
                "Get a list of PIM requests with principal details.",
                "m365 entra pim role request list --withPrincipalDetails",
            ),
            (
                "Get a list of PIM requests with pending approvals.",
                "m365 entra pim role request list --status PendingApproval",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra policy list",
        summary: "Returns policies from Entra ID",
        options: &[(
            "type",
            "The type of policies to return. Allowed values `activityBasedTimeout`, `adminConsentRequest`, `appManagement`, `authenticationFlows`, `authenticationMethods`, `authenticationStrength`, `authorization`, `claimsMapping`, `conditionalAccess`, `crossTenantAccess`, `defaultAppManagement`, `deviceRegistration`, `featureRolloutPolicy`, `homeRealmDiscovery`, `identitySecurityDefaultsEnforcement`, `permissionGrant`, `roleManagement`, `tokenIssuance`, `tokenLifetime`. If omitted, all policies are returned.",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Policy.Read.All"),
            ("Application", "Microsoft Graph", "Policy.Read.All"),
        ],
        examples: &[
            (
                "Returns all policies from Entra ID.",
                "m365 entra policy list",
            ),
            (
                "Returns claim-mapping policies from Entra ID.",
                "m365 entra policy list --type \"claimsMapping\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/policy-overview?view=graph-rest-1.0",
            "https://learn.microsoft.com/graph/api/resources/policy-overview?view=graph-rest-1.0",
        )],
    },
    Help {
        name: "entra resourcenamespace list",
        summary: "Gets a list of the RBAC resource namespaces and their properties",
        options: &[],
        notes: &[
            "The command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
        ],
        examples: &[(
            "Retrieve all resource namespaces.",
            "m365 entra resourcenamespace list",
        )],
        links: &[],
    },
    Help {
        name: "entra roleassignment add",
        summary: "Assigns a Entra ID role to a user and specifies the scope for which the user has been granted access",
        options: &[
            ("roleDefinitionId", "Id of a role to be assigned."),
            ("roleDefinitionName", "Name of a role to be assigned."),
            (
                "principal",
                "Id of a user, group or service principal or user principal name or group mail nickname to which the assignment is granted.",
            ),
            ("userId", "Id of a user to which the assignment is scoped."),
            (
                "userName",
                "User principal name of a user to which the assignment is scoped.",
            ),
            (
                "administrativeUnitId",
                "Id of an administrative unit to which the assignment is scoped.",
            ),
            (
                "administrativeUnitName",
                "Name of an administrative unit to which the assignment is scoped.",
            ),
            (
                "applicationId",
                "Id of an application to which the assignment is scoped.",
            ),
            (
                "applicationObjectId",
                "Object Id of an application to which the assignment is scoped.",
            ),
            (
                "applicationName",
                "Name of an application to which the assignment is scoped.",
            ),
            (
                "servicePrincipalId",
                "Id of a service principal to which the assignment is scoped.",
            ),
            (
                "servicePrincipalName",
                "Name of a service principal to which the assignment is scoped.",
            ),
            (
                "groupId",
                "Id of a group to which the assignment is scoped.",
            ),
            (
                "groupName",
                "Name of a group to which the assignment is scoped.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Assign a role specified by name to a user specified by id and scope the assignment to the whole tenant",
                "m365 entra roleassignment add --roleDefinitionName 'SharePoint Administrator' --principal 7a2ca997-9461-402e-9882-58088a370889",
            ),
            (
                "Assign a role specified by id to a user specified by UPN and scope the assignment to a service principal specified by id",
                "m365 entra roleassignment add --roleDefinitionId 777b752-f9b7-4205-a2b1-5db0d6a0ccfc --principal john.doe@contoso.com --servicePrincipalId a4738dd8-fc0f-4646-87fb-47539f5c651b",
            ),
            (
                "Assign a role specified by name to a group specified by mail nickname and scope the assignment to an administrative unit specified by name",
                "m365 entra roleassignment add --roleDefinitionName 'User Administrator' --principal 'contoso-group' --administrativeUnitName 'Marketing'",
            ),
            (
                "Assign a role specified by name to a service principal specified by id and scope the assignment to an application specified by id",
                "m365 entra roleassignment add --roleDefinitionName 'Application Administrator' --principal ab2ca997-9461-402e-9882-58088a370889 --applicationId 47539fd8-fc0f-4646-87fb-a4738d5c651b",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra roledefinition add",
        summary: "Creates a custom Microsoft Entra ID role definition",
        options: &[
            ("displayName", "The display name for the role definition."),
            (
                "allowedResourceActions",
                "Comma-separated list of resource actions allowed for the role.",
            ),
            ("description", "The description for the role definition."),
            (
                "enabled",
                "Indicates if the role is enabled for the assignment. If not specified, the role is enabled by default.",
            ),
            ("version", "The version of the role definition."),
        ],
        notes: &[
            "Use the `m365 entra rolepermission list --resourceNamespace microsoft.directory` command to get a list of available resource actions.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Create a custom Microsoft Entra ID role",
                "m365 entra roledefinition add --displayName 'Application Remover' --description 'Allows to remove any Entra ID application' --allowedResourceActions 'microsoft.directory/applications/delete'",
            ),
            (
                "Create a custom Microsoft Entra ID role, but disable it for the assignment",
                "m365 entra roledefinition add --displayName 'Application Remover' --version '1.0' --enabled false --allowedResourceActions 'microsoft.directory/applications/delete,microsoft.directory/applications/owners/update'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra roledefinition get",
        summary: "Gets a specific Microsoft Entra ID role definition.",
        options: &[
            ("id", "The id of the role definition."),
            ("displayName", "The display name of the role definition."),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
        ],
        examples: &[
            (
                "Get info about a role specified by the id",
                "m365 entra roledefinition get --id 62e90394-69f5-4237-9190-012177145e10",
            ),
            (
                "Get limited info about a role specified by the display name",
                "m365 entra roledefinition get --displayName 'Custom SharePoint Role' --properties 'description,isEnabled'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra roledefinition list",
        summary: "Lists all Microsoft Entra ID role definitions.",
        options: &[
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            (
                "filter",
                "OData filter to apply when retrieving the role definitions.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.Read.Directory",
            ),
        ],
        examples: &[
            (
                "Retrieve all Microsoft Entra ID role definitions",
                "m365 entra roledefinition list",
            ),
            (
                "Retrieve only the names of the role definitions",
                "m365 entra roledefinition list --properties 'displayName'",
            ),
            (
                "Retrieve only custom role definitions",
                "m365 entra roledefinition list --filter 'isBuiltIn eq false'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra roledefinition remove",
        summary: "Removes a custom Microsoft Entra role definition",
        options: &[
            ("id", "The id of the role definition."),
            ("displayName", "The name of the role definition."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "When the role definition is removed, all the associated role assignments are deleted.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Remove a role definition specified by id without prompting",
                "m365 entra roledefinition remove --id 0bed8b86-5026-4a93-ac7d-56750cc099f1 --force",
            ),
            (
                "Remove a role definition specified by name and prompt for confirmation",
                "m365 entra roledefinition remove --displayName 'Custom Role'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra roledefinition set",
        summary: "Updates a custom Microsoft Entra ID role definition",
        options: &[
            ("id", "The id of the role definition to be updated."),
            (
                "displayName",
                "The display name of the role definition to be updated.",
            ),
            (
                "newDisplayName",
                "Updated display name for the role definition.",
            ),
            (
                "description",
                "Updated description for the role definition.",
            ),
            (
                "enabled",
                "Indicates if the role is enabled for the assignment.",
            ),
            (
                "allowedResourceActions",
                "Updated comma-separated list of resource actions allowed for the role.",
            ),
            ("version", "Updated version of the role definition."),
        ],
        notes: &[
            "Use the `m365 entra rolepermission list --resourceNamespace microsoft.directory` command to get a list of available resource actions.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RoleManagement.ReadWrite.Directory",
            ),
        ],
        examples: &[
            (
                "Update a custom Microsoft Entra ID role specified by the id",
                "m365 entra roledefinition set --id fadbc488-151d-4431-9143-6abbffae759f --newDisplayName 'Application Remover' --description 'Allows to remove any Entra ID application' --allowedResourceActions 'microsoft.directory/applications/delete'",
            ),
            (
                "Update a custom Microsoft Entra ID role specified by the display name",
                "m365 entra roledefinition set --displayName 'Application Remover' --version '1.0' --enabled true --allowedResourceActions 'microsoft.directory/applications/delete,microsoft.directory/applications/owners/update'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra rolepermission list",
        summary: "Lists all Microsoft Entra ID role permissions",
        options: &[
            (
                "resourceNamespace",
                "The namespace of the resource for which to retrieve role permissions.",
            ),
            ("privileged", "Retrieve only sensitive role permissions."),
        ],
        notes: &[
            "The command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "Use the `m365 entra resourcenamespace list` command to get a list of available resource namespaces.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get a list of role permissions",
                "m365 entra rolepermission list --resourceNamespace 'microsoft.directory'",
            ),
            (
                "Get a list of sensitive role permissions",
                "m365 entra rolepermission list --resourceNamespace 'microsoft.directory' --privileged",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra siteclassification disable",
        summary: "Disables site classification",
        options: &[(
            "force",
            "Don't prompt for confirming disabling site classification.",
        )],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Disable site classification.",
                "m365 entra siteclassification disable",
            ),
            (
                "Disable site classification without confirmation.",
                "m365 entra siteclassification disable --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
        )],
    },
    Help {
        name: "entra siteclassification enable",
        summary: "Enables site classification configuration",
        options: &[
            (
                "classifications",
                "Comma-separated list of classifications to enable in the tenant.",
            ),
            ("defaultClassification", "Classification to use by default."),
            (
                "usageGuidelinesUrl",
                "URL with usage guidelines for members.",
            ),
            (
                "guestUsageGuidelinesUrl",
                "URL with usage guidelines for guests.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Enable site classification.",
                "m365 entra siteclassification enable --classifications \"High, Medium, Low\" --defaultClassification \"Medium\"",
            ),
            (
                "Enable site classification with a usage guidelines URL.",
                "m365 entra siteclassification enable --classifications \"High, Medium, Low\" --defaultClassification \"Medium\" --usageGuidelinesUrl \"http://aka.ms/pnp\"",
            ),
            (
                "Enable site classification with usage guidelines URLs for guests and members.",
                "m365 entra siteclassification enable --classifications \"High, Medium, Low\" --defaultClassification \"Medium\" --usageGuidelinesUrl \"http://aka.ms/pnp\" --guestUsageGuidelinesUrl \"http://aka.ms/pnp\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
        )],
    },
    Help {
        name: "entra siteclassification get",
        summary: "Gets site classification configuration",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "GroupSettings.Read.All"),
            ("Application", "Microsoft Graph", "GroupSettings.Read.All"),
        ],
        examples: &[(
            "Get information about the Microsoft 365 Tenant site classification.",
            "m365 entra siteclassification get",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
        )],
    },
    Help {
        name: "entra siteclassification set",
        summary: "Updates site classification configuration",
        options: &[
            (
                "classifications",
                "Comma-separated list of classifications.",
            ),
            ("defaultClassification", "Classification to use by default."),
            (
                "usageGuidelinesUrl",
                "URL with usage guidelines for members.",
            ),
            (
                "guestUsageGuidelinesUrl",
                "URL with usage guidelines for guests.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "GroupSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Update Microsoft 365 Tenant site classification configuration.",
                "m365 entra siteclassification set --classifications \"High, Medium, Low\" --defaultClassification \"Medium\"",
            ),
            (
                "Update only the default classification.",
                "m365 entra siteclassification set --defaultClassification \"Low\"",
            ),
            (
                "Update site classification with a usage guidelines URL.",
                "m365 entra siteclassification set --usageGuidelinesUrl \"http://aka.ms/pnp\"",
            ),
            (
                "Update site classification with usage guidelines URLs for guests and members.",
                "m365 entra siteclassification set --usageGuidelinesUrl \"http://aka.ms/pnp\" --guestUsageGuidelinesUrl \"http://aka.ms/pnp\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
            "https://learn.microsoft.com/sharepoint/dev/solution-guidance/modern-experience-site-classification",
        )],
    },
    Help {
        name: "entra user add",
        summary: "Creates a new user",
        options: &[
            (
                "displayName",
                "The name to display in the address book for the user.",
            ),
            (
                "userName",
                "The user principal name (someuser@contoso.com).",
            ),
            (
                "accountEnabled",
                "Whether the account is enabled. Allowed values: `true`, `false`. Defaults to `true`.",
            ),
            (
                "mailNickname",
                "The mail alias for the user. By default this value will be extracted from `userName`.",
            ),
            (
                "password",
                "The password for the user. When not specified, a password will be generated.",
            ),
            (
                "firstName",
                "The given name (first name) of the user. Maximum length is 64 characters.",
            ),
            (
                "lastName",
                "The user's surname (family name or last name). Maximum length is 64 characters.",
            ),
            (
                "forceChangePasswordNextSignIn",
                "Whether the user should change his/her password on the next login.",
            ),
            (
                "forceChangePasswordNextSignInWithMfa",
                "Whether the user should change his/her password on the next login and setup MFA.",
            ),
            (
                "usageLocation",
                "A two letter country code (ISO standard 3166). Required for users that will be assigned licenses.",
            ),
            (
                "officeLocation",
                "The office location in the user's place of business.",
            ),
            (
                "jobTitle",
                "The user's job title. Maximum length is 128 characters.",
            ),
            (
                "companyName",
                "The company name which the user is associated. The maximum length is 64 characters.",
            ),
            (
                "department",
                "The name for the department in which the user works. Maximum length is 64 characters.",
            ),
            (
                "preferredLanguage",
                "The preferred language for the user. Should follow ISO 639-1 Code. Example: `en-US`.",
            ),
            (
                "managerUserId",
                "User ID of the user's manager. Specify `managerUserId` or `managerUserName` but not both.",
            ),
            (
                "managerUserName",
                "User principal name of the manager. Specify `managerUserId` or `managerUserName` but not both.",
            ),
        ],
        notes: &[
            "After running this command, it may take a minute before the user is effectively created in the tenant.",
            "This command allows using unknown options. For a comprehensive list of user properties, please refer to the Graph documentation page.",
            "If the specified option is not found, you will receive a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
            "To use this command you must be a Global administrator, User administrator, or Privileged Authentication administrator.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.ReadWrite.All"),
            ("Application", "Microsoft Graph", "User.ReadWrite.All"),
        ],
        examples: &[
            (
                "Create a user and let him/her update the password at first login.",
                "m365 entra user add --displayName \"John Doe\" --userName \"john.doe@contoso.com\" --password \"SomePassw0rd\" --forceChangePasswordNextSignIn",
            ),
            (
                "Create a user with job information.",
                "m365 entra user add --displayName \"John Doe\" --userName \"john.doe@contoso.com\" --password \"SomePassw0rd\" --firstName John --lastName Doe --jobTitle \"Sales Manager\" --companyName Contoso --department Sales --officeLocation Vosselaar --forceChangePasswordNextSignIn",
            ),
            (
                "Create a user with language information.",
                "m365 entra user add --displayName \"John Doe\" --userName \"john.doe@contoso.com\" --preferredLanguage \"nl-BE\" --usageLocation BE --forceChangePasswordNextSignIn",
            ),
            (
                "Create a user with a manager.",
                "m365 entra user add --displayName \"John Doe\" --userName \"john.doe@contoso.com\" --managerUserName \"adele@contoso.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user get",
        summary: "Gets information about the specified user",
        options: &[
            ("id", "The ID of the user to retrieve information for."),
            (
                "userName",
                "The name of the user to retrieve information for.",
            ),
            (
                "email",
                "The email of the user to retrieve information for.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
            ("withManager", "Retrieve the direct manager of the user."),
        ],
        notes: &[
            "If the user with the specified id, user name, or email doesn't exist, you will get a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Application", "Microsoft Graph", "User.Read.All"),
        ],
        examples: &[
            (
                "Get information about the user by id.",
                "m365 entra user get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
            ),
            (
                "Get information about the user by user name.",
                "m365 entra user get --userName AarifS@contoso.onmicrosoft.com",
            ),
            (
                "Get information about the user by email.",
                "m365 entra user get --email AarifS@contoso.onmicrosoft.com",
            ),
            (
                "For the user with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 retrieve the user name, e-mail address and full name.",
                "m365 entra user get --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --properties \"userPrincipalName,mail,displayName\"",
            ),
            (
                "Get information about the currently logged user using the Id token.",
                "m365 entra user get --id \"@meId\"",
            ),
            (
                "Get information about the currently logged in user using the UserName token.",
                "m365 entra user get --userName \"@meUserName\"",
            ),
            (
                "Get information about a user with the manager's details.",
                "m365 entra user get --userName AarifS@contoso.onmicrosoft.com --withManager",
            ),
        ],
        links: &[(
            "https://developer.microsoft.com/en-us/graph/docs/api-reference/v1.0/resources/user#properties",
            "https://developer.microsoft.com/en-us/graph/docs/api-reference/v1.0/resources/user#properties",
        )],
    },
    Help {
        name: "entra user groupmembership list",
        summary: "Retrieves all groups where the user is a member of",
        options: &[
            ("userId", "The ID of the user to retrieve information for."),
            (
                "userName",
                "The name of the user to retrieve information for.",
            ),
            (
                "userEmail",
                "The email of the user to retrieve information for.",
            ),
            (
                "securityEnabledOnly",
                "Specifies that only security-enabled groups that the user is a member of should be returned",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "User.ReadBasic.All, GroupMember.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "User.ReadBasic.All, GroupMember.Read.All",
            ),
        ],
        examples: &[
            (
                "Retrieves groups that the user is a member of",
                "m365 entra user groupmembership list --userId '1caf7dcd-7e83-4c3a-94f7-932a1299c844'",
            ),
            (
                "Retrieves only security groups that the user is a member of",
                "m365 entra user groupmembership list --userName 'john.doe@contoso.com' --securityEnabledOnly",
            ),
            (
                "Retrieves groups where the currently logged user is a member of",
                "m365 entra user groupmembership list --userId '@meId'",
            ),
            (
                "Retrieves groups where the currently logged user is a member of",
                "m365 entra user groupmembership list --userName '@meUserName'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user guest add",
        summary: "Invites an external user to the organization",
        options: &[
            ("emailAddress", "The email address of the user."),
            ("displayName", "The display name of the user."),
            (
                "inviteRedirectUrl",
                "The URL the user should be redirected to once the invitation is redeemed. If not specified, default URL https://myapplications.microsoft.com will be set.",
            ),
            (
                "welcomeMessage",
                "Personal welcome message which will be added to the email along with the default email.",
            ),
            (
                "ccRecipients",
                "Additional recipients the invitation message should be sent to. Currently only 1 additional recipient is supported.",
            ),
            (
                "messageLanguage",
                "The language you want to send the default message in. The language format should be in ISO 639. The default is `en-US`.",
            ),
            (
                "sendInvitationMessage",
                "Indicates whether an email should be sent to the user.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Invite.All"),
            ("Application", "Microsoft Graph", "User.Invite.All"),
        ],
        examples: &[
            (
                "Invite a user via email and set the display name.",
                "m365 entra user guest add --emailAddress john.doe@contoso.com --displayName \"John Doe\" --sendInvitationMessage",
            ),
            (
                "Invite a user with a custom email and custom redirect url.",
                "m365 entra user guest add --emailAddress john.doe@contoso.com --welcomeMessage \"Hi John, welcome to the organization!\" --inviteRedirectUrl https://contoso.sharepoint.com --sendInvitationMessage",
            ),
            (
                "Invite a user and send an invitation mail in Dutch.",
                "m365 entra user guest add --emailAddress john.doe@contoso.com --messageLanguage nl-BE --sendInvitationMessage",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user hibp",
        summary: "Allows you to retrieve all accounts that have been pwned with the specified username",
        options: &[
            (
                "userName",
                "The name of the user to retrieve information for.",
            ),
            (
                "apiKey",
                "Have I been pwned `API Key`. You can buy it from https://haveibeenpwned.com/API/Key",
            ),
            (
                "domain",
                "Limit the returned breaches only contain results with the domain specified.",
            ),
        ],
        notes: &[
            "If the user with the specified user name doesn't involved in any breach, you will get a `No pwnage found` message when running in debug or verbose mode.",
            "If `API Key` is invalid, you will get a `Required option apiKey not specified` error.",
            "This command does not require any Microsoft 365 permissions.",
        ],
        permissions: &[],
        examples: &[
            (
                "Check if user with by a user name is in a data breach.",
                "m365 entra user hibp --userName account-exists@hibp-integration-tests.com --apiKey _YOUR-API-KEY_",
            ),
            (
                "Check if user by a user name is in a data breach against the domain specified.",
                "m365 entra user hibp --userName account-exists@hibp-integration-tests.com --apiKey _YOUR-API-KEY_ --domain adobe.com",
            ),
        ],
        links: &[(
            "https://haveibeenpwned.com/API/v3",
            "https://haveibeenpwned.com/API/v3",
        )],
    },
    Help {
        name: "entra user license add",
        summary: "Assigns a license to a user",
        options: &[
            ("userId", "The ID of the user."),
            ("userName", "User principal name of the user."),
            (
                "ids",
                "A comma separated list of IDs that specify the licenses to add.",
            ),
        ],
        notes: &["The user must have a `usageLocation` value in order to assign a license to it."],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "LicenseAssignment.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "LicenseAssignment.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Assign specific licenses to a specific user by UPN.",
                "m365 entra user license add --userName \"john.doe@contoso.com\" --ids \"45715bb8-13f9-4bf6-927f-ef96c102d394,bea13e0c-3828-4daa-a392-28af7ff61a0f\"",
            ),
            (
                "Assign specific licenses from a specific user by ID.",
                "m365 entra user license add --userId \"5c241023-2ba5-4ea8-a516-a2481a3e6c51\" --ids \"45715bb8-13f9-4bf6-927f-ef96c102d394,bea13e0c-3828-4daa-a392-28af7ff61a0f\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user license list",
        summary: "Lists the license details for a given user",
        options: &[
            (
                "userId",
                "The ID of the user. Specify either `userId` or `userName` but not both.",
            ),
            (
                "userName",
                "User principal name of the user. Specify either `userId` or `userName` but not both.",
            ),
        ],
        notes: &[
            "If you don't specify any option, the command will list the license details of the current logged in user.",
            "This command does not support application permissions.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "LicenseAssignment.Read.All")],
        examples: &[
            (
                "List license details of the current logged in user.",
                "m365 entra user license list",
            ),
            (
                "List license details of a specific user by its UPN.",
                "m365 entra user license list --userName john.doe@contoso.com",
            ),
            (
                "List license details of a specific user by its ID.",
                "m365 entra user license list --userId 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user license remove",
        summary: "Removes a license from a user",
        options: &[
            ("userId", "The ID of the user."),
            ("userName", "User principal name of the user."),
            (
                "ids",
                "A comma separated list of IDs that specify the licenses to remove.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "LicenseAssignment.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "LicenseAssignment.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Remove specific licenses from a specific user by UPN.",
                "m365 entra user license remove --userName \"john.doe@contoso.com\" --ids \"45715bb8-13f9-4bf6-927f-ef96c102d394,bea13e0c-3828-4daa-a392-28af7ff61a0f\"",
            ),
            (
                "Remove specific licenses from a specific user by ID.",
                "m365 entra user license remove --userId 5c241023-2ba5-4ea8-a516-a2481a3e6c51 --ids \"45715bb8-13f9-4bf6-927f-ef96c102d394,bea13e0c-3828-4daa-a392-28af7ff61a0f\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user list",
        summary: "Lists users matching specified criteria",
        options: &[
            (
                "type",
                "Filter the results to only users of a given type: `Member` or `Guest`. By default, all users are listed.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of user properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will output the default properties returned by Graph.",
            "When the `properties` option includes values with a `/`, for example: `manager/displayName`, an additional `$expand` query parameter will be included on `manager`.",
            "To filter the list of users, include additional options that match the user property that you want to filter with. For example `--displayName Patt` will return all users whose `displayName` starts with `Patt`. Multiple filters will be combined using the `and` operator.",
            "Certain properties cannot be returned within a user collection. The following properties are only supported when retrieving an single user using: `aboutMe`, `birthday`, `hireDate`, `interests`, `mySite`, `pastProjects`, `preferredName`, `responsibilities`, `schools`, `skills`, `mailboxSettings`.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Application", "Microsoft Graph", "User.Read.All"),
        ],
        examples: &[
            ("List all users in the tenant.", "m365 entra user list"),
            (
                "List all guest users in the tenant.",
                "m365 entra user list --type Guest",
            ),
            (
                "List all users in the tenant. For each one return the display name and e-mail address.",
                "m365 entra user list --properties \"displayName,mail\"",
            ),
            (
                "Show users whose display name starts with Patt.",
                "m365 entra user list --displayName Patt",
            ),
            (
                "Show all account managers whose display name starts with Patt.",
                "m365 entra user list --displayName Patt --jobTitle 'Account manager'",
            ),
            (
                "List users from the tenant. For each one return the display name, e-mail address, and manager information.",
                "m365 entra user list --properties \"displayName,mail,manager/*\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/resources/user?view=graph-rest-1.0#properties",
            "https://learn.microsoft.com/graph/api/resources/user?view=graph-rest-1.0#properties",
        )],
    },
    Help {
        name: "entra user password validate",
        summary: "Checks a user's password against the organization's password validation policy",
        options: &[("password", "The password to be validated.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "User.ReadWrite")],
        examples: &[(
            "Validate password cli365P@ssW0rd against the organization's password validation policy.",
            "m365 entra user password validate --password \"cli365P@ssW0rd\"",
        )],
        links: &[],
    },
    Help {
        name: "entra user recyclebinitem clear",
        summary: "Removes all users from the tenant recycle bin",
        options: &[("force", "Don't prompt for confirmation.")],
        notes: &[
            "To use this command you must be a Global administrator, User administrator or Privileged Authentication administrator.",
            "After running this command, it may take a minute before all deleted users are effectively removed from the tenant.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.DeleteRestore.All"),
            ("Application", "Microsoft Graph", "User.DeleteRestore.All"),
        ],
        examples: &[
            (
                "Removes all users from the tenant recycle bin.",
                "m365 entra user recyclebinitem clear",
            ),
            (
                "Removes all users from the tenant recycle bin without confirmation prompt.",
                "m365 entra user recyclebinitem clear --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user recyclebinitem list",
        summary: "Lists users from the recycle bin in the current tenant",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Application", "Microsoft Graph", "User.Read.All"),
        ],
        examples: &[(
            "List all removed users.",
            "m365 entra user recyclebinitem list",
        )],
        links: &[],
    },
    Help {
        name: "entra user recyclebinitem remove",
        summary: "Removes a user from the recycle bin in the current tenant",
        options: &[
            ("id", "ID of the deleted user."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be a Global administrator, User administrator or Privileged Authentication administrator.",
            "After running this command, it may take a minute before the deleted user is effectively removed from the tenant.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.DeleteRestore.All"),
            ("Application", "Microsoft Graph", "User.DeleteRestore.All"),
        ],
        examples: &[
            (
                "Removes a specific user from the recycle bin.",
                "m365 entra user recyclebinitem remove --id 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
            (
                "Removes a specific user from the recycle bin without confirmation prompt.",
                "m365 entra user recyclebinitem remove --id 59f80e08-24b1-41f8-8586-16765fd830d3 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user recyclebinitem restore",
        summary: "Restores a user from the recycle bin in the current tenant",
        options: &[("id", "ID of the deleted user.")],
        notes: &[
            "To use this command you must be a Global administrator, User administrator or Privileged Authentication administrator",
            "After running this command, it may take a minute before the user will reappear in the active users.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.DeleteRestore.All"),
            ("Application", "Microsoft Graph", "User.DeleteRestore.All"),
        ],
        examples: &[(
            "Restore user from the recycle bin.",
            "m365 entra user recyclebinitem restore --id 990e2425-f595-43bc-85ed-b89a44093793",
        )],
        links: &[],
    },
    Help {
        name: "entra user registrationdetails list",
        summary: "Retrieves a list of the authentication methods registered for users",
        options: &[
            (
                "isAdmin",
                "Filter for users who are admins. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "userType",
                "Filter for members or guest users. Allowed values are `member` or `guest`. If not specified, returns all users.",
            ),
            (
                "userPreferredMethodForSecondaryAuthentication",
                "Filter users by selected method as default second-factor authentication. Allowed values are `push`, `oath`, `voiceMobile`, `voiceAlternateMobile`, `voiceOffice`, `sms` or `none`. Specify either one method or more methods separated by a comma.",
            ),
            (
                "systemPreferredAuthenticationMethods",
                "Filter users by most secure authentication methods registered for second-factor authentication. Allowed values are `push`, `oath`, `voiceMobile`, `voiceAlternateMobile`, `voiceOffice`, `sms` or `none`. Specify either one method or more methods separated by a comma.",
            ),
            (
                "isSelfServicePasswordResetRegistered",
                "Filter for users who have registered for self-service password reset. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isSelfServicePasswordResetEnabled",
                "Filter for users who have been enabled for self-service password reset. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isSelfServicePasswordResetCapable",
                "Filter for users who are ready to perform self-service password reset. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isMfaRegistered",
                "Filter for users who are registered for multi-factor authentication. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isMfaCapable",
                "Filter for users who are ready to perform password reset or multi-factor authentication. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isPasswordlessCapable",
                "Filter for user who have registered a password less strong authentication method. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "isSystemPreferredAuthenticationMethodEnabled",
                "Filter for users who have enabled system preferred authentication method. Allowed values are `true` or `false`. If not specified, returns all users.",
            ),
            (
                "methodsRegistered",
                "Filter users by registered methods used during registration. Allowed values are `mobilePhone`, `email`, `fido2`, `microsoftAuthenticatorPush` or `softwareOneTimePasscode`. Specify either one method or more methods separated by a comma.",
            ),
            (
                "userIds",
                "Filter users by ids. Specify at most 20 ids separated by a comma.",
            ),
            (
                "userPrincipalNames",
                "Filter users by user principal names. Specify at most 20 UPN separated by a comma.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve.",
            ),
        ],
        notes: &[
            "Using the `--properties` option, you can specify a comma-separated list of registration details properties to retrieve from the Microsoft Graph. If you don't specify any properties, the command will retrieve user's account name, registered methods and timestamp of last update.",
            "When multiple values are specified for `--userPreferredMethodForSecondaryAuthentication` option, the command returns registration details with at least one specified selected method as default second-factor authentication.",
            "When multiple values are specified for `--systemPreferredAuthenticationMethods` option, the command returns registration details with at least one specified most secure authentication methods registered for second-factor authentication.",
            "When multiple values are specified for `--registeredMethods` option, the command returns registration details with at least one specified registered methods used during registration.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "AuditLog.Read.All"),
            ("Application", "Microsoft Graph", "AuditLog.Read.All"),
        ],
        examples: &[
            (
                "Retrieve registration details for all users.",
                "m365 entra user registrationdetails list",
            ),
            (
                "Retrieve user registration details and returns only specific properties.",
                "m365 entra user registrationdetails list --properties 'id,isAdmin'",
            ),
            (
                "Retrieve registration details for admins.",
                "m365 entra user registrationdetails list --isAdmin true",
            ),
            (
                "Retrieve registration details for guest users.",
                "m365 entra user registrationdetails list --userType guest",
            ),
            (
                "Retrieve registration details for users who selected either sms or push authentication method as the default second-factor for performing multifactor authentication.",
                "m365 entra user registrationdetails list --userPreferredMethodForSecondaryAuthentication 'sms,push'",
            ),
            (
                "Retrieve registration details for users with push authentication method as the most secure authentication method among the registered methods for second factor authentication determined by the system.",
                "m365 entra user registrationdetails list --systemPreferredAuthenticationMethods push",
            ),
            (
                "Retrieve registration details for users who have used either Microsoft Authenticator app or mobile phone during registration.",
                "m365 entra user registrationdetails list --methodsRegistered 'microsoftAuthenticatorPush,mobilePhone'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user remove",
        summary: "Removes a specific user",
        options: &[
            ("id", "The ID of the user."),
            ("userName", "User principal name of the user."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "If the user with the specified id or user name doesn't exist, you will get a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
            "After running this command, it may take a minute before the user is effectively moved to the recycle bin.",
            "To use this command you must be a Global administrator, User administrator, or Privileged Authentication administrator.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.ReadWrite.All"),
            ("Application", "Microsoft Graph", "User.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a specific user by id.",
                "m365 entra user remove --id a33bd401-9117-4e0e-bb7b-3f61c1539e10",
            ),
            (
                "Removes a specific user by its UPN.",
                "m365 entra user remove --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user session revoke",
        summary: "Revokes all sign-in sessions for a given user",
        options: &[
            ("userId", "The id of the user."),
            ("userName", "The user principal name of the user."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either User Administrator or Global Administrator.",
            "There might be a small delay of a few minutes before tokens are revoked.",
            "This API doesn't revoke sign-in sessions for external users, because external users sign in through their home tenant.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.RevokeSessions.All"),
            ("Application", "Microsoft Graph", "User.RevokeSessions.All"),
        ],
        examples: &[
            (
                "Revoke sign-in sessions of a user specified by id",
                "m365 entra user session revoke --userId 4fb72b9b-d0b0-4a35-8bc1-83f9a6488c48",
            ),
            (
                "Revoke sign-in sessions of a user specified by its UPN",
                "m365 entra user session revoke --userName john.doe@contoso.onmicrosoft.com",
            ),
            (
                "Revoke sign-in sessions of a user specified by its UPN without prompting for confirmation",
                "m365 entra user session revoke --userName john.doe@contoso.onmicrosoft.com --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user set",
        summary: "Updates information of the specified user",
        options: &[
            ("id", "The object ID of the user to update."),
            ("userName", "User principal name of the user to update."),
            (
                "accountEnabled",
                "Boolean value specifying whether the account is enabled. Valid values are `true` or `false`.",
            ),
            (
                "resetPassword",
                "If specified, the password of the user will be reset. This will make the parameter `newPassword` required.",
            ),
            (
                "forceChangePasswordNextSignIn",
                "If specified, the user will have to change his password the next time they log in. Can only be set in combination with `resetPassword`.",
            ),
            (
                "forceChangePasswordNextSignInWithMfa",
                "Whether the user should change his/her password on the next login and setup MFA. Can only be set in combination with `resetPassword`.",
            ),
            (
                "currentPassword",
                "Current password of the user that is signed in. If this parameter is set, `newPassword` is mandatory. Can't be combined with `resetPassword`.",
            ),
            (
                "newPassword",
                "New password to be set. Must be set when specifying either `resetPassword` or `currentPassword`.",
            ),
            (
                "displayName",
                "The name to display in the address book for the user.",
            ),
            (
                "firstName",
                "The given name (first name) of the user. Maximum length is 64 characters.",
            ),
            (
                "lastName",
                "The user's surname (family name or last name). Maximum length is 64 characters.",
            ),
            (
                "usageLocation",
                "A two letter country code (ISO standard 3166). Required for users that will be assigned licenses.",
            ),
            (
                "officeLocation",
                "The office location in the user's place of business.",
            ),
            (
                "jobTitle",
                "The user's job title. Maximum length is 128 characters.",
            ),
            (
                "companyName",
                "The company name which the user is associated. The maximum length is 64 characters.",
            ),
            (
                "department",
                "The name for the department in which the user works. Maximum length is 64 characters.",
            ),
            (
                "preferredLanguage",
                "The preferred language for the user. Should follow ISO 639-1 Code. Example: `en-US`.",
            ),
            (
                "managerUserId",
                "User ID of the user's manager. Specify `managerUserId`, `managerUserName` or `removeManager` but not both.",
            ),
            (
                "managerUserName",
                "User principal name of the manager. Specify `managerUserId`, `managerUserName` or `removeManager` but not both.",
            ),
            (
                "removeManager",
                "Remove currently set manager. The user will have no manager when this flag is set. Specify `managerUserId`, `managerUserName` or `removeManager` but not both.",
            ),
        ],
        notes: &[
            "This command allows using unknown options. For a comprehensive list of user properties, please refer to the Graph documentation page.",
            "If the user with the specified ID or username doesn't exist, or if the specified option is not found, you will receive a `Resource 'xyz' does not exist or one of its queried reference-property objects are not present.` error.",
            "To use this command you must be a Global administrator, User administrator, or Privileged Authentication administrator.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.ReadWrite.All"),
            ("Application", "Microsoft Graph", "User.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update specific property department of user with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
                "m365 entra user set --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --department IT",
            ),
            (
                "Update multiple properties of user by name",
                "m365 entra user set --userName steve@contoso.onmicrosoft.com --firstName John --lastName Doe --jobTitle \"Sales Manager\" --companyName Contoso --department Sales --officeLocation \"New York\"",
            ),
            (
                "Enable user with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
                "m365 entra user set --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --accountEnabled true",
            ),
            (
                "Disable user with id 1caf7dcd-7e83-4c3a-94f7-932a1299c844",
                "m365 entra user set --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --accountEnabled false",
            ),
            (
                "Reset password of a given user by userName and require the user to change the password on the next sign in",
                "m365 entra user set --userName steve@contoso.onmicrosoft.com --resetPassword --newPassword 6NLUId79Lc24 --forceChangePasswordNextSignIn",
            ),
            (
                "Change password of the currently logged in user",
                "m365 entra user set --id 1caf7dcd-7e83-4c3a-94f7-932a1299c844 --currentPassword SLBF5gnRtyYc --newPassword 6NLUId79Lc24",
            ),
            (
                "Updates a user with a manager",
                "m365 entra user set --displayName \"John Doe\" --userName \"john.doe@contoso.com\" --managerUserName \"adele@contoso.com\"",
            ),
            (
                "Updates a user by removing its manager",
                "m365 entra user set --userName \"john.doe@contoso.com\" --removeManager",
            ),
        ],
        links: &[],
    },
    Help {
        name: "entra user signin list",
        summary: "Retrieves the Entra ID user sign-ins for the tenant",
        options: &[
            (
                "userName",
                "Filter the user sign-ins by given User's UPN (user principal name), eg. `johndoe@example.com`. Specify either userName or userId",
            ),
            (
                "userId",
                "Filter the user sign-ins by given User's Id. Specify either userName or userId",
            ),
            (
                "appDisplayName",
                "Filter the user sign-ins by the given application display name. Specify either appDisplayName or appId",
            ),
            (
                "appId",
                "Filter the user sign-ins by the given application identifier. Specify either appDisplayName or appId",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "AuditLog.Read.All"),
            ("Application", "Microsoft Graph", "AuditLog.Read.All"),
        ],
        examples: &[
            (
                "Get all user's sign-ins in your tenant.",
                "m365 entra user signin list",
            ),
            (
                "Get all user's sign-ins filter by given user's UPN in the tenant.",
                "m365 entra user signin list --userName 'johndoe@example.com'",
            ),
            (
                "Get all user's sign-ins filter by given user's Id in the tenant.",
                "m365 entra user signin list --userId '11111111-1111-1111-1111-111111111111'",
            ),
            (
                "Get all user's sign-ins filter by given application display name in the tenant.",
                "m365 entra user signin list --appDisplayName 'Graph explorer'",
            ),
            (
                "Get all user's sign-ins filter by given application identifier in the tenant.",
                "m365 entra user signin list --appId '00000000-0000-0000-0000-000000000000'",
            ),
            (
                "Get all user's sign-ins filter by given user's UPN and application display name in the tenant.",
                "m365 entra user signin list --userName 'johndoe@example.com' --appDisplayName 'Graph explorer'",
            ),
            (
                "Get all user's sign-ins filter by given user's Id and application display name in the tenant.",
                "m365 entra user signin list --userId '11111111-1111-1111-1111-111111111111' --appDisplayName 'Graph explorer'",
            ),
            (
                "Get all user's sign-ins filter by given user's UPN and application identifier in the tenant.",
                "m365 entra user signin list --userName 'johndoe@example.com' --appId '00000000-0000-0000-0000-000000000000'",
            ),
            (
                "Get all user's sign-ins filter by given user's Id and application identifier in the tenant.",
                "m365 entra user signin list --userId '11111111-1111-1111-1111-111111111111' --appId '00000000-0000-0000-0000-000000000000'",
            ),
        ],
        links: &[],
    },
];

//! Help for the `m365 spe ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "spe container activate",
        summary: "Activates a container",
        options: &[("id", "The Id of the container instance.")],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[(
            "Activates a container.",
            "m365 spe container activate --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
        )],
        links: &[],
    },
    Help {
        name: "spe container add",
        summary: "Creates a new container",
        options: &[
            ("name", "The display name of the new container."),
            ("description", "The description of the new container."),
            (
                "containerTypeId",
                "The container type ID of the container instance.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance.",
            ),
            (
                "ocrEnabled",
                "Indicates whether Optical Character Recognition (OCR) is enabled for the container. Possible values: `true`, `false`. Defaults to `false`.",
            ),
            (
                "itemMajorVersionLimit",
                "The maximum major versions allowed for items in the container. Defaults to `500`.",
            ),
            (
                "itemVersioningEnabled",
                "Indicates whether versioning is enabled for items in the container. Possible values: `true`, `false`. Defaults to `true`.",
            ),
        ],
        notes: &[
            "This command only supports application permissions when using the `containerTypeId` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Creates a new container by specifying the container type ID",
                "m365 spe container add --name Invoices --containerTypeId bba89883-47c2-455b-956b-7a3d8db007fb",
            ),
            (
                "Creates a new container by specifying the container type name",
                "m365 spe container add --name Invoices --containerTypeName \"Invoice app container type\"",
            ),
            (
                "Creates a new container with additional options",
                "m365 spe container add --name Invoices --containerTypeId bba89883-47c2-455b-956b-7a3d8db007fb --ocrEnabled true --itemMajorVersionLimit 200 --itemVersioningEnabled true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container get",
        summary: "Gets a container of a specific container type",
        options: &[
            ("id", "The Id of the container instance."),
            ("name", "Display name of the container."),
            (
                "containerTypeId",
                "The container type ID of the container instance. Specify either `containerTypeId` or `containerTypeName` when using `name` but not both.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance. Specify either `containerTypeId` or `containerTypeName` when using `name` but not both.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Gets a container of a specific type by id.",
                "m365 spe container get --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
            ),
            (
                "Gets a container of a specific type by display name.",
                "m365 spe container get --name \"My Application Storage Container\" --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\"",
            ),
            (
                "Gets container by using its name and container type name.",
                "m365 spe container get --name \"Invoices\" --containerTypeName \"My container type name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container list",
        summary: "Lists containers of a specific Container Type",
        options: &[
            (
                "containerTypeId",
                "The Container Type Id of the container instance.",
            ),
            (
                "containerTypeName",
                "The Container Type name of the container instance.",
            ),
        ],
        notes: &[
            "This command only supports application permissions when using the `containerTypeId` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "List containers of a specific type by id.",
                "m365 spe container list --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\"",
            ),
            (
                "List containers of a specific type by name.",
                "m365 spe container list --containerTypeName \"trial container\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container permission add",
        summary: "Adds permission to SharePoint Embedded container for a specified user",
        options: &[
            ("containerId", "ID of a SharePoint Embedded container."),
            ("containerName", "Display name of the Container."),
            (
                "containerTypeId",
                "The ID of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "containerTypeName",
                "The name of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "roles",
                "Comma-separated list of roles. Possible values are `reader`, `writer`, `manager`, `owner`.",
            ),
            ("userId", "The ID of the user to assign the role to."),
            ("userName", "The UPN of user to assign the role to."),
        ],
        notes: &[
            "When using `--userId`, Microsoft Graph also requires `User.ReadBasic.All` for delegated authentication or `User.Read.All` for application authentication to resolve the user's principal name.",
            "When using `--containerTypeName`, resolving the container type requires delegated authentication, the `FileStorageContainerType.Manage.All` permission, and the SharePoint Administrator or Global Administrator role. For application authentication, use `--containerId`, or `--containerName` with `--containerTypeId`, instead.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Adds a reader role for the specified user to the container",
                "m365 spe container permission add --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --roles \"reader\" --userName \"jacob@fabrikam.com\"",
            ),
            (
                "Adds multiple roles for the specified user to the container",
                "m365 spe container permission add --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --roles \"reader,writer\" --userName \"jacob@fabrikam.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container permission list",
        summary: "Lists permissions of a SharePoint Embedded Container",
        options: &[
            (
                "containerId",
                "The ID of the SharePoint Embedded Container.",
            ),
            (
                "containerName",
                "Display name of the SharePoint Embedded Container.",
            ),
            (
                "containerTypeId",
                "The container type ID of the container instance. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Lists Container permissions by id.",
                "m365 spe container permission list --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
            ),
            (
                "Lists Container permissions by display name.",
                "m365 spe container permission list --containerName \"My Application Storage Container\" --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\"",
            ),
            (
                "Lists Container permissions by display name and container type name.",
                "m365 spe container permission list --containerName \"My Application Storage Container\"  --containerTypeName \"My container type name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container permission remove",
        summary: "Removes permissions from a SharePoint Embedded Container",
        options: &[
            ("id", "ID of the permission to remove."),
            ("containerId", "ID of a SharePoint Embedded container."),
            ("containerName", "Display name of the Container."),
            (
                "containerTypeId",
                "The ID of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "containerTypeName",
                "The name of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "`FileStorageContainerType.Manage.All` is required only when using `containerTypeName`.",
            "This command only supports application permissions when not using the `containerTypeName` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Removes the specified permission from a container",
                "m365 spe container permission remove --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"  --id \"cmVhZGVyX2k6MCMuZnxtZW1iZXJzaGlwfHJvcnlicjExMUBvdXRsb29rLmNvbQ\"",
            ),
            (
                "Removes the specified permission from a container and doesn't prompt for confirmation",
                "m365 spe container permission remove --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"  --id \"cmVhZGVyX2k6MCMuZnxtZW1iZXJzaGlwfHJvcnlicjExMUBvdXRsb29rLmNvbQ\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container permission set",
        summary: "Updates SharePoint Embedded Container permission",
        options: &[
            ("id", "ID of the permission which will be modified."),
            ("containerId", "ID of a SharePoint Embedded container."),
            ("containerName", "Display name of the Container."),
            (
                "containerTypeId",
                "The ID of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "containerTypeName",
                "The name of the container type. Specify either `containerTypeId` or `containerTypeName` when using `containerName` but not both.",
            ),
            (
                "roles",
                "Comma separated list of permissions. Possible values are `reader`, `writer`, `manager`, `owner`.",
            ),
        ],
        notes: &[
            "`FileStorageContainerType.Manage.All` is required only when using `containerTypeName`.",
            "This command only supports application permissions when not using the `containerTypeName` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Updates role for the specified container",
                "m365 spe container permission set --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --id \"cmVhZGVyX2k6MCMuZnxtZW1iZXJzaGlwfHJvcnlicjExMUBvdXRsb29rLmNvbQ\" --roles \"reader\"",
            ),
            (
                "Updates multiple roles for the specified container",
                "m365 spe container permission set --containerId \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --id \"cmVhZGVyX2k6MCMuZnxtZW1iZXJzaGlwfHJvcnlicjExMUBvdXRsb29rLmNvbQ\" --roles \"reader,writer\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container recyclebinitem list",
        summary: "Lists deleted containers of a specific container type",
        options: &[
            (
                "containerTypeId",
                "The container type ID of the container instance.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance.",
            ),
        ],
        notes: &[
            "This command only supports application permissions when using the `containerTypeId` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "List deleted containers of a specific container type specified by id.",
                "m365 spe container recyclebinitem list --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\"",
            ),
            (
                "List deleted containers of a specific container type specified by name.",
                "m365 spe container recyclebinitem list --containerTypeName \"My container type name\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container recyclebinitem remove",
        summary: "Permanently removes a container from the recycle bin",
        options: &[
            ("id", "The container id."),
            ("name", "The display name of the Container."),
            (
                "containerTypeId",
                "The container type ID of the container instance. Use either `containerTypeId` or `containerTypeName` but not both.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance. Use either `containerTypeId` or `containerTypeName` but not both.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command only supports application permissions when using the `containerTypeId` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Remove a deleted container by ID.",
                "m365 spe container recyclebinitem remove --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
            ),
            (
                "Remove a deleted container by using its name and container type ID.",
                "m365 spe container recyclebinitem remove --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\" --name \"Invoices\"",
            ),
            (
                "Remove a deleted container by using its name and container type name.",
                "m365 spe container recyclebinitem remove --containerTypeName \"My container type name\" --name \"Invoices\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container recyclebinitem restore",
        summary: "Restores a deleted container",
        options: &[
            ("id", "The container id."),
            ("name", "The display name of the Container."),
            (
                "containerTypeId",
                "The container type ID of the container instance. Use either `containerTypeId` or `containerTypeName` but not both.",
            ),
            (
                "containerTypeName",
                "The container type name of the container instance. Use either `containerTypeId` or `containerTypeName` but not both.",
            ),
        ],
        notes: &[
            "This command only supports application permissions when using the `containerTypeId` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Restore a deleted container by ID.",
                "m365 spe container recyclebinitem restore --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
            ),
            (
                "Restore a deleted container by using its name and container type ID.",
                "m365 spe container recyclebinitem restore --containerTypeId \"91710488-5756-407f-9046-fbe5f0b4de73\" --name \"Invoices\"",
            ),
            (
                "Restore a deleted container by using its name and container type name.",
                "m365 spe container recyclebinitem restore --containerTypeName \"My container type name\" --name \"Invoices\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container remove",
        summary: "Removes a container",
        options: &[
            ("id", "The ID of the container instance."),
            ("name", "The name of the container instance."),
            (
                "containerTypeId",
                "The ID of the container type. Specify either `containerTypeId` or `containerTypeName` when using `name` but not both.",
            ),
            (
                "containerTypeName",
                "The name of the container type. Specify either `containerTypeId` or `containerTypeName` when using `name` but not both.",
            ),
            (
                "permanent",
                "Permanently delete the container instance instead of moving it to the recycle bin.",
            ),
            ("force", "Do not prompt for confirmation."),
        ],
        notes: &[
            "This command only supports application permissions when not using the `containerTypeName` option.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected, FileStorageContainerType.Manage.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Move a container specified by ID to the recycle bin.",
                "m365 spe container remove --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\"",
            ),
            (
                "Permanently remove a container by ID.",
                "m365 spe container remove --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --permanent",
            ),
            (
                "Remove a container by name specifying the container type id.",
                "m365 spe container remove --name \"My Application Storage Container\" --containerTypeId d248c449-da7e-4119-b5f6-7bb0865a42f9",
            ),
            (
                "Remove a container by name specifying the container type name.",
                "m365 spe container remove --name \"My Application Storage Container\" --containerTypeName \"My Application Container Type\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe container set",
        summary: "Updates a SharePoint Embedded container",
        options: &[
            ("id", "The Id of the container."),
            ("newName", "New display name for the container."),
            ("description", "Description of the container."),
            (
                "isOcrEnabled",
                "Indicates whether OCR is enabled for the container. Possible values: `true`, `false`.",
            ),
            (
                "isItemVersioningEnabled",
                "Indicates whether item versioning is enabled. Possible values: `true`, `false`.",
            ),
            (
                "itemMajorVersionLimit",
                "Maximum number of major versions to keep. Requires versioning to be enabled.",
            ),
        ],
        notes: &[
            "In addition to Graph permissions, the app/user must have container-type level permission for the respective container type. The caller must have write access to the container (for example, writer, manager, or owner role) to update metadata/settings.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
            (
                "Application",
                "Microsoft Graph",
                "FileStorageContainer.Selected",
            ),
        ],
        examples: &[
            (
                "Update the container display name by Id.",
                "m365 spe container set --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --newName \"Contoso Project A\"",
            ),
            (
                "Update description only.",
                "m365 spe container set --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --description \"Files for the Contoso Project A team\"",
            ),
            (
                "Disable OCR for a container.",
                "m365 spe container set --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --isOcrEnabled false",
            ),
            (
                "Enable versioning and set the major version limit.",
                "m365 spe container set --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --isItemVersioningEnabled true --itemMajorVersionLimit 100",
            ),
            (
                "Update name, description, and settings together.",
                "m365 spe container set --id \"b!ISJs1WRro0y0EWgkUYcktDa0mE8zSlFEqFzqRn70Zwp1CEtDEBZgQICPkRbil_5Z\" --newName \"Contoso Project A - Phase 2\" --description \"Phase 2 workspace\" --isOcrEnabled true --isItemVersioningEnabled true --itemMajorVersionLimit 50",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spe containertype add",
        summary: "Creates a new container type",
        options: &[
            ("name", "Container type name."),
            (
                "appId",
                "Application ID of the Microsoft Entra ID app that will be the owner of the container type. By default, the current app is used.",
            ),
            (
                "billingType",
                "The billing type. The possible values are: `standard`, `trial`, `directToCustomer`, `unknownFutureValue`. The default value is `standard`.",
            ),
            (
                "consumingTenantOverridables",
                "A comma-separated list of settings that can be overridden in the consuming tenant. The possible values are: `urlTemplate`, `isDiscoverabilityEnabled`, `isSearchEnabled`, `isItemVersioningEnabled`, `itemMajorVersionLimit`, `maxStoragePerContainerInBytes`.",
            ),
            (
                "isDiscoverabilityEnabled",
                "Indicates whether items from containers are surfaced in experiences such as My Activity or Microsoft 365. Valid values: `true`, `false`. The default value is `false`.",
            ),
            (
                "isItemVersioningEnabled",
                "Indicates whether item versioning is enabled for the container. Valid values: `true`, `false`. The default value is `true`.",
            ),
            (
                "itemMajorVersionLimit",
                "Maximum number of versions. `isItemVersioningEnabled` must be enabled. Default value is `500`.",
            ),
            (
                "isSearchEnabled",
                "Indicates whether search is enabled. Valid values: `true`, `false`. The default value is `true`.",
            ),
            (
                "isSharingRestricted",
                "Only the manager and owner can share files in the container if restricted sharing is enabled. Valid values: `true`, `false`. The default value is `false`.",
            ),
            (
                "maxStoragePerContainerInBytes",
                "Controls maximum storage in bytes.",
            ),
            (
                "sharingCapability",
                "Sharing capabilities permitted for containers. The possible values are: `disabled`, `externalUserSharingOnly`, `externalUserAndGuestSharing`, `existingExternalUserSharingOnly`.",
            ),
            ("urlTemplate", "Pattern used to redirect files."),
        ],
        notes: &[
            "The command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "The tenant must own the application that is assigned as the owner of the container type.",
            "You can only create one Trial container type per tenant, a single application registration can only contain one container type and a tenant can contain a maximum of five containers types in total.",
            "The registration of a container type in a newly created tenant can fail if the tenant isn't yet fully ready. You might need to wait at least an hour before you can register a container type in a new tenant.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "FileStorageContainerType.Manage.All",
        )],
        examples: &[
            (
                "Adds a new trial container type.",
                "m365 spe containertype add --name 'trial container' --appId '1b3b8660-9a44-4a7c-9c02-657f3ff5d5ac' --billingType trial",
            ),
            (
                "Adds a new container type using a standard billing type for the current application.",
                "m365 spe containertype add --name 'standard container'",
            ),
            (
                "Adds a new container type with additional settings",
                "m365 spe containertype add --name 'Contoso container type' --appId 5181cccf-7a36-44c1-a00d-62581ca4d14a --billingType directToCustomer --consumingTenantOverridables \"isDiscoverabilityEnabled, isSearchEnabled\" --isDiscoverabilityEnabled true --isItemVersioningEnabled true --isSearchEnabled false --itemMajorVersionLimit 25 --isSharingRestricted true --maxStoragePerContainerInBytes 102030 --sharingCapability externalUserSharingOnly --urlTemplate \"https://app.contoso.com/redirect?item={item-id}\"",
            ),
        ],
        links: &[(
            "SharePoint Embedded",
            "https://learn.microsoft.com/en-us/sharepoint/dev/embedded/concepts/app-concepts/containertypes#sharepoint-embedded-trial-container-types",
        )],
    },
    Help {
        name: "spe containertype get",
        summary: "Gets a specific container type",
        options: &[
            (
                "id",
                "The ID of the container type. Specify either id or name but not both.",
            ),
            (
                "name",
                "The container type name. Specify either id or name but not both.",
            ),
        ],
        notes: &[
            "The command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "FileStorageContainerType.Manage.All",
        )],
        examples: &[
            (
                "Gets container type by ID",
                "m365 spe containertype get --id 5de5c251-61c2-40f5-8947-5a719c1c0b71",
            ),
            (
                "Gets container type by name",
                "m365 spe containertype get --name 'Container Type 1'",
            ),
        ],
        links: &[(
            "SharePoint Embedded",
            "https://learn.microsoft.com/en-us/sharepoint/dev/embedded/concepts/app-concepts/containertypes#sharepoint-embedded-trial-container-types",
        )],
    },
    Help {
        name: "spe containertype list",
        summary: "Lists all container types",
        options: &[],
        notes: &[
            "The command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
            "This command does not support application permissions.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "FileStorageContainerType.Manage.All",
        )],
        examples: &[(
            "Retrieves a list of Container Types created for a SharePoint Embedded Application from the tenant.",
            "m365 spe containertype list",
        )],
        links: &[(
            "SharePoint Embedded",
            "https://learn.microsoft.com/sharepoint/dev/embedded/concepts/app-concepts/containertypes#sharepoint-embedded-trial-container-types",
        )],
    },
    Help {
        name: "spe containertype remove",
        summary: "Removes a specific container type",
        options: &[
            ("id", "The ID of the container type."),
            ("name", "The container type name."),
            (
                "force",
                "Force removal of the container type without confirmation.",
            ),
        ],
        notes: &[
            "This command relies on a different API endpoint due to current API limitations. As a result, it requires different permissions than other `spe containertype` commands. We plan to align the permissions in the future once these limitations are resolved.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "Container.Selected"),
            ("Application", "SharePoint", "Container.Selected"),
        ],
        examples: &[
            (
                "Removes a container type by ID.",
                "m365 spe containertype remove --id 4ec4aefd-4fa3-0e4a-20c3-6e68389e7138",
            ),
            (
                "Removes a container type by name.",
                "m365 spe containertype remove --name 'My container type'",
            ),
        ],
        links: &[],
    },
];

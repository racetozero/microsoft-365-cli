//! Help for the `m365 purview ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "purview auditlog list",
        summary: "Lists audit logs within your tenant",
        options: &[
            (
                "contentType",
                "Content type of logs to retrieve. Possible values: `AzureActiveDirectory`, `Exchange`, `SharePoint`, `General`, `DLP`.",
            ),
            (
                "startTime",
                "Time indicating the inclusive start of a time range of content to return. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z). Start time cannot be more than 7 days in the past. Default value is 24h ago.",
            ),
            (
                "endTime",
                "Time indicating the exclusive end of a time range of content to return. This should be defined as a valid ISO 8601 string (2021-12-16T18:28:48.6964197Z). Default value is now.",
            ),
        ],
        notes: &[
            "Before you can access audit log data, you must enable unified audit logging for your Microsoft 365 tenant. For instructions, check out the page Turn auditing on or off.",
            "When running this command for the first time for a certain content type, a subscription for this content type is created. It can take up to 12 hours for the first content blobs to become available for that subscription.",
            "Retrieving audit logs is an intensive process, especially for large or active tenants. In this case it may take some time to retrieve all audit logs. It can be useful to add the `--verbose` option to your command to view the command progress.",
            "When you specify a value for `contentType`, consider the following:",
            "- `AzureActiveDirectory` includes all Microsoft Entra ID audit logs.",
            "- `Exchange` includes all Exchange audit logs.",
            "- `SharePoint` includes all SharePoint audit logs.",
            "- `General` includes all other workloads not included in the previous content types.",
            "- `DLP` DLP events only for all workloads.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all SharePoint audit logs for the past 24 hours",
                "m365 purview auditlog list --contentType SharePoint",
            ),
            (
                "List all SharePoint audit logs within a specific time frame",
                "m365 purview auditlog list --contentType SharePoint --startTime \"2023-01-01T00:00:00Z\" --endTime \"2023-01-03T09:00:00Z\"",
            ),
            (
                "Get all SharePoint audit logs within a specific time frame and save the logs to a file",
                "m365 purview auditlog list --contentType SharePoint --startTime \"2023-01-01T00:00:00Z\" --endTime \"2023-01-01T12:00:00Z\" > auditLogs.json",
            ),
        ],
        links: &[
            (
                "Explanation of all available audit log activities/operations",
                "https://learn.microsoft.com/purview/audit-log-activities",
            ),
            (
                "Explanation of all available audit log properties",
                "https://learn.microsoft.com/purview/audit-log-detailed-properties",
            ),
        ],
    },
    Help {
        name: "purview retentionevent add",
        summary: "Creates a retention event",
        options: &[
            ("displayName", "The display name of the event."),
            (
                "eventTypeName",
                "Name of the event type associated with the event.",
            ),
            (
                "eventTypeId",
                "Id of the event type associated with the event.",
            ),
            ("description", "A description for the event."),
            (
                "triggerDateTime",
                "Optional time when the event should be triggered.",
            ),
            (
                "assetIds",
                "The Asset IDs for items in SharePoint and OneDrive that are related to this event. Only items that have labels associated with the event type you chose will be retained.",
            ),
            (
                "keywords",
                "The keywords for items in Exchange that are related to this event. Only items that have labels associated with the event type you chose will be retained.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Create a retention event to start retention at the end of 2022 for all employee documents that have been labeled and have the Asset ID EmployeeNr1234.",
            "m365 purview retentionevent add --displayName 'Employee information expiration' --description 'Employee documents expired due to offboarding' --eventTypeName 'CustomRetentionTime' --triggerDateTime '2022-12-31' --assetIds 'ComplianceAssetId:EmployeeNr1234'",
        )],
        links: &[(
            "Read more on event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentionevent get",
        summary: "Gets a retention event",
        options: &[("id", "The Id of the retention event.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "RecordsManagement.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.Read.All",
            ),
        ],
        examples: &[(
            "Get a retention event by id.",
            "m365 purview retentionevent get --id c37d695e-d581-4ae9-82a0-9364eba4291e",
        )],
        links: &[(
            "Read more on event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentionevent list",
        summary: "Gets a list of retention events",
        options: &[],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "RecordsManagement.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.Read.All",
            ),
        ],
        examples: &[(
            "Get a list of retention events.",
            "m365 purview retentionevent list",
        )],
        links: &[],
    },
    Help {
        name: "purview retentionevent remove",
        summary: "Deletes a retention event",
        options: &[
            ("id", "The Id of the retention event."),
            (
                "force",
                "Don't prompt for confirmation to remove the retention event.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Delete a retention event by id",
            "m365 purview retentionevent remove --id c37d695e-d581-4ae9-82a0-9364eba4291e",
        )],
        links: &[(
            "Read more on event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentioneventtype add",
        summary: "Creates a retention event type",
        options: &[
            ("displayName", "The display name of the event type."),
            ("description", "A description for the event type."),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Create a retention event type Contract Expiry.",
            "m365 purview retentioneventtype add --displayName 'Contract Expiry' --description 'A retention event type to start a retention period based on the date that a contract expired.'",
        )],
        links: &[(
            "Read more about event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentioneventtype get",
        summary: "Gets a retention event type",
        options: &[("id", "The Id of the retention event type.")],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "RecordsManagement.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.Read.All",
            ),
        ],
        examples: &[(
            "Get a retention event type by id.",
            "m365 purview retentioneventtype get --id c37d695e-d581-4ae9-82a0-9364eba4291e",
        )],
        links: &[(
            "Read more on event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentioneventtype list",
        summary: "Gets a list of retention event types",
        options: &[],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "RecordsManagement.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.Read.All",
            ),
        ],
        examples: &[(
            "Get a list of retention event types.",
            "m365 purview retentioneventtype list",
        )],
        links: &[],
    },
    Help {
        name: "purview retentioneventtype remove",
        summary: "Deletes a retention event type",
        options: &[
            ("id", "The Id of the retention event type."),
            (
                "force",
                "Don't prompt for confirmation to remove the retention event type.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Delete a retention event type by id.",
            "m365 purview retentioneventtype remove --id c37d695e-d581-4ae9-82a0-9364eba4291e",
        )],
        links: &[(
            "Read more about event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentioneventtype set",
        summary: "Updates a retention event type",
        options: &[
            ("id", "The id of the retention event type."),
            ("description", "The new description of the event type."),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "RecordsManagement.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Update the description of a retention event type.",
            "m365 purview retentioneventtype set --id c37d695e-d581-4ae9-82a0-9364eba4291e --description 'some extra information'",
        )],
        links: &[(
            "Read more about event-based retention here",
            "https://learn.microsoft.com/microsoft-365/compliance/event-driven-retention?view=o365-worldwide",
        )],
    },
    Help {
        name: "purview retentionlabel add",
        summary: "Creates a retention label",
        options: &[
            ("displayName", "The display name of the label."),
            (
                "behaviorDuringRetentionPeriod",
                "Specifies how the behavior of a document with this label should be during the retention period. Allowed values: `doNotRetain`, `retain`, `retainAsRecord`, `retainAsRegulatoryRecord`.",
            ),
            (
                "actionAfterRetentionPeriod",
                "Specifies the action to take on a document with this label applied after the retention period. Allowed values: `none`, `delete`, `startDispositionReview`.",
            ),
            (
                "retentionDuration",
                "The number of days to retain the content.",
            ),
            (
                "retentionTrigger",
                "Specifies whether the retention duration is calculated from the content creation date, labeled date, or last modification date. Allowed values: `dateLabeled`, `dateCreated`, `dateModified`, `dateOfEvent`. Defaults to `dateLabeled`.",
            ),
            (
                "defaultRecordBehavior",
                "Specifies the locked or unlocked state of a record label when it is created. Allowed values: `startLocked`, `startUnlocked`, Defaults to `startLocked`.",
            ),
            ("descriptionForUsers", "The label information for the user."),
            (
                "descriptionForAdmins",
                "The label information for the admin.",
            ),
            (
                "labelToBeApplied",
                "Specifies the replacement label to be applied automatically after the retention period of the current label ends.",
            ),
            (
                "eventTypeId",
                "The Id of the event type that is used to in case of an event-based label. Specify when using retentionTrigger with the value `dateOfEvent`. Specify either `eventTypeId` or `eventTypeName`, but not both.",
            ),
            (
                "eventTypeName",
                "The display name of the event type that is used to in case of an event-based label. Specify when using retentionTrigger with the value `dateOfEvent`. Specify either `eventTypeId` or `eventTypeName`, but not both.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "RecordsManagement.ReadWrite.All",
        )],
        examples: &[
            (
                "Create a retention label that retains documents and deletes them after one year.",
                "m365 purview retentionlabel add --displayName 'some label' --behaviorDuringRetentionPeriod retain --actionAfterRetentionPeriod delete --retentionDuration 365",
            ),
            (
                "Create a retention label that retains documents as records and does not take any action one year after the last modification date.",
                "m365 purview retentionlabel add --displayName 'some label' --behaviorDuringRetentionPeriod retainAsRecord --actionAfterRetentionPeriod none --retentionDuration 365 --retentionTrigger dateModified",
            ),
            (
                "Create an event-based retention label that retains documents as records and deletes them one year after a Contract Expiry event date.",
                "m365 purview retentionlabel add --displayName 'some label' --behaviorDuringRetentionPeriod retain --actionAfterRetentionPeriod delete --retentionDuration 365 --retentionTrigger dateOfEvent --eventTypeName \"Contract Expiry\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/security-retentionlabel-post?view=graph-rest-beta&tabs=http",
            "https://learn.microsoft.com/graph/api/security-retentionlabel-post?view=graph-rest-beta&tabs=http",
        )],
    },
    Help {
        name: "purview retentionlabel get",
        summary: "Gets a retention label",
        options: &[("id", "The Id of the retention label.")],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "RecordsManagement.Read.All")],
        examples: &[(
            "Get a retention label.",
            "m365 purview retentionlabel get --id c37d695e-d581-4ae9-82a0-9364eba4291e",
        )],
        links: &[],
    },
    Help {
        name: "purview retentionlabel list",
        summary: "Gets a list of retention labels",
        options: &[],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[("Delegated", "Microsoft Graph", "RecordsManagement.Read.All")],
        examples: &[(
            "Get a list of retention labels.",
            "m365 purview retentionlabel list",
        )],
        links: &[],
    },
    Help {
        name: "purview retentionlabel remove",
        summary: "Deletes a retention label",
        options: &[
            ("id", "The Id of the retention label."),
            ("force", "Don't prompt for confirming to remove the label."),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "RecordsManagement.ReadWrite.All",
        )],
        examples: &[(
            "Delete a retention label.",
            "m365 purview retentionlabel remove --id 'e554d69c-0992-4f9b-8a66-fca3c4d9c531'",
        )],
        links: &[],
    },
    Help {
        name: "purview retentionlabel set",
        summary: "Updates a retention label",
        options: &[
            ("id", "The Id of the retention label."),
            (
                "behaviorDuringRetentionPeriod",
                "Specifies how the behavior of a document with this label should be during the retention period. Allowed values: `doNotRetain`, `retain`, `retainAsRecord`, `retainAsRegulatoryRecord`.",
            ),
            (
                "actionAfterRetentionPeriod",
                "Specifies the action to take on a document with this label applied after the retention period. Allowed values: `none`, `delete`, `startDispositionReview`.",
            ),
            (
                "retentionDuration",
                "The number of days to retain the content.",
            ),
            (
                "retentionTrigger",
                "Specifies whether the retention duration is calculated from the content creation date, labeled date, or last modification date. Allowed values: `dateLabeled`, `dateCreated`, `dateModified`, `dateOfEvent`.",
            ),
            (
                "defaultRecordBehavior",
                "Specifies the locked or unlocked state of a record label when it is created. Allowed values: `startLocked`, `startUnlocked`.",
            ),
            ("descriptionForUsers", "The label information for the user."),
            (
                "descriptionForAdmins",
                "The label information for the admin.",
            ),
            (
                "labelToBeApplied",
                "Specifies the replacement label to be applied automatically after the retention period of the current label ends.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "RecordsManagement.ReadWrite.All",
        )],
        examples: &[
            (
                "Update a retention label so that it retains documents as records and deletes them after one year.",
                "m365 purview retentionlabel set --id c37d695e-d581-4ae9-82a0-9364eba4291e --behaviorDuringRetentionPeriod retainAsRecord --actionAfterRetentionPeriod delete --retentionDuration 365",
            ),
            (
                "Update a retention label so that it retains documents as regulatory records and starts a disposition review one year after the last modification date.",
                "m365 purview retentionlabel set --id c37d695e-d581-4ae9-82a0-9364eba4291e --behaviorDuringRetentionPeriod retainAsRegulatoryRecord --actionAfterRetentionPeriod startDispositionReview --retentionDuration 365 --retentionTrigger dateModified",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview sensitivitylabel get",
        summary: "Retrieves the specified sensitivity label",
        options: &[
            ("id", "The Id of the sensitivity label."),
            (
                "userId",
                "User's Microsoft Entra ID. Optionally specify this if you want to get a list of sensitivity labels that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Optionally specify this if you want to get a list of sensitivity labels that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
            "When operating in app-only mode, you have the option to use either the `userName` or `userId` parameters to retrieve the sensitivity policy settings for a specific user. Without specifying either of these parameters, the command will retrieve the sensitivity policy settings for the currently authenticated user when operating in delegated mode.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a sensitivity label.",
                "m365 purview sensitivitylabel get --id 6f4fb2db-ecf4-4279-94ba-23d059bf157e",
            ),
            (
                "Get a sensitivity label that a specific user has access to by its Id.",
                "m365 purview sensitivitylabel get --id 6f4fb2db-ecf4-4279-94ba-23d059bf157e --userId 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
            (
                "Get a sensitivity label that a specific user has access to by its UPN.",
                "m365 purview sensitivitylabel get --id 6f4fb2db-ecf4-4279-94ba-23d059bf157e --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview sensitivitylabel list",
        summary: "Gets a list of sensitivity labels",
        options: &[
            (
                "userId",
                "User's Microsoft Entra ID. Optionally specify this if you want to get a list of sensitivity labels that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Optionally specify this if you want to get a list of sensitivity labels that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
            "When operating in app-only mode, you have the option to use either the `userName` or `userId` parameters to retrieve the sensitivity label for a specific user. Without specifying either of these parameters, the command will retrieve the sensitivity label for the currently authenticated user when operating in delegated mode.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a list of sensitivity labels.",
                "m365 purview sensitivitylabel list",
            ),
            (
                "Get a list of sensitivity labels that a specific user has access to by its Id.",
                "m365 purview sensitivitylabel list --userId 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
            (
                "Get a list of sensitivity labels that a specific user has access to by its UPN.",
                "m365 purview sensitivitylabel list --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview sensitivitylabel policysettings list",
        summary: "Gets a list of policy settings for a sensitivity label.",
        options: &[
            (
                "userId",
                "User's Microsoft Entra ID. Optionally specify this if you want to get a list of policy settings for a sensitivity label that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
            (
                "userName",
                "User's UPN (user principal name, e.g. johndoe@example.com). Optionally specify this if you want to get a list of policy settings for a sensitivity label that the user has access to. Specify either `userId` or `userName` but not both.",
            ),
        ],
        notes: &[
            "This command is based on a Microsoft Graph API that is currently in preview and is subject to change once the API reached general availability.",
            "When operating in app-only mode, you have the option to use either the `userName` or `userId` parameters to retrieve the sensitivity policy settings for a specific user. Without specifying either of these parameters, the command will retrieve the sensitivity policy settings for the currently authenticated user when operating in delegated mode.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read",
            ),
            (
                "Application",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a list of policy settings for a sensitivity label.",
                "m365 purview sensitivitylabel policysettings list",
            ),
            (
                "Get a list of policy settings for a sensitivity label that a specific user has access to by its Id.",
                "m365 purview sensitivitylabel policysettings list --userId 59f80e08-24b1-41f8-8586-16765fd830d3",
            ),
            (
                "Get a list of policy settings for a sensitivity label that a specific user has access to by its UPN.",
                "m365 purview sensitivitylabel policysettings list --userName john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview threatassessment add",
        summary: "Creates a threat assessment",
        options: &[
            (
                "type",
                "The type of threat assessment to retrieve. Supports `file` and `url`.",
            ),
            (
                "expectedAssessment",
                "The expected assessment from submitter. Possible values are: `block` and `unblock`.",
            ),
            (
                "category",
                "The threat category. Possible values are: `spam`, `phishing`, `malware`.",
            ),
            (
                "path",
                "Local path to the file to upload. Can only be used for threat assessment with type `file`.",
            ),
            (
                "url",
                "The URL string. Can only be used for threat assessment with type `url`.",
            ),
        ],
        notes: &["    This command currently only supports delegated permissions."],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "ThreatAssessment.ReadWrite.All",
        )],
        examples: &[
            (
                "Create a file threat assessment",
                "m365 purview threatassessment add --type file --expectedAssessment block --category malware --path 'C:\\Path\\To\\File.txt'",
            ),
            (
                "Create a url threat assessment",
                "m365 purview threatassessment add --type url --expectedAssessment block --category phishing --url 'http://contoso.com'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview threatassessment get",
        summary: "Gets a threat assessment",
        options: &[
            ("id", "The Id of the threat assessment."),
            ("withResults", "Include the threat assessment results."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ThreatAssessment.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ThreatAssessment.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a threat assessment.",
                "m365 purview threatassessment get --id c37d695e-d581-4ae9-82a0-9364eba4291e",
            ),
            (
                "Get a threat assessment including results.",
                "m365 purview threatassessment get --id c37d695e-d581-4ae9-82a0-9364eba4291e --withResults",
            ),
        ],
        links: &[],
    },
    Help {
        name: "purview threatassessment list",
        summary: "Gets a list of threat assessments",
        options: &[(
            "type",
            "The type of threat assessment to retrieve. Allowed values are `mail`, `file`, `emailFile` and `url`.",
        )],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ThreatAssessment.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ThreatAssessment.Read.All",
            ),
        ],
        examples: &[
            (
                "Get a list of threat assessments",
                "m365 purview threatassessment list",
            ),
            (
                "Get a list of threat assessments of type mail",
                "m365 purview threatassessment list --type mail",
            ),
        ],
        links: &[],
    },
];

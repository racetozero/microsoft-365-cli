//! Help for the `m365 tenant ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "tenant id get",
        summary: "Gets Microsoft 365 tenant ID for the specified domain",
        options: &[(
            "domainName",
            "The domain name for which to retrieve the Microsoft 365 tenant ID",
        )],
        notes: &[
            "If no domain name is specified, the command will return the tenant ID of the tenant to which you are currently logged in.",
            "The command doesn't require any permissions.",
            "The command doesn't require any permissions.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get Microsoft 365 tenant ID for the specified domain",
                "m365 tenant id get --domainName contoso.com",
            ),
            (
                "Get Microsoft 365 tenant ID of the the tenant to which you are currently logged in",
                "m365 tenant id get",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant info get",
        summary: "Gets information about any tenant",
        options: &[
            (
                "domainName",
                "The primary domain name of a tenant. Optionally specify either `domainName` or `tenantId` but not both. If none are specified, the tenantId of the currently signed-in user is used.",
            ),
            (
                "tenantId",
                "The unique tenant identifier of a tenant. Optionally specify either `domainName` or `tenantId` but not both. If none are specified, the tenantId of the currently signed-in user is used.",
            ),
        ],
        notes: &[
            "If no domain name or tenantId is specified, the command will return the tenant information of the currently signed in user.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "CrossTenantInformation.ReadBasic.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "CrossTenantInformation.ReadBasic.All",
            ),
        ],
        examples: &[
            (
                "Get tenant information for the currently signed in user.",
                "m365 tenant info get",
            ),
            (
                "Get tenant information for the Contoso tenant.",
                "m365 tenant info get --domainName contoso.com",
            ),
            (
                "Get tenant information by id.",
                "m365 tenant info get --tenantId e65b162c-6f87-4eb1-a24e-1b37d3504663",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant people profilecardproperty add",
        summary: "Adds an additional attribute to the profile card properties",
        options: &[
            (
                "name",
                "The name of the property to add. Allowed values: `UserPrincipalName`, `Fax`, `StreetAddress`, `PostalCode`, `StateOrProvince`, `Alias`, `customAttribute1`, `customAttribute2`, `customAttribute3`, `customAttribute4`, `customAttribute5`, `customAttribute6`, `customAttribute7`, `customAttribute8`, `customAttribute9`, `customAttribute10`, `customAttribute11`, `customAttribute12`, `customAttribute13`, `customAttribute14`, `customAttribute15`.",
            ),
            (
                "displayName",
                "The display name of a property, only use this together with a custom extension attribute.",
            ),
        ],
        notes: &[
            "To use this command you must be either Tenant Administrator or Global Administrator.",
            "You can specify localized values for the `displayName` as well. These can be entered by suffixing the displayName option with a language code: `--displayName-nl-NL \"Kostencentrum\"`, `--displayName-de \"Kostenstelle\"`.",
            "When adding an attribute to a profile card, it takes up to 24 hours for the addition to be displayed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Add the UPN as a profile property to the profile cards properties",
                "m365 tenant people profilecardproperty add --name UserPrincipalName",
            ),
            (
                "Add a custom extension attribute Cost Center as a profile property to the profile cards properties",
                "m365 tenant people profilecardproperty add --name customAttribute1 --displayName 'Cost Center'",
            ),
            (
                "Add a custom extension attribute Cost Center as a profile property to the profile cards properties with a Dutch localization",
                "m365 tenant people profilecardproperty add --name customAttribute1 --displayName 'Cost Center' --displayName-nl-NL 'Kostencentrum'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant people profilecardproperty get",
        summary: "Retrieves information about a specific profile card property",
        options: &[(
            "name",
            "The name of the property to retrieve. Allowed values: `UserPrincipalName`, `Fax`, `StreetAddress`, `PostalCode`, `StateOrProvince`, `Alias`, `customAttribute1`, `customAttribute2`, `customAttribute3`, `customAttribute4`, `customAttribute5`, `customAttribute6`, `customAttribute7`, `customAttribute8`, `customAttribute9`, `customAttribute10`, `customAttribute11`, `customAttribute12`, `customAttribute13`, `customAttribute14`, `customAttribute15`.",
        )],
        notes: &[
            "To use this command you must be either Tenant Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "PeopleSettings.Read.All"),
            ("Application", "Microsoft Graph", "PeopleSettings.Read.All"),
        ],
        examples: &[(
            "Retrieve information about a specific profile card property",
            "m365 tenant people profilecardproperty get --name customAttribute1",
        )],
        links: &[],
    },
    Help {
        name: "tenant people profilecardproperty list",
        summary: "Lists all profile card properties",
        options: &[],
        notes: &[
            "To use this command you must be either Tenant Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "PeopleSettings.Read.All"),
            ("Application", "Microsoft Graph", "PeopleSettings.Read.All"),
        ],
        examples: &[(
            "Lists all profile card properties",
            "m365 tenant people profilecardproperty list",
        )],
        links: &[],
    },
    Help {
        name: "tenant people profilecardproperty remove",
        summary: "Removes an additional attribute from the profile card properties",
        options: &[
            (
                "name",
                "The name of the property to remove. Allowed values: `UserPrincipalName`, `Fax`, `StreetAddress`, `PostalCode`, `StateOrProvince`, `Alias`, `customAttribute1`, `customAttribute2`, `customAttribute3`, `customAttribute4`, `customAttribute5`, `customAttribute6`, `customAttribute7`, `customAttribute8`, `customAttribute9`, `customAttribute10`, `customAttribute11`, `customAttribute12`, `customAttribute13`, `customAttribute14`, `customAttribute15`.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either Tenant Administrator or Global Administrator.",
            "When removing an attribute from a profile card, it takes up to 24 hours for the change to be displayed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Removes the UPN as a profile property from the profile cards",
                "m365 tenant people profilecardproperty remove --name UserPrincipalName",
            ),
            (
                "Removes the UPN as a profile property from the profile cards, without showing a confirmation prompt",
                "m365 tenant people profilecardproperty remove --name UserPrincipalName --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant people profilecardproperty set",
        summary: "Updates a custom attribute to the profile card property",
        options: &[
            (
                "name",
                "The name of the property to update. Allowed values: `customAttribute1`, `customAttribute2`, `customAttribute3`, `customAttribute4`, `customAttribute5`, `customAttribute6`, `customAttribute7`, `customAttribute8`, `customAttribute9`, `customAttribute10`, `customAttribute11`, `customAttribute12`, `customAttribute13`, `customAttribute14`, `customAttribute15`.",
            ),
            ("displayName", "The display name of the property."),
        ],
        notes: &[
            "To use this command you must be either Tenant Administrator or Global Administrator.",
            "You can specify localized values for the `displayName` as well. These can be entered by suffixing the displayName option with a language code: `--displayName-nl-NL \"Kostencentrum\"`, `--displayName-de \"Kostenstelle\"`.",
            "When updating an attribute to a profile card, it takes up to 24 hours for the addition to be displayed.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Updates a custom extension attribute to Cost Center",
                "m365 tenant people profilecardproperty set --name customAttribute1 --displayName \"Cost Center\"",
            ),
            (
                "Updates a custom extension attribute with translations",
                "m365 tenant people profilecardproperty set --name customAttribute1 --displayName \"Cost Center\" --displayName-nl-NL \"Kostencentrum\" --displayName-de \"Kostenstelle\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant people pronouns get",
        summary: "Retrieves information about pronouns settings for an organization",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "PeopleSettings.Read.All"),
            ("Application", "Microsoft Graph", "PeopleSettings.Read.All"),
        ],
        examples: &[(
            "Retrieve information about pronouns settings",
            "m365 tenant people pronouns get",
        )],
        links: &[],
    },
    Help {
        name: "tenant people pronouns set",
        summary: "Manages pronouns settings for an organization",
        options: &[(
            "enabled",
            "Specifies whether the pronouns are enabled in the organization or not.",
        )],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "PeopleSettings.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Enable pronouns in the organization",
                "m365 tenant people pronouns set --enabled true",
            ),
            (
                "Disable pronouns in the organization",
                "m365 tenant people pronouns set --enabled false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant report activeusercounts",
        summary: "Gets the count of daily active users in the reporting period by product.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of daily active users in the reporting period by product for the last week",
                "m365 tenant report activeusercounts --period D7",
            ),
            (
                "Gets the count of daily active users in the reporting period by product for the last week and exports the report data in the specified path in text format",
                "m365 tenant report activeusercounts --period D7 --output text > \"activeusercounts.txt\"",
            ),
            (
                "Gets the count of daily active users in the reporting period by product for the last week and exports the report data in the specified path in json format",
                "m365 tenant report activeusercounts --period D7 --output json > \"activeusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant report activeuserdetail",
        summary: "Gets details about Microsoft 365 active users",
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
                "Gets details about Microsoft 365 active users for the last week",
                "m365 tenant report activeuserdetail --period D7",
            ),
            (
                "Gets details about Microsoft 365 active users for May 1, 2019",
                "m365 tenant report activeuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about Microsoft 365 active users for the last week and exports the report data in the specified path in text format",
                "m365 tenant report activeuserdetail --period D7 --output text > \"activeuserdetail.txt\"",
            ),
            (
                "Gets details about Microsoft 365 active users for the last week and exports the report data in the specified path in json format",
                "m365 tenant report activeuserdetail --period D7 --output json > \"activeuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant report office365activationcounts",
        summary: "Gets the count of Microsoft 365 activations on desktops and devices",
        options: &[],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[(
            "Get the count of Microsoft 365 activations on desktops and devices",
            "m365 tenant report office365activationcounts",
        )],
        links: &[],
    },
    Help {
        name: "tenant report office365activationsusercounts",
        summary: "Gets the count of users that are enabled and those that have activated the Office subscription on desktop or devices or shared computers",
        options: &[],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[(
            "Get the count of users that are enabled and those that have activated the Office subscription on desktop or devices or shared computers",
            "m365 tenant report office365activationsusercounts",
        )],
        links: &[],
    },
    Help {
        name: "tenant report office365activationsuserdetail",
        summary: "Gets details about users who have activated Microsoft 365",
        options: &[],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[(
            "Get details about users who have activated Microsoft 365",
            "m365 tenant report office365activationsuserdetail",
        )],
        links: &[],
    },
    Help {
        name: "tenant report servicesusercounts",
        summary: "Gets the count of users by activity type and service.",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the count of users by activity type and service for the last week",
                "m365 tenant report servicesusercounts --period D7",
            ),
            (
                "Gets the count of users by activity type and service for the last week and exports the report data in the specified path in text format",
                "m365 tenant report servicesusercounts --period D7 --output text > \"servicesusercounts.txt\"",
            ),
            (
                "Gets the count of users by activity type and service for the last week and exports the report data in the specified path in json format",
                "m365 tenant report servicesusercounts --period D7 --output json > \"servicesusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant report settings get",
        summary: "Gets the tenant-level settings for Microsoft 365 reports",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ReportSettings.Read.All"),
            ("Application", "Microsoft Graph", "ReportSettings.Read.All"),
        ],
        examples: &[(
            "Get the tenant-level settings for Microsoft 365 reports.",
            "m365 tenant report settings get",
        )],
        links: &[],
    },
    Help {
        name: "tenant report settings set",
        summary: "Updates tenant-level settings for Microsoft 365 reports",
        options: &[(
            "displayConcealedNames",
            "Determines whether all reports conceal user information such as usernames, groups, and sites. If false, all reports show identifiable information.",
        )],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "ReportSettings.ReadWrite.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "ReportSettings.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Configure the tenant to display concealed user information in Microsoft 365 reports",
            "m365 tenant report settings set --displayConcealedNames true",
        )],
        links: &[],
    },
    Help {
        name: "tenant security alerts list",
        summary: "Gets the security alerts for a tenant",
        options: &[(
            "vendor",
            "The vendor to return alerts for. Allowed values: `Azure Advanced Threat Protection`, `Azure Security Center`, `Microsoft Cloud App Security`, `Azure Active Directory Identity Protection`, `Azure Sentinel`, `Microsoft Defender ATP`. If omitted, all alerts are returned.",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "SecurityEvents.Read.All"),
            ("Application", "Microsoft Graph", "SecurityEvents.Read.All"),
        ],
        examples: &[
            (
                "Get all security alerts for a tenant",
                "m365 tenant security alerts list",
            ),
            (
                "Get security alerts for a vendor with name Azure Sentinel",
                "m365 tenant security alerts list --vendor \"Azure Sentinel\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant serviceannouncement health get",
        summary: "Gets the health report of a specified service for a tenant",
        options: &[
            (
                "serviceName",
                "The service name to retrieve the health report for.",
            ),
            (
                "issues",
                "Return the collection of issues that happened on the service, with detailed information for each issue. Is only returned in JSON output mode.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceHealth.Read.All"),
            ("Application", "Microsoft Graph", "ServiceHealth.Read.All"),
        ],
        examples: &[
            (
                "Get the health report for the service Exchange Online",
                "m365 tenant serviceannouncement health get --serviceName \"Exchange Online\"",
            ),
            (
                "Get the health report for the service Exchange Online including the issues of the service",
                "m365 tenant serviceannouncement health get --serviceName \"Exchange Online\" --issues",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant serviceannouncement health list",
        summary: "Gets the health report of all subscribed services for a tenant",
        options: &[(
            "issues",
            "Return the collection of issues that happened on the service, with detailed information for each issue. Is only returned in JSON output mode.",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceHealth.Read.All"),
            ("Application", "Microsoft Graph", "ServiceHealth.Read.All"),
        ],
        examples: &[
            (
                "Get the health report of all subscribed services for a tenant",
                "m365 tenant serviceannouncement health list",
            ),
            (
                "Get the health report of all subscribed services for a tenant including the issues that happend on each service",
                "m365 tenant serviceannouncement health list --issues",
            ),
        ],
        links: &[],
    },
    Help {
        name: "tenant serviceannouncement healthissue get",
        summary: "Gets a specified service health issue for tenant.",
        options: &[("id", "The issue id to get details for")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceHealth.Read.All"),
            ("Application", "Microsoft Graph", "ServiceHealth.Read.All"),
        ],
        examples: &[(
            "Gets information about issue with ID MO226784",
            "m365 tenant serviceannouncement healthissue get --id MO226784",
        )],
        links: &[],
    },
    Help {
        name: "tenant serviceannouncement healthissue list",
        summary: "Gets all service health issues for the tenant.",
        options: &[(
            "service",
            "Retrieve service health issues for the particular service. If not provided, retrieves health issues for all services",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceHealth.Read.All"),
            ("Application", "Microsoft Graph", "ServiceHealth.Read.All"),
        ],
        examples: &[
            (
                "Get service health issues of all services in Microsoft 365",
                "m365 tenant serviceannouncement healthissue list",
            ),
            (
                "Get service health issues for Microsoft Forms",
                "m365 tenant serviceannouncement healthissue list --service \"Microsoft Forms\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/serviceannouncement-list-issues",
            "https://learn.microsoft.com/graph/api/serviceannouncement-list-issues",
        )],
    },
    Help {
        name: "tenant serviceannouncement message get",
        summary: "Retrieves a specified service update message for the tenant",
        options: &[("id", "The ID of the service update message.")],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceMessage.Read.All"),
            ("Application", "Microsoft Graph", "ServiceMessage.Read.All"),
        ],
        examples: &[(
            "Get service update message with ID MC001337",
            "m365 tenant serviceannouncement message get --id MC001337",
        )],
        links: &[(
            "https://learn.microsoft.com/graph/api/serviceupdatemessage-get",
            "https://learn.microsoft.com/graph/api/serviceupdatemessage-get",
        )],
    },
    Help {
        name: "tenant serviceannouncement message list",
        summary: "Gets all service update messages for the tenant.",
        options: &[(
            "service",
            "Retrieve service update messages for the particular service. If not provided, retrieves messages for all services",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "ServiceMessage.Read.All"),
            ("Application", "Microsoft Graph", "ServiceMessage.Read.All"),
        ],
        examples: &[
            (
                "Get service update messages of all services in Microsoft 365",
                "m365 tenant serviceannouncement message list",
            ),
            (
                "Get service update messages for Microsoft Teams",
                "m365 tenant serviceannouncement message list --service \"Microsoft Teams\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/serviceannouncement-list-messages",
            "https://learn.microsoft.com/graph/api/serviceannouncement-list-messages",
        )],
    },
];

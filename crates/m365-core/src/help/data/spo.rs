//! Help for the `m365 spo ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "spo agent add",
        summary: "Adds a new SharePoint agent",
        options: &[
            ("webUrl", "URL of the site where the agent should be added."),
            ("name", "The name of the agent."),
            (
                "agentInstructions",
                "The definition of the agent's role, tone, and limitations.",
            ),
            ("welcomeMessage", "The welcome message."),
            (
                "sourceUrls",
                "Comma-separated list of URLs of the SharePoint sites, libraries, or files that are the sources of information.",
            ),
            (
                "description",
                "The brief description of the agent's objective.",
            ),
            ("icon", "The URL of the icon."),
            (
                "conversationStarters",
                "Starter prompts. Please provide each prompt separated by a comma.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Add a SharePoint agent with site and library sources.",
                "m365 spo agent add --webUrl https://contoso.sharepoint.com/sites/projectSite --name \"Project Assistant\" --agentInstructions \"You are a helpful assistant for project management tasks. Be professional and concise.\" --welcomeMessage \"Hello! I'm here to help you with project management tasks.\" --sourceUrls \"https://contoso.sharepoint.com/sites/projectSite,https://contoso.sharepoint.com/sites/projectSite/Shared Documents/Forms/AllItems.aspx\" --description \"A helpful agent for project management assistance\"",
            ),
            (
                "Add a SharePoint agent with icon and conversation starters.",
                "m365 spo agent add --webUrl https://contoso.sharepoint.com/sites/projectSite --name \"HR Assistant\" --agentInstructions \"You are an HR assistant. Help with HR policies and procedures.\" --welcomeMessage \"Welcome! I can help you with HR-related questions.\" --sourceUrls \"https://contoso.sharepoint.com/sites/hr\" --icon \"https://contoso.sharepoint.com/sites/projectSite/SiteAssets/hr-icon.png\" --conversationStarters \"What are the vacation policies?,How do I submit a timesheet?,Where can I find the employee handbook?\" --description \"A helpful agent for project management assistance\"",
            ),
            (
                "Add a basic SharePoint agent with minimal configuration.",
                "m365 spo agent add --webUrl https://contoso.sharepoint.com/sites/marketing --name \"Marketing Bot\" --agentInstructions \"Assist with marketing campaign information and resources.\" --welcomeMessage \"Hi! I can help you find marketing resources.\" --sourceUrls \"https://contoso.sharepoint.com/sites/marketing/Campaigns\" --description \"A helpful agent for project management assistance\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo app add",
        summary: "Adds an app to the specified SharePoint Online app catalog",
        options: &[
            (
                "filePath",
                "Absolute or relative path to the solution package file to add to the app catalog.",
            ),
            ("overwrite", "Set to overwrite the existing package file."),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
            (
                "appCatalogUrl",
                "The URL of the app catalog where the solution package will be added. It must be specified when the scope is `sitecollection`.",
            ),
        ],
        notes: &[
            "When specifying the path to the app package file you can use both relative and absolute paths. Note, that `~` in the path, will not be resolved and will most likely result in an error.",
            "When adding an app to the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to add the app to a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
            "If you try to upload a package that already exists in the app catalog without specifying the `--overwrite` option, the command will fail with an error stating that the specified package already exists.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Add the package to the tenant app catalog.",
                "m365 spo app add --filePath /Users/pnp/spfx/sharepoint/solution/spfx.sppkg",
            ),
            (
                "Overwrite the package in the tenant app catalog with the newer version.",
                "m365 spo app add --filePath sharepoint/solution/spfx.sppkg --overwrite",
            ),
            (
                "Add the package to the site collection app catalog of the specified site.",
                "m365 spo app add --filePath c:\\spfx.sppkg --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app deploy",
        summary: "Deploys the specified app in the specified app catalog",
        options: &[
            ("id", "ID of the app to deploy."),
            ("name", "Name of the app to deploy."),
            (
                "appCatalogUrl",
                "URL of the tenant or site collection app catalog. It must be specified when the scope is `sitecollection`.",
            ),
            (
                "skipFeatureDeployment",
                "If the app supports tenant-wide deployment, deploy it to the whole tenant.",
            ),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
        ],
        notes: &[
            "When deploying an app in the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to deploy the app in a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error. Before you can deploy an app, you have to add it to the app catalog first using the spo app add command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Deploy the specified app in the tenant app catalog. Try to resolve the URL of the tenant app catalog automatically.",
                "m365 spo app deploy --id 058140e3-0e37-44fc-a1d3-79c487d371a3",
            ),
            (
                "Deploy the specified app in the site collection app catalog of the specified site.",
                "m365 spo app deploy --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1",
            ),
            (
                "Deploy the app with the specified name in the tenant app catalog. Try to resolve the URL of the tenant app catalog automatically.",
                "m365 spo app deploy --name solution.sppkg",
            ),
            (
                "Deploy the specified app in the tenant app catalog.",
                "m365 spo app deploy --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --appCatalogUrl https://contoso.sharepoint.com/sites/apps",
            ),
            (
                "Deploy the specified app to the whole tenant at once. Features included in the solution will not be activated.",
                "m365 spo app deploy --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --skipFeatureDeployment",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app get",
        summary: "Gets information about the specific app from the specified app catalog",
        options: &[
            ("id", "ID of the app to retrieve information for."),
            ("name", "Name of the app to retrieve information for."),
            (
                "appCatalogUrl",
                "URL of the tenant or site collection app catalog. It must be specified when the scope is `sitecollection`.",
            ),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
        ],
        notes: &[
            "When getting information about an app from the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to get information about an app from a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return details about the app with the specified ID available in the tenant app catalog.",
                "m365 spo app get --id b2307a39-e878-458b-bc90-03bc578531d6",
            ),
            (
                "Return details about the app with the specified name available in the tenant app catalog. Will try to detect the app catalog URL.",
                "m365 spo app get --name solution.sppkg",
            ),
            (
                "Return details about the app with the specified name available in the tenant app catalog using the specified app catalog URL.",
                "m365 spo app get --name solution.sppkg --appCatalogUrl https://contoso.sharepoint.com/sites/apps",
            ),
            (
                "Return details about the app with the specified ID available in the site collection app catalog of the specified site.",
                "m365 spo app get --id b2307a39-e878-458b-bc90-03bc578531d6 --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app install",
        summary: "Installs an app from the specified app catalog in the site",
        options: &[
            ("id", "ID of the app to install."),
            ("siteUrl", "Absolute URL of the site to install the app in."),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
        ],
        notes: &[
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error. Before you can install app in a site, you have to add it to the tenant or site collection app catalog first using the spo app add command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Install the app with the specified ID in the specified site.",
                "m365 spo app install --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "Install the app with the specified ID in the specified site from site collection app catalog.",
                "m365 spo app install --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com --appCatalogScope sitecollection",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app instance list",
        summary: "Retrieves apps installed in a site",
        options: &[(
            "siteUrl",
            "URL of the site collection to retrieve the apps for.",
        )],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "Return a list of installed apps on the specified site.",
            "m365 spo app instance list --siteUrl https://contoso.sharepoint.com/sites/site1",
        )],
        links: &[],
    },
    Help {
        name: "spo app list",
        summary: "Lists apps from the specified app catalog",
        options: &[
            (
                "appCatalogScope",
                "Target app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
            (
                "appCatalogUrl",
                "URL of the tenant or site collection app catalog. It must be specified when the scope is `sitecollection`.",
            ),
        ],
        notes: &[
            "When listing information about apps available in the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to list information about apps in a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
            "When using the text output type (default), the command lists only the values of the `Title`, `ID`, `Deployed` and `AppCatalogVersion` properties of the app. When setting the output type to JSON, all available properties are included in the command output.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return the list of available apps from the tenant app catalog. Show the installed version in the site if applicable.",
                "m365 spo app list",
            ),
            (
                "Return the list of available apps from a site collection app catalog of the specified site.",
                "m365 spo app list --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app remove",
        summary: "Removes the specified app from the specified app catalog",
        options: &[
            (
                "id",
                "ID of the app to remove. Needs to be available in the app catalog.",
            ),
            (
                "appCatalogUrl",
                "URL of the tenant or site collection app catalog. It must be specified when the scope is `sitecollection`.",
            ),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
            ("force", "Don't prompt for confirming removing the app."),
        ],
        notes: &[
            "When removing an app from the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to remove the app from a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove the specified app from the tenant app catalog. Try to resolve the URL of the tenant app catalog automatically. Additionally, will prompt for confirmation before actually retracting the app.",
                "m365 spo app remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3",
            ),
            (
                "Remove the specified app from the tenant app catalog. Additionally, will prompt for confirmation before actually retracting the app.",
                "m365 spo app remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --appCatalogUrl https://contoso.sharepoint.com/sites/apps",
            ),
            (
                "Remove the specified app from the tenant app catalog. Don't prompt for confirmation.",
                "m365 spo app remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --appCatalogUrl https://contoso.sharepoint.com/sites/apps --force",
            ),
            (
                "Remove the specified app from a site collection app catalog of the specified site.",
                "m365 spo app remove --id d95f8c94-67a1-4615-9af8-361ad33be93c --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1/AppCatalog",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app retract",
        summary: "Retracts the specified app from the specified app catalog",
        options: &[
            (
                "id",
                "ID of the app to retract. Needs to be available in the app catalog.",
            ),
            (
                "appCatalogUrl",
                "URL of the tenant or site collection app catalog. It must be specified when the scope is `sitecollection`.",
            ),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
            ("force", "Don't prompt for confirming retracting the app."),
        ],
        notes: &[
            "When getting information about an app from the tenant app catalog, it's not necessary to specify the tenant app catalog URL. When the URL is not specified, the CLI will try to resolve the URL itself. Specifying the app catalog URL is required when you want to get information about an app from a site collection app catalog.",
            "When specifying site collection app catalog, you can specify the URL either with our without the AppCatalog part, for example `https://contoso.sharepoint.com/sites/team-a/AppCatalog` or `https://contoso.sharepoint.com/sites/team-a`. CLI will accept both formats.",
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Retract the specified app from the tenant app catalog. Try to resolve the URL of the tenant app catalog automatically. Additionally, will prompt for confirmation before actually retracting the app.",
                "m365 spo app retract --id 058140e3-0e37-44fc-a1d3-79c487d371a3",
            ),
            (
                "Retract the specified app from the tenant app catalog. Additionally, will prompt for confirmation before actually retracting the app.",
                "m365 spo app retract --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --appCatalogUrl https://contoso.sharepoint.com/sites/apps",
            ),
            (
                "Retract the specified app from the tenant app catalog. Try to resolve the URL of the tenant app catalog automatically. Will not prompt for confirmation before retracting the app.",
                "m365 spo app retract --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --force",
            ),
            (
                "Retract the specified app from a site collection app catalog of the specified site.",
                "m365 spo app retract --id d95f8c94-67a1-4615-9af8-361ad33be93c --appCatalogScope sitecollection --appCatalogUrl https://contoso.sharepoint.com/sites/site1",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app teamspackage download",
        summary: "Downloads Teams app package for an SPFx solution deployed to tenant app catalog",
        options: &[
            (
                "appItemUniqueId",
                "The unique ID of the SPFx app to download the Teams package for.",
            ),
            (
                "appItemId",
                "The ID of the list item behind the SPFx app to download the Teams package for.",
            ),
            (
                "appName",
                "The name of the sppkg file to download the Teams package for.",
            ),
            (
                "fileName",
                "Name of the file to save the package to. If not specified will use the name of the sppkg file with a `.zip` extension.",
            ),
            (
                "appCatalogUrl",
                "URL of the tenant app catalog. If not specified, the command will try to autodiscover it.",
            ),
        ],
        notes: &[
            "Download the Teams app package for an SPFx solution works only for solutions deployed to the tenant app catalog.",
            "If you try to download Teams app package for an SPFx solution that doesn't support deployment to Teams, you'll get the Request failed with status code 404 error.",
            "For maximum performance, specify the URL of the tenant app catalog, the item ID (`appItemId`) of the SPFx package for which you want to download the Teams app package and the name of the file where you want to save the downloaded package to (`fileName`).",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Downloads the Teams app package for the SPFx solution deployed to the tenant app catalog with the specified ID to a file with .zip extension named after the .sppkg file.",
                "m365 spo app teamspackage download --appItemId 1",
            ),
            (
                "Downloads the Teams app package for the SPFx solution deployed to the tenant app catalog with the specified unique item ID to the specified file.",
                "m365 spo app teamspackage download --appItemUniqueId 335a5612-3e85-462d-9d5b-c014b5abeac5 --fileName my-app.zip",
            ),
            (
                "Downloads the Teams app package for the SPFx solution deployed to the specified tenant app catalog.",
                "m365 spo app teamspackage download --appName my-app.sppkg --appCatalogUrl https://contoso.sharepoint.com/sites/appcatalog",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo app uninstall",
        summary: "Uninstalls an app from the site",
        options: &[
            ("id", "ID of the app to uninstall."),
            (
                "siteUrl",
                "Absolute URL of the site to uninstall the app from.",
            ),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
            ("force", "Don't prompt for confirming uninstalling the app."),
        ],
        notes: &[
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Uninstall the app with the specified ID from the specified site.",
                "m365 spo app uninstall --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "Uninstall the app with the specified ID from the specified site without prompting for confirmation.",
                "m365 spo app uninstall --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com --force",
            ),
            (
                "Uninstall the app with the specified ID from the specified site where the app is deployed to the site collection app catalog.",
                "m365 spo app uninstall --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com --appCatalogScope sitecollection",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo app upgrade",
        summary: "Upgrades app in the specified site",
        options: &[
            ("id", "ID of the app to upgrade."),
            ("siteUrl", "Absolute URL of the site to upgrade the app in."),
            (
                "appCatalogScope",
                "Scope of the app catalog. Allowed values: `tenant`, `sitecollection`. Defaults to `tenant`.",
            ),
        ],
        notes: &[
            "If the app with the specified ID doesn't exist in the app catalog, the command will fail with an error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Upgrade the app with the specified ID in the specified site.",
                "m365 spo app upgrade --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "Upgrade the app with the specified ID in the specified site from site collection app catalog.",
                "m365 spo app upgrade --id b2307a39-e878-458b-bc90-03bc578531d6 --siteUrl https://contoso.sharepoint.com --appCatalogScope sitecollection",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
            "https://learn.microsoft.com/sharepoint/dev/apis/alm-api-for-spfx-add-ins",
        )],
    },
    Help {
        name: "spo applicationcustomizer add",
        summary: "Adds an Application Customizer to a site",
        options: &[
            ("title", "The title of the Application Customizer."),
            ("webUrl", "URL of the site."),
            (
                "clientSideComponentId",
                "Client-side component ID of the Application Customizer (GUID).",
            ),
            ("description", "Description of the Application Customizer."),
            (
                "clientSideComponentProperties",
                "JSON string with Application Customizer properties.",
            ),
            (
                "hostProperties",
                "Set the host properties of the Application Customizer.",
            ),
            (
                "scope",
                "Scope of the Application Customizer. Allowed values: `Site`, `Web`. Defaults to `Site`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treats quotes differently. For example, this is how an Application Customizer can be created from the Windows cmd.exe:",
            "    m365 spo applicationcustomizer add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourAppCustomizer\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` or `--hostProperties` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for configuring an Application Customizer on a specific site. To configure an Application Customizer tenant-wide, view our dedicated spo tenant applicationcustomizer add command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Adds an Application Customizer to the sales site.",
                "m365 spo applicationcustomizer add --title 'Some customizer' --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Adds an Application Customizer to the sales site with some properties.",
                "m365 spo applicationcustomizer add --title 'Some customizer' --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --clientSideComponentProperties '{ \"someProperty\": \"Some value\" }' --webUrl https://contoso.sharepoint.com/sites/sales --scope 'Site'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo applicationcustomizer get",
        summary: "Gets an application customizer that is added to a site",
        options: &[
            ("webUrl", "URL of the site."),
            ("title", "The title of the Application Customizer."),
            ("id", "The id of the Application Customizer."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the application customizer.",
            ),
            (
                "scope",
                "Scope of the application customizer. Allowed values: `Site`, `Web`, `All`. Defaults to `All`.",
            ),
            (
                "clientSideComponentProperties",
                "Only output the client-side component properties.",
            ),
        ],
        notes: &[
            "This command can be used for retrieving an application customizer from a specific site. To retrieve an application customizer that's installed tenant-wide, view our dedicated spo tenant applicationcustomizer get command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieves an application customizer by title.",
                "m365 spo applicationcustomizer get --title \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Retrieves an application customizer by id.",
                "m365 spo applicationcustomizer get --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Retrieves an application customizer by clientSideComponentId.",
                "m365 spo applicationcustomizer get --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Retrieves an application customizer by title available at the site scope.",
                "m365 spo applicationcustomizer get --title \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales --scope site",
            ),
            (
                "Retrieves the client-side component properties of a application customizer.",
                "m365 spo applicationcustomizer get --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --clientSideComponentProperties",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo applicationcustomizer list",
        summary: "Gets a list of application customizers that are added to a site",
        options: &[
            ("webUrl", "The url of the site."),
            (
                "scope",
                "Scope of the application customizers. Allowed values `Site`, `Web`, `All`. Defaults to `All`.",
            ),
        ],
        notes: &[
            "This command can be used for retrieving a list of application customizers from a specific site. To retrieve a list of application customizers that are installed tenant-wide, view our dedicated spo tenant applicationcustomizer list command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "Retrieves a list of application customizers.",
            "m365 spo applicationcustomizer list --webUrl https://contoso.sharepoint.com/sites/sales",
        )],
        links: &[],
    },
    Help {
        name: "spo applicationcustomizer remove",
        summary: "Removes an application customizer that is added to a site",
        options: &[
            ("webUrl", "The url of the site."),
            ("title", "The title of the Application Customizer."),
            ("id", "The id of the Application Customizer."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the application customizer.",
            ),
            (
                "scope",
                "Scope of the application customizer. Allowed values: `Site`, `Web`, and `All`. Defaults to `All`.",
            ),
            (
                "force",
                "Don't prompt for confirming removal of the application customizer.",
            ),
        ],
        notes: &[
            "If the command finds multiple application customizers with the specified title or clientSideComponentId, it will prompt you to disambiguate which customizer it should remove, listing the discovered IDs.",
            "This command can be used for removing an application customizer added to a specific site. To remove an application customizer that's installed tenant-wide, view our dedicated spo tenant applicationcustomizer remove command.",
            "This command can be used for removing an application customizer from a specific site. To remove a tenant-wide installed application customizer, view our dedicated spo tenant applicationcustomizer remove command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove an application customizer by id.",
                "m365 spo applicationcustomizer remove --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Remove an application customizer by title.",
                "m365 spo applicationcustomizer remove --title \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Remove an application customizer by clientSideComponentId.",
                "m365 spo applicationcustomizer remove --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Remove an application customizer by its id without prompting for confirmation.",
                "m365 spo applicationcustomizer remove --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --force",
            ),
            (
                "Remove an application customizer from a site collection by its id without prompting for confirmation.",
                "m365 spo applicationcustomizer remove --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --force --scope Site",
            ),
            (
                "Remove an application customizer from a site by its id without prompting for confirmation.",
                "m365 spo applicationcustomizer remove --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --force --scope Web",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo applicationcustomizer set",
        summary: "Updates an existing Application Customizer on a site",
        options: &[
            (
                "webUrl",
                "The site to update the Application Customizer on.",
            ),
            (
                "title",
                "The title of the Application Customizer to update.",
            ),
            ("id", "The id of the Application Customizer to update."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the Application Customizer to update.",
            ),
            (
                "newTitle",
                "The new title of the Application Customizer. Specify an empty string to clear.",
            ),
            (
                "description",
                "Description of the Application Customizer. Specify an empty string to clear.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the Application Customizer.",
            ),
            (
                "hostProperties",
                "Set the host properties for the Application Customizer. Set to an empty string to clear the value.",
            ),
            (
                "scope",
                "The scope where to lookup the Application Customizer. Allowed values: `Site`, `Web`, and `All`. Defaults to `All`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for `clientSideComponentProperties` and `hostProperties` options that have JSON value because the command shell treats quotes differently. For example, this is how Application Customizer can be updated from the Windows cmd.exe:",
            "    m365 spo applicationcustomizer set --webUrl https://contoso.sharepoint.com/sites/sales --id b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --newTitle \"Some customizer\" --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option (--clientSideComponentProperties) has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` or `--hostProperties` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for updating an Application Customizer on a specific site. To update an Application Customizer that's installed tenant-wide, view our dedicated spo tenant applicationcustomizer set command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Updates the title of an Application Customizer on the sales site.",
                "m365 spo applicationcustomizer set --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --newTitle \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Updates the properties of an Application Customizer on the sales site.",
                "m365 spo applicationcustomizer set --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --clientSideComponentProperties '{ \"testMessage\": \"Test message\" }' --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo apppage add",
        summary: "Creates a single-part app page",
        options: &[
            (
                "webUrl",
                "The URL of the site where the page should be created.",
            ),
            ("title", "The title of the page to be created."),
            (
                "webPartData",
                "JSON string of the web part to put on the page.",
            ),
            (
                "addToQuickLaunch",
                "Set, to add the page to the quick launch.",
            ),
        ],
        notes: &[
            "If you want to add the single-part app page to quick launch, use the addToQuickLaunch flag.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[(
            "Create a single-part app page in the specified site, webpart data is stored in the `$webPartData` variable",
            "m365 spo apppage add --title \"Contoso\" --webUrl \"https://contoso.sharepoint.com\" --webPartData $webPartData --addToQuickLaunch",
        )],
        links: &[],
    },
    Help {
        name: "spo apppage set",
        summary: "Updates the single-part app page",
        options: &[
            (
                "webUrl",
                "The URL of the site where the page to update is located.",
            ),
            ("name", "The name of the page to be updated, eg. page.aspx."),
            (
                "webPartData",
                "JSON string of the web part to update on the page.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[(
            "Updates the single-part app page located in a specified site. Web part data is stored in the `$webPartData` variable.",
            "m365 spo apppage set --webUrl \"https://contoso.sharepoint.com\" --name \"Contoso.aspx\" --webPartData $webPartData",
        )],
        links: &[],
    },
    Help {
        name: "spo brandcenter colors list",
        summary: "Lists the brand center colors",
        options: &[],
        notes: &[
            "If the brand colors list has not been created yet in the Brand Center, the command will return an empty array.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "List all brand center colors",
            "m365 spo brandcenter colors list",
        )],
        links: &[],
    },
    Help {
        name: "spo brandcenter settings list",
        summary: "Lists the brand center configuration",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "List all brand center config",
            "m365 spo brandcenter settings list",
        )],
        links: &[],
    },
    Help {
        name: "spo cdn get",
        summary: "Views current status of the specified Microsoft 365 CDN",
        options: &[(
            "type",
            "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
        )],
        notes: &[
            "Using the `-t, --type` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Show if the Public CDN is currently enabled or not.",
                "m365 spo cdn get",
            ),
            (
                "Show if the Private CDN is currently enabled or not.",
                "m365 spo cdn get --type Private",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn origin add",
        summary: "Adds CDN origin to the current SharePoint Online tenant",
        options: &[
            (
                "type",
                "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
            ),
            ("origin", "Origin to add to the current CDN configuration."),
        ],
        notes: &[
            "Using the `-t, --type` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[(
            "Add */CDN to the list of origins of the Public CDN.",
            "m365 spo cdn origin add --type Public --origin */CDN",
        )],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn origin list",
        summary: "Lists CDN origins settings for the current SharePoint Online tenant",
        options: &[(
            "type",
            "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
        )],
        notes: &[
            "Using the `-t, --type` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Show the list of origins configured for the Public CDN.",
                "m365 spo cdn origin list",
            ),
            (
                "Show the list of origins configured for the Private CDN.",
                "m365 spo cdn origin list --type Private",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn origin remove",
        summary: "Removes CDN origin for the current SharePoint Online tenant",
        options: &[
            (
                "type",
                "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
            ),
            (
                "origin",
                "Origin to remove from the current CDN configuration.",
            ),
            (
                "force",
                "Don't prompt for confirming removal of a tenant property.",
            ),
        ],
        notes: &[
            "Using the `-t, --type` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[(
            "Remove */CDN from the list of origins of the Public CDN.",
            "m365 spo cdn origin remove --type Public --origin */CDN",
        )],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn policy list",
        summary: "Lists CDN policies settings for the current SharePoint Online tenant",
        options: &[(
            "cdnType",
            "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
        )],
        notes: &[
            "Using the `-t, --cdnType` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Show the list of policies configured for the Public CDN.",
                "m365 spo cdn policy list",
            ),
            (
                "Show the list of policies configured for the Private CDN.",
                "m365 spo cdn policy list --cdnType Private",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn policy set",
        summary: "Sets CDN policy value for the current SharePoint Online tenant",
        options: &[
            (
                "cdnType",
                "Type of CDN to manage. Allowed values are: `Public`, `Private`. Default `Public`.",
            ),
            (
                "policy",
                "CDN policy to configure. Allowed values are: `IncludeFileExtensions`, `ExcludeRestrictedSiteClassifications`.",
            ),
            ("value", "Value for the policy to configure."),
        ],
        notes: &[
            "Using the `-t, --cdnType` option you can choose whether you want to manage the settings of the Public (default) or Private CDN. If you don't use the option, the command will use the Public CDN.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[(
            "Set the list of extensions supported by the Public CDN",
            "m365 spo cdn policy set --cdnType Public --policy IncludeFileExtensions --value CSS,EOT,GIF,ICO,JPEG,JPG,JS,MAP,PNG,SVG,TTF,WOFF,JSON",
        )],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo cdn set",
        summary: "Enables or disables the specified Microsoft 365 CDN",
        options: &[
            (
                "enabled",
                "Set to true to enable CDN or to false to disable it. Allowed values: `true`, `false`.",
            ),
            (
                "type",
                "Type of CDN to update. Allowed values: `Public`, `Private`, `Both`. Defaults to `Public`.",
            ),
            (
                "noDefaultOrigins",
                "Specify this flag to not create the default origins.",
            ),
        ],
        notes: &[
            "Using the `-t, --type` option you can choose whether you want to manage the settings of the Public (default), Private CDN or both. If you don't use the option, the command will use the Public CDN.",
            "Using the `-e, --enabled` option you can specify whether the given CDN type should be enabled or disabled. Use true to enable the specified CDN and false to disable it.",
            "Using the `--noDefaultOrigins` option you can specify to skip the creation of the default origins.",
            "To use this command you must be a Global or SharePoint administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Enable the Microsoft 365 Public CDN on the current tenant",
                "m365 spo cdn set --type Public --enabled true",
            ),
            (
                "Disable the Microsoft 365 Public CDN on the current tenant",
                "m365 spo cdn set --type Public --enabled false",
            ),
            (
                "Enable the Microsoft 365 Private CDN on the current tenant",
                "m365 spo cdn set --type Private --enabled true",
            ),
            (
                "Enable the Microsoft 365 Private and Public CDN on the current tenant with default origins",
                "m365 spo cdn set --type Both --enabled true",
            ),
            (
                "Enable the Microsoft 365 Private and Public CDN on the current tenant without default origins",
                "m365 spo cdn set --type Both --enabled true --noDefaultOrigins",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
            "https://learn.microsoft.com/microsoft-365/enterprise/use-microsoft-365-cdn-with-spo?view=o365-worldwide",
        )],
    },
    Help {
        name: "spo commandset add",
        summary: "Adds a ListView Command Set to a site.",
        options: &[
            ("title", "The title of the ListView Command Set."),
            ("webUrl", "The site to add the ListView Command Set."),
            (
                "listType",
                "The list or library type to register the Command Set on. Allowed values are: `List`, `Library` or `SitePages`.",
            ),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set.",
            ),
            ("description", "Description of the ListView Command Set."),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the ListView Command Set.",
            ),
            (
                "scope",
                "Scope of the ListView Command Set. Allowed values are: `Site`, `Web`. Defaults to `Web`.",
            ),
            (
                "location",
                "The location of the ListView Command Set. Allowed values are: `ContextMenu`, `CommandBar` or `Both`. Defaults to `CommandBar`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML, or JavaScript values because the command shell treats quotes differently. For example, this is how a ListView Command Set can be created from the Windows cmd.exe:",
            "    m365 spo commandset add --webUrl https://contoso.sharepoint.com/sites/test --title \"CLI Commandset\" --location \"ContextMenu\" --listType \"List\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option (-p) has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for configuring ListView Command Sets on a specific site. To configure a ListView Command Set tenant-wide, view our dedicated spo tenant commandset add command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Adds a ListView Command Set to lists on the sales site.",
                "m365 spo commandset add --title \"Some customizer\" --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --listType List --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Adds a ListView Command Set to lists on the sales site with some properties.",
                "m365 spo commandset add --title \"Some customizer\" --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --clientSideComponentProperties '{ \"someProperty\": \"Some value\" }' --listType List --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo commandset get",
        summary: "Gets a ListView Command Set that is added to a site",
        options: &[
            ("webUrl", "Url of the site."),
            ("title", "The title of the ListView Command Set."),
            ("id", "The id of the ListView Command Set."),
            (
                "clientSideComponentId",
                "The id of the ListView Command Set.",
            ),
            (
                "scope",
                "Scope of the ListView Command Set. Allowed values: `Site`, `Web`, `All`. Defaults to `All`.",
            ),
            (
                "clientSideComponentProperties",
                "Only output the client-side component properties.",
            ),
        ],
        notes: &[
            "If the command finds multiple command sets with the specified title, it will prompt you to disambiguate which command set it should use, listing the discovered IDs.",
            "This command can be used for retrieving a ListView Command Set from a specific site. To retrieve a ListView Command Set that's installed tenant-wide, view our dedicated spo tenant commandset get command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieves a ListView Command Set by title.",
                "m365 spo commandset get --title \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Retrieves a ListView Command Set by id with scope 'Web'.",
                "m365 spo commandset get --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --scope Web",
            ),
            (
                "Retrieves a ListView Command Set by clientSideComponentId with scope 'Site'.",
                "m365 spo commandset get --clientSideComponentId c1cbd896-5140-428d-8b0c-4873be19f5ac --webUrl https://contoso.sharepoint.com/sites/sales --scope Site",
            ),
            (
                "Retrieves the client-side component properties of a ListView Command Set.",
                "m365 spo commandset get --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales --clientSideComponentProperties",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo commandset list",
        summary: "Gets a list of ListView Command Sets that are added to a site",
        options: &[
            ("webUrl", "The url of the site."),
            (
                "scope",
                "Scope of the ListView Command Sets. Allowed values: `Site`, `Web`, `All`. Defaults to `All`.",
            ),
        ],
        notes: &[
            "This command can be used for retrieving a list of ListView Command Sets from a specific site. To retrieve a list of ListView Command Sets that are installed tenant-wide, view our dedicated spo tenant commandset list command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "Retrieves a list of ListView Command Sets.",
            "m365 spo commandset list --webUrl https://contoso.sharepoint.com/sites/sales",
        )],
        links: &[],
    },
    Help {
        name: "spo commandset remove",
        summary: "Removes a ListView Command Set that is added to a site",
        options: &[
            ("webUrl", "Url of the site."),
            ("title", "The title of the ListView Command Set."),
            ("id", "The id of the ListView Command Set."),
            (
                "clientSideComponentId",
                "The id of the ListView Command Set.",
            ),
            (
                "scope",
                "Scope of the ListView Command Set. Allowed values are: `Site`, `Web`, `All`. Defaults to `All`.",
            ),
            (
                "force",
                "Don't prompt for confirming removal of the ListView Command Set.",
            ),
        ],
        notes: &[
            "This command can be used for removing a ListView Command Set from a specific site. To remove a ListView Command Set that's installed tenant-wide, view our dedicated spo tenant commandset remove command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove a ListView Command Set by title.",
                "m365 spo commandset remove --title \"Some customizer\" --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Remove a ListView Command Set by id.",
                "m365 spo commandset remove --id 14125658-a9bc-4ddf-9c75-1b5767c9a337 --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Remove a ListView Command Set by clientSideComponentId.",
                "m365 spo commandset remove --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo commandset set",
        summary: "Updates a ListView Command Set on a site",
        options: &[
            ("webUrl", "The site to update the ListView Command Set on."),
            ("title", "The title of the ListView Command Set to update."),
            ("id", "The id of the ListView Command Set to update."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set to update.",
            ),
            (
                "newClientSideComponentId",
                "The new Client Side Component Id (GUID) of the ListView Command Set.",
            ),
            ("newTitle", "The new title of the ListView Command Set."),
            (
                "description",
                "Description of the ListView Command Set. Specify an empty string to clear.",
            ),
            (
                "listType",
                "The list or library type to register the Command Set on. Allowed values are: `List`, `Library` or `SitePages`.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the ListView Command Set.",
            ),
            (
                "scope",
                "The scope where to lookup the ListView Command Set: at site level or web level. Allowed values are: `Site`, `Web`, `All`. Defaults to `All`.",
            ),
            (
                "location",
                "The location of the ListView Command Set. Allowed values are: `ContextMenu`, `CommandBar` or `Both`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treats quotes differently. For example, this is how a ListView Command Set can be created from the Windows cmd.exe:",
            "    m365 spo commandset set --webUrl https://contoso.sharepoint.com/sites/test --title \"CLI Commandset\" --location \"Both\" --listType \"List\" --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for updating a ListView Command Set on a specific site. To update a ListView Command Set that's installed tenant-wide, view our dedicated spo tenant commandset set command.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Updates the title and location of a ListView Command Set on the sales site.",
                "m365 spo commandset set --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --newTitle \"Some new title\" --location Both --webUrl https://contoso.sharepoint.com/sites/sales --scope Site",
            ),
            (
                "Updates a ListView Command Set on the sales site with some properties.",
                "m365 spo commandset set --title \"Some customizer\" --clientSideComponentProperties '{ \"someProperty\": \"Some value\" }' --webUrl https://contoso.sharepoint.com/sites/sales --scope Site",
            ),
            (
                "Updates the Client Side Component Id of a ListView Command Set.",
                "m365 spo commandset set --title \"Some customizer\" --newClientSideComponentId b2c5faf3-638f-44ae-bfde-1730d94283bf --webUrl https://contoso.sharepoint.com/sites/sales",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype add",
        summary: "Adds a new list or site content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type should be created.",
            ),
            (
                "listTitle",
                "Title of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "ID of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "id",
                "The ID of the content type. Determines the parent content type.",
            ),
            ("name", "The name of the content type."),
            ("description", "The description of the content type."),
            (
                "group",
                "The group with which the content type should be associated.",
            ),
        ],
        notes: &[
            "If the specified content type already exists, you will get a A duplicate content type \"Your Content Type\" was found. error.",
            "The ID of the content type specifies the parent content type from which this content type inherits.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Create a site content type that inherits from the List item content type.",
                "m365 spo contenttype add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --name 'PnP Alert' --id 0x01007926A45D687BA842B947286090B8F67D --group 'PnP Content Types'",
            ),
            (
                "Create a list (retrieved by Title) content type that inherits from the List item content type.",
                "m365 spo contenttype add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Alerts --name 'PnP Alert' --id 0x01007926A45D687BA842B947286090B8F67D",
            ),
            (
                "Create a list (retrieved by ID) content type that inherits from the List item content type.",
                "m365 spo contenttype add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '8c7a0fcd-9d64-4634-85ea-ce2b37b2ec0c' --name 'PnP Alert' --id 0x01007926A45D687BA842B947286090B8F67D",
            ),
            (
                "Create a list (retrieved by URL) content type that inherits from the List item content type.",
                "m365 spo contenttype add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/Shared Documents' --name 'PnP Alert' --id 0x01007926A45D687BA842B947286090B8F67D",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/previous-versions/office/developer/sharepoint-2010/aa543822(v%3Doffice.14)",
            "https://learn.microsoft.com/previous-versions/office/developer/sharepoint-2010/aa543822(v%3Doffice.14",
        )],
    },
    Help {
        name: "spo contenttype field list",
        summary: "Lists fields for a given site or list content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type is located.",
            ),
            (
                "contentTypeId",
                "The ID of the content type for which to list fields.",
            ),
            (
                "contentTypeName",
                "The name of the content type for which to list fields.",
            ),
            (
                "listTitle",
                "Optional title of the list where the content type is located.",
            ),
            (
                "listId",
                "Optional ID of the list where the content type is located.",
            ),
            (
                "listUrl",
                "Optional server- or web-relative URL of the list where the content type is located.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve. Will retrieve all properties if not specified.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieves fields for the specified site content type.",
                "m365 spo contenttype field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --contentTypeId \"0x01\"",
            ),
            (
                "Retrieves fields for the specified list content type in the Documents library.",
                "m365 spo contenttype field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --contentTypeName \"Document\" --listTitle \"Documents\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype field remove",
        summary: "Removes a column from a site or list content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type is located.",
            ),
            (
                "listTitle",
                "Title of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "ID of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "contentTypeId",
                "The ID of the content type to remove the column from.",
            ),
            ("id", "The ID of the column to remove."),
            ("updateChildContentTypes", "Update child content types."),
            (
                "force",
                "Don't prompt for confirming removal of a column from content type.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove column with a specific ID from a content type with a specific ID from a specific site.",
                "m365 spo contenttype field remove --contentTypeId \"0x0100CA0FA0F5DAEF784494B9C6020C3020A6\" --id \"880d2f46-fccb-43ca-9def-f88e722cef80\" --webUrl https://contoso.sharepoint.com --force",
            ),
            (
                "Remove column with a specific ID from a content type with a specific ID from a specific site and updating the child content types.",
                "m365 spo contenttype field remove --contentTypeId \"0x0100CA0FA0F5DAEF784494B9C6020C3020A6\" --id \"880d2f46-fccb-43ca-9def-f88e722cef80\" --webUrl https://contoso.sharepoint.com --updateChildContentTypes",
            ),
            (
                "Remove fieldLink with a specific ID from a list (obtained by Title) content type with a specific ID on a specific site.",
                "m365 spo contenttype field remove --contentTypeId \"0x0100CA0FA0F5DAEF784494B9C6020C3020A60062F089A38C867747942DB2C3FC50FF6A\" --id \"880d2f46-fccb-43ca-9def-f88e722cef80\" --webUrl https://contoso.sharepoint.com --listTitle \"Documents\"",
            ),
            (
                "Remove fieldLink with a specific ID from a list (obtained by ID) content type with a specific ID on a specific site.",
                "m365 spo contenttype field remove --contentTypeId \"0x0100CA0FA0F5DAEF784494B9C6020C3020A60062F089A38C867747942DB2C3FC50FF6A\" --id \"880d2f46-fccb-43ca-9def-f88e722cef80\" --webUrl https://contoso.sharepoint.com --listId \"8c7a0fcd-9d64-4634-85ea-ce2b37b2ec0c\"",
            ),
            (
                "Remove fieldLink with a specific ID from a list (obtained by URL) content type with a specific ID on a specific site.",
                "m365 spo contenttype field remove --contentTypeId \"0x0100CA0FA0F5DAEF784494B9C6020C3020A60062F089A38C867747942DB2C3FC50FF6A\" --id \"880d2f46-fccb-43ca-9def-f88e722cef80\" --webUrl https://contoso.sharepoint.com --listUrl \"/shared documents\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype field set",
        summary: "Adds or updates a site column reference in a site content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type is located.",
            ),
            (
                "contentTypeId",
                "ID of the content type on which the field reference should be set.",
            ),
            (
                "id",
                "ID of the field to which the reference should be set.",
            ),
            (
                "required",
                "Set to `true`, if the field should be required or to `false` if it should be optional.",
            ),
            (
                "hidden",
                "Set to `true`, if the field should be hidden or to `false` if it should be visible.",
            ),
        ],
        notes: &[
            "If the field reference already exists, the command will update its required and hidden properties as specified in the command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add the specified site column to the specified content type as an optional and visible field.",
                "m365 spo contenttype field set --webUrl https://contoso.sharepoint.com/sites/portal --contentTypeId 0x01007926A45D687BA842B947286090B8F67D --id ebe7e498-44ff-43da-a7e5-99b444f656a5",
            ),
            (
                "Add the specified site column to the specified content type as a required field.",
                "m365 spo contenttype field set --webUrl https://contoso.sharepoint.com/sites/portal --contentTypeId 0x01007926A45D687BA842B947286090B8F67D --id ebe7e498-44ff-43da-a7e5-99b444f656a5 --required true",
            ),
            (
                "Update the existing site column reference in the specified content type to optional.",
                "m365 spo contenttype field set --webUrl https://contoso.sharepoint.com/sites/portal --contentTypeId 0x01007926A45D687BA842B947286090B8F67D --id ebe7e498-44ff-43da-a7e5-99b444f656a5 --required false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype get",
        summary: "Retrieves information about the specified list or site content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type is located.",
            ),
            (
                "listTitle",
                "Title of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "ID of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list (if it is a list content type). Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "id",
                "The ID of the content type to retrieve. Specify either id or name but not both.",
            ),
            (
                "name",
                "The name of the content type to retrieve. Specify either id or name but not both.",
            ),
        ],
        notes: &[
            "If no content type with the specified is found in the site or the list, you will get the Content type with ID 0x010012 not found error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieve site content type by id.",
                "m365 spo contenttype get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --id 0x0100558D85B7216F6A489A499DB361E1AE2F",
            ),
            (
                "Retrieve site content type by name.",
                "m365 spo contenttype get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --name 'Document'",
            ),
            (
                "Retrieve list (retrieved by Title) content type.",
                "m365 spo contenttype get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --id 0x0100558D85B7216F6A489A499DB361E1AE2F",
            ),
            (
                "Retrieve list (retrieved by ID) content type.",
                "m365 spo contenttype get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '8c7a0fcd-9d64-4634-85ea-ce2b37b2ec0c' --id 0x0100558D85B7216F6A489A499DB361E1AE2F",
            ),
            (
                "Retrieve list (retrieved by URL) content type.",
                "m365 spo contenttype get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/Shared Documents' --id 0x0100558D85B7216F6A489A499DB361E1AE2F",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype list",
        summary: "Lists content types from specified site",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site for which to list content types.",
            ),
            (
                "category",
                "Category name of content types. When defined will return only content types from specified category.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieve site content types.",
                "m365 spo contenttype list --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"",
            ),
            (
                "Retrieve site content types from the 'List Content Types' category.",
                "m365 spo contenttype list --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --category \"List Content Types\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype remove",
        summary: "Deletes site content type",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the content type is located.",
            ),
            ("id", "The ID of the content type to remove."),
            ("name", "The name of the content type to remove."),
            (
                "force",
                "Don't prompt for confirming removal of the content type.",
            ),
        ],
        notes: &[
            "If the specified content type is in use by a list and cannot be removed, you will be returned the error: Another site or list is still using this content type. SharePoint will not allow a content type to be removed unless any dependent objects are also emptied from the recycle bin including the second-stage recycle bin.",
            "The content type you wish to remove can be selected by the ID or Name of the content type. Either ID or Name parameter must be specified.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove a site content type by ID",
                "m365 spo contenttype remove --id \"0x01007926A45D687BA842B947286090B8F67D\" --webUrl https://contoso.sharepoint.com",
            ),
            (
                "Remove a site content type by Name",
                "m365 spo contenttype remove --name \"My Content Type\" --webUrl https://contoso.sharepoint.com --force",
            ),
            (
                "Remove a site content type without prompting for confirmation",
                "m365 spo contenttype remove --name \"My Content Type\" --webUrl https://contoso.sharepoint.com --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype set",
        summary: "Updates an existing content type",
        options: &[
            (
                "webUrl",
                "URL of the site where the content type to update is defined.",
            ),
            ("id", "ID of the content type to update."),
            ("name", "Name of the content type to update."),
            (
                "listTitle",
                "Title of the list if you want to update a list content type.",
            ),
            (
                "listId",
                "ID of the list if you want to update a list content type.",
            ),
            (
                "listUrl",
                "URL of the list if you want to update a list content type.",
            ),
            (
                "updateChildren",
                "Specify if you want to push updates to child content types.",
            ),
        ],
        notes: &[
            "When specifying the `--updateChildren` flag, SharePoint will only propagate the changes that are made in the current request. If you want to know more about updating a content type and propagating changes to child content types, be sure to read more here#considerations-when-updating-child-content-types).",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Move site content type to a different group.",
                "m365 spo contenttype set --id 0x001001 --webUrl https://contoso.sharepoint.com --Group \"My group\"",
            ),
            (
                "Rename list content type.",
                "m365 spo contenttype set --name \"My old item\" --webUrl https://contoso.sharepoint.com --listTitle \"My list\" --Name \"My item\"",
            ),
            (
                "Configure a form customizer with manifest ID 19890cce-15d8-4af9-bfcb-72da06d13ed8 on a site content type and push changes to child content types.",
                "m365 spo contenttype set --name \"My content type\" --webUrl https://contoso.sharepoint.com --DisplayFormClientSideComponentId \"19890cce-15d8-4af9-bfcb-72da06d13ed8\" --EditFormClientSideComponentId \"19890cce-15d8-4af9-bfcb-72da06d13ed8\" --NewFormClientSideComponentId \"19890cce-15d8-4af9-bfcb-72da06d13ed8\" --updateChildren",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttype sync",
        summary: "Adds a published content type from the content type hub to a site or syncs its latest changes.",
        options: &[
            ("webUrl", "The site to sync the Content Type to."),
            (
                "id",
                "The Id of the published Content Type to sync from the Content Type Hub.",
            ),
            (
                "name",
                "The name of the published Content Type to sync from the Content Type Hub.",
            ),
            (
                "listTitle",
                "Title of the list. Omit to sync as a site content type.",
            ),
            (
                "listId",
                "ID of the list. Omit to sync as a site content type.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list. Omit to sync as a site content type.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Syncs a given published content type from the hub to the specified site.",
                "m365 spo contenttype sync —webUrl https://contoso.sharepoint.com/sites/sales --id 0x01007926A45D687BA842B947286090B8F67D",
            ),
            (
                "Syncs a given published content type from the hub to the specified site and adds it to the specified list.",
                "m365 spo contenttype sync —webUrl https://contoso.sharepoint.com/sites/sales --id 0x01007926A45D687BA842B947286090B8F67D --listTitle Contacts",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo contenttypehub get",
        summary: "Returns the URL of the SharePoint Content Type Hub of the Tenant",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Retrieve the Content Type Hub URL",
            "m365 spo contenttypehub get",
        )],
        links: &[],
    },
    Help {
        name: "spo customaction add",
        summary: "Adds a user custom action for site or site collection",
        options: &[
            (
                "webUrl",
                "Url of the site or site collection to add the custom action",
            ),
            ("name", "The name of the custom action"),
            ("title", "The title of the custom action"),
            (
                "location",
                "The actual location where this custom action need to be added like `CommandUI.Ribbon`",
            ),
            (
                "group",
                "The group where this custom action needs to be added like `SiteActions`",
            ),
            ("description", "The description of the custom action"),
            (
                "sequence",
                "Sequence of this CustomAction being injected. Use when you have a specific sequence with which to have multiple CustomActions being added to the page",
            ),
            (
                "actionUrl",
                "The URL, URI or JavaScript function associated with the action. URL example `~site/_layouts/sampleurl.aspx` or `~sitecollection/_layouts/sampleurl.aspx`",
            ),
            (
                "imageUrl",
                "The URL of the image associated with the custom action",
            ),
            (
                "commandUIExtension",
                "XML fragment that determines user interface properties of the custom action",
            ),
            (
                "registrationId",
                "Specifies the identifier of the list or item content type that this action is associated with, or the file type or programmatic identifier",
            ),
            (
                "registrationType",
                "Specifies the type of object associated with the custom action. Allowed values `None`, `List`, `ContentType`, `ProgId`, `FileType`. Default `None`",
            ),
            (
                "rights",
                "A case sensitive string array that contain the permissions needed for the custom action. Allowed values `EmptyMask`, `ViewListItems`, `AddListItems`, `EditListItems`, `DeleteListItems`, `ApproveItems`, `OpenItems`, `ViewVersions`, `DeleteVersions`, `CancelCheckout`, `ManagePersonalViews`, `ManageLists`, `ViewFormPages`, `AnonymousSearchAccessList`, `Open`, `ViewPages`, `AddAndCustomizePages`, `ApplyThemeAndBorder`, `ApplyStyleSheets`, `ViewUsageData`, `CreateSSCSite`, `ManageSubwebs`, `CreateGroups`, `ManagePermissions`, `BrowseDirectories`, `BrowseUserInfo`, `AddDelPrivateWebParts`, `UpdatePersonalWebParts`, `ManageWeb`, `AnonymousSearchAccessWebLists`, `UseClientIntegration`, `UseRemoteAPIs`, `ManageAlerts`, `CreateAlerts`, `EditMyUserInfo`, `EnumeratePermissions`, `FullMask`. Default `EmptyMask`",
            ),
            (
                "scope",
                "Scope of the custom action. Allowed values `Site`, `Web`. Default `Web`",
            ),
            (
                "scriptBlock",
                "Specifies a block of script to be executed. This attribute is only applicable when the Location attribute is set to ScriptLink",
            ),
            (
                "scriptSrc",
                "Specifies a file that contains script to be executed. This attribute is only applicable when the Location attribute is set to ScriptLink",
            ),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the custom action",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component Properties of the custom action. Specify values as a JSON string : `'{\"testMessage\":\"Test message\"}'`",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treats quotes differently. For example, this is how ApplicationCustomizer user custom action can be created from the Windows cmd.exe:",
            "    m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourAppCustomizer\" --name \"YourName\" --location \"ClientSideExtension.ApplicationCustomizer\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option (-p) has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "The `--rights` option accepts case sensitive values.",
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Adds a tenant-wide SharePoint Framework Application Customizer extension in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourAppCustomizer\" --name \"YourName\" --location \"ClientSideExtension.ApplicationCustomizer\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\"testMessage\":\"Test message\"}'",
            ),
            (
                "Adds a tenant-wide SharePoint Framework modern List View Command Set extension in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourCommandSet\" --name \"YourName\" --location \"ClientSideExtension.ListViewCommandSet\" --clientSideComponentId db3e6e35-363c-42b9-a254-ca661e437848 --clientSideComponentProperties '{\"sampleTextOne\":\"One item is selected in the list.\", \"sampleTextTwo\":\"This command is always visible.\"}' --registrationId 100 --registrationType List",
            ),
            (
                "Creates a url custom action in the SiteActions menu in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"Microsoft.SharePoint.StandardMenu\" --group \"SiteActions\" --actionUrl \"~site/SitePages/Home.aspx\" --sequence 100",
            ),
            (
                "Creates a custom action in the classic Document Library edit context menu on the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"EditControlBlock\" --actionUrl \"javascript:(function(){ return console.log('CLI for Microsoft 365 rocks!'); })();\" --registrationId 101 --registrationType List",
            ),
            (
                "Creates a ScriptLink custom action with script source in classic pages in the specified site collection.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"ScriptLink\" --scriptSrc \"~sitecollection/SiteAssets/YourScript.js\" --sequence 101 --scope Site",
            ),
            (
                "Creates a ScriptLink custom action with script block in classic pages in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"ScriptLink\" --scriptBlock \"(function(){ return console.log('Hello CLI for Microsoft 365!'); })();\" --sequence 102",
            ),
            (
                "Creates a classic List View custom action located in the Ribbon in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"CommandUI.Ribbon\" --commandUIExtension '<CommandUIExtension><CommandUIDefinitions><CommandUIDefinition Location=\"Ribbon.List.Share.Controls._children\"><Button Id=\"Ribbon.List.Share.GetItemsCountButton\" Alt=\"Get list items count\" Sequence=\"11\" Command=\"Invoke_GetItemsCountButtonRequest\" LabelText=\"Get Items Count\" TemplateAlias=\"o1\" Image32by32=\"_layouts/15/images/placeholder32x32.png\" Image16by16=\"_layouts/15/images/placeholder16x16.png\" /></CommandUIDefinition></CommandUIDefinitions><CommandUIHandlers><CommandUIHandler Command=\"Invoke_GetItemsCountButtonRequest\" CommandAction=\"javascript: alert(ctx.TotalListItems);\" EnabledScript=\"javascript: function checkEnable() { return (true);} checkEnable();\"/></CommandUIHandlers></CommandUIExtension>'",
            ),
            (
                "Creates a custom action with delegated rights in the SiteActions menu in the specified site.",
                "m365 spo customaction add --webUrl https://contoso.sharepoint.com/sites/test --title \"YourTitle\" --name \"YourName\" --location \"Microsoft.SharePoint.StandardMenu\" --group \"SiteActions\" --actionUrl \"~site/SitePages/Home.aspx\" --rights \"AddListItems,DeleteListItems,ManageLists\"",
            ),
        ],
        links: &[
            (
                "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
                "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/bb802730.aspx",
                "https://msdn.microsoft.com/en-us/library/office/bb802730.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/ms460194.aspx",
                "https://msdn.microsoft.com/en-us/library/office/ms460194.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/microsoft.sharepoint.spbasepermissions.aspx",
                "https://msdn.microsoft.com/en-us/library/office/microsoft.sharepoint.spbasepermissions.aspx",
            ),
        ],
    },
    Help {
        name: "spo customaction clear",
        summary: "Deletes all custom actions from site or site collection",
        options: &[
            (
                "webUrl",
                "Url of the site or site collection to clear the custom actions from.",
            ),
            (
                "scope",
                "Scope of the custom action. Allowed values `Site`, `Web`, `All`. Default `All`.",
            ),
            (
                "force",
                "Don't prompt for confirming removing all custom actions.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Clears all user custom actions for both site and site collection. Skips the confirmation prompt message.",
                "m365 spo customaction clear --webUrl https://contoso.sharepoint.com/sites/test --force",
            ),
            (
                "Clears all user custom actions for the specified site.",
                "m365 spo customaction clear --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
            (
                "Clears all user custom actions for the specified site collection.",
                "m365 spo customaction clear --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo customaction get",
        summary: "Gets information about a user custom action for site or site collection",
        options: &[
            (
                "id",
                "ID of the user custom action to retrieve information for.",
            ),
            (
                "title",
                "Title of the user custom action to retrieve information for.",
            ),
            (
                "clientSideComponentId",
                "clientSideComponentId of the user custom action to retrieve information for.",
            ),
            (
                "webUrl",
                "Url of the site or site collection to retrieve the custom action from.",
            ),
            (
                "scope",
                "Scope of the custom action. Allowed values `Site`, `Web`, `All`. Default `All`.",
            ),
        ],
        notes: &[
            "If the command finds multiple user custom actions with the specified `title` or `clientSideComponentId`, it will prompt you to disambiguate which user custom action it should get, listing the discovered IDs.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return details about the user custom action based on the id and a given url.",
                "m365 spo customaction get --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Return details about the user custom action based on the title and a given url.",
                "m365 spo customaction get --title \"YourAppCustomizer\" --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Return details about the user custom action based on the clientSideComponentId and a given url.",
                "m365 spo customaction get --clientSideComponentId \"34a019f9-6198-4053-a3b6-fbdea9a107fd\" --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Return details about the user custom action based on the id and a given url and the scope.",
                "m365 spo customaction get --id \"058140e3-0e37-44fc-a1d3-79c487d371a3\" --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
            (
                "Return details about the user custom action based on the id and a given url and the scope.",
                "m365 spo customaction get --id \"058140e3-0e37-44fc-a1d3-79c487d371a3\" --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
            (
                "Return details about the user custom action based on the id and a given url and the scope.",
                "m365 spo customaction get --id \"058140e3-0e37-44fc-a1d3-79c487d371a3\" --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo customaction list",
        summary: "Lists user custom actions for site or site collection",
        options: &[
            (
                "webUrl",
                "Url of the site or site collection to retrieve the custom action from.",
            ),
            (
                "scope",
                "Scope of the custom action. Allowed values `Site`, `Web`, `All`. Default `All`.",
            ),
        ],
        notes: &[
            "When using the text output type (default), the command lists only the values of the `Name`, `Location`, `Scope` and `Id` properties of the custom action. When setting the output type to JSON, all available properties are included in the command output.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return details about all user custom actions located in the specified site or site collection.",
                "m365 spo customaction list --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Return details about all user custom actions located in the specified site collection.",
                "m365 spo customaction list --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
            (
                "Return details about all user custom actions located in the specified site.",
                "m365 spo customaction list --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
        ],
        links: &[(
            "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
            "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
        )],
    },
    Help {
        name: "spo customaction remove",
        summary: "Removes specified custom action from site or site collection",
        options: &[
            ("id", "Id (GUID) of the custom action to remove."),
            (
                "title",
                "Title of the user custom action to retrieve information for.",
            ),
            (
                "webUrl",
                "Url of the site or site collection to remove the custom action from.",
            ),
            (
                "scope",
                "Scope of the custom action. Allowed values `Site`, `Web`, `All`. Default `All`.",
            ),
            (
                "force",
                "Don't prompt for confirming removal of a user custom action.",
            ),
        ],
        notes: &[
            "If the command finds multiple user custom actions with the specified title, it will prompt you to disambiguate which user custom action it should use, listing the discovered IDs.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Removes user custom action with given ID located in the specified site or site collection.",
                "m365 spo customaction remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Removes user custom action with given Title located in the specified site or site collection.",
                "m365 spo customaction remove --title \"YourAppCustomizer\" --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Removes user custom action with given ID located in the specified site or site collection. Skips the confirmation prompt message.",
                "m365 spo customaction remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --webUrl https://contoso.sharepoint.com/sites/test --force",
            ),
            (
                "Removes user custom action with given Title located in the specified site or site collection. Skips the confirmation prompt message.",
                "m365 spo customaction remove --title \"YourAppCustomizer\" --webUrl https://contoso.sharepoint.com/sites/test --force",
            ),
            (
                "Removes user custom action with given ID located in the specified site collection.",
                "m365 spo customaction remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
            (
                "Removes user custom action with given Title located in site collection https://contoso.sharepoint.com/sites/test",
                "m365 spo customaction remove --title \"YourAppCustomizer\" --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
            (
                "Removes user custom action with given ID located in the specified site.",
                "m365 spo customaction remove --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
            (
                "Removes user custom action with given Title located in the specified site.",
                "m365 spo customaction remove --title \"YourAppCustomizer\" --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
        ],
        links: &[(
            "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
            "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
        )],
    },
    Help {
        name: "spo customaction set",
        summary: "Updates a user custom action for site or site collection",
        options: &[
            (
                "webUrl",
                "Url of the site or site collection to update the custom action.",
            ),
            ("id", "ID of the custom action to update."),
            ("name", "The name of the custom action."),
            ("title", "The title of the custom action."),
            (
                "location",
                "The actual location where this custom action need to be added like `CommandUI.Ribbon`.",
            ),
            (
                "group",
                "The group of the custom action like `SiteActions`.",
            ),
            ("description", "The description of the custom action."),
            (
                "sequence",
                "Sequence of this CustomAction being injected. Use when you have a specific sequence with which to have multiple CustomActions being added to the page.",
            ),
            (
                "actionUrl",
                "The URL, URI or JavaScript function associated with the action. URL example `~site/_layouts/sampleurl.aspx` or `~sitecollection/_layouts/sampleurl.aspx`.",
            ),
            (
                "imageUrl",
                "The URL of the image associated with the custom action.",
            ),
            (
                "commandUIExtension",
                "XML fragment that determines user interface properties of the custom action.",
            ),
            (
                "registrationId",
                "Specifies the identifier of the list or item content type that this action is associated with, or the file type or programmatic identifier.",
            ),
            (
                "registrationType",
                "Specifies the type of object associated with the custom action. Allowed values `None`, `List`, `ContentType`, `ProgId`, `FileType`. Defaults to `None`.",
            ),
            (
                "rights",
                "A case-sensitive string array that contain the permissions needed for the custom action. Allowed values `EmptyMask`, `ViewListItems`, `AddListItems`, `EditListItems`, `DeleteListItems`, `ApproveItems`, `OpenItems`, `ViewVersions`, `DeleteVersions`, `CancelCheckout`, `ManagePersonalViews`, `ManageLists`, `ViewFormPages`, `AnonymousSearchAccessList`, `Open`, `ViewPages`, `AddAndCustomizePages`, `ApplyThemeAndBorder`, `ApplyStyleSheets`, `ViewUsageData`, `CreateSSCSite`, `ManageSubwebs`, `CreateGroups`, `ManagePermissions`, `BrowseDirectories`, `BrowseUserInfo`, `AddDelPrivateWebParts`, `UpdatePersonalWebParts`, `ManageWeb`, `AnonymousSearchAccessWebLists`, `UseClientIntegration`, `UseRemoteAPIs`, `ManageAlerts`, `CreateAlerts`, `EditMyUserInfo`, `EnumeratePermissions`, `FullMask`. Defaults to `EmptyMask`.",
            ),
            (
                "scope",
                "Scope of the existing custom action. Allowed values `Site`, `Web`, `All`. Default `All`. Note, this would not update the scope, but might speed up the execution of the scope of the custom action is known.",
            ),
            (
                "scriptBlock",
                "Specifies a block of script to be executed. This attribute is only applicable when the Location attribute is set to ScriptLink.",
            ),
            (
                "scriptSrc",
                "Specifies a file that contains script to be executed. This attribute is only applicable when the Location attribute is set to ScriptLink.",
            ),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the custom action.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component Properties of the custom action. Specify values as a JSON string : `'{\"testMessage\":\"Test message\"}'`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treats quotes differently. For example, this is how ApplicationCustomizer user custom action can be created from the Windows cmd.exe:",
            "    m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "The `--rights` option accepts case-sensitive values.",
            "Note, specifying the scope option might speed up the execution of the command, but would not update the scope. If the scope has to be changed, then the existing custom action should be removed and new should be added with different scope.",
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Updates a tenant-wide SharePoint Framework Application Customizer extension properties in the specified site.",
                "m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --clientSideComponentProperties '{\"testMessage\":\"Test message\"}'",
            ),
            (
                "Updates a tenant-wide SharePoint Framework modern List View Command Set extension in the specified site.",
                "m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --clientSideComponentProperties '{\"sampleTextOne\":\"One item is selected in the list.\", \"sampleTextTwo\":\"This command is always visible.\"}' --sequence 100",
            ),
            (
                "Updates a url custom action in the SiteActions menu in the specified site.",
                "m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --actionUrl \"~site/SitePages/Home.aspx\"",
            ),
            (
                "Updates a ScriptLink custom action with script source in classic pages in the specified site collection.",
                "m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --scriptSrc \"~sitecollection/SiteAssets/YourScript.js\"",
            ),
            (
                "Creates a custom action with delegated rights in the SiteActions menu in the specified site.",
                "m365 spo customaction set --webUrl https://contoso.sharepoint.com/sites/test --id 058140e3-0e37-44fc-a1d3-79c487d371a3 --rights \"AddListItems,DeleteListItems,ManageLists\"",
            ),
        ],
        links: &[
            (
                "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
                "https://msdn.microsoft.com/en-us/library/office/dn531432.aspx#bk_UserCustomAction",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/bb802730.aspx",
                "https://msdn.microsoft.com/en-us/library/office/bb802730.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/ms460194.aspx",
                "https://msdn.microsoft.com/en-us/library/office/ms460194.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/office/microsoft.sharepoint.spbasepermissions.aspx",
                "https://msdn.microsoft.com/en-us/library/office/microsoft.sharepoint.spbasepermissions.aspx",
            ),
        ],
    },
    Help {
        name: "spo eventreceiver get",
        summary: "Retrieves specific event receiver for the specified web, site or list by event receiver name or id.",
        options: &[
            (
                "webUrl",
                "The URL of the web for which to retrieve the event receivers.",
            ),
            (
                "listTitle",
                "The title of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "The id of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "The url of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            ("name", "The name of the event receiver to retrieve."),
            ("id", "The id of the event receiver to retrieve."),
            (
                "scope",
                "The scope of which to retrieve the event receivers. Can be either `site` or `web`. Defaults to `web`. Only applicable when not specifying any of the list properties.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieve an event receiver with the given name in the specified site.",
                "m365 spo eventreceiver get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --name 'PnP Test Receiver'",
            ),
            (
                "Retrieve an event receiver with the given id in the specified site.",
                "m365 spo eventreceiver get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --scope site --id c5a6444a-9c7f-4a0d-9e29-fc6fe30e34ec",
            ),
            (
                "Retrieve an event receiver with the given name for a list with the given title in the specified site.",
                "m365 spo eventreceiver get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --name 'PnP Test Receiver'",
            ),
            (
                "Retrieve an event receiver with the given id for a list with the given ID in the specified site.",
                "m365 spo eventreceiver get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '202b8199-b9de-43fd-9737-7f213f51c991' --id c5a6444a-9c7f-4a0d-9e29-fc6fe30e34ec",
            ),
            (
                "Retrieve an event receiver with the given name for a list with the given url in the specified site.",
                "m365 spo eventreceiver get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/sites/contoso-sales/lists/Events' --name 'PnP Test Receiver'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo eventreceiver list",
        summary: "Retrieves event receivers for the specified web, site or list.",
        options: &[
            (
                "webUrl",
                "The URL of the web for which to retrieve the event receivers.",
            ),
            (
                "listTitle",
                "The title of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "The id of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "The url of the list for which to retrieve the event receivers, if the event receivers should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "scope",
                "The scope of which to retrieve the Event Receivers. Can be either \"site\" or \"web\". Defaults to \"web\". Only applicable when not specifying any of the list properties.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieves event receivers in the specified site.",
                "m365 spo eventreceiver list --webUrl https://contoso.sharepoint.com/sites/contoso-sales",
            ),
            (
                "Retrieves event receivers in the specified site.",
                "m365 spo eventreceiver list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --scope site",
            ),
            (
                "Retrieves event receivers for list with given title in the specified site.",
                "m365 spo eventreceiver list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events",
            ),
            (
                "Retrieves event receivers for list with given ID in the specified site.",
                "m365 spo eventreceiver list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '202b8199-b9de-43fd-9737-7f213f51c991'",
            ),
            (
                "Retrieves event receivers for list with given url in the specified site.",
                "m365 spo eventreceiver list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/sites/contoso-sales/lists/Events'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo eventreceiver remove",
        summary: "Removes event receivers for the specified web, site, or list.",
        options: &[
            ("webUrl", "The URL of the web."),
            (
                "listTitle",
                "The title of the list, if the event receiver should be removed from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "The id of the list, if the event receiver should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "The url of the list, if the event receiver should be retrieved from a list. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            ("name", "The name."),
            ("id", "The id."),
            (
                "scope",
                "The scope. Can be either \"site\" or \"web\". Defaults to \"web\". Only applicable when not specifying any of the list properties.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "You can only remove an event receiver if it has been registered by an app with the same ID.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove event receiver in a specific web by name.",
                "m365 spo eventreceiver remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --name 'PnP Test Receiver'",
            ),
            (
                "Remove event receiver in a specific site by id.",
                "m365 spo eventreceiver remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --scope site --id c5a6444a-9c7f-4a0d-9e29-fc6fe30e34ec",
            ),
            (
                "Remove event receiver in a specific list retrieved by title by name.",
                "m365 spo eventreceiver remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --name 'PnP Test Receiver'",
            ),
            (
                "Remove event receiver in a specific list retrieved by list id by id.",
                "m365 spo eventreceiver remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '202b8199-b9de-43fd-9737-7f213f51c991' --id c5a6444a-9c7f-4a0d-9e29-fc6fe30e34ec",
            ),
            (
                "Remove event receiver in a specific list retrieved by list url by name.",
                "m365 spo eventreceiver remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/sites/contoso-sales/lists/Events' --name 'PnP Test Receiver'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo externaluser list",
        summary: "Lists external users in the tenant",
        options: &[
            (
                "filter",
                "Limits the results to only those users whose first name, last name or email address begins with the text in the string, using a case-insensitive comparison.",
            ),
            (
                "pageSize",
                "Specifies the maximum number of users to be returned in the collection. The value must be less than or equal to `50`.",
            ),
            (
                "position",
                "Use to specify the zero-based index of the position in the sorted collection of the first result to be returned.",
            ),
            (
                "sortOrder",
                "Specifies the sort results in Ascending or Descending order on the `SPOUser.Email` property should occur. Allowed values `asc|desc`. Default `asc`.",
            ),
            (
                "siteUrl",
                "Specifies the site to retrieve external users for. If no site is specified, the external users for all sites are returned.",
            ),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all external users from the current tenant. Show the first batch of 50 users.",
                "m365 spo externaluser list --pageSize 50 --position 0",
            ),
            (
                "List all external users from the current tenant whose first name, last name or email address begins with `Vesa`. Show the first batch of 50 users.",
                "m365 spo externaluser list --pageSize 50 --position 0 --filter Vesa",
            ),
            (
                "List all external users from the specified site. Show the first batch of 50 users.",
                "m365 spo externaluser list --pageSize 50 --position 0 --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "List all external users from the current tenant. Show the first batch of 50 users sorted descending by e-mail.",
                "m365 spo externaluser list --pageSize 50 --position 0 --sortOrder desc",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo feature disable",
        summary: "Disables a feature for the specified site or web",
        options: &[
            (
                "webUrl",
                "The URL of the site or web for which to disable the feature.",
            ),
            ("id", "The ID of the feature to disable."),
            (
                "scope",
                "Scope of the Feature to disable. Allowed values: `Site`, `Web`. Defaults to `Web`.",
            ),
            (
                "force",
                "Specifies whether to continue if an error occurs when disabling the feature.",
            ),
        ],
        notes: &[
            "If the specified url doesn't refer to an existing site collection, you will get a `\"404 FILE NOT FOUND\"` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Disable site feature.",
                "m365 spo feature disable --webUrl https://contoso.sharepoint.com/sites/salis --id 915c240e-a6cc-49b8-8b2c-0bff8b553ed3 --scope Site",
            ),
            (
                "Disable web feature (with force to ignore errors).",
                "m365 spo feature disable --webUrl https://contoso.sharepoint.com/sites/salis --id 00bfea71-5932-4f9c-ad71-1557e5751100 --scope Web --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo feature enable",
        summary: "Enables feature for the specified site or web",
        options: &[
            (
                "webUrl",
                "The URL of the site or web for which to enable the feature.",
            ),
            ("id", "The ID of the feature to enable."),
            (
                "scope",
                "Scope of the Feature to enable. Allowed values: `Site`, `Web`. Defaults to `Web`.",
            ),
            (
                "force",
                "Specifies whether to overwrite an existing feature with the same feature identifier. This parameter is ignored if there are no errors.",
            ),
        ],
        notes: &[
            "If the specified url doesn't refer to an existing site collection, you will get a `\"404 FILE NOT FOUND\"` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Enable site feature.",
                "m365 spo feature enable --webUrl https://contoso.sharepoint.com/sites/sales --id 915c240e-a6cc-49b8-8b2c-0bff8b553ed3 --scope Site",
            ),
            (
                "Enable web feature (with force to overwrite feature with same id).",
                "m365 spo feature enable --webUrl https://contoso.sharepoint.com/sites/sales --id 00bfea71-5932-4f9c-ad71-1557e5751100 --scope Web --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo feature list",
        summary: "Lists Features activated in the specified site or site collection",
        options: &[
            (
                "webUrl",
                "URL of the site (collection) to retrieve the activated Features from",
            ),
            (
                "scope",
                "Scope of the Features to retrieve. Allowed values `Site,Web`. Default `Web`",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Return details about Features activated in the specified site collection",
                "m365 spo feature list --webUrl https://contoso.sharepoint.com/sites/test --scope Site",
            ),
            (
                "Return details about Features activated in the specified site",
                "m365 spo feature list --webUrl https://contoso.sharepoint.com/sites/test --scope Web",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo field add",
        summary: "Adds a new list or site column using the CAML field definition",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the field should be created.",
            ),
            (
                "listTitle",
                "Title of the list where the field should be created (if it should be created as a list column).",
            ),
            (
                "listId",
                "ID of the list where the field should be created (if it should be created as a list column).",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list where the field should be created (if it should be created as a list column).",
            ),
            ("xml", "CAML field definition."),
            (
                "options",
                "The options to use to add to the field. Allowed values: `DefaultValue`,`AddToDefaultContentType`, `AddToNoContentType`, `AddToAllContentTypes`, `AddFieldInternalNameHint`, `AddFieldToDefaultView`, `AddFieldCheckDisplayName`.",
            ),
        ],
        notes: &[
            "If the specified field already exists, you will get a A duplicate field name \"your-field\" was found. error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Create a date time site column.",
                "m365 spo field add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --xml '`<Field Type=\"DateTime\" DisplayName=\"Start date-time\" Required=\"FALSE\" EnforceUniqueValues=\"FALSE\" Indexed=\"FALSE\" Format=\"DateTime\" Group=\"PnP Columns\" FriendlyDisplayFormat=\"Disabled\" ID=\"{5ee2dd25-d941-455a-9bdb-7f2c54aed11b}\" SourceID=\"{4f118c69-66e0-497c-96ff-d7855ce0713d}\" StaticName=\"PnPAlertStartDateTime\" Name=\"PnPAlertStartDateTime\"><Default>[today]</Default></Field>`'",
            ),
            (
                "Create a URL list column to a list retrieved by Title.",
                "m365 spo field add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --xml '`<Field Type=\"URL\" DisplayName=\"More information link\" Required=\"FALSE\" EnforceUniqueValues=\"FALSE\" Indexed=\"FALSE\" Format=\"Hyperlink\" Group=\"PnP Columns\" ID=\"{6085e32a-339b-4da7-ab6d-c1e013e5ab27}\" SourceID=\"{4f118c69-66e0-497c-96ff-d7855ce0713d}\" StaticName=\"PnPAlertMoreInformation\" Name=\"PnPAlertMoreInformation\"></Field>`'",
            ),
            (
                "Create a URL list column and add it to all content types.",
                "m365 spo field add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --xml '`<Field Type=\"URL\" DisplayName=\"More information link\" Required=\"FALSE\" EnforceUniqueValues=\"FALSE\" Indexed=\"FALSE\" Format=\"Hyperlink\" Group=\"PnP Columns\" ID=\"{6085e32a-339b-4da7-ab6d-c1e013e5ab27}\" SourceID=\"{4f118c69-66e0-497c-96ff-d7855ce0713d}\" StaticName=\"PnPAlertMoreInformation\" Name=\"PnPAlertMoreInformation\"></Field>`' --options AddToAllContentTypes",
            ),
            (
                "Create a URL list column to a list retrieved by ID.",
                "m365 spo field add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId e785fe80-2ca1-409e-b9d1-19055a85cd38 --xml '`<Field Type=\"URL\" DisplayName=\"More information link\" Required=\"FALSE\" EnforceUniqueValues=\"FALSE\" Indexed=\"FALSE\" Format=\"Hyperlink\" Group=\"PnP Columns\" ID=\"{6085e32a-339b-4da7-ab6d-c1e013e5ab27}\" SourceID=\"{4f118c69-66e0-497c-96ff-d7855ce0713d}\" StaticName=\"PnPAlertMoreInformation\" Name=\"PnPAlertMoreInformation\"></Field>`'",
            ),
            (
                "Create a date time list column to a list retrieved by URL.",
                "m365 spo field add --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl /sites/contoso-sales/lists/Events --xml '`<Field Type=\"DateTime\" DisplayName=\"Start date-time\" Required=\"FALSE\" EnforceUniqueValues=\"FALSE\" Indexed=\"FALSE\" Format=\"DateTime\" Group=\"PnP Columns\" FriendlyDisplayFormat=\"Disabled\" ID=\"{5ee2dd25-d941-455a-9bdb-7f2c54aed11b}\" SourceID=\"{4f118c69-66e0-497c-96ff-d7855ce0713d}\" StaticName=\"PnPAlertStartDateTime\" Name=\"PnPAlertStartDateTime\"><Default>[today]</Default></Field>`'",
            ),
        ],
        links: &[(
            "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.addfieldoptions.aspx",
            "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.addfieldoptions.aspx",
        )],
    },
    Help {
        name: "spo field get",
        summary: "Retrieves information about the specified list- or site column",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the field is located.",
            ),
            (
                "listTitle",
                "Title of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "ID of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "Server- or web-relative URL of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            ("id", "The ID of the field to retrieve."),
            (
                "title",
                "The display name (case-sensitive) of the field to retrieve.",
            ),
            (
                "internalName",
                "The internal name (case-sensitive) of the field to retrieve.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieves site column by id located in the specified site.",
                "m365 spo field get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --id 5ee2dd25-d941-455a-9bdb-7f2c54aed11b",
            ),
            (
                "Retrieves list column by id located in the specified site. Retrieves the list by its title.",
                "m365 spo field get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --id 5ee2dd25-d941-455a-9bdb-7f2c54aed11b",
            ),
            (
                "Retrieves list column by display name located in the specified site. Retrieves the list by its url.",
                "m365 spo field get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl \"Lists/Events\" --title \"Title\"",
            ),
            (
                "Retrieves list column by internal name located in the specified site. Retrieves the list by its url.",
                "m365 spo field get --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl \"Lists/Events\" --internalName \"Title\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo field list",
        summary: "Retrieves columns for the specified list or site",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where fields are located.",
            ),
            ("listTitle", "Title of the list where fields are located."),
            ("listId", "ID of the list where fields are located."),
            (
                "listUrl",
                "Server- or web-relative URL of the list where fields are located.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieves site columns for the specified site.",
                "m365 spo field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales",
            ),
            (
                "Retrieves list columns for list Events in the specified site.",
                "m365 spo field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events",
            ),
            (
                "Retrieves list columns for the specified list in the specified site.",
                "m365 spo field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listId '202b8199-b9de-43fd-9737-7f213f51c991'",
            ),
            (
                "Retrieves list columns for the specified list in the specified site.",
                "m365 spo field list --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl '/sites/contoso-sales/lists/Events'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo field remove",
        summary: "Removes the specified list- or site column",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the field to remove is located.",
            ),
            (
                "listTitle",
                "Title of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listId",
                "ID of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            (
                "listUrl",
                "Server- or web-relative URL of the list where the field is located. Specify either `listTitle`, `listId` or `listUrl`.",
            ),
            ("id", "The ID of the field to remove."),
            (
                "title",
                "The display name (case-sensitive) of the field to remove.",
            ),
            (
                "internalName",
                "The internal name (case-sensitive) of the field to remove.",
            ),
            (
                "group",
                "Delete all fields from this group (case-sensitive).",
            ),
            ("force", "Don't prompt for confirming removing the field."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove the site column with the specified ID, located in the specified site.",
                "m365 spo field remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --id 5ee2dd25-d941-455a-9bdb-7f2c54aed11b",
            ),
            (
                "Remove the list column with the specified ID, located in the specified site. Retrieves the list by its title.",
                "m365 spo field remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listTitle Events --id 5ee2dd25-d941-455a-9bdb-7f2c54aed11b",
            ),
            (
                "Remove the list column with the specified display name, located in the specified site. Retrieves the list by its url.",
                "m365 spo field remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl \"Lists/Events\" --title \"Title\"",
            ),
            (
                "Remove the list column with the specified display name, located in the specified site. Retrieves the list by its url.",
                "m365 spo field remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --listUrl \"Lists/Events\" --internalName \"Title\"",
            ),
            (
                "Remove all site columns from the specified group.",
                "m365 spo field remove --webUrl https://contoso.sharepoint.com/sites/contoso-sales --group \"MyGroup\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo field set",
        summary: "Updates existing list or site column",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site where the field is located.",
            ),
            (
                "listId",
                "ID of the list where the field is located (if list column).",
            ),
            (
                "listTitle",
                "Title of the list where the field is located (if list column).",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list where the field is located (if list column).",
            ),
            ("id", "ID of the field to update."),
            ("title", "Title of the field to update."),
            ("internalName", "Internal name of the field to update."),
            (
                "updateExistingLists",
                "Set, to push the update to existing lists. Otherwise, the changes will apply to new lists only.",
            ),
        ],
        notes: &[
            "Specify properties to update using their names, eg. `--Title 'New Title' --JSLink jslink.js`.",
            "When updating column formatting for a field with the `--CustomFormatter` option, it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Update the title of the site column specified by its internal name and push changes to existing lists.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --internalName 'MyColumn' --updateExistingLists --Title 'My column'",
            ),
            (
                "Update the title of the site column specified by its title and push changes to existing lists.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --title 'MyColumn' --updateExistingLists --Title 'My column'",
            ),
            (
                "Update the title of the list column specified by its ID.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce --Title 'My column'",
            ),
            (
                "Update the description of a column specified by the ID on a list retrieved by the URL.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/Lists/My List' --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce --Description 'My column Description'",
            ),
            (
                "Update column formatting of the specified list column based on title.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --title 'MyColumn' --CustomFormatter '{\"schema\":\"https://developer.microsoft.com/json-schemas/sp/column-formatting.schema.json\", \"elmType\": \"div\", \"txtContent\": \"@currentField\"}'",
            ),
            (
                "Update column formatting of the specified list column based on internalName.",
                "m365 spo field set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --internalName 'MyColumn' --CustomFormatter '{\"schema\":\"https://developer.microsoft.com/json-schemas/sp/column-formatting.schema.json\", \"elmType\": \"div\", \"txtContent\": \"@currentField\"}'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file add",
        summary: "Uploads file to the specified folder",
        options: &[
            (
                "webUrl",
                "The URL of the site where the file should be uploaded to.",
            ),
            (
                "folder",
                "The server- or site-relative decoded URL to the folder where the file should be uploaded.",
            ),
            ("path", "Local path to the file to upload."),
            (
                "contentType",
                "Content type name or ID to assign to the file.",
            ),
            (
                "checkOut",
                "If versioning is enabled, this will check out the file first if it exists, upload the file, then check it in again.",
            ),
            (
                "overwrite",
                "Overwrite the file if it already exists in the target folder.",
            ),
            (
                "fileName",
                "The name of the file in SharePoint after upload. If not specified, the name of the uploaded file will be used.",
            ),
            (
                "checkInComment",
                "Comment to set when checking the file in.",
            ),
            ("approve", "Will automatically approve the uploaded file."),
            ("approveComment", "Comment to set when approving the file."),
            ("publish", "Will automatically publish the uploaded file."),
            ("publishComment", "Comment to set when publishing the file."),
        ],
        notes: &[
            "This command allows using unknown properties. Each property corresponds to the list item field that should be set when uploading the file.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Adds the specified file to the specified site in the specified folder.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg'",
            ),
            (
                "Adds the specified file to the specified site in the specified sub folder.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents/Sub Folder 1' --path 'C:\\MS365.jpg'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder specifying server-relative folder url.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder '/sites/project-x/Shared Documents' --path 'C:\\MS365.jpg'",
            ),
            (
                "Uploads a file to SharePoint with a different name.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --fileName 'MS365-New.jpg'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder with specified content type.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --contentType 'Picture'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder, but checks out existing file before the upload.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --checkOut --checkInComment 'check in comment x'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and approves it (when list moderation is enabled).",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --approve --approveComment 'approve comment x'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and publishes it.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --publish --publishComment 'publish comment x'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes single text field value of the list item.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --Title \"New Title\"",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes person/group field and DateTime field values.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --Editor \"[{'Key':'i:0#.f|membership|john.smith@contoso.com'}]\" --Modified '6/23/2018 10:15 PM'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes hyperlink or picture field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --URL 'https://contoso.com, Contoso'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes taxonomy field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --Topic \"HR services|c17baaeb-67cd-4378-9389-9d97a945c701\"",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes taxonomy multi-value field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --Topic \"HR services|c17baaeb-67cd-4378-9389-9d97a945c701;Inclusion ＆ Diversity|66a67671-ed89-44a7-9be4-e80c06b41f35\"",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes choice field and multi-choice field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --ChoiceField1 'Option3' --MultiChoiceField1 'Option2;#Option3'",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes person/group field that allows multi-user selection.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --AllowedUsers \"[{'Key':'i:0#.f|membership|john.smith@contoso.com'},{'Key':'i:0#.f|membership|velin.georgiev@contoso.com'}]\"",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes yes/no field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --HasCar true",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes number field and currency field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --NumberField 100 --CurrencyField 20",
            ),
            (
                "Adds the specified file to the specified site in the specified folder and changes lookup field and multi-lookup field.",
                "m365 spo file add --webUrl https://contoso.sharepoint.com/sites/project-x --folder 'Shared Documents' --path 'C:\\MS365.jpg' --LookupField 1 --MultiLookupField \"2;#;#3;#;#4;#\"",
            ),
        ],
        links: &[
            (
                "https://robertschouten.com/2018/04/30/update-file-metadata-with-rest-api-using-validateupdatelistitem-method/",
                "https://robertschouten.com/2018/04/30/update-file-metadata-with-rest-api-using-validateupdatelistitem-method/",
            ),
            (
                "https://www.linkedin.com/pulse/list-items-system-update-options-sharepoint-online-andrew-koltyakov/",
                "https://www.linkedin.com/pulse/list-items-system-update-options-sharepoint-online-andrew-koltyakov/",
            ),
        ],
    },
    Help {
        name: "spo file archive",
        summary: "Archives a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to archive.",
            ),
            ("id", "The unique ID (GUID) of the file to archive."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "You will be able to reactivate an archived file instantly for the first 7 days. After that, it will take up to 24 hours to reactivate.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Archive a file by id without prompting for confirmation",
                "m365 spo file archive --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7a8c9207-7745-4cda-b0e2-be2618ee3030 --force",
            ),
            (
                "Archive a file by URL with prompting for confirmation",
                "m365 spo file archive --webUrl https://contoso.sharepoint.com/sites/Marketing --url '/sites/Marketing/shared documents/document.docx'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file checkin",
        summary: "Checks in specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("id", "The unique ID (GUID) of the file to retrieve."),
            (
                "type",
                "Type of the check in. Allowed values: `Minor`, `Major`, `Overwrite`. Defaults to `Major`.",
            ),
            (
                "comment",
                "Comment to set when checking the file in. It\\'s length must be less than 1024 letters. Default is empty string.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Checks in file with the specified unique ID located in the specified site.",
                "m365 spo file checkin --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Checks in file with the specified server-relative url located in the specified site.",
                "m365 spo file checkin --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx'",
            ),
            (
                "Checks in minor version of file with the specified server-relative url located in the specified site.",
                "m365 spo file checkin --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx' --type Minor",
            ),
            (
                "Checks in the specified file with comment located in the specified site.",
                "m365 spo file checkin --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx' --comment 'approved'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file checkout",
        summary: "Checks out specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("id", "The unique ID (GUID) of the file to retrieve."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Checks out file with the specified unique ID located in the specified site.",
                "m365 spo file checkout --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Checks out file with the specified server-relative url located in the specified site.",
                "m365 spo file checkout --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file checkout undo",
        summary: "Discards a checked out file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("fileId", "The unique ID (GUID) of the file to retrieve."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Discards a checked-out file with a specific ID.",
                "m365 spo file checkout undo --webUrl https://contoso.sharepoint.com/sites/project-x --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Discards a checked-out file with a specific URL without prompting for confirmation.",
                "m365 spo file checkout undo --webUrl https://contoso.sharepoint.com/sites/project-x --fileUrl '/sites/project-x/documents/Test1.docx' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file copy",
        summary: "Copies a file to another location",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "sourceUrl",
                "Site-relative, server-relative or absolute decoded URL of the file.",
            ),
            ("sourceId", "The Unique ID (GUID) of the file."),
            (
                "targetUrl",
                "Server-relative or absolute decoded URL of the location.",
            ),
            ("newName", "New name of the destination file."),
            (
                "nameConflictBehavior",
                "Behavior when a document with the same name is already present at the destination. Allowed values: `fail`, `replace`, `rename`. Defaults to `fail`.",
            ),
            (
                "bypassSharedLock",
                "This indicates whether a file with a share lock can still be copied. Use this option to copy a file that is locked.",
            ),
            (
                "ignoreVersionHistory",
                "Only copy the most recent version of the file and ignore the version history.",
            ),
            ("skipWait", "Don't wait for the copy operation to complete."),
        ],
        notes: &[
            "When you specify a value for `nameConflictBehavior`, consider the following:",
            "- `fail` will throw an error when the destination file already exists.",
            "- `replace` will replace the destination file if it already exists.",
            "- `rename` will add a suffix (e.g. Document1.pdf) when the destination file already exists.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Copy a file by server-relative URL to a document library in another site collection with server relative URL.",
                "m365 spo file copy --webUrl https://contoso.sharepoint.com/sites/project --sourceUrl \"/sites/project/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\"",
            ),
            (
                "Copy a file by site-relative URL to a document library in another site collection and ignore the file history.",
                "m365 spo file copy --webUrl https://contoso.sharepoint.com/sites/project --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\" --ignoreVersionHistory",
            ),
            (
                "Copy file by sourceId (unique ID) to a document library in another site collection and rename the file (without specifying the file extension).",
                "m365 spo file copy --webUrl https://contoso.sharepoint.com/sites/project --sourceId \"b2307a39-e878-458b-bc90-03bc578531d6\" --targetUrl \"/sites/IT/Shared Documents\" --newName \"Report.pdf\"",
            ),
            (
                "Copy file by sourceId (unique ID) to a document library in another site collection with a new name, rename the file if it already exists.",
                "m365 spo file copy --webUrl https://contoso.sharepoint.com/sites/project --sourceId \"b2307a39-e878-458b-bc90-03bc578531d6\" --targetUrl \"/sites/IT/Shared Documents\" --newName \"Report.pdf\" --nameConflictBehavior rename",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file get",
        summary: "Gets information about the specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("id", "The unique ID (GUID) of the file to retrieve."),
            (
                "asString",
                "Set to retrieve the contents of the specified file as string.",
            ),
            ("asListItem", "Set to retrieve the underlying list item."),
            (
                "asFile",
                "Set to save the file to the path specified in the path option.",
            ),
            (
                "path",
                "The local path where to save the retrieved file. Must be specified when the `--asFile` option is used.",
            ),
            (
                "withPermissions",
                "Set if you want to return associated roles and permissions.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get file properties for a file with id (unique ID) parameter located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Get contents of the file with id (unique ID) parameter located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6' --asString",
            ),
            (
                "Get list item properties for a file with id (unique ID) parameter located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6' --asListItem",
            ),
            (
                "Saves the file with id (unique ID) parameter located in a site to a local file.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6' --asFile --path /Users/user/documents/SavedAsTest1.docx",
            ),
            (
                "Return file properties for a file with server-relative url located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx'",
            ),
            (
                "Returns a file as string for a file with server-relative url located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx' --asString",
            ),
            (
                "Returns the list item properties for a file with the server-relative url located in a site.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx' --asListItem",
            ),
            (
                "Saves a file with the server-relative url located in a site to a local file.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/documents/Test1.docx' --asFile --path /Users/user/documents/SavedAsTest1.docx",
            ),
            (
                "Gets the file properties for a file with id (unique ID) parameter located in a site with permissions.",
                "m365 spo file get --webUrl https://contoso.sharepoint.com/sites/project-x --id 'b2307a39-e878-458b-bc90-03bc578531d6' --withPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file list",
        summary: "Gets all files within the specified folder and site",
        options: &[
            (
                "webUrl",
                "The URL of the site where the folder from which to retrieve files is located.",
            ),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the parent folder from which to retrieve files.",
            ),
            (
                "fields",
                "Comma-separated list of fields to retrieve. Will retrieve all fields if not specified.",
            ),
            (
                "filter",
                "OData filter to use to query the list of items with.",
            ),
            ("recursive", "Set to retrieve files from subfolders."),
        ],
        notes: &[
            "When the `fields` option includes values with a `/`, for example: `ListItemAllFields/Id`, an additional `$expand` query parameter will be included on `ListItemAllFields`.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return all files from a folder.",
                "m365 spo file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents'",
            ),
            (
                "Return all files from a folder and all the sub-folders.",
                "m365 spo file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents' --recursive",
            ),
            (
                "Return the files from a folder with specific fields which will be expanded.",
                "m365 spo file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents' --fields \"Title,Length\"",
            ),
            (
                "Return the files from a folder that meet the criteria of the filter with specific fields which will be expanded.",
                "m365 spo file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents' --fields ListItemAllFields/Id --filter \"Name eq 'document.docx'\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file move",
        summary: "Moves a file to another location",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "sourceUrl",
                "The server-, site-relative or absolute decoded URL of the file to move.",
            ),
            ("sourceId", "The ID of the file to move."),
            (
                "targetUrl",
                "Server-relative or absolute decoded URL where to move the file.",
            ),
            ("newName", "The new name of the destination file."),
            (
                "nameConflictBehavior",
                "Behavior when a file or folder with the same name is already present at the destination. Allowed values: `fail`, `replace`, `rename`. Defaults to `fail`.",
            ),
            (
                "withItemPermissions",
                "Ensure that item-level permissions are preserved during the move.",
            ),
            (
                "bypassSharedLock",
                "This indicates whether a file with a share lock can still be moved. Use this option to move a file that is locked.",
            ),
            ("skipWait", "Don't wait for the move operation to complete."),
        ],
        notes: &[
            "When you specify a value for `nameConflictBehavior`, consider the following:",
            "- `fail` will throw an error when the destination file already exists.",
            "- `replace` will replace the destination file if it already exists.",
            "- `rename` will add a suffix (e.g. Document1.pdf) when the destination file already exists.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Move a file to a document library in another site collection by server relative URL.",
                "m365 spo file move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/sites/project-x/Shared Documents/sp1.pdf\" --targetUrl \"/sites/project-y/Archived documents\"",
            ),
            (
                "Move a file specified by a server-relative URL to a document library and rename it.",
                "m365 spo file move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/Shared Documents/sp1.pdf\" --targetUrl \"/sites/project-x/My Documents\" --newName Report.pdf",
            ),
            (
                "Move a file to another document library and replace a file with the same name.",
                "m365 spo file move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/Shared Documents/Report project-x.pdf\" --targetUrl \"/sites/project-x/My Documents\" --newName Report.pdf --nameConflictBehavior replace",
            ),
            (
                "Move a file referenced by its ID to another document library and retain item-level permissions.",
                "m365 spo file move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceId b8cc341b-9c11-4f2d-aa2b-0ce9c18bcba2 --targetUrl \"/sites/project-x/My Documents\" --withItemPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file remove",
        summary: "Removes the specified file",
        options: &[
            (
                "webUrl",
                "URL of the site where the file to remove is located.",
            ),
            ("id", "The ID of the file to remove."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to remove.",
            ),
            (
                "permanent",
                "Permanently delete the file instead of moving it to the recycle bin.",
            ),
            (
                "bypassSharedLock",
                "Remove the file even if it is locked for shared use.",
            ),
            ("force", "Don't prompt for confirming removing the file."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a file by ID.",
                "m365 spo file remove --webUrl https://contoso.sharepoint.com/sites/project-x --id 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Remove a file by site-relative URL.",
                "m365 spo file remove --webUrl https://contoso.sharepoint.com/sites/project-x --url \"/Shared Documents/Test.docx\"",
            ),
            (
                "Permanently remove a file by server-relative URL.",
                "m365 spo file remove --webUrl https://contoso.sharepoint.com/sites/project-x --url \"/sites/project-x/Shared Documents/Test.docx\" --permanent",
            ),
            (
                "Remove a file that is locked",
                "m365 spo file remove --webUrl https://contoso.sharepoint.com/sites/project-x --url \"/Shared Documents/Test.docx\" --bypassSharedLock",
            ),
            (
                "Remove a page template by site-relative URL.",
                "m365 spo page template remove --webUrl https://contoso.sharepoint.com/sites/project-x --url \"/Shared Documents/Forms/Template.dotx\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file rename",
        summary: "Renames a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "sourceUrl",
                "The server- or site-relative decoded URL of the file to rename.",
            ),
            ("targetFileName", "New file name of the file."),
            (
                "force",
                "If a file already exists with target file name, it will be moved to the recycle bin. If omitted, the rename operation will be canceled if a file already exists with the specified file name.",
            ),
        ],
        notes: &[
            "If you try to rename a file without the `--force` flag and a file with this name already exists, the operation will be cancelled.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Renames a file.",
                "m365 spo file rename --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl '/Shared Documents/Test1.docx' --targetFileName 'Test2.docx'",
            ),
            (
                "Renames a file. If the file with the target file name already exists, this file will be moved to the recycle bin.",
                "m365 spo file rename --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl '/Shared Documents/Test1.docx' --targetFileName 'Test2.docx' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file retentionlabel ensure",
        summary: "Applies a retention label to a file",
        options: &[
            (
                "webUrl",
                "URL of the site where the retention label from a file to apply is located.",
            ),
            (
                "fileUrl",
                "The site- or server-relative decoded URL of the file that should be labelled.",
            ),
            (
                "fileId",
                "The unique ID (GUID) of the file that should be labelled.",
            ),
            ("name", "Name of the retention label to apply to the file."),
            (
                "assetId",
                "A Compliance Asset Id to set on the item when it's labeled. See below for more information.",
            ),
        ],
        notes: &[
            "You can also use spo listitem retentionlabel remove for removing the retention label from a list item.",
            "The `--assetId` option has to do with event-based retention. Event-based retention is about starting a retention period when a specific event occurs, instead of the moment a document was labeled or created.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Applies a retention label to a file based on the label name and the fileUrl.",
                "m365 spo file retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --fileUrl '/Shared Documents/Document.docx' --name 'Some label'",
            ),
            (
                "Applies a retention label to a file based on the label name and the fileId.",
                "m365 spo file retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --fileId '26541f96-017c-4189-a604-599e083533b8' --name 'Some label'",
            ),
            (
                "Applies a event-based retention label to a file and updates the Asset Id field.",
                "m365 spo file retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --fileId '26541f96-017c-4189-a604-599e083533b8' --name 'Some label' --assetId 'XYZ'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file retentionlabel remove",
        summary: "Clears the retention label from a file",
        options: &[
            (
                "webUrl",
                "URL of the site where the retention label from a file to remove is located.",
            ),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file of which the label should be removed.",
            ),
            (
                "fileId",
                "The unique ID (GUID) of the file of which the label should be removed.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the retention label from a file.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes the retention label from a file in a given site based on the file id.",
                "m365 spo file retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --fileId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Removes the retention label from a file in a given site based on the file url.",
                "m365 spo file retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --fileUrl /sites/project-x/Shared Documents/Document.docx",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file roleassignment add",
        summary: "Adds a role assignment to the specified file.",
        options: &[
            ("webUrl", "URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("fileId", "The unique ID (GUID) of the file to retrieve."),
            (
                "principalId",
                "The SharePoint Id of the principal. It may be either a user id or group id to add a role assignment for.",
            ),
            ("upn", "The upn/email of user to assign role to."),
            (
                "groupName",
                "The group name of the SharePoint group Specify either `upn`, `groupName`, `entraGroupId`, `entraGroupName`, or `principalId` but not multiple. This option is only used for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to add."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to add.",
            ),
            ("roleDefinitionId", "ID of role definition."),
            (
                "roleDefinitionName",
                "The name of a role definition, like 'Contribute', 'Read', etc.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Adds a role assignment to a file with a specified id. It will use a principal id and a specific role definition id.",
                "m365 spo file roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Adds a role assignment to a file with a specified site-relative URL for a specific upn and a role definition name.",
                "m365 spo file roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"Shared Documents/Test1.docx\" --upn \"testuser@tenant.onmicrosoft.com\" --roleDefinitionName \"Full Control\"",
            ),
            (
                "Adds a role assignment to a file with a specified server-relative URL the for a specific SharePoint Online group and a role definition name.",
                "m365 spo file roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"/sites/project-x/documents/Test1.docx\" --groupName \"Sales Group\" --roleDefinitionName \"Read\"",
            ),
            (
                "Adds a role assignment to a file with a specified site-relative URL for a specific Entra Group Id and a role definition name.",
                "m365 spo file roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"Shared Documents/Test1.docx\" --entraGroupId 3c97e01e-978d-417b-a196-b6a3e37fa873 --roleDefinitionName \"Full Control\"",
            ),
            (
                "Adds a role assignment to a file with a specified site-relative URL for a specific Entra Group Name and a role definition name.",
                "m365 spo file roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"Shared Documents/Test1.docx\" --entraGroupName \"Marketing Memebers\" --roleDefinitionName \"Full Control\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file roleassignment remove",
        summary: "Removes a role assignment from a file.",
        options: &[
            ("webUrl", "URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            (
                "principalId",
                "The SharePoint Id of the principal. It may be either a user id or group id.",
            ),
            ("upn", "The upn/email of the user."),
            (
                "groupName",
                "The group name of the SharePoint group Specify either `upn`, `groupName`, `entraGroupId`, `entraGroupName`, or `principalId` but not multiple. This option is only used for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to remove."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to remove.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove a role assignment by principal id from a file by id.",
                "m365 spo file roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --principalId 2",
            ),
            (
                "Remove a role assignment by upn from a file by url.",
                "m365 spo file roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --fileUrl \"/sites/contoso-sales/documents/Test1.docx\" --upn \"user1@contoso.onmicrosoft.com\"",
            ),
            (
                "Remove a role assignment by group name from a file by id.",
                "m365 spo file roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --groupName \"saleGroup\"",
            ),
            (
                "Remove a role assignment by entra group id from a file by id without prompt",
                "m365 spo file roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --entraGroupId 3c97e01e-978d-417b-a196-b6a3e37fa873 --force",
            ),
            (
                "Remove a role assignment by entra group name from a file by id.",
                "m365 spo file roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --entraGroupName \"Marketing Members\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file roleinheritance break",
        summary: "Breaks inheritance of a file. Keeping existing permissions is the default behavior.",
        options: &[
            ("webUrl", "URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            (
                "clearExistingPermissions",
                "Clear all existing permissions from the file.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Break the inheritance of a file with a specific id (unique ID).",
                "m365 spo file roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
            (
                "Break the inheritance of a file with a specific id (unique ID) and clear all existing permissions.",
                "m365 spo file roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\" --clearExistingPermissions",
            ),
            (
                "Break the inheritance of a file with a specific server-relative URL.",
                "m365 spo file roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"/sites/project-x/documents/Test1.docx\"",
            ),
            (
                "Break the inheritance of a file with a specific server-relative URL and clear all existing permissions.",
                "m365 spo file roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"/sites/project-x/documents/Test1.docx\" --clearExistingPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file roleinheritance reset",
        summary: "Restores the role inheritance of a file",
        options: &[
            ("webUrl", "URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("fileId", "The unique ID (GUID) of the file to retrieve."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Reset inheritance of file with the specified id (unique ID) located in the specified site.",
                "m365 spo file roleinheritance reset --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
            (
                "Reset inheritance of file with the specified server-relative url located in the specified site. It will not prompt for confirmation before resetting.",
                "m365 spo file roleinheritance reset --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --fileUrl \"/sites/project-x/documents/Test1.docx\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinginfo get",
        summary: "Generates a sharing information report for the specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file for which to build the report.",
            ),
            (
                "fileId",
                "The unique ID (GUID) of the file for which to build the report.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get file sharing information report for the file with the specified server-relative url located in the specified site.",
                "m365 spo file sharinginfo get --webUrl https://contoso.sharepoint.com/sites/project-x --fileUrl \"/sites/M365CLI/Shared Documents/SharedFile.docx\"",
            ),
            (
                "Get file sharing information report for file with the specified id (unique ID) located in the specified site.",
                "m365 spo file sharinginfo get --webUrl https://contoso.sharepoint.com/sites/project-x --fileId \"b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink add",
        summary: "Creates a new sharing link for a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            (
                "type",
                "The type of sharing link to create. Either `view` or `edit`.",
            ),
            (
                "expirationDateTime",
                "The date and time to set the expiration. This should be defined as a valid ISO 8601 string.",
            ),
            (
                "scope",
                "The scope of link to create. Either `anonymous` or `organization`. If not specified, the default of the organization will be used.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.ReadWrite"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.ReadWrite.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Creates a sharing link of a specific type for a file by id.",
                "m365 spo file sharinglink add --webUrl https://contoso.sharepoint.com --fileId daebb04b-a773-4baa-b1d1-3625418e3234 --type view",
            ),
            (
                "Creates a sharing link of a specific type for a file by url.",
                "m365 spo file sharinglink add --webUrl https://contoso.sharepoint.com --fileUrl \"/sites/demo/Shared Documents/Test1.docx\" --type edit",
            ),
            (
                "Creates a sharing link of a file by url with type, scope and expirationDateTime parameter.",
                "m365 spo file sharinglink add --webUrl https://contoso.sharepoint.com --fileUrl \"/sites/demo/Shared Documents/Test1.docx\" --type edit --scope anonymous --expirationDateTime \"2023-01-09T16:20:00Z\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink clear",
        summary: "Removes sharing links of a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            (
                "scope",
                "Scope of the sharing link. Possible options are: `anonymous`, `users` or `organization`. If not specified, all links will be removed.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.ReadWrite"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.ReadWrite.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Removes all sharing links from a file specified by id without prompting for confirmation.",
                "m365 spo file sharinglink clear --webUrl https://contoso.sharepoint.com/sites/demo --fileId daebb04b-a773-4baa-b1d1-3625418e3234 --force",
            ),
            (
                "Removes sharing links of type anonymous from a file specified by url with prompting for confirmation.",
                "m365 spo file sharinglink clear --webUrl https://contoso.sharepoint.com/sites/demo --fileUrl '/sites/demo/Shared Documents/document.docx' --scope anonymous",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink get",
        summary: "Gets details about a specific sharing link of a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            ("id", "The ID of the sharing link."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.Read"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.Read.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets a specific sharing link of a file by id.",
                "m365 spo file sharinglink get --webUrl 'https://contoso.sharepoint.com/sites/demo' --fileId daebb04b-a773-4baa-b1d1-3625418e3234 --id 1ba739c5-e693-4c16-9dfa-042e4ec62972",
            ),
            (
                "Gets a specific sharing link of a file by a specified site-relative URL.",
                "m365 spo file sharinglink get --webUrl 'https://contoso.sharepoint.com/sites/demo' --fileUrl 'Shared Documents/document.docx' --id 1ba739c5-e693-4c16-9dfa-042e4ec62972",
            ),
            (
                "Gets a specific sharing link of a file by a specified server-relative URL.",
                "m365 spo file sharinglink get --webUrl 'https://contoso.sharepoint.com/sites/demo' --fileUrl '/sites/demo/Shared Documents/document.docx' --id 1ba739c5-e693-4c16-9dfa-042e4ec62972",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink list",
        summary: "Lists all the sharing links of a specific file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            (
                "scope",
                "Filter the results to only sharing links of a given scope: `anonymous`, `users` or `organization`. By default all sharing links are listed.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.Read"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.Read.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List sharing links of a file by id.",
                "m365 spo file sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --fileId daebb04b-a773-4baa-b1d1-3625418e3234",
            ),
            (
                "List sharing links of a file by url.",
                "m365 spo file sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --fileUrl \"/sites/demo/shared documents/document.docx\"",
            ),
            (
                "List anonymous sharing links of a file by url.",
                "m365 spo file sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --fileUrl \"/sites/demo/shared documents/document.docx\" --scope anonymous",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink remove",
        summary: "Removes a specific sharing link of a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            ("id", "The ID of the sharing link."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.ReadWrite"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.ReadWrite.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Removes a specific sharing link from a file by id without prompting for confirmation.",
                "m365 spo file sharinglink remove --webUrl https://contoso.sharepoint.com/sites/demo --fileId daebb04b-a773-4baa-b1d1-3625418e3234 --id c391b57d-5783-4c53-9236-cefb5c6ef323 --force",
            ),
            (
                "Removes a specific sharing link from a file by a specified site-relative URL.",
                "m365 spo file sharinglink remove --webUrl https://contoso.sharepoint.com/sites/demo --fileUrl 'Shared Documents/document.docx' --id c391b57d-5783-4c53-9236-cefb5c6ef323",
            ),
            (
                "Removes a specific sharing link from a file by a specified server-relative URL.",
                "m365 spo file sharinglink remove --webUrl https://contoso.sharepoint.com/sites/demo --fileUrl '/sites/demo/Shared Documents/document.docx' --id c391b57d-5783-4c53-9236-cefb5c6ef323",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file sharinglink set",
        summary: "Updates a sharing link of a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            ("id", "The ID of the sharing link."),
            (
                "expirationDateTime",
                "The date and time to set the expiration. This should be defined as a valid ISO 8601 string. This options only works for anonymous links.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.ReadWrite"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "Microsoft Graph", "Files.ReadWrite.All"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Updates an anonymous sharing link from a file by a specified site-relative URL with the expirationDateTime parameter.",
                "m365 spo file sharinglink set --webUrl https://contoso.sharepoint.com --fileUrl \"/sites/demo/Shared Documents/Document.docx\" --id 7c9f97c9-1bda-433c-9364-bb83e81771ee --expirationDateTime \"2023-01-09T16:57:00.000Z\"",
            ),
            (
                "Updates an anonymous sharing link from a file by id with the expirationDateTime parameter.",
                "m365 spo file sharinglink set --webUrl https://contoso.sharepoint.com --fileId daebb04b-a773-4baa-b1d1-3625418e3234 --id 7c9f97c9-1bda-433c-9364-bb83e81771ee --expirationDateTime \"2023-01-09T16:57:00.000Z\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file unarchive",
        summary: "Unarchives a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the file to unarchive.",
            ),
            ("id", "The unique ID (GUID) of the file to unarchive."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "Reactivation could take up to 24 hours. Files that are reactivated cannot be archived again for 120 days.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Unarchive a file by id without prompting for confirmation",
                "m365 spo file unarchive --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7a8c9207-7745-4cda-b0e2-be2618ee3030 --force",
            ),
            (
                "Unarchive a file by URL with prompting for confirmation",
                "m365 spo file unarchive --webUrl https://contoso.sharepoint.com/sites/Marketing --url '/sites/Marketing/shared documents/document.docx'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version clear",
        summary: "Removes all version history of a specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes all file version history in a specific site based on fileId and prompts for confirmation.",
                "m365 spo file version clear --webUrl https://contoso.sharepoint.com --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Removes all file version history in a specific site based on fileUrl and prompts for confirmation.",
                "m365 spo file version clear --webUrl https://contoso.sharepoint.com --fileUrl '/Shared Documents/Document.docx'",
            ),
            (
                "Removes all file version history in a specific site based on fileId without prompting for confirmation.",
                "m365 spo file version clear --webUrl https://contoso.sharepoint.com --fileId 'b2307a39-e878-458b-bc90-03bc578531d6' --force",
            ),
            (
                "Removes all file version history in a specific site based on fileUrl without prompting for confirmation.",
                "m365 spo file version clear --webUrl https://contoso.sharepoint.com --fileUrl '/Shared Documents/Document.docx' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version get",
        summary: "Gets information about a specific version of a specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("label", "Label of version which will be retrieved."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("fileId", "The unique ID (GUID) of the file to retrieve."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get file version in a specific site based on fileId.",
                "m365 spo file version get --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Get file in a specific site based on fileUrl.",
                "m365 spo file version get --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileUrl '/Shared Documents/Document.docx'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version keep",
        summary: "Ensures that a specific file version will never expire",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("fileUrl", "The server- or site-relative decoded URL."),
            ("fileId", "The unique ID (GUID) of the file."),
            ("label", "Label of the version."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Mark a file version as never expiring by file URL.",
                "m365 spo file version keep --webUrl \"https://contoso.sharepoint.com/sites/marketing\" --fileUrl \"/sites/marketing/Documents/report.docx\" --label \"6.0\"",
            ),
            (
                "Mark a file version as never expiring by file ID.",
                "m365 spo file version keep --webUrl \"https://contoso.sharepoint.com/sites/marketing\" --fileId \"12345678-90ab-cdef-1234-567890abcdef\" --label \"6.0\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version list",
        summary: "Retrieves all versions of a file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file.",
            ),
            ("fileId", "The unique ID (GUID) of the file."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List file versions of a specific file based on the ID of the file.",
                "m365 spo file version list --webUrl https://contoso.sharepoint.com --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "List file versions of a specific file based on the site-relative URL of the file.",
                "m365 spo file version list --webUrl https://contoso.sharepoint.com --fileUrl '/Shared Documents/Document.docx'",
            ),
            (
                "List file versions of a specific file based on the server-relative URL of the file.",
                "m365 spo file version list --webUrl https://contoso.sharepoint.com/sites/project-x --fileUrl '/sites/project-x/Shared Documents/Document.docx'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version remove",
        summary: "Removes a specific version of a specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("label", "Label of version which will be removed."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file to retrieve.",
            ),
            ("fileId", "The unique ID (GUID) of the file to retrieve."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a file version in a specific site based on fileId and prompts for confirmation.",
                "m365 spo file version remove --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Removes a file version in a specific site based on fileUrl and prompts for confirmation.",
                "m365 spo file version remove --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileUrl '/Shared Documents/Document.docx'",
            ),
            (
                "Removes a file version in a specific site based on fileId without prompting for confirmation.",
                "m365 spo file version remove --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileId 'b2307a39-e878-458b-bc90-03bc578531d6' --force",
            ),
            (
                "Removes a file version in a specific site based on fileUrl without prompting for confirmation.",
                "m365 spo file version remove --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileUrl '/Shared Documents/Document.docx' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo file version restore",
        summary: "Restores a specific version of a specified file",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("label", "Label of version which will be restored."),
            (
                "fileUrl",
                "The server- or site-relative decoded URL of the file whose version will be restored.",
            ),
            (
                "fileId",
                "The unique ID (GUID) of the file whose version will be restored.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Restores a file version in a specific site based on fileId and prompts for confirmation.",
                "m365 spo file version restore --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileId 'b2307a39-e878-458b-bc90-03bc578531d6'",
            ),
            (
                "Restores a file version in a specific site based on fileUrl and prompts for confirmation.",
                "m365 spo file version restore --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileUrl '/Shared Documents/Document.docx'",
            ),
            (
                "Restores a file version in a specific site based on fileId without prompting for confirmation.",
                "m365 spo file version restore --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileId 'b2307a39-e878-458b-bc90-03bc578531d6' --force",
            ),
            (
                "Restores a file version in a specific site based on fileUrl without prompting for confirmation.",
                "m365 spo file version restore --webUrl https://contoso.sharepoint.com --label \"1.0\" --fileUrl '/Shared Documents/Document.docx' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder add",
        summary: "Creates a folder within a parent folder",
        options: &[
            (
                "webUrl",
                "The URL of the site where the folder will be created.",
            ),
            (
                "parentFolderUrl",
                "The server- or site-relative decoded URL of the parent folder.",
            ),
            ("name", "Name of the new folder to be created."),
            (
                "color",
                "Visual color of the folder. Valid values are a color name or a number. Check remarks for more info.",
            ),
            (
                "ensureParentFolders",
                "Ensure that the parent folder path is available. Any missing folders will be created recursively.",
            ),
        ],
        notes: &[
            "When you specify a value for `color`, consider the following:",
            "Color | Number value | String value",
            "Yellow | 0 | yellow",
            "Dark red | 1 | darkRed",
            "Dark orange | 2 | darkOrange",
            "Dark green | 3 | darkGreen",
            "Dark teal | 4 | darkTeal",
            "Dark blue | 5 | darkBlue",
            "Dark purple | 6 | darkPurple",
            "Dark pink | 7 | darkPink",
            "Grey | 8 | grey",
            "Light red | 9 | lightRed",
            "Light orange | 10 | lightOrange",
            "Light green | 11 | lightGreen",
            "Light teal | 12 | lightTeal",
            "Light blue | 13 | lightBlue",
            "Light purple | 14 | lightPurple",
            "Light pink | 15 | lightPink",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Creates folder in a specific library within the site.",
                "m365 spo folder add --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/Shared Documents' --name 'My Folder Name'",
            ),
            (
                "Creates folder in a specific folder within the site.",
                "m365 spo folder add --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/sites/project-x/Shared Documents/Reports' --name 'Financial reports'",
            ),
            (
                "Create a folder with a dark blue look",
                "m365 spo folder add --webUrl https://contoso.sharepoint.com --parentFolderUrl '/ProjectFiles' --name 'Project-x' --color 5",
            ),
            (
                "Create a folder with a light teal look",
                "m365 spo folder add --webUrl https://contoso.sharepoint.com --parentFolderUrl '/ProjectFiles' --name 'Project-x' --color lightTeal",
            ),
            (
                "Create a folder and make sure that any missing folders specified as parent folders are created.",
                "m365 spo folder add --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/Projects/2024/Q1/Reports' --name Financial --ensureParentFolders",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder archive",
        summary: "Archives a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the folder to archive.",
            ),
            ("id", "The unique ID (GUID) of the folder to archive."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "You will be able to reactivate an archived folder instantly for the first 7 days. After that, it will take up to 24 hours to reactivate.",
            "Folder archival can take some time to complete, as the status might not be reflected immediately.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Archive a folder by id without prompting for confirmation.",
                "m365 spo folder archive --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7a8c9207-7745-4cda-b0e2-be2618ee3030 --force",
            ),
            (
                "Archive a folder by URL with prompting for confirmation.",
                "m365 spo folder archive --webUrl https://contoso.sharepoint.com/sites/Marketing --url '/sites/Marketing/shared documents/folder'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder copy",
        summary: "Copies a folder to another location",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "sourceUrl",
                "The server-, site-relative or absolute decoded URL of the folder to copy.",
            ),
            ("sourceId", "The unique ID (GUID) of the folder."),
            (
                "targetUrl",
                "Server-relative or absolute decoded URL where to copy the folder.",
            ),
            ("newName", "New name of the destination folder."),
            (
                "nameConflictBehavior",
                "Behavior when a file or folder with the same name is already present at the destination. Allowed values: `fail`, `rename`. Defaults to `fail`.",
            ),
            ("skipWait", "Don't wait for the copy operation to complete."),
        ],
        notes: &[
            "When you specify a value for `nameConflictBehavior`, consider the following:",
            "- `fail` will throw an error when the destination folder already exists.",
            "- `rename` will add a suffix (e.g. Folder1) when the destination folder already exists.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Copy a folder a folder to another location using server-relative URLs",
                "m365 spo folder copy --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/sites/project-x/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents/Project files\"",
            ),
            (
                "Copy a folder a folder to another location using site-relative URLs and rename it",
                "m365 spo folder copy --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents\" --newName \"Reports January\"",
            ),
            (
                "Copy a folder to another location and use new name on conflict",
                "m365 spo folder copy --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/sites/project-x/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents/Project files\" --nameConflictBehavior rename",
            ),
            (
                "Copy a folder referenced by its ID to another document library and don't wait for the copy operation to finish",
                "m365 spo folder copy --webUrl https://contoso.sharepoint.com/sites/project-x --sourceId b8cc341b-9c11-4f2d-aa2b-0ce9c18bcba2 --targetUrl \"/sites/project-x/Project files\" --skipWait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder get",
        summary: "Gets information about the specified folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the folder to retrieve. Specify either `folderUrl` or `id` but not both.",
            ),
            ("id", "The unique ID (GUID) of the folder to retrieve."),
            (
                "withPermissions",
                "Set if you want to return associated roles and permissions of the folder.",
            ),
        ],
        notes: &[
            "If no folder exists at the specified URL, you will get a `Please check the folder URL. Folder might not exist on the specified URL` error.",
            "If root level folder is passed, you will get a `Please ensure the specified folder URL or folder Id does not refer to a root folder. Use \\'spo list get\\' with withPermissions instead' error.` Please use the command 'spo list get'.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get folder properties for a folder with a specific site-relative URL.",
                "m365 spo folder get --webUrl https://contoso.sharepoint.com/sites/project-x --url \"/Shared Documents\"",
            ),
            (
                "Get folder properties for a folder with a specific id (unique ID).",
                "m365 spo folder get --webUrl https://contoso.sharepoint.com/sites/project-x --id \"b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
            (
                "Get folder properties with permissions for a folder with server-relative URL.",
                "m365 spo folder get --webUrl https://contoso.sharepoint.com/sites/test --url \"/sites/test/Shared Documents/Test1\" --withPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder list",
        summary: "Returns all folders under the specified parent folder",
        options: &[
            (
                "webUrl",
                "The URL of the site where the folders to list are located.",
            ),
            (
                "parentFolderUrl",
                "The server- or site-relative decoded URL of the parent folder.",
            ),
            (
                "fields",
                "Comma-separated list of fields to retrieve. Will retrieve all fields if not specified and json output is requested.",
            ),
            (
                "filter",
                "OData filter to use to query the list of folders with.",
            ),
            ("recursive", "Set to retrieve nested folders."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets list of folders under a parent folder with site-relative URL.",
                "m365 spo folder list --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/Shared Documents'",
            ),
            (
                "Gets recursive list of folders under a specific folder on a specific site.",
                "m365 spo folder list --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/sites/project-x/Shared Documents' --recursive",
            ),
            (
                "Return a filtered list of folders and only return the list item ID.",
                "m365 spo folder list --webUrl https://contoso.sharepoint.com/sites/project-x --parentFolderUrl '/Shared Documents' --fields ListItemAllFields/Id --filter \"startswith(Name,'Folder')\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder move",
        summary: "Moves a folder to another location",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "sourceUrl",
                "The server-, site-relative or absolute decoded URL of the folder to move.",
            ),
            ("sourceId", "The ID of the folder to move."),
            (
                "targetUrl",
                "Server-relative or absolute decoded URL where to move the folder.",
            ),
            ("newName", "The new name of the destination folder."),
            (
                "nameConflictBehavior",
                "Behavior when a file or folder with the same name is already present at the destination. Allowed values: `fail`, `rename`. Defaults to `fail`.",
            ),
            ("skipWait", "Don't wait for the move operation to complete."),
        ],
        notes: &[
            "When you specify a value for `nameConflictBehavior`, consider the following:",
            "- `fail` will throw an error when the destination folder already exists.",
            "- `rename` will add a suffix (e.g. folder1) when the destination folder already exists.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Move a folder to a document library in another site collection by server-relative URL.",
                "m365 spo folder move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/sites/project-x/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents/Reports January\"",
            ),
            (
                "Move a folder to another location using site-relative URLs and rename it.",
                "m365 spo folder move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents\" --newName \"Reports January\"",
            ),
            (
                "Move a folder to another location and use a new name on conflict.",
                "m365 spo folder move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceUrl \"/sites/project-x/Shared Documents/Reports\" --targetUrl \"/sites/project-y/Shared Documents/Project files\" --nameConflictBehavior rename",
            ),
            (
                "Move a folder referenced by its ID to another document library and don't wait for the move job to finish.",
                "m365 spo folder move --webUrl https://contoso.sharepoint.com/sites/project-x --sourceId b8cc341b-9c11-4f2d-aa2b-0ce9c18bcba2 --targetUrl \"/sites/project-x/Project files\" --skipWait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder remove",
        summary: "Deletes the specified folder",
        options: &[
            (
                "webUrl",
                "The URL of the site where the folder to be deleted is located.",
            ),
            (
                "url",
                "The server- or site-relative decoded URL of the folder to delete.",
            ),
            (
                "permanent",
                "Permanently deletes the folder instead of moving it to the recycle bin.",
            ),
            ("force", "Don't prompt for confirming deleting the folder."),
        ],
        notes: &[
            "The `spo folder remove` command will remove folder only if it is empty. If the folder contains any files, deleting the folder will fail.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a folder with a specific site-relative URL.",
                "m365 spo folder remove --webUrl https://contoso.sharepoint.com/sites/project-x --url '/Shared Documents/My Folder'",
            ),
            (
                "Permanently remove a folder with a specific server-relative URL.",
                "m365 spo folder remove --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/Shared Documents/My Folder' --permanent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder retentionlabel ensure",
        summary: "Applies a retention label to a folder",
        options: &[
            (
                "webUrl",
                "URL of the site where the retention label from a file to apply is located.",
            ),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder that should be labelled.",
            ),
            (
                "folderId",
                "The unique ID (GUID) of the folder that should be labelled.",
            ),
            (
                "name",
                "Name of the retention label to apply to the folder.",
            ),
        ],
        notes: &[
            "You can also use spo listitem retentionlabel remove for removing the retention label from a list item.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Applies a retention label to a folder based on the label name and the folderUrl.",
                "m365 spo folder retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl '/Shared Documents' --name 'Some label'",
            ),
            (
                "Applies a retention label to a folder based on the label name and the folderId.",
                "m365 spo folder retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --folderId '26541f96-017c-4189-a604-599e083533b8'  --name 'Some label'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder retentionlabel remove",
        summary: "Clears the retention label from a folder",
        options: &[
            ("webUrl", "The url of the web."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder of which the label should be removed.",
            ),
            (
                "folderId",
                "The unique ID (GUID) of the folder of which the label should be removed.",
            ),
            ("force", "Don't prompt for confirming to remove the label."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes the retention label from a folder in a given site based on the folder id.",
                "m365 spo folder retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --folderId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Removes the retention label from a folder in a given site based on the folder url.",
                "m365 spo folder retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl /sites/project-x/Shared Documents/Folder",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder roleassignment add",
        summary: "Adds a role assignment from the specified folder.",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            (
                "principalId",
                "The SharePoint principal id. It may be either an user id or group id for which the role assignment will be addd.",
            ),
            ("upn", "The upn/email of the user."),
            ("groupName", "The SharePoint group name."),
            ("entraGroupId", "ID of the Microsoft Entra group to add."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to add.",
            ),
            ("roleDefinitionId", "ID of the role definition."),
            (
                "roleDefinitionName",
                "The name of the role definition. E.g. 'Contribute', 'Read'.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Add the role assignment to the specified folder based on the group name and role definition name.",
                "m365 spo folder roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl  \"/Shared Documents/FolderPermission\" --groupName \"saleGroup\" --roleDefinitionName \"Edit\"",
            ),
            (
                "Add the role assignment to the specified folder based on the principal Id and role definition id.",
                "m365 spo folder roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --principalId 2 --roleDefinitionId 1073741827",
            ),
            (
                "Add the role assignment to the specified folder based on the upn and role definition name.",
                "m365 spo folder roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --upn \"test@contoso.onmicrosoft.com\" --roleDefinitionName \"Edit\"",
            ),
            (
                "Add the role assignment to the root folder based on the upn and role definition name.",
                "m365 spo folder roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents\" --upn \"test@contoso.onmicrosoft.com\" --roleDefinitionName \"Edit\"",
            ),
            (
                "Add the role assignment to the specified folder based on the Entra Group Id and role definition id.",
                "m365 spo folder roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\" --roleDefinitionId 1073741827",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder roleassignment remove",
        summary: "Removes a role assignment from the specified folder.",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            (
                "principalId",
                "The SharePoint principal id. It may be either an user id or group id for which the role assignment will be removed.",
            ),
            ("upn", "The upn/email of the user."),
            ("groupName", "The SharePoint group name."),
            ("entraGroupId", "ID of the Microsoft Entra group to remove."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to remove.",
            ),
            (
                "force",
                "Don't prompt for confirmation when removing the role assignment.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove roleassignment from folder based on group name.",
                "m365 spo folder roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl  \"/Shared Documents/FolderPermission\" --groupName \"saleGroup\"",
            ),
            (
                "Remove the role assignment from the specified folder based on the principal id.",
                "m365 spo folder roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --principalId 2",
            ),
            (
                "Remove the role assignment from the specified folder based on the principal id without prompting for removal confirmation.",
                "m365 spo folder roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --principalId 2 --force",
            ),
            (
                "Remove the role assignment from the specified folder based on the upn.",
                "m365 spo folder roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --upn \"test@contoso.onmicrosoft.com\"",
            ),
            (
                "Remove the role assignment from the specified folder based on the Entra group id.",
                "m365 spo folder roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --folderUrl \"/Shared Documents/FolderPermission\" --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder roleinheritance break",
        summary: "Breaks the role inheritance of a folder.",
        options: &[
            ("webUrl", "URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            (
                "clearExistingPermissions",
                "Clear all existing permissions from the folder.",
            ),
            (
                "force",
                "Don't prompt for confirmation to breaking role inheritance of the folder.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Break the inheritance of a folder with a specified site-relative URL.",
                "m365 spo folder roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"Shared Documents/TestFolder\"",
            ),
            (
                "Break the inheritance of a folder with a specified server-relative URL. It will clear the existing permissions of the folder. It will not prompt for confirmation before breaking the inheritance.",
                "m365 spo folder roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"/sites/project-x/Shared Documents/TestFolder\" --clearExistingPermissions --force",
            ),
            (
                "Break the inheritance of the specified root folder for the library with a specified server-relative URL. It will clear the existing permissions of the root folder. It will not prompt for confirmation before breaking the inheritance.",
                "m365 spo folder roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"/sites/project-x/Shared Documents\" --clearExistingPermissions --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder roleinheritance reset",
        summary: "Restores the role inheritance of a folder.",
        options: &[
            ("webUrl", "URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            (
                "force",
                "Don't prompt for confirmation to reset role inheritance of the folder.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Reset inheritance of folder with a specific site-relative URL.",
                "m365 spo folder roleinheritance reset --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"Shared Documents/TestFolder\"",
            ),
            (
                "Reset inheritance of folder with a specific server-relative URL. It will not prompt for confirmation before resetting.",
                "m365 spo folder roleinheritance reset --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"/sites/project-x/Shared Documents/TestFolder\" --force",
            ),
            (
                "Reset inheritance of the specified root folder for the library with a specific server-relative URL. It will not prompt for confirmation before resetting.",
                "m365 spo folder roleinheritance reset --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --folderUrl \"/sites/project-x/Shared Documents\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder set",
        summary: "Updates a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("name", "New name for the folder."),
            (
                "color",
                "Visual color of the folder. Valid values are a color name or a number. Check remarks for more info.",
            ),
        ],
        notes: &[
            "When you specify a value for `color`, consider the following:",
            "Color | Number value | String value",
            "Yellow | 0 | yellow",
            "Dark red | 1 | darkRed",
            "Dark orange | 2 | darkOrange",
            "Dark green | 3 | darkGreen",
            "Dark teal | 4 | darkTeal",
            "Dark blue | 5 | darkBlue",
            "Dark purple | 6 | darkPurple",
            "Dark pink | 7 | darkPink",
            "Grey | 8 | grey",
            "Light red | 9 | lightRed",
            "Light orange | 10 | lightOrange",
            "Light green | 11 | lightGreen",
            "Light teal | 12 | lightTeal",
            "Light blue | 13 | lightBlue",
            "Light purple | 14 | lightPurple",
            "Light pink | 15 | lightPink",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Set the folder name to 'My Folder 2'",
                "m365 spo folder set --webUrl https://contoso.sharepoint.com/sites/project-x --url '/Shared Documents/My Folder 1' --name 'My Folder 2'",
            ),
            (
                "Set the folder name to 'My folder 2'",
                "m365 spo folder set --webUrl https://contoso.sharepoint.com/sites/project-x --url '/sites/project-x/Shared Documents/My Folder 1' --name 'My Folder 2'",
            ),
            (
                "Set the folder color to dark blue",
                "m365 spo folder set --webUrl https://contoso.sharepoint.com --url '/ProjectFiles/Project-x' --color 5",
            ),
            (
                "Set the folder color to light teal",
                "m365 spo folder set --webUrl https://contoso.sharepoint.com --url '/ProjectFiles/Project-x' --color lightTeal",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink add",
        summary: "Creates a new sharing link to a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The unique ID (GUID) of the folder."),
            (
                "type",
                "The type of sharing link to create. Either `view` or `edit`.",
            ),
            (
                "expirationDateTime",
                "The date and time to set the expiration. This should be defined as a valid ISO 8601 string.",
            ),
            (
                "scope",
                "The scope of link to create. Either `anonymous`, `organization` or `users`. If not specified, the default of the organization will be used.",
            ),
            (
                "retainInheritedPermissions",
                "If `true`, any existing inherited permissions are retained on the shared item when sharing this item for the first time. If `false`, all existing permissions are removed when sharing for the first time.",
            ),
            (
                "recipients",
                "Comma separated list of users with whom we wish to share the item with. Required when using scope `users`.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.Read.All",
            ),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.ReadWrite.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Creates a view-only anonymous sharing link of a folder by id.",
                "m365 spo folder sharinglink add --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234 --type view --scope anonymous",
            ),
            (
                "Creates an edit organization sharing link of a folder by url with a specific expiration date.",
                "m365 spo folder sharinglink add --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl /sites/demo/shared%20documents/Folder --type edit --scope organization --expirationDateTime '2022-11-30T00:00:00Z'",
            ),
            (
                "Creates a user sharing link of a folder by id.",
                "m365 spo folder sharinglink add --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234 --type view --scope users --recipients john@contoso.com,doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink clear",
        summary: "Removes sharing links of a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The Unique ID (GUID) of the folder."),
            (
                "scope",
                "Scope of the sharing link. Possible options are: `anonymous`, `users` or `organization`. If not specified, all links will be removed.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.Read.All",
            ),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.ReadWrite.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Removes all sharing links from a folder specified by id without prompting for confirmation.",
                "m365 spo folder sharinglink clear --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234 --force",
            ),
            (
                "Removes sharing links of type anonymous from a folder specified by url with prompting for confirmation.",
                "m365 spo folder sharinglink clear --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl '/sites/demo/Shared Documents/folder1' --scope anonymous",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink get",
        summary: "Gets details about a specific sharing link on a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The Unique ID (GUID) of the folder."),
            ("id", "The sharing link ID."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.Read, Sites.Read.All"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.Read.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets a specific sharing link of a folder by id.",
                "m365 spo folder sharinglink get --webUrl https://contoso.sharepoint.com/sites/demo --id 45fa6aed-362f-48b1-b04e-6da85a526506 --folderId daebb04b-a773-4baa-b1d1-3625418e3234",
            ),
            (
                "Gets a specific sharing link of a folder by url.",
                "m365 spo folder sharinglink get --webUrl https://contoso.sharepoint.com/sites/demo --id 45fa6aed-362f-48b1-b04e-6da85a526506 --folderUrl \"/sites/demo/shared documents/folder\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink list",
        summary: "Lists all the sharing links of a specific folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The unique ID (GUID) of the folder."),
            (
                "scope",
                "Filter the results to only sharing links of a given scope: `anonymous`, `users` or `organization`. By default all sharing links are listed.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.Read, Sites.Read.All"),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.Read.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List sharing links of a folder by id.",
                "m365 spo folder sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234",
            ),
            (
                "List sharing links of a folder by url.",
                "m365 spo folder sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl \"/sites/demo/shared documents/folder\"",
            ),
            (
                "List anonymous sharing links of a folder by url.",
                "m365 spo folder sharinglink list --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl \"/sites/demo/shared documents/folder\" --scope anonymous",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink remove",
        summary: "Removes a specific sharing link of a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The unique ID (GUID) of the folder."),
            ("id", "The ID of the sharing link."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.Read.All",
            ),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.ReadWrite.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Removes a specific sharing link from a folder by id without prompting for confirmation.",
                "m365 spo folder sharinglink remove --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234 --id c391b57d-5783-4c53-9236-cefb5c6ef323 --force",
            ),
            (
                "Removes a specific sharing link from a folder by url with prompting for confirmation.",
                "m365 spo folder sharinglink remove --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl /sites/demo/shared%20documents/Folder --id c391b57d-5783-4c53-9236-cefb5c6ef323",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder sharinglink set",
        summary: "Updates a sharing link of a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "folderUrl",
                "The server- or site-relative decoded URL of the folder.",
            ),
            ("folderId", "The unique ID (GUID) of the folder."),
            ("id", "The sharing link ID."),
            (
                "expirationDateTime",
                "The date and time to set the expiration. This should be defined as a valid ISO 8601 string. This option only works for anonymous links.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.Read.All",
            ),
            ("Delegated", "SharePoint", "AllSites.Read"),
            (
                "Application",
                "Microsoft Graph",
                "Files.ReadWrite.All, Sites.Read.All",
            ),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Updates the expiration datetime of an anonymous sharing link for a folder specific by folder ID.",
                "m365 spo folder sharinglink set --webUrl https://contoso.sharepoint.com/sites/demo --folderId daebb04b-a773-4baa-b1d1-3625418e3234 --id 7c9f97c9-1bda-433c-9364-bb83e81771ee --expirationDateTime '2022-11-30T00:00:00Z'",
            ),
            (
                "Updates the expiration datetime of an anonymous sharing link for a folder specific by folder URL.",
                "m365 spo folder sharinglink set --webUrl https://contoso.sharepoint.com/sites/demo --folderUrl /sites/demo/shared%20documents/Folder --id 7c9f97c9-1bda-433c-9364-bb83e81771ee --expirationDateTime '2022-11-30T00:00:00Z'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo folder unarchive",
        summary: "Unarchives a folder",
        options: &[
            ("webUrl", "The URL of the site where the folder is located."),
            (
                "url",
                "The server- or site-relative decoded URL of the folder to unarchive.",
            ),
            ("id", "The unique ID (GUID) of the folder to unarchive."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "Reactivation could take up to 24 hours. Folders that are reactivated cannot be archived again for 120 days.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Unarchive a folder without prompting for confirmation",
                "m365 spo folder unarchive --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7a8c9207-7745-4cda-b0e2-be2618ee3030 --force",
            ),
            (
                "Unarchive a folder by URL with prompting for confirmation",
                "m365 spo folder unarchive --webUrl https://contoso.sharepoint.com/sites/Marketing --url '/sites/Marketing/documents/general'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo get",
        summary: "Gets the context URL for the root SharePoint site collection and SharePoint tenant admin site",
        options: &[],
        notes: &[
            "CLI for Microsoft 365 automatically discovers the URL of the root SharePoint site collection/SharePoint tenant admin site (whichever is needed to run the particular command). Using this command you can see which URLs the CLI has discovered.",
        ],
        permissions: &[],
        examples: &[(
            "Get the context URL for the root SharePoint site collection and SharePoint tenant admin site",
            "m365 spo get --output json",
        )],
        links: &[],
    },
    Help {
        name: "spo group add",
        summary: "Creates group in the specified site",
        options: &[
            ("webUrl", "URL of the site where the group should be added."),
            ("name", "Name of the group to add."),
            ("description", "The description for the group."),
            (
                "allowMembersEditMembership",
                "Who can edit the membership of the group? When `true` members can edit membership, otherwise only owners can do this. Default `false`.",
            ),
            (
                "onlyAllowMembersViewMembership",
                "Who can view the membership of the group? When `false` everyone can view the group membership, otherwise only group members can. Default `true`.",
            ),
            (
                "allowRequestToJoinLeave",
                "Specify whether to allow users to request membership in this group and allow users to request to leave the group. Default `false`.",
            ),
            (
                "autoAcceptRequestToJoinLeave",
                "If auto-accept is enabled, users will automatically be added or removed when they make a request. Default `false`.",
            ),
            (
                "requestToJoinLeaveEmailSetting",
                "All membership requests will be sent to the email address specified.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Create group with title and description.",
                "m365 spo group add --webUrl https://contoso.sharepoint.com/sites/project-x --name \"Project leaders\" --description \"This group contains all project leaders\"",
            ),
            (
                "Create group with membership requests.",
                "m365 spo group add --webUrl https://contoso.sharepoint.com/sites/project-x --name \"Project leaders\" --allowRequestToJoinLeave true --requestToJoinLeaveEmailSetting john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group get",
        summary: "Gets site group",
        options: &[
            ("webUrl", "URL of the site where the group is located."),
            ("id", "ID of the site group to get."),
            ("name", "Name of the site group to get."),
            (
                "associatedGroup",
                "Type of the associated group to get. Available values: `Owner`, `Member`, `Visitor`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get group with ID for the specified web.",
                "m365 spo group get --webUrl https://contoso.sharepoint.com/sites/project-x --id 7",
            ),
            (
                "Get group with name for the specified web.",
                "m365 spo group get --webUrl https://contoso.sharepoint.com/sites/project-x --name \"Team Site Members\"",
            ),
            (
                "Get the associated owner group of a specified site.",
                "m365 spo group get --webUrl https://contoso.sharepoint.com/sites/project-x --associatedGroup Owner",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group list",
        summary: "Lists all the groups within specific web",
        options: &[
            ("webUrl", "Url of the web to list the group within."),
            (
                "associatedGroupsOnly",
                "Get only the associated visitor, member and owner groups of the site.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Lists all the groups within a specific web.",
                "m365 spo group list --webUrl \"https://contoso.sharepoint.com/sites/contoso\"",
            ),
            (
                "Lists the associated groups within a specific web.",
                "m365 spo group list --webUrl \"https://contoso.sharepoint.com/sites/contoso\" --associatedGroupsOnly",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group member add",
        summary: "Adds members to a SharePoint Group",
        options: &[
            (
                "webUrl",
                "URL of the site where the SharePoint group is available.",
            ),
            (
                "groupId",
                "Id of the SharePoint Group to which the user needs to be added.",
            ),
            (
                "groupName",
                "Name of the SharePoint Group to which the user needs to be added.",
            ),
            (
                "userNames",
                "User's UPN (user principal name, eg. megan.bowen@contoso.com). If multiple users need to be added, they have to be comma-separated (e.g. megan.bowen@contoso.com,alex.wilber@contoso.com).",
            ),
            (
                "emails",
                "User's email (eg. megan.bowen@contoso.com). If multiple users need to be added, they have to be comma-separated (e.g. megan.bowen@contoso.com,alex.wilber@contoso.com).",
            ),
            (
                "userIds",
                "The user Id of the user to add as a member. (Id of the site user, for example: 14). If multiple users need to be added, the Ids have to be comma-separated.",
            ),
            (
                "entraGroupIds",
                "The object Id of the Entra group to add as a member. If multiple groups need to be added, the Ids have to be comma-separated.",
            ),
            (
                "entraGroupNames",
                "The name of the Entra group to add as a member. If multiple groups need to be added, they have to be comma-separated.",
            ),
        ],
        notes: &[
            "For the `userIds`, `userNames`, `emails`, `entraGroupIds`, or `entraGroupNames` options you can specify multiple values by separating them with a comma. If one of the specified entries is not valid, the command will fail with an error message showing the list of invalid values.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "Microsoft Graph", "User.Read.All"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Add a user with the userNames parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --userNames \"Alex.Wilber@contoso.com\"",
            ),
            (
                "Add multiple users with the userNames parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --userNames \"Alex.Wilber@contoso.com,Adele.Vance@contoso.com\"",
            ),
            (
                "Add a user with the emails parameter to a SharePoint group with the groupName parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Contoso Site Owners\" --emails \"Alex.Wilber@contoso.com\"",
            ),
            (
                "Add multiple users with the emails parameter to a SharePoint group with the groupName parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Contoso Site Owners\" --emails \"Alex.Wilber@contoso.com,Adele.Vance@contoso.com\"",
            ),
            (
                "Add a user with the userIds parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --userIds 5",
            ),
            (
                "Add multiple users with the userIds parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --userIds \"5,12\"",
            ),
            (
                "Add multiple users with the entraGroupIds parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --entraGroupIds \"f2fb2f10-cfd2-4054-8ffd-64533657a5ab,3e86049e-89e6-4c27-bccb-d7549f0bbd06\"",
            ),
            (
                "Add multiple users with the entraGroupNames parameter to a SharePoint group with the groupId parameter.",
                "m365 spo group member add --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --entraGroupNames \"Azure group one, Azure group two\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group member list",
        summary: "Lists the members of a SharePoint Group",
        options: &[
            ("webUrl", "URL of the SharePoint site."),
            ("groupId", "Id of the SharePoint group."),
            ("groupName", "Name of the SharePoint group."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List the members of the group with ID for the specified web.",
                "m365 spo group member list --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5",
            ),
            (
                "List the members of the group with name for the specified web.",
                "m365 spo group member list --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Contoso Site Members\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group member remove",
        summary: "Removes the specified member from a SharePoint group",
        options: &[
            (
                "webUrl",
                "URL of the site where the SharePoint group is available.",
            ),
            (
                "groupId",
                "Id of the SharePoint group from which the user has to be removed.",
            ),
            (
                "groupName",
                "Name of the SharePoint group from which user has to be removed.",
            ),
            (
                "userName",
                "The UPN (user principal name, eg. megan.bowen@contoso.com) of the user that needs to be removed.",
            ),
            ("email", "The email of the user to remove as a member."),
            (
                "userId",
                "The user Id (Id of the site user, eg. 14) of the user to remove as a member.",
            ),
            (
                "entraGroupId",
                "The object Id of the Entra group to remove as a member.",
            ),
            (
                "entraGroupName",
                "The name of the Entra group to remove as a member.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "Microsoft Graph", "User.Read.All"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove a user from a SharePoint group based on the id on a given web.",
                "m365 spo group member remove --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --userName \"Alex.Wilber@contoso.com\"",
            ),
            (
                "Remove a user from a SharePoint group based on the username on a given web.",
                "m365 spo group member remove --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Site A Visitors\" --email \"Alex.Wilber@contoso.com\"",
            ),
            (
                "Remove a user from a SharePoint group by email.",
                "m365 spo group member remove --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Site A Visitors\" --userId 14",
            ),
            (
                "Remove an Entra group from a SharePoint group based on the Entra group name on a given web.",
                "m365 spo group member remove --webUrl https://contoso.sharepoint.com/sites/SiteA --groupId 5 --entraGroupName \"Microsoft Entra ID Security Group\"",
            ),
            (
                "Remove an Entra group from a SharePoint group based on the Entra group ID on a given web.",
                "m365 spo group member remove --webUrl https://contoso.sharepoint.com/sites/SiteA --groupName \"Site A Visitors\" --entraGroupId \"5786b8e8-c495-4734-b345-756733960730\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group remove",
        summary: "Removes group from specific web",
        options: &[
            ("webUrl", "Url of the web to remove the group from."),
            (
                "id",
                "ID of the group to remove. Use ID or name but not both.",
            ),
            (
                "name",
                "Name of the group to remove. Use ID or name but not both.",
            ),
            ("force", "Confirm removal of the group."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Removes group with id from the specified web.",
                "m365 spo group remove --webUrl https://contoso.sharepoint.com/sites/mysite --id 5",
            ),
            (
                "Removes group with name from the specified web.",
                "m365 spo group remove --webUrl https://contoso.sharepoint.com/sites/mysite --name \"Team Site Owners\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo group set",
        summary: "Updates a group in the specified site",
        options: &[
            ("webUrl", "URL of the site where the group is located."),
            ("id", "ID of the group to update."),
            ("name", "Name of the group."),
            ("newName", "New name for the group."),
            ("description", "The description for the group."),
            (
                "allowMembersEditMembership",
                "Who can edit the membership of the group? When `true` members can edit membership, otherwise only owners can do this.",
            ),
            (
                "onlyAllowMembersViewMembership",
                "Who can view the membership of the group? When `true` only group members can view the membership, otherwise everyone can.",
            ),
            (
                "allowRequestToJoinLeave",
                "Specify whether to allow users to request membership in this group and allow users to request to leave the group.",
            ),
            (
                "autoAcceptRequestToJoinLeave",
                "If auto-accept is enabled, users will automatically be added or removed when they make a request.",
            ),
            (
                "requestToJoinLeaveEmailSetting",
                "All membership requests will be sent to the email address specified.",
            ),
            (
                "ownerEmail",
                "Set user with this email as owner of the group.",
            ),
            (
                "ownerUserName",
                "Set user with this login name as owner of the group.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "User.Read.All"),
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "Microsoft Graph", "User.Read.All"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Update group title and description.",
                "m365 spo group set --webUrl https://contoso.sharepoint.com/sites/project-x --id 18 --newName \"Project leaders\" --description \"This group contains all project leaders\"",
            ),
            (
                "Update group with membership requests.",
                "m365 spo group set --webUrl https://contoso.sharepoint.com/sites/project-x --name \"Project leaders\" --allowRequestToJoinLeave true --requestToJoinLeaveEmailSetting john.doe@contoso.com",
            ),
            (
                "Sets a specified user as group owner.",
                "m365 spo group set --webUrl https://contoso.sharepoint.com/sites/project-x --id 18 --ownerEmail john.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo hidedefaultthemes get",
        summary: "Gets the current value of the HideDefaultThemes setting",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Get the current value of the HideDefaultThemes setting.",
            "m365 spo hidedefaultthemes get",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
        )],
    },
    Help {
        name: "spo hidedefaultthemes set",
        summary: "Sets the value of the HideDefaultThemes setting",
        options: &[(
            "hideDefaultThemes",
            "Set to `true` to hide default themes and to `false` to show them.",
        )],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Hide default themes and allow users to use organization themes only.",
            "m365 spo hidedefaultthemes set --hideDefaultThemes true",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
        )],
    },
    Help {
        name: "spo homesite add",
        summary: "Adds a home site",
        options: &[
            ("url", "URL of the site to use as a home site."),
            (
                "isInDraftMode",
                "Specifies whether the home site is in draft mode. Accepts `true` or `false`. Default is `false`.",
            ),
            (
                "vivaConnectionsDefaultStart",
                "Specifies whether the home site is the default start for Viva Connections. Accepts `true` or `false`. Default is `true`.",
            ),
            (
                "audienceIds",
                "Comma-separated list of Microsoft Entra group IDs to be used as audiences.",
            ),
            (
                "audienceNames",
                "Comma-separated list of Microsoft Entra group names to be used as audiences. You must use either `audienceIds` or `audienceNames`, but not both.",
            ),
            (
                "order",
                "Order of the home site. Must be a positive integer.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Add a home site",
                "m365 spo homesite add --url \"https://contoso.sharepoint.com/sites/testcomms\"",
            ),
            (
                "Add a home site with audiences specified by their group ids and with additional options",
                "m365 spo homesite add --url \"https://contoso.sharepoint.com/sites/testcomms\" --isInDraftMode true --vivaConnectionsDefaultStart false --audienceIds \"af8c0bc8-7b1b-44b4-b087-ffcc8df70d16,754ff15c-76b1-44cb-88c7-0065a4d3cfb7\" --order 2",
            ),
            (
                "Add a home site with audiences specified by their group names and with additional options",
                "m365 spo homesite add --url \"https://contoso.sharepoint.com/sites/testcomms\" --isInDraftMode true --vivaConnectionsDefaultStart false --audienceNames \"Marketing, IT department\" --order 2",
            ),
        ],
        links: &[(
            "Viva Connections set up",
            "https://learn.microsoft.com/en-us/viva/connections/set-up-admin-center",
        )],
    },
    Help {
        name: "spo homesite get",
        summary: "Gets information about a home site",
        options: &[("url", "URL of the home site.")],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[(
            "Get information about a home site.",
            "m365 spo homesite get --url https://contoso.sharepoint.com/sites/Home",
        )],
        links: &[(
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
        )],
    },
    Help {
        name: "spo homesite list",
        summary: "Lists all home sites",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[("List all home sites", "m365 spo homesite list")],
        links: &[(
            "Viva Connections set up",
            "https://learn.microsoft.com/en-us/viva/connections/set-up-admin-center",
        )],
    },
    Help {
        name: "spo homesite remove",
        summary: "Removes a home site",
        options: &[
            ("url", "URL of the home site to remove."),
            ("force", "Do not prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Removes a home site specified by URL.",
                "m365 spo homesite remove --url \"https://contoso.sharepoint.com/sites/Europe\"",
            ),
            (
                "Removes a home site specified by URL without prompting for confirmation.",
                "m365 spo homesite remove --url \"https://contoso.sharepoint.com/sites/Europe\" --force",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
        )],
    },
    Help {
        name: "spo homesite set",
        summary: "Updates an existing SharePoint home site.",
        options: &[
            ("url", "The URL of the site to set as Home site."),
            (
                "draftMode",
                "Set draft status. Valid values are: `true`, `false`.",
            ),
            (
                "audienceIds",
                "Comma-separated list of group Ids to set as audiences.",
            ),
            (
                "audienceNames",
                "Comma-separated list of group names to set as audiences. You must use either `audienceIds` or `audienceNames`, but not both. Pass `''` to clear audience.",
            ),
            (
                "targetedLicenseType",
                "Value of either `everyone`, `frontLineWorkers` or `informationWorkers` to create an experience for a distinct audience by license type.",
            ),
            (
                "vivaConnectionsDefaultStart",
                "Specifies whether the home site is the default start for Viva Connections. Accepts `true` or `false`.",
            ),
            (
                "order",
                "The position of the home site among the Viva Connections home sites (a positive integer).",
            ),
        ],
        notes: &["To use this command you must be a Global or SharePoint administrator."],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Sets the home site to the provided site collection url and sets the Viva Connections landing experience to the SharePoint home site",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/comms --vivaConnectionsDefaultStart true",
            ),
            (
                "Set the home site in draft mode",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --draftMode true",
            ),
            (
                "Set the audience to everyone",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --targetedLicenseType \"everyone\"",
            ),
            (
                "Set the audience to groups using groups Ids",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --audienceIds \"978b5280-4f80-47ea-a1db-b0d1d2fb1ba4,21af775d-17b3-4637-94a4-2ba8625277cb\"",
            ),
            (
                "Set the audience using their display name",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --audienceNames \"Marketing, IT department\"",
            ),
            (
                "Clear the audience and set to draft",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --audienceNames '' --draftMode true",
            ),
            (
                "Configure multiple options",
                "m365 spo homesite set --url https://contoso.sharepoint.com/sites/contosoportal --audienceNames 'Marketing, IT department' --draftMode true --targetedLicenseType \"everyone\" --vivaConnectionsDefaultStart true",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
            "https://techcommunity.microsoft.com/t5/Microsoft-SharePoint-Blog/SharePoint-home-sites-a-landing-for-your-organization-on-the/ba-p/621933",
        )],
    },
    Help {
        name: "spo hubsite connect",
        summary: "Connects a hub site to a parent hub site",
        options: &[
            ("id", "ID of the hub site."),
            ("title", "Title of the hub site."),
            ("url", "Absolute or server-relative URL of the hub site."),
            ("parentId", "ID of the parent hub site."),
            ("parentTitle", "Title of the parent hub site."),
            (
                "parentUrl",
                "Absolute or server-relative URL of the parent hub site.",
            ),
        ],
        notes: &[
            "To use this command you must be a Global or SharePoint administrator.",
            "To connect a regular site to a hub site, use command spo site hubsite connect.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Connect a specific hub site to specific parent hub site by ID.",
                "m365 spo hubsite connect --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --parentId 637ed2ea-b65b-4a4b-a3d7-ad86953224a4",
            ),
            (
                "Connect a specific hub site to specific parent hub site by URL.",
                "m365 spo hubsite connect --url https://contoso.sharepoint.com/sites/project-x --parentUrl https://contoso.sharepoint.com/sites/projects",
            ),
            (
                "Connect a specific hub site with title to a parent hub site with ID.",
                "m365 spo hubsite connect --title \"My hub site\" --parentId 637ed2ea-b65b-4a4b-a3d7-ad86953224a4",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo hubsite data get",
        summary: "Gets hub site data for the specified site",
        options: &[
            (
                "webUrl",
                "URL of the site for which to retrieve hub site data.",
            ),
            (
                "forceRefresh",
                "Set, to refresh the server cache with the latest updates.",
            ),
        ],
        notes: &[
            "By default, the hub site data is returned from the server's cache. To refresh the data with the latest updates, use the `--forceRefresh` option. Use this option, if you just made changes and need to see them right away.",
            "If the specified site is not connected to a hub site site and is not a hub site itself, no data will be retrieved.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "Get information about the hub site data for a specific site with URL.",
            "m365 spo hubsite data get --webUrl https://contoso.sharepoint.com/sites/project-x",
        )],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite disconnect",
        summary: "Disconnects a hub site from its parent hub site",
        options: &[
            ("id", "ID of the hub site."),
            ("title", "Title of the hub site."),
            ("url", "Absolute or server-relative URL of the hub site."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be a Global or SharePoint administrator.",
            "To disconnect a regular site from a hub site, use command spo site hubsite disconnect.",
            "If the specified id doesn't point to a valid hub site, you will get a ResourceNotFoundException error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Disconnect a specific hub site with id from its parent hub site.",
                "m365 spo hubsite disconnect --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Disconnect a specific hub site with url from its parent hub site.",
                "m365 spo hubsite disconnect --url https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Disconnect a specific hub site with title from its parent hub site.",
                "m365 spo hubsite disconnect --title \"My hub site\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo hubsite get",
        summary: "Gets information about the specified hub site",
        options: &[
            ("id", "ID of the hub site."),
            ("title", "Title of the hub site."),
            ("url", "URL of the hub site."),
            (
                "withAssociatedSites",
                "Include the associated sites in the result (only in JSON output)",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get information about the hub site with specific ID.",
                "m365 spo hubsite get --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Get information about the hub site with specific title.",
                "m365 spo hubsite get --title 'My Hub Site'",
            ),
            (
                "Get information about the hub site with specific URL.",
                "m365 spo hubsite get --url 'https://contoso.sharepoint.com/sites/HubSite'",
            ),
            (
                "Get information about the hub site with specific ID, including its associated sites. Associated site info is only shown in JSON output.",
                "m365 spo hubsite get --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --withAssociatedSites --output json",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite list",
        summary: "Lists hub sites in the current tenant",
        options: &[(
            "withAssociatedSites",
            "Include the associated sites in the result (only in JSON output).",
        )],
        notes: &[
            "When using the text or csv output type, the command lists only the values of the `ID`, `SiteUrl` and `Title` properties of the hub site. With the output type as JSON, all available properties are included in the command output.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List hub sites in the current tenant",
                "m365 spo hubsite list",
            ),
            (
                "List hub sites, including their associated sites, in the current tenant. Associated site info is only shown in JSON output.",
                "m365 spo hubsite list --withAssociatedSites --output json",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite register",
        summary: "Registers the specified site collection as a hub site",
        options: &[(
            "siteUrl",
            "URL of the site collection to register as a hub site.",
        )],
        notes: &[
            "To use this command you must be a Global or SharePoint administrator.",
            "If the specified site collection is already registered as a hub site, you will get a `This site is already a HubSite.` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[(
            "Register the site collection with URL https://contoso.sharepoint.com/sites/sales as a hub site",
            "m365 spo hubsite register --siteUrl https://contoso.sharepoint.com/sites/sales",
        )],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite rights grant",
        summary: "Grants permissions to join the hub site for one or more principals",
        options: &[
            ("hubSiteUrl", "The URL of the hub site to grant rights on."),
            (
                "principals",
                "Comma-separated list of principals to grant join rights. Principals can be users or mail-enabled security groups in the form of `alias` or `alias@.com`.",
            ),
            (
                "rights",
                "Rights to grant to principals. Available values `Join`.",
            ),
        ],
        notes: &["To use this command you must be a Global or SharePoint administrator."],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Grant user with specific alias, permission to join sites to the hub site with specific URL.",
                "m365 spo hubsite rights grant --hubSiteUrl https://contoso.sharepoint.com/sites/sales --principals PattiF --rights Join",
            ),
            (
                "Grant users with specific aliases, permission to join sites to the hub site with specific URL.",
                "m365 spo hubsite rights grant --hubSiteUrl https://contoso.sharepoint.com/sites/sales --principals \"PattiF,AdeleV\" --rights Join",
            ),
            (
                "Grant user with specific email, permission to join sites to the hub site with specific URL.",
                "m365 spo hubsite rights grant --hubSiteUrl https://contoso.sharepoint.com/sites/sales --principals PattiF@contoso.com --rights Join",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite rights revoke",
        summary: "Revokes rights to join sites to the specified hub site for one or more principals",
        options: &[
            ("hubSiteUrl", "The URL of the hub site to revoke rights on."),
            (
                "principals",
                "Comma-separated list of principals to revoke join rights. Principals can be users or mail-enabled security groups in the form of `alias` or `alias@.com`.",
            ),
            ("force", "Don't prompt for confirming revoking rights"),
        ],
        notes: &["To use this command you must be a Global or SharePoint administrator."],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Revoke rights to join sites to the hub site with specific URL. from user with specific alias. Will prompt for confirmation before revoking the rights.",
                "m365 spo hubsite rights revoke --hubSiteUrl https://contoso.sharepoint.com/sites/sales --principals PattiF",
            ),
            (
                "Revoke rights to join sites to the hub site with specific URL. from user with specific aliases without prompting for confirmation.",
                "m365 spo hubsite rights revoke --hubSiteUrl https://contoso.sharepoint.com/sites/sales --principals \"PattiF,AdeleV\" --force",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite set",
        summary: "Updates properties of the specified hub site",
        options: &[
            ("id", "ID of the hub site."),
            ("title", "The new title for the hub site."),
            ("description", "The new description for the hub site."),
            ("logoUrl", "The URL of the new logo for the hub site."),
        ],
        notes: &[
            "To use this command you must be a Global or SharePoint administrator.",
            "If the specified `id` doesn't refer to an existing hub site, you will get an `Unknown Error` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Update hub site's title.",
                "m365 spo hubsite set --id 255a50b2-527f-4413-8485-57f4c17a24d1 --title Sales",
            ),
            (
                "Update hub site's title and description.",
                "m365 spo hubsite set --id 255a50b2-527f-4413-8485-57f4c17a24d1 --title Sales --description \"All things sales\"",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo hubsite unregister",
        summary: "Unregisters the specified site collection as a hub site",
        options: &[
            ("url", "URL of the site collection."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be a Global or SharePoint administrator.",
            "If the specified site collection is not registered as a hub site, you will get a `hubSiteId` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Unregister the site collection with specific URL. as a hub site. Will prompt for confirmation before unregistering the hub site.",
                "m365 spo hubsite unregister --url https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Unregister the site collection with specific URL. as a hub site without prompting for confirmation.",
                "m365 spo hubsite unregister --url https://contoso.sharepoint.com/sites/sales --force",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo knowledgehub get",
        summary: "Gets the Knowledge Hub Site URL for your tenant",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Gets the Knowledge Hub Site URL for your tenant.",
            "m365 spo knowledgehub get",
        )],
        links: &[],
    },
    Help {
        name: "spo knowledgehub remove",
        summary: "Removes the Knowledge Hub Site setting for your tenant",
        options: &[(
            "force",
            "Do not prompt for confirmation before removing the Knowledge Hub Site setting for your tenant.",
        )],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the Knowledge Hub Site setting for your tenant.",
                "m365 spo knowledgehub remove",
            ),
            (
                "Removes the Knowledge Hub Site setting for your tenant without confirmation.",
                "m365 spo knowledgehub remove --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo knowledgehub set",
        summary: "Sets the Knowledge Hub Site for your tenant",
        options: &[("siteUrl", "URL of the site to set as Knowledge Hub.")],
        notes: &[
            "If the specified url doesn't refer to an existing site collection, you will get a `404 - \"404 FILE NOT FOUND\"` error.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Sets the Knowledge Hub Site for your tenant.",
            "m365 spo knowledgehub set --siteUrl https://contoso.sharepoint.com/sites/knowledgesite",
        )],
        links: &[],
    },
    Help {
        name: "spo list add",
        summary: "Creates list in the specified site",
        options: &[
            ("title", "Title of the list to add."),
            (
                "baseTemplate",
                "The list definition type on which the list is based. Allowed values `Announcements`, `Contacts`, `CustomGrid`, `DataSources`,`DiscussionBoard`, `DocumentLibrary`, `Events`, `GanttTasks`, `GenericList`, `IssuesTracking`, `Links`, `NoCodeWorkflows`,`PictureLibrary`, `Survey`, `Tasks`, `WebPageLibrary`, `WorkflowHistory`, `WorkflowProcess`, `XmlForm`. Default value is `GenericList`.",
            ),
            ("webUrl", "URL of the site where the list should be added."),
            ("description", "The description for the list."),
            (
                "templateFeatureId",
                "The globally unique identifier (GUID) of a template feature that is associated with the list.",
            ),
            (
                "allowDeletion",
                "Boolean value specifying whether the list can be deleted. Valid values are `true`, `false`.",
            ),
            (
                "allowEveryoneViewItems",
                "Boolean value specifying whether everyone can view documents in the documentlibrary or attachments to items in the list. Valid values are `true`, `false`.",
            ),
            (
                "allowMultiResponses",
                "Boolean value specifying whether users are allowed to give multiple responses to the survey. Valid values are `true`, `false`.",
            ),
            (
                "contentTypesEnabled",
                "Boolean value specifying whether content types are enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "crawlNonDefaultViews",
                "Boolean value specifying whether to crawl non default views. Valid values are `true`, `false`.",
            ),
            (
                "defaultContentApprovalWorkflowId",
                "Value that specifies the default workflow identifier for content approval onthe list (GUID).",
            ),
            (
                "defaultDisplayFormUrl",
                "Value that specifies the location of the default display form for the list.",
            ),
            (
                "defaultEditFormUrl",
                "Value that specifies the URL of the edit form to use for list items in the list.",
            ),
            (
                "direction",
                "Value that specifies the reading order of the list. Valid values are `NONE`, `LTR`, `RTL`.",
            ),
            (
                "disableCommenting",
                "Property for enabling or disabling commenting on the list. Valid values are `true`, `false`.",
            ),
            (
                "disableGridEditing",
                "Property for assigning or retrieving grid editing on the list. Valid values are `true`, `false`.",
            ),
            (
                "draftVersionVisibility",
                "Value that specifies the minimum permission required to view minor versions and drafts within the list. Allowed values `Reader`, `Author`, `Approver`. Default `Reader`.",
            ),
            (
                "emailAlias",
                "If e-mail notification is enabled, gets or sets the e-mail address to use tonotify to the owner of an item when an assignment has changed or the item has been updated.",
            ),
            (
                "enableAssignToEmail",
                "Boolean value specifying whether e-mail notification is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableAttachments",
                "Boolean value that specifies whether attachments can be added to items in the list. Valid values are `true`, `false`.",
            ),
            (
                "enableDeployWithDependentList",
                "Boolean value that specifies whether the list can be deployed with a dependent list. Valid values are `true`, `false`.",
            ),
            (
                "enableFolderCreation",
                "Boolean value that specifies whether folders can be created for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableMinorVersions",
                "Boolean value that specifies whether minor versions are enabled when versioning is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "enableModeration",
                "Boolean value that specifies whether Content Approval is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enablePeopleSelector",
                "Enable user selector on event list. Valid values are `true`, `false`.",
            ),
            (
                "enableResourceSelector",
                "Enables resource selector on an event list. Valid values are `true`, `false`.",
            ),
            (
                "enableSchemaCaching",
                "Boolean value specifying whether schema caching is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableSyndication",
                "Boolean value that specifies whether RSS syndication is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableThrottling",
                "Indicates whether throttling for this list is enabled or not. Valid values are `true`, `false`.",
            ),
            (
                "enableVersioning",
                "Boolean value that specifies whether versioning is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "enforceDataValidation",
                "Value that indicates whether certain field properties are enforced when an item is added or updated. Valid values are `true`, `false`.",
            ),
            (
                "excludeFromOfflineClient",
                "Value that indicates whether the list should be downloaded to the client during offline synchronization. Valid values are `true`, `false`.",
            ),
            (
                "fetchPropertyBagForListView",
                "Specifies whether property bag information, as part of the list schema JSON,is retrieved when the list is being rendered on the client. Valid values are `true`, `false`.",
            ),
            (
                "followable",
                "Can a list be followed in an activity feed?. Valid values are `true`, `false`.",
            ),
            (
                "forceCheckout",
                "Boolean value that specifies whether forced checkout is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "forceDefaultContentType",
                "Specifies whether we want to return the default Document root content type. Valid values are `true`, `false`.",
            ),
            (
                "hidden",
                "Boolean value that specifies whether the list is hidden. Valid values are `true`, `false`.",
            ),
            (
                "includedInMyFilesScope",
                "Specifies whether this list is accessible to an app principal that has been granted an OAuth scope that contains the string \"myfiles\" by a case-insensitive comparison when the current user is a site collection administrator of the personal site that contains the list.",
            ),
            (
                "irmEnabled",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) is enabled for the list.",
            ),
            (
                "irmExpire",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) expiration is enabled for the list.",
            ),
            (
                "irmReject",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) rejection is enabled for the list.",
            ),
            (
                "isApplicationList",
                "Indicates whether this list should be treated as a top level navigation object or not.",
            ),
            (
                "listExperienceOptions",
                "Gets or sets the list experience for the list. Allowed values `Auto`, `NewExperience`, `ClassicExperience`. Default `Auto`.",
            ),
            (
                "majorVersionLimit",
                "Gets or sets the maximum number of major versions allowed for an item in a document library that uses version control with major versions only.",
            ),
            (
                "majorWithMinorVersionsLimit",
                "Gets or sets the maximum number of major versions that are allowed for an item in a document library that uses version control with both major and minor versions.",
            ),
            (
                "multipleDataList",
                "Gets or sets a Boolean value that specifies whether the list in a Meeting Workspace sitecontains data for multiple meeting instances within the site",
            ),
            (
                "navigateForFormsPages",
                "Indicates whether to navigate for forms pages or use a modal dialog.",
            ),
            (
                "needUpdateSiteClientTag",
                "A boolean value that determines whether to editing documents in this list should increment the ClientTag for the site. The tag is used to allow clients to cache JS/CSS/resources that are retrieved from the Content DB, including custom CSR templates.",
            ),
            (
                "noCrawl",
                "Gets or sets a Boolean value specifying whether crawling is enabled for the list.",
            ),
            (
                "onQuickLaunch",
                "Gets or sets a Boolean value that specifies whether the list appears on the Quick Launcharea of the home page.",
            ),
            (
                "ordered",
                "Gets or sets a Boolean value that specifies whether the option to allow users to reorderitems in the list is available on the Edit View page for the list.",
            ),
            (
                "parserDisabled",
                "Gets or sets a Boolean value that specifies whether the parser should be disabled.",
            ),
            (
                "readOnlyUI",
                "A boolean value that indicates whether the UI for this list should be presented in a read-only fashion. This will not affect security nor will it actually prevent changes to the list from occurring - it only affects the way the UI is displayed.",
            ),
            (
                "readSecurity",
                "Gets or sets the Read security setting for the list. Valid values are 1 (All users have Read access to all items)|2 (Users have Read access only to items that they create).",
            ),
            (
                "requestAccessEnabled",
                "Gets or sets a Boolean value that specifies whether the option to allow users to requestaccess to the list is available.",
            ),
            (
                "restrictUserUpdates",
                "A boolean value that indicates whether the this list is a restricted one or not The value can't be changed if there are existing items in the list.",
            ),
            (
                "sendToLocationName",
                "Gets or sets a file name to use when copying an item in the list to another document library.",
            ),
            (
                "sendToLocationUrl",
                "Gets or sets a URL to use when copying an item in the list to another document library.",
            ),
            (
                "showUser",
                "Gets or sets a Boolean value that specifies whether names of users are shown in the results of the survey.",
            ),
            (
                "useFormsForDisplay",
                "Indicates whether forms should be considered for display context or not.",
            ),
            (
                "validationFormula",
                "Gets or sets a formula that is evaluated each time that a list item is added or updated.",
            ),
            (
                "validationMessage",
                "Gets or sets the message that is displayed when validation fails for a list item.",
            ),
            (
                "writeSecurity",
                "Gets or sets the Write security setting for the list. Valid values are 1 (All users can modify all items) | 2 (Users can modify only items that they create) | 4 (Users cannot modify any list item).",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Add a list with specific title and baseTemplate in a specific site.",
                "m365 spo list add --title Announcements --baseTemplate Announcements --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Add a list with specific title and baseTemplate in a specific site with content types and versioning enabled and major version limit set.",
                "m365 spo list add --webUrl https://contoso.sharepoint.com/sites/project-x --title Announcements --baseTemplate Announcements --contentTypesEnabled true --enableVersioning true --majorVersionLimit 50",
            ),
        ],
        links: &[
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.list_members.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.list_members.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listtemplatetype.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listtemplatetype.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.draftvisibilitytype.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.draftvisibilitytype.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listexperience.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listexperience.aspx",
            ),
        ],
    },
    Help {
        name: "spo list contenttype add",
        summary: "Adds content type to list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the content type to add to the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Adds a specific existing content type to a list retrieved by id in a specific site.",
                "m365 spo list contenttype add --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id 0x0120",
            ),
            (
                "Adds a specific existing content type to a list retrieved by title in a specific site.",
                "m365 spo list contenttype add --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --id 0x0120",
            ),
            (
                "Adds a specific existing content type to a list retrieved by server relative URL in a specific site.",
                "m365 spo list contenttype add --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'sites/project-x/Documents' --id 0x0120",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list contenttype default set",
        summary: "Sets the default content type for a list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("contentTypeId", "ID of the content type"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Set a content type with a specific id as default a list retrieved by id located in a specific site.",
                "m365 spo list contenttype default set --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --contentTypeId 0x0120",
            ),
            (
                "Set a content type with a specific id as default a list retrieved by title located in a specific site.",
                "m365 spo list contenttype default set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --contentTypeId 0x0120",
            ),
            (
                "Set a content type with a specific id as default a list retrieved by server relative URL located in a specific site.",
                "m365 spo list contenttype default set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'sites/project-x/Documents' --contentTypeId 0x0120",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list contenttype list",
        summary: "Lists content types configured on the list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List all content types configured on a specific list retrieved by id in a specific site.",
                "m365 spo list contenttype list --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "List all content types configured on a specific list retrieved by title in a specific site.",
                "m365 spo list contenttype list --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents",
            ),
            (
                "List all content types configured on a specific list retrieved by server relative URL in a specific site.",
                "m365 spo list contenttype list --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'sites/project-x/Documents'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list contenttype remove",
        summary: "Removes content type from list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the content type to remove from the list."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove content type with a specific id from the list retrieved by id in a specific site.",
                "m365 spo list contenttype remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id 0x010109010053EE7AEB1FC54A41B4D9F66ADBDC312A",
            ),
            (
                "Remove content type with a specific id from the list retrieved by title in a specific site.",
                "m365 spo list contenttype remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --id 0x010109010053EE7AEB1FC54A41B4D9F66ADBDC312A",
            ),
            (
                "Remove content type with a specific id from the list retrieved by server relative URL in a specific site. This will not prompt for confirmation.",
                "m365 spo list contenttype remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'sites/project-x/Documents' --id 0x010109010053EE7AEB1FC54A41B4D9F66ADBDC312A --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list defaultvalue clear",
        summary: "Clears default column values for a specific document library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "fieldName",
                "Internal name of the field to clear over the entire list. Specify either `fieldName` or `folderUrl`, but not both.",
            ),
            (
                "folderUrl",
                "Clear all fields of a specific folder by specifying a server- or site-relative URL. Specify either `fieldName` or `folderUrl`, but not both.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove all default column values from a library",
                "m365 spo list defaultvalue clear --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos",
            ),
            (
                "Clear a field from all folders of a library",
                "m365 spo list defaultvalue clear --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Company",
            ),
            (
                "Clear all fields from a specific folder in a library",
                "m365 spo list defaultvalue clear --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --folderUrl \"/sites/Marketing/Logos/Contoso\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list defaultvalue get",
        summary: "Gets a specific default column value from a specific document library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("fieldName", "Internal name of the field."),
            (
                "folderUrl",
                "Get a specific field of a specific folder by specifying a server- or site-relative URL.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get default column value from the root folder of the list",
                "m365 spo list defaultvalue get --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Company",
            ),
            (
                "Get default column value from a specific folder of the list",
                "m365 spo list defaultvalue get --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Company --folderUrl \"/sites/Marketing/Logos/Contoso\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list defaultvalue list",
        summary: "Retrieves default column values for a specific document library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "folderUrl",
                "Only get default column values from a specific folder.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List all default column values of a list specified by title",
                "m365 spo list defaultvalue list --webUrl https://contoso.sharepoint.com/sites/marketing --listTitle \"Project Documents\"",
            ),
            (
                "List all default column values of a list specified by ID",
                "m365 spo list defaultvalue list --webUrl https://contoso.sharepoint.com/sites/marketing --listId 12345678-90ab-cdef-1234-567890abcdef",
            ),
            (
                "List all default column values of a list specified by server relative URL",
                "m365 spo list defaultvalue list --webUrl https://contoso.sharepoint.com/sites/marketing --listUrl \"/sites/marketing/Project Documents\"",
            ),
            (
                "List all default column values of a specific folder in a list specified by site relative URLs",
                "m365 spo list defaultvalue list --webUrl https://contoso.sharepoint.com/sites/marketing --listUrl \"/Project Documents\" --folderUrl \"/Project Documents/Archive\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list defaultvalue remove",
        summary: "Removes a specific default column value for a specific document library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("fieldName", "Internal name of the field."),
            (
                "folderUrl",
                "Server- or site-relative URL of a specific folder to remove the field from. By default, the root folder of the list is used.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove a default column value from the root folder of the list",
                "m365 spo list defaultvalue remove --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Company",
            ),
            (
                "Remove a column default value from a specific folder using web-relative URL",
                "m365 spo list defaultvalue remove --webUrl https://contoso.sharepoint.com/sites/Marketing --listUrl '/Logos' --fieldName Company --folderUrl '/Logos/Branding'",
            ),
            (
                "Remove a column default value from a specific folder by server relative URL",
                "m365 spo list defaultvalue remove --webUrl https://contoso.sharepoint.com/sites/Marketing --listId fc6d514f-e7da-47d4-b48b-f72e2fb9c82a --fieldName Company --folderUrl '/sites/Marketing/Logos/Branding'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list defaultvalue set",
        summary: "Sets default column values for a specific document library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("fieldName", "Internal name of the field."),
            ("fieldValue", "Default value of the field."),
            (
                "folderUrl",
                "Set the value for a specific folder by specifying a server- or site-relative URL. By default, the root folder of the list is used.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Set a default folder value on the root folder of a list for a text field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Company --fieldValue Contoso",
            ),
            (
                "Set a default folder value for a taxonomy field on a specific folder",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --folderUrl \"/sites/Marketing/Logos/Contoso\" --fieldName Country --fieldValue \"-1;#Belgium|442affc2-7fab-4f33-9590-330403a579c2\"",
            ),
            (
                "Set a default folder value for a multi-taxonomy field on a specific folder",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listUrl /sites/marketing/Logos --folderUrl \"/sites/Marketing/Logos/Contoso\" --fieldName Countries --fieldValue \"-1;#Belgium|442affc2-7fab-4f33-9590-330403a579c2;#-1;#France|14888324-5c48-46db-b748-215cbe24eb4c\"",
            ),
            (
                "Set a default folder value for a date field to today",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listId 3540460d-cb6e-4bc5-8380-bc1844401ee3 --fieldName Published --fieldValue \"[today]\"",
            ),
            (
                "Set a default folder value for a date field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName Published --fieldValue \"2020-05-03T11:00:00Z\"",
            ),
            (
                "Set a default folder value for a choice field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listUrl /sites/marketing/Logos --fieldName FileType --fieldValue Logo",
            ),
            (
                "Set a default folder value for a multi-choice field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle Logos --fieldName FileTypes --fieldValue \"Logo;#Brand\"",
            ),
            (
                "Set a default folder value for a yes/no field using site-relative list URL",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listUrl /Logos --fieldName Active --fieldValue 1",
            ),
            (
                "Set a default folder value for a user field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listId 3540460d-cb6e-4bc5-8380-bc1844401ee3 --fieldName Responsible --fieldValue \"1;#john.doe@contoso.com\"",
            ),
            (
                "Set a default folder value for a multi-user field",
                "m365 spo list defaultvalue set --webUrl https://contoso.sharepoint.com/sites/Marketing --listId 3540460d-cb6e-4bc5-8380-bc1844401ee3 --fieldName Responsible --fieldValue \"1;#john.doe@contoso.com;#2;#adele.vance@contoso.com\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list get",
        summary: "Gets information about the specific list",
        options: &[
            (
                "webUrl",
                "URL of the site where the list to retrieve is located.",
            ),
            ("id", "ID of the list to retrieve information for."),
            ("title", "Title of the list to retrieve information for."),
            ("url", "Server- or site-relative URL of the list."),
            ("default", "Set to retrieve the default list from the site."),
            (
                "properties",
                "Comma-separated list of properties to retrieve from the list. Will retrieve all properties possible from default response, if not specified.",
            ),
            (
                "withPermissions",
                "Set if you want to return associated roles and permissions of the list.",
            ),
        ],
        notes: &[
            "When the `properties` option includes values with a `/`, for example: `ListItemAllFields/Id`, an additional `$expand` query parameter will be included on `ListItemAllFields`.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get the default document library located in the specified site.",
                "m365 spo list get --webUrl https://contoso.sharepoint.com/sites/project-x --default",
            ),
            (
                "Get information about a list with specified ID located in the specified site.",
                "m365 spo list get --id 0cd891ef-afce-4e55-b836-fce03286cccf --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get information about a list with specified title located in the specified site.",
                "m365 spo list get --title Documents --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get information about a list with specified server relative url located in the specified site.",
                "m365 spo list get --url 'sites/project-x/Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get information about a list with specified site-relative URL located in the specified site.",
                "m365 spo list get --url 'Shared Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get information about a list returning the specified list properties.",
                "m365 spo list get --title Documents --webUrl https://contoso.sharepoint.com/sites/project-x --properties \"Title,Id,HasUniqueRoleAssignments,AllowContentTypes\"",
            ),
            (
                "Get information about a list returning the list Id, Title and ServerRelativeUrl properties.",
                "m365 spo list get --title Documents --webUrl https://contoso.sharepoint.com/sites/project-x --properties \"Title,Id,RootFolder/ServerRelativeUrl\"",
            ),
            (
                "Get information about a list along with the roles and permissions.",
                "m365 spo list get --title Documents --webUrl https://contoso.sharepoint.com/sites/project-x --withPermissions",
            ),
        ],
        links: &[(
            "https://msdn.microsoft.com/en-us/library/office/dn531433.aspx#bk_ListEndpoint",
            "https://msdn.microsoft.com/en-us/library/office/dn531433.aspx#bk_ListEndpoint",
        )],
    },
    Help {
        name: "spo list list",
        summary: "Gets all lists within the specified site",
        options: &[
            (
                "webUrl",
                "URL of the site where the lists to retrieve are located.",
            ),
            (
                "properties",
                "Comma-separated list of properties to retrieve. Will retrieve all properties if not specified.",
            ),
            ("filter", "OData filter to use to query the lists with."),
        ],
        notes: &[
            "When the `--properties` option includes values with a `/`, for example: `RootFolder/ServerRelativeUrl`, an additional `$expand` query parameter should be included on `RootFolder`.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Return all lists located in in a specific site.",
                "m365 spo list list --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Return all lists located in in a specific site with specific properties.",
                "m365 spo list list --webUrl https://contoso.sharepoint.com/sites/project-x --properties \"BaseTemplate,ParentWebUrl\"",
            ),
            (
                "Return all lists located in in a specific site with the Id, Title and ServerRelativeUrl properties.",
                "m365 spo list list --webUrl https://contoso.sharepoint.com/sites/project-x --properties \"Id,Title,RootFolder/ServerRelativeUrl\"",
            ),
            (
                "Return all lists located in in a specific site based on the given filter.",
                "m365 spo list list --webUrl https://contoso.sharepoint.com/sites/project-x --filter \"BaseTemplate eq 100\"",
            ),
        ],
        links: &[(
            "https://msdn.microsoft.com/en-us/library/office/dn531433.aspx#bk_ListEndpoint",
            "https://msdn.microsoft.com/en-us/library/office/dn531433.aspx#bk_ListEndpoint",
        )],
    },
    Help {
        name: "spo list remove",
        summary: "Removes the specified list",
        options: &[
            (
                "webUrl",
                "URL of the site where the list to remove is located.",
            ),
            ("id", "The ID of the list to remove."),
            ("title", "Title of the list to remove."),
            (
                "permanent",
                "Permanently delete the list instead of moving it to the recycle bin.",
            ),
            ("force", "Don't prompt for confirming removing the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Remove the list with a specific ID located in a specific site",
                "m365 spo list remove --webUrl https://contoso.sharepoint.com/sites/project-x --id 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Remove the list with a specific title located in a specific site.",
                "m365 spo list remove --webUrl https://contoso.sharepoint.com/sites/project-x --title 'List 1'",
            ),
            (
                "Permanently remove a list specified by ID.",
                "m365 spo list remove --webUrl https://contoso.sharepoint.com/sites/project-x --id 0cd891ef-afce-4e55-b836-fce03286cccf --permanent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list retentionlabel ensure",
        summary: "Sets a default retention label on the specified list or library.",
        options: &[
            ("webUrl", "The URL of the site where the list is located."),
            ("name", "The label name to set on the list."),
            (
                "listTitle",
                "The title of the list on which to set the label.",
            ),
            ("listId", "The ID of the list on which to set the label."),
            (
                "listUrl",
                "Server- or web-relative URL of the list on which to set the label.",
            ),
            (
                "syncToItems",
                "Specify, to set the label on all existing items in the list.",
            ),
        ],
        notes: &[
            "A list retention label is a default label that will be applied to all new items in the list. If you specify `syncToItems`, it is also synced to existing items.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Sets a retention label by name on a given list.",
                "m365 spo list retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'Shared Documents' --name 'Some label'",
            ),
            (
                "Sets a retention label by name and syncs this to all existing items for a given list.",
                "m365 spo list retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'Documents' --name 'Some label' --syncToItems",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list retentionlabel get",
        summary: "Gets the default retention label set on the specified list or library.",
        options: &[
            (
                "webUrl",
                "URL of the site where the list to get the label from is located.",
            ),
            ("listId", "ID of the list to get the label from."),
            ("listTitle", "Title of the list to get the label from."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets retention label set on the list with specified title located in the specified site.",
                "m365 spo list retentionlabel get --listTitle ContosoList --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Gets retention label set on the list with specified id located in the specified site.",
                "m365 spo list retentionlabel get --listId cc27a922-8224-4296-90a5-ebbc54da2e85 --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Gets retention label set on the list with specified server relative url located in the specified site.",
                "m365 spo list retentionlabel get --listUrl 'sites/project-x/Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Gets retention label set on the list with specified site-relative URL located in the specified site.",
                "m365 spo list retentionlabel get --listUrl 'Shared Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list retentionlabel remove",
        summary: "Clears the retention label on the specified list or library.",
        options: &[
            ("webUrl", "The URL of the site where the list is located."),
            (
                "listTitle",
                "The title of the list on which to remove the label.",
            ),
            ("listId", "The ID of the list on which to remove the label."),
            (
                "listUrl",
                "Server- or web-relative URL of the list on which to remove the label.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Clears the retention label on a given list by URL.",
                "m365 spo list retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'Shared Documents'",
            ),
            (
                "Clears the retention label on a given list by title.",
                "m365 spo list retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'Documents'",
            ),
            (
                "Clears the retention label on a given list by ID without confirmation.",
                "m365 spo list retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 'Documents' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list roleassignment add",
        summary: "Adds a role assignment to list permissions",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Relative URL of the list."),
            (
                "principalId",
                "SharePoint ID of principal it may be either user id or group id we want to add permissions to.",
            ),
            ("upn", "The upn/email of user to assign role to."),
            (
                "entraGroupId",
                "ID of the Microsoft Entra group to assign permissions to.",
            ),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to assign permissions to.",
            ),
            ("groupName", "The group name of the SharePoint group."),
            ("roleDefinitionId", "ID of role definition."),
            (
                "roleDefinitionName",
                "The name of a role definition, like 'Contribute', 'Read', etc.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Adds role assignment to a list by title located in a specific site for given principal id and role definition id.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Adds role assignment to list by id located in a specific site for given principal id and role definition id.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listId \"0CD891EF-AFCE-4E55-B836-FCE03286CCCF\" --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Adds role assignment to list by url located in a specific site for given principal id and role definition id.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listUrl \"sites/documents\" --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Adds role assignment to list by title located in a specific site for given upn and role definition id.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --upn \"someaccount@tenant.onmicrosoft.com\" --roleDefinitionId 1073741829",
            ),
            (
                "Adds role assignment to list by title located in a specific site for given group and role definition id.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --groupName \"someGroup\" --roleDefinitionId 1073741829",
            ),
            (
                "Adds role assignment to list by title located in a specific site for given principal id and role definition name.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --principalId 11 --roleDefinitionName \"Full Control\"",
            ),
            (
                "Adds role assignment to list for a Microsoft Entra group specified by display name.",
                "m365 spo list roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --entraGroupName \"Marketing\" --roleDefinitionName \"Full Control\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list roleassignment remove",
        summary: "Removes a role assignment from list permissions",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Relative URL of the list."),
            (
                "principalId",
                "SharePoint ID of principal it may be either user id or group id you want to remove permissions.",
            ),
            ("upn", "The upn/email of user."),
            ("groupName", "The group name of the SharePoint group."),
            ("entraGroupId", "ID of the Microsoft Entra group."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the role assignment.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove roleassignment from list by title based on group name.",
                "m365 spo list roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listTitle \"someList\" --groupName \"saleGroup\"",
            ),
            (
                "Remove roleassignment from list by title based on principal Id.",
                "m365 spo list roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listTitle \"Events\" --principalId 2",
            ),
            (
                "Remove roleassignment from list by url based on principal Id.",
                "m365 spo list roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listUrl '/sites/contoso-sales/lists/Events' --principalId 2",
            ),
            (
                "Remove roleassignment from Microsoft Entra group specified by display name.",
                "m365 spo list roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listUrl '/sites/contoso-sales/lists/Events' --entraGroupName \"Sales Team\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list roleinheritance break",
        summary: "Breaks role inheritance on list or library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "clearExistingPermissions",
                "Flag if used clears all roles from the list.",
            ),
            (
                "force",
                "Do not prompt for confirmation before breaking role inheritance.",
            ),
        ],
        notes: &[
            "By default, when breaking permissions inheritance, the list will retain existing permissions. To remove existing permissions, use the `--clearExistingPermissions` option.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Break inheritance of list by title in a specific site.",
                "m365 spo list roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\"",
            ),
            (
                "Break inheritance of list by id in a specific site.",
                "m365 spo list roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listId \"202b8199-b9de-43fd-9737-7f213f51c991\"",
            ),
            (
                "Break inheritance of list by title located in a specific site and clearing the existing permissions.",
                "m365 spo list roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --clearExistingPermissions",
            ),
            (
                "Break inheritance of list by url in a specific site and clearing the existing permissions.",
                "m365 spo list roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listUrl '/sites/project-x/lists/events' --clearExistingPermissions",
            ),
            (
                "Break inheritance of list by ID located in in a specific site with clearing permissions without prompting for confirmation.",
                "m365 spo list roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listId \"202b8199-b9de-43fd-9737-7f213f51c991\" --clearExistingPermissions --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list roleinheritance reset",
        summary: "Restores role inheritance on list or library",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "force",
                "Do not prompt for confirmation before resetting role inheritance.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Restores role inheritance of a specific list by id in a specific site.",
                "m365 spo list roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Restores role inheritance of a specific list by title in a specific site.",
                "m365 spo list roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle test",
            ),
            (
                "Restores role inheritance of a specific list by url in a specific site.",
                "m365 spo list roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/events'",
            ),
            (
                "Restores role inheritance of list a specific list by title without prompting for confirmation.",
                "m365 spo list roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle test --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list sensitivitylabel ensure",
        summary: "Applies a default sensitivity label to the specified document library",
        options: &[
            (
                "webUrl",
                "The URL of the site where the library is located.",
            ),
            ("name", "The label name to apply to the list."),
            ("id", "The Id of the label to apply to the list."),
            (
                "listTitle",
                "The title of the library on which to apply the label.",
            ),
            (
                "listId",
                "The ID of the library on which to apply the label.",
            ),
            (
                "listUrl",
                "Server- or web-relative URL of the library on which to apply the label.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reaches general availability.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read",
            ),
            ("Delegated", "SharePoint", "AllSites.Write"),
            (
                "Application",
                "Microsoft Graph",
                "InformationProtectionPolicy.Read.All",
            ),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Applies a sensitivity label by id to a document library based on the list title.",
                "m365 spo list sensitivitylabel ensure --webUrl 'https://contoso.sharepoint.com' --listTitle 'Shared Documents' --id '0d39dc11-75ff-4309-8b32-ff94f0e41607'",
            ),
            (
                "Applies a sensitivity label by name to a document library based on the list title.",
                "m365 spo list sensitivitylabel ensure --webUrl 'https://contoso.sharepoint.com' --listTitle 'Shared Documents' --name 'Confidential'",
            ),
            (
                "Applies a sensitivity label by name to a document library based on the list url.",
                "m365 spo list sensitivitylabel ensure --webUrl 'https://contoso.sharepoint.com' --listUrl '/Shared Documents' --name 'Confidential'",
            ),
            (
                "Applies a sensitivity label by name to a document library based on the list id.",
                "m365 spo list sensitivitylabel ensure --webUrl 'https://contoso.sharepoint.com' --listId 'b4cfa0d9-b3d7-49ae-a0f0-f14ffdd005f7' --name 'Confidential'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list sensitivitylabel remove",
        summary: "Clears a default sensitivity label from a document library",
        options: &[
            ("webUrl", "The URL of the site where the list is located."),
            (
                "listTitle",
                "The title of the library on which to remove the label.",
            ),
            (
                "listId",
                "The ID of the library on which to remove the label.",
            ),
            (
                "listUrl",
                "Server- or web-relative URL of the library on which to remove the label.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Removes a sensitivity label from a document library based on the list title.",
                "m365 spo list sensitivitylabel remove --webUrl 'https://contoso.sharepoint.com' --listTitle 'Shared Documents'",
            ),
            (
                "Removes a sensitivity label from a document library based on the list url.",
                "m365 spo list sensitivitylabel remove --webUrl 'https://contoso.sharepoint.com' --listUrl '/Shared Documents'",
            ),
            (
                "Removes a sensitivity label from a document library based on the list id without prompting for confirmation.",
                "m365 spo list sensitivitylabel remove --webUrl 'https://contoso.sharepoint.com' --listId 'b4cfa0d9-b3d7-49ae-a0f0-f14ffdd005f7' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list set",
        summary: "Updates the settings of the specified list",
        options: &[
            ("webUrl", "URL of the site."),
            ("id", "ID of the list."),
            ("title", "Title of the list."),
            ("url", "Relative URL of the list."),
            ("newTitle", "New title for the list."),
            (
                "allowDeletion",
                "Boolean value specifying whether the list can be deleted. Valid values are `true`, `false`.",
            ),
            (
                "allowEveryoneViewItems",
                "Boolean value specifying whether everyone can view documents in the documentlibrary or attachments to items in the list. Valid values are `true`, `false`.",
            ),
            (
                "allowMultiResponses",
                "Boolean value specifying whether users are allowed to give multiple responses to the survey. Valid values are `true`, `false`.",
            ),
            (
                "contentTypesEnabled",
                "Boolean value specifying whether content types are enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "crawlNonDefaultViews",
                "Boolean value specifying whether to crawl non default views. Valid values are `true`, `false`.",
            ),
            (
                "defaultContentApprovalWorkflowId",
                "Value that specifies the default workflow identifier for content approval on the list (GUID).",
            ),
            (
                "defaultDisplayFormUrl",
                "Value that specifies the location of the default display form for the list.",
            ),
            (
                "defaultEditFormUrl",
                "Value that specifies the URL of the edit form to use for list items in the list.",
            ),
            ("description", "The description for the list."),
            (
                "direction",
                "Value that specifies the reading order of the list. Valid values are `NONE`, `LTR`, `RTL`.",
            ),
            (
                "disableCommenting",
                "Property for enabling or disabling commenting on the list. Valid values are `true`, `false`.",
            ),
            (
                "disableGridEditing",
                "Property for assigning or retrieving grid editing on the list. Valid values are `true`, `false`.",
            ),
            (
                "draftVersionVisibility",
                "Value that specifies the minimum permission required to view minor versions and drafts within the list. Allowed values `Reader`, `Author`, `Approver`.",
            ),
            (
                "emailAlias",
                "If e-mail notification is enabled, gets or sets the e-mail address to use tonotify to the owner of an item when an assignment has changed or the item has been updated.",
            ),
            (
                "enableAssignToEmail",
                "Boolean value specifying whether e-mail notification is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableAttachments",
                "Boolean value that specifies whether attachments can be added to items in the list. Valid values are `true`, `false`.",
            ),
            (
                "enableDeployWithDependentList",
                "Boolean value that specifies whether the list can be deployed with a dependent list. Valid values are `true`, `false`.",
            ),
            (
                "enableFolderCreation",
                "Boolean value that specifies whether folders can be created for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableMinorVersions",
                "Boolean value that specifies whether minor versions are enabled when versioning is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "enableModeration",
                "Boolean value that specifies whether Content Approval is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enablePeopleSelector",
                "Enable user selector on event list. Valid values are `true`, `false`.",
            ),
            (
                "enableResourceSelector",
                "Enables resource selector on an event list. Valid values are `true`, `false`.",
            ),
            (
                "enableSchemaCaching",
                "Boolean value specifying whether schema caching is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableSyndication",
                "Boolean value that specifies whether RSS syndication is enabled for the list. Valid values are `true`, `false`.",
            ),
            (
                "enableThrottling",
                "Indicates whether throttling for this list is enabled or not. Valid values are `true`, `false`.",
            ),
            (
                "enableVersioning",
                "Boolean value that specifies whether versioning is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "enforceDataValidation",
                "Value that indicates whether certain field properties are enforced when an item is added or updated. Valid values are `true`, `false`.",
            ),
            (
                "excludeFromOfflineClient",
                "Value that indicates whether the list should be downloaded to the client during offline synchronization. Valid values are `true`, `false`.",
            ),
            (
                "fetchPropertyBagForListView",
                "Specifies whether property bag information, as part of the list schema JSON,is retrieved when the list is being rendered on the client. Valid values are `true`, `false`.",
            ),
            (
                "followable",
                "Can a list be followed in an activity feed?. Valid values are `true`, `false`.",
            ),
            (
                "forceCheckout",
                "Boolean value that specifies whether forced checkout is enabled for the document library. Valid values are `true`, `false`.",
            ),
            (
                "forceDefaultContentType",
                "Specifies whether we want to return the default Document root content type. Valid values are `true`, `false`.",
            ),
            (
                "hidden",
                "Boolean value that specifies whether the list is hidden. Valid values are `true`, `false`.",
            ),
            (
                "includedInMyFilesScope",
                "Specifies whether this list is accessible to an app principal that has been granted an OAuth scope that contains the string \"myfiles\" by a case-insensitive comparison when the current user is a site collection administrator of the personal site that contains the list.",
            ),
            (
                "irmEnabled",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) is enabled for the list.",
            ),
            (
                "irmExpire",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) expiration is enabled for the list.",
            ),
            (
                "irmReject",
                "Gets or sets a Boolean value that specifies whether Information Rights Management (IRM) rejection is enabled for the list.",
            ),
            (
                "isApplicationList",
                "Indicates whether this list should be treated as a top level navigation object or not.",
            ),
            (
                "listExperienceOptions",
                "Gets or sets the list experience for the list. Allowed values `Auto`, `NewExperience`, `ClassicExperience`.",
            ),
            (
                "majorVersionLimit",
                "Gets or sets the maximum number of major versions allowed for an item in a document library that uses version control with major versions only. When specified, versioning is automatically enabled.",
            ),
            (
                "majorWithMinorVersionsLimit",
                "Gets or sets the maximum number of major versions that are allowed for an item in a document library that uses version control with both major and minor versions.",
            ),
            (
                "multipleDataList",
                "Gets or sets a Boolean value that specifies whether the list in a Meeting Workspace sitecontains data for multiple meeting instances within the site.",
            ),
            (
                "navigateForFormsPages",
                "Indicates whether to navigate for forms pages or use a modal dialog.",
            ),
            (
                "needUpdateSiteClientTag",
                "A boolean value that determines whether to editing documents in this list should increment the ClientTag for the site. The tag is used to allow clients to cache JS/CSS/resources that are retrieved from the Content DB, including custom CSR templates.",
            ),
            (
                "noCrawl",
                "Gets or sets a Boolean value specifying whether crawling is enabled for the list.",
            ),
            (
                "onQuickLaunch",
                "Gets or sets a Boolean value that specifies whether the list appears on the Quick Launch area of the home page.",
            ),
            (
                "ordered",
                "Gets or sets a Boolean value that specifies whether the option to allow users to reorder items in the list is available on the Edit View page for the list.",
            ),
            (
                "parserDisabled",
                "Gets or sets a Boolean value that specifies whether the parser should be disabled.",
            ),
            (
                "readOnlyUI",
                "A boolean value that indicates whether the UI for this list should be presented in a read-only fashion. This will not affect security nor will it actually prevent changes to the list from occurring - it only affects the way the UI is displayed.",
            ),
            (
                "readSecurity",
                "Gets or sets the Read security setting for the list. Valid values are 1 (All users have Read access to all items)|2 (Users have Read access only to items that they create).",
            ),
            (
                "requestAccessEnabled",
                "Gets or sets a Boolean value that specifies whether the option to allow users to request access to the list is available.",
            ),
            (
                "restrictUserUpdates",
                "A boolean value that indicates whether the this list is a restricted one or not The value can't be changed if there are existing items in the list.",
            ),
            (
                "sendToLocationName",
                "Gets or sets a file name to use when copying an item in the list to another document library.",
            ),
            (
                "sendToLocationUrl",
                "Gets or sets a URL to use when copying an item in the list to another document library.",
            ),
            (
                "showUser",
                "Gets or sets a Boolean value that specifies whether names of users are shown in the results of the survey.",
            ),
            (
                "templateFeatureId",
                "The globally unique identifier (GUID) of a template feature that is associated with the list.",
            ),
            (
                "useFormsForDisplay",
                "Indicates whether forms should be considered for display context or not.",
            ),
            (
                "validationFormula",
                "Gets or sets a formula that is evaluated each time that a list item is added or updated.",
            ),
            (
                "validationMessage",
                "Gets or sets the message that is displayed when validation fails for a list item.",
            ),
            (
                "versionAutoExpireTrim",
                "Boolean that indicates whether the automatic version trim should be enabled. Cannot be used together with `versionExpireAfterDays` unless set to `false`. Valid values are `true`, `false`.",
            ),
            (
                "versionExpireAfterDays",
                "Number that defines how many days a version should be removed after. When using this option, be sure to also set `majorVersionLimit`. Cannot be used together with `versionAutoExpireTrim` set to `true`.",
            ),
            (
                "writeSecurity",
                "Gets or sets the Write security setting for the list. Valid values are 1 (All users can modify all items) | 2 (Users can modify only items that they create) | 4 (Users cannot modify any list item).",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Manage"),
            ("Application", "SharePoint", "Sites.Manage.All"),
        ],
        examples: &[
            (
                "Update the contentTypesEnabled property of the list with id located in a specific site.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --id 3EA5A977-315E-4E25-8B0F-E4F949BF6B8F --contentTypesEnabled true",
            ),
            (
                "Enable versioning and set the number of major versions to keep on the list with id located in a specific site.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --id 3EA5A977-315E-4E25-8B0F-E4F949BF6B8F --enableVersioning true --majorVersionLimit 50",
            ),
            (
                "Enable content types and versioning in the list with id located in a specific site.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --id 3EA5A977-315E-4E25-8B0F-E4F949BF6B8F --contentTypesEnabled true --enableVersioning true --majorVersionLimit 50 --majorWithMinorVersionsLimit 100",
            ),
            (
                "Update the title of a list retrieved by it's original title.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --title Documents --newTitle 'Different Title'",
            ),
            (
                "Enable automatic version trimming on a list.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --id 3EA5A977-315E-4E25-8B0F-E4F949BF6B8F --versionAutoExpireTrim true",
            ),
            (
                "Set versions to expire after 30 days and keep a maximum of 500 major versions.",
                "m365 spo list set --webUrl https://contoso.sharepoint.com/sites/project-x --id 3EA5A977-315E-4E25-8B0F-E4F949BF6B8F --versionExpireAfterDays 30 --majorVersionLimit 500",
            ),
        ],
        links: &[
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.list_members.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.list_members.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listtemplatetype.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listtemplatetype.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.draftvisibilitytype.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.draftvisibilitytype.aspx",
            ),
            (
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listexperience.aspx",
                "https://msdn.microsoft.com/en-us/library/microsoft.sharepoint.client.listexperience.aspx",
            ),
        ],
    },
    Help {
        name: "spo list sitescript get",
        summary: "Extracts a site script from a SharePoint list",
        options: &[
            (
                "webUrl",
                "URL of the site where the list to extract the site script from is located.",
            ),
            ("listId", "ID of the list to extract the site script from."),
            (
                "listTitle",
                "Title of the list to extract the site script from.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Extract a site script from an existing SharePoint list with specified title located in the specified site.",
                "m365 spo list sitescript get --listTitle ContosoList --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Extract a site script from an existing SharePoint list with specified id located in the specified site.",
                "m365 spo list sitescript get --listId cc27a922-8224-4296-90a5-ebbc54da2e85 --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Extract a site script from an existing SharePoint list with specified server relative url located in the specified site.",
                "m365 spo list sitescript get --listUrl 'sites/project-x/Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Extract a site script from an existing SharePoint list with specified site-relative URL located in the specified site.",
                "m365 spo list sitescript get --listUrl 'Shared Documents' --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view add",
        summary: "Adds a new view to a SharePoint list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            (
                "listId",
                "ID of the list to which the view should be added.",
            ),
            (
                "listTitle",
                "Title of the list to which the view should be added.",
            ),
            (
                "listUrl",
                "Relative URL of the list to which the view should be added.",
            ),
            ("title", "Title of the view to be created for the list."),
            (
                "type",
                "Type of the view. Possible values are: `list`, `calendar`, `gallery`, `kanban`. Defaults to `list`.",
            ),
            (
                "fields",
                "Comma-separated list of case-sensitive internal names of the fields to add to the view. Optional when `type` is set to `calendar`.",
            ),
            (
                "viewQuery",
                "XML representation of the list query for the underlying view.",
            ),
            (
                "calendarStartDateField",
                "Internal name of the field that contains the start date of the calendar event. Required when `type` is set to `calendar`.",
            ),
            (
                "calendarEndDateField",
                "Internal name of the field that contains the end date of the calendar event. Required when `type` is set to `calendar`.",
            ),
            (
                "calendarTitleField",
                "Internal name of the field that contains the title of the calendar event. Required when `type` is set to `calendar`.",
            ),
            (
                "calendarSubTitleField",
                "Internal name of the field that contains the subtitle of the calendar event.",
            ),
            (
                "calendarDefaultLayout",
                "Default layout of the calendar view. Possible values are: `month`, `week`, `workWeek`, `day`. Defaults to `month`.",
            ),
            (
                "kanbanBucketField",
                "Internal name of the field that contains the bucket for the Kanban board view. Required when `type` is set to `kanban`.",
            ),
            (
                "personal",
                "View will be created as personal view, if specified.",
            ),
            ("default", "View will be set as default view, if specified."),
            (
                "paged",
                "View supports paging. Valid values are `true`, `false`. Defaults to `true`.",
            ),
            (
                "rowLimit",
                "Sets the number of items to display for the view. Default value is 30.",
            ),
            (
                "customFormatter",
                "JSON that formats the view (the view formatting definition).",
            ),
        ],
        notes: &[
            "We recommend using the `paged` option. Paging is enabled by default: the view shows items page by page, up to the specified `rowLimit` per page. To disable paging, set `paged` to `false`; in a non-paged view, the `rowLimit` is absolute and there is no link to see more items.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add a list view called to a list with specific title.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Customers\" --title \"All customers\" --fields \"LinkTitle,Country,Sector,Country,Address,Contact\"",
            ),
            (
                "Add a gallery view as default view to a list with a specific URL.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listUrl \"/Lists/Customers\" --title \"All customers\" --type gallery --fields \"LinkTitle,Country,Sector,Country,Address,Contact\" --default",
            ),
            (
                "Add a view with defined filter and sorting.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Customers\" --title \"Transport customers\" --fields \"LinkTitle,Country,Country,Address,Contact\" --viewQuery \"<OrderBy><FieldRef Name='LinkTitle' Ascending='TRUE' /></OrderBy><Where><Eq><FieldRef Name='Sector' /><Value Type='Text'>Transportation</Value></Eq></Where>\"",
            ),
            (
                "Add a gallery view as personal view.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Customers\" --title \"All customers\" --type gallery --fields \"LinkTitle,Country,Sector,Country,Address,Contact\" --personal",
            ),
            (
                "Add a calendar view with month layout.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Events\" --title \"All events\" --type calendar --fields \"EventType,InternalExternal\" --calendarStartDateField EventStartDate --calendarEndDateField EventEndDate --calendarTitleField LinkTitle",
            ),
            (
                "Add a calendar view with week layout and subtitle.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Events\" --title \"All events\" --type calendar --fields \"EventType,InternalExternal\" --calendarStartDateField EventStartDate --calendarEndDateField EventEndDate --calendarTitleField LinkTitle --calendarSubTitleField Location --calendarDefaultLayout week",
            ),
            (
                "Add a Kanban board view.",
                "m365 spo list view add --webUrl https://contoso.sharepoint.com/sites/Sales --listTitle \"Tasks\" --title \"All tasks\" --type kanban --fields \"Title,AssignedTo\" --kanbanBucketField Status",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view field add",
        summary: "Adds the specified field to list view",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list where the view is located."),
            ("listTitle", "Title of the list where the view is located."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("viewId", "ID of the view to update."),
            ("viewTitle", "Title of the view to update."),
            ("id", "ID of the field to add."),
            (
                "title",
                "The case-sensitive internal name or display name of the field to add.",
            ),
            (
                "position",
                "The zero-based index of the position for the field.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add field with ID to view with ID of the list with ID located in a specific site.",
                "m365 spo list view field add --webUrl https://contoso.sharepoint.com/sites/project-x --listId 1f187321-f086-4d3d-8523-517e94cc9df9 --viewId 3d760127-982c-405e-9c93-e1f76e1a1110 --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce",
            ),
            (
                "Add field with title to view with title of the list with title Documents located in a specific site.",
                "m365 spo list view field add --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --viewTitle 'All Documents' --title 'Custom field'",
            ),
            (
                "Add field with title at the position to view with title of the list with url located in a specific site.",
                "m365 spo list view field add --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/Events' --viewTitle 'My Events' --title 'Custom field' --position 0",
            ),
            (
                "Add field with title to view with title of the list with site-relative URL located in a specific site.",
                "m365 spo list view field add --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'Shared Documents' --viewTitle 'All Documents' --title 'Custom field'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view field remove",
        summary: "Removes the specified field from list view",
        options: &[
            ("webUrl", "URL of the site where the list is located"),
            ("listId", "ID of the list where the view is located."),
            ("listTitle", "Title of the list where the view is located."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("viewId", "ID of the view to update."),
            ("viewTitle", "Title of the view to update."),
            (
                "id",
                "ID of the field to remove. Specify id or title but not both",
            ),
            (
                "title",
                "The case-sensitive internal name or display name of the field to remove.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove field with ID 330f29c5-5c4c-465f-9f4b-7903020ae1ce from view with ID 3d760127-982c-405e-9c93-e1f76e1a1110 from the list with ID 1f187321-f086-4d3d-8523-517e94cc9df9 located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 1f187321-f086-4d3d-8523-517e94cc9df9 --viewId 3d760127-982c-405e-9c93-e1f76e1a1110 --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce",
            ),
            (
                "Remove field with title Custom field from view with title Custom view from the list with title Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field remove --webUrl https://contoso.sharepoint.com/sites/project-x --title 'Custom field' --listTitle Documents --viewTitle 'Custom view'",
            ),
            (
                "Remove field with title Custom field from view with title Custom view from the list with url /sites/project-x/lists/Events located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field remove --webUrl https://contoso.sharepoint.com/sites/project-x --title 'Custom field' --listUrl '/sites/project-x/lists/Events' --viewTitle 'Custom view'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view field set",
        summary: "Updates existing column in an existing view (eg. move to a specific position)",
        options: &[
            ("webUrl", "URL of the site where the list is located"),
            ("listId", "ID of the list where the view is located."),
            ("listTitle", "Title of the list where the view is located."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("viewId", "ID of the view to update."),
            ("viewTitle", "Title of the view to update."),
            ("id", "ID of the field to update."),
            (
                "title",
                "The case-sensitive internal name or display name of the field to update.",
            ),
            (
                "position",
                "The zero-based index of the position to which to move the field",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Move field with ID 330f29c5-5c4c-465f-9f4b-7903020ae1ce to the front in view with ID 3d760127-982c-405e-9c93-e1f76e1a1110 of list with ID 1f187321-f086-4d3d-8523-517e94cc9df9 located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field set --webUrl https://contoso.sharepoint.com/sites/project-x --listId 1f187321-f086-4d3d-8523-517e94cc9df9 --viewId 3d760127-982c-405e-9c93-e1f76e1a1110 --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce --position 0",
            ),
            (
                "Move field with title Custom field to position 1 in view with title All Documents of the list with title Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --viewTitle 'All Documents' --title 'Custom field' --position 1",
            ),
            (
                "Move field with title Custom field to position 1 in view with title All Documents of the list with url /sites/project-x/lists/Events located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/Events' --viewTitle 'All Documents' --title 'Custom field' --position 1",
            ),
            (
                "Move field with title Custom field to position 1 in view with title All Documents of the list with site-relative URL /Shared Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view field set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/Shared Documents' --viewTitle 'All Documents' --title 'Custom field' --position 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view get",
        summary: "Gets information about specific list view",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list where the view is located."),
            ("listTitle", "Title of the list where the view is located."),
            (
                "listUrl",
                "Server- or web-relative URL of the list where the view is located.",
            ),
            ("id", "ID of the view to get."),
            ("title", "Title of the view to get."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets a list view by name from a list located in the specified site.",
                "m365 spo list view get --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --title 'All Items'",
            ),
            (
                "Gets a list view by ID from a list located in the specified site.",
                "m365 spo list view get --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl 'Lists/My List' --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce",
            ),
            (
                "Gets a list view by name from a list located in the specified site. Retrieve the list by its ID",
                "m365 spo list view get --webUrl https://contoso.sharepoint.com/sites/project-x --listId 330f29c5-5c4c-465f-9f4b-7903020ae1c1 --title 'All Items'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view list",
        summary: "Lists views configured on the specified list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            (
                "listId",
                "ID of the list for which to list configured views.",
            ),
            (
                "listTitle",
                "Title of the list for which to list configured views.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "List all views for a list by title.",
                "m365 spo list view list --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents",
            ),
            (
                "List all views for a list by ID.",
                "m365 spo list view list --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "List all views for a list by URL.",
                "m365 spo list view list --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/Events'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view remove",
        summary: "Deletes the specified view from the list",
        options: &[
            (
                "webUrl",
                "URL of the site where the list to remove the view from is located",
            ),
            (
                "listId",
                "ID of the list from which the view should be removed.",
            ),
            (
                "listTitle",
                "Title of the list from which the view should be removed.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the view to remove."),
            ("title", "Title of the view to remove."),
            ("force", "Don't prompt for confirming removing the view"),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove view from the list by ID 0cd891ef-afce-4e55-b836-fce03286cccf located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id cc27a922-8224-4296-90a5-ebbc54da2e81",
            ),
            (
                "Remove view with ID cc27a922-8224-4296-90a5-ebbc54da2e81 from the list with title Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --id cc27a922-8224-4296-90a5-ebbc54da2e81",
            ),
            (
                "Remove view with title MyView from a list with title Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --title MyView",
            ),
            (
                "Remove view with title MyView from a list with url /sites/project-x/lists/Events located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo list view remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/Events' --title MyView",
            ),
            (
                "Remove view with ID cc27a922-8224-4296-90a5-ebbc54da2e81 from a list with title Documents located in site https://contoso.sharepoint.com/sites/project-x without being asked for confirmation",
                "m365 spo list view remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --id cc27a922-8224-4296-90a5-ebbc54da2e81 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list view set",
        summary: "Updates existing list view",
        options: &[
            ("webUrl", "URL of the site where the list is located"),
            ("listId", "ID of the list where the view is located."),
            ("listTitle", "Title of the list where the view is located."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the view to update."),
            ("title", "Title of the view to update."),
        ],
        notes: &[
            "Specify properties to update using their names, eg. `--Title 'New Title' --JSLink jslink.js`",
            "When updating list formatting, the value of the CustomFormatter property must be XML-escaped, eg. `<` instead of `<`.",
            "When updating list view formatting for a view with the `--CustomFormatter` option, it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update the title of the list view specified by its name",
                "m365 spo list view set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --title 'All items' --Title 'All events'",
            ),
            (
                "Update the title of the list view specified by its ID",
                "m365 spo list view set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/lists/Events' --id 330f29c5-5c4c-465f-9f4b-7903020ae1ce --Title 'All events'",
            ),
            (
                "Update view formatting of the specified list view",
                "m365 spo list view set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --title 'All items' --CustomFormatter '{\\\"schema\\\":\\\"https://developer.microsoft.com/json-schemas/sp/view-formatting.schema.json\\\",\\\"additionalRowClass\\\": \\\"=if([$DueDate] &lt;= @now, ''sp-field-severity--severeWarning'', '''')\\\"}'",
            ),
            (
                "",
                "m365 spo list view set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'My List' --title 'All items' --CustomFormatter \"{\\\"schema\\\":\\\"https://developer.microsoft.com/json-schemas/sp/view-formatting.schema.json\\\",\\\"additionalRowClass\\\": \\\"=if([$DueDate] &lt;= @now, 'sp-field-severity--severeWarning', '')\\\"}\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list webhook add",
        summary: "Adds a new webhook to the specified list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("notificationUrl", "The notification URL."),
            (
                "expirationDateTime",
                "The expiration date. Will be set to max (6 months from today) if not provided.",
            ),
            (
                "clientState",
                "A client state information that will be passed through notifications.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add a web hook to the list retrieved by Title located in a specific site with a specific notification url and the default expiration date",
                "m365 spo list webhook add --webUrl https://contoso.sharepoint.com/sites/ninja --listTitle Documents --notificationUrl https://contoso-funcions.azurewebsites.net/webhook",
            ),
            (
                "Add a web hook to the list retrieved by URL located in a specific site with a specific notification url and a specific expiration date",
                "m365 spo list webhook add --webUrl https://contoso.sharepoint.com/sites/ninja --listUrl '/sites/ninja/Documents' --notificationUrl https://contoso-funcions.azurewebsites.net/webhook --expirationDateTime 2019-01-21",
            ),
            (
                "Add a web hook to the list retrieved by ID located in a specific site with a specific notification url, a specific expiration date and a client state",
                "m365 spo list webhook add --webUrl https://contoso.sharepoint.com/sites/ninja --listId '3d6aefa0-f438-4789-b0cd-6e865f5d65b5' --notificationUrl https://contoso-funcions.azurewebsites.net/webhook --expirationDateTime '2019-03-02T18:15' --clientState \"Hello State!\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list webhook get",
        summary: "Gets information about the specific webhook",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the webhook."),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing webhook, you will get a `404 - \"404 FILE NOT FOUND\"` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Return information about a specific webhook which belongs to a list retrieved by ID in a specific site",
                "m365 spo list webhook get --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id cc27a922-8224-4296-90a5-ebbc54da2e85",
            ),
            (
                "Return information about a specific webhook which belongs to a list retrieved by Title in a specific site",
                "m365 spo list webhook get --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents --id cc27a922-8224-4296-90a5-ebbc54da2e85",
            ),
            (
                "Return information about a specific webhook which belongs to a list retrieved by URL in a specific site",
                "m365 spo list webhook get --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/Documents' --id cc27a922-8224-4296-90a5-ebbc54da2e85",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list webhook list",
        summary: "Lists all webhooks for the specified list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "List all webhooks for a list with a specific ID in a specific site",
                "m365 spo list webhook list --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "List all webhooks for a list with a specific title in a specific site",
                "m365 spo list webhook list --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle Documents",
            ),
            (
                "List all webhooks for a list with a specific URL in a specific site",
                "m365 spo list webhook list --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/Documents'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list webhook remove",
        summary: "Removes the specified webhook from the list",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the webhook."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "If the specified id doesn't refer to an existing webhook, you will get a `404 - \"404 FILE NOT FOUND\"` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove webhook with a specific ID from a list retrieved by ID in a specific site",
                "m365 spo list webhook remove --webUrl https://contoso.sharepoint.com/sites/ninja --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id cc27a922-8224-4296-90a5-ebbc54da2e81",
            ),
            (
                "Remove webhook with a specific ID from a list retrieved by Title in a specific site",
                "m365 spo list webhook remove --webUrl https://contoso.sharepoint.com/sites/ninja --listTitle Documents --id cc27a922-8224-4296-90a5-ebbc54da2e81",
            ),
            (
                "Remove webhook with a specific ID from a list retrieved by URL in a specific site without being asked for confirmation",
                "m365 spo list webhook remove --webUrl https://contoso.sharepoint.com/sites/ninja --listUrl '/sites/ninja/Documents' --id cc27a922-8224-4296-90a5-ebbc54da2e81 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo list webhook set",
        summary: "Updates the specified webhook",
        options: &[
            ("webUrl", "URL of the site where the list is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("id", "ID of the webhook to update."),
            ("notificationUrl", "The new notification url."),
            (
                "expirationDateTime",
                "The new expiration date. Note: Expiration Time cannot be more than 6 months in future",
            ),
            (
                "clientState",
                "A client state information that will be passed through notifications.",
            ),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing webhook, you will get a `404 - \"404 FILE NOT FOUND\"` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update the notification url of a webhook with a specific ID attached to a list with a specific ID in a specific site to a specific URL",
                "m365 spo list webhook set --webUrl https://contoso.sharepoint.com/sites/ninja --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id cc27a922-8224-4296-90a5-ebbc54da2e81 --notificationUrl https://contoso-functions.azurewebsites.net/webhook",
            ),
            (
                "Update the expiration date of a webhook with a specific ID attached to a list with a specific title in a specific site to a specfic date",
                "m365 spo list webhook set --webUrl https://contoso.sharepoint.com/sites/ninja --listTitle Documents --id cc27a922-8224-4296-90a5-ebbc54da2e81 --expirationDateTime 2018-10-09T18:15",
            ),
            (
                "Update the notification url and clientState of a webhook with a specific ID attached to a list with a specific URL in a specific site to a specific URL and the expiration date to a specific date",
                "m365 spo list webhook set --webUrl https://contoso.sharepoint.com/sites/ninja --listUrl '/sites/ninja/Documents' --id cc27a922-8224-4296-90a5-ebbc54da2e81 --notificationUrl https://contoso-functions.azurewebsites.net/webhook --expirationDateTime 2019-03-02 --clientState 'My client state'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem add",
        summary: "Creates a list item in the specified list",
        options: &[
            ("webUrl", "URL of the site where the item should be added"),
            ("listId", "ID of the list where the item should be added."),
            (
                "listTitle",
                "Title of the list where the item should be added.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "contentType",
                "The name or the ID of the content type to associate with the new item",
            ),
            (
                "folder",
                "The list-relative URL of the folder where the item should be created",
            ),
        ],
        notes: &[
            "When creating a list item with a DateTime field, use the timezone and the format that the site expects, based on its regional settings. Alternatively, a format which works on all regions is the following: `yyyy-MM-dd HH:mm:ss`. However, you should use the local timezone in all situations. UTC date/time or ISO 8601 formatted date/time is not supported.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add an item with Title Demo Item and content type name Item to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --contentType Item --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Item\"",
            ),
            (
                "Add an item with Title Demo Multi Managed Metadata Field and a single-select metadata field named SingleMetadataField to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Single Managed Metadata Field\" --SingleMetadataField \"TermLabel1|fa2f6bfd-1fad-4d18-9c89-289fe6941377;\"",
            ),
            (
                "Add an item with Title Demo Multi Managed Metadata Field and a multi-select metadata field named MultiMetadataField to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Multi Managed Metadata Field\" --MultiMetadataField \"TermLabel1|cf8c72a1-0207-40ee-aebd-fca67d20bc8a;TermLabel2|e5cc320f-8b65-4882-afd5-f24d88d52b75;\"",
            ),
            (
                "Add an item with Title Demo Single Person Field and a single-select people field named SinglePeopleField to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Single Person Field\" --SinglePeopleField \"[{'Key':'i:0#.f|membership|markh@conotoso.com'}]\"",
            ),
            (
                "Add an item with Title Demo Multi Person Field and a multi-select people field named MultiPeopleField to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Multi Person Field\" --MultiPeopleField \"[{'Key':'i:0#.f|membership|markh@conotoso.com'},{'Key':'i:0#.f|membership|john.doe@conotoso.com'}]\"",
            ),
            (
                "Add an item with Title Demo Hyperlink Field and a hyperlink field named CustomHyperlink to list with title Demo List in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Hyperlink Field\" --CustomHyperlink \"https://www.bing.com, Bing\"",
            ),
            (
                "Add an item with a specific title to a list retrieved by server-relative URL in a specific site",
                "m365 spo listitem add --contentType Item --listUrl /sites/project-x/Documents --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Item\"",
            ),
            (
                "Add an item with a specific Title and multi-choice value",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo multi-choice Field\" --MultiChoiceField \"Choice 1;#Choice 2;#Choice 3\"",
            ),
            (
                "Add an item with a specific Title and DateTime value",
                "m365 spo listitem add --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo DateTime Field\" --SomeDateTimeField \"2023-01-16 15:30:00\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem attachment add",
        summary: "Adds an attachment to a list item",
        options: &[
            ("webUrl", "URL of the site where the list item is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "The ID of the list item."),
            (
                "filePath",
                "Local path to the file that will be added as an attachment to the list item.",
            ),
            (
                "fileName",
                "Name for the file. If no name is provided, the name from `filePath` will be utilized.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Add a new attachment to a list item from a local file by using list title",
                "m365 spo listitem attachment add --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"DemoList\" --listItemId 147 --filePath \"C:/Reports/File1.jpg\"",
            ),
            (
                "Add a new attachment to a list item from a local file by using list URL with a different filename",
                "m365 spo listitem attachment add --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/Lists/DemoList\" --listItemId 147 --filePath \"C:/Reports/File1.jpg\" --fileName \"File2\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem attachment get",
        summary: "Gets an attachment from a list item",
        options: &[
            ("webUrl", "URL of the site where the list item is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "The ID of the list item."),
            ("fileName", "Name of the file to get."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Get an attachment from a list item by using list title.",
                "m365 spo listitem attachment get --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 147 --fileName \"File1.jpg\"",
            ),
            (
                "Get an attachment from a list item by using list URL.",
                "m365 spo listitem attachment get --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/Lists/DemoList\" --listItemId 147 --fileName \"File1.jpg\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem attachment list",
        summary: "Gets the attachments associated to a list item",
        options: &[
            (
                "webUrl",
                "URL of the site from which the item should be retrieved.",
            ),
            (
                "listId",
                "ID of the list where the item should be retrieved.",
            ),
            (
                "listTitle",
                "Title of the list where the item should be retrieved.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "ID of the list item."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Gets the attachments from list item with the specified listItemId in list with the specified title in the specified site.",
                "m365 spo listitem attachment list --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 147",
            ),
            (
                "Gets the attachments from list item with the specified listItemId in list with the specified id in the specified site.",
                "m365 spo listitem attachment list --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 147",
            ),
            (
                "Gets the attachments from a specific list item in a specific list obtained by server-relative URL in a specific site.",
                "m365 spo listitem attachment list --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/Documents --listItemId 147",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem attachment remove",
        summary: "Removes an attachment from a list item",
        options: &[
            ("webUrl", "URL of the site where the list item is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "The ID of the list item."),
            ("fileName", "Name of the file to remove."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Remove an attachment from a list item using list tile.",
                "m365 spo listitem attachment remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 147 --fileName \"File1.jpg\"",
            ),
            (
                "Remove an attachment from a list item using list URL.",
                "m365 spo listitem attachment remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/Lists/DemoList\" --listItemId 147 --fileName \"File1.jpg\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem attachment set",
        summary: "Updates an attachment from a list item",
        options: &[
            ("webUrl", "URL of the site where the list item is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "The ID of the list item."),
            (
                "filePath",
                "Local path used for updating the attachment contents.",
            ),
            ("fileName", "Name of the file to update."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update an attachment from a list item by using list title",
                "m365 spo listitem attachment set --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 147 --fileName \"File1.jpg\" --filePath \"C:/Reports/File2.jpg\"",
            ),
            (
                "Update an attachment from a list item by using list URL",
                "m365 spo listitem attachment set --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/Lists/DemoList\" --listItemId 147 --fileName \"File1.jpg\" --filePath \"C:/Reports/File2.jpg\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem batch add",
        summary: "Creates list items in a batch",
        options: &[
            ("webUrl", "URL of the site."),
            (
                "filePath",
                "The absolute or relative path to a flat file containing the list items.",
            ),
            (
                "csvContent",
                "A string content in CSV-format containing the list items.",
            ),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[
            "A sample CSV can be found below. The first line of the CSV-file should contain the internal column names that you wish to set.",
            "    ContentType,Title,SingleChoiceField,MultiChoiceField,SingleMetadataField,MultiMetadataField,SinglePeopleField,MultiPeopleField,CustomHyperlink,NumberField,DateTimeField\n    Item,Title A,Choice 1,Choice 1;#Choice 2,Engineering|4a3cc5f3-a4a6-433e-a07a-746978ff1760;,Engineering|4a3cc5f3-a4a6-433e-a07a-746978ff1760;Finance|f994a4ac-cf34-448e-a22c-2b35fd9bbffa;,[{'Key':'i:0#.f|membership|markh@contoso.com'}],\"[{'Key':'i:0#.f|membership|markh@contoso.com'},{'Key':'i:0#.f|membership|john.doe@contoso.com'}]\",\"https://bing.com, URL\",5,2023-01-01 10:00:00",
            "When creating list items with a DateTime field, use the timezone and the format that the site expects, based on its regional settings. Alternatively, a format which works on all regions is the following: `yyyy-MM-dd HH:mm:ss`. However, you should use the local timezone in all situations. UTC date/time or ISO 8601 formatted date/time is not supported.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add a batch of items to a list retrieved by title in a specific site",
                "m365 spo listitem batch add --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\"",
            ),
            (
                "Add a batch of items to a list retrieved by title in a specific site using csv content",
                "$obj = @(\n[PSCustomObject]@{\nTitle = \"Item A\"\nAge = 10\n},\n[PSCustomObject]@{\nTitle = \"Item B\"\nAge = 20\n}\n)\n$csvContent = $obj | ConvertTo-Csv -NoTypeInformation -Delimiter \",\" | Out-String | ForEach-Object { $_.Replace('\"','\\\"') }\nm365 spo listitem batch add --csvContent $csvContent --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\"",
            ),
            (
                "Add a batch of items to a list retrieved by Id in a specific site",
                "m365 spo listitem batch add --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listId fe54c47b-22e4-4cab-8a10-3fc54003fb4c",
            ),
            (
                "Add a batch of items to a list defined by server-relative URL in a specific site",
                "m365 spo listitem batch add --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/lists/Demo List\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem batch remove",
        summary: "Removes items from a list in batch",
        options: &[
            ("webUrl", "URL of the SharePoint site."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "filePath",
                "The absolute or relative path to a flat CSV file containing the list items.",
            ),
            ("ids", "Comma separated list of list item IDs."),
            (
                "permanent",
                "Permanently delete the list items instead of moving them to the recycle bin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "A sample CSV can be found below. The first line of the CSV-file should contain the internal column name of the ID-column.",
            "    ID\n    123\n    234\n    345",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove a list of IDs from a list with a csv",
                "m365 spo listitem batch remove --filePath ./IDlist.csv --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\"",
            ),
            (
                "Remove a list of IDs from a list by specifying the IDs",
                "m365 spo listitem batch remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --ids \"123,234,345\"",
            ),
            (
                "Permanently remove a list of IDs from a list.",
                "m365 spo listitem batch remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --ids \"123,234,345\" --permanent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem batch set",
        summary: "Updates list items in a batch",
        options: &[
            (
                "filePath",
                "The absolute or relative path to a flat file containing the list items.",
            ),
            ("webUrl", "URL of the site."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "idColumn",
                "Name of the column in the csv containing the IDs of the items to set. Defaults to `ID`.",
            ),
            (
                "systemUpdate",
                "Update the item without updating the modified date and modified by fields.",
            ),
        ],
        notes: &[
            "A sample CSV can be found below. The first line of the CSV-file should contain the internal column names that you wish to set.",
            "    ID,ContentType,Title,SingleChoiceField,MultiChoiceField,SingleMetadataField,MultiMetadataField,SinglePeopleField,MultiPeopleField,CustomHyperlink,NumberField,LookupList,LookupListMulti\n    5,Item,Title Update,Choice 1,Choice 1;#Choice 2,Engineering|4a3cc5f3-a4a6-433e-a07a-746978ff1760,Engineering|4a3cc5f3-a4a6-433e-a07a-746978ff1760;Finance|f994a4ac-cf34-448e-a22c-2b35fd9bbffa,john@contoso.com,john@contoso.com;doe@contoso.com,\"https://bing.com, URL\",5,2,2;3",
        ],
        permissions: &[],
        examples: &[
            (
                "Updates a batch of items to a list retrieved by title in a specific site",
                "m365 spo listitem batch set --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\"",
            ),
            (
                "Updates a batch of items to a list retrieved by Id in a specific site with a specific `idColumn`",
                "m365 spo listitem batch set --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listId fe54c47b-22e4-4cab-8a10-3fc54003fb4c --idColumn id",
            ),
            (
                "Updates a batch of items to a list defined by server-relative URL in a specific site without updating the modified details",
                "m365 spo listitem batch set --filePath \"C:\\Path\\To\\Csv\\CsvFile.csv\" --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl \"/sites/project-x/lists/Demo List\" --systemUpdate",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem get",
        summary: "Gets a list item from the specified list",
        options: &[
            ("webUrl", "URL of the site where the item is located."),
            ("id", "ID of the item to retrieve."),
            ("uniqueId", "The Unique ID (GUID) of the item to retrieve."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "fields",
                "Comma-separated list of fields to retrieve. Will retrieve all fields if not specified and json output is requested.",
            ),
            (
                "withPermissions",
                "Set if you want to return associated roles and permissions.",
            ),
        ],
        notes: &[
            "If you want to specify a lookup type in the `fields` option, define which columns from the related list should be returned.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get an item with the ID parameter from a given list in a given site.",
                "m365 spo listitem get --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get an item with the Unique ID parameter from a given list in a given site.",
                "m365 spo listitem get --listTitle \"Demo List\" --uniqueId \"64dc28c4-3c43-45f6-ba66-307d9eb7e6aa\" --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get an item columns with the ID parameter from a given list in a given site.",
                "m365 spo listitem get --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --fields \"Title,Created\"",
            ),
            (
                "Get an item columns and lookup column with the ID parameter from a given list in a given site.",
                "m365 spo listitem get --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --fields \"Title,Created,Company/Title\"",
            ),
            (
                "Get an item with specific fields from a given list based on the server-relative URL in a specific site.",
                "m365 spo listitem get --listUrl /sites/project-x/documents --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --fields \"Title,Created,Company/Title\"",
            ),
            (
                "Get an item with ID parameter from a given list based on the server-relative URL in a specific site with permissions.",
                "m365 spo listitem get --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --withPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem isrecord",
        summary: "Checks if the specified list item is a record",
        options: &[
            ("webUrl", "The URL of the site where the list is located"),
            ("id", "The ID of the list item to check if it is a record"),
            ("listId", "ID of the list where the item should be checked."),
            (
                "listTitle",
                "Title of the list where the item should be checked.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Check whether the document with id 1 in list with title Documents located in site https://contoso.sharepoint.com/sites/project-x is a record",
                "m365 spo listitem isrecord --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'Documents' --id 1",
            ),
            (
                "Check whether the document with id 1 in list with id 0cd891ef-afce-4e55-b836-fce03286cccf located in site https://contoso.sharepoint.com/sites/project-x is a record",
                "m365 spo listitem isrecord --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id 1",
            ),
            (
                "Check whether a document with a specific id in a list retrieved by server-relative URL in a specific site is a record",
                "m365 spo listitem isrecord --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/documents --id 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem list",
        summary: "Gets a list of items from the specified list",
        options: &[
            (
                "webUrl",
                "URL of the site from which the item should be retrieved.",
            ),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "camlQuery",
                "CAML query to use to query the list of items with.",
            ),
            (
                "fields",
                "Comma-separated list of fields to retrieve. Will retrieve all fields if not specified and json output is requested. Specify `camlQuery` or `fields` but not both.",
            ),
            (
                "filter",
                "OData filter to use to query the list of items with. Specify `camlQuery` or `filter` but not both.",
            ),
            (
                "pageSize",
                "Number of list items to return. Specify `camlQuery` or `pageSize` but not both. The default value is 5000.",
            ),
            (
                "pageNumber",
                "Page number to return if `pageSize` is specified (first page is indexed as value of 0).",
            ),
        ],
        notes: &[
            "This command retrieves all items in the list, even if there are more than 5000. Use the `pageSize` and `pageNumber` options if you only want a specific amount of items. When using a CAML query, include a `RowLimit`-node to get all items. If you run into list view threshold exceptions, remove any Query-conditions or filters and also include a `RowLimit`-node.",
            "`pageNumber` is specified as a 0-based index. A value of `2` returns the third page of items.",
            "If you want to specify a lookup type in the `properties` option, define which columns from the related list should be returned.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get all items from a list named Demo List",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get all items from a list with ID 935c13a0-cc53-4103-8b48-c1d0828eaa7f",
                "m365 spo listitem list --listId 935c13a0-cc53-4103-8b48-c1d0828eaa7f --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Get all items from list named Demo List. For each item, retrieve the value of the ID, Title and Modified fields",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --fields \"ID,Title,Modified\"",
            ),
            (
                "Get all items from list named Demo List. For each item, retrieve the value of the ID, Title, Modified fields, and the value of lookup field Company",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --fields \"ID,Title,Modified,Company/Title\"",
            ),
            (
                "From a list named Demo List get all items with title Demo list item using an OData filter",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --filter \"Title eq 'Demo list item'\"",
            ),
            (
                "From a list named Demo List get the first 100 items",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --pageSize 100 --pageNumber 0",
            ),
            (
                "From a list named Demo List get the second batch of 10 items",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --pageSize 10 --pageNumber 1",
            ),
            (
                "Get all items from a list by server-relative URL",
                "m365 spo listitem list --listUrl /sites/project-x/documents --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "From a list named Demo List get all items with title Demo list item using a CAML query",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --camlQuery \"<View><Query><Where><Eq><FieldRef Name='Title' /><Value Type='Text'>Demo list item</Value></Eq></Where></Query><RowLimit Paged='TRUE'>5000</RowLimit></View>\"",
            ),
            (
                "From a library named Demo Library with 5000+ files and folders, get all items recursively without running into a list view threshold exception, using a CAML query",
                "m365 spo listitem list --listTitle \"Demo List\" --webUrl https://contoso.sharepoint.com/sites/project-x --camlQuery \"<View Scope='RecursiveAll'><Query></Query><ViewFields><FieldRef Name='Title'/></ViewFields><RowLimit Paged='TRUE'>5000</RowLimit></View>\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem record declare",
        summary: "Declares the specified list item as a record",
        options: &[
            ("webUrl", "URL of the site where the list is located"),
            ("listId", "The ID of the list where the item is located."),
            (
                "listTitle",
                "The title of the list where the item is located.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("listItemId", "The ID of the list item to declare as record"),
            (
                "date",
                "Record declaration date in ISO format. eg. 2019-12-31",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Declare a document with id 1 as a record in list with title Demo List located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem record declare --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 1",
            ),
            (
                "Declare a document with id 1 as a record in list with id ea8e1109-2013-1a69-bc05-1403201257fc located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem record declare --webUrl https://contoso.sharepoint.com/sites/project-x --listId ea8e1109-2013-1a69-bc05-1403201257fc --listItemId 1",
            ),
            (
                "Declare a document with id 1 as a record with record declaration date March 14, 2012 in list with title Demo List located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem record declare --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle \"Demo List\" --listItemId 1 --date 2012-03-14",
            ),
            (
                "Declare a document with a specific id as a record with a record declaration date a list retrieved by server-relative URL located in a specific site",
                "m365 spo listitem record declare --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/Lists/Demo List' --listItemId 1 --date 2013-09-03",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem record lock",
        summary: "Locks the list item record",
        options: &[
            ("webUrl", "The url of the web"),
            ("listItemId", "ID of the list item that should be locked"),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Locks the list item record in a given site based on the list id",
                "m365 spo listitem record lock --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 1",
            ),
            (
                "Locks the list item record in a given site based on the list title",
                "m365 spo listitem record lock --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --listItemId 1",
            ),
            (
                "Locks the list item record in a given site based on the server relative list url",
                "m365 spo listitem record lock --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --listItemId 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem record undeclare",
        summary: "Undeclares list item as a record",
        options: &[
            ("webUrl", "The URL of the site where the list is located"),
            (
                "listItemId",
                "ID of the list item to be undeclared as a record.",
            ),
            ("listId", "The ID of the list where the item is located."),
            (
                "listTitle",
                "The title of the list where the item is located.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list where the item is located.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Undeclare the list item as a record with ID 1 from list with ID 0cd891ef-afce-4e55-b836-fce03286cccf located in site _https://contoso.sharepoint.com/sites/project-x",
                "spo listitem record undeclare --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 1",
            ),
            (
                "Undeclare the list item as a record with ID 1 from list with title List 1 located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem record undeclare --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --listItemId 1",
            ),
            (
                "Undeclare a list item with a specific id as a record from a list retrieved by server-relative URL located in a specific site",
                "m365 spo listitem record undeclare --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl '/sites/project-x/Lists/Lists 1' --listItemId 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem record unlock",
        summary: "Unlocks the list item record",
        options: &[
            ("webUrl", "The url of the web"),
            ("listItemId", "ID of the list item that should be unlocked"),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Unlocks the list item record in a given site based on the list id",
                "m365 spo listitem record unlock --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 1",
            ),
            (
                "Unlocks the list item record in a given site based on the list title",
                "m365 spo listitem record unlock --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --listItemId 1",
            ),
            (
                "Unlocks the list item record in a given site based on the server relative list url",
                "m365 spo listitem record unlock --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --listItemId 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem remove",
        summary: "Removes the specified list item",
        options: &[
            (
                "webUrl",
                "URL of the site where the list item to remove is located",
            ),
            ("id", "The ID of the list item to remove."),
            ("listId", "ID of the list where the item should be removed."),
            (
                "listTitle",
                "Title of the list where the item should be removed.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "permanent",
                "Permanently delete the list item instead of moving it to the recycle bin.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the list item",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove the list item located in a given site based on the list id",
                "m365 spo listitem remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --id 1",
            ),
            (
                "Remove the list item located in a given site based on the list title",
                "m365 spo listitem remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --id 1",
            ),
            (
                "Remove the list item located in a given site based on the server-relative list url",
                "m365 spo listitem remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --id 1",
            ),
            (
                "Permanently remove a list item.",
                "m365 spo listitem remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --id 1 --permanent",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem retentionlabel ensure",
        summary: "Applies a retention label to a list item",
        options: &[
            (
                "webUrl",
                "URL of the site where the retentionlabel from a list item to apply is located.",
            ),
            (
                "listItemId",
                "The ID of the list item for which the retention label should be applied.",
            ),
            (
                "listId",
                "ID of the list where the retention label should be applied.",
            ),
            (
                "listTitle",
                "Title of the list where the retention label should be applied.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            ("name", "The name of the retention label."),
            ("id", "The id of the retention label."),
            (
                "assetId",
                "A Compliance Asset Id to set on the item when it's labeled. See below for more information.",
            ),
        ],
        notes: &[
            "The `--assetId` option has to do with event-based retention. Event-based retention is about starting a retention period when a specific event occurs, instead of the moment a document was labeled or created.",
        ],
        permissions: &[],
        examples: &[
            (
                "Applies a retention label to a list item in a given site based on the list id and label name",
                "m365 spo listitem retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 1 --name 'Some label'",
            ),
            (
                "Applies a retention label to a list item in a given site based on the list title and label id",
                "m365 spo listitem retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --listItemId 1 --id '7a621a91-063b-461b-aff6-d713d5fb23eb'",
            ),
            (
                "Applies a retention label to a list item in a given site based on the server relative list url",
                "m365 spo listitem retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --listItemId 1 --name 'Some label'",
            ),
            (
                "Applies a retention label to a list item in a given site based on the server relative list url and updates the Asset Id field",
                "m365 spo listitem retentionlabel ensure --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --listItemId 1 --name 'Some label' --assetId 'XYZ'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem retentionlabel remove",
        summary: "Clears the retention label from a list item",
        options: &[
            (
                "webUrl",
                "URL of the site where the retentionlabel from a list item to remove is located",
            ),
            (
                "listItemId",
                "The ID of the list item for which the retention label should be removed.",
            ),
            (
                "listId",
                "ID of the list where the retention label should be removed.",
            ),
            (
                "listTitle",
                "Title of the list where the retention label should be removed.",
            ),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "force",
                "Don't prompt for confirming removing the retention label from a list item",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the retention label from a list item in a given site based on the list id",
                "m365 spo listitem retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listId 0cd891ef-afce-4e55-b836-fce03286cccf --listItemId 1",
            ),
            (
                "Removes the retention label from a list item in a given site based on the list title",
                "m365 spo listitem retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listTitle 'List 1' --listItemId 1",
            ),
            (
                "Removes the retention label from a list item in a given site based on the server relative list url",
                "m365 spo listitem retentionlabel remove --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/lists/TestList --listItemId 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem roleassignment add",
        summary: "Adds a role assignment to a listitem.",
        options: &[
            ("webUrl", "URL of the site where the list item is located"),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Relative URL of the list."),
            ("listItemId", "Id of the list item to assign the role to."),
            (
                "principalId",
                "The SharePoint Id of the principal. It may be either a user id or group id to add a role assignment for.",
            ),
            ("upn", "The upn/email of user to assign role to."),
            (
                "groupName",
                "The group name of the SharePoint group. Use this option exclusively for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to add."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to add.",
            ),
            ("roleDefinitionId", "ID of role definition."),
            (
                "roleDefinitionName",
                "The name of a role definition, like 'Contribute', 'Read', etc.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Add role assignment to specified list item in specified list located in specified site for specified principal id and specified role definition id.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --listItemId 1 --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified principal id and specified role definition id.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listId \"0CD891EF-AFCE-4E55-B836-FCE03286CCCF\" --listItemId 1 --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified principal id and specified role definition id.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listUrl \"sites/documents\" --listItemId 1 --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified upn and specified role definition id.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --listItemId 1 --upn \"someaccount@tenant.onmicrosoft.com\" --roleDefinitionId 1073741829",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified group and specified role definition id.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --listItemId 1 --groupName \"someGroup\" --roleDefinitionId 1073741829",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified principal id and specified role definition name.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --listItemId 1 --principalId 11 --roleDefinitionName \"Full Control\"",
            ),
            (
                "Add role assignment to specified list item in specified list located in specified site for specified Entra group id and specified role definition name.",
                "m365 spo listitem roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"someList\" --listItemId 1 --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\" --roleDefinitionName \"Full Control\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem roleassignment remove",
        summary: "Removes a role assignment from a listitem.",
        options: &[
            ("webUrl", "URL of the site where the list item is located."),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Relative URL of the list."),
            ("listItemId", "Id of the list item to remove the role from."),
            (
                "principalId",
                "SharePoint ID of principal it may be either user id or group id we want to remove permissions.",
            ),
            ("upn", "The upn/email of user."),
            (
                "groupName",
                "The group name of the SharePoint group. Use this option exclusively for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to remove."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to remove.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the role assignment.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove roleassignment from list item getting list by title based on group name",
                "m365 spo listitem roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listTitle \"someList\" --listItemId 1 --groupName \"saleGroup\"",
            ),
            (
                "Remove roleassignment from list item getting list by title based on principal Id",
                "m365 spo listitem roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listTitle \"Events\" --listItemId 1 --principalId 2",
            ),
            (
                "Remove roleassignment from list item getting list by url based on principal Id",
                "m365 spo listitem roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listUrl \"/sites/contoso-sales/lists/Events\" --listItemId 1 --principalId 2",
            ),
            (
                "Remove roleassignment from list item getting list by url based on principal Id without prompting for confirmation",
                "m365 spo listitem roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listUrl \"/sites/contoso-sales/lists/Events\" --listItemId 1 --principalId 2 --force",
            ),
            (
                "Remove roleassignment from list item getting list by url based on Entra group Id without prompting for confirmation",
                "m365 spo listitem roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\" --listUrl \"/sites/contoso-sales/lists/Events\" --listItemId 1 --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem roleinheritance break",
        summary: "Breaks inheritance of list item.",
        options: &[
            (
                "webUrl",
                "URL of the site where the item for which to break role inheritance is located",
            ),
            (
                "listItemId",
                "ID of the item for which to break role inheritance",
            ),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "clearExistingPermissions",
                "Set to clear existing roles from the list item",
            ),
            (
                "force",
                "Do not prompt for confirmation before breaking role inheritance.",
            ),
        ],
        notes: &[
            "By default, when breaking permissions inheritance, the list item will retain existing permissions. To remove existing permissions, use the `--clearExistingPermissions` option.",
        ],
        permissions: &[],
        examples: &[
            (
                "Break inheritance of list item 1 in list someList located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"_someList_\" --listItemId 1",
            ),
            (
                "Break inheritance of list item 1 in list with ID 202b8199-b9de-43fd-9737-7f213f51c991 located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem roleinheritance break --webUrl https://contoso.sharepoint.com/sites/project-x --listId 202b8199-b9de-43fd-9737-7f213f51c991 --listItemId 1",
            ),
            (
                "Break inheritance of list item 1 in list someList located in site https://contoso.sharepoint.com/sites/project-x with clearing permissions",
                "m365 spo listitem roleinheritance break --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --listTitle \"_someList_\" --listItemId 1 --clearExistingPermissions",
            ),
            (
                "Break inheritance of a specific list item in a list retrieved by server-relative URL in a specific site and clear the existing permissions",
                "m365 spo listitem roleinheritance break --webUrl https://contoso.sharepoint.com/sites/project-x --listUrl /sites/project-x/Documents --listItemId 1 --clearExistingPermissions",
            ),
            (
                "Break inheritance of list item 1 in list with ID 202b8199-b9de-43fd-9737-7f213f51c991 located in site https://contoso.sharepoint.com/sites/project-x with clearing permissions without prompting for confirmation",
                "m365 spo listitem roleinheritance break --webUrl https://contoso.sharepoint.com/sites/project-x --listId 202b8199-b9de-43fd-9737-7f213f51c991 --listItemId 1 --clearExistingPermissions --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem roleinheritance reset",
        summary: "Restores the role inheritance of list item, file, or folder",
        options: &[
            (
                "webUrl",
                "URL of the site where the item for which to reset role inheritance is located",
            ),
            (
                "listItemId",
                "ID of the item for which to reset role inheritance",
            ),
            ("listId", "ID of the list."),
            ("listTitle", "Title of the list."),
            ("listUrl", "Server- or site-relative URL of the list."),
            (
                "force",
                "Do not prompt for confirmation before resetting role inheritance.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Restore role inheritance of list item with id 8 from list with ID 0cd891ef-afce-4e55-b836-fce03286cccf located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listItemId 8 --listId 0cd891ef-afce-4e55-b836-fce03286cccf",
            ),
            (
                "Restore role inheritance of list item with id 8 from list with title test located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo listitem roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listItemId 8 --listTitle test",
            ),
            (
                "Restore role inheritance of a list item with a specific id in a list retrieved by server relative URL located in a specific site without prompting for confirmation",
                "m365 spo listitem roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --listItemId 8 --listUrl /sites/project-x/lists/test --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo listitem set",
        summary: "Updates a list item in the specified list",
        options: &[
            ("webUrl", "URL of the site where the item should be updated"),
            ("id", "ID of the list item to update."),
            ("listId", "ID of the list where the item should be updated."),
            (
                "listTitle",
                "Title of the list where the item should be updated.",
            ),
            (
                "listUrl",
                "Server- or site-relative URL of the list where the item should be updated.",
            ),
            (
                "contentType",
                "The name or the ID of the content type to associate with the updated item",
            ),
            (
                "systemUpdate",
                "Update the item without updating the modified date and modified by fields",
            ),
        ],
        notes: &[
            "When updating a list item with a DateTime field, use the timezone and the format that the site expects, based on its regional settings. Alternatively, a format which works on all regions is the following: `yyyy-MM-dd HH:mm:ss`. However, you should use the local timezone in all situations. UTC date/time or ISO 8601 formatted date/time is not supported.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update an item with id 147 with title Demo Item and content type name Item in list with title Demo List in a site",
                "m365 spo listitem set --contentType Item --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Item\"",
            ),
            (
                "Update an item with id 147 with title Demo Multi Managed Metadata Field and a single-select metadata field named SingleMetadataField in list with title Demo List in a site",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Single Managed Metadata Field\" --SingleMetadataField \"TermLabel1|fa2f6bfd-1fad-4d18-9c89-289fe6941377;\"",
            ),
            (
                "Update an item with id 147 with Title Demo Multi Managed Metadata Field and a multi-select metadata field named MultiMetadataField in list with title Demo List in a site",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Multi Managed Metadata Field\" --MultiMetadataField \"TermLabel1|cf8c72a1-0207-40ee-aebd-fca67d20bc8a;TermLabel2|e5cc320f-8b65-4882-afd5-f24d88d52b75;\"",
            ),
            (
                "Update an item with id 147 with Title Demo Single Person Field and a single-select people field named SinglePeopleField to list with title Demo List in a site",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Single Person Field\" --SinglePeopleField \"[{'Key':'i:0#.f|membership|markh@conotoso.com'}]\"",
            ),
            (
                "Update an item with id 147 with Title Demo Multi Person Field and a multi-select people field named MultiPeopleField to list with title Demo List in a site",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Multi Person Field\" --MultiPeopleField \"[{'Key':'i:0#.f|membership|markh@conotoso.com'},{'Key':'i:0#.f|membership|john.doe@conotoso.com'}]\"",
            ),
            (
                "Update the field Title and CustomHyperlink of an item with a specific id in a list retrieved by server-relative URL in a specific site",
                "m365 spo listitem set --listUrl '/sites/project-x/lists/Demo List' --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo Hyperlink Field\" --CustomHyperlink \"https://www.bing.com, Bing\"",
            ),
            (
                "Update an item with a specific Title and multi-choice value",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo multi-choice Field\" --MultiChoiceField \"Choice 1;#Choice 2;#Choice 3\"",
            ),
            (
                "Update an item with a specific Title and DateTime value",
                "m365 spo listitem set --listTitle \"Demo List\" --id 147 --webUrl https://contoso.sharepoint.com/sites/project-x --Title \"Demo DateTime Field\" --SomeDateTimeField \"2023-01-16 15:30:00\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo navigation node add",
        summary: "Adds a navigation node to the specified site navigation",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site to which navigation should be modified.",
            ),
            (
                "location",
                "Navigation type where the node should be added. Available options: `QuickLaunch`, `TopNavigationBar`.",
            ),
            ("title", "Navigation node title."),
            (
                "url",
                "Navigation node URL. When not specified a linkless label will be created.",
            ),
            (
                "parentNodeId",
                "ID of the node below which the node should be added.",
            ),
            (
                "audienceIds",
                "Comma-separated list of group IDs that will be used for audience targeting. The limit is 10 ids per navigation node.",
            ),
            (
                "openInNewWindow",
                "Set, if the link has to be opened in a new window.",
            ),
        ],
        notes: &[
            "To enable/disable audience targeting for the navigation bar, use the `spo web set` command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add a navigation node pointing to a SharePoint page to the top navigation.",
                "m365 spo navigation node add --webUrl https://contoso.sharepoint.com/sites/team-a --location TopNavigationBar --title About --url /sites/team-s/sitepages/about.aspx",
            ),
            (
                "Add a navigation node pointing to an external page to the quick launch.",
                "m365 spo navigation node add --webUrl https://contoso.sharepoint.com/sites/team-a --location QuickLaunch --title \"About us\" --url https://contoso.com/about-us",
            ),
            (
                "Add a navigation node below an existing node and opens it in a new window.",
                "m365 spo navigation node add --webUrl https://contoso.sharepoint.com/sites/team-a --parentNodeId 2010 --title About --url /sites/team-s/sitepages/about.aspx --openInNewWindow",
            ),
            (
                "Add a navigation node to the top navigation which is audience targetted and open it in a new window.",
                "m365 spo navigation node add --webUrl https://contoso.sharepoint.com/sites/team-a --location TopNavigationBar --title About --url /sites/team-s/sitepages/about.aspx --audienceIds \"7aa4a1ca-4035-4f2f-bac7-7beada59b5ba,4bbf236f-a131-4019-b4a2-315902fcfa3a\" --openInNewWindow",
            ),
            (
                "Add a linkless label to the quick launch.",
                "m365 spo navigation node add --webUrl https://contoso.sharepoint.com/sites/team-a --location QuickLaunch --title Resources",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo navigation node get",
        summary: "Gets information about a specific navigation node",
        options: &[
            ("webUrl", "Absolute URL of the site."),
            (
                "location",
                "Navigation type where the node is located. Available options: `QuickLaunch`, `TopNavigationBar`.",
            ),
            ("id", "Id of the navigation node."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieve information for a specific navigation node from the quick launch.",
                "m365 spo navigation node get --webUrl https://contoso.sharepoint.com/sites/Marketing --location QuickLaunch --id 2209",
            ),
            (
                "Retrieve information for a specific navigation node from the top navigation.",
                "m365 spo navigation node get --webUrl https://contoso.sharepoint.com/sites/Marketing --location TopNavigationBar --id 2209",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo navigation node list",
        summary: "Lists nodes from the specified site navigation",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site for which to retrieve navigation.",
            ),
            (
                "location",
                "Navigation type to retrieve. Available options: `QuickLaunch`, `TopNavigationBar`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[
            (
                "Retrieve nodes from the top navigation.",
                "m365 spo navigation node list --webUrl https://contoso.sharepoint.com/sites/Marketing --location TopNavigationBar",
            ),
            (
                "Retrieve nodes from the quick launch.",
                "m365 spo navigation node list --webUrl https://contoso.sharepoint.com/sites/Marketing --location QuickLaunch",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo navigation node remove",
        summary: "Removes the specified navigation node",
        options: &[
            (
                "webUrl",
                "Absolute URL of the site to which navigation should be modified",
            ),
            (
                "location",
                "Navigation type where the node should be removed. Available options: `QuickLaunch,TopNavigationBar`",
            ),
            ("id", "ID of the node to remove"),
            ("force", "Don't prompt for confirming removing the node"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove a node from the top navigation. Will prompt for confirmation",
                "m365 spo navigation node remove --webUrl https://contoso.sharepoint.com/sites/team-a --location TopNavigationBar --id 2003",
            ),
            (
                "Remove a node from the quick launch without prompting for confirmation",
                "m365 spo navigation node remove --webUrl https://contoso.sharepoint.com/sites/team-a --location QuickLaunch --id 2003 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo navigation node set",
        summary: "Updates a SharePoint navigation node",
        options: &[
            ("webUrl", "Absolute URL of the site."),
            (
                "location",
                "Navigation type where the node is located. Available options: `QuickLaunch`, `TopNavigationBar`.",
            ),
            ("id", "Id of the navigation node."),
            ("title", "New title of the navigation node."),
            (
                "url",
                "New URL of the navigation node. Specify an empty string `\"\"` to update the navigation label to a linkless label.",
            ),
            (
                "audienceIds",
                "Comma-separated list of group IDs that will be used for audience targeting. Specify an empty string `\"\"` to clear this value. The limit is 10 ids per navigation node.",
            ),
            (
                "openInNewWindow",
                "Open the URL in a new window. Valid values: `true` or `false`.",
            ),
        ],
        notes: &[
            "To enable/disable audience targeting for the navigation bar, use the `spo web set` command.",
            "SharePoint doesn't allow changing the URL of a navigation node pointing to a page in the current site into an external URL. In that case, the URL isn't updated and no error is returned. To point such a node to an external URL, remove the node using `spo navigation node remove` and add it again using `spo navigation node add`.",
        ],
        permissions: &[],
        examples: &[
            (
                "Updates the title of a navigation node",
                "m365 spo navigation node set --webUrl https://contoso.sharepoint.com/sites/marketing --location TopNavigationBar --id 2209 --title \"Pictures\"",
            ),
            (
                "Updates the URL of a navigation node",
                "m365 spo navigation node set --webUrl https://contoso.sharepoint.com/sites/marketing --location QuickLaunch --id 2209 --url \"https://www.microsoft.com\"",
            ),
            (
                "Updates audience targeting of a navigation node with 3 groups",
                "m365 spo navigation node set --webUrl https://contoso.sharepoint.com/sites/marketing --location QuickLaunch --id 2209 --audienceIds \"61f78c73-f71a-471e-a3b9-15daa936e200,9524e6b4-e663-44fe-b179-210c963e37e7,c42b8756-494d-4141-a575-45f01320e26a\"",
            ),
            (
                "Updates the navigation node so that it opens in a new window",
                "m365 spo navigation node set --webUrl https://contoso.sharepoint.com/sites/marketing --location QuickLaunch --id 2209 --openInNewWindow true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo orgassetslibrary add",
        summary: "Promotes an existing library to become an organization assets library",
        options: &[
            ("libraryUrl", "The URL of the library to promote."),
            ("thumbnailUrl", "The URL of the thumbnail to render."),
            (
                "cdnType",
                "Specifies the CDN type. Allowed values `Public`, `Private`. Default `Private`.",
            ),
            (
                "orgAssetType",
                "Specifies the type of organizational library. Allowed values `ImageDocumentLibrary`, `OfficeTemplateLibrary`, `OfficeFontLibrary`. Defaults to `ImageDocumentLibrary`.",
            ),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
            "To enable CDN in your tenant use the spo cdn set command.",
            "The document library on which you run the command must at least have read permissions for all users, except external users.",
        ],
        permissions: &[],
        examples: &[
            (
                "Promotes an existing library to become an organization assets library",
                "m365 spo orgassetslibrary add --libraryUrl \"https://contoso.sharepoint.com/SiteAssets\"",
            ),
            (
                "Promotes an existing library to become an organization assets library with Thumbnail",
                "m365 spo orgassetslibrary --libraryUrl \"https://contoso.sharepoint.com/SiteAssets\" --thumbnailUrl \"https://contoso.sharepoint.com/assets/logo.png\"",
            ),
            (
                "Promotes an existing library to become an organization assets Office template library",
                "m365 spo orgassetslibrary add --libraryUrl \"https://contoso.sharepoint.com/SiteAssets\" --orgAssetType \"OfficeTemplateLibrary\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo orgassetslibrary list",
        summary: "Lists all libraries that are assigned as asset library",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "List all libraries that are assigned as asset library",
            "m365 spo orgassetslibrary list",
        )],
        links: &[],
    },
    Help {
        name: "spo orgassetslibrary remove",
        summary: "Removes a library that was designated as a central location for organization assets across the tenant.",
        options: &[
            (
                "libraryUrl",
                "The server relative URL of the library to be removed as a central location for organization assets.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the organization asset library.",
            ),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Removes organization assets library without prompting for confirmation",
            "m365 spo orgassetslibrary remove --libraryUrl \"/sites/branding/assets\" --force",
        )],
        links: &[],
    },
    Help {
        name: "spo orgnewssite list",
        summary: "Lists all organizational news sites",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "List all organizational news sites",
            "m365 spo orgnewssite list",
        )],
        links: &[],
    },
    Help {
        name: "spo orgnewssite remove",
        summary: "Removes a site from the list of organizational news sites",
        options: &[
            ("url", "Absolute URL of the site to remove."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove a site from the list of organizational news",
                "m365 spo orgnewssite remove --url https://contoso.sharepoint.com/sites/site1",
            ),
            (
                "Remove a site from the list of organizational news sites, without prompting for confirmation",
                "m365 spo orgnewssite remove --url https://contoso.sharepoint.com/sites/site1 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo orgnewssite set",
        summary: "Marks site as an organizational news site",
        options: &[(
            "url",
            "The URL of the site to mark as an organizational news site.",
        )],
        notes: &[
            "Using the `-u, --url` option you can specify which site to add to the list of organizational news sites.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Set a site as an organizational news site",
            "m365 spo orgnewssite set --url https://contoso.sharepoint.com/sites/site1",
        )],
        links: &[],
    },
    Help {
        name: "spo page add",
        summary: "Creates modern page",
        options: &[
            ("name", "Name of the page to create."),
            (
                "webUrl",
                "URL of the site where the page should be created.",
            ),
            (
                "title",
                "Title of the page to create. If not specified, will use the page name as its title.",
            ),
            (
                "layoutType",
                "Layout of the page. Allowed values `Article`, `Home`, `SingleWebPartAppPage`, `RepostPage`, `HeaderlessSearchResults`, `Spaces`, `Topic`. Default `Article`.",
            ),
            (
                "promoteAs",
                "Create the page for a specific purpose. Allowed values `HomePage`, `NewsPage`, `Template`.",
            ),
            ("commentsEnabled", "Set to enable comments on the page."),
            ("publish", "Set to publish the page."),
            ("publishMessage", "Message to set when publishing the page."),
            ("description", "The description to set for the page."),
        ],
        notes: &[
            "If you try to create a page with a name of a page that already exists, you will get a `The file exists` error.",
            "If you choose to promote the page using the `promoteAs` option or enable page comments, you will see the result only after publishing the page.",
        ],
        permissions: &[],
        examples: &[
            (
                "Create new modern page. Use the Article layout",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team",
            ),
            (
                "Create new modern page and set its title",
                "m365 spo page add --name new-page.aspx --title 'My page' --webUrl https://contoso.sharepoint.com/sites/a-team",
            ),
            (
                "Create new modern page. Use the Home page layout and include the default set of web parts",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --layoutType Home",
            ),
            (
                "Create new article page and promote it as a news article",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --promoteAs NewsPage",
            ),
            (
                "Create new page and set it as the site's home page",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --layoutType Home --promoteAs HomePage",
            ),
            (
                "Create new article page and promote it as a template",
                "m365 spo page add --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --promoteAs Template",
            ),
            (
                "Create new article page and enable comments on the page",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --commentsEnabled",
            ),
            (
                "Create new article page and publish it",
                "m365 spo page add --name new-page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --publish",
            ),
            (
                "Create new article page with description",
                "m365 spo page add --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --description \"Description to add for the page\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page clientsidewebpart add",
        summary: "Adds a client-side web part to a modern page",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to add the web part to is located.",
            ),
            ("pageName", "Name of the page to which add the web part."),
            (
                "standardWebPart",
                "Name of the standard web part to add (see the possible values below).",
            ),
            ("webPartId", "ID of the custom web part to add."),
            (
                "webPartProperties",
                "JSON string with web part properties to set on the web part. Specify either `webPartProperties` or `webPartData` but not both.",
            ),
            (
                "webPartData",
                "JSON string with web part data as retrieved from the web part maintenance mode. Specify either `webPartProperties` or `webPartData` but not both.",
            ),
            (
                "section",
                "Number of the section to which the web part should be added (1 or higher). Specify `section` or `verticalSection` but not both.",
            ),
            (
                "column",
                "Number of the column in which the web part should be added (1 or higher). Use `column` in combination with `section`, not with `verticalSection`.",
            ),
            ("order", "Order of the web part in the column."),
            (
                "verticalSection",
                "Specifies that the web part should be added to the vertical section.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
            "To add a standard web part to the page, specify one of the following values: ContentRollup, BingMap, ContentEmbed, DocumentEmbed, Image, ImageGallery, LinkPreview, NewsFeed, NewsReel, PowerBIReportEmbed, QuickChart, SiteActivity, VideoEmbed, YammerEmbed, Events, GroupCalendar, Hero, List, PageTitle, People, QuickLinks, CustomMessageRegion, Divider, MicrosoftForms, Spacer.",
            "When using the `--webPartProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add the standard Bing Map web part to a modern page in the first available location on the page",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --standardWebPart BingMap",
            ),
            (
                "Add the standard Bing Map web part to a modern page in the third column of the second section",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --standardWebPart BingMap --section 2 --column 3",
            ),
            (
                "Add a custom web part to the modern page",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --webPartId 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1",
            ),
            (
                "Add the standard Bing Map web part with the specific properties to a modern page",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --standardWebPart BingMap --webPartProperties '{\"Title\":\"Foo location\"}'",
            ),
            (
                "Add the standard Image web part with the preconfigured image",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --standardWebPart Image --webPartData '{ \"dataVersion\": \"1.8\", \"serverProcessedContent\": {\"htmlStrings\":{},\"searchablePlainTexts\":{\"captionText\":\"\"},\"imageSources\":{\"imageSource\":\"/sites/team-a/SiteAssets/work-life-balance.png\"},\"links\":{}}, \"properties\": {\"imageSourceType\":2,\"altText\":\"a group of people on a beach\",\"overlayText\":\"Work life balance\",\"fileName\":\"48146-OFF12_Justice_01.png\",\"siteId\":\"27664b85-067d-4be9-a7d7-89b2e804d09f\",\"webId\":\"a7664b85-067d-4be9-a7d7-89b2e804d09f\",\"listId\":\"37664b85-067d-4be9-a7d7-89b2e804d09f\",\"uniqueId\":\"67664b85-067d-4be9-a7d7-89b2e804d09f\",\"imgWidth\":650,\"imgHeight\":433,\"fixAspectRatio\":false,\"isOverlayTextEnabled\":true}}'",
            ),
            (
                "Add the standard Bing Map web part to a modern page in the vertical section",
                "m365 spo page clientsidewebpart add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --standardWebPart BingMap --verticalSection",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page column get",
        summary: "Gets information about a specific column of a modern page",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            ("pageName", "Name of the page to get column information of."),
            ("section", "ID of the section where the column is located."),
            (
                "column",
                "ID of the column for which to retrieve more information.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[(
            "Get information about the first column in the first section of a modern page",
            "m365 spo page column get --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --section 1 --column 1",
        )],
        links: &[],
    },
    Help {
        name: "spo page column list",
        summary: "Lists columns in the specific section of a modern page",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            ("pageName", "Name of the page to list columns of."),
            ("section", "ID of the section for which to list columns."),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[(
            "List columns in the first section of a modern page",
            "m365 spo page column list --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --section 1",
        )],
        links: &[],
    },
    Help {
        name: "spo page control get",
        summary: "Gets information about the specific control on a modern page",
        options: &[
            ("id", "ID of the control to retrieve information for."),
            ("pageName", "Name of the page where the control is located."),
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a `File doesn't exists` error.",
        ],
        permissions: &[],
        examples: &[(
            "Get information about the control with ID 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1 placed on a modern page with name home.aspx",
            "m365 spo page control get --id 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1 --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx",
        )],
        links: &[],
    },
    Help {
        name: "spo page control list",
        summary: "Lists controls on the specific modern page",
        options: &[
            ("pageName", "Name of the page to list controls of."),
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a `File doesn't exists` error.",
        ],
        permissions: &[],
        examples: &[(
            "List controls on the modern page",
            "m365 spo page control list --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx",
        )],
        links: &[],
    },
    Help {
        name: "spo page control remove",
        summary: "Removes a control from a modern page",
        options: &[
            ("webUrl", "URL of the site where the page is located."),
            ("pageName", "Name of the page where the control is located."),
            ("id", "The ID of the control to be removed."),
            (
                "draft",
                "Save the page as draft instead of republishing it.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove a control from a SharePoint page, with confirmation",
                "m365 spo page control remove --webUrl https://contoso.sharepoint.com/sites/Marketing --pageName home.aspx --id b3d6ebc9-6c7b-4e1e-99c1-4c08b682e8c1",
            ),
            (
                "Remove a control from a SharePoint page and don't republish the page",
                "m365 spo page control remove --webUrl https://contoso.sharepoint.com/sites/Marketing --pageName \"home.aspx\" --id b3d6ebc9-6c7b-4e1e-99c1-4c08b682e8c1 --draft",
            ),
            (
                "Remove a control from a SharePoint page, that is located in a folder",
                "m365 spo page control remove --webUrl https://contoso.sharepoint.com/sites/Marketing --pageName \"Archive/home.aspx\" --id b3d6ebc9-6c7b-4e1e-99c1-4c08b682e8c1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page control set",
        summary: "Updates web part data or properties of a control on a modern page",
        options: &[
            ("id", "ID of the control to update properties of."),
            ("pageName", "Name of the page where the control is located."),
            ("webUrl", "URL of the site where the page is located."),
            (
                "webPartData",
                "JSON string with web part data as retrieved from the web part maintenance mode.",
            ),
            (
                "webPartProperties",
                "JSON string with web part data as retrieved from the web part maintenance mode.",
            ),
            ("innerHtml", "Inner HTML to set on the control."),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a `File doesn't exists` error.",
            "When using the `webPartProperties` or `webPartData` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "When using `webPartProperties` or `webPartData`, only the properties you specify will be updated. Any properties not included in the JSON string will remain unchanged.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update web part data for the control, placed on a modern page",
                "m365 spo page control set --id 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1 --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --webPartData '{\"title\":\"New WP Title\",\"properties\": {\"description\": \"New description\"}}'",
            ),
            (
                "Update web part properties for the control, placed on a modern page",
                "m365 spo page control set --id 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1 --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --webPartProperties '{\"description\": \"New description\"}'",
            ),
            (
                "Update the text of a text web part",
                "m365 spo page control set --id 3ede60d3-dc2c-438b-b5bf-cc40bb2351e1 --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --innerHtml '<p>This text was updated by <a href=\"https://aka.ms/cli-m365\">CLI for Microsoft 365</a></p>'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page copy",
        summary: "Creates a copy of a modern page or template",
        options: &[
            ("sourceName", "The name of the source file."),
            (
                "targetUrl",
                "The URL of the target file. You can specify page's name or relative- or absolute URL.",
            ),
            (
                "overwrite",
                "Overwrite the target page when it already exists.",
            ),
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
        ],
        notes: &[
            "If another page exists at the specified target location, copying the page will fail, unless you include the `--overwrite` option.",
        ],
        permissions: &[],
        examples: &[
            (
                "Create a copy of the specified page.",
                "m365 spo page copy --webUrl https://contoso.sharepoint.com/sites/team-a --sourceName \"home.aspx\" --targetUrl \"home-copy.aspx\"",
            ),
            (
                "Overwrite the target page if it already exists.",
                "m365 spo page copy --webUrl https://contoso.sharepoint.com/sites/team-a --sourceName \"home.aspx\" --targetUrl \"home-copy.aspx\" --overwrite",
            ),
            (
                "Create a copy of a page template.",
                "m365 spo page copy --webUrl https://contoso.sharepoint.com/sites/team-a --sourceName \"templates/PageTemplate.aspx\" --targetUrl \"page.aspx\"",
            ),
            (
                "Copy the page to another site.",
                "m365 spo page copy --webUrl https://contoso.sharepoint.com/sites/team-a --sourceName \"templates/PageTemplate.aspx\" --targetUrl \"https://contoso.sharepoint.com/sites/team-b/sitepages/page.aspx\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page get",
        summary: "Gets information about the specific modern page",
        options: &[
            ("name", "Name of the page to retrieve."),
            ("default", "Get the homepage of a specific web."),
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            (
                "metadataOnly",
                "Specify to only retrieve the metadata without the section and control information.",
            ),
        ],
        notes: &[
            "If the specified name doesn't refer to an existing modern page, you will get a `File doesn't exists` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the modern page.",
                "m365 spo page get --webUrl https://contoso.sharepoint.com/sites/team-a --name home.aspx",
            ),
            (
                "Get information about the home page.",
                "m365 spo page get --webUrl https://contoso.sharepoint.com/sites/team-a --default",
            ),
            (
                "Get all the metadata from the modern page, without the section and control count information.",
                "m365 spo page get --webUrl https://contoso.sharepoint.com/sites/team-a --name home.aspx --metadataOnly",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page header set",
        summary: "Sets modern page header",
        options: &[
            ("pageName", "Name of the page to set the header for."),
            (
                "webUrl",
                "URL of the site where the page to update is located.",
            ),
            (
                "type",
                "Type of header, allowed values `None`, `Default`, `Custom`.",
            ),
            (
                "imageUrl",
                "Server-relative URL of the image to use in the header. Image must be stored in the same site collection as the page.",
            ),
            ("altText", "Header image alt text."),
            ("translateX", "X focal point of the header image."),
            ("translateY", "Y focal point of the header image."),
            (
                "layout",
                "Layout to use in the header. Allowed values `FullWidthImage`, `NoImage`, `ColorBlock`, `CutInShape`.",
            ),
            (
                "textAlignment",
                "How to align text in the header. Allowed values `Center`, `Left`.",
            ),
            (
                "showTopicHeader",
                "Show the topic header. Valid values: `true`, `false`.",
            ),
            (
                "showPublishDate",
                "Show the publishing date. Valid values: `true`, `false`.",
            ),
            (
                "showTimeToRead",
                "Show the estimated time to read the page. Valid values: `true`, `false`.",
            ),
            (
                "topicHeader",
                "Text to show in the topic header, when `showTopicHeader` is set.",
            ),
            (
                "authors",
                "Comma-separated list of page authors to show in the header.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Update the page header authors",
                "m365 spo page header set --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --authors \"steve@contoso.com, bob@contoso.com\"",
            ),
            (
                "Reset the page header to default and set authors",
                "m365 spo page header set --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --type Default --authors \"steve@contoso.com, bob@contoso.com\"",
            ),
            (
                "Use the specified image focused on the given coordinates in the page header",
                "m365 spo page header set --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --type Custom --imageUrl /sites/team-a/SiteAssets/hero.jpg --altText 'Sunset over the ocean' --translateX 42.3837520042758 --translateY 56.4285714285714",
            ),
            (
                "Center the page title in the header and show the publishing date and time to read",
                "m365 spo page header set --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --textAlignment Center --showPublishDate true --showTimeToRead true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page list",
        summary: "Lists all modern pages in the given site",
        options: &[(
            "webUrl",
            "URL of the site from which to retrieve available pages.",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List all modern pages in the specific site.",
            "m365 spo page list --webUrl https://contoso.sharepoint.com/sites/team-a",
        )],
        links: &[],
    },
    Help {
        name: "spo page publish",
        summary: "Publishes a modern page",
        options: &[
            ("webUrl", "URL of the site where the page is located."),
            ("name", "Name of the page."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Publish a modern page",
                "m365 spo page publish --webUrl https://contoso.sharepoint.com/sites/Marketing --name \"Style guide.aspx\"",
            ),
            (
                "Publish a modern page in a subfolder",
                "m365 spo page publish --webUrl https://contoso.sharepoint.com/sites/Marketing --name \"/Styles/Guide.aspx\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page remove",
        summary: "Removes a modern page",
        options: &[
            ("webUrl", "URL of the site where the page is located."),
            ("name", "Name of the page."),
            (
                "permanent",
                "Permanently delete the page instead of moving it to the recycle bin.",
            ),
            (
                "bypassSharedLock",
                "Remove the page even if it is locked for shared use.",
            ),
            ("force", "Do not prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove a modern page",
                "m365 spo page remove --name HR.aspx --webUrl https://contoso.sharepoint.com/sites/Marketing",
            ),
            (
                "Remove a modern page without a confirmation prompt",
                "m365 spo page remove --name HR.aspx --webUrl https://contoso.sharepoint.com/sites/Marketing --force",
            ),
            (
                "Permanently remove a page",
                "m365 spo page remove --name HR.aspx --webUrl https://contoso.sharepoint.com/sites/Marketing --permanent",
            ),
            (
                "Remove a page that is locked",
                "m365 spo page remove --name HR.aspx --webUrl https://contoso.sharepoint.com/sites/Marketing --bypassSharedLock",
            ),
            (
                "Remove a page that is located in a subfolder",
                "m365 spo page remove --name /Departments/People/HR.aspx --webUrl https://contoso.sharepoint.com/sites/Marketing",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page section add",
        summary: "Adds section to modern page",
        options: &[
            ("pageName", "Name of the page to add section to."),
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            (
                "sectionTemplate",
                "Type of section to add. Allowed values `OneColumn`, `OneColumnFullWidth`, `TwoColumn`, `ThreeColumn`, `TwoColumnLeft`, `TwoColumnRight`, `Vertical`, `Flexible`.",
            ),
            ("order", "Order of the section to add."),
            (
                "zoneEmphasis",
                "Section background shading. Allowed values `None`, `Neutral`, `Soft`, `Strong`, `Image`,`Gradient`",
            ),
            (
                "isLayoutReflowOnTop",
                "The position of the Vertical section for smaller screens. Applied only for `Vertical` section.",
            ),
            ("isCollapsibleSection", "Set section to be collapsible."),
            ("showDivider", "Shows a divider line between sections."),
            (
                "iconAlignment",
                "Specifies the alignment of the expand/collapse icon. Sets `Left` alignment if not specified.",
            ),
            (
                "isExpanded",
                "Sets the default display state of the collapsible section. Sets `false` if not specified.",
            ),
            (
                "headingLevel",
                "Heading level of collapsible section Allowed values `2`, `3`, `4`.",
            ),
            (
                "gradientText",
                "Sets the gradient setting of the background of a section. Required when `zoneEmphasis` is `Gradient`.",
            ),
            (
                "imageUrl",
                "The background image URL. Required when `zoneEmphasis` is `Image`.",
            ),
            (
                "imageHeight",
                "The height of the background image. Applied only when when `zoneEmphasis` is `Image`. Sets `955` value if not specified.",
            ),
            (
                "imageWidth",
                "The width of the background image. Applied only when `zoneEmphasis` is `Image`. Sets `555` value if not specified.",
            ),
            (
                "fillMode",
                "The fill mode of the background image. Applied only when `zoneEmphasis` is `Image`. Possible values are `ScaleToFill`, `ScaleToFit`, `Tile`, `OriginalSize`. Sets `ScaleToFill` value if not specified.",
            ),
            (
                "useLightText",
                "Specifies whether to use light text for the background. Applied only when `zoneEmphasis` is `Image`.",
            ),
            (
                "overlayColor",
                "The overlay color for the background in #RRGGBB format. Applied only when `zoneEmphasis` is `Image` or `Gradient`. Sets `#ffffff` value if not specified.",
            ),
            (
                "overlayOpacity",
                "The overlay opacity for the background. Applied only when `zoneEmphasis` is `Image` or `Gradient`. Sets `60` value if not specified.",
            ),
            (
                "collapsibleTitle",
                "The display name of the collapsible section.",
            ),
            (
                "zoneReflowStrategy",
                "Mobile and email reflow. Allowed values `TopToBottom`, `LeftToRight`. Applied only for `Flexible` section.",
            ),
            (
                "zoneHeight",
                "Section height. Minimum value is `34`. Applied only for `Flexible` section.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add section to the modern page",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate OneColumn --order 1",
            ),
            (
                "Add section with background shading to the modern page",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate OneColumn --zoneEmphasis Strong",
            ),
            (
                "Add a vertical section to the modern page. There can only be one vertical section per page. If a vertical section already exists on the page, the command will not have any effect.",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate Vertical",
            ),
            (
                "Add Vertical section with background shading to the modern page with adjusting the position of this section for smaller screens to the top",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate Vertical --zoneEmphasis Neutral --isLayoutReflowOnTop",
            ),
            (
                "Add One Column section as a collapsible section with title",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate OneColumn --isCollapsibleSection --collapsibleTitle \"Collapsible Section Title\"",
            ),
            (
                "Add OneColumn section as a collapsible section as expanded with icon aligned to the left",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate OneColumn --isCollapsibleSection --isExpanded --iconAlignment Left",
            ),
            (
                "Add OneColumn section as a collapsible section as expanded with headingLevel 3",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate OneColumn --isCollapsibleSection --isExpanded --headingLevel 3",
            ),
            (
                "Add TwoColumn section with Image background",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate TwoColumn --imageUrl \"https://contoso.com/image.jpg\" --zoneEmphasis Image --fillMode Tile",
            ),
            (
                "Add OneColumn section with Gradient background",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate TwoColumn --zoneEmphasis Gradient --gradientText \"linear-gradient(72.44deg, #E6FBFE 0%, #EDDDFB 100%)\"",
            ),
            (
                "Add a Flexible section to the modern page with mobile and email reflow and section height.",
                "m365 spo page section add --pageName home.aspx --webUrl https://contoso.sharepoint.com/sites/newsletter --sectionTemplate Flexible --zoneHeight 100 --zoneReflowStrategy LeftToRight",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page section get",
        summary: "Gets information about the specified modern page section",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            (
                "pageName",
                "Name of the page to get section information of.",
            ),
            (
                "section",
                "ID of the section for which to retrieve information.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[(
            "Get information about the specified section of the modern page",
            "m365 spo page section get --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --section 1",
        )],
        links: &[],
    },
    Help {
        name: "spo page section list",
        summary: "Lists sections in the specific modern page",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to retrieve is located.",
            ),
            ("pageName", "Name of the page to list sections of."),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[(
            "List sections of a modern page",
            "m365 spo page section list --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx",
        )],
        links: &[],
    },
    Help {
        name: "spo page section remove",
        summary: "Removes the specified section from a modern page",
        options: &[
            ("webUrl", "URL of the site where the page is located."),
            (
                "pageName",
                "Name of the page from which to remove a section.",
            ),
            ("section", "ID of the section to be removed."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[(
            "Remove section of a modern page",
            "m365 spo page section remove --webUrl https://contoso.sharepoint.com/sites/team-a --pageName home.aspx --section 1",
        )],
        links: &[],
    },
    Help {
        name: "spo page set",
        summary: "Updates modern page properties",
        options: &[
            ("name", "Name of the page to update."),
            (
                "webUrl",
                "URL of the site where the page to update is located.",
            ),
            (
                "layoutType",
                "Layout of the page. Allowed values: `Article`, `Home`, `SingleWebPartAppPage`, `RepostPage`, `HeaderlessSearchResults`, `Spaces`, `Topic`.",
            ),
            (
                "promoteAs",
                "Update the page purpose. Allowed values: `HomePage`, `NewsPage`, `Template`.",
            ),
            (
                "demoteFrom",
                "Update the page purpose back to a regular article. Allowed values: `NewsPage`.",
            ),
            (
                "commentsEnabled",
                "Set to `true`, to enable comments on the page. Allowed values: `true`, `false`.",
            ),
            (
                "isRetired",
                "Marks the page as retired, showing a message that the page may be out of date. Allowed values: `true`, `false`.",
            ),
            ("publish", "Set to publish the page."),
            ("publishMessage", "Message to set when publishing the page."),
            ("description", "The description to set for the page."),
            ("title", "The title to set for the page."),
            ("content", "JSON string containing page content."),
        ],
        notes: &[
            "If you try to create a page with a name of a page that already exists, you will get a `The file doesn't exists` error.",
            "If you choose to promote the page using the `promoteAs` option or enable page comments, you will see the result only after publishing the page.",
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treat quotes differently. For example, this is how you can add page content from the Windows cmd.exe:",
            "    m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --content '[{\\\"controlType\\\": 4,\\\"id\\\": \\\"42b8afe8-dafe-4c11-bfbf-df5ef5b1feb7\\\",\\\"position\\\": {\\\"layoutIndex\\\": 1,\\\"zoneIndex\\\": 1,\\\"sectionIndex\\\": 1,\\\"sectionFactor\\\": 12,\\\"controlIndex\\\": 1},\\\"addedFromPersistedData\\\": true,\\\"innerHTML\\\":\\\"<p>Hello World</p>\\\"}]'",
            "Note, how the content option has escaped double quotes `'[{\\\"controlType\\\": 4,\\\"id\\\": \\\"42b8afe8-dafe-4c11-bfbf-df5ef5b1feb7\\\",\\\"position\\\": {\\\"layoutIndex\\\": 1,\\\"zoneIndex\\\": 1,\\\"sectionIndex\\\": 1,\\\"sectionFactor\\\": 12,\\\"controlIndex\\\": 1},\\\"addedFromPersistedData\\\": true,\\\"innerHTML\\\":\\\"Hello World\\\"}]'` compared to execution from bash `''[{\"controlType\": 4,\"id\": \"42b8afe8-dafe-4c11-bfbf-df5ef5b1feb7\",\"position\": {\"layoutIndex\": 1,\"zoneIndex\": 1,\"sectionIndex\": 1,\"sectionFactor\": 12,\"controlIndex\": 1},\"addedFromPersistedData\": true,\"innerHTML\":\"Hello World\"}]'`.",
            "When using the `--content` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Change the layout of the existing page to Article",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --layoutType Article",
            ),
            (
                "Promote the existing article page as a news article",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --promoteAs NewsPage",
            ),
            (
                "Promote the existing article page as a template",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --promoteAs Template",
            ),
            (
                "Demote the existing news page",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --demoteFrom NewsPage",
            ),
            (
                "Change the page's layout to Home and set it as the site's home page",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --layoutType Home --promoteAs HomePage",
            ),
            (
                "Enable comments on the existing page",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --commentsEnabled true",
            ),
            (
                "Publish existing page",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --publish",
            ),
            (
                "Set page description",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --description \"Description to add for the page\"",
            ),
            (
                "Marks a page as retired",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --isRetired true",
            ),
            (
                "Set page content",
                "m365 spo page set --name page.aspx --webUrl https://contoso.sharepoint.com/sites/a-team --content '[{\\\"controlType\\\": 4,\\\"id\\\": \\\"42b8afe8-dafe-4c11-bfbf-df5ef5b1feb7\\\",\\\"position\\\": {\\\"layoutIndex\\\": 1,\\\"zoneIndex\\\": 1,\\\"sectionIndex\\\": 1,\\\"sectionFactor\\\": 12,\\\"controlIndex\\\": 1},\\\"addedFromPersistedData\\\": true,\\\"innerHTML\\\":\\\"<p>Hello World</p>\\\"}]'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page template list",
        summary: "Lists all page templates in the given site",
        options: &[(
            "webUrl",
            "URL of the site from which to retrieve available pages.",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Lists all page templates in the given site",
            "m365 spo page template list --webUrl https://contoso.sharepoint.com/sites/team-a",
        )],
        links: &[],
    },
    Help {
        name: "spo page text add",
        summary: "Adds text to a modern page",
        options: &[
            (
                "webUrl",
                "URL of the site where the page to add the text to is located.",
            ),
            ("pageName", "Name of the page to which add the text."),
            ("text", "Text to add to the page"),
            (
                "section",
                "Number of the section to which the text should be added (1 or higher).",
            ),
            (
                "column",
                "Number of the column in which the text should be added (1 or higher).",
            ),
            (
                "order",
                "Order of the text in the column. Must be integer value of 1 or higher.",
            ),
        ],
        notes: &[
            "If the specified `pageName` doesn't refer to an existing modern page, you will get a File doesn't exists error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add text to a modern page in the first available location on the page",
                "m365 spo page text add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --text 'Hello world'",
            ),
            (
                "Add text to a modern page in the third column of the second section",
                "m365 spo page text add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --text 'Hello world' --section 2 --column 3",
            ),
            (
                "Add text at the beginning of the default column on a modern page",
                "m365 spo page text add --webUrl https://contoso.sharepoint.com/sites/a-team --pageName page.aspx --text 'Hello world' --order 1",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo page unpublish",
        summary: "Unpublishes a modern page",
        options: &[
            ("webUrl", "URL of the site where the page is located."),
            ("name", "Name of the page."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Write"),
            ("Application", "SharePoint", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Unpublish a modern page",
                "m365 spo page unpublish --webUrl https://contoso.sharepoint.com/sites/Marketing --name \"Style guide.aspx\"",
            ),
            (
                "Unpublish a modern page in a subfolder",
                "m365 spo page unpublish --webUrl https://contoso.sharepoint.com/sites/Marketing --name \"/Styles/Guide.aspx\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo propertybag get",
        summary: "Gets the value of the specified property from the property bag",
        options: &[
            (
                "webUrl",
                "The URL of the site from which the property bag value should be retrieved.",
            ),
            (
                "key",
                "Key of the property for which the value should be retrieved. Case-sensitive.",
            ),
            (
                "folder",
                "Site-relative URL of the folder from which to retrieve property bag value. Case-sensitive.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Returns the value of the property from the property bag located in the given site",
                "m365 spo propertybag get --webUrl https://contoso.sharepoint.com/sites/test --key key1",
            ),
            (
                "Returns the value of the property from the property bag located in root folder of the given site",
                "m365 spo propertybag get --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder /",
            ),
            (
                "Returns the value of the property from the property bag located in document library of the given site",
                "m365 spo propertybag get --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder '/Shared Documents'",
            ),
            (
                "Returns the value of the property from the property bag located in folder in a document library located in the given site",
                "m365 spo propertybag get --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder '/Shared Documents/MyFolder'",
            ),
            (
                "Returns the value of the property from the property bag located in a list in the given site",
                "m365 spo propertybag get --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder /Lists/MyList",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo propertybag list",
        summary: "Gets property bag values",
        options: &[
            (
                "webUrl",
                "The URL of the site from which the property bag value should be retrieved.",
            ),
            (
                "folder",
                "Site-relative URL of the folder from which to retrieve property bag value. Case-sensitive.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Return property bag values located in the given site",
                "m365 spo propertybag list --webUrl https://contoso.sharepoint.com/sites/test",
            ),
            (
                "Return property bag values located in the given site root folder",
                "m365 spo propertybag list --webUrl https://contoso.sharepoint.com/sites/test --folder /",
            ),
            (
                "Return property bag values located in the given site document library",
                "m365 spo propertybag list --webUrl https://contoso.sharepoint.com/sites/test --folder '/Shared Documents'",
            ),
            (
                "Return property bag values located in folder in the given site document library",
                "m365 spo propertybag list --webUrl https://contoso.sharepoint.com/sites/test --folder '/Shared Documents/MyFolder'",
            ),
            (
                "Return property bag values located in the given site list",
                "m365 spo propertybag list --webUrl https://contoso.sharepoint.com/sites/test --folder /Lists/MyList",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo propertybag remove",
        summary: "Removes specified property from the property bag",
        options: &[
            (
                "webUrl",
                "The URL of the site from which the property should be removed.",
            ),
            ("key", "Key of the property to be removed. Case-sensitive."),
            (
                "folder",
                "Site-relative URL of the folder from which to remove the property bag value.",
            ),
            (
                "force",
                "Don't prompt for confirming removal of property bag value.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the value of the property from the property bag located the given site",
                "m365 spo propertybag remove --webUrl https://contoso.sharepoint.com/sites/test --key key1",
            ),
            (
                "Removes the value of the property from the property bag located in the given site root folder",
                "m365 spo propertybag remove --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder / --force",
            ),
            (
                "Removes the value of the property from the property bag located in the given site document library",
                "m365 spo propertybag remove --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder '/Shared Documents'",
            ),
            (
                "Removes the value of the property from the property bag located in folder in the given site document library",
                "m365 spo propertybag remove --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder '/Shared Documents/MyFolder'",
            ),
            (
                "Removes the value of the property from the property bag located in the given site list",
                "m365 spo propertybag remove --webUrl https://contoso.sharepoint.com/sites/test --key key1 --folder /Lists/MyList",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo propertybag set",
        summary: "Sets the value of the specified property in the property bag. Adds the property if it does not exist",
        options: &[
            (
                "webUrl",
                "The URL of the site in which the property should be set",
            ),
            ("key", "Key of the property to be set. Case-sensitive"),
            ("value", "Value of the property to be set"),
            (
                "folder",
                "Site-relative URL of the folder on which the property should be set",
            ),
        ],
        notes: &[
            "If a site has NoScript enabled, setting property bag values may result in an Access is denied error. To resolve this, you can configure a tenant-wide property to allow property bag updates on NoScript sites.",
        ],
        permissions: &[],
        examples: &[
            (
                "Sets the value of the property in the property bag of the given site",
                "m365 spo propertybag set --webUrl https://contoso.sharepoint.com/sites/test --key key1 --value value1",
            ),
            (
                "Sets the value of the property in the property bag of the root folder of the given site",
                "m365 spo propertybag set --webUrl https://contoso.sharepoint.com/sites/test --key key1 --value value1 --folder /",
            ),
            (
                "Sets the value of the property in the property bag of a document library located in the given site",
                "m365 spo propertybag set --webUrl https://contoso.sharepoint.com/sites/test --key key1 --value value1 --folder '/Shared Documents'",
            ),
            (
                "Sets the value of the property in the property bag of a folder in a document library located in the given site",
                "m365 spo propertybag set --webUrl https://contoso.sharepoint.com/sites/test --key key1 --value value1 --folder '/Shared Documents/MyFolder'",
            ),
            (
                "Sets the value of the property in the property bag of a list in the given site",
                "m365 spo propertybag set --webUrl https://contoso.sharepoint.com/sites/test --key key1 --value value1 --folder /Lists/MyList",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report activityfilecounts",
        summary: "Gets the number of unique, licensed users who interacted with files stored on SharePoint sites",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of unique, licensed users who interacted with files stored on SharePoint sites for the last week",
                "m365 spo report activityfilecounts --period D7",
            ),
            (
                "Gets the number of unique, licensed users who interacted with files stored on SharePoint sites for the last week and exports the report data in the specified path in text format",
                "m365 spo report activityfilecounts --period D7 --output text > \"activityfilecounts.txt\"",
            ),
            (
                "Gets the number of unique, licensed users who interacted with files stored on SharePoint sites for the last week and exports the report data in the specified path in json format",
                "m365 spo report activityfilecounts --period D7 --output json > \"activityfilecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report activitypages",
        summary: "Gets the number of unique pages visited by users",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of unique pages visited by users for the last week",
                "m365 spo report activitypages --period D7",
            ),
            (
                "Gets the number of unique pages visited by users for the last week and exports the report data in the specified path in text format",
                "m365 spo report activitypages --period D7 --output text > \"activitypages.txt\"",
            ),
            (
                "Gets the number of unique pages visited by users for the last week and exports the report data in the specified path in json format",
                "m365 spo report activitypages --period D7 --output json > \"activitypages.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report activityusercounts",
        summary: "Gets the trend in the number of active users",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "A user is considered active if he or she has executed a file activity (save, sync, modify, or share) or visited a page within the specified time period",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the trend in the number of active users for the last week",
                "m365 spo report activityusercounts --period D7",
            ),
            (
                "Gets the trend in the number of active users for the last week and exports the report data in the specified path in text format",
                "m365 spo report activityusercounts --period D7 --output text > \"activityusercounts.txt\"",
            ),
            (
                "Gets the trend in the number of active users for the last week and exports the report data in the specified path in json format",
                "m365 spo report activityusercounts --period D7 --output json > \"activityusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report activityuserdetail",
        summary: "Gets details about SharePoint activity by user.",
        options: &[
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`. Specify either `date` or `period`, but not both.",
            ),
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`. Specify either `date` or `period`, but not both.",
            ),
        ],
        notes: &[
            "As this report is only available for the past 28 days, date parameter value should be a date from that range.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets details about SharePoint activity by user for the last week",
                "m365 spo report activityuserdetail --period D7",
            ),
            (
                "Gets details about SharePoint activity by user for May 1, 2019",
                "m365 spo report activityuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about SharePoint activity by user for the last week and exports the report data in the specified path in text format",
                "m365 spo report activityuserdetail --period D7 --output text > \"activityuserdetail.txt\"",
            ),
            (
                "Gets details about SharePoint activity by user for the last week and exports the report data in the specified path in json format",
                "m365 spo report activityuserdetail --period D7 --output json > \"activityuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report siteusagedetail",
        summary: "Gets details about SharePoint site usage",
        options: &[
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is `YYYY-MM-DD`. Specify either `date` or `period`, but not both.",
            ),
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`. Specify either `date` or `period`, but not both.",
            ),
        ],
        notes: &[
            "As this report is only available for the past 28 days, date parameter value should be a date from that range.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets details about SharePoint site usage for the last week",
                "m365 spo report siteusagedetail --period D7",
            ),
            (
                "Gets details about SharePoint site usage for May 1, 2019",
                "m365 spo report siteusagedetail --date 2019-05-01",
            ),
            (
                "Gets details about SharePoint site usage for the last week and exports the report data in the specified path in text format",
                "m365 spo report siteusagedetail --period D7 --output text > \"siteusagedetail.txt\"",
            ),
            (
                "Gets details about SharePoint site usage for the last week and exports the report data in the specified path in json format",
                "m365 spo report siteusagedetail --period D7 --output json > \"siteusagedetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report siteusagefilecounts",
        summary: "Gets the total number of files across all sites and the number of active files",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "A file (user or system) is considered active if it has been saved, synced, modified, or shared within the specified time period.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get the total number of files across all sites and the number of active files for the last week",
                "m365 spo report siteusagefilecounts --period D7",
            ),
            (
                "Get the total number of files across all sites and the number of active files for the last week and exports the report data in the specified path in text format",
                "m365 spo report siteusagefilecounts --period D7 --output text > \"siteusagefilecounts.txt\"",
            ),
            (
                "Get the total number of files across all sites and the number of active files for the last week and exports the report data in the specified path in json format",
                "m365 spo report siteusagefilecounts --period D7 --output json > \"siteusagefilecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report siteusagepages",
        summary: "Gets the number of pages viewed across all sites",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of pages viewed across all sites for the last week",
                "m365 spo report siteusagepages --period D7",
            ),
            (
                "Gets the number of pages viewed across all sites for the last week and exports the report data in the specified path in text format",
                "m365 spo report siteusagepages --period D7 --output text > \"siteusagepages.txt\"",
            ),
            (
                "Gets the number of pages viewed across all sites for the last week and exports the report data in the specified path in json format",
                "m365 spo report siteusagepages --period D7 --output json > \"siteusagepages.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report siteusagesitecounts",
        summary: "Gets the total number of files across all sites and the number of active files",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "A file (user or system) is considered active if it has been saved, synced, modified, or shared within the specified time period.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the total number of files across all sites and the number of active files for the last week",
                "m365 spo report siteusagesitecounts --period D7",
            ),
            (
                "Gets the total number of files across all sites and the number of active files for the last week and exports the report data in the specified path in text format",
                "m365 spo report siteusagesitecounts --period D7 --output text > \"siteusagesitecounts.txt\"",
            ),
            (
                "Gets the total number of files across all sites and the number of active files for the last week and exports the report data in the specified path in json format",
                "m365 spo report siteusagesitecounts --period D7 --output json > \"siteusagesitecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo report siteusagestorage",
        summary: "Gets the trend of storage allocated and consumed during the reporting period",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trend of storage allocated and consumed during the last week",
                "m365 spo report siteusagestorage --period D7",
            ),
            (
                "Gets the trend of storage allocated and consumed during the last week and exports the report data in the specified path in text format",
                "m365 spo report siteusagestorage --period D7 --output text > \"siteusagestorage.txt\"",
            ),
            (
                "Gets the trend of storage allocated and consumed during the last week and exports the report data in the specified path in json format",
                "m365 spo report siteusagestorage --period D7 --output json > \"siteusagestorage.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo roledefinition add",
        summary: "Adds a new roledefinition to web",
        options: &[
            ("webUrl", "URL of the site to which role should be added."),
            ("name", "role definition name."),
            ("description", "role definition description."),
            (
                "rights",
                "A case-sensitive string array that contain the permissions needed for the custom action. Allowed values `EmptyMask`, `ViewListItems`, `AddListItems`, `EditListItems`, `DeleteListItems`, `ApproveItems`, `OpenItems`, `ViewVersions`, `DeleteVersions`, `CancelCheckout`, `ManagePersonalViews`, `ManageLists`, `ViewFormPages`, `AnonymousSearchAccessList`, `Open`, `ViewPages`, `AddAndCustomizePages`, `ApplyThemeAndBorder`, `ApplyStyleSheets`, `ViewUsageData`, `CreateSSCSite`, `ManageSubwebs`, `CreateGroups`, `ManagePermissions`, `BrowseDirectories`, `BrowseUserInfo`, `AddDelPrivateWebParts`,`UpdatePersonalWebParts`, `ManageWeb`, `AnonymousSearchAccessWebLists`, `UseClientIntegration`, `UseRemoteAPIs`, `ManageAlerts`, `CreateAlerts`, `EditMyUserInfo`, `EnumeratePermissions`, `FullMask`. Default `EmptyMask`.",
            ),
        ],
        notes: &["The `--rights` option accepts case-sensitive values."],
        permissions: &[],
        examples: &[
            (
                "Adds the role definition for the given site",
                "m365 spo roledefinition add --webUrl https://contoso.sharepoint.com/sites/project-x --name test",
            ),
            (
                "Adds the role definition for the given site with description and rights",
                "m365 spo roledefinition add --webUrl https://contoso.sharepoint.com/sites/project-x --name test --description \"test description\" --rights \"ViewListItems,AddListItems,EditListItems,DeleteListItems\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo roledefinition get",
        summary: "Gets specified role definition from web",
        options: &[
            (
                "webUrl",
                "URL of the site for which to retrieve the role definition.",
            ),
            ("id", "The Id of the role definition to retrieve."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Retrieve the role definition for the given site",
            "m365 spo roledefinition get --webUrl https://contoso.sharepoint.com/sites/project-x --id 1",
        )],
        links: &[],
    },
    Help {
        name: "spo roledefinition list",
        summary: "Gets list of role definitions for the specified site",
        options: &[(
            "webUrl",
            "URL of the site for which to retrieve role definitions.",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Return list of role definitions for the given site",
            "m365 spo roledefinition list --webUrl https://contoso.sharepoint.com/sites/project-x",
        )],
        links: &[],
    },
    Help {
        name: "spo roledefinition remove",
        summary: "Removes the role definition from the specified site",
        options: &[
            (
                "webUrl",
                "URL of the site from which role should be removed.",
            ),
            ("id", "ID of the role definition to remove."),
            (
                "force",
                "Don't prompt for confirming removing the role definition.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove the role definition from the given site",
                "m365 spo roledefinition remove --webUrl https://contoso.sharepoint.com/sites/project-x --id 1",
            ),
            (
                "Remove the role definition from the given site and don't prompt for confirmation",
                "m365 spo roledefinition remove --webUrl https://contoso.sharepoint.com/sites/project-x --id 1 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo search",
        summary: "Executes a search query",
        options: &[
            ("queryText", "Query to be executed in KQL format"),
            (
                "selectProperties",
                "Comma-separated list of properties to retrieve. Will retrieve all properties if not specified and json output is requested.",
            ),
            (
                "webUrl",
                "The web against which we want to execute the query. If the parameter is not defined, the query is executed against the web that's used when logging in to the SPO environment.",
            ),
            (
                "allResults",
                "Set, to get all results of the search query in batches of 500.",
            ),
            (
                "rowLimit",
                "The number of rows to be returned. When the `allResults` option is used, the specified value will define the size of retrieved batches",
            ),
            (
                "sourceId",
                "The identifier (GUID) of the result source to be used to run the query.",
            ),
            (
                "trimDuplicates",
                "Set, to remove near duplicate items from the search results.",
            ),
            ("enableStemming", "Set, to enable stemming."),
            ("culture", "The locale for the query."),
            (
                "refinementFilters",
                "The set of refinement filters used when issuing a refinement query.",
            ),
            (
                "queryTemplate",
                "A string that contains the text that replaces the query text, as part of a query transformation.",
            ),
            (
                "sortList",
                "The list of properties by which the search results are ordered.",
            ),
            (
                "rankingModelId",
                "The ID of the ranking model to use for the query.",
            ),
            (
                "startRow",
                "The first row that is included in the search results that are returned. You use this parameter when you want to implement manual paging for search results.",
            ),
            ("properties", "Additional properties for the query."),
            (
                "sourceName",
                "Specified the name of the result source to be used to run the query.",
            ),
            (
                "refiners",
                "The set of refiners to return in a search result.",
            ),
            (
                "hiddenConstraints",
                "The additional query terms to append to the query.",
            ),
            (
                "clientType",
                "The type of the client that issued the query.",
            ),
            (
                "enablePhonetic",
                "Set, to use the phonetic forms of the query terms to find matches. (Default = `false`).",
            ),
            (
                "processBestBets",
                "Set, to return best bet results for the query.",
            ),
            (
                "enableQueryRules",
                "Set, to enable query rules for the query.",
            ),
            (
                "processPersonalFavorites",
                "Set, to return personal favorites with the search results.",
            ),
            (
                "rawOutput",
                "Set, to return the unparsed, raw results of the REST call to the search API.",
            ),
        ],
        notes: &[
            "When using the `--allResults` option, you cannot use the `--startRow` option. Only use `--startRow` for manual paging purposes.",
        ],
        permissions: &[],
        examples: &[
            (
                "Execute search query to retrieve all Document Sets (ContentTypeId = 0x0120D520) for the English locale",
                "m365 spo search --queryText \"ContentTypeId:0x0120D520\" --culture 1033",
            ),
            (
                "Retrieve all documents. For each document, retrieve the Path, Author and FileType.",
                "m365 spo search --queryText \"IsDocument:1\" --selectProperties \"Path,Author,FileType\" --allResults",
            ),
            (
                "Return the top 50 items of which the title starts with Marketing while trimming duplicates.",
                "m365 spo search --queryText \"Title:Marketing*\" --rowLimit=50 --trimDuplicates",
            ),
            (
                "Return only items from a specific result source (using the source id).",
                "m365 spo search --queryText \"*\" --sourceId \"6e71030e-5e16-4406-9bff-9c1829843083\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal grant add",
        summary: "Grants the service principal permission to the specified API",
        options: &[
            (
                "resource",
                "The name of the resource for which permissions should be granted.",
            ),
            (
                "scope",
                "The name of the permission that should be granted.",
            ),
        ],
        notes: &["To use this command you must be a Global administrator."],
        permissions: &[],
        examples: &[
            (
                "Grant the service principal permission to read email using the Microsoft Graph",
                "m365 spo serviceprincipal grant add --resource 'Microsoft Graph' --scope 'Mail.Read'",
            ),
            (
                "Grant the service principal permission to a custom API",
                "m365 spo serviceprincipal grant add --resource 'contoso-api' --scope 'user_impersonation'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal grant list",
        summary: "Lists permissions granted to the service principal",
        options: &[],
        notes: &["To use this command you must be a Global administrator."],
        permissions: &[],
        examples: &[(
            "List all permissions granted to the service principal",
            "m365 spo serviceprincipal grant list",
        )],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal grant revoke",
        summary: "Revokes the specified set of permissions granted to the service principal",
        options: &[
            ("id", "`ObjectId` of the permission grant to revoke."),
            (
                "scope",
                "Scope to revoke. If not specified, will revoke all permissions.",
            ),
        ],
        notes: &[
            "To use this command you must be a Global administrator.",
            "The permission grant you want to revoke is denoted using its `ObjectId`. You can retrieve it using the spo serviceprincipal grant list command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Revoke permission grant with all permissions",
                "m365 spo serviceprincipal grant revoke --id 50NAzUm3C0K9B6p8ORLtIsQccg4rMERGvFGRtBsk2fA",
            ),
            (
                "Revoke a specific permission",
                "m365 spo serviceprincipal grant revoke --id 50NAzUm3C0K9B6p8ORLtIsQccg4rMERGvFGRtBsk2fA --scope \"Mail.Read\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal permissionrequest approve",
        summary: "Approves the specified permission request",
        options: &[
            ("id", "ID of the permission request to approve."),
            ("all", "approve all pending permission requests."),
            (
                "resource",
                "The resource of the permissions requests to approve.",
            ),
        ],
        notes: &[
            "The admin role that's required to approve permissions depends on the API. To approve permissions to any of the third-party APIs registered in the tenant, the application administrator role is sufficient. To approve permissions for Microsoft Graph or any other Microsoft API, the Global Administrator role is required.",
            "The permission request you want to approve is denoted using its `ID`. You can retrieve it using the spo serviceprincipal permissionrequest list command.",
        ],
        permissions: &[],
        examples: &[(
            "Approve permission request",
            "m365 spo serviceprincipal permissionrequest approve --id 4dc4c043-25ee-40f2-81d3-b3bf63da7538",
        )],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal permissionrequest deny",
        summary: "Denies the specified permission request",
        options: &[("id", "ID of the permission request to deny.")],
        notes: &[
            "The admin role that's required to deny permissions depends on the API. To approve permissions to any of the third-party APIs registered in the tenant, the application administrator role is sufficient. To approve permissions for Microsoft Graph or any other Microsoft API, the Global Administrator role is required.",
            "The permission request you want to approve is denoted using its `ID`. You can retrieve it using the spo serviceprincipal permissionrequest list command.",
        ],
        permissions: &[],
        examples: &[(
            "Deny permission request",
            "m365 spo serviceprincipal permissionrequest deny --id 4dc4c043-25ee-40f2-81d3-b3bf63da7538",
        )],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal permissionrequest list",
        summary: "Lists pending permission requests",
        options: &[],
        notes: &[
            "The admin role that's required to list permissions depends on the API. To approve permissions to any of the third-party APIs registered in the tenant, the application administrator role is sufficient. To approve permissions for Microsoft Graph or any other Microsoft API, the Global Administrator role is required.",
        ],
        permissions: &[],
        examples: &[(
            "List all pending permission requests",
            "m365 spo serviceprincipal permissionrequest list",
        )],
        links: &[],
    },
    Help {
        name: "spo serviceprincipal set",
        summary: "Enables or disables the service principal",
        options: &[
            (
                "enabled",
                "Set to `true` to enable the service principal or to `false` to disable it. Valid values are `true`, `false`.",
            ),
            (
                "force",
                "Don't prompt for confirming enabling/disabling the service principal.",
            ),
        ],
        notes: &[
            "To use this command you must be a Global administrator.",
            "Using the `-e, --enabled` option you can specify whether the service principal should be enabled or disabled. Use `true` to enable the service principal and `false` to disable it.",
        ],
        permissions: &[],
        examples: &[
            (
                "Enable the service principal. Will prompt for confirmation",
                "m365 spo serviceprincipal set --enabled true",
            ),
            (
                "Disable the service principal. Will prompt for confirmation",
                "m365 spo serviceprincipal set --enabled false",
            ),
            (
                "Enable the service principal without prompting for confirmation",
                "m365 spo serviceprincipal set --enabled true --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo set",
        summary: "Sets the URL of the root SharePoint site collection for use in SPO commands",
        options: &[(
            "url",
            "The URL of the root SharePoint site collection to use in SPO commands",
        )],
        notes: &[
            "CLI for Microsoft 365 automatically discovers the URL of the root SharePoint site collection/SharePoint tenant admin site (whichever is needed to run the particular command). In specific cases, like when managing multi-geo Microsoft 365 tenants, it could be desirable to make the CLI manage the specific geography. For such cases, you can use this command to explicitly specify the SPO URL that should be used when executing SPO commands.",
        ],
        permissions: &[],
        examples: &[(
            "Set SPO URL to the specified URL",
            "m365 spo set --url https://contoso.sharepoint.com",
        )],
        links: &[],
    },
    Help {
        name: "spo site accessrequest setting set",
        summary: "Updates access requests for a specific site",
        options: &[
            ("siteUrl", "URL of the site collection."),
            ("disabled", "Disable site requests for the entire site."),
            ("ownerGroup", "Send access requests to the owner group."),
            ("email", "Send access requests to a specific email."),
            (
                "message",
                "Add a custom message to the access request page. Cannot be used with the `--disabled` option.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Disable access requests for a specific site.",
                "m365 spo site accessrequest setting set --siteUrl https://contoso.sharepoint.com/sites/Management --disabled",
            ),
            (
                "Send all access requests to the owner group of a specific site.",
                "m365 spo site accessrequest setting set --siteUrl https://contoso.sharepoint.com/sites/Management --ownerGroup",
            ),
            (
                "Send all access requests to a specific email address and set a custom message.",
                "m365 spo site accessrequest setting set --siteUrl https://contoso.sharepoint.com/sites/Management --email john.doe@contoso.com --message \"Motivate why you need access.\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site add",
        summary: "Creates new SharePoint Online site",
        options: &[
            (
                "type",
                "Type of sites to add. Allowed values `TeamSite`, `CommunicationSite`, `ClassicSite`, `BrandCenter`, default `TeamSite`",
            ),
            ("title", "Site title"),
            (
                "alias",
                "Site alias, used in the URL and in the team site group e-mail (applies to type TeamSite)",
            ),
            (
                "url",
                "Site URL (applies to type CommunicationSite, ClassicSite, BrandCenter)",
            ),
            (
                "timeZone",
                "Integer representing time zone to use for the site (applies to type ClassicSite)",
            ),
            ("description", "Site description"),
            (
                "lcid",
                "Site language in the LCID format, eg. 1033 for en-US. See SharePoint documentation for the list of supported languages",
            ),
            (
                "owners",
                "Comma-separated list of users to set as site owners (applies to type TeamSite, ClassicSite)",
            ),
            (
                "isPublic",
                "Determines if the associated group is public or not (applies to type TeamSite)",
            ),
            (
                "classification",
                "Site classification (applies to type TeamSite, CommunicationSite, BrandCenter)",
            ),
            (
                "siteDesign",
                "Type of communication site to create. Allowed values `Topic`, `Showcase`, `Blank`, default `Topic`. When creating a communication site, specify either `siteDesign` or `siteDesignId` (applies to type CommunicationSite)",
            ),
            (
                "siteDesignId",
                "Id of the custom site design to use to create the site. When creating a communication site, specify either `siteDesign` or `siteDesignId` (applies to type CommunicationSite)",
            ),
            (
                "shareByEmailEnabled",
                "Determines whether it's allowed to share file with guests (applies to type CommunicationSite, BrandCenter)",
            ),
            (
                "webTemplate",
                "Template to use for creating the site. Default `STS#0` (applies to type ClassicSite)",
            ),
            (
                "resourceQuota",
                "The quota for this site collection in Sandboxed Solutions units. Default `0` (applies to type ClassicSite)",
            ),
            (
                "resourceQuotaWarningLevel",
                "The warning level for the resource quota. Default `0` (applies to type ClassicSite)",
            ),
            (
                "storageQuota",
                "The storage quota for this site collection in megabytes. Default `100` (applies to type ClassicSite)",
            ),
            (
                "storageQuotaWarningLevel",
                "The warning level for the storage quota in megabytes. Default `100` (applies to type ClassicSite)",
            ),
            (
                "removeDeletedSite",
                "Set, to remove existing deleted site with the same URL from the Recycle Bin (applies to type ClassicSite)",
            ),
            (
                "withAppCatalog",
                "Set, to create a site collection app catalog on the newly created site.",
            ),
            (
                "wait",
                "Wait for the site to be provisioned before completing the command (applies to type ClassicSite)",
            ),
            (
                "force",
                "Don't prompt for confirmation to create BrandCenter",
            ),
        ],
        notes: &[
            "Remarks for classic sites:",
            "Using the `-z, --timeZone` option you have to specify the time zone of the site. For more information about the valid values see https://msdn.microsoft.com/library/microsoft.sharepoint.spregionalsettings.timezones.aspx.",
            "The value of the `--resourceQuota` option must not exceed the company's aggregate available Sandboxed Solutions quota. For more information, see Resource Usage Limits on Sandboxed Solutions in SharePoint 2010: http://msdn.microsoft.com/en-us/library/gg615462.aspx.",
            "The value of the `--resourceQuotaWarningLevel` option must not exceed the value of the `--resourceQuota` option.",
            "The value of the `--storageQuota` option must not exceed the company's available quota.",
            "The value of the `--storageQuotaWarningLevel` option must not exceed the the value of the `--storageQuota` option.",
            "If you try to create a site with the same URL as a site that has been previously moved to the recycle bin, you will get an error. To avoid this error, you can use the `--removeDeletedSite` option. Prior to creating the site, the spo site add command will check if the site with the specified URL has been previously moved to the recycle bin and if so, will remove it. Because removing sites from the recycle bin might take a moment, it should be used in conjunction with the `--wait` option so that the new site is not created before the old site is fully removed.",
            "Deleting and creating classic site collections is by default asynchronous and depending on the current state of Office 365, might take up to few minutes. If you're building a script with steps that require the site to be fully provisioned, you should use the `--wait` flag. When using this flag, the spo site add command will keep running until it received confirmation from Office 365 that the site has been fully provisioned.",
            "Remarks for modern sites:",
            "The `--owners` option is mandatory for creating `CommunicationSite or `BrandCenter` sites with app-only permissions.",
            "When trying to create a team site using app-only permissions, you will get an Insufficient privileges to complete the operation. error. As a workaround, you can use the `entra m365group add` command, followed by `spo site set` to further configure the Team site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Create modern team site with private group",
                "m365 spo site add --alias team1 --title \"Team 1\"",
            ),
            (
                "Create modern team site with description and classification",
                "m365 spo site add --type TeamSite --alias team1 --title \"Team 1\" --description \"Site of team 1\" --classification LBI",
            ),
            (
                "Create modern team site with public group",
                "m365 spo site add --type TeamSite --alias team1 --title \"Team 1\" --isPublic",
            ),
            (
                "Create modern team site using the Dutch language",
                "m365 spo site add --alias team1 --title \"Team 1\" --lcid 1043",
            ),
            (
                "Create modern team site with the specified users as owners",
                "m365 spo site add --alias team1 --title \"Team 1\" --owners \"steve@contoso.com, bob@contoso.com\"",
            ),
            (
                "Create a modern team site and enable the site collection app catalog",
                "m365 spo site add --alias team1 --title \"Team 1\" --withAppCatalog",
            ),
            (
                "Create communication site using the Topic design",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing",
            ),
            (
                "Create communication site using app-only permissions",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing --owners \"john.smith@contoso.com\"",
            ),
            (
                "Create communication site using the Showcase design",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing --siteDesign Showcase",
            ),
            (
                "Create communication site using a custom site design",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing --siteDesignId 99f410fe-dd79-4b9d-8531-f2270c9c621c",
            ),
            (
                "Create communication site using the Blank design with description and classification",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing --description Site of the marketing department --classification MBI --siteDesign Blank",
            ),
            (
                "Create a communication site and enable the site collection app catalog",
                "m365 spo site add --type CommunicationSite --url https://contoso.sharepoint.com/sites/marketing --title Marketing --withAppCatalog",
            ),
            (
                "Create a new classic site collection using the Team site template. Set time zone to `UTC+01:00`. Don't wait for the site provisioning to complete",
                "m365 spo site add --type ClassicSite --url https://contoso.sharepoint.com/sites/team --title Team --owners admin@contoso.onmicrosoft.com --timeZone 4",
            ),
            (
                "Create a new classic site collection using the Team site template. Set time zone to `UTC+01:00`. Wait for the site provisioning to complete",
                "m365 spo site add --type ClassicSite --url https://contoso.sharepoint.com/sites/team --title Team --owners admin@contoso.onmicrosoft.com --timeZone 4 --webTemplate STS#0 --wait",
            ),
            (
                "Create a new classic site collection using the Team site template. Set time zone to `UTC+01:00`. If a site with the same URL is in the recycle bin, delete it. Wait for the site provisioning to complete",
                "m365 spo site add --type ClassicSite --url https://contoso.sharepoint.com/sites/team --title Team --owners admin@contoso.onmicrosoft.com --timeZone 4 --webTemplate STS#0 --removeDeletedSite --wait",
            ),
            (
                "Create a new classic site and enable the site collection app catalog",
                "m365 spo site add --type ClassicSite --url https://contoso.sharepoint.com/sites/team --title Team --owners admin@contoso.onmicrosoft.com --timeZone 4 --webTemplate STS#0 --withAppCatalog",
            ),
            (
                "Create a brand center site",
                "m365 spo site add --type BrandCenter --url https://contoso.sharepoint.com/sites/brandcenter --title Branding",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/apis/communication-site-creation-rest",
            "https://learn.microsoft.com/sharepoint/dev/apis/communication-site-creation-rest",
        )],
    },
    Help {
        name: "spo site admin add",
        summary: "Adds a user or group as a site collection administrator",
        options: &[
            ("siteUrl", "The URL of the SharePoint site"),
            (
                "userId",
                "The ID of the user to add as a site collection admin",
            ),
            (
                "userName",
                "The user principal name of the user to add as a site collection admin",
            ),
            (
                "groupId",
                "The ID of the Microsoft Entra ID group to add as a site collection admin",
            ),
            (
                "groupName",
                "The name of the Microsoft Entra ID group to add as a site collection admin",
            ),
            (
                "primary",
                "If set, will add the user as primary site collection admin. The old primary site collection admin will be replaced and set as secondary site collection admin",
            ),
            (
                "asAdmin",
                "If specified, we will use the SharePoint admin center to execute the command",
            ),
        ],
        notes: &[
            "To use this command with the `--asAdmin` mode, you have to have permissions to access the tenant admin site.",
            "Without this parameter, you have to have site collection admin permissions for the requested site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add user as primary site collection administrator",
                "m365 spo site admin add --siteUrl https://contoso.sharepoint.com --userId 600713c5-53c6-4f24-b454-3c35e22b2639 --primary",
            ),
            (
                "Adds group as secondary site collection administrator as SharePoint admin",
                "m365 spo site admin add --siteUrl https://contoso.sharepoint.com --groupName SP_Administrators --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site admin list",
        summary: "Lists all administrators of a specific SharePoint site",
        options: &[
            ("siteUrl", "The URL of the SharePoint site"),
            (
                "asAdmin",
                "List admins as admin for sites you don't have permission to",
            ),
        ],
        notes: &[
            "To use this command with the `--asAdmin` mode, you must be at least SharePoint administrator.",
            "Without this parameter, you must have site collection admin permissions for the requested site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Lists all admins of a SharePoint site",
                "m365 spo site admin list --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "Lists all admins of a SharePoint site as admin",
                "m365 spo site admin list --siteUrl https://contoso.sharepoint.com --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site admin remove",
        summary: "Removes a user or group as site collection administrator",
        options: &[
            ("siteUrl", "The URL of the SharePoint site"),
            (
                "userId",
                "The ID of the user to remove as a site collection admin",
            ),
            (
                "userName",
                "The user principal name of the user to remove as a site collection admin",
            ),
            (
                "groupId",
                "The ID of the Microsoft Entra ID group to remove as a site collection admin",
            ),
            (
                "groupName",
                "The name of the Microsoft Entra ID group to remove as a site collection admin",
            ),
            (
                "asAdmin",
                "If specified, we will use the SharePoint admin center to execute the command",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[
            "To use this command with the `--asAdmin` mode, you have to have permissions to access the tenant admin site.",
            "Without this parameter, you have to have site collection admin permissions for the requested site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes user as site collection admin",
                "m365 spo site admin remove --siteUrl https://contoso.sharepoint.com --userId 600713c5-53c6-4f24-b454-3c35e22b2639",
            ),
            (
                "Removes group as site collection admin without prompting for confirmation as SharePoint Admin",
                "m365 spo site admin remove --siteUrl https://contoso.sharepoint.com --groupName SP_Administrators --force --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site appcatalog add",
        summary: "Creates a site collection app catalog in the specified site",
        options: &[(
            "siteUrl",
            "URL of the site collection where the app catalog should be added",
        )],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Add a site collection app catalog to the specified site",
            "m365 spo site appcatalog add --siteUrl https://contoso.sharepoint.com/sites/site",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/general-development/site-collection-app-catalog",
            "https://learn.microsoft.com/sharepoint/dev/general-development/site-collection-app-catalog",
        )],
    },
    Help {
        name: "spo site appcatalog list",
        summary: "Lists all site collection app catalogs within the tenant",
        options: &[(
            "excludeDeletedSites",
            "Exclude site collections that are no longer active",
        )],
        notes: &[
            "To use this command you need to have at least read permissions on the SharePoint root site.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all site collection app catalogs within the tenant",
                "m365 spo site appcatalog list",
            ),
            (
                "List all site collection app catalogs within the tenant and exclude deleted sites.",
                "m365 spo site appcatalog list --excludeDeletedSites",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site appcatalog remove",
        summary: "Removes site collection app catalog from the specified site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection containing the app catalog to disable.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "While the command uses the term 'remove', like its equivalent PowerShell cmdlet, it does not remove the special library Apps for SharePoint from the site collection. Instead, it disables the site collection app catalog in that site. Packages deployed to the app catalog are not available within the site collection.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove the site collection app catalog from specified site.",
                "m365 spo site appcatalog remove --siteUrl https://contoso.sharepoint/sites/site",
            ),
            (
                "Remove the site collection app catalog from specified site without prompting for confirmation.",
                "m365 spo site appcatalog remove --siteUrl https://contoso.sharepoint/sites/site --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/general-development/site-collection-app-catalog",
            "https://learn.microsoft.com/sharepoint/dev/general-development/site-collection-app-catalog",
        )],
    },
    Help {
        name: "spo site apppermission add",
        summary: "Adds a specific application permissions to the site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection to add the permission",
            ),
            (
                "permission",
                "Permission to site (`read`, `write`, `manage` or `fullcontrol`)",
            ),
            (
                "appId",
                "Client ID of the Microsoft Entra app for which to grant permissions",
            ),
            (
                "appDisplayName",
                "Display name of the Microsoft Entra app for which to grant permissions",
            ),
        ],
        notes: &[
            "To set permissions, specify at minimum either `appId` or `appDisplayName`. For best performance specify both values to avoid extra lookup.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Sites.FullControl.All, Application.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Sites.FullControl.All, Application.Read.All",
            ),
        ],
        examples: &[
            (
                "Grants the app with the specified display name the read permission to a specific site",
                "m365 spo site apppermission add --siteUrl https://contoso.sharepoint.com/sites/project-x --permission read --appDisplayName Foo",
            ),
            (
                "Grants the app with the specified ID the write permission to a specific site",
                "m365 spo site apppermission add --siteUrl https://contoso.sharepoint.com/sites/project-x --permission write --appId 5c89fbbf-670e-48e7-a0bc-fa8942c895a2",
            ),
            (
                "Grants the app with the specified ID and display name the manage permission to a specific site",
                "m365 spo site apppermission add --siteUrl https://contoso.sharepoint.com/sites/project-x --permission manage --appId 5c89fbbf-670e-48e7-a0bc-fa8942c895a2 --appDisplayName Foo",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site apppermission get",
        summary: "Gets a specific application permissions for the site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection where the permission to retrieve is located",
            ),
            ("id", "ID of the permission to retrieve"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Return a specific application permissions for the https://contoso.sharepoint.com/sites/project-x site collection with permission id aTowaS50fG1zLnNwLmV4dHw4OWVhNWM5NC03NzM2LTRlMjUtOTVhZC0zZmE5NWY2MmI2NmVAZGUzNDhiYzctMWFlYi00NDA2LThjYjMtOTdkYjAyMWNhZGI0",
            "m365 spo site apppermission get --siteUrl https://contoso.sharepoint.com/sites/project-x --id aTowaS50fG1zLnNwLmV4dHw4OWVhNWM5NC03NzM2LTRlMjUtOTVhZC0zZmE5NWY2MmI2NmVAZGUzNDhiYzctMWFlYi00NDA2LThjYjMtOTdkYjAyMWNhZGI0",
        )],
        links: &[],
    },
    Help {
        name: "spo site apppermission list",
        summary: "Lists application permissions for a site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection to retrieve information for",
            ),
            ("appId", "Id of the application to filter by"),
            (
                "appDisplayName",
                "Display name of the application to filter by",
            ),
        ],
        notes: &["To filter by an app, pass in either `appId` or `appDisplayName` not both"],
        permissions: &[],
        examples: &[
            (
                "Return list of application permissions for the https://contoso.sharepoint.com/sites/project-x site collection.",
                "m365 spo site apppermission list --siteUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Return list of application permissions for the https://contoso.sharepoint.com/sites/project-x site collection and filter by an application called Foo",
                "m365 spo site apppermission list --siteUrl https://contoso.sharepoint.com/sites/project-x --appDisplayName Foo",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site apppermission remove",
        summary: "Removes a specific application permission from a site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection where the permission to remove is located",
            ),
            ("appId", "App Id"),
            ("appDisplayName", "App display name"),
            ("id", "ID of the permission to remove"),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes all application permissions for application with id 89ea5c94-7736-4e25-95ad-3fa95f62b66e on site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo site apppermission remove --siteUrl https://contoso.sharepoint.com/sites/project-x --appId 89ea5c94-7736-4e25-95ad-3fa95f62b66e",
            ),
            (
                "Removes all application permissions for application named Foo on site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo site apppermission remove --siteUrl https://contoso.sharepoint.com/sites/project-x --appDisplayName Foo",
            ),
            (
                "Removes the application permission with the specified ID on site https://contoso.sharepoint.com/sites/project-x",
                "m365 spo site apppermission remove --siteUrl https://contoso.sharepoint.com/sites/project-x --id aTowaS50fG1zLnNwLmV4dHw4OWVhNWM5NC03NzM2LTRlMjUtOTVhZC0zZmE5NWY2MmI2NmVAZGUzNDhiYzctMWFlYi00NDA2LThjYjMtOTdkYjAyMWNhZGI0",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site apppermission set",
        summary: "Updates a specific application permission for a site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection where the permission to retrieve is located",
            ),
            ("id", "ID of the permission to update."),
            (
                "appId",
                "Client ID of the Microsoft Entra app for which to update permissions.",
            ),
            (
                "appDisplayName",
                "Display name of the Microsoft Entra app for which to update permissions.",
            ),
            (
                "permission",
                "Permission to site (`read`, `write`, `manage` or `fullcontrol`)",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Updates a specific application permission to read for the https://contoso.sharepoint.com/sites/project-x site collection with permission id aTowaS50fG1zLnNwLmV4dHw4OWVhNWM5NC03NzM2LTRlMjUtOTVhZC0zZmE5NWY2MmI2NmVAZGUzNDhiYzctMWFlYi00NDA2LThjYjMtOTdkYjAyMWNhZGI0",
                "m365 spo site apppermission set --siteUrl https://contoso.sharepoint.com/sites/project-x --id aTowaS50fG1zLnNwLmV4dHw4OWVhNWM5NC03NzM2LTRlMjUtOTVhZC0zZmE5NWY2MmI2NmVAZGUzNDhiYzctMWFlYi00NDA2LThjYjMtOTdkYjAyMWNhZGI0 --permission read",
            ),
            (
                "Updates a specific application permission to read for the https://contoso.sharepoint.com/sites/project-x site collection with an application called Foo",
                "m365 spo site apppermission set --siteUrl https://contoso.sharepoint.com/sites/project-x --appDisplayName Foo --permission read",
            ),
            (
                "Updates a specific application permission to read for the https://contoso.sharepoint.com/sites/project-x site collection with an application id 89ea5c94-7736-4e25-95ad-3fa95f62b66e",
                "m365 spo site apppermission set --siteUrl https://contoso.sharepoint.com/sites/project-x --appId 89ea5c94-7736-4e25-95ad-3fa95f62b66e --permission read",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site archive",
        summary: "Archives a site collection",
        options: &[
            ("url", "URL of the site collection."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command, you must be a Global or SharePoint administrator.",
            "After running this command, it may take a few minutes for the site to be fully archived.",
        ],
        permissions: &[],
        examples: &[
            (
                "Archive a specific SharePoint site collection",
                "m365 spo site archive --url \"https://contoso.sharepoint.com/sites/Marketing\"",
            ),
            (
                "Archive a specific SharePoint site collection without confirmation prompt",
                "m365 spo site archive --url \"https://contoso.sharepoint.com/sites/Marketing\" --force",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/microsoft-365/archive/archive-pricing",
                "https://learn.microsoft.com/microsoft-365/archive/archive-pricing",
            ),
            (
                "https://learn.microsoft.com/microsoft-365/archive/archive-setup",
                "https://learn.microsoft.com/microsoft-365/archive/archive-setup",
            ),
        ],
    },
    Help {
        name: "spo site chrome set",
        summary: "Sets the chrome header and footer for the specified site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection to which you want to change the chrome header/footer",
            ),
            (
                "headerLayout",
                "Header layout to set on the site. Options: `Standard|Compact|Minimal|Extended`.",
            ),
            (
                "headerEmphasis",
                "Header background to set on the site. Options: `Lightest|Light|Dark|Darkest`.",
            ),
            (
                "logoAlignment",
                "Logo position when header layout set to `Extended`. Ignored otherwise. Options: `Left|Center|Right`.",
            ),
            (
                "footerLayout",
                "Footer layout to set on the site. Options: `Simple|Extended`.",
            ),
            (
                "footerEmphasis",
                "Footer background color to set. Options: `Lightest|Light|Dark|Darkest`.",
            ),
            (
                "disableMegaMenu",
                "Set to `true` to disable the mega menu and to `false` to enable it. Disabling mega menu results in using the cascading navigation (classic experience). Options: `true|false`.",
            ),
            (
                "hideTitleInHeader",
                "Set to `true` to hide the site title in the header and to `false` to show it. Options: `true|false`.",
            ),
            (
                "disableFooter",
                "Set to `true` to disable the footer on the site and to `false` to enable it. Options: `true|false`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Show site header in compact mode",
                "m365 spo site chrome set --siteUrl https://contoso.sharepoint.com/sites/project-x --headerLayout Compact",
            ),
            (
                "Show site header in extended mode and display the logo on the right",
                "m365 spo site chrome set --siteUrl https://contoso.sharepoint.com/sites/project-x  --headerLayout Extended --logoAlignment Right",
            ),
            (
                "Disable the footer on the site",
                "m365 spo site chrome set --siteUrl https://contoso.sharepoint.com/sites/project-x --disableFooter true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site commsite enable",
        summary: "Enables communication site features on the specified site",
        options: &[
            (
                "url",
                "The URL of the site to enable communication site features on",
            ),
            (
                "designPackageId",
                "The ID of the site design to apply when enabling communication site features. Specify designPackageId or designPackage but not both.",
            ),
            (
                "designPackage",
                "The site design to apply when enabling communication site features. Valid values are: Topic, Showcase, or Blank. Defaults to Topic. Specify designPackageId or designPackage but not both.",
            ),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Enable communication site features on an existing site",
                "m365 spo site commsite enable --url https://contoso.sharepoint.com",
            ),
            (
                "Enable communication site features on an existing site and apply the Showcase design package using its ID.",
                "m365 spo site commsite enable --url https://contoso.sharepoint.com --designPackageId 6142d2a0-63a5-4ba0-aede-d9fefca2c767",
            ),
            (
                "Enable communication site features on an existing site and apply the Showcase design package using its friendly name.",
                "m365 spo site commsite enable --url https://contoso.sharepoint.com --designPackage Showcase",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site ensure",
        summary: "Ensures that the particular site collection exists and updates its properties if necessary.",
        options: &[
            ("url", "URL of the site to ensure"),
            (
                "type",
                "Expected type of the site. Allowed values `TeamSite`, `CommunicationSite`, `ClassicSite`. If not specified, will not check the type of the site if one exists at the specified URL.",
            ),
            ("title", "Site title"),
            (
                "alias",
                "Site alias, used in the URL and in the team site group e-mail (applies to type `TeamSite`)",
            ),
            (
                "timeZone",
                "Integer representing time zone to use for the site (applies to type `ClassicSite`)",
            ),
            ("description", "Site description"),
            (
                "lcid",
                "Site language in the LCID format, eg. 1033 for en-US. See SharePoint documentation for the list of supported languages",
            ),
            (
                "owners",
                "Comma-separated list of users to set as site owners (applies to type TeamSite, ClassicSite)",
            ),
            (
                "isPublic",
                "Determines if the associated group is public or not (applies to type `TeamSite`)",
            ),
            (
                "classification",
                "Site classification (applies to type `TeamSite`, `CommunicationSite`)",
            ),
            (
                "siteDesign",
                "Type of communication site to create. Allowed values `Topic`, `Showcase`, `Blank`, default `Topic`. Used only for creating site when one doesn't exist.",
            ),
            (
                "siteDesignId",
                "Id of the custom site design to use to create the site. When creating a communication site, specify either `siteDesign` or `siteDesignId` (applies to type `CommunicationSite`)",
            ),
            (
                "shareByEmailEnabled",
                "Determines whether it's allowed to share file with guests (applies to type `CommunicationSite`)",
            ),
            (
                "webTemplate",
                "Template to use for creating the site. Default `STS#0` (applies to type `ClassicSite`)",
            ),
            (
                "resourceQuota",
                "The quota for this site collection in Sandboxed Solutions units. Default `0` (applies to type `ClassicSite`)",
            ),
            (
                "resourceQuotaWarningLevel",
                "The warning level for the resource quota. Default `0` (applies to type `ClassicSite`)",
            ),
            (
                "storageQuota",
                "The storage quota for this site collection in megabytes. Default `100` (applies to type `ClassicSite`)",
            ),
            (
                "storageQuotaWarningLevel",
                "The warning level for the storage quota in megabytes. Default `100` (applies to type `ClassicSite`)",
            ),
            (
                "removeDeletedSite",
                "Set, to remove existing deleted site with the same URL from the Recycle Bin (applies to type `ClassicSite`)",
            ),
            (
                "wait",
                "Wait for the site to be provisioned before completing the command (applies to type `ClassicSite`)",
            ),
            (
                "disableFlows",
                "Set to `true` to disable using Microsoft Flow in this site collection",
            ),
            (
                "sharingCapability",
                "The sharing capability for the site. Allowed values: `Disabled`, `ExternalUserSharingOnly`, `ExternalUserAndGuestSharing`, `ExistingExternalUserSharingOnly`.",
            ),
        ],
        notes: &[
            "This command checks if a site collection exists at the specified URL.",
            "If no site is found, this command will create one using the specified options. For more information about creating site collections and things that you should into account, see the `spo site add` command.",
            "If a site is found at the specified URL, this command will update its properties using the specified values. For more information about updating site collections and things that you should take into account see the `spo site set` command.",
            "If another site exists at the specified URL and you haven't specified the `type` option, this command will proceed with updating its properties. If you specified the `type` and it doesn't match the type of the existing site, this command will return an error.",
            "If you set the type to `ClassicSite`, you should also set a value of the `webTemplate` option. If you don't, checking the type of existing site will be skipped. If no site exists at the specified URL, creating the site will fail.",
        ],
        permissions: &[],
        examples: &[
            (
                "Ensure that a site exists at the specified URL. Create a modern team site if no site exists. If a site is found, don't check its type and update its title.",
                "m365 spo site ensure --url https://contoso.sharepoint.com/teams/team1 --alias team1 --title \"Team 1\"",
            ),
            (
                "Ensure that a communication site exists at the specified URL. Create a communication site if no site exists. If a site of different type is found, return an error.",
                "m365 spo site ensure --url https://contoso.sharepoint.com/sites/comms --title Comms --type CommunicationSite",
            ),
            (
                "Ensure that a classic site exists at the specified URL. Create a classic team site if no site exists. If a site of different type is found, return an error.",
                "m365 spo site ensure --url https://contoso.sharepoint.com/sites/classic --title Classic --type ClassicSite",
            ),
            (
                "Ensure that a site exists at the specified URL. Create a modern team site if no site exists. If a site is found, don't check its type and update its title and properties.",
                "m365 spo site ensure --url https://contoso.sharepoint.com/sites/team1 --alias team1 --title \"Team 1\" --isPublic --shareByEmailEnabled",
            ),
        ],
        links: &[
            ("`spo site add`", "./site-add.mdx"),
            ("`spo site set`", "./site-set.mdx"),
        ],
    },
    Help {
        name: "spo site get",
        summary: "Gets information about the specific site collection",
        options: &[(
            "url",
            "URL of the site collection to retrieve information for",
        )],
        notes: &["This command can retrieve information for both classic and modern sites."],
        permissions: &[],
        examples: &[(
            "Return information about the https://contoso.sharepoint.com/sites/project-x site collection.",
            "m365 spo site get -u https://contoso.sharepoint.com/sites/project-x",
        )],
        links: &[],
    },
    Help {
        name: "spo site groupify",
        summary: "Connects site collection to an Microsoft 365 Group",
        options: &[
            (
                "url",
                "URL of the site collection being connected to new Microsoft 365 Group",
            ),
            (
                "alias",
                "The email alias for the new Microsoft 365 Group that will be created",
            ),
            (
                "displayName",
                "The name of the new Microsoft 365 Group that will be created",
            ),
            ("description", "The group’s description"),
            (
                "classification",
                "The classification value, if classifications are set for the organization. If no value is provided, the default classification will be set, if one is configured",
            ),
            (
                "isPublic",
                "Determines the Microsoft 365 Group’s privacy setting. If set, the group will be public, otherwise it will be private",
            ),
            (
                "keepOldHomepage",
                "For sites that already have a modern page set as homepage, set this option, to keep it as the homepage",
            ),
        ],
        notes: &[
            "This command is based on a SharePoint API that is currently in preview and is subject to change once the API reached general availability.",
            "When connecting site collection to an Microsoft 365 Group, SharePoint will create a new group using the specified information. If a group with the same name already exists, you will get a `The group alias already exists.` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Connect site collection to an Microsoft 365 Group",
                "m365 spo site groupify --url https://contoso.sharepoin.com/sites/team-a --alias team-a --displayName 'Team A'",
            ),
            (
                "Connect site collection to an Microsoft 365 Group and make the group public",
                "m365 spo site groupify --url https://contoso.sharepoin.com/sites/team-a --alias team-a --displayName 'Team A' --isPublic",
            ),
            (
                "Connect site collection to an Microsoft 365 Group and set the group classification",
                "m365 spo site groupify --url https://contoso.sharepoin.com/sites/team-a --alias team-a --displayName 'Team A' --classification HBI",
            ),
            (
                "Connect site collection to an Microsoft 365 Group and keep the old home page",
                "m365 spo site groupify --url https://contoso.sharepoin.com/sites/team-a --alias team-a --displayName 'Team A' --keepOldHomepage",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/features/groupify/groupify-overview",
            "https://learn.microsoft.com/sharepoint/dev/features/groupify/groupify-overview",
        )],
    },
    Help {
        name: "spo site hubsite connect",
        summary: "Connects the specified site collection to the given hub site",
        options: &[
            (
                "siteUrl",
                "The URL of the site collection to connect to the hub site.",
            ),
            (
                "id",
                "The ID of the hub site to which to connect the site collection.",
            ),
            (
                "asAdmin",
                "Connect the site to a hub site as SharePoint admin.",
            ),
        ],
        notes: &[
            "If the specified site collection is already connected to a hub site, it will be disconnected and connected to the newly specified hub site.",
            "If the specified `id` doesn't point to a valid hub site, you will get a `ResourceNotFoundException` error.",
        ],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Connect a specific site collection to a hub site",
                "m365 spo site hubsite connect --siteUrl https://contoso.sharepoint.com/sites/contoso-sales --id 255a50b2-527f-4413-8485-57f4c17a24d1",
            ),
            (
                "Connect a specific site collection to a hub site as SharePoint admin",
                "m365 spo site hubsite connect --siteUrl https://contoso.sharepoint.com/sites/contoso-sales --id 255a50b2-527f-4413-8485-57f4c17a24d1 --asAdmin",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo site hubsite disconnect",
        summary: "Disconnects the specified site collection from its hub site",
        options: &[
            (
                "siteUrl",
                "URL of the site collection to disconnect from its hub site.",
            ),
            (
                "asAdmin",
                "Disconnect the site from a hub site as SharePoint admin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Disconnect a specific site collection from its hub site. Will prompt for confirmation before disconnecting from the hub site.",
                "m365 spo site hubsite disconnect --siteUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Disconnect a specific site collection from its hub site as SharePoint admin.",
                "m365 spo site hubsite disconnect --siteUrl https://contoso.sharepoint.com/sites/sales --asAdmin",
            ),
            (
                "Disconnect a specific site collection from its hub site without prompting for confirmation.",
                "m365 spo site hubsite disconnect --siteUrl https://contoso.sharepoint.com/sites/sales --force",
            ),
        ],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo site hubsite theme sync",
        summary: "Applies any theme updates from the hub site to the site it is connected to.",
        options: &[(
            "webUrl",
            "URL of the site to apply theme updates from the hub site to.",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Applies any theme updates from the hub site to a specific site",
            "m365 spo site hubsite theme sync --webUrl https://contoso.sharepoint.com/sites/project-x",
        )],
        links: &[(
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
            "https://techcommunity.microsoft.com/t5/SharePoint-Blog/SharePoint-hub-sites-new-in-Office-365/ba-p/109547",
        )],
    },
    Help {
        name: "spo site inplacerecordsmanagement set",
        summary: "Activates or deactivates in-place records management for a site collection",
        options: &[
            (
                "siteUrl",
                "The URL of the site on which to activate or deactivate in-place records management",
            ),
            (
                "enabled",
                "Set to `true` to activate in-place records management and to `false` to deactivate it",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Activates in-place records management for site https://contoso.sharepoint.com/sites/team-a",
                "m365 spo site inplacerecordsmanagement set --siteUrl https://contoso.sharepoint.com/sites/team-a --enabled true",
            ),
            (
                "Deactivates in-place records management for site https://contoso.sharepoint.com/sites/team-a",
                "m365 spo site inplacerecordsmanagement set --siteUrl https://contoso.sharepoint.com/sites/team-a --enabled false",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site list",
        summary: "Lists modern sites of the given type",
        options: &[
            (
                "type",
                "convenience option for type of sites to list. Allowed values are `TeamSite` and `CommunicationSite`.",
            ),
            (
                "state",
                "option to filter sites by their archived state. Allowed values are `active`, `fullyArchived`, `recentlyArchived` and `archived`.",
            ),
            (
                "webTemplate",
                "type of sites to list. To be used with values like `GROUP#0` and `SITEPAGEPUBLISHING#0`. Specify either `type` or `webTemplate`, but not both.",
            ),
            ("filter", "filter to apply when retrieving sites."),
            (
                "withOneDriveSites",
                "use this switch to include OneDrive sites in the result when retrieving sites. Do not specify the `type` or `webTemplate` options when using this.",
            ),
        ],
        notes: &[
            "When providing a value for option `state`, consider the following:",
            "- `active`: return sites that are not archived",
            "- `fullyArchived`: return sites that are fully archived (longer than 7 days)",
            "- `recentlyArchived`: return sites that are recently archived (within 7 days)",
            "- `archived`: return both recently archived and fully archived sites",
            "Using the `--filter` option you can specify which sites you want to retrieve. For example, to get sites with project in their URL, use `Url -like 'project'` as the filter.",
            "When using the text output type, the command lists only the values of the `Title`, and `Url` properties of the site. When setting the output type to JSON, all available properties are included in the command output.",
            "If you wish to list deleted sites in your tenant, you should use the command spo tenant recyclebinitem list",
            "To use this command you have to have permissions to access the tenant admin site.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all sites in the currently connected tenant.",
                "m365 spo site list",
            ),
            (
                "List all group connected team sites in the currently connected tenant.",
                "m365 spo site list --type TeamSite",
            ),
            (
                "List all communication sites in the currently connected tenant.",
                "m365 spo site list --type CommunicationSite",
            ),
            (
                "List all group connected team sites that contain project in the URL.",
                "m365 spo site list --type TeamSite --filter \"Url -like 'project'\"",
            ),
            (
                "List all sites in the currently connected tenant including OneDrive sites.",
                "m365 spo site list --withOneDriveSites",
            ),
            (
                "List all active (non-archived) sites in the currently connected tenant.",
                "m365 spo site list --state active",
            ),
            (
                "List all archived sites in the currently connected tenant.",
                "m365 spo site list --state archived",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site membership list",
        summary: "Retrieves information about default site groups' membership",
        options: &[
            ("siteUrl", "The URL of the site."),
            (
                "role",
                "Filter the results to include only users with the specified roles: `Owner`, `Member`, or `Visitor`.",
            ),
        ],
        notes: &[
            "To use this command, you must be a Global or SharePoint administrator.",
            "For other scenarios, refer to the spo web get --withGroups and spo group member list commands.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieve information about default site groups' owners, members, and visitors of the site.",
                "m365 spo site membership list --siteUrl https://contoso.sharepoint.com",
            ),
            (
                "Retrieve information about site owners.",
                "m365 spo site membership list --siteUrl https://contoso.sharepoint.com --role Owner",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site recyclebinitem clear",
        summary: "Permanently removes all items in a site recycle bin",
        options: &[
            (
                "siteUrl",
                "URL of the site where the recycle bin is located.",
            ),
            (
                "secondary",
                "Remove all items from the second-stage recycle bin. When not specified, items from the first-stage recycle bin will be cleared.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "Items in the recycle bin will be permanently removed without the ability to restore them.",
        ],
        permissions: &[],
        examples: &[
            (
                "Clear all items from the first-stage recycle bin",
                "m365 spo site recyclebinitem clear --siteUrl https://contoso.sharepoint.com/sites/sales",
            ),
            (
                "Clear all items from the second-stage recycle bin",
                "m365 spo site recyclebinitem clear --siteUrl https://contoso.sharepoint.com/sites/sales --secondary",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site recyclebinitem list",
        summary: "Lists items from recycle bin",
        options: &[
            (
                "siteUrl",
                "URL of the site for which to retrieve the recycle bin items",
            ),
            (
                "type",
                "Type of items which should be retrieved. Valid values: `listItems`, `folders`, `files`.",
            ),
            (
                "secondary",
                "Use this switch to retrieve items from secondary recycle bin",
            ),
        ],
        notes: &[
            "When type is not specified then the command will return all items in the recycle bin",
        ],
        permissions: &[],
        examples: &[
            (
                "Lists all files, items and folders from recycle bin for site https://contoso.sharepoint.com/site",
                "m365 spo site recyclebinitem list --siteUrl https://contoso.sharepoint.com/site",
            ),
            (
                "Lists only files from recycle bin for site https://contoso.sharepoint.com/site",
                "m365 spo site recyclebinitem list --siteUrl https://contoso.sharepoint.com/site --type files",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site recyclebinitem move",
        summary: "Moves items from the first-stage recycle bin to the second-stage recycle bin",
        options: &[
            (
                "siteUrl",
                "URL of the site where the recycle bin is located.",
            ),
            ("ids", "Comma separated list of item IDs."),
            (
                "all",
                "Move all first-stage recycle bin items to the second-stage recycle bin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Move 2 specific items from the first-stage recycle bin to the second-stage",
                "m365 spo site recyclebinitem move --siteUrl https://contoso.sharepoint.com/sites/sales --ids \"06ca4fe4-3048-4b76-bd41-296fed4c9881,d679c17b-d7b8-429a-9307-34e1d9e631e7\"",
            ),
            (
                "Move all first-stage recycle bin items to the second-stage",
                "m365 spo site recyclebinitem move --siteUrl https://contoso.sharepoint.com/sites/sales --all",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site recyclebinitem remove",
        summary: "Permanently deletes specific items from the site recycle bin",
        options: &[
            (
                "siteUrl",
                "URL of the site where the recycle bin is located.",
            ),
            ("ids", "Comma separated list of item IDs."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "Items in the recycle bin will be permanently removed without the ability to restore them.",
        ],
        permissions: &[],
        examples: &[
            (
                "Permanently remove 2 specific items from the recycle bin",
                "m365 spo site recyclebinitem remove --siteUrl https://contoso.sharepoint.com/sites/sales --ids \"06ca4fe4-3048-4b76-bd41-296fed4c9881,d679c17b-d7b8-429a-9307-34e1d9e631e7\"",
            ),
            (
                "Permanently remove 2 specific items from the recycle bin and skip the confirmation prompt",
                "m365 spo site recyclebinitem remove --siteUrl https://contoso.sharepoint.com/sites/sales --ids \"06ca4fe4-3048-4b76-bd41-296fed4c9881,d679c17b-d7b8-429a-9307-34e1d9e631e7\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site recyclebinitem restore",
        summary: "Restores given items from the site recycle bin",
        options: &[
            ("siteUrl", "URL of the site."),
            (
                "ids",
                "List of ids of items which will be restored from the site recycle bin. Specify `ids` or `allPrimary` and/or `allSecondary`.",
            ),
            (
                "allPrimary",
                "Restore all items from the first-stage recycle bin. Specify `ids` or `allPrimary` and/or `allSecondary`.",
            ),
            (
                "allSecondary",
                "Restore all items from the second-stage recycle bin. Specify `ids` or `allPrimary` and/or `allSecondary`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Restore specific items by given ids from recycle bin of given site.",
                "m365 spo site recyclebinitem restore --siteUrl https://contoso.sharepoint.com/sites/project --ids \"ae6f97a7-280e-48d6-b481-0ea986c323da,aadbf916-1f71-42ee-abf2-8ee4802ae291\"",
            ),
            (
                "Restore all items from the first-stage recycle bin of a given site.",
                "m365 spo site recyclebinitem restore --siteUrl https://contoso.sharepoint.com/sites/project --allPrimary",
            ),
            (
                "Restore all items from the second-stage recycle bin of a given site.",
                "m365 spo site recyclebinitem restore --siteUrl https://contoso.sharepoint.com --allSecondary",
            ),
            (
                "Restore all items from both the first-stage and second-stage recycle bins of a given site.",
                "m365 spo site recyclebinitem restore --siteUrl https://contoso.sharepoint.com --allPrimary --allSecondary",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site remove",
        summary: "Removes the specified site",
        options: &[
            ("url", "URL of the site."),
            (
                "permanent",
                "Permanently remove the site without leaving it in the recycle bin.",
            ),
            (
                "fromRecycleBin",
                "Set to remove the site from the recycle bin.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "If the argument `--fromRecycleBin` is passed, the selected site will be permanently removed even if it's a groupified one.",
            "To use this command you must be a Global or SharePoint administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove the specified site and place it in the recycle bin.",
                "m365 spo site remove --url https://contoso.sharepoint.com/sites/demosite",
            ),
            (
                "Remove the site without moving it to the recycle bin.",
                "m365 spo site remove --url https://contoso.sharepoint.com/sites/demosite --permanent",
            ),
            (
                "Remove the previously deleted site from the recycle bin.",
                "m365 spo site remove --url https://contoso.sharepoint.com/sites/demosite --fromRecycleBin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site rename",
        summary: "Renames the URL and title of a site collection",
        options: &[
            ("url", "The URL of the site to rename"),
            ("newUrl", "New URL for the site collection"),
            ("newTitle", "New title for the site"),
            (
                "suppressMarketplaceAppCheck",
                "Suppress marketplace app check",
            ),
            ("suppressWorkflow2013Check", "Suppress 2013 workflow check"),
            ("wait", "Wait for the job to complete"),
        ],
        notes: &[
            "Renaming site collections is by default asynchronous and depending on the current state of Microsoft 365, might take up to few minutes. If you're building a script with steps that require the operation to complete fully, you should use the `--wait` flag. When using this flag, the `spo site rename` command will keep running until it receives confirmation from Microsoft 365 that the site rename operation has completed.",
            "To use this command you must have permissions to access the tenant admin site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Starts the rename of the site collection with name \"samplesite\" to \"renamed\" without modifying the title",
                "m365 spo site rename --url http://contoso.sharepoint.com/samplesite --newUrl http://contoso.sharepoint.com/renamed",
            ),
            (
                "Starts the rename of the site collection with name \"samplesite\" to \"renamed\" modifying the title of the site to \"New Title\"",
                "m365 spo site rename --url http://contoso.sharepoint.com/samplesite --newUrl http://contoso.sharepoint.com/renamed --newTitle \"New Title\"",
            ),
            (
                "Renames the specified site collection and waits for the operation to complete",
                "m365 spo site rename --url http://contoso.sharepoint.com/samplesite --newUrl http://contoso.sharepoint.com/renamed --newTitle \"New Title\" --wait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site set",
        summary: "Updates properties of the specified site",
        options: &[
            ("url", "The URL of the site collection to update"),
            ("title", "The new title for the site collection"),
            ("description", "The site description"),
            (
                "classification",
                "The new classification for the site collection",
            ),
            (
                "disableFlows",
                "Set to `true` to disable using Microsoft Flow in this site collection",
            ),
            (
                "socialBarOnSitePagesDisabled",
                "Set to `true` to disable Social Bar for the site collection",
            ),
            (
                "isPublic",
                "Set to `true` to make the group linked to the site public or to `false` to make it private",
            ),
            (
                "owners",
                "Comma-separated list of users to add as site collection administrators",
            ),
            (
                "shareByEmailEnabled",
                "Set to true to allow to share files with guests and to false to disallow it",
            ),
            (
                "siteDesignId",
                "Id of the custom site design to apply to the site",
            ),
            (
                "sharingCapability",
                "The sharing capability for the site. Allowed values: `Disabled`, `ExternalUserSharingOnly`, `ExternalUserAndGuestSharing`, `ExistingExternalUserSharingOnly`.",
            ),
            (
                "siteLogoUrl",
                "Set the logo for the site collection. This can be an absolute or relative URL to a file on the current site collection.",
            ),
            (
                "siteThumbnailUrl",
                "Set the thumbnail for the site collection. This can be an absolute or relative URL to a file on the current site collection.",
            ),
            (
                "resourceQuota",
                "The quota for this site collection in Sandboxed Solutions units",
            ),
            (
                "resourceQuotaWarningLevel",
                "The warning level for the resource quota",
            ),
            (
                "storageQuota",
                "The storage quota for this site collection in megabytes",
            ),
            (
                "storageQuotaWarningLevel",
                "The warning level for the storage quota in megabytes",
            ),
            (
                "allowSelfServiceUpgrade",
                "Set to allow tenant administrators to upgrade the site collection",
            ),
            (
                "lockState",
                "Sets site's lock state. Allowed values `Unlock`, `NoAdditions`, `ReadOnly`, `NoAccess`",
            ),
            (
                "noScriptSite",
                "Specifies if the site allows custom script or not",
            ),
            (
                "wait",
                "Wait for the settings to be applied before completing the command",
            ),
        ],
        notes: &[
            "If the specified url doesn't refer to an existing site collection, you will get a `404 - \"404 FILE NOT FOUND\"` error.",
            "The `isPublic` property can be set only on groupified site collections. If you try to set it on a site collection without a group, you will get an error.",
            "When setting owners, the specified owners will be added to the already configured owners. Existing owners will not be removed.",
            "The value of the `--resourceQuota` option must not exceed the company's aggregate available Sandboxed Solutions quota. For more information, see Resource Usage Limits on Sandboxed Solutions in SharePoint 2010: http://msdn.microsoft.com/en-us/library/gg615462.aspx.",
            "The value of the `--resourceQuotaWarningLevel` option must not exceed the value of the `--resourceQuota` option or the current value of the UserCodeMaximumLevel property.",
            "The value of the `--storageQuota` option must not exceed the company's available quota.",
            "The value of the `--storageQuotaWarningLevel` option must not exceed the the value of the `--storageQuota` option or the current value of the StorageMaximumLevel property.",
            "For more information on locking sites see https://technet.microsoft.com/en-us/library/cc263238.aspx.",
            "For more information on configuring no script sites see https://support.office.com/en-us/article/Turn-scripting-capabilities-on-or-off-1f2c515f-5d7e-448a-9fd7-835da935584f.",
            "Setting site properties is by default asynchronous and depending on the current state of Microsoft 365, might take up to few minutes. If you're building a script with steps that require the site to be fully configured, you should use the `--wait` flag. When using this flag, the `spo site set` command will keep running until it received confirmation from Microsoft 365 that the site has been fully configured.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update site collection's classification",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --classification MBI",
            ),
            (
                "Reset site collection's classification.",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --classification",
            ),
            (
                "Disable using Microsoft Flow on the site collection",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --disableFlows true",
            ),
            (
                "Disable Social Bar for the site collection",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --socialBarOnSitePagesDisabled true",
            ),
            (
                "Update the visibility of the Microsoft 365 group behind the specified groupified site collection to public",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --isPublic true",
            ),
            (
                "Update site collection's owners",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --owners \"john@contoso.onmicrosoft.com,steve@contoso.onmicrosoft.com\"",
            ),
            (
                "Allow sharing files in the site collection with guests",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --shareByEmailEnabled true",
            ),
            (
                "Apply the specified site ID to the site collection",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --siteDesignId \"eb2f31da-9461-4fbf-9ea1-9959b134b89e\"",
            ),
            (
                "Update site collection's title",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --title \"My new site\"",
            ),
            (
                "Update site collection's description",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --description \"my description\"",
            ),
            (
                "Restrict external sharing to already available external users only",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --sharingCapability ExistingExternalUserSharingOnly",
            ),
            (
                "Set the logo on the site",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --siteLogoUrl \"/sites/sales/SiteAssets/parker-ms-1200.png\"",
            ),
            (
                "Unset the logo on the site",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --siteLogoUrl \"\"",
            ),
            (
                "Set the thumbnail on the site",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --siteThumbnailUrl \"/sites/sales/SiteAssets/parker-ms-1200.png\"",
            ),
            (
                "Unset the thumbnail on the site",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/sales --siteThumbnailUrl \"\"",
            ),
            (
                "Lock the site preventing users from accessing it. Wait for the configuration to complete",
                "m365 spo site set --url https://contoso.sharepoint.com/sites/team --lockState NoAccess --wait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo site sharingpermission set",
        summary: "Controls how a site and its components can be shared",
        options: &[
            (
                "capability",
                "Define how the site is shared. Possible values: `full`, `limited`, `ownersOnly`.",
            ),
            ("siteUrl", "URL of the site."),
        ],
        notes: &[
            "When specifying `capability`, consider the following:",
            "- `full`: Site owners and members can share files, folders, and the site. People with Edit permissions can share files and folders.",
            "- `limited`: Site owners and members, and people with Edit permissions can share files and folders, but only site owners can share the site.",
            "- `ownersOnly`: Only site owners can share files, folders, and the site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update the sharing permissions for a site so only owners can share files and the site",
                "m365 spo site sharingpermission set --siteUrl https://siteaddress.com/sites/sitename  --capability ownersOnly",
            ),
            (
                "Update the sharing permissions for a site where so both owners and members can share files and the site",
                "m365 spo site sharingpermission set --siteUrl https://siteaddress.com/sites/sitename  --capability full",
            ),
            (
                "Update the sharing permissions for a site where so owners can share the site, but members can only share files",
                "m365 spo site sharingpermission set --siteUrl https://siteaddress.com/sites/sitename --capability limited",
            ),
        ],
        links: &[(
            "https://support.microsoft.com/office/share-a-site-958771a8-d041-4eb8-b51c-afea2eae3658",
            "https://support.microsoft.com/office/share-a-site-958771a8-d041-4eb8-b51c-afea2eae3658",
        )],
    },
    Help {
        name: "spo site unarchive",
        summary: "Unarchives a site collection",
        options: &[
            ("url", "URL of the site collection."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "To use this command, you must be a Global or SharePoint administrator.",
            "If a site has been archived for more than seven days, reactivating it triggers a 120-day cooldown period during which you can't archive the site again.",
            "After running this command, it may take a few minutes for the site to be fully restored.",
        ],
        permissions: &[],
        examples: &[
            (
                "Unarchive a specific site collection.",
                "m365 spo site unarchive --url \"https://contoso.sharepoint.com/sites/Marketing\"",
            ),
            (
                "Unarchive a specific site collection without confirmation prompt.",
                "m365 spo site unarchive --url \"https://contoso.sharepoint.com/sites/Marketing\" --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/microsoft-365/archive/archive-pricing",
            "https://learn.microsoft.com/microsoft-365/archive/archive-pricing",
        )],
    },
    Help {
        name: "spo site versionpolicy get",
        summary: "Retrieves the version policy settings of a specific site.",
        options: &[("siteUrl", "URL of the site.")],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.Read"),
            ("Application", "SharePoint", "Sites.Read.All"),
        ],
        examples: &[(
            "Retrieve the version policy settings of a specific site.",
            "m365 spo site versionpolicy get --siteUrl \"https://contoso.sharepoint.com/sites/Marketing\"",
        )],
        links: &[],
    },
    Help {
        name: "spo sitedesign add",
        summary: "Adds site design for creating modern sites",
        options: &[
            ("title", "The display name of the site design"),
            (
                "webTemplate",
                "Identifies which base template to add the design to. Allowed values `TeamSite,CommunicationSite`",
            ),
            (
                "siteScripts",
                "Comma-separated list of site script IDs. The scripts will run in the order listed",
            ),
            ("description", "The display description of site design"),
            (
                "previewImageUrl",
                "The URL of a preview image. If none is specified SharePoint will use a generic image",
            ),
            (
                "previewImageAltText",
                "The alt text description of the image for accessibility",
            ),
            (
                "thumbnailUrl",
                "The new URL of a thumbnail image. If none is specified SharePoint will use a generic image",
            ),
            (
                "isDefault",
                "Set if the site design is applied as the default site design",
            ),
        ],
        notes: &[
            "Each time you execute the `spo sitedesign add` command, it will create a new site design with a unique ID. Before creating a site design, be sure that another design with the same name doesn't already exist.",
            "When specifying IDs of site scripts to use with your site design, ensure that the IDs refer to existing site scripts or provisioning sites using the design will lead to unexpected results.",
        ],
        permissions: &[],
        examples: &[
            (
                "Create new site design for provisioning modern team sites",
                "m365 spo sitedesign add --title \"Contoso team site\" --webTemplate TeamSite --siteScripts \"19b0e1b2-e3d1-473f-9394-f08c198ef43e,b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
            (
                "Create new default site design for provisioning modern communication sites",
                "m365 spo sitedesign add --title \"Contoso communication site\" --webTemplate CommunicationSite --siteScripts \"19b0e1b2-e3d1-473f-9394-f08c198ef43e\" --isDefault",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            ),
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/customize-default-site-design",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/customize-default-site-design",
            ),
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-json-schema",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-json-schema",
            ),
        ],
    },
    Help {
        name: "spo sitedesign apply",
        summary: "Applies a site design to an existing site collection",
        options: &[
            ("id", "The ID of the site design to apply"),
            ("webUrl", "The URL of the site to apply the site design to"),
            (
                "asTask",
                "Apply site design as task. Required for large site designs",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Apply the site design with ID 9b142c22-037f-4a7f-9017-e9d8c0e34b98 to the site collection https://contoso.sharepoint.com/sites/project-x",
                "m365 spo sitedesign apply --id 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Apply large site design to the specified site",
                "m365 spo sitedesign apply --id 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --webUrl https://contoso.sharepoint.com/sites/project-x --asTask",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign get",
        summary: "Gets information about the specified site design",
        options: &[("id", "Site design ID."), ("title", "Site design title.")],
        notes: &[
            "If the specified `id` or `title` doesn't refer to an existing site design, you will get a `File not found` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
                "m365 spo sitedesign get --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Get information about the site design with title Contoso Site Design",
                "m365 spo sitedesign get --title \"Contoso Site Design\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign list",
        summary: "Lists available site designs for creating modern sites",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[("List available site designs", "m365 spo sitedesign list")],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign remove",
        summary: "Removes the specified site design",
        options: &[
            ("id", "Site design ID"),
            (
                "force",
                "Don't prompt for confirming removing the site design",
            ),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing site design, you will get a `File not found` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a. Will prompt for confirmation before removing the design",
                "m365 spo sitedesign remove --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Remove site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a without prompting for confirmation",
                "m365 spo sitedesign remove --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign rights grant",
        summary: "Grants access to a site design for one or more principals",
        options: &[
            (
                "siteDesignId",
                "The ID of the site design to grant rights on",
            ),
            (
                "principals",
                "Comma-separated list of principals to grant view rights. Principals can be users or mail-enabled security groups in the form of `alias` or `alias@.com`",
            ),
            (
                "rights",
                "Rights to grant to principals. Available values `View`",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Grant user with alias PattiF view permission to the specified site design",
                "m365 spo sitedesign rights grant --siteDesignId 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --principals PattiF --rights View",
            ),
            (
                "Grant users with aliases PattiF and AdeleV view permission to the specified site design",
                "m365 spo sitedesign rights grant --siteDesignId 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --principals \"PattiF,AdeleV\" --rights View",
            ),
            (
                "Grant user by email view permission to the specified site design",
                "m365 spo sitedesign rights grant --siteDesignId 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --principals PattiF@contoso.com --rights View",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign rights list",
        summary: "Gets a list of principals that have access to a site design",
        options: &[(
            "siteDesignId",
            "The ID of the site design to get rights information from",
        )],
        notes: &[
            "If the specified `siteDesignId` doesn't refer to an existing site script, you will get a `File not found` error.",
            "If no permissions are listed, it means that the particular site design is visible to everyone.",
        ],
        permissions: &[],
        examples: &[(
            "Get information about rights granted for the site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            "m365 spo sitedesign rights list --siteDesignId 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign rights revoke",
        summary: "Revokes access from a site design for one or more principals",
        options: &[
            (
                "siteDesignId",
                "The ID of the site design to revoke rights from",
            ),
            (
                "principals",
                "Comma-separated list of principals to revoke view rights from. Principals can be users or mail-enabled security groups in the form of `alias` or `alias@.com`",
            ),
            (
                "force",
                "Don't prompt for confirming removing the site design",
            ),
        ],
        notes: &[
            "If the specified id doesn't refer to an existing site design, you will get a `File not found` error.",
            "If all principals have rights revoked on the site design, the site design becomes viewable to everyone.",
            "If you try to revoke access for a user that doesn't have access granted to the specified site design you will get a `The specified user or domain group was not found error`.",
        ],
        permissions: &[],
        examples: &[
            (
                "Revoke access to the site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a from user with alias PattiF. Will prompt for confirmation before revoking the access",
                "m365 spo sitedesign rights revoke --siteDesignId 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --principals PattiF",
            ),
            (
                "Revoke access to the site design with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a from users with aliases PattiF and AdeleV without prompting for confirmation",
                "m365 spo sitedesign rights revoke --siteDesignId 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --principals \"PattiF,AdeleV\" --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign run list",
        summary: "Lists information about site designs applied to the specified site",
        options: &[
            (
                "webUrl",
                "The URL of the site for which to list applied site designs",
            ),
            (
                "siteDesignId",
                "The ID of the site design for which to display information",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List site designs applied to the specified site",
                "m365 spo sitedesign run list --webUrl https://contoso.sharepoint.com/sites/team-a",
            ),
            (
                "List information about the specified site design applied to the specified site",
                "m365 spo sitedesign run list --webUrl https://contoso.sharepoint.com/sites/team-a --siteDesignId 6ec3ca5b-d04b-4381-b169-61378556d76e",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign run status get",
        summary: "Gets information about the site scripts executed for the specified site design",
        options: &[
            (
                "webUrl",
                "The URL of the site for which to get the information",
            ),
            (
                "runId",
                "ID of the site design applied to the site as retrieved using `spo sitedesign run list`",
            ),
        ],
        notes: &[
            "For text output mode, displays the name of the action, site script and the outcome of the action. For JSON output mode, displays all available information.",
        ],
        permissions: &[],
        examples: &[(
            "List information about site scripts executed for the specified site design",
            "m365 spo sitedesign run status get --webUrl https://contoso.sharepoint.com/sites/team-a --runId b4411557-308b-4545-a3c4-55297d5cd8c8",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign set",
        summary: "Updates a site design with new values",
        options: &[
            ("id", "The ID of the site design to update"),
            ("title", "The new display name of the updated site design"),
            (
                "webTemplate",
                "The new template to add the site design to. Allowed values `TeamSite,CommunicationSite`",
            ),
            (
                "siteScripts",
                "Comma-separated list of new site script IDs. The scripts will run in the order listed",
            ),
            (
                "description",
                "The new display description of updated site design",
            ),
            (
                "previewImageUrl",
                "The new URL of a preview image. If none is specified SharePoint will use a generic image",
            ),
            (
                "previewImageAltText",
                "The new alt text description of the image for accessibility",
            ),
            (
                "thumbnailUrl",
                "The new URL of a thumbnail image. If none is specified SharePoint will use a generic image",
            ),
            ("version", "The new version number for the site design"),
            (
                "isDefault",
                "Set to true if the site design is applied as the default site design",
            ),
        ],
        notes: &[
            "If you had previously set the `isDefault` option to `true`, and wish for it to remain `true`, you must pass in this option again or it will be reset to `false`.",
            "When specifying IDs of site scripts to use with your site design, ensure that the IDs refer to existing site scripts or provisioning sites using the design will lead to unexpected results.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update the site design title and version",
                "m365 spo sitedesign set --id 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --title \"Contoso site design\" --version 2",
            ),
            (
                "Update the site design to be the default design for provisioning modern communication sites",
                "m365 spo sitedesign set --id 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --webTemplate CommunicationSite  --isDefault true",
            ),
            (
                "Update the site design to be the default design for provisioning modern communication sites, with specific scripts",
                "m365 spo sitedesign set --id 9b142c22-037f-4a7f-9017-e9d8c0e34b98 --webTemplate CommunicationSite  --isDefault true --siteScripts \"19b0e1b2-e3d1-473f-9394-f08c198ef43e,b2307a39-e878-458b-bc90-03bc578531d6\"",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            ),
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/customize-default-site-design",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/customize-default-site-design",
            ),
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-json-schema",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-json-schema",
            ),
        ],
    },
    Help {
        name: "spo sitedesign task get",
        summary: "Gets information about the specified site design scheduled for execution",
        options: &[(
            "id",
            "The ID of the site design task to get information for",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get information about the specified site design scheduled for execution",
            "m365 spo sitedesign task get --id 6ec3ca5b-d04b-4381-b169-61378556d76e",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign task list",
        summary: "Lists site designs scheduled for execution on the specified site",
        options: &[(
            "webUrl",
            "The URL of the site for which to list site designs scheduled for execution",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List site designs scheduled for execution on the specified site",
            "m365 spo sitedesign task list --webUrl https://contoso.sharepoint.com/sites/team-a",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitedesign task remove",
        summary: "Removes the specified site design scheduled for execution",
        options: &[
            ("id", "The ID of the site design task to remove"),
            (
                "force",
                "Don't prompt for confirming removing the site design task",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the specified site design task with id 6ec3ca5b-d04b-4381-b169-61378556d76e scheduled for execution without prompting confirmation",
                "m365 spo sitedesign task remove --id 6ec3ca5b-d04b-4381-b169-61378556d76e --force",
            ),
            (
                "Removes the specified site design task with id 6ec3ca5b-d04b-4381-b169-61378556d76e scheduled for execution with prompt for confirmation before removing",
                "m365 spo sitedesign task remove --id 6ec3ca5b-d04b-4381-b169-61378556d76e",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitescript add",
        summary: "Adds site script for use with site designs",
        options: &[
            ("title", "Site script title"),
            ("content", "JSON string containing the site script"),
            ("description", "Site script description"),
        ],
        notes: &[
            "Each time you execute the `spo sitescript add` command, it will create a new site script with a unique ID. Before creating a site script, be sure that another script with the same name doesn't already exist.",
        ],
        permissions: &[],
        examples: &[(
            "Create new site script for use with site designs. Script contents are stored in the `$script` variable",
            "m365 spo sitescript add --title \"Contoso\" --description \"Contoso theme script\" --content $script",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitescript get",
        summary: "Gets information about the specified site script",
        options: &[
            ("id", "Site script ID."),
            (
                "content",
                "Specify to only retrieve the content of the site script.",
            ),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing site script, you will get a `File not found` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the site script with the specified ID.",
                "m365 spo sitescript get --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Returns the site script contents:",
                "m365 spo sitescript get --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --content",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitescript list",
        summary: "Lists site script available for use with site designs",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List all site scripts available for use with site designs",
            "m365 spo sitescript list",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitescript remove",
        summary: "Removes the specified site script",
        options: &[
            ("id", "Site script ID"),
            (
                "force",
                "Don't prompt for confirming removing the site script",
            ),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing site script, you will get a `File not found` error.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove site script with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a. Will prompt for confirmation before removing the script",
                "m365 spo sitescript remove --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a",
            ),
            (
                "Remove site script with ID 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a without prompting for confirmation",
                "m365 spo sitescript remove --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo sitescript set",
        summary: "Updates existing site script",
        options: &[
            ("id", "Site script ID."),
            ("title", "Site script title."),
            ("description", "Site script description."),
            ("version", "Site script version."),
            ("content", "JSON string containing the site script."),
        ],
        notes: &[
            "If the specified `id` doesn't refer to an existing site script, you will get a `File not found` error.",
        ],
        permissions: &[],
        examples: &[(
            "Update title of the existing site script with the specified ID.",
            "m365 spo sitescript set --id 2c1ba4c4-cd9b-4417-832f-92a34bc34b2a --title \"Contoso\"",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-design-overview",
        )],
    },
    Help {
        name: "spo storageentity get",
        summary: "Gets details for the specified tenant property",
        options: &[
            ("key", "Name of the tenant property to retrieve"),
            (
                "appCatalogUrl",
                "URL of the app catalog site. If not specified, the tenant app catalog URL will be used.",
            ),
        ],
        notes: &[
            "Tenant properties are stored in the app catalog site associated with the site to which you are currently connected. When retrieving the specified tenant property, SharePoint will use the specified app catalog URL or automatically resolve the tenant app catalog URL.",
        ],
        permissions: &[],
        examples: &[
            (
                "Show the value, description and comment of the AnalyticsId tenant property using the tenant app catalog",
                "m365 spo storageentity get -k AnalyticsId",
            ),
            (
                "Show the value, description and comment of the AnalyticsId tenant property from a specific app catalog site",
                "m365 spo storageentity get -k AnalyticsId -u https://contoso.sharepoint.com/sites/appcatalog",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
        )],
    },
    Help {
        name: "spo storageentity list",
        summary: "Lists tenant properties stored on the specified SharePoint Online app catalog",
        options: &[(
            "appCatalogUrl",
            "URL of the app catalog site. If not specified, the tenant app catalog URL will be used.",
        )],
        notes: &[
            "Tenant properties are stored in the app catalog site. To list all tenant properties, you can specify the absolute URL of the app catalog site. If not specified, the command will automatically resolve the tenant app catalog URL. If you specify an incorrect URL, or the site at the given URL is not an app catalog site, no properties will be retrieved.",
        ],
        permissions: &[],
        examples: &[
            (
                "List all tenant properties using the tenant app catalog",
                "m365 spo storageentity list",
            ),
            (
                "List all tenant properties stored in the https://contoso.sharepoint.com/sites/appcatalog app catalog site",
                "m365 spo storageentity list -u https://contoso.sharepoint.com/sites/appcatalog",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
        )],
    },
    Help {
        name: "spo storageentity remove",
        summary: "Removes tenant property stored on the specified SharePoint Online app catalog",
        options: &[
            (
                "appCatalogUrl",
                "URL of the app catalog site. If not specified, the tenant app catalog URL will be used.",
            ),
            ("key", "Name of the tenant property to retrieve"),
            (
                "force",
                "Don't prompt for confirming removal of a tenant property",
            ),
        ],
        notes: &[
            "Tenant properties are stored in the app catalog site associated with that tenant. To remove a property, you can specify the absolute URL of the app catalog site. If not specified, the command will automatically resolve the tenant app catalog URL. If you specify the URL of a site different than the app catalog, you will get an access denied error.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove the AnalyticsId tenant property using the tenant app catalog. Yields a confirmation prompt before actually removing the property",
                "m365 spo storageentity remove -k AnalyticsId",
            ),
            (
                "Remove the AnalyticsId tenant property from a specific app catalog. Suppresses the confirmation prompt",
                "m365 spo storageentity remove -k AnalyticsId --force -u https://contoso.sharepoint.com/sites/appcatalog",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
        )],
    },
    Help {
        name: "spo storageentity set",
        summary: "Sets tenant property on the specified SharePoint Online app catalog",
        options: &[
            (
                "appCatalogUrl",
                "URL of the app catalog site. If not specified, the tenant app catalog URL will be used.",
            ),
            ("key", "Name of the tenant property to retrieve"),
            ("value", "Value to set for the property"),
            (
                "description",
                "Description to set for the property (optional)",
            ),
            ("comment", "Comment to set for the property (optional)"),
        ],
        notes: &[
            "Tenant properties are stored in the app catalog site associated with that tenant. To set a property, you can specify the absolute URL of the app catalog site without a trailing slash. If not specified, the command will automatically resolve the tenant app catalog URL. If you specify the URL with trailing slash you get the error `The managed path sites/apps is not a managed path in this tenant.`",
            "If you specify the URL of a site different than the app catalog, you will get an access denied error.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Set 123 as the value of the AnalyticsId tenant property using the tenant app catalog",
                "m365 spo storageentity set -k AnalyticsId -v 123 -d 'Web analytics ID' -c 'Use on all sites'",
            ),
            (
                "Set 123 as the value of the AnalyticsId tenant property on a specific app catalog site",
                "m365 spo storageentity set -k AnalyticsId -v 123 -d 'Web analytics ID' -c 'Use on all sites' -u https://contoso.sharepoint.com/sites/appcatalog",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
            "https://learn.microsoft.com/sharepoint/dev/spfx/tenant-properties",
        )],
    },
    Help {
        name: "spo tenant appcatalog add",
        summary: "Creates new tenant app catalog site",
        options: &[
            ("url", "The absolute site url"),
            ("owner", "The account name of the site owner"),
            (
                "timeZone",
                "Integer representing time zone to use for the site",
            ),
            (
                "wait",
                "Wait for the site to be provisioned before completing the command",
            ),
            (
                "force",
                "Force creating a new app catalog site if one already exists",
            ),
        ],
        notes: &[
            "If there is an app catalog registered in your tenant, creating a new app catalog using this command will fail, unless you use the `force` option.",
            "If you use the `force` option, and either the app catalog or the site at the specified URL exists (or both), this command will delete both sites bypassing the recycle bin.",
            "Creating an app catalog site might take a while. If you need to wait for the site to be created before continuing, use the `wait` option.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Creates new app catalog. Will fail if another app catalog or site at the specified URL exists",
                "m365 spo tenant appcatalog add --url https://contoso.sharepoint.com/sites/apps --owner admin@contoso.com --timeZone 4",
            ),
            (
                "Creates new app catalog and waits for completion. Will fail if another app catalog or site at the specified URL exists",
                "m365 spo tenant appcatalog add --url https://contoso.sharepoint.com/sites/apps --owner admin@contoso.com --timeZone 4 --wait",
            ),
            (
                "Creates new app catalog and deletes existing app catalog and existing site at the specified URL",
                "m365 spo tenant appcatalog add --url https://contoso.sharepoint.com/sites/apps --owner admin@contoso.com --timeZone 4 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant appcatalogurl get",
        summary: "Gets the URL of the tenant app catalog",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Get the URL of the tenant app catalog",
            "m365 spo tenant appcatalogurl get",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant applicationcustomizer add",
        summary: "Adds an Application Customizer as a tenant-wide extension",
        options: &[
            ("title", "The title of the Application Customizer."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the Application Customizer.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the Application Customizer.",
            ),
            (
                "hostProperties",
                "Set the host properties of the Application Customizer.",
            ),
            (
                "webTemplate",
                "Optionally add a web template (e.g. STS#3, SITEPAGEPUBLISHING#0, etc) as a filter for what kind of sites the Application Customizer is registered on.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML or JavaScript values because the command shell treats quotes differently. For example, this is how ApplicationCustomizer user custom action can be created from the Windows cmd.exe:",
            "    m365 spo tenant applicationcustomizer add --title \"YourAppCustomizer\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` or `--hostProperties` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "To use this command, you need to be a SharePoint Admin.",
            "This command can be used for configuring a tenant-wide Application Customizer. To configure an Application Customizer on a specific site, view our dedicated spo applicationcustomizer add command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Adds an Application Customizer that's deployed tenant wide",
                "m365 spo tenant applicationcustomizer add --title \"Some customizer\" --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc",
            ),
            (
                "Adds an Application Customizer that is configured for all communication sites.",
                "m365 spo tenant applicationcustomizer add --title \"Some customizer\" --clientSideComponentId 799883f5-7962-4384-a10a-105adaec6ffc --webTemplate \"SITEPAGEPUBLISHING#0\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant applicationcustomizer get",
        summary: "Gets an application customizer that is installed tenant-wide",
        options: &[
            ("title", "The title of the application customizer."),
            ("id", "The id of the application customizer."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the application customizer.",
            ),
            (
                "tenantWideExtensionComponentProperties",
                "Only output the tenant wide extension component properties.",
            ),
        ],
        notes: &[
            "This command can be used for retrieving an application customizer that's installed tenant-wide. To retrieve an application customizer from a specific site, view our dedicated spo applicationcustomizer get command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieves an application customizer by title.",
                "m365 spo tenant applicationcustomizer get --title \"Some customizer\"",
            ),
            (
                "Retrieves an application customizer by id.",
                "m365 spo tenant applicationcustomizer get --id 3",
            ),
            (
                "Retrieves an application customizer by clientSideComponentId.",
                "m365 spo tenant applicationcustomizer get --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant applicationcustomizer list",
        summary: "Retrieves a list of application customizers that are installed tenant-wide",
        options: &[],
        notes: &[
            "This command can be used for retrieving a list of tenant-wide installed application customizers. To retrieve a list of application customizers from a specific site, view our dedicated spo applicationcustomizer list command.",
        ],
        permissions: &[],
        examples: &[(
            "Retrieves a list of application customizers.",
            "m365 spo tenant applicationcustomizer list",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant applicationcustomizer remove",
        summary: "Removes an application customizer that is installed tenant-wide",
        options: &[
            ("title", "The title of the Application Customizer."),
            ("id", "The id of the Application Customizer."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the application customizer.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command can be used for removing an application customizer that's installed tenant-wide. To remove an application customizer from a specific site, view our dedicated spo applicationcustomizer remove command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes an application customizer by title.",
                "m365 spo tenant applicationcustomizer remove --title \"Some customizer\"",
            ),
            (
                "Removes an application customizer by id.",
                "m365 spo tenant applicationcustomizer remove --id 3",
            ),
            (
                "Removes an application customizer by clientSideComponentId.",
                "m365 spo tenant applicationcustomizer remove --clientSideComponentId \"7096cded-b83d-4eab-96f0-df477ed7c0bc\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant applicationcustomizer set",
        summary: "Updates an Application Customizer that is deployed as a tenant-wide extension",
        options: &[
            (
                "title",
                "The title of the Application Customizer to update.",
            ),
            ("id", "The id of the Application Customizer to update."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the Application Customizer to update.",
            ),
            (
                "newTitle",
                "The updated title of the Application Customizer.",
            ),
            (
                "newClientSideComponentId",
                "The new Client Side Component Id (GUID) of the Application Customizer.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the Application Customizer.",
            ),
            (
                "hostProperties",
                "Set the host properties of the Application Customizer. Set to an empty string to clear the value.",
            ),
            (
                "webTemplate",
                "Optionally add a web template (e.g. STS#3, SITEPAGEPUBLISHING#0, etc) as a filter for what kind of sites the Application Customizer is registered on.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for `clientSideComponentProperties` and `hostProperties` options that have JSON values because the command shell treats quotes differently. For example, this is how Application Customizer can be updated from the Windows cmd.exe:",
            "    m365 spo tenant applicationcustomizer set --id 3 --clientSideComponentProperties '{\\\"someProperty\\\":\\\"Some value\\\"}'",
            "Note, how the `--clientSideComponentProperties` option has escaped double quotes `'{\\\"someProperty\\\":\\\"Some value\\\"}'` compared to execution from bash `'{\"someProperty\": \"Some value\"}'`.",
            "When using the `--clientSideComponentProperties` or `--hostProperties` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for updating an application customizer that's installed tenant-wide. To update an application customizer from a specific site, view our dedicated spo applicationcustomizer set command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Updates the title of an Application Customizer that is deployed as a tenant-wide extension by its id",
                "m365 spo tenant applicationcustomizer set --id 3 --newTitle \"Some customizer\"",
            ),
            (
                "Updates the Client Side Component Id of an Application Customizer that is deployed as a tenant-wide extension by its id",
                "m365 spo tenant applicationcustomizer set --id 3 --newClientSideComponentId \"b44a5182-9877-4029-baec-0181c70dacbc\"",
            ),
            (
                "Updates the properties of an Application Customizer that is deployed as a tenant-wide extension by its id",
                "m365 spo tenant applicationcustomizer set --id 3 --clientSideComponentProperties '{ \"someProperty\": \"Some value\" }'",
            ),
            (
                "Updates the title of an Application Customizer that is deployed as a tenant-wide extension by its title",
                "m365 spo tenant applicationcustomizer set --title \"Some customizer\" --newTitle \"Updated customizer\"",
            ),
            (
                "Updates the title of an Application Customizer that is deployed as a tenant-wide extension by its clientSideComponentId",
                "m365 spo tenant applicationcustomizer set --clientSideComponentId \"7f8fd1f2-9d26-4a4a-a607-bf4622d7ec11\" --newTitle \"Some customizer\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant commandset add",
        summary: "Adds a ListView Command Set as a tenant-wide extension",
        options: &[
            ("title", "The title of the ListView Command Set."),
            (
                "listType",
                "The list or library type to register the ListView Command Set on. Allowed values `List` or `Library`.",
            ),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the ListView Command Set.",
            ),
            (
                "webTemplate",
                "Optionally add a web template (e.g. STS#3, SITEPAGEPUBLISHING#0, etc) as a filter for what kind of sites the ListView Command Set is registered on.",
            ),
            (
                "location",
                "The location of the ListView Command Set. Allowed values `ContextMenu`, `CommandBar` or `Both`. Defaults to `CommandBar`.",
            ),
        ],
        notes: &[
            "Running this command from the Windows Command Shell (cmd.exe) or PowerShell for Windows OS XP, 7, 8, 8.1 without bash installed might require additional formatting for command options that have JSON, XML, or JavaScript values because the command shell treats quotes differently. For example, this is how commandset user custom action can be created from the Windows cmd.exe:",
            "    m365 spo tenant commandset add --title \"YourAppCustomizer\" --clientSideComponentId b41916e7-e69d-467f-b37f-ff8ecf8f99f2 --clientSideComponentProperties '{\\\"testMessage\\\":\\\"Test message\\\"}'",
            "Note, how the clientSideComponentProperties option has escaped double quotes `'{\\\"testMessage\\\":\\\"Test message\\\"}'` compared to execution from bash `'{\"testMessage\":\"Test message\"}'`.",
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "To use this command, you need to be a SharePoint Admin.",
            "This command can be used for configuring a tenant-wide ListView Command Set. To configure a ListView Command Set on a specific site, view our dedicated spo commandset add command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Adds a ListView Command Set that's deployed tenant wide to CommandBars of Lists.",
                "m365 spo tenant commandset add --title \"Some customizer\" --clientSideComponentId  799883f5-7962-4384-a10a-105adaec6ffc --listType List",
            ),
            (
                "Adds a ListView Command Set that is configured to the context menu of communication site libraries.",
                "m365 spo tenant commandset  add --title \"Some customizer\" --clientSideComponentId  799883f5-7962-4384-a10a-105adaec6ffc --webTemplate \"SITEPAGEPUBLISHING#0\" --listType Library --location ContextMenu",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant commandset get",
        summary: "Gets a ListView Command Set that is installed tenant-wide",
        options: &[
            ("title", "The title of the ListView Command Set."),
            ("id", "The id of the ListView Command Set."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set.",
            ),
            (
                "tenantWideExtensionComponentProperties",
                "Only output the tenant wide extension component properties.",
            ),
        ],
        notes: &[
            "This command can be used for retrieving a ListView Command Set that's installed tenant-wide. To retrieve a ListView Command Set from a specific site, view our dedicated spo commandset get command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Retrieves a ListView Command Set by title.",
                "m365 spo tenant commandset get --title \"Some customizer\"",
            ),
            (
                "Retrieves a ListView Command Set by id.",
                "m365 spo tenant commandset get --id 3",
            ),
            (
                "Retrieves a ListView Command Set by clientSideComponentId.",
                "m365 spo tenant commandset get --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant commandset list",
        summary: "Retrieves a list of ListView Command Sets that are installed tenant-wide",
        options: &[],
        notes: &[
            "This command can be used for retrieving a list of ListView Command Sets that are installed tenant-wide. To retrieve a list of ListView Command Sets added to a specific site, view our dedicated spo commandset list command.",
        ],
        permissions: &[],
        examples: &[(
            "Retrieves a list of ListView Command Sets.",
            "m365 spo tenant commandset list",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant commandset remove",
        summary: "Removes a ListView Command Set that is installed tenant-wide",
        options: &[
            ("title", "The title of the ListView Command Set."),
            ("id", "The id of the ListView Command Set."),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "This command can be used for removing a ListView Command Set that's installed tenant-wide. To remove a ListView Command Set from a specific site, view our dedicated spo commandset remove command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes an ListView Command Set by title.",
                "m365 spo tenant commandset remove --title \"Some command set\"",
            ),
            (
                "Removes a ListView Command Set by id.",
                "m365 spo tenant commandset remove --id 3",
            ),
            (
                "Removes a ListView Command Set by clientSideComponentId.",
                "m365 spo tenant commandset remove --clientSideComponentId 7096cded-b83d-4eab-96f0-df477ed7c0bc",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant commandset set",
        summary: "Updates a ListView Command Set that is installed tenant-wide",
        options: &[
            (
                "id",
                "The id of the ListView Command Set to update. Specify either `title`, `id` or `clientSideComponentId`.",
            ),
            (
                "clientSideComponentId",
                "The Client Side Component Id (GUID) of the ListView Command Set to update. Specify either `title`, `id` or `clientSideComponentId`.",
            ),
            ("newTitle", "The updated title of the ListView Command Set."),
            (
                "listType",
                "The list or library type to register the ListView Command Set on. Allowed values `List` or `Library`.",
            ),
            (
                "clientSideComponentProperties",
                "The Client Side Component properties of the ListView Command Set.",
            ),
            (
                "webTemplate",
                "Optionally add a web template (e.g. STS#3, SITEPAGEPUBLISHING#0, etc) as a filter for what kind of sites the ListView Command Set is registered on.",
            ),
            (
                "location",
                "The location of the ListView Command Set. Allowed values `ContextMenu`, `CommandBar` or `Both`. Defaults to `CommandBar`.",
            ),
        ],
        notes: &[
            "When using the `--clientSideComponentProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "This command can be used for updating a ListView Command Set that's installed tenant-wide. To update a ListView Command Set on a specific site, view our dedicated spo commandset set command.",
        ],
        permissions: &[],
        examples: &[
            (
                "Updates the title of a ListView Command Set that's deployed tenant wide by its id.",
                "m365 spo tenant commandset set --id 4 --newTitle \"Some customizer\"",
            ),
            (
                "Updates the properties of a ListView Command Set that's deployed tenant wide by its title.",
                "m365 spo tenant commandset set --newTitle \"Some Command Set\" --clientSideComponentProperties '{ \"someProperty\": \"Some value\" }'",
            ),
            (
                "Updates the Client Side Component Id (GUID) of a ListView Command Set that's deployed tenant wide by its clientSideComponentId.",
                "m365 spo tenant commandset set --clientSideComponentId \"07ee7d3d-923e-49be-b050-86f53edb45fb\" --clientSideComponentId \"b44a5182-9877-4029-baec-0181c70dacbc\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant recyclebinitem list",
        summary: "Returns all modern and classic site collections in the tenant scoped recycle bin",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Returns all modern and classic site collections in the tenant scoped recycle bin",
            "m365 spo tenant recyclebinitem list",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant recyclebinitem remove",
        summary: "Removes the specified deleted Site Collection from Tenant Recycle Bin",
        options: &[
            ("siteUrl", "URL of the site to remove"),
            (
                "wait",
                "Wait for the site collection to be removed before completing the command",
            ),
            (
                "force",
                "Don't prompt for confirming removing the deleted site collection",
            ),
        ],
        notes: &[
            "Removing a site collection is by default asynchronous and depending on the current state of Microsoft 365, might take up to few minutes. If you're building a script with steps that require the site to be fully removed, you should use the `--wait` flag. When using this flag, the `m365 spo tenant recyclebinitem remove` command will keep running until it received confirmation from Microsoft 365 that the site has been fully removed.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Removes the specified deleted site collection from tenant recycle bin",
                "m365 spo tenant recyclebinitem remove --siteUrl https://contoso.sharepoint.com/sites/team",
            ),
            (
                "Removes the specified deleted site collection from tenant recycle bin and wait for the removing process to complete",
                "m365 spo tenant recyclebinitem remove --siteUrl https://contoso.sharepoint.com/sites/team --wait",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant recyclebinitem restore",
        summary: "Restores the specified deleted Site Collection from Tenant Recycle Bin",
        options: &[("siteUrl", "URL of the site.")],
        notes: &["To use this command you must be a Global or SharePoint administrator."],
        permissions: &[],
        examples: &[(
            "Restores a deleted site collection from tenant recycle bin.",
            "m365 spo tenant recyclebinitem restore --siteUrl https://contoso.sharepoint.com/sites/team",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant settings list",
        summary: "Lists the global tenant settings",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Lists the settings of the tenant",
            "m365 spo tenant settings list",
        )],
        links: &[],
    },
    Help {
        name: "spo tenant settings set",
        summary: "Sets tenant global settings",
        options: &[
            (
                "MinCompatibilityLevel",
                "Specifies the lower bound on the compatibility level for new sites.",
            ),
            (
                "MaxCompatibilityLevel",
                "Specifies the upper bound on the compatibility level for new sites.",
            ),
            (
                "ExternalServicesEnabled",
                "Enables external services for a tenant. External services are defined as services that are not in the Microsoft 365 data centers. Allowed values `true`, `false`.",
            ),
            (
                "NoAccessRedirectUrl",
                "Specifies the URL of the redirected site for those site collections which have the locked state \"NoAccess\".",
            ),
            (
                "SharingCapability",
                "Determines what level of sharing is available for the site. The valid values are: ExternalUserAndGuestSharing (default) - External user sharing (share by email) and guest link sharing are both enabled. Disabled - External user sharing (share by email) and guest link sharing are both disabled. ExternalUserSharingOnly - External user sharing (share by email) is enabled, but guest link sharing is disabled. Allowed values `Disabled`, `ExternalUserSharingOnly`, `ExternalUserAndGuestSharing`, `ExistingExternalUserSharingOnly`.",
            ),
            (
                "DisplayStartASiteOption",
                "Determines whether tenant users see the Start a Site menu option. Allowed values `true`, `false`.",
            ),
            (
                "StartASiteFormUrl",
                "Specifies URL of the form to load in the Start a Site dialog. The valid values are: \"\" (default) - Blank by default, this will also remove or clear any value that has been set. Full URL - Example:\"https://contoso.sharepoint.com/path/to/form\".",
            ),
            (
                "ShowEveryoneClaim",
                "Enables the administrator to hide the Everyone claim in the People Picker. When users share an item with Everyone, it is accessible to all authenticated users in the tenant's Microsoft Entra ID, including any active external users who have previously accepted invitations. Note, that some SharePoint system resources such as templates and pages are required to be shared with Everyone and this type of sharing does not expose any user data or metadata. Allowed values `true`, `false`.",
            ),
            (
                "ShowAllUsersClaim",
                "Enables the administrator to hide the All Users claim groups in People Picker. When users share an item with \"All Users (x)\", it is accessible to all organization members in the tenant's Microsoft Entra ID who have authenticated via this method. When users share an item with \"All Users (x)\" it is accessible to all organization members in the tenant that used NTLM to authenticate with SharePoint. Allowed values `true`, `false`.",
            ),
            (
                "ShowEveryoneExceptExternalUsersClaim",
                "Enables the administrator to hide the \"Everyone except external users\" claim in the People Picker. When users share an item with \"Everyone except external users\", it is accessible to all organization members in the tenant's Microsoft Entra ID, but not to any users who have previously accepted invitations. Allowed values `true`, `false`.",
            ),
            (
                "SearchResolveExactEmailOrUPN",
                "Removes the search capability from People Picker. Note, recently resolved names will still appear in the list until the browser cache is cleared or expired. SharePoint Administrators will still be able to use starts with or partial name matching when enabled. Allowed values `true`, `false`.",
            ),
            (
                "OfficeClientADALDisabled",
                "When set to true this will disable the ability to use Modern Authentication that leverages ADAL across the tenant. Allowed values `true`, `false`.",
            ),
            (
                "LegacyAuthProtocolsEnabled",
                "By default, this value is set to true. Setting this parameter prevents Office clients using non-modern authentication protocols from accessing SharePoint Online resources. A value of true - Enables Office clients using non-modern authentication protocols (such as Forms-Based Authentication (FBA) or Identity Client Runtime Library (IDCRL)) to access SharePoint resources. Allowed values `true`, `false`.",
            ),
            (
                "RequireAcceptingAccountMatchInvitedAccount",
                "Ensures that an external user can only accept an external sharing invitation with an account matching the invited email address. Administrators who desire increased control over external collaborators should consider enabling this feature. Allowed values `true`, `false`.",
            ),
            (
                "ProvisionSharedWithEveryoneFolder",
                "Creates a Shared with Everyone folder in every user's new OneDrive for Business document library. The valid values are: True (default) - The Shared with Everyone folder is created. False - No folder is created when the site and OneDrive for Business document library are created. Allowed values `true`, `false`.",
            ),
            (
                "SignInAccelerationDomain",
                "Specifies the home realm discovery value to be sent to Microsoft Entra ID during the user sign-in process. When the organization uses a third-party identity provider, this prevents the user from seeing the Microsoft Entra Home Realm Discovery web page and ensures the user only sees their company's Identity Provider's portal. This value can also be used with Microsoft Entra ID Premium to customize the Microsoft Entra ID login page. Acceleration will not occur on-site collections that are shared externally. This value should be configured with the login domain that is used by your company (that is, example@contoso.com). If your company has multiple third-party identity providers, configuring the sign-in acceleration value will break sign-in for your organization. The valid values are: \"\" (default) - Blank by default, this will also remove or clear any value that has been set. Login Domain - For example: \"contoso.com\". No value assigned by default.",
            ),
            (
                "EnableGuestSignInAcceleration",
                "Accelerates guest-enabled site collections as well as member-only site collections when the SignInAccelerationDomain parameter is set. Allowed values `true`, `false`.",
            ),
            (
                "UsePersistentCookiesForExplorerView",
                "Lets SharePoint issue a special cookie that will allow this feature to work even when \"Keep Me Signed In\" is not selected. \"Open with Explorer\" requires persisted cookies to operate correctly. When the user does not select \"Keep Me Signed in\" at the time of sign-in, \"Open with Explorer\" will fail. This special cookie expires after 30 minutes and cannot be cleared by closing the browser or signing out of SharePoint Online.To clear this cookie, the user must log out of their Windows session. The valid values are: False(default) - No special cookie is generated and the normal Microsoft 365 sign-in length/timing applies. True - Generates a special cookie that will allow \"Open with Explorer\" to function if the \"Keep Me Signed In\" box is not checked at sign-in. Allowed values `true`, `false`.",
            ),
            (
                "BccExternalSharingInvitations",
                "When the feature is enabled, all external sharing invitations that are sent will blindly copy the e-mail messages listed in the BccExternalSharingsInvitationList. Allowed values `true`, `false`.",
            ),
            (
                "BccExternalSharingInvitationsList",
                "Specifies a list of e-mail addresses to be BCC'd when the BCC for External Sharing feature is enabled. Multiple addresses can be specified by creating a comma-separated list with no spaces.",
            ),
            (
                "UserVoiceForFeedbackEnabled",
                "Enables or disables the User Voice Feedback button. Allowed values `true`, `false`.",
            ),
            (
                "PublicCdnEnabled",
                "Enables or disables the publish CDN. Allowed values `true`, `false`.",
            ),
            (
                "PublicCdnAllowedFileTypes",
                "Sets public CDN-allowed file types.",
            ),
            (
                "RequireAnonymousLinksExpireInDays",
                "Specifies all anonymous links that have been created (or will be created) will expire after the set number of days. To remove the expiration requirement, set the value to zero (0).",
            ),
            (
                "SharingAllowedDomainList",
                "Specifies a list of email domains that are allowed for sharing with external collaborators. Use the space character as the delimiter for entering multiple values. For example, \"contoso.com fabrikam.com\".",
            ),
            (
                "SharingBlockedDomainList",
                "Specifies a list of email domains that are blocked or prohibited from sharing with external collaborators. Use space character as the delimiter for entering multiple values. For example, \"contoso.com fabrikam.com\".",
            ),
            (
                "SharingDomainRestrictionMode",
                "Specifies the external sharing mode for domains. Allowed values `None`, `AllowList`, `BlockList`.",
            ),
            (
                "OneDriveStorageQuota",
                "Sets a default OneDrive for Business storage quota for the tenant. It will be used for new OneDrive for Business sites created. A typical use will be to reduce the amount of storage associated with OneDrive for Business to a level below what the License entitles the users. For example, it could be used to set the quota to 10 gigabytes (GB) by default.",
            ),
            (
                "OneDriveForGuestsEnabled",
                "Lets OneDrive for Business creation for administrator-managed guest users. Administrator-managed Guest users use credentials in the resource tenant to access the resources. Allowed values `true`, `false`.",
            ),
            (
                "IPAddressEnforcement",
                "Allows access from network locations that are defined by an administrator. The values are true and false. The default value is false which means the setting is disabled. Before the iPAddressEnforcement parameter is set, make sure you add a valid IPv4 or IPv6 address to the iPAddressAllowList parameter. Allowed values `true`, `false`.",
            ),
            (
                "IPAddressAllowList",
                "Configures multiple IP addresses or IP address ranges (IPv4 or IPv6). Use commas to separate multiple IP addresses or IP address ranges. Verify there are no overlapping IP addresses and ensure IP ranges use Classless Inter-Domain Routing (CIDR) notation. For example, `172.16.0.0, 192.168.1.0/27`. No value is assigned by default.",
            ),
            (
                "IPAddressWACTokenLifetime",
                "Sets IP Address WAC token lifetime'.",
            ),
            (
                "UseFindPeopleInPeoplePicker",
                "Sets use to find people in PeoplePicker to true or false. Note: When set to true, users aren't able to share with security groups or SharePoint groups. Allowed values `true`, `false`.",
            ),
            (
                "DefaultSharingLinkType",
                "Let's administrators choose what type of link appears is selected in the “Get a link” sharing dialog box in OneDrive for Business and SharePoint Online. Allowed values `None`, `Direct`, `Internal`, `AnonymousAccess`.",
            ),
            (
                "ODBMembersCanShare",
                "Lets administrators set policy on re-sharing behavior in OneDrive for Business. Allowed values `Unspecified`, `On`, `Off`.",
            ),
            (
                "ODBAccessRequests",
                "Lets administrators set policy on access requests and requests to share in OneDrive for Business. Allowed values `Unspecified`, `On`, `Off`.",
            ),
            (
                "PreventExternalUsersFromResharing",
                "Prevents external users from resharing. Allowed values `true`, `false`.",
            ),
            (
                "ShowPeoplePickerSuggestionsForGuestUsers",
                "Shows people picker suggestions for guest users. Allowed values `true`, `false`.",
            ),
            (
                "FileAnonymousLinkType",
                "Sets the file anonymous link type to `None`, `View` or `Edit`.",
            ),
            (
                "FolderAnonymousLinkType",
                "Sets the folder anonymous link type to `None`, `View` or `Edit`.",
            ),
            (
                "NotifyOwnersWhenItemsReshared",
                "When this parameter is set to true and another user re-shares a document from a user's OneDrive for Business, the OneDrive for Business owner is notified by email. For additional information about how to configure notifications for external sharing, see Configure notifications for external sharing for OneDrive for Business. Allowed values `true`, `false`.",
            ),
            (
                "NotifyOwnersWhenInvitationsAccepted",
                "When this parameter is set to true and when an external user accepts an invitation to a resource in a user's OneDrive for Business, the OneDrive for Business owner is notified by email. For additional information about how to configure notifications for external sharing, see Configure notifications for external sharing for OneDrive for Business. Allowed values `true`, `false`.",
            ),
            (
                "NotificationsInOneDriveForBusinessEnabled",
                "Enables or disables notifications in OneDrive for business. Allowed values `true`, `false`.",
            ),
            (
                "NotificationsInSharePointEnabled",
                "Enables or disables notifications in SharePoint. Allowed values `true`, `false`.",
            ),
            (
                "OwnerAnonymousNotification",
                "Enables or disables owner-anonymous notification. Allowed values `true`, `false`.",
            ),
            (
                "CommentsOnSitePagesDisabled",
                "Enables or disables comments on site pages. Allowed values `true`, `false`.",
            ),
            (
                "SocialBarOnSitePagesDisabled",
                "Enables or disables social bar on site pages. Allowed values `true`, `false`.",
            ),
            (
                "OrphanedPersonalSitesRetentionPeriod",
                "Specifies the number of days after a user's Active Directory account is deleted that their OneDrive for Business content will be deleted. The value range is in days, between 30 and 3650. The default value is 30.",
            ),
            (
                "DisallowInfectedFileDownload",
                "Prevents the Download button from being displayed on the Virus Found warning page. Allowed values `true`, `false`.",
            ),
            (
                "DefaultLinkPermission",
                "Choose the default permission that is selected when users share. This applies to anonymous access, internal and direct links. Allowed values `None`, `View`, `Edit`.",
            ),
            (
                "ConditionalAccessPolicy",
                "Configures conditional access policy. Allowed values `AllowFullAccess`, `AllowLimitedAccess`, `BlockAccess`.",
            ),
            (
                "AllowDownloadingNonWebViewableFiles",
                "Allows downloading non-web viewable files. The Allowed values `true`, `false`.",
            ),
            (
                "AllowEditing",
                "Allows editing. Allowed values `true`, `false`.",
            ),
            (
                "ApplyAppEnforcedRestrictionsToAdHocRecipients",
                "Applies app-enforced restrictions to AdHoc recipients. Allowed values `true`, `false`.",
            ),
            (
                "FilePickerExternalImageSearchEnabled",
                "Enables file picker external image search. Allowed values `true`, `false`.",
            ),
            (
                "EmailAttestationRequired",
                "Sets email attestation to required. Allowed values `true`, `false`.",
            ),
            (
                "EmailAttestationReAuthDays",
                "Sets email attestation re-auth days.",
            ),
            (
                "HideDefaultThemes",
                "Defines if the default themes are visible or hidden. Allowed values `true`, `false`.",
            ),
            (
                "BlockAccessOnUnmanagedDevices",
                "Blocks access on unmanaged devices. Allowed values `true`, `false`.",
            ),
            (
                "AllowLimitedAccessOnUnmanagedDevices",
                "Allows limited access on unmanaged device blocks. Allowed values `true`, `false`.",
            ),
            (
                "BlockDownloadOfAllFilesForGuests",
                "Blocks download of all files for guests. Allowed values `true`, `false`.",
            ),
            (
                "BlockDownloadOfAllFilesOnUnmanagedDevices",
                "Blocks download of all files on unmanaged devices. Allowed values `true`, `false`.",
            ),
            (
                "BlockDownloadOfViewableFilesForGuests",
                "Blocks download of viewable files for guests. Allowed values `true`, `false`.",
            ),
            (
                "BlockDownloadOfViewableFilesOnUnmanagedDevices",
                "Blocks download of viewable files on unmanaged devices. Allowed values `true`, `false`.",
            ),
            (
                "BlockMacSync",
                "Blocks Mac sync. Allowed values `true`, `false`.",
            ),
            (
                "DisableReportProblemDialog",
                "Disables report problem dialog. Allowed values `true`, `false`.",
            ),
            (
                "DisplayNamesOfFileViewers",
                "Displays names of file viewers. Allowed values `true`, `false`.",
            ),
            (
                "EnableMinimumVersionRequirement",
                "Enables minimum version requirement. Allowed values `true`, `false`.",
            ),
            (
                "HideSyncButtonOnODB",
                "Hides the sync button on One Drive for Business. Allowed values `true`, `false`.",
            ),
            (
                "IsUnmanagedSyncClientForTenantRestricted",
                "Is unmanaged sync client for tenant restricted. Allowed values `true`, `false`.",
            ),
            (
                "LimitedAccessFileType",
                "Allows users to preview only Office files in the browser. This option increases security but may be a barrier to user productivity. Allowed values `OfficeOnlineFilesOnly`, `WebPreviewableFiles`, `OtherFiles`.",
            ),
            (
                "OptOutOfGrooveBlock",
                "Opts out of the groove block. Allowed values `true`, `false`.",
            ),
            (
                "OptOutOfGrooveSoftBlock",
                "Opts out of Groove soft block. Allowed values `true`, `false`.",
            ),
            ("OrgNewsSiteUrl", "Organization news site url."),
            (
                "PermissiveBrowserFileHandlingOverride",
                "Permissive browser file handling override. Allowed values `true`, `false`.",
            ),
            (
                "ShowNGSCDialogForSyncOnODB",
                "Show NGSC dialog for sync on OneDrive for Business. Allowed values `true`, `false`.",
            ),
            (
                "SpecialCharactersStateInFileFolderNames",
                "Sets the special characters state in file and folder names in SharePoint and OneDrive for Business. Allowed values `NoPreference`, `Allowed`, `Disallowed`.",
            ),
            (
                "SyncPrivacyProfileProperties",
                "Syncs privacy profile properties. Allowed values `true`, `false`.",
            ),
            (
                "ExcludedFileExtensionsForSyncClient",
                "Excluded file extensions for sync client. Array of strings split by comma.",
            ),
            (
                "AllowedDomainListForSyncClient",
                "Sets allowed domain list for sync client. Array of GUIDs split by comma. Example: `c9b1909e-901a-0000-2cdb-e91c3f46320a,c9b1909e-901a-0000-2cdb-e91c3f463201`.",
            ),
            (
                "DisabledWebPartIds",
                "Sets disabled web part Ids. Array of GUIDs split by comma. Example: `c9b1909e-901a-0000-2cdb-e91c3f46320a,c9b1909e-901a-0000-2cdb-e91c3f463201`.",
            ),
            (
                "DisableCustomAppAuthentication",
                "Configure if ACS-based app-only auth should be disabled or not. Allowed values `true`, `false`.",
            ),
            (
                "CommentsOnListItemsDisabled",
                "Enables or disables comments on lists. Allowed values `true`, `false`.",
            ),
            (
                "EnableAzureADB2BIntegration",
                "Enables the preview for OneDrive and SharePoint integration with Microsoft Entra B2B. Allowed values `true`, `false`. Microsoft Entra one-time passcode needs to be enabled for this integration to work. For more information see https://aka.ms/spo-b2b-integration.",
            ),
            (
                "SyncAadB2BManagementPolicy",
                "Syncs Azure B2B Management Policies. Allowed values `true`, `false`. For more information, see SharePoint and OneDrive integration with Microsoft Entra B2B.",
            ),
            (
                "AllowWebPropertyBagUpdateWhenDenyAddAndCustomizePagesIsEnabled",
                "Enables or disables web property bag updates when DenyAddAndCustomizePages is enabled. Allowed values `true`, `false`.",
            ),
        ],
        notes: &[
            "To use this command you have to have permission to access the tenant admin site.",
        ],
        permissions: &[],
        examples: &[
            (
                "Sets single tenant global setting.",
                "m365 spo tenant settings set --UserVoiceForFeedbackEnabled true",
            ),
            (
                "Sets multiple tenant global settings at once.",
                "m365 spo tenant settings set --UserVoiceForFeedbackEnabled true --HideSyncButtonOnODB true --AllowedDomainListForSyncClient c9b1909e-901a-0000-2cdb-e91c3f46320a,c9b1909e-901a-0000-2cdb-e91c3f463201",
            ),
            (
                "Enable Microsoft Entra B2B integration for SharePoint and OneDrive and sync the Microsoft Entra B2B management policies.",
                "m365 spo tenant settings set --EnableAzureADB2BIntegration true\nm365 spo tenant settings set --SyncAadB2BManagementPolicy true",
            ),
            (
                "Disable Microsoft Entra B2B integration for SharePoint and OneDrive.",
                "m365 spo tenant settings set --EnableAzureADB2BIntegration false",
            ),
            (
                "Disable comments on list items for SharePoint and Microsoft Lists.",
                "m365 spo tenant settings set --CommentsOnListItemsDisabled true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo tenant site get",
        summary: "Retrieves the tenant site information",
        options: &[
            ("id", "The ID of the site collection."),
            ("title", "Title of the site collection."),
            ("url", "The site URL of the site collection."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Retrieve info about a site using its ID",
                "m365 spo tenant site get --id 3ae83bc5-1f27-45c1-9eee-1bd1e2ddce69",
            ),
            (
                "Retrieve info about a site using its URL",
                "m365 spo tenant site get --url \"https://contoso.sharepoint.com/sites/Marketing\"",
            ),
            (
                "Retrieve info about a site using its Title",
                "m365 spo tenant site get --title Marketing",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term add",
        summary: "Adds taxonomy term",
        options: &[
            ("name", "Name of the term to add"),
            (
                "webUrl",
                "If specified, allows you to add a term to the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            (
                "termSetId",
                "ID of the term set in which to create the term. Specify `termSetId` or `termSetName` but not both",
            ),
            (
                "termSetName",
                "Name of the term set in which to create the term. Specify `termSetId` or `termSetName` but not both",
            ),
            (
                "termGroupId",
                "ID of the term group to which the term set belongs.",
            ),
            (
                "termGroupName",
                "Name of the term group to which the term set belongs.",
            ),
            ("id", "ID of the term to add"),
            ("description", "Description of the term to add"),
            (
                "parentTermId",
                "ID of the term below which the term should be added",
            ),
            (
                "customProperties",
                "JSON string with key-value pairs representing custom properties to set on the term",
            ),
            (
                "localCustomProperties",
                "JSON string with key-value pairs representing local custom properties to set on the term",
            ),
        ],
        notes: &[
            "When using the `--customProperties` and/or `--localCustomProperties` options it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "To use this command without the `--webUrl` option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. It allows you to add a term to a term set in the tenant term store if you are listed as a term store administrator. It allows you to add terms to a term set in the sitecollection term store if you are a site owner.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add taxonomy term with the specified name to the term group and term set specified by their names.",
                "m365 spo term add --name IT --termSetName Department --termGroupName People",
            ),
            (
                "Add taxonomy term with the specified name to the term group and term set specified by their IDs.",
                "m365 spo term add --name IT --termSetId 8ed8c9ea-7052-4c1d-a4d7-b9c10bffea6f --termGroupId 5c928151-c140-4d48-aab9-54da901c7fef",
            ),
            (
                "Add taxonomy term with the specified name and ID.",
                "m365 spo term add --name IT --id 5c928151-c140-4d48-aab9-54da901c7fef --termSetName Department --termGroupName People",
            ),
            (
                "Add taxonomy term to the specified sitecollection with the specified name and ID.",
                "m365 spo term add --name IT --id 5c928151-c140-4d48-aab9-54da901c7fef --termSetName Department --termGroupName People --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Add taxonomy term with custom properties.",
                "m365 spo term add --name IT --termSetName Department --termGroupName People --customProperties '{\"Property\": \"Value\"}'",
            ),
            (
                "Add taxonomy term below the specified term.",
                "m365 spo term add --name IT --parentTermId 5c928151-c140-4d48-aab9-54da901c7fef --termGroupName People",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term get",
        summary: "Gets information about the specified taxonomy term",
        options: &[
            (
                "webUrl",
                "If specified, allows you to get a term from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            ("id", "ID of the term to retrieve."),
            ("name", "Name of the term to retrieve."),
            (
                "termGroupId",
                "ID of the term group to which the term set belongs. Specify `termGroupId` or `termGroupName` but not both.",
            ),
            (
                "termGroupName",
                "Name of the term group to which the term set belongs. Specify `termGroupId` or `termGroupName` but not both.",
            ),
            (
                "termSetId",
                "ID of the term set to which the term belongs. Specify `termSetId` or `termSetName` but not both.",
            ),
            (
                "termSetName",
                "Name of the term set to which the term belongs. Specify `termSetId` or `termSetName` but not both.",
            ),
        ],
        notes: &[
            "When retrieving term by its ID, it's sufficient to specify just the ID. When retrieving it by its name however, you need to specify the parent term group and term set using either their names or IDs.",
            "To use this command without the --webUrl option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to get a term from the tenant term store as well as a term from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about a taxonomy term using its ID from the specified sitecollection.",
                "m365 spo term get --webUrl https://contoso.sharepoint.com/sites/project-x --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "Get information about a taxonomy term using its ID.",
                "m365 spo term get --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "Get information about a taxonomy term using its name, retrieving the parent term group and term set using their names.",
                "m365 spo term get --name IT --termGroupName People --termSetName Department",
            ),
            (
                "Get information about a taxonomy term using its name, retrieving the parent term group and term set using their IDs.",
                "m365 spo term get --name IT --termGroupId 5c928151-c140-4d48-aab9-54da901c7fef --termSetId 8ed8c9ea-7052-4c1d-a4d7-b9c10bffea6f",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term group add",
        summary: "Adds taxonomy term group",
        options: &[
            ("name", "Name of the term group to add"),
            ("id", "ID of the term group to add"),
            ("description", "Description of the term group to add"),
            (
                "webUrl",
                "If specified, allows you to add a term group to the tenant term store as a term store administrator without the SharePoint administrator role.",
            ),
        ],
        notes: &[
            "To use this command without the `--webUrl` option you must have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Adminstrator role. It allows you to add a term group to the tenant term store if you are listed as a term store administrator.",
            "This command does not create a sitecollection specific term store/group when using the `--webUrl` option.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add a new taxonomy term group with the specified name",
                "m365 spo term group add --name PnPTermSets",
            ),
            (
                "Add a new taxonomy term group with the specified name and id",
                "m365 spo term group add --name PnPTermSets --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "Add a new taxonomy term group with the specified name and description with limited permissions",
                "m365 spo term group add --name PnPTermSets --description 'Term sets for PnP' --webUrl 'https://contoso.sharepoint.com'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term group get",
        summary: "Gets information about the specified taxonomy term group",
        options: &[
            (
                "webUrl",
                "If specified, allows you to get a term group from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            ("id", "ID of the term group to retrieve."),
            ("name", "Name of the term group to retrieve."),
        ],
        notes: &[
            "To use this command without the `--webUrl` option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to get a term group from the tenant term store as well as a term group from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about a taxonomy term group using its ID.",
                "m365 spo term group get --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "Get information about a taxonomy term group using its name.",
                "m365 spo term group get --name PnPTermSets",
            ),
            (
                "Get information about a taxonomy term group from the specified sitecollection using its ID.",
                "m365 spo term group get --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term group list",
        summary: "Lists taxonomy term groups",
        options: &[(
            "webUrl",
            "If specified, allows you to list term groups from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
        )],
        notes: &[
            "To use this command without the --webUrl option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to list term groups from the tenant term store as well as term groups from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            ("List taxonomy term groups.", "m365 spo term group list"),
            (
                "List taxonomy term groups from the specified sitecollection.",
                "m365 spo term group list --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term list",
        summary: "Lists taxonomy terms from the given term set",
        options: &[
            (
                "webUrl",
                "If specified, allows you to list terms from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            (
                "termGroupId",
                "ID of the term group where the term set is located.",
            ),
            (
                "termGroupName",
                "Name of the term group where the term set is located.",
            ),
            (
                "termSetId",
                "ID of the term set for which to retrieve terms.",
            ),
            (
                "termSetName",
                "Name of the term set for which to retrieve terms.",
            ),
            (
                "withChildTerms",
                "If specified, child terms are loaded as well.",
            ),
        ],
        notes: &[
            "To use this command without the --webUrl option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to list terms from the tenant term store as well as terms from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            (
                "List taxonomy terms from the specified sitecollection, the term group and term set with the given name",
                "m365 spo term list --webUrl https://contoso.sharepoint.com/sites/project-x --termGroupName PnPTermSets --termSetName PnP-Organizations",
            ),
            (
                "List taxonomy terms from the term group and term set with the given name.",
                "m365 spo term list --termGroupName PnPTermSets --termSetName PnP-Organizations",
            ),
            (
                "List taxonomy terms from the term group and term set with the given ID.",
                "m365 spo term list --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb --termSetId 0e8f395e-ff58-4d45-9ff7-e331ab728bec",
            ),
            (
                "List taxonomy terms from the term group and term set with the given ID including child terms if any are found.",
                "m365 spo term list --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb --termSetId 0e8f395e-ff58-4d45-9ff7-e331ab728bec --withChildTerms",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term set add",
        summary: "Adds taxonomy term set",
        options: &[
            ("name", "Name of the term set to add."),
            (
                "webUrl",
                "If specified, allows you to add a term set to the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            (
                "termGroupId",
                "ID of the term group in which to create the term set.",
            ),
            (
                "termGroupName",
                "Name of the term group in which to create the term set.",
            ),
            ("id", "ID of the term set to add."),
            ("description", "Description of the term set to add."),
            (
                "customProperties",
                "JSON string with key-value pairs representing custom properties to set on the term set.",
            ),
        ],
        notes: &[
            "When using the `--customProperties` option it's possible to enter a JSON string. In PowerShell 5 to 7.2 specific escaping rules apply due to an issue. Remember that you can also use file tokens instead.",
            "To use this command without the `--webUrl` option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to add a term set to a term group in the tenant term store if you are listed as a term store administrator. It allows you to add a term set to a term group in the sitecollection term store if you are a site owner.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add taxonomy term set to the term group specified by ID.",
                "m365 spo term set add --name PnP-Organizations --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "Add taxonomy term set to the specified sitecollection's term group specified by ID.",
                "m365 spo term set add --name PnP-Organizations --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Add taxonomy term set to the term group specified by name. Create the term set with the specified ID.",
                "m365 spo term set add --name PnP-Organizations --termGroupName PnPTermSets --id aa70ede6-83d1-466d-8d95-30d29e9bbd7c",
            ),
            (
                "Add taxonomy term set and set its description.",
                "m365 spo term set add --name PnP-Organizations --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb --description 'Contains a list of organizations'",
            ),
            (
                "Add taxonomy term set and set its custom properties.",
                "m365 spo term set add --name PnP-Organizations --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb --customProperties '{\"Property\":\"Value\"}'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term set get",
        summary: "Gets information about the specified taxonomy term set",
        options: &[
            (
                "webUrl",
                "If specified, allows you to get a term set from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            ("id", "ID of the term set to retrieve."),
            ("name", "Name of the term set to retrieve."),
            (
                "termGroupId",
                "ID of the term group to which the term set belongs.",
            ),
            (
                "termGroupName",
                "Name of the term group to which the term set belongs.",
            ),
        ],
        notes: &[
            "To use this command without the --webUrl option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to get a term set from the tenant term store as well as a term set from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about a taxonomy term set using its ID.",
                "m365 spo term set get --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb --termGroupName PnPTermSets",
            ),
            (
                "Get information about a taxonomy term set using its name.",
                "m365 spo term set get --name PnP-Organizations --termGroupId 0a099ee9-e231-4ae9-a5b6-d7f94a0d241d",
            ),
            (
                "Get information about a taxonomy term set using its ID from the specified sitecollection.",
                "m365 spo term set get --id 0e8f395e-ff58-4d45-9ff7-e331ab728beb --termGroupName PnPTermSets --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo term set list",
        summary: "Lists taxonomy term sets from the given term group",
        options: &[
            (
                "webUrl",
                "If specified, allows you to list term sets from the tenant term store as well as the sitecollection specific term store. Defaults to the tenant admin site.",
            ),
            (
                "termGroupId",
                "ID of the term group from which to retrieve term sets.",
            ),
            (
                "termGroupName",
                "Name of the term group from which to retrieve term sets.",
            ),
        ],
        notes: &[
            "To use this command without the --webUrl option you have to have permissions to access the tenant admin site.",
            "When using the `--webUrl` option you can connect to the term store with limited permissions, and do not need the SharePoint Administrator role. You need to be a site visitor or more. It allows you to list term sets from the tenant term store as well as term sets from the sitecollection term store.",
        ],
        permissions: &[],
        examples: &[
            (
                "List taxonomy term sets from the term group with the given name.",
                "m365 spo term set list --termGroupName PnPTermSets",
            ),
            (
                "List taxonomy term sets from the term group with the given ID.",
                "m365 spo term set list --termGroupId 0e8f395e-ff58-4d45-9ff7-e331ab728beb",
            ),
            (
                "List taxonomy term sets from the specified sitecollection, from the term group with the given name.",
                "m365 spo term set list --termGroupName PnPTermSets --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo theme apply",
        summary: "Applies theme to the specified site",
        options: &[
            ("name", "Name of the theme to apply"),
            (
                "webUrl",
                "URL of the site to which the theme should be applied",
            ),
            (
                "sharePointTheme",
                "Set to specify if the supplied theme name is a standard SharePoint theme",
            ),
        ],
        notes: &[
            "Following standard SharePoint themes are supported by the CLI for Microsoft 365: Blue, Orange, Red, Purple, Green, Gray, Dark Yellow and Dark Blue.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Apply theme to the specified site",
                "m365 spo theme apply --name Contoso-Blue --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Apply a standard SharePoint theme to the specified site",
                "m365 spo theme apply --name Blue --webUrl https://contoso.sharepoint.com/sites/project-x --sharePointTheme",
            ),
        ],
        links: &[
            (
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
                "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            ),
            (
                "https://aka.ms/themedesigner",
                "https://aka.ms/themedesigner",
            ),
        ],
    },
    Help {
        name: "spo theme get",
        summary: "Gets custom theme information",
        options: &[("name", "The name of the theme to retrieve")],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Get information about a theme",
            "m365 spo theme get --name Contoso-Blue",
        )],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
        )],
    },
    Help {
        name: "spo theme list",
        summary: "Retrieves the list of custom themes",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[("List available themes", "m365 spo theme list")],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
        )],
    },
    Help {
        name: "spo theme remove",
        summary: "Removes existing theme",
        options: &[
            ("name", "Name of the theme to remove"),
            (
                "force",
                "Do not prompt for confirmation before removing theme",
            ),
        ],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Remove theme. Will prompt for confirmation before removing the theme",
                "m365 spo theme remove --name Contoso-Blue",
            ),
            (
                "Remove theme without prompting for confirmation",
                "m365 spo theme remove --name Contoso-Blue --force",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            "https://learn.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
        )],
    },
    Help {
        name: "spo theme set",
        summary: "Adds or updates a theme",
        options: &[
            ("name", "Name of the theme to add or update"),
            ("theme", "Theme JSON contents"),
            ("isInverted", "Set to specify that the theme is inverted"),
        ],
        notes: &[
            "To prevent the accidental creation of invalid themes the CLI for Microsoft 365 implements a set of checks. These checks are executed against the provided json file. A valid theme JSON file is as follows:",
            "    {\n      \"themePrimary\": \"#d81e05\",\n      \"themeLighterAlt\": \"#fdf5f4\",\n      \"themeLighter\": \"#f9d6d2\",\n      \"themeLight\": \"#f4b4ac\",\n      \"themeTertiary\": \"#e87060\",\n      \"themeSecondary\": \"#dd351e\",\n      \"themeDarkAlt\": \"#c31a04\",\n      \"themeDark\": \"#a51603\",\n      \"themeDarker\": \"#791002\",\n      \"neutralLighterAlt\": \"#eeeeee\",\n      \"neutralLighter\": \"#f5f5f5\",\n      \"neutralLight\": \"#e1e1e1\",\n      \"neutralQuaternaryAlt\": \"#d1d1d1\",\n      \"neutralQuaternary\": \"#c8c8c8\",\n      \"neutralTertiaryAlt\": \"#c0c0c0\",\n      \"neutralTertiary\": \"#c2c2c2\",\n      \"neutralSecondary\": \"#858585\",\n      \"neutralPrimaryAlt\": \"#4b4b4b\",\n      \"neutralPrimary\": \"#333333\",\n      \"neutralDark\": \"#272727\",\n      \"black\": \"#1d1d1d\",\n      \"white\": \"#f5f5f5\"\n    }",
            "When executing the `m365 spo theme set` command the following checks are executed:",
            "- Validate if the provided theme is a valid `JSON` string.",
            "- Validate if the provided theme, once deserialized, contains all properties of the sample above.",
            "- Validate if the provided theme, once deserialized, contains only the properties of the sample above.",
            "- Validate if each of the properties contains a valid hex color value prefixed with a `#`.",
            "If any of these checks fails you are presented with a `The specified theme is not valid` error.",
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[
            (
                "Add or update a theme from a theme JSON file",
                "m365 spo theme set --name Contoso-Blue --theme @/Users/user/themes/contoso-blue.json",
            ),
            (
                "Add or update an inverted theme from a theme JSON file",
                "m365 spo theme set --name Contoso-Blue --theme @/Users/user/themes/contoso-blue.json --isInverted",
            ),
            (
                "Add or update a theme from inline JSON",
                "m365 spo theme set --name Contoso-Red --theme '{\"themePrimary\":\"#d81e05\",\"themeLighterAlt\":\"#fdf5f4\",\"themeLighter\":\"#f9d6d2\",\"themeLight\":\"#f4b4ac\",\"themeTertiary\":\"#e87060\",\"themeSecondary\":\"#dd351e\",\"themeDarkAlt\":\"#c31a04\",\"themeDark\":\"#a51603\",\"themeDarker\":\"#791002\",\"neutralLighterAlt\":\"#eeeeee\",\"neutralLighter\":\"#f5f5f5\",\"neutralLight\":\"#e1e1e1\",\"neutralQuaternaryAlt\":\"#d1d1d1\",\"neutralQuaternary\":\"#c8c8c8\",\"neutralTertiaryAlt\":\"#c0c0c0\",\"neutralTertiary\":\"#c2c2c2\",\"neutralSecondary\":\"#858585\",\"neutralPrimaryAlt\":\"#4b4b4b\",\"neutralPrimary\":\"#333333\",\"neutralDark\":\"#272727\",\"black\":\"#1d1d1d\",\"white\":\"#f5f5f5\"}'",
            ),
            (
                "Add or update an inverted theme from inline JSON",
                "m365 spo theme set --name Contoso-Red --theme '{\"themePrimary\":\"#d81e05\",\"themeLighterAlt\":\"#fdf5f4\",\"themeLighter\":\"#f9d6d2\",\"themeLight\":\"#f4b4ac\",\"themeTertiary\":\"#e87060\",\"themeSecondary\":\"#dd351e\",\"themeDarkAlt\":\"#c31a04\",\"themeDark\":\"#a51603\",\"themeDarker\":\"#791002\",\"neutralLighterAlt\":\"#eeeeee\",\"neutralLighter\":\"#f5f5f5\",\"neutralLight\":\"#e1e1e1\",\"neutralQuaternaryAlt\":\"#d1d1d1\",\"neutralQuaternary\":\"#c8c8c8\",\"neutralTertiaryAlt\":\"#c0c0c0\",\"neutralTertiary\":\"#c2c2c2\",\"neutralSecondary\":\"#858585\",\"neutralPrimaryAlt\":\"#4b4b4b\",\"neutralPrimary\":\"#333333\",\"neutralDark\":\"#272727\",\"black\":\"#1d1d1d\",\"white\":\"#f5f5f5\"}' --isInverted",
            ),
        ],
        links: &[
            (
                "https://docs.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
                "https://docs.microsoft.com/sharepoint/dev/declarative-customization/site-theming/sharepoint-site-theming-overview",
            ),
            (
                "https://aka.ms/themedesigner",
                "https://aka.ms/themedesigner",
            ),
        ],
    },
    Help {
        name: "spo user ensure",
        summary: "Ensures that a user is available on a specific site",
        options: &[
            ("webUrl", "Absolute URL of the site."),
            ("entraId", "Id of the user in Entra ID."),
            (
                "userName",
                "User's UPN (user principal name, e.g. john@contoso.com).",
            ),
            ("loginName", "The login name of the principal."),
            ("entraGroupId", "ID of the Microsoft Entra group."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Ensure a user by its Entra Id.",
                "m365 spo user ensure --webUrl https://contoso.sharepoint.com/sites/project --entraId e254750a-eaa4-44f6-9517-b74f65cdb747",
            ),
            (
                "Ensure a user by its user principal name.",
                "m365 spo user ensure --webUrl https://contoso.sharepoint.com/sites/project --userName john@contoso.com",
            ),
            (
                "Ensure a user by its login name.",
                "m365 spo user ensure --webUrl https://contoso.sharepoint.com/sites/Marketing --loginName \"i:0#.f|membership|john.doe@contoso.com\"",
            ),
            (
                "Ensure a Microsoft Entra group by ID.",
                "m365 spo user ensure --webUrl https://contoso.sharepoint.com/sites/Marketing --entraGroupId e08e899f-ba40-4e91-ab36-44d4fbaa454e",
            ),
            (
                "Ensure a Microsoft Entra group by display name.",
                "m365 spo user ensure --webUrl https://contoso.sharepoint.com/sites/Marketing --entraGroupName \"Marketing team\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo user get",
        summary: "Gets a site user within specific web",
        options: &[
            ("webUrl", "URL of the web to get the user within."),
            ("id", "ID of the user to retrieve information for."),
            ("email", "Email of the user to retrieve information for."),
            (
                "loginName",
                "Login name of the user to retrieve information for.",
            ),
            (
                "userName",
                "User's UPN (user principal name, eg. megan.bowen@contoso.com).",
            ),
            (
                "entraGroupId",
                "The object ID of the Microsoft Entra group.",
            ),
            ("entraGroupName", "The name of the Microsoft Entra group."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Get user by email for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --email john.doe@mytenant.onmicrosoft.com",
            ),
            (
                "Get user by ID for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --id 6",
            ),
            (
                "Get user by login name for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --loginName \"i:0#.f|membership|john.doe@mytenant.onmicrosoft.com\"",
            ),
            (
                "Get user by user's UPN for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --userName \"john.doe@mytenant.onmicrosoft.com\"",
            ),
            (
                "Get user by entraGroupId for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --entraGroupId f832a493-de73-4fef-87ed-8c6fffd91be6",
            ),
            (
                "Get user by entraGroupName for a web.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x --entraGroupName \"Test Members\"",
            ),
            (
                "Get the currently logged-in user.",
                "m365 spo user get --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo user list",
        summary: "Lists all the users within specific web",
        options: &[("webUrl", "URL of the web to list the users from")],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get list of users in a web",
            "m365 spo user list --webUrl https://contoso.sharepoint.com/sites/project-x",
        )],
        links: &[],
    },
    Help {
        name: "spo user remove",
        summary: "Removes user from specific web",
        options: &[
            ("webUrl", "URL of the web to remove user."),
            ("id", "ID of the user to remove from web."),
            ("loginName", "Login name of the user to remove from web."),
            ("userName", "User name of the user to remove from web."),
            ("email", "Email of the user to remove from web."),
            ("entraGroupId", "Object ID of the Entra group ID to remove."),
            ("entraGroupName", "Name of the Entra group to remove."),
            (
                "force",
                "Do not prompt for confirmation before removing user from web",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes user by id from a web without prompting for confirmation",
                "m365 spo user remove --webUrl \"https://contoso.sharepoint.com/sites/HR\" --id 10 --force",
            ),
            (
                "Removes user by loginName from a web",
                "m365 spo user remove --webUrl \"https://contoso.sharepoint.com/sites/HR\" --loginName \"i:0#.f|membership|john.doe@mytenant.onmicrosoft.com\"",
            ),
            (
                "Removes user by userName from a web",
                "m365 spo user remove --webUrl \"https://contoso.sharepoint.com/sites/HR\" --userName \"john.doe_hotmail.com#ext#@mytenant.onmicrosoft.com\"",
            ),
            (
                "Removes user by email from a web",
                "m365 spo user remove --webUrl \"https://contoso.sharepoint.com/sites/HR\" --email \"john.doe@mytenant.onmicrosoft.com\"",
            ),
            (
                "Removes user by entraGroupId from a web",
                "m365 spo user remove --webUrl \"https://contoso.sharepoint.com/sites/HR\" --entraGroupId f832a493-de73-4fef-87ed-8c6fffd91be6",
            ),
            (
                "Removes user by entraGroupName from a web",
                "m365 spo user remove --webUrl _https://contoso.sharepoint.com/sites/HR_ --entraGroupName \"Test Members\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/powershell/module/sharepoint-pnp/remove-pnpuser?view=sharepoint-ps",
            "https://learn.microsoft.com/powershell/module/sharepoint-pnp/remove-pnpuser?view=sharepoint-ps",
        )],
    },
    Help {
        name: "spo userprofile get",
        summary: "Gets SharePoint user profile properties for the specified user",
        options: &[("userName", "Account name of the user")],
        notes: &[
            "You have to have tenant admin permissions in order to use this command to get profile properties of other users.",
        ],
        permissions: &[],
        examples: &[(
            "Get SharePoint user profile for the specified user",
            "m365 spo userprofile get --userName 'john.doe@mytenant.onmicrosoft.com'",
        )],
        links: &[],
    },
    Help {
        name: "spo userprofile set",
        summary: "Sets user profile property for a SharePoint user",
        options: &[
            ("userName", "Account name of the user"),
            ("propertyName", "The name of the property to be set"),
            ("propertyValue", "The value of the property to be set"),
        ],
        notes: &[
            "You have to have tenant admin permissions in order to use this command to update profile properties of other users.",
        ],
        permissions: &[],
        examples: &[
            (
                "Updates the single-value AboutMe property",
                "m365 spo userprofile set --userName 'john.doe@mytenant.onmicrosoft.com' --propertyName 'AboutMe' --propertyValue 'Working as a Microsoft 365 developer'",
            ),
            (
                "Updates the multi-value SPS-Skills property",
                "m365 spo userprofile set --userName 'john.doe@mytenant.onmicrosoft.com' --propertyName 'SPS-Skills' --propertyValue 'CSS, HTML'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web add",
        summary: "Creates new subsite",
        options: &[
            ("title", "Subsite title"),
            ("description", "Subsite description"),
            ("url", "Subsite relative url"),
            (
                "webTemplate",
                "Subsite template, eg. `STS#0` (Classic team site)",
            ),
            (
                "parentWebUrl",
                "URL of the parent site under which to create the subsite",
            ),
            (
                "locale",
                "Subsite locale LCID, eg. `1033` for en-US. Default `1033`",
            ),
            (
                "breakInheritance",
                "Set to not inherit permissions from the parent site",
            ),
            (
                "inheritNavigation",
                "Set to inherit the navigation from the parent site",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Create subsite using the Team site template in the en-US locale",
                "m365 spo web add --title Subsite --description Subsite --url subsite --webTemplate STS#0 --parentWebUrl https://contoso.sharepoint.com --locale 1033",
            ),
            (
                "Create subsite with unique permissions using the default en-US locale",
                "m365 spo web add --title Subsite --url subsite --webTemplate STS#0 --parentWebUrl https://contoso.sharepoint.com --breakInheritance",
            ),
            (
                "Create subsite with the same navigation as the parent site",
                "m365 spo web add --title Subsite --url subsite --webTemplate STS#0 --parentWebUrl https://contoso.sharepoint.com --inheritNavigation",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web clientsidewebpart list",
        summary: "Lists available client-side web parts",
        options: &[(
            "webUrl",
            "URL of the site for which to retrieve the information",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Lists all the available client-side web parts for the specified site",
            "m365 spo web clientsidewebpart list --webUrl https://contoso.sharepoint.com",
        )],
        links: &[],
    },
    Help {
        name: "spo web get",
        summary: "Retrieves information about the specified site",
        options: &[
            (
                "url",
                "URL of the site for which to retrieve the information",
            ),
            (
                "withGroups",
                "Set if you want to return associated groups (associatedOwnerGroup, associatedMemberGroup and associatedVisitorGroup) along with other properties",
            ),
            (
                "withPermissions",
                "Set if you want to return associated roles and permissions",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieve information about a site",
                "m365 spo web get --url https://contoso.sharepoint.com/subsite",
            ),
            (
                "Retrieve information about a site along with associated groups for the web",
                "m365 spo web get --url https://contoso.sharepoint.com/subsite --withGroups",
            ),
            (
                "Retrieve information about a site along with the RoleAssignments for the web",
                "m365 spo web get --url https://contoso.sharepoint.com/subsite --withPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web installedlanguage list",
        summary: "Lists all installed languages on site",
        options: &[(
            "webUrl",
            "URL of the site for which to retrieve the list of installed languages",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Return all installed languages from site https://contoso.sharepoint.com/",
            "m365 spo web installedlanguage list --webUrl https://contoso.sharepoint.com",
        )],
        links: &[],
    },
    Help {
        name: "spo web list",
        summary: "Lists subsites of the specified site",
        options: &[(
            "url",
            "URL of the parent site for which to retrieve the list of subsites",
        )],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Return all subsites from site https://contoso.sharepoint.com/",
            "m365 spo web list --url https://contoso.sharepoint.com",
        )],
        links: &[],
    },
    Help {
        name: "spo web reindex",
        summary: "Requests reindexing the specified subsite",
        options: &[("url", "URL of the subsite to reindex")],
        notes: &[
            "If the subsite to be reindexed is a no-script site, the command will request reindexing all lists from the subsite that haven't been excluded from the search index.",
        ],
        permissions: &[],
        examples: &[(
            "Request reindexing the subsite https://contoso.sharepoint.com/subsite",
            "m365 spo web reindex --url https://contoso.sharepoint.com/subsite",
        )],
        links: &[],
    },
    Help {
        name: "spo web remove",
        summary: "Deletes specified subsite",
        options: &[
            ("url", "URL of the subsite to remove"),
            (
                "force",
                "Do not prompt for confirmation before deleting the subsite",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Delete subsite without prompting for confirmation",
            "m365 spo web remove --url https://contoso.sharepoint.com/subsite --force",
        )],
        links: &[],
    },
    Help {
        name: "spo web retentionlabel list",
        summary: "Gets a list of retention labels that are available on a site",
        options: &[("webUrl", "URL of the site.")],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get a list of retention labels for the Sales site",
            "m365 spo web retentionlabel list --webUrl 'https://contoso.sharepoint.com/sites/sales'",
        )],
        links: &[],
    },
    Help {
        name: "spo web roleassignment add",
        summary: "Adds a role assignment to web permissions.",
        options: &[
            ("webUrl", "URL of the site."),
            (
                "principalId",
                "SharePoint ID of principal it may be either user id or group id we want to add permissions to.",
            ),
            ("upn", "The upn/email of user to assign role to."),
            (
                "groupName",
                "The group name of the SharePoint group. Use this option exclusively for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to add."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to add.",
            ),
            ("roleDefinitionId", "ID of role definition."),
            (
                "roleDefinitionName",
                "The name of a role definition, like 'Contribute', 'Read', etc.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "add role assignment to a site principal id 11 and role definition id 1073741829",
                "m365 spo web roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --principalId 11 --roleDefinitionId 1073741829",
            ),
            (
                "add role assignment to a site for a upn and role definition id 1073741829",
                "m365 spo web roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --upn \"someaccount@tenant.onmicrosoft.com\" --roleDefinitionId 1073741829",
            ),
            (
                "add role assignment to a site for group someGroup and role definition id 1073741829",
                "m365 spo web roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --groupName \"someGroup\" --roleDefinitionId 1073741829",
            ),
            (
                "add role assignment to a site for principal id 11 and role definition name Full Control",
                "m365 spo web roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --principalId 11 --roleDefinitionName \"Full Control\"",
            ),
            (
                "add role assignment using a Entra Group ID and a role definition",
                "m365 spo web roleassignment add --webUrl \"https://contoso.sharepoint.com/sites/project-x\" --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\" --roleDefinitionId 1073741829",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web roleassignment remove",
        summary: "Removes a role assignment from web permissions.",
        options: &[
            ("webUrl", "URL of the site."),
            (
                "principalId",
                "SharePoint ID of principal it may be either user id or group id we want to add permissions to.",
            ),
            ("upn", "The upn/email of user to assign role to."),
            (
                "groupName",
                "The group name of the SharePoint group. Use this option exclusively for SharePoint Online groups.",
            ),
            ("entraGroupId", "ID of the Microsoft Entra group to remove."),
            (
                "entraGroupName",
                "Display name of the Microsoft Entra group to remove.",
            ),
            (
                "force",
                "Don't prompt for confirming removing the roleassignment.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Remove roleassignment from web based on group name",
                "m365 spo web roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"  --groupName \"saleGroup\"",
            ),
            (
                "Remove roleassignment from web based on principal Id",
                "m365 spo web roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"  --principalId 2",
            ),
            (
                "Remove roleassignment from web based on upn",
                "m365 spo web roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"  --upn \"someaccount@tenant.onmicrosoft.com\"",
            ),
            (
                "Remove roleassignment from web based on principal Id without prompting for confirmation",
                "m365 spo web roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"  --principalId 2 --force",
            ),
            (
                "Remove roleassignment from web based on Entra Group Id and don't prompt for confirmation",
                "m365 spo web roleassignment remove --webUrl \"https://contoso.sharepoint.com/sites/contoso-sales\"  --entraGroupId \"27ae47f1-48f1-46f3-980b-d3c1470e398d\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web roleinheritance break",
        summary: "Breaks role inheritance of subsite",
        options: &[
            ("webUrl", "URL of the site."),
            (
                "clearExistingPermissions",
                "Clears all existing permissions from the web.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[
            "By default, when breaking permissions inheritance, the web will retain existing permissions. To remove existing permissions, use the `--clearExistingPermissions` option.",
        ],
        permissions: &[],
        examples: &[
            (
                "Break role inheritance of a web and keep the existing permissions",
                "m365 spo web roleinheritance break --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Break role inheritance of a web and clear the existing permissions",
                "m365 spo web roleinheritance break --webUrl https://contoso.sharepoint.com/sites/project-x --clearExistingPermissions",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web roleinheritance reset",
        summary: "Restores role inheritance of subsite.",
        options: &[
            ("webUrl", "URL of the site"),
            (
                "force",
                "Do not prompt for confirmation before resetting role inheritance.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Restore role inheritance of subsite https://contoso.sharepoint.com/sites/project-x",
                "m365 spo web roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x",
            ),
            (
                "Restore role inheritance of subsite https://contoso.sharepoint.com/sites/project-x without prompting for confirmation",
                "m365 spo web roleinheritance reset --webUrl https://contoso.sharepoint.com/sites/project-x --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web rule get",
        summary: "Retrieves information about a specific alert from a SharePoint list",
        options: &[
            ("webUrl", "The URL of the SharePoint site."),
            ("id", "The ID of the rule."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Delegated", "Microsoft Graph", "User.ReadBasic.All"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
            ("Application", "Microsoft Graph", "User.ReadBasic.All"),
        ],
        examples: &[(
            "Retrieve a rule by ID",
            "m365 spo web rule get --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7cbb4c8d-8e4d-4d2e-9c6f-3f1d8b2e6a0e",
        )],
        links: &[],
    },
    Help {
        name: "spo web rule list",
        summary: "Lists all SharePoint list alerts",
        options: &[
            ("webUrl", "The URL of the SharePoint site."),
            ("listId", "The ID of the SharePoint list."),
            ("listUrl", "The URL of the SharePoint list."),
            ("listTitle", "The title of the SharePoint list."),
            ("userName", "The user name to filter alerts for."),
            ("userId", "The user ID to filter alerts for."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Delegated", "Microsoft Graph", "User.ReadBasic.All"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
            ("Application", "Microsoft Graph", "User.ReadBasic.All"),
        ],
        examples: &[
            (
                "List all alerts for a SharePoint site:",
                "m365 spo web rule list --webUrl https://contoso.sharepoint.com/sites/Marketing",
            ),
            (
                "List alerts for a SharePoint list by listUrl:",
                "m365 spo web rule list --webUrl https://contoso.sharepoint.com/sites/Marketing --listUrl /sites/Marketing/lists/Tasks",
            ),
            (
                "List alerts for a specific user by userId:",
                "m365 spo web rule list --webUrl https://contoso.sharepoint.com/sites/Marketing --userId 7cbb4c8d-8e4d-4d2e-9c6f-3f1d8b2e6a0e",
            ),
            (
                "List alerts for a specific list by title and user by username:",
                "m365 spo web rule list --webUrl https://contoso.sharepoint.com/sites/Marketing --listTitle \"Tasks\" --userName jane.doe@contoso.com",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web rule remove",
        summary: "Removes an alert from a SharePoint list",
        options: &[
            ("webUrl", "The URL of the SharePoint site."),
            ("id", "The ID of the alert."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "SharePoint", "AllSites.FullControl"),
            ("Application", "SharePoint", "Sites.FullControl.All"),
        ],
        examples: &[
            (
                "Remove an alert by ID",
                "m365 spo web rule remove --webUrl https://contoso.sharepoint.com/sites/Marketing --id 7cbb4c8d-8e4d-4d2e-9c6f-3f1d8b2e6a0e",
            ),
            (
                "Remove another alert without confirmation",
                "m365 spo web rule remove --webUrl https://contoso.sharepoint.com/sites/Marketing --id 2b6f1c8a-3e6a-4c7e-b8c0-7bf4c8e6d7f1 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spo web set",
        summary: "Updates subsite properties",
        options: &[
            ("url", "URL of the subsite to update"),
            ("title", "New title for the subsite"),
            ("description", "New description for the subsite"),
            (
                "siteLogoUrl",
                "New site logo URL for the subsite. Set to empty string to reset to default.",
            ),
            (
                "quickLaunchEnabled",
                "Set to `true` to enable quick launch and to `false` to disable it",
            ),
            (
                "headerLayout",
                "Configures the site header. Allowed values `standard,compact`",
            ),
            (
                "headerEmphasis",
                "Configures the site header background. Allowed values `0`, `1`, `2`, `3`",
            ),
            (
                "megaMenuEnabled",
                "Set to `true` to change the menu style to megamenu. Set to `false` to use the cascading menu style",
            ),
            (
                "footerEnabled",
                "Set to `true` to enable footer and to `false` to disable it",
            ),
            (
                "navAudienceTargetingEnabled",
                "Enable or disable site navigation audience targeting. Allowed values: `true` or `false`.",
            ),
            (
                "searchScope",
                "Search scope to set in the site. Allowed values `DefaultScope`, `Tenant`, `Hub`, `Site`.",
            ),
            (
                "welcomePage",
                "Site-relative URL of the welcome page for the site",
            ),
        ],
        notes: &[
            "Next to updating web properties corresponding to the options of this command, you can update the value of any other web property using its CSOM name, eg. `--AllowAutomaticASPXPageIndexing`. At this moment, the CLI supports properties of types `Boolean`, `String` and `Int32`.",
        ],
        permissions: &[],
        examples: &[
            (
                "Update subsite title",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --title Team-a",
            ),
            (
                "Hide quick launch on the subsite",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --quickLaunchEnabled false",
            ),
            (
                "Set site header layout to compact",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --headerLayout compact",
            ),
            (
                "Set site header color to primary theme background color",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --headerEmphasis 0",
            ),
            (
                "Enable megamenu in the site",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --megaMenuEnabled true",
            ),
            (
                "Hide footer in the site",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --footerEnabled false",
            ),
            (
                "Enable navigation audience targetting in the site",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --navAudienceTargetingEnabled true",
            ),
            (
                "Set search scope to tenant scope",
                "m365 spo web set --url https://contoso.sharepoint.com/sites/team-a --searchScope tenant",
            ),
            (
                "Set welcome page for the web",
                "m365 spo web set  --url https://contoso.sharepoint.com/sites/team-a --welcomePage \"SitePages/new-home.aspx\"",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/previous-versions/office/sharepoint-server/ee545886(v=office.15)",
            "https://learn.microsoft.com/previous-versions/office/sharepoint-server/ee545886(v=office.15",
        )],
    },
];

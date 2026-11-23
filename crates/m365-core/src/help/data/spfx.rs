//! Help for the `m365 spfx ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "spfx doctor",
        summary: "Verifies environment configuration for using the specific version of the SharePoint Framework",
        options: &[
            (
                "env",
                "Version of SharePoint for which to check compatibility: `sp2016|sp2019|spo`",
            ),
            (
                "spfxVersion",
                "Version of the SharePoint Framework Yeoman generator to check compatibility for without `v`, eg. `1.11.0`",
            ),
            (
                "query",
                "JMESPath query string. See http://jmespath.org/ for more information and examples",
            ),
            ("output", "Output type. `text` or `json`. Default `text`"),
            ("verbose", "Runs command with verbose logging"),
            ("debug", "Runs command with debug logging"),
        ],
        notes: &[
            "This commands helps you to verify if your environment meets all prerequisites for building solutions using a particular version of the SharePoint Framework.",
            "The command starts by detecting the version of SharePoint Framework that you want to use. First, it looks at the current project. If you didn't run the command in the context of a SharePoint Framework project, the command will try to determine the SharePoint Framework version based on the SharePoint Framework Yeoman generator that you have installed either in the current directory or globally.",
            "Based on the determined version of the SharePoint Framework, the command will look at other dependencies such as Node.js, npm, Yeoman, Gulp CLI and TypeScript to verify if their meet the requirements of that particular version of the SharePoint Framework.",
            "If you miss any required tools or use a version that doesn't meet the SharePoint Framework requirements, the command will give you a list of recommendation how to address these issues.",
            "Next to verifying the readiness of your environment to use a particular version of the SharePoint Framework, you can also check if the version of the SharePoint Framework that you use is compatible with the specific version of SharePoint. Supported versions are `sp2016`, `sp2019` and `spo`.",
            "Checks ran by this command are based on what is officially supported by Microsoft. It's possible that using different package managers or packages versions will work just fine.",
            "This command supports only text or json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Verify if your environment meets the requirements to work with SharePoint Framework based on the globally installed version of the SharePoint Framework Yeoman generator or the current project",
                "m365 spfx doctor --output text",
            ),
            (
                "Verify if your environment meets the requirements to work with the SharePoint Framework and also if the version of the SharePoint Framework that you're using is compatible with SharePoint 2019",
                "m365 spfx doctor --env sp2019 --output text",
            ),
            (
                "Verify if your environment meets the requirements to work with SharePoint Framework v1.11.0",
                "m365 spfx doctor --spfxVersion 1.11.0 --output text",
            ),
            (
                "Verify if your environment meets the requirements to work with SharePoint Framework v1.11.0 and outputs it in a JSON format.",
                "m365 spfx doctor --spfxVersion 1.11.0 --output json",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spfx package generate",
        summary: "Generates SharePoint Framework solution package with a no-framework web part rendering the specified HTML snippet",
        options: &[
            (
                "webPartTitle",
                "Title of the web part to generate. Displayed in the tool box when adding web part to page",
            ),
            (
                "webPartDescription",
                "Description of the web part to generate. Displayed in the tool box when adding web part to page",
            ),
            (
                "name",
                "Name of the package to generate. Used among others for the .sppkg file. Must be unique in the app catalog to avoid collisions with other solutions.",
            ),
            (
                "html",
                "HTML snippet to embed in the web part. Can contain `` and `` tags.",
            ),
            (
                "enableForTeams",
                "Specify, to make the generated web part available in Microsoft Teams. Specify `tab` to make the web part available as a configurable tab, `personalTab` to make it available as a personal tab or `all` to make it available both as a configurable and personal tab. By default the web part will not be available in Microsoft Teams.",
            ),
            (
                "exposePageContextGlobally",
                "Set, to make the `legacyPageContext` exposed by SharePoint Framework available at `window._spPageContextInfo` for use in the HTML snippet of the web part",
            ),
            (
                "exposeTeamsContextGlobally",
                "Set, to make the Microsoft Teams context exposed by SharePoint Framework available at `window._teamsContextInfo` for use in the HTML snippet of the web part",
            ),
            (
                "allowTenantWideDeployment",
                "Set, to allow the solution package to be deployed globally to all sites",
            ),
            (
                "developerName",
                "Name of your organization. Displayed in Microsoft Teams when adding the solution as a (personal) tab. If not specified set to `Contoso`.",
            ),
            (
                "developerPrivacyUrl",
                "URL of the privacy policy for this solution. Displayed in Microsoft Teams when adding the solution as a (personal) tab. If not specified, set to `https://contoso.com/privacy`.",
            ),
            (
                "developerTermsOfUseUrl",
                "URL of the terms of use for this solution. Displayed in Microsoft Teams when adding the solution as a (personal) tab. If not specified, set to `https://contoso.com/terms-of-use`.",
            ),
            (
                "developerWebsiteUrl",
                "URL of your organization's website. Displayed in Microsoft Teams when adding the solution as a (personal) tab. If not specified, set to `https://contoso.com/my-app`.",
            ),
            (
                "developerMpnId",
                "Microsoft Partner Network ID of your organization. If not specified, set to `000000`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Generate a web part that shows the weather for Amsterdam. Load web part contents from a local file. Allow the web part to be deployed to all sites. Expose the web part in Teams as a personal tab.",
            "m365 spfx package generate --webPartTitle \"Amsterdam weather\" --webPartDescription \"Shows weather in Amsterdam\" --name amsterdam-weather --html @amsterdam-weather.html --allowTenantWideDeployment --enableForTeams all",
        )],
        links: &[],
    },
    Help {
        name: "spfx project azuredevops pipeline add",
        summary: "Adds a Azure DevOps Pipeline for a SharePoint Framework project",
        options: &[
            (
                "name",
                "Name of the pipeline run that will be created. If none is specified a default name generated by Azure DevOps will be used",
            ),
            (
                "branchName",
                "Specify the branch name which should trigger the workflow on push. If none is specified a default will be used which is 'main'",
            ),
            (
                "loginMethod",
                "Specify the login method used for the login action. Possible options are: `user`, `application`. Default `application`",
            ),
            (
                "scope",
                "Scope of the app catalog: `tenant`, `sitecollection`. Default is `tenant`",
            ),
            (
                "siteUrl",
                "The URL of the site collection where the solution package will be added. Required if scope is set to `sitecollection`",
            ),
            (
                "skipFeatureDeployment",
                "When specified the app will be added to all sites. When deployed to the site app catalog it will be added to all sub-sites of that site.",
            ),
        ],
        notes: &[
            "The `spfx project azuredevops pipeline add` will create a workflow .yml file in the `.azuredevops/pipelines` directory in your project. If such directory does not exist the command will automatically create it.",
            "The command will not create the Azure DevOps pipeline. You will need to manually create it in Azure DevOps. The command will only create the workflow file which you can then push to your repo and create a new yaml pipline based on it.",
            "For the `application` login method the command does not register an Entra application nor does it create the required certificate. In order to proceed, obtain (create) a self-signed certificate and register a new Entra application with certificate authentication. After that you will need to fill the following variables, or what is more preferable, create a dedicated variable group to store those properties:",
            "- `CertificateBase64Encoded` - base 64 encoded certificate. Use either `CertificateBase64Encoded` or `CertificateSecureFileId` but not both",
            "- `CertificateSecureFileId` - id of a certificate file in the secure files section of the DevOps library. `.pfx` file. Use either `CertificateBase64Encoded` or `CertificateSecureFileId` but not both",
            "- `CertificatePassword` - certificate password. This applies only if the certificate is encoded which is the recommended approach",
            "- `EntraAppId` - client id of the registered Entra application",
            "This use case is perfect in a production context as it does not create any dependencies on an account",
            "For the `user` login method you will need to fill the following variables, or what is more preferable, create a dedicated variable group to store those properties:",
            "- `UserName` - user email",
            "- `Password` - password",
            "This method is perfect to test your workflow, in a dev context, for personal usage. It will not work for accounts with MFA.",
            "Run this command in the SPFx solution folder.",
        ],
        permissions: &[],
        examples: &[
            (
                "Adds an Azure DevOps Pipeline for a SharePoint Framework project triggered on push to main",
                "m365 spfx project azuredevops pipeline add",
            ),
            (
                "Adds an Azure DevOps Pipeline for a SharePoint Framework project with `user` login method triggered on push to main",
                "m365 spfx project azuredevops pipeline add --loginMethod \"user\"",
            ),
            (
                "Adds an Azure DevOps Pipeline for a SharePoint Framework project with deployment to a site collection app catalog",
                "m365 spfx project azuredevops pipeline add --scope \"sitecollection\" --siteUrl \"https://some.sharepoint.com/sites/someSite\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spfx project doctor",
        summary: "Validates correctness of a SharePoint Framework project",
        options: &[
            (
                "packageManager",
                "The package manager you use. Supported managers `npm`, `pnpm`, `yarn`. Default `npm`",
            ),
            (
                "query",
                "JMESPath query string. See http://jmespath.org/ for more information and examples",
            ),
            (
                "output",
                "Output type. `json`, `text`, `tour`, `csv`. Default `json`",
            ),
            ("verbose", "Runs command with verbose logging"),
            ("debug", "Runs command with debug logging"),
        ],
        notes: &[
            "The `spfx project doctor` command helps you validate that your SharePoint Framework project is set up correctly. The command automatically detects the version of your project using version information specified in the project's .yo-rc.json file or package.json (if no version information is included in .yo-rc.json). Based on the detected project version, the command executes several checks and reports any issues in the specified format.",
            "This command doesn't change your project files. Instead, it gives you a report with all steps necessary to validate your project to the specified version of the SharePoint Framework. Changing project files is error-prone, especially when it comes to updating your solution's code. This is why at this moment, this command produces a report that you can use yourself to perform the necessary updates and verify that everything is working as expected.",
            "Run this command in the folder where the project that you want to validate is located. This command doesn't change your project files.",
        ],
        permissions: &[],
        examples: &[
            (
                "Validate if your project is correctly set up and save the findings in a Markdown file",
                "m365 spfx project doctor --output md > \"doctor-report.md\"",
            ),
            (
                "Validate if your project is correctly set up and show the summary of the findings in the terminal",
                "m365 spfx project doctor --output text",
            ),
            (
                "Validate if your project is correctly set up and get instructions to fix any issues using pnpm",
                "m365 spfx project doctor --packageManager pnpm --output text",
            ),
            (
                "Validate if your project is correctly set up and get instructions to fix any issues in a CodeTour file",
                "m365 spfx project doctor --output tour",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spfx project externalize",
        summary: "Externalizes SharePoint Framework project dependencies",
        options: &[
            (
                "query",
                "JMESPath query string. See http://jmespath.org/ for more information and examples",
            ),
            (
                "output",
                "Output type. `json`, `text`, `csv`, `md`. Default `json`",
            ),
            ("verbose", "Runs command with verbose logging"),
            ("debug", "Runs command with debug logging"),
        ],
        notes: &[
            "Run this command in the folder where the project for which you want to externalize dependencies is located. This command doesn't change your project files.",
            "This command is in preview and could change once it's officially released. If you see any room for improvement, we'd love to hear from you at https://github.com/pnp/cli-microsoft365/issues.",
            "The `spfx project externalize` command helps you externalize your SharePoint Framework project dependencies using the unpkg CDN.",
            "This command doesn't change your project files. Instead, it gives you a report with all steps necessary to externalize your project dependencies. Externalizing project dependencies is error-prone, especially when it comes to updating your solution's code. This is why at this moment, this command produces a report that you can use yourself to perform the necessary changes and verify that everything is working as expected.",
            "Supported SharePoint Framework versions are 1.0.0, 1.0.1, 1.0.2, 1.1.0, 1.1.1, 1.1.3, 1.2.0, 1.3.0, 1.3.1, 1.3.2, 1.3.4, 1.4.0, 1.4.1, 1.5.0, 1.5.1, 1.6.0, 1.7.0, 1.7.1, 1.8.0, 1.8.1, 1.8.2, 1.9.1.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get instructions to externalize the current SharePoint Framework project dependencies and save the findings in a Markdown file",
                "m365 spfx project externalize --output md > \"deps-report.md\"",
            ),
            (
                "Get instructions to externalize the current SharePoint Framework project dependencies and show the summary of the findings in the terminal",
                "m365 spfx project externalize",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spfx project github workflow add",
        summary: "Adds a GitHub workflow for a SharePoint Framework project",
        options: &[
            (
                "name",
                "Name of the workflow that will be created. If none is specified a default name will be used 'Deploy Solution ${name of sppkg file}'.",
            ),
            (
                "branchName",
                "Specify the branch name which should trigger the workflow on push. If none is specified a default will be used which is 'main'.",
            ),
            (
                "manuallyTrigger",
                "When specified a manual trigger option will be added to the workflow: `workflow_dispatch`.",
            ),
            (
                "loginMethod",
                "Specify the login method used for the login action. Possible options are: `user`, `application`. Default `application`.",
            ),
            (
                "scope",
                "Scope of the app catalog: `tenant`, `sitecollection`. Default is `tenant`.",
            ),
            (
                "siteUrl",
                "The URL of the site collection where the solution package will be added. Required if scope is set to `sitecollection`.",
            ),
            (
                "skipFeatureDeployment",
                "When specified and the app supports tenant-wide deployment, deploy it to the whole tenant.",
            ),
        ],
        notes: &[
            "The `spfx project github workflow add` will create a workflow .yml file in the `.github/workflows` directory in your project. If such directory does not exist the command will automatically create it.",
            "For the `application` login method the command does not register a Microsoft Entra application nor create the required certificate. In order for you to proceed you will need to first obtain (create) a self-signed certificate and register a new Microsoft Entra application with certificate authentication. After that in GitHub repo settings, you will need to create the following secrets:",
            "- `APP_ID` - client id of the registered Microsoft Entra application",
            "- `CERTIFICATE_ENCODED` - application's encoded certificate",
            "- `CERTIFICATE_PASSWORD` - certificate password. This applies only if the certificate is encoded which is the recommended approach",
            "This use case is perfect in a production context as it does not create any dependencies on an account",
            "For the `user` login method you will need to create the following secrets in GitHub repo settings:",
            "- `ADMIN_USERNAME` - username",
            "- `ADMIN_PASSWORD` - password",
            "This method is perfect to test your workflow, in a dev context, for personal usage. It will not work for accounts with MFA.",
            "The workflow will overwrite the existing .sppkg if it is already deployed in the app catalog. Overwriting your sppkg file on every deployment is required to make continuous delivery of the latest version of your app which is the aim of the continuous delivery pipeline.",
            "Run this command in the SPFx solution folder.",
        ],
        permissions: &[],
        examples: &[
            (
                "Adds a GitHub workflow for a SharePoint Framework project with `application` login method triggered on push to main.",
                "m365 spfx project github workflow add",
            ),
            (
                "Adds a GitHub workflow for a SharePoint Framework project with `user` login method triggered on push to main and when manually triggered.",
                "m365 spfx project github workflow add --manuallyTrigger --loginMethod \"user\"",
            ),
            (
                "Adds a GitHub workflow for a SharePoint Framework project with deployment to a site collection app catalog.",
                "m365 spfx project github workflow add --scope \"sitecollection\" --siteUrl \"https://some.sharepoint.com/sites/someSite\"",
            ),
        ],
        links: &[(
            "https://pnp.github.io/cli-microsoft365/user-guide/github-actions",
            "https://pnp.github.io/cli-microsoft365/user-guide/github-actions",
        )],
    },
    Help {
        name: "spfx project permissions grant",
        summary: "Grants API permissions defined in the current SPFx project",
        options: &[],
        notes: &[
            "Run this command in the folder where the project is located from where you want to grant the permissions.",
            "This command grant the permissions defined in: package-solution.json.",
        ],
        permissions: &[],
        examples: &[(
            "Grant API permissions requested in the current SPFx project",
            "m365 spfx project permissions grant",
        )],
        links: &[],
    },
    Help {
        name: "spfx project rename",
        summary: "Renames SharePoint Framework project",
        options: &[
            ("newName", "New name for the project"),
            (
                "generateNewId",
                "Generate a new solution ID for the project",
            ),
        ],
        notes: &[
            "Run this command in the folder where the project that you want to rename is located.",
            "This command will update the project name in: package.json, .yo-rc.json, package-solution.json, deploy-azure-storage.json and README.md.",
        ],
        permissions: &[],
        examples: &[
            (
                "Renames SharePoint Framework project to contoso",
                "m365 spfx project rename --newName contoso",
            ),
            (
                "Renames SharePoint Framework project to contoso with new solution ID",
                "m365 spfx project rename --newName contoso --generateNewId",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spfx project upgrade",
        summary: "Upgrades SharePoint Framework project to the specified version",
        options: &[
            (
                "toVersion",
                "The version of SharePoint Framework to which upgrade the project",
            ),
            (
                "packageManager",
                "The package manager you use. Supported managers `npm` or `pnpm`. Default `npm`",
            ),
            (
                "shell",
                "The shell you use. Supported shells `bash`, `powershell`, `cmd`. Default `powershell`",
            ),
            (
                "preview",
                "Upgrade project to the latest SPFx preview version",
            ),
            (
                "query",
                "JMESPath query string. See http://jmespath.org/ for more information and examples.",
            ),
            (
                "output",
                "Output type. `json`, `text`, `csv`, `md`, `tour`, `none`. Default `json`.",
            ),
            ("verbose", "Runs command with verbose logging."),
            ("debug", "Runs command with debug logging."),
        ],
        notes: &[
            "The `spfx project upgrade` command helps you upgrade your SharePoint Framework project to the specified version. If no version is specified, the command will upgrade to the latest version of the SharePoint Framework it supports (v1.24.1).",
            "This command doesn't change your project files. Instead, it gives you a report with all steps necessary to upgrade your project to the specified version of the SharePoint Framework. Changing project files is error-prone, especially when it comes to updating your solution's code. This is why at this moment, this command produces a report that you can use yourself to perform the necessary updates and verify that everything is working as expected.",
            "Run this command in the folder where the project that you want to upgrade is located. This command doesn't change your project files.",
        ],
        permissions: &[],
        examples: &[
            (
                "Get instructions to upgrade the current SharePoint Framework project to SharePoint Framework version 1.5.0 and save the findings in a Markdown file",
                "m365 spfx project upgrade --toVersion 1.5.0 --output md > \"upgrade-report.md\"",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to SharePoint Framework version 1.5.0 and show the summary of the findings in the shell",
                "m365 spfx project upgrade --toVersion 1.5.0 --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the latest preview version",
                "m365 spfx project upgrade --preview --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the specified preview version",
                "m365 spfx project upgrade --toVersion 1.12.1-rc.0 --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the latest SharePoint Framework version supported by the CLI for Microsoft 365 using pnpm",
                "m365 spfx project upgrade --packageManager pnpm --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the latest SharePoint Framework version supported by the CLI for Microsoft 365",
                "m365 spfx project upgrade --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the latest SharePoint Framework version supported by the CLI for Microsoft 365 using PowerShell",
                "m365 spfx project upgrade --output text",
            ),
            (
                "Get instructions to upgrade the current SharePoint Framework project to the latest version of SharePoint Framework and save the findings in a CodeTour file",
                "m365 spfx project upgrade --output tour",
            ),
        ],
        links: &[],
    },
];

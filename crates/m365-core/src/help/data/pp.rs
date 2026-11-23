//! Help for the `m365 pp ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "pp aibuildermodel get",
        summary: "Gets a specific AI builder model in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the AI Builder model."),
            ("name", "The name of the AI Builder model."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Get an AI Builder model in a specific environment.",
                "m365 pp aibuildermodel get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --name \"Document Processing 11/29/2022, 12:58:43 PM\"",
            ),
            (
                "Get an AI Builder models in a specific environment by id as admin.",
                "m365 pp aibuildermodel get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --id \"08ffffbe-ec1c-4e64-b64b-dd1db926c613\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp aibuildermodel list",
        summary: "Lists available AI builder models in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all AI Builder models in a specific environment.",
                "m365 pp aibuildermodel list --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\"",
            ),
            (
                "List all AI Builder models in a specific environment as admin.",
                "m365 pp aibuildermodel list --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp aibuildermodel remove",
        summary: "Removes the specified Microsoft Power Platform aibuildermodel in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the AI builder model."),
            ("name", "The name of the AI builder model."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the AI builder model owned by the currently signed-in user based on the name parameter",
                "m365 pp aibuildermodel remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"AI Builder Model Name\"",
            ),
            (
                "Removes the AI builder model owned by the currently signed-in user based on the name parameter without confirmation",
                "m365 pp aibuildermodel remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"AI Builder Model Name\" --force",
            ),
            (
                "Removes the AI builder model owned by another user based on the id parameter",
                "m365 pp aibuildermodel remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 9d9a13d0-6255-ed11-bba2-000d3adf774e  --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp copilot get",
        summary: "Gets information about the specified copilot",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the copilot."),
            ("name", "The name of the copilot."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Get a specific copilot in a specific environment based on name.",
                "m365 pp copilot get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --name \"CLI 365 Copilot\"",
            ),
            (
                "Get a specific copilot in a specific environment based on name as admin.",
                "m365 pp copilot get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --name \"CLI 365 Copilot\" --asAdmin",
            ),
            (
                "Get a specific copilot in a specific environment based on id.",
                "m365 pp copilot get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --id \"3a081d91-5ea8-40a7-8ac9-abbaa3fcb893\"",
            ),
            (
                "Get a specific copilot in a specific environment based on id as admin.",
                "m365 pp copilot get --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --id \"3a081d91-5ea8-40a7-8ac9-abbaa3fcb893\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp copilot list",
        summary: "Lists Microsoft Power Platform copilots in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List copilots in a specific environment.",
                "m365 pp copilot list --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\"",
            ),
            (
                "List copilots in a specific environment as admin.",
                "m365 pp copilot list --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp copilot remove",
        summary: "Removes the specified copilot",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the copilot."),
            ("name", "The name of the copilot."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the specified Microsoft Power Platform copilot owned by the currently signed-in user based on name",
                "m365 pp copilot remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Chatbot Name\"",
            ),
            (
                "Removes the specified Microsoft Power Platform copilot owned by the currently signed-in user based on id without confirmation",
                "m365 pp copilot remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 9d9a13d0-6255-ed11-bba2-000d3adf774e --force",
            ),
            (
                "Removes the specified Microsoft Power Platform copilot owned by another user based on name",
                "m365 pp copilot remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Chatbot Name\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp dataverse table get",
        summary: "Gets a dataverse table in a given environment",
        options: &[
            (
                "environmentName",
                "The name of the environment to list a table for.",
            ),
            (
                "name",
                "The name of the dataverse table to retrieve rows from.",
            ),
            (
                "asAdmin",
                "Set, to retrieve the dataverse table as admin for environments you are not a member of.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List a table for the given environment.",
                "m365 pp dataverse table get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"aaduser\"",
            ),
            (
                "List a table for the given environment as Admin.",
                "m365 pp dataverse table get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"aaduser\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp dataverse table list",
        summary: "Lists dataverse tables in a given environment",
        options: &[
            (
                "environmentName",
                "The name of the environment to list all tables for.",
            ),
            (
                "asAdmin",
                "Set, to retrieve the dataverse tables as admin for environments you are not a member of.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all tables for the given environment.",
                "m365 pp dataverse table list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\"",
            ),
            (
                "List all tables for the given environment as Admin.",
                "m365 pp dataverse table list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp dataverse table remove",
        summary: "Removes a dataverse table in a given environment",
        options: &[
            (
                "environmentName",
                "The name of the environment to remove a table from.",
            ),
            ("name", "The name of the dataverse table to remove."),
            ("force", "Don't prompt for confirmation"),
            (
                "asAdmin",
                "Set, to remove the dataverse table as admin for environments you are not a member of.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes a dataverse table in a given environment",
                "m365 pp dataverse table remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"aaduser\"",
            ),
            (
                "Removes a dataverse table in a given environment as Admin",
                "m365 pp dataverse table remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"aaduser\" --asAdmin",
            ),
            (
                "Removes a dataverse table in a given environment without prompting for confirmation",
                "m365 pp dataverse table remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"aaduser\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp dataverse table row list",
        summary: "Lists table rows for the given Dataverse table",
        options: &[
            ("environmentName", "The name of the environment"),
            ("entitySetName", "The entity set name of the table."),
            ("tableName", "The name of the table."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all table rows for the given environment based on the entity set name",
                "m365 pp dataverse table row list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --entitySetName \"cr6c3_accounts\"",
            ),
            (
                "List all table rows for the given environment based on the table name",
                "m365 pp dataverse table row list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --tableName \"cr6c3_account\"",
            ),
            (
                "List all table rows for the given environment based on the entity set name as Admin",
                "m365 pp dataverse table row list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --entitySetName \"cr6c3_accounts\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp dataverse table row remove",
        summary: "Removes a row from a dataverse table in a given environment.",
        options: &[
            (
                "environmentName",
                "The name of the environment to remove a row from a table from.",
            ),
            ("id", "The id of the row to remove."),
            ("entitySetName", "The entity set name of the table."),
            ("tableName", "The name of the table."),
            ("force", "Don't prompt for confirmation"),
            (
                "asAdmin",
                "Set, to remove the row from the dataverse table as admin for environments you are not a member of.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes a row from a dataverse table in a given environment",
                "m365 pp dataverse table row remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --tableName \"aadusers\" --id \"21d01cf4-356c-ed11-9561-000d3a4bbea4\"",
            ),
            (
                "Removes a row from a dataverse table in a given environment as Admin",
                "m365 pp dataverse table row remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --tableName \"aadusers\" --id \"21d01cf4-356c-ed11-9561-000d3a4bbea4\" --asAdmin",
            ),
            (
                "Removes a row from a dataverse table in a given environment without prompting for confirmation",
                "m365 pp dataverse table row remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --tableName \"aadusers\" --id \"21d01cf4-356c-ed11-9561-000d3a4bbea4\" --force",
            ),
            (
                "Removes a row from a dataverse table in a given environment based on the entity set name without prompting for confirmation",
                "m365 pp dataverse table row remove --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --entitySetName \"cr6c3_accounts\" --id \"21d01cf4-356c-ed11-9561-000d3a4bbea4\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp environment get",
        summary: "Gets information about the specified Power Platform environment",
        options: &[
            (
                "name",
                "The name of the environment. When not specified, the default environment is retrieved. Specify either `name` or `default`, but not both.",
            ),
            (
                "default",
                "Indicates that the default environment should be retrieved. Specify either `name` or `default`, but not both.",
            ),
            (
                "asAdmin",
                "Run the command as admin and retrieve details of environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
            "Register CLI for Microsoft 365 or Microsoft Entra application as a management application for the Power Platform using",
            "`m365 pp managementapp add [options]`",
        ],
        permissions: &[],
        examples: &[
            (
                "Get information about the Power Platform environment by name.",
                "m365 pp environment get --name Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Get information as admin about the Power Platform environment by name.",
                "m365 pp environment get --name Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --asAdmin",
            ),
            (
                "Get information about the default Power Platform environment.",
                "m365 pp environment get --default",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp environment list",
        summary: "Lists Microsoft Power Platform environments",
        options: &[(
            "asAdmin",
            "Run the command as admin and retrieve all environments. Lists only environments you have explicitly are assigned permissions to by default.",
        )],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability. Register CLI for Microsoft 365 or Microsoft Entra application as a management application for the Power Platform using m365 pp managementapp add [options]",
        ],
        permissions: &[],
        examples: &[
            (
                "List Microsoft Power Platform environments.",
                "m365 pp environment list",
            ),
            (
                "List all Microsoft Power Platform environments.",
                "m365 pp environment list --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp gateway get",
        summary: "Gets information about the specified gateway",
        options: &[("id", "ID of the Gateway.")],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Get information about the specified gateway.",
            "m365 pp gateway get --id 1f69e798-5852-4fdd-ab01-33bb14b6e934",
        )],
        links: &[],
    },
    Help {
        name: "pp gateway list",
        summary: "Returns a list of gateways for which the user is an admin",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List gateways for which the user is an admin.",
            "m365 pp gateway list",
        )],
        links: &[],
    },
    Help {
        name: "pp managementapp add",
        summary: "Registers management application for Power Platform",
        options: &[
            (
                "appId",
                "Application (client) ID of the Microsoft Entra application registration to register as a management app.",
            ),
            (
                "objectId",
                "Object ID of the Microsoft Entra application registration to register as a management app.",
            ),
            (
                "name",
                "Name of the Microsoft Entra application registration to register as a management app.",
            ),
        ],
        notes: &[
            "To execute this command the first time you'll need sign in using the Microsoft Azure PowerShell application registration. You can do this by executing `m365 login --appId 1950a258-227b-4e31-a9cf-717495945fc2`. To register the Microsoft Entra application registration that CLI for Microsoft 365 uses by default, execute `m365 pp managementapp add--appId 31359c7f-bd7e-475c-86db-fdb8c937548e`.",
            "For best performance use the `appId` option to reference the Microsoft Entra application registration to update. If you use `objectId` or `name`, this command will first need to find the corresponding `appId` for that application.",
            "If the command finds multiple Microsoft Entra application registrations with the specified app name, it will prompt you to disambiguate which app it should use, listing the discovered object IDs.",
        ],
        permissions: &[],
        examples: &[
            (
                "Register CLI for Microsoft 365 as a management application for the Power Platform.",
                "m365 pp managementapp add --appId 31359c7f-bd7e-475c-86db-fdb8c937548e",
            ),
            (
                "Register a Microsoft Entra application with the specified object ID as a management application for the Power Platform.",
                "m365 pp managementapp add --objectId d75be2e1-0204-4f95-857d-51a37cf40be8",
            ),
            (
                "Register a Microsoft Entra application named My app as a management application for the Power Platform.",
                "m365 pp managementapp add --name \"My app\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp managementapp list",
        summary: "Lists management applications for Power Platform",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Lists management applications for Power Platform",
            "m365 pp managementapp list",
        )],
        links: &[],
    },
    Help {
        name: "pp solution get",
        summary: "Gets a specific solution in a given environment.",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The ID of the solution."),
            (
                "name",
                "The unique name (not the display name) of the solution.",
            ),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Gets a specific solution in a specific environment based on name.",
                "m365 pp solution get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"Default\"",
            ),
            (
                "Gets a specific solution in a specific environment based on name as Admin.",
                "m365 pp solution get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"Default\" --asAdmin",
            ),
            (
                "Gets a specific solution in a specific environment based on id.",
                "m365 pp solution get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --id \"ee62fd63-e49e-4c09-80de-8fae1b9a427e\"",
            ),
            (
                "Gets a specific solution in a specific environment based on id as Admin.",
                "m365 pp solution get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --id \"ee62fd63-e49e-4c09-80de-8fae1b9a427e\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution list",
        summary: "Lists solutions in a given environment.",
        options: &[
            ("environmentName", "The name of the environment"),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all solutions in a specific environment",
                "m365 pp solution list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\"",
            ),
            (
                "List all solutions in a specific environment as Admin",
                "m365 pp solution list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution publish",
        summary: "Publishes the components of a solution in a given Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the solution."),
            (
                "name",
                "The unique name (not the display name) of the solution.",
            ),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
            ("wait", "Wait for the job to complete"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Publishes the components of a specified solution with a specific name, owned by the currently signed-in user",
                "m365 pp solution publish --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Solution Name\"",
            ),
            (
                "Publishes the components of a specified solution owned by the currently signed-in user based on the id parameter and waits for completion",
                "m365 pp solution publish --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b --wait",
            ),
            (
                "Publishes the components of a specified solution owned by another user based on the name parameter",
                "m365 pp solution publish --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Solution Name\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution publisher add",
        summary: "Adds a specified publisher in a given environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("name", "The unique name of the publisher."),
            ("displayName", "The friendly name of the publisher."),
            ("prefix", "The prefix of the publisher."),
            (
                "choiceValuePrefix",
                "The choice value prefix of the publisher.",
            ),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Add a specific publisher in a specific environment",
                "m365 pp solution publisher add --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --name \"Contoso\" --displayName \"Contoso\" --prefix \"new\" --choiceValuePrefix 10000",
            ),
            (
                "Adds a specific publisher in a specific environment as Admin",
                "m365 pp solution publisher add --environmentName \"Default-d87a7535-dd31-4437-bfe1-95340acd55c5\" --name \"Contoso\" --displayName \"Contoso\" --prefix \"new\" --choiceValuePrefix 10000 --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution publisher get",
        summary: "Gets information about the specified publisher in a given environment.",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The ID of the publisher."),
            ("name", "The unique name of the publisher."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Gets a specific publisher in a specific environment based on name.",
                "m365 pp solution publisher get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"MicrosoftCorporation\"",
            ),
            (
                "Gets a specific publisher in a specific environment based on name as Admin.",
                "m365 pp solution publisher get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --name \"MicrosoftCorporation\" --asAdmin",
            ),
            (
                "Gets a specific publisher in a specific environment based on id.",
                "m365 pp solution publisher get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --id \"ee62fd63-e49e-4c09-80de-8fae1b9a427e\"",
            ),
            (
                "Gets a specific publisher in a specific environment based on id as Admin.",
                "m365 pp solution publisher get --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --id \"ee62fd63-e49e-4c09-80de-8fae1b9a427e\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution publisher list",
        summary: "Lists publishers in a given environment.",
        options: &[
            ("environmentName", "The name of the environment."),
            (
                "withMicrosoftPublishers",
                "Include the Microsoft Publishers.",
            ),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "List all publishers in a specific environment.",
                "m365 pp solution publisher list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\"",
            ),
            (
                "List all publishers in a specific environment as Admin.",
                "m365 pp solution publisher list --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --asAdmin",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution publisher remove",
        summary: "Removes the specified publisher in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the publisher."),
            ("name", "The unique name of the publisher."),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the specified publisher based on the name parameter",
                "m365 pp solution publisher remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Publisher Name\"",
            ),
            (
                "Removes the specified publisher based on the name parameter without confirmation",
                "m365 pp solution publisher remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Publisher Name\" --force",
            ),
            (
                "Removes the specified publisher based on the name parameter as admin",
                "m365 pp solution publisher remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Publisher Name\" --asAdmin",
            ),
            (
                "Removes the specified publisher owned by the currently signed-in user based on the id parameter",
                "m365 pp solution publisher remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp solution remove",
        summary: "Removes the specified solution in the specified Power Platform environment",
        options: &[
            ("environmentName", "The name of the environment."),
            ("id", "The id of the solution."),
            (
                "name",
                "The unique name (not the display name) of the solution.",
            ),
            (
                "asAdmin",
                "Run the command as admin for environments you do not have explicitly assigned permissions to.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the specified solution with a specific name, owned by the currently signed-in user",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Solution Name\"",
            ),
            (
                "Removes the specified solution owned by the currently signed-in user based on the name parameter without confirmation",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Solution Name\" --force",
            ),
            (
                "Removes the specified solution owned by another user based on the name parameter",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"Solution Name\" --asAdmin",
            ),
            (
                "Removes the specified solution owned by another user based on the name parameter without confirmation",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --name \"solution Name\" --asAdmin --force",
            ),
            (
                "Removes the specified solution owned by the currently signed-in user based on the id parameter",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b",
            ),
            (
                "Removes the specified solution owned by the currently signed-in user based on the id parameter without confirmation",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b --force",
            ),
            (
                "Removes the specified solution owned by another user based on the id parameter",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b --asAdmin",
            ),
            (
                "Removes the specified solution owned by another user based on the id parameter without confirmation",
                "m365 pp solution remove --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --id 00000001-0000-0000-0001-00000000009b --asAdmin --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp tenant settings list",
        summary: "Lists the global Power Platform tenant settings",
        options: &[],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[(
            "Lists the global Power Platform settings of the tenant",
            "m365 pp tenant settings list",
        )],
        links: &[],
    },
    Help {
        name: "pp tenant settings set",
        summary: "Sets the global Power Platform configuration of the tenant",
        options: &[
            (
                "walkMeOptOut",
                "Ability to opt out of guided experiences using WalkMe in Power Platform. Valid values: `true`, `false`.",
            ),
            (
                "disableNPSCommentsReachout",
                "Ability to disable re-surveying users who left prior feedback via NPS prompts in Power Platform. Valid values: `true`, `false`.",
            ),
            (
                "disableNewsletterSendout",
                "Ability to disable the newsletter sendout feature. Valid values: `true`, `false`.",
            ),
            (
                "disableEnvironmentCreationByNonAdminUsers",
                "Restrict all environments to be created by Global Admins, Power Platform Admins, or Dynamics365 Service Admins. Valid values: `true`, `false`.",
            ),
            (
                "disablePortalsCreationByNonAdminUsers",
                "Restrict all portals to be created by Global Admins, Power Platform Admins, or Dynamics365 Service Admins. Valid values: `true`, `false`.",
            ),
            (
                "disableSurveyFeedback",
                "Ability to disable all NPS survey feedback prompts in Power Platform. Valid values: `true`, `false`.",
            ),
            (
                "disableTrialEnvironmentCreationByNonAdminUsers",
                "Restrict all trial environments to be created by Global Admins, Power Platform Admins, or Dynamics365 Service Admins. Valid values: `true`, `false`.",
            ),
            (
                "disableCapacityAllocationByEnvironmentAdmins",
                "Ability to disable capacity allocation by environment administrators. Valid values: `true`, `false`.",
            ),
            (
                "disableSupportTicketsVisibleByAllUsers",
                "Ability to disable support ticket creation by non-admin users in the tenant. Valid values: `true`, `false`.",
            ),
            (
                "disableDocsSearch",
                "When this setting is true, users in the environment will see a message that Microsoft Learn and Documentation search categories have been turned off by the administrator in the search results page. Valid values: `true`, `false`.",
            ),
            (
                "disableCommunitySearch",
                "When this setting is true, users in the environment will see a message that Community and Blog search categories have been turned off by the administrator in the search results page. Valid values: `true`, `false`.",
            ),
            (
                "disableBingVideoSearch",
                "When this setting is true, users in the environment will see a message that video search categories have been turned off by the administrator in the search results page. Valid values: `true`, `false`.",
            ),
            (
                "shareWithColleaguesUserLimit",
                "Maximum value setting for the number of users in a security group used to share an app built using Power Apps on Microsoft Teams. Specify any number as a value.",
            ),
            (
                "disableShareWithEveryone",
                "Ability to disable the Share With Everyone capability in all Power Apps. Valid values: `true`, `false`.",
            ),
            (
                "enableGuestsToMake",
                "Ability to allow guest users in your tenant to create Power Apps. Valid values: `true`, `false`.",
            ),
            (
                "disableMembersIndicator",
                "Ability to disable the display of a members indicator. Valid values: `true`, `false`.",
            ),
            (
                "disableMakerMatch",
                "Ability to disable the Maker Match feature, which helps organizations find internal resources with the necessary skills to develop custom apps and solutions using Power Platform. Valid values: `true`, `false`.",
            ),
            (
                "disablePreferredDataLocationForTeamsEnvironment",
                "Ability to disable the selection of a preferred data location for Teams environment. Valid values: `true`, `false`.",
            ),
            (
                "disableAdminDigest",
                "When true, the entire organization is unsubscribed from the weekly digest. Valid values: `true`, `false`.",
            ),
            (
                "disableDeveloperEnvironmentCreationByNonAdminUsers",
                "Restrict all developer environments to be created by Global Admins, Power Platform Admins, or Dynamics365 Service Admins. Valid values: `true`, `false`.",
            ),
            (
                "disableBillingPolicyCreationByNonAdminUsers",
                "Restrict billing policies to be created by Global Admins, Power Platform Admins, or Dynamics365 Service Admins. Valid values: `true`, `false`.",
            ),
            (
                "storageCapacityConsumptionWarningThreshold",
                "Ability to set a threshold for storage capacity consumption warnings. Specify any number as a value.",
            ),
            (
                "disableChampionsInvitationReachout",
                "Ability to disable all invitations to become a Power Platform champion. Valid values: `true`, `false`.",
            ),
            (
                "disableSkillsMatchInvitationReachout",
                "Ability to disable all skills match invitations to become part of the makers community. Valid values: `true`, `false`.",
            ),
            (
                "disableCopilot",
                "Ability to turn off or disable the Copilot feature. Valid values: `true`, `false`.",
            ),
            (
                "enableOpenAiBotPublishing",
                "Ability to enable or disable the publishing of OpenAI bots within the organization's environment. Valid values: `true`, `false`.",
            ),
            (
                "enableModelDataSharing",
                "Ability to enable or disable the sharing of model data within the organization's environment. Valid values: `true`, `false`.",
            ),
        ],
        notes: &[
            "This command is based on an API that is currently in preview and is subject to change once the API reached general availability.",
        ],
        permissions: &[],
        examples: &[
            (
                "Disable environment creation for non-admin users",
                "m365 pp tenant settings set --disableEnvironmentCreationByNonAdminUsers true --disableTrialEnvironmentCreationByNonAdminUsers true --disableDeveloperEnvironmentCreationByNonAdminUsers true",
            ),
            (
                "Enable Power App creation for guest users",
                "m365 pp tenant settings set --enableGuestsToMake true",
            ),
            (
                "Disable guided experience, survey feedback and newsletter",
                "m365 pp tenant settings set --walkMeOptOut true --disableNewsletterSendout true --disableSurveyFeedback true",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp website get",
        summary: "Gets information about the specified Power Pages website.",
        options: &[
            ("url", "The URL of the website to retrieve."),
            ("name", "The name of the website to retrieve."),
            ("id", "The WebSite Id (GUID) of the website to retrieve."),
            (
                "environmentName",
                "The name of the environment from which to retrieve the Power Pages website.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieve Demo Power Pages website by name in the given environment using `name`.",
                "m365 pp website get --name Demo --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Retrieve Demo Power Pages website by name in the given environment using `id`.",
                "m365 pp website get --id 4916bb2c-91e1-4716-91d5-b6171928fac9 --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Retrieve Demo Power Pages website by name in the given environment using `url`.",
                "m365 pp website get --url https://site-0uaq9.powerappsportals.com --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp website remove",
        summary: "Removes the specified Power Pages website from the list of active sites.",
        options: &[
            ("url", "The URL of the website to remove."),
            ("name", "The name of the website to remove."),
            ("id", "The WebSite Id (GUID) of the website to remove."),
            (
                "environmentName",
                "The name of the environment from which to remove the Power Pages website.",
            ),
            ("force", "Don't prompt for confirmation"),
        ],
        notes: &[],
        permissions: &[(
            "Delegated",
            "Power Platform API",
            "PowerPages.Websites.Write",
        )],
        examples: &[
            (
                "Remove Power Pages website by name.",
                "m365 pp website remove --name Demo --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Remove Power Pages website by name without confirmation.",
                "m365 pp website remove --name Demo --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --force",
            ),
            (
                "Remove Power Pages website by id.",
                "m365 pp website remove --id 4916bb2c-91e1-4716-91d5-b6171928fac9 --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Remove Power Pages website by id without confirmation.",
                "m365 pp website remove --id 4916bb2c-91e1-4716-91d5-b6171928fac9 --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --force",
            ),
            (
                "Remove Power Pages website by url.",
                "m365 pp website remove --url https://site-0uaq9.powerappsportals.com --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5",
            ),
            (
                "Remove Power Pages website by url without confirmation.",
                "m365 pp website remove --url https://site-0uaq9.powerappsportals.com --environmentName Default-d87a7535-dd31-4437-bfe1-95340acd55c5 --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "pp website webrole list",
        summary: "Lists all webroles for the specified Power Pages website.",
        options: &[
            ("websiteId", "ID of the Power Pages website."),
            (
                "websiteName",
                "The unique name (not the display name) of the Power Pages website.",
            ),
            (
                "environmentName",
                "The name of the environment where the Power Pages websites are located.",
            ),
            (
                "asAdmin",
                "Run the command as admin and retrieve Power Pages websites for environments you do not have explicitly assigned permissions to.",
            ),
        ],
        notes: &[],
        permissions: &[
            (
                "Delegated",
                "Power Platform API",
                "PowerPages.Websites.Read",
            ),
            ("Delegated", "Dynamics CRM", "user_impersonation"),
        ],
        examples: &[
            (
                "List all webroles for the site by name.",
                "m365 pp website webrole list --websiteName \"Contoso\" --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\"",
            ),
            (
                "List all webroles for the site by name as admin.",
                "m365 pp website webrole list --websiteName \"Contoso\" --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --asAdmin",
            ),
            (
                "List all webroles for the site by id.",
                "m365 pp website webrole list --websiteId \"2ca3eaa5-140f-4175-8261-3272edf9f339\" --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\"",
            ),
            (
                "List all webroles for the site by id as admin.",
                "m365 pp website webrole list --websiteId \"2ca3eaa5-140f-4175-8261-3272edf9f339\" --environmentName \"Default-2ca3eaa5-140f-4175-8261-3272edf9f339\" --asAdmin",
            ),
        ],
        links: &[],
    },
];

//! Help for the `m365 onenote ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "onenote notebook add",
        summary: "Creates a new OneNote notebook.",
        options: &[
            (
                "name",
                "Name of the notebook. Notebook names must be unique. The name cannot contain more than 128 characters or contain the following characters: `?*/:<>`",
            ),
            (
                "userId",
                "Id of the user. Specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, but not multiple.",
            ),
            (
                "userName",
                "Name of the user. Specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, but not multiple.",
            ),
            (
                "groupId",
                "Id of the SharePoint group. Specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, but not multiple.",
            ),
            (
                "groupName",
                "Name of the SharePoint group. Specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, but not multiple.",
            ),
            (
                "webUrl",
                "URL of the SharePoint site. Specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, but not multiple.",
            ),
        ],
        notes: &["This command does not support application permissions."],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "Notes.Create, Sites.Read.All, User.ReadBasic.All, Group.Read.All",
        )],
        examples: &[
            (
                "Create a Microsoft OneNote notebook for the currently logged in user",
                "m365 onenote notebook add --name \"Private Notebook\"",
            ),
            (
                "Create a Microsoft OneNote notebook in a group specified by id.",
                "m365 onenote notebook add --name \"Private Notebook\" --groupId 233e43d0-dc6a-482e-9b4e-0de7a7bce9b4",
            ),
            (
                "Create a Microsoft OneNote notebook in a group specified by displayName.",
                "m365 onenote notebook add --name \"Private Notebook\" --groupName \"MyGroup\"",
            ),
            (
                "Create a Microsoft OneNote notebook for a user specified by name",
                "m365 onenote notebook add --name \"Private Notebook\" --userName user1@contoso.onmicrosoft.com",
            ),
            (
                "Create a Microsoft OneNote notebook for a user specified by id",
                "m365 onenote notebook add --name \"Private Notebook\" --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "Creates a Microsoft OneNote notebooks for a site",
                "m365 onenote notebook add --name \"Private Notebook\" --webUrl https://contoso.sharepoint.com/sites/testsite",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onenote notebook list",
        summary: "Retrieves a list of notebooks.",
        options: &[
            (
                "userId",
                "Id of the user. Use either userId or userName, but not both.",
            ),
            (
                "userName",
                "Name of the user. Use either userId or userName, but not both.",
            ),
            (
                "groupId",
                "Id of the SharePoint group. Use either groupName or groupId, but not both",
            ),
            (
                "groupName",
                "Name of the SharePoint group. Use either groupName or groupId, but not both.",
            ),
            ("webUrl", "URL of the SharePoint site."),
        ],
        notes: &["This command does not support application permissions."],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "Notes.Read.All, Sites.Read.All, User.ReadBasic.All, Group.Read.All",
        )],
        examples: &[
            (
                "List Microsoft OneNote notebooks for the currently logged in user",
                "m365 onenote notebook list",
            ),
            (
                "List Microsoft OneNote notebooks in group 233e43d0-dc6a-482e-9b4e-0de7a7bce9b4",
                "m365 onenote notebook list --groupId 233e43d0-dc6a-482e-9b4e-0de7a7bce9b4",
            ),
            (
                "List Microsoft OneNote notebooks in group My Group",
                "m365 onenote notebook list --groupName \"MyGroup\"",
            ),
            (
                "List Microsoft OneNote notebooks from a user by user name",
                "m365 onenote notebook list --userName user1@contoso.onmicrosoft.com",
            ),
            (
                "List Microsoft OneNote notebooks for user 2609af39-7775-4f94-a3dc-0dd67657e900",
                "m365 onenote notebook list --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "List Microsoft OneNote notebooks for site https://contoso.sharepoint.com/sites/testsite",
                "m365 onenote notebook list --webUrl https://contoso.sharepoint.com/sites/testsite",
            ),
        ],
        links: &[(
            "https://learn.microsoft.com/graph/api/onenote-list-notebooks?view=graph-rest-1.0&tabs=http",
            "https://learn.microsoft.com/graph/api/onenote-list-notebooks?view=graph-rest-1.0&tabs=http",
        )],
    },
    Help {
        name: "onenote page list",
        summary: "Retrieves a list of OneNote pages.",
        options: &[
            (
                "userId",
                "Id of the user. Use either `userId`, `userName`, `groupId`, `groupName` or `webUrl` but not multiple.",
            ),
            (
                "userName",
                "Name of the user. Use either `userId`, `userName`, `groupId`, `groupName` or `webUrl` but not multiple.",
            ),
            (
                "groupId",
                "Id of the SharePoint group. Use either `userId`, `userName`, `groupId`, `groupName` or `webUrl` but not multiple.",
            ),
            (
                "groupName",
                "Name of the SharePoint group. Use either `userId`, `userName`, `groupId`, `groupName` or `webUrl` but not multiple.",
            ),
            (
                "webUrl",
                "URL of the SharePoint site. Use either `userId`, `userName`, `groupId`, `groupName` or `webUrl` but not multiple.",
            ),
        ],
        notes: &[
            "When you don't specify either `userId`, `userName`, `groupId`, `groupName` or `webUrl`, the OneNote pages will be retrieved of the currently logged in user.",
            "This command does not support application permissions.",
        ],
        permissions: &[(
            "Delegated",
            "Microsoft Graph",
            "Notes.Read.All, Sites.Read.All, User.ReadBasic.All, Group.Read.All",
        )],
        examples: &[
            (
                "List Microsoft OneNote pages for the currently logged in user.",
                "m365 onenote page list",
            ),
            (
                "List Microsoft OneNote pages in a specific group specified by id.",
                "m365 onenote page list --groupId 233e43d0-dc6a-482e-9b4e-0de7a7bce9b4",
            ),
            (
                "List Microsoft OneNote pages in a specific group specified by name.",
                "m365 onenote page list --groupName \"MyGroup\"",
            ),
            (
                "List Microsoft OneNote pages for a specific user specified by name.",
                "m365 onenote page list --userName user1@contoso.onmicrosoft.com",
            ),
            (
                "List Microsoft OneNote pages for a specific user specified by id.",
                "m365 onenote page list --userId 2609af39-7775-4f94-a3dc-0dd67657e900",
            ),
            (
                "List Microsoft OneNote pages for a specific site.",
                "m365 onenote page list --webUrl https://contoso.sharepoint.com/sites/testsite",
            ),
        ],
        links: &[],
    },
];

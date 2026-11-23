//! Help for the `m365 exo ...`.

use super::Help;

pub static HELP: &[Help] = &[Help {
    name: "exo approleassignment add",
    summary: "Grants permissions to an application that's accessing data in Exchange Online and specify which mailboxes an app can access.",
    options: &[
        ("roleDefinitionId", "Id of a role to be assigned."),
        ("roleDefinitionName", "Name of a role to be assigned."),
        (
            "principalId",
            "Id of a service principal to which the assignment is granted.",
        ),
        (
            "principalName",
            "Name of a service principal to which the assignment is granted.",
        ),
        (
            "scope",
            "Scope of the roleassignment. Allowed values are: `tenant`, `administrativeUnit`, `group`, `user`, `custom`.",
        ),
        (
            "userId",
            "Id of a user to which the assignment is scoped. Specify either `userId` or `userName` when scope is set to `user`.",
        ),
        (
            "userName",
            "UPN of a user to which the assignment is scoped. Specify either `userId` or `userName` when scope is set to `user`.",
        ),
        (
            "groupId",
            "Id of a group to which the assignment is scoped. Specify either `groupId` or `groupName` when scope is set to `group`.",
        ),
        (
            "groupName",
            "Name of a group to which the assignment is scoped. Specify either `groupId` or `groupName` when scope is set to `group`.",
        ),
        (
            "administrativeUnitId",
            "Id of an administrative unit to which the assignment is scoped. Specify either `administrativeUnitId` or `administrativeUnitName` when scope is set to `administrativeUnit`.",
        ),
        (
            "administrativeUnitName",
            "Name of an administrative unit to which the assignment is scoped. Specify either `administrativeUnitId` or `administrativeUnitName` when scope is set to `administrativeUnit`.",
        ),
        (
            "customAppScopeId",
            "Id of a custom application scope to which the assignment is scoped. Specify either `customAppScopeId` or `customAppScopeName` when scope is set to `custom`.",
        ),
        (
            "customAppScopeName",
            "Name of a custom application scope to which the assignment is scoped. Specify either `customAppScopeId` or `customAppScopeName` when scope is set to `custom`.",
        ),
    ],
    notes: &["To use this command you must be at least Privileged Role Administrator."],
    permissions: &[
        (
            "Delegated",
            "Microsoft Graph",
            "RoleManagement.ReadWrite.Exchange",
        ),
        (
            "Application",
            "Microsoft Graph",
            "RoleManagement.ReadWrite.Exchange",
        ),
    ],
    examples: &[
        (
            "Assign a role specified by id to a service principal specified by id and scope the assignment to the whole tenant",
            "m365 exo approleassignment add --roleDefinitionId 777b752-f9b7-4205-a2b1-5db0d6a0ccfc --principalId 7a2ca997-9461-402e-9882-58088a370889 --scope tenant",
        ),
        (
            "Assign a role specified by id to a service principal specified by id and scope the assignment to a user specified by id",
            "m365 exo approleassignment add --roleDefinitionId 777b752-f9b7-4205-a2b1-5db0d6a0ccfc --principalId 7a2ca997-9461-402e-9882-58088a370889 --scope user --userId a4738dd8-fc0f-4646-87fb-47539f5c651b",
        ),
        (
            "Assign a role specified by name to a service principal specified by name and scope the assignment to a group specified by name",
            "m365 exo approleassignment add --roleDefinitionName 'Application Contacts.ReadWrite' --principalName 'ContactsSyncApp' --scope group --groupName 'Marketing'",
        ),
        (
            "Assign a role specified by name to a service principal specified by id and scope the assignment to an administrative unit specified by name",
            "m365 exo approleassignment add --roleDefinitionName 'Application Calendars.Read' --principalId fa631c4d-ac9f-4884-a7f5-13c659d177e3 --scope administrativeUnit --administrativeUnitName 'Equipment - EMEA'",
        ),
    ],
    links: &[(
        "Role assignment",
        "https://learn.microsoft.com/graph/api/rbacapplication-post-roleassignments?view=graph-rest-beta",
    )],
}];

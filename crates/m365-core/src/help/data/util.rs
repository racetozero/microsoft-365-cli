//! Help for the `m365 util ...`.

use super::Help;

pub static HELP: &[Help] = &[Help {
    name: "util accesstoken get",
    summary: "Gets access token for the specified resource",
    options: &[
        (
            "resource",
            "The resource for which to retrieve an access token",
        ),
        (
            "new",
            "Retrieve a new access token to ensure that it's valid for as long as possible",
        ),
        ("decoded", "Retrieve a decoded access token"),
    ],
    notes: &[
        "Instead of specifying the full URL of the resource, you can use one of the following shorthand identifiers: `sharepoint`, `graph`.",
        "The `util accesstoken get` command returns an access token for the specified resource. If an access token has been previously retrieved and is still valid, the command will return the cached token. If you want to ensure that the returned access token is valid for as long as possible, you can force the command to retrieve a new access token by using the `--new` option.",
    ],
    permissions: &[],
    examples: &[
        (
            "Get access token for the Microsoft Graph",
            "m365 util accesstoken get --resource https://graph.microsoft.com",
        ),
        (
            "Get access token for SharePoint Online using the shorthand identifier",
            "m365 util accesstoken get --resource sharepoint",
        ),
        (
            "Get access token for Microsoft Graph using the shorthand identifier",
            "m365 util accesstoken get --resource graph",
        ),
        (
            "Get a new access token for SharePoint Online",
            "m365 util accesstoken get --resource https://contoso.sharepoint.com --new",
        ),
        (
            "Get a decoded access token for SharePoint Online",
            "m365 util accesstoken get --resource https://contoso.sharepoint.com --decoded",
        ),
    ],
    links: &[],
}];

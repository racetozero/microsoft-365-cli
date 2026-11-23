//! Help for the `m365 spp ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "spp autofillcolumn set",
        summary: "Applies the autofill option to the selected column",
        options: &[
            ("siteUrl", "The URL of the target site."),
            (
                "listTitle",
                "The title of the library on which to apply the model.",
            ),
            (
                "listId",
                "The ID of the library on which to apply the model.",
            ),
            (
                "listUrl",
                "Server or web-relative URL of the library on which to apply the model.",
            ),
            (
                "columnId",
                "ID of the column to which the autofill option will be assigned.",
            ),
            (
                "columnTitle",
                "Title of the column to which the autofill option will be assigned.",
            ),
            (
                "columnInternalName",
                "The internal name (case-sensitive) of the column to which the autofill option will be assigned.",
            ),
            (
                "prompt",
                "The text in natural language that will be used to extract specific information or generate information from files within a SharePoint library.",
            ),
            (
                "isEnabled",
                "Enables or disables the autofill column feature.",
            ),
        ],
        notes: &[
            "The `prompt` parameter is required when setting the autofill column for the first time.",
        ],
        permissions: &[],
        examples: &[
            (
                "Applies an autofill column on a selected column to a document library based on the list id.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnId \"1045e69d-21fb-4214-a25a-9bdfa7cb63a2\" --prompt \"Write a 2-line summary of the document\"",
            ),
            (
                "Applies an autofill column on a selected column to a document library based on the list title.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listTitle \"Documents\" --columnId \"1045e69d-21fb-4214-a25a-9bdfa7cb63a2\" --prompt \"Write a 2-line summary of the document\"",
            ),
            (
                "Applies an autofill column on a selected column to a document library based on the list url.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listUrl '/Shared Documents' --columnId \"1045e69d-21fb-4214-a25a-9bdfa7cb63a2\" --prompt \"Write a 2-line summary of the document\"",
            ),
            (
                "Applies an autofill column on a selected column to a document library based on the list id and column title.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnTitle \"ColumnTitle\" --prompt \"Write a 2-line summary of the document\"",
            ),
            (
                "Applies an autofill column on a selected column to a document library based on the list id and column internal name.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnInternalName \"ColumnTitle\" --prompt \"Write a 2-line summary of the document\"",
            ),
            (
                "Disables the autofill column for a selected column in a document library, based on the list id and column title.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnTitle \"ColumnTitle\" --isEnabled false",
            ),
            (
                "Reenable the autofill column for a selected column in a document library, based on the list id and column title.",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnTitle \"ColumnTitle\" --isEnabled true",
            ),
            (
                "Modifies the prompt for the autofill column",
                "m365 spp autofillcolumn set --siteUrl \"https://contoso.sharepoint.com/sites/mainSite\" --listId \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --columnTitle \"ColumnTitle\" --isEnabled true --prompt \"Write a 1-line summary of the document\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spp contentcenter list",
        summary: "Gets information about the SharePoint Premium content centers",
        options: &[],
        notes: &[
            "To use this command you must be either SharePoint Administrator or Global Administrator.",
        ],
        permissions: &[],
        examples: &[(
            "Gets information about the SharePoint Premium content centers",
            "m365 spp contentcenter list",
        )],
        links: &[],
    },
    Help {
        name: "spp model apply",
        summary: "Applies (or syncs) a trained document understanding model to a document library",
        options: &[
            ("webUrl", "The URL of the web where the library is located."),
            (
                "contentCenterUrl",
                "The URL of the content center site where model is located.",
            ),
            ("id", "The unique ID of the model."),
            ("title", "The display name of the model."),
            (
                "listTitle",
                "The title of the document library to which the model will be applied.",
            ),
            (
                "listId",
                "The ID of the library to which the model will be applied.",
            ),
            (
                "listUrl",
                "Server or web-relative URL of the library to which the model will be applied.",
            ),
            (
                "viewOption",
                "Defines whether the model view should be set as the default view for the document library. Allowed values are: `NewViewAsDefault`, `DoNotChangeDefault`, `TileViewAsDefault`. The default value is `NewViewAsDefault`.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Applies a trained document understanding model using its unique ID to a document library, identified by its title.",
                "m365 spp model apply --webUrl \"https://contoso.sharepoint.com\" --contentCenterUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --id \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\" --listTitle \"Shared Documents\"",
            ),
            (
                "Applies a trained document understanding model using its display name to a document library, identified by its title.",
                "m365 spp model apply --webUrl \"https://contoso.sharepoint.com\" --contentCenterUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"ModelExample\" --listTitle \"Shared Documents\"",
            ),
            (
                "Applies a trained document understanding model using its display name to a document library, identified by its URL.",
                "m365 spp model apply --webUrl \"https://contoso.sharepoint.com\" --contentCenterUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"ModelExample\" --listUrl \"/Shared Documents\"",
            ),
            (
                "Applies a trained document understanding model using its display name to a document library, identified by its unique ID.",
                "m365 spp model apply --webUrl \"https://contoso.sharepoint.com\" --contentCenterUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"ModelExample\" --listId \"b4cfa0d9-b3d7-49ae-a0f0-f14ffdd005f7\"",
            ),
            (
                "Applies a trained document understanding model using its display name to a document library, identified by its unique ID. Without changing a default document library view.",
                "m365 spp model apply --webUrl \"https://contoso.sharepoint.com\" --contentCenterUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"ModelExample\" --listId \"b4cfa0d9-b3d7-49ae-a0f0-f14ffdd005f7\" --viewOption \"DoNotChangeDefault\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spp model get",
        summary: "Retrieves information about a document understanding model",
        options: &[
            ("siteUrl", "The URL of the content center site."),
            ("id", "The unique ID of the model to retrieve."),
            ("title", "The display name of the model to retrieve."),
            (
                "withPublications",
                "Retrieves information about the library where it has been applied.",
            ),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Retrieve information about a SharePoint Premium document understanding model, using model unique ID.",
                "m365 spp model get --siteUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --id \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\"",
            ),
            (
                "Retrieve information about a SharePoint Premium document understanding model, using model Title",
                "m365 spp model get --siteUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"climicrosoft365Model\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "spp model list",
        summary: "Retrieves the list of unstructured document processing models",
        options: &[("siteUrl", "The URL of the content center site.")],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Retrieve a list of SharePoint Premium unstructured document processing models on the content center site.",
            "m365 spp model list --siteUrl https://contoso.sharepoint.com/sites/ContentCenter",
        )],
        links: &[],
    },
    Help {
        name: "spp model remove",
        summary: "Deletes a document understanding model",
        options: &[
            ("siteUrl", "The URL of the content center site."),
            ("id", "The unique ID of the model."),
            ("title", "The display name of the model."),
            ("force", "Don't prompt for confirmation."),
        ],
        notes: &["The model must be removed from all libraries before it can be deleted."],
        permissions: &[],
        examples: &[
            (
                "Delete a SharePoint Premium document understanding model using the model’s unique ID.",
                "m365 spp model remove --siteUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --id \"7645e69d-21fb-4a24-a17a-9bdfa7cb63dc\"",
            ),
            (
                "Delete a SharePoint Premium document understanding model using the model’s title.",
                "m365 spp model remove --siteUrl \"https://contoso.sharepoint.com/sites/ContentCenter\" --title \"climicrosoft365Model.classifier\"",
            ),
        ],
        links: &[],
    },
];

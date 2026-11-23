//! Help for the `m365 context ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "context init",
        summary: "Initiates CLI for Microsoft 365 context in the current working folder",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Initiates CLI for Microsoft 365 context in the current working folder",
            "m365 context init",
        )],
        links: &[],
    },
    Help {
        name: "context option list",
        summary: "Lists all options added to the context",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "List all options added to the context",
            "m365 context option list",
        )],
        links: &[],
    },
    Help {
        name: "context option remove",
        summary: "Removes an already available name from local context file.",
        options: &[
            (
                "name",
                "The name of the option which will be deleted from the context",
            ),
            ("force", "Don't prompt for confirming removing the option"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes an already available name from the local context file",
                "m365 context option remove --name \"listName\"",
            ),
            (
                "Removes an already available name from the local context file without confirmation",
                "m365 context option remove --name \"listName\" --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "context option set",
        summary: "Allows to add a new name for the option and value to the local context file.",
        options: &[
            ("name", "The option name for which we will define the value"),
            ("value", "Default value for the option"),
        ],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Define a new default value for name listName in the context",
            "m365 context option set --name 'listName' --value 'testList'",
        )],
        links: &[],
    },
    Help {
        name: "context remove",
        summary: "Removes the CLI for Microsoft 365 context in the current working folder",
        options: &[(
            "force",
            "Don't prompt for confirmation to remove the context",
        )],
        notes: &[],
        permissions: &[],
        examples: &[
            (
                "Removes the CLI for Microsoft 365 context in the current working folder",
                "m365 context remove",
            ),
            (
                "Removes the CLI for Microsoft 365 context in the current working folder and does not prompt for confirmation before deleting.",
                "m365 context remove --force",
            ),
        ],
        links: &[],
    },
];

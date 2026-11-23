//! Help for the `m365 connection ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "connection list",
        summary: "Shows the list of available connections",
        options: &[],
        notes: &[
            "If you are logged in to Microsoft 365 with multiple identities, this command will show you a list of users and/or applications used to sign in.",
        ],
        permissions: &[],
        examples: &[(
            "Show the list of available identities used to sign in to Microsoft 365",
            "m365 connection list",
        )],
        links: &[],
    },
    Help {
        name: "connection remove",
        summary: "Removes the specified connection",
        options: &[
            ("name", "The name of the connection to remove."),
            (
                "force",
                "Don't prompt for confirmation to remove the connection.",
            ),
        ],
        notes: &["The value for `--name` can be found by running m365 connection list."],
        permissions: &[],
        examples: &[
            (
                "Remove an existing connection",
                "m365 connection remove --name '0bb7cb89-7fae-4775-a01a-c372cc167371'",
            ),
            (
                "Remove an existing connection without prompting for confirmation",
                "m365 connection remove --name '0bb7cb89-7fae-4775-a01a-c372cc167371' --force",
            ),
        ],
        links: &[],
    },
    Help {
        name: "connection set",
        summary: "Renames the specified connection",
        options: &[
            ("name", "The name of the connection to update."),
            ("newName", "The new name of the connection."),
        ],
        notes: &[
            "The value for `--name` can be found by running m365 connection list. You can update the name of a connection to any value to make switching connections easier.",
        ],
        permissions: &[],
        examples: &[(
            "Update an existing connection",
            "m365 connection set --name '0bb7cb89-7fae-4775-a01a-c372cc167371' --newName 'myworkaccount'",
        )],
        links: &[],
    },
    Help {
        name: "connection use",
        summary: "Activates the specified Microsoft 365 tenant connection",
        options: &[("name", "The name of the connection to switch to.")],
        notes: &[
            "If you haven't disabled the \"prompt\" setting, running this command without options will show a list of available connections. You can then select the connection to activate it.",
            "You can update the name of a connection by running m365 connection set.",
        ],
        permissions: &[],
        examples: &[
            (
                "Switch to a different connection",
                "m365 connection use --name '6e70c8ea-571d-4daf-bc48-9e1b49ac3390'",
            ),
            (
                "Switch to another connection by a custom connection name",
                "m365 connection use --name 'myworkaccount'",
            ),
        ],
        links: &[],
    },
];

//! Help for the `m365 booking ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "booking business get",
        summary: "Retrieves the specified Microsoft Bookings business.",
        options: &[
            ("id", "ID of the business."),
            ("name", "Name of the business."),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Bookings.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "BookingsAppointment.ReadWrite.All",
            ),
        ],
        examples: &[
            (
                "Retrieve the specified Microsoft Bookings business with the specified id.",
                "m365 booking business get --id 'business@contoso.onmicrosoft.com'",
            ),
            (
                "Retrieve the specified Microsoft Bookings business with the specified name.",
                "m365 booking business get --name 'business name'",
            ),
        ],
        links: &[],
    },
    Help {
        name: "booking business list",
        summary: "Lists all Microsoft Bookings businesses that are created for the tenant.",
        options: &[],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Bookings.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "BookingsAppointment.ReadWrite.All",
            ),
        ],
        examples: &[(
            "Returns a list of all Microsoft Bookings businesses that are created for the tenant.",
            "m365 booking business list",
        )],
        links: &[],
    },
];

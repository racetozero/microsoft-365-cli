//! Help for the `m365 onedrive ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "onedrive list",
        summary: "Retrieves a list of OneDrive sites",
        options: &[],
        notes: &[],
        permissions: &[],
        examples: &[(
            "Retrieves a list of OneDrive sites from the tenant.",
            "m365 onedrive list",
        )],
        links: &[],
    },
    Help {
        name: "onedrive report activityfilecounts",
        summary: "Gets the number of unique, licensed users that performed file interactions against any OneDrive account",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the number of unique, licensed users that performed file interactions against any OneDrive account for the last week",
                "m365 onedrive report activityfilecounts --period D7",
            ),
            (
                "Gets the number of unique, licensed users that performed file interactions against any OneDrive account for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report activityfilecounts --period D7 --output text > \"activityfilecounts.txt\"",
            ),
            (
                "Gets the number of unique, licensed users that performed file interactions against any OneDrive account for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report activityfilecounts --period D7 --output json > \"activityfilecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report activityusercounts",
        summary: "Gets the trend in the number of active OneDrive users",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trend in the number of active OneDrive users for the last week",
                "m365 onedrive report activityusercounts --period D7",
            ),
            (
                "Gets the trend in the number of active OneDrive users for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report activityusercounts --period D7 --output text > \"activityusercounts.txt\"",
            ),
            (
                "Gets the trend in the number of active OneDrive users for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report activityusercounts --period D7 --output json > \"activityusercounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report activityuserdetail",
        summary: "Gets details about OneDrive activity by user",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is YYYY-MM-DD. Specify the date or period, but not both`",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about OneDrive activity by user for the last week",
                "m365 onedrive report activityuserdetail --period D7",
            ),
            (
                "Gets details about OneDrive activity by user for May 1, 2019",
                "m365 onedrive report activityuserdetail --date 2019-05-01",
            ),
            (
                "Gets details about OneDrive activity by user for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report activityuserdetail --period D7 --output text > \"onedriveactivityuserdetail.txt\"",
            ),
            (
                "Gets details about OneDrive activity by user for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report activityuserdetail --period D7 --output json > \"onedriveactivityuserdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report usageaccountcounts",
        summary: "Gets the trend in the number of active OneDrive for Business sites",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "This command supports only csv and json output.",
            "Any site on which users viewed, modified, uploaded, downloaded, shared, or synced files is considered an active site",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the trend in the number of active OneDrive for Business sites for the last week",
                "m365 onedrive report usageaccountcounts --period D7",
            ),
            (
                "Gets the trend in the number of active OneDrive for Business sites for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report usageaccountcounts --period D7 --output text > \"usageaccountcounts.txt\"",
            ),
            (
                "Gets the trend in the number of active OneDrive for Business sites for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report usageaccountcounts --period D7 --output json > \"usageaccountcounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report usageaccountdetail",
        summary: "Gets details about OneDrive usage by account",
        options: &[
            (
                "period",
                "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
            ),
            (
                "date",
                "The date for which you would like to view the users who performed any activity. Supported date format is YYYY-MM-DD. Specify the date or period, but not both`",
            ),
        ],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets details about OneDrive usage by account for the last week",
                "m365 onedrive report usageaccountdetail --period D7",
            ),
            (
                "Gets details about OneDrive usage by account for May 1, 2019",
                "m365 onedrive report usageaccountdetail --date 2019-05-01",
            ),
            (
                "Gets details about OneDrive usage by account for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report usageaccountdetail --period D7 --output text > \"onedriveusageaccountdetail.txt\"",
            ),
            (
                "Gets details about OneDrive usage by account for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report usageaccountdetail --period D7 --output json > \"onedriveusageaccountdetail.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report usagefilecounts",
        summary: "Gets the total number of files across all sites and how many are active files",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &[
            "A file is considered active if it has been saved, synced, modified, or shared within the specified time period.",
            "This command supports only csv and json output.",
        ],
        permissions: &[],
        examples: &[
            (
                "Gets the total number of files across all sites and how many are active files for the last week",
                "m365 onedrive report usagefilecounts --period D7",
            ),
            (
                "Gets the total number of files across all sites and how many are active files for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report usagefilecounts --period D7 --output text > \"usagefilecounts.txt\"",
            ),
            (
                "Gets the total number of files across all sites and how many are active files for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report usagefilecounts --period D7 --output json > \"usagefilecounts.json\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "onedrive report usagestorage",
        summary: "Gets the trend on the amount of storage you are using in OneDrive for Business",
        options: &[(
            "period",
            "The length of time over which the report is aggregated. Supported values `D7`, `D30`, `D90`, `D180`.",
        )],
        notes: &["This command supports only csv and json output."],
        permissions: &[],
        examples: &[
            (
                "Gets the trend on the amount of storage you are using in OneDrive for Business for the last week",
                "m365 onedrive report usagestorage --period D7",
            ),
            (
                "Gets the trend on the amount of storage you are using in OneDrive for Business for the last week and exports the report data in the specified path in text format",
                "m365 onedrive report usagestorage --period D7 --output text > \"usagestorage.txt\"",
            ),
            (
                "Gets the trend on the amount of storage you are using in OneDrive for Business for the last week and exports the report data in the specified path in json format",
                "m365 onedrive report usagestorage --period D7 --output json > \"usagestorage.json\"",
            ),
        ],
        links: &[],
    },
];

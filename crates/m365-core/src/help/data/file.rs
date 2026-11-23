//! Help for the `m365 file ...`.

use super::Help;

pub static HELP: &[Help] = &[
    Help {
        name: "file add",
        summary: "Uploads file to the specified site using Microsoft Graph",
        options: &[
            (
                "folderUrl",
                "The URL of the document library where the file should be uploaded to",
            ),
            ("filePath", "Local path to the file to upload"),
            (
                "siteUrl",
                "URL of the site to which upload the file. Specify to suppress lookup.",
            ),
        ],
        notes: &[
            "The `folderUrl` must be an absolute URL to the document library where the file should be uploaded. The document library can be located in any site collection in your tenant, including OneDrive for Business. The `folderUrl` can also point to a (sub)folder in the document library.",
            "By default, the `file add` command will automatically lookup the ID of the site where you want to upload the file based on the specified `folderUrl`. It will do this, by breaking the URL into chunks and incrementally calling Microsoft Graph to retrieve site information. This is necessary, because there is no other way looking at the URL to distinguish where the site URL ends and the document library URL starts. If you want to speed up uploading files, or you use resource-specific consent and your Microsoft Entra app only has access to the specific site, you can use the `siteUrl` option to specify the URL of the site yourself.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.ReadWrite.All",
            ),
            ("Application", "Microsoft Graph", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Uploads file from the current folder to the root folder of a document library in the root site collection",
                "m365 file add --filePath file.pdf --folderUrl \"https://contoso.sharepoint.com/Shared Documents\"",
            ),
            (
                "Uploads file from the current folder to a subfolder of a document library in the root site collection",
                "m365 file add --filePath file.pdf --folderUrl \"https://contoso.sharepoint.com/Shared Documents/Folder\"",
            ),
            (
                "Uploads file from the current folder to a document library in OneDrive for Business",
                "m365 file add --filePath file.pdf --folderUrl \"https://contoso-my.sharepoint.com/personal/steve_contoso_com/Documents\"",
            ),
            (
                "Uploads file from the current folder to a document library in a non-root site collection",
                "m365 file add --filePath file.pdf --folderUrl \"https://contoso.sharepoint.com/sites/Contoso/Shared Documents\"",
            ),
            (
                "Uploads file from the current folder to a document library in the specified site",
                "m365 file add --filePath file.pdf --folderUrl \"https://contoso.sharepoint.com/sites/Contoso/Shared Documents\" --siteUrl \"https://contoso.sharepoint.com/sites/Contoso\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "file convert pdf",
        summary: "Converts the specified file to PDF using Microsoft Graph",
        options: &[
            (
                "sourceFile",
                "Local path or full URL to the source file to be converted to PDF",
            ),
            (
                "targetFile",
                "Local path or full URL to where the generated PDF file should be saved",
            ),
        ],
        notes: &[
            "To convert a local file to PDF, CLI will temporarily upload the file to a document library. If you're using app-only authentication, CLI will upload the file to the default document library in the root site collection on your tenant. If you're authenticated as a user, CLI will upload the file to the default document library in your OneDrive for Business. After the conversion, CLI will remove the uploaded file.",
            "If you choose to store the converted file in a document library, CLI will store the converted PDF file temporarily in the temp folder on your computer. After the PDF file has been uploaded to the location you specified, CLI will remove the temporary file.",
            "If the conversion process fails, CLI will attempt to clean up the temporary files (the source file uploaded to your tenant, if you're converting a local file to PDF, and the converted PDF file on your disk if you chose to upload the converted file to your tenant). If removing the temporary files fails, you will need to clean them up yourself. CLI will list the URL and/or path of the files to remove.",
        ],
        permissions: &[
            (
                "Delegated",
                "Microsoft Graph",
                "Files.ReadWrite, Sites.Read.All",
            ),
            (
                "Application",
                "Microsoft Graph",
                "Files.ReadWrite.All, Sites.Read.All",
            ),
        ],
        examples: &[
            (
                "Converts local file to PDF and stores the converted file on the disk",
                "m365 file convert pdf --sourceFile file.docx --targetFile file.pdf",
            ),
            (
                "Converts local file to PDF and uploads the converted file to a document library",
                "m365 file convert pdf --sourceFile file.docx --targetFile \"https://contoso.sharepoint.com/Shared Documents/file.pdf\"",
            ),
            (
                "Converts file from a document library to PDF and saves the converted file to disk",
                "m365 file convert pdf --sourceFile \"https://contoso.sharepoint.com/Shared Documents/file.docx\" --targetFile file.pdf",
            ),
            (
                "Converts file from a document library to PDF and saves the converted file to the same document library",
                "m365 file convert pdf --sourceFile \"https://contoso.sharepoint.com/Shared Documents/file.docx\" --targetFile \"https://contoso.sharepoint.com/Shared Documents/file.pdf\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "file copy",
        summary: "Copies a file to another location using the Microsoft Graph",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("sourceUrl", "Server-relative or absolute URL of the file."),
            (
                "targetUrl",
                "Server-relative or absolute URL of the location.",
            ),
            ("newName", "New name of the destination file."),
            (
                "nameConflictBehavior",
                "Behavior when a document with the same name is already present at the destination. Possible values: `fail`, `replace`, `rename`. Default is `fail`.",
            ),
        ],
        notes: &[],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Sites.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Copy a file by server-relative URL to a document library in another site collection with server relative URL",
                "m365 file copy --webUrl https://contoso.sharepoint.com/sites/project --sourceUrl \"/sites/project/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\"",
            ),
            (
                "Copy a file by absolute URL to a document library in another site collection with absolute URL",
                "m365 file copy --webUrl https://contoso.sharepoint.com --sourceUrl \"https://contoso.sharepoint.com/Shared Documents/Document.pdf\" --targetUrl \"https://contoso.sharepoint.com/sites/IT/Shared Documents\"",
            ),
            (
                "Copy a file to a document library in another site collection and rename the file",
                "m365 file copy --webUrl https://contoso.sharepoint.com --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\" --newName \"newName\"",
            ),
            (
                "Copy a file to a document library in another site collection and rename the file if a file with the same name already exists.",
                "m365 file copy --webUrl https://contoso.sharepoint.com --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\" --nameConflictBehavior rename",
            ),
            (
                "Copy a file to Onedrive for business",
                "m365 file copy --webUrl https://contoso.sharepoint.com --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"https://contoso-my.sharepoint.com/personal/john_contoso_onmicrosoft_com/documents\"",
            ),
        ],
        links: &[],
    },
    Help {
        name: "file list",
        summary: "Retrieves files from the specified folder and site",
        options: &[
            (
                "webUrl",
                "The URL of the site where the folder from which to retrieve files is located",
            ),
            (
                "folderUrl",
                "The server- or site-relative URL of the folder from which to retrieve files",
            ),
            ("recursive", "Set to retrieve files from subfolders"),
        ],
        notes: &[
            "This command is an improved version of the `spo file list` command. The main difference between the two commands is, that `file list` uses Microsoft Graph and properly supports retrieving files from large folders. Because `file list` uses Microsoft Graph and `spo file list` uses SharePoint REST APIs, the data returned by both commands is different.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Files.Read, Sites.Read.All"),
            (
                "Application",
                "Microsoft Graph",
                "Files.Read.All, Sites.Read.All",
            ),
        ],
        examples: &[
            (
                "Return all files from the folder Shared Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents'",
            ),
            (
                "Return all files from the folder Shared Documents and all the sub-folders of Shared Documents located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents' --recursive",
            ),
            (
                "Return all files from the Important folder in the Shared Documents document library located in site https://contoso.sharepoint.com/sites/project-x",
                "m365 file list --webUrl https://contoso.sharepoint.com/sites/project-x --folderUrl 'Shared Documents/Important'",
            ),
            (
                "",
                "[\n{\n\"@microsoft.graph.downloadUrl\": \"https://contoso.sharepoint.com/_layouts/15/download.aspx?UniqueId=9bdc8c49-644a-4875-aaa3-82914fb23226&Translate=false&tempauth=&ApiVersion=2.0\",\n\"createdDateTime\": \"2024-02-14T22:41:41Z\",\n\"eTag\": \"\\\"{9BDC8C49-644A-4875-AAA3-82914FB23226},1\\\"\",\n\"id\": \"013TMHP6SJRTOJWSTEOVEKVI4CSFH3EMRG\",\n\"lastModifiedDateTime\": \"2024-02-14T22:41:41Z\",\n\"name\": \"Document.docx\",\n\"webUrl\": \"https://contoso.sharepoint.com/_layouts/15/Doc.aspx?sourcedoc=%7B9BDC8C49-644A-4875-AAA3-82914FB23226%7D&file=Document.Docx&action=default&mobileredirect=true\",\n\"cTag\": \"\\\"c:{9BDC8C49-644A-4875-AAA3-82914FB23226},2\\\"\",\n\"size\": 41554,\n\"createdBy\": {\n\"user\": {\n\"email\": \"john@contoso.com\",\n\"id\": \"fe36f75e-c103-410b-a18a-2bf6df06ac3a\",\n\"displayName\": \"John Doe\"\n}\n},\n\"lastModifiedBy\": {\n\"user\": {\n\"email\": \"john@contoso.com\",\n\"id\": \"fe36f75e-c103-410b-a18a-2bf6df06ac3a\",\n\"displayName\": \"John Doe\"\n}\n},\n\"parentReference\": {\n\"driveType\": \"documentLibrary\",\n\"driveId\": \"b!T4-bE44OMEa7ClAUQtubZID9tc40pGJKpguycvELod_YogR_WYHeSaTWzIZ6qMyU\",\n\"id\": \"013TMHP6V6Y2GOVW7725BZO354PWSELRRZ\",\n\"name\": \"TEST\",\n\"path\": \"/drives/b!T4-bE44OMEa7ClAUQtubZID9tc40pGJKpguycvELod_YogR_WYHeSaTWzIZ6qMyU/root:\",\n\"siteId\": \"0f9b8f4f-0e8e-4630-bb0a-501442db9b64\"\n},\n\"file\": {\n\"mimeType\": \"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\",\n\"hashes\": {\n\"quickXorHash\": \"MX9jeQT/AfhDDMpYygXlCPYI/kc=\"\n}\n},\n\"fileSystemInfo\": {\n\"createdDateTime\": \"2024-02-14T22:41:41Z\",\n\"lastModifiedDateTime\": \"2024-02-14T22:41:41Z\"\n},\n\"shared\": {\n\"scope\": \"users\"\n},\n\"lastModifiedByUser\": \"John Doe\"\n}\n]",
            ),
            (
                "",
                "lastModifiedByUser: John Doe\nname              : Document.docx",
            ),
            (
                "",
                "@microsoft.graph.downloadUrl,createdDateTime,eTag,id,lastModifiedDateTime,name,webUrl,cTag,size,lastModifiedByUser\nhttps://contoso.sharepoint.com/_layouts/15/download.aspx?UniqueId=9bdc8c49-644a-4875-aaa3-82914fb23226&Translate=false&tempauth=&ApiVersion=2.0,2024-02-14T22:41:41Z,\"\"\"{9BDC8C49-644A-4875-AAA3-82914FB23226},1\"\"\",013TMHP6SJRTOJWSTEOVEKVI4CSFH3EMRG,2024-02-14T22:41:41Z,Thailand.xlsx,https://contoso.sharepoint.com/_layouts/15/Doc.aspx?sourcedoc=%7B9BDC8C49-644A-4875-AAA3-82914FB23226%7D&file=Document.docx&action=default&mobileredirect=true,\"\"\"c:{9BDC8C49-644A-4875-AAA3-82914FB23226},2\"\"\",41554,John Doe",
            ),
            (
                "",
                "# file list --webUrl \"https://contoso.sharepoint.com\" --folderUrl \"Test\"\nDate: 14/02/2024\n## Document.docx (013TMHP6SJRTOJWSTEOVEKVI4CSFH3EMRG)\nProperty | Value\n---------|-------\n@microsoft.graph.downloadUrl | https://contoso.sharepoint.com/\\_layouts/15/download.aspx?UniqueId=9bdc8c49-644a-4875-aaa3-82914fb23226&Translate=false&tempauth=&ApiVersion=2.0\ncreatedDateTime | 2024-02-14T22:41:41Z\neTag | \"{9BDC8C49-644A-4875-AAA3-82914FB23226},1\"\nid | 013TMHP6SJRTOJWSTEOVEKVI4CSFH3EMRG\nlastModifiedDateTime | 2024-02-14T22:41:41Z\nname | Document.docx\nwebUrl | https://contoso.sharepoint.com/\\_layouts/15/Doc.aspx?sourcedoc=%7B9BDC8C49-644A-4875-AAA3-82914FB23226%7D&file=Thailand.xlsx&action=default&mobileredirect=true\ncTag | \"c:{9BDC8C49-644A-4875-AAA3-82914FB23226},2\"\nsize | 41554\nlastModifiedByUser | John Doe",
            ),
        ],
        links: &[],
    },
    Help {
        name: "file move",
        summary: "Moves a file to another location using the Microsoft Graph",
        options: &[
            ("webUrl", "The URL of the site where the file is located."),
            ("sourceUrl", "Server-relative or absolute URL of the file."),
            (
                "targetUrl",
                "Server-relative or absolute URL of the location.",
            ),
            ("newName", "New name of the destination file."),
            (
                "nameConflictBehavior",
                "Behavior when a document with the same name is already present at the destination. Possible values: `fail`, `replace`, `rename`. Default is `fail`.",
            ),
        ],
        notes: &[
            "- If the source and target locations are within the same document library or drive, the command will utilize the Move DriveItem API, preserving the version history of the file.",
            "- If the source and target locations are in different document libraries or drives, the command will use a copy-and-delete combination to move the file. Please note that in this case, version history will not be retained.",
        ],
        permissions: &[
            ("Delegated", "Microsoft Graph", "Sites.ReadWrite.All"),
            ("Application", "Microsoft Graph", "Sites.ReadWrite.All"),
        ],
        examples: &[
            (
                "Move a file by server-relative URL to a folder in the same document library",
                "m365 file move --webUrl \"https://contoso.sharepoint.com/sites/project\" --sourceUrl \"/sites/project/Shared Documents/Document.pdf\" --targetUrl \"/sites/project/Shared Documents/NewFolder\"",
            ),
            (
                "Move a file by server-relative URL to a document library in another site collection with server relative URL",
                "m365 file move --webUrl \"https://contoso.sharepoint.com/sites/project\" --sourceUrl \"/sites/project/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\"",
            ),
            (
                "Move a file by absolute URL to a document library in another site collection with absolute URL",
                "m365 file move --webUrl \"https://contoso.sharepoint.com\" --sourceUrl \"https://contoso.sharepoint.com/Shared Documents/Document.pdf\" --targetUrl \"https://contoso.sharepoint.com/sites/IT/Shared Documents\"",
            ),
            (
                "Move a file to a document library in another site collection and rename the file",
                "m365 file move --webUrl \"https://contoso.sharepoint.com\" --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\" --newName \"newName\"",
            ),
            (
                "Move a file to a document library in another site collection and rename the file if a file with the same name already exists.",
                "m365 file move --webUrl \"https://contoso.sharepoint.com\" --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"/sites/IT/Shared Documents\" --nameConflictBehavior rename",
            ),
            (
                "Move a file to Onedrive for business",
                "m365 file move --webUrl \"https://contoso.sharepoint.com\" --sourceUrl \"/Shared Documents/Document.pdf\" --targetUrl \"https://contoso-my.sharepoint.com/personal/john_contoso_onmicrosoft_com/documents\"",
            ),
        ],
        links: &[],
    },
];

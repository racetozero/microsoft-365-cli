//! The help text of every command, one file per suite. Edit these files to
//! change `m365 <command> --help`; `scripts/gen-help.mjs` only bootstraps
//! them from a new upstream release.

mod adaptivecard;
mod app;
mod booking;
mod cli;
mod connection;
mod context;
mod entra;
mod exo;
mod external;
mod file;
mod flow;
mod general;
mod graph;
mod onedrive;
mod onenote;
mod outlook;
mod pa;
mod planner;
mod pp;
mod purview;
mod spe;
mod spfx;
mod spo;
mod spp;
mod teams;
mod tenant;
mod todo;
mod util;
mod viva;

/// The help of one command.
pub struct Help {
    /// The command name, for example `entra user get`.
    pub name: &'static str,
    /// One sentence that says what the command does.
    pub summary: &'static str,
    /// (long option name without dashes, description).
    pub options: &'static [(&'static str, &'static str)],
    /// Paragraphs shown under Notes.
    pub notes: &'static [&'static str],
    /// (kind such as Delegated or Application, resource, permissions).
    pub permissions: &'static [(&'static str, &'static str, &'static str)],
    /// (caption, command line).
    pub examples: &'static [(&'static str, &'static str)],
    /// (text, address).
    pub links: &'static [(&'static str, &'static str)],
}

/// Every suite's table.
pub static ALL: &[&[Help]] = &[
    adaptivecard::HELP,
    app::HELP,
    booking::HELP,
    cli::HELP,
    connection::HELP,
    context::HELP,
    entra::HELP,
    exo::HELP,
    external::HELP,
    file::HELP,
    flow::HELP,
    general::HELP,
    graph::HELP,
    onedrive::HELP,
    onenote::HELP,
    outlook::HELP,
    pa::HELP,
    planner::HELP,
    pp::HELP,
    purview::HELP,
    spe::HELP,
    spfx::HELP,
    spo::HELP,
    spp::HELP,
    teams::HELP,
    tenant::HELP,
    todo::HELP,
    util::HELP,
    viva::HELP,
];

/// The help of a command by its name (not an alias).
pub fn find(name: &str) -> Option<&'static Help> {
    ALL.iter().flat_map(|s| s.iter()).find(|h| h.name == name)
}

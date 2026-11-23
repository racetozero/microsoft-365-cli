//! Command tables: `generated/manifest.rs`, written by `crates/m365-codegen`.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptType {
    String,
    Boolean,
    Number,
    Unknown,
}

#[derive(Debug)]
pub struct OptMeta {
    pub name: &'static str,
    pub long: Option<&'static str>,
    pub short: Option<&'static str>,
    pub ty: OptType,
    pub required: bool,
    pub autocomplete: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FType {
    String,
    Boolean,
    Number,
    Enum,
    Array,
    Union,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub enum Lit {
    Bool(bool),
    Str(&'static str),
    Num(f64),
}

/// The message of a check: zod's default, a fixed template (`{input}` is
/// the rejected value) or one computed by the command.
#[derive(Clone, Copy, Debug)]
pub enum Msg {
    Default,
    Text(&'static str),
    Dynamic,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Validator {
    Guid,
    GuidArray,
    UserPrincipalName,
    UserPrincipalNameArray,
    SharePointUrl,
    ISODateTime,
    ISODuration,
    ISODate,
    ISODateDashOnly,
    GraphDateTime,
    PowerPagesUrl,
    PositiveInteger,
    PositiveIntegerArray,
    JsonOrEmpty,
    Theme,
    TeamsChannelId,
    TeamsChatId,
    Permission,
    MailNickname,
}

#[derive(Clone, Copy, Debug)]
pub enum Check {
    MinLen(u32, Msg),
    MaxLen(u32, Msg),
    Gt(f64, bool, Msg),
    Lt(f64, bool, Msg),
    SafeInt(Msg),
    Uuid(Msg),
    Guid(Msg),
    Regex(&'static str, Msg),
    /// A well-known validator; the flag is true when the source compares the
    /// result with `=== true`.
    Pred(Validator, bool, Msg),
    /// A command-specific predicate, implemented by the command's
    /// `field_check` hook. The index counts custom checks per command; the
    /// flag marks async refinements, whose issues zod reports last.
    Custom(u16, &'static str, Msg, bool),
    /// A predicate compiled to an expression tree (JSON, see `expr.rs`);
    /// the index is the one the custom check would have.
    Expr(u16, &'static str, Msg),
    Overwrite(&'static str),
    /// Comma-separated items, each one of the values (a piped enum array).
    ItemsEnum(&'static [&'static str]),
}

#[derive(Debug)]
pub struct FieldMeta {
    pub name: &'static str,
    pub t: FType,
    pub optional: bool,
    pub default: Option<Lit>,
    pub values: &'static [&'static str],
    pub ci: bool,
    pub coerce: bool,
    pub type_msg: Msg,
    pub transform: Option<&'static str>,
    pub union: &'static [FType],
    pub checks: &'static [Check],
}

#[derive(Clone, Copy, Debug)]
pub enum Rule {
    ExactlyOne(&'static [&'static str]),
    AtMostOne(&'static [&'static str]),
    AtLeastOne(&'static [&'static str]),
    AtLeastOneTruthy(&'static [&'static str]),
    ExactlyOneTruthy(&'static [&'static str]),
    AtMostOneTruthy(&'static [&'static str]),
    NotAllTruthy(&'static [&'static str]),
    /// `!o.a || o.b`
    Implies(&'static [&'static str]),
    /// A predicate compiled to an expression tree (JSON, see `expr.rs`).
    Expr(&'static str),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefineCode {
    None,
    OptionSet,
    Required,
}

#[derive(Debug)]
pub struct Refine {
    pub rule: Option<Rule>,
    pub msg: Msg,
    pub path: &'static [&'static str],
    pub code: RefineCode,
    pub set_options: &'static [&'static str],
    pub src: &'static str,
}

#[derive(Debug)]
pub struct OptionSet {
    pub options: &'static [&'static str],
    pub runs_when: Option<&'static str>,
}

#[derive(Debug)]
pub struct Legacy {
    /// Number of command-specific validators (beyond the four built-in ones).
    pub custom_validators: u64,
    pub strings: &'static [&'static str],
    pub booleans: &'static [&'static str],
    pub option_sets: &'static [OptionSet],
}

pub mod base {
    pub const ANONYMOUS: u32 = 1 << 0;
    pub const SPO: u32 = 1 << 1;
    pub const GRAPH: u32 = 1 << 2;
    pub const GRAPH_DELEGATED: u32 = 1 << 3;
    pub const GRAPH_APPLICATION: u32 = 1 << 4;
    pub const POWER_PLATFORM: u32 = 1 << 5;
    pub const POWER_APPS: u32 = 1 << 6;
    pub const POWER_AUTOMATE: u32 = 1 << 7;
    pub const POWER_BI: u32 = 1 << 8;
    pub const VIVA_ENGAGE: u32 = 1 << 9;
    pub const PLANNER: u32 = 1 << 10;
    pub const O365_MGMT: u32 = 1 << 11;
    pub const APP: u32 = 1 << 12;
    pub const CONTEXT: u32 = 1 << 13;
    pub const PERIOD_REPORT: u32 = 1 << 14;
    pub const DATE_PERIOD_REPORT: u32 = 1 << 15;
    pub const BASE_PROJECT: u32 = 1 << 16;
    pub const SPO_APP_BASE: u32 = 1 << 17;
    pub const SPO_PROPERTY_BAG: u32 = 1 << 18;
}

/// The `handleRejected*` helper the command's `commandAction` uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrStyle {
    Json,
    /// `VivaEngageCommand.handleRejectedODataJsonPromise`.
    Viva,
    OData,
    Plain,
    None,
}

#[derive(Debug)]
pub struct CmdMeta {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
    pub help: &'static str,
    pub file: &'static str,
    pub bases: u32,
    pub options: &'static [OptMeta],
    pub default_properties: Option<&'static [&'static str]>,
    pub allow_unknown: bool,
    /// The command overrides `action` (login, logout, status, connection commands)
    /// and handles authentication itself.
    pub custom_action: bool,
    pub allowed_outputs: &'static [&'static str],
    pub resource: Option<&'static str>,
    /// The Graph report function of `PeriodBasedReport` commands.
    pub usage_endpoint: Option<&'static str>,
    pub url_options: &'static [&'static str],
    pub strict: bool,
    /// `z.looseObject`: unknown options pass through to the command.
    pub loose: bool,
    /// How the command translates failures it does not handle itself.
    pub errors: ErrStyle,
    pub fields: &'static [FieldMeta],
    pub refines: &'static [Refine],
    pub legacy: Option<Legacy>,
}

impl CmdMeta {
    pub fn is(&self, base: u32) -> bool {
        self.bases & base != 0
    }

    pub fn option(&self, name: &str) -> Option<&'static OptMeta> {
        self.options.iter().find(|o| o.name == name)
    }

    pub fn field(&self, name: &str) -> Option<&'static FieldMeta> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn allowed_outputs(&self) -> &'static [&'static str] {
        if self.allowed_outputs.is_empty() {
            &["csv", "json", "md", "text", "none"]
        } else {
            self.allowed_outputs
        }
    }
}

// the command tables, generated by `examples/gen_manifest.rs`
include!("generated/manifest.rs");

/// Finds a command by name or alias.
pub fn find(name: &str) -> Option<&'static CmdMeta> {
    NAME_INDEX
        .binary_search_by(|(n, _)| (*n).cmp(name))
        .ok()
        .map(|i| &COMMANDS[NAME_INDEX[i].1 as usize])
}

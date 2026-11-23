//! `m365`: the Rust port of the CLI for Microsoft 365.

use m365_core::Spec;

fn specs() -> &'static [Spec] {
    let mut v: Vec<Spec> = Vec::with_capacity(900);
    m365_cli::register(&mut v);
    m365_entra::register(&mut v);
    m365_spo::register(&mut v);
    m365_teams::register(&mut v);
    m365_office::register(&mut v);
    m365_pp::register(&mut v);
    m365_spfx::register(&mut v);
    // the usage report commands share one implementation
    let ported: std::collections::HashSet<&str> = v.iter().map(|s| s.name).collect();
    v.extend(
        m365_core::report::specs()
            .into_iter()
            .filter(|r| !ported.contains(r.name)),
    );
    Box::leak(v.into_boxed_slice())
}

/// omelette's protocol: `--completion` prints the shell script and
/// `--compgen <cword> <prev> <line>` prints replies.
fn completion(args: &[String]) -> Option<i32> {
    if args.first().is_some_and(|a| a == "--completion") {
        print!("{}", m365_cli::completion::shell_script());
        return Some(0);
    }
    let pos = args.iter().position(|a| a == "--compgen")?;
    let cword: usize = args.get(pos + 1).and_then(|s| s.parse().ok()).unwrap_or(1);
    let line = args.get(pos + 3).map(String::as_str).unwrap_or("");
    for r in m365_core::completion::replies(cword, line) {
        println!("{r}");
    }
    Some(0)
}

// Return from main instead of calling `process::exit`: on Windows that calls
// `ExitProcess`, which skips the atexit handler that writes PGO profiles.
fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = run(&args);
    std::process::ExitCode::from(code as u8)
}

fn run(args: &[String]) -> i32 {
    if let Some(code) = completion(args) {
        return code;
    }
    // `m365 skill` is the port's own command (see `m365_cli::skill`)
    let mut rest = args.to_vec();
    m365_core::agent::take_flag(&mut rest);
    if rest.first().is_some_and(|a| a == "skill") {
        return m365_cli::skill::run(&rest[1..]);
    }
    // `m365 --version` / `-V`, like atl (upstream printed the full help)
    if rest.len() == 1 && (rest[0] == "--version" || rest[0] == "-V") {
        println!("m365 {}", env!("CARGO_PKG_VERSION"));
        return 0;
    }
    if rest.first().is_some_and(|a| a == "update") {
        return m365_cli::update::run(&rest[1..]);
    }
    let check = m365_cli::update::start(args);
    let code = m365_core::runner::main(specs());
    m365_cli::update::finish(check);
    code
}

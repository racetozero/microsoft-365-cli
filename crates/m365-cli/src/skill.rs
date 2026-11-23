//! `m365 skill`: the agent skills, so an agent can drive `m365` from a shell.
//! Not an upstream command: `main` handles it before the command runner, so
//! upstream's help and command list stay unchanged.
//!
//! The skills live in `.agents/skills/<name>/SKILL.md` (where `npx skills
//! add` finds them); `crates/m365-cli/skills/<name>.md` holds the copies the
//! binary embeds, and a test keeps them equal.

use m365_core::agent::{self, Code};
use m365_core::json::Obj;
use serde_json::Value;

/// Every skill as (name, SKILL.md text). The first one is the core skill.
pub const SKILLS: [(&str, &str); 10] = [
    ("m365", include_str!("../skills/m365.md")),
    ("m365-admin", include_str!("../skills/m365-admin.md")),
    ("m365-entra", include_str!("../skills/m365-entra.md")),
    ("m365-files", include_str!("../skills/m365-files.md")),
    ("m365-outlook", include_str!("../skills/m365-outlook.md")),
    (
        "m365-power-platform",
        include_str!("../skills/m365-power-platform.md"),
    ),
    ("m365-spfx", include_str!("../skills/m365-spfx.md")),
    ("m365-spo", include_str!("../skills/m365-spo.md")),
    (
        "m365-spo-admin",
        include_str!("../skills/m365-spo-admin.md"),
    ),
    ("m365-teams", include_str!("../skills/m365-teams.md")),
];

fn description(text: &str) -> &str {
    text.lines()
        .find_map(|l| l.strip_prefix("description: "))
        .unwrap_or_default()
}

fn ok(data: Value) -> i32 {
    agent::print_ok_with(data);
    0
}

fn fail(message: String) -> i32 {
    if agent::enabled() {
        return agent::print_error(Code::Invalid, &Value::from(message), Obj::new());
    }
    eprintln!("Error: {message}");
    3
}

/// `m365 skill [NAME] [--list] [--install DIR] [--agent]`; returns the exit code.
pub fn run(args: &[String]) -> i32 {
    agent::set_command("skill");
    let mut name: Option<&str> = None;
    let mut list = false;
    let mut install: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--list" | "-l" => list = true,
            "--install" => {
                // an optional folder; `.claude/skills` like `atl skill --install`
                match args.get(i + 1).filter(|a| !a.starts_with('-')) {
                    Some(d) => {
                        install = Some(d);
                        i += 1;
                    }
                    None => install = Some(".claude/skills"),
                }
            }
            "--help" | "-h" => {
                println!(
                    "Usage: m365 skill [name] [--list] [--install [folder]] [--agent]\n\nPrints an agent skill (default: m365), lists them, or writes them all as <folder>/<name>/SKILL.md (default .claude/skills)."
                );
                return 0;
            }
            a if a.starts_with('-') => return fail(format!("unknown option `{a}`")),
            a => name = Some(a),
        }
        i += 1;
    }
    if list {
        let rows: Vec<Value> = SKILLS
            .iter()
            .map(|(n, t)| serde_json::json!({ "name": n, "description": description(t) }))
            .collect();
        if agent::enabled() {
            return ok(Value::Array(rows));
        }
        for (n, t) in SKILLS {
            println!("{n:<20} {}", description(t));
        }
        return 0;
    }
    if let Some(dir) = install {
        let mut installed = Vec::new();
        for (n, t) in SKILLS {
            let folder = std::path::Path::new(dir).join(n);
            let path = folder.join("SKILL.md");
            if let Err(e) = std::fs::create_dir_all(&folder).and_then(|_| std::fs::write(&path, t))
            {
                return fail(format!("cannot write {}: {e}", path.display()));
            }
            installed.push(Value::from(path.display().to_string()));
        }
        if agent::enabled() {
            return ok(serde_json::json!({ "installed": installed }));
        }
        println!("Installed {} skills in {dir}", SKILLS.len());
        return 0;
    }
    let name = name.unwrap_or("m365");
    let Some((_, text)) = SKILLS.iter().find(|(n, _)| *n == name) else {
        let names: Vec<&str> = SKILLS.iter().map(|(n, _)| *n).collect();
        return fail(format!(
            "there is no skill named `{name}`; use one of: {}",
            names.join(", ")
        ));
    };
    if agent::enabled() {
        return ok(serde_json::json!({ "name": name, "skill": text }));
    }
    // plain Markdown, so `m365 skill m365-spo > SKILL.md` works
    println!("{}", text.trim_end());
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The embedded copies equal `.agents/skills/<name>/SKILL.md`, and every
    /// skill there is embedded. `M365_UPDATE_SKILLS=1` rewrites the copies.
    #[test]
    fn embedded_skills_match_agents_skills() {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let mut on_disk: Vec<String> = std::fs::read_dir(format!("{root}/.agents/skills"))
            .expect(".agents/skills")
            .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
            .collect();
        on_disk.sort();
        let embedded: Vec<&str> = SKILLS.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            on_disk, embedded,
            "add each skill in .agents/skills to SKILLS"
        );
        for (name, text) in SKILLS {
            let source = std::fs::read_to_string(format!("{root}/.agents/skills/{name}/SKILL.md"))
                .expect("SKILL.md");
            if std::env::var_os("M365_UPDATE_SKILLS").is_some() {
                std::fs::write(format!("{root}/crates/m365-cli/skills/{name}.md"), &source)
                    .expect("write");
                continue;
            }
            assert!(
                source == text,
                "crates/m365-cli/skills/{name}.md differs from .agents/skills/{name}/SKILL.md; run `M365_UPDATE_SKILLS=1 cargo test -p m365-cli skill`"
            );
            assert!(
                text.starts_with(&format!("---\nname: {name}\n")),
                "{name}: front matter names the skill"
            );
            assert!(
                !description(text).is_empty(),
                "{name}: front matter has a description"
            );
            // YAML reads `: ` inside a plain value as a mapping and skills tools skip the skill
            assert!(
                !description(text).contains(": "),
                "{name}: the description must not contain `: `"
            );
        }
    }

    /// Every `m365 ...` in the skills names a real command (or suite), and
    /// its `--options` exist (unless the command takes any option).
    #[test]
    fn skills_reference_real_commands_and_options() {
        let globals = ["agent", "query", "output", "debug", "verbose", "help"];
        let mut bad = Vec::new();
        for (skill, text) in SKILLS {
            for span in text
                .split('`')
                .skip(1)
                .step_by(2)
                .filter(|s| s.starts_with("m365 "))
            {
                let words: Vec<&str> = span.split_whitespace().skip(1).collect();
                if words
                    .first()
                    .is_some_and(|w| *w == "skill" || *w == "update" || w.starts_with('<'))
                {
                    continue;
                }
                let names: Vec<&str> = words
                    .iter()
                    .copied()
                    .take_while(|w| {
                        !w.starts_with('-') && !w.starts_with('[') && !w.starts_with('<')
                    })
                    .collect();
                let meta = (1..=names.len())
                    .rev()
                    .find_map(|n| m365_core::manifest::find(&names[..n].join(" ")).map(|m| (m, n)));
                let Some((meta, used)) = meta else {
                    // `m365 <suite> --help`
                    let suite = names.first().copied().unwrap_or_default();
                    let known = m365_core::manifest::COMMANDS
                        .iter()
                        .any(|c| c.name.split(' ').next() == Some(suite));
                    if !(known && names.len() == 1) {
                        bad.push(format!("{skill}: unknown command in `{span}`"));
                    }
                    continue;
                };
                if used != names.len() && !meta.allow_unknown && !meta.loose {
                    bad.push(format!(
                        "{skill}: `{span}` has extra words after `{}`",
                        meta.name
                    ));
                }
                for w in &words {
                    let Some(opt) = w.trim_start_matches('[').strip_prefix("--") else {
                        continue;
                    };
                    let opt = opt.split(['=', ']']).next().unwrap_or(opt);
                    if globals.contains(&opt) || meta.allow_unknown || meta.loose {
                        continue;
                    }
                    if meta.option(opt).is_none() {
                        bad.push(format!("{skill}: `{}` has no option --{opt}", meta.name));
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
}

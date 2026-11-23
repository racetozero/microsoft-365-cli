//! Shell completion (omelette protocol): the commands tree and replies.

use crate::json::Obj;
use crate::manifest::{COMMANDS, CmdMeta, UPSTREAM_ORDER};
use serde_json::Value;

/// The `commands.json` tree omelette and the PowerShell script complete from.
pub fn tree() -> Obj {
    build(&[], true)
}

/// The part of the tree that holds the node at `prefix`: only commands whose
/// words start with `prefix`. Unless `every_leaf`, only that node gets its
/// options, as replies list the keys of one node and never look below it.
fn build(prefix: &[&str], every_leaf: bool) -> Obj {
    let key = prefix.join(" ");
    let depth = if every_leaf {
        usize::MAX
    } else {
        prefix.len() + 1
    };
    let mut root = Obj::new();
    for &i in UPSTREAM_ORDER {
        let c = &COMMANDS[i as usize];
        for name in std::iter::once(c.name).chain(c.aliases.iter().copied()) {
            let below = name
                .strip_prefix(key.as_str())
                .is_some_and(|rest| key.is_empty() || rest.is_empty() || rest.starts_with(' '));
            if !below {
                continue;
            }
            let chunks: Vec<&str> = name.split(' ').collect();
            let mut parent = &mut root;
            for (i, ch) in chunks.iter().take(depth).enumerate() {
                let node = parent.entry(*ch).or_insert_with(|| {
                    Value::Object(
                        if i + 1 == chunks.len() && (every_leaf || i + 1 == prefix.len()) {
                            options(c)
                        } else {
                            Obj::new()
                        },
                    )
                });
                parent = match node {
                    Value::Object(o) => o,
                    _ => break,
                };
            }
        }
    }
    root
}

/// A command's leaf: each option name, with its autocomplete values.
fn options(c: &CmdMeta) -> Obj {
    let names = c
        .options
        .iter()
        .filter_map(|o| o.short)
        .chain(c.options.iter().filter_map(|o| o.long))
        .map(|o| {
            if o.chars().count() == 1 {
                format!("-{o}")
            } else {
                format!("--{o}")
            }
        })
        .chain(["--help".to_string(), "-h".to_string()]);
    let mut oo = Obj::new();
    for n in names {
        let bare = n.trim_start_matches('-');
        let opt = c
            .options
            .iter()
            .find(|o| o.long == Some(bare) || o.short == Some(bare))
            .filter(|o| !o.autocomplete.is_empty());
        let value = match opt {
            Some(o) => Value::Array(o.autocomplete.iter().map(|x| Value::from(*x)).collect()),
            None => Value::Object(Obj::new()),
        };
        oo.insert(n, value);
    }
    oo
}

fn walk<'a>(tree: &'a Value, words: &[&str]) -> Option<&'a Value> {
    words.iter().try_fold(tree, |cur, w| cur.get(*w))
}

/// `handleAutocomplete`: replies for the words typed so far (`line` without
/// the trailing partial word handling of omelette).
pub fn replies(fragment: usize, line: &str) -> Vec<String> {
    let mut all_words: Vec<&str> = Vec::new();
    let list: Vec<String> = if fragment == 1 {
        build(&[], false).keys().cloned().collect()
    } else {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut words_all: Vec<&str> = parts.iter().skip(1).copied().collect();
        if line.ends_with(char::is_whitespace) {
            words_all.push("");
        }
        words_all.pop();
        all_words = words_all.clone();
        let words: Vec<&str> = words_all
            .iter()
            .enumerate()
            .filter(|(i, e)| {
                if !e.starts_with('-') {
                    *i == 0 || !words_all[i - 1].starts_with('-')
                } else {
                    *i == words_all.len() - 1
                }
            })
            .map(|(_, e)| *e)
            .collect();
        // only the last word can be an option
        let ws: Vec<&str> = words
            .iter()
            .filter(|w| !w.starts_with('-'))
            .copied()
            .collect();
        let tree = Value::Object(build(&ws, false));
        let mut replies = walk(&tree, &words);
        if words.last().is_some_and(|w| w.starts_with('-')) && !replies.is_some_and(Value::is_array)
        {
            replies = walk(&tree, &ws);
        }
        match replies {
            Some(Value::Array(a)) => a.iter().map(crate::json::to_js_string).collect(),
            Some(Value::Object(o)) => o.keys().cloned().collect(),
            _ => Vec::new(),
        }
    };
    list.into_iter()
        .filter(|r| !r.starts_with('-') || !all_words.contains(&r.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `replies` as upstream computes it: a walk of the full tree.
    fn reference(tree: &Value, fragment: usize, line: &str) -> Vec<String> {
        if fragment == 1 {
            return tree.as_object().unwrap().keys().cloned().collect();
        }
        let mut all: Vec<&str> = line.split_whitespace().skip(1).collect();
        if line.ends_with(char::is_whitespace) {
            all.push("");
        }
        all.pop();
        let words: Vec<&str> = all
            .iter()
            .enumerate()
            .filter(|(i, e)| {
                if e.starts_with('-') {
                    *i == all.len() - 1
                } else {
                    *i == 0 || !all[i - 1].starts_with('-')
                }
            })
            .map(|(_, e)| *e)
            .collect();
        let mut node = walk(tree, &words);
        if words.last().is_some_and(|w| w.starts_with('-')) && !node.is_some_and(Value::is_array) {
            let ws: Vec<&str> = words
                .iter()
                .filter(|w| !w.starts_with('-'))
                .copied()
                .collect();
            node = walk(tree, &ws);
        }
        let list: Vec<String> = match node {
            Some(Value::Array(a)) => a.iter().map(crate::json::to_js_string).collect(),
            Some(Value::Object(o)) => o.keys().cloned().collect(),
            _ => Vec::new(),
        };
        list.into_iter()
            .filter(|r| !r.starts_with('-') || !all.contains(&r.as_str()))
            .collect()
    }

    #[test]
    fn replies_match_a_walk_of_the_full_tree() {
        let tree = Value::Object(tree());
        let mut lines =
            std::collections::BTreeSet::from([(1, "m365 ".to_string()), (2, "m365 ".to_string())]);
        for c in COMMANDS {
            for name in std::iter::once(c.name).chain(c.aliases.iter().copied()) {
                let words: Vec<&str> = name.split(' ').collect();
                for n in 1..=words.len() {
                    let typed = words[..n].join(" ");
                    lines.insert((n + 1, format!("m365 {typed} ")));
                    lines.insert((n, format!("m365 {typed}")));
                }
                let opts = c.options.iter().filter_map(|o| o.long.or(o.short));
                for o in opts.chain(["help", "nope"]) {
                    let n = words.len() + 2;
                    lines.insert((n, format!("m365 {name} --{o} ")));
                    lines.insert((n + 2, format!("m365 {name} --{o} x --")));
                }
            }
        }
        for (fragment, line) in &lines {
            assert_eq!(
                replies(*fragment, line),
                reference(&tree, *fragment, line),
                "replies for {fragment} {line:?}"
            );
        }
    }
}

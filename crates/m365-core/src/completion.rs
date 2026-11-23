//! Shell completion (omelette protocol): the commands tree and replies.

use crate::json::Obj;
use serde_json::Value;

/// The `commands.json` tree omelette and the PowerShell script complete from.
pub fn tree() -> Obj {
    let mut root = Obj::new();
    for &i in crate::manifest::UPSTREAM_ORDER {
        let c = &crate::manifest::COMMANDS[i as usize];
        for name in std::iter::once(c.name).chain(c.aliases.iter().copied()) {
            let chunks: Vec<&str> = name.split(' ').collect();
            let mut parent = &mut root;
            for (i, ch) in chunks.iter().enumerate() {
                if !parent.contains_key(*ch) {
                    if i < chunks.len() - 1 {
                        parent.insert(ch.to_string(), Value::Object(Obj::new()));
                    } else {
                        let mut opts: Vec<String> = c
                            .options
                            .iter()
                            .filter_map(|o| o.short)
                            .map(|s| s.to_string())
                            .collect();
                        opts.extend(
                            c.options
                                .iter()
                                .filter_map(|o| o.long)
                                .map(|s| s.to_string()),
                        );
                        let mut names: Vec<String> = opts
                            .iter()
                            .map(|o| {
                                if o.chars().count() == 1 {
                                    format!("-{o}")
                                } else {
                                    format!("--{o}")
                                }
                            })
                            .collect();
                        names.push("--help".into());
                        names.push("-h".into());
                        let mut oo = Obj::new();
                        for n in names {
                            let bare = n.trim_start_matches('-');
                            let opt = c
                                .options
                                .iter()
                                .find(|o| o.long == Some(bare) || o.short == Some(bare));
                            match opt.filter(|o| !o.autocomplete.is_empty()) {
                                Some(o) => oo.insert(
                                    n,
                                    Value::Array(
                                        o.autocomplete.iter().map(|x| Value::from(*x)).collect(),
                                    ),
                                ),
                                None => oo.insert(n, Value::Object(Obj::new())),
                            };
                        }
                        parent.insert(ch.to_string(), Value::Object(oo));
                    }
                }
                parent = match parent.get_mut(*ch) {
                    Some(Value::Object(o)) => o,
                    _ => break,
                };
            }
        }
    }
    root
}

/// `handleAutocomplete`: replies for the words typed so far (`line` without
/// the trailing partial word handling of omelette).
pub fn replies(fragment: usize, line: &str) -> Vec<String> {
    let tree = Value::Object(tree());
    let mut all_words: Vec<&str> = Vec::new();
    let mut replies: Value;
    if fragment == 1 {
        replies = Value::Array(
            tree.as_object()
                .unwrap()
                .keys()
                .map(|k| Value::from(k.as_str()))
                .collect(),
        );
    } else {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut words_all: Vec<&str> = parts.iter().skip(1).copied().collect();
        if line.ends_with(char::is_whitespace) {
            words_all.push("");
        }
        if !words_all.is_empty() {
            words_all.pop();
        }
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
        let walk = |ws: &[&str]| -> Option<Value> {
            let mut cur = &tree;
            for w in ws {
                cur = cur.get(*w)?;
            }
            Some(cur.clone())
        };
        replies = walk(&words).unwrap_or(Value::Null);
        if words.last().is_some_and(|w| w.starts_with('-')) && !replies.is_array() {
            let ws: Vec<&str> = words
                .iter()
                .filter(|w| !w.starts_with('-'))
                .copied()
                .collect();
            replies = walk(&ws)
                .map(|v| {
                    Value::Array(
                        v.as_object()
                            .map(|o| o.keys().map(|k| Value::from(k.as_str())).collect())
                            .unwrap_or_default(),
                    )
                })
                .unwrap_or(Value::Null);
        }
    }
    let list: Vec<String> = match replies {
        Value::Array(a) => a.iter().map(crate::json::to_js_string).collect(),
        Value::Object(o) => o.keys().cloned().collect(),
        _ => Vec::new(),
    };
    list.into_iter()
        .filter(|r| !r.starts_with('-') || !all_words.contains(&r.as_str()))
        .collect()
}

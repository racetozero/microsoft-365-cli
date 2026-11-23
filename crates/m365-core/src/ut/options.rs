//! `src/utils/optionsUtils.ts`: options that are not part of the command
//! schema (for commands that accept unknown options).

use crate::json::Obj;
use crate::manifest::CmdMeta;

/// `optionsUtils.getUnknownOptions(options, knownOptions)`.
pub fn unknown_options(opts: &Obj, meta: &CmdMeta) -> Obj {
    let mut out = opts.clone();
    out.shift_remove("_");
    for o in meta.options {
        if let Some(l) = o.long {
            out.shift_remove(l);
        }
        if let Some(s) = o.short
            && s.len() == 1
            && s.chars().all(|c| c.is_ascii_lowercase())
        {
            out.shift_remove(s);
        }
    }
    out
}

/// `optionsUtils.addUnknownOptionsToPayload`.
pub fn add_unknown_options_to_payload(payload: &mut Obj, unknown: &Obj) {
    for (k, v) in unknown {
        payload.insert(k.clone(), v.clone());
    }
}

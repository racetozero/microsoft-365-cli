//! Top-level, `cli`, `connection`, `context`, `util` and `setup` commands.

mod cli;
pub mod completion;
mod general;
pub mod skill;
pub mod update;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    use cli::*;
    use general::*;
    v.extend([
        spec!("version", version),
        spec!("docs", docs),
        spec!("status", status),
        spec!("logout", logout),
        spec!("login", login, refine = login_refine, field = login_field),
        spec!("request", request, refine = request_refine),
        spec!(
            "search",
            search,
            refine = search_refine,
            field = search_field
        ),
        spec!("setup", setup, refine = setup_refine),
        spec!("cli consent", consent),
        spec!("cli doctor", doctor),
        spec!("cli issue", issue),
        spec!("cli config get", config_get),
        spec!("cli config list", config_list),
        spec!("cli config reset", config_reset),
        spec!("cli config set", config_set, refine = config_set_refine),
        spec!("cli completion sh setup", completion_sh_setup),
        spec!("cli completion sh update", completion_sh_update),
        spec!("cli completion pwsh setup", completion_pwsh_setup),
        spec!("cli completion pwsh update", completion_pwsh_update),
        spec!("cli completion clink update", completion_clink_update),
        spec!("cli app add", app_add, refine = app_add_refine),
        spec!("cli app reconsent", app_reconsent),
        spec!("connection list", connection_list),
        spec!("connection remove", connection_remove),
        spec!(
            "connection set",
            connection_set,
            field = connection_set_field
        ),
        spec!("connection use", connection_use),
        spec!("context init", context_init),
        spec!("context option list", context_option_list),
        spec!("context option remove", context_option_remove),
        spec!("context option set", context_option_set),
        spec!("context remove", context_remove),
        spec!("util accesstoken get", accesstoken_get),
    ]);
}

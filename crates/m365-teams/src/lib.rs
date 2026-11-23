//! `teams *` commands.

mod app;
mod channel;
mod meeting;
mod message;
mod team;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("teams app install", app::install),
        spec!("teams app list", app::list),
        spec!("teams app publish", app::publish, field = app::file_field),
        spec!("teams app remove", app::remove),
        spec!("teams app uninstall", app::uninstall),
        spec!("teams app update", app::update, field = app::file_field),
        spec!(
            "teams cache remove",
            app::cache_remove,
            validate = app::cache_validate
        ),
        spec!("teams callrecord get", app::callrecord_get),
        spec!(
            "teams callrecord list",
            app::callrecord_list,
            field = app::callrecord_list_field,
            refine = app::callrecord_list_refine
        ),
        spec!(
            "teams channel add",
            channel::add,
            refine = channel::add_refine
        ),
        spec!("teams channel get", channel::get),
        spec!("teams channel list", channel::list),
        spec!("teams channel member add", channel::member_add),
        spec!("teams channel member list", channel::member_list),
        spec!("teams channel member remove", channel::member_remove),
        spec!("teams channel member set", channel::member_set),
        spec!("teams channel remove", channel::remove),
        spec!(
            "teams channel set",
            channel::set,
            field = channel::set_field
        ),
        spec!(
            "teams chat get",
            channel::chat_get,
            field = channel::emails_field
        ),
        spec!("teams chat list", channel::chat_list),
        spec!(
            "teams chat member add",
            channel::chat_member_add,
            refine = channel::chat_member_add_refine
        ),
        spec!("teams chat member list", channel::chat_member_list),
        spec!("teams chat member remove", channel::chat_member_remove),
        spec!(
            "teams chat message list",
            channel::chat_message_list,
            refine = channel::chat_message_list_refine
        ),
        spec!(
            "teams chat message send",
            channel::chat_message_send,
            field = channel::emails_field
        ),
        spec!("teams funsettings list", meeting::funsettings_list),
        spec!("teams funsettings set", meeting::funsettings_set),
        spec!("teams guestsettings list", meeting::guestsettings_list),
        spec!("teams guestsettings set", meeting::guestsettings_set),
        spec!("teams membersettings list", meeting::membersettings_list),
        spec!("teams membersettings set", meeting::membersettings_set),
        spec!(
            "teams messagingsettings list",
            meeting::messagingsettings_list
        ),
        spec!(
            "teams messagingsettings set",
            meeting::messagingsettings_set
        ),
        spec!(
            "teams meeting add",
            meeting::add,
            field = meeting::add_field,
            refine = meeting::add_refine
        ),
        spec!(
            "teams meeting attendancereport get",
            meeting::attendancereport_get
        ),
        spec!(
            "teams meeting attendancereport list",
            meeting::attendancereport_list
        ),
        spec!("teams meeting get", meeting::get),
        spec!(
            "teams meeting list",
            meeting::list,
            refine = meeting::list_refine
        ),
        spec!(
            "teams meeting transcript get",
            meeting::transcript_get,
            field = meeting::transcript_get_field
        ),
        spec!("teams meeting transcript list", meeting::transcript_list),
        spec!("teams message get", message::get),
        spec!(
            "teams message list",
            message::list,
            field = message::list_field
        ),
        spec!("teams message remove", message::remove),
        spec!("teams message reply list", message::reply_list),
        spec!("teams message restore", message::restore),
        spec!("teams message send", message::send),
        spec!(
            "teams report directroutingcalls",
            team::report_directroutingcalls,
            refine = team::calls_refine
        ),
        spec!(
            "teams report pstncalls",
            team::report_pstncalls,
            refine = team::calls_refine
        ),
        spec!("teams tab add", message::tab_add),
        spec!("teams tab get", message::tab_get),
        spec!("teams tab list", message::tab_list),
        spec!("teams tab remove", message::tab_remove),
        spec!("teams team add", team::add, refine = team::add_refine),
        spec!("teams team app list", team::app_list),
        spec!("teams team archive", team::archive),
        spec!("teams team clone", team::clone, field = team::clone_field),
        spec!("teams team get", team::get),
        spec!("teams team list", team::list, refine = team::list_refine),
        spec!("teams team remove", team::remove),
        spec!("teams team set", team::set),
        spec!("teams team unarchive", team::unarchive),
        spec!("teams user app add", message::user_app_add),
        spec!("teams user app list", message::user_app_list),
        spec!("teams user app remove", message::user_app_remove),
        spec!("teams user app upgrade", message::user_app_upgrade),
        spec!("teams user list", message::user_list),
    ]);
}

//! `spo *`: SharePoint Online commands.

mod app;
pub mod clientsidepages;
mod contenttype;
mod customaction;
mod defaultvalue;
mod extension;
mod file;
mod folder;
mod group;
mod hub;
mod list;
mod listitem;
mod listview;
mod misc;
mod misc2;
mod nav;
mod page;
mod site;
mod sitedesign;
#[allow(dead_code)]
mod sp;
mod tenant;
mod tenant1;
mod term;
mod themes;
mod web;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("spo web add", web::add, field = web::add_field),
        spec!(
            "spo web clientsidewebpart list",
            web::clientsidewebpart_list
        ),
        spec!("spo web get", web::get),
        spec!(
            "spo web installedlanguage list",
            web::installedlanguage_list
        ),
        spec!("spo web list", web::list),
        spec!("spo web reindex", web::reindex),
        spec!("spo web remove", web::remove),
        spec!("spo web retentionlabel list", web::retentionlabel_list),
        spec!("spo web roleassignment add", web::roleassignment_add),
        spec!("spo web roleassignment remove", web::roleassignment_remove),
        spec!("spo web roleinheritance break", web::roleinheritance_break),
        spec!("spo web roleinheritance reset", web::roleinheritance_reset),
        spec!("spo web rule get", web::rule_get),
        spec!("spo web rule list", web::rule_list),
        spec!("spo web rule remove", web::rule_remove),
        spec!("spo web set", web::set),
        spec!("spo propertybag get", misc::propertybag_get),
        spec!("spo propertybag list", misc::propertybag_list),
        spec!("spo propertybag remove", misc::propertybag_remove),
        spec!("spo propertybag set", misc::propertybag_set),
        spec!("spo storageentity get", misc::storageentity_get),
        spec!("spo storageentity list", misc::storageentity_list),
        spec!("spo storageentity remove", misc::storageentity_remove),
        spec!("spo storageentity set", misc::storageentity_set),
        spec!("spo cdn get", misc::cdn_get),
        spec!("spo cdn origin add", misc::cdn_origin_add),
        spec!("spo cdn origin list", misc::cdn_origin_list),
        spec!("spo cdn origin remove", misc::cdn_origin_remove),
        spec!("spo cdn policy list", misc::cdn_policy_list),
        spec!("spo cdn policy set", misc::cdn_policy_set),
        spec!("spo cdn set", misc::cdn_set),
        spec!(
            "spo theme apply",
            misc::theme_apply,
            refine = misc::theme_apply_refine
        ),
        spec!("spo theme get", misc::theme_get),
        spec!("spo theme list", misc::theme_list),
        spec!("spo theme remove", misc::theme_remove),
        spec!("spo theme set", misc::theme_set),
        spec!("spo page add", page::add),
        spec!(
            "spo page clientsidewebpart add",
            page::clientsidewebpart_add,
            field = page::clientsidewebpart_add_field
        ),
        spec!("spo page column get", page::column_get),
        spec!("spo page column list", page::column_list),
        spec!("spo page control get", page::control_get),
        spec!("spo page control list", page::control_list),
        spec!("spo page control remove", page::control_remove),
        spec!("spo page control set", page::control_set),
        spec!("spo page copy", page::copy),
        spec!("spo page get", page::get_cmd),
        spec!("spo page header set", page::header_set),
        spec!("spo page list", page::list),
        spec!("spo page publish", page::publish),
        spec!("spo page remove", page::remove),
        spec!("spo page section add", page::section_add),
        spec!("spo page section get", page::section_get),
        spec!("spo page section list", page::section_list),
        spec!("spo page section remove", page::section_remove),
        spec!("spo page set", page::set),
        spec!("spo page template list", page::template_list),
        spec!("spo page text add", page::text_add),
        spec!("spo page unpublish", page::unpublish),
        spec!(
            "spo tenant appcatalog add",
            tenant::appcatalog_add,
            field = tenant::number_field
        ),
        spec!("spo tenant appcatalogurl get", tenant::appcatalogurl_get),
        spec!(
            "spo tenant applicationcustomizer add",
            tenant::applicationcustomizer_add,
            field = tenant::json_field
        ),
        spec!(
            "spo tenant applicationcustomizer get",
            tenant::applicationcustomizer_get
        ),
        spec!(
            "spo tenant applicationcustomizer list",
            tenant::applicationcustomizer_list
        ),
        spec!(
            "spo tenant applicationcustomizer remove",
            tenant::applicationcustomizer_remove
        ),
        spec!(
            "spo tenant applicationcustomizer set",
            tenant::applicationcustomizer_set
        ),
        spec!("spo tenant commandset add", tenant::commandset_add),
        spec!("spo tenant commandset get", tenant::commandset_get),
        spec!("spo tenant commandset list", tenant::commandset_list),
        spec!("spo tenant commandset remove", tenant::commandset_remove),
        spec!("spo tenant commandset set", tenant::commandset_set),
        spec!(
            "spo tenant recyclebinitem list",
            tenant::recyclebinitem_list
        ),
        spec!(
            "spo tenant recyclebinitem remove",
            tenant::recyclebinitem_remove
        ),
        spec!(
            "spo tenant recyclebinitem restore",
            tenant::recyclebinitem_restore
        ),
        spec!("spo tenant settings list", tenant::settings_list),
        spec!(
            "spo tenant settings set",
            tenant::settings_set,
            refine = tenant::settings_set_refine,
            field = tenant::number_field
        ),
        spec!("spo tenant site get", tenant::site_get),
        spec!(
            "spo site ensure",
            site::ensure,
            validate = site::url_validate
        ),
        spec!(
            "spo site sharingpermission set",
            site::sharingpermission_set
        ),
        spec!(
            "spo site unarchive",
            site::unarchive,
            validate = site::url_validate
        ),
        spec!("spo site versionpolicy get", site::versionpolicy_get),
        spec!("spo site set", site::set, validate = site::set_validate),
        spec!(
            "spo site remove",
            site::remove,
            validate = site::remove_validate
        ),
        spec!(
            "spo site rename",
            site::rename,
            validate = site::rename_validate
        ),
        spec!("spo site hubsite connect", site::hubsite_connect),
        spec!("spo site hubsite disconnect", site::hubsite_disconnect),
        spec!(
            "spo site hubsite theme sync",
            site::hubsite_theme_sync,
            validate = site::web_url_validate
        ),
        spec!(
            "spo site inplacerecordsmanagement set",
            site::inplacerecordsmanagement_set,
            validate = site::site_url_validate
        ),
        spec!("spo site list", site::list),
        spec!(
            "spo site membership list",
            site::membership_list,
            validate = site::membership_validate
        ),
        spec!(
            "spo site recyclebinitem clear",
            site::recyclebinitem_clear,
            validate = site::site_url_validate
        ),
        spec!(
            "spo site recyclebinitem list",
            site::recyclebinitem_list,
            validate = site::recyclebinitem_list_validate
        ),
        spec!(
            "spo site recyclebinitem move",
            site::recyclebinitem_move,
            validate = site::recyclebinitem_move_validate
        ),
        spec!(
            "spo site recyclebinitem remove",
            site::recyclebinitem_remove,
            validate = site::recyclebinitem_remove_validate
        ),
        spec!(
            "spo site recyclebinitem restore",
            site::recyclebinitem_restore,
            validate = site::recyclebinitem_restore_validate
        ),
        spec!(
            "spo site archive",
            site::archive,
            validate = site::url_validate
        ),
        spec!(
            "spo site chrome set",
            site::chrome_set,
            validate = site::chrome_validate
        ),
        spec!(
            "spo site commsite enable",
            site::commsite_enable,
            validate = site::commsite_validate
        ),
        spec!("spo site get", site::get),
        spec!(
            "spo site groupify",
            site::groupify,
            validate = site::url_validate
        ),
        spec!(
            "spo site apppermission add",
            site::apppermission_add,
            validate = site::apppermission_add_validate
        ),
        spec!(
            "spo site apppermission get",
            site::apppermission_get,
            validate = site::site_url_validate
        ),
        spec!(
            "spo site apppermission list",
            site::apppermission_list,
            validate = site::apppermission_list_validate
        ),
        spec!(
            "spo site apppermission remove",
            site::apppermission_remove,
            validate = site::apppermission_remove_validate
        ),
        spec!(
            "spo site apppermission set",
            site::apppermission_set,
            validate = site::apppermission_add_validate
        ),
        spec!(
            "spo site admin add",
            site::admin_add,
            validate = site::admin_add_validate
        ),
        spec!(
            "spo site admin list",
            site::admin_list,
            validate = site::site_url_validate
        ),
        spec!(
            "spo site admin remove",
            site::admin_remove,
            validate = site::admin_remove_validate
        ),
        spec!(
            "spo site appcatalog add",
            site::appcatalog_add,
            validate = site::site_url_validate
        ),
        spec!("spo site appcatalog list", site::appcatalog_list),
        spec!(
            "spo site appcatalog remove",
            site::appcatalog_remove,
            validate = site::site_url_validate
        ),
        spec!(
            "spo site accessrequest setting set",
            site::accessrequest_setting_set
        ),
        spec!("spo site add", site::add, validate = site::add_validate),
        spec!("spo app add", app::add, field = app::add_field),
        spec!("spo app deploy", app::deploy),
        spec!("spo app get", app::get),
        spec!("spo app install", app::install),
        spec!("spo app instance list", app::instance_list),
        spec!("spo app list", app::list),
        spec!("spo app remove", app::remove),
        spec!("spo app retract", app::retract),
        spec!("spo app teamspackage download", app::teamspackage_download),
        spec!("spo app uninstall", app::uninstall),
        spec!("spo app upgrade", app::upgrade),
        spec!("spo file add", file::add),
        spec!("spo file archive", file::archive_cmd),
        spec!("spo file checkin", file::checkin),
        spec!("spo file checkout", file::checkout),
        spec!("spo file checkout undo", file::checkout_undo),
        spec!("spo file copy", file::copy),
        spec!("spo file get", file::get),
        spec!("spo file list", file::list),
        spec!("spo file move", file::move_cmd),
        spec!("spo file remove", file::remove),
        spec!("spo file rename", file::rename),
        spec!(
            "spo file retentionlabel ensure",
            file::retentionlabel_ensure
        ),
        spec!(
            "spo file retentionlabel remove",
            file::retentionlabel_remove
        ),
        spec!("spo file roleassignment add", file::roleassignment_add),
        spec!(
            "spo file roleassignment remove",
            file::roleassignment_remove
        ),
        spec!(
            "spo file roleinheritance break",
            file::roleinheritance_break
        ),
        spec!(
            "spo file roleinheritance reset",
            file::roleinheritance_reset
        ),
        spec!("spo file sharinginfo get", file::sharinginfo_get),
        spec!("spo file sharinglink add", file::sharinglink_add),
        spec!("spo file sharinglink clear", file::sharinglink_clear),
        spec!("spo file sharinglink get", file::sharinglink_get),
        spec!("spo file sharinglink list", file::sharinglink_list),
        spec!("spo file sharinglink remove", file::sharinglink_remove),
        spec!("spo file sharinglink set", file::sharinglink_set),
        spec!("spo file unarchive", file::unarchive),
        spec!("spo file version clear", file::version_clear),
        spec!("spo file version get", file::version_get),
        spec!("spo file version keep", file::version_keep),
        spec!("spo file version list", file::version_list),
        spec!("spo file version remove", file::version_remove),
        spec!("spo file version restore", file::version_restore),
        spec!("spo folder add", folder::add),
        spec!("spo folder archive", folder::archive_cmd),
        spec!("spo folder copy", folder::copy),
        spec!("spo folder get", folder::get),
        spec!("spo folder list", folder::list),
        spec!("spo folder move", folder::move_cmd),
        spec!("spo folder remove", folder::remove),
        spec!(
            "spo folder retentionlabel ensure",
            folder::retentionlabel_ensure
        ),
        spec!(
            "spo folder retentionlabel remove",
            folder::retentionlabel_remove
        ),
        spec!("spo folder roleassignment add", folder::roleassignment_add),
        spec!(
            "spo folder roleassignment remove",
            folder::roleassignment_remove
        ),
        spec!(
            "spo folder roleinheritance break",
            folder::roleinheritance_break
        ),
        spec!(
            "spo folder roleinheritance reset",
            folder::roleinheritance_reset
        ),
        spec!("spo folder set", folder::set),
        spec!("spo folder sharinglink add", folder::sharinglink_add),
        spec!("spo folder sharinglink clear", folder::sharinglink_clear),
        spec!("spo folder sharinglink get", folder::sharinglink_get),
        spec!("spo folder sharinglink list", folder::sharinglink_list),
        spec!("spo folder sharinglink remove", folder::sharinglink_remove),
        spec!("spo folder sharinglink set", folder::sharinglink_set),
        spec!("spo folder unarchive", folder::unarchive),
        spec!("spo list defaultvalue clear", defaultvalue::clear),
        spec!("spo list defaultvalue get", defaultvalue::get),
        spec!("spo list defaultvalue list", defaultvalue::list),
        spec!("spo list defaultvalue remove", defaultvalue::remove),
        spec!(
            "spo list defaultvalue set",
            defaultvalue::set,
            field = defaultvalue::set_field
        ),
        spec!("spo list add", list::add, validate = list::add_validate),
        spec!(
            "spo list contenttype add",
            list::contenttype_add,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list contenttype default set",
            list::contenttype_default_set,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list contenttype list",
            list::contenttype_list,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list contenttype remove",
            list::contenttype_remove,
            validate = list::list_id_validate
        ),
        spec!("spo list get", list::get, validate = list::id_validate),
        spec!("spo list list", list::list, validate = list::url_validate),
        spec!(
            "spo list remove",
            list::remove,
            validate = list::id_validate
        ),
        spec!(
            "spo list retentionlabel ensure",
            list::retentionlabel_ensure,
            validate = list::guid_then_url_validate
        ),
        spec!(
            "spo list retentionlabel get",
            list::retentionlabel_get,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list retentionlabel remove",
            list::retentionlabel_remove,
            validate = list::guid_then_url_validate
        ),
        spec!(
            "spo list roleassignment add",
            list::roleassignment_add,
            validate = list::roleassignment_add_validate
        ),
        spec!(
            "spo list roleassignment remove",
            list::roleassignment_remove,
            validate = list::roleassignment_remove_validate
        ),
        spec!(
            "spo list roleinheritance break",
            list::roleinheritance_break,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list roleinheritance reset",
            list::roleinheritance_reset,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list sensitivitylabel ensure",
            list::sensitivitylabel_ensure,
            validate = list::sensitivity_validate
        ),
        spec!(
            "spo list sensitivitylabel remove",
            list::sensitivitylabel_remove
        ),
        spec!("spo list set", list::set, refine = list::set_refine),
        spec!(
            "spo list sitescript get",
            listview::sitescript_get,
            validate = list::list_id_validate
        ),
        spec!("spo list view add", listview::view_add),
        spec!(
            "spo list view field add",
            listview::field_add,
            validate = listview::field_add_validate
        ),
        spec!(
            "spo list view field remove",
            listview::field_remove,
            validate = listview::field_remove_validate
        ),
        spec!(
            "spo list view field set",
            listview::field_set,
            validate = listview::field_set_validate
        ),
        spec!(
            "spo list view get",
            listview::view_get,
            validate = listview::view_get_validate
        ),
        spec!(
            "spo list view list",
            listview::view_list,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list view remove",
            listview::view_remove,
            validate = listview::view_remove_validate
        ),
        spec!(
            "spo list view set",
            listview::view_set,
            validate = listview::view_get_validate
        ),
        spec!(
            "spo list webhook add",
            listview::webhook_add,
            validate = listview::webhook_add_validate
        ),
        spec!(
            "spo list webhook get",
            listview::webhook_get,
            validate = listview::webhook_id_validate
        ),
        spec!(
            "spo list webhook list",
            listview::webhook_list,
            validate = list::list_id_validate
        ),
        spec!(
            "spo list webhook remove",
            listview::webhook_remove,
            validate = listview::webhook_id_validate
        ),
        spec!(
            "spo list webhook set",
            listview::webhook_set,
            validate = listview::webhook_set_validate
        ),
        spec!("spo listitem add", listitem::add),
        spec!("spo listitem attachment add", listitem::attachment_add),
        spec!("spo listitem attachment get", listitem::attachment_get),
        spec!("spo listitem attachment list", listitem::attachment_list),
        spec!(
            "spo listitem attachment remove",
            listitem::attachment_remove
        ),
        spec!("spo listitem attachment set", listitem::attachment_set),
        spec!("spo listitem batch add", listitem::batch_add),
        spec!(
            "spo listitem batch remove",
            listitem::batch_remove,
            field = listitem::batch_remove_field
        ),
        spec!("spo listitem batch set", listitem::batch_set),
        spec!("spo listitem get", listitem::get),
        spec!("spo listitem isrecord", listitem::isrecord),
        spec!("spo listitem list", listitem::list),
        spec!("spo listitem record declare", listitem::record_declare),
        spec!("spo listitem record lock", listitem::record_lock),
        spec!("spo listitem record undeclare", listitem::record_undeclare),
        spec!("spo listitem record unlock", listitem::record_unlock),
        spec!("spo listitem remove", listitem::remove),
        spec!(
            "spo listitem retentionlabel ensure",
            listitem::retentionlabel_ensure
        ),
        spec!(
            "spo listitem retentionlabel remove",
            listitem::retentionlabel_remove
        ),
        spec!(
            "spo listitem roleassignment add",
            listitem::roleassignment_add
        ),
        spec!(
            "spo listitem roleassignment remove",
            listitem::roleassignment_remove
        ),
        spec!(
            "spo listitem roleinheritance break",
            listitem::roleinheritance_break
        ),
        spec!(
            "spo listitem roleinheritance reset",
            listitem::roleinheritance_reset
        ),
        spec!("spo listitem set", listitem::set),
        spec!(
            "spo sitedesign add",
            sitedesign::add,
            field = sitedesign::scripts_field
        ),
        spec!("spo sitedesign apply", sitedesign::apply),
        spec!("spo sitedesign get", sitedesign::get),
        spec!("spo sitedesign list", sitedesign::list),
        spec!("spo sitedesign remove", sitedesign::remove),
        spec!("spo sitedesign rights grant", sitedesign::rights_grant),
        spec!("spo sitedesign rights list", sitedesign::rights_list),
        spec!("spo sitedesign rights revoke", sitedesign::rights_revoke),
        spec!("spo sitedesign run list", sitedesign::run_list),
        spec!("spo sitedesign run status get", sitedesign::run_status_get),
        spec!(
            "spo sitedesign set",
            sitedesign::set,
            field = sitedesign::scripts_field
        ),
        spec!("spo sitedesign task get", sitedesign::task_get),
        spec!("spo sitedesign task list", sitedesign::task_list),
        spec!("spo sitedesign task remove", sitedesign::task_remove),
        spec!("spo serviceprincipal grant add", hub::grant_add),
        spec!("spo serviceprincipal grant list", hub::grant_list),
        spec!("spo serviceprincipal grant revoke", hub::grant_revoke),
        spec!(
            "spo serviceprincipal permissionrequest approve",
            hub::permissionrequest_approve,
            refine = hub::approve_refine
        ),
        spec!(
            "spo serviceprincipal permissionrequest deny",
            hub::permissionrequest_deny
        ),
        spec!(
            "spo serviceprincipal permissionrequest list",
            hub::permissionrequest_list
        ),
        spec!("spo serviceprincipal set", hub::sp_set),
        spec!("spo hubsite connect", hub::connect),
        spec!("spo hubsite data get", hub::data_get),
        spec!("spo hubsite disconnect", hub::disconnect),
        spec!("spo hubsite get", hub::get),
        spec!("spo hubsite list", hub::list),
        spec!("spo hubsite register", hub::register),
        spec!("spo hubsite rights grant", hub::rights_grant),
        spec!("spo hubsite rights revoke", hub::rights_revoke),
        spec!("spo hubsite set", hub::set),
        spec!("spo hubsite unregister", hub::unregister),
        spec!("spo applicationcustomizer add", extension::app_add),
        spec!("spo applicationcustomizer get", extension::app_get),
        spec!("spo applicationcustomizer list", extension::app_list),
        spec!("spo applicationcustomizer remove", extension::app_remove),
        spec!("spo applicationcustomizer set", extension::app_set),
        spec!(
            "spo commandset add",
            extension::cmd_add,
            field = extension::guid_field
        ),
        spec!(
            "spo commandset get",
            extension::cmd_get,
            field = extension::guid_field
        ),
        spec!("spo commandset list", extension::cmd_list),
        spec!(
            "spo commandset remove",
            extension::cmd_remove,
            field = extension::guid_field
        ),
        spec!(
            "spo commandset set",
            extension::cmd_set,
            field = extension::guid_field
        ),
        spec!("spo externaluser list", misc2::externaluser_list),
        spec!("spo get", misc2::get),
        spec!("spo set", misc2::set),
        spec!("spo search", misc2::search),
        spec!("spo agent add", misc2::agent_add),
        spec!("spo apppage add", misc2::apppage_add),
        spec!("spo apppage set", misc2::apppage_set),
        spec!(
            "spo brandcenter colors list",
            misc2::brandcenter_colors_list
        ),
        spec!(
            "spo brandcenter settings list",
            misc2::brandcenter_settings_list
        ),
        spec!("spo contenttype add", contenttype::add),
        spec!("spo contenttype field list", contenttype::field_list),
        spec!("spo contenttype field remove", contenttype::field_remove),
        spec!("spo contenttype field set", contenttype::field_set),
        spec!("spo contenttype get", contenttype::get),
        spec!("spo contenttype list", contenttype::list),
        spec!("spo contenttype remove", contenttype::remove),
        spec!("spo contenttype set", contenttype::set),
        spec!("spo contenttype sync", contenttype::sync),
        spec!("spo contenttypehub get", contenttype::hub_get),
        spec!(
            "spo customaction add",
            customaction::add,
            field = customaction::rights_field
        ),
        spec!("spo customaction clear", customaction::clear),
        spec!("spo customaction get", customaction::get),
        spec!("spo customaction list", customaction::list),
        spec!("spo customaction remove", customaction::remove),
        spec!(
            "spo customaction set",
            customaction::set,
            field = customaction::rights_field
        ),
        spec!(
            "spo field add",
            customaction::field_add,
            field = customaction::field_add_field
        ),
        spec!("spo field get", customaction::field_get),
        spec!("spo field list", customaction::field_list),
        spec!("spo field remove", customaction::field_remove),
        spec!("spo field set", customaction::field_set),
        spec!("spo term add", term::add, refine = term::add_refine),
        spec!("spo term get", term::get),
        spec!("spo term group add", term::group_add),
        spec!("spo term group get", term::group_get),
        spec!("spo term group list", term::group_list),
        spec!("spo term list", term::list),
        spec!(
            "spo term set add",
            term::set_add,
            refine = term::set_add_refine
        ),
        spec!("spo term set get", term::set_get),
        spec!("spo term set list", term::set_list),
        spec!("spo sitescript add", term::sitescript_add),
        spec!("spo sitescript get", term::sitescript_get),
        spec!("spo sitescript list", term::sitescript_list),
        spec!("spo sitescript remove", term::sitescript_remove),
        spec!("spo sitescript set", term::sitescript_set),
        spec!("spo user ensure", group::user_ensure),
        spec!("spo user get", group::user_get),
        spec!("spo user list", group::user_list),
        spec!("spo user remove", group::user_remove),
        spec!("spo userprofile get", group::userprofile_get),
        spec!("spo userprofile set", group::userprofile_set),
        spec!("spo group add", group::group_add),
        spec!("spo group get", group::group_get),
        spec!("spo group list", group::group_list),
        spec!("spo group member add", group::group_member_add),
        spec!("spo group member list", group::group_member_list),
        spec!("spo group member remove", group::group_member_remove),
        spec!("spo group remove", group::group_remove),
        spec!("spo group set", group::group_set),
        spec!("spo navigation node add", nav::node_add),
        spec!("spo navigation node get", nav::node_get),
        spec!("spo navigation node list", nav::node_list),
        spec!("spo navigation node remove", nav::node_remove),
        spec!("spo navigation node set", nav::node_set),
        spec!("spo eventreceiver get", nav::eventreceiver_get),
        spec!("spo eventreceiver list", nav::eventreceiver_list),
        spec!("spo eventreceiver remove", nav::eventreceiver_remove),
        spec!("spo feature disable", nav::feature_disable),
        spec!("spo feature enable", nav::feature_enable),
        spec!("spo feature list", nav::feature_list),
        spec!("spo roledefinition get", nav::roledefinition_get),
        spec!("spo roledefinition list", nav::roledefinition_list),
        spec!("spo roledefinition remove", nav::roledefinition_remove),
        spec!(
            "spo roledefinition add",
            nav::roledefinition_add,
            refine = nav::roledefinition_add_refine
        ),
        spec!("spo homesite add", tenant1::homesite_add),
        spec!("spo homesite get", tenant1::homesite_get),
        spec!("spo homesite list", tenant1::homesite_list),
        spec!("spo homesite remove", tenant1::homesite_remove),
        spec!("spo homesite set", tenant1::homesite_set),
        spec!("spo hidedefaultthemes get", tenant1::hidedefaultthemes_get),
        spec!("spo hidedefaultthemes set", tenant1::hidedefaultthemes_set),
        spec!("spo orgassetslibrary add", tenant1::orgassetslibrary_add),
        spec!("spo orgassetslibrary list", tenant1::orgassetslibrary_list),
        spec!(
            "spo orgassetslibrary remove",
            tenant1::orgassetslibrary_remove
        ),
        spec!("spo orgnewssite list", tenant1::orgnewssite_list),
        spec!("spo orgnewssite remove", tenant1::orgnewssite_remove),
        spec!("spo orgnewssite set", tenant1::orgnewssite_set),
        spec!("spo knowledgehub get", tenant1::knowledgehub_get),
        spec!("spo knowledgehub remove", tenant1::knowledgehub_remove),
        spec!("spo knowledgehub set", tenant1::knowledgehub_set),
    ]);
}

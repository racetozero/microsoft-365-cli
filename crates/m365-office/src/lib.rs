//! Microsoft 365 workload commands: planner, todo, outlook, onedrive,
//! onenote, booking, file, viva, purview, spe, spp, tenant, external, graph,
//! exo, app and adaptivecard.

mod external;
mod file;
mod graph;
mod misc;
mod outlook;
mod planner;
mod purview;
mod spe;
mod spp;
mod template;
mod tenant;
mod todo;
mod viva;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("planner bucket add", planner::bucket_add),
        spec!("planner bucket get", planner::bucket_get),
        spec!("planner bucket list", planner::bucket_list),
        spec!("planner bucket remove", planner::bucket_remove),
        spec!("planner bucket set", planner::bucket_set),
        spec!("planner plan add", planner::plan_add),
        spec!("planner plan get", planner::plan_get),
        spec!("planner plan list", planner::plan_list),
        spec!("planner plan remove", planner::plan_remove),
        spec!(
            "planner plan set",
            planner::plan_set,
            refine = planner::plan_set_refine
        ),
        spec!("planner roster add", planner::roster_add),
        spec!("planner roster get", planner::roster_get),
        spec!("planner roster member add", planner::roster_member_add),
        spec!("planner roster member get", planner::roster_member_get),
        spec!("planner roster member list", planner::roster_member_list),
        spec!(
            "planner roster member remove",
            planner::roster_member_remove
        ),
        spec!("planner roster plan list", planner::roster_plan_list),
        spec!("planner roster remove", planner::roster_remove),
        spec!(
            "planner task add",
            planner::task_add,
            field = planner::task_add_field
        ),
        spec!(
            "planner task checklistitem add",
            planner::task_checklistitem_add
        ),
        spec!(
            "planner task checklistitem list",
            planner::task_checklistitem_list
        ),
        spec!(
            "planner task checklistitem remove",
            planner::task_checklistitem_remove
        ),
        spec!("planner task get", planner::task_get),
        spec!("planner task list", planner::task_list),
        spec!("planner task reference add", planner::task_reference_add),
        spec!("planner task reference list", planner::task_reference_list),
        spec!(
            "planner task reference remove",
            planner::task_reference_remove
        ),
        spec!("planner task remove", planner::task_remove),
        spec!(
            "planner task set",
            planner::task_set,
            field = planner::task_set_field
        ),
        spec!(
            "planner tenant settings list",
            planner::tenant_settings_list
        ),
        spec!("planner tenant settings set", planner::tenant_settings_set),
        spec!("todo list add", todo::list_add),
        spec!("todo list get", todo::list_get),
        spec!("todo list list", todo::list_list),
        spec!("todo list remove", todo::list_remove),
        spec!("todo list set", todo::list_set),
        spec!(
            "todo task add",
            todo::task_add,
            validate = todo::task_add_validate
        ),
        spec!("todo task get", todo::task_get),
        spec!("todo task list", todo::task_list),
        spec!("todo task remove", todo::task_remove),
        spec!(
            "todo task set",
            todo::task_set,
            validate = todo::task_set_validate
        ),
        spec!("onenote notebook add", misc::notebook_add),
        spec!("onenote notebook list", misc::notebook_list),
        spec!("onenote page list", misc::page_list),
        spec!("booking business get", misc::business_get),
        spec!("booking business list", misc::business_list),
        spec!("file add", file::add),
        spec!("file convert pdf", file::convert_pdf),
        spec!("file copy", file::copy),
        spec!("file list", file::list),
        spec!("file move", file::r#move),
        spec!("outlook calendar add", outlook::calendar_add),
        spec!("outlook calendar get", outlook::calendar_get),
        spec!("outlook calendar list", outlook::calendar_list),
        spec!("outlook calendar remove", outlook::calendar_remove),
        spec!("outlook calendar set", outlook::calendar_set),
        spec!("outlook calendargroup add", outlook::calendargroup_add),
        spec!("outlook calendargroup get", outlook::calendargroup_get),
        spec!("outlook calendargroup list", outlook::calendargroup_list),
        spec!(
            "outlook calendargroup remove",
            outlook::calendargroup_remove
        ),
        spec!("outlook calendargroup set", outlook::calendargroup_set),
        spec!(
            "outlook event add",
            outlook::event_add,
            field = outlook::event_add_field
        ),
        spec!("outlook event cancel", outlook::event_cancel),
        spec!("outlook event get", outlook::event_get),
        spec!("outlook event list", outlook::event_list),
        spec!("outlook event remove", outlook::event_remove),
        spec!(
            "outlook mail searchfolder add",
            outlook::mail_searchfolder_add
        ),
        spec!(
            "outlook mail send",
            outlook::mail_send,
            refine = outlook::mail_send_refine
        ),
        spec!(
            "outlook mailbox settings get",
            outlook::mailbox_settings_get
        ),
        spec!(
            "outlook mailbox settings set",
            outlook::mailbox_settings_set
        ),
        spec!("outlook message get", outlook::message_get),
        spec!("outlook message list", outlook::message_list),
        spec!("outlook message move", outlook::message_move),
        spec!("outlook message remove", outlook::message_remove),
        spec!("outlook room list", outlook::room_list),
        spec!("outlook roomlist list", outlook::roomlist_list),
        spec!("onedrive list", outlook::onedrive_list),
        spec!(
            "external connection add",
            external::connection_add,
            field = external::connection_add_field
        ),
        spec!("external connection doctor", external::connection_doctor),
        spec!("external connection get", external::connection_get),
        spec!("external connection list", external::connection_list),
        spec!("external connection remove", external::connection_remove),
        spec!(
            "external connection schema add",
            external::connection_schema_add
        ),
        spec!(
            "external connection urltoitemresolver add",
            external::connection_urltoitemresolver_add
        ),
        spec!("external item add", external::item_add),
        spec!(
            "exo approleassignment add",
            external::exo_approleassignment_add
        ),
        spec!("app get", external::app_get),
        spec!("app open", external::app_open),
        spec!("app permission add", external::app_permission_add),
        spec!("app permission list", external::app_permission_list),
        spec!("adaptivecard send", external::adaptivecard_send),
        spec!(
            "graph changelog list",
            graph::changelog_list,
            refine = graph::changelog_refine
        ),
        spec!(
            "graph directoryextension add",
            graph::directoryextension_add
        ),
        spec!(
            "graph directoryextension get",
            graph::directoryextension_get
        ),
        spec!(
            "graph directoryextension list",
            graph::directoryextension_list
        ),
        spec!(
            "graph directoryextension remove",
            graph::directoryextension_remove
        ),
        spec!("graph openextension add", graph::openextension_add),
        spec!("graph openextension get", graph::openextension_get),
        spec!("graph openextension list", graph::openextension_list),
        spec!("graph openextension remove", graph::openextension_remove),
        spec!("graph openextension set", graph::openextension_set),
        spec!(
            "graph schemaextension add",
            graph::schemaextension_add,
            refine = graph::schemaextension_add_refine
        ),
        spec!("graph schemaextension get", graph::schemaextension_get),
        spec!("graph schemaextension list", graph::schemaextension_list),
        spec!(
            "graph schemaextension remove",
            graph::schemaextension_remove
        ),
        spec!(
            "graph schemaextension set",
            graph::schemaextension_set,
            refine = graph::schemaextension_set_refine
        ),
        spec!(
            "graph subscription add",
            graph::subscription_add,
            refine = graph::subscription_add_refine
        ),
        spec!("tenant id get", tenant::id_get),
        spec!(
            "tenant info get",
            tenant::info_get,
            refine = tenant::info_get_refine
        ),
        spec!(
            "tenant people profilecardproperty add",
            tenant::profilecard_add,
            refine = tenant::profilecard_add_refine
        ),
        spec!(
            "tenant people profilecardproperty get",
            tenant::profilecard_get
        ),
        spec!(
            "tenant people profilecardproperty list",
            tenant::profilecard_list
        ),
        spec!(
            "tenant people profilecardproperty remove",
            tenant::profilecard_remove
        ),
        spec!(
            "tenant people profilecardproperty set",
            tenant::profilecard_set,
            refine = tenant::profilecard_set_refine
        ),
        spec!("tenant people pronouns get", tenant::pronouns_get),
        spec!("tenant people pronouns set", tenant::pronouns_set),
        spec!(
            "tenant report office365activationcounts",
            tenant::report_office365activationcounts
        ),
        spec!(
            "tenant report office365activationsusercounts",
            tenant::report_office365activationsusercounts
        ),
        spec!(
            "tenant report office365activationsuserdetail",
            tenant::report_office365activationsuserdetail
        ),
        spec!("tenant report settings get", tenant::report_settings_get),
        spec!("tenant report settings set", tenant::report_settings_set),
        spec!("tenant security alerts list", tenant::security_alerts_list),
        spec!("tenant serviceannouncement health get", tenant::health_get),
        spec!(
            "tenant serviceannouncement health list",
            tenant::health_list
        ),
        spec!(
            "tenant serviceannouncement healthissue get",
            tenant::healthissue_get
        ),
        spec!(
            "tenant serviceannouncement healthissue list",
            tenant::healthissue_list
        ),
        spec!(
            "tenant serviceannouncement message get",
            tenant::message_get
        ),
        spec!(
            "tenant serviceannouncement message list",
            tenant::message_list
        ),
        spec!(
            "viva connections app create",
            viva::connections_app_create,
            field = viva::app_create_field,
            refine = viva::app_create_refine
        ),
        spec!(
            "viva engage community add",
            viva::community_add,
            refine = viva::community_add_refine
        ),
        spec!("viva engage community get", viva::community_get),
        spec!("viva engage community list", viva::community_list),
        spec!("viva engage community remove", viva::community_remove),
        spec!("viva engage community set", viva::community_set),
        spec!("viva engage community user add", viva::community_user_add),
        spec!("viva engage community user list", viva::community_user_list),
        spec!(
            "viva engage community user remove",
            viva::community_user_remove
        ),
        spec!("viva engage message add", viva::message_add),
        spec!("viva engage message get", viva::message_get),
        spec!("viva engage message like set", viva::message_like_set),
        spec!("viva engage message list", viva::message_list),
        spec!("viva engage message remove", viva::message_remove),
        spec!("viva engage network list", viva::network_list),
        spec!("viva engage role list", viva::role_list),
        spec!("viva engage role member add", viva::role_member_add),
        spec!("viva engage role member list", viva::role_member_list),
        spec!("viva engage role member remove", viva::role_member_remove),
        spec!("viva engage search", viva::search),
        spec!("viva engage user get", viva::user_get),
        spec!("viva engage user list", viva::user_list),
        spec!(
            "purview auditlog list",
            purview::auditlog_list,
            field = purview::auditlog_field
        ),
        spec!("purview retentionevent add", purview::retentionevent_add),
        spec!("purview retentionevent get", purview::retentionevent_get),
        spec!("purview retentionevent list", purview::retentionevent_list),
        spec!(
            "purview retentionevent remove",
            purview::retentionevent_remove
        ),
        spec!(
            "purview retentioneventtype add",
            purview::retentioneventtype_add
        ),
        spec!(
            "purview retentioneventtype get",
            purview::retentioneventtype_get
        ),
        spec!(
            "purview retentioneventtype list",
            purview::retentioneventtype_list
        ),
        spec!(
            "purview retentioneventtype remove",
            purview::retentioneventtype_remove
        ),
        spec!(
            "purview retentioneventtype set",
            purview::retentioneventtype_set
        ),
        spec!("purview retentionlabel add", purview::retentionlabel_add),
        spec!("purview retentionlabel get", purview::retentionlabel_get),
        spec!("purview retentionlabel list", purview::retentionlabel_list),
        spec!(
            "purview retentionlabel remove",
            purview::retentionlabel_remove
        ),
        spec!("purview retentionlabel set", purview::retentionlabel_set),
        spec!(
            "purview sensitivitylabel get",
            purview::sensitivitylabel_get
        ),
        spec!(
            "purview sensitivitylabel list",
            purview::sensitivitylabel_list
        ),
        spec!(
            "purview sensitivitylabel policysettings list",
            purview::sensitivitylabel_policysettings_list
        ),
        spec!(
            "purview threatassessment add",
            purview::threatassessment_add
        ),
        spec!(
            "purview threatassessment get",
            purview::threatassessment_get
        ),
        spec!(
            "purview threatassessment list",
            purview::threatassessment_list
        ),
        spec!("spe container activate", spe::container_activate),
        spec!("spe container add", spe::container_add),
        spec!("spe container get", spe::container_get),
        spec!("spe container list", spe::container_list),
        spec!(
            "spe container permission add",
            spe::container_permission_add
        ),
        spec!(
            "spe container permission list",
            spe::container_permission_list
        ),
        spec!(
            "spe container permission remove",
            spe::container_permission_remove
        ),
        spec!(
            "spe container permission set",
            spe::container_permission_set
        ),
        spec!(
            "spe container recyclebinitem list",
            spe::recyclebinitem_list
        ),
        spec!(
            "spe container recyclebinitem remove",
            spe::recyclebinitem_remove
        ),
        spec!(
            "spe container recyclebinitem restore",
            spe::recyclebinitem_restore
        ),
        spec!("spe container remove", spe::container_remove),
        spec!("spe container set", spe::container_set),
        spec!(
            "spe containertype add",
            spe::containertype_add,
            field = spe::containertype_add_field
        ),
        spec!("spe containertype get", spe::containertype_get),
        spec!("spe containertype list", spe::containertype_list),
        spec!("spe containertype remove", spe::containertype_remove),
        spec!("spp autofillcolumn set", spp::autofillcolumn_set),
        spec!("spp contentcenter list", spp::contentcenter_list),
        spec!("spp model apply", spp::model_apply),
        spec!("spp model get", spp::model_get),
        spec!("spp model list", spp::model_list),
        spec!("spp model remove", spp::model_remove),
    ]);
}

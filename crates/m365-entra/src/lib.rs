//! `entra *` commands.

mod administrativeunit;
mod agent;
mod app;
mod approleassignment;
mod group;
mod groupsetting;
mod m365group;
mod misc;
mod pim;
mod user;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("entra administrativeunit add", administrativeunit::add),
        spec!("entra administrativeunit get", administrativeunit::get),
        spec!("entra administrativeunit list", administrativeunit::list),
        spec!(
            "entra administrativeunit member add",
            administrativeunit::member_add
        ),
        spec!(
            "entra administrativeunit member get",
            administrativeunit::member_get
        ),
        spec!(
            "entra administrativeunit member list",
            administrativeunit::member_list
        ),
        spec!(
            "entra administrativeunit member remove",
            administrativeunit::member_remove
        ),
        spec!(
            "entra administrativeunit remove",
            administrativeunit::remove
        ),
        spec!(
            "entra administrativeunit roleassignment add",
            administrativeunit::roleassignment_add
        ),
        spec!("entra agent blueprint list", agent::blueprint_list),
        spec!(
            "entra agent blueprintprincipal list",
            agent::blueprintprincipal_list
        ),
        spec!("entra agent list", agent::list),
        spec!("entra app add", app::add, refine = app::add_refine),
        spec!("entra app get", app::get),
        spec!("entra app list", app::list),
        spec!("entra app permission add", app::permission_add),
        spec!("entra app permission list", app::permission_list),
        spec!("entra app permission remove", app::permission_remove),
        spec!("entra app remove", app::remove),
        spec!(
            "entra app role add",
            app::role_add,
            field = app::role_add_field
        ),
        spec!("entra app role list", app::role_list),
        spec!("entra app role remove", app::role_remove),
        spec!("entra app set", app::set, refine = app::set_refine),
        spec!("entra approleassignment add", approleassignment::add),
        spec!("entra approleassignment list", approleassignment::list),
        spec!("entra approleassignment remove", approleassignment::remove),
        spec!(
            "entra enterpriseapp add",
            approleassignment::enterpriseapp_add
        ),
        spec!(
            "entra enterpriseapp get",
            approleassignment::enterpriseapp_get
        ),
        spec!(
            "entra enterpriseapp list",
            approleassignment::enterpriseapp_list
        ),
        spec!(
            "entra enterpriseapp remove",
            approleassignment::enterpriseapp_remove
        ),
        spec!(
            "entra group add",
            group::add,
            refine = group::add_refine,
            field = group::mail_nickname_field
        ),
        spec!("entra group get", group::get),
        spec!("entra group list", group::list),
        spec!(
            "entra group member add",
            group::member_add,
            refine = group::member_add_refine
        ),
        spec!("entra group member list", group::member_list),
        spec!(
            "entra group member remove",
            group::member_remove,
            refine = group::member_remove_refine
        ),
        spec!("entra group member set", group::member_set),
        spec!("entra group remove", group::remove),
        spec!(
            "entra group set",
            group::set,
            field = group::mail_nickname_field
        ),
        spec!("entra license list", group::license_list),
        spec!("entra groupsetting add", groupsetting::add),
        spec!("entra groupsetting get", groupsetting::get),
        spec!("entra groupsetting list", groupsetting::list),
        spec!("entra groupsetting remove", groupsetting::remove),
        spec!("entra groupsetting set", groupsetting::set),
        spec!("entra groupsettingtemplate get", groupsetting::template_get),
        spec!(
            "entra groupsettingtemplate list",
            groupsetting::template_list
        ),
        spec!(
            "entra m365group add",
            m365group::add,
            field = m365group::add_field,
            refine = m365group::add_refine
        ),
        spec!(
            "entra m365group conversation list",
            m365group::conversation_list
        ),
        spec!(
            "entra m365group conversation post list",
            m365group::conversation_post_list
        ),
        spec!("entra m365group get", m365group::get),
        spec!("entra m365group list", m365group::list),
        spec!(
            "entra m365group recyclebinitem clear",
            m365group::recyclebinitem_clear
        ),
        spec!(
            "entra m365group recyclebinitem list",
            m365group::recyclebinitem_list
        ),
        spec!(
            "entra m365group recyclebinitem remove",
            m365group::recyclebinitem_remove
        ),
        spec!(
            "entra m365group recyclebinitem restore",
            m365group::recyclebinitem_restore
        ),
        spec!("entra m365group remove", m365group::remove),
        spec!("entra m365group renew", m365group::renew),
        spec!(
            "entra m365group set",
            m365group::set,
            field = m365group::set_field,
            refine = m365group::set_refine
        ),
        spec!("entra m365group teamify", m365group::teamify),
        spec!("entra m365group user add", m365group::user_add),
        spec!("entra m365group user list", m365group::user_list),
        spec!("entra m365group user remove", m365group::user_remove),
        spec!("entra m365group user set", m365group::user_set),
        spec!("entra multitenant add", misc::multitenant_add),
        spec!("entra multitenant get", misc::multitenant_get),
        spec!("entra multitenant remove", misc::multitenant_remove),
        spec!("entra multitenant set", misc::multitenant_set),
        spec!("entra oauth2grant add", misc::oauth2grant_add),
        spec!("entra oauth2grant list", misc::oauth2grant_list),
        spec!("entra oauth2grant remove", misc::oauth2grant_remove),
        spec!("entra oauth2grant set", misc::oauth2grant_set),
        spec!("entra organization list", misc::organization_list),
        spec!("entra organization set", misc::organization_set),
        spec!("entra policy list", misc::policy_list),
        spec!("entra resourcenamespace list", misc::resourcenamespace_list),
        spec!(
            "entra roleassignment add",
            misc::roleassignment_add,
            field = misc::roleassignment_add_field,
            refine = misc::roleassignment_add_refine
        ),
        spec!("entra rolepermission list", misc::rolepermission_list),
        spec!(
            "entra siteclassification disable",
            misc::siteclassification_disable
        ),
        spec!(
            "entra siteclassification enable",
            misc::siteclassification_enable
        ),
        spec!("entra siteclassification get", misc::siteclassification_get),
        spec!("entra siteclassification set", misc::siteclassification_set),
        spec!(
            "entra pim role assignment add",
            pim::assignment_add,
            refine = pim::assignment_add_refine
        ),
        spec!(
            "entra pim role assignment eligibility list",
            pim::assignment_eligibility_list
        ),
        spec!("entra pim role assignment list", pim::assignment_list),
        spec!("entra pim role assignment remove", pim::assignment_remove),
        spec!("entra pim role request list", pim::request_list),
        spec!("entra roledefinition add", pim::roledefinition_add),
        spec!(
            "entra roledefinition get",
            pim::roledefinition_get,
            refine = pim::roledefinition_refine
        ),
        spec!("entra roledefinition list", pim::roledefinition_list),
        spec!(
            "entra roledefinition remove",
            pim::roledefinition_remove,
            refine = pim::roledefinition_refine
        ),
        spec!(
            "entra roledefinition set",
            pim::roledefinition_set,
            refine = pim::roledefinition_refine
        ),
        spec!("entra user add", user::add),
        spec!("entra user get", user::get),
        spec!(
            "entra user groupmembership list",
            user::groupmembership_list
        ),
        spec!("entra user guest add", user::guest_add),
        spec!("entra user hibp", user::hibp),
        spec!(
            "entra user license add",
            user::license_add,
            field = user::license_ids_field
        ),
        spec!(
            "entra user license list",
            user::license_list,
            refine = user::license_list_refine
        ),
        spec!(
            "entra user license remove",
            user::license_remove,
            field = user::license_ids_field
        ),
        spec!("entra user list", user::list),
        spec!("entra user password validate", user::password_validate),
        spec!(
            "entra user recyclebinitem clear",
            user::recyclebinitem_clear
        ),
        spec!("entra user recyclebinitem list", user::recyclebinitem_list),
        spec!(
            "entra user recyclebinitem remove",
            user::recyclebinitem_remove
        ),
        spec!(
            "entra user recyclebinitem restore",
            user::recyclebinitem_restore
        ),
        spec!(
            "entra user registrationdetails list",
            user::registrationdetails_list,
            field = user::registrationdetails_field
        ),
        spec!("entra user remove", user::remove),
        spec!("entra user session revoke", user::session_revoke),
        spec!("entra user set", user::set, refine = user::set_refine),
        spec!("entra user signin list", user::signin_list),
    ]);
}

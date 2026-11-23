//! Power Platform commands: `pa` (Power Apps), `flow` (Power Automate) and `pp`.

mod flow;
mod pa;
mod pp;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("pa app consent set", pa::app_consent_set),
        spec!("pa app export", pa::app_export),
        spec!("pa app get", pa::app_get),
        spec!("pa app list", pa::app_list),
        spec!("pa app owner set", pa::app_owner_set),
        spec!("pa app permission ensure", pa::app_permission_ensure),
        spec!("pa app permission list", pa::app_permission_list),
        spec!("pa app permission remove", pa::app_permission_remove),
        spec!("pa app remove", pa::app_remove),
        spec!("pa connector export", pa::connector_export),
        spec!("pa connector list", pa::connector_list),
        spec!("pa environment get", pa::environment_get),
        spec!("pa environment list", pa::environment_list),
        spec!("flow disable", flow::disable),
        spec!("flow enable", flow::enable),
        spec!("flow environment get", flow::environment_get),
        spec!("flow environment list", flow::environment_list),
        spec!("flow export", flow::export),
        spec!("flow get", flow::get),
        spec!("flow list", flow::list),
        spec!("flow owner ensure", flow::owner_ensure),
        spec!("flow owner list", flow::owner_list),
        spec!("flow owner remove", flow::owner_remove),
        spec!("flow recyclebinitem list", flow::recyclebinitem_list),
        spec!("flow recyclebinitem restore", flow::recyclebinitem_restore),
        spec!("flow remove", flow::remove),
        spec!("flow run cancel", flow::run_cancel),
        spec!("flow run get", flow::run_get),
        spec!("flow run list", flow::run_list),
        spec!("flow run resubmit", flow::run_resubmit),
        spec!("pp aibuildermodel get", pp::aibuildermodel_get),
        spec!("pp aibuildermodel list", pp::aibuildermodel_list),
        spec!("pp aibuildermodel remove", pp::aibuildermodel_remove),
        spec!("pp copilot get", pp::copilot_get),
        spec!("pp copilot list", pp::copilot_list),
        spec!("pp copilot remove", pp::copilot_remove),
        spec!("pp dataverse table get", pp::dataverse_table_get),
        spec!("pp dataverse table list", pp::dataverse_table_list),
        spec!("pp dataverse table remove", pp::dataverse_table_remove),
        spec!("pp dataverse table row list", pp::dataverse_table_row_list),
        spec!(
            "pp dataverse table row remove",
            pp::dataverse_table_row_remove
        ),
        spec!("pp environment get", pp::environment_get),
        spec!("pp environment list", pp::environment_list),
        spec!("pp gateway get", pp::gateway_get),
        spec!("pp gateway list", pp::gateway_list),
        spec!("pp managementapp add", pp::managementapp_add),
        spec!("pp managementapp list", pp::managementapp_list),
        spec!("pp solution get", pp::solution_get),
        spec!("pp solution list", pp::solution_list),
        spec!("pp solution publish", pp::solution_publish),
        spec!("pp solution publisher add", pp::solution_publisher_add),
        spec!("pp solution publisher get", pp::solution_publisher_get),
        spec!("pp solution publisher list", pp::solution_publisher_list),
        spec!(
            "pp solution publisher remove",
            pp::solution_publisher_remove
        ),
        spec!("pp solution remove", pp::solution_remove),
        spec!("pp tenant settings list", pp::tenant_settings_list),
        spec!("pp tenant settings set", pp::tenant_settings_set),
        spec!("pp website get", pp::website_get),
        spec!("pp website remove", pp::website_remove),
        spec!("pp website webrole list", pp::website_webrole_list),
    ]);
}

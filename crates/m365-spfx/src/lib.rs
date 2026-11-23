//! `spfx` commands.
#![forbid(unsafe_code)]

mod custom;
mod doctor;
pub mod jsonast;
mod matrix;
mod package;
mod projcmds;
mod project;
mod report;
mod rules;
pub mod semver;
mod spfx_doctor;
mod tslex;
mod upgrade;
mod yaml;

use m365_core::{Spec, spec};

pub fn register(v: &mut Vec<Spec>) {
    v.extend([
        spec!("spfx doctor", spfx_doctor::run),
        spec!("spfx package generate", package::generate),
        spec!(
            "spfx project azuredevops pipeline add",
            projcmds::azuredevops_pipeline_add
        ),
        spec!("spfx project doctor", doctor::run),
        spec!("spfx project externalize", projcmds::externalize),
        spec!(
            "spfx project github workflow add",
            projcmds::github_workflow_add
        ),
        spec!(
            "spfx project permissions grant",
            projcmds::permissions_grant
        ),
        spec!("spfx project rename", projcmds::rename),
        spec!("spfx project upgrade", upgrade::run),
    ]);
}

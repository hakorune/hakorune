//! Pre-link provider admission projections.
//!
//! The mutable `BoxCallableRegistry` remains a compatibility/draft surface.
//! This child owns only the consuming TextScan admission used by the selected
//! AOT activation checkpoint.  It has no runtime lookup or executable address.

mod admitted_registry;
mod aot_admission;
mod call_metadata;
mod seal;

pub(crate) use aot_admission::PreparedAotExecutableAdmissionV1;
pub(crate) use admitted_registry::TextScanAdmittedRoleV1;
pub(crate) use call_metadata::{
    project_dynamic_v2_aot_call_metadata, DynamicV2AotCallMetadataProjectionV1,
};
pub(crate) use seal::{
    ProviderAdmissionRejectV1, ProviderAdmissionSealV1, TextScanAliasProjectionV1,
};

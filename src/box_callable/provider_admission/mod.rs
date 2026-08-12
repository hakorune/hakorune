//! Pre-link provider admission projections.
//!
//! The mutable `BoxCallableRegistry` remains a compatibility/draft surface.
//! This child owns only the consuming TextScan admission used by the selected
//! AOT activation checkpoint.  It has no runtime lookup or executable address.

mod admitted_registry;
mod aot_admission;
mod seal;

pub(crate) use aot_admission::PreparedAotExecutableAdmissionV1;
pub(crate) use seal::{
    ProviderAdmissionRejectV1, ProviderAdmissionSealV1, TextScanAliasProjectionV1,
};

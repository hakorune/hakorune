//! Canonical normal-module transaction boundary.
//!
//! The schema remains Builder-free. The Main transaction prepares every
//! fallible draft/verification step before one atomic candidate commit, then
//! exposes one consuming opaque publication terminal. Source-entry selection,
//! VM result decoding, process projection, and runner policy remain outside.

mod entry_target;
mod rejection;
mod schema;

pub(in crate::mir) use entry_target::{
    canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1,
};
pub(in crate::mir::builder) use rejection::NormalModuleTransactionSchemaErrorV1;
#[cfg(test)]
pub(in crate::mir::builder) use rejection::RejectedNormalModuleTransactionSchemaV1;
#[cfg(test)]
pub(in crate::mir::builder) use schema::NormalModuleDraftRoleV1;
pub(in crate::mir::builder) use schema::{
    NormalModuleDraftExpectationV1, NormalModuleEntryRelationV1, NormalModuleTransactionDraftV1,
    NormalModuleTransactionSchemaV1,
};

#[cfg(test)]
mod tests;

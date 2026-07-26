//! Passive schema for one future heterogeneous canonical normal-module batch.
//!
//! This module owns no function lowering, collector mutation, module
//! publication, entry execution, or process projection.

mod canonical_batch;
mod entry_target;
mod rejection;
mod schema;

pub(in crate::mir::builder) use canonical_batch::{
    NormalCanonicalModuleBatchErrorV1, NormalCanonicalModuleBatchV1,
    PreparedNormalCanonicalModuleBatchV1, RejectedNormalCanonicalModuleBatchV1,
};
pub(in crate::mir) use entry_target::{
    canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1,
};
pub(in crate::mir::builder) use rejection::{
    NormalModuleTransactionSchemaErrorV1, RejectedNormalModuleTransactionSchemaV1,
};
pub(in crate::mir::builder) use schema::{
    NormalModuleDraftExpectationV1, NormalModuleDraftRoleV1, NormalModuleEntryRelationV1,
    NormalModuleTransactionDraftV1, NormalModuleTransactionSchemaV1,
};

#[cfg(test)]
mod canonical_batch_tests;
#[cfg(test)]
mod tests;

//! Canonical normal-module transaction boundary.
//!
//! The schema remains Builder-free. The Main transaction prepares every
//! fallible draft/verification step before one atomic candidate commit, then
//! exposes one consuming opaque publication terminal. Source-entry selection,
//! VM result decoding, process projection, and runner policy remain outside.

mod callable_draft_prefix;
mod canonical_batch;
mod entry_target;
mod main_transaction;
mod physical_thunk;
mod rejection;
mod result_type;
mod schema;
mod source_draft;

pub(in crate::mir) use callable_draft_prefix::{
    ConsumedNormalHelperLoweringReceiptV1, NormalHelperDraftCorrespondenceErrorV1,
    NormalHelperDraftPrefixFailureV1, PreparedNormalHelperDraftPrefixV1,
    RejectedNormalHelperDraftPrefixV1, RetainedNormalHelperDraftPrefixV1,
    VerifiedNormalHelperDraftV1,
};
pub(in crate::mir::builder) use canonical_batch::{
    NormalCanonicalModuleBatchErrorV1, NormalCanonicalModuleBatchV1,
    PreparedNormalCanonicalModuleBatchV1, RejectedNormalCanonicalModuleBatchV1,
};
pub(in crate::mir) use entry_target::{
    canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1,
};
pub(in crate::mir) use main_transaction::{
    CompletedNormalMainModuleCandidateV1, NormalMainBatchCorrespondenceErrorV1,
    NormalMainModuleTransactionErrorV1, NormalMainModuleTransactionStageV1,
    PreparedNormalMainModuleTransactionV1, PublishedNormalMainInvocationV1,
    RejectedNormalMainModuleTransactionV1, RetainedNormalMainPreparedDraftsV1,
    RetainedNormalMainTransactionEvidenceV1,
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
mod main_transaction_tests;
#[cfg(test)]
mod tests;

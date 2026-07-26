//! Canonical normal-module transaction boundary.
//!
//! The schema remains Builder-free. The Main transaction prepares every
//! fallible draft/verification step before one atomic candidate commit, then
//! exposes one consuming opaque publication terminal. Source-entry selection,
//! VM result decoding, process projection, and runner policy remain outside.

mod callable_batch;
mod callable_commit;
mod callable_draft_prefix;
mod callable_main_physical;
mod canonical_batch;
mod entry_target;
mod main_transaction;
mod physical_thunk;
mod rejection;
mod result_type;
mod schema;
mod script_transaction;
mod source_draft;

#[cfg(test)]
pub(super) use callable_batch::reject_normal_callable_batch_for_test;
pub(in crate::mir) use callable_batch::{
    NormalCallableBatchErrorV1, PreparedNormalCallableBatchV1, RejectedNormalCallableBatchV1,
};
#[cfg(test)]
pub(super) use callable_commit::reject_normal_callable_commit_for_test;
pub(in crate::mir) use callable_commit::{
    CompletedNormalCallableCandidateV1, NormalCallableCommitErrorV1,
    PreparedNormalCallableCommitV1, RejectedNormalCallableCommitV1,
};
pub(in crate::mir) use callable_draft_prefix::{
    ConsumedNormalHelperLoweringReceiptV1, NormalHelperDraftCorrespondenceErrorV1,
    NormalHelperDraftPrefixFailureV1, PreparedNormalHelperDraftPrefixV1,
    RejectedNormalHelperDraftPrefixV1, RetainedNormalHelperDraftPrefixV1,
    VerifiedNormalHelperDraftV1,
};
#[cfg(test)]
pub(super) use callable_main_physical::{
    reject_normal_callable_main_physical_at_stage_for_test, NormalCallableMainPhysicalTestStageV1,
};
pub(in crate::mir) use callable_main_physical::{
    NormalCallableMainPhysicalStageV1, PreparedNormalCallableMainPhysicalV1,
    RejectedNormalCallableMainPhysicalV1,
};
pub(in crate::mir) use canonical_batch::{
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
pub(in crate::mir) use script_transaction::{
    CompletedNormalScriptModuleCandidateV1, NormalScriptModuleTransactionErrorV1,
    PreparedNormalScriptModuleTransactionV1, RejectedNormalScriptModuleTransactionV1,
};

#[cfg(test)]
mod callable_batch_tests;
#[cfg(test)]
mod callable_commit_tests;
#[cfg(test)]
mod callable_main_physical_tests;
#[cfg(test)]
mod canonical_batch_tests;
#[cfg(test)]
mod main_transaction_tests;
#[cfg(test)]
mod tests;

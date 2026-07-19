//! Disconnected exact-i64 result contracts for same-module static callables.

mod activation;
mod activation_error;
mod activation_source_gate;
mod call_proof;
mod call_row;
mod call_substitution;
mod caller_ledger;
mod caller_ledger_error;
mod disposition;
mod error;
mod expression_proof;
mod function_proof;
mod located_legacy;
mod located_legacy_error;
mod loop_claim_batch;
mod loop_claim_schedule;
mod requirements;
mod solver;

#[allow(unused_imports)]
pub(crate) use call_row::{VerifiedCallableResultCallSiteV1, VerifiedCallableResultEvidenceV1};
pub(crate) use disposition::{
    CallableResultUnavailableReasonV1, VerifiedCallableResultDispositionV1,
};
pub(crate) use error::CallableResultCatalogErrorV1;
pub(crate) use solver::VerifiedSameModuleCallableResultCatalogV1;

#[cfg(test)]
mod tests;
#[allow(unused_imports)]
pub(crate) use activation::{
    CallableResultActivationDispositionV1, VerifiedCallableResultActivationPlanV1,
    VerifiedCallableResultActivationRowsV1,
};
pub(crate) use activation_error::CallableResultActivationErrorV1;
#[allow(unused_imports)]
pub(crate) use activation_source_gate::{
    classify_activation_source_site_v1, CallableResultActivationSourceDecisionV1,
    CallableResultActivationUnselectedReasonV1,
};
#[allow(unused_imports)]
pub(crate) use caller_ledger::{
    CallableResultBodySuffixDecisionV1, ClaimedCallableResultActivationSiteV1,
    VerifiedCallableResultCallerLedgerV1, VerifiedCallableResultInactiveBodySuffixV1,
    VerifiedCallableResultInactiveBodyV1, VerifiedCallableResultInactivePrefixV1,
};
pub(crate) use caller_ledger_error::CallableResultCallerLedgerErrorV1;
#[allow(unused_imports)]
pub(crate) use located_legacy::{
    LegacyBodyInputV1, LegacyExprInputV1, LegacyStmtInputV1, LocatedLegacyBodySuffixV1,
    VerifiedCallableResultLegacySourceViewV1,
};
pub(crate) use located_legacy_error::CallableResultLegacyLocationErrorV1;
#[allow(unused_imports)]
pub(crate) use loop_claim_batch::{
    CallableResultLoopClaimBatchErrorV1, ClaimedCallableResultLoopBatchV1,
};
#[allow(unused_imports)]
pub(crate) use loop_claim_schedule::{
    CallableResultLoopClaimScheduleErrorV1, VerifiedCallableResultLoopClaimScheduleV1,
};
#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use tests::actual_parser_add_fixture;
#[cfg(test)]
pub(crate) use tests::generic_selected_activation_fixture;

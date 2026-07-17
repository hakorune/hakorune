//! Disconnected exact-i64 result contracts for same-module static callables.

mod activation;
mod activation_error;
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
pub(crate) use caller_ledger::{
    ClaimedCallableResultActivationSiteV1, VerifiedCallableResultCallerLedgerV1,
    VerifiedCallableResultInactivePrefixV1,
};
pub(crate) use caller_ledger_error::CallableResultCallerLedgerErrorV1;
#[allow(unused_imports)]
pub(crate) use located_legacy::{
    LegacyBodyInputV1, LegacyExprInputV1, LegacyStmtInputV1,
    VerifiedCallableResultLegacySourceViewV1,
};
pub(crate) use located_legacy_error::CallableResultLegacyLocationErrorV1;

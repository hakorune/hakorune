//! Disconnected exact-i64 result contracts for same-module static callables.

mod activation;
mod activation_error;
mod call_proof;
mod call_row;
mod call_substitution;
mod disposition;
mod error;
mod expression_proof;
mod function_proof;
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

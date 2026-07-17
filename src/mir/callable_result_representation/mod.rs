//! Disconnected exact-i64 result contracts for same-module static callables.

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

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
mod static_call_result_publication;
mod static_call_result_publication_owner;
mod static_exact_i64_requirement;

pub(crate) use call_row::{
    CallableResultEvidenceV1, OwnedCallableResultCallSiteV1, VerifiedCallableResultCallSiteV1,
    VerifiedCallableResultEvidenceV1,
};
pub(crate) use disposition::{
    CallableResultUnavailableReasonV1, CallableSourceValueClassV1, VerifiedCallableResultDispositionV1,
    VerifiedCallableResultRepresentationV1,
};
pub(crate) use error::CallableResultCatalogErrorV1;
pub(crate) use solver::VerifiedSameModuleCallableResultCatalogV1;
pub(crate) use static_call_result_publication::{
    VerifiedStaticCallResultPublicationDemandV1, VerifiedStaticCallResultPublicationHandoffV1,
};
pub(crate) use static_call_result_publication_owner::{
    StaticCallResultPublicationOwnerFinishErrorV1, StaticCallResultPublicationOwnerTakeErrorV1,
    StaticCallResultPublicationTakeV1, StaticCallResultTargetOnlyV1,
    VerifiedStaticCallResultPublicationOwnerV1,
};
pub(crate) use static_exact_i64_requirement::{
    project_static_exact_i64_requirement_v1, StaticExactI64RequirementErrorV1,
    VerifiedStaticExactI64RequirementV1,
};

#[cfg(test)]
mod tests;

//! Source-backed parameter contract authority for direct callable methods.
//!
//! One complete resolved callable batch lends exact declaration identity,
//! optional source type spelling, and resolver-owned parameter bindings. This
//! module classifies only explicit `i64` as `ExactTrivial`; an absent ordinary
//! spelling remains `OpaqueHandle`. HomeDemand is a one-way projection and is
//! not the contract authority.

mod issuer;
mod model;

pub(crate) use issuer::{issue_callable_parameter_contract_v1, CallableParameterContractIssueV1};
pub(crate) use model::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
    VerifiedCallableParameterContractCatalogV1, VerifiedCallableParameterContractDeclarationRefV1,
    VerifiedCallableParameterContractV1,
};

#[cfg(test)]
mod tests;

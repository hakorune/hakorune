//! Source-bound callable parameter demand authority.
//!
//! One already-resolved callable semantic batch lends the exact owner and
//! parameter bindings used to project one complete non-splittable demand
//! catalog. This module owns no resolver forest, receiver, result, Recipe,
//! CFG, MIR, or physical ABI meaning.

mod issuer;
mod model;

pub(crate) use issuer::{issue_callable_parameter_demands_v1, CallableParameterDemandIssueV1};
pub(crate) use model::{
    CallableParameterDeclarationModeV1, VerifiedCallableParameterDemandCatalogV1,
    VerifiedCallableParameterDemandDeclarationRefV1, VerifiedCallableParameterDemandV1,
};

#[cfg(test)]
mod tests;

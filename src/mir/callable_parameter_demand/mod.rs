//! Source-bound callable parameter demand authority.
//!
//! One parser transaction and one canonical resolver session produce one
//! complete non-splittable demand catalog. This module does not own receiver,
//! result, Recipe, CFG, MIR, or physical ABI meaning.

mod issuer;
mod model;

pub(crate) use issuer::{issue_callable_parameter_demands_v1, CallableParameterDemandIssueV1};
pub(crate) use model::{
    CallableParameterDeclarationModeV1, VerifiedCallableParameterDemandCatalogV1,
    VerifiedCallableParameterDemandDeclarationRefV1, VerifiedCallableParameterDemandV1,
};

#[cfg(test)]
mod tests;

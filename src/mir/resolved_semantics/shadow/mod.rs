//! SA1 shadow resolver and SA3 construction-local resolution core.
//!
//! This family may observe canonical syntax, but it must never construct the
//! canonical arena product or become a Planner/Lower input. SA3 may consume
//! its draft records only inside `resolver.rs`, immediately canonicalize them,
//! and seal the canonical product before publication.

mod expr;
mod ids;
mod path;
mod product;
mod resolver;
mod stmt;
mod vocabulary;

pub(super) use ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
pub(super) use product::{
    ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowControlExitV0, ShadowRegionKindV0,
    ShadowResolveErrorV0, ShadowResolvedFunctionV0, ShadowScopeKindV0,
};
use resolver::resolve_function_shadow_v0;
pub(super) use resolver::resolve_function_shadow_view_v0;

#[cfg(test)]
mod assignment_traversal_tests;
#[cfg(test)]
mod leaf_traversal_tests;
#[cfg(test)]
mod scope_container_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vocabulary_tests;

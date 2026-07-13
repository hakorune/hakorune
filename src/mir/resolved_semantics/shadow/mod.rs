//! Disconnected SA1 shadow resolver.
//!
//! This family may observe canonical syntax, but it must never construct the
//! canonical arena product or become a Planner/Lower input.

mod expr;
mod ids;
mod path;
mod product;
mod resolver;
mod stmt;
mod vocabulary;

use ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
use product::{
    ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowControlExitV0, ShadowRegionKindV0,
    ShadowResolveErrorV0, ShadowResolvedFunctionV0,
};
use resolver::resolve_function_shadow_v0;

#[cfg(test)]
mod tests;

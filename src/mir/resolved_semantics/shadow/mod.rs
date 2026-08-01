//! SA1 shadow resolver and SA3 construction-local resolution core.
//!
//! This family may observe canonical syntax, but it must never construct the
//! canonical arena product or become a Planner/Lower input. SA3 may consume
//! its draft records only inside `resolver.rs`, immediately canonicalize them,
//! and seal the canonical product before publication.

mod block_expr;
mod expr;
mod ids;
mod owner_boundary;
mod path;
mod product;
mod resolver;
mod root_traversal;
mod script_root_window;
mod stmt;
mod traversal_profile;
mod vocabulary;

pub(super) use ids::{ShadowBindingOrdinalV0, ShadowRegionIdV0, ShadowScopeIdV0};
pub(super) use owner_boundary::ShadowLambdaSyntaxV0;
pub(super) use product::{
    ShadowAssignmentTargetV0, ShadowBindingKindV0, ShadowControlExitV0, ShadowDirectCallUseV0,
    ShadowExitOriginV0, ShadowExitRecordV0, ShadowLexicalRefV0, ShadowRegionKindV0,
    ShadowResolvedFunctionV0, ShadowResolvedOwnerV0, ShadowScopeKindV0,
};
pub(in crate::mir) use product::{ShadowMethodCallObservationV0, ShadowMethodCallReceiverV0};
pub(in crate::mir) use product::{ShadowQualifiedReceiverDispositionV0, ShadowResolveErrorV0};
use resolver::resolve_function_shadow_v0;
pub(in crate::mir) use resolver::{
    observe_method_calls_shadow_view_v0, observe_qualified_receiver_shadow_view_v0,
};
pub(super) use resolver::{
    resolve_function_shadow_view_v0, resolve_owner_shadow_view_v0, resolve_script_shadow_view_v0,
};
pub(crate) use script_root_window::{
    ScriptDeferredBoundaryV1, ScriptDiagnosticBoundaryV1, ScriptRootBindingRebindAdmissionV1,
    ScriptRootDemandWindowSealErrorV1, ScriptRootIfControlAdmissionV1,
    ScriptRootQMarkPropagationAdmissionV1, ScriptRootResolvedDemandV1,
    ScriptRootReturnExitAdmissionV1, ScriptRootRuntimeDispositionV1,
    ScriptRootSemanticDispositionV1, ScriptTransferredBoundaryV1, ScriptTransparentBoundaryV1,
    VerifiedScriptRootDemandEntryV1, VerifiedScriptRootDemandWindowV1,
};

#[cfg(test)]
mod assignment_traversal_tests;
#[cfg(test)]
mod leaf_traversal_tests;
#[cfg(test)]
mod method_call_observation_tests;
#[cfg(test)]
mod scope_container_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod vocabulary_tests;

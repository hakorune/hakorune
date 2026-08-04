//! Portable fixed-shell If recipe contract.
//!
//! This row is intentionally disconnected from production lowering. It owns
//! only recipe-local wire identities, source-claim structure, structural
//! verification/normalization, and caller-zero logical JoinSig elaboration.
//! PHI and Builder physicalization belong to later rows.

mod error;
mod ids;
mod join_sig;
mod nested_join_sig;
mod nested_schema;
mod nested_verify;
mod normalize;
mod physical_input;
mod schema;
mod source_binding;
mod verify;

#[cfg(test)]
mod tests;

pub(crate) use error::IfRecipeRejectReasonV1;
pub(crate) use ids::{IfBindingKeyV1, IfBlockKeyV1, IfItemKeyV1, IfValueKeyV1};
pub(crate) use join_sig::{
    IfJoinEdgeRoleV1, IfJoinEdgeV1, IfJoinObligationV1, IfJoinPortV1, IfJoinSigElaboratorV1,
    IfJoinSigRejectReasonV1, IfJoinSigV1, IfJoinValueEdgeV1, VerifiedIfJoinSigV1,
};
pub(crate) use nested_join_sig::{
    NestedIfJoinCompositionRoleV1, NestedIfJoinCompositionV1, NestedIfJoinNodeSigV1,
    NestedIfJoinSigComposerV1, NestedIfJoinSigRejectReasonV1, NestedIfJoinSigV1,
    VerifiedNestedIfJoinSigV1,
};
pub(crate) use nested_schema::{
    NestedIfAssignmentV1, NestedIfBindingV1, NestedIfBindingKeyV1, NestedIfBinaryOpV1,
    NestedIfContinuationV1, NestedIfExprKindV1, NestedIfExprV1, NestedIfJoinRowV1,
    NestedIfNodeKeyV1, NestedIfNodeV1, NestedIfRecipeArtifactV1, NestedIfRecipeProfileV1,
    NestedIfRecipeProvenanceV1, NestedIfRecipeSourceBindingV1, NestedIfRecipeV1,
    NestedIfSourceClaimRoleV1, NestedIfSourceClaimV1, NestedIfSourcePathStepV1,
    NestedIfSourcePathV1, NestedIfValueClassV1, NestedIfValueKeyV1,
    NESTED_IF_RECIPE_SCHEMA_VERSION_V1,
};
pub(crate) use normalize::{IfRecipeDecodeErrorV1, IfRecipeNormalizerV1};
pub(crate) use physical_input::{IfPhysicalInputRejectReasonV1, VerifiedIfPhysicalInputV1};
pub(crate) use schema::{
    IfBinaryOpV1, IfBindingRoleV1, IfBlockRoleV1, IfCompareOpV1, IfContinuationV1,
    IfElseDispositionV1, IfJoinRowV1, IfOperationV1, IfRecipeArtifactV1, IfRecipeBindingV1,
    IfRecipeBlockV1, IfRecipeItemRowV1, IfRecipeProfileV1, IfRecipeProvenanceV1,
    IfRecipeSourceBindingV1, IfRecipeSourceOwnerV1, IfRecipeV1, IfRecipeValueV1,
    IfSourceClaimRoleV1, IfSourceClaimV1, IfSourcePathStepV1, IfSourcePathV1, IfValueClassV1,
    IF_RECIPE_SCHEMA_VERSION_V1,
};
pub(crate) use verify::{IfRecipeVerifierV1, VerifiedIfRecipeArtifactV1, VerifiedIfRecipeV1};
pub(crate) use nested_verify::{
    NestedIfRecipeRejectReasonV1, NestedIfRecipeVerifierV1, VerifiedNestedIfRecipeArtifactV1,
};

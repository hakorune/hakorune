//! Neutral selfhost-portable recursive Loop recipe contract.

mod direct_accum_producer;
mod error;
mod ids;
mod join_sig;
mod normalize;
pub(crate) mod route_id;
mod schema;
mod source_binding;
mod verify;

#[cfg(test)]
#[path = "direct_accum_producer_tests.rs"]
mod direct_accum_producer_tests;

#[cfg(test)]
mod tests;

// M2 is intentionally disconnected. Keep one stable facade for later producers
// without turning caller-zero exports into warning noise.
#[allow(unused_imports)]
pub(crate) use direct_accum_producer::{
    produce_direct_accum_recipe_v1, DirectAccumRecipeProducerRejectV1,
    VerifiedDirectAccumRecipeProductV1,
};
#[allow(unused_imports)]
pub(crate) use error::LoopRecipeRejectReasonV1;
#[allow(unused_imports)]
pub(crate) use ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
#[allow(unused_imports)]
pub(crate) use join_sig::{
    LoopJoinEdgeRoleV1, LoopJoinEdgeV1, LoopJoinLoopV1, LoopJoinPayloadV1, LoopJoinPortV1,
    LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopJoinSigV1, VerifiedLoopJoinSigV1,
};
#[allow(unused_imports)]
pub(crate) use normalize::{LoopRecipeDecodeErrorV1, LoopRecipeNormalizerV1};
#[allow(unused_imports)]
pub(crate) use schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1,
    LoopNodeSourceBindingV1, LoopNodeV1, LoopOperationV1, LoopRecipeArtifactV1,
    LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeExitV1,
    LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1,
    LoopRecipeSourceOwnerV1, LoopRecipeV1, LoopRecipeValueV1, LoopSourcePathStepV1,
    LoopSourcePathV1, LoopValueClassV1, LOOP_RECIPE_SCHEMA_VERSION_V1,
};
#[allow(unused_imports)]
pub(crate) use verify::{LoopRecipeVerifierV1, VerifiedLoopRecipeV1};

/// Test-only end-to-end seam. The structural source-claim capability remains
/// private even when sibling modules exercise artifact verification.
#[cfg(test)]
pub(crate) fn verify_artifact_for_test(
    artifact: LoopRecipeArtifactV1,
) -> Result<(), LoopRecipeRejectReasonV1> {
    verify::LoopRecipeVerifierV1::verify_artifact(artifact).map(drop)
}

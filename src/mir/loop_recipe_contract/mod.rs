//! Neutral selfhost-portable recursive Loop recipe contract.

mod continuation;
mod direct_accum_producer;
mod error;
mod ids;
mod input_source;
mod join_sig;
mod join_sig_branch;
mod loop_true_break_continue_producer;
mod normalize;
mod operation_carrier_demand;
mod operation_effect;
mod operation_physical_demand;
mod physical_input;
mod physical_layout;
mod producer_id;
pub(crate) mod route_id;
mod schema;
mod semantic_context;
mod source_binding;
mod source_bound_core;
mod verify;

#[cfg(test)]
#[path = "direct_accum_producer_tests.rs"]
mod direct_accum_producer_tests;

#[cfg(test)]
#[path = "nested_predicate_tests.rs"]
mod nested_predicate_tests;

#[cfg(test)]
#[path = "join_sig_branch_tests.rs"]
mod join_sig_branch_tests;

#[cfg(test)]
#[path = "loop_true_break_continue_producer_tests.rs"]
mod loop_true_break_continue_producer_tests;

#[cfg(test)]
#[path = "producer_id_migration_tests.rs"]
mod producer_id_migration_tests;

#[cfg(test)]
#[path = "join_sig_nested_shadow_tests.rs"]
mod join_sig_nested_shadow_tests;

#[cfg(test)]
#[path = "join_sig_after_binding_tests.rs"]
mod join_sig_after_binding_tests;

#[cfg(test)]
#[path = "source_bound_core_tests.rs"]
mod source_bound_core_tests;

#[cfg(test)]
mod generic_g0_demand;

#[cfg(test)]
mod generic_g0;

#[cfg(test)]
pub(crate) use direct_accum_producer_tests::direct_accum_product_for_test;

#[cfg(test)]
pub(crate) use generic_g0_demand::{
    issue_generic_g0_recipe_demand_v1, GenericG0RecipeDemandIssueV1, GenericG0RoleLeaseRejectV1,
    VerifiedGenericG0RoleLeaseV1, VerifiedGenericRecipeDemandG0,
};

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use generic_g0::{
    generic_operation_demand_parts_for_test, produce_generic_g0_recipe_v1,
    GenericG0RecipeProducerRejectV1, VerifiedGenericG0TailCapabilityV1,
    VerifiedGenericRecipeProductG0,
};

#[cfg(test)]
pub(crate) use source_bound_core::issue_source_bound_core_for_test;

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "operation_effect_tests.rs"]
mod operation_effect_tests;

#[cfg(test)]
mod operation_effect_parity;

#[cfg(test)]
#[path = "operation_physical_demand_tests.rs"]
mod operation_physical_demand_tests;

#[cfg(test)]
pub(crate) use operation_effect_parity::{
    issue_operation_effect_parity_receipt_v1, LoopOperationEffectParityReceiptV1,
    LoopOperationEffectParityRejectV1, LoopOperationEffectParitySideV1,
};

// M2 is intentionally disconnected. Keep one stable facade for later producers
// without turning caller-zero exports into warning noise.
#[allow(unused_imports)]
pub(crate) use continuation::VerifiedLoopContinuationContractV1;
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
pub(crate) use input_source::{
    issue_initialized_local_input_source_set_v1, LoopInitializedLocalInputSourceRelationV1,
    LoopInitializedLocalInputSourceSetRejectV1, VerifiedLoopInitializedLocalInputSourceSetV1,
};
#[allow(unused_imports)]
pub(crate) use join_sig::{
    LoopJoinBranchExitV1, LoopJoinBranchV1, LoopJoinEdgeRoleV1, LoopJoinEdgeV1, LoopJoinLoopV1,
    LoopJoinPayloadV1, LoopJoinPortBindingV1, LoopJoinPortV1, LoopJoinSigElaboratorV1,
    LoopJoinSigRejectReasonV1, LoopJoinSigV1, VerifiedLoopAfterBindingV1, VerifiedLoopJoinSigV1,
};
#[allow(unused_imports)]
pub(crate) use loop_true_break_continue_producer::{
    produce_loop_true_break_continue_recipe_v1, LoopTrueBreakContinueRecipeProducerRejectV1,
    VerifiedLoopTrueBreakContinueRecipeProductV1,
};
#[allow(unused_imports)]
pub(crate) use normalize::{LoopRecipeDecodeErrorV1, LoopRecipeNormalizerV1};
#[allow(unused_imports)]
pub(crate) use operation_carrier_demand::PreparedLoopDerivedCarrierSeedRowV1;
#[allow(unused_imports)]
pub(crate) use operation_effect::{
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationSourceEvidenceV1,
};
#[allow(unused_imports)]
pub(crate) use operation_physical_demand::{
    LoopOperationCoverageReceiptV1, LoopOperationPhysicalDemandRejectV1,
    PreparedLoopOperationProgramV1, PreparedLoopOperationRowV1, PreparedLoopOperationScheduleRowV1,
    PreparedLoopReadBindingRowV1, PreparedLoopWriteBindingRowV1,
    VerifiedLoopOperationPhysicalDemandV1,
};
#[allow(unused_imports)]
pub(crate) use physical_input::{VerifiedLoopPhysicalBoundaryV1, VerifiedLoopPhysicalInputV1};
#[allow(unused_imports)]
pub(crate) use physical_layout::{
    LoopPhysicalLayoutCoverageReceiptV1, LoopPhysicalLayoutRejectV1, LoopPhysicalSegmentKeyV1,
    LoopPhysicalTargetV1, LoopPhysicalTransferV1, PreparedLoopControlSegmentV1,
    PreparedLoopPhysicalLayoutV1,
};
#[allow(unused_imports)]
pub(crate) use producer_id::LoopRecipeProducerIdV1;
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
pub(crate) use semantic_context::VerifiedLoopSemanticContextV1;
#[allow(unused_imports)]
pub(crate) use source_bound_core::{
    issue_source_bound_core_from_artifact_v1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
    VerifiedLoopBindingEffectRelationV1, VerifiedLoopCoreProductV1,
    VerifiedLoopRecipeBindingRelationV1,
};
#[allow(unused_imports)]
pub(crate) use verify::{
    verify_source_bound_recipe_v1, LoopRecipeVerifierV1, VerifiedLoopRecipeV1,
};

/// Test-only end-to-end seam. The structural source-claim capability remains
/// private even when sibling modules exercise artifact verification.
#[cfg(test)]
pub(crate) fn verify_artifact_for_test(
    artifact: LoopRecipeArtifactV1,
) -> Result<(), LoopRecipeRejectReasonV1> {
    verify::LoopRecipeVerifierV1::verify_artifact(artifact).map(drop)
}

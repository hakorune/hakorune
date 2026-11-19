//! Neutral selfhost-portable recursive Loop recipe contract.

mod continuation;
// Shared resolver-function declaration lookup for physical-ready producers.
mod binding_declaration;
// Caller-zero common V2 operation/control/coverage projections.
// Caller-zero typed source-backed After boundary; no allocation or CFG.
// Caller-zero V2 physical-ID-free layout input transport.
// Caller-zero source-backed CompareI64 producer relation transport.
// Caller-zero source-block validation for JoinSig If continuation targets.
// Caller-zero source-backed condition operand inventory transport.
// Caller-zero source-backed initial-index seed transport; no physical effect.
// Caller-zero source-backed StringLen target realization; no physical Call.
// Caller-zero source-backed StringSubstring target realization; no physical Call.
// Caller-zero source-backed complete predicate branch-plan transport.
// Caller-zero same-cohort Return-read logical/layout/Join co-seal view.
// Caller-zero source-segment allocation demand; synthetic After is separate.
mod direct_accum_producer;
mod error;
mod ids;
mod input_source;
mod join_sig;
// Canonical draft builder: single key allocator + source-anchor co-recorder.
// Main0 Continue and the selected variable-bound Mul producer use this owner.
mod internal_declaration;
mod join_sig_branch;
mod loop_cond_break_continue_producer;
mod loop_true_break_continue_producer;
mod normalize;
mod operation_carrier_demand;
mod operation_effect;
mod operation_physical_demand;
mod physical_input;
mod physical_layout;
mod physical_transfer;
mod producer_id;
mod recipe_draft;
pub(crate) mod route_id;
mod s6c_scan_with_init;
// Caller-zero source-to-Recipe/Join Return binding; no physical effect.
mod s6c_return_source_binding;
// Caller-zero product-first JOINIR input façade; no physical consumer.
mod s6c_scan_with_init_joinir;
// Caller-zero logical output product and typed consumer; no JoinIR/MIR materialization.
mod s6c_scan_with_init_joinir_output;
// Caller-zero Builder-free prephysical ingress; no physical IDs or session state.
mod s6c_prephysical_ingress;
mod s6c_scan_with_init_joinir_output_rows;
// Caller-zero parent-retaining TextEq site contract; no physical target.
// Source-bound TextEq occurrence view; no physical IDs/runtime wire.
// Caller-zero source-backed scalar-scan corridor view; no physical effect.
// Typed row façade is currently exercised by focused tests only.
mod bool_region;
mod s6c_scan_with_init_rows;
mod schema;
mod schema_v2;
pub(crate) use bool_region::{
    LoopBoolCompareDomainV1, LoopBoolCompareV1, LoopBoolNodeV1, LoopBoolRegionV1,
};
mod source_loop_execution;
mod source_loop_schedule;
mod source_loop_v2;
pub(crate) use source_loop_schedule::{
    SourceLoopBranchPhiV2, SourceLoopScheduledStepV2, SourceLoopTextBranchPhiV2,
};
pub(crate) use source_loop_v2::{produce_source_loop_recipe_v2, VerifiedSourceLoopRecipeProductV2};
mod source_loop_bool;
pub(crate) use source_loop_bool::{project_source_loop_bool_v1, CheckedLoopBoolExpressionV1};
mod read_only_inputs;
mod semantic_context;
mod source_binding;
mod source_bound_core;
mod typed_schema_v2;
mod typed_schema_v2_structure;
mod variable_accum_recurrence_producer;
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
#[path = "loop_cond_break_continue_producer_tests.rs"]
mod loop_cond_break_continue_producer_tests;

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
#[path = "s6c_scan_with_init_tests.rs"]
mod s6c_scan_with_init_tests;

#[cfg(test)]
#[path = "s6c_prephysical_ingress_tests.rs"]
mod s6c_prephysical_ingress_tests;

#[cfg(test)]
#[path = "source_bound_core_tests.rs"]
mod source_bound_core_tests;

// S7A caller-zero wire-parity harness: `.hako` emission vs Rust-assembled
// artifact through the same decode/verify/normalize owner.
#[cfg(test)]
#[path = "wire_parity_tests.rs"]
mod wire_parity_tests;

// S7G caller-zero all-route wire-parity census: remaining attested-route
// arms (nested, loop-true), the DirectAccum live-producer anchor, and one
// coverage class per canonical route cross-checked against the sealed
// attestation and migration inventories.
#[cfg(test)]
#[path = "wire_route_parity_tests.rs"]
mod wire_route_parity_tests;

mod generic_g0_demand;

mod generic_g0;

#[cfg(test)]
pub(crate) use direct_accum_producer_tests::direct_accum_product_for_test;

pub(crate) use generic_g0_demand::{
    issue_generic_g0_recipe_demand_from_observation_v1, issue_generic_g0_recipe_demand_v1,
    GenericG0RecipeDemandIssueV1,
};

pub(crate) use generic_g0::{
    produce_generic_g0_recipe_v1, GenericG0RecipeProducerRejectV1,
    VerifiedGenericG0TailCapabilityV1, VerifiedGenericRecipeProductG0,
};

#[cfg(test)]
mod tests;

#[cfg(test)]
#[path = "operation_effect_tests.rs"]
mod operation_effect_tests;

#[cfg(test)]
mod operation_effect_parity;

#[cfg(test)]
#[path = "internal_declaration_tests.rs"]
mod internal_declaration_tests;

#[cfg(test)]
#[path = "operation_physical_demand_tests.rs"]
mod operation_physical_demand_tests;

#[cfg(test)]
#[path = "typed_schema_v2_text_tests.rs"]
mod typed_schema_v2_text_tests;

#[cfg(test)]
#[path = "typed_schema_v2_tests.rs"]
mod typed_schema_v2_tests;

#[cfg(test)]
#[path = "typed_schema_v2_dynamic_operation_tests.rs"]
mod typed_schema_v2_dynamic_operation_tests;

#[cfg(test)]
#[path = "typed_schema_v2_structure_tests.rs"]
mod typed_schema_v2_structure_tests;

// Keep one stable facade for the caller-zero common-V2 parent; the products
// remain source-only and do not open the physical session.
pub(crate) use binding_declaration::{
    resolve_loop_binding_declaration_v1, ResolvedLoopBindingDeclarationV1,
};
pub(crate) use continuation::VerifiedLoopContinuationContractV1;
pub(crate) use direct_accum_producer::{
    produce_direct_accum_recipe_v1, DirectAccumRecipeProducerRejectV1,
    VerifiedDirectAccumRecipeProductV1,
};
pub(crate) use error::LoopRecipeRejectReasonV1;
pub(crate) use ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
pub(crate) use input_source::{
    issue_initialized_local_input_source_set_v1, LoopInitializedLocalInputSourceRelationV1,
    LoopInitializedLocalInputSourceSetRejectV1, VerifiedLoopInitializedLocalInputSourceSetV1,
};
pub(crate) use internal_declaration::LoopInternalDeclarationModeV1;
pub(crate) use join_sig::{
    issue_sole_root_carrier_join_closure_v2, LoopJoinBranchArmTransferRefV2, LoopJoinBranchArmV2,
    LoopJoinBranchExitRefV2, LoopJoinBranchExitTargetV2, LoopJoinClosureRejectV2,
    LoopJoinEdgeRoleV1, LoopJoinEdgeV2, LoopJoinLogicalTransferRejectV2,
    LoopJoinLogicalTransferViewV2, LoopJoinNextItemV1, LoopJoinPayloadV2, LoopJoinPortBindingV2,
    LoopJoinPortV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1, LoopJoinSigV1,
    VerifiedLoopJoinClosureV2, VerifiedLoopJoinSigV1,
};
pub(crate) use loop_cond_break_continue_producer::{
    produce_loop_cond_break_continue_recipe_v1, LoopCondBreakContinueRecipeProducerRejectV1,
    VerifiedLoopCondBreakContinueRecipeProductV1,
};
pub(crate) use loop_true_break_continue_producer::{
    produce_loop_true_break_continue_recipe_v1, LoopTrueBreakContinueRecipeProducerRejectV1,
    VerifiedLoopTrueBreakContinueRecipeProductV1,
};
pub(crate) use normalize::{LoopRecipeDecodeErrorV2, LoopRecipeNormalizerV2};
pub(crate) use operation_carrier_demand::PreparedLoopDerivedCarrierSeedRowV1;
pub(crate) use operation_effect::{
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1,
    VerifiedLoopOperationEffectProductV1, VerifiedLoopOperationSourceEvidenceV1,
};
pub(crate) use operation_physical_demand::{
    LoopOperationPhysicalDemandRejectV1, PreparedLoopOperationProgramV1,
    PreparedLoopReadBindingRowV1, PreparedLoopWriteBindingRowV1,
    VerifiedLoopOperationPhysicalDemandV1,
};
pub(crate) use physical_input::VerifiedLoopPhysicalInputV1;
pub(crate) use physical_layout::{
    LoopPhysicalLayoutRejectV1, LoopPhysicalSegmentKeyV1, LoopPhysicalSegmentRoleV1,
    LoopPhysicalTargetV1, LoopPhysicalTransferV1, PreparedLoopPhysicalLayoutV1,
};
pub(crate) use producer_id::LoopRecipeProducerIdV1;
pub(crate) use recipe_draft::{LoopRecipeDraftRejectV1, LoopRecipeDraftV1};
pub(crate) use s6c_prephysical_ingress::{
    issue_s6c_prephysical_ingress_v2, S6CPrephysicalCompletionParityRefV2,
    S6CPrephysicalCompletionRefV2, S6CPrephysicalIngressRejectV2, S6CPrephysicalOperationRoleV2,
    VerifiedS6CPrephysicalIngressV2,
};
pub(crate) use s6c_return_source_binding::VerifiedS6CReturnSourceRecipeBindingV1;
pub(crate) use s6c_scan_with_init::{
    produce_s6c_scan_with_init_recipe_v2, S6CScanWithInitRecipeProducerRejectV2,
    VerifiedS6CScanWithInitRecipeProductV2,
};
pub(crate) use s6c_scan_with_init_joinir::S6CLogicalCallRoleV1;
pub(crate) use s6c_scan_with_init_joinir_output::issue_s6c_scan_with_init_logical_output_v1;
pub(crate) use s6c_scan_with_init_joinir_output_rows::{
    S6CLogicalCallArgsV1, S6CLogicalCallSlotV1, S6CLogicalItemV1,
};
pub(crate) use schema::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopExitKindV1,
    LoopNodeSourceBindingV1, LoopNodeV1, LoopOperationV1, LoopRecipeArtifactV1,
    LoopRecipeBindingV1, LoopRecipeBlockV1, LoopRecipeCarrierV1, LoopRecipeItemRowV1,
    LoopRecipeItemV1, LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1,
    LoopRecipeV1, LoopRecipeValueV1, LoopSourcePathStepV1, LoopSourcePathV1, LoopValueClassV1,
};
pub(crate) use schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopNodeV2,
    LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2, LoopOperationV2,
    LoopRecipeBindingV2, LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeExitV2,
    LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopValueClassV2,
    LOOP_RECIPE_SCHEMA_VERSION_V2,
};
pub(crate) use semantic_context::VerifiedLoopSemanticContextV1;
pub(crate) use source_bound_core::{
    issue_source_bound_core_from_artifact_v1, LoopBindingEffectAnchorV1,
    LoopBindingEffectRelationV1, LoopBindingEffectRoleV1, LoopRecipeBindingRelationV1,
    VerifiedLoopCoreProductV1, VerifiedLoopRecipeBindingRelationV1,
};
pub(crate) use typed_schema_v2::{
    LoopRecipeV2RejectReason, LoopRecipeVerifierV2, VerifiedLoopRecipeArtifactV2,
    VerifiedLoopRecipeV2,
};
pub(crate) use variable_accum_recurrence_producer::{
    produce_variable_accum_recurrence_recipe_v1, VerifiedVariableAccumRecurrenceRecipeProductV1,
};
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

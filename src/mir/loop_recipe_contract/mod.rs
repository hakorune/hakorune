//! Neutral selfhost-portable recursive Loop recipe contract.

mod continuation;
// Caller-zero common V2 operation/control/coverage projections.
mod common_v2_issuers;
// Caller-zero typed source-backed After boundary; no allocation or CFG.
mod common_v2_after_boundary;
// Caller-zero V2 physical-ID-free layout input transport.
mod common_v2_layout_input;
// Caller-zero source-backed CompareI64 producer relation transport.
mod common_v2_condition_producer;
// Caller-zero source-backed condition operand inventory transport.
mod common_v2_condition_operand_inventory;
// Caller-zero source-backed initial-index seed transport; no physical effect.
mod common_v2_initial_index_seed;
// Caller-zero source-backed StringLen target realization; no physical Call.
mod common_v2_string_len_target_plan;
// Caller-zero source-backed complete predicate branch-plan transport.
mod common_v2_predicate_branch_plan;
// Caller-zero source-segment allocation demand; synthetic After is separate.
mod common_v2_segment_allocation;
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
mod physical_transfer;
mod producer_id;
pub(crate) mod route_id;
#[allow(dead_code)]
mod s6c_scan_with_init;
// Caller-zero product-first JOINIR input façade; no physical consumer.
#[allow(dead_code)]
mod s6c_scan_with_init_joinir;
// Caller-zero logical output product and typed consumer; no JoinIR/MIR materialization.
#[allow(dead_code)]
mod s6c_scan_with_init_joinir_output;
// Caller-zero Builder-free prephysical ingress; no physical IDs or session state.
#[allow(dead_code)]
mod s6c_prephysical_ingress;
#[allow(dead_code)]
mod s6c_scan_with_init_joinir_output_rows;
#[allow(dead_code)]
mod s6c_scan_with_init_logical_consumer;
// Caller-zero parent-retaining TextEq site contract; no physical target.
#[allow(dead_code)]
mod s6c_text_eq_site_contract;
// Typed row façade is currently exercised by focused tests only.
#[allow(dead_code)]
mod s6c_scan_with_init_rows;
mod schema;
mod schema_v2;
mod semantic_context;
mod source_binding;
mod source_bound_core;
mod typed_schema_v2;
mod typed_schema_v2_structure;
mod variable_accum_break_producer;
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
#[path = "common_v2_initial_index_seed_tests.rs"]
mod common_v2_initial_index_seed_tests;

#[cfg(test)]
#[path = "s6c_text_eq_site_contract_tests.rs"]
mod s6c_text_eq_site_contract_tests;

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
#[path = "typed_schema_v2_tests.rs"]
mod typed_schema_v2_tests;

#[cfg(test)]
#[path = "typed_schema_v2_dynamic_operation_tests.rs"]
mod typed_schema_v2_dynamic_operation_tests;

#[cfg(test)]
#[path = "typed_schema_v2_structure_tests.rs"]
mod typed_schema_v2_structure_tests;

#[cfg(test)]
pub(crate) use operation_effect_parity::{
    issue_operation_effect_parity_receipt_v1, LoopOperationEffectParityReceiptV1,
    LoopOperationEffectParityRejectV1, LoopOperationEffectParitySideV1,
};

// Keep one stable facade for the caller-zero common-V2 parent; the products
// remain source-only and do not open the physical session.
#[allow(unused_imports)]
pub(crate) use common_v2_after_boundary::{
    AfterBoundaryIssueRejectV1, LoopV2AfterBoundaryRelationV1,
    VerifiedLoopV2AfterBoundarySourceRelationV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_condition_operand_inventory::{
    ConditionOperandInventoryRejectV1, PreparedLoopV2ConditionOperandInventoryV1,
    PreparedLoopV2ConditionOperandKindV1, PreparedLoopV2ConditionOperandRowV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_issuers::{
    issue_s6c_common_v2_pre_session_v1, CommonV2IssuerRejectV1, PreparedLoopControlPlacementV2,
    PreparedLoopControlTransferProgramV2, PreparedLoopOperationProgramV2,
    PreparedLoopOperationRowV2, PreparedLoopV2PreSessionEnvelopeV1,
    VerifiedLoopV2EnvelopeCoverageV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_layout_input::{
    LayoutInputRejectV1, PreparedLoopV2LayoutLoopV1, PreparedLoopV2LayoutSegmentRefV1,
    PreparedLoopV2PhysicalLayoutInputV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_predicate_branch_plan::{
    issue_s6c_v2_predicate_branch_plan_v1, PredicateBranchPlanRejectV1,
    PreparedLoopV2ConditionCarrierRequirementV1, PreparedLoopV2PredicateBranchPlanV1,
    PreparedLoopV2PredicateFalseTargetV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_segment_allocation::{
    issue_v2_segment_allocation_plan, PreparedLoopV2SegmentAllocationPlanV1,
    SegmentAllocationPlanRejectV1,
};
#[allow(unused_imports)]
pub(crate) use common_v2_string_len_target_plan::{
    issue_s6c_v2_string_len_call_target_plan_v1, PreparedLoopV2StringLenCallTargetPlanV1,
    StringLenCallTargetPlanRejectV1,
};
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
    issue_sole_root_carrier_join_closure_v2, LoopJoinBoundaryTransferRefV1,
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchArmV1, LoopJoinBranchArmV2,
    LoopJoinBranchExitRefV2, LoopJoinBranchExitTargetV2, LoopJoinBranchExitV1,
    LoopJoinBranchExitV2, LoopJoinBranchV1, LoopJoinBranchV2, LoopJoinClosureRejectV2,
    LoopJoinEdgeRoleV1, LoopJoinEdgeV1, LoopJoinEdgeV2, LoopJoinLogicalTransferRejectV1,
    LoopJoinLogicalTransferRejectV2, LoopJoinLogicalTransferViewV1, LoopJoinLogicalTransferViewV2,
    LoopJoinLoopV1, LoopJoinLoopV2, LoopJoinPayloadV1, LoopJoinPayloadV2, LoopJoinPortBindingV1,
    LoopJoinPortBindingV2, LoopJoinPortV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1,
    LoopJoinSigV1, LoopJoinSigV2, VerifiedLoopAfterBindingV1, VerifiedLoopJoinClosureV2,
    VerifiedLoopJoinSigV1, VerifiedLoopJoinSigV2,
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
    LoopPhysicalSegmentRoleV1, LoopPhysicalTargetV1, LoopPhysicalTransferV1,
    PreparedLoopControlSegmentV1, PreparedLoopPhysicalLayoutV1,
};
#[allow(unused_imports)]
pub(crate) use producer_id::LoopRecipeProducerIdV1;
#[allow(unused_imports)]
pub(crate) use s6c_prephysical_ingress::{
    issue_s6c_prephysical_ingress_v2, S6CPrephysicalCompletionParityRefV2,
    S6CPrephysicalCompletionRefV2, S6CPrephysicalIngressRejectV2, S6CPrephysicalOperationRoleV2,
    VerifiedS6CPrephysicalIngressV2,
};
#[allow(unused_imports)]
pub(crate) use s6c_scan_with_init::{
    produce_s6c_scan_with_init_recipe_v2, S6CScanWithInitRecipeProducerRejectV2,
    S6CScanWithInitRecipeProductRefV2, S6CScanWithInitRecipeRolesRefV2,
    S6CVerifiedRecipeReadViewV2, VerifiedS6CJoinRoleSealV2, VerifiedS6CScanWithInitRecipeProductV2,
};
#[allow(unused_imports)]
pub(crate) use s6c_scan_with_init_joinir::{
    with_s6c_scan_with_init_logical_join_input, S6CLogicalCallInputRefV1, S6CLogicalCallRoleV1,
    S6CLogicalJoinInputRejectV1, S6CScanWithInitLogicalJoinInputRefV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_scan_with_init_joinir_output::{
    issue_s6c_scan_with_init_logical_output_v1, S6CLogicalCallPairsRefV1,
    S6CLogicalCallWithSourceRefV1, S6CScanWithInitLogicalOutputRefV1,
    VerifiedS6CScanWithInitLogicalOutputV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_scan_with_init_logical_consumer::{
    consume_s6c_scan_with_init_logical_output_v1, S6CLogicalConsumerRejectV1,
    S6CLogicalConsumerResultV1,
};
#[allow(unused_imports)]
pub(crate) use s6c_text_eq_site_contract::{
    issue_s6c_text_eq_source_binding_v1, LoopTextEqSiteRefV1, TextEqualityLawV1,
    VerifiedS6CTextEqSourceBindingV1,
};
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
pub(crate) use schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopNodeV2,
    LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2, LoopOperationV2,
    LoopRecipeArtifactV2, LoopRecipeBindingV2, LoopRecipeBlockV2, LoopRecipeCarrierV2,
    LoopRecipeExitV2, LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2,
    LoopValueClassV2, LOOP_RECIPE_SCHEMA_VERSION_V2,
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
pub(crate) use typed_schema_v2::{
    LoopRecipeV2RejectReason, LoopRecipeVerifierV2, VerifiedLoopRecipeArtifactV2,
    VerifiedLoopRecipeV2,
};
#[allow(unused_imports)]
pub(crate) use variable_accum_break_producer::{
    produce_variable_accum_break_recipe_v1, VariableAccumBreakControlSourceReceiptV1,
    VariableAccumBreakRecipeProducerRejectV1, VerifiedVariableAccumBreakRecipeProductV1,
};
#[allow(unused_imports)]
pub(crate) use variable_accum_recurrence_producer::{
    produce_variable_accum_recurrence_recipe_v1, VariableAccumRecurrenceRecipeProducerRejectV1,
    VerifiedVariableAccumRecurrenceRecipeProductV1,
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

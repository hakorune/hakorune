//! Neutral selfhost-portable recursive Loop recipe contract.

// Shared resolver-function declaration lookup for physical-ready producers.
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
mod error;
mod ids;
mod join_sig;
// Canonical draft builder: single key allocator + source-anchor co-recorder.
// Main0 Continue and the selected variable-bound Mul producer use this owner.
mod join_sig_branch;
mod producer_id;
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
mod source_binding;
mod typed_schema_v2;
mod typed_schema_v2_structure;





#[cfg(test)]
#[path = "producer_id_migration_tests.rs"]
mod producer_id_migration_tests;



#[cfg(test)]
#[path = "s6c_scan_with_init_tests.rs"]
mod s6c_scan_with_init_tests;

#[cfg(test)]
#[path = "s6c_prephysical_ingress_tests.rs"]
mod s6c_prephysical_ingress_tests;


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
pub(crate) use ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
pub(crate) use join_sig::{
    issue_sole_root_carrier_join_closure_v2, LoopJoinClosureRejectV2, LoopJoinLogicalTransferRejectV2,
    LoopJoinLogicalTransferViewV2, VerifiedLoopJoinClosureV2,
};
#[cfg(test)]
pub(crate) use join_sig::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchArmV2, LoopJoinBranchExitTargetV2,
    LoopJoinEdgeRoleV1, LoopJoinEdgeV2, LoopJoinExitOccurrenceV1, LoopJoinPayloadV2,
    LoopJoinPortBindingV2, LoopJoinPortV1, LoopJoinSigRejectReasonV1,
};
pub(crate) use producer_id::LoopRecipeProducerIdV1;
pub(crate) use s6c_prephysical_ingress::{
    issue_s6c_prephysical_ingress_v2, S6CPrephysicalCompletionParityRefV2,
    S6CPrephysicalIngressRejectV2, VerifiedS6CPrephysicalIngressV2,
};
#[cfg(test)]
pub(crate) use s6c_prephysical_ingress::S6CPrephysicalOperationRoleV2;
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
    LoopNodeSourceBindingV1,
    LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1, LoopRecipeSourceOwnerV1, LoopSourcePathStepV1,
    LoopSourcePathV1,
};
pub(crate) use schema_v2::{
    LoopBinaryI64OpV2, LoopCompareI64OpV2, LoopConditionV2, LoopExitKindV2, LoopNodeV2,
    LoopOperationExecutionClassV2, LoopOperationFaultFamilyV2, LoopOperationV2,
    LoopRecipeBindingV2, LoopRecipeBlockV2, LoopRecipeCarrierV2, LoopRecipeExitV2,
    LoopRecipeItemRowV2, LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopValueClassV2,
    LOOP_RECIPE_SCHEMA_VERSION_V2,
};
pub(crate) use typed_schema_v2::{
    LoopRecipeV2RejectReason, LoopRecipeVerifierV2, VerifiedLoopRecipeArtifactV2,
    VerifiedLoopRecipeV2,
};

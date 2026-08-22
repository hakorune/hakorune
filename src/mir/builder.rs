/*!
 * MIR Builder - Converts AST to MIR/SSA form
 *
 * Implements AST → MIR conversion with SSA construction
 */

use super::{
    BasicBlock, BasicBlockId, CompareOp, ConstValue, Effect, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
pub(crate) use calls::CallTarget;
use hakorune_mir_builder::CoreContext;
mod array_element_write;
mod assignment_lowering;
mod brand_constructor_lowering_projection;
mod builder_build;
mod builder_debug;
mod builder_emit;
mod builder_init;
mod builder_metadata;
mod builder_method_index;
mod builder_publication_target; // PUBLICATION0 target quiescence/receipt
#[cfg(test)]
mod builder_test_api;
mod builder_value_kind;
mod call_resolution; // ChatGPT5 Pro: Type-safe call resolution utilities
mod callable_declaration_catalog; // Complete same-module callable declaration authority
mod literal_lowering;
#[cfg(test)]
mod literal_postemit_retirement_tests;
mod new_expression;
mod normal_callable_binding_materialization;
mod normal_callable_binding_materialization_port; // existing formal ValueId handoff
mod normal_callable_catalog_owner_link; // exact catalog key/owner/source ingress co-seal
mod normal_callable_dynamic_loop_prepare; // pre-effect Dynamic Loop ingress co-seal
mod normal_callable_dynamic_loop_rebind; // exact-once Dynamic Add/rebind terminal
mod normal_callable_dynamic_operation_source; // source-backed Dynamic Loop operation relations
mod normal_callable_dynamic_origin; // source-backed Dynamic -> existing physical receipts
mod normal_callable_dynamic_source; // source-backed untyped formal/Loop carrier authority
mod normal_callable_loop_handoff; // callable Loop source/BindingRef S0 handoff
#[allow(dead_code)]
mod normal_callable_loop_source_facts; // callable source-aware Facts/Recipe P0 caller-zero issuer
mod normal_callable_prepared_operation; // Builder-free full-demand ingress
mod normal_callable_semantic_loan_port; // Thin installed-package port adapter
mod normal_callable_semantic_lowering_state; // Callable BindingRef-to-ValueId projection
mod normal_callable_semantic_source; // Co-sealed selected callable source authority
mod normal_callable_semantic_source_lookup; // Exact legacy source-site/view lookup during cutover
mod normal_cataloged_box_method_lowering;
mod variable_read;
pub(crate) use callable_declaration_catalog::{
    issue_source_backed_same_module_callable_catalog_v1, CanonicalSameModuleCallableKeyV1,
    SameModuleCallableCatalogBrandV1, SameModuleCallableNamespaceV1,
    SelectedCallableConsumptionRoleV1, SelectedNormalCallableKeyV1,
    SourceBackedCallableCatalogIssueV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1, VerifiedSelectedNormalCallableSourceInventoryV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
#[cfg(test)]
pub(crate) use main_expansion::with_test_main_static_children;
pub(in crate::mir) use main_expansion::VerifiedMainStaticChildV1;
pub(in crate::mir) use normal_callable_catalog_owner_link::{
    issue_catalog_callable_owner_link_v1, CatalogCallableOwnerLinkIssueV1,
    VerifiedCatalogCallableOwnerLinkV1,
};
pub(in crate::mir) use normal_callable_dynamic_source::{
    issue_source_backed_dynamic_callable_v1, VerifiedSourceBackedDynamicCallableV1,
};
pub(in crate::mir) use normal_callable_semantic_source::{
    NormalCallableSemanticAdmissionV1, VerifiedNormalCallableSemanticSourceV1,
};
mod calls; // Call system modules (refactored from builder_calls)
#[allow(dead_code)]
mod canonical_physical_drain;
#[cfg(test)]
mod canonical_root_completion_receipt0_p0; // CUT0-I0-ROOT0-CANON0-RECEIPT0 fixtures
#[cfg(test)]
mod canonical_root_completion_recursive0_p0; // CUT0-I0-ROOT0-CANON0-RECURSIVE0 fixtures
mod collection_literals; // ArrayLiteral / MapLiteral lowering
mod compilation_context; // Phase 136 follow-up (Step 7/7): CompilationContext extraction
pub(crate) use compilation_context::CompilationContext;
mod compound_assignment; // evaluated Place read-modify-write lowering
mod decls; // declarations lowering split
#[allow(dead_code)]
mod drained_module_candidate; // HEADERPORT0-I0-DRAIN0-S0 disconnected candidate
#[cfg(test)]
mod drained_module_candidate_p0; // HEADERPORT0-I0-DRAIN0-P0 fixtures
mod entry_materialization; // source-only callable Main materialization facts
mod enum_match_scopebox;
mod exprs_call;
mod exprs_check; // CheckExpr lowering
mod exprs_enum_match; // narrow direct-MIR enum match lowering for guard-let sugar
mod fact_session;
#[cfg(test)]
mod fact_session_p0_tests;
mod fastmem; // fastmem source -> MIR MemOp metadata lowering
mod fastmem_context; // FastMemory region context helpers
mod function_lowering_state;
#[allow(dead_code)]
mod function_signature_lookup; // HEADERPORT0-S0 neutral header lookup surface
mod function_state_transaction;
mod generic_loop_admission_observation;
mod instance_box_constructor_batch;
mod instance_box_declaration_lifecycle;
mod instance_box_declaration_metadata;
mod instance_box_method_batch;
#[allow(dead_code)]
mod main_expansion; // HEADERPORT0-I0-MAINROLE0-S0 source-only Main expansion
mod nested_box_method_source;
mod normal_instance_constructor_admission;
mod normal_instance_constructor_demand_loan;
mod normal_instance_constructor_semantic_scope;
mod normal_runtime_inputs; // selected normal ingress runtime snapshot
mod normal_script_instance_box_transfer;
mod normal_script_composite_partition;
mod normal_script_direct_static_lookup;
mod normal_script_neutral_window;
mod raw_required_condition_draft; // ROOTBATCH0-S0b typed condition producer
#[cfg(test)]
mod raw_required_condition_draft_p0; // ROOTBATCH0-S0b exact factory contract
mod raw_root_body_exit; // RAW-BODY-RETURN0 sole signature/Return/witness owner
pub(in crate::mir) use raw_root_body_exit::{RawVmSourceEntryDecodeKindV1, RawVmUnitOriginV1};
mod raw_root_body_lowering; // RAW-SOURCE0-LOWER0-ROOT0-BODY0 recipe-only value lowerer
#[cfg(test)]
mod raw_root_body_lowering_p0; // BODY0-S0-B disconnected lowerer fixtures
#[allow(dead_code)]
mod raw_root_environment_install;
mod raw_root_physical; // RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PHYSICAL0 empty carrier
mod raw_static_main_compat_batch; // explicit raw static-Main source partition
mod script_physical_exit; // NORMAL-SCRIPT0 brand-free Script terminal kernel
pub(in crate::mir) use builder_publication_target::{
    check_builder_external_commit_quiescence, BuilderPublicationReceiptV1,
};
pub(in crate::mir) use normal_module_transaction::{
    canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1,
};
pub(in crate::mir) use raw_root_physical::publication_terminal::RawPublishedModuleV1;
pub(in crate::mir) use root_batch_slot::RawMainEntryTargetV1;
pub(in crate::mir) use script_physical_exit::{
    CompletedScriptPhysicalFunctionV1, OpenScriptPhysicalEntrySessionV1,
    ScriptPhysicalEntrySessionErrorV1,
};
#[allow(dead_code)]
mod raw_source_projection; // RAW-SOURCE0-PLAN0 owned source locators
pub(in crate::mir) use raw_required_condition_draft::RawRequiredConditionDraftV1;
pub(in crate::mir) use raw_root_environment_install::{
    CompletedRawRootBodyPhysicalV1, InstalledRawRootEnvironmentV1,
    PreparedRawRootEnvironmentInstallV1, RawRootBodyLoweringErrorV1,
    RawRootEnvironmentInstallErrorV1, RawRootEnvironmentInstallOwnerV1,
    RawRootEnvironmentInstallRouteV1, RawRootEnvironmentProjectionV1,
    RejectedRawRootBodyPhysicalV1, RejectedRawRootEnvironmentInstallV1,
};
pub(in crate::mir) use raw_root_physical::callable_main_terminal::{
    CompletedRawCallableMainPhysicalV1, RawRootPhysicalCallableMainErrorV1,
    RejectedRawCallableMainPhysicalV1,
};
pub(in crate::mir) use raw_root_physical::child_terminal::RawRootPhysicalChildErrorV1;
pub(in crate::mir) use raw_root_physical::root_batch_terminal::{
    CompletedRawRootBatchPhysicalV1, RawRootBatchPhysicalErrorV1, RejectedRawRootBatchPhysicalV1,
};
pub(in crate::mir) use raw_root_physical::RawRootBodyPhysicalErrorV1;
pub(in crate::mir) use raw_root_physical::RawRootPhysicalStateV1;
mod raw_root_child_work; // RAW-SOURCE0-LOWER0-ROOT0-CHILDREN0 source-bound helper work
pub(in crate::mir) use raw_root_child_work::{
    RawCallableMainWorkV1, RawRootStaticChildWorkErrorV1, RawRootStaticChildWorkV1,
};
mod raw_root_static_child_admission;
pub(in crate::mir) use entry_materialization::{
    CallableMainMaterializationPolicyV1, CallableMainMaterializationTargetV1,
    NormalEntryMaterializationSourceReceiptV1, RawEntryMaterializationSourceReceiptV1,
};
pub(in crate::mir) use normal_runtime_inputs::NormalRuntimeInputSnapshotV1;
pub(in crate::mir) use raw_root_static_child_admission::PreparedRawRootStaticChildDraftV1;
pub(in crate::mir) use raw_source_projection::{
    OwnedRawRootProjectionV1, OwnedRawSourceV1, RawRootProjectionPartsV1, RawSourceLocatorV1,
    RawSourceOriginV1, RawSourceProjectionErrorV1,
};
#[allow(dead_code)]
mod main_pending_draft; // HEADERPORT0-I0-MAINPENDING0-S0 disconnected handoff
#[cfg(test)]
mod main_pending_draft_p0; // HEADERPORT0-I0-MAINPENDING0-P0 fixtures
#[allow(dead_code)]
mod main_root_wiring; // HEADERPORT0-REENTRANT-TERM0-I0-WIRING-S0 disconnected order
#[allow(dead_code)]
mod me_call_header_observation; // ACCESS0-MEHEADER-S0 typed source snapshot
mod metadata_context; // Phase 136 follow-up (Step 6/7): MetadataContext extraction
mod method_call_handlers;
mod module_completion_candidate;
#[cfg(test)]
mod module_declaration_fact_shell_commit_p0; // HEADERPORT0 BORROW-P0-ROOT-P0c proof
#[allow(dead_code)]
mod module_declaration_facts; // HEADERPORT0-I0-SHELLFACT0-S0 disconnected facts
#[cfg(test)]
mod module_declaration_facts_p0; // HEADERPORT0-I0-SHELLFACT0-P0 fixtures
mod module_draft_collector;
#[cfg(test)]
mod module_draft_collector_receipt_p0; // ROUTEINV-P0a-RECEIPT-P0 matrix
#[cfg(test)]
mod module_draft_collector_receipt_tests; // ROUTEINV-P0a-RECEIPT-S0 fixtures
#[cfg(test)]
mod module_finalization_candidate_p0; // HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0 matrix
#[allow(dead_code)]
mod module_finalization_once; // CUT0-S0 Builder-free finalizer
#[cfg(test)]
mod module_finalization_once_p0; // CUT0-S0 finalizer fixtures
#[allow(dead_code)]
mod module_finalization_split; // HEADERPORT0-I0-MODULEFINAL0-SPLIT0 input
#[cfg(test)]
mod module_finalization_split_p0; // HEADERPORT0-I0-MODULEFINAL0-SPLIT0-P0 fixtures
#[allow(dead_code)]
mod module_invocation_brand0; // CUT0-I0-ROOT0-BRAND0 real owner
#[cfg(test)]
mod module_invocation_brand_p0; // CUT0-I0-ID0-P0 fixtures
#[allow(dead_code)]
mod module_invocation_drain; // HEADERPORT0 I0-SHELL-I0-S0 disconnected drain
#[cfg(test)]
mod module_invocation_drain_s0_tests; // CUT0-S0 same-state drain fixtures
#[allow(dead_code)]
mod module_invocation_identity; // CUT0-I0-ID0-S0 disconnected identity/token
#[cfg(test)]
mod module_invocation_identity_p0; // CUT0-I0-ID0-S0 fixtures
#[allow(dead_code)]
mod module_invocation_owner_chain; // CUT0-I0-ID0-P0 disconnected brand chain
#[allow(dead_code)]
mod module_invocation_route_matrix;
#[allow(dead_code)]
mod normal_module_transaction; // NORMAL-MODULE-TX0-L0 disconnected schema
pub(in crate::mir) use canonical_physical_drain::{
    CanonicalDrainedCallablePhysicalV1, CanonicalDrainedSinglePhysicalV1,
    CanonicalPhysicalDrainPrepareErrorV1, PreparedCanonicalCallablePhysicalDrainV1,
    PreparedCanonicalSinglePhysicalDrainV1, RejectedCanonicalCallablePhysicalDrainV1,
    RejectedCanonicalSinglePhysicalDrainV1,
};
pub(in crate::mir) use module_draft_collector::{
    CollectedDraftAdmissionReceiptV1, CommitCallableCollectorBatchReceiptV1,
    CommitCollectedDraftAdmissionReceiptV1,
};
pub(in crate::mir) use module_invocation_brand0::{
    CanonicalCallableCapabilityWitnessV1, CanonicalPhysicalCollectionErrorV1,
    CollectedCanonicalCallablePhysicalV1, CollectedCanonicalSinglePhysicalV1,
    InvocationPhysicalStateV1, RejectedCanonicalPhysicalCollectionV1,
};
pub(in crate::mir) use module_invocation_owner_chain::InvocationBranded;
pub(in crate::mir) use module_lowering_invocation::ModuleLoweringPortChildErrorV1;
pub(in crate::mir) use module_lowering_shell::ModuleLoweringShellErrorV1;
#[cfg(test)]
pub(in crate::mir) use normal_module_transaction::completed_for_main_physical;
pub(in crate::mir) use normal_module_transaction::{
    CompletedNormalCallableCandidateV1, CompletedNormalCallableModuleEvidenceV1,
    CompletedNormalMainModuleCandidateV1, CompletedNormalMainModuleEvidenceV1,
    CompletedNormalScriptModuleCandidateV1, CompletedNormalScriptModuleEvidenceV1,
    NormalCallableCandidateVerificationReceiptV1, NormalCanonicalModuleBatchErrorV1,
    NormalCanonicalModuleBatchV1, NormalMainCandidateVerificationReceiptV1,
    NormalMainModuleTransactionErrorV1, NormalScriptCandidateVerificationReceiptV1,
    PreparedNormalScriptModuleTransactionV1, RejectedNormalCallableBatchV1,
    RejectedNormalCallableCommitV1, RejectedNormalCallableMainPhysicalV1,
    RejectedNormalHelperDraftPrefixV1, RejectedNormalScriptModuleTransactionV1,
    VerifiedScriptEntryResultContractV1,
};
pub(in crate::mir) use raw_root_physical::drain_terminal::{
    PreparedRawPhysicalDrainV1, RawDrainWitnessV1, RawDrainedPhysicalV1, RawPhysicalDrainErrorV1,
    RejectedRawPhysicalDrainV1,
};
pub(in crate::mir) use raw_root_physical::finalization_terminal::{
    PreparedRawDrainedPhysicalFinalizationV1, RawFinalizationParitySealV1, RawFinalizedPhysicalV1,
    RawPhysicalFinalizationErrorV1 as RawRootPhysicalFinalizationErrorV1,
    RejectedRawPhysicalFinalizationV1,
};
pub(in crate::mir) use raw_root_physical::postprocess_terminal::{
    RawExternalCommitModuleV1, RawExternalCommitPhysicalErrorV1,
    RawExternalCommitPhysicalHandoffV1, RawPostprocessCarrierParityErrorV1,
    RawPostprocessModuleLoanV1, RawPostprocessParitySealV1, RawPostprocessPhysicalOwnerV1,
    RawPostprocessProgressV1, RawPostprocessedPhysicalV1,
};
mod canonical_root_completion; // CUT0-I0-ROOT0-CANON0 route-specific completion
mod canonical_root_completion_error; // CUT0-I0-ROOT0-CANON0 shared error vocabulary
#[allow(dead_code)]
mod module_invocation_callable_batch; // CUT0-I0-COLLECT0-BATCH0 source/co-seal
#[cfg(test)]
mod module_invocation_collect0_s0_p0; // CUT0-I0-COLLECT0-S0 fixtures
#[allow(dead_code)]
mod module_invocation_collection; // CUT0-I0-COLLECT0-S0 co-seal terminal
mod module_invocation_session; // Shared isolated Builder transaction
mod pinned_text_invocation_binding; // Session-owned pinned-Text target/brand ingress
pub(in crate::mir) use module_invocation_session::{
    BuilderCommitReadinessErrorV1, BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1,
    PreparedBuilderExternalCommitV1, PreparedBuilderModuleSessionV1,
    RejectedPreparedBuilderModuleSessionV1,
};
#[cfg(test)]
mod module_invocation_session_p0; // CUT0-I0-SESSION0 fixtures
mod normal_cataloged_box_method_admission; // Selected normal cataloged-child identity
pub(in crate::mir) use normal_cataloged_box_method_admission::{
    CatalogedBoxMethodPhysicalHeaderProjectionV1, NormalCatalogedBoxMethodAdmissionErrorV1,
    NormalCatalogedBoxMethodDraftAdmissionV1,
};
mod normal_default_root_catalog_lifecycle; // Selected normal root/catalog lifecycle
mod normal_default_root_catalog_post_install; // Existing post-install lowering consumer
mod normal_script_boundary_receipt_pack; // Script retained boundary receipts
mod normal_script_direct_static_join_handoff; // Script source/Facts Recipe handoff
mod normal_script_direct_static_physical_publication; // Script ExactI64 physical publication
mod normal_script_direct_static_recipe; // Dedicated Script direct-static Recipe producer
mod normal_script_direct_static_result_bundle; // Script source/result Facts bundle
mod normal_script_direct_static_result_publication_owner; // Script source/Facts result owner
mod normal_script_operational_demand_receipt_pack; // Script structured demand receipts
mod normal_script_pre_effect_source_observation; // AST-free Script source handoff before Builder effects
mod normal_script_semantic_lowering_input; // Retained Script source products into lowering
mod normal_script_semantic_lowering_projection; // Immutable Script lowering projection
mod normal_script_semantic_lowering_state; // Script BindingRef -> ValueId ledger
mod normal_script_semantic_source; // Producer-backed lexical Script source
mod normal_script_semantic_source_core; // Shared Script source/forest/projection core
mod normal_script_source_continuation; // Resolver-issued Script source continuation
#[cfg(test)]
#[path = "builder/normal_script_source_continuation_tests.rs"]
mod normal_script_source_continuation_tests;
mod program_root_lowering; // Shared typed/generic Program root owner
pub(in crate::mir) use normal_default_root_catalog_lifecycle::{
    CompletedNormalDefaultRootCatalogLifecycleV1, NormalDefaultRootCatalogLifecycleErrorV1,
    NormalDefaultRootCatalogLifecycleStageV1, PreparedNormalDefaultProgramRootV1,
    RejectedNormalDefaultRootCatalogLifecycleV1,
};
#[allow(dead_code)]
mod cataloged_box_method_collector_handoff;
mod module_lowering_access_port; // HEADERPORT0 I0-ACCESS0-S0 disconnected vocabulary
#[cfg(test)]
mod module_lowering_borrow_root_p0; // HEADERPORT0 WIRING-I0-BORROW-P0-ROOT proof
#[cfg(test)]
mod module_lowering_borrow_root_p0d; // HEADERPORT0 WIRING-I0-BORROW-P0-ROOT-P0d proof
#[allow(dead_code)]
mod module_lowering_borrow_schedule; // HEADERPORT0 WIRING-I0-BORROW-S0 passive schedule
mod module_lowering_invocation;
#[allow(dead_code)]
mod module_lowering_invocation_access; // HEADERPORT0 WIRING-S0 live bundle
#[cfg(test)]
mod module_lowering_invocation_access_tests;
#[allow(dead_code)]
mod module_lowering_invocation_candidate; // HEADERPORT0 CANDIDATE0-S0 disconnected abort owner
#[allow(dead_code)]
mod module_lowering_invocation_candidate_p0; // HEADERPORT0 CANDIDATE0-P0 route co-seal
#[allow(dead_code)]
mod module_lowering_invocation_legacy_term;
#[cfg(test)]
mod module_lowering_invocation_reentrant_tests;
#[allow(dead_code)]
mod module_lowering_invocation_state; // HEADERPORT0 I0-STATE0-S0 disconnected seam
#[allow(dead_code)]
mod module_lowering_shell; // HEADERPORT0 I0-SHELL-S0 disconnected shell
#[allow(dead_code)]
mod module_wiring_parity_p0; // HEADERPORT0 WIRING-P0 disconnected parity
#[cfg(test)]
mod module_wiring_route_matrix_p0e; // ROUTEINV-P0e test-only matrix closure
mod nonmain_static_box_method_batch;
#[allow(dead_code)]
mod raw_expansion_receipt_ledger; // ROUTEINV-P0b-RAWLEDGER-S0 disconnected owner
pub(in crate::mir) use raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, SealedRawExpansionReceiptLedgerV1,
};
#[cfg(test)]
mod raw_expansion_receipt_ledger_p0; // ROUTEINV-P0b-RAWLEDGER-P0 proof matrix
#[cfg(test)]
mod raw_expansion_receipt_ledger_tests; // ROUTEINV-P0b-RAWLEDGER-S0 fixtures
mod raw_expression_dispatch; // single raw AST expression dispatcher
#[allow(dead_code)]
mod raw_loop_child_entry; // LOOPBRIDGE0-S0 pure raw Loop child-entry quarantine
mod raw_loop_child_port; // CALLABLE-LOOP-ORDINARY-BRIDGE-S0 behavior-neutral port boundary
#[allow(dead_code)]
mod raw_root_completion; // CUT0-I0-ROOT0-RAW0 retained raw root witness
pub(in crate::mir) use raw_root_completion::RawInvocationRootWitnessV1;
#[allow(dead_code)]
mod raw_root_completion_preflight; // ROOT-RETENTION0-PREFLIGHT borrowed owner checks
#[cfg(test)]
mod resolved_owner_header_p0; // ROUTEINV-P0c-SINGLEHDR-P0 matrix
#[allow(dead_code)]
mod root_batch_slot; // HEADERPORT0-I0-ROOTBATCH0-S0 identity SSOT
pub(in crate::mir) use root_batch_slot::raw_main_entry_target;
mod enum_match_source_demand;
mod enum_variant_source_demand;
mod qmark_source_demand;
mod raw_expression_recursion_guard;
mod raw_invocation_body;
mod raw_invocation_source_item_site;
mod raw_invocation_source_statement_classification;
mod raw_invocation_source_transport;
mod static_result_publication_ingress;
mod raw_structured_child_scope;
mod record_literal_source_demand;
mod recursive_child_lowering_port;
mod recursive_child_lowering;
#[cfg(test)]
mod recursive_child_lowering_port_tests;
#[cfg(test)]
mod recursive_child_lowering_rawport_header_tests;
#[cfg(test)]
mod recursive_child_lowering_rawport_tests;
#[cfg(test)]
mod recursive_child_lowering_tests;
#[allow(dead_code)]
mod root_body_completion; // HEADERPORT0-I0-BODYDRAIN0-S0 disconnected witness
#[cfg(test)]
mod root_body_completion_p0; // HEADERPORT0-I0-BODYDRAIN0-P0 fixtures
#[allow(dead_code)]
mod root_draft_batch; // HEADERPORT0-I0-ROOTBATCH0-S0 disconnected batch // CUT0-I0-POST0-RAW-S0 physical retention
#[cfg(test)]
mod root_draft_batch_commit_p0; // HEADERPORT0 BORROW-P0-ROOT-P0b proof
#[cfg(test)]
mod root_draft_batch_p0; // HEADERPORT0-I0-ROOTBATCH0-P0 fixtures
#[allow(dead_code)]
mod route_owned_invocation_inventory; // HEADERPORT0 WIRING-I0-ROUTEINV-S0 policy
mod variable_context; // Phase 136 follow-up (Step 5/7): VariableContext extraction // Method call handler separation (Phase 3) // call(expr)
                      // include lowering removed (using is handled in runner)
mod control_flow; // thin wrappers to centralize control-flow entrypoints
#[cfg(test)]
pub(crate) use control_flow::joinir::route_entry::registry::{
    execute_legacy_policy_parity_v1, LegacyPolicyAttemptDispositionV1, LegacyPolicyParityReceiptV1,
};

/// Test-only bridge for the Nested parity oracle. It projects the existing
/// facts/registry winner without entering any legacy route or touching a
/// Builder.
#[cfg(test)]
pub(crate) fn loop_route_effective_winner_for_test(
    condition: &crate::ast::ASTNode,
    body: &[crate::ast::ASTNode],
) -> Result<Option<crate::mir::loop_recipe_contract::route_id::LoopRouteId>, String> {
    let facts = control_flow::plan::facts::try_build_loop_facts(condition, body)
        .map_err(|error| error.to_string())?;
    let Some(facts) = facts else {
        return Ok(None);
    };
    let canonical = control_flow::lower::normalize::canonicalize_loop_facts(facts);
    let selection =
        control_flow::joinir::route_entry::registry::select_recipe_first_routes(Some(&canonical));
    Ok(control_flow::joinir::route_entry::registry::effective_route_for_test(&selection))
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LegacyGenericCarrierSummaryV1 {
    CompleteNoRecursive,
    CompleteRecursive(Box<[String]>),
    Unavailable(String),
    Ambiguous(String),
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LegacyGenericFactsStatusV1 {
    Available,
    Absent,
    Frozen(&'static str),
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LegacyGenericLoopObservationV1 {
    pub(crate) status: LegacyGenericFactsStatusV1,
    pub(crate) v0_present: bool,
    pub(crate) v1_present: bool,
    pub(crate) carrier: Option<LegacyGenericCarrierSummaryV1>,
    pub(crate) raw_schedule: Box<[crate::mir::loop_recipe_contract::route_id::LoopRouteId]>,
}

/// Test-only adapter at the legacy facts owner. It exposes a value summary so
/// sibling test seams never name private LoopFacts or canonicalize a winner.
#[cfg(test)]
pub(crate) fn observe_legacy_generic_loop_for_test(
    condition: &crate::ast::ASTNode,
    body: &[crate::ast::ASTNode],
) -> LegacyGenericLoopObservationV1 {
    let facts = match control_flow::plan::facts::try_build_loop_facts(condition, body) {
        Ok(Some(facts)) => facts,
        Ok(None) => {
            return LegacyGenericLoopObservationV1 {
                status: LegacyGenericFactsStatusV1::Absent,
                v0_present: false,
                v1_present: false,
                carrier: None,
                raw_schedule: Box::new([]),
            }
        }
        Err(freeze) => {
            return LegacyGenericLoopObservationV1 {
                status: LegacyGenericFactsStatusV1::Frozen(freeze.tag),
                v0_present: false,
                v1_present: false,
                carrier: None,
                raw_schedule: Box::new([]),
            }
        }
    };
    let v0_present = facts.generic_loop_v0().is_some();
    let carrier = facts.generic_loop_v1().map(|facts| {
        use control_flow::plan::facts::GenericLoopCarrierObservationV1;

        match &facts.carrier_observation {
            GenericLoopCarrierObservationV1::CompleteNoRecursiveCarrier => {
                LegacyGenericCarrierSummaryV1::CompleteNoRecursive
            }
            GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(bindings) => {
                LegacyGenericCarrierSummaryV1::CompleteRecursive(
                    bindings.clone().into_boxed_slice(),
                )
            }
            GenericLoopCarrierObservationV1::Unavailable(reason) => {
                LegacyGenericCarrierSummaryV1::Unavailable(reason.clone())
            }
            GenericLoopCarrierObservationV1::Ambiguous(reason) => {
                LegacyGenericCarrierSummaryV1::Ambiguous(reason.clone())
            }
        }
    });
    let v1_present = carrier.is_some();
    let canonical = control_flow::lower::normalize::canonicalize_loop_facts(facts);
    let selection =
        control_flow::joinir::route_entry::registry::select_recipe_first_routes(Some(&canonical));
    LegacyGenericLoopObservationV1 {
        status: LegacyGenericFactsStatusV1::Available,
        v0_present,
        v1_present,
        carrier,
        raw_schedule: selection.raw_execution_routes().to_vec().into_boxed_slice(),
    }
}

#[cfg(test)]
pub(crate) fn reset_loop_physical_effect_probe() {
    control_flow::reset_loop_physical_effect_probe();
}

#[cfg(test)]
pub(crate) fn take_loop_physical_effect_probe() -> usize {
    control_flow::take_loop_physical_effect_probe()
}
mod weak_field_write_route;

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use control_flow::detect_skip_whitespace_shape;
// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use control_flow::detect_read_digits_loop_true_shape;
// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use control_flow::detect_continue_shape;
// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use control_flow::detect_parse_number_shape;
pub(crate) use control_flow::detect_parse_string_shape;
// Phase 91 P5b: Re-export escape skip shape detection for loop_canonicalizer
pub(crate) use control_flow::detect_escape_skip_shape;

/// Phase 129: Public (crate) wrapper for StepTree capability guard.
///
/// `control_flow` is intentionally private to keep control-flow entrypoints centralized.
/// Shadow pipelines outside `mir::builder` must call this wrapper instead of reaching into
/// `control_flow::*` directly.
pub(crate) fn check_step_tree_capabilities(
    tree: &crate::mir::control_tree::StepTree,
    func_name: &str,
    strict: bool,
    dev: bool,
) -> Result<(), String> {
    let planner_required = crate::config::env::joinir_dev::planner_required_enabled();
    control_flow::joinir::control_tree_capability_guard::check(
        tree,
        func_name,
        strict,
        dev,
        planner_required,
    )
}
mod exprs_peek; // peek expression
mod exprs_qmark; // ?-propagate
mod field_facts; // Field/property receiver facts and declared-type helpers
mod field_receiver_provenance; // Bounded current-receiver Copy/Phi proof; one field-fact consumer
mod fields; // field access/assignment lowering split
mod if_form;
mod indexing; // indexing expression/assignment lowering
mod raw_lambda_capture_lifecycle; // consuming raw Lambda capture/publication lifecycle
mod raw_lambda_closure_emission; // source-neutral NewClosure/body publication terminal
mod raw_lambda_lexical_observation; // source-only raw Lambda lexical observer
mod weak_field_write;
// Phase 29bq+: sealing 層中立化
use control_flow::edgecfg::api::FragEmitSession;
mod declaration_order; // Deterministic box-member traversal owner
pub mod joinir_id_remapper; // Phase 189: JoinIR ID remapping (ValueId/BlockId translation) - Public for tests
mod joinir_inline_boundary_injector; // Phase 189: JoinInlineBoundary Copy instruction injector
mod loop_api_impl; // CLEAN-D: LoopBuilderApi wiring kept inside builder layer
mod module_compat_policy; // CUT0-S0-COMPAT0 ingress policy snapshot
#[cfg(test)]
mod module_compat_policy_p0; // CUT0-S0-COMPAT0 typed failure fixtures
mod module_finalization_declaration_metadata; // Shared finalizer declaration-metadata handoff
mod module_finalization_function_metadata; // Shared finalizer function-metadata handoff
#[cfg(test)]
mod module_invocation_cut0_p0; // CUT0-P0 disconnected all-route adapter
mod module_lifecycle; // Phase 29bq+: Module lifecycle orchestrator (prepare → lower → finalize)
#[cfg(test)]
mod module_lifecycle_capture_tests;
mod normal_script_deferred_residual_registry; // named selected-Script residual ownership
mod normal_script_direct_statement_owner; // Selected Script direct statement terminals
mod normal_script_program_item_admission; // Selected Script Program-item source admission
#[cfg(test)]
mod normal_script_root_admission_witness; // selected Script root shape/disposition proof
mod normal_script_root_demand_window; // Selected Script source-only semantic demand receipt
mod normal_script_runtime_block_port;
mod normal_script_runtime_work; // Selected Script runtime Box callable admission
mod normal_script_resolution; // typed Script resolver outcome transport
#[cfg(test)]
mod normal_script_selected_occurrence; // typed selected-Script work-plan-to-semantics handoff
mod normal_top_level_function_admission; // Selected top-level callable source/physical admission
mod ops;
mod phi;
#[allow(dead_code)]
mod phi_completion; // PHI0-S0: disconnected semantic completion vocabulary
mod phi_merge;
#[allow(dead_code)]
mod phi_type_publication;
#[allow(dead_code)]
mod port_aware_function_draft_impl;
mod program_declaration_facts; // Normal Program source-only declaration facts
mod program_root_work_plan; // Normal Program source-only work partition
mod program_static_table_metadata; // Normal Program paired static-table metadata
mod return_type_strategy; // finalization-only return-type strategy
mod type_hint_providers; // Phase 29bq+: Type hint provision (call results, method signatures) // Phase 25.1q: Unified PHI merge helper // prepare/lower_root/finalize split
                         // legacy large-match remains inline for now (planned extraction)
pub(in crate::mir) mod emission; // emission::*（Const/Compare/Branch の薄い発行箱）
pub(crate) use emission::copy_emitter;
mod emit_guard; // EmitGuardBox（emit直前の最終関所）
mod metadata; // MetadataPropagationBox（type/originの伝播）
mod name_const; // NameConstBox（関数名Const生成）
mod observe; // P0: dev-only observability helpers（ssa/resolve）
mod origin; // P0: origin inference（me/Known）と PHI 伝播（軽量）
mod plugin_sigs; // plugin signature loader
mod properties;
mod property_reads;
mod receiver; // ReceiverMaterializationBox（Method recv の pin+LocalSSA 集約）
mod record_helper_args; // RECORD-VALUE-HELPER-001: local record helper argument scalarization
mod record_values; // C205b: builder-local record value scalarization
pub(in crate::mir) mod resolved_lowering; // sealed source/product -> exact BindingRef lowering
pub(in crate::mir) use resolved_lowering::issue_selected_dynamic_v2_emission_plan;
#[cfg(test)]
pub(in crate::mir) use resolved_lowering::issue_selected_dynamic_v2_physical_capability_admission;
pub(in crate::mir) use resolved_lowering::with_common_v2_canonical_session;
pub(in crate::mir) use resolved_lowering::CanonicalResolvedBuildErrorV1;
mod rewrite; // P1: Known rewrite & special consolidation
mod router; // RouterPolicyBox（Unified vs BoxCall）
mod schedule; // BlockScheduleBox（物理順序: PHI→materialize→body）
mod scope_context; // Phase 136 follow-up (Step 3/7): ScopeContext extraction
mod ssa; // LocalSSA helpers (in-block materialization)
mod static_scalar_facts; // Narrow verified static-scalar method fact surface
mod stmts;
mod type_context; // Phase 136 follow-up: TypeContext extraction
mod type_facts; // Phase 136 follow-up: Type inference facts box
pub(crate) mod type_registry;
mod types; // types::annotation / inference（型注釈/推論の箱: 推論は後段）
pub(crate) use types::annotation::{
    infer_method_return_type as infer_known_method_return_type,
    infer_return_type as infer_known_return_type,
};
mod utils;
mod vars; // variables/scope helpers // small loop helpers (header/exit context) // TypeRegistryBox（型情報管理の一元化）
          // Phase 288 Box化: repl_session moved to src/runner/repl/repl_session.rs

/// MIR builder for converting AST to SSA form
pub struct MirBuilder {
    /// Current module being built
    pub(super) current_module: Option<MirModule>,

    /// The sole physical owner of every FunctionOwned lowering surface.
    ///
    /// Module, observer, and legacy-compatibility state intentionally remains
    /// outside this component. S0b preserves the existing prepare/restore
    /// transaction; S0c owns its later consolidation.
    function_state: function_lowering_state::FunctionLoweringStateV1,

    /// Phase 136 follow-up (Step 2/7): Core ID generation context
    /// Consolidates value_gen, block_gen, next_binding_id, temp_slot_counter, debug_join_counter.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) core_ctx: CoreContext,

    /// Observation-only scope state. FunctionOwned scope leaves live in
    /// `function_state.scope`.
    pub(super) scope_ctx: scope_context::ScopeContext,

    /// Observation-only metadata. FunctionOwned ValueId origins live in
    /// `function_state.value_origins`.
    pub(super) metadata_ctx:
        metadata_context::MetadataContext<crate::ast::Span, crate::mir::region::RegionId>,

    /// Module, observation, and compatibility compilation state. FunctionOwned
    /// reservations/body/record scratch live in `function_state.compilation`.
    pub(super) comp_ctx: compilation_context::CompilationContext,

    /// Recursion depth counter for debugging stack overflow.
    /// Tracks raw recursive expression descent to detect infinite loops.
    pub(super) recursion_depth: usize,

    /// Root lowering mode: how to treat top-level Program
    /// - None: not decided yet (lower_root not called)
    /// - Some(true): App mode (static box Main.main is entry)
    /// - Some(false): Script/Test mode (top-level Program runs sequentially)
    pub(super) root_is_app_mode: Option<bool>,

    /// Phase 288 P2: REPL mode flag - enables implicit local declarations
    /// File mode: false (explicit local required)
    /// REPL mode: true (暗黙 local 許可)
    pub(crate) repl_mode: bool,
}

#[cfg(test)]
impl MirBuilder {
    /// Full candidate-abort fingerprint for the M1 proof fixture.
    ///
    /// This is test-only observation: it does not define a production snapshot
    /// or introduce a second publication path.
    pub(crate) fn loop_candidate_test_fingerprint(&self) -> String {
        format!(
            "{:?}",
            (
                &self.current_module,
                &self.function_state,
                &self.core_ctx,
                &self.scope_ctx,
                &self.metadata_ctx,
                &self.comp_ctx,
                self.recursion_depth,
                self.root_is_app_mode,
                self.repl_mode,
            )
        )
    }
}

impl Default for MirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod binding_id_tests {
    use super::*;

    #[test]
    fn test_binding_map_initialization() {
        let builder = MirBuilder::new();
        assert_eq!(builder.core_ctx.next_binding_id, 0);
        // Phase 2-6: binding_ctx is now SSOT (legacy field removed)
        assert!(builder.function_state.binding_ctx.is_empty());
    }

    #[test]
    fn test_binding_allocation_sequential() {
        let mut builder = MirBuilder::new();
        let bid0 = builder.allocate_binding_id().unwrap();
        let bid1 = builder.allocate_binding_id().unwrap();
        let bid2 = builder.allocate_binding_id().unwrap();

        assert_eq!(bid0.raw(), 0);
        assert_eq!(bid1.raw(), 1);
        assert_eq!(bid2.raw(), 2);
        assert_eq!(builder.core_ctx.next_binding_id, 3);
    }

    #[test]
    fn test_shadowing_binding_restore() {
        let mut builder = MirBuilder::new();

        // Simulate function entry scope
        builder.push_lexical_scope_for_test();

        // Declare outer x
        // Phase 136 P0: Use SSOT allocator for function scope simulation
        let outer_vid = builder.next_value_id();
        builder
            .declare_local_in_current_scope("x", outer_vid)
            .unwrap();
        // Phase 2-6: Check binding_ctx (SSOT)
        let outer_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
        assert_eq!(outer_bid.raw(), 0);

        // Enter inner scope and shadow x
        builder.push_lexical_scope_for_test();
        // Phase 136 P0: Use SSOT allocator for function scope simulation
        let inner_vid = builder.next_value_id();
        builder
            .declare_local_in_current_scope("x", inner_vid)
            .unwrap();
        // Phase 2-6: Check binding_ctx (SSOT)
        let inner_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
        assert_eq!(inner_bid.raw(), 1);

        // Exit inner scope - should restore outer binding
        builder.pop_lexical_scope_for_test();
        // Phase 2-6: Check binding_ctx (SSOT)
        let restored_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
        assert_eq!(restored_bid, outer_bid);
        assert_eq!(restored_bid.raw(), 0);

        // Cleanup
        builder.pop_lexical_scope_for_test();
    }

    #[test]
    fn test_valueid_binding_parallel_allocation() {
        let mut builder = MirBuilder::new();

        // Phase 136 P0: Use SSOT allocator (next_value_id)
        // Note: Without current_function, next_value_id() falls back to value_gen.next()
        // so this test still validates ValueId/BindingId independence
        // Allocate ValueIds and BindingIds in parallel
        let vid0 = builder.next_value_id();
        let bid0 = builder.allocate_binding_id().unwrap();
        let vid1 = builder.next_value_id();
        let bid1 = builder.allocate_binding_id().unwrap();

        // ValueId and BindingId should be independent
        assert_eq!(vid0.0, 0);
        assert_eq!(bid0.raw(), 0);
        assert_eq!(vid1.0, 1);
        assert_eq!(bid1.raw(), 1);

        // Allocating more ValueIds should not affect BindingId counter
        let _ = builder.next_value_id();
        let _ = builder.next_value_id();
        let bid2 = builder.allocate_binding_id().unwrap();
        assert_eq!(bid2.raw(), 2); // Still sequential

        // Allocating more BindingIds should not affect ValueId counter
        let _ = builder.allocate_binding_id().unwrap();
        let _ = builder.allocate_binding_id().unwrap();
        let vid2 = builder.next_value_id();
        assert_eq!(vid2.0, 4); // Continues from where we left off
    }
}

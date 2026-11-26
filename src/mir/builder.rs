/*!
 * MIR Builder - Converts AST to MIR/SSA form
 *
 * Implements AST → MIR conversion with SSA construction
 */

use super::{
    BasicBlockId, CompareOp, ConstValue, Effect, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirModule, MirType, ValueId,
};
pub(crate) use calls::CallTarget;
use hakorune_mir_builder::CoreContext;
mod array_element_write;
pub(in crate::mir) use array_element_write::CompletedArrayElementWriteV1;
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
mod callable_declaration_catalog; // Complete same-module callable declaration authority
mod literal_lowering;
#[cfg(test)]
mod literal_postemit_retirement_tests;
mod map_read_physical_consumer; // source-issued Map read Facts to typed MIR operations
mod new_expression;
mod normal_callable_binding_materialization;
mod normal_callable_binding_materialization_port; // existing formal ValueId handoff
mod normal_local_materialization;
mod normal_callable_catalog_owner_link; // exact catalog key/owner/source ingress co-seal
mod normal_callable_dynamic_origin; // source-backed Dynamic -> existing physical receipts
mod normal_callable_dynamic_source; // source-backed untyped formal/Loop carrier authority
mod normal_callable_loop_handoff; // callable Loop source/BindingRef S0 handoff
pub(in crate::mir) use normal_callable_loop_handoff::CallableLoopSourceBindingClassV1;
mod normal_callable_loop_source_facts; // callable source-aware Facts Ready issuer
pub(in crate::mir) use normal_callable_loop_source_facts::{
    issue_callable_loop_break_source_facts_v1, CallableLoopBreakSourceFactsDispositionV1,
    CallableLoopBreakSourceFactsIssueV1, VerifiedCallableLoopBreakSourceCandidateV1,
    VerifiedCallableLoopBreakSourceFactsV1,
};
mod normal_callable_loop_source_port; // source-aware callable-loop expression capability
pub(in crate::mir) use normal_callable_loop_source_facts::{
    issue_preissued_callable_loop_source_plan_v1, PreissuedCallableLoopSourcePlanV1,
    RetainedCallableLoopSourceV1,
};
mod normal_callable_loop_source_route; // callable source-owned LoopCond route token
mod normal_callable_package_bridge; // Builder-private package install boundary
                                    // Caller-zero until the Main0 canonical-root handoff wires the installed App
                                    // Main caller; recheck at the production-switch slice.
                                    // Caller-zero second bounded Main0 profile; same canonical-root handoff.
                                    // Caller-zero third bounded Main0 profile; same canonical-root handoff.
mod normal_callable_prepared_operation; // retained callable pipeline, F22 HOLD
mod normal_callable_semantic_loan_port; // Thin installed-package port adapter
mod normal_callable_semantic_lowering_state;
pub(in crate::mir) use normal_callable_semantic_lowering_state::validate_map_local_annotation;
mod function_fault_frame; // Shared physical frame; source owners select the role
mod normal_callable_semantic_source; // Co-sealed selected callable source authority
mod normal_callable_semantic_source_lookup; // Exact legacy source-site/view lookup during cutover
pub(in crate::mir) use control_flow::plan::LoopFactsPolicyFrameV1;
mod normal_cataloged_box_method_lowering;
mod ordinary_new_admission;
mod variable_read;
pub(in crate::mir) use callable_declaration_catalog::issue_source_backed_same_module_callable_catalog_v1;
pub(crate) use callable_declaration_catalog::{
    AppMainCatalogCoSealV1, CanonicalSameModuleCallableKeyV1, SameModuleCallableCatalogBrandV1,
    SameModuleCallableNamespaceV1, SelectedCallableConsumptionRoleV1, SelectedNormalCallableKeyV1,
    SourceBackedCallableCatalogIssueV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    VerifiedSameModuleCallableDeclarationV1, VerifiedSelectedNormalCallableSourceInventoryV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
pub(in crate::mir) use main_expansion::{
    MainExpansionErrorV1, PreparedAdmittedNormalRootExpansionV1, VerifiedMainStaticChildV1,
};
pub(in crate::mir) use normal_callable_catalog_owner_link::{
    issue_catalog_callable_owner_link_v1, CatalogCallableOwnerLinkIssueV1,
    VerifiedCatalogCallableOwnerLinkV1,
};
pub(in crate::mir) use normal_callable_dynamic_source::{
    issue_source_backed_dynamic_callable_v1, VerifiedSourceBackedDynamicCallableV1,
};
#[cfg(test)]
pub(in crate::mir) use normal_callable_semantic_source::NormalCallableSemanticAdmissionV1;
pub(in crate::mir) use normal_callable_semantic_source::VerifiedNormalCallableSemanticSourceV1;
mod calls; // Call system modules (refactored from builder_calls)
mod collection_literals; // ArrayLiteral / MapLiteral lowering
mod compilation_context; // Phase 136 follow-up (Step 7/7): CompilationContext extraction
pub(crate) use compilation_context::CompilationContext;
pub(in crate::mir) use normal_callable_package_bridge::{
    BuilderInstallConsumerV1, BuilderPrivateCallableLoweringScopeV1,
    BuilderPrivateInstalledCallablePackageBundleV1,
};
mod compound_assignment; // evaluated Place read-modify-write lowering
mod decls; // declarations lowering split
mod emit_debug_policy;
mod entry_materialization; // source-only callable Main materialization facts
mod enum_match_scopebox;
mod exprs_check; // CheckExpr lowering
mod fastmem; // fastmem source -> MIR MemOp metadata lowering
mod fastmem_context; // FastMemory region context helpers
mod function_lowering_state;
mod function_signature_lookup; // HEADERPORT0-S0 neutral header lookup surface
mod function_state_transaction;
mod generic_loop_admission_observation;
mod instance_box_constructor_batch;
mod instance_box_declaration_lifecycle;
mod instance_box_declaration_metadata;
mod instance_box_method_batch;
mod main_expansion; // HEADERPORT0-I0-MAINROLE0-S0 source-only Main expansion
mod nested_box_method_source;
mod normal_instance_constructor_admission;
mod normal_instance_constructor_demand_loan;
mod normal_instance_constructor_semantic_scope;
mod normal_main_arguments;
mod normal_root_execution;
mod normal_runtime_inputs; // selected normal ingress runtime snapshot
mod normal_script_composite_partition;
mod normal_script_direct_static_lookup;
mod normal_script_instance_box_transfer;
mod normal_script_neutral_window;
mod raw_static_main_compat_batch; // explicit raw static-Main source partition
#[cfg(test)]
pub(in crate::mir) use normal_module_transaction::{
    canonical_normal_main_entry_target, CanonicalNormalMainEntryTargetV1,
};
mod raw_source_projection; // RAW-SOURCE0-PLAN0 owned source locators
mod raw_compat_runtime_box_fate;
mod raw_compatibility_child_terminal;
pub(in crate::mir) use entry_materialization::{
    CallableMainMaterializationPolicyV1, CallableMainMaterializationTargetV1,
    NormalEntryMaterializationSourceReceiptV1, RawEntryMaterializationSourceReceiptV1,
};
pub(in crate::mir) use normal_main_arguments::MainArgumentsEmissionV1;
#[cfg(test)]
pub(in crate::mir) use normal_root_execution::NormalRootExecutionConsumerRejectV1;
pub(in crate::mir) use normal_root_execution::{
    AdmittedNormalRootExecutionModeV1, ConsumedNormalRootCallableSourceV1,
    NormalRootExecutionConsumerV1,
};
pub(in crate::mir) use normal_runtime_inputs::NormalRuntimeInputSnapshotV1;
pub(in crate::mir) use raw_source_projection::{OwnedRawRootProjectionV1, RawSourceLocatorV1};
#[cfg(test)]
pub(in crate::mir) use raw_source_projection::{OwnedRawSourceV1, RawSourceOriginV1};
mod me_call_header_observation; // ACCESS0-MEHEADER-S0 typed source snapshot
mod metadata_context; // Phase 136 follow-up (Step 6/7): MetadataContext extraction
mod method_call_handlers;
mod module_draft_collector;
#[cfg(test)]
mod module_draft_collector_receipt_p0; // ROUTEINV-P0a-RECEIPT-P0 matrix
#[cfg(test)]
mod module_draft_collector_receipt_tests; // ROUTEINV-P0a-RECEIPT-S0 fixtures
#[cfg(test)]
mod module_finalization_candidate_p0; // HEADERPORT0-I0-MODULEFINAL0-CANDIDATE0-P0 matrix
mod module_invocation_identity; // CUT0-I0-ID0-S0 disconnected identity/token
#[cfg(test)]
mod module_invocation_identity_p0; // CUT0-I0-ID0-S0 fixtures
#[cfg(test)]
mod module_invocation_owner_chain;
#[cfg(test)]
mod normal_module_transaction; // NORMAL-MODULE-TX0-L0 disconnected schema
mod module_invocation_session; // Shared isolated Builder transaction
mod pinned_text_invocation_binding; // Session-owned pinned-Text target/brand ingress
pub(in crate::mir::builder) use module_invocation_session::UnpublishedCallableLoopRootScopeV1;
pub(in crate::mir) use module_invocation_session::{
    BuilderInvocationConfigV1, ModuleBuilderInvocationSessionV1, PreparedBuilderExternalCommitV1,
};
#[cfg(test)]
mod module_invocation_session_p0; // CUT0-I0-SESSION0 fixtures
mod normal_cataloged_box_method_admission; // Selected normal cataloged-child identity
pub(in crate::mir) use normal_cataloged_box_method_admission::{
    CatalogedBoxMethodPhysicalHeaderProjectionV1, NormalCatalogedBoxMethodAdmissionErrorV1,
    NormalCatalogedBoxMethodDraftAdmissionV1,
};
mod normal_default_program_root; // Closed source-backed/compatibility root ingress
mod normal_default_root_catalog_lifecycle; // Selected normal root/catalog lifecycle
mod normal_default_root_catalog_post_install; // Existing post-install lowering consumer
mod normal_script_boundary_receipt_pack; // Script retained boundary receipts
mod normal_script_direct_static_join_handoff; // Script source/Facts Recipe handoff
mod normal_script_direct_static_physical_publication; // Script ExactI64 physical publication
#[path = "builder/normal_script/direct_static/semantic/normal_script_direct_static_recipe.rs"]
mod normal_script_direct_static_recipe; // Dedicated Script direct-static Recipe producer
mod normal_script_direct_static_result_bundle; // Script source/result Facts bundle
mod normal_script_direct_static_result_publication_owner; // Script source/Facts result owner
mod normal_script_operational_demand_receipt_pack; // Script structured demand receipts
mod normal_script_pre_effect_source_observation; // AST-free Script source handoff before Builder effects
mod normal_script_semantic_lowering_input; // Retained Script source products into lowering
mod normal_script_semantic_lowering_projection; // Immutable Script lowering projection
mod normal_script_semantic_lowering_state; // Script BindingRef -> ValueId ledger
pub(crate) use normal_script_semantic_lowering_state::FinalizedScriptArrayV1;
pub(crate) use normal_script_source_continuation::RootResult as ScriptArrayRootResultV1;
mod normal_script_semantic_source; // Producer-backed lexical Script source
mod normal_script_semantic_source_core; // Shared Script source/forest/projection core
mod normal_script_source_continuation; // Resolver-issued Script source continuation
#[cfg(test)]
#[path = "builder/normal_script_source_continuation_tests.rs"]
mod normal_script_source_continuation_tests;
mod program_root_lowering; // Shared typed/generic Program root owner
pub(in crate::mir) use normal_default_program_root::PreparedNormalDefaultProgramRootV1;
pub(in crate::mir) use normal_default_root_catalog_lifecycle::CompletedNormalDefaultRootCatalogLifecycleV1;
#[cfg(test)]
pub(in crate::mir) use normal_default_root_catalog_lifecycle::NormalDefaultRootCatalogLifecycleStageV1;
mod module_lowering_invocation;
mod module_lowering_invocation_legacy_term;
#[cfg(test)]
mod module_lowering_invocation_reentrant_tests;
mod nonmain_static_box_method_batch;
#[cfg(test)]
mod runtime_box_fate_tests;
mod raw_expression_dispatch; // single raw AST expression dispatcher
mod raw_loop_child_entry; // LOOPBRIDGE0-S0 pure raw Loop child-entry quarantine
mod raw_loop_child_port; // single located-source Loop child-entry boundary
mod enum_variant_source_demand;
mod raw_expression_recursion_guard;
mod raw_invocation_body;
mod raw_invocation_source_item_site;
mod raw_invocation_source_statement_classification;
mod raw_invocation_source_transport;
mod raw_structured_child_scope;
mod record_literal_source_demand;
mod recursive_child_lowering;
mod recursive_child_lowering_port;
#[cfg(test)]
mod recursive_child_lowering_port_tests;
#[cfg(test)]
mod recursive_child_lowering_rawport_header_tests;
#[cfg(test)]
mod recursive_child_lowering_rawport_tests;
#[cfg(test)]
mod recursive_child_lowering_tests;
mod static_result_publication_ingress;
mod variable_context; // Phase 136 follow-up (Step 5/7): VariableContext extraction // Method call handler separation (Phase 3) // call(expr)
                      // include lowering removed (using is handled in runner)
mod control_flow; // thin wrappers to centralize control-flow entrypoints
pub(in crate::mir) use control_flow::plan::lowerer::{
    capture_exact_loop_phis_v2, EmittedSourceLoopGraphV2, VerifiedLoopCondAfterPhysicalRoleV1,
};
pub(in crate::mir) use emission::constant::CompletedConstV1;
pub(in crate::mir) use normal_callable_loop_source_facts::{
    CompletedSourceLoopPhiPointsV2, PreparedSourceLoopPhiCompletionV2,
};
pub(in crate::mir) use ordinary_new_admission::selected::source_loop_cleanup::{
    EmittedSourceCleanupV2, EmittedSourceLoopCleanupV2,
};
pub(in crate::mir) use ordinary_new_admission::selected::{
    EmittedExpressionBodyV1, ExpressionOwnedTemporaryV1,
};

mod weak_field_write_route;


mod condition_materialization;
mod field_facts; // Field/property receiver facts and declared-type helpers
mod field_receiver_provenance; // Bounded current-receiver Copy/Phi proof; one field-fact consumer
mod fields; // field access/assignment lowering split
pub(in crate::mir) use condition_materialization::ConditionMaterializationV1;
mod if_form;
pub(in crate::mir) use if_form::IfConditionMaterializationV1;
mod indexing; // indexing expression/assignment lowering
mod weak_field_write;
// Phase 29bq+: sealing 層中立化
mod declaration_order; // Deterministic box-member traversal owner
#[cfg(test)]
pub mod joinir_id_remapper; // Reference-only JoinIR ID remapping (ValueId/BlockId translation)
mod loop_api_impl; // CLEAN-D: LoopBuilderApi wiring kept inside builder layer
mod mir_value_id_inventory; // Active lifecycle ValueId inventory; no remap authority
mod module_compat_policy; // CUT0-S0-COMPAT0 ingress policy snapshot
#[cfg(test)]
mod module_compat_policy_p0; // CUT0-S0-COMPAT0 typed failure fixtures
mod module_finalization_declaration_metadata; // Shared finalizer declaration-metadata handoff
mod module_finalization_function_metadata; // Shared finalizer function-metadata handoff
mod module_lifecycle; // Phase 29bq+: Module lifecycle orchestrator (prepare → lower → finalize)
#[cfg(test)]
mod module_lifecycle_capture_tests;
mod normal_script_deferred_residual_registry; // named selected-Script residual ownership
mod normal_script_direct_statement_owner; // Selected Script direct statement terminals
mod normal_script_program_item_admission; // Selected Script Program-item source admission
mod normal_script_resolution; // typed Script resolver outcome transport
#[cfg(test)]
mod normal_script_root_admission_witness; // selected Script root shape/disposition proof
mod normal_script_root_demand_window; // Selected Script source-only semantic demand receipt
mod normal_script_runtime_block_port;
mod normal_script_runtime_work; // Selected Script runtime Box callable admission
#[cfg(test)]
mod normal_script_selected_occurrence; // typed selected-Script work-plan-to-semantics handoff
pub(in crate::mir) mod ops;
mod phi;
mod phi_completion; // PHI0-S0: disconnected semantic completion vocabulary
mod phi_merge;
pub(in crate::mir) use phi_merge::reusable_branch_value_v1;
mod phi_type_publication;
mod port_aware_function_draft_impl;
mod program_declaration_facts; // Normal Program source-only declaration facts
mod program_root_work_plan; // Normal Program source-only work partition
mod program_static_table_metadata; // Normal Program paired static-table metadata
#[cfg(test)]
mod raw_compat_child_terminal_tests; // phase2160 RawCompatibility terminal evidence
mod return_type_strategy; // finalization-only return-type strategy
mod type_hint_providers; // Phase 29bq+: Type hint provision (call results, method signatures) // Phase 25.1q: Unified PHI merge helper // prepare/lower_root/finalize split
                         // legacy large-match remains inline for now (planned extraction)
pub(in crate::mir) mod emission; // emission::*（Const/Compare/Branch の薄い発行箱）
pub(crate) use emission::copy_emitter;
mod emit_guard; // EmitGuardBox（emit直前の最終関所）
mod metadata; // MetadataPropagationBox（type/originの伝播）
mod observe; // P0: dev-only observability helpers（ssa/resolve）
mod origin; // P0: origin inference（me/Known）と PHI 伝播（軽量）
mod plugin_sigs; // plugin signature loader
mod properties;
mod property_reads;
mod receiver; // ReceiverMaterializationBox（Method recv の pin+LocalSSA 集約）
mod record_helper_args; // RECORD-VALUE-HELPER-001: local record helper argument scalarization
mod record_values; // C205b: builder-local record value scalarization
pub(in crate::mir) mod resolved_lowering; // sealed source/product -> exact BindingRef lowering
pub(in crate::mir) use resolved_lowering::CanonicalResolvedBuildErrorV1;
mod rewrite; // P1: Known rewrite & special consolidation
mod router; // RouterPolicyBox（Unified vs BoxCall）
mod schedule; // BlockScheduleBox（物理順序: PHI→materialize→body）
mod scope_context; // Phase 136 follow-up (Step 3/7): ScopeContext extraction
mod ssa; // LocalSSA helpers (in-block materialization)
pub(in crate::mir) use ssa::local::{LocalSsaPhysicalIndexV1, LocalSsaPhysicalRelationV1};
mod static_scalar_facts; // Narrow verified static-scalar method fact surface
mod stmts;
pub(in crate::mir) use stmts::variable_assignment_completion::CompletedVariableAssignmentV1;
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
#[path = "builder/builder_binding_id_tests.rs"]
mod binding_id_tests;

pub(in crate::mir) use normal_callable_semantic_lowering_state::{
    BindingValueObservationV2, ExactLexicalReadV1, LocalProvenanceV1,
};
#[cfg(test)]
mod lexical_call_projection_test_fixture;
#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::document_completion_fixture as lexical_call_projection_document_completion_fixture;
#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::fixture as lexical_call_projection_test_fixture;

#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::artifact_fixture as lexical_call_projection_artifact_fixture;
#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::assert_artifact_reseal_rejected as assert_lexical_call_projection_reseal_rejected;
#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::finished_artifact_fixture as lexical_call_projection_finished_artifact_fixture;

#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::borrowed_fixture as lexical_call_projection_borrowed_fixture;

#[cfg(test)]
pub(in crate::mir) use lexical_call_projection_test_fixture::forwarded_fixture as lexical_call_projection_forwarded_fixture;

pub(in crate::mir) use normal_callable_semantic_lowering_state::construction::RetainedConstructionDrafts;

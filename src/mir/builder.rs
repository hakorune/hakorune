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
mod builder_build;
mod builder_debug;
mod builder_emit;
mod builder_init;
mod builder_metadata;
mod builder_method_index;
#[cfg(test)]
mod builder_test_api;
mod builder_value_kind;
mod call_resolution; // ChatGPT5 Pro: Type-safe call resolution utilities
mod callable_declaration_catalog; // Complete same-module callable declaration authority
#[cfg(test)]
mod literal_postemit_retirement_tests;
pub(crate) use callable_declaration_catalog::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1, VerifiedSameModuleCallableDeclarationV1,
};
mod calls; // Call system modules (refactored from builder_calls)
mod collection_literals; // ArrayLiteral / MapLiteral lowering
mod compilation_context; // Phase 136 follow-up (Step 7/7): CompilationContext extraction
mod compound_assignment; // evaluated Place read-modify-write lowering
mod decls; // declarations lowering split
#[allow(dead_code)]
mod drained_module_candidate; // HEADERPORT0-I0-DRAIN0-S0 disconnected candidate
#[cfg(test)]
mod drained_module_candidate_p0; // HEADERPORT0-I0-DRAIN0-P0 fixtures
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
mod located_legacy_lowering;
#[allow(dead_code)]
mod main_expansion; // HEADERPORT0-I0-MAINROLE0-S0 source-only Main expansion
#[allow(dead_code)]
mod main_pending_draft; // HEADERPORT0-I0-MAINPENDING0-S0 disconnected handoff
#[cfg(test)]
mod main_pending_draft_p0; // HEADERPORT0-I0-MAINPENDING0-P0 fixtures
#[allow(dead_code)]
mod me_call_header_observation; // ACCESS0-MEHEADER-S0 typed source snapshot
mod metadata_context; // Phase 136 follow-up (Step 6/7): MetadataContext extraction
mod method_call_handlers;
mod module_completion_candidate;
#[allow(dead_code)]
mod module_declaration_facts; // HEADERPORT0-I0-SHELLFACT0-S0 disconnected facts
#[cfg(test)]
mod module_declaration_facts_p0; // HEADERPORT0-I0-SHELLFACT0-P0 fixtures
mod module_draft_collector;
#[allow(dead_code)]
mod module_invocation_drain; // HEADERPORT0 I0-SHELL-I0-S0 disconnected drain
#[allow(dead_code)]
mod module_invocation_route_matrix; // HEADERPORT0 I0-SHELL-I0-P0 disconnected matrix
#[allow(dead_code)]
mod module_lowering_access_port; // HEADERPORT0 I0-ACCESS0-S0 disconnected vocabulary
mod module_lowering_invocation;
#[allow(dead_code)]
mod module_lowering_invocation_candidate; // HEADERPORT0 CANDIDATE0-S0 disconnected abort owner
#[allow(dead_code)]
mod module_lowering_invocation_candidate_p0; // HEADERPORT0 CANDIDATE0-P0 route co-seal
#[cfg(test)]
mod module_lowering_invocation_legacyterm_tests;
#[cfg(test)]
mod module_lowering_invocation_reentrant_tests;
#[allow(dead_code)]
mod module_lowering_invocation_state; // HEADERPORT0 I0-STATE0-S0 disconnected seam
#[allow(dead_code)]
mod module_lowering_shell; // HEADERPORT0 I0-SHELL-S0 disconnected shell
mod raw_expression_dispatch; // single raw AST expression dispatcher
#[allow(dead_code)]
mod raw_loop_child_entry; // LOOPBRIDGE0-S0 pure raw Loop child-entry quarantine
#[allow(dead_code)]
mod root_body_completion; // HEADERPORT0-I0-BODYDRAIN0-S0 disconnected witness
#[cfg(test)]
mod root_body_completion_p0; // HEADERPORT0-I0-BODYDRAIN0-P0 fixtures
#[allow(dead_code)]
mod root_draft_batch; // HEADERPORT0-I0-ROOTBATCH0-S0 disconnected batch
#[cfg(test)]
mod root_draft_batch_p0; // HEADERPORT0-I0-ROOTBATCH0-P0 fixtures // HEADERPORT0-I0-SHELLFACT0-S0 disconnected facts
#[allow(unused_imports)]
pub(in crate::mir) use located_legacy_lowering::{
    LocatedLegacyLoweringErrorV1, LocatedLegacyLoweringSessionV1,
};
#[cfg(test)]
mod phi_observation_tests;
mod recursive_child_lowering;
#[cfg(test)]
mod recursive_child_lowering_rawport_tests;
#[cfg(test)]
mod recursive_child_lowering_tests;
mod variable_context; // Phase 136 follow-up (Step 5/7): VariableContext extraction // Method call handler separation (Phase 3) // call(expr)
                      // include lowering removed (using is handled in runner)
mod control_flow; // thin wrappers to centralize control-flow entrypoints
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
mod exprs_lambda; // lambda lowering
mod exprs_peek; // peek expression
mod exprs_qmark; // ?-propagate
mod field_facts; // Field/property receiver facts and declared-type helpers
mod field_receiver_provenance; // Bounded current-receiver Copy/Phi proof; one field-fact consumer
mod fields; // field access/assignment lowering split
mod if_form;
mod indexing; // indexing expression/assignment lowering
mod weak_field_write;
// Phase 29bq+: sealing 層中立化
use control_flow::edgecfg::api::FragEmitSession;
mod declaration_indexer; // Phase 29bq+: Declaration indexing (user boxes, static methods)
mod declaration_order; // Deterministic box-member traversal owner
pub mod joinir_id_remapper; // Phase 189: JoinIR ID remapping (ValueId/BlockId translation) - Public for tests
mod joinir_inline_boundary_injector; // Phase 189: JoinInlineBoundary Copy instruction injector
mod loop_api_impl; // CLEAN-D: LoopBuilderApi wiring kept inside builder layer
mod module_lifecycle; // Phase 29bq+: Module lifecycle orchestrator (prepare → lower → finalize)
mod ops;
mod phi;
#[allow(dead_code)]
mod phi_completion; // PHI0-S0: disconnected semantic completion vocabulary
mod phi_merge;
mod phi_type_inference; // Phase 29bq+: PHI type inference (multi-phase fallback chain)
#[allow(dead_code)]
mod phi_type_publication;
#[allow(dead_code)]
mod port_aware_function_draft; // HEADERPORT0-S0 disconnected body/finalizer vocabulary
#[allow(dead_code)]
mod port_aware_function_draft_impl; // HEADERPORT0-P0 port-aware draft siblings
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
mod resolved_lowering; // sealed source/product -> exact BindingRef lowering
pub(in crate::mir) use resolved_lowering::CanonicalResolvedBuildErrorV1;
mod rewrite; // P1: Known rewrite & special consolidation
mod router; // RouterPolicyBox（Unified vs BoxCall）
mod schedule; // BlockScheduleBox（物理順序: PHI→materialize→body）
mod scope_context; // Phase 136 follow-up (Step 3/7): ScopeContext extraction
mod ssa; // LocalSSA helpers (in-block materialization)
mod static_scalar_facts; // Narrow verified static-scalar method fact surface
mod stmts;
#[cfg(test)]
pub(crate) use stmts::block_suffix_parity_reference::{
    run_block_suffix_parity_reference_v1, BlockSuffixParityInputV1, StatementDescentReferenceV1,
};
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

    /// Recursion depth counter for debugging stack overflow
    /// Tracks the depth of build_expression calls to detect infinite loops
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
        builder.push_lexical_scope();

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
        builder.push_lexical_scope();
        // Phase 136 P0: Use SSOT allocator for function scope simulation
        let inner_vid = builder.next_value_id();
        builder
            .declare_local_in_current_scope("x", inner_vid)
            .unwrap();
        // Phase 2-6: Check binding_ctx (SSOT)
        let inner_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
        assert_eq!(inner_bid.raw(), 1);

        // Exit inner scope - should restore outer binding
        builder.pop_lexical_scope();
        // Phase 2-6: Check binding_ctx (SSOT)
        let restored_bid = builder.function_state.binding_ctx.lookup("x").unwrap();
        assert_eq!(restored_bid, outer_bid);
        assert_eq!(restored_bid.raw(), 0);

        // Cleanup
        builder.pop_lexical_scope();
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

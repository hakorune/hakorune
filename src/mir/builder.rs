/*!
 * MIR Builder - Converts AST to MIR/SSA form
 *
 * Implements AST → MIR conversion with SSA construction
 */

use super::{
    BasicBlock, BasicBlockId, CompareOp, ConstValue, Effect, EffectMask, FunctionSignature,
    MirFunction, MirInstruction, MirModule, MirType, ValueId,
};
pub(crate) use builder_calls::CallTarget;
use hakorune_mir_builder::{BindingContext, CoreContext};
use std::collections::HashMap;
mod builder_build;
mod builder_calls;
mod builder_debug;
mod builder_emit;
mod builder_init;
mod builder_metadata;
mod builder_method_index;
#[cfg(test)]
mod builder_test_api;
mod builder_value_kind;
mod call_resolution; // ChatGPT5 Pro: Type-safe call resolution utilities
mod calls; // Call system modules (refactored from builder_calls)
mod compilation_context; // Phase 136 follow-up (Step 7/7): CompilationContext extraction
mod decls; // declarations lowering split
mod exprs; // expression lowering split
mod exprs_call;
mod metadata_context; // Phase 136 follow-up (Step 6/7): MetadataContext extraction
mod method_call_handlers;
#[cfg(test)]
mod phi_observation_tests;
mod variable_context; // Phase 136 follow-up (Step 5/7): VariableContext extraction // Method call handler separation (Phase 3) // call(expr)
                      // include lowering removed (using is handled in runner)
mod control_flow; // thin wrappers to centralize control-flow entrypoints

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
mod fields; // field access/assignment lowering split
mod if_form;
mod weak_field_validator; // Phase 285A1: Weak field contract validator
                          // Phase 29bq+: sealing 層中立化
use control_flow::edgecfg::api::FragEmitSession;
mod declaration_indexer; // Phase 29bq+: Declaration indexing (user boxes, static methods)
mod declaration_order; // Deterministic box-member traversal owner
pub mod joinir_id_remapper; // Phase 189: JoinIR ID remapping (ValueId/BlockId translation) - Public for tests
mod joinir_inline_boundary_injector; // Phase 189: JoinInlineBoundary Copy instruction injector
mod loop_api_impl; // CLEAN-D: LoopBuilderApi wiring kept inside builder layer
pub(crate) mod loops;
mod module_lifecycle; // Phase 29bq+: Module lifecycle orchestrator (prepare → lower → finalize)
mod ops;
mod phi;
mod phi_merge;
mod phi_type_inference; // Phase 29bq+: PHI type inference (multi-phase fallback chain)
mod type_hint_providers; // Phase 29bq+: Type hint provision (call results, method signatures) // Phase 25.1q: Unified PHI merge helper // prepare/lower_root/finalize split
                         // legacy large-match remains inline for now (planned extraction)
mod emission; // emission::*（Const/Compare/Branch の薄い発行箱）
pub(crate) use emission::copy_emitter;
mod emit_guard; // EmitGuardBox（emit直前の最終関所）
mod metadata; // MetadataPropagationBox（type/originの伝播）
mod name_const; // NameConstBox（関数名Const生成）
mod observe; // P0: dev-only observability helpers（ssa/resolve）
mod origin; // P0: origin inference（me/Known）と PHI 伝播（軽量）
mod plugin_sigs; // plugin signature loader
mod receiver; // ReceiverMaterializationBox（Method recv の pin+LocalSSA 集約）
mod rewrite; // P1: Known rewrite & special consolidation
mod router; // RouterPolicyBox（Unified vs BoxCall）
mod schedule; // BlockScheduleBox（物理順序: PHI→materialize→body）
mod scope_context; // Phase 136 follow-up (Step 3/7): ScopeContext extraction
mod ssa; // LocalSSA helpers (in-block materialization)
mod stmts;
mod type_context; // Phase 136 follow-up: TypeContext extraction
mod type_facts; // Phase 136 follow-up: Type inference facts box
pub(crate) mod type_registry;
mod types; // types::annotation / inference（型注釈/推論の箱: 推論は後段）
mod utils;
mod vars; // variables/scope helpers // small loop helpers (header/exit context) // TypeRegistryBox（型情報管理の一元化）
          // Phase 288 Box化: repl_session moved to src/runner/repl/repl_session.rs

// Unified member property kinds for computed/once/birth_once
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum PropertyKind {
    Computed,
    Once,
    BirthOnce,
}

/// MIR builder for converting AST to SSA form
pub struct MirBuilder {
    /// Current module being built
    pub(super) current_module: Option<MirModule>,

    /// Current basic block being built
    pub(super) current_block: Option<BasicBlockId>,

    /// Phase 136 follow-up (Step 2/7): Core ID generation context
    /// Consolidates value_gen, block_gen, next_binding_id, temp_slot_counter, debug_join_counter.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) core_ctx: CoreContext,

    /// Phase 136 follow-up: Type information context
    /// Consolidates value_types, value_kinds, value_origin_newbox for better organization.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) type_ctx: type_context::TypeContext,

    /// Phase 136 follow-up (Step 3/7): Scope and control flow context
    /// Consolidates lexical_scope_stack, loop stacks, if_merge_stack, current_function,
    /// function_param_names, debug_scope_stack for better organization.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) scope_ctx: scope_context::ScopeContext,

    /// Phase 136 follow-up (Step 4/7): Binding context
    /// Consolidates binding_map (String -> BindingId mapping).
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) binding_ctx: BindingContext,

    /// Phase 136 follow-up (Step 5/7): Variable context
    /// Consolidates variable_map (String -> ValueId mapping for SSA conversion).
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) variable_ctx: variable_context::VariableContext,

    /// Phase 136 follow-up (Step 6/7): Metadata context
    /// Consolidates current_span, source_file, hint_sink, current_region_stack.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) metadata_ctx:
        metadata_context::MetadataContext<crate::ast::Span, crate::mir::region::RegionId>,

    /// Phase 136 follow-up (Step 7/7): Compilation context
    /// Consolidates compilation_context, current_static_box, user_defined_boxes, reserved_value_ids,
    /// fn_body_ast, weak_fields_by_box, property_getters_by_box, field_origin_class, field_origin_by_box,
    /// static_method_index, method_tail_index, type_registry, current_slot_registry, plugin_method_sigs.
    /// Direct field access for backward compatibility (migration in progress).
    pub(super) comp_ctx: compilation_context::CompilationContext,

    /// Pending phi functions to be inserted
    #[allow(dead_code)]
    pub(super) pending_phis: Vec<(BasicBlockId, ValueId, String)>,

    // Phase 2-5: binding_map removed - use binding_ctx methods instead

    // include guards removed
    // フェーズM: no_phi_modeフィールド削除（常にPHI使用）

    // ---- Try/Catch/Cleanup lowering context ----
    /// When true, `return` statements are deferred: they assign to `return_defer_slot`
    /// and jump to `return_defer_target` (typically the cleanup/exit block).
    pub(super) return_defer_active: bool,
    /// Slot value to receive deferred return values (edge-copy mode friendly).
    pub(super) return_defer_slot: Option<ValueId>,
    /// Target block to jump to on deferred return.
    pub(super) return_defer_target: Option<BasicBlockId>,
    /// Set to true when a deferred return has been emitted in the current context.
    pub(super) return_deferred_emitted: bool,
    /// True while lowering the cleanup block.
    pub(super) in_cleanup_block: bool,
    /// Policy flags (snapshotted at entry of try/catch lowering)
    pub(super) cleanup_allow_return: bool,
    pub(super) cleanup_allow_throw: bool,

    /// If true, skip entry materialization of pinned slots on the next start_new_block call.
    suppress_pin_entry_copy_next: bool,

    // ----------------------
    // Debug scope context (dev only; zero-cost when unused)
    // ----------------------
    /// Local SSA cache: ensure per-block materialization for critical operands (e.g., recv)
    /// Key: (bb, original ValueId, kind) -> local ValueId
    /// kind: 0=recv, 1+ reserved for future (args etc.)
    pub(super) local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>,
    /// BlockSchedule cache: deduplicate materialize copies per (bb, src)
    pub(super) schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>,
    /// Mapping from ValueId to its pin slot name (e.g., "__pin$3$@recv")
    /// Used by LocalSSA to redirect old pinned values to the latest slot value.
    pub(super) pin_slot_names: HashMap<ValueId, String>,

    /// Guard flag to prevent re-entering emit_unified_call from BoxCall fallback.
    /// Used when RouterPolicyBox in emit_unified_call has already decided to
    /// route a given Method call to BoxCall; emit_box_or_plugin_call must not
    /// bounce back into the unified path for the same call, otherwise an
    /// infinite recursion (emit_unified_call → emit_box_or_plugin_call →
    /// emit_unified_call …) can occur when routing decisions disagree.
    pub(super) in_unified_boxcall_fallback: bool,

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

    /// Phase 29bq+: Frag emit session (per-function sealing)
    /// 各関数開始時に reset() する（lifecycle.rs / calls/lowering.rs）
    pub(super) frag_emit_session: FragEmitSession,
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
        assert!(builder.binding_ctx.is_empty());
    }

    #[test]
    fn test_binding_allocation_sequential() {
        let mut builder = MirBuilder::new();
        let bid0 = builder.allocate_binding_id();
        let bid1 = builder.allocate_binding_id();
        let bid2 = builder.allocate_binding_id();

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
        let outer_bid = builder.binding_ctx.lookup("x").unwrap();
        assert_eq!(outer_bid.raw(), 0);

        // Enter inner scope and shadow x
        builder.push_lexical_scope();
        // Phase 136 P0: Use SSOT allocator for function scope simulation
        let inner_vid = builder.next_value_id();
        builder
            .declare_local_in_current_scope("x", inner_vid)
            .unwrap();
        // Phase 2-6: Check binding_ctx (SSOT)
        let inner_bid = builder.binding_ctx.lookup("x").unwrap();
        assert_eq!(inner_bid.raw(), 1);

        // Exit inner scope - should restore outer binding
        builder.pop_lexical_scope();
        // Phase 2-6: Check binding_ctx (SSOT)
        let restored_bid = builder.binding_ctx.lookup("x").unwrap();
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
        let bid0 = builder.allocate_binding_id();
        let vid1 = builder.next_value_id();
        let bid1 = builder.allocate_binding_id();

        // ValueId and BindingId should be independent
        assert_eq!(vid0.0, 0);
        assert_eq!(bid0.raw(), 0);
        assert_eq!(vid1.0, 1);
        assert_eq!(bid1.raw(), 1);

        // Allocating more ValueIds should not affect BindingId counter
        let _ = builder.next_value_id();
        let _ = builder.next_value_id();
        let bid2 = builder.allocate_binding_id();
        assert_eq!(bid2.raw(), 2); // Still sequential

        // Allocating more BindingIds should not affect ValueId counter
        let _ = builder.allocate_binding_id();
        let _ = builder.allocate_binding_id();
        let vid2 = builder.next_value_id();
        assert_eq!(vid2.0, 4); // Continues from where we left off
    }
}

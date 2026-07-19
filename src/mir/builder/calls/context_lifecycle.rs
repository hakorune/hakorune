//! 🎯 箱理論: Prepare/restore context, lowering context lifecycle management
//!
//! 責務:
//! - LoweringContext struct definition (context state management)
//! - prepare_lowering_context() (context setup before function lowering)
//! - restore_lowering_context() (context restoration after function lowering)
//!
//! Context管理:
//! - BoxCompilationContext vs Legacy mode の切り替え
//! - variable_map, type_ctx, static_box context の保存・復元
//! - FunctionSlotRegistry の関数境界管理

use crate::mir::builder::compilation_context::RecordLocalValue;
use crate::mir::builder::type_context::TypeContextSnapshot;
use crate::mir::builder::{FragEmitSession, MirBuilder};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::region::RegionId;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{BTreeMap, HashMap, HashSet}; // Phase 25.1: 決定性確保

#[derive(Debug)]
pub(super) struct ScopeStacksSnapshot {
    pub(super) lexical_scope_stack:
        Vec<crate::mir::builder::vars::lexical_scope::LexicalScopeFrame>,
    pub(super) loop_header_stack: Vec<BasicBlockId>,
    pub(super) loop_exit_stack: Vec<BasicBlockId>,
    pub(super) if_merge_stack: Vec<BasicBlockId>,
    pub(super) debug_scope_stack: Vec<String>,
    pub(super) function_param_names: HashSet<String>,
    pub(super) fastmem_region_stack: Vec<FastMemRegionId>,
}

/// 🎯 箱理論: Lowering Context（準備と復元）
pub(super) struct LoweringContext {
    pub(super) context_active: bool,
    pub(super) saved_var_map: Option<BTreeMap<String, crate::mir::ValueId>>, // Phase 25.1: BTreeMap化
    pub(super) saved_type_ctx: Option<TypeContextSnapshot>,
    pub(super) saved_static_ctx: Option<String>,
    pub(super) saved_function: Option<crate::mir::builder::MirFunction>,
    pub(super) saved_block: Option<crate::mir::builder::BasicBlockId>,
    pub(super) saved_slot_registry: Option<FunctionSlotRegistry>,
    pub(super) saved_reserved_value_ids: HashSet<ValueId>,
    pub(super) saved_fn_body_ast: Option<Vec<crate::ast::ASTNode>>,
    pub(super) saved_frag_emit_session: FragEmitSession,
    pub(super) saved_current_span: crate::ast::Span,
    pub(super) saved_region_stack: Vec<RegionId>,

    // Function lowering is re-entrant (nested method lowering while building another function).
    // Preserve the caller function's per-function state so lexical scopes and SSA caches stay balanced.
    pub(super) saved_binding_ctx: hakorune_mir_builder::BindingContext,
    pub(super) saved_resolved_binding_state:
        crate::mir::builder::vars::resolved_binding_state::ResolvedBindingLoweringStateV1,
    pub(super) saved_scope_stacks: ScopeStacksSnapshot,
    pub(super) saved_pending_phis: Vec<(BasicBlockId, ValueId, String)>,
    pub(super) saved_local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>,
    pub(super) saved_schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>,
    pub(super) saved_pin_slot_names: HashMap<ValueId, String>,
    pub(super) saved_record_local_values: HashMap<ValueId, RecordLocalValue>,
    pub(super) saved_return_defer_active: bool,
    pub(super) saved_return_defer_slot: Option<ValueId>,
    pub(super) saved_return_defer_target: Option<BasicBlockId>,
    pub(super) saved_return_deferred_emitted: bool,
    pub(super) saved_in_cleanup_block: bool,
    pub(super) saved_cleanup_allow_return: bool,
    pub(super) saved_cleanup_allow_throw: bool,
    pub(super) saved_suppress_pin_entry_copy_next: bool,
    pub(super) saved_in_unified_boxcall_fallback: bool,
    pub(super) saved_recursion_depth: usize,
}

impl MirBuilder {
    /// 🎯 箱理論: Step 1 - Lowering Context準備
    pub(super) fn prepare_lowering_context(&mut self, func_name: &str) -> LoweringContext {
        // Snapshot the caller function first. No later fallible step owns this
        // state, so the session can restore even if skeleton creation fails.
        let saved_function = self.function_state.current_function.take();
        let saved_block = self.function_state.current_block.take();

        // Static box context設定
        let saved_static_ctx = self.comp_ctx.current_static_box.clone();
        if let Some(pos) = func_name.find('.') {
            let box_name = &func_name[..pos];
            if !box_name.is_empty() {
                self.comp_ctx.current_static_box = Some(box_name.to_string());
            }
        }

        // BoxCompilationContext vs saved_var_map モード判定
        let context_active = self.comp_ctx.compilation_context.is_some();
        let saved_var_map = if !context_active {
            Some(std::mem::take(
                &mut self.function_state.variable_ctx.variable_map,
            ))
        } else {
            None
        };
        // ValueId は関数ローカルなので、snapshot path では type_ctx も関数境界で必ず分離する。
        // そうしないと別関数の ValueId と衝突し、box_name 推論がランダムに壊れる（phase29aq flake 根因）。
        let saved_type_ctx = if !context_active {
            Some(self.function_state.type_ctx.take_snapshot())
        } else {
            None
        };

        // 関数スコープ SlotRegistry は元の関数側から退避しておくよ。
        let saved_slot_registry = self.comp_ctx.current_slot_registry.take();
        let saved_reserved_value_ids =
            std::mem::take(&mut self.function_state.compilation.reserved_value_ids);
        let saved_fn_body_ast = self.function_state.compilation.fn_body_ast.take();
        let saved_frag_emit_session = std::mem::take(&mut self.function_state.frag_emit_session);
        let saved_current_span = self.metadata_ctx.current_span();
        let saved_region_stack = self.metadata_ctx.current_region_stack().to_vec();

        // Nested function lowering must not destroy the caller's lexical scopes / SSA caches.
        let saved_binding_ctx = std::mem::take(&mut self.function_state.binding_ctx);
        let saved_resolved_binding_state =
            std::mem::take(&mut self.function_state.resolved_binding_state);
        let saved_scope_stacks = ScopeStacksSnapshot {
            lexical_scope_stack: std::mem::take(&mut self.function_state.scope.lexical_scope_stack),
            loop_header_stack: std::mem::take(&mut self.function_state.scope.loop_header_stack),
            loop_exit_stack: std::mem::take(&mut self.function_state.scope.loop_exit_stack),
            if_merge_stack: std::mem::take(&mut self.function_state.scope.if_merge_stack),
            debug_scope_stack: std::mem::take(&mut self.scope_ctx.debug_scope_stack),
            function_param_names: std::mem::take(
                &mut self.function_state.scope.function_param_names,
            ),
            fastmem_region_stack: std::mem::take(
                &mut self.function_state.scope.fastmem_region_stack,
            ),
        };
        let saved_pending_phis = std::mem::take(&mut self.function_state.pending_phis);
        let saved_local_ssa_map = std::mem::take(&mut self.function_state.local_ssa_map);
        let saved_schedule_mat_map = std::mem::take(&mut self.function_state.schedule_mat_map);
        let saved_pin_slot_names = std::mem::take(&mut self.function_state.pin_slot_names);
        let saved_record_local_values =
            std::mem::take(&mut self.function_state.compilation.record_local_values);
        let saved_return_defer_active = self.function_state.return_defer_active;
        let saved_return_defer_slot = self.function_state.return_defer_slot;
        let saved_return_defer_target = self.function_state.return_defer_target;
        let saved_return_deferred_emitted = self.function_state.return_deferred_emitted;
        let saved_in_cleanup_block = self.function_state.in_cleanup_block;
        let saved_cleanup_allow_return = self.function_state.cleanup_allow_return;
        let saved_cleanup_allow_throw = self.function_state.cleanup_allow_throw;
        let saved_suppress_pin_entry_copy_next = self.function_state.suppress_pin_entry_copy_next;
        let saved_in_unified_boxcall_fallback = self.function_state.in_unified_boxcall_fallback;
        let saved_recursion_depth = self.recursion_depth;

        // Function boundary: clear per-function state to avoid ValueId leaks across functions.
        self.function_state.binding_ctx.clear_for_function_entry();
        self.function_state.scope.clear_for_function_entry();
        self.scope_ctx.clear_debug_scope_for_function_entry();
        self.function_state.variable_ctx =
            crate::mir::builder::variable_context::VariableContext::new();
        self.function_state.pending_phis.clear();
        self.function_state.local_ssa_map.clear();
        self.function_state.schedule_mat_map.clear();
        self.function_state.pin_slot_names.clear();
        self.function_state.return_defer_active = false;
        self.function_state.return_defer_slot = None;
        self.function_state.return_defer_target = None;
        self.function_state.return_deferred_emitted = false;
        self.function_state.in_cleanup_block = false;
        self.function_state.cleanup_allow_return = false;
        self.function_state.cleanup_allow_throw = false;
        self.function_state.suppress_pin_entry_copy_next = false;
        self.function_state.in_unified_boxcall_fallback = false;
        self.recursion_depth = 0;

        // BoxCompilationContext mode: clear()で完全独立化
        if context_active {
            self.function_state.variable_ctx.variable_map.clear();
            self.function_state.type_ctx.value_origin_newbox.clear();
            self.function_state.compilation.clear_record_local_values();
            // value_types も static box 単位で独立させる。
            // これにより、前の static box で使用された ValueId に紐づく型情報が
            // 次の box にリークして誤った box_name 推論（例: Stage1UsingResolverBox）
            // を引き起こすことを防ぐ。
            self.function_state.type_ctx.value_types.clear();
            self.function_state.type_ctx.value_kinds.clear();
        }

        LoweringContext {
            context_active,
            saved_var_map,
            saved_type_ctx,
            saved_static_ctx,
            saved_function,
            saved_block,
            saved_slot_registry,
            saved_reserved_value_ids,
            saved_fn_body_ast,
            saved_frag_emit_session,
            saved_current_span,
            saved_region_stack,
            saved_binding_ctx,
            saved_resolved_binding_state,
            saved_scope_stacks,
            saved_pending_phis,
            saved_local_ssa_map,
            saved_schedule_mat_map,
            saved_pin_slot_names,
            saved_record_local_values,
            saved_return_defer_active,
            saved_return_defer_slot,
            saved_return_defer_target,
            saved_return_deferred_emitted,
            saved_in_cleanup_block,
            saved_cleanup_allow_return,
            saved_cleanup_allow_throw,
            saved_suppress_pin_entry_copy_next,
            saved_in_unified_boxcall_fallback,
            saved_recursion_depth,
        }
    }

    /// 🎯 箱理論: Step 6 - Context復元
    pub(super) fn restore_lowering_context(&mut self, ctx: LoweringContext) {
        // Phase 136 Step 3/7: Restore to scope_ctx (SSOT)
        self.function_state.current_function = ctx.saved_function;
        self.function_state.current_block = ctx.saved_block;

        // モード別にcontext復元
        if ctx.context_active {
            // BoxCompilationContext mode: clear のみ（次回も完全独立）
            self.function_state.variable_ctx.variable_map.clear();
            self.function_state.type_ctx.value_origin_newbox.clear();
            self.function_state.compilation.clear_record_local_values();
            // static box ごとに型情報も独立させる（前 box の型メタデータを引きずらない）
            self.function_state.type_ctx.value_types.clear();
            self.function_state.type_ctx.value_kinds.clear();
        } else if let Some(saved) = ctx.saved_var_map {
            // Legacy mode: Main.main 側の variable_map を元に戻す
            self.function_state.variable_ctx.variable_map = saved;
            if let Some(saved_type_ctx) = ctx.saved_type_ctx {
                self.function_state
                    .type_ctx
                    .restore_snapshot(saved_type_ctx);
            }
        }

        // Static box context復元
        self.comp_ctx.current_static_box = ctx.saved_static_ctx;
        // 関数スコープ SlotRegistry も元の関数に戻すよ。
        self.comp_ctx.current_slot_registry = ctx.saved_slot_registry;
        self.function_state.compilation.reserved_value_ids = ctx.saved_reserved_value_ids;
        self.function_state.compilation.fn_body_ast = ctx.saved_fn_body_ast;
        self.function_state.frag_emit_session = ctx.saved_frag_emit_session;
        self.metadata_ctx.set_current_span(ctx.saved_current_span);
        while self.metadata_ctx.pop_region().is_some() {}
        for region in ctx.saved_region_stack {
            self.metadata_ctx.push_region(region);
        }

        // Restore caller function state (lexical scopes / SSA caches / try-cleanup flags).
        self.function_state.binding_ctx = ctx.saved_binding_ctx;
        self.function_state.resolved_binding_state = ctx.saved_resolved_binding_state;
        self.function_state.scope.lexical_scope_stack = ctx.saved_scope_stacks.lexical_scope_stack;
        self.function_state.scope.loop_header_stack = ctx.saved_scope_stacks.loop_header_stack;
        self.function_state.scope.loop_exit_stack = ctx.saved_scope_stacks.loop_exit_stack;
        self.function_state.scope.if_merge_stack = ctx.saved_scope_stacks.if_merge_stack;
        self.scope_ctx.debug_scope_stack = ctx.saved_scope_stacks.debug_scope_stack;
        self.function_state.scope.function_param_names =
            ctx.saved_scope_stacks.function_param_names;
        self.function_state.scope.fastmem_region_stack =
            ctx.saved_scope_stacks.fastmem_region_stack;
        self.function_state.pending_phis = ctx.saved_pending_phis;
        self.function_state.local_ssa_map = ctx.saved_local_ssa_map;
        self.function_state.schedule_mat_map = ctx.saved_schedule_mat_map;
        self.function_state.pin_slot_names = ctx.saved_pin_slot_names;
        self.function_state.compilation.record_local_values = ctx.saved_record_local_values;
        self.function_state.return_defer_active = ctx.saved_return_defer_active;
        self.function_state.return_defer_slot = ctx.saved_return_defer_slot;
        self.function_state.return_defer_target = ctx.saved_return_defer_target;
        self.function_state.return_deferred_emitted = ctx.saved_return_deferred_emitted;
        self.function_state.in_cleanup_block = ctx.saved_in_cleanup_block;
        self.function_state.cleanup_allow_return = ctx.saved_cleanup_allow_return;
        self.function_state.cleanup_allow_throw = ctx.saved_cleanup_allow_throw;
        self.function_state.suppress_pin_entry_copy_next = ctx.saved_suppress_pin_entry_copy_next;
        self.function_state.in_unified_boxcall_fallback = ctx.saved_in_unified_boxcall_fallback;
        self.recursion_depth = ctx.saved_recursion_depth;
    }
}

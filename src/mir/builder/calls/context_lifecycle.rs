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

use crate::mir::builder::type_context::TypeContextSnapshot;
use crate::mir::builder::MirBuilder;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::{BTreeMap, HashMap, HashSet}; // Phase 25.1: 決定性確保

#[derive(Debug)]
pub(super) struct ScopeStacksSnapshot {
    pub(super) lexical_scope_stack: Vec<crate::mir::builder::scope_context::LexicalScopeFrame>,
    pub(super) loop_header_stack: Vec<BasicBlockId>,
    pub(super) loop_exit_stack: Vec<BasicBlockId>,
    pub(super) if_merge_stack: Vec<BasicBlockId>,
    pub(super) debug_scope_stack: Vec<String>,
    pub(super) function_param_names: HashSet<String>,
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

    // Function lowering is re-entrant (nested method lowering while building another function).
    // Preserve the caller function's per-function state so lexical scopes and SSA caches stay balanced.
    pub(super) saved_binding_ctx: hakorune_mir_builder::BindingContext,
    pub(super) saved_scope_stacks: ScopeStacksSnapshot,
    pub(super) saved_pending_phis: Vec<(BasicBlockId, ValueId, String)>,
    pub(super) saved_local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>,
    pub(super) saved_schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>,
    pub(super) saved_pin_slot_names: HashMap<ValueId, String>,
    pub(super) saved_return_defer_active: bool,
    pub(super) saved_return_defer_slot: Option<ValueId>,
    pub(super) saved_return_defer_target: Option<BasicBlockId>,
    pub(super) saved_return_deferred_emitted: bool,
    pub(super) saved_in_cleanup_block: bool,
    pub(super) saved_cleanup_allow_return: bool,
    pub(super) saved_cleanup_allow_throw: bool,
    pub(super) saved_suppress_pin_entry_copy_next: bool,
}

impl MirBuilder {
    /// 🎯 箱理論: Step 1 - Lowering Context準備
    pub(super) fn prepare_lowering_context(&mut self, func_name: &str) -> LoweringContext {
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
            Some(std::mem::take(&mut self.variable_ctx.variable_map))
        } else {
            None
        };
        // ValueId は関数ローカルなので、legacy mode では type_ctx も関数境界で必ず分離する。
        // そうしないと別関数の ValueId と衝突し、box_name 推論がランダムに壊れる（phase29aq flake 根因）。
        let saved_type_ctx = if !context_active {
            Some(self.type_ctx.take_snapshot())
        } else {
            None
        };

        // 関数スコープ SlotRegistry は元の関数側から退避しておくよ。
        let saved_slot_registry = self.comp_ctx.current_slot_registry.take();

        // Phase 201-A: Clear reserved ValueIds at function entry (function-local).
        self.comp_ctx.clear_reserved_value_ids();

        // Nested function lowering must not destroy the caller's lexical scopes / SSA caches.
        let saved_binding_ctx = std::mem::take(&mut self.binding_ctx);
        let saved_scope_stacks = ScopeStacksSnapshot {
            lexical_scope_stack: std::mem::take(&mut self.scope_ctx.lexical_scope_stack),
            loop_header_stack: std::mem::take(&mut self.scope_ctx.loop_header_stack),
            loop_exit_stack: std::mem::take(&mut self.scope_ctx.loop_exit_stack),
            if_merge_stack: std::mem::take(&mut self.scope_ctx.if_merge_stack),
            debug_scope_stack: std::mem::take(&mut self.scope_ctx.debug_scope_stack),
            function_param_names: std::mem::take(&mut self.scope_ctx.function_param_names),
        };
        let saved_pending_phis = std::mem::take(&mut self.pending_phis);
        let saved_local_ssa_map = std::mem::take(&mut self.local_ssa_map);
        let saved_schedule_mat_map = std::mem::take(&mut self.schedule_mat_map);
        let saved_pin_slot_names = std::mem::take(&mut self.pin_slot_names);
        let saved_return_defer_active = self.return_defer_active;
        let saved_return_defer_slot = self.return_defer_slot;
        let saved_return_defer_target = self.return_defer_target;
        let saved_return_deferred_emitted = self.return_deferred_emitted;
        let saved_in_cleanup_block = self.in_cleanup_block;
        let saved_cleanup_allow_return = self.cleanup_allow_return;
        let saved_cleanup_allow_throw = self.cleanup_allow_throw;
        let saved_suppress_pin_entry_copy_next = self.suppress_pin_entry_copy_next;

        // Function boundary: clear per-function state to avoid ValueId leaks across functions.
        self.binding_ctx.clear_for_function_entry();
        self.scope_ctx.clear_for_function_entry();
        self.variable_ctx = crate::mir::builder::variable_context::VariableContext::new();
        self.pending_phis.clear();
        self.local_ssa_map.clear();
        self.schedule_mat_map.clear();
        self.pin_slot_names.clear();
        self.return_defer_active = false;
        self.return_defer_slot = None;
        self.return_defer_target = None;
        self.return_deferred_emitted = false;
        self.in_cleanup_block = false;
        self.cleanup_allow_return = false;
        self.cleanup_allow_throw = false;
        self.suppress_pin_entry_copy_next = false;

        // BoxCompilationContext mode: clear()で完全独立化
        if context_active {
            self.variable_ctx.variable_map.clear();
            self.type_ctx.value_origin_newbox.clear();
            // value_types も static box 単位で独立させる。
            // これにより、前の static box で使用された ValueId に紐づく型情報が
            // 次の box にリークして誤った box_name 推論（例: Stage1UsingResolverBox）
            // を引き起こすことを防ぐ。
            self.type_ctx.value_types.clear();
            self.type_ctx.value_kinds.clear();
        }

        LoweringContext {
            context_active,
            saved_var_map,
            saved_type_ctx,
            saved_static_ctx,
            saved_function: None,
            saved_block: None,
            saved_slot_registry,
            saved_binding_ctx,
            saved_scope_stacks,
            saved_pending_phis,
            saved_local_ssa_map,
            saved_schedule_mat_map,
            saved_pin_slot_names,
            saved_return_defer_active,
            saved_return_defer_slot,
            saved_return_defer_target,
            saved_return_deferred_emitted,
            saved_in_cleanup_block,
            saved_cleanup_allow_return,
            saved_cleanup_allow_throw,
            saved_suppress_pin_entry_copy_next,
        }
    }

    /// 🎯 箱理論: Step 6 - Context復元
    pub(super) fn restore_lowering_context(&mut self, ctx: LoweringContext) {
        // Phase 136 Step 3/7: Restore to scope_ctx (SSOT)
        self.scope_ctx.current_function = ctx.saved_function;
        self.current_block = ctx.saved_block;

        // モード別にcontext復元
        if ctx.context_active {
            // BoxCompilationContext mode: clear のみ（次回も完全独立）
            self.variable_ctx.variable_map.clear();
            self.type_ctx.value_origin_newbox.clear();
            // static box ごとに型情報も独立させる（前 box の型メタデータを引きずらない）
            self.type_ctx.value_types.clear();
            self.type_ctx.value_kinds.clear();
        } else if let Some(saved) = ctx.saved_var_map {
            // Legacy mode: Main.main 側の variable_map を元に戻す
            self.variable_ctx.variable_map = saved;
            if let Some(saved_type_ctx) = ctx.saved_type_ctx {
                self.type_ctx.restore_snapshot(saved_type_ctx);
            }
        }

        // Static box context復元
        self.comp_ctx.current_static_box = ctx.saved_static_ctx;
        // 関数スコープ SlotRegistry も元の関数に戻すよ。
        self.comp_ctx.current_slot_registry = ctx.saved_slot_registry;

        // Restore caller function state (lexical scopes / SSA caches / try-cleanup flags).
        self.binding_ctx = ctx.saved_binding_ctx;
        self.scope_ctx.lexical_scope_stack = ctx.saved_scope_stacks.lexical_scope_stack;
        self.scope_ctx.loop_header_stack = ctx.saved_scope_stacks.loop_header_stack;
        self.scope_ctx.loop_exit_stack = ctx.saved_scope_stacks.loop_exit_stack;
        self.scope_ctx.if_merge_stack = ctx.saved_scope_stacks.if_merge_stack;
        self.scope_ctx.debug_scope_stack = ctx.saved_scope_stacks.debug_scope_stack;
        self.scope_ctx.function_param_names = ctx.saved_scope_stacks.function_param_names;
        self.pending_phis = ctx.saved_pending_phis;
        self.local_ssa_map = ctx.saved_local_ssa_map;
        self.schedule_mat_map = ctx.saved_schedule_mat_map;
        self.pin_slot_names = ctx.saved_pin_slot_names;
        self.return_defer_active = ctx.saved_return_defer_active;
        self.return_defer_slot = ctx.saved_return_defer_slot;
        self.return_defer_target = ctx.saved_return_defer_target;
        self.return_deferred_emitted = ctx.saved_return_deferred_emitted;
        self.in_cleanup_block = ctx.saved_in_cleanup_block;
        self.cleanup_allow_return = ctx.saved_cleanup_allow_return;
        self.cleanup_allow_throw = ctx.saved_cleanup_allow_throw;
        self.suppress_pin_entry_copy_next = ctx.saved_suppress_pin_entry_copy_next;
    }
}

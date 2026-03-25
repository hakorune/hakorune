use super::plugin_sigs;
use super::{
    compilation_context, metadata_context, scope_context, type_context, variable_context,
    MirBuilder,
};
use crate::mir::BindingId;
use hakorune_mir_builder::{BindingContext, CoreContext};
use std::collections::HashMap;

impl MirBuilder {
    /// Create a new MIR builder
    pub fn new() -> Self {
        let plugin_method_sigs = plugin_sigs::load_plugin_method_sigs();
        let core_ctx = CoreContext::new();

        // Phase 136 Step 7/7: Compilation context (new SSOT)
        let comp_ctx =
            compilation_context::CompilationContext::with_plugin_sigs(plugin_method_sigs.clone());

        // フェーズM: no_phi_mode初期化削除
        #[allow(deprecated)]
        Self {
            current_module: None,
            current_block: None,

            // Phase 136 Step 2/7: Core context (new SSOT)
            core_ctx,

            type_ctx: type_context::TypeContext::new(), // Phase 136: Type context
            scope_ctx: scope_context::ScopeContext::new(), // Phase 136 Step 3/7: Scope context
            binding_ctx: BindingContext::new(),         // Phase 136 Step 4/7: Binding context
            variable_ctx: variable_context::VariableContext::new(), // Phase 136 Step 5/7: Variable context
            metadata_ctx: metadata_context::MetadataContext::new(crate::ast::Span::unknown()), // Phase 136 Step 6/7: Metadata context
            comp_ctx, // Phase 136 Step 7/7: Compilation context
            pending_phis: Vec::new(),

            // Phase 2-5: binding_map initialization removed

            // フェーズM: no_phi_modeフィールド削除
            return_defer_active: false,
            return_defer_slot: None,
            return_defer_target: None,
            return_deferred_emitted: false,
            in_cleanup_block: false,
            cleanup_allow_return: false,
            cleanup_allow_throw: false,
            suppress_pin_entry_copy_next: false,

            local_ssa_map: HashMap::new(),
            schedule_mat_map: HashMap::new(),
            pin_slot_names: HashMap::new(),

            in_unified_boxcall_fallback: false,
            recursion_depth: 0,
            root_is_app_mode: None,
            repl_mode: false, // Phase 288 P2: REPL mode (default: file mode)
            frag_emit_session: super::FragEmitSession::new(), // Phase 29bq+: sealing 層中立化
        }
    }

    // Phase 2-5: BindingContext sync helpers removed - binding_ctx is now SSOT
    // Phase 2-6: VariableContext sync helpers removed - variable_ctx is now SSOT

    /// Push/pop helpers for If merge context (best-effort; optional usage)
    pub(super) fn push_if_merge(&mut self, bb: super::BasicBlockId) {
        // Phase 2-4: Use scope_ctx only (legacy field removed)
        self.scope_ctx.push_if_merge(bb);
    }
    pub(super) fn pop_if_merge(&mut self) {
        // Phase 2-4: Use scope_ctx only (legacy field removed)
        let _ = self.scope_ctx.pop_if_merge();
    }

    /// Suppress entry pin copy for the next start_new_block (used for merge blocks).
    pub(super) fn suppress_next_entry_pin_copy(&mut self) {
        self.suppress_pin_entry_copy_next = true;
    }

    // ---- Phase 74: BindingId allocation ----
    /// Allocate a new BindingId (parallel to ValueId allocation)
    ///
    /// ## Parallel ValueId/BindingId Allocation
    ///
    /// BindingId allocation is completely independent from ValueId allocation:
    /// - `next_value_id()` increments `value_gen` counter
    /// - `allocate_binding_id()` increments `next_binding_id` counter
    ///
    /// This parallelism enables:
    /// 1. **Stable binding identity** across SSA transformations
    /// 2. **Independent shadowing tracking** separate from SSA renaming
    /// 3. **Future ScopeManager migration** (Phase 75+) without breaking SSA
    ///
    /// Example:
    /// ```ignore
    /// // local x = 1;      <- allocate_binding_id() -> BindingId(0)
    /// //                      next_value_id() -> ValueId(10)
    /// // {
    /// //   local x = 2;    <- allocate_binding_id() -> BindingId(1)
    /// //                      next_value_id() -> ValueId(20)
    /// // }
    /// ```
    pub fn allocate_binding_id(&mut self) -> BindingId {
        // Phase 136 Step 2/7 + Phase 2-2: Use core_ctx as SSOT (no sync needed)
        self.core_ctx.next_binding()
    }
}

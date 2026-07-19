use super::plugin_sigs;
use super::{
    compilation_context, function_lowering_state, metadata_context, scope_context, MirBuilder,
};
use crate::mir::BindingId;
use hakorune_mir_builder::CoreContext;

impl MirBuilder {
    pub(in crate::mir) fn current_function_name(&self) -> Option<&str> {
        self.function_state
            .current_function
            .as_ref()
            .map(|function| function.signature.name.as_str())
    }

    pub(in crate::mir) fn current_function_entry_block(&self) -> Option<super::BasicBlockId> {
        self.function_state
            .current_function
            .as_ref()
            .map(|function| function.entry_block)
    }

    pub(in crate::mir) fn current_function_instructions(&self) -> Vec<super::MirInstruction> {
        self.function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.instructions.iter().cloned())
            .collect()
    }

    pub(in crate::mir) fn current_function_instruction_blocks(
        &self,
    ) -> Vec<(super::BasicBlockId, Vec<super::MirInstruction>)> {
        self.function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.iter())
            .map(|(block, data)| (*block, data.instructions.clone()))
            .collect()
    }

    pub(in crate::mir) fn current_variable(&self, name: &str) -> Option<super::ValueId> {
        self.function_state.variable_ctx.lookup(name)
    }

    pub(in crate::mir) fn function_parameter_bindings(&self) -> Vec<(String, super::ValueId)> {
        self.function_state
            .scope
            .function_param_names
            .iter()
            .filter_map(|name| {
                self.current_variable(name)
                    .map(|value| (name.clone(), value))
            })
            .collect()
    }

    pub(in crate::mir) fn variable_bindings(&self) -> Vec<(String, super::ValueId)> {
        self.function_state
            .variable_ctx
            .variable_map()
            .iter()
            .map(|(name, value)| (name.clone(), *value))
            .collect()
    }

    pub(in crate::mir) fn value_type(&self, value: super::ValueId) -> Option<&super::MirType> {
        self.function_state.type_ctx.value_types.get(&value)
    }

    pub(in crate::mir) fn current_block_id(&self) -> Option<super::BasicBlockId> {
        self.function_state.current_block
    }

    pub(in crate::mir) fn checked_current_block_terminated(&self) -> Result<bool, String> {
        let block = self
            .current_block_id()
            .ok_or_else(|| "No current block".to_string())?;
        Ok(self
            .function_state
            .current_function
            .as_ref()
            .and_then(|function| function.get_block(block))
            .is_some_and(|block| block.is_terminated()))
    }

    pub(in crate::mir) fn capture_current_predecessor_and_jump(
        &mut self,
        target: super::BasicBlockId,
    ) -> Result<Option<super::BasicBlockId>, String> {
        let current = self
            .current_block_id()
            .ok_or_else(|| "No current block".to_string())?;
        if self.checked_current_block_terminated()? {
            return Ok(None);
        }
        let function = self
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                crate::mir::diagnostics::FreezeContract::new(
                    "builder/capture_jump_without_function",
                )
                .field("target_bb", format!("{:?}", target))
                .build()
            })?;
        crate::mir::ssot::cf_common::set_jump(function, current, target);
        Ok(Some(current))
    }

    pub(in crate::mir) fn next_function_value_id_or_core(&mut self) -> super::ValueId {
        self.function_state.current_function.as_mut().map_or_else(
            || self.core_ctx.next_value(),
            |function| function.next_value_id(),
        )
    }

    pub(in crate::mir) fn propagate_record_local_value_from_phi(
        &mut self,
        inputs: &[(super::BasicBlockId, super::ValueId)],
        dst: super::ValueId,
    ) {
        self.function_state
            .compilation
            .propagate_record_local_value_from_phi(inputs, dst);
    }

    pub(in crate::mir) fn record_value_origin_span(
        &mut self,
        value: super::ValueId,
        span: crate::ast::Span,
    ) {
        self.function_state.value_origins.record_span(value, span);
    }

    pub(in crate::mir) fn value_origin_span(
        &self,
        value: super::ValueId,
    ) -> Option<crate::ast::Span> {
        self.function_state.value_origins.span(value)
    }

    pub(in crate::mir) fn record_value_origin_caller(
        &mut self,
        value: super::ValueId,
        caller: &'static std::panic::Location<'static>,
    ) {
        self.function_state
            .value_origins
            .record_caller(value, caller);
    }

    pub(in crate::mir) fn value_origin_caller(&self, value: super::ValueId) -> Option<&str> {
        self.function_state.value_origins.caller(value)
    }

    pub(in crate::mir) fn value_origin_caller_rows(&self) -> Vec<(super::ValueId, String)> {
        self.function_state.value_origins.caller_rows()
    }
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
            function_state: function_lowering_state::FunctionLoweringStateV1::default(),

            // Phase 136 Step 2/7: Core context (new SSOT)
            core_ctx,

            scope_ctx: scope_context::ScopeContext::new(), // Phase 136 Step 3/7: Scope context
            metadata_ctx: metadata_context::MetadataContext::new(crate::ast::Span::unknown()), // Phase 136 Step 6/7: Metadata context
            comp_ctx, // Phase 136 Step 7/7: Compilation context

            recursion_depth: 0,
            root_is_app_mode: None,
            repl_mode: false, // Phase 288 P2: REPL mode (default: file mode)
        }
    }

    // Phase 2-5: BindingContext sync helpers removed - binding_ctx is now SSOT
    // Phase 2-6: VariableContext sync helpers removed - variable_ctx is now SSOT

    /// Push/pop helpers for If merge context (best-effort; optional usage)
    pub(super) fn push_if_merge(&mut self, bb: super::BasicBlockId) {
        self.function_state.scope.if_merge_stack.push(bb);
    }
    pub(super) fn pop_if_merge(&mut self) {
        let _ = self.function_state.scope.if_merge_stack.pop();
    }

    /// Suppress entry pin copy for the next start_new_block (used for merge blocks).
    pub(super) fn suppress_next_entry_pin_copy(&mut self) {
        self.function_state.suppress_pin_entry_copy_next = true;
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
    pub fn allocate_binding_id(&mut self) -> Result<BindingId, String> {
        self.function_state
            .resolved_binding_state
            .veto_legacy_allocation()?;
        // Phase 136 Step 2/7 + Phase 2-2: Use core_ctx as SSOT (no sync needed)
        Ok(self.core_ctx.next_binding())
    }
}

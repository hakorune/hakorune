//! Basic block management utilities
//!
//! Phase 25.1b: Pin slot entry handling (disabled - now managed by PHI system)

use super::super::BasicBlockId;

impl super::super::MirBuilder {
    /// Ensure a basic block exists in the current function
    pub(crate) fn ensure_block_exists(&mut self, block_id: BasicBlockId) -> Result<(), String> {
        if let Some(ref mut function) = self.function_state.current_function {
            if !function.blocks.contains_key(&block_id) {
                self.function_state
                    .canonical_control
                    .borrow_mut()
                    .cfg
                    .create_block(function, block_id)
                    .map_err(|error| error.to_string())?;
            }
            Ok(())
        } else {
            Err("No current function".to_string())
        }
    }

    /// Append an already selected physical packet without changing the current block.
    /// Block creation and forward cleanup edges belong to the function CFG.
    pub(in crate::mir::builder) fn append_packet_block_v1(
        &mut self,
        block: BasicBlockId,
        instructions: &[crate::mir::MirInstruction],
    ) -> Result<(), String> {
        self.ensure_block_exists(block)?;
        let function = self
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "No current function".to_owned())?;
        let control = self.function_state.canonical_control.borrow();
        for instruction in instructions {
            control
                .cfg
                .append_packet_instruction(
                    function,
                    block,
                    instruction.clone(),
                    crate::ast::Span::unknown(),
                )
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    /// Start a new basic block and set as current
    pub(crate) fn start_new_block(&mut self, block_id: BasicBlockId) -> Result<(), String> {
        self.ensure_block_exists(block_id)?;
        if self.function_state.current_function.is_some() {
            self.function_state.current_block = Some(block_id);
            // Local SSA cache is per-block; clear on block switch
            self.function_state.local_ssa_map.clear();
            // BlockSchedule materialize cache is per-block as well
            self.function_state.schedule_mat_map.clear();
            // Entry materialization for pinned slots: re-read from variable_map after PHIs are emitted.
            // This ensures pinned slots reflect the correct PHI values in merge blocks.
            //
            // Strategy: Instead of emitting Copy instructions (which would be before PHIs),
            // we simply update the variable_map to point to the current block's values.
            // LoopBuilder and IfBuilder already update variable_map with PHI values, so
            // pinned slots will automatically pick up the correct values.
            //
            // No action needed here - just clear caches.
            if !self.function_state.suppress_pin_entry_copy_next {
                // Cache clearing is already done above, so nothing more to do here.
                // The key insight: pinned slot variables are part of variable_map,
                // and LoopBuilder/IfBuilder already manage PHIs for ALL variables in variable_map,
                // including pinned slots.
            }
            // Phase 25.1b: Disabled pin entry copy handling (~32 lines legacy code removed)
            // Old approach (if false block) saved to git history before deletion.
            // Reason: LoopBuilder/IfBuilder already handle PHI merges for ALL variables,
            // including pinned slots, so explicit copy is redundant and causes SSA issues.

            // Reset suppression flag after use (one-shot)
            self.function_state.suppress_pin_entry_copy_next = false;
            Ok(())
        } else {
            Err("No current function".to_string())
        }
    }
}

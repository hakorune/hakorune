//! Value pinning (slotification) utilities
//!
//! Phase 25.1b: Function-local allocator usage to avoid SSA verification failures.
//!
//! ## Slot naming convention
//! Pinned slots use the format: `__pin$<id>$<hint>`
//! - `<id>`: Unique slot ID from core_ctx.next_temp_slot()
//! - `<hint>`: Human-readable description (e.g., "recv", "arg", "cond")
//!
//! ## Correctness-first strategy
//! Always pin to ensure:
//! - Block-local definition (avoids undefined use across blocks)
//! - PHI participation (registered in variable_map for merge points)
//! - Metadata propagation (type/origin tracking)


impl super::super::MirBuilder {
    /// Pin a block-crossing ephemeral value into a pseudo local slot and register it in variable_map
    /// so it participates in PHI merges across branches/blocks. Safe default for correctness-first.
    pub(crate) fn pin_to_slot(
        &mut self,
        v: super::super::ValueId,
        hint: &str,
    ) -> Result<super::super::ValueId, String> {
        let slot_id = self.core_ctx.next_temp_slot();
        let slot_name = format!("__pin${}${}", slot_id, hint);
        // Phase 25.1b: Use function-local ID allocator to avoid SSA verification failures
        let dst = if let Some(ref mut f) = self.function_state.current_function {
            f.next_value_id() // Function context: use local ID
        } else {
            self.core_ctx.next_value() // Module context: use core_ctx SSOT
        };
        self.emit_instruction(super::super::MirInstruction::Copy { dst, src: v })?;
        let function = self.function_state.current_function.as_ref()
            .ok_or("[freeze:contract][local-ssa-correspondence/pin-function]")?;
        let block = self.function_state.current_block
            .ok_or("[freeze:contract][local-ssa-correspondence/pin-block]")?;
        self.function_state.local_ssa_correspondence.record_pin_copy(function, v, dst, block)?;
        if super::builder_debug_enabled() || crate::config::env::builder_pin_trace() {
            super::builder_debug_log(&format!("pin slot={} src={} dst={}", slot_name, v.0, dst.0));
        }
        // Propagate lightweight metadata so downstream resolution/type inference remains stable
        crate::mir::builder::metadata::propagate::propagate(self, v, dst);
        // Remember pin slot name for both the original and the pinned value.
        // LocalSSA uses this to redirect old pinned values to the latest slot value.
        self.function_state
            .pin_slot_names
            .insert(v, slot_name.clone());
        self.function_state
            .pin_slot_names
            .insert(dst, slot_name.clone());
        self.function_state
            .variable_ctx
            .variable_map
            .insert(slot_name, dst);
        Ok(dst)
    }
}

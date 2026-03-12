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

use crate::mir::SpannedInstruction;

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
        let dst = if let Some(ref mut f) = self.scope_ctx.current_function {
            f.next_value_id() // Function context: use local ID
        } else {
            self.core_ctx.next_value() // Module context: use core_ctx SSOT
        };
        self.emit_instruction(super::super::MirInstruction::Copy { dst, src: v })?;
        if super::builder_debug_enabled() || crate::config::env::builder_pin_trace() {
            super::builder_debug_log(&format!("pin slot={} src={} dst={}", slot_name, v.0, dst.0));
        }
        // Propagate lightweight metadata so downstream resolution/type inference remains stable
        crate::mir::builder::metadata::propagate::propagate(self, v, dst);
        // Remember pin slot name for both the original and the pinned value.
        // LocalSSA uses this to redirect old pinned values to the latest slot value.
        self.pin_slot_names.insert(v, slot_name.clone());
        self.pin_slot_names.insert(dst, slot_name.clone());
        self.variable_ctx.variable_map.insert(slot_name, dst);
        Ok(dst)
    }

    /// Ensure a value has a local definition in the current block by inserting a Copy.
    #[allow(dead_code)]
    pub(crate) fn materialize_local(
        &mut self,
        v: super::super::ValueId,
    ) -> Result<super::super::ValueId, String> {
        // Phase 25.1b: Use function-local ID allocator to avoid SSA verification failures
        let dst = if let Some(ref mut f) = self.scope_ctx.current_function {
            f.next_value_id() // Function context: use local ID
        } else {
            self.core_ctx.next_value() // Module context: use core_ctx SSOT
        };
        self.emit_instruction(super::super::MirInstruction::Copy { dst, src: v })?;
        // Propagate metadata (type/origin) from source to the new local copy
        crate::mir::builder::metadata::propagate::propagate(self, v, dst);
        Ok(dst)
    }

    /// Insert a Copy immediately after PHI nodes in the current block (position-stable).
    #[allow(dead_code)]
    pub(crate) fn insert_copy_after_phis(
        &mut self,
        dst: super::super::ValueId,
        src: super::super::ValueId,
    ) -> Result<(), String> {
        if let (Some(ref mut function), Some(bb)) =
            (&mut self.scope_ctx.current_function, self.current_block)
        {
            if crate::config::env::builder_schedule_trace() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[utils/insert-copy-after-phis] bb={:?} dst=%{} src=%{} attempting...",
                    bb, dst.0, src.0
                ));
            }
            if let Some(block) = function.get_block_mut(bb) {
                if crate::config::env::builder_schedule_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[utils/insert-copy-after-phis] bb={:?} dst=%{} src=%{} phi_count={} SUCCESS",
                        bb, dst.0, src.0, block.phi_instructions().count()));
                }
                // Propagate effects on the block
                block.insert_spanned_after_phis(SpannedInstruction {
                    inst: super::super::MirInstruction::Copy { dst, src },
                    span: self.metadata_ctx.current_span(),
                });
                // Lightweight metadata propagation (unified)
                crate::mir::builder::metadata::propagate::propagate(self, src, dst);
                return Ok(());
            } else {
                if crate::config::env::builder_schedule_trace() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[utils/insert-copy-after-phis] bb={:?} dst=%{} src=%{} FAILED: block not found",
                        bb, dst.0, src.0));
                }
            }
        }
        Err("No current function/block to insert copy".to_string())
    }

    /// Ensure a value is safe to use in the current block by slotifying (pinning) it.
    /// Currently correctness-first: always pin to get a block-local def and PHI participation.
    #[allow(dead_code)]
    pub(crate) fn ensure_slotified_for_use(
        &mut self,
        v: super::super::ValueId,
        hint: &str,
    ) -> Result<super::super::ValueId, String> {
        self.pin_to_slot(v, hint)
    }
}

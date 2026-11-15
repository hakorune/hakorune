//! WeakRef and Barrier operations
//!
//! WeakRef type tracking
//! Barrier operations for memory ordering.

use crate::mir::WeakRefOp;

impl super::super::MirBuilder {
    pub(in crate::mir::builder) fn emit_weak_new(
        &mut self,
        box_val: super::super::ValueId,
    ) -> Result<super::super::ValueId, String> {
        let dst = self.next_value_id();

        // Track WeakRef type before emitting its construction.
        self.function_state
            .type_ctx
            .value_types
            .insert(dst, crate::mir::types::MirType::WeakRef);

        self.emit_instruction(super::super::MirInstruction::WeakRef {
            dst,
            op: WeakRefOp::New,
            value: box_val,
        })?;
        Ok(dst)
    }

    pub(in crate::mir::builder) fn emit_weak_load(
        &mut self,
        weak_ref: super::super::ValueId,
    ) -> Result<super::super::ValueId, String> {
        let dst = self.next_value_id();
        self.emit_instruction(super::super::MirInstruction::WeakRef {
            dst,
            op: WeakRefOp::Load,
            value: weak_ref,
        })?;
        Ok(dst)
    }
}

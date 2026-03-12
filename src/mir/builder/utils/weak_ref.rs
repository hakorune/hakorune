//! WeakRef and Barrier operations
//!
//! Phase 285A1: WeakRef type tracking (even in pure mode)
//! Barrier operations for memory ordering.

use crate::mir::{BarrierOp, WeakRefOp};

impl super::super::MirBuilder {
    #[allow(dead_code)]
    pub(in crate::mir::builder) fn emit_weak_new(
        &mut self,
        box_val: super::super::ValueId,
    ) -> Result<super::super::ValueId, String> {
        let dst = self.next_value_id();

        // Phase 285A1: Track WeakRef type (even in pure mode)
        self.type_ctx
            .value_types
            .insert(dst, crate::mir::types::MirType::WeakRef);

        // Phase 285A1: WeakRef type must be tracked even in pure mode
        if crate::config::env::mir_core13_pure() {
            // Pure mode: still track type, but skip instruction
            return Ok(dst);
        }

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
        if crate::config::env::mir_core13_pure() {
            return Ok(weak_ref);
        }
        let dst = self.next_value_id();
        self.emit_instruction(super::super::MirInstruction::WeakRef {
            dst,
            op: WeakRefOp::Load,
            value: weak_ref,
        })?;
        Ok(dst)
    }

    #[allow(dead_code)]
    pub(in crate::mir::builder) fn emit_barrier_read(
        &mut self,
        ptr: super::super::ValueId,
    ) -> Result<(), String> {
        self.emit_instruction(super::super::MirInstruction::Barrier {
            op: BarrierOp::Read,
            ptr,
        })
    }

    #[allow(dead_code)]
    pub(in crate::mir::builder) fn emit_barrier_write(
        &mut self,
        ptr: super::super::ValueId,
    ) -> Result<(), String> {
        self.emit_instruction(super::super::MirInstruction::Barrier {
            op: BarrierOp::Write,
            ptr,
        })
    }
}

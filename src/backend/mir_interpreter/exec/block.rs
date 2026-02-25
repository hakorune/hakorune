use super::super::{MirInterpreter, VMError, VMValue};
use crate::mir::basic_block::BasicBlock;
use crate::mir::{BasicBlockId, MirInstruction, WeakRefOp};

pub(crate) enum BlockOutcome {
    Return(VMValue),
    Next {
        target: BasicBlockId,
        predecessor: BasicBlockId,
    },
}

impl MirInterpreter {
    pub(crate) fn execute_block_instructions(&mut self, block: &BasicBlock) -> Result<(), VMError> {
        // Hot path: when diagnostics are fully off, skip per-inst tracing/stat branches.
        if !self.vm_capture_last_inst_enabled
            && !self.joinir_debug_enabled
            && !self.vm_stats_enabled
            && !self.trace_enabled()
        {
            let mut start = 0usize;
            while start < block.instructions.len()
                && matches!(block.instructions[start], MirInstruction::Phi { .. })
            {
                start += 1;
            }
            for inst in &block.instructions[start..] {
                match inst {
                    MirInstruction::Const { dst, value } => self.handle_const(*dst, value)?,
                    MirInstruction::BinOp { dst, op, lhs, rhs } => {
                        self.handle_binop(*dst, *op, *lhs, *rhs)?
                    }
                    MirInstruction::UnaryOp { dst, op, operand } => {
                        self.handle_unary_op(*dst, *op, *operand)?
                    }
                    MirInstruction::Compare { dst, op, lhs, rhs } => {
                        self.handle_compare(*dst, *op, *lhs, *rhs)?
                    }
                    MirInstruction::Copy { dst, src } => self.handle_copy(*dst, *src)?,
                    MirInstruction::Load { dst, ptr } => self.handle_load(*dst, *ptr)?,
                    MirInstruction::Store { ptr, value } => self.handle_store(*ptr, *value)?,
                    MirInstruction::TypeOp { dst, op, value, ty } => {
                        self.handle_type_op(*dst, *op, *value, ty)?
                    }
                    MirInstruction::Select {
                        dst,
                        cond,
                        then_val,
                        else_val,
                    } => {
                        let cond_val = self.reg_load(*cond)?;
                        let is_true = cond_val.as_bool()?;
                        let selected_val = if is_true {
                            self.reg_load(*then_val)?
                        } else {
                            self.reg_load(*else_val)?
                        };
                        self.write_reg(*dst, selected_val);
                    }
                    MirInstruction::WeakRef { dst, op, value } => match op {
                        WeakRefOp::New => self.handle_weak_new(*dst, *value)?,
                        WeakRefOp::Load => self.handle_weak_load(*dst, *value)?,
                    },
                    MirInstruction::RefNew { dst, box_val } => {
                        self.handle_ref_new(*dst, *box_val)?
                    }
                    MirInstruction::Call {
                        dst,
                        func,
                        callee,
                        args,
                        ..
                    } => self.handle_call(*dst, *func, callee.as_ref(), args)?,
                    _ => self.execute_instruction(inst)?,
                }
            }
            return Ok(());
        }

        let phi_count = block.phi_instructions().count();
        for (idx, sp) in block.iter_spanned_enumerated().skip(phi_count) {
            let inst = sp.inst;
            if self.vm_capture_last_inst_enabled {
                self.last_block = Some(block.id);
                self.last_inst_index = Some(idx);
                self.last_inst = Some(inst.clone());
            }
            if self.joinir_debug_enabled {
                self.record_step_trace(block.id, Some(idx), Some(inst));
            }
            if self.trace_enabled() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0
                    .log
                    .debug(&format!("[vm-trace] inst bb={:?} {:?}", block.id, inst));
            }
            // Dev counters: count non-phi instructions and compares
            if self.vm_stats_enabled {
                self.inst_count = self.inst_count.wrapping_add(1);
                if let MirInstruction::Compare { .. } = inst {
                    self.compare_count = self.compare_count.wrapping_add(1);
                }
            }
            self.execute_instruction(inst)?;
        }
        Ok(())
    }

    pub(crate) fn handle_terminator(
        &mut self,
        block: &BasicBlock,
    ) -> Result<BlockOutcome, VMError> {
        if self.vm_capture_last_inst_enabled {
            if let Some(term) = &block.terminator {
                self.last_block = Some(block.id);
                self.last_inst_index = Some(block.instructions.len());
                self.last_inst = Some(term.clone());
                if self.joinir_debug_enabled {
                    self.record_step_trace(block.id, Some(block.instructions.len()), Some(term));
                }
            }
        }
        match &block.terminator {
            Some(MirInstruction::Return { value }) => {
                let result = if let Some(v) = value {
                    self.reg_load(*v)?
                } else {
                    VMValue::Void
                };
                Ok(BlockOutcome::Return(result))
            }
            Some(MirInstruction::Jump { target, .. }) => Ok(BlockOutcome::Next {
                target: *target,
                predecessor: block.id,
            }),
            Some(MirInstruction::Branch {
                condition,
                then_bb,
                else_bb,
                ..
            }) => {
                // Dev counter: count branch terminators actually evaluated
                if self.vm_stats_enabled {
                    self.branch_count = self.branch_count.wrapping_add(1);
                }
                let branch = if let Some(b) = self.vm_fast_read_bool(*condition) {
                    b
                } else if let Some(i) = self.vm_fast_read_i64(*condition) {
                    i != 0
                } else {
                    match self.reg_peek_resolved(*condition) {
                        Some(cond) => super::super::to_bool_vm(cond).map_err(VMError::TypeError)?,
                        None => {
                            let cond = self.reg_load(*condition)?;
                            super::super::to_bool_vm(&cond).map_err(VMError::TypeError)?
                        }
                    }
                };
                let target = if branch { *then_bb } else { *else_bb };
                Ok(BlockOutcome::Next {
                    target,
                    predecessor: block.id,
                })
            }
            None => Err(VMError::InvalidBasicBlock(format!(
                "unterminated block {:?}",
                block.id
            ))),
            Some(other) => Err(VMError::InvalidInstruction(format!(
                "invalid terminator in MIR interp: {:?}",
                other
            ))),
        }
    }
}

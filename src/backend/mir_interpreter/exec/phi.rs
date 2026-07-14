use super::super::{MirInterpreter, VMError, VMValue};
use crate::mir::BasicBlock;
use crate::mir::{BasicBlockId, MirInstruction};

#[inline]
fn select_phi_input(
    inputs: &[(BasicBlockId, crate::mir::ValueId)],
    pred: BasicBlockId,
) -> Option<crate::mir::ValueId> {
    match inputs {
        [(bb0, v0)] if *bb0 == pred => Some(*v0),
        [(bb0, v0), (_, _)] if *bb0 == pred => Some(*v0),
        [(_, _), (bb1, v1)] if *bb1 == pred => Some(*v1),
        _ => inputs.iter().find(|(bb, _)| *bb == pred).map(|(_, v)| *v),
    }
}

impl MirInterpreter {
    pub(crate) fn apply_phi_nodes(
        &mut self,
        block: &BasicBlock,
        last_pred: Option<BasicBlockId>,
    ) -> Result<(), VMError> {
        self.apply_owned_phi_nodes(block, last_pred)?;
        let trace_phi = self.vm_trace_phi_enabled;
        let tolerate_undefined = self.vm_phi_tolerate_undefined_enabled;
        let strict = self.vm_phi_strict_enabled;
        let trace_exec = self.trace_enabled();

        #[inline]
        fn load_phi_value(
            this: &mut MirInterpreter,
            val: crate::mir::ValueId,
            tolerate_undefined: bool,
            trace_exec: bool,
            trace_msg: &str,
        ) -> Result<VMValue, VMError> {
            if let Some(v) = this.reg_peek_resolved(val) {
                return Ok(v.clone());
            }
            match this.reg_load(val) {
                Ok(v) => Ok(v),
                Err(e) => {
                    if tolerate_undefined {
                        if trace_exec {
                            crate::runtime::get_global_ring0()
                                .log
                                .debug(&format!("{} {:?} -> Void (err={:?})", trace_msg, val, e));
                        }
                        Ok(VMValue::Void)
                    } else {
                        Err(e)
                    }
                }
            }
        }

        // Hot path: strict PHI + diagnostics off.
        if !self.vm_capture_last_inst_enabled
            && !trace_phi
            && !trace_exec
            && !tolerate_undefined
            && strict
        {
            for inst in block.phi_instructions() {
                if let MirInstruction::Phi { dst, inputs, .. } = inst {
                    if self.is_active_owned_value(*dst) {
                        continue;
                    }
                    let dst_id = *dst;
                    let selected = if let Some(pred) = last_pred {
                        select_phi_input(inputs, pred).ok_or_else(|| {
                            VMError::InvalidInstruction(format!(
                                "phi pred mismatch at {:?}: no input for predecessor {:?}",
                                dst_id, pred
                            ))
                        })?
                    } else if let Some((_, val)) = inputs.first() {
                        *val
                    } else {
                        continue;
                    };
                    let v = match self.reg_peek_resolved(selected) {
                        Some(v) => v.clone(),
                        None => self.reg_load(selected)?,
                    };
                    match &v {
                        VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst_id, *i),
                        VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst_id, *b),
                        _ => self.vm_fast_cache_clear(dst_id),
                    }
                    self.write_reg(dst_id, v);
                }
            }
            return Ok(());
        }

        if trace_phi {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[vm-trace-phi] enter bb={:?} last_pred={:?} preds={:?}",
                block.id, last_pred, block.predecessors
            ));
        }
        for (idx, inst) in block.phi_instructions().enumerate() {
            if self.vm_capture_last_inst_enabled {
                self.last_block = Some(block.id);
                self.last_inst_index = Some(idx);
                self.last_inst = Some(inst.clone());
            }
            if let MirInstruction::Phi { dst, inputs, .. } = inst {
                if self.is_active_owned_value(*dst) {
                    continue;
                }
                let dst_id = *dst;
                if trace_phi {
                    let in_preds: Vec<_> = inputs.iter().map(|(bb, _)| *bb).collect();
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace-phi] phi dst={:?} inputs.pred={:?}",
                        dst_id, in_preds
                    ));
                }
                if let Some(pred) = last_pred {
                    if let Some(val) = select_phi_input(inputs, pred) {
                        let v = load_phi_value(
                            self,
                            val,
                            tolerate_undefined,
                            trace_exec,
                            "[vm-trace] phi tolerate undefined input",
                        )?;
                        match &v {
                            VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst_id, *i),
                            VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst_id, *b),
                            _ => self.vm_fast_cache_clear(dst_id),
                        }
                        self.write_reg(dst_id, v);
                        if trace_exec {
                            crate::runtime::get_global_ring0().log.debug(&format!(
                                "[vm-trace] phi dst={:?} take pred={:?} val={:?}",
                                dst_id, pred, val
                            ));
                        }
                    } else {
                        // No matching predecessor in PHI inputs. Fallback policy:
                        //  - Prefer first input when available to avoid use-before-def crashes.
                        //  - Emit a trace note when tracing is enabled.
                        // Strict mode: fail-fast when enabled.
                        // Strict PHI (default ON). Can be disabled with HAKO_VM_PHI_STRICT=0 (or NYASH_VM_PHI_STRICT=0).
                        if strict {
                            if trace_phi {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[vm-trace-phi][strict] mismatch dst={:?} last_pred={:?} inputs={:?} preds_of_bb={:?}",
                                    dst_id,
                                    pred,
                                    inputs,
                                    block.predecessors
                                ));
                            }
                            return Err(VMError::InvalidInstruction(format!(
                                "phi pred mismatch at {:?}: no input for predecessor {:?}",
                                dst_id, pred
                            )));
                        }
                        if let Some((_, val0)) = inputs.first() {
                            let v = load_phi_value(
                                self,
                                *val0,
                                tolerate_undefined,
                                trace_exec,
                                "[vm-trace] phi missing pred fallback first input",
                            )?;
                            match &v {
                                VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst_id, *i),
                                VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst_id, *b),
                                _ => self.vm_fast_cache_clear(dst_id),
                            }
                            self.write_reg(dst_id, v);
                            if trace_exec {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[vm-trace] phi dst={:?} pred-miss fallback first val={:?}",
                                    dst_id, val0
                                ));
                            }
                        } else {
                            // Empty inputs — assign Void (with optional trace) to avoid undefined dst
                            let v = VMValue::Void;
                            self.vm_fast_cache_clear(dst_id);
                            self.write_reg(dst_id, v);
                            if trace_exec {
                                crate::runtime::get_global_ring0().log.debug(&format!(
                                    "[vm-trace] phi dst={:?} no inputs; assign Void",
                                    dst_id
                                ));
                            }
                        }
                    }
                } else if let Some((_, val)) = inputs.first() {
                    let v = load_phi_value(
                        self,
                        *val,
                        tolerate_undefined,
                        trace_exec,
                        "[vm-trace] phi tolerate undefined default input",
                    )?;
                    match &v {
                        VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst_id, *i),
                        VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst_id, *b),
                        _ => self.vm_fast_cache_clear(dst_id),
                    }
                    self.write_reg(dst_id, v);
                    if trace_exec {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[vm-trace] phi dst={:?} take default val={:?}",
                            dst_id, val
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    fn is_active_owned_value(&self, value: crate::mir::ValueId) -> bool {
        self.active_ownership_ssa
            .as_ref()
            .and_then(|witness| witness.kind(value))
            == Some(crate::mir::ownership_ssa::MirOwnershipKindV1::Owned)
    }

    fn apply_owned_phi_nodes(
        &mut self,
        block: &BasicBlock,
        last_pred: Option<BasicBlockId>,
    ) -> Result<(), VMError> {
        if self.active_ownership_ssa.is_none() {
            return Ok(());
        }
        let mut transfers = Vec::new();
        for instruction in block.phi_instructions() {
            let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                continue;
            };
            if !self.is_active_owned_value(*dst) {
                continue;
            }
            let predecessor = last_pred.ok_or_else(|| {
                VMError::InvalidInstruction(format!(
                    "[freeze:contract][vm/ownership:owned_phi_without_predecessor] dst={dst}"
                ))
            })?;
            let source = select_phi_input(inputs, predecessor).ok_or_else(|| {
                VMError::InvalidInstruction(format!(
                    "[freeze:contract][vm/ownership:owned_phi_input_missing] dst={dst} pred={predecessor}"
                ))
            })?;
            if transfers.iter().any(|(_, existing)| *existing == source) {
                return Err(VMError::InvalidInstruction(format!(
                    "[freeze:contract][vm/ownership:duplicate_phi_source] source={source} pred={predecessor}"
                )));
            }
            transfers.push((*dst, source));
        }

        let mut values = Vec::with_capacity(transfers.len());
        for (destination, source) in &transfers {
            let value = self.take_reg(*source).ok_or_else(|| {
                VMError::InvalidValue(format!("Owned Phi source is undefined: {source}"))
            })?;
            if !matches!(value, VMValue::BoxRef(_)) {
                return Err(VMError::TypeError(format!(
                    "Owned Phi expects BoxRef at {source}, got {value:?}"
                )));
            }
            values.push((*destination, value));
        }
        for (destination, value) in values {
            self.write_reg(destination, value);
        }
        Ok(())
    }
}

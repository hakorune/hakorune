use crate::ast::Span;
use crate::mir::types::ConstValue;
use crate::mir::{MirFunction, MirInstruction, MirType};

pub(crate) fn log_dbg(message: impl AsRef<str>) {
    if crate::config::env::joinir_test_debug_enabled() {
        crate::runtime::get_global_ring0()
            .log
            .debug(message.as_ref());
    }
}

pub(crate) fn finalize_block(
    mir_func: &mut MirFunction,
    block_id: crate::mir::BasicBlockId,
    instructions: Vec<MirInstruction>,
    terminator: MirInstruction,
) {
    log_dbg(format!(
        "[joinir_block/finalize_block] block_id={:?}, instructions.len()={}",
        block_id,
        instructions.len()
    ));
    if let Some(block) = mir_func.blocks.get_mut(&block_id) {
        // Phase 189 FIX: Preserve existing PHI instructions at block start
        // PHI instructions must remain at the beginning of the block
        let existing_phis: Vec<_> = block
            .instructions
            .iter()
            .filter(|i| matches!(i, MirInstruction::Phi { .. }))
            .cloned()
            .collect();
        let phi_count = existing_phis.len();

        if phi_count > 0 {
            log_dbg(format!(
                "[joinir_block/finalize_block] Preserving {} PHI instructions in block {:?}",
                phi_count, block_id
            ));
            // PHI first, then new instructions
            let mut merged = existing_phis;
            merged.extend(instructions);
            let total_count = merged.len();
            block.instructions = merged;
            block.instruction_spans = vec![Span::unknown(); total_count];
        } else {
            let inst_count = instructions.len();
            block.instructions = instructions;
            block.instruction_spans = vec![Span::unknown(); inst_count];
        }
        block.set_terminator(terminator);
    }
}

pub(crate) fn finalize_remaining_instructions(
    mir_func: &mut MirFunction,
    current_block_id: crate::mir::BasicBlockId,
    current_instructions: &mut Vec<MirInstruction>,
) {
    if !current_instructions.is_empty() {
        log_dbg(format!(
            "[joinir_block] Final block {:?} has {} remaining instructions",
            current_block_id,
            current_instructions.len()
        ));
        if let Some(block) = mir_func.blocks.get_mut(&current_block_id) {
            // Phase 189 FIX: Use extend() instead of assignment to preserve
            // existing instructions (e.g., PHI from handle_select())
            let new_insts = std::mem::take(current_instructions);
            let new_count = new_insts.len();
            block.instructions.extend(new_insts);
            block
                .instruction_spans
                .extend(vec![Span::unknown(); new_count]);
        }
    }
}

pub(crate) fn annotate_value_types_for_inst(mir_func: &mut MirFunction, inst: &MirInstruction) {
    match inst {
        MirInstruction::Const { dst, value } => {
            let ty = match value {
                ConstValue::Integer(_) => Some(MirType::Integer),
                ConstValue::Float(_) => Some(MirType::Float),
                ConstValue::Bool(_) => Some(MirType::Bool),
                ConstValue::String(_) => Some(MirType::String),
                ConstValue::Void => Some(MirType::Void),
                _ => None,
            };
            if let Some(ty) = ty {
                mir_func.metadata.value_types.insert(*dst, ty);
            }
        }
        MirInstruction::BinOp { dst, lhs, rhs, .. } => {
            // Conservative typing for numeric operations.
            //
            // This avoids forcing numeric semantics for `+` in cases where the operands are not
            // known to be numeric (e.g. string concatenation patterns).
            let lhs_ty = mir_func.metadata.value_types.get(lhs);
            let rhs_ty = mir_func.metadata.value_types.get(rhs);

            // Phase 275 P0: Int + Float promotion
            // If either operand is Float, result is Float
            if matches!(lhs_ty, Some(MirType::Float)) || matches!(rhs_ty, Some(MirType::Float)) {
                mir_func.metadata.value_types.insert(*dst, MirType::Float);
            } else if matches!(lhs_ty, Some(MirType::Integer))
                && matches!(rhs_ty, Some(MirType::Integer))
            {
                mir_func.metadata.value_types.insert(*dst, MirType::Integer);
            }
        }
        MirInstruction::Compare { dst, .. } => {
            mir_func.metadata.value_types.insert(*dst, MirType::Bool);
        }
        MirInstruction::TypeOp { dst, op, ty, .. } => {
            let inferred = match op {
                crate::mir::TypeOpKind::Check => MirType::Bool,
                crate::mir::TypeOpKind::Cast => ty.clone(),
            };
            mir_func.metadata.value_types.insert(*dst, inferred);
        }
        MirInstruction::Phi {
            dst,
            type_hint,
            inputs,
        } => {
            if let Some(ty) = type_hint.clone() {
                mir_func.metadata.value_types.insert(*dst, ty);
            } else {
                // Phase 275 P0: Infer PHI type from incoming values
                // If all incoming values have the same type, use that type
                let mut inferred_type: Option<MirType> = None;
                for (_, incoming_vid) in inputs.iter() {
                    if let Some(incoming_ty) = mir_func.metadata.value_types.get(incoming_vid) {
                        match &inferred_type {
                            None => {
                                inferred_type = Some(incoming_ty.clone());
                            }
                            Some(existing) => {
                                // If types don't match, we can't infer
                                if existing != incoming_ty {
                                    inferred_type = None;
                                    break;
                                }
                            }
                        }
                    }
                }
                if let Some(ty) = inferred_type {
                    mir_func.metadata.value_types.insert(*dst, ty);
                }
            }
        }
        _ => {}
    }
}

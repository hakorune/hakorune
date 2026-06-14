//! PHI input materialization SSOT.
//!
//! A PHI input is an edge value: the incoming value must be valid at the
//! predecessor block attached to that input. If a pure value was defined on a
//! sibling path, this helper rematerializes an equivalent value in the
//! predecessor before PHI insertion.

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::HashSet;

fn find_def_inst(
    func: &MirFunction,
    value: ValueId,
) -> Option<(BasicBlockId, Option<MirInstruction>)> {
    if func.params.iter().any(|param| *param == value) {
        return Some((func.entry_block, None));
    }

    for (bb, block) in &func.blocks {
        for inst in &block.instructions {
            if inst.dst_value() == Some(value) {
                return Some((*bb, Some(inst.clone())));
            }
        }
        if let Some(term) = &block.terminator {
            if term.dst_value() == Some(value) {
                return Some((*bb, Some(term.clone())));
            }
        }
    }

    None
}

fn rematerialize_for_pred(
    func: &mut MirFunction,
    pred: BasicBlockId,
    value: ValueId,
    context: &str,
    edge_kind: &str,
    visited: &mut HashSet<ValueId>,
) -> Result<ValueId, String> {
    func.update_cfg();
    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
    let dominators = crate::mir::verification::utils::compute_dominators(func);
    let dominates_pred = def_blocks
        .get(&value)
        .copied()
        .map(|def_bb| dominators.dominates(def_bb, pred))
        .unwrap_or(false);

    if !visited.insert(value) {
        return Err(format!(
            "[freeze:contract][ssa/phi_input/remat_cycle] fn={} pred={:?} context={} edge={} value=%{}",
            func.signature.name, pred, context, edge_kind, value.0
        ));
    }

    let Some((def_bb, def_inst)) = find_def_inst(func, value) else {
        return Err(format!(
            "[freeze:contract][ssa/phi_input/without_def] fn={} pred={:?} context={} edge={} value=%{}",
            func.signature.name, pred, context, edge_kind, value.0
        ));
    };

    let Some(def_inst) = def_inst else {
        if dominates_pred {
            visited.remove(&value);
            return Ok(value);
        }
        return Err(format!(
            "[freeze:contract][ssa/phi_input/non_dominating_param] fn={} pred={:?} context={} edge={} value=%{} def_block={:?}",
            func.signature.name, pred, context, edge_kind, value.0, def_bb
        ));
    };

    if def_bb == pred {
        visited.remove(&value);
        return Ok(value);
    }

    let remat = match def_inst {
        MirInstruction::Const {
            value: const_value, ..
        } => {
            let dst = func.next_value_id();
            MirInstruction::Const {
                dst,
                value: const_value,
            }
        }
        MirInstruction::Copy { src, .. } => {
            let src = rematerialize_for_pred(func, pred, src, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::Copy { dst, src }
        }
        MirInstruction::BinOp { op, lhs, rhs, .. } => {
            let lhs = rematerialize_for_pred(func, pred, lhs, context, edge_kind, visited)?;
            let rhs = rematerialize_for_pred(func, pred, rhs, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::BinOp { dst, op, lhs, rhs }
        }
        MirInstruction::Compare { op, lhs, rhs, .. } => {
            let lhs = rematerialize_for_pred(func, pred, lhs, context, edge_kind, visited)?;
            let rhs = rematerialize_for_pred(func, pred, rhs, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::Compare { dst, op, lhs, rhs }
        }
        MirInstruction::UnaryOp { op, operand, .. } => {
            let operand = rematerialize_for_pred(func, pred, operand, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::UnaryOp { dst, op, operand }
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            let cond = rematerialize_for_pred(func, pred, cond, context, edge_kind, visited)?;
            let then_val =
                rematerialize_for_pred(func, pred, then_val, context, edge_kind, visited)?;
            let else_val =
                rematerialize_for_pred(func, pred, else_val, context, edge_kind, visited)?;
            let dst = func.next_value_id();
            MirInstruction::Select {
                dst,
                cond,
                then_val,
                else_val,
            }
        }
        other => {
            if dominates_pred {
                visited.remove(&value);
                return Ok(value);
            }
            return Err(format!(
                "[freeze:contract][ssa/phi_input/non_rematerializable] fn={} pred={:?} context={} edge={} value=%{} def_block={:?} def_kind={:?}",
                func.signature.name, pred, context, edge_kind, value.0, def_bb, other
            ));
        }
    };

    let dst = remat
        .dst_value()
        .ok_or_else(|| "[ssa/phi_input] rematerialized instruction missing dst".to_string())?;
    let fn_name = func.signature.name.clone();
    let block = func.get_block_mut(pred).ok_or_else(|| {
        format!(
            "[freeze:contract][ssa/phi_input/missing_pred_block] fn={} pred={:?} context={} edge={} value=%{}",
            fn_name, pred, context, edge_kind, value.0
        )
    })?;
    block.add_instruction_before_terminator(remat);
    visited.remove(&value);
    Ok(dst)
}

pub(in crate::mir::builder) fn for_pred(
    func: &mut MirFunction,
    pred: BasicBlockId,
    value: ValueId,
    context: &str,
    edge_kind: &str,
) -> Result<ValueId, String> {
    rematerialize_for_pred(func, pred, value, context, edge_kind, &mut HashSet::new())
}

pub(in crate::mir::builder) fn materialize_all_phi_inputs(
    func: &mut MirFunction,
    context: &str,
) -> Result<usize, String> {
    let mut work = Vec::new();
    for (block_id, block) in &func.blocks {
        for (inst_idx, inst) in block.instructions.iter().enumerate() {
            if let MirInstruction::Phi { inputs, .. } = inst {
                for (input_idx, (pred, incoming)) in inputs.iter().enumerate() {
                    work.push((*block_id, inst_idx, input_idx, *pred, *incoming));
                }
            }
        }
    }

    let mut changed = 0usize;
    for (block_id, inst_idx, input_idx, pred, incoming) in work {
        let materialized = for_pred(func, pred, incoming, context, "phi")?;
        if materialized == incoming {
            continue;
        }
        let fn_name = func.signature.name.clone();
        let block = func.get_block_mut(block_id).ok_or_else(|| {
            format!(
                "[freeze:contract][ssa/phi_input/missing_phi_block] fn={} block={:?} context={}",
                fn_name, block_id, context
            )
        })?;
        let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get_mut(inst_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_inst] fn={} block={:?} inst_idx={} context={}",
                fn_name, block_id, inst_idx, context
            ));
        };
        let Some((_, slot)) = inputs.get_mut(input_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_input] fn={} block={:?} inst_idx={} input_idx={} context={}",
                fn_name, block_id, inst_idx, input_idx, context
            ));
        };
        *slot = materialized;
        changed += 1;
    }

    Ok(changed)
}

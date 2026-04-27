/*!
 * SimplifyCFG - first structural simplification slice for the semantic bundle.
 *
 * This cut intentionally stays narrow:
 * - fold copied-constant `Branch` terminators to `Jump`
 * - fold constant `Compare` instructions to `Const Bool`, then let branch
 *   folding consume the resulting value
 * - follow copied / passthrough single-input PHIs when they carry a constant
 *   value into compare or branch conditions
 * - thread a branch arm through an empty jump trampoline when the final target
 *   has no PHIs, or only PHIs that can be trivially rewritten from the
 *   trampoline predecessor to the branching block
 * - branch-arm edge-args may be dropped only when they are dead for a
 *   PHI-free final target after threading
 * - merge `pred -> middle` only for a direct `Jump`
 * - `middle` must be reachable, non-entry, have exactly one predecessor, and
 *   carry only trivial single-input PHIs from that predecessor
 * - loop-carried/self-edge cases stay out of scope
 */

use crate::mir::join_ir_ops::{eval_compare, JoinValue};
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::{BasicBlockId, ConstValue, MirFunction, MirInstruction, MirModule, ValueId};

#[path = "simplify_cfg/flow.rs"]
mod flow;
use self::flow::{
    find_single_predecessor_jump_merge, find_threadable_branch_jump,
    merge_single_predecessor_jump_block, thread_branch_arm,
};

pub fn simplify(module: &mut MirModule) -> usize {
    let mut simplified = 0usize;
    for function in module.functions.values_mut() {
        simplified += simplify_function(function);
    }
    simplified
}

fn simplify_function(function: &mut MirFunction) -> usize {
    let mut simplified = 0usize;

    loop {
        function.update_cfg();
        if let Some((block_id, inst_idx, value)) = find_constant_compare_fold(function) {
            fold_constant_compare(function, block_id, inst_idx, value);
            simplified += 1;
            continue;
        }
        if let Some((block_id, target, edge_args)) = find_constant_branch_fold(function) {
            fold_constant_branch(function, block_id, target, edge_args);
            simplified += 1;
            continue;
        }
        if let Some((block_id, arm, middle_id, target, rewrite_phi, clear_edge_args)) =
            find_threadable_branch_jump(function)
        {
            thread_branch_arm(
                function,
                block_id,
                arm,
                middle_id,
                target,
                rewrite_phi,
                clear_edge_args,
            );
            simplified += 1;
            continue;
        }
        let Some((pred_id, middle_id)) = find_single_predecessor_jump_merge(function) else {
            break;
        };
        merge_single_predecessor_jump_block(function, pred_id, middle_id);
        simplified += 1;
    }

    simplified
}

fn find_constant_branch_fold(
    function: &MirFunction,
) -> Option<(BasicBlockId, BasicBlockId, Option<crate::mir::EdgeArgs>)> {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);
    let def_map = build_value_def_map(function);

    for block_id in function.block_ids() {
        if !reachable_blocks.contains(&block_id) {
            continue;
        }

        let block = function.blocks.get(&block_id)?;
        let MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
        } = block.terminator.as_ref()?
        else {
            continue;
        };

        let Some(condition_value) = const_bool_value(function, &def_map, *condition) else {
            continue;
        };

        let (target, edge_args) = if condition_value {
            (*then_bb, then_edge_args.clone())
        } else {
            (*else_bb, else_edge_args.clone())
        };
        if target == block_id {
            continue;
        }
        return Some((block_id, target, edge_args));
    }

    None
}

fn find_constant_compare_fold(function: &MirFunction) -> Option<(BasicBlockId, usize, bool)> {
    let def_map = build_value_def_map(function);

    for block_id in function.block_ids() {
        let block = function.blocks.get(&block_id)?;
        for (inst_idx, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Compare { op, lhs, rhs, .. } = instruction else {
                continue;
            };
            let Some(value) = compare_to_bool_const(function, &def_map, *op, *lhs, *rhs) else {
                continue;
            };
            return Some((block_id, inst_idx, value));
        }
    }

    None
}

fn compare_to_bool_const(
    function: &MirFunction,
    def_map: &ValueDefMap,
    op: crate::mir::CompareOp,
    lhs: ValueId,
    rhs: ValueId,
) -> Option<bool> {
    let lhs = const_to_join_value(function, def_map, lhs)?;
    let rhs = const_to_join_value(function, def_map, rhs)?;
    let result = eval_compare(to_join_compare_op(op), &lhs, &rhs).ok()?;
    match result {
        JoinValue::Bool(b) => Some(b),
        _ => None,
    }
}

fn const_bool_value(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> Option<bool> {
    match const_value(function, def_map, value) {
        Some(ConstValue::Bool(b)) => Some(b),
        _ => None,
    }
}

fn const_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<ConstValue> {
    let mut visited = std::collections::BTreeSet::new();
    const_value_from_origin(function, def_map, value, &mut visited)
}

fn const_value_from_origin(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    visited: &mut std::collections::BTreeSet<ValueId>,
) -> Option<ConstValue> {
    let origin = resolve_value_origin(function, def_map, value);
    if !visited.insert(origin) {
        return None;
    }
    let (block_id, inst_idx) = def_map.get(&origin).copied()?;
    let block = function.blocks.get(&block_id)?;
    match block.instructions.get(inst_idx)? {
        MirInstruction::Const { value, .. } => Some(value.clone()),
        MirInstruction::Compare { op, lhs, rhs, .. } => {
            let lhs = const_to_join_value(function, def_map, *lhs)?;
            let rhs = const_to_join_value(function, def_map, *rhs)?;
            let result = eval_compare(to_join_compare_op(*op), &lhs, &rhs).ok()?;
            match result {
                JoinValue::Bool(b) => Some(ConstValue::Bool(b)),
                _ => None,
            }
        }
        MirInstruction::Phi { inputs, .. } => {
            let [(.., incoming)] = inputs.as_slice() else {
                return None;
            };
            const_value_from_origin(function, def_map, *incoming, visited)
        }
        _ => None,
    }
}

fn const_to_join_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> Option<JoinValue> {
    match const_value(function, def_map, value)? {
        ConstValue::Integer(n) => Some(JoinValue::Int(n)),
        ConstValue::Float(_) => None,
        ConstValue::Bool(b) => Some(JoinValue::Bool(b)),
        ConstValue::String(s) => Some(JoinValue::Str(s)),
        ConstValue::Null | ConstValue::Void => None,
    }
}

fn to_join_compare_op(op: crate::mir::CompareOp) -> crate::mir::join_ir::CompareOp {
    match op {
        crate::mir::CompareOp::Lt => crate::mir::join_ir::CompareOp::Lt,
        crate::mir::CompareOp::Le => crate::mir::join_ir::CompareOp::Le,
        crate::mir::CompareOp::Gt => crate::mir::join_ir::CompareOp::Gt,
        crate::mir::CompareOp::Ge => crate::mir::join_ir::CompareOp::Ge,
        crate::mir::CompareOp::Eq => crate::mir::join_ir::CompareOp::Eq,
        crate::mir::CompareOp::Ne => crate::mir::join_ir::CompareOp::Ne,
    }
}

fn fold_constant_compare(
    function: &mut MirFunction,
    block_id: BasicBlockId,
    inst_idx: usize,
    value: bool,
) {
    let block = function
        .blocks
        .get_mut(&block_id)
        .expect("constant-compare block must exist");
    let instruction = block
        .instructions
        .get_mut(inst_idx)
        .expect("constant-compare instruction must exist");
    let MirInstruction::Compare { dst, .. } = instruction else {
        return;
    };
    let dst = *dst;
    *instruction = MirInstruction::Const {
        dst,
        value: ConstValue::Bool(value),
    };
}

fn fold_constant_branch(
    function: &mut MirFunction,
    block_id: BasicBlockId,
    target: BasicBlockId,
    edge_args: Option<crate::mir::EdgeArgs>,
) {
    let block = function
        .blocks
        .get_mut(&block_id)
        .expect("constant-branch block must exist");
    block.set_terminator(MirInstruction::Jump { target, edge_args });
    function.update_cfg();
}

#[cfg(test)]
#[path = "simplify_cfg_tests.rs"]
mod tests;

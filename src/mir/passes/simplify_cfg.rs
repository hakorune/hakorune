/*!
 * SimplifyCFG - first structural simplification slice for the semantic bundle.
 *
 * This cut intentionally stays narrow:
 * - fold copied-constant `Branch` terminators to `Jump`
 * - fold constant `Compare` instructions to `Const Bool`, then let branch
 *   folding consume the resulting value
 * - thread a branch arm through an empty jump trampoline when the final target
 *   has no PHIs, or only PHIs that can be trivially rewritten from the
 *   trampoline predecessor to the branching block
 * - merge `pred -> middle` only for a direct `Jump`
 * - `middle` must be reachable, non-entry, have exactly one predecessor, and
 *   carry only trivial single-input PHIs from that predecessor
 * - loop-carried/self-edge cases stay out of scope
 */

use crate::mir::join_ir_ops::{eval_compare, JoinValue};
use crate::mir::{
    build_value_def_map, definitions::call_unified::Callee, resolve_value_origin, BasicBlock,
    BasicBlockId, ConstValue, EffectMask, MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThreadArm {
    Then,
    Else,
}

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
        if let Some((block_id, arm, middle_id, target, rewrite_phi)) =
            find_threadable_branch_jump(function)
        {
            thread_branch_arm(function, block_id, arm, middle_id, target, rewrite_phi);
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
    def_map: &crate::mir::ValueDefMap,
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

fn const_bool_value(
    function: &MirFunction,
    def_map: &crate::mir::ValueDefMap,
    value: ValueId,
) -> Option<bool> {
    match const_value(function, def_map, value) {
        Some(ConstValue::Bool(b)) => Some(b),
        _ => None,
    }
}

fn const_value(
    function: &MirFunction,
    def_map: &crate::mir::ValueDefMap,
    value: ValueId,
) -> Option<ConstValue> {
    let origin = resolve_value_origin(function, def_map, value);
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
        _ => None,
    }
}

fn const_to_join_value(
    function: &MirFunction,
    def_map: &crate::mir::ValueDefMap,
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

fn find_threadable_branch_jump(
    function: &MirFunction,
) -> Option<(BasicBlockId, ThreadArm, BasicBlockId, BasicBlockId, bool)> {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);

    for block_id in function.block_ids() {
        if !reachable_blocks.contains(&block_id) {
            continue;
        }

        let block = function.blocks.get(&block_id)?;
        let MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
            ..
        } = block.terminator.as_ref()?
        else {
            continue;
        };

        if let Some((target, rewrite_phi)) =
            threadable_jump_target(function, block_id, *then_bb, then_edge_args.as_ref())
        {
            if target != *else_bb {
                return Some((block_id, ThreadArm::Then, *then_bb, target, rewrite_phi));
            }
            if !rewrite_phi && *else_edge_args == None {
                return Some((block_id, ThreadArm::Then, *then_bb, target, rewrite_phi));
            }
        }

        if let Some((target, rewrite_phi)) =
            threadable_jump_target(function, block_id, *else_bb, else_edge_args.as_ref())
        {
            if target != *then_bb {
                return Some((block_id, ThreadArm::Else, *else_bb, target, rewrite_phi));
            }
            if !rewrite_phi && *then_edge_args == None {
                return Some((block_id, ThreadArm::Else, *else_bb, target, rewrite_phi));
            }
        }
    }

    None
}

fn threadable_jump_target(
    function: &MirFunction,
    pred_id: BasicBlockId,
    middle_id: BasicBlockId,
    edge_args: Option<&crate::mir::EdgeArgs>,
) -> Option<(BasicBlockId, bool)> {
    if edge_args.is_some() {
        return None;
    }

    let middle_block = function.blocks.get(&middle_id)?;
    if !middle_block.instructions.is_empty() {
        return None;
    }
    if middle_block.phi_instructions().next().is_some() {
        return None;
    }

    let MirInstruction::Jump {
        target,
        edge_args: None,
    } = middle_block.terminator.as_ref()?
    else {
        return None;
    };
    if *target == middle_id || *target == function.entry_block {
        return None;
    }

    let final_block = function.blocks.get(target)?;
    if final_block.phi_instructions().next().is_some() {
        if middle_block.predecessors.len() != 1 || !middle_block.predecessors.contains(&pred_id) {
            return None;
        }
        if !can_rewrite_threaded_phi_predecessor(final_block, middle_id, pred_id) {
            return None;
        }
        return Some((*target, true));
    }

    Some((*target, false))
}

fn thread_branch_arm(
    function: &mut MirFunction,
    block_id: BasicBlockId,
    arm: ThreadArm,
    middle_id: BasicBlockId,
    target: BasicBlockId,
    rewrite_phi: bool,
) {
    {
        let block = function
            .blocks
            .get_mut(&block_id)
            .expect("threading block must exist");
        let MirInstruction::Branch {
            then_bb,
            else_bb,
            then_edge_args,
            else_edge_args,
            ..
        } = block
            .terminator
            .as_mut()
            .expect("threading block must terminate")
        else {
            return;
        };

        match arm {
            ThreadArm::Then => {
                *then_bb = target;
            }
            ThreadArm::Else => {
                *else_bb = target;
            }
        }

        if *then_bb == *else_bb && then_edge_args == else_edge_args {
            let target = *then_bb;
            let edge_args = then_edge_args.clone();
            block.set_terminator(MirInstruction::Jump { target, edge_args });
        } else {
            block.successors = block.successors_from_terminator();
        }
    }

    if rewrite_phi {
        rewrite_phi_predecessor(function, middle_id, block_id);
    }

    function.update_cfg();
}

fn can_rewrite_threaded_phi_predecessor(
    final_block: &BasicBlock,
    old_predecessor: BasicBlockId,
    new_predecessor: BasicBlockId,
) -> bool {
    let mut saw_phi = false;
    for instruction in final_block.phi_instructions() {
        let MirInstruction::Phi { inputs, .. } = instruction else {
            unreachable!("phi_instructions() must yield only PHI instructions");
        };
        let mut saw_old = false;
        for (incoming_block, _) in inputs {
            if *incoming_block == new_predecessor {
                return false;
            }
            if *incoming_block == old_predecessor {
                if saw_old {
                    return false;
                }
                saw_old = true;
            }
        }
        if !saw_old {
            return false;
        }
        saw_phi = true;
    }
    saw_phi
}

fn find_single_predecessor_jump_merge(
    function: &MirFunction,
) -> Option<(BasicBlockId, BasicBlockId)> {
    let reachable_blocks = crate::mir::verification::utils::compute_reachable_blocks(function);

    for pred_id in function.block_ids() {
        if !reachable_blocks.contains(&pred_id) {
            continue;
        }

        let pred_block = function.blocks.get(&pred_id)?;
        let MirInstruction::Jump {
            target: middle_id,
            edge_args: _,
        } = pred_block.terminator.as_ref()?
        else {
            continue;
        };

        if *middle_id == pred_id || *middle_id == function.entry_block {
            continue;
        }

        let middle_block = function.blocks.get(middle_id)?;
        if !reachable_blocks.contains(middle_id) {
            continue;
        }
        if middle_block.terminator.is_none() {
            continue;
        }
        if middle_block.predecessors.len() != 1 || !middle_block.predecessors.contains(&pred_id) {
            continue;
        }
        if collect_trivial_phi_rewrites(middle_block, pred_id).is_none() {
            continue;
        }
        if middle_block.successors.contains(&pred_id) {
            continue;
        }

        return Some((pred_id, *middle_id));
    }

    None
}

fn merge_single_predecessor_jump_block(
    function: &mut MirFunction,
    pred_id: BasicBlockId,
    middle_id: BasicBlockId,
) {
    let mut middle_block = function
        .blocks
        .remove(&middle_id)
        .expect("merge candidate middle block must exist");
    let phi_rewrites = collect_trivial_phi_rewrites(&middle_block, pred_id)
        .expect("merge candidate middle block must have only trivial single-input PHIs");

    for (phi_dst, incoming_value) in &phi_rewrites {
        rewrite_value_uses_in_function(function, *phi_dst, *incoming_value);
        rewrite_value_uses_in_block(&mut middle_block, *phi_dst, *incoming_value);
    }
    if !phi_rewrites.is_empty() {
        middle_block.instructions.drain(0..phi_rewrites.len());
        middle_block.instruction_spans.drain(0..phi_rewrites.len());
    }

    rewrite_phi_predecessor(function, middle_id, pred_id);

    let pred_block = function
        .blocks
        .get_mut(&pred_id)
        .expect("merge candidate predecessor block must exist");

    pred_block.instructions.extend(middle_block.instructions);
    pred_block
        .instruction_spans
        .extend(middle_block.instruction_spans);
    pred_block.terminator = middle_block.terminator;
    pred_block.terminator_span = middle_block.terminator_span;
    pred_block.return_env = middle_block.return_env;
    pred_block.return_env_layout = middle_block.return_env_layout;
    pred_block.successors = pred_block.successors_from_terminator();
    recompute_effects(pred_block);

    function.update_cfg();
}

fn collect_trivial_phi_rewrites(
    middle_block: &BasicBlock,
    pred_id: BasicBlockId,
) -> Option<Vec<(ValueId, ValueId)>> {
    let mut rewrites = Vec::new();
    for instruction in middle_block.phi_instructions() {
        let MirInstruction::Phi { dst, inputs, .. } = instruction else {
            unreachable!("phi_instructions() must yield only PHI instructions");
        };
        let [(incoming_block, incoming_value)] = inputs.as_slice() else {
            return None;
        };
        if *incoming_block != pred_id {
            return None;
        }
        rewrites.push((*dst, *incoming_value));
    }
    Some(rewrites)
}

fn rewrite_phi_predecessor(
    function: &mut MirFunction,
    old_predecessor: BasicBlockId,
    new_predecessor: BasicBlockId,
) {
    for block in function.blocks.values_mut() {
        for instruction in &mut block.instructions {
            let MirInstruction::Phi { inputs, .. } = instruction else {
                continue;
            };

            for (incoming_block, _) in inputs.iter_mut() {
                if *incoming_block == old_predecessor {
                    *incoming_block = new_predecessor;
                }
            }
        }
    }
}

fn rewrite_value_uses_in_function(function: &mut MirFunction, from: ValueId, to: ValueId) {
    for block in function.blocks.values_mut() {
        rewrite_value_uses_in_block(block, from, to);
    }
}

fn rewrite_value_uses_in_block(block: &mut BasicBlock, from: ValueId, to: ValueId) {
    for instruction in &mut block.instructions {
        rewrite_value_uses_in_instruction(instruction, from, to);
    }
    if let Some(terminator) = &mut block.terminator {
        rewrite_value_uses_in_instruction(terminator, from, to);
    }
    if let Some(return_env) = &mut block.return_env {
        for value in return_env {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_uses_in_instruction(instruction: &mut MirInstruction, from: ValueId, to: ValueId) {
    match instruction {
        MirInstruction::Const { .. } | MirInstruction::Catch { .. } | MirInstruction::Safepoint => {
        }
        MirInstruction::BinOp { lhs, rhs, .. } | MirInstruction::Compare { lhs, rhs, .. } => {
            rewrite_value_use(lhs, from, to);
            rewrite_value_use(rhs, from, to);
        }
        MirInstruction::UnaryOp { operand, .. }
        | MirInstruction::Load { ptr: operand, .. }
        | MirInstruction::FieldGet { base: operand, .. }
        | MirInstruction::VariantTag { value: operand, .. }
        | MirInstruction::VariantProject { value: operand, .. }
        | MirInstruction::TypeOp { value: operand, .. }
        | MirInstruction::Copy { src: operand, .. }
        | MirInstruction::Debug { value: operand, .. }
        | MirInstruction::Throw {
            exception: operand, ..
        }
        | MirInstruction::RefNew {
            box_val: operand, ..
        }
        | MirInstruction::WeakRef { value: operand, .. }
        | MirInstruction::Barrier { ptr: operand, .. }
        | MirInstruction::FutureNew { value: operand, .. }
        | MirInstruction::Await {
            future: operand, ..
        } => rewrite_value_use(operand, from, to),
        MirInstruction::Store { value, ptr } => {
            rewrite_value_use(value, from, to);
            rewrite_value_use(ptr, from, to);
        }
        MirInstruction::FieldSet { base, value, .. } => {
            rewrite_value_use(base, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::VariantMake { payload, .. } => {
            if let Some(payload) = payload {
                rewrite_value_use(payload, from, to);
            }
        }
        MirInstruction::Call {
            func, callee, args, ..
        } => {
            if callee.is_none() {
                rewrite_value_use(func, from, to);
            }
            if let Some(callee) = callee {
                match callee {
                    Callee::Method { receiver, .. } => {
                        if let Some(receiver) = receiver {
                            rewrite_value_use(receiver, from, to);
                        }
                    }
                    Callee::Closure {
                        captures,
                        me_capture,
                        ..
                    } => {
                        for (_, capture) in captures {
                            rewrite_value_use(capture, from, to);
                        }
                        if let Some(me_capture) = me_capture {
                            rewrite_value_use(me_capture, from, to);
                        }
                    }
                    Callee::Value(value) => rewrite_value_use(value, from, to),
                    Callee::Global(_) | Callee::Constructor { .. } | Callee::Extern(_) => {}
                }
            }
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::NewClosure { captures, me, .. } => {
            for (_, capture) in captures {
                rewrite_value_use(capture, from, to);
            }
            if let Some(me) = me {
                rewrite_value_use(me, from, to);
            }
        }
        MirInstruction::Branch {
            condition,
            then_edge_args,
            else_edge_args,
            ..
        } => {
            rewrite_value_use(condition, from, to);
            rewrite_edge_args_values(then_edge_args, from, to);
            rewrite_edge_args_values(else_edge_args, from, to);
        }
        MirInstruction::Jump { edge_args, .. } => {
            rewrite_edge_args_values(edge_args, from, to);
        }
        MirInstruction::Return { value } => {
            if let Some(value) = value {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::Phi { inputs, .. } => {
            for (_, incoming_value) in inputs {
                rewrite_value_use(incoming_value, from, to);
            }
        }
        MirInstruction::NewBox { args, .. } => {
            for arg in args {
                rewrite_value_use(arg, from, to);
            }
        }
        MirInstruction::KeepAlive { values } | MirInstruction::ReleaseStrong { values } => {
            for value in values {
                rewrite_value_use(value, from, to);
            }
        }
        MirInstruction::FutureSet { future, value } => {
            rewrite_value_use(future, from, to);
            rewrite_value_use(value, from, to);
        }
        MirInstruction::Select {
            cond,
            then_val,
            else_val,
            ..
        } => {
            rewrite_value_use(cond, from, to);
            rewrite_value_use(then_val, from, to);
            rewrite_value_use(else_val, from, to);
        }
    }
}

fn rewrite_edge_args_values(
    edge_args: &mut Option<crate::mir::EdgeArgs>,
    from: ValueId,
    to: ValueId,
) {
    if let Some(edge_args) = edge_args {
        for value in &mut edge_args.values {
            rewrite_value_use(value, from, to);
        }
    }
}

fn rewrite_value_use(value: &mut ValueId, from: ValueId, to: ValueId) {
    if *value == from {
        *value = to;
    }
}

fn recompute_effects(block: &mut BasicBlock) {
    let mut effects = EffectMask::PURE;
    for instruction in &block.instructions {
        effects = effects | instruction.effects();
    }
    if let Some(terminator) = &block.terminator {
        effects = effects | terminator.effects();
    }
    block.effects = effects;
}

#[cfg(test)]
#[path = "simplify_cfg_tests.rs"]
mod tests;

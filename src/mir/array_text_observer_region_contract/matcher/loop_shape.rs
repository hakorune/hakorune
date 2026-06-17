use super::{block_predecessors, block_terminal, const_i64, root};
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{BasicBlock, BasicBlockId, CompareOp, MirFunction, MirInstruction, ValueId};

pub(super) fn single_preheader_jump_to_header(
    function: &MirFunction,
    header_block: BasicBlockId,
    latch_block: BasicBlockId,
) -> Option<BasicBlockId> {
    let mut non_latch_predecessors = block_predecessors(function, header_block)
        .into_iter()
        .filter(|predecessor| *predecessor != latch_block);
    let preheader = non_latch_predecessors.next()?;
    if non_latch_predecessors.next().is_some() {
        return None;
    }
    let preheader_block = function.blocks.get(&preheader)?;
    match block_terminal(preheader_block)? {
        MirInstruction::Jump { target, .. } if *target == header_block => Some(preheader),
        _ => None,
    }
}

pub(super) fn match_loop_index_condition(
    function: &MirFunction,
    def_map: &ValueDefMap,
    header: &BasicBlock,
) -> Option<(ValueId, ValueId)> {
    let condition = match block_terminal(header)? {
        MirInstruction::Branch { condition, .. } => *condition,
        _ => return None,
    };
    let condition = root(function, def_map, condition);
    let compare = header.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Compare {
            dst,
            op: CompareOp::Lt,
            lhs,
            rhs,
        } if root(function, def_map, *dst) == condition => Some((*lhs, *rhs)),
        _ => None,
    })?;
    let loop_index_phi_value = root(function, def_map, compare.0);
    if !is_phi_dst(header, loop_index_phi_value) {
        return None;
    }
    Some((loop_index_phi_value, root(function, def_map, compare.1)))
}

fn is_phi_dst(block: &BasicBlock, value: ValueId) -> bool {
    block.instructions.iter().any(|inst| {
        matches!(
            inst,
            MirInstruction::Phi { dst, .. } if *dst == value
        )
    })
}

pub(super) fn phi_input_from(
    block: &BasicBlock,
    phi_value: ValueId,
    predecessor: BasicBlockId,
) -> Option<ValueId> {
    block.instructions.iter().find_map(|inst| match inst {
        MirInstruction::Phi { dst, inputs, .. } if *dst == phi_value => inputs
            .iter()
            .find_map(|(block, value)| (*block == predecessor).then_some(*value)),
        _ => None,
    })
}

pub(super) fn is_add_const_one_from(
    function: &MirFunction,
    def_map: &ValueDefMap,
    block: &BasicBlock,
    next_value: ValueId,
    source_value: ValueId,
) -> bool {
    let next_value = root(function, def_map, next_value);
    block.instructions.iter().any(|inst| match inst {
        MirInstruction::BinOp {
            dst,
            op: crate::mir::BinaryOp::Add,
            lhs,
            rhs,
        } if *dst == next_value => {
            (root(function, def_map, *lhs) == source_value
                && const_i64(function, def_map, *rhs) == Some(1))
                || (root(function, def_map, *rhs) == source_value
                    && const_i64(function, def_map, *lhs) == Some(1))
        }
        _ => false,
    })
}

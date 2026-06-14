use crate::mir::definitions::call_unified::Callee;
use crate::mir::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use crate::mir::verification::utils::compute_dominators;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

/// Rewrite Method receiver operands from Copy carriers to their dominating root.
///
/// This pass intentionally runs after callsite canonicalization has materialized
/// Method callees. LocalSSA emission does not own cross-block dominance; this
/// pass uses the final MIR CFG and keeps the rewrite receiver-only.
pub fn rewrite_cfg_stable_receiver_operands(function: &mut MirFunction) -> usize {
    if !cfg_successors_are_synced(function) {
        return 0;
    }

    let def_map = build_value_def_map(function);
    let dominators = compute_dominators(function);
    let mut edits: Vec<ReceiverEdit> = Vec::new();

    for (block_id, block) in &function.blocks {
        for (inst_index, inst) in block.instructions.iter().enumerate() {
            let Some(receiver) = method_receiver(inst) else {
                continue;
            };
            let Some(root) = copy_chain_root(function, &def_map, receiver) else {
                continue;
            };
            let Some((root_block, _)) = def_map.get(&root).copied() else {
                continue;
            };
            if root_block == *block_id {
                continue;
            }
            if !dominators.dominates(root_block, *block_id) {
                continue;
            }
            edits.push(ReceiverEdit {
                block_id: *block_id,
                inst_index,
                old_receiver: receiver,
                new_receiver: root,
            });
        }
    }

    let mut rewritten = 0usize;
    for edit in edits {
        let Some(block) = function.blocks.get_mut(&edit.block_id) else {
            continue;
        };
        let Some(inst) = block.instructions.get_mut(edit.inst_index) else {
            continue;
        };
        if rewrite_method_receiver(inst, edit.old_receiver, edit.new_receiver) {
            rewritten += 1;
        }
    }
    rewritten
}

#[derive(Debug, Clone, Copy)]
struct ReceiverEdit {
    block_id: BasicBlockId,
    inst_index: usize,
    old_receiver: ValueId,
    new_receiver: ValueId,
}

fn cfg_successors_are_synced(function: &MirFunction) -> bool {
    function
        .blocks
        .values()
        .all(|block| block.successors == block.successors_from_terminator())
}

fn method_receiver(inst: &MirInstruction) -> Option<ValueId> {
    let MirInstruction::Call {
        callee:
            Some(Callee::Method {
                receiver: Some(receiver),
                ..
            }),
        ..
    } = inst
    else {
        return None;
    };
    Some(*receiver)
}

fn copy_chain_root(
    function: &MirFunction,
    def_map: &ValueDefMap,
    receiver: ValueId,
) -> Option<ValueId> {
    if !is_copy_defined(function, def_map, receiver) {
        return None;
    }
    let root = resolve_value_origin(function, def_map, receiver);
    (root != receiver).then_some(root)
}

fn is_copy_defined(function: &MirFunction, def_map: &ValueDefMap, value: ValueId) -> bool {
    let Some((block_id, inst_index)) = def_map.get(&value).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&block_id) else {
        return false;
    };
    matches!(
        block.instructions.get(inst_index),
        Some(MirInstruction::Copy { .. })
    )
}

fn rewrite_method_receiver(
    inst: &mut MirInstruction,
    old_receiver: ValueId,
    new_receiver: ValueId,
) -> bool {
    let MirInstruction::Call {
        callee:
            Some(Callee::Method {
                receiver: Some(receiver),
                ..
            }),
        ..
    } = inst
    else {
        return false;
    };
    if *receiver != old_receiver {
        return false;
    }
    *receiver = new_receiver;
    true
}

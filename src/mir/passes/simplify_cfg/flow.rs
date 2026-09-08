//! CFG threading and merging; value substitution is delegated after admission.
use crate::mir::{BasicBlock, BasicBlockId, EffectMask, MirFunction, MirInstruction, ValueId};
use std::collections::HashSet;

#[path = "value_uses.rs"]
mod value_uses;
use value_uses::{rewrite_value_uses_in_block, rewrite_value_uses_in_function};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ThreadArm {
    Then,
    Else,
}

fn preserve_emit_mir_string_phi_trampoline(
    function: &MirFunction,
    middle_block: &BasicBlock,
) -> bool {
    if !crate::config::env::stage1::emit_mir_json() {
        return false;
    }

    middle_block.phi_instructions().any(|instruction| {
        let MirInstruction::Phi { dst, type_hint, .. } = instruction else {
            return false;
        };
        match type_hint
            .as_ref()
            .or_else(|| function.metadata.value_types.get(dst))
        {
            Some(crate::mir::MirType::String) => true,
            Some(crate::mir::MirType::Box(box_name)) => {
                matches!(box_name.as_str(), "StringBox" | "RuntimeDataBox")
            }
            _ => false,
        }
    })
}

pub(super) fn find_threadable_branch_jump(
    function: &MirFunction,
    reachable_blocks: &HashSet<BasicBlockId>,
) -> Option<(
    BasicBlockId,
    ThreadArm,
    BasicBlockId,
    BasicBlockId,
    bool,
    bool,
)> {
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

        if let Some((target, rewrite_phi, clear_edge_args)) =
            threadable_jump_target(function, block_id, *then_bb, then_edge_args.as_ref())
        {
            if target != *else_bb {
                return Some((
                    block_id,
                    ThreadArm::Then,
                    *then_bb,
                    target,
                    rewrite_phi,
                    clear_edge_args,
                ));
            }
            let effective_then_edge_args = if clear_edge_args {
                None
            } else {
                then_edge_args.clone()
            };
            if !rewrite_phi && effective_then_edge_args == *else_edge_args {
                return Some((
                    block_id,
                    ThreadArm::Then,
                    *then_bb,
                    target,
                    rewrite_phi,
                    clear_edge_args,
                ));
            }
        }

        if let Some((target, rewrite_phi, clear_edge_args)) =
            threadable_jump_target(function, block_id, *else_bb, else_edge_args.as_ref())
        {
            if target != *then_bb {
                return Some((
                    block_id,
                    ThreadArm::Else,
                    *else_bb,
                    target,
                    rewrite_phi,
                    clear_edge_args,
                ));
            }
            let effective_else_edge_args = if clear_edge_args {
                None
            } else {
                else_edge_args.clone()
            };
            if !rewrite_phi && *then_edge_args == effective_else_edge_args {
                return Some((
                    block_id,
                    ThreadArm::Else,
                    *else_bb,
                    target,
                    rewrite_phi,
                    clear_edge_args,
                ));
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
) -> Option<(BasicBlockId, bool, bool)> {
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
        if edge_args.is_some() {
            return None;
        }
        if middle_block.predecessors.len() != 1 || !middle_block.predecessors.contains(&pred_id) {
            return None;
        }
        if !can_rewrite_threaded_phi_predecessor(final_block, middle_id, pred_id) {
            return None;
        }
        return Some((*target, true, false));
    }

    Some((*target, false, edge_args.is_some()))
}

pub(super) fn thread_branch_arm(
    function: &mut MirFunction,
    block_id: BasicBlockId,
    arm: ThreadArm,
    middle_id: BasicBlockId,
    target: BasicBlockId,
    rewrite_phi: bool,
    clear_edge_args: bool,
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
                if clear_edge_args {
                    *then_edge_args = None;
                }
            }
            ThreadArm::Else => {
                *else_bb = target;
                if clear_edge_args {
                    *else_edge_args = None;
                }
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

pub(super) fn find_single_predecessor_jump_merge(
    function: &MirFunction,
    reachable_blocks: &HashSet<BasicBlockId>,
) -> Option<(BasicBlockId, BasicBlockId)> {
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
        if preserve_emit_mir_string_phi_trampoline(function, middle_block) {
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

pub(super) fn merge_single_predecessor_jump_block(
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
    // The Invoke moves with this block's terminator; its result projection
    // must keep the exact origin rather than the removed block identity.
    for block in function.blocks.values_mut() {
        for inst in &mut block.instructions {
            if let MirInstruction::InvokeNormalResult { invoke_block, .. } = inst {
                if *invoke_block == middle_id {
                    *invoke_block = pred_id;
                }
            }
        }
    }

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

//! Terminator remapping utilities
//!
//! Phase 260 P0.1 Step 5: Extracted from instruction_rewriter.rs
//! Handles Jump/Branch block ID and ValueId remapping.
//! Phase 284 P1: Use block_remapper SSOT for block ID remapping

use super::super::block_remapper::remap_block_id; // Phase 284 P1: SSOT
use crate::mir::builder::joinir_id_remapper::JoinIrIdRemapper;
use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::collections::BTreeMap;

/// Remap edge_args ValueIds using remapper
///
/// Helper function used by Jump/Branch remapping to remap edge argument values
/// from JoinIR value space to HOST value space.
fn remap_edge_args(
    remapper: &JoinIrIdRemapper,
    edge_args: &Option<crate::mir::EdgeArgs>,
) -> Option<crate::mir::EdgeArgs> {
    edge_args.as_ref().map(|args| crate::mir::EdgeArgs {
        layout: args.layout,
        values: args
            .values
            .iter()
            .map(|&v| remapper.remap_value(v))
            .collect(),
    })
}

/// Remap Jump instruction
///
/// Applies block ID remapping (local_block_map + skipped_entry_redirects) and
/// edge_args ValueId remapping.
///
/// # Phase 284 P1: Use block_remapper SSOT
///
/// Delegates block remapping to block_remapper::remap_block_id for consistent priority rule.
pub(in crate::mir::builder::control_flow::joinir::merge) fn remap_jump(
    remapper: &JoinIrIdRemapper,
    target: BasicBlockId,
    edge_args: &Option<crate::mir::EdgeArgs>,
    skipped_entry_redirects: &BTreeMap<BasicBlockId, BasicBlockId>,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
) -> MirInstruction {
    // Phase 284 P1: Use SSOT for block remapping
    let remapped_target = remap_block_id(target, local_block_map, skipped_entry_redirects);

    MirInstruction::Jump {
        target: remapped_target,
        edge_args: remap_edge_args(remapper, edge_args),
    }
}

/// Remap Branch instruction
///
/// Applies block ID remapping (local_block_map + skipped_entry_redirects) for
/// both then/else branches, condition ValueId remapping, and edge_args remapping.
///
/// # Phase 284 P1: Use block_remapper SSOT
///
/// Delegates block remapping to block_remapper::remap_block_id for consistent priority rule.
pub(in crate::mir::builder::control_flow::joinir::merge) fn remap_branch(
    remapper: &JoinIrIdRemapper,
    condition: ValueId,
    then_bb: BasicBlockId,
    else_bb: BasicBlockId,
    then_edge_args: &Option<crate::mir::EdgeArgs>,
    else_edge_args: &Option<crate::mir::EdgeArgs>,
    skipped_entry_redirects: &BTreeMap<BasicBlockId, BasicBlockId>,
    local_block_map: &BTreeMap<BasicBlockId, BasicBlockId>,
) -> MirInstruction {
    // Phase 284 P1: Use SSOT for block remapping
    let remapped_then = remap_block_id(then_bb, local_block_map, skipped_entry_redirects);
    let remapped_else = remap_block_id(else_bb, local_block_map, skipped_entry_redirects);

    MirInstruction::Branch {
        condition: remapper.remap_value(condition),
        then_bb: remapped_then,
        else_bb: remapped_else,
        then_edge_args: remap_edge_args(remapper, then_edge_args),
        else_edge_args: remap_edge_args(remapper, else_edge_args),
    }
}

/// Apply remapped terminator to block
///
/// Handles special cases for Jump/Branch (set_jump_with_edge_args /
/// set_branch_with_edge_args) vs other terminators (set_terminator).
///
/// # Why Special Handling?
///
/// Jump/Branch with edge_args need special setters to maintain invariants
/// (successors, predecessors, jump_args synchronization).
pub(in crate::mir::builder::control_flow::joinir::merge) fn apply_remapped_terminator(
    block: &mut crate::mir::BasicBlock,
    term: MirInstruction,
) {
    match term {
        MirInstruction::Jump { target, edge_args } => {
            if edge_args.is_some() {
                block.set_jump_with_edge_args(target, edge_args);
            } else {
                block.set_terminator(MirInstruction::Jump {
                    target,
                    edge_args: None,
                });
            }
        }
        MirInstruction::Branch {
            condition,
            then_bb,
            then_edge_args,
            else_bb,
            else_edge_args,
        } => {
            block.set_branch_with_edge_args(
                condition,
                then_bb,
                then_edge_args,
                else_bb,
                else_edge_args,
            );
        }
        _ => block.set_terminator(term),
    }
}

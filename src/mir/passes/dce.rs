//! Dead Code Elimination (pure instruction DCE)
//!
//! Extracted from the monolithic optimizer to enable modular pass composition.

#[path = "dce/elimination.rs"]
mod elimination;
#[path = "dce/local_fields.rs"]
mod local_fields;
#[cfg(test)]
#[path = "dce/tests/mod.rs"]
mod tests;

use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::HashSet;

/// Eliminate dead code (unused results of pure instructions) across the module
/// and prune unreachable blocks as structural CFG cleanup.
///
/// This pass also removes pure no-dst calls plus dead field reads and writes on
/// definitely non-escaping local boxes when they are otherwise unused.
///
/// Returns the number of eliminated instructions.
pub fn eliminate_dead_code(module: &mut MirModule) -> usize {
    let mut eliminated_total = 0usize;
    for (_func_name, func) in &mut module.functions {
        eliminated_total += elimination::eliminate_dead_code_in_function(func);
    }
    eliminated_total
}

fn propagate_used_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    used_values: &mut HashSet<ValueId>,
) {
    let mut changed = true;
    while changed {
        changed = false;
        for (bid, block) in &function.blocks {
            if !reachable_blocks.contains(bid) {
                continue;
            }
            for instruction in &block.instructions {
                if let Some(dst) = instruction.dst_value() {
                    if used_values.contains(&dst) {
                        for u in instruction.used_values() {
                            if used_values.insert(u) {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn is_removable_no_dst_pure_instruction(inst: &crate::mir::MirInstruction) -> bool {
    matches!(
        inst,
        crate::mir::MirInstruction::Safepoint | crate::mir::MirInstruction::Call { dst: None, .. }
    )
}

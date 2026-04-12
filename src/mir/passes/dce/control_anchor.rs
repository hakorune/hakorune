use crate::mir::{MirFunction, ValueId};
use std::collections::HashSet;

pub(super) fn seed_control_anchor_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    base_used_values: &mut HashSet<ValueId>,
) {
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        // Branch/Jump/Return are routed into `block.terminator` by BasicBlock and
        // should not rely on legacy instruction-list seeding here.
        if let Some(term) = &block.terminator {
            for u in term.used_values() {
                base_used_values.insert(u);
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for u in args.values {
                    base_used_values.insert(u);
                }
            }
        }
    }
}

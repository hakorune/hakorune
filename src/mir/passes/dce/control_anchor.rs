use crate::mir::MirFunction;
use std::collections::HashSet;

pub(in crate::mir::passes) fn seed_control_anchor_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    mut insert: impl FnMut(crate::mir::ValueId),
) {
    for (bid, block) in &function.blocks {
        // Captured metadata users remain obligations even on unreachable blocks.
        if let Some(values) = &block.return_env {
            for value in values {
                insert(*value);
            }
        }
        if !reachable_blocks.contains(bid) {
            continue;
        }
        // The Normal projection is the structural definition of an Invoke's
        // result, even when its value has no remaining ordinary consumers.
        for inst in &block.instructions {
            if let crate::mir::MirInstruction::InvokeNormalResult { dst, .. } = inst {
                insert(*dst);
            }
        }
        // Branch/Jump/Return are routed into `block.terminator` by BasicBlock and
        // should not rely on legacy instruction-list seeding here.
        if let Some(term) = &block.terminator {
            for u in term.used_values() {
                insert(u);
            }
        }
        for edge in block.out_edges() {
            if let Some(args) = edge.args {
                for u in args.values {
                    insert(u);
                }
            }
        }
    }
}

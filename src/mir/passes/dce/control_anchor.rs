use super::LiveValueSet;
use crate::mir::MirFunction;
use std::collections::HashSet;

pub(super) fn seed_control_anchor_values(
    function: &MirFunction,
    reachable_blocks: &HashSet<crate::mir::BasicBlockId>,
    base_used_values: &mut LiveValueSet,
) {
    for (bid, block) in &function.blocks {
        if !reachable_blocks.contains(bid) {
            continue;
        }
        // The Normal projection is the structural definition of an Invoke's
        // result, even when its value has no remaining ordinary consumers.
        for inst in &block.instructions {
            if let crate::mir::MirInstruction::InvokeNormalResult { dst, .. } = inst {
                base_used_values.insert(*dst);
            }
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

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::{BasicBlockId, Callee, EffectMask, MirFunction, MirInstruction, ValueId};

use super::planner::Concat3Plan;
use super::CONCAT3_EXTERN;

pub(super) fn apply_plans(
    function: &mut MirFunction,
    plans_by_block: BTreeMap<BasicBlockId, Vec<Concat3Plan>>,
) -> usize {
    let mut rewritten = 0usize;

    for (bbid, plans) in plans_by_block {
        if plans.is_empty() {
            continue;
        }
        let Some(block) = function.blocks.get_mut(&bbid) else {
            continue;
        };

        let mut remove_indices: BTreeSet<usize> = BTreeSet::new();
        let mut replacements: BTreeMap<usize, MirInstruction> = BTreeMap::new();
        let mut removed_values: Vec<ValueId> = Vec::new();

        for plan in plans {
            remove_indices.insert(plan.inner_idx);
            removed_values.push(plan.inner_dst);
            replacements.insert(
                plan.outer_idx,
                MirInstruction::Call {
                    dst: Some(plan.outer_dst),
                    func: ValueId::INVALID,
                    callee: Some(Callee::Extern(CONCAT3_EXTERN.to_string())),
                    args: plan.args.to_vec(),
                    effects: EffectMask::PURE,
                },
            );
            rewritten += 1;
        }

        let insts = std::mem::take(&mut block.instructions);
        let spans = std::mem::take(&mut block.instruction_spans);
        let mut new_insts = Vec::with_capacity(insts.len());
        let mut new_spans = Vec::with_capacity(spans.len());

        for (idx, (inst, span)) in insts.into_iter().zip(spans.into_iter()).enumerate() {
            if remove_indices.contains(&idx) {
                continue;
            }
            if let Some(rewritten_inst) = replacements.get(&idx) {
                new_insts.push(rewritten_inst.clone());
                new_spans.push(span);
            } else {
                new_insts.push(inst);
                new_spans.push(span);
            }
        }

        block.instructions = new_insts;
        block.instruction_spans = new_spans;

        for vid in removed_values {
            function.metadata.value_types.remove(&vid);
        }
    }

    rewritten
}

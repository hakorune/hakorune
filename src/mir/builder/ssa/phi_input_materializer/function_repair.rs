//! Legacy whole-function PHI repair.
//!
//! This module preserves the existing post-Lower repair behavior. It is not
//! the completion mechanism for the future canonical Binding SSA owner.

use super::edge_rematerialization::{
    rematerialize_for_pred, PhiInputMaterializationAnalysis, PhiInputRematContext,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::collections::{HashMap, HashSet};

pub(in crate::mir::builder) fn materialize_all_phi_inputs(
    func: &mut MirFunction,
    context: &str,
) -> Result<usize, String> {
    let mut changed = prune_unused_phi_instructions(func);
    changed += complete_missing_self_carried_phi_inputs(func);
    let mut work = Vec::new();
    for (block_id, block) in &func.blocks {
        for (inst_idx, inst) in block.instructions.iter().enumerate() {
            if let MirInstruction::Phi { inputs, .. } = inst {
                for (input_idx, (pred, incoming)) in inputs.iter().enumerate() {
                    work.push((*block_id, inst_idx, input_idx, *pred, *incoming));
                }
            }
        }
    }

    let analysis = PhiInputMaterializationAnalysis::new(func);
    let mut remat_contexts: HashMap<BasicBlockId, PhiInputRematContext> = HashMap::new();
    for (block_id, inst_idx, input_idx, pred, incoming) in work {
        let remat_ctx = remat_contexts
            .entry(pred)
            .or_insert_with(|| PhiInputRematContext::new(pred));
        let materialized =
            rematerialize_for_pred(func, &analysis, incoming, context, "phi", remat_ctx)?;
        if materialized == incoming {
            continue;
        }
        let fn_name = func.signature.name.clone();
        let block = func.get_block_mut(block_id).ok_or_else(|| {
            format!(
                "[freeze:contract][ssa/phi_input/missing_phi_block] fn={} block={:?} context={}",
                fn_name, block_id, context
            )
        })?;
        let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get_mut(inst_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_inst] fn={} block={:?} inst_idx={} context={}",
                fn_name, block_id, inst_idx, context
            ));
        };
        let Some((_, slot)) = inputs.get_mut(input_idx) else {
            return Err(format!(
                "[freeze:contract][ssa/phi_input/missing_phi_input] fn={} block={:?} inst_idx={} input_idx={} context={}",
                fn_name, block_id, inst_idx, input_idx, context
            ));
        };
        *slot = materialized;
        changed += 1;
    }

    Ok(changed)
}

fn prune_unused_phi_instructions(func: &mut MirFunction) -> usize {
    let mut used = HashSet::new();
    for block in func.blocks.values() {
        for inst in block.all_instructions() {
            for value in inst.used_values() {
                used.insert(value);
            }
        }
    }

    let mut changed = 0usize;
    for block in func.blocks.values_mut() {
        let mut remove_indices = Vec::new();
        for (idx, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::Phi { dst, .. } = inst else {
                continue;
            };
            if !used.contains(dst) {
                remove_indices.push(idx);
            }
        }

        for idx in remove_indices.into_iter().rev() {
            block.instructions.remove(idx);
            if idx < block.instruction_spans.len() {
                block.instruction_spans.remove(idx);
            }
            changed += 1;
        }
    }
    changed
}

pub(super) fn complete_missing_self_carried_phi_inputs(func: &mut MirFunction) -> usize {
    func.update_cfg();
    let preds = crate::mir::verification::utils::compute_predecessors(func);
    let reachable = crate::mir::verification::utils::compute_reachable_blocks(func);
    let def_blocks = crate::mir::verification::utils::compute_def_blocks(func);
    let dominators = crate::mir::verification::utils::compute_dominators(func);

    let mut additions = Vec::new();
    for (block_id, block) in &func.blocks {
        if !reachable.contains(block_id) {
            continue;
        }
        let Some(expected_preds) = preds.get(block_id) else {
            continue;
        };

        for (inst_idx, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::Phi { dst, inputs, .. } = inst else {
                continue;
            };
            let input_preds: HashSet<BasicBlockId> = inputs.iter().map(|(pred, _)| *pred).collect();

            for pred in expected_preds {
                if !reachable.contains(pred) || input_preds.contains(pred) {
                    continue;
                }
                // A missing input can be completed as "unchanged on this edge"
                // only when the PHI definition block dominates that predecessor.
                // This covers loop-invariant / unchanged-carrier backedges while
                // avoiding fabricated values for unrelated merge predecessors.
                if dominators.dominates(*block_id, *pred) {
                    additions.push((*block_id, inst_idx, *pred, *dst));
                    continue;
                }

                let mut dominating_inputs = inputs
                    .iter()
                    .filter_map(|(_, incoming)| {
                        let def_bb = def_blocks.get(incoming).copied()?;
                        dominators.dominates(def_bb, *pred).then_some(*incoming)
                    })
                    .collect::<Vec<_>>();
                dominating_inputs.sort_by_key(|value| value.0);
                dominating_inputs.dedup();
                if dominating_inputs.len() == 1 {
                    additions.push((*block_id, inst_idx, *pred, dominating_inputs[0]));
                }
            }
        }
    }

    let changed = additions.len();
    for (block_id, inst_idx, pred, dst) in additions {
        let Some(block) = func.get_block_mut(block_id) else {
            continue;
        };
        let Some(MirInstruction::Phi { inputs, .. }) = block.instructions.get_mut(inst_idx) else {
            continue;
        };
        if !inputs
            .iter()
            .any(|(existing_pred, _)| *existing_pred == pred)
        {
            inputs.push((pred, dst));
        }
    }
    changed
}

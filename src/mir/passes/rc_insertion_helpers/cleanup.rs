#[cfg(feature = "rc-insertion-minimal")]
use std::collections::HashMap;

#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

#[cfg(feature = "rc-insertion-minimal")]
use super::util::sorted_release_values;

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn collect_break_cleanup_values(
    func: &MirFunction,
    predecessors: &HashMap<BasicBlockId, Vec<BasicBlockId>>,
    end_states: &HashMap<BasicBlockId, HashMap<ValueId, ValueId>>,
    join_states: &HashMap<BasicBlockId, HashMap<ValueId, ValueId>>,
) -> HashMap<BasicBlockId, Vec<ValueId>> {
    let mut by_pred_block: HashMap<BasicBlockId, Vec<ValueId>> = HashMap::new();

    for (ret_bid, ret_block) in &func.blocks {
        if !matches!(
            ret_block.terminator.as_ref(),
            Some(MirInstruction::Return { .. })
        ) {
            continue;
        }
        // Safety: keep break cleanup limited to immediate-return exits only.
        if !ret_block.instructions.is_empty() {
            continue;
        }

        let preds = predecessors.get(ret_bid).cloned().unwrap_or_default();
        if preds.len() < 2 {
            continue;
        }

        let empty_join: HashMap<ValueId, ValueId> = HashMap::new();
        let join_state = join_states.get(ret_bid).unwrap_or(&empty_join);

        for pred_bid in preds {
            let Some(pred_block) = func.blocks.get(&pred_bid) else {
                continue;
            };
            let Some(MirInstruction::Jump { target, .. }) = pred_block.terminator.as_ref() else {
                continue;
            };
            if target != ret_bid {
                continue;
            }
            // Require an explicit break-dispatch style block:
            // - no body instructions
            // - has an incoming edge (avoid treating entry-like blocks as break paths)
            if !pred_block.instructions.is_empty() {
                continue;
            }
            if predecessors.get(&pred_bid).map_or(0, Vec::len) == 0 {
                continue;
            }

            let Some(pred_end_state) = end_states.get(&pred_bid) else {
                continue;
            };
            let mut edge_values: Vec<ValueId> = pred_end_state
                .iter()
                .filter_map(|(ptr, val)| {
                    if join_state.get(ptr) == Some(val) {
                        None
                    } else {
                        Some(*val)
                    }
                })
                .collect();
            edge_values = sorted_release_values(edge_values);
            if edge_values.is_empty() {
                continue;
            }
            by_pred_block
                .entry(pred_bid)
                .or_default()
                .extend(edge_values);
        }
    }

    for values in by_pred_block.values_mut() {
        *values = sorted_release_values(std::mem::take(values));
    }

    by_pred_block
}

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn collect_continue_cleanup_values(
    func: &MirFunction,
    predecessors: &HashMap<BasicBlockId, Vec<BasicBlockId>>,
    end_states: &HashMap<BasicBlockId, HashMap<ValueId, ValueId>>,
) -> HashMap<BasicBlockId, Vec<ValueId>> {
    let mut by_pred_block: HashMap<BasicBlockId, Vec<ValueId>> = HashMap::new();

    for (target_bid, target_block) in &func.blocks {
        // Continue target is loop-header-like Branch block in this minimal contract.
        if !matches!(
            target_block.terminator.as_ref(),
            Some(MirInstruction::Branch { .. })
        ) {
            continue;
        }
        let preds = predecessors.get(target_bid).cloned().unwrap_or_default();
        if preds.len() < 2 {
            continue;
        }

        let mut pred_end_states: Vec<&HashMap<ValueId, ValueId>> = Vec::new();
        for pred_bid in &preds {
            if let Some(st) = end_states.get(pred_bid) {
                pred_end_states.push(st);
            }
        }
        if pred_end_states.len() != preds.len() || pred_end_states.is_empty() {
            continue;
        }

        let mut join_state: HashMap<ValueId, ValueId> =
            pred_end_states[0].iter().map(|(k, v)| (*k, *v)).collect();
        for other_state in pred_end_states.iter().skip(1) {
            join_state.retain(|ptr, val| other_state.get(ptr) == Some(val));
        }

        for pred_bid in preds {
            let Some(pred_block) = func.blocks.get(&pred_bid) else {
                continue;
            };
            let Some(MirInstruction::Jump { target, .. }) = pred_block.terminator.as_ref() else {
                continue;
            };
            if target != target_bid {
                continue;
            }
            // Continue-dispatch style block:
            // - no body instructions
            // - has an incoming edge (avoid entry/preheader-only jump)
            if !pred_block.instructions.is_empty() {
                continue;
            }
            if predecessors.get(&pred_bid).map_or(0, Vec::len) == 0 {
                continue;
            }

            let Some(pred_end_state) = end_states.get(&pred_bid) else {
                continue;
            };
            let mut edge_values: Vec<ValueId> = pred_end_state
                .iter()
                .filter_map(|(ptr, val)| {
                    if join_state.get(ptr) == Some(val) {
                        None
                    } else {
                        Some(*val)
                    }
                })
                .collect();
            edge_values = sorted_release_values(edge_values);
            if edge_values.is_empty() {
                continue;
            }
            by_pred_block
                .entry(pred_bid)
                .or_default()
                .extend(edge_values);
        }
    }

    for values in by_pred_block.values_mut() {
        *values = sorted_release_values(std::mem::take(values));
    }

    by_pred_block
}

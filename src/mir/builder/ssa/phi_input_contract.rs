use crate::mir::verification::utils::{
    compute_def_blocks, compute_dominators, compute_predecessors,
};
use crate::mir::{BasicBlockId, MirFunction, ValueId};
use std::collections::HashSet;

pub(crate) fn check_phi_input_contract(
    func: &MirFunction,
    use_bb: BasicBlockId,
    kind_label: &'static str,
    value: ValueId,
    phi_def_bb: BasicBlockId,
    phi_inputs: &[(BasicBlockId, ValueId)],
    fn_name: &str,
) -> Result<(), String> {
    let mut pred_list = compute_predecessors(func)
        .get(&phi_def_bb)
        .cloned()
        .unwrap_or_default();
    pred_list.sort_by_key(|b| b.0);
    let pred_set: HashSet<BasicBlockId> = pred_list.iter().copied().collect();

    // Skip phi input contract check if CFG is not yet wired
    // This avoids false positives during intermediate states where edge CFG hasn't been emitted yet.
    if pred_list.is_empty() {
        // Debug-only trace for skipped checks
        if crate::config::env::joinir_dev::debug_enabled() {
            let (phi0_pred, phi0_in) = if let Some((pred, incoming)) = phi_inputs.first() {
                (pred.0, incoming.0)
            } else {
                (crate::mir::BasicBlockId(0).0, ValueId(0).0)
            };
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("[phi_input_contract:skip_unwired] fn={} use_bb={:?} v=%{} phi_def_bb={:?} inputs={} phi0_pred={:?} phi0_in=%{}",
                fn_name, use_bb, value.0, phi_def_bb, phi_inputs.len(), phi0_pred, phi0_in
            ));
        }
        return Ok(());
    }

    // Skip phi input contract check for provisional PHIs (not yet patched, block not sealed)
    // This avoids false positives during intermediate states where:
    // - PHI inputs haven't been populated yet (empty)
    // - Block is still open to new predecessors (!sealed)
    // Once sealed, empty inputs are a real error (forgot to patch).
    if phi_inputs.is_empty() {
        if let Some(block) = func.blocks.get(&phi_def_bb) {
            if !block.is_sealed() {
                if crate::config::env::joinir_dev::debug_enabled() {
                    let ring0 = crate::runtime::get_global_ring0();
                    ring0.log.debug(&format!("[phi_input_contract:skip_provisional] fn={} use_bb={:?} v=%{} phi_def_bb={:?} preds={} inputs=0 sealed=false",
                        fn_name, use_bb, value.0, phi_def_bb, pred_list.len()
                    ));
                }
                return Ok(());
            }
        }
    }

    let mut inputs_sorted = phi_inputs.to_vec();
    inputs_sorted.sort_by_key(|(pred, _)| pred.0);
    let mut input_pred_set = HashSet::new();
    for (pred, _) in &inputs_sorted {
        input_pred_set.insert(*pred);
    }

    for (pred_bb, incoming) in &inputs_sorted {
        if !pred_set.contains(pred_bb) {
            return Err(format!(
                "[freeze:contract][local_ssa/non_dominating_use] fn={} bb={:?} kind={} v=%{} phi_def_bb={:?} reason=phantom_pred bad_pred={:?} bad_in=%{} in_def_bb=None",
                fn_name, use_bb, kind_label, value.0, phi_def_bb, pred_bb, incoming.0
            ));
        }
    }

    for pred_bb in &pred_list {
        if !input_pred_set.contains(pred_bb) {
            return Err(format!(
                "[freeze:contract][local_ssa/non_dominating_use] fn={} bb={:?} kind={} v=%{} phi_def_bb={:?} reason=missing_input bad_pred={:?} bad_in=none in_def_bb=None",
                fn_name, use_bb, kind_label, value.0, phi_def_bb, pred_bb
            ));
        }
    }

    let def_blocks = compute_def_blocks(func);
    let dominators = compute_dominators(func);

    for (pred_bb, incoming) in &inputs_sorted {
        if let Some(in_def_bb) = def_blocks.get(incoming) {
            let dominates = dominators.dominates(*in_def_bb, *pred_bb);
            if !dominates {
                return Err(format!(
                    "[freeze:contract][local_ssa/non_dominating_use] fn={} bb={:?} kind={} v=%{} phi_def_bb={:?} reason=incoming_not_dominating bad_pred={:?} bad_in=%{} in_def_bb={:?}",
                    fn_name, use_bb, kind_label, value.0, phi_def_bb, pred_bb, incoming.0, in_def_bb
                ));
            }
        } else {
            return Err(format!(
                "[freeze:contract][local_ssa/non_dominating_use] fn={} bb={:?} kind={} v=%{} phi_def_bb={:?} reason=incoming_not_dominating bad_pred={:?} bad_in=%{} in_def_bb=None",
                fn_name, use_bb, kind_label, value.0, phi_def_bb, pred_bb, incoming.0
            ));
        }
    }

    Ok(())
}

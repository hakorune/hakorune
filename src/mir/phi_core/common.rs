/*!
 * phi_core::common – shared types and invariants (scaffold)
 *
 * Phase 1 keeps this minimal; future phases may move debug asserts and
 * predicate set checks here for both if/loop PHI normalization.
 */

/// Placeholder for future shared PHI input type alias.
/// Using the same tuple form as MIR Phi instruction inputs.
pub type PhiInput = (crate::mir::BasicBlockId, crate::mir::ValueId);

pub fn debug_verify_phi_inputs(
    function: &crate::mir::MirFunction,
    merge_bb: crate::mir::BasicBlockId,
    inputs: &[(crate::mir::BasicBlockId, crate::mir::ValueId)],
) {
    use std::collections::HashSet;
    // Always compute when env toggle is set; otherwise no-op in release use.
    let verify_on = std::env::var("HAKO_PHI_VERIFY")
        .ok()
        .map(|v| v.to_ascii_lowercase())
        .map(|v| v == "1" || v == "true" || v == "on")
        .unwrap_or(false)
        || std::env::var("NYASH_PHI_VERIFY")
            .ok()
            .map(|v| v.to_ascii_lowercase())
            .map(|v| v == "1" || v == "true" || v == "on")
            .unwrap_or(false);
    if !verify_on {
        return;
    }

    // Rebuild CFG to avoid stale predecessor sets
    let mut func = function.clone();
    func.update_cfg();

    // Duplicate check
    let mut seen = HashSet::new();
    for (pred, _v) in inputs.iter() {
        if *pred == merge_bb {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[phi/check][bad-self] merge={:?} pred={:?}",
                merge_bb, pred
            ));
        }
        if !seen.insert(*pred) {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "[phi/check][dup] merge={:?} pred={:?}",
                merge_bb, pred
            ));
        }
    }

    // Missing predecessor inputs check
    if let Some(block) = func.blocks.get(&merge_bb) {
        for pred in &block.predecessors {
            let has = inputs.iter().any(|(bb, _)| bb == pred);
            if !has {
                crate::runtime::get_global_ring0().log.debug(&format!(
                    "[phi/check][missing] merge={:?} pred={:?}",
                    merge_bb, pred
                ));
            }
        }
    }
}

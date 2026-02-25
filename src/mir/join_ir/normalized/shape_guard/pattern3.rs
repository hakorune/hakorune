//! Pattern 3 shape detectors (if-sum patterns with conditional carrier updates)

use super::utils::{find_loop_step, name_guard_exact};
use crate::mir::join_ir::{JoinInst, JoinModule};

/// Phase 47-A: Check if module matches Pattern3 if-sum minimal shape
pub(crate) fn is_pattern3_if_sum_minimal(module: &JoinModule) -> bool {
    // Structure-based detection (avoid name-based heuristics)

    // Must have exactly 3 functions: main, loop_step, k_exit
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }

    // Find loop_step function
    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // P3 characteristics:
    // - Has Compare instruction (loop condition)
    // - Has Select instruction (conditional carrier update: if-then-else)
    // - Has tail call (Call with k_next: None)

    let has_compare = loop_step.body.iter().any(|inst| {
        matches!(
            inst,
            JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Compare { .. })
        )
    });

    // Phase 220: Select can be either JoinInst::Select or Compute(MirLikeInst::Select)
    let has_select = loop_step.body.iter().any(|inst| match inst {
        JoinInst::Select { .. } => true,
        JoinInst::Compute(mir_inst) => {
            matches!(mir_inst, crate::mir::join_ir::MirLikeInst::Select { .. })
        }
        _ => false,
    });

    let has_tail_call = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }));

    // P3 minimal/multi/json: typically 2-6 params (i + carriers + len/host)
    let reasonable_param_count = (2..=6).contains(&loop_step.params.len());

    has_compare && has_select && has_tail_call && reasonable_param_count
}

/// Phase 47-B: P3 if-sum (multi-carrier) shape detector
pub(crate) fn is_pattern3_if_sum_multi(module: &JoinModule) -> bool {
    if !is_pattern3_if_sum_minimal(module) {
        return false;
    }
    name_guard_exact(module, "pattern3_if_sum_multi_min")
}

/// Phase 47-B: P3 if-sum (JsonParser mini) shape detector
pub(crate) fn is_pattern3_if_sum_json(module: &JoinModule) -> bool {
    if !is_pattern3_if_sum_minimal(module) {
        return false;
    }
    name_guard_exact(module, "jsonparser_if_sum_min")
}

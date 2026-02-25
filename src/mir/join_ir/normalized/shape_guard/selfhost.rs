//! Selfhost-specific shape detectors (P2/P3 selfhost patterns)

use super::utils::{count_compare_ops, find_loop_step, name_guard_exact};
use crate::mir::join_ir::{CompareOp, JoinInst, JoinModule};

/// Phase 52: Selfhost P2 core family structure signature (dev-only).
///
/// This is intentionally narrow to avoid swallowing generic P2 shapes:
/// - loop_step params: 3..=4 (i + host + 1..2 carriers)
/// - P2 break-loop skeleton (cond jump + tail call)
/// - no Select / BoxCall in body
pub(super) fn is_selfhost_p2_core_family_candidate(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(3..=4).contains(&loop_func.params.len()) {
        return false;
    }

    let has_cond_jump = loop_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }));
    let has_tail_call = loop_func
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }));

    let has_select = loop_func.body.iter().any(|inst| match inst {
        JoinInst::Select { .. } => true,
        JoinInst::Compute(mir_inst) => {
            matches!(mir_inst, crate::mir::join_ir::MirLikeInst::Select { .. })
        }
        _ => false,
    });

    let has_boxcall = loop_func.body.iter().any(|inst| match inst {
        JoinInst::Compute(mir_inst) => {
            matches!(mir_inst, crate::mir::join_ir::MirLikeInst::BoxCall { .. })
        }
        _ => false,
    });

    has_cond_jump && has_tail_call && !has_select && !has_boxcall
}

/// Phase 52: Selfhost P3 if-sum family structure signature (dev-only).
///
/// Note: current selfhost baseline is still P2-like (normalize_pattern2_minimal),
/// so the signature avoids requiring Select and focuses on the explicit break-if.
///
/// Distinguish selfhost P3 from canonical P3 by requiring:
/// - loop_step params == 4 (i + host + sum + count)
/// - an explicit Ge compare between params (break-if)
/// - P2/P3 loop skeleton (cond jump + tail call)
/// - no BoxCall in body
pub(super) fn is_selfhost_p3_if_sum_family_candidate(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if loop_step.params.len() != 4 {
        return false;
    }

    let has_cond_jump = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }));
    let has_tail_call = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }));

    let param_set: std::collections::BTreeSet<_> = loop_step.params.iter().copied().collect();

    let has_ge_compare_between_params = loop_step.body.iter().any(|inst| match inst {
        JoinInst::Compute(mir_inst) => match mir_inst {
            crate::mir::join_ir::MirLikeInst::Compare { op, lhs, rhs, .. } => {
                *op == CompareOp::Ge && param_set.contains(lhs) && param_set.contains(rhs)
            }
            _ => false,
        },
        _ => false,
    });

    let has_boxcall = loop_step.body.iter().any(|inst| match inst {
        JoinInst::Compute(mir_inst) => {
            matches!(mir_inst, crate::mir::join_ir::MirLikeInst::BoxCall { .. })
        }
        _ => false,
    });

    has_cond_jump && has_tail_call && has_ge_compare_between_params && !has_boxcall
}

pub(crate) fn is_selfhost_token_scan_p2(module: &JoinModule) -> bool {
    is_selfhost_p2_core_family_candidate(module) && name_guard_exact(module, "selfhost_token_scan_p2")
}

pub(crate) fn is_selfhost_token_scan_p2_accum(module: &JoinModule) -> bool {
    is_selfhost_p2_core_family_candidate(module)
        && name_guard_exact(module, "selfhost_token_scan_p2_accum")
}

pub(crate) fn is_selfhost_if_sum_p3(module: &JoinModule) -> bool {
    is_selfhost_p3_if_sum_family_candidate(module) && name_guard_exact(module, "selfhost_if_sum_p3")
}

pub(crate) fn is_selfhost_if_sum_p3_ext(module: &JoinModule) -> bool {
    is_selfhost_p3_if_sum_family_candidate(module)
        && name_guard_exact(module, "selfhost_if_sum_p3_ext")
}

/// Phase 53: selfhost args-parse P2 detector (practical variation with string carrier)
///
/// Two-stage detection:
/// 1. Structural primary check (P2 break pattern, 1-3 carriers)
/// 2. dev-only name guard for final confirmation (ambiguity resolver)
pub(crate) fn is_selfhost_args_parse_p2(module: &JoinModule) -> bool {
    // 1. Structural primary check (P2 core family)
    if !is_selfhost_p2_core_family_candidate(module) {
        return false;
    }

    // 2. dev-only name guard for final confirmation
    name_guard_exact(module, "selfhost_args_parse_p2")
}

/// Phase 53: selfhost stmt-count P3 detector (practical variation with multi-branch if-else)
///
/// Two-stage detection:
/// 1. Structural primary check (P3 if-sum pattern, 2-10 carriers, multi-branch)
/// 2. dev-only name guard for final confirmation (ambiguity resolver)
pub(crate) fn is_selfhost_stmt_count_p3(module: &JoinModule) -> bool {
    // 1. Structural primary check
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }

    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // Allow 2-10 carriers (5 statement counters: r/e/l/iff/lp + i)
    let carrier_count = loop_step.params.len();
    if !(2..=10).contains(&carrier_count) {
        return false;
    }

    // Must have conditional jump (break pattern)
    let has_cond_jump = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }));

    // Must have tail call (loop continuation)
    let has_tail_call = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }));

    if !has_cond_jump || !has_tail_call {
        return false;
    }

    // 2. dev-only name guard for final confirmation
    name_guard_exact(module, "selfhost_stmt_count_p3")
}

/// Phase 54: selfhost verify-schema P2 detector (Ne-heavy pattern, early return diversity)
///
/// Two-stage detection:
/// 1. Structural primary check (P2 break pattern, 2-3 carriers, Ne conditions)
/// 2. dev-only name guard for final confirmation (ambiguity resolver)
pub(crate) fn is_selfhost_verify_schema_p2(module: &JoinModule) -> bool {
    // 1. Structural primary check (P2 core family)
    if !is_selfhost_p2_core_family_candidate(module) {
        return false;
    }

    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // verify_schema pattern: 2-3 carriers (ver + kind + host param)
    let carrier_count = loop_step.params.len();
    if !(2..=3).contains(&carrier_count) {
        return false;
    }

    // Ne condition pattern (verify != expected)
    let ne_count = count_compare_ops(module, CompareOp::Ne);
    if ne_count < 1 {
        return false; // Ne条件必須
    }

    // 2. dev-only name guard for final confirmation
    name_guard_exact(module, "selfhost_verify_schema_p2")
}

/// Phase 54: selfhost detect-format P3 detector (String return branching, null check)
///
/// Two-stage detection:
/// 1. Structural primary check (P3 if-sum pattern, 2-4 carriers, conditional jump)
/// 2. dev-only name guard for final confirmation (ambiguity resolver)
pub(crate) fn is_selfhost_detect_format_p3(module: &JoinModule) -> bool {
    // 1. Structural primary check
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }

    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // Lightweight P3: 2-4 carriers (conditional branching 3-way + loop variable)
    let carrier_count = loop_step.params.len();
    if !(2..=4).contains(&carrier_count) {
        return false;
    }

    // Conditional branching pattern (multiple if)
    let has_cond_jump = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }));

    if !has_cond_jump {
        return false;
    }

    // 2. dev-only name guard for final confirmation
    name_guard_exact(module, "selfhost_detect_format_p3")
}

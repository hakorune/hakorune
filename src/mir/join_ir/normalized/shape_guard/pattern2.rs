//! Pattern 2 shape detectors (simple break-loop patterns)

use super::utils::{find_loop_step, name_guard_exact};
use crate::mir::join_ir::{JoinInst, JoinModule};

pub(super) fn is_pattern2_mini(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(1..=3).contains(&loop_func.params.len()) {
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

    has_cond_jump && has_tail_call
}

pub(super) fn is_pattern1_mini(module: &JoinModule) -> bool {
    module.is_structured() && find_loop_step(module).is_some()
}

pub(crate) fn is_jsonparser_skip_ws_mini(module: &JoinModule) -> bool {
    is_pattern2_mini(module) && name_guard_exact(module, "jsonparser_skip_ws_mini")
}

pub(crate) fn is_jsonparser_skip_ws_real(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(2..=6).contains(&loop_func.params.len()) {
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

    has_cond_jump && has_tail_call && name_guard_exact(module, "jsonparser_skip_ws_real")
}

pub(crate) fn is_jsonparser_atoi_mini(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(3..=8).contains(&loop_func.params.len()) {
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

    has_cond_jump && has_tail_call && name_guard_exact(module, "jsonparser_atoi_mini")
}

pub(crate) fn is_jsonparser_atoi_real(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(3..=10).contains(&loop_func.params.len()) {
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

    has_cond_jump && has_tail_call && name_guard_exact(module, "jsonparser_atoi_real")
}

pub(crate) fn is_jsonparser_parse_number_real(module: &JoinModule) -> bool {
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }
    let loop_func = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };
    if !(3..=12).contains(&loop_func.params.len()) {
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

    has_cond_jump && has_tail_call && name_guard_exact(module, "jsonparser_parse_number_real")
}

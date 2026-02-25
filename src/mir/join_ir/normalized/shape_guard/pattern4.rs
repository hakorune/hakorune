//! Pattern 4 shape detectors (continue patterns with loop-internal control flow)

use super::utils::{find_loop_step, name_guard_exact};
use crate::mir::join_ir::normalized::loop_step_inspector::LoopStepInspector;
use crate::mir::join_ir::{JoinInst, JoinModule};

/// Phase 48-A: Check if module matches Pattern4 continue minimal shape
///
/// Phase 89: Tightened to prevent continue + early return misdetection:
/// - Requires at least one Select instruction (continue's core)
/// - Requires exactly one conditional Jump to k_exit (loop break, not early return)
pub(crate) fn is_pattern4_continue_minimal(module: &JoinModule) -> bool {
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

    // P4 characteristics (use Inspector for common logic):
    // - Has Compare instruction (loop condition or continue check)
    // - Has Select instruction (continue's core - carrier switching)
    // - Has tail call (loop back)
    // - Has exactly one conditional Jump to k_exit (loop break only)
    //
    // Phase 89: Tightened to exclude loop-internal return patterns

    let has_compare = LoopStepInspector::has_compare_instruction(loop_step);
    let has_select = LoopStepInspector::has_select_instruction(loop_step);
    let k_exit_jumps_count = LoopStepInspector::count_conditional_jumps(loop_step);
    let has_tail_call = LoopStepInspector::has_tail_call(loop_step);
    let reasonable_param_count = LoopStepInspector::has_reasonable_param_count(loop_step);

    // Phase 89: Tightened conditions
    has_compare
        && has_select
        && has_tail_call
        && reasonable_param_count
        && k_exit_jumps_count == 1 // Exactly one loop break (not early return)
}

pub(crate) fn is_jsonparser_parse_array_continue_skip_ws(module: &JoinModule) -> bool {
    is_pattern4_continue_minimal(module)
        && name_guard_exact(module, "jsonparser_parse_array_continue_skip_ws")
}

pub(crate) fn is_jsonparser_parse_object_continue_skip_ws(module: &JoinModule) -> bool {
    is_pattern4_continue_minimal(module)
        && name_guard_exact(module, "jsonparser_parse_object_continue_skip_ws")
}

/// Phase 89: Check if module matches Continue + Early Return pattern
///
/// Structural characteristics:
/// - 3 functions (main, loop_step, k_exit)
/// - Has Select instruction (continue's core)
/// - Has TWO or more conditional Jumps to k_exit (loop break + early return)
/// - Has Compare instruction
/// - Has tail call (loop back)
pub(crate) fn is_pattern_continue_return_minimal(module: &JoinModule) -> bool {
    // Must have exactly 3 functions
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }

    // Find loop_step function
    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // Continue + Return characteristics (use Inspector for common logic):
    // - Has Select instruction (continue's core)
    // - Has TWO or more conditional Jumps (loop break + early return)
    // - Has Compare instruction
    // - Has tail call (loop back)

    let has_compare = LoopStepInspector::has_compare_instruction(loop_step);
    let has_select = LoopStepInspector::has_select_instruction(loop_step);
    let k_exit_jumps_count = LoopStepInspector::count_conditional_jumps(loop_step);
    let has_tail_call = LoopStepInspector::has_tail_call(loop_step);
    let reasonable_param_count = LoopStepInspector::has_reasonable_param_count(loop_step);

    // Phase 89: Continue + Return pattern requires >= 2 conditional Jumps
    has_compare
        && has_select
        && has_tail_call
        && reasonable_param_count
        && k_exit_jumps_count >= 2 // At least 2: loop break + early return
}

/// Phase 90: Check if module matches Parse String Composite pattern
///
/// Structural characteristics:
/// - 3 functions (main, loop_step, k_exit)
/// - Has Select instruction (continue's core)
/// - Has TWO or more conditional Jumps to k_exit (loop break + early return)
/// - Has Compare instruction
/// - Has tail call (loop back)
/// - Has variable step increment (distinguishing feature from ContinueReturn)
///
/// Distinguishing from ContinueReturn:
/// - ParseStringComposite has i+=2 in continue branch (escape character handling)
/// - ContinueReturn has i+=1 in continue branch
/// - Detection: Check for BinOp Add with const value 2 in loop body
pub(crate) fn is_parse_string_composite_minimal(module: &JoinModule) -> bool {
    // Must match basic Continue + Return structure first
    if !is_pattern_continue_return_minimal(module) {
        return false;
    }

    // Find loop_step function
    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // Additional check: Must have BinOp Add with const value 2 (escape handling)
    // This distinguishes ParseStringComposite from generic ContinueReturn
    let has_variable_step = loop_step.body.iter().any(|inst| match inst {
        JoinInst::Compute(mir_inst) => match mir_inst {
            crate::mir::join_ir::MirLikeInst::BinOp { op, rhs, .. } => {
                // Check if it's Add operation
                if *op != crate::mir::join_ir::BinOpKind::Add {
                    return false;
                }
                // Check if rhs is a const value 2 (indicating i+=2 for escape)
                // We need to check if rhs points to a Const instruction with value 2
                loop_step.body.iter().any(|other_inst| match other_inst {
                    JoinInst::Compute(crate::mir::join_ir::MirLikeInst::Const {
                        dst,
                        value,
                    }) => {
                        dst == rhs
                            && matches!(value, crate::mir::join_ir::ConstValue::Integer(2))
                    }
                    _ => false,
                })
            }
            _ => false,
        },
        _ => false,
    });

    has_variable_step
}

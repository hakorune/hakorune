//! Step placement validation helpers for generic loop analysis

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::reject_reason::{
    handoff_tables, log_reject, RejectReason,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::verify::coreloop_body_contract::is_effect_only_stmt;

use super::super::facts::stmt_classifier::{
    body_has_continue, is_effect_if, is_exit_if, is_local_init, is_simple_assignment,
    stmt_uses_loop_var,
};
use super::super::facts_helpers::reject_or_false;

/// Checks if there is control flow after the step index
pub(in crate::mir::builder) fn has_control_flow_after_step(
    body: &[ASTNode],
    step_index: usize,
) -> bool {
    for stmt in body.iter().skip(step_index + 1) {
        if is_exit_if(stmt) {
            return true;
        }
        if matches!(
            stmt,
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. }
        ) {
            return true;
        }
    }
    false
}

/// Validates in-body step placement for v0 (strict: no continue allowed)
pub(in crate::mir::builder) fn validate_in_body_step(
    body: &[ASTNode],
    step_index: usize,
    loop_var: &str,
    loop_increment: &ASTNode,
    strict: bool,
) -> Result<bool, Freeze> {
    if body_has_continue(body) {
        log_reject(
            "generic_loop",
            RejectReason::InBodyStepWithContinue,
            handoff_tables::for_generic_loop,
        );
        return reject_or_false(
            strict,
            RejectReason::InBodyStepWithContinue.as_freeze_message(),
        );
    }
    for stmt in body.iter().skip(step_index + 1) {
        if is_exit_if(stmt) || is_effect_if(stmt, loop_var, loop_increment) {
            log_reject(
                "generic_loop_v0",
                RejectReason::ControlFlowAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::ControlFlowAfterInBodyStep.as_freeze_message(),
            );
        }
        if matches!(
            stmt,
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. }
        ) {
            log_reject(
                "generic_loop_v0",
                RejectReason::ExitAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::ExitAfterInBodyStep.as_freeze_message(),
            );
        }
        if stmt_uses_loop_var(stmt, loop_var) {
            log_reject(
                "generic_loop_v0",
                RejectReason::LoopVarUsedAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::LoopVarUsedAfterInBodyStep.as_freeze_message(),
            );
        }
        if is_simple_assignment(stmt, loop_var)
            || is_local_init(stmt, loop_var)
            || is_effect_only_stmt(stmt)
        {
            continue;
        }
        log_reject(
            "generic_loop_v0",
            RejectReason::UnsupportedStmtAfterInBodyStep,
            handoff_tables::for_generic_loop,
        );
        return reject_or_false(
            strict,
            RejectReason::UnsupportedStmtAfterInBodyStep.as_freeze_message(),
        );
    }
    Ok(true)
}

/// Validates in-body step placement for v1 (more permissive: continue allowed)
pub(in crate::mir::builder) fn validate_in_body_step_v1(
    body: &[ASTNode],
    step_index: usize,
    loop_var: &str,
    loop_increment: &ASTNode,
    strict: bool,
) -> Result<bool, Freeze> {
    for stmt in body.iter().skip(step_index + 1) {
        if is_exit_if(stmt) || is_effect_if(stmt, loop_var, loop_increment) {
            log_reject(
                "generic_loop_v1",
                RejectReason::ControlFlowAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::ControlFlowAfterInBodyStep.as_freeze_message(),
            );
        }
        if matches!(
            stmt,
            ASTNode::Break { .. } | ASTNode::Continue { .. } | ASTNode::Return { .. }
        ) {
            log_reject(
                "generic_loop_v1",
                RejectReason::ExitAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::ExitAfterInBodyStep.as_freeze_message(),
            );
        }
        if stmt_uses_loop_var(stmt, loop_var) {
            log_reject(
                "generic_loop_v1",
                RejectReason::LoopVarUsedAfterInBodyStep,
                handoff_tables::for_generic_loop,
            );
            return reject_or_false(
                strict,
                RejectReason::LoopVarUsedAfterInBodyStep.as_freeze_message(),
            );
        }
        if is_simple_assignment(stmt, loop_var)
            || is_local_init(stmt, loop_var)
            || is_effect_only_stmt(stmt)
        {
            continue;
        }
        log_reject(
            "generic_loop_v1",
            RejectReason::UnsupportedStmtAfterInBodyStep,
            handoff_tables::for_generic_loop,
        );
        return reject_or_false(
            strict,
            RejectReason::UnsupportedStmtAfterInBodyStep.as_freeze_message(),
        );
    }
    Ok(true)
}

/// Validates continue-if step placement
pub(in crate::mir::builder) fn validate_continue_if_step(
    body: &[ASTNode],
    step_index: usize,
    strict: bool,
) -> Result<bool, Freeze> {
    let tail = if step_index + 1 >= body.len() {
        &[]
    } else {
        &body[step_index + 1..]
    };
    if tail.is_empty() {
        return Ok(true);
    }
    if tail.len() == 1 && matches!(tail[0], ASTNode::Break { .. } | ASTNode::Return { .. }) {
        return Ok(true);
    }
    log_reject(
        "generic_loop_v0",
        RejectReason::ContinueIfStepRequiresTrailingExit,
        handoff_tables::for_generic_loop,
    );
    reject_or_false(
        strict,
        RejectReason::ContinueIfStepRequiresTrailingExit.as_freeze_message(),
    )
}

/// Validates break-else-if step placement
pub(in crate::mir::builder) fn validate_break_else_if_step(
    body: &[ASTNode],
    step_index: usize,
    strict: bool,
) -> Result<bool, Freeze> {
    if step_index + 1 == body.len() {
        return Ok(true);
    }
    log_reject(
        "generic_loop_v0",
        RejectReason::BreakElseStepMustBeFinalStmt,
        handoff_tables::for_generic_loop,
    );
    reject_or_false(
        strict,
        RejectReason::BreakElseStepMustBeFinalStmt.as_freeze_message(),
    )
}

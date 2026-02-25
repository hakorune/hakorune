//! Phase 29bq P2: LoopTrueBreakContinueFacts (Facts SSOT)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, is_true_literal, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopTrueBreakContinueFacts {
    pub body: Vec<ASTNode>,
}

pub(in crate::mir::builder) fn try_extract_loop_true_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopTrueBreakContinueFacts>, Freeze> {
    if !is_true_literal(condition) {
        return Ok(None);
    }

    let mut detector = ControlFlowDetector::default();
    detector.count_returns = true;
    let counts = count_control_flow(body, detector);
    if counts.has_nested_loop {
        return Ok(None);
    }
    if counts.return_count > 0 {
        return Ok(None);
    }
    if counts.break_count == 0 && counts.continue_count == 0 {
        return Ok(None);
    }

    let mut exit_if_seen = 0usize;
    let mut idx = 0usize;
    while idx < body.len() {
        let stmt = &body[idx];
        let next = body.get(idx + 1);

        if is_loop_true_break_continue_stmt(stmt, next, &mut exit_if_seen) {
            // If we consumed an if+tail-exit pair, skip the next statement.
            if is_if_tail_exit_pair(stmt, next)
            {
                idx += 2;
                continue;
            }
            idx += 1;
            continue;
        }

        return Ok(None);
    }

    if exit_if_seen == 0 {
        return Ok(None);
    }

    Ok(Some(LoopTrueBreakContinueFacts {
        body: body.to_vec(),
    }))
}

fn is_loop_true_break_continue_stmt(
    stmt: &ASTNode,
    _next: Option<&ASTNode>,
    exit_if_seen: &mut usize,
) -> bool {
    match stmt {
        ASTNode::Assignment { .. }
        | ASTNode::Local { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FunctionCall { .. } => true,
        ASTNode::If { then_body, else_body, .. } => {
            // Accept "if { ... break/continue }" blocks with effect-only prefix.
            if !is_exit_block(then_body) {
                return false;
            }
            if let Some(else_body) = else_body {
                if !is_exit_block(else_body) {
                    return false;
                }
            }
            *exit_if_seen += 1;
            true
        }
        _ => false,
    }
}

fn is_if_tail_exit_pair(stmt: &ASTNode, next: Option<&ASTNode>) -> bool {
    let ASTNode::If { then_body, else_body: None, .. } = stmt else {
        return false;
    };
    let Some(next) = next else {
        return false;
    };
    let Some(last) = then_body.last() else {
        return false;
    };

    match (last, next) {
        (ASTNode::Continue { .. }, ASTNode::Break { .. }) => true,
        (ASTNode::Break { .. }, ASTNode::Continue { .. }) => true,
        _ => false,
    }
}

fn is_exit_block(body: &[ASTNode]) -> bool {
    if body.is_empty() {
        return false;
    }
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { .. }
            | ASTNode::Local { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FunctionCall { .. } => {
                if is_last {
                    // Exit must be explicit at the end.
                    return false;
                }
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last {
                    return false;
                }
            }
            _ => return false,
        }
    }
    true
}

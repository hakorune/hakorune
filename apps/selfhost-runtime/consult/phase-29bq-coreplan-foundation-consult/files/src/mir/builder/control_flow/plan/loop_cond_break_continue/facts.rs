//! Phase 29bq P2: LoopCondBreakContinueFacts (Facts SSOT)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use crate::mir::builder::control_flow::plan::extractors::common_helpers::{
    count_control_flow, is_true_literal, ControlFlowDetector,
};
use crate::mir::builder::control_flow::plan::planner::Freeze;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopCondBreakContinueFacts {
    pub condition: ASTNode,
    pub body: Vec<ASTNode>,
}

pub(in crate::mir::builder) fn try_extract_loop_cond_break_continue_facts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopCondBreakContinueFacts>, Freeze> {
    if is_true_literal(condition) {
        return Ok(None);
    }
    if !is_supported_bool_expr(condition) {
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
    // Phase 29bq P2.5: allow break+continue mixtures, but keep the scope conservative.
    // - break-only remains supported (existing gate/fixture)
    // - continue-only is rejected to avoid unintended overlaps with Pattern4-style loops
    if counts.continue_count > 0 && counts.break_count == 0 {
        return Ok(None);
    }

    let mut exit_if_seen = 0usize;
    for stmt in body {
        if is_loop_cond_break_continue_stmt(stmt, &mut exit_if_seen) {
            continue;
        }
        return Ok(None);
    }

    if exit_if_seen == 0 {
        return Ok(None);
    }

    Ok(Some(LoopCondBreakContinueFacts {
        condition: condition.clone(),
        body: body.to_vec(),
    }))
}

fn is_loop_cond_break_continue_stmt(stmt: &ASTNode, exit_if_seen: &mut usize) -> bool {
    match stmt {
        ASTNode::Assignment { .. }
        | ASTNode::Local { .. }
        | ASTNode::MethodCall { .. }
        | ASTNode::FunctionCall { .. } => true,
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            if is_exit_if_stmt(condition, then_body, else_body.as_ref()) {
                *exit_if_seen += 1;
                return true;
            }
            if is_conditional_update_if(condition, then_body, else_body.as_ref()) {
                return true;
            }
            false
        }
        _ => false,
    }
}

fn is_exit_if_stmt(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> bool {
    if !is_supported_bool_expr(condition) {
        return false;
    }
    if then_body.len() != 1 {
        return false;
    }
    if !matches!(then_body[0], ASTNode::Break { .. } | ASTNode::Continue { .. }) {
        return false;
    }
    if let Some(else_body) = else_body {
        if else_body.len() != 1 {
            return false;
        }
        if !matches!(else_body[0], ASTNode::Break { .. } | ASTNode::Continue { .. }) {
            return false;
        }
    }
    true
}

fn is_conditional_update_if(
    condition: &ASTNode,
    then_body: &[ASTNode],
    else_body: Option<&Vec<ASTNode>>,
) -> bool {
    if !is_supported_bool_expr(condition) {
        return false;
    }

    let mut saw_assignment = false;
    if !is_conditional_update_branch(then_body, &mut saw_assignment) {
        return false;
    }
    if let Some(else_body) = else_body {
        if !is_conditional_update_branch(else_body, &mut saw_assignment) {
            return false;
        }
    }

    saw_assignment
}

fn is_conditional_update_branch(body: &[ASTNode], saw_assignment: &mut bool) -> bool {
    let mut saw_exit = false;
    for (idx, stmt) in body.iter().enumerate() {
        let is_last = idx + 1 == body.len();
        match stmt {
            ASTNode::Assignment { target, value, .. } => {
                if !matches!(target.as_ref(), ASTNode::Variable { .. }) {
                    return false;
                }
                if !is_pure_value_expr(value) {
                    return false;
                }
                *saw_assignment = true;
            }
            ASTNode::Break { .. } | ASTNode::Continue { .. } => {
                if !is_last || saw_exit {
                    return false;
                }
                saw_exit = true;
            }
            _ => return false,
        }
    }
    true
}

fn is_supported_value_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::MethodCall { .. } => true,
        ASTNode::BinaryOp {
            operator,
            left: _,
            right: _,
            ..
        } => matches!(
            operator,
            BinaryOperator::Add
                | BinaryOperator::Subtract
                | BinaryOperator::Multiply
                | BinaryOperator::Divide
                | BinaryOperator::Modulo
        ),
        _ => false,
    }
}

fn is_pure_value_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::Variable { .. } => true,
        ASTNode::Literal { .. } => true,
        ASTNode::UnaryOp { operand, .. } => is_pure_value_expr(operand),
        ASTNode::BinaryOp { operator, left, right, .. } => {
            matches!(
                operator,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
                    | BinaryOperator::Less
                    | BinaryOperator::LessEqual
                    | BinaryOperator::Greater
                    | BinaryOperator::GreaterEqual
                    | BinaryOperator::Equal
                    | BinaryOperator::NotEqual
            ) && is_pure_value_expr(left)
                && is_pure_value_expr(right)
        }
        _ => false,
    }
}

fn is_supported_bool_expr(ast: &ASTNode) -> bool {
    match ast {
        ASTNode::MethodCall { .. } | ASTNode::Variable { .. } => true,
        ASTNode::Literal {
            value: LiteralValue::Bool(_),
            ..
        } => true,
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => match operator {
            BinaryOperator::And | BinaryOperator::Or => {
                is_supported_bool_expr(left) && is_supported_bool_expr(right)
            }
            BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual
            | BinaryOperator::Equal
            | BinaryOperator::NotEqual => {
                is_supported_value_expr(left) && is_supported_value_expr(right)
            }
            _ => false,
        },
        _ => false,
    }
}

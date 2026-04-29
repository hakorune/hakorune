//! loop_break body-local promotion facts helpers.

use crate::ast::{ASTNode, BinaryOperator};
use crate::mir::builder::control_flow::plan::planner::Freeze;

use super::body_local_facts::LoopBreakBodyLocalFacts;
use super::body_local_facts_shape_matchers::{try_match_digit_pos, try_match_trim_seg};

pub(super) fn try_extract_loop_break_body_local_facts_inner(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<LoopBreakBodyLocalFacts>, Freeze> {
    let Some(loop_var) = extract_loop_var(condition) else {
        return Ok(None);
    };
    let Some((break_condition, break_idx)) = find_break_guard_if(body) else {
        return Ok(None);
    };

    if let Some((body_local_var, shape)) =
        try_match_trim_seg(break_condition, body, break_idx, &loop_var)
    {
        return Ok(Some(LoopBreakBodyLocalFacts {
            loop_var,
            body_local_var,
            shape,
        }));
    }

    if let Some((body_local_var, shape)) =
        try_match_digit_pos(break_condition, body, break_idx, &loop_var)
    {
        return Ok(Some(LoopBreakBodyLocalFacts {
            loop_var,
            body_local_var,
            shape,
        }));
    }

    Ok(None)
}

fn extract_loop_var(condition: &ASTNode) -> Option<String> {
    let ASTNode::BinaryOp { operator, left, .. } = condition else {
        return None;
    };
    if !matches!(operator, BinaryOperator::Less | BinaryOperator::LessEqual) {
        return None;
    }
    let ASTNode::Variable { name, .. } = left.as_ref() else {
        return None;
    };
    Some(name.clone())
}

fn find_break_guard_if(body: &[ASTNode]) -> Option<(&ASTNode, usize)> {
    for (idx, stmt) in body.iter().enumerate() {
        let ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } = stmt
        else {
            continue;
        };
        if else_body.is_some() {
            continue;
        }
        let has_break_at_end = then_body
            .last()
            .map(|n| matches!(n, ASTNode::Break { .. }))
            .unwrap_or(false);
        if !has_break_at_end {
            continue;
        }
        return Some((condition.as_ref(), idx));
    }
    None
}

pub(super) fn find_local_init_expr(body: &[ASTNode], name: &str) -> Option<(usize, ASTNode)> {
    for (idx, stmt) in body.iter().enumerate() {
        let ASTNode::Local {
            variables,
            initial_values,
            ..
        } = stmt
        else {
            continue;
        };
        if variables.len() != 1 || initial_values.len() != 1 {
            continue;
        }
        if variables[0] != name {
            continue;
        }
        let Some(expr) = initial_values[0].as_ref() else {
            return None;
        };
        return Some((idx, (*expr.clone()).clone()));
    }
    None
}

//! Break/If pattern extraction helpers for loop_break facts.

use crate::ast::ASTNode;

pub(in crate::mir::builder::control_flow::plan) fn extract_break_if_parts(
    stmt: &ASTNode,
) -> Option<(ASTNode, Option<ASTNode>)> {
    let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = stmt
    else {
        return None;
    };
    if else_body.is_some() {
        return None;
    }

    let has_break_at_end = then_body
        .last()
        .map(|n| matches!(n, ASTNode::Break { .. }))
        .unwrap_or(false);
    if !has_break_at_end {
        return None;
    }

    let carrier_update_in_break = if then_body.len() == 1 {
        None
    } else if then_body.len() == 2 {
        match &then_body[0] {
            ASTNode::Assignment { value, .. } => Some(value.as_ref().clone()),
            _ => return None,
        }
    } else {
        return None;
    };

    Some((condition.as_ref().clone(), carrier_update_in_break))
}

pub(in crate::mir::builder::control_flow::plan) fn find_break_if_parts(
    body: &[ASTNode],
) -> Option<(usize, ASTNode, Option<ASTNode>)> {
    for (idx, stmt) in body.iter().enumerate() {
        if let Some(parts) = extract_break_if_parts(stmt) {
            return Some((idx, parts.0, parts.1));
        }
    }
    None
}

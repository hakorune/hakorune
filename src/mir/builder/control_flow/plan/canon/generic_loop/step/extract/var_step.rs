use crate::ast::{ASTNode, BinaryOperator};
use std::collections::BTreeSet;

use super::shared::collect_assigned_var_names;

pub(super) fn extract_var_step_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    for stmt in body {
        let ASTNode::Assignment { target, value, .. } = stmt else {
            continue;
        };
        let ASTNode::Variable { name, .. } = target.as_ref() else {
            continue;
        };
        if name != loop_var {
            continue;
        }
        let ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } = value.as_ref()
        else {
            continue;
        };
        if *operator != BinaryOperator::Add {
            continue;
        }

        let is_loop_var =
            |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
        let is_any_var = |node: &ASTNode| matches!(node, ASTNode::Variable { .. });

        if (is_loop_var(left.as_ref()) && is_any_var(right.as_ref()))
            || (is_any_var(left.as_ref()) && is_loop_var(right.as_ref()))
        {
            return Some(value.as_ref().clone());
        }
    }
    if let Some(step) = extract_tail_assigned_local_var_step_increment(body, loop_var) {
        return Some(step);
    }
    extract_direct_var_step_increment(body, loop_var)
}

fn extract_tail_assigned_local_var_step_increment(
    body: &[ASTNode],
    loop_var: &str,
) -> Option<ASTNode> {
    let ASTNode::Assignment { target, value, .. } = body.last()? else {
        return None;
    };
    let ASTNode::Variable {
        name: target_name, ..
    } = target.as_ref()
    else {
        return None;
    };
    if target_name != loop_var {
        return None;
    }
    let ASTNode::Variable { name: step_var, .. } = value.as_ref() else {
        return None;
    };
    if step_var == loop_var {
        return None;
    }

    let has_top_level_local = body.iter().any(|stmt| {
        matches!(
            stmt,
            ASTNode::Local { variables, .. } if variables.iter().any(|name| name == step_var)
        )
    });
    if !has_top_level_local {
        return None;
    }

    let mut assigned = BTreeSet::new();
    for stmt in body {
        collect_assigned_var_names(stmt, &mut assigned);
    }
    if !assigned.contains(step_var) {
        return None;
    }

    Some(value.as_ref().clone())
}

fn extract_direct_var_step_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    if body.len() != 4 {
        return None;
    }
    let step_var = extract_single_local_name(&body[0])?;
    if !matches!(body[1], ASTNode::Local { .. }) {
        return None;
    }
    if !matches!(body[2], ASTNode::If { .. }) {
        return None;
    }
    let ASTNode::Assignment { target, value, .. } = &body[3] else {
        return None;
    };
    let ASTNode::Variable { name, .. } = target.as_ref() else {
        return None;
    };
    if name != loop_var {
        return None;
    }
    match value.as_ref() {
        ASTNode::Variable { name, .. } if name == &step_var => Some(value.as_ref().clone()),
        _ => None,
    }
}

fn extract_single_local_name(stmt: &ASTNode) -> Option<String> {
    let ASTNode::Local { variables, .. } = stmt else {
        return None;
    };
    if variables.len() != 1 {
        return None;
    }
    Some(variables[0].clone())
}

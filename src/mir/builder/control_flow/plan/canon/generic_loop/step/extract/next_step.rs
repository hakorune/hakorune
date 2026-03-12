use crate::ast::{ASTNode, BinaryOperator, LiteralValue};
use std::collections::BTreeSet;

use super::shared::collect_assigned_var_names;

pub(super) fn extract_next_i_increment(body: &[ASTNode], loop_var: &str) -> Option<ASTNode> {
    let mut next_vars = Vec::new();
    let mut next_init: Option<ASTNode> = None;
    for stmt in body {
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
        let name = &variables[0];
        if name == loop_var {
            continue;
        }
        let Some(init) = &initial_values[0] else {
            continue;
        };
        if is_loop_var_plus_one(init, loop_var) {
            next_vars.push(name.clone());
            if next_init.is_none() {
                next_init = Some(init.as_ref().clone());
            }
        }
    }

    let Some(next_init) = next_init else {
        return None;
    };

    let mut assigned = BTreeSet::new();
    for stmt in body {
        collect_assigned_var_names(stmt, &mut assigned);
    }

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
        let ASTNode::Variable {
            name: value_name, ..
        } = value.as_ref()
        else {
            continue;
        };
        if next_vars.iter().any(|var| var == value_name) {
            if assigned.contains(value_name) {
                return Some(value.as_ref().clone());
            }
            return Some(next_init.clone());
        }
    }
    None
}

fn is_loop_var_plus_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var =
        |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    let is_one = |node: &ASTNode| {
        matches!(
            node,
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        )
    };
    (is_loop_var(left.as_ref()) && is_one(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_one(left.as_ref()))
}

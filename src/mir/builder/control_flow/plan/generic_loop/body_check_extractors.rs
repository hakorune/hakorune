//! Extraction helpers for generic loop analysis
//!
//! Provides utilities for extracting variables and values from loop body statements.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

use super::facts::stmt_classifier::is_local_init;

/// Extracts the next "i" variable from a local declaration.
///
/// Looks for `local i = loop_var + 1` pattern where the variable name
/// is not the loop variable itself.
///
/// # Arguments
/// * `stmt` - The statement to examine
/// * `loop_var` - The loop variable name to exclude
///
/// # Returns
/// * `Some(name)` - If the statement is `local x = loop_var + 1` where x != loop_var
/// * `None` - Otherwise
pub(super) fn extract_next_i_var(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    let ASTNode::Local {
        variables,
        initial_values,
        ..
    } = stmt
    else {
        return None;
    };
    if variables.len() != 1 || initial_values.len() != 1 {
        return None;
    }
    let name = variables[0].clone();
    if name == loop_var {
        return None;
    }
    let Some(init) = &initial_values[0] else {
        return None;
    };
    if !is_loop_var_plus_one(init, loop_var) {
        return None;
    }
    Some(name)
}

/// Collects all variables that are initialized to `loop_var + 1`.
///
/// Scans the body for statements like `local x = loop_var + 1` and
/// collects the variable names.
///
/// # Arguments
/// * `body` - The loop body statements
/// * `loop_var` - The loop variable name
///
/// # Returns
/// Vector of variable names that are initialized to `loop_var + 1`
pub(super) fn collect_next_step_vars(body: &[ASTNode], loop_var: &str) -> Vec<String> {
    let mut out = Vec::new();
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
            out.push(name.clone());
        }
    }
    out
}

/// Checks if an expression is `loop_var + 1` (commutative).
///
/// # Arguments
/// * `expr` - The expression to check
/// * `loop_var` - The loop variable name
///
/// # Returns
/// * `true` - If expr is `loop_var + 1` or `1 + loop_var`
/// * `false` - Otherwise
pub(super) fn is_loop_var_plus_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var = |node: &ASTNode| matches!(node, ASTNode::Variable { name, .. } if name == loop_var);
    let is_one = |node: &ASTNode| {
        matches!(node, ASTNode::Literal { value: LiteralValue::Integer(1), .. })
    };
    (is_loop_var(left.as_ref()) && is_one(right.as_ref()))
        || (is_loop_var(right.as_ref()) && is_one(left.as_ref()))
}

/// Checks if an expression is `loop_var - 1`.
///
/// # Arguments
/// * `expr` - The expression to check
/// * `loop_var` - The loop variable name
///
/// # Returns
/// * `true` - If expr is `loop_var - 1`
/// * `false` - Otherwise
pub(super) fn is_loop_var_minus_one(expr: &ASTNode, loop_var: &str) -> bool {
    let ASTNode::BinaryOp {
        operator: BinaryOperator::Subtract,
        left,
        right,
        ..
    } = expr
    else {
        return false;
    };
    let is_loop_var = |node: &ASTNode| {
        matches!(node, ASTNode::Variable { name, .. } if name == loop_var)
    };
    matches!(
        (left.as_ref(), right.as_ref()),
        (
            node_left,
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        ) if is_loop_var(node_left)
    )
}

/// Extracts the local variable name from a local initialization statement.
///
/// # Arguments
/// * `stmt` - The statement to examine
/// * `loop_var` - The loop variable name (used for validation)
///
/// # Returns
/// * `Some(name)` - If the statement is a local initialization
/// * `None` - Otherwise
pub(super) fn extract_local_init_var(stmt: &ASTNode, loop_var: &str) -> Option<String> {
    if !is_local_init(stmt, loop_var) {
        return None;
    }
    let ASTNode::Local { variables, .. } = stmt else {
        return None;
    };
    if variables.len() != 1 {
        return None;
    }
    Some(variables[0].clone())
}

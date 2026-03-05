//! Control flow utility functions.
//!
//! This module provides helper functions for control flow analysis and manipulation.

use crate::ast::{ASTNode, BinaryOperator};

/// Extract loop variable name from condition.
///
/// # Examples
///
/// - For `i < 3`, extracts `i`
/// - For `arr.length() > 0`, extracts `arr`
///
/// # Implementation
///
/// This is a minimal implementation that handles simple comparison patterns.
/// It looks for binary comparison operators (< > <= >=) and extracts the
/// variable name from the left side of the comparison.
///
/// # Errors
///
/// Returns an error if:
/// - The condition is not a binary comparison
/// - The left side of the comparison is not a simple variable
pub(in crate::mir::builder) fn extract_loop_variable_from_condition(
    condition: &ASTNode,
) -> Result<String, String> {
    match condition {
        ASTNode::BinaryOp { operator, left, .. }
            if matches!(
                operator,
                BinaryOperator::Less
                    | BinaryOperator::Greater
                    | BinaryOperator::LessEqual
                    | BinaryOperator::GreaterEqual
            ) =>
        {
            // Binary comparison: extract variable from left side
            match &**left {
                ASTNode::Variable { name, .. } => Ok(name.clone()),
                _ => Err(format!(
                    "[cf_loop/loop_var_extract] Cannot extract loop variable from condition: {:?}",
                    condition
                )),
            }
        }
        _ => Err(format!(
            "[cf_loop/loop_var_extract] Unsupported loop condition shape: {:?}",
            condition
        )),
    }
}

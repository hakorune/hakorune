//! Phase 100 P1-2: Pinned Local Analyzer
//!
//! Pure AST box for identifying pinned loop-outer locals (read-only locals used in loop body).
//! Pinned locals are variables that:
//! - Are defined before the loop
//! - Are referenced in the loop body
//! - Are NOT assigned in the loop body
//!
//! This is the "judgment box" - it decides what should be pinned, without any MIR dependencies.

use crate::ast::ASTNode;
use std::collections::BTreeSet;

/// Analyzes loop body AST to identify pinned locals.
///
/// # Arguments
///
/// * `loop_body` - AST nodes of the loop body
/// * `candidate_locals` - Set of candidate local variable names (from ScopeManager)
///
/// # Returns
///
/// * `Ok(BTreeSet<String>)` - Set of pinned local names
/// * `Err(String)` - Error message if input validation fails
///
/// # Invariants
///
/// A local is pinned if ALL of the following hold:
/// 1. It appears in `candidate_locals` (defined before loop)
/// 2. It is referenced in `loop_body` (used in expressions)
/// 3. It is NOT assigned in `loop_body` (read-only)
///
/// # Fail-Fast
///
/// Returns `Err` with clear reason if:
/// - `loop_body` is empty (validation error)
/// - `candidate_locals` is empty (no candidates to analyze)
pub fn analyze_pinned_locals(
    loop_body: &[ASTNode],
    candidate_locals: &BTreeSet<String>,
) -> Result<BTreeSet<String>, String> {
    // Fail-Fast: Input validation
    if loop_body.is_empty() {
        return Err("Loop body is empty (cannot analyze pinned locals)".to_string());
    }

    if candidate_locals.is_empty() {
        return Ok(BTreeSet::new()); // No candidates, no pinned locals
    }

    // Step 1: Collect all variables referenced in loop body
    let mut referenced_vars = BTreeSet::new();
    collect_referenced_vars(loop_body, &mut referenced_vars);

    // Step 2: Collect all variables assigned in loop body
    let mut assigned_vars = BTreeSet::new();
    collect_assigned_vars(loop_body, &mut assigned_vars);

    // Step 3: Filter candidates to find pinned locals
    let mut pinned_locals = BTreeSet::new();
    for name in candidate_locals {
        // Must be referenced AND NOT assigned
        if referenced_vars.contains(name) && !assigned_vars.contains(name) {
            pinned_locals.insert(name.clone());
        }
    }

    Ok(pinned_locals)
}

/// Recursively collects all variable names referenced in the AST.
fn collect_referenced_vars(nodes: &[ASTNode], result: &mut BTreeSet<String>) {
    for node in nodes {
        collect_referenced_vars_in_node(node, result);
    }
}

/// Helper function to collect variable references from a single AST node.
fn collect_referenced_vars_in_node(node: &ASTNode, result: &mut BTreeSet<String>) {
    match node {
        ASTNode::Variable { name, .. } => {
            result.insert(name.clone());
        }
        ASTNode::BinaryOp { left, right, .. } => {
            collect_referenced_vars_in_node(left, result);
            collect_referenced_vars_in_node(right, result);
        }
        ASTNode::UnaryOp { operand, .. } => {
            collect_referenced_vars_in_node(operand, result);
        }
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            collect_referenced_vars_in_node(object, result);
            for arg in arguments {
                collect_referenced_vars_in_node(arg, result);
            }
        }
        ASTNode::Call { arguments, .. } => {
            for arg in arguments {
                collect_referenced_vars_in_node(arg, result);
            }
        }
        ASTNode::Assignment { value, .. } => {
            // Only collect from RHS (value expression)
            collect_referenced_vars_in_node(value, result);
        }
        ASTNode::Local {
            initial_values, ..
        } => {
            for init_opt in initial_values {
                if let Some(init) = init_opt {
                    collect_referenced_vars_in_node(init, result);
                }
            }
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            collect_referenced_vars_in_node(condition, result);
            collect_referenced_vars(then_body, result);
            if let Some(else_stmts) = else_body {
                collect_referenced_vars(else_stmts, result);
            }
        }
        ASTNode::Loop { condition, body, .. } => {
            collect_referenced_vars_in_node(condition, result);
            collect_referenced_vars(body, result);
        }
        ASTNode::Return { value, .. } => {
            if let Some(val) = value {
                collect_referenced_vars_in_node(val, result);
            }
        }
        ASTNode::MatchExpr {
            scrutinee, arms, else_expr, ..
        } => {
            collect_referenced_vars_in_node(scrutinee, result);
            for (_pattern, arm_expr) in arms {
                collect_referenced_vars_in_node(arm_expr, result);
            }
            collect_referenced_vars_in_node(else_expr, result);
        }
        // Literals, Box declarations, etc. don't reference variables
        _ => {}
    }
}

/// Recursively collects all variable names assigned in the AST.
fn collect_assigned_vars(nodes: &[ASTNode], result: &mut BTreeSet<String>) {
    for node in nodes {
        collect_assigned_vars_in_node(node, result);
    }
}

/// Helper function to collect variable assignments from a single AST node.
fn collect_assigned_vars_in_node(node: &ASTNode, result: &mut BTreeSet<String>) {
    match node {
        ASTNode::Assignment { target, .. } => {
            // Collect from LHS (target)
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                result.insert(name.clone());
            }
        }
        ASTNode::Local { variables, .. } => {
            // Local declarations are assignments (but handled separately in scope analysis)
            for var_name in variables {
                result.insert(var_name.clone());
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            collect_assigned_vars(then_body, result);
            if let Some(else_stmts) = else_body {
                collect_assigned_vars(else_stmts, result);
            }
        }
        ASTNode::Loop { body, .. } => {
            collect_assigned_vars(body, result);
        }
        ASTNode::MatchExpr { .. } => {
            // Match expressions are expressions, not statements
            // So we don't collect assignments from them
        }
        // Other nodes don't contain assignments
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_pinned_when_no_assignment() {
        // Build AST for: local digit = digits.indexOf(ch)
        // Expected: 'digits' is pinned (referenced, not assigned)

        let loop_body = vec![ASTNode::Local {
            variables: vec!["digit".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "digits".to_string(),
                    span: Span::unknown(),
                }),
                method: "indexOf".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let mut candidates = BTreeSet::new();
        candidates.insert("digits".to_string());

        let result = analyze_pinned_locals(&loop_body, &candidates).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result.contains("digits"));
    }

    #[test]
    fn test_not_pinned_when_assigned() {
        // Build AST for: digits = "abc"
        // Expected: 'digits' is NOT pinned (assigned in loop body)

        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "digits".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::String("abc".to_string()),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let mut candidates = BTreeSet::new();
        candidates.insert("digits".to_string());

        let result = analyze_pinned_locals(&loop_body, &candidates).unwrap();

        // Should be empty because 'digits' is assigned in loop body
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_empty_candidates_returns_empty() {
        let loop_body = vec![ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(42),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let candidates = BTreeSet::new();

        let result = analyze_pinned_locals(&loop_body, &candidates).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_empty_loop_body_returns_error() {
        let loop_body = vec![];
        let mut candidates = BTreeSet::new();
        candidates.insert("x".to_string());

        let result = analyze_pinned_locals(&loop_body, &candidates);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Loop body is empty (cannot analyze pinned locals)"));
    }

    #[test]
    fn test_referenced_in_condition_and_body() {
        // Build AST for if-statement in loop body using 'table'
        let loop_body = vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::MethodCall {
                    object: Box::new(ASTNode::Variable {
                        name: "table".to_string(), // Referenced here
                        span: Span::unknown(),
                    }),
                    method: "length".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: None,
            span: Span::unknown(),
        }];

        let mut candidates = BTreeSet::new();
        candidates.insert("table".to_string());

        let result = analyze_pinned_locals(&loop_body, &candidates).unwrap();

        assert_eq!(result.len(), 1);
        assert!(result.contains("table"));
    }
}

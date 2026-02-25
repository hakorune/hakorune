//! Condition Variable Extractor
//!
//! This module provides utilities for extracting variable names from condition AST nodes.
//! It is used to determine which variables need to be available in JoinIR scope when
//! lowering loop conditions.
//!
//! ## Design Philosophy
//!
//! **Single Responsibility**: This module ONLY extracts variable names from AST.
//! It does NOT:
//! - Lower AST to JoinIR (that's condition_lowerer.rs)
//! - Manage variable environments (that's condition_env.rs)
//! - Perform type inference or validation

use crate::ast::ASTNode;
use std::collections::BTreeSet;

/// Extract all variable names used in a condition AST
///
/// This helper recursively traverses the condition AST and collects all
/// unique variable names. Used to determine which variables need to be
/// available in JoinIR scope.
///
/// # Arguments
///
/// * `cond_ast` - AST node representing the condition
/// * `exclude_vars` - Variable names to exclude (e.g., loop parameters already registered)
///
/// # Returns
///
/// Sorted vector of unique variable names found in the condition
///
/// # Example
///
/// ```ignore
/// // For condition: start < end && i < len
/// let vars = extract_condition_variables(
///     condition_ast,
///     &["i".to_string()],  // Exclude loop variable 'i'
/// );
/// // Result: ["end", "len", "start"] (sorted, 'i' excluded)
/// ```
pub fn extract_condition_variables(cond_ast: &ASTNode, exclude_vars: &[String]) -> Vec<String> {
    let mut all_vars = BTreeSet::new();
    collect_variables_recursive(cond_ast, &mut all_vars);

    // Filter out excluded variables and return sorted list
    all_vars
        .into_iter()
        .filter(|name| !exclude_vars.contains(name))
        .collect()
}

/// Recursive helper to collect variable names
///
/// Traverses the AST and accumulates all variable names in a BTreeSet
/// (automatically sorted and deduplicated).
fn collect_variables_recursive(ast: &ASTNode, vars: &mut BTreeSet<String>) {
    match ast {
        ASTNode::Variable { name, .. } => {
            vars.insert(name.clone());
        }
        ASTNode::BinaryOp { left, right, .. } => {
            collect_variables_recursive(left, vars);
            collect_variables_recursive(right, vars);
        }
        ASTNode::UnaryOp { operand, .. } => {
            collect_variables_recursive(operand, vars);
        }
        ASTNode::Literal { .. } => {
            // Literals have no variables
        }
        // Phase 251 Fix: Handle complex condition expressions
        ASTNode::MethodCall { object, arguments, .. } => {
            // Recurse into object (e.g., 'arr' in 'arr.length()')
            collect_variables_recursive(object, vars);
            // Recurse into arguments (e.g., 'i' in 'arr.get(i)')
            for arg in arguments {
                collect_variables_recursive(arg, vars);
            }
        }
        ASTNode::FieldAccess { object, .. } => {
            // Recurse into object (e.g., 'obj' in 'obj.field')
            collect_variables_recursive(object, vars);
        }
        ASTNode::Index { target, index, .. } => {
            // Recurse into target (e.g., 'arr' in 'arr[i]')
            collect_variables_recursive(target, vars);
            // Recurse into index (e.g., 'i' in 'arr[i]')
            collect_variables_recursive(index, vars);
        }
        ASTNode::Call { callee, arguments, .. } => {
            // Recurse into callee (e.g., function references)
            collect_variables_recursive(callee, vars);
            // Recurse into arguments
            for arg in arguments {
                collect_variables_recursive(arg, vars);
            }
        }
        _ => {
            // Other AST nodes not expected in conditions
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, Span};

    #[test]
    fn test_extract_simple() {
        // AST: start < end
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "start".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &[]);
        assert_eq!(vars, vec!["end", "start"]); // Sorted order
    }

    #[test]
    fn test_extract_with_exclude() {
        // AST: i < end
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "end".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["end"]); // 'i' excluded
    }

    #[test]
    fn test_extract_complex() {
        // AST: start < end && i < len
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "start".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "end".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "len".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["end", "len", "start"]); // Sorted, 'i' excluded
    }

    #[test]
    fn test_extract_duplicates() {
        // AST: x < y && x < z (x appears twice)
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "y".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "z".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &[]);
        assert_eq!(vars, vec!["x", "y", "z"]); // 'x' deduplicated
    }

    #[test]
    fn test_extract_method_call() {
        // AST: i < arr.length()
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "arr".to_string(),
                    span: Span::unknown(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["arr"]); // Should extract 'arr'
    }

    #[test]
    fn test_extract_field_access() {
        // AST: i < obj.count
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::FieldAccess {
                object: Box::new(ASTNode::Variable {
                    name: "obj".to_string(),
                    span: Span::unknown(),
                }),
                field: "count".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["obj"]);
    }

    #[test]
    fn test_extract_index() {
        // AST: i < arr[j]
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Index {
                target: Box::new(ASTNode::Variable {
                    name: "arr".to_string(),
                    span: Span::unknown(),
                }),
                index: Box::new(ASTNode::Variable {
                    name: "j".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["arr", "j"]); // Both 'arr' and 'j' (sorted)
    }

    #[test]
    fn test_extract_complex_method_call_with_args() {
        // AST: i < arr.get(j)
        let ast = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "arr".to_string(),
                    span: Span::unknown(),
                }),
                method: "get".to_string(),
                arguments: vec![ASTNode::Variable {
                    name: "j".to_string(),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = extract_condition_variables(&ast, &["i".to_string()]);
        assert_eq!(vars, vec!["arr", "j"]); // Both 'arr' and 'j' (sorted)
    }
}

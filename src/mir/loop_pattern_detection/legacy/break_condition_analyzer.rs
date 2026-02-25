//! Break Condition Analysis
//!
//! Phase 33-23: Extracts break condition analysis logic from ast_feature_extractor.rs.
//! Responsible for:
//! - Extracting break conditions from if-else-break patterns
//! - Validating break statement structure
//! - Analyzing else clause content
//!
//! # Design Philosophy
//!
//! - **Pure functions**: No side effects, only AST analysis
//! - **Reusability**: Can be used by Pattern 2 and future break-based patterns
//! - **Testability**: Independent unit tests without MirBuilder context
//! - **Pattern detection**: Focus on structural analysis of break patterns

use crate::ast::{ASTNode, UnaryOperator};
use std::collections::HashSet;

pub struct BreakConditionAnalyzer;

impl BreakConditionAnalyzer {
    /// Extract break condition from if-else-break pattern
    ///
    /// Finds the condition used in the if statement that guards
    /// the break statement in the else clause.
    ///
    /// # Pattern Detection
    ///
    /// - Pattern 1: `if (cond) { break }` → returns `cond`
    /// - Pattern 2: `if (cond) { ... } else { break }` → returns `!cond` (negated)
    ///
    /// # Arguments
    ///
    /// * `body` - Loop body statements to search
    ///
    /// # Returns
    ///
    /// `Ok(&ASTNode)` - The condition AST node (may be negated for else-break)
    /// `Err(message)` - No if-else-break pattern found
    ///
    /// # Examples
    ///
    /// ```nyash
    /// // Pattern 1: if condition { break }
    /// loop(i < 3) {
    ///   if i >= 2 { break }  // Returns "i >= 2"
    ///   i = i + 1
    /// }
    ///
    /// // Pattern 2: if condition { ... } else { break }
    /// loop(start < end) {
    ///   if ch == " " { start = start + 1 } else { break }
    ///   // Returns "!(ch == " ")" (negated condition)
    /// }
    /// ```
    pub fn extract_break_condition(body: &[ASTNode]) -> Result<&ASTNode, String> {
        for stmt in body {
            if let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            {
                // Pattern 1: Check if the then_body contains a break statement
                if Self::has_break_in_stmts(then_body) {
                    return Ok(condition.as_ref());
                }

                // Pattern 2: Check if the else_body contains a break statement
                if let Some(else_stmts) = else_body {
                    if Self::has_break_in_stmts(else_stmts) {
                        // For else-break pattern, return the condition
                        // Note: Caller must negate this condition
                        return Ok(condition.as_ref());
                    }
                }
            }
        }
        Err("No if-else-break pattern found".to_string())
    }

    /// Extract a break condition as an owned AST node suitable for lowering.
    ///
    /// This returns the condition in the "break when <cond> is true" form:
    /// - `if cond { break }`                 -> `cond`
    /// - `if cond { ... } else { break }`   -> `!cond`
    ///
    /// Use this when the caller needs a normalized break condition without separately
    /// re-checking whether the break was in then/else.
    pub fn extract_break_condition_node(body: &[ASTNode]) -> Result<ASTNode, String> {
        for stmt in body {
            if let ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } = stmt
            {
                if Self::has_break_in_stmts(then_body) {
                    return Ok(condition.as_ref().clone());
                }
                if let Some(else_stmts) = else_body {
                    if Self::has_break_in_stmts(else_stmts) {
                        return Ok(Self::negate_condition(condition.as_ref()));
                    }
                }
            }
        }
        Err("No if-else-break pattern found".to_string())
    }

    /// Check if break exists in else clause
    ///
    /// Helper function to determine if a break statement is in the else clause
    /// of an if-else statement.
    ///
    /// # Arguments
    ///
    /// * `body` - Loop body statements to search
    ///
    /// # Returns
    ///
    /// `true` if an `if ... else { break }` pattern is found
    pub fn has_break_in_else_clause(body: &[ASTNode]) -> bool {
        for stmt in body {
            if let ASTNode::If {
                else_body: Some(else_body),
                ..
            } = stmt
            {
                if Self::has_break_in_stmts(else_body) {
                    return true;
                }
            }
        }
        false
    }

    /// Validate break condition structure
    ///
    /// Ensures the condition is well-formed for JoinIR lowering.
    ///
    /// # Arguments
    ///
    /// * `cond` - Condition AST node to validate
    ///
    /// # Returns
    ///
    /// Ok(()) if condition is valid, Err(message) otherwise
    ///
    /// # Supported Conditions
    ///
    /// - Literals (Integer, Bool, String, etc.)
    /// - Variables
    /// - Binary operations (comparison, arithmetic, logical)
    /// - Unary operations (not, negate)
    ///
    /// # Unsupported (for now)
    ///
    /// - Method calls (may be supported in future)
    pub fn validate_break_structure(cond: &ASTNode) -> Result<(), String> {
        match cond {
            ASTNode::Literal { .. } => Ok(()),
            ASTNode::Variable { name, .. } => {
                if name.is_empty() {
                    Err("Variable name is empty".to_string())
                } else {
                    Ok(())
                }
            }
            ASTNode::BinaryOp { .. } => Ok(()),
            ASTNode::UnaryOp { .. } => Ok(()),
            ASTNode::MethodCall { .. } => {
                Err("MethodCall in break condition not yet supported".to_string())
            }
            _ => Err(format!("Unsupported break condition type: {:?}", cond)),
        }
    }

    /// Extract all variables from break condition
    ///
    /// Recursively traverses the condition AST to collect all variable names.
    /// Useful for dependency analysis.
    ///
    /// # Arguments
    ///
    /// * `cond` - Condition AST node to analyze
    ///
    /// # Returns
    ///
    /// HashSet of variable names found in the condition
    ///
    /// # Example
    ///
    /// ```rust
    /// // Condition: x > 0 && y < 10
    /// // Returns: {"x", "y"}
    /// ```
    pub fn extract_condition_variables(cond: &ASTNode) -> HashSet<String> {
        let mut vars = HashSet::new();
        Self::collect_variables_recursive(cond, &mut vars);
        vars
    }

    /// Negate a condition AST node
    ///
    /// Wraps the condition in a UnaryOp::Not node.
    ///
    /// # Arguments
    ///
    /// * `cond` - Condition to negate
    ///
    /// # Returns
    ///
    /// New AST node representing !cond
    pub fn negate_condition(cond: &ASTNode) -> ASTNode {
        ASTNode::UnaryOp {
            operator: UnaryOperator::Not,
            operand: Box::new(cond.clone()),
            span: crate::ast::Span::unknown(),
        }
    }

    // Helper: Check if statements contain break (recursive)
    fn has_break_in_stmts(stmts: &[ASTNode]) -> bool {
        stmts.iter().any(Self::has_break_node)
    }

    fn has_break_node(node: &ASTNode) -> bool {
        match node {
            ASTNode::Break { .. } => true,
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                then_body.iter().any(Self::has_break_node)
                    || else_body
                        .as_ref()
                        .map_or(false, |e| e.iter().any(Self::has_break_node))
            }
            ASTNode::Loop { body, .. } => body.iter().any(Self::has_break_node),
            _ => false,
        }
    }

    // Helper: Recursively collect variables
    fn collect_variables_recursive(node: &ASTNode, vars: &mut HashSet<String>) {
        match node {
            ASTNode::Variable { name, .. } => {
                vars.insert(name.clone());
            }
            ASTNode::BinaryOp { left, right, .. } => {
                Self::collect_variables_recursive(left, vars);
                Self::collect_variables_recursive(right, vars);
            }
            ASTNode::UnaryOp { operand, .. } => {
                Self::collect_variables_recursive(operand, vars);
            }
            ASTNode::MethodCall {
                object, arguments, ..
            } => {
                Self::collect_variables_recursive(object, vars);
                for arg in arguments {
                    Self::collect_variables_recursive(arg, vars);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_has_break_in_else_clause() {
        // Create: if (cond) { ... } else { break }
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        };

        assert!(BreakConditionAnalyzer::has_break_in_else_clause(&[if_stmt]));
    }

    #[test]
    fn test_has_break_in_else_clause_negative() {
        // Create: if (cond) { break }
        let if_stmt = ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        };

        assert!(!BreakConditionAnalyzer::has_break_in_else_clause(&[
            if_stmt
        ]));
    }

    #[test]
    fn test_extract_break_condition_then_branch() {
        // if (x) { break }
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }];

        let result = BreakConditionAnalyzer::extract_break_condition(&body);
        assert!(result.is_ok());
        // Result should be the variable "x"
        if let ASTNode::Variable { name, .. } = result.unwrap() {
            assert_eq!(name, "x");
        } else {
            panic!("Expected Variable node");
        }
    }

    #[test]
    fn test_extract_break_condition_else_branch() {
        // if (x) { ... } else { break }
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let result = BreakConditionAnalyzer::extract_break_condition(&body);
        assert!(result.is_ok());
        // Result should be the variable "x" (caller must negate)
    }

    #[test]
    fn test_extract_break_condition_not_found() {
        // No break statement
        let body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = BreakConditionAnalyzer::extract_break_condition(&body);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_break_condition_node_then_break_returns_cond() {
        let condition = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::If {
            condition: Box::new(condition.clone()),
            then_body: vec![ASTNode::Break {
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }];

        let result = BreakConditionAnalyzer::extract_break_condition_node(&body).unwrap();
        assert!(matches!(result, ASTNode::Variable { name, .. } if name == "x"));
    }

    #[test]
    fn test_extract_break_condition_node_else_break_returns_not_cond() {
        let condition = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };
        let body = vec![ASTNode::If {
            condition: Box::new(condition.clone()),
            then_body: vec![],
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let result = BreakConditionAnalyzer::extract_break_condition_node(&body).unwrap();
        match result {
            ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } => {
                assert!(matches!(*operand, ASTNode::Variable { name, .. } if name == "x"));
            }
            other => panic!("expected UnaryOp::Not, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_break_structure_valid() {
        let var = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };
        assert!(BreakConditionAnalyzer::validate_break_structure(&var).is_ok());

        let binary = ASTNode::BinaryOp {
            operator: BinaryOperator::Greater,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        assert!(BreakConditionAnalyzer::validate_break_structure(&binary).is_ok());
    }

    #[test]
    fn test_validate_break_structure_invalid() {
        let method_call = ASTNode::MethodCall {
            object: Box::new(ASTNode::Variable {
                name: "s".to_string(),
                span: Span::unknown(),
            }),
            method: "length".to_string(),
            arguments: vec![],
            span: Span::unknown(),
        };
        assert!(BreakConditionAnalyzer::validate_break_structure(&method_call).is_err());
    }

    #[test]
    fn test_extract_condition_variables() {
        // Create: x || y
        let or_expr = ASTNode::BinaryOp {
            operator: BinaryOperator::Or,
            left: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "y".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = BreakConditionAnalyzer::extract_condition_variables(&or_expr);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_extract_condition_variables_nested() {
        // Create: (x > 0) && (y < 10)
        let complex_expr = ASTNode::BinaryOp {
            operator: BinaryOperator::And,
            left: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "y".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let vars = BreakConditionAnalyzer::extract_condition_variables(&complex_expr);
        assert!(vars.contains("x"));
        assert!(vars.contains("y"));
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_negate_condition() {
        let var = ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        };

        let negated = BreakConditionAnalyzer::negate_condition(&var);

        // Should be wrapped in UnaryOp::Not
        if let ASTNode::UnaryOp {
            operator, operand, ..
        } = negated
        {
            assert!(matches!(operator, UnaryOperator::Not));
            if let ASTNode::Variable { name, .. } = *operand {
                assert_eq!(name, "x");
            } else {
                panic!("Expected Variable operand");
            }
        } else {
            panic!("Expected UnaryOp node");
        }
    }
}

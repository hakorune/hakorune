//! Phase 100 P2-1: Mutable Accumulator Analyzer
//!
//! Pure AST box for detecting accumulator patterns (s = s + x form only).
//! This analyzer ONLY detects the shape from AST, without checking:
//! - Whether target is loop-outer or loop-local (delegated to ScopeManager)
//! - Whether RHS is read-only (delegated to ScopeManager)
//!
//! Minimal spec: `target = target + x` where x ∈ {Variable, Literal}
//!
//! Phase 100 P3-1: Added AccumulatorKind::{Int, String} for type-based routing

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// RHS expression kind in accumulator pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RhsExprKind {
    /// Variable reference (e.g., "ch")
    Var,
    /// Literal value (e.g., "\"hello\"" or "42")
    Literal,
}

/// Accumulator kind (Phase 100 P3-1: Int vs String)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccumulatorKind {
    /// Integer accumulator (e.g., count = count + 1)
    Int,
    /// String accumulator (e.g., out = out + ch)
    String,
}

/// Accumulator specification detected from AST
#[derive(Debug, Clone, PartialEq)]
pub struct MutableAccumulatorSpec {
    /// Target variable name (e.g., "out")
    pub target_name: String,
    /// RHS expression kind
    pub rhs_expr_kind: RhsExprKind,
    /// RHS variable name or literal string representation
    pub rhs_var_or_lit: String,
    /// Binary operator (currently only Add supported)
    pub op: BinaryOperator,
    /// Accumulator kind (Phase 100 P3-1)
    pub kind: AccumulatorKind,
}

/// Analyzer for detecting accumulator patterns in loop body AST.
///
/// # Responsibility
///
/// Detects accumulator **shape** from AST only. Does NOT check:
/// - Whether target is loop-outer or loop-local
/// - Whether RHS is read-only
///
/// These checks are delegated to Pattern2 (ScopeManager).
pub struct MutableAccumulatorAnalyzer;

impl MutableAccumulatorAnalyzer {
    /// Analyzes loop body AST to detect accumulator pattern.
    ///
    /// # Arguments
    ///
    /// * `loop_body` - AST nodes of the loop body
    ///
    /// # Returns
    ///
    /// * `Ok(Some(spec))` - Accumulator pattern detected (target = target + x)
    /// * `Ok(None)` - No accumulator pattern found (other patterns may apply)
    /// * `Err(String)` - Internal consistency error (should be rare)
    ///
    /// # Detection Strategy (Phase 253)
    ///
    /// Returns `Ok(None)` for non-accumulator patterns:
    /// - Multiple assignments to the same variable
    /// - Operator not Add (e.g., i = i - 1)
    /// - Left operand not matching target (e.g., x = y + x)
    /// - RHS contains function call (e.g., x = x + f())
    /// - RHS is complex expression (e.g., x = x + (i + 1))
    /// - Value is not BinaryOp (e.g., i = s.length())
    ///
    /// This allows other loop lowering patterns to handle non-accumulator cases.
    pub fn analyze(loop_body: &[ASTNode]) -> Result<Option<MutableAccumulatorSpec>, String> {
        // Collect all assignments in loop body
        let assignments = collect_assignments(loop_body);

        if assignments.is_empty() {
            return Ok(None); // No assignments, no accumulator
        }

        // Check for multiple assignments to the same variable
        // Note: We're looking for ONE accumulator variable with ONE assignment
        // If there are multiple assignments to the same variable, it's not the simple
        // accumulator pattern we support, so return None (not an error)
        let mut assignment_counts = std::collections::BTreeMap::new();
        for (target_name, _, _) in &assignments {
            *assignment_counts.entry(target_name.clone()).or_insert(0) += 1;
        }

        // If any variable has multiple assignments, this is not the simple accumulator
        // pattern we're looking for. Return None to allow other patterns to handle it.
        for (_target_name, count) in &assignment_counts {
            if *count > 1 {
                return Ok(None); // Not our pattern, let other code handle it
            }
        }

        // Try to find accumulator pattern: target = target + x
        for (target_name, value_node, _span) in assignments {
            // Check if value is BinaryOp
            if let ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } = value_node
            {
                // Phase 253: Check operator is Add (return Ok(None) if not)
                // Only Add is supported as accumulator pattern
                if operator != &BinaryOperator::Add {
                    return Ok(None); // Not our pattern (e.g., i = i - 1)
                }

                // Check left operand is Variable with same name as target
                if let ASTNode::Variable { name, .. } = left.as_ref() {
                    if name == &target_name {
                        // Check RHS is Variable or Literal (no MethodCall)
                        match right.as_ref() {
                            ASTNode::Variable { name: rhs_name, .. } => {
                                // Phase 100 P3-1: Variable RHS - default to Int for backward compat
                                // Actual type validation happens in Pattern2 wiring (P3-3)
                                return Ok(Some(MutableAccumulatorSpec {
                                    target_name,
                                    rhs_expr_kind: RhsExprKind::Var,
                                    rhs_var_or_lit: rhs_name.clone(),
                                    op: BinaryOperator::Add,
                                    kind: AccumulatorKind::Int, // Will be refined in P3-3
                                }));
                            }
                            ASTNode::Literal { value, .. } => {
                                // Phase 100 P3-1: Detect kind from literal type
                                let kind = match value {
                                    LiteralValue::String(_) => AccumulatorKind::String,
                                    LiteralValue::Integer(_) => AccumulatorKind::Int,
                                    _ => AccumulatorKind::Int, // Default to Int for other literals
                                };
                                return Ok(Some(MutableAccumulatorSpec {
                                    target_name,
                                    rhs_expr_kind: RhsExprKind::Literal,
                                    rhs_var_or_lit: format!("{:?}", value),
                                    op: BinaryOperator::Add,
                                    kind,
                                }));
                            }
                            ASTNode::MethodCall { .. } | ASTNode::Call { .. } => {
                                // Phase 253: RHS with function call is not accumulator pattern
                                return Ok(None); // Not our pattern (e.g., x = x + f())
                            }
                            _ => {
                                // Phase 253: Complex RHS expression is not accumulator pattern
                                return Ok(None); // Not our pattern (e.g., x = x + (i + 1))
                            }
                        }
                    } else {
                        // Phase 253: Left operand is different variable (reversed operands)
                        return Ok(None); // Not our pattern (e.g., x = y + x)
                    }
                } else {
                    // Phase 253: Left operand is not Variable
                    return Ok(None); // Not our pattern
                }
            } else {
                // Phase 253: Value is not BinaryOp - not accumulator pattern
                // e.g., i = s.length() - 1 (MethodCall result, not x = x + y)
                return Ok(None); // Not our pattern
            }
        }

        Ok(None) // No accumulator pattern found
    }
}

/// Collects all assignments in loop body AST.
///
/// Returns: Vec<(target_name, value_node, span)>
fn collect_assignments<'a>(nodes: &'a [ASTNode]) -> Vec<(String, &'a ASTNode, crate::ast::Span)> {
    let mut assignments = Vec::new();
    for node in nodes {
        collect_assignments_in_node(node, &mut assignments);
    }
    assignments
}

/// Recursively collects assignments from a single AST node.
fn collect_assignments_in_node<'a>(
    node: &'a ASTNode,
    assignments: &mut Vec<(String, &'a ASTNode, crate::ast::Span)>,
) {
    match node {
        ASTNode::Assignment {
            target, value, span, ..
        } => {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                assignments.push((name.clone(), value.as_ref(), span.clone()));
            }
        }
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            for node in then_body {
                collect_assignments_in_node(node, assignments);
            }
            if let Some(else_stmts) = else_body {
                for node in else_stmts {
                    collect_assignments_in_node(node, assignments);
                }
            }
        }
        ASTNode::Loop { body, .. } => {
            for node in body {
                collect_assignments_in_node(node, assignments);
            }
        }
        // Other nodes don't contain assignments
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    #[test]
    fn test_mutable_accumulator_spec_simple() {
        // Build AST for: out = out + ch
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "out".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body).unwrap();
        assert!(result.is_some());

        let spec = result.unwrap();
        assert_eq!(spec.target_name, "out");
        assert_eq!(spec.rhs_expr_kind, RhsExprKind::Var);
        assert_eq!(spec.rhs_var_or_lit, "ch");
        assert_eq!(spec.op, BinaryOperator::Add);
        assert_eq!(spec.kind, AccumulatorKind::Int); // Phase 100 P3-1: Variable defaults to Int
    }

    #[test]
    fn test_mutable_accumulator_spec_ng_reversed() {
        // Phase 253: Build AST for: out = ch + out (reversed operands)
        // Expected: Ok(None) - not accumulator pattern
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "out".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Reversed operands should return None");
    }

    #[test]
    fn test_mutable_accumulator_spec_ng_multiple() {
        // Build AST for: out = out + ch; out = out + "x"
        // Expected: Ok(None) because multiple assignments means not our simple pattern
        let loop_body = vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "out".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Variable {
                        name: "ch".to_string(),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "out".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::String("x".to_string()),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
        ];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Multiple assignments should return None, not error");
    }

    #[test]
    fn test_mutable_accumulator_spec_ng_method_call() {
        // Phase 253: Build AST for: out = out + f()
        // Expected: Ok(None) - RHS contains function call
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "out".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::MethodCall {
                    object: Box::new(ASTNode::Variable {
                        name: "obj".to_string(),
                        span: Span::unknown(),
                    }),
                    method: "f".to_string(),
                    arguments: vec![],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "RHS with function call should return None");
    }

    #[test]
    fn test_no_accumulator_when_no_assignment() {
        // Build AST for: local x = 1
        let loop_body = vec![ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_accumulator_with_literal_rhs() {
        // Build AST for: count = count + 1
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "count".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "count".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body).unwrap();
        assert!(result.is_some());

        let spec = result.unwrap();
        assert_eq!(spec.target_name, "count");
        assert_eq!(spec.rhs_expr_kind, RhsExprKind::Literal);
        assert_eq!(spec.op, BinaryOperator::Add);
        assert_eq!(spec.kind, AccumulatorKind::Int); // Phase 100 P3-1: Integer literal
    }

    #[test]
    fn test_string_accumulator_spec() {
        // Phase 100 P3-1: String accumulator with Variable RHS
        // Build AST for: out = out + ch (ch is string-typed variable)
        // Note: At AST-only stage, Variable defaults to Int
        // Type refinement happens in Pattern2 wiring (P3-3)
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "out".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "out".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body).unwrap();
        assert!(result.is_some());

        let spec = result.unwrap();
        assert_eq!(spec.target_name, "out");
        assert_eq!(spec.rhs_expr_kind, RhsExprKind::Var);
        assert_eq!(spec.rhs_var_or_lit, "ch");
        assert_eq!(spec.op, BinaryOperator::Add);
        // Phase 100 P3-1: Variable RHS defaults to Int at AST stage
        // Will be refined to String in P3-3 based on actual types
        assert_eq!(spec.kind, AccumulatorKind::Int);
    }

    #[test]
    fn test_int_accumulator_spec_unchanged() {
        // Phase 100 P3-1: Integer accumulator (existing behavior)
        // Build AST for: count = count + 1
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "count".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "count".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body).unwrap();
        assert!(result.is_some());

        let spec = result.unwrap();
        assert_eq!(spec.target_name, "count");
        assert_eq!(spec.rhs_expr_kind, RhsExprKind::Literal);
        assert_eq!(spec.op, BinaryOperator::Add);
        assert_eq!(spec.kind, AccumulatorKind::Int);
    }

    #[test]
    fn test_decrement_not_accumulator() {
        // Phase 253: Build AST for: i = i - 1
        // Expected: Ok(None) - decrement is not accumulator pattern (only Add supported)
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Decrement (i = i - 1) should return None");
    }

    #[test]
    fn test_complex_rhs_not_accumulator() {
        // Phase 253: Build AST for: x = x + (i + 1)
        // Expected: Ok(None) - complex RHS expression
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Complex RHS (x = x + (i + 1)) should return None");
    }

    #[test]
    fn test_non_binop_assignment_not_accumulator() {
        // Phase 253: Build AST for: i = s.length()
        // Expected: Ok(None) - not BinaryOp (MethodCall result)
        let loop_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: "s".to_string(),
                    span: Span::unknown(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }];

        let result = MutableAccumulatorAnalyzer::analyze(&loop_body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "Non-BinaryOp assignment (i = s.length()) should return None");
    }
}

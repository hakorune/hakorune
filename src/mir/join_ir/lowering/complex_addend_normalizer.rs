//! Phase 192: Complex Addend Normalizer
//!
//! Normalizes complex addend patterns in loop carrier updates to simpler forms
//! that can be handled by NumberAccumulation pattern.
//!
//! ## Purpose
//!
//! Transforms patterns like:
//! ```nyash
//! result = result * 10 + digits.indexOf(ch)
//! ```
//!
//! Into:
//! ```nyash
//! local temp_result_addend = digits.indexOf(ch)
//! result = result * 10 + temp_result_addend
//! ```
//!
//! This allows the NumberAccumulation pattern (Phase 190) to handle the update
//! after the complex addend has been extracted to a body-local variable.
//!
//! ## Design Philosophy (箱理論 - Box Theory)
//!
//! - **Single Responsibility**: Only handles AST normalization (no emission)
//! - **Clear Boundaries**: Operates on AST, outputs normalized AST
//! - **Fail-Fast**: Unsupported patterns → explicit error
//! - **Reusability**: Works with any complex addend pattern
//!
//! ## Integration
//!
//! This normalizer is a **preprocessing step** that runs before:
//! - LoopUpdateAnalyzer (re-analysis after normalization)
//! - LoopBodyLocalInitLowerer (Phase 191 - handles temp initialization)
//! - CarrierUpdateLowerer (Phase 190 - emits NumberAccumulation)

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Result of complex addend normalization
#[derive(Debug, Clone)]
pub enum NormalizationResult {
    /// No normalization needed (already simple pattern)
    Unchanged,

    /// Successfully normalized to temp variable
    Normalized {
        /// Temporary variable definition: `local temp = <complex_expr>`
        temp_def: ASTNode,

        /// Normalized assignment: `lhs = lhs * base + temp`
        new_assign: ASTNode,

        /// Name of temporary variable
        temp_name: String,
    },
}

/// Complex addend pattern normalizer
pub struct ComplexAddendNormalizer;

impl ComplexAddendNormalizer {
    /// Normalize assignment with complex addend pattern
    ///
    /// Detects pattern: `lhs = lhs * base + <complex_expr>`
    /// where `<complex_expr>` is a MethodCall or complex BinaryOp.
    ///
    /// # Arguments
    ///
    /// * `assign` - Assignment AST node to analyze
    ///
    /// # Returns
    ///
    /// * `NormalizationResult::Normalized` - Successfully normalized with temp variable
    /// * `NormalizationResult::Unchanged` - Not a complex addend pattern (no action needed)
    ///
    /// # Example
    ///
    /// ```ignore
    /// let assign = parse("result = result * 10 + digits.indexOf(ch)");
    /// match ComplexAddendNormalizer::normalize_assign(&assign) {
    ///     NormalizationResult::Normalized { temp_def, new_assign, temp_name } => {
    ///         // temp_def: local temp_result_addend = digits.indexOf(ch)
    ///         // new_assign: result = result * 10 + temp_result_addend
    ///         // temp_name: "temp_result_addend"
    ///     }
    ///     NormalizationResult::Unchanged => {
    ///         // Not a complex pattern, use original assignment
    ///     }
    /// }
    /// ```
    pub fn normalize_assign(assign: &ASTNode) -> NormalizationResult {
        // Extract assignment components
        let (target, value) = match assign {
            ASTNode::Assignment { target, value, .. } => (target, value),
            _ => return NormalizationResult::Unchanged,
        };

        // Extract LHS variable name
        let lhs_name = match target.as_ref() {
            ASTNode::Variable { name, .. } => name.clone(),
            _ => return NormalizationResult::Unchanged,
        };

        // Check RHS structure: lhs * base + addend
        let (base, addend) = match value.as_ref() {
            ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left,
                right,
                ..
            }
            | ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left,
                right,
                ..
            } => {
                // Left side should be: lhs * base
                match left.as_ref() {
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Multiply,
                        left: mul_left,
                        right: mul_right,
                        ..
                    } => {
                        // Check if multiplication LHS matches assignment LHS
                        if let ASTNode::Variable { name, .. } = mul_left.as_ref() {
                            if name == &lhs_name {
                                // Extract base (must be constant)
                                if let ASTNode::Literal {
                                    value: LiteralValue::Integer(base_val),
                                    ..
                                } = mul_right.as_ref()
                                {
                                    // Check if addend (right side) is complex
                                    if Self::is_complex_expr(right) {
                                        (*base_val, right.as_ref())
                                    } else {
                                        return NormalizationResult::Unchanged;
                                    }
                                } else {
                                    return NormalizationResult::Unchanged;
                                }
                            } else {
                                return NormalizationResult::Unchanged;
                            }
                        } else {
                            return NormalizationResult::Unchanged;
                        }
                    }
                    _ => return NormalizationResult::Unchanged,
                }
            }
            _ => return NormalizationResult::Unchanged,
        };

        // Generate temp variable name
        let temp_name = format!("temp_{}_addend", lhs_name);

        // Create temp definition: local temp = <complex_expr>
        let temp_def = ASTNode::Local {
            variables: vec![temp_name.clone()],
            initial_values: vec![Some(Box::new(addend.clone()))],
            span: crate::ast::Span::unknown(),
        };

        // Create normalized assignment: lhs = lhs * base + temp
        let span = crate::ast::Span::unknown();
        let new_assign = ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: lhs_name.clone(),
                span: span.clone(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: Box::new(ASTNode::Variable {
                        name: lhs_name.clone(),
                        span: span.clone(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(base),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: temp_name.clone(),
                    span: span.clone(),
                }),
                span: span.clone(),
            }),
            span,
        };

        NormalizationResult::Normalized {
            temp_def,
            new_assign,
            temp_name,
        }
    }

    /// Check if expression is "complex" (requires normalization)
    ///
    /// Complex expressions:
    /// - MethodCall (e.g., `digits.indexOf(ch)`)
    /// - Call (e.g., `parseInt(s)`)
    /// - Nested BinaryOp (e.g., `a + b * c`)
    ///
    /// Simple expressions (no normalization needed):
    /// - Variable (e.g., `digit`)
    /// - Literal (e.g., `5`)
    fn is_complex_expr(expr: &ASTNode) -> bool {
        matches!(
            expr,
            ASTNode::MethodCall { .. } | ASTNode::Call { .. } | ASTNode::BinaryOp { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    /// Helper: Create variable AST node
    fn var(name: &str) -> Box<ASTNode> {
        Box::new(ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        })
    }

    /// Helper: Create integer literal AST node
    fn int(value: i64) -> Box<ASTNode> {
        Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        })
    }

    /// Helper: Create method call AST node
    fn method_call(receiver: &str, method: &str, args: Vec<Box<ASTNode>>) -> Box<ASTNode> {
        Box::new(ASTNode::MethodCall {
            object: var(receiver),
            method: method.to_string(),
            arguments: args.into_iter().map(|b| *b).collect(),
            span: Span::unknown(),
        })
    }

    #[test]
    fn test_normalize_complex_addend_method_call() {
        // Pattern: result = result * 10 + digits.indexOf(ch)
        let assign = ASTNode::Assignment {
            target: var("result"),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: var("result"),
                    right: int(10),
                    span: Span::unknown(),
                }),
                right: method_call("digits", "indexOf", vec![var("ch")]),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        match ComplexAddendNormalizer::normalize_assign(&assign) {
            NormalizationResult::Normalized {
                temp_def,
                new_assign,
                temp_name,
            } => {
                assert_eq!(temp_name, "temp_result_addend");

                // Verify temp_def structure
                if let ASTNode::Local {
                    variables,
                    initial_values,
                    ..
                } = temp_def
                {
                    assert_eq!(variables.len(), 1);
                    assert_eq!(variables[0], "temp_result_addend");
                    assert!(initial_values[0].is_some());
                } else {
                    panic!("Expected Local node for temp_def");
                }

                // Verify new_assign structure
                if let ASTNode::Assignment { target, value, .. } = new_assign {
                    // Target should be "result"
                    if let ASTNode::Variable { name, .. } = target.as_ref() {
                        assert_eq!(name, "result");
                    } else {
                        panic!("Expected Variable for assignment target");
                    }

                    // Value should be: result * 10 + temp_result_addend
                    if let ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if let ASTNode::Variable { name, .. } = right.as_ref() {
                            assert_eq!(name, "temp_result_addend");
                        } else {
                            panic!("Expected Variable for addend");
                        }
                    } else {
                        panic!("Expected BinaryOp Add for assignment value");
                    }
                } else {
                    panic!("Expected Assignment node for new_assign");
                }
            }
            NormalizationResult::Unchanged => {
                panic!("Expected Normalized, got Unchanged");
            }
        }
    }

    #[test]
    fn test_normalize_simple_variable_unchanged() {
        // Pattern: result = result * 10 + digit (simple variable - no normalization)
        let assign = ASTNode::Assignment {
            target: var("result"),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: var("result"),
                    right: int(10),
                    span: Span::unknown(),
                }),
                right: var("digit"),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        match ComplexAddendNormalizer::normalize_assign(&assign) {
            NormalizationResult::Unchanged => {
                // Expected - simple variable doesn't need normalization
            }
            NormalizationResult::Normalized { .. } => {
                panic!("Expected Unchanged for simple variable pattern");
            }
        }
    }

    #[test]
    fn test_normalize_wrong_lhs_unchanged() {
        // Pattern: result = other * 10 + digits.indexOf(ch) (wrong LHS - no match)
        let assign = ASTNode::Assignment {
            target: var("result"),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: var("other"), // Wrong variable!
                    right: int(10),
                    span: Span::unknown(),
                }),
                right: method_call("digits", "indexOf", vec![var("ch")]),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        match ComplexAddendNormalizer::normalize_assign(&assign) {
            NormalizationResult::Unchanged => {
                // Expected - LHS mismatch means not a valid pattern
            }
            NormalizationResult::Normalized { .. } => {
                panic!("Expected Unchanged for wrong LHS pattern");
            }
        }
    }

    #[test]
    fn test_normalize_no_multiplication_unchanged() {
        // Pattern: result = result + digits.indexOf(ch) (no multiplication - no match)
        let assign = ASTNode::Assignment {
            target: var("result"),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: var("result"),
                right: method_call("digits", "indexOf", vec![var("ch")]),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        match ComplexAddendNormalizer::normalize_assign(&assign) {
            NormalizationResult::Unchanged => {
                // Expected - no multiplication means not NumberAccumulation pattern
            }
            NormalizationResult::Normalized { .. } => {
                panic!("Expected Unchanged for pattern without multiplication");
            }
        }
    }

    #[test]
    fn test_normalize_subtraction_complex_addend() {
        // Pattern: result = result * 10 - digits.indexOf(ch)
        // Should also normalize (subtraction variant)
        let assign = ASTNode::Assignment {
            target: var("result"),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Subtract,
                left: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Multiply,
                    left: var("result"),
                    right: int(10),
                    span: Span::unknown(),
                }),
                right: method_call("digits", "indexOf", vec![var("ch")]),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        match ComplexAddendNormalizer::normalize_assign(&assign) {
            NormalizationResult::Normalized { temp_name, .. } => {
                assert_eq!(temp_name, "temp_result_addend");
            }
            NormalizationResult::Unchanged => {
                panic!("Expected Normalized for subtraction variant");
            }
        }
    }
}

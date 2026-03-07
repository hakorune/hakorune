//! Phase 224-E: DigitPos Condition Normalizer Box
//!
//! Transforms digit_pos comparison patterns to boolean carrier expressions.
//!
//! ## Problem
//!
//! After DigitPosPromoter promotes `digit_pos: i32` to `is_digit_pos: bool`,
//! the break condition AST still contains `digit_pos < 0`, which causes type errors
//! when the alias resolves to `Bool(is_digit_pos) < Integer(0)`.
//!
//! ## Solution
//!
//! Transform the condition AST before lowering:
//!
//! ```text
//! digit_pos < 0  →  !is_digit_pos
//! ```
//!
//! ## Design Principles
//!
//! - **Single Responsibility**: Only handles AST transformation
//! - **Fail-Safe**: Non-matching patterns returned unchanged
//! - **Stateless**: Pure function with no side effects

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::runtime::get_global_ring0;

/// Phase 224-E: DigitPos Condition Normalizer Box
pub struct DigitPosConditionNormalizer;

impl DigitPosConditionNormalizer {
    /// Normalize digit_pos condition AST
    ///
    /// Transforms: `<promoted_var> < 0` → `!<carrier_name>`
    ///
    /// # Shape Matching
    ///
    /// Matches:
    /// - BinaryOp with operator Lt (Less than)
    /// - Left operand is Var(promoted_var)
    /// - Right operand is Const(0)
    ///
    /// Transforms to:
    /// - UnaryOp { op: Not, expr: Var(carrier_name) }
    ///
    /// # Arguments
    ///
    /// * `cond` - Break/continue condition AST
    /// * `promoted_var` - Original variable name (e.g., "digit_pos")
    /// * `carrier_name` - Promoted carrier name (e.g., "is_digit_pos")
    ///
    /// # Returns
    ///
    /// Normalized AST (or original if the shape doesn't match)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let original = parse("digit_pos < 0");
    /// let normalized = DigitPosConditionNormalizer::normalize(
    ///     &original,
    ///     "digit_pos",
    ///     "is_digit_pos",
    /// );
    /// // normalized is equivalent to parse("!is_digit_pos")
    /// ```
    pub fn normalize(cond: &ASTNode, promoted_var: &str, carrier_name: &str) -> ASTNode {
        // Shape: BinaryOp { op: Lt, lhs: Var(promoted_var), rhs: Const(0) }
        match cond {
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                span,
            } => {
                // Check operator is Lt (Less than)
                if *operator != BinaryOperator::Less {
                    return cond.clone();
                }

                // Check left operand is Var(promoted_var)
                let is_promoted_var = match left.as_ref() {
                    ASTNode::Variable { name, .. } => name == promoted_var,
                    _ => false,
                };
                if !is_promoted_var {
                    return cond.clone();
                }

                // Check right operand is Const(0)
                let is_zero_literal = match right.as_ref() {
                    ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        ..
                    } => true,
                    _ => false,
                };
                if !is_zero_literal {
                    return cond.clone();
                }

                // Shape matched: transform to !carrier_name
                if crate::config::env::joinir_dev::debug_enabled() {
                    get_global_ring0().log.debug(&format!(
                        "[digitpos_normalizer] Transforming '{}' < 0 → !'{}'",
                        promoted_var, carrier_name
                    ));
                }

                ASTNode::UnaryOp {
                    operator: UnaryOperator::Not,
                    operand: Box::new(ASTNode::Variable {
                        name: carrier_name.to_string(),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                }
            }
            _ => {
                // Not a binary operation, return unchanged
                cond.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn var_node(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }

    fn int_literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn binary_op(left: ASTNode, op: BinaryOperator, right: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(left),
            right: Box::new(right),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_normalize_digit_pos_lt_zero() {
        // Case: digit_pos < 0
        let cond = binary_op(var_node("digit_pos"), BinaryOperator::Less, int_literal(0));

        let normalized = DigitPosConditionNormalizer::normalize(&cond, "digit_pos", "is_digit_pos");

        // Should transform to: !is_digit_pos
        match normalized {
            ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } => match operand.as_ref() {
                ASTNode::Variable { name, .. } => {
                    assert_eq!(name, "is_digit_pos");
                }
                _ => panic!("Expected Variable node"),
            },
            _ => panic!("Expected UnaryOp Not node"),
        }
    }

    #[test]
    fn test_no_normalize_wrong_operator() {
        // Case: digit_pos >= 0 (Greater or equal, not Less)
        let cond = binary_op(
            var_node("digit_pos"),
            BinaryOperator::GreaterEqual,
            int_literal(0),
        );

        let normalized = DigitPosConditionNormalizer::normalize(&cond, "digit_pos", "is_digit_pos");

        // Should NOT transform (return original)
        match normalized {
            ASTNode::BinaryOp { operator, .. } => {
                assert_eq!(operator, BinaryOperator::GreaterEqual);
            }
            _ => panic!("Expected unchanged BinaryOp node"),
        }
    }

    #[test]
    fn test_no_normalize_wrong_variable() {
        // Case: other_var < 0 (different variable name)
        let cond = binary_op(var_node("other_var"), BinaryOperator::Less, int_literal(0));

        let normalized = DigitPosConditionNormalizer::normalize(&cond, "digit_pos", "is_digit_pos");

        // Should NOT transform (return original)
        match normalized {
            ASTNode::BinaryOp { operator, left, .. } => {
                assert_eq!(operator, BinaryOperator::Less);
                match left.as_ref() {
                    ASTNode::Variable { name, .. } => {
                        assert_eq!(name, "other_var");
                    }
                    _ => panic!("Expected Variable node"),
                }
            }
            _ => panic!("Expected unchanged BinaryOp node"),
        }
    }

    #[test]
    fn test_no_normalize_wrong_constant() {
        // Case: digit_pos < 10 (different constant, not 0)
        let cond = binary_op(var_node("digit_pos"), BinaryOperator::Less, int_literal(10));

        let normalized = DigitPosConditionNormalizer::normalize(&cond, "digit_pos", "is_digit_pos");

        // Should NOT transform (return original)
        match normalized {
            ASTNode::BinaryOp {
                operator, right, ..
            } => {
                assert_eq!(operator, BinaryOperator::Less);
                match right.as_ref() {
                    ASTNode::Literal {
                        value: LiteralValue::Integer(val),
                        ..
                    } => {
                        assert_eq!(*val, 10);
                    }
                    _ => panic!("Expected Integer literal"),
                }
            }
            _ => panic!("Expected unchanged BinaryOp node"),
        }
    }

    #[test]
    fn test_no_normalize_non_binary_op() {
        // Case: just a variable (not a binary operation)
        let cond = var_node("digit_pos");

        let normalized = DigitPosConditionNormalizer::normalize(&cond, "digit_pos", "is_digit_pos");

        // Should NOT transform (return original)
        match normalized {
            ASTNode::Variable { name, .. } => {
                assert_eq!(name, "digit_pos");
            }
            _ => panic!("Expected unchanged Variable node"),
        }
    }
}

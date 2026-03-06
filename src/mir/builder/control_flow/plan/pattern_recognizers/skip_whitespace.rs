//! Skip Whitespace Route-Shape Detection
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module detects skip_whitespace and trim route shapes.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Skip whitespace route-shape information
///
/// This struct holds the extracted information from a recognized skip_whitespace route shape.
#[derive(Debug, Clone, PartialEq)]
pub struct SkipWhitespaceInfo {
    /// Carrier variable name (e.g., "p")
    pub carrier_name: String,
    /// Constant step increment (e.g., 1 for `p = p + 1`)
    pub delta: i64,
    /// Body statements before the if-else (may be empty)
    pub body_stmts: Vec<ASTNode>,
}

/// Detect a skip_whitespace / trim leading/trailing route shape in the loop body
///
/// Phase 142 P0: Generalized to handle both +1 and -1 route shapes
///
/// Shape structure:
/// ```
/// loop(cond) {
///     // ... optional body statements (Body)
///     if check_cond {
///         carrier = carrier (+|-) const
///     } else {
///         break
///     }
/// }
/// ```
///
/// Recognized route shapes:
/// - skip_whitespace: `p < len`, `p = p + 1`
/// - trim_leading: `start < end`, `start = start + 1`
/// - trim_trailing: `end > start`, `end = end - 1`
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `Some(SkipWhitespaceInfo)` if the shape matches, `None` otherwise
///
/// # Notes
///
/// This is the SSOT for skip_whitespace/trim route-shape detection.
/// Used by both loop_canonicalizer (Phase 137) and future route-shape analyzers.
pub fn detect_skip_whitespace_pattern(body: &[ASTNode]) -> Option<SkipWhitespaceInfo> {
    if body.is_empty() {
        return None;
    }

    // Last statement must be if-else with break
    let last_stmt = &body[body.len() - 1];

    let (then_body, else_body) = match last_stmt {
        ASTNode::If {
            then_body,
            else_body: Some(else_body),
            ..
        } => (then_body, else_body),
        _ => return None,
    };

    // Then branch must be single assignment: carrier = carrier (+|-) const
    if then_body.len() != 1 {
        return None;
    }

    let (carrier_name, delta) = match &then_body[0] {
        ASTNode::Assignment { target, value, .. } => {
            // Extract target variable name
            let target_name = match target.as_ref() {
                ASTNode::Variable { name, .. } => name.clone(),
                _ => return None,
            };

            // Value must be: target (+|-) const
            match value.as_ref() {
                ASTNode::BinaryOp {
                    operator,
                    left,
                    right,
                    ..
                } => {
                    // Phase 142 P0: Accept both Add (+1) and Subtract (-1)
                    let op_multiplier = match operator {
                        BinaryOperator::Add => 1,
                        BinaryOperator::Subtract => -1,
                        _ => return None,
                    };

                    // Left must be same variable
                    let left_name = match left.as_ref() {
                        ASTNode::Variable { name, .. } => name,
                        _ => return None,
                    };

                    if left_name != &target_name {
                        return None;
                    }

                    // Right must be integer literal
                    let const_val = match right.as_ref() {
                        ASTNode::Literal {
                            value: LiteralValue::Integer(n),
                            ..
                        } => *n,
                        _ => return None,
                    };

                    // Calculate delta with sign (e.g., +1 or -1)
                    let delta = const_val * op_multiplier;

                    (target_name, delta)
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    // Else branch must be single break
    if else_body.len() != 1 {
        return None;
    }

    match &else_body[0] {
        ASTNode::Break { .. } => {
            // Success! Extract body statements (all except last if)
            let body_stmts = body[..body.len() - 1].to_vec();
            Some(SkipWhitespaceInfo {
                carrier_name,
                delta,
                body_stmts,
            })
        }
        _ => None,
    }
}

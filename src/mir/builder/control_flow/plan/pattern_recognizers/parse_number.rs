//! Parse Number/Digit Route-Shape Detection
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module detects parse_number and digit collection route shapes.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Parse number route-shape information
///
/// This struct holds the extracted information from a recognized parse_number route shape.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseNumberInfo {
    /// Carrier variable name (e.g., "i")
    pub carrier_name: String,
    /// Constant step increment (e.g., 1 for `i = i + 1`)
    pub delta: i64,
    /// Body statements before the break check (may be empty)
    pub body_stmts: Vec<ASTNode>,
    /// Rest statements after break check (usually includes result append and carrier update)
    pub rest_stmts: Vec<ASTNode>,
}

/// Detect the parse_number / digit collection route shape in the loop body
///
/// Phase 143-P0: Route shape with break in THEN clause (opposite of skip_whitespace)
///
/// Shape structure:
/// ```
/// loop(cond) {
///     // ... optional body statements (ch, digit_pos computation)
///     if invalid_cond {
///         break
///     }
///     // ... rest statements (result append, carrier update)
///     carrier = carrier + const
/// }
/// ```
///
/// Recognized route shape:
/// - parse_number: `i < len`, `if digit_pos < 0 { break }`, `i = i + 1`
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `Some(ParseNumberInfo)` if the shape matches, `None` otherwise
///
/// # Notes
///
/// This is complementary to the skip_whitespace route shape (which has break in ELSE clause).
/// Used by loop_canonicalizer (Phase 143) for digit collection route shapes.
pub fn detect_parse_number_pattern(body: &[ASTNode]) -> Option<ParseNumberInfo> {
    if body.is_empty() {
        return None;
    }

    // Find the if statement with break in THEN clause
    let mut if_idx = None;
    for (i, stmt) in body.iter().enumerate() {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            // Check if then_body contains break and else_body is None
            if else_body.is_none()
                && then_body.len() == 1
                && matches!(then_body[0], ASTNode::Break { .. })
            {
                if_idx = Some(i);
                break;
            }
        }
    }

    let if_idx = if_idx?;

    // Extract body statements before the if
    let body_stmts = body[..if_idx].to_vec();

    // Extract rest statements after the if (should include carrier update)
    let rest_stmts = body[if_idx + 1..].to_vec();

    if rest_stmts.is_empty() {
        return None;
    }

    // Find carrier update in rest_stmts (last statement should be carrier = carrier + const)
    let last_stmt = &rest_stmts[rest_stmts.len() - 1];

    let (carrier_name, delta) = match last_stmt {
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
                    // Accept both Add (+1) and Subtract (-1)
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

                    // Calculate delta with sign
                    let delta = const_val * op_multiplier;

                    (target_name, delta)
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    Some(ParseNumberInfo {
        carrier_name,
        delta,
        body_stmts,
        rest_stmts,
    })
}

/// loop(true) + break-only digits pattern information
#[derive(Debug, Clone, PartialEq)]
pub struct ReadDigitsLoopTrueInfo {
    /// Counter variable name (e.g., "i")
    pub carrier_name: String,
    /// Constant step increment (currently only supports +1)
    pub delta: i64,
    /// Body statements before the digit-check if (may include `ch = substring(...)`, `if ch==\"\" { break }`, etc.)
    pub body_stmts: Vec<ASTNode>,
}

/// Detect read_digits_from-like pattern in loop body (loop(true) expected at callsite)
///
/// Recognized minimal shape (JsonCursorBox/MiniJsonLoader):
/// ```text
/// loop(true) {
///   local ch = s.substring(i, i+1)
///   if ch == "" { break }
///   if is_digit(ch) { out = out + ch; i = i + 1 } else { break }
/// }
/// ```
///
/// Contract (Phase 104 minimal):
/// - Last statement is `if ... { ... } else { break }`
/// - Then branch contains an update `i = i + 1`
/// - Then branch may contain other updates (e.g., `out = out + ch`)
pub fn detect_read_digits_loop_true_pattern(body: &[ASTNode]) -> Option<ReadDigitsLoopTrueInfo> {
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

    // Else branch must be single break
    if else_body.len() != 1 || !matches!(else_body[0], ASTNode::Break { .. }) {
        return None;
    }

    // Then branch must include `i = i + 1` (allow other statements too)
    let mut carrier_name: Option<String> = None;
    let mut delta: Option<i64> = None;
    for stmt in then_body {
        let (name, d) = match stmt {
            ASTNode::Assignment { target, value, .. } => {
                let target_name = match target.as_ref() {
                    ASTNode::Variable { name, .. } => name.clone(),
                    _ => continue,
                };
                match value.as_ref() {
                    ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } => {
                        let left_name = match left.as_ref() {
                            ASTNode::Variable { name, .. } => name,
                            _ => continue,
                        };
                        if left_name != &target_name {
                            continue;
                        }
                        let const_val = match right.as_ref() {
                            ASTNode::Literal {
                                value: LiteralValue::Integer(n),
                                ..
                            } => *n,
                            _ => continue,
                        };
                        (target_name, const_val)
                    }
                    _ => continue,
                }
            }
            _ => continue,
        };

        // Phase 104 minimal: only accept +1 step
        if d == 1 {
            carrier_name = Some(name);
            delta = Some(1);
            break;
        }
    }

    let carrier_name = carrier_name?;
    let delta = delta?;

    let body_stmts = body[..body.len() - 1].to_vec();
    Some(ReadDigitsLoopTrueInfo {
        carrier_name,
        delta,
        body_stmts,
    })
}

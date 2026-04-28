//! Phase 91 P5b: Escape Route-Shape Recognizer Module
//!
//! Specialized recognizer for escape sequence handling route shapes in string parsers.
//! Extracted from ast_feature_extractor.rs for improved modularity and reusability.
//!
//! # Design
//!
//! - **Single Responsibility**: Handles only P5b (escape sequence) route-shape detection
//! - **Isolated Helpers**: Private helpers for break/escape detection
//! - **Clean Interface**: Exports only `detect_escape_skip_shape()`

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

/// Information about a detected escape skip route shape
///
/// Phase 92 P1-2: Responsibility limited to cond/delta extraction only.
/// Body-local variable handling (`ch`) should be done by canonicalizer/caller.
#[derive(Debug, Clone)]
pub struct EscapeSkipShapeInfo {
    pub counter_name: String,
    pub normal_delta: i64,
    pub escape_delta: i64,
    /// Index of the break-guard `if ... { break }` within the loop body.
    #[allow(dead_code)] // Phase 291x-126: kept for route-shape diagnostics / handoff context.
    pub break_idx: usize,
    /// Phase 92 P0-3: The condition expression for conditional increment
    /// e.g., `ch == '\\'` for escape sequence handling
    pub escape_cond: Box<ASTNode>,
    /// Body statements before break check (for reference)
    /// Note: Caller should handle body-local variable extraction (e.g., `ch`)
    pub body_stmts: Vec<ASTNode>,
}

/// Detect an escape sequence handling route shape in the loop body
///
/// # Shape Structure
///
/// ```text
/// loop(i < n) {
///     // ... body statements
///     if ch == "\"" { break }                  // Break check
///     if ch == "\\" { i = i + 2 } else { i = i + 1 }  // Escape check with conditional delta
/// }
/// ```
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `Some(EscapeSkipShapeInfo)` if the shape matches, `None` otherwise
///
/// # Notes
///
/// This is the recognizer for P5b (Escape Sequence Handling).
/// Used by loop_canonicalizer (Phase 91) for route-shape detection and decision routing.
pub fn detect_escape_skip_shape(body: &[ASTNode]) -> Option<EscapeSkipShapeInfo> {
    if body.len() < 3 {
        return None; // Need at least: body statements, break check, escape check
    }

    // Find break statement - scan for "if { ... break ... }"
    let break_idx = find_break_in_if(body)?;

    // Find escape check after break - scan for second "if" with increment
    let escape_idx = find_escape_in_if(body, break_idx)?;

    // Extract counter_name, escape_delta, normal_delta, and condition from the escape if statement
    // Phase 92 P0-3: Now also extracts condition expression
    let (counter_name, escape_delta, normal_delta, escape_cond) =
        extract_delta_pair_from_if(body, escape_idx)?;

    // Extract body statements before break check
    let body_stmts = body[..break_idx].to_vec();

    Some(EscapeSkipShapeInfo {
        counter_name,
        normal_delta,
        escape_delta,
        break_idx,
        escape_cond, // Phase 92 P0-3: Condition for JoinIR Select
        body_stmts,
    })
}

// ============================================================================
// Private Helpers (P5b-specific)
// ============================================================================

/// Find if statement containing break
fn find_break_in_if(body: &[ASTNode]) -> Option<usize> {
    for (idx, stmt) in body.iter().enumerate() {
        if let ASTNode::If {
            then_body,
            else_body: None,
            ..
        } = stmt
        {
            // Check if then_body contains break
            if then_body.len() == 1 && matches!(&then_body[0], ASTNode::Break { .. }) {
                return Some(idx);
            }
        }
    }
    None
}

/// Find if statement containing counter increment (escape check)
///
/// Handles both:
/// - if ch == escape_char { i = i + 2 } (no else)
/// - if ch == escape_char { i = i + 2 } else { i = i + 1 } (with else)
fn find_escape_in_if(body: &[ASTNode], after_idx: usize) -> Option<usize> {
    for (idx, stmt) in body[(after_idx + 1)..].iter().enumerate() {
        let actual_idx = after_idx + 1 + idx;
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = stmt
        {
            // Check if then_body contains an increment assignment (escape case)
            let has_then_increment = then_body.iter().any(|s| {
                if let ASTNode::Assignment { target, value, .. } = s {
                    try_extract_increment_assignment(target, value).is_some()
                } else {
                    false
                }
            });

            if has_then_increment {
                // If-else format: check if else_body also has increment (normal case)
                if let Some(else_stmts) = else_body {
                    let has_else_increment = else_stmts.iter().any(|s| {
                        if let ASTNode::Assignment { target, value, .. } = s {
                            try_extract_increment_assignment(target, value).is_some()
                        } else {
                            false
                        }
                    });
                    if has_else_increment {
                        return Some(actual_idx);
                    }
                } else {
                    // No-else format: just having then increment is enough
                    return Some(actual_idx);
                }
            }
        }
    }
    None
}

/// Extract escape_delta, normal_delta, and condition from if statement
///
/// Handles both:
/// - if ch == escape_char { i = i + 2 } else { i = i + 1 }
/// - if ch == escape_char { i = i + 2 } (followed by separate increment)
///
/// Phase 92 P0-3: Now returns the condition expression for JoinIR Select generation
fn extract_delta_pair_from_if(
    body: &[ASTNode],
    idx: usize,
) -> Option<(String, i64, i64, Box<ASTNode>)> {
    if idx >= body.len() {
        return None;
    }

    if let ASTNode::If {
        condition,
        then_body,
        else_body,
        ..
    } = &body[idx]
    {
        // Extract escape_delta from then_body
        let mut escape_delta: Option<i64> = None;
        let mut counter_name: Option<String> = None;

        for stmt in then_body.iter() {
            if let ASTNode::Assignment { target, value, .. } = stmt {
                if let Some((name, delta)) = try_extract_increment_assignment(target, value) {
                    escape_delta = Some(delta);
                    counter_name = Some(name);
                    break;
                }
            }
        }

        let (mut escape_delta, counter_name) = match (escape_delta, counter_name) {
            (Some(d), Some(n)) => (d, n),
            _ => return None,
        };

        // Extract normal_delta
        let normal_delta = if let Some(else_stmts) = else_body {
            // If-else format: extract from else_body
            let mut found_delta: Option<i64> = None;
            for stmt in else_stmts.iter() {
                if let ASTNode::Assignment { target, value, .. } = stmt {
                    if let Some((name, delta)) = try_extract_increment_assignment(target, value) {
                        if name == counter_name {
                            found_delta = Some(delta);
                            break;
                        }
                    }
                }
            }
            found_delta?
        } else {
            // No-else format: look for separate increment after this if
            let mut found_delta: Option<i64> = None;
            for stmt in body[(idx + 1)..].iter() {
                if let ASTNode::Assignment { target, value, .. } = stmt {
                    if let Some((name, delta)) = try_extract_increment_assignment(target, value) {
                        if name == counter_name {
                            found_delta = Some(delta);
                            break;
                        }
                    }
                }
            }
            found_delta?
        };

        if else_body.is_none() {
            escape_delta += normal_delta;
        }

        // Phase 92 P0-3: Return condition along with deltas
        Some((counter_name, escape_delta, normal_delta, condition.clone()))
    } else {
        None
    }
}

/// Try to extract increment assignment (counter = counter (+|-) const)
fn try_extract_increment_assignment(target: &ASTNode, value: &ASTNode) -> Option<(String, i64)> {
    let target_name = match target {
        ASTNode::Variable { name, .. } => name.clone(),
        _ => return None,
    };

    match value {
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let op_multiplier = match operator {
                BinaryOperator::Add => 1,
                BinaryOperator::Subtract => -1,
                _ => return None,
            };

            let left_name = match left.as_ref() {
                ASTNode::Variable { name, .. } => name,
                _ => return None,
            };

            if left_name != &target_name {
                return None;
            }

            let const_val = match right.as_ref() {
                ASTNode::Literal {
                    value: LiteralValue::Integer(n),
                    ..
                } => *n,
                _ => return None,
            };

            let delta = const_val * op_multiplier;
            Some((target_name, delta))
        }
        _ => None,
    }
}

//! Parse String/Array Route-Shape Detection
//!
//! Phase 287 P1: Extracted from ast_feature_extractor.rs
//!
//! This module detects parse_string and parse_array route shapes with continue + return exits.

use crate::ast::{ASTNode, BinaryOperator, LiteralValue};

// Re-export has_continue_node from continue_break module
use super::continue_break::has_continue_node;

/// Parse string/array route-shape information
///
/// This struct holds the extracted information from a recognized parse_string or parse_array route shape.
/// Both route shapes share the same structure: continue + return exits with carrier updates.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseStringInfo {
    /// Carrier variable name (e.g., "p")
    pub carrier_name: String,
    /// Base constant step increment (e.g., 1 for `p = p + 1`)
    pub delta: i64,
    /// Body statements before the return/continue checks
    pub body_stmts: Vec<ASTNode>,
}

/// Detect a parse_string or parse_array route shape in the loop body
///
/// Phase 143-P1/P2: Route shape with both continue (escape/separator handling) AND return (stop condition)
///
/// Shape structure (parse_string example):
/// ```
/// loop(p < len) {
///     local ch = s.substring(p, p + 1)
///
///     // Check for closing quote (return)
///     if ch == "\"" {
///         return result
///     }
///
///     // Check for escape sequence (continue after processing)
///     if ch == "\\" {
///         result = result + ch
///         p = p + 1
///         if p < len {
///             result = result + s.substring(p, p + 1)
///             p = p + 1
///             continue
///         }
///     }
///
///     // Regular character
///     result = result + ch
///     p = p + 1
/// }
/// ```
///
/// Shape structure (parse_array example):
/// ```
/// loop(p < len) {
///     local ch = s.substring(p, p + 1)
///
///     // Check for array end (return)
///     if ch == "]" {
///         return result
///     }
///
///     // Check for separator (continue after processing)
///     if ch == "," {
///         arr.push(elem)
///         elem = ""
///         p = p + 1
///         continue
///     }
///
///     // Accumulate element
///     elem = elem + ch
///     p = p + 1
/// }
/// ```
///
/// Recognized characteristics:
/// - Has return statement (early exit on stop condition: quote for string, ']' for array)
/// - Has continue statement (skip after separator: escape for string, ',' for array)
/// - Variable step update (p++ normally, but p+=2 on escape for string)
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `Some(ParseStringInfo)` if the shape matches, `None` otherwise
///
/// # Notes
///
/// This detector handles both parse_string and parse_array route shapes as they share
/// the same structural characteristics:
/// - Multiple exit types (return AND continue)
/// - Variable step increment (conditional on separator/escape)
/// - Nested control flow (separator/escape has nested if inside)
pub fn detect_parse_string_shape(body: &[ASTNode]) -> Option<ParseStringInfo> {
    if body.is_empty() {
        return None;
    }

    // We need to find:
    // 1. An if statement with return in then_body
    // 2. An if statement with continue in then_body (nested inside)
    // 3. Carrier updates (normal and escape-case)

    let mut has_return = false;
    let mut has_continue = false;
    let mut carrier_name = None;
    let mut delta = None;

    // Scan for return statement
    for stmt in body {
        if let ASTNode::If { then_body, .. } = stmt {
            if then_body
                .iter()
                .any(|s| matches!(s, ASTNode::Return { .. }))
            {
                has_return = true;
                break;
            }
        }
    }

    if !has_return {
        return None;
    }

    // Scan for continue statement and carrier update (with recursive check for nested continue)
    for stmt in body {
        if let ASTNode::If { then_body, .. } = stmt {
            // Check for continue in then_body (including nested)
            if then_body.iter().any(|s| has_continue_node(s)) {
                has_continue = true;
            }

            // Extract carrier update from then_body
            for s in then_body {
                if let ASTNode::Assignment { target, value, .. } = s {
                    if let ASTNode::Variable { name, .. } = target.as_ref() {
                        if let ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            left,
                            right,
                            ..
                        } = value.as_ref()
                        {
                            if let ASTNode::Variable {
                                name: left_name, ..
                            } = left.as_ref()
                            {
                                if left_name == name {
                                    if let ASTNode::Literal {
                                        value: LiteralValue::Integer(n),
                                        ..
                                    } = right.as_ref()
                                    {
                                        carrier_name = Some(name.clone());
                                        delta = Some(*n);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Also check for carrier update in main body
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if let ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left,
                    right,
                    ..
                } = value.as_ref()
                {
                    if let ASTNode::Variable {
                        name: left_name, ..
                    } = left.as_ref()
                    {
                        if left_name == name {
                            if let ASTNode::Literal {
                                value: LiteralValue::Integer(n),
                                ..
                            } = right.as_ref()
                            {
                                if carrier_name.is_none() {
                                    carrier_name = Some(name.clone());
                                    delta = Some(*n);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if !has_return || !has_continue {
        return None;
    }

    let carrier_name = carrier_name?;
    let delta = delta?;

    // Extract body statements (for now, just the first statement which should be ch assignment)
    let body_stmts = if !body.is_empty() {
        vec![body[0].clone()]
    } else {
        vec![]
    };

    Some(ParseStringInfo {
        carrier_name,
        delta,
        body_stmts,
    })
}

/// Continue route-shape information
///
/// This struct holds the extracted information from a recognized continue route shape.
#[derive(Debug, Clone, PartialEq)]
pub struct ContinueShapeInfo {
    /// Carrier variable name (e.g., "i")
    pub carrier_name: String,
    /// Constant step increment (e.g., 1 for `i = i + 1`)
    pub delta: i64,
    /// Body statements before the continue check (may be empty)
    pub body_stmts: Vec<ASTNode>,
    /// Body statements after the continue check (usually includes carrier update)
    pub rest_stmts: Vec<ASTNode>,
}

/// Detect the continue route shape in the loop body
///
/// Shape structure:
/// ```
/// loop(cond) {
///     // ... optional body statements (Body)
///     if skip_cond {
///         carrier = carrier + const  // Optional update before continue
///         continue
///     }
///     // ... rest of body statements (Rest)
///     carrier = carrier + const  // Carrier update
/// }
/// ```
///
/// # Arguments
///
/// * `body` - Loop body statements to analyze
///
/// # Returns
///
/// `Some(ContinueShapeInfo)` if the shape matches, `None` otherwise
pub fn detect_continue_shape(body: &[ASTNode]) -> Option<ContinueShapeInfo> {
    if body.is_empty() {
        return None;
    }

    // Find the if statement with continue
    let mut if_idx = None;
    for (i, stmt) in body.iter().enumerate() {
        if let ASTNode::If { then_body, .. } = stmt {
            // Check if then_body contains continue
            if then_body
                .iter()
                .any(|s| matches!(s, ASTNode::Continue { .. }))
            {
                if_idx = Some(i);
                break;
            }
        }
    }

    let if_idx = if_idx?;

    // Extract body statements before the if
    let body_stmts = body[..if_idx].to_vec();

    // Extract the if statement
    let if_stmt = &body[if_idx];

    // The if must have continue in then branch
    let then_body = match if_stmt {
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            // For simple continue route shape, else_body should be None
            if else_body.is_some() {
                return None;
            }
            then_body
        }
        _ => return None,
    };

    // Check if then_body contains carrier update before continue
    // For now, we'll look for the pattern after the if statement

    // Extract rest statements after the if
    let rest_stmts = body[if_idx + 1..].to_vec();

    // Find carrier update in rest_stmts (last statement should be carrier = carrier +/- const)
    if rest_stmts.is_empty() {
        return None;
    }

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

    // Check if then_body has carrier update before continue
    // If so, we need to validate it matches
    for stmt in then_body {
        if let ASTNode::Assignment { target, .. } = stmt {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                if name == &carrier_name {
                    // There's a carrier update before continue
                    // For now, we'll just check it exists
                    // Could validate it matches the pattern later
                }
            }
        }
    }

    Some(ContinueShapeInfo {
        carrier_name,
        delta,
        body_stmts,
        rest_stmts,
    })
}

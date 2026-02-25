//! Phase 286 P2.4: Pattern8 (BoolPredicateScan) Extraction
//!
//! Minimal subset extractor for Pattern8 Plan line.
//!
//! # Supported subset (PoC safety)
//!
//! - Loop condition: `i < s.length()`
//! - Body: if statement with predicate check + loop increment
//!   - If condition: `not me.is_digit(s.substring(i, i + 1))`
//!   - Then branch: `return false`
//!   - Loop increment: `i = i + 1`
//! - Post-loop: `return true` (enforced by caller)
//! - Step literal: 1 (forward scan only)
//!
//! Returns Ok(None) for unsupported patterns → legacy fallback

use crate::ast::{ASTNode, BinaryOperator, LiteralValue, UnaryOperator};
use crate::mir::builder::control_flow::plan::{DomainPlan, Pattern8BoolPredicateScanPlan};

/// Phase 286 P2.4: Minimal subset extractor for Pattern8 Plan line
///
/// # Detection Criteria
///
/// 1. **Condition**: `i < s.length()` (forward scan)
/// 2. **Body**: if statement with predicate check
///    - Condition: `not me.is_digit(s.substring(i, i + 1))`
///    - Then branch: `return false`
/// 3. **Body**: loop increment `i = i + 1`
/// 4. **Step literal**: 1 (P0: forward scan only)
///
/// # Returns
///
/// - `Ok(Some(plan))`: Pattern8 match confirmed
/// - `Ok(None)`: Not Pattern8 (構造不一致 or unsupported)
/// - `Err(msg)`: Logic bug (malformed AST)
pub(crate) fn extract_pattern8_plan(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<DomainPlan>, String> {
    // Step 1: Validate loop condition: i < s.length()
    let (loop_var, haystack) = match validate_loop_condition_plan(condition) {
        Some((var, hay)) => (var, hay),
        None => return Ok(None), // Unsupported condition format
    };

    // Step 2: Extract predicate check (if not predicate() { return false })
    let (predicate_receiver, predicate_method) = match extract_predicate_check(body, &loop_var, &haystack) {
        Some((receiver, method)) => (receiver, method),
        None => return Ok(None), // No predicate pattern found
    };

    // Step 3: Extract loop increment (i = i + 1)
    let step_lit = match extract_loop_increment(body, &loop_var) {
        Some(lit) => lit,
        None => return Ok(None), // No increment found
    };

    // P0: Step must be 1 (forward scan only)
    if step_lit != 1 {
        return Ok(None);
    }

    Ok(Some(DomainPlan::Pattern8BoolPredicateScan(
        Pattern8BoolPredicateScanPlan {
            loop_var,
            haystack,
            predicate_receiver,
            predicate_method,
            condition: condition.clone(),
            step_lit,
        },
    )))
}

/// Validate loop condition: supports `i < s.length()` only
fn validate_loop_condition_plan(cond: &ASTNode) -> Option<(String, String)> {
    if let ASTNode::BinaryOp {
        operator: BinaryOperator::Less,
        left,
        right,
        ..
    } = cond
    {
        // Left must be a variable (loop_var)
        let loop_var = if let ASTNode::Variable { name, .. } = left.as_ref() {
            name.clone()
        } else {
            return None;
        };

        // Right must be s.length()
        let haystack = if let ASTNode::MethodCall {
            object,
            method,
            ..
        } = right.as_ref()
        {
            if method == "length" {
                if let ASTNode::Variable { name, .. } = object.as_ref() {
                    name.clone()
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        };

        Some((loop_var, haystack))
    } else {
        None
    }
}

/// Extract predicate check from body
///
/// Looks for: `if not receiver.method(s.substring(i, i + 1)) { return false }`
///
/// Returns: Some((receiver, method)) or None
fn extract_predicate_check(
    body: &[ASTNode],
    loop_var: &str,
    haystack: &str,
) -> Option<(String, String)> {
    for stmt in body.iter() {
        if let ASTNode::If {
            condition: if_cond,
            then_body,
            ..
        } = stmt
        {
            // Check if condition is: not receiver.method(...)
            if let ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand,
                ..
            } = if_cond.as_ref()
            {
                // Operand must be MethodCall
                if let ASTNode::MethodCall {
                    object,
                    method,
                    arguments,
                    ..
                } = operand.as_ref()
                {
                    // Extract receiver (e.g., "me")
                    let receiver = match object.as_ref() {
                        ASTNode::Variable { name, .. } => name.clone(),
                        ASTNode::Me { .. } | ASTNode::This { .. } => "me".to_string(),
                        _ => continue,
                    };

                    // P0: Expect 1 argument: s.substring(i, i + 1)
                    if arguments.len() != 1 {
                        continue;
                    }

                    // Validate argument is substring call
                    if !validate_substring_call(&arguments[0], haystack, loop_var) {
                        continue;
                    }

                    // Check then_body contains: return false
                    if then_body.len() == 1 {
                        if let ASTNode::Return { value, .. } = &then_body[0] {
                            if let Some(ret_val) = value {
                                if matches!(
                                    ret_val.as_ref(),
                                    ASTNode::Literal {
                                        value: LiteralValue::Bool(false),
                                        ..
                                    }
                                ) {
                                    return Some((receiver, method.clone()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Validate substring call: s.substring(i, i + 1)
fn validate_substring_call(arg: &ASTNode, haystack: &str, loop_var: &str) -> bool {
    if let ASTNode::MethodCall {
        object: substr_obj,
        method: substr_method,
        arguments: substr_args,
        ..
    } = arg
    {
        if substr_method != "substring" {
            return false;
        }

        // Object must be haystack
        if let ASTNode::Variable { name, .. } = substr_obj.as_ref() {
            if name != haystack {
                return false;
            }
        } else {
            return false;
        }

        // Args: (i, i + 1)
        if substr_args.len() != 2 {
            return false;
        }

        // Arg 0: loop_var
        if !matches!(
            &substr_args[0],
            ASTNode::Variable { name, .. } if name == loop_var
        ) {
            return false;
        }

        // Arg 1: loop_var + 1
        if let ASTNode::BinaryOp {
            operator: BinaryOperator::Add,
            left,
            right,
            ..
        } = &substr_args[1]
        {
            // Left: loop_var
            if !matches!(left.as_ref(), ASTNode::Variable { name, .. } if name == loop_var) {
                return false;
            }

            // Right: Literal(1)
            if !matches!(
                right.as_ref(),
                ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    ..
                }
            ) {
                return false;
            }

            true
        } else {
            false
        }
    } else {
        false
    }
}

/// Extract loop increment from body
///
/// Looks for: `i = i + 1`
///
/// Returns: Some(step_lit) or None
fn extract_loop_increment(body: &[ASTNode], loop_var: &str) -> Option<i64> {
    for stmt in body {
        if let ASTNode::Assignment { target, value, .. } = stmt {
            if let ASTNode::Variable {
                name: target_name, ..
            } = target.as_ref()
            {
                if target_name == loop_var {
                    if let ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left,
                        right,
                        ..
                    } = value.as_ref()
                    {
                        if let ASTNode::Variable { name: left_name, .. } = left.as_ref() {
                            if left_name == loop_var {
                                if let ASTNode::Literal {
                                    value: LiteralValue::Integer(lit),
                                    ..
                                } = right.as_ref()
                                {
                                    return Some(*lit);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn make_condition(var: &str, haystack: &str) -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: var.to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::MethodCall {
                object: Box::new(ASTNode::Variable {
                    name: haystack.to_string(),
                    span: Span::unknown(),
                }),
                method: "length".to_string(),
                arguments: vec![],
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    fn make_predicate_if(receiver: &str, method: &str, haystack: &str, loop_var: &str) -> ASTNode {
        ASTNode::If {
            condition: Box::new(ASTNode::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(ASTNode::MethodCall {
                    object: Box::new(if receiver == "me" {
                        ASTNode::Me {
                            span: Span::unknown(),
                        }
                    } else {
                        ASTNode::Variable {
                            name: receiver.to_string(),
                            span: Span::unknown(),
                        }
                    }),
                    method: method.to_string(),
                    arguments: vec![ASTNode::MethodCall {
                        object: Box::new(ASTNode::Variable {
                            name: haystack.to_string(),
                            span: Span::unknown(),
                        }),
                        method: "substring".to_string(),
                        arguments: vec![
                            ASTNode::Variable {
                                name: loop_var.to_string(),
                                span: Span::unknown(),
                            },
                            ASTNode::BinaryOp {
                                operator: BinaryOperator::Add,
                                left: Box::new(ASTNode::Variable {
                                    name: loop_var.to_string(),
                                    span: Span::unknown(),
                                }),
                                right: Box::new(ASTNode::Literal {
                                    value: LiteralValue::Integer(1),
                                    span: Span::unknown(),
                                }),
                                span: Span::unknown(),
                            },
                        ],
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Return {
                value: Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(false),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            }],
            else_body: None,
            span: Span::unknown(),
        }
    }

    fn make_loop_increment(var: &str, step: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: var.to_string(),
                span: Span::unknown(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: var.to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(step),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_extract_pattern8_success() {
        // loop(i < s.length()) { if not me.is_digit(s.substring(i, i + 1)) { return false } i = i + 1 }
        let condition = make_condition("i", "s");
        let body = vec![
            make_predicate_if("me", "is_digit", "s", "i"),
            make_loop_increment("i", 1),
        ];

        let result = extract_pattern8_plan(&condition, &body);
        assert!(result.is_ok());
        let plan = result.unwrap();
        assert!(plan.is_some());

        if let Some(DomainPlan::Pattern8BoolPredicateScan(p)) = plan {
            assert_eq!(p.loop_var, "i");
            assert_eq!(p.haystack, "s");
            assert_eq!(p.predicate_receiver, "me");
            assert_eq!(p.predicate_method, "is_digit");
            assert_eq!(p.step_lit, 1);
        } else {
            panic!("Expected Pattern8BoolPredicateScan");
        }
    }

    #[test]
    fn test_extract_pattern8_wrong_step_returns_none() {
        // loop(i < s.length()) { ... i = i + 2 } <- wrong step
        let condition = make_condition("i", "s");
        let body = vec![
            make_predicate_if("me", "is_digit", "s", "i"),
            make_loop_increment("i", 2),
        ];

        let result = extract_pattern8_plan(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Wrong step → None
    }

    #[test]
    fn test_extract_pattern8_no_predicate_returns_none() {
        // loop(i < s.length()) { i = i + 1 } <- no predicate check
        let condition = make_condition("i", "s");
        let body = vec![make_loop_increment("i", 1)];

        let result = extract_pattern8_plan(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // No predicate → None
    }
}

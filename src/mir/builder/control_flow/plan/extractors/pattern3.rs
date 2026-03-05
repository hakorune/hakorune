//! Phase 282 P5: if_phi_join extraction
//! (legacy label: Pattern3)

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::canon::generic_loop::canon_update_for_loop_var;

#[derive(Debug, Clone)]
pub(crate) struct Pattern3Parts {
    pub loop_var: String,      // Loop variable name (e.g., "i")
    pub merged_var: String,    // Primary PHI carrier (e.g., "sum")
    pub carrier_count: usize,  // Validation: 1-2 accumulators
    // Note: has_else (always true), phi_like_merge (implicit) omitted
    // AST reused from ctx - no duplication
}

/// Extract if_phi_join parts (legacy label: Pattern3)
///
/// # Detection Criteria
///
/// 1. **Condition**: 比較演算 (left=variable)
/// 2. **Body**: At least one if-else statement (else REQUIRED)
/// 3. **Assignments**: Both branches assign to same variable(s)
/// 4. **Control Flow**: NO break/continue/nested-if (return → Ok(None))
///
/// # Four-Phase Validation
///
/// **Phase 1**: Validate condition structure (reuse loop_simple_while extractor,
/// legacy label: Pattern1)
/// **Phase 2**: Find if-else statement (else branch REQUIRED)
/// **Phase 3**: Validate PHI assignments (intersection of then/else)
/// **Phase 4**: Validate NO control flow
///
/// # Fail-Fast Rules
///
/// - `Ok(Some(parts))`: if_phi_join confirmed
/// - `Ok(None)`: Not if_phi_join (structural mismatch)
/// - `Err(msg)`: Logic bug (malformed AST)
pub(crate) fn extract_loop_with_if_phi_parts(
    condition: &ASTNode,
    body: &[ASTNode],
) -> Result<Option<Pattern3Parts>, String> {
    // Phase 1: Validate condition (reuse loop_simple_while extractor, legacy label: Pattern1)
    use super::pattern1::validate_condition_structure;
    let loop_var = match validate_condition_structure(condition) {
        Some(var) => var,
        None => return Ok(None),
    };

    // Phase 2: Find if-else statement
    let if_stmt = match super::common_helpers::find_if_else_statement(body) {
        Some(stmt) => stmt,
        None => return Ok(None), // No if-else → Not if_phi_join
    };

    // Phase 3: Validate PHI assignments
    let merged_vars = match extract_phi_assignments(if_stmt) {
        Some(vars) if !vars.is_empty() => vars,
        _ => return Ok(None), // No matching assignments → Not if_phi_join
    };

    // Phase 4a: Check for return (early Ok(None) - let other patterns try)
    if super::common_helpers::has_return_statement(body) {
        return Ok(None); // Has return → delegate to other patterns
    }

    // Phase 4b: Validate allowed top-level statements (if + loop increment only)
    let mut inc_count = 0usize;
    for stmt in body {
        if std::ptr::eq(stmt, if_stmt) {
            continue;
        }
        if canon_update_for_loop_var(stmt, &loop_var).is_some() {
            inc_count += 1;
            if inc_count > 1 {
                return Ok(None);
            }
            continue;
        }
        return Ok(None);
    }

    // Phase 4c: Validate NO forbidden control flow (break/continue/nested-if only)
    // if_phi_join allows ONE if-else (the PHI pattern) but rejects nested if
    if has_forbidden_control_flow_for_pattern3(body) {
        return Ok(None); // Has break/continue/nested-if → Not if_phi_join
    }

    // Extract primary carrier (first merged var)
    let merged_var = merged_vars[0].clone();
    let carrier_count = merged_vars.len();

    Ok(Some(Pattern3Parts {
        loop_var,
        merged_var,
        carrier_count,
    }))
}

/// Extract variables assigned in BOTH then and else branches
fn extract_phi_assignments(if_stmt: &ASTNode) -> Option<Vec<String>> {
    let (then_body, else_body) = match if_stmt {
        ASTNode::If {
            then_body,
            else_body: Some(else_body),
            ..
        } => (then_body, else_body),
        _ => return None,
    };

    let then_assignments = extract_assignment_targets(then_body);
    let else_assignments = extract_assignment_targets(else_body);

    // Find intersection
    let mut merged_vars = Vec::new();
    for var in &then_assignments {
        if else_assignments.contains(var) {
            merged_vars.push(var.clone());
        }
    }

    if merged_vars.is_empty() {
        return None;
    }

    // Use first occurrence (AST order) - deterministic and meaningful SSOT
    // Don't sort alphabetically - preserve natural appearance order
    Some(merged_vars)
}

fn extract_assignment_targets(body: &[ASTNode]) -> Vec<String> {
    let mut targets = Vec::new();
    for stmt in body {
        if let ASTNode::Assignment { target, .. } = stmt {
            if let ASTNode::Variable { name, .. } = target.as_ref() {
                targets.push(name.clone());
            }
        }
    }
    targets
}

/// Check for forbidden control flow (if_phi_join-specific, legacy label: Pattern3)
///
/// if_phi_join allows ONE if-else (that's the PHI pattern) but rejects:
/// - break/continue statements
/// - NESTED if statements (if inside then/else branches)
///
/// Return is checked separately (not forbidden, just delegated to other routes).
fn has_forbidden_control_flow_for_pattern3(body: &[ASTNode]) -> bool {
    for stmt in body {
        if has_forbidden_control_flow_recursive_p3(stmt) {
            return true;
        }
    }
    false
}

fn has_forbidden_control_flow_recursive_p3(node: &ASTNode) -> bool {
    match node {
        ASTNode::Break { .. } | ASTNode::Continue { .. } => true,
        // Return removed - checked separately
        ASTNode::If {
            then_body,
            else_body,
            ..
        } => {
            // Check for NESTED if (reject)
            let has_nested_then = then_body.iter().any(|n| matches!(n, ASTNode::If { .. }));
            let has_nested_else = else_body
                .as_ref()
                .map_or(false, |b| b.iter().any(|n| matches!(n, ASTNode::If { .. })));

            if has_nested_then || has_nested_else {
                return true;
            }

            // Check control flow INSIDE branches
            then_body.iter().any(has_forbidden_control_flow_recursive_p3)
                || else_body
                    .as_ref()
                    .map_or(false, |b| b.iter().any(has_forbidden_control_flow_recursive_p3))
        }

        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_extract_if_phi_success() {
        // loop(i < 3) { if (i > 0) { sum = sum + 1 } else { sum = sum + 0 } i = i + 1 }
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Greater,
                    left: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "sum".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                else_body: Some(vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "sum".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(0),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
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
            },
        ];

        let result = extract_loop_with_if_phi_parts(&condition, &body);
        assert!(result.is_ok());
        let parts = result.unwrap();
        assert!(parts.is_some());
        let parts = parts.unwrap();
        assert_eq!(parts.loop_var, "i");
        assert_eq!(parts.merged_var, "sum");
        assert_eq!(parts.carrier_count, 1);
    }

    #[test]
    fn test_extract_no_else_returns_none() {
        // loop(i < 3) { if (i > 0) { sum = sum + 1 } i = i + 1 }  // No else
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "sum".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: None, // ← No else
            span: Span::unknown(),
        }];

        let result = extract_loop_with_if_phi_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // No else → Not Pattern3
    }

    #[test]
    fn test_extract_different_vars_returns_none() {
        // loop(i < 3) { if (i > 0) { sum = 1 } else { count = 1 } i = i + 1 }
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "sum".to_string(), // then: sum
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }],
            else_body: Some(vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "count".to_string(), // else: count (different!)
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let result = extract_loop_with_if_phi_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Different vars → Not Pattern3
    }

    #[test]
    fn test_extract_with_break_returns_none() {
        // loop(i < 3) { if (i > 0) { sum = sum + 1; break } else { sum = sum + 0 } i = i + 1 }
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Greater,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            then_body: vec![
                ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "sum".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                },
                ASTNode::Break {
                    span: Span::unknown(),
                }, // ← break
            ],
            else_body: Some(vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "sum".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let result = extract_loop_with_if_phi_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Has break → Not Pattern3
    }

    #[test]
    fn test_extract_with_prelude_local_returns_none() {
        // loop(i < 3) { local c = 0; if (i > 0) { sum = sum + 1 } else { sum = sum + 0 } i = i + 1 }
        let condition = ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: Span::unknown(),
            }),
            right: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(3),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };

        let body = vec![
            ASTNode::Local {
                variables: vec!["c".to_string()],
                initial_values: vec![Some(Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(0),
                    span: Span::unknown(),
                }))],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Greater,
                    left: Box::new(ASTNode::Variable {
                        name: "i".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(0),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "sum".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }],
                else_body: Some(vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "sum".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "sum".to_string(),
                            span: Span::unknown(),
                        }),
                        right: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(0),
                            span: Span::unknown(),
                        }),
                        span: Span::unknown(),
                    }),
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
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
            },
        ];

        let result = extract_loop_with_if_phi_parts(&condition, &body);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Prelude local → Not Pattern3
    }
}

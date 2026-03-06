//! Phase 284 P1: Return Collector for JoinIR Lowering
//!
//! SSOT for detecting return statements in loop bodies.
//! Shared by JoinIR route lowerers that need loop-body return detection.
//!
//! # P1 Scope
//!
//! - Single return statement only (multiple → Err)
//! - Return value must be integer literal (other types → Err)
//! - Return can be in top-level if's then/else (recursive scan)
//! - Return in nested loop → Err

use crate::ast::{ASTNode, LiteralValue};

/// Phase 284 P1: Return statement info from loop body
#[derive(Debug, Clone)]
pub struct ReturnInfo {
    /// The integer literal value (P1 scope: only integer literals supported)
    pub value: i64,
}

/// Collect return statement from loop body
///
/// # P1 Scope
///
/// - Single return statement only
/// - Return value must be integer literal
/// - Return can be in top-level if's then/else (recursive scan)
/// - Return in nested loop is Err
///
/// # Returns
///
/// - `Ok(Some(info))` - Single return found with integer literal value
/// - `Ok(None)` - No return found
/// - `Err(msg)` - Unsupported pattern (Fail-Fast)
pub fn collect_return_from_body(body: &[ASTNode]) -> Result<Option<ReturnInfo>, String> {
    let mut found_returns: Vec<ReturnInfo> = Vec::new();

    collect_returns_recursive(body, &mut found_returns)?;

    match found_returns.len() {
        0 => Ok(None),
        1 => Ok(Some(found_returns.remove(0))),
        n => Err(format!(
            "Phase 284 P1 scope: multiple return statements not yet supported (found {})",
            n
        )),
    }
}

/// Recursive helper to collect return statements from if branches
fn collect_returns_recursive(
    body: &[ASTNode],
    found: &mut Vec<ReturnInfo>,
) -> Result<(), String> {
    for stmt in body {
        match stmt {
            ASTNode::Return { value, .. } => {
                // P1 scope: return value must be integer literal
                let return_value = match value {
                    Some(boxed_value) => match boxed_value.as_ref() {
                        ASTNode::Literal {
                            value: LiteralValue::Integer(n),
                            ..
                        } => *n,
                        _ => {
                            return Err(
                                "Phase 284 P1 scope: return value must be integer literal"
                                    .to_string(),
                            );
                        }
                    },
                    None => {
                        return Err(
                            "Phase 284 P1 scope: return must have a value (void return not supported)"
                                .to_string(),
                        );
                    }
                };

                found.push(ReturnInfo { value: return_value });
            }
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                // Recurse into if branches
                collect_returns_recursive(then_body, found)?;
                if let Some(else_b) = else_body {
                    collect_returns_recursive(else_b, found)?;
                }
            }
            ASTNode::Loop { body: nested, .. } => {
                // P1 scope: return in nested loop is NOT supported
                if has_return_in_body(nested) {
                    return Err(
                        "Phase 284 P1 scope: return in nested loop not yet supported".to_string(),
                    );
                }
                // Don't recurse into nested loops for return collection
            }
            _ => {}
        }
    }
    Ok(())
}

/// Helper: Check if body contains any return statement (recursive)
fn has_return_in_body(body: &[ASTNode]) -> bool {
    for stmt in body {
        if matches!(stmt, ASTNode::Return { .. }) {
            return true;
        }
        match stmt {
            ASTNode::If {
                then_body,
                else_body,
                ..
            } => {
                if has_return_in_body(then_body) {
                    return true;
                }
                if let Some(else_b) = else_body {
                    if has_return_in_body(else_b) {
                        return true;
                    }
                }
            }
            ASTNode::Loop { body: nested, .. } => {
                if has_return_in_body(nested) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;

    fn make_int_return(n: i64) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(n),
                span: Span::unknown(),
            })),
            span: Span::unknown(),
        }
    }

    fn make_if_with_return(n: i64) -> ASTNode {
        ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![make_int_return(n)],
            else_body: None,
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_no_return() {
        let body = vec![];
        let result = collect_return_from_body(&body).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_single_return() {
        let body = vec![make_int_return(7)];
        let result = collect_return_from_body(&body).unwrap().unwrap();
        assert_eq!(result.value, 7);
    }

    #[test]
    fn test_return_in_if() {
        let body = vec![make_if_with_return(5)];
        let result = collect_return_from_body(&body).unwrap().unwrap();
        assert_eq!(result.value, 5);
    }

    #[test]
    fn test_multiple_returns_error() {
        let body = vec![make_int_return(1), make_int_return(2)];
        let result = collect_return_from_body(&body);
        assert!(result.is_err());
    }

    #[test]
    fn test_return_in_nested_loop_error() {
        let body = vec![ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: Span::unknown(),
            }),
            body: vec![make_int_return(3)],
            span: Span::unknown(),
        }];
        let result = collect_return_from_body(&body);
        assert!(result.is_err());
    }
}

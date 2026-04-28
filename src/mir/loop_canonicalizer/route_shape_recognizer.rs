//! Route-Shape Recognition Helpers
//!
//! Phase 140-P4-B: This module now delegates to SSOT implementations in ast_feature_extractor.
//! Provides route-shape wrappers for canonicalizer callsites.

use crate::ast::ASTNode;
use crate::mir::builder::{
    detect_continue_shape, detect_escape_skip_shape as ast_detect_escape,
    detect_parse_number_shape as ast_detect_parse_number,
    detect_parse_string_shape as ast_detect_parse_string,
    detect_read_digits_loop_true_shape as ast_detect_read_digits,
    detect_skip_whitespace_shape as ast_detect,
};

// ============================================================================
// Skip Whitespace Route Shape (Phase 140-P4-B SSOT Wrapper)
// ============================================================================

/// Try to extract skip_whitespace route shape from loop
///
/// Route shape:
/// ```
/// loop(cond) {
///     // ... optional body statements (Body)
///     if check_cond {
///         carrier = carrier + const
///     } else {
///         break
///     }
/// }
/// ```
///
/// Returns (carrier_name, delta, body_stmts) if the route shape matches.
///
/// # Phase 140-P4-B: SSOT Migration
///
/// This function delegates to the skip-whitespace route-shape recognizer owner
/// for SSOT implementation. The wrapper keeps the canonicalizer's tuple-shaped
/// adapter API while detector facts stay owned by builder/control-flow.
pub fn try_extract_skip_whitespace_shape(body: &[ASTNode]) -> Option<(String, i64, Vec<ASTNode>)> {
    ast_detect(body).map(|info| (info.carrier_name, info.delta, info.body_stmts))
}

// ============================================================================
// Phase 104: Read Digits loop(true) Route Shape
// ============================================================================

/// Try to extract read_digits_from-like route shape from loop(true) body.
///
/// Returns (carrier_name, delta, body_stmts) if the route shape matches.
pub fn try_extract_read_digits_loop_true_shape(
    body: &[ASTNode],
) -> Option<(String, i64, Vec<ASTNode>)> {
    ast_detect_read_digits(body).map(|info| (info.carrier_name, info.delta, info.body_stmts))
}

// ============================================================================
// Parse Number Route Shape (Phase 143-P0)
// ============================================================================

/// Try to extract parse_number route shape from loop
///
/// Route shape:
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
/// Returns (carrier_name, delta, body_stmts, rest_stmts) if the route shape matches.
///
/// # Phase 143-P0: Parse Number Route Detection
///
/// This function delegates to the parse-number route-shape recognizer owner
/// for SSOT implementation.
pub fn try_extract_parse_number_shape(
    body: &[ASTNode],
) -> Option<(String, i64, Vec<ASTNode>, Vec<ASTNode>)> {
    ast_detect_parse_number(body).map(|info| {
        (
            info.carrier_name,
            info.delta,
            info.body_stmts,
            info.rest_stmts,
        )
    })
}

// ============================================================================
// Parse String/Array Route Shape (Phase 143-P1/P2)
// ============================================================================

/// Try to extract parse_string or parse_array route shape from loop
///
/// Route shape:
/// ```
/// loop(cond) {
///     // ... body statements (ch computation)
///     if stop_cond {        // quote for string, ']' for array
///         return result
///     }
///     if separator_cond {   // escape for string, ',' for array
///         // ... separator handling
///         carrier = carrier + const
///         continue
///     }
///     // ... regular processing
///     carrier = carrier + const
/// }
/// ```
///
/// Returns (carrier_name, delta, body_stmts) if the route shape matches.
///
/// # Phase 143-P1/P2: Parse String/Array Route Detection
///
/// This function delegates to the parse-string route-shape recognizer owner
/// for SSOT implementation. The same detector handles both parse_string and
/// parse_array route shapes as they share the same structural characteristics.
pub fn try_extract_parse_string_shape(body: &[ASTNode]) -> Option<(String, i64, Vec<ASTNode>)> {
    ast_detect_parse_string(body).map(|info| (info.carrier_name, info.delta, info.body_stmts))
}

// ============================================================================
// Continue Route Shape (Phase 142-P1)
// ============================================================================

/// Try to extract continue route shape from loop
///
/// Route shape:
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
/// Returns (carrier_name, delta, body_stmts, rest_stmts) if the route shape matches.
///
/// # Phase 142-P1: Continue Route Detection
///
/// This function delegates to the parse-string route-shape recognizer owner
/// for SSOT implementation.
pub fn try_extract_continue_shape(
    body: &[ASTNode],
) -> Option<(String, i64, Vec<ASTNode>, Vec<ASTNode>)> {
    detect_continue_shape(body).map(|info| {
        (
            info.carrier_name,
            info.delta,
            info.body_stmts,
            info.rest_stmts,
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_skip_whitespace_basic_shape() {
        // Build: if is_ws { p = p + 1 } else { break }
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "is_ws".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
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
            else_body: Some(vec![ASTNode::Break {
                span: Span::unknown(),
            }]),
            span: Span::unknown(),
        }];

        let result = try_extract_skip_whitespace_shape(&body);
        assert!(result.is_some());

        let (carrier_name, delta, body_stmts) = result.unwrap();
        assert_eq!(carrier_name, "p");
        assert_eq!(delta, 1);
        assert_eq!(body_stmts.len(), 0);
    }

    #[test]
    fn test_skip_whitespace_with_body() {
        // Build: local ch = get_char(p); if is_ws { p = p + 1 } else { break }
        let body = vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "ch".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::FunctionCall {
                    name: "get_char".to_string(),
                    arguments: vec![ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }],
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(ASTNode::Variable {
                    name: "is_ws".to_string(),
                    span: Span::unknown(),
                }),
                then_body: vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(ASTNode::BinaryOp {
                        operator: BinaryOperator::Add,
                        left: Box::new(ASTNode::Variable {
                            name: "p".to_string(),
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
                else_body: Some(vec![ASTNode::Break {
                    span: Span::unknown(),
                }]),
                span: Span::unknown(),
            },
        ];

        let result = try_extract_skip_whitespace_shape(&body);
        assert!(result.is_some());

        let (carrier_name, delta, body_stmts) = result.unwrap();
        assert_eq!(carrier_name, "p");
        assert_eq!(delta, 1);
        assert_eq!(body_stmts.len(), 1); // The assignment before the if
    }

    #[test]
    fn test_skip_whitespace_rejects_no_else() {
        // Build: if is_ws { p = p + 1 } (no else)
        let body = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "is_ws".to_string(),
                span: Span::unknown(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Add,
                    left: Box::new(ASTNode::Variable {
                        name: "p".to_string(),
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
            else_body: None,
            span: Span::unknown(),
        }];

        let result = try_extract_skip_whitespace_shape(&body);
        assert!(result.is_none());
    }
}

// ============================================================================
// Escape Skip Route Shape (Phase 91 P5b)
// ============================================================================

/// Try to extract escape skip route shape from loop
///
/// Phase 91 P5b: Route shape for string parsers with escape sequence support
///
/// Route shape:
/// ```
/// loop(i < n) {
///     // ... optional body statements
///     if ch == "\"" { break }
///     if ch == "\\" { i = i + escape_delta; ... }
///     out = out + ch
///     i = i + 1
/// }
/// ```
///
/// Returns (counter_name, normal_delta, escape_delta, quote_char, escape_char, body_stmts, escape_cond)
/// if the route shape matches.
///
/// # Phase 91 P5b: Escape Sequence Route Detection
///
/// This function delegates to the escape route-shape recognizer owner
/// for SSOT implementation.
///
/// # Phase 92 P0-3: Added escape_cond
///
/// The escape_cond is the condition expression for the conditional increment
/// (e.g., `ch == '\\'`). This is needed for JoinIR Select generation.
pub fn try_extract_escape_skip_shape(
    body: &[ASTNode],
) -> Option<(String, i64, i64, char, char, Vec<ASTNode>, Box<ASTNode>)> {
    ast_detect_escape(body).map(|info| {
        (
            info.counter_name,
            info.normal_delta,
            info.escape_delta,
            '"',  // Phase 92 P1-2: Default quote_char for JSON/CSV
            '\\', // Phase 92 P1-2: Default escape_char for JSON/CSV
            info.body_stmts,
            info.escape_cond, // Phase 92 P0-3: Condition for JoinIR Select
        )
    })
}

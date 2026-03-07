//! Phase 33-19: Continue Branch Normalizer
//!
//! Normalize if/else continue branches to simplify `loop_continue_only` lowering
//!
//! Route shape: if (cond) { body } else { continue }
//! Transforms to: if (!cond) { continue } else { body }
//!
//! This allows `loop_continue_only` lowering to handle continue branches uniformly by
//! ensuring continue is always in the then branch.

use crate::ast::{ASTNode, UnaryOperator};
use crate::runtime::get_global_ring0;

pub struct ContinueBranchNormalizer;

impl ContinueBranchNormalizer {
    /// Normalize if node for continue handling
    ///
    /// Returns transformed AST node if pattern matches, otherwise returns clone of input.
    ///
    /// # Route Detection
    ///
    /// Matches: `if (cond) { body } else { continue }`
    /// Where else_body is a single Continue statement (or block containing only Continue).
    ///
    /// # Transformation
    ///
    /// - Creates negated condition: `!cond`
    /// - Swaps branches: then becomes else, continue becomes then
    /// - Result: `if (!cond) { continue } else { body }`
    ///
    /// # Examples
    ///
    /// ```text
    /// Input:  if (i != M) { sum = sum + i } else { continue }
    /// Output: if (!(i != M)) { continue } else { sum = sum + i }
    /// ```
    pub fn normalize_if_for_continue(if_node: &ASTNode) -> ASTNode {
        // Check if this is an If node with else branch containing only continue
        match if_node {
            ASTNode::If {
                condition,
                then_body,
                else_body,
                span,
            } => {
                // Check if else_body is a single continue statement
                if let Some(else_stmts) = else_body {
                    if Self::is_continue_only(else_stmts) {
                        // Route shape matched: if (cond) { ... } else { continue }
                        // Transform to: if (!cond) { continue } else { ... }

                        if crate::config::env::joinir_dev::debug_enabled() {
                            let ring0 = get_global_ring0();
                            ring0
                                .log
                                .debug("[continue_normalizer] Route shape matched: else-continue detected");
                            ring0.log.debug(&format!(
                                "[continue_normalizer] Original condition: {:?}",
                                condition
                            ));
                        }

                        // Create negated condition: !cond
                        let negated_cond = Box::new(ASTNode::UnaryOp {
                            operator: UnaryOperator::Not,
                            operand: condition.clone(),
                            span: span.clone(),
                        });

                        if crate::config::env::joinir_dev::debug_enabled() {
                            get_global_ring0()
                                .log
                                .debug("[continue_normalizer] Negated condition created");
                        }

                        // Swap branches: continue becomes then, body becomes else
                        let result = ASTNode::If {
                            condition: negated_cond,
                            then_body: else_stmts.clone(), // Continue
                            else_body: Some(then_body.clone()), // Original body
                            span: span.clone(),
                        };

                        if crate::config::env::joinir_dev::debug_enabled() {
                            get_global_ring0()
                                .log
                                .debug("[continue_normalizer] Transformation complete");
                        }
                        return result;
                    }
                }

                // Route shape not matched: return original if unchanged
                if_node.clone()
            }
            _ => {
                // Not an if node: return as-is
                if_node.clone()
            }
        }
    }

    /// Check if statements contain only a continue statement
    ///
    /// Returns true if:
    /// - Single statement: Continue
    /// - Or statements that effectively contain only continue
    fn is_continue_only(stmts: &[ASTNode]) -> bool {
        if stmts.is_empty() {
            return false;
        }

        // Check for single continue
        if stmts.len() == 1 {
            matches!(stmts[0], ASTNode::Continue { .. })
        } else {
            // For now, only handle single continue statement
            // Could be extended to handle blocks with continue as last statement
            false
        }
    }

    /// Check if a loop body contains an if-else pattern with else-continue
    ///
    /// This is used by the route router to detect else-continue route shapes.
    pub fn has_else_continue_pattern(body: &[ASTNode]) -> bool {
        for node in body {
            if let ASTNode::If { else_body, .. } = node {
                if let Some(else_stmts) = else_body {
                    if Self::is_continue_only(else_stmts) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Normalize all if-statements in a loop body for continue handling
    ///
    /// This is called by the loop lowerer before route-shape matching.
    /// It transforms all else-continue route shapes to then-continue route shapes.
    pub fn normalize_loop_body(body: &[ASTNode]) -> Vec<ASTNode> {
        body.iter()
            .map(|node| Self::normalize_if_for_continue(node))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{BinaryOperator, LiteralValue, Span};

    #[test]
    fn test_normalize_else_continue() {
        // if (i != M) { sum = sum + i } else { continue }
        // → if (!(i != M)) { continue } else { sum = sum + i }

        let span = Span::unknown();
        let condition = Box::new(ASTNode::BinaryOp {
            operator: BinaryOperator::NotEqual,
            left: Box::new(ASTNode::Variable {
                name: "i".to_string(),
                span: span.clone(),
            }),
            right: Box::new(ASTNode::Variable {
                name: "M".to_string(),
                span: span.clone(),
            }),
            span: span.clone(),
        });

        let then_body = vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "sum".to_string(),
                span: span.clone(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Add,
                left: Box::new(ASTNode::Variable {
                    name: "sum".to_string(),
                    span: span.clone(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: span.clone(),
                }),
                span: span.clone(),
            }),
            span: span.clone(),
        }];

        let else_body = Some(vec![ASTNode::Continue { span: span.clone() }]);

        let input = ASTNode::If {
            condition,
            then_body,
            else_body,
            span: span.clone(),
        };

        let result = ContinueBranchNormalizer::normalize_if_for_continue(&input);

        // Verify result is an If node
        if let ASTNode::If {
            condition: result_cond,
            then_body: result_then,
            else_body: result_else,
            ..
        } = result
        {
            // Condition should be negated (UnaryOp Not)
            assert!(matches!(
                *result_cond,
                ASTNode::UnaryOp {
                    operator: UnaryOperator::Not,
                    ..
                }
            ));

            // Then body should be continue
            assert_eq!(result_then.len(), 1);
            assert!(matches!(result_then[0], ASTNode::Continue { .. }));

            // Else body should be original then body
            assert!(result_else.is_some());
            let else_stmts = result_else.unwrap();
            assert_eq!(else_stmts.len(), 1);
            assert!(matches!(else_stmts[0], ASTNode::Assignment { .. }));
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_no_op_then_continue() {
        // if (i != M) { continue } else { sum = sum + i }
        // Should NOT transform (continue is in then branch)

        let span = Span::unknown();
        let condition = Box::new(ASTNode::Variable {
            name: "cond".to_string(),
            span: span.clone(),
        });

        let then_body = vec![ASTNode::Continue { span: span.clone() }];
        let else_body = Some(vec![ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: span.clone(),
            }),
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: span.clone(),
            }),
            span: span.clone(),
        }]);

        let input = ASTNode::If {
            condition: condition.clone(),
            then_body: then_body.clone(),
            else_body: else_body.clone(),
            span: span.clone(),
        };

        let result = ContinueBranchNormalizer::normalize_if_for_continue(&input);

        // Should return unchanged (continue already in then)
        // We can't use PartialEq on ASTNode due to Span, so check structure
        if let ASTNode::If {
            condition: result_cond,
            then_body: result_then,
            ..
        } = result
        {
            // Condition should NOT be negated
            assert!(matches!(*result_cond, ASTNode::Variable { .. }));

            // Then should still be continue
            assert_eq!(result_then.len(), 1);
            assert!(matches!(result_then[0], ASTNode::Continue { .. }));
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_no_op_no_else() {
        // if (i != M) { sum = sum + i }
        // Should NOT transform (no else branch)

        let span = Span::unknown();
        let input = ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: span.clone(),
            }),
            then_body: vec![ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: span.clone(),
                }),
                span: span.clone(),
            }],
            else_body: None,
            span: span.clone(),
        };

        let result = ContinueBranchNormalizer::normalize_if_for_continue(&input);

        // Should return unchanged
        if let ASTNode::If { else_body, .. } = result {
            assert!(else_body.is_none());
        } else {
            panic!("Expected If node");
        }
    }

    #[test]
    fn test_has_else_continue_pattern() {
        let span = Span::unknown();

        // Body with else-continue pattern
        let body_with = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: span.clone(),
            }),
            then_body: vec![],
            else_body: Some(vec![ASTNode::Continue { span: span.clone() }]),
            span: span.clone(),
        }];

        assert!(ContinueBranchNormalizer::has_else_continue_pattern(
            &body_with
        ));

        // Body without else-continue pattern
        let body_without = vec![ASTNode::If {
            condition: Box::new(ASTNode::Variable {
                name: "cond".to_string(),
                span: span.clone(),
            }),
            then_body: vec![ASTNode::Continue { span: span.clone() }],
            else_body: None,
            span: span.clone(),
        }];

        assert!(!ContinueBranchNormalizer::has_else_continue_pattern(
            &body_without
        ));
    }
}

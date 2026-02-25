//! Router Parity Verification (Dev-only)
//!
//! Ensures the canonicalizer's pattern choice matches the router's pattern_kind.
//! This module provides validation to ensure consistency between the two systems.

use crate::ast::{ASTNode, Span};
use crate::mir::builder::MirBuilder;

impl MirBuilder {
    /// Phase 137-4: Verify router parity between canonicalizer and router
    ///
    /// Dev-only: Ensures the canonicalizer's pattern choice matches the router's
    /// pattern_kind. On mismatch:
    /// - Debug mode (HAKO_JOINIR_DEBUG=1): Log warning
    /// - Strict mode (HAKO_JOINIR_STRICT=1 or NYASH_JOINIR_STRICT=1): Return error
    ///
    /// Phase 92 P0-2: Now returns (Result<(), String>, Option<LoopSkeleton>)
    /// The skeleton can be used by Pattern2 lowerer for ConditionalStep handling.
    pub(super) fn verify_router_parity(
        &self,
        condition: &ASTNode,
        body: &[ASTNode],
        func_name: &str,
        ctx: &super::patterns::LoopPatternContext,
    ) -> (Result<(), String>, Option<crate::mir::loop_canonicalizer::LoopSkeleton>) {
        use crate::mir::loop_canonicalizer::canonicalize_loop_expr;

        // Reconstruct loop AST for canonicalizer
        let loop_ast = ASTNode::Loop {
            condition: Box::new(condition.clone()),
            body: body.to_vec(),
            span: Span::unknown(),
        };

        // Run canonicalizer
        let (skeleton, decision) = match canonicalize_loop_expr(&loop_ast) {
            Ok((skel, dec)) => (Some(skel), dec),
            Err(e) => {
                let err_msg = format!("[loop_canonicalizer/PARITY] Canonicalizer error: {}", e);
                return (Err(err_msg), None);
            }
        };

        // Compare patterns only if canonicalizer succeeded
        let result = if let Some(canonical_pattern) = decision.chosen {
            let actual_pattern = ctx.pattern_kind;

            if canonical_pattern != actual_pattern {
                // Pattern mismatch detected
                let msg = format!(
                    "[loop_canonicalizer/PARITY] MISMATCH in function '{}': \
                     canonical={:?}, actual={:?}",
                    func_name, canonical_pattern, actual_pattern
                );

                // Phase 138-P2-B: Use SSOT for environment variable check
                use crate::config::env::joinir_dev;
                let is_strict = joinir_dev::strict_enabled();

                if is_strict {
                    // Strict mode: fail fast
                    Err(msg)
                } else {
                    // Debug mode: log only
                    super::trace::trace().dev("loop_canonicalizer/parity", &msg);
                    Ok(())
                }
            } else {
                // Patterns match - success!
                super::trace::trace().dev(
                    "loop_canonicalizer/parity",
                    &format!(
                        "[loop_canonicalizer/PARITY] OK in function '{}': canonical and actual agree on {:?}",
                        func_name, canonical_pattern
                    ),
                );
                Ok(())
            }
        } else {
            // Canonicalizer failed (Fail-Fast)
            // Log but don't error - router might still handle it
            super::trace::trace().dev(
                "loop_canonicalizer/parity",
                &format!(
                    "[loop_canonicalizer/PARITY] Canonicalizer failed for '{}': {}",
                    func_name,
                    decision.notes.join("; ")
                ),
            );
            Ok(())
        };

        // Phase 92 P0-2: Return both parity result and skeleton
        (result, skeleton)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    /// Test helper: Create a skip_whitespace pattern loop AST
    fn build_skip_whitespace_loop() -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "p".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Variable {
                    name: "len".to_string(),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            body: vec![ASTNode::If {
                condition: Box::new(ASTNode::BinaryOp {
                    operator: BinaryOperator::Equal,
                    left: Box::new(ASTNode::Variable {
                        name: "is_ws".to_string(),
                        span: Span::unknown(),
                    }),
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: Span::unknown(),
                    }),
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
            }],
            span: Span::unknown(),
        }
    }

    #[test]
    fn test_parity_check_skip_whitespace_match() {
        use crate::mir::builder::control_flow::plan::ast_feature_extractor as ast_features;
        use crate::mir::loop_canonicalizer::canonicalize_loop_expr;

        let loop_ast = build_skip_whitespace_loop();

        // Extract condition and body
        let (condition, body) = match &loop_ast {
            ASTNode::Loop {
                condition, body, ..
            } => (condition.as_ref(), body.as_slice()),
            _ => panic!("Expected loop node"),
        };

        // Run canonicalizer
        let (_, canonical_decision) = canonicalize_loop_expr(&loop_ast).unwrap();
        let canonical_pattern = canonical_decision
            .chosen
            .expect("Canonicalizer should succeed");

        // Run router's pattern detection
        let has_continue = ast_features::detect_continue_in_body(body);
        let has_break = ast_features::detect_break_in_body(body);
        let features = ast_features::extract_features(condition, body, has_continue, has_break);
        let actual_pattern = crate::mir::loop_pattern_detection::classify(&features);

        // Phase 137-5: Verify MATCH (ExitContract policy fix)
        // Both canonicalizer and router should agree on Pattern2Break
        // because has_break=true (ExitContract determines pattern choice)
        assert_eq!(
            canonical_pattern,
            crate::mir::loop_pattern_detection::LoopPatternKind::Pattern2Break,
            "Canonicalizer should choose Pattern2Break for has_break=true"
        );
        assert_eq!(
            actual_pattern,
            crate::mir::loop_pattern_detection::LoopPatternKind::Pattern2Break,
            "Router should classify as Pattern2Break for has_break=true"
        );
        assert_eq!(
            canonical_pattern, actual_pattern,
            "Phase 137-5: Canonicalizer and router should agree (SSOT policy)"
        );
    }

    #[test]
    fn test_parity_check_match_simple_while() {
        use crate::mir::builder::control_flow::plan::ast_feature_extractor as ast_features;
        use crate::mir::loop_canonicalizer::canonicalize_loop_expr;

        // Simple while loop: no break, no continue, no if
        let loop_ast = ASTNode::Loop {
            condition: Box::new(ASTNode::BinaryOp {
                operator: BinaryOperator::Less,
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: Span::unknown(),
                }),
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(10),
                    span: Span::unknown(),
                }),
                span: Span::unknown(),
            }),
            body: vec![ASTNode::Assignment {
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
            }],
            span: Span::unknown(),
        };

        // Extract condition and body
        let (condition, body) = match &loop_ast {
            ASTNode::Loop {
                condition, body, ..
            } => (condition.as_ref(), body.as_slice()),
            _ => panic!("Expected loop node"),
        };

        // Canonicalizer will fail for simple patterns (not yet implemented)
        let canonical_result = canonicalize_loop_expr(&loop_ast);

        // Router's pattern detection
        let has_continue = ast_features::detect_continue_in_body(body);
        let has_break = ast_features::detect_break_in_body(body);
        let features = ast_features::extract_features(condition, body, has_continue, has_break);
        let actual_pattern = crate::mir::loop_pattern_detection::classify(&features);

        // Router should classify as Pattern1SimpleWhile
        assert_eq!(
            actual_pattern,
            crate::mir::loop_pattern_detection::LoopPatternKind::Pattern1SimpleWhile
        );

        // Canonicalizer should fail (not implemented yet for Pattern1)
        assert!(canonical_result.is_ok());
        let (_, decision) = canonical_result.unwrap();
        assert!(
            decision.is_fail_fast(),
            "Canonicalizer should fail for simple patterns (Phase 3 only supports skip_whitespace)"
        );
    }
}

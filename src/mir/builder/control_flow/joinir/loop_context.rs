//! Loop Processing Context - SSOT for AST + Skeleton + Route
//!
//! Phase 140-P5: Unified context for loop processing that integrates:
//! - AST information (condition, body, span)
//! - Canonicalizer output (LoopSkeleton, RoutingDecision)
//! - Router information (route kind)
//!
//! This eliminates duplicate AST reconstruction and centralizes loop processing state.

use crate::ast::{ASTNode, Span};
use crate::mir::loop_canonicalizer::{LoopSkeleton, RoutingDecision};
use crate::mir::loop_pattern_detection::LoopPatternKind;

/// Loop processing context - SSOT for AST + Skeleton + Route
///
/// This context integrates all information needed for loop processing:
/// - AST: The source structure (condition, body, span)
/// - Canonicalizer: Normalized skeleton and routing decision (optional)
/// - Router: Route classification
///
/// # Lifecycle
///
/// 1. Create with `new()` - AST + Router info only
/// 2. Call `set_canonicalizer_result()` - Add Canonicalizer output
/// 3. Call `verify_parity()` - Check consistency (dev-only)
#[derive(Debug)]
pub struct LoopProcessingContext<'a> {
    // ========================================================================
    // AST Information
    // ========================================================================
    /// Loop condition AST node
    pub condition: &'a ASTNode,

    /// Loop body statements
    pub body: &'a [ASTNode],

    /// Source location for debugging
    pub span: Span,

    // ========================================================================
    // Canonicalizer Output (Optional - set after canonicalization)
    // ========================================================================
    /// Normalized loop skeleton (None = canonicalizer not run yet)
    pub skeleton: Option<LoopSkeleton>,

    /// Routing decision from canonicalizer (None = canonicalizer not run yet)
    pub decision: Option<RoutingDecision>,

    // ========================================================================
    // Router Information (Always Present)
    // ========================================================================
    /// Route kind determined by router
    pub route_kind: LoopPatternKind,

}

impl<'a> LoopProcessingContext<'a> {
    /// Create new context (canonicalizer not run yet)
    ///
    /// # Parameters
    ///
    /// - `condition`: Loop condition AST node
    /// - `body`: Loop body statements
    /// - `span`: Source location
    /// - `route_kind`: Route classification from router
    pub fn new(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
        span: Span,
        route_kind: LoopPatternKind,
    ) -> Self {
        Self {
            condition,
            body,
            span,
            skeleton: None,
            decision: None,
            route_kind,
        }
    }

    /// Set canonicalizer result (skeleton + decision)
    ///
    /// This should be called after running the canonicalizer.
    /// After this call, `verify_parity()` can be used to check consistency.
    pub fn set_canonicalizer_result(&mut self, skeleton: LoopSkeleton, decision: RoutingDecision) {
        self.skeleton = Some(skeleton);
        self.decision = Some(decision);
    }

    /// Reconstruct loop AST for canonicalizer input
    ///
    /// This is used for parity verification when we need to run the canonicalizer
    /// on the same AST that the router processed.
    pub fn to_loop_ast(&self) -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(self.condition.clone()),
            body: self.body.to_vec(),
            span: self.span.clone(),
        }
    }

    /// Verify parity between canonicalizer and router (dev-only)
    ///
    /// Checks that the canonicalizer's route choice matches the router's
    /// route_kind. On mismatch:
    /// - Strict mode (HAKO_JOINIR_STRICT=1): Returns error
    /// - Debug mode: Logs warning
    ///
    /// Returns Ok(()) if:
    /// - Canonicalizer not run yet (decision is None)
    /// - Route kinds match
    /// - Non-strict mode (mismatch logged only)
    pub fn verify_parity(&self) -> Result<(), String> {
        use crate::config::env::joinir_dev;

        // Canonicalizer not run yet - skip verification
        let decision = match &self.decision {
            Some(d) => d,
            None => return Ok(()),
        };

        // Canonicalizer failed (Fail-Fast) - no route kind to compare
        let canonical_route = match decision.chosen {
            Some(p) => p,
            None => return Ok(()), // Router might still handle it
        };

        // Compare route kinds
        let actual_route = self.route_kind;

        if canonical_route != actual_route {
            let msg = format!(
                "[loop_canonicalizer/PARITY] MISMATCH: canonical={:?}, actual={:?}",
                canonical_route, actual_route
            );

            if joinir_dev::strict_enabled() {
                // Strict mode: fail fast
                return Err(msg);
            } else {
                // Debug mode: log only
                crate::mir::builder::control_flow::joinir::trace::trace()
                    .dev("loop_canonicalizer/parity", &msg);
            }
        } else {
            // Route kinds match - success!
            crate::mir::builder::control_flow::joinir::trace::trace().dev(
                "loop_canonicalizer/parity",
                &format!(
                    "[loop_canonicalizer/PARITY] OK: canonical and actual agree on {:?}",
                    canonical_route
                ),
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::LiteralValue;

    /// Helper: Create a simple loop context for testing
    fn make_simple_context<'a>(
        condition: &'a ASTNode,
        body: &'a [ASTNode],
    ) -> LoopProcessingContext<'a> {
        LoopProcessingContext::new(
            condition,
            body,
            Span::unknown(),
            LoopPatternKind::LoopSimpleWhile,
        )
    }

    #[test]
    fn test_context_creation() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let body = vec![];
        let ctx = make_simple_context(&condition, &body);

        // Check AST fields
        assert!(matches!(
            ctx.condition,
            ASTNode::Literal {
                value: LiteralValue::Integer(1),
                ..
            }
        ));
        assert_eq!(ctx.body.len(), 0);
        assert_eq!(ctx.span, Span::unknown());

        // Check canonicalizer fields (not set yet)
        assert!(ctx.skeleton.is_none());
        assert!(ctx.decision.is_none());

        // Check router fields
        assert_eq!(ctx.route_kind, LoopPatternKind::LoopSimpleWhile);
    }

    #[test]
    fn test_to_loop_ast_reconstruction() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let body = vec![];
        let ctx = make_simple_context(&condition, &body);
        let loop_ast = ctx.to_loop_ast();

        // Check reconstructed AST
        match loop_ast {
            ASTNode::Loop {
                condition,
                body,
                span,
            } => {
                assert!(matches!(
                    *condition,
                    ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        ..
                    }
                ));
                assert_eq!(body.len(), 0);
                assert_eq!(span, Span::unknown());
            }
            _ => panic!("Expected Loop node"),
        }
    }

    #[test]
    fn test_verify_parity_without_canonicalizer() {
        let condition = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let body = vec![];
        let ctx = make_simple_context(&condition, &body);

        // Should succeed (canonicalizer not run yet)
        assert!(ctx.verify_parity().is_ok());
    }

    #[test]
    fn test_verify_parity_with_matching_patterns() {
        use crate::mir::loop_canonicalizer::{ExitContract, RoutingDecision};

        let condition = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let body = vec![];
        let mut ctx = make_simple_context(&condition, &body);

        // Set canonicalizer result with matching route kind
        let decision = RoutingDecision::success(LoopPatternKind::LoopSimpleWhile);
        ctx.set_canonicalizer_result(
            LoopSkeleton {
                steps: vec![],
                carriers: vec![],
                exits: ExitContract::none(),
                captured: None,
                span: Span::unknown(),
            },
            decision,
        );

        // Should succeed (route kinds match)
        assert!(ctx.verify_parity().is_ok());
    }

    #[test]
    fn test_verify_parity_with_fail_fast() {
        use crate::mir::loop_canonicalizer::{CapabilityTag, ExitContract, RoutingDecision};

        let condition = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let body = vec![];
        let mut ctx = make_simple_context(&condition, &body);

        // Set canonicalizer result with fail-fast decision
        let decision = RoutingDecision::fail_fast(
            vec![CapabilityTag::ConstStep],
            "Test fail-fast".to_string(),
        );
        ctx.set_canonicalizer_result(
            LoopSkeleton {
                steps: vec![],
                carriers: vec![],
                exits: ExitContract::none(),
                captured: None,
                span: Span::unknown(),
            },
            decision,
        );

        // Should succeed (canonicalizer failed, no route kind to compare)
        assert!(ctx.verify_parity().is_ok());
    }
}

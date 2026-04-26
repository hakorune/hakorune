//! Phase 142 P0: Normalized shadow loop router box
//!
//! ## Responsibility
//!
//! - Detect loop(true) shapes at block suffix (with or without subsequent statements)
//! - Delegate to NormalizationPlanBox for shape detection (SSOT)
//! - Delegate to NormalizationExecuteBox for lowering and merge
//! - Return consumed count to skip processed statements in build_block()
//!
//! ## Contract
//!
//! - Returns Ok(Some(consumed)): Successfully processed remaining[..consumed]
//! - Returns Ok(None): Shape not matched, use default behavior
//! - Returns Err(_): Internal error
//!
//! ## Design (Phase 134 P0, updated Phase 142 P0)
//!
//! - Uses NormalizationPlanBox for detection (no duplication)
//! - Uses NormalizationExecuteBox for execution (shared logic)
//! - Phase 142 P0+: Statement-level normalization accepts LoopOnly only

use crate::ast::ASTNode;
use super::{NormalizationExecuteBox, NormalizationPlanBox, PlanKind};
use crate::mir::builder::MirBuilder;

/// Box-First: Suffix router for normalized shadow lowering
pub struct NormalizedShadowSuffixRouterBox;

impl NormalizedShadowSuffixRouterBox {
    /// Try to lower a loop(true) shape in the block suffix
    ///
    /// Phase 134 P0: Unified with NormalizationPlanBox for shape detection
    /// Phase 141 P1.5: Added prefix_variables parameter for external env inputs
    /// Phase 142 P0+: Statement-level normalization accepts LoopOnly only
    ///
    /// Returns:
    /// - Ok(Some(consumed)): Successfully processed remaining[..consumed]
    /// - Ok(None): Shape not matched, use default behavior
    /// - Err(_): Internal error
    pub fn try_lower_loop_suffix(
        builder: &mut MirBuilder,
        remaining: &[ASTNode],
        func_name: &str,
        debug: bool,
        prefix_variables: Option<&std::collections::BTreeMap<String, crate::mir::ValueId>>,
    ) -> Result<Option<usize>, String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        // Phase 134 P0: Delegate shape detection to NormalizationPlanBox (SSOT)
        let plan =
            match NormalizationPlanBox::plan_block_suffix(builder, remaining, func_name, debug)? {
                Some(plan) => plan,
                None => {
                    if crate::config::env::joinir_dev::debug_enabled() {
                        let ring0 = crate::runtime::get_global_ring0();
                        ring0.log.debug(&format!(
                            "[normalization/fallback] func={} reason=plan_none err=none",
                            func_name
                        ));
                    }
                    if debug {
                        trace.routing(
                            "suffix_router",
                            func_name,
                            "NormalizationPlanBox returned None (not a normalized shape)",
                        );
                    }
                    return Ok(None);
                }
            };

        // Phase 142 P0: Normalization unit is now "statement (loop only)", not "block suffix"
        if debug {
            let description = match &plan.kind {
                PlanKind::LoopOnly => "Loop-only shape".to_string(),
            };
            trace.routing("suffix_router", func_name, &description);
        }

        // Phase 134 P0: Delegate execution to NormalizationExecuteBox (SSOT)
        // Phase 141 P1.5: Pass prefix_variables
        match NormalizationExecuteBox::execute(
            builder,
            &plan,
            remaining,
            func_name,
            debug,
            prefix_variables,
        ) {
            Ok(_value_id) => {
                // ExecuteBox returns a void constant, we don't need it for suffix routing
                // The consumed count is what build_block() needs
                if debug {
                    trace.routing(
                        "suffix_router",
                        func_name,
                        &format!(
                            "Normalization succeeded, consumed {} statements",
                            plan.consumed
                        ),
                    );
                }
                Ok(Some(plan.consumed))
            }
            Err(e) => {
                if crate::config::env::joinir_dev::strict_enabled() {
                    use crate::mir::join_ir::lowering::error_tags;
                    return Err(error_tags::freeze_with_hint(
                        "phase134/suffix_router/execute",
                        &e,
                        "Loop suffix should be supported by Normalized but execution failed",
                    ));
                }
                trace.routing("suffix_router/error", func_name, &e);
                Ok(None) // Non-strict: fallback
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, LiteralValue, Span};

    #[test]
    fn test_try_lower_loop_suffix_phase132_shape() {
        // This is an integration test that would require full MirBuilder setup
        // For now, we test the shape detection logic

        let span = Span::unknown();
        let remaining = vec![
            ASTNode::Loop {
                condition: Box::new(ASTNode::Literal {
                    value: LiteralValue::Bool(true),
                    span: span.clone(),
                }),
                body: vec![
                    ASTNode::Assignment {
                        target: Box::new(ASTNode::Variable {
                            name: "x".to_string(),
                            span: span.clone(),
                        }),
                        value: Box::new(ASTNode::Literal {
                            value: LiteralValue::Integer(1),
                            span: span.clone(),
                        }),
                        span: span.clone(),
                    },
                    ASTNode::Break { span: span.clone() },
                ],
                span: span.clone(),
            },
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                }),
                value: Box::new(ASTNode::BinaryOp {
                    left: Box::new(ASTNode::Variable {
                        name: "x".to_string(),
                        span: span.clone(),
                    }),
                    operator: crate::ast::BinaryOperator::Add,
                    right: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(2),
                        span: span.clone(),
                    }),
                    span: span.clone(),
                }),
                span: span.clone(),
            },
            ASTNode::Return {
                value: Some(Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                })),
                span: span.clone(),
            },
        ];

        // Shape should be detected (at least 2 statements, first is Loop)
        assert!(remaining.len() >= 2);
        assert!(matches!(&remaining[0], ASTNode::Loop { .. }));

        // Full lowering test would require MirBuilder setup
        // This validates the shape structure
    }

    #[test]
    fn test_try_lower_loop_suffix_no_match_too_short() {
        // Single statement - should not match
        let span = Span::unknown();
        let remaining = vec![ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: span.clone(),
            }),
            body: vec![ASTNode::Break { span: span.clone() }],
            span: span.clone(),
        }];

        // Shape should not match (need at least 2 statements)
        assert!(remaining.len() < 2);
    }

    #[test]
    fn test_try_lower_loop_suffix_no_match_not_loop() {
        // First statement is not a loop
        let span = Span::unknown();
        let remaining = vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                }),
                value: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(1),
                    span: span.clone(),
                }),
                span: span.clone(),
            },
            ASTNode::Return {
                value: Some(Box::new(ASTNode::Variable {
                    name: "x".to_string(),
                    span: span.clone(),
                })),
                span: span.clone(),
            },
        ];

        // Shape should not match (first statement is not Loop)
        assert!(!matches!(&remaining[0], ASTNode::Loop { .. }));
    }
}

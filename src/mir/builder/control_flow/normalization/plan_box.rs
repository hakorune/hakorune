//! NormalizationPlanBox: Shape detection for Normalized shadow (Phase 134 P0)
//!
//! ## Responsibility
//!
//! - Detect block suffix patterns that can be normalized
//! - Return plan specifying what to consume and how to lower
//! - SSOT for "what to normalize" decision
//!
//! ## Contract
//!
//! - Returns Ok(Some(plan)): Shape detected, proceed with normalization
//! - Returns Ok(None): Not a normalized pattern, use legacy fallback
//! - Returns Err(_): Internal error (malformed AST)

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::builder::MirBuilder;
use super::plan::NormalizationPlan;

#[cfg(test)]
use super::plan::PlanKind;

/// Box-First: Shape detection for Normalized shadow
pub struct NormalizationPlanBox;

impl NormalizationPlanBox {
    /// Detect Normalized pattern in block suffix
    ///
    /// Returns:
    /// - Ok(Some(plan)): Normalized pattern detected
    /// - Ok(None): Not a normalized pattern (use legacy)
    /// - Err(_): Internal error
    pub fn plan_block_suffix(
        _builder: &MirBuilder,
        remaining: &[ASTNode],
        func_name: &str,
        debug: bool,
    ) -> Result<Option<NormalizationPlan>, String> {
        let trace = crate::mir::builder::control_flow::joinir::trace::trace();

        if debug {
            trace.routing(
                "normalization/plan",
                func_name,
                &format!("Checking {} remaining statements", remaining.len()),
            );
        }

        // Empty suffix - not normalized
        if remaining.is_empty() {
            return Ok(None);
        }

        // First statement must be a loop with condition `true`
        // Phase 131-136 ONLY support loop(true), not loop(i < n) etc.
        let (is_loop_true, loop_body) = match &remaining[0] {
            ASTNode::Loop { condition, body, .. } => {
                // Only accept loop(true) - literal Bool true
                let is_true = matches!(
                    condition.as_ref(),
                    ASTNode::Literal { value: LiteralValue::Bool(true), .. }
                );
                (is_true, Some(body.as_slice()))
            }
            _ => (false, None),
        };
        if !is_loop_true {
            if debug {
                trace.routing(
                    "normalization/plan",
                    func_name,
                    "First statement is not loop(true), returning None (not a normalized pattern)",
                );
            }
            return Ok(None);
        }

        // Phase 286 P3.2: Reject loop_true_early_exit-style loops (if with early return/break)
        // These are handled by the Plan line (loop_true_early_exit), not StepTree
        if let Some(body) = loop_body {
            if !body.is_empty() {
                if let ASTNode::If { then_body, else_body, .. } = &body[0] {
                    // Check if it's a loop_true_early_exit-style if (no else, then contains return or break)
                    if else_body.is_none() {
                        let has_early_exit = then_body.iter().any(|stmt| {
                            matches!(stmt, ASTNode::Return { .. } | ASTNode::Break { .. })
                        });
                        if has_early_exit {
                            if debug {
                                trace.routing(
                                    "normalization/plan",
                                    func_name,
                                    "Loop body has if with early return/break - loop_true_early_exit (Plan line), returning None",
                                );
                            }
                            return Ok(None);
                        }
                    }
                }
            }
        }

        // Phase 142 P0: Only return loop_only when loop body is in Normalized scope
        // Normalization unit is now "statement (loop 1個)" not "block suffix"
        // Subsequent statements (return, assignments, etc.) handled by normal MIR lowering
        if let Some(body) = loop_body {
            if !loop_true_body_supported_for_normalized(body) {
                if debug {
                    trace.routing(
                        "normalization/plan",
                        func_name,
                        "Loop(true) body is out of scope for Normalized (returning None)",
                    );
                }
                return Ok(None);
            }
        }
        if debug {
            trace.routing(
                "normalization/plan",
                func_name,
                "Detected loop(true) - Phase 142 P0: returning loop_only (consumed=1)",
            );
        }
        Ok(Some(NormalizationPlan::loop_only()))
    }
}

fn loop_true_body_supported_for_normalized(body: &[ASTNode]) -> bool {
    if body.is_empty() {
        return false;
    }

    if body.len() == 1 {
        if let ASTNode::If {
            then_body,
            else_body,
            ..
        } = &body[0]
        {
            if is_break_or_continue_only(then_body)
                && else_body
                    .as_ref()
                    .map_or(true, |branch| is_break_or_continue_only(branch))
            {
                return true;
            }
        }
    }

    if !matches!(body.last(), Some(ASTNode::Break { .. })) {
        return false;
    }

    body[..body.len() - 1].iter().all(|stmt| {
        matches!(stmt, ASTNode::Assignment { .. } | ASTNode::Local { .. })
    })
}

fn is_break_or_continue_only(stmts: &[ASTNode]) -> bool {
    if stmts.len() != 1 {
        return false;
    }
    matches!(stmts[0], ASTNode::Break { .. } | ASTNode::Continue { .. })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span, BinaryOperator};

    fn make_span() -> Span {
        Span::unknown()
    }

    fn make_loop() -> ASTNode {
        ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: make_span(),
            }),
            body: vec![
                ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "x".to_string(),
                        span: make_span(),
                    }),
                    value: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: make_span(),
                    }),
                    span: make_span(),
                },
                ASTNode::Break { span: make_span() },
            ],
            span: make_span(),
        }
    }

    fn make_assignment(var: &str, value: i64) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(ASTNode::Variable {
                name: var.to_string(),
                span: make_span(),
            }),
            value: Box::new(ASTNode::BinaryOp {
                left: Box::new(ASTNode::Variable {
                    name: var.to_string(),
                    span: make_span(),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(ASTNode::Literal {
                    value: LiteralValue::Integer(value),
                    span: make_span(),
                }),
                span: make_span(),
            }),
            span: make_span(),
        }
    }

    fn make_return(var: &str) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(ASTNode::Variable {
                name: var.to_string(),
                span: make_span(),
            })),
            span: make_span(),
        }
    }

    #[test]
    fn test_plan_block_suffix_phase131_loop_only() {
        use crate::mir::builder::MirBuilder;

        let remaining = vec![make_loop()];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.consumed, 1);
        assert_eq!(plan.kind, PlanKind::LoopOnly);
        assert!(!plan.requires_return);
    }

    #[test]
    fn test_plan_block_suffix_phase142_loop_with_subsequent_stmts() {
        use crate::mir::builder::MirBuilder;

        // Phase 142 P0: loop(true) returns loop_only regardless of subsequent statements
        let remaining = vec![
            make_loop(),
            make_assignment("x", 2),
            make_return("x"),
        ];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.consumed, 1); // Phase 142: only consume loop
        assert_eq!(plan.kind, PlanKind::LoopOnly);
        assert!(!plan.requires_return); // Loop itself doesn't require return
    }

    #[test]
    fn test_plan_block_suffix_phase142_loop_only_always() {
        use crate::mir::builder::MirBuilder;

        // Phase 142 P0: loop(true) always returns loop_only, even with multiple statements after
        let remaining = vec![
            make_loop(),
            make_assignment("x", 2),
            make_assignment("x", 3),
            make_return("x"),
        ];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.consumed, 1); // Phase 142: only consume loop
        assert_eq!(plan.kind, PlanKind::LoopOnly);
        assert!(!plan.requires_return);
    }

    #[test]
    fn test_plan_block_suffix_no_match_empty() {
        use crate::mir::builder::MirBuilder;

        let remaining: Vec<ASTNode> = vec![];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        assert!(plan.is_none());
    }

    #[test]
    fn test_plan_block_suffix_no_match_not_loop() {
        use crate::mir::builder::MirBuilder;

        let remaining = vec![
            make_assignment("x", 1),
            make_return("x"),
        ];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        assert!(plan.is_none());
    }

    #[test]
    fn test_plan_block_suffix_phase142_loop_with_trailing_stmt() {
        use crate::mir::builder::MirBuilder;

        // Phase 142 P0: loop(true) returns loop_only even if no return follows
        let remaining = vec![
            make_loop(),
            make_assignment("x", 2),
            // No return - but still returns loop_only
        ];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        // Phase 142: loop(true) always matches, regardless of subsequent statements
        assert!(plan.is_some());
        let plan = plan.unwrap();
        assert_eq!(plan.consumed, 1); // Only consume loop
        assert_eq!(plan.kind, PlanKind::LoopOnly);
    }

    #[test]
    fn test_plan_block_suffix_no_match_loop_not_true() {
        use crate::mir::builder::MirBuilder;

        // Phase 97 regression test: loop(i < n) should NOT match
        // Normalized shadow only supports loop(true)
        let loop_conditional = ASTNode::Loop {
            condition: Box::new(ASTNode::BinaryOp {
                left: Box::new(ASTNode::Variable {
                    name: "i".to_string(),
                    span: make_span(),
                }),
                operator: BinaryOperator::Less,
                right: Box::new(ASTNode::Variable {
                    name: "n".to_string(),
                    span: make_span(),
                }),
                span: make_span(),
            }),
            body: vec![
                ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "x".to_string(),
                        span: make_span(),
                    }),
                    value: Box::new(ASTNode::Literal {
                        value: LiteralValue::Integer(1),
                        span: make_span(),
                    }),
                    span: make_span(),
                },
                ASTNode::Break { span: make_span() },
            ],
            span: make_span(),
        };

        let remaining = vec![loop_conditional, make_return("x")];

        let builder = MirBuilder::new();
        let plan = NormalizationPlanBox::plan_block_suffix(&builder, &remaining, "test", false)
            .expect("Should not error");

        // loop(i < n) is NOT supported by Normalized - should return None
        assert!(plan.is_none(), "loop(i < n) should NOT match Normalized pattern");
    }
}

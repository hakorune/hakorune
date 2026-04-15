use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;

const STAGE: &str = "generic_loop_body::nested_loop_plan";

pub(super) fn finish_generic_nested_loop_reject_tail(
    outcome: &PlanBuildOutcome,
    nested_ctx: &LoopRouteContext,
) -> Result<LoweredRecipe, String> {
    if let Some(err) = composer::strict_nested_loop_guard(outcome, nested_ctx) {
        plan_trace::trace_outcome_path(STAGE, "nested_loop_guard_error");
        return Err(err);
    }
    plan_trace::trace_outcome_path(STAGE, "freeze_no_plan");
    Err("[normalizer] generic nested loop: nested loop has no plan".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn less_cond() -> ASTNode {
        ASTNode::BinaryOp {
            operator: BinaryOperator::Less,
            left: Box::new(var("i")),
            right: Box::new(int(10)),
            span: span(),
        }
    }

    #[test]
    fn generic_nested_loop_reject_tail_freezes_when_no_plan() {
        let condition = less_cond();
        let body = vec![ASTNode::Assignment {
            target: Box::new(var("x")),
            value: Box::new(int(1)),
            span: span(),
        }];
        let ctx = LoopRouteContext::new(&condition, &body, "<nested>", false, false);
        let outcome = PlanBuildOutcome {
            facts: None,
            recipe_contract: None,
        };

        let err = finish_generic_nested_loop_reject_tail(&outcome, &ctx).expect_err("must fail");

        assert!(err.contains("nested loop has no plan"));
    }

    #[test]
    fn generic_nested_loop_reject_tail_prefers_strict_guard_error() {
        let condition = less_cond();
        let body = vec![ASTNode::Loop {
            condition: Box::new(ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: span(),
            }),
            body: vec![],
            span: span(),
        }];
        let ctx = LoopRouteContext::new(&condition, &body, "<nested>", false, false);
        let outcome = PlanBuildOutcome {
            facts: None,
            recipe_contract: None,
        };

        let err = finish_generic_nested_loop_reject_tail(&outcome, &ctx).expect_err("must fail");

        assert!(err.contains("strict_nested_loop_guard"));
    }
}

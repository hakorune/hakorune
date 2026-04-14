//! Shared recipe fallback orchestration for nested-loop lowering.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::nested_loop_plan_break_continue::try_compose_loop_cond_break_continue_recipe_bridge;
use crate::mir::builder::control_flow::plan::nested_loop_plan_continue_with_return::try_compose_loop_cond_continue_with_return_recipe_bridge;
use crate::mir::builder::control_flow::plan::nested_loop_plan_recipe_fallback_policy::{
    select_nested_loop_recipe_fallback, NestedLoopRecipeFallbackKind,
};
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_nested_loop_recipe_fallback(
    builder: &mut MirBuilder,
    outcome: &PlanBuildOutcome,
    nested_ctx: &LoopRouteContext,
    stage: &str,
    planner_required: bool,
) -> Result<Option<LoweredRecipe>, String> {
    match select_nested_loop_recipe_fallback(outcome, planner_required) {
        Some(NestedLoopRecipeFallbackKind::ContinueWithReturn) => {
            try_compose_loop_cond_continue_with_return_recipe_bridge(
                builder,
                outcome,
                nested_ctx,
                stage,
                planner_required,
            )
        }
        Some(NestedLoopRecipeFallbackKind::BreakContinue) => {
            try_compose_loop_cond_break_continue_recipe_bridge(
                builder,
                outcome,
                nested_ctx,
                stage,
                planner_required,
            )
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;

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
    fn nested_loop_recipe_fallback_skips_when_not_required() {
        let mut builder = MirBuilder::new();
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

        let result = try_compose_nested_loop_recipe_fallback(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_recipe_fallback",
            false,
        )
        .expect("fallback should not error");

        assert!(result.is_none());
    }

    #[test]
    fn nested_loop_recipe_fallback_skips_without_facts() {
        let mut builder = MirBuilder::new();
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

        let result = try_compose_nested_loop_recipe_fallback(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_recipe_fallback",
            true,
        )
        .expect("fallback should not error");

        assert!(result.is_none());
    }
}

//! Break-continue recipe bridge for nested-loop fallback.

use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_loop_cond_break_continue_recipe_bridge(
    builder: &mut MirBuilder,
    outcome: &PlanBuildOutcome,
    nested_ctx: &LoopRouteContext,
    stage: &str,
    planner_required: bool,
) -> Result<Option<LoweredRecipe>, String> {
    if !planner_required {
        return Ok(None);
    }
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.loop_cond_break_continue.is_none() {
        return Ok(None);
    }
    if outcome.recipe_contract.is_none() {
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "loop_cond_break_continue requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
    }
    plan_trace::trace_outcome_path(stage, "recipe_loop_cond_break_continue");
    RecipeComposer::compose_loop_cond_break_continue_recipe(builder, facts, nested_ctx)
        .map(Some)
        .map_err(|e| e.to_string())
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
    fn nested_loop_plan_bc_bridge_skips_when_not_required() {
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

        let result = try_compose_loop_cond_break_continue_recipe_bridge(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_plan_bc_bridge",
            false,
        )
        .expect("bridge should not error");

        assert!(result.is_none());
    }

    #[test]
    fn nested_loop_plan_bc_bridge_skips_without_facts() {
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

        let result = try_compose_loop_cond_break_continue_recipe_bridge(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_plan_bc_bridge",
            true,
        )
        .expect("bridge should not error");

        assert!(result.is_none());
    }
}

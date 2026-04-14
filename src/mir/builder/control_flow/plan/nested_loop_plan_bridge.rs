//! Shared recipe-first fallback bridge for nested-loop lowering.

use crate::ast::ASTNode;
use crate::config::env::joinir_dev;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1;
use crate::mir::builder::control_flow::plan::features::nested_loop_depth1_preheader::apply_nested_loop_preheader_freshness;
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::MirBuilder;

pub(in crate::mir::builder) fn try_compose_loop_cond_continue_with_return_recipe_bridge(
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
    if facts.facts.loop_cond_continue_with_return.is_none() {
        return Ok(None);
    }
    if outcome.recipe_contract.is_none() {
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "loop_cond_continue_with_return requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
    }
    plan_trace::trace_outcome_path(stage, "recipe_loop_cond_continue_with_return");
    RecipeComposer::compose_loop_cond_continue_with_return_recipe(builder, facts, nested_ctx)
        .map(Some)
        .map_err(|e| e.to_string())
}

pub(in crate::mir::builder) fn lower_nested_loop_plan_with_recipe_first_bridge(
    builder: &mut MirBuilder,
    condition: &ASTNode,
    body: &[ASTNode],
    ctx: &LoopRouteContext,
    error_prefix: &str,
    tag: &str,
) -> Result<LoweredRecipe, String> {
    if let Ok(plan) =
        nested_loop_depth1::lower_nested_loop_depth1_any(builder, condition, body, error_prefix)
    {
        return Ok(apply_nested_loop_preheader_freshness(builder, plan));
    }

    let nested_ctx =
        LoopRouteContext::new(condition, body, ctx.func_name, ctx.debug, ctx.in_static_box);
    let outcome = single_planner::try_build_outcome(&nested_ctx)?;
    plan_trace::trace_outcome_snapshot(
        "nested_loop_plan_with_recipe_first",
        false,
        outcome.facts.is_some(),
        outcome.recipe_contract.is_some(),
    );

    let strict_or_dev = joinir_dev::strict_enabled() || crate::config::env::joinir_dev_enabled();
    let planner_required = strict_or_dev && joinir_dev::planner_required_enabled();
    if let Some(recipe) = try_compose_loop_cond_continue_with_return_recipe_bridge(
        builder,
        &outcome,
        &nested_ctx,
        "nested_loop_plan_with_recipe_first",
        planner_required,
    )? {
        return Ok(recipe);
    }

    if planner_required {
        if let Some(facts) = outcome.facts.as_ref() {
            if facts.facts.loop_cond_break_continue.is_some() && outcome.recipe_contract.is_some() {
                plan_trace::trace_outcome_path(
                    "nested_loop_plan_with_recipe_first",
                    "recipe_loop_cond_break_continue",
                );
                return RecipeComposer::compose_loop_cond_break_continue_recipe(
                    builder,
                    facts,
                    &nested_ctx,
                )
                .map_err(|e| e.to_string());
            }
        }
    }

    plan_trace::trace_outcome_path("nested_loop_plan_with_recipe_first", "freeze_no_plan");
    Err(format!(
        "[freeze:contract][{tag}] nested loop has no plan: ctx={error_prefix}"
    ))
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
    fn nested_loop_plan_bridge_skips_continue_with_return_when_not_required() {
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

        let result = try_compose_loop_cond_continue_with_return_recipe_bridge(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_plan_bridge",
            false,
        )
        .expect("bridge should not error");

        assert!(result.is_none());
    }

    #[test]
    fn nested_loop_plan_bridge_skips_continue_with_return_without_facts() {
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

        let result = try_compose_loop_cond_continue_with_return_recipe_bridge(
            &mut builder,
            &outcome,
            &ctx,
            "nested_loop_plan_bridge",
            true,
        )
        .expect("bridge should not error");

        assert!(result.is_none());
    }
}

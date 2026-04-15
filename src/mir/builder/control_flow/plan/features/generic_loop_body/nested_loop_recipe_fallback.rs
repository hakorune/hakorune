use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::plan::nested_loop_plan::try_compose_loop_cond_continue_with_return_recipe;
use crate::mir::builder::control_flow::plan::planner::tags::planner_first_tag_with_label;
use crate::mir::builder::control_flow::plan::planner::Freeze;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::builder::control_flow::plan::trace as plan_trace;
use crate::mir::builder::control_flow::plan::LoweredRecipe;
use crate::mir::builder::control_flow::plan::PlanBuildOutcome;
use crate::mir::builder::MirBuilder;

const STAGE: &str = "generic_loop_body::nested_loop_plan";

pub(super) fn try_compose_generic_nested_loop_recipe_fallback(
    builder: &mut MirBuilder,
    outcome: &PlanBuildOutcome,
    nested_ctx: &LoopRouteContext,
    strict_or_dev: bool,
    planner_required: bool,
) -> Result<Option<LoweredRecipe>, String> {
    if let Some(recipe) = try_compose_loop_cond_continue_with_return_recipe(
        builder,
        outcome,
        nested_ctx,
        STAGE,
        planner_required,
    )? {
        return Ok(Some(recipe));
    }

    if let Some(facts) = outcome.facts.as_ref() {
        if planner_required && facts.facts.loop_true_break_continue.is_some() {
            if outcome.recipe_contract.is_none() {
                return Err(Freeze::contract(
                    "loop_true_break_continue requires recipe_contract in planner_required mode",
                )
                .to_string());
            }
            plan_trace::trace_outcome_path(STAGE, "recipe_loop_true_break_continue");
            return RecipeComposer::compose_loop_true_break_continue_recipe(
                builder, facts, nested_ctx,
            )
            .map(Some)
            .map_err(|e| e.to_string());
        }
        if facts.facts.loop_cond_return_in_body.is_some() {
            plan_trace::trace_outcome_path(STAGE, "recipe_loop_cond_return_in_body");
            maybe_emit_planner_first_tag(strict_or_dev, PlanRuleId::LoopCondReturnInBody);
            return RecipeComposer::compose_loop_cond_return_in_body_recipe(
                builder, facts, nested_ctx,
            )
            .map(Some)
            .map_err(|e| e.to_string());
        }
        if facts.facts.loop_cond_break_continue.is_some() {
            plan_trace::trace_outcome_path(STAGE, "recipe_loop_cond_break_continue");
            maybe_emit_planner_first_tag(strict_or_dev, PlanRuleId::LoopCondBreak);
            return RecipeComposer::compose_loop_cond_break_continue_recipe(
                builder, facts, nested_ctx,
            )
            .map(Some)
            .map_err(|e| e.to_string());
        }
        if facts.facts.generic_loop_v0.is_some() {
            plan_trace::trace_outcome_path(STAGE, "recipe_generic_loop_v0");
            return RecipeComposer::compose_generic_loop_v0_recipe(builder, facts, nested_ctx)
                .map(Some)
                .map_err(|e| e.to_string());
        }
        if facts.facts.generic_loop_v1.is_some() {
            plan_trace::trace_outcome_path(STAGE, "recipe_generic_loop_v1");
            return RecipeComposer::compose_generic_loop_v1_recipe(builder, facts, nested_ctx)
                .map(Some)
                .map_err(|e| e.to_string());
        }
    }

    if let Some(facts) = outcome.facts.as_ref() {
        if facts.facts.nested_loop_minimal().is_some() {
            plan_trace::trace_outcome_path(STAGE, "recipe_nested_loop_minimal");
            let core_plan =
                composer::try_compose_core_loop_v2_nested_minimal(builder, facts, nested_ctx)?
                    .ok_or_else(|| {
                        "nested_loop_minimal strict/dev adopt failed: compose rejected".to_string()
                    })?;
            return Ok(Some(core_plan));
        }
    }

    Ok(None)
}

fn maybe_emit_planner_first_tag(strict_or_dev: bool, rule_id: PlanRuleId) {
    if !strict_or_dev {
        return;
    }
    let msg = planner_first_tag_with_label(rule_id);
    if crate::config::env::joinir_dev::strict_planner_required_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        let _ = ring0.io.stderr_write(format!("{}\n", msg).as_bytes());
    } else if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug(&msg);
    }
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
    fn generic_nested_loop_recipe_fallback_skips_when_not_required() {
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

        let result = try_compose_generic_nested_loop_recipe_fallback(
            &mut builder,
            &outcome,
            &ctx,
            false,
            false,
        )
        .expect("fallback should not error");

        assert!(result.is_none());
    }

    #[test]
    fn generic_nested_loop_recipe_fallback_skips_without_facts() {
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

        let result = try_compose_generic_nested_loop_recipe_fallback(
            &mut builder,
            &outcome,
            &ctx,
            true,
            true,
        )
        .expect("fallback should not error");

        assert!(result.is_none());
    }
}

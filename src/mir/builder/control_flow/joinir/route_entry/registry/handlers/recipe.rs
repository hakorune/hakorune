use crate::mir::builder::control_flow::joinir::route_entry::router::{
    lower_verified_core_plan, LoopRouteContext,
};
use crate::mir::builder::control_flow::lower::{
    planner_rule_route_label, Freeze, PlanBuildOutcome, PlanLowerer, PlanRuleId,
};
use crate::mir::builder::control_flow::plan::composer::strict_nested_loop_guard;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::types::{PlannerFirstMode, RouterEnv};
use super::super::utils::{
    emit_planner_first, loop_break_recipe_needs_flowbox_adopt_tag_in_strict,
};
use super::common::debug_log_recipe_entry;

pub(crate) fn route_loop_break_recipe(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(
            "LoopBreakRecipe requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first(
        PlannerFirstMode::StrictOrDev,
        env,
        PlanRuleId::LoopBreakRecipe,
    );
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::LoopBreakRecipe), env);

    let facts = outcome
        .facts
        .as_ref()
        .expect("loop_break_recipe facts present");
    let core_plan = RecipeComposer::compose_loop_break_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    if env.strict_or_dev {
        let loop_break_facts = facts
            .facts
            .loop_break()
            .expect("loop_break_recipe is present");
        let needs_flowbox_tag = env.has_body_local
            || loop_break_recipe_needs_flowbox_adopt_tag_in_strict(loop_break_facts);

        if needs_flowbox_tag {
            return lower_verified_core_plan(
                builder,
                ctx,
                env.strict_or_dev,
                outcome.facts.as_ref(),
                core_plan,
                FlowboxVia::Shadow,
            );
        }

        PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
        return PlanLowerer::lower(builder, core_plan, ctx);
    }

    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        FlowboxVia::Release,
    )
}

pub(crate) fn route_if_phi_join(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(
            "IfPhiJoin requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first(PlannerFirstMode::StrictOrDev, env, PlanRuleId::IfPhiJoin);
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::IfPhiJoin), env);

    let facts = outcome.facts.as_ref().expect("if_phi_join facts present");
    let core_plan = RecipeComposer::compose_if_phi_join_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    let via = if env.strict_or_dev {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        via,
    )
}

pub(crate) fn route_loop_continue_only(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(
            "LoopContinueOnly requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first(
        PlannerFirstMode::StrictOrDev,
        env,
        PlanRuleId::LoopContinueOnly,
    );
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::LoopContinueOnly), env);
    if env.planner_required {
        if let Some(err) = strict_nested_loop_guard(outcome, ctx) {
            flowbox_tags::emit_flowbox_freeze_tag_from_facts(
                env.strict_or_dev,
                "unstructured",
                outcome.facts.as_ref(),
            );
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("{}", err));
            return Err(err);
        }
    }

    let facts = outcome
        .facts
        .as_ref()
        .expect("loop_continue_only facts present");
    let core_plan = RecipeComposer::compose_loop_continue_only_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;
    let via = if env.strict_or_dev {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        via,
    )
}

pub(crate) fn route_accum_const_loop(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(
            "AccumConstLoop requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first(
        PlannerFirstMode::StrictOrDev,
        env,
        PlanRuleId::AccumConstLoop,
    );
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::AccumConstLoop), env);

    let facts = outcome
        .facts
        .as_ref()
        .expect("accum_const_loop facts present");
    let core_plan = RecipeComposer::compose_accum_const_loop_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    if env.strict_or_dev {
        PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
        return PlanLowerer::lower(builder, core_plan, ctx);
    }

    lower_verified_core_plan(
        builder,
        ctx,
        env.strict_or_dev,
        outcome.facts.as_ref(),
        core_plan,
        FlowboxVia::Release,
    )
}

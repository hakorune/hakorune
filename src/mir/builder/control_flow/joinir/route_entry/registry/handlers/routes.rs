use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
use crate::mir::builder::control_flow::plan::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::plan::planner::PlanBuildOutcome;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::{
    planner_rule_route_label, PlanRuleId,
};
use crate::mir::builder::control_flow::plan::verifier::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::super::router::{lower_verified_core_plan, LoopRouteContext};
use super::super::types::{route_labels, PlannerFirstMode, RouterEnv, StandardEntry};
use super::super::utils::loop_break_recipe_needs_flowbox_adopt_tag_in_strict;
use super::{debug_log_recipe_entry, emit_planner_first};
use super::{
    release_allows_loop_cond_break_continue, release_allows_loop_cond_continue_only,
    release_skips_nested_loop, route_standard,
};

pub(crate) fn route_loop_break_recipe(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "LoopBreakRecipe requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
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
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "IfPhiJoin requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
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
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "LoopContinueOnly requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
    }
    emit_planner_first(
        PlannerFirstMode::StrictOrDev,
        env,
        PlanRuleId::LoopContinueOnly,
    );
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::LoopContinueOnly), env);
    if env.planner_required {
        if let Some(err) = composer::strict_nested_loop_guard(outcome, ctx) {
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

pub(crate) fn route_loop_true_early_exit(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopTrueEarlyExit),
        missing_contract_msg: "LoopTrueEarlyExit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_true_early_exit_recipe,
        planner_required_only: false,
        skip_without_contract: true,
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::LoopTrueEarlyExit),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_simple_while(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if detect_nested_loop(ctx.body) {
        return Ok(None);
    }
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopSimpleWhile),
        missing_contract_msg: "LoopSimpleWhile requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_simple_while_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopSimpleWhile),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Release,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_char_map(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::LOOP_CHAR_MAP,
        missing_contract_msg: "LoopCharMap requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_char_map_recipe,
        planner_required_only: true,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_array_join(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::LOOP_ARRAY_JOIN,
        missing_contract_msg: "LoopArrayJoin requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_array_join_recipe,
        planner_required_only: false,
        skip_without_contract: true,
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_scan_with_init(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::ScanWithInit),
        missing_contract_msg: "ScanWithInit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_scan_with_init_recipe,
        planner_required_only: false,
        skip_without_contract: true,
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::ScanWithInit),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_split_scan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::SplitScan),
        missing_contract_msg: "SplitScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_split_scan_recipe,
        planner_required_only: false,
        skip_without_contract: true,
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::SplitScan),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_bool_predicate_scan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::BoolPredicateScan),
        missing_contract_msg: "BoolPredicateScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_bool_predicate_scan_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::BoolPredicateScan),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_accum_const_loop(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(
            crate::mir::builder::control_flow::plan::planner::Freeze::contract(
                "AccumConstLoop requires recipe_contract in planner_required mode",
            )
            .to_string(),
        );
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

pub(crate) fn route_loop_scan_methods_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::SCAN_METHODS_V0,
        missing_contract_msg:
            "loop_scan_methods_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_scan_methods_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopSimpleWhile),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_scan_methods_block_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::SCAN_METHODS_BLOCK_V0,
        missing_contract_msg:
            "loop_scan_methods_block_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_scan_methods_block_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopSimpleWhile),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_scan_phi_vars_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::SCAN_PHI_VARS_V0,
        missing_contract_msg:
            "loop_scan_phi_vars_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_scan_phi_vars_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_scan_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::SCAN_V0,
        missing_contract_msg: "loop_scan_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_scan_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_collect_using_entries_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::COLLECT_USING_ENTRIES_V0,
        missing_contract_msg:
            "loop_collect_using_entries_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_collect_using_entries_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopSimpleWhile),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_nested_loop_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    debug_log_recipe_entry(route_labels::NESTED_LOOP_MINIMAL, env);
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.nested_loop_minimal().is_none() {
        return Ok(None);
    }

    let Some(core_plan) = composer::try_compose_core_loop_v2_nested_minimal(builder, facts, ctx)?
    else {
        if env.strict_or_dev {
            return Err(
                "nested_loop_minimal strict/dev route failed: compose rejected".to_string(),
            );
        }
        return Ok(None);
    };

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

pub(crate) fn route_loop_bundle_resolver_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::BUNDLE_RESOLVER_V0,
        missing_contract_msg:
            "loop_bundle_resolver_v0 requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_bundle_resolver_v0,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_true_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if release_skips_nested_loop(ctx, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopTrueBreak),
        missing_contract_msg:
            "loop_true_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_true_break_continue_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopTrueBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if !release_allows_loop_cond_break_continue(ctx, outcome, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondBreak),
        missing_contract_msg:
            "loop_cond_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_break_continue_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_only(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if !release_allows_loop_cond_continue_only(ctx, outcome, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueOnly),
        missing_contract_msg:
            "loop_cond_continue_only requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_only_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueOnly),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_with_return(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    if release_skips_nested_loop(ctx, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueWithReturn),
        missing_contract_msg:
            "loop_cond_continue_with_return requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_with_return_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueWithReturn),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

pub(crate) fn route_loop_cond_return_in_body(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondReturnInBody),
        missing_contract_msg:
            "loop_cond_return_in_body requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_return_in_body_recipe,
        planner_required_only: false,
        skip_without_contract: false,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondReturnInBody),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, outcome, env, &ENTRY)
}

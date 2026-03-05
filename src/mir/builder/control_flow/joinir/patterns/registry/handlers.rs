use crate::mir::builder::control_flow::plan::composer;
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakAcceptKind;
use crate::mir::builder::control_flow::plan::lowerer::PlanLowerer;
use crate::mir::builder::control_flow::plan::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::builder::control_flow::plan::verifier::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::router::{lower_verified_core_plan, LoopRouteContext};
use super::types::{PlannerFirstMode, RouterEnv, StandardEntry};
use super::utils::{emit_planner_first, loop_break_recipe_needs_flowbox_adopt_tag_in_strict};

fn route_standard(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
    entry: &StandardEntry,
) -> Result<Option<ValueId>, String> {
    if entry.planner_required_only && !env.planner_required {
        return Ok(None);
    }
    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(entry.missing_contract_msg).to_string());
    }
    if !env.planner_required && outcome.recipe_contract.is_none() && entry.skip_without_contract {
        return Ok(None);
    }

    if let Some(rule) = entry.plan_rule {
        emit_planner_first(entry.planner_first, env, rule);
    }

    let facts = outcome
        .facts
        .as_ref()
        .expect("facts present for route_standard");
    let core_plan = (entry.compose)(builder, facts, ctx).map_err(|freeze| freeze.to_string())?;
    let via = if env.strict_or_dev {
        entry.flowbox_via_strict
    } else {
        entry.flowbox_via_release
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

fn release_skips_nested_loop(ctx: &LoopRouteContext, env: &RouterEnv) -> bool {
    !env.planner_required && detect_nested_loop(ctx.body)
}

fn release_allows_loop_cond_break_continue(
    _ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> bool {
    if env.planner_required {
        return true;
    }
    let Some(facts) = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.loop_cond_break_continue.as_ref())
    else {
        return false;
    };
    // Release route allows nested-loop shapes only when loop_cond_break_continue
    // found an explicit exit-driven form. Keep passive cluster forms blocked.
    !matches!(
        facts.accept_kind,
        LoopCondBreakAcceptKind::NestedLoopOnly | LoopCondBreakAcceptKind::ProgramBlockNoExit
    )
}

fn release_allows_loop_scan_methods_block_v0(outcome: &PlanBuildOutcome, env: &RouterEnv) -> bool {
    if env.planner_required {
        return true;
    }
    let Some(facts) = outcome
        .facts
        .as_ref()
        .and_then(|facts| facts.facts.loop_scan_methods_block_v0.as_ref())
    else {
        return false;
    };
    !facts.recipe.segments.iter().any(|segment| {
        matches!(
            segment,
            crate::mir::builder::control_flow::plan::loop_scan_methods_block_v0::ScanSegment::NestedLoop(_)
        )
    })
}

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
    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:entry] loop_break_recipe: recipe-first");
    }

    let facts = outcome
        .facts
        .as_ref()
        .expect("loop_break_recipe facts present");
    let core_plan = RecipeComposer::compose_pattern2_break_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    if env.strict_or_dev {
        let loop_break_facts = facts
            .facts
            .pattern2_break
            .as_ref()
            .expect("loop_break_recipe is present");
        let needs_flowbox_tag = env.has_loopbodylocal
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
    use crate::config::env::joinir_dev;

    if env.planner_required && outcome.recipe_contract.is_none() {
        return Err(Freeze::contract(
            "IfPhiJoin requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first(PlannerFirstMode::StrictOrDev, env, PlanRuleId::IfPhiJoin);
    if joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0.log.debug("[recipe:entry] if_phi_join: recipe-first");
    }

    let facts = outcome.facts.as_ref().expect("if_phi_join facts present");
    let core_plan = RecipeComposer::compose_pattern3_ifphi_recipe(builder, facts, ctx)
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
        PlanRuleId::LoopContinueRecipe,
    );
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
    let core_plan = RecipeComposer::compose_pattern4_continue_recipe(builder, facts, ctx)
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
        missing_contract_msg:
            "LoopTrueEarlyExit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern5_infinite_early_exit_recipe,
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
    const ENTRY: StandardEntry = StandardEntry {
        missing_contract_msg:
            "LoopSimpleWhile requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern1_simple_while_recipe,
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
        missing_contract_msg: "LoopCharMap requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern1_char_map_recipe,
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
        missing_contract_msg: "LoopArrayJoin requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern1_array_join_recipe,
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
        missing_contract_msg:
            "ScanWithInit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern6_scan_with_init_recipe,
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
        missing_contract_msg: "SplitScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern7_split_scan_recipe,
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
        missing_contract_msg:
            "BoolPredicateScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_pattern8_bool_predicate_scan_recipe,
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
    if crate::config::env::joinir_dev::debug_enabled() {
        let ring0 = crate::runtime::get_global_ring0();
        ring0
            .log
            .debug("[recipe:entry] accum_const_loop: recipe-first");
    }

    let facts = outcome
        .facts
        .as_ref()
        .expect("accum_const_loop facts present");
    let core_plan = RecipeComposer::compose_pattern9_accum_const_loop_recipe(builder, facts, ctx)
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
    if !release_allows_loop_scan_methods_block_v0(outcome, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
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
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.pattern6_nested_minimal.is_none() {
        return Ok(None);
    }

    let Some(core_plan) =
        composer::try_compose_core_loop_v2_nested_minimal(builder, facts, ctx)?
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
    if release_skips_nested_loop(ctx, env) {
        return Ok(None);
    }

    const ENTRY: StandardEntry = StandardEntry {
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

pub(crate) fn route_generic_loop_v1(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.generic_loop_v1.is_none() {
        return Ok(None);
    }
    let core_plan = match RecipeComposer::compose_generic_loop_v1_recipe(builder, facts, ctx) {
        Ok(core_plan) => core_plan,
        Err(_err) if !env.strict_or_dev => return Ok(None),
        Err(err) => return Err(err.to_string()),
    };
    // In strict/dev, nested loops must emit the FlowBox shadow-adopt tag.
    if env.strict_or_dev && facts.nested_loop {
        return lower_verified_core_plan(
            builder,
            ctx,
            env.strict_or_dev,
            outcome.facts.as_ref(),
            core_plan,
            FlowboxVia::Shadow,
        );
    }
    if !env.strict_or_dev {
        if PlanVerifier::verify(&core_plan).is_err() {
            return Ok(None);
        }
        return match PlanLowerer::lower(builder, core_plan, ctx) {
            Ok(value) => Ok(value),
            Err(_) => Ok(None),
        };
    }
    // Preserve the pre-plan adopt behavior for non-nested generic loops.
    PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
    PlanLowerer::lower(builder, core_plan, ctx)
}

pub(crate) fn route_generic_loop_v0(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    outcome: &PlanBuildOutcome,
    env: &RouterEnv,
) -> Result<Option<ValueId>, String> {
    let Some(facts) = outcome.facts.as_ref() else {
        return Ok(None);
    };
    if facts.facts.generic_loop_v0.is_none() {
        return Ok(None);
    }
    let core_plan = match RecipeComposer::compose_generic_loop_v0_recipe(builder, facts, ctx) {
        Ok(core_plan) => core_plan,
        Err(_err) if !env.strict_or_dev => return Ok(None),
        Err(err) => return Err(err.to_string()),
    };
    if env.strict_or_dev && facts.nested_loop {
        return lower_verified_core_plan(
            builder,
            ctx,
            env.strict_or_dev,
            outcome.facts.as_ref(),
            core_plan,
            FlowboxVia::Shadow,
        );
    }
    if !env.strict_or_dev {
        if PlanVerifier::verify(&core_plan).is_err() {
            return Ok(None);
        }
        return match PlanLowerer::lower(builder, core_plan, ctx) {
            Ok(value) => Ok(value),
            Err(_) => Ok(None),
        };
    }
    PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
    PlanLowerer::lower(builder, core_plan, ctx)
}

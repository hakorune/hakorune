use crate::mir::builder::control_flow::lower::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::lower::PlanLowerer;
use crate::mir::builder::control_flow::lower::{planner_rule_route_label, PlanRuleId};
use crate::mir::builder::control_flow::plan::composer::{
    strict_nested_loop_guard_from_observations, try_compose_core_loop_v2_nested_minimal,
};
use crate::mir::builder::control_flow::plan::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::{self, FlowboxVia};
use crate::mir::builder::control_flow::verify::PlanVerifier;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::super::router::{lower_verified_core_plan, LoopRouteContext};
use super::super::execution_witness::{RouteAttemptOutcomeV1, RouteExecutionAttemptV1};
use super::super::types::{
    route_labels, PlannerFirstMode, SharedAbsentContractDeclineRouteV1, StandardEntry,
};
use super::super::utils::loop_break_recipe_needs_flowbox_adopt_tag_in_strict;
use super::{debug_log_recipe_entry, emit_planner_first_from_attempt};
use super::{
    release_allows_loop_cond_break_continue, release_allows_loop_cond_continue_only,
    release_skips_nested_loop, route_standard,
};

pub(crate) fn route_loop_break_recipe(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if attempt.planner_required() && !attempt.recipe_contract_present() {
        return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
            "LoopBreakRecipe requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first_from_attempt(
        PlannerFirstMode::StrictOrDev,
        attempt,
        PlanRuleId::LoopBreakRecipe,
    );
    debug_log_recipe_entry(
        planner_rule_route_label(PlanRuleId::LoopBreakRecipe),
        attempt,
    );

    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::SelectedFactsUnavailable,
        ));
    };
    let core_plan = RecipeComposer::compose_loop_break_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    if attempt.strict_or_dev() {
        let loop_break_facts = facts
            .facts
            .loop_break()
            .expect("loop_break_recipe is present");
        let needs_flowbox_tag = attempt.has_body_local()
            || loop_break_recipe_needs_flowbox_adopt_tag_in_strict(loop_break_facts);

        if needs_flowbox_tag {
            return lower_verified_core_plan(
                builder,
                ctx,
                attempt.strict_or_dev(),
                compose_facts,
                core_plan,
                FlowboxVia::Shadow,
            )
            .and_then(RouteAttemptOutcomeV1::from_selected_loop_option);
        }

        if !matches!(
            &core_plan,
            crate::mir::builder::control_flow::lower::CorePlan::Loop(_)
        ) {
            return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
                "selected LoopBreakRecipe produced a non-Loop CorePlan root",
            )
            .to_string());
        }
        PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
        return PlanLowerer::lower(builder, core_plan, ctx)
            .and_then(RouteAttemptOutcomeV1::from_selected_loop_option);
    }

    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        FlowboxVia::Release,
    )
    .and_then(RouteAttemptOutcomeV1::from_selected_loop_option)
}

pub(crate) fn route_if_phi_join(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if attempt.planner_required() && !attempt.recipe_contract_present() {
        return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
            "IfPhiJoin requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first_from_attempt(
        PlannerFirstMode::StrictOrDev,
        attempt,
        PlanRuleId::IfPhiJoin,
    );
    debug_log_recipe_entry(planner_rule_route_label(PlanRuleId::IfPhiJoin), attempt);

    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::SelectedFactsUnavailable,
        ));
    };
    let core_plan = RecipeComposer::compose_if_phi_join_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    let via = if attempt.strict_or_dev() {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        via,
    )
    .and_then(RouteAttemptOutcomeV1::from_selected_loop_option)
}

pub(crate) fn route_loop_continue_only(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if attempt.planner_required() && !attempt.recipe_contract_present() {
        return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
            "LoopContinueOnly requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first_from_attempt(
        PlannerFirstMode::StrictOrDev,
        attempt,
        PlanRuleId::LoopContinueOnly,
    );
    debug_log_recipe_entry(
        planner_rule_route_label(PlanRuleId::LoopContinueOnly),
        attempt,
    );
    if attempt.planner_required() {
        if let Some(err) = strict_nested_loop_guard_from_observations(
            compose_facts,
            attempt.recipe_contract_present(),
            ctx,
        ) {
            flowbox_tags::emit_flowbox_freeze_tag_from_facts(
                attempt.strict_or_dev(),
                "unstructured",
                compose_facts,
            );
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!("{}", err));
            return Err(err);
        }
    }

    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::SelectedFactsUnavailable,
        ));
    };
    let core_plan = RecipeComposer::compose_loop_continue_only_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;
    let via = if attempt.strict_or_dev() {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        via,
    )
    .and_then(RouteAttemptOutcomeV1::from_selected_loop_option)
}

pub(crate) fn route_loop_true_early_exit(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopTrueEarlyExit),
        missing_contract_msg: "LoopTrueEarlyExit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_true_early_exit_recipe,
        planner_required_only: false,
        absent_contract_decline: Some(SharedAbsentContractDeclineRouteV1::LoopTrueEarlyExit),
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::LoopTrueEarlyExit),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_simple_while(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if detect_nested_loop(ctx.body) {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            super::super::execution_witness::PreEffectDeclineReasonV1::NestedLoopShapeUnavailable,
        ));
    }
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopSimpleWhile),
        missing_contract_msg: "LoopSimpleWhile requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_simple_while_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopSimpleWhile),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Release,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_char_map(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::LOOP_CHAR_MAP,
        missing_contract_msg: "LoopCharMap requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_char_map_recipe,
        planner_required_only: true,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_array_join(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: route_labels::LOOP_ARRAY_JOIN,
        missing_contract_msg: "LoopArrayJoin requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_array_join_recipe,
        planner_required_only: false,
        absent_contract_decline: Some(SharedAbsentContractDeclineRouteV1::LoopArrayJoin),
        planner_first: PlannerFirstMode::Never,
        plan_rule: None,
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_scan_with_init(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::ScanWithInit),
        missing_contract_msg: "ScanWithInit requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_scan_with_init_recipe,
        planner_required_only: false,
        absent_contract_decline: Some(SharedAbsentContractDeclineRouteV1::ScanWithInit),
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::ScanWithInit),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_split_scan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::SplitScan),
        missing_contract_msg: "SplitScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_split_scan_recipe,
        planner_required_only: false,
        absent_contract_decline: Some(SharedAbsentContractDeclineRouteV1::SplitScan),
        planner_first: PlannerFirstMode::StrictOrDevPlannerRequired,
        plan_rule: Some(PlanRuleId::SplitScan),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_bool_predicate_scan(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::BoolPredicateScan),
        missing_contract_msg: "BoolPredicateScan requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_bool_predicate_scan_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::BoolPredicateScan),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_accum_const_loop(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if attempt.planner_required() && !attempt.recipe_contract_present() {
        return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
            "AccumConstLoop requires recipe_contract in planner_required mode",
        )
        .to_string());
    }
    emit_planner_first_from_attempt(
        PlannerFirstMode::StrictOrDev,
        attempt,
        PlanRuleId::AccumConstLoop,
    );
    debug_log_recipe_entry(
        planner_rule_route_label(PlanRuleId::AccumConstLoop),
        attempt,
    );

    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::SelectedFactsUnavailable,
        ));
    };
    let core_plan = RecipeComposer::compose_accum_const_loop_recipe(builder, facts, ctx)
        .map_err(|freeze| freeze.to_string())?;

    if attempt.strict_or_dev() {
        if !matches!(
            &core_plan,
            crate::mir::builder::control_flow::lower::CorePlan::Loop(_)
        ) {
            return Err(crate::mir::builder::control_flow::lower::Freeze::contract(
                "selected AccumConstLoop produced a non-Loop CorePlan root",
            )
            .to_string());
        }
        PlanVerifier::verify(&core_plan).map_err(|e| e.to_string())?;
        return PlanLowerer::lower(builder, core_plan, ctx)
            .and_then(RouteAttemptOutcomeV1::from_selected_loop_option);
    }

    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        FlowboxVia::Release,
    )
    .and_then(RouteAttemptOutcomeV1::from_selected_loop_option)
}

pub(crate) fn route_nested_loop_minimal(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    debug_log_recipe_entry(route_labels::NESTED_LOOP_MINIMAL, attempt);
    let Some(facts) = compose_facts else {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            super::super::execution_witness::PreEffectDeclineReasonV1::NestedLoopFactsUnavailable,
        ));
    };
    if facts.facts.nested_loop_minimal().is_none() {
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            super::super::execution_witness::PreEffectDeclineReasonV1::NestedLoopFactsUnavailable,
        ));
    }

    let Some(core_plan) = try_compose_core_loop_v2_nested_minimal(builder, facts, ctx)? else {
        if attempt.strict_or_dev() {
            return Err(
                "nested_loop_minimal strict/dev route failed: compose rejected".to_string(),
            );
        }
        return Ok(RouteAttemptOutcomeV1::PreEffectDeclined(
            super::super::execution_witness::PreEffectDeclineReasonV1::NestedComposerUnavailable,
        ));
    };

    let via = if attempt.strict_or_dev() {
        FlowboxVia::Shadow
    } else {
        FlowboxVia::Release
    };
    lower_verified_core_plan(
        builder,
        ctx,
        attempt.strict_or_dev(),
        compose_facts,
        core_plan,
        via,
    )
    .and_then(RouteAttemptOutcomeV1::from_selected_loop_option)
}

pub(crate) fn route_loop_true_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if release_skips_nested_loop(ctx, attempt) {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::ReleaseNestedLoopGate,
        ));
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopTrueBreak),
        missing_contract_msg:
            "loop_true_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_true_break_continue_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopTrueBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_cond_break_continue(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if !release_allows_loop_cond_break_continue(ctx, compose_facts, attempt) {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::ReleaseLoopCondGate,
        ));
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondBreak),
        missing_contract_msg:
            "loop_cond_break_continue requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_break_continue_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondBreak),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_only(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if !release_allows_loop_cond_continue_only(ctx, compose_facts, attempt) {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::ReleaseLoopCondGate,
        ));
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueOnly),
        missing_contract_msg:
            "loop_cond_continue_only requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_only_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueOnly),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_cond_continue_with_return(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    if release_skips_nested_loop(ctx, attempt) {
        return Ok(RouteAttemptOutcomeV1::PreEffectBlocked(
            super::super::execution_witness::PreEffectBlockedReasonV1::ReleaseNestedLoopGate,
        ));
    }

    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondContinueWithReturn),
        missing_contract_msg:
            "loop_cond_continue_with_return requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_continue_with_return_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondContinueWithReturn),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

pub(crate) fn route_loop_cond_return_in_body(
    builder: &mut MirBuilder,
    ctx: &LoopRouteContext,
    compose_facts: Option<&CanonicalLoopFacts>,
    attempt: &RouteExecutionAttemptV1<'_, '_>,
) -> Result<RouteAttemptOutcomeV1<ValueId>, String> {
    const ENTRY: StandardEntry = StandardEntry {
        route_label: planner_rule_route_label(PlanRuleId::LoopCondReturnInBody),
        missing_contract_msg:
            "loop_cond_return_in_body requires recipe_contract in planner_required mode",
        compose: RecipeComposer::compose_loop_cond_return_in_body_recipe,
        planner_required_only: false,
        absent_contract_decline: None,
        planner_first: PlannerFirstMode::StrictOrDev,
        plan_rule: Some(PlanRuleId::LoopCondReturnInBody),
        flowbox_via_strict: FlowboxVia::Shadow,
        flowbox_via_release: FlowboxVia::Shadow,
    };
    route_standard(builder, ctx, compose_facts, attempt, &ENTRY)
}

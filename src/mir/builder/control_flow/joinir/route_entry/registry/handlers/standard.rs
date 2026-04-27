use crate::mir::builder::control_flow::facts::feature_facts::detect_nested_loop;
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
use crate::mir::builder::control_flow::lower::{
    planner_rule_route_label, PlanBuildOutcome, PlanRuleId,
};
use crate::mir::builder::control_flow::plan::recipe_tree::RecipeComposer;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

use super::super::types::{route_labels, PlannerFirstMode, RouterEnv, StandardEntry};
use super::common::route_standard;

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

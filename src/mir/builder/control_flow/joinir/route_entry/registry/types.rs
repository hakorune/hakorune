use super::super::router::LoopRouteContext;
use crate::mir::builder::control_flow::plan::normalize::CanonicalLoopFacts;
use crate::mir::builder::control_flow::verify::observability::flowbox_tags::FlowboxVia;
use crate::mir::builder::control_flow::plan::planner::{Freeze, PlanBuildOutcome};
use crate::mir::builder::control_flow::plan::single_planner::PlanRuleId;
use crate::mir::builder::control_flow::plan::CorePlan;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;

pub(crate) mod entry_keys {
    pub(crate) const LOOP_BREAK_RECIPE: &str = "loop_break_recipe";
    pub(crate) const IF_PHI_JOIN: &str = "if_phi_join";
    pub(crate) const LOOP_CONTINUE_ONLY: &str = "loop_continue_only";
    pub(crate) const LOOP_TRUE_EARLY_EXIT: &str = "loop_true_early_exit";
    pub(crate) const LOOP_SIMPLE_WHILE: &str = "loop_simple_while";
    pub(crate) const LOOP_CHAR_MAP: &str = "loop_char_map";
    pub(crate) const LOOP_ARRAY_JOIN: &str = "loop_array_join";
    pub(crate) const SCAN_WITH_INIT: &str = "scan_with_init";
    pub(crate) const SPLIT_SCAN: &str = "split_scan";
    pub(crate) const BOOL_PREDICATE_SCAN: &str = "bool_predicate_scan";
    pub(crate) const ACCUM_CONST_LOOP: &str = "accum_const_loop";
    pub(crate) const LOOP_SCAN_METHODS_V0: &str = "loop_scan_methods_v0";
    pub(crate) const LOOP_SCAN_METHODS_BLOCK_V0: &str = "loop_scan_methods_block_v0";
    pub(crate) const LOOP_SCAN_PHI_VARS_V0: &str = "loop_scan_phi_vars_v0";
    pub(crate) const LOOP_SCAN_V0: &str = "loop_scan_v0";
    pub(crate) const LOOP_COLLECT_USING_ENTRIES_V0: &str = "loop_collect_using_entries_v0";
    pub(crate) const NESTED_LOOP_MINIMAL: &str = "nested_loop_minimal";
    pub(crate) const LOOP_BUNDLE_RESOLVER_V0: &str = "loop_bundle_resolver_v0";
    pub(crate) const LOOP_TRUE_BREAK_CONTINUE: &str = "loop_true_break_continue";
    pub(crate) const LOOP_COND_BREAK_CONTINUE: &str = "loop_cond_break_continue";
    pub(crate) const LOOP_COND_CONTINUE_ONLY: &str = "loop_cond_continue_only";
    pub(crate) const LOOP_COND_CONTINUE_WITH_RETURN: &str = "loop_cond_continue_with_return";
    pub(crate) const LOOP_COND_RETURN_IN_BODY: &str = "loop_cond_return_in_body";
    pub(crate) const GENERIC_LOOP_V0: &str = "generic_loop_v0";
    pub(crate) const GENERIC_LOOP_V1: &str = "generic_loop_v1";
}

pub(crate) mod route_labels {
    pub(crate) const LOOP_CHAR_MAP: &str = "loop_char_map";
    pub(crate) const LOOP_ARRAY_JOIN: &str = "loop_array_join";
    pub(crate) const SCAN_METHODS_V0: &str = "scan_methods_v0";
    pub(crate) const SCAN_METHODS_BLOCK_V0: &str = "scan_methods_block_v0";
    pub(crate) const SCAN_PHI_VARS_V0: &str = "scan_phi_vars_v0";
    pub(crate) const SCAN_V0: &str = "scan_v0";
    pub(crate) const COLLECT_USING_ENTRIES_V0: &str = "collect_using_entries_v0";
    pub(crate) const NESTED_LOOP_MINIMAL: &str = "nested_loop_minimal";
    pub(crate) const BUNDLE_RESOLVER_V0: &str = "bundle_resolver_v0";
    pub(crate) const GENERIC_LOOP_V0: &str = "generic_loop_v0";
    pub(crate) const GENERIC_LOOP_V1: &str = "generic_loop_v1";
}

pub(crate) struct RouterEnv {
    pub strict_or_dev: bool,
    pub planner_required: bool,
    pub has_body_local: bool,
}

pub(crate) type PredicateFn = fn(&CanonicalLoopFacts) -> bool;
pub(crate) type RouteFn = fn(
    &mut MirBuilder,
    &LoopRouteContext,
    &PlanBuildOutcome,
    &RouterEnv,
) -> Result<Option<ValueId>, String>;

pub(crate) struct Entry {
    pub name: &'static str,
    pub predicate: PredicateFn,
    pub route: Option<RouteFn>,
}

#[derive(Clone, Copy)]
pub(crate) enum PlannerFirstMode {
    Never,
    StrictOrDev,
    StrictOrDevPlannerRequired,
}

pub(crate) type ComposeFn =
    fn(&mut MirBuilder, &CanonicalLoopFacts, &LoopRouteContext) -> Result<CorePlan, Freeze>;

pub(crate) struct StandardEntry {
    pub route_label: &'static str,
    pub missing_contract_msg: &'static str,
    pub compose: ComposeFn,
    pub planner_required_only: bool,
    pub skip_without_contract: bool,
    pub planner_first: PlannerFirstMode,
    pub plan_rule: Option<PlanRuleId>,
    pub flowbox_via_strict: FlowboxVia,
    pub flowbox_via_release: FlowboxVia,
}

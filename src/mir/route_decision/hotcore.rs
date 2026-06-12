//! HotCore direct-exact route decision planning.

use super::RouteDecision;
use crate::mir::{MirFunction, MirModule};

pub fn refresh_module_hotcore_route_decisions(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        let fallback_policy = direct_exact_route_policy(function);
        function
            .metadata
            .route_decisions
            .retain(|decision| decision.source_plan_kind != "DirectExactHotCoreCallPlan");
        function.metadata.route_decisions.extend(
            function
                .metadata
                .direct_exact_hotcore_call_plans
                .iter()
                .map(|plan| {
                    RouteDecision::from_direct_exact_hotcore_call_plan(plan, fallback_policy)
                }),
        );
    }
}

/// Determine the fallback policy for direct-memory (array access) routes.
/// Called from `mod.rs::refresh_function_route_decisions`.
pub(super) fn direct_memory_route_policy(function: &MirFunction) -> &'static str {
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            region.relevant_access_policy == "direct_memory"
                && region.route_requirement == "fastpath_plan_required"
                && region.fallback_policy == "fail_fast"
        })
    {
        return "require_fastpath";
    }
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            region.relevant_access_policy == "direct_memory"
                && region.fallback_policy == "report_if_slow"
        })
    {
        return "report_if_slow";
    }
    "opportunistic"
}

fn direct_exact_route_policy(function: &MirFunction) -> &'static str {
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            is_direct_exact_region(region.relevant_access_policy)
                && is_direct_exact_requirement(region.route_requirement)
                && region.fallback_policy == "fail_fast"
        })
    {
        return "require_direct_exact";
    }
    if function
        .metadata
        .required_fastpath_regions
        .iter()
        .any(|region| {
            is_direct_exact_region(region.relevant_access_policy)
                && region.fallback_policy == "report_if_slow"
        })
    {
        return "report_if_slow";
    }
    "opportunistic"
}

fn is_direct_exact_region(relevant_access_policy: &str) -> bool {
    matches!(
        relevant_access_policy,
        "direct_exact" | "direct_exact_call" | "hotcore_call"
    )
}

fn is_direct_exact_requirement(route_requirement: &str) -> bool {
    matches!(
        route_requirement,
        "direct_exact_required" | "static_exact_call_required" | "fastpath_plan_required"
    )
}

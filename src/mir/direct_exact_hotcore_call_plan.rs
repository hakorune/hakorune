/*!
 * Report-only direct-exact call plans for selected HotCore call edges.
 *
 * This is the call-boundary companion to `HotCoreMethodSummaryV0`. It records
 * where existing user-box method route metadata already proves a static exact
 * call candidate. It does not inline bodies or change lowering.
 */

use std::collections::BTreeMap;

use crate::mir::core_method_op::LoweringPlanEmitKind;
use crate::mir::{BasicBlockId, MirModule, ValueId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectExactHotCoreCallPlan {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub caller: String,
    pub callee: String,
    pub box_name: String,
    pub method: String,
    pub receiver_value: ValueId,
    pub result_value: Option<ValueId>,
    pub receiver_exact: bool,
    pub same_module: bool,
    pub dispatch_policy: &'static str,
    pub call_boundary_policy: &'static str,
    pub return_shape: Option<&'static str>,
    pub value_demand: &'static str,
    pub callee_summary_status: &'static str,
    pub lowering_consumer_enabled: bool,
    pub generic_method_dispatch: bool,
    pub dynamic_route: bool,
    pub boxed_fallback: bool,
    pub summary: &'static str,
    pub failure_reason: Option<&'static str>,
}

pub fn refresh_module_direct_exact_hotcore_call_plans(module: &mut MirModule) {
    let summary_status = hotcore_summary_status_by_method(module);
    for function in module.functions.values_mut() {
        function.metadata.direct_exact_hotcore_call_plans.clear();
        let caller = function.signature.name.clone();
        for route in &function.metadata.user_box_method_routes {
            let callee = route.target_symbol().to_string();
            if !is_selected_direct_exact_hotcore_callee(&caller, &callee) {
                continue;
            }

            let direct_exact = route.lowering_emit_kind() == LoweringPlanEmitKind::DirectFunctionCall;
            let return_shape = route.return_shape();
            let scalar_return = return_shape == Some("scalar_i64");
            let callee_summary_status = summary_status
                .get(callee.as_str())
                .copied()
                .unwrap_or("not_required");
            let failure_reason =
                first_failure_reason(direct_exact, scalar_return, callee_summary_status);

            function
                .metadata
                .direct_exact_hotcore_call_plans
                .push(DirectExactHotCoreCallPlan {
                    block: route.block(),
                    instruction_index: route.instruction_index(),
                    caller: caller.clone(),
                    callee,
                    box_name: route.box_name().to_string(),
                    method: route.method().to_string(),
                    receiver_value: route.receiver_value(),
                    result_value: route.result_value(),
                    receiver_exact: route.type_id().is_some(),
                    same_module: route.target_exists(),
                    dispatch_policy: if direct_exact {
                        "static_exact"
                    } else {
                        "generic_or_dynamic"
                    },
                    call_boundary_policy: "thin_direct_call_candidate",
                    return_shape,
                    value_demand: route.value_demand(),
                    callee_summary_status,
                    lowering_consumer_enabled: false,
                    generic_method_dispatch: !direct_exact,
                    dynamic_route: !direct_exact,
                    boxed_fallback: return_shape
                        .map(|shape| shape == "object_handle" || shape == "string_handle")
                        .unwrap_or(true),
                    summary: if failure_reason.is_none() {
                        "ok"
                    } else {
                        "failed"
                    },
                    failure_reason,
                });
        }
    }
}

fn hotcore_summary_status_by_method(module: &MirModule) -> BTreeMap<String, &'static str> {
    let mut statuses = BTreeMap::new();
    for function in module.functions.values() {
        for summary in &function.metadata.hotcore_method_summaries {
            statuses.insert(summary.method.clone(), summary.summary);
        }
    }
    statuses
}

fn is_selected_direct_exact_hotcore_callee(caller: &str, callee: &str) -> bool {
    matches!(
        callee,
        "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1"
            | "HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock/2"
            | "HakoAllocPageModel.acquireFreshSmall/1"
            | "HakoAllocPageModel.releaseLocalKnownLive/1"
    ) && (caller == "Main.runOne/2"
        || caller.starts_with("HakoAllocObjectLifecycleHotCore."))
}

fn first_failure_reason(
    direct_exact: bool,
    scalar_return: bool,
    callee_summary_status: &'static str,
) -> Option<&'static str> {
    if !direct_exact {
        return Some("route_not_static_exact");
    }
    if !scalar_return {
        return Some("return_not_scalar_i64");
    }
    if callee_summary_status == "failed" {
        return Some("callee_hotcore_summary_failed");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_callee_vocabulary_is_narrow() {
        assert!(is_selected_direct_exact_hotcore_callee(
            "Main.runOne/2",
            "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1"
        ));
        assert!(is_selected_direct_exact_hotcore_callee(
            "HakoAllocObjectLifecycleHotCore.objectLifecycleReleaseBlock/2",
            "HakoAllocPageModel.releaseLocalKnownLive/1"
        ));
        assert!(!is_selected_direct_exact_hotcore_callee(
            "Main.runOne/2",
            "HakoAllocPageModel.resetToFresh/0"
        ));
        assert!(!is_selected_direct_exact_hotcore_callee(
            "Other.run/0",
            "HakoAllocObjectLifecycleHotCore.objectLifecycleSmallAlloc/1"
        ));
    }
}

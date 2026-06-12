/*!
 * Planner-owned route decisions.
 *
 * RouteDecisionV0 is a report-only outcome view: planners may prefer fast
 * routes, but MIRBuilder must only preserve origins/spans/types and lowering
 * must consume the selected route instead of re-deciding policy.
 */

mod hotcore;
mod typed_object;

pub use hotcore::refresh_module_hotcore_route_decisions;
pub use typed_object::refresh_module_typed_object_exact_slot_route_decisions;

use crate::mir::direct_array_access_plan::{DirectArrayAccessOp, DirectArrayAccessPlan};
use crate::mir::direct_exact_hotcore_call_plan::DirectExactHotCoreCallPlan;
use crate::mir::map_lookup_fusion_plan::MapLookupFusionRoute;
use crate::mir::{BasicBlockId, MirFunction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RouteDecision {
    pub site_id: String,
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub semantic_op: &'static str,
    pub access_kind: &'static str,
    pub preferred_route: &'static str,
    pub selected_route: &'static str,
    pub fallback_route: &'static str,
    pub fallback_policy: &'static str,
    pub proof_ids: Vec<&'static str>,
    pub miss_reason: Option<&'static str>,
    pub source_plan_kind: &'static str,
    pub selected_i64_const: Option<i64>,
    pub selected_bool_const: Option<bool>,
    pub selected_lowering_form: Option<&'static str>,
    pub selected_bridge_symbol: Option<&'static str>,
    pub selected_slot: Option<u32>,
    pub selected_storage: Option<&'static str>,
    pub receiver_box_name: Option<String>,
    pub field_id: Option<String>,
}

impl RouteDecision {
    pub(super) fn from_direct_array_access_plan(
        plan: &DirectArrayAccessPlan,
        fallback_policy: &'static str,
    ) -> Self {
        let (semantic_op, access_kind, fallback_route) = match plan.op() {
            DirectArrayAccessOp::Load => (
                "ArrayGet",
                "direct_array_i64_load",
                "generic_array_get_helper",
            ),
            DirectArrayAccessOp::Store => (
                "ArraySet",
                "direct_array_i64_store",
                "generic_array_set_helper",
            ),
        };
        Self {
            site_id: format!("b{}.i{}", plan.block().as_u32(), plan.instruction_index()),
            block: plan.block(),
            instruction_index: plan.instruction_index(),
            semantic_op,
            access_kind,
            preferred_route: plan.route(),
            selected_route: plan.route(),
            fallback_route,
            fallback_policy,
            proof_ids: plan.proof_ids().to_vec(),
            miss_reason: None,
            source_plan_kind: "DirectArrayAccessPlan",
            selected_i64_const: None,
            selected_bool_const: None,
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }

    pub(super) fn from_direct_exact_hotcore_call_plan(
        plan: &DirectExactHotCoreCallPlan,
        fallback_policy: &'static str,
    ) -> Self {
        let selected_route = if plan.lowering_consumer_enabled {
            "static_exact_call"
        } else {
            "generic_method_dispatch"
        };
        let proof_ids = if plan.lowering_consumer_enabled {
            vec!["same_module", "static_exact", "scalar_return"]
        } else {
            Vec::new()
        };
        Self {
            site_id: format!("b{}.i{}", plan.block.as_u32(), plan.instruction_index),
            block: plan.block,
            instruction_index: plan.instruction_index,
            semantic_op: "MethodCall",
            access_kind: "hotcore_call",
            preferred_route: "static_exact_call",
            selected_route,
            fallback_route: "generic_method_dispatch",
            fallback_policy,
            proof_ids,
            miss_reason: plan.failure_reason,
            source_plan_kind: "DirectExactHotCoreCallPlan",
            selected_i64_const: None,
            selected_bool_const: None,
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }

    fn from_map_lookup_fusion_route(
        plan: &MapLookupFusionRoute,
        fallback_policy: &'static str,
        access_kind: &'static str,
        fallback_route: &'static str,
        instruction_index: usize,
        semantic_op: &'static str,
    ) -> Self {
        let selected_route = if plan.stored_value_proof_tag() != "unknown_scalar" {
            "map_lookup_const_fold"
        } else {
            fallback_route
        };
        let mut proof_ids = vec![plan.proof_tag()];
        if plan.stored_value_proof_tag() != "unknown_scalar" {
            proof_ids.push(plan.stored_value_proof_tag());
        }
        Self {
            site_id: format!("b{}.i{}", plan.block().as_u32(), instruction_index),
            block: plan.block(),
            instruction_index,
            semantic_op,
            access_kind,
            preferred_route: "map_lookup_const_fold",
            selected_route,
            fallback_route,
            fallback_policy,
            proof_ids,
            miss_reason: if selected_route == "map_lookup_const_fold" {
                None
            } else {
                Some("stored_value_proof_missing")
            },
            source_plan_kind: "MapLookupFusionRoute",
            selected_i64_const: if selected_route == "map_lookup_const_fold"
                && semantic_op == "MapGet"
            {
                plan.stored_value_const()
            } else {
                None
            },
            selected_bool_const: if selected_route == "map_lookup_const_fold"
                && semantic_op == "MapHas"
            {
                Some(true)
            } else {
                None
            },
            selected_lowering_form: None,
            selected_bridge_symbol: None,
            selected_slot: None,
            selected_storage: None,
            receiver_box_name: None,
            field_id: None,
        }
    }
}

pub fn refresh_function_route_decisions(function: &mut MirFunction) {
    let fallback_policy = hotcore::direct_memory_route_policy(function);

    let mut decisions = function
        .metadata
        .direct_array_access_plans
        .iter()
        .map(|plan| RouteDecision::from_direct_array_access_plan(plan, fallback_policy))
        .collect::<Vec<_>>();

    let map_lookup_decisions = function
        .metadata
        .map_lookup_fusion_routes
        .iter()
        .flat_map(|plan| {
            [
                RouteDecision::from_map_lookup_fusion_route(
                    plan,
                    "opportunistic",
                    "map_lookup_same_key_get",
                    "generic_map_get_helper",
                    plan.get_instruction_index(),
                    "MapGet",
                ),
                RouteDecision::from_map_lookup_fusion_route(
                    plan,
                    "opportunistic",
                    "map_lookup_same_key_has",
                    "generic_map_has_helper",
                    plan.has_instruction_index(),
                    "MapHas",
                ),
            ]
        });
    decisions.extend(map_lookup_decisions);
    decisions.sort_by_key(|decision| {
        (
            decision.block.as_u32(),
            decision.instruction_index,
            decision.source_plan_kind,
            decision.semantic_op,
            decision.access_kind,
        )
    });
    function.metadata.route_decisions = decisions;
}

#[cfg(test)]
mod tests;

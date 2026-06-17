use super::candidates::{
    is_i64_map_set_route, local_i64_key_map_shadow_receivers, map_storage_receiver_value,
    set_route_key_value_operands, LocalI64MapShadowCandidate,
};
use super::plans::{
    LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan,
    LocalMapStorageRealizationPlan, MapReprPlan,
};
use crate::mir::value_origin::{build_value_def_map, ValueDefMap};
use crate::mir::{MirFunction, MirModule, ValueId};
use std::collections::HashMap;

pub fn refresh_module_map_repr_plans(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_map_repr_plans(function);
    }
}

pub fn refresh_function_map_repr_plans(function: &mut MirFunction) {
    let mut plans = Vec::new();
    let local_i64_shadow_receivers = local_i64_key_map_shadow_receivers(function);
    let def_map = build_value_def_map(function);

    for route in &function.metadata.generic_method_routes {
        if let Some(plan) = MapReprPlan::generic_hash_runtime(route) {
            plans.push(plan);
        }
        let storage_receiver = map_storage_receiver_value(function, route);
        if local_i64_shadow_receivers.contains_key(&storage_receiver) {
            if let Some(plan) = MapReprPlan::local_i64_key_map_shadow(route, storage_receiver) {
                plans.push(plan);
            }
        }
    }

    plans.sort_by_key(|plan| (plan.block().as_u32(), plan.instruction_index()));
    function.metadata.local_map_storage_realization_plans =
        build_local_map_storage_realization_plans(&local_i64_shadow_receivers);
    function.metadata.local_i64_map_direct_storage_plans =
        build_local_i64_map_direct_storage_plans(&local_i64_shadow_receivers);
    function.metadata.local_i64_map_entry_value_tracking_plans =
        build_local_i64_map_entry_value_tracking_plans(
            function,
            &def_map,
            &local_i64_shadow_receivers,
        );
    function.metadata.map_repr_plans = plans;
}

fn build_local_map_storage_realization_plans(
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalMapStorageRealizationPlan> {
    let mut plans: Vec<_> = local_i64_candidates
        .iter()
        .map(|(receiver, candidate)| {
            LocalMapStorageRealizationPlan::local_i64_key_map(*receiver, candidate)
        })
        .collect();
    plans.sort_by_key(|plan| plan.receiver_value().as_u32());
    plans
}

fn build_local_i64_map_direct_storage_plans(
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalI64MapDirectStoragePlan> {
    let mut plans: Vec<_> = local_i64_candidates
        .iter()
        .map(|(receiver, candidate)| {
            LocalI64MapDirectStoragePlan::closed_world_i64_key_value_table(*receiver, candidate)
        })
        .collect();
    plans.sort_by_key(|plan| plan.receiver_value().as_u32());
    plans
}

fn build_local_i64_map_entry_value_tracking_plans(
    function: &MirFunction,
    def_map: &ValueDefMap,
    local_i64_candidates: &HashMap<ValueId, LocalI64MapShadowCandidate>,
) -> Vec<LocalI64MapEntryValueTrackingPlan> {
    let mut plans = Vec::new();
    for route in &function.metadata.generic_method_routes {
        let receiver = map_storage_receiver_value(function, route);
        if !local_i64_candidates.contains_key(&receiver) {
            continue;
        }
        if !is_i64_map_set_route(route) {
            continue;
        }
        let Some((key_value, value_value)) = set_route_key_value_operands(function, route) else {
            continue;
        };
        plans.push(LocalI64MapEntryValueTrackingPlan::from_set_site(
            function,
            def_map,
            route,
            receiver,
            key_value,
            value_value,
        ));
    }
    plans.sort_by_key(|plan| {
        (
            plan.receiver_value().as_u32(),
            plan.set_block().as_u32(),
            plan.set_instruction_index(),
        )
    });
    plans
}

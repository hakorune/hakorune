use super::super::generic_method_route_facts::{GenericMethodKeyRoute, GenericMethodValueDemand};
use super::super::generic_method_route_plan::GenericMethodRoute;
use super::super::{MirFunction, MirInstruction, ValueId};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub(super) struct LocalI64MapShadowCandidate {
    pub(super) i64_set_count: usize,
    pub(super) scalar_get_count: usize,
    disallowed_route_count: usize,
}

pub(super) fn local_i64_key_map_shadow_receivers(
    function: &MirFunction,
) -> HashMap<ValueId, LocalI64MapShadowCandidate> {
    let mut candidates: HashMap<ValueId, LocalI64MapShadowCandidate> = HashMap::new();

    for route in &function.metadata.generic_method_routes {
        if route.receiver_origin_box() != Some("MapBox") {
            continue;
        }
        let receiver = map_storage_receiver_value(function, route);
        let entry = candidates.entry(receiver).or_default();
        if is_i64_map_set_route(route) {
            entry.i64_set_count += 1;
        } else if is_scalar_i64_map_get_route(route) {
            entry.scalar_get_count += 1;
        } else if is_public_map_get_read_route(route) {
            // A later public read forces the generic fallback path for that site,
            // but it does not invalidate pre-publication scalar get candidates.
        } else {
            entry.disallowed_route_count += 1;
        }
    }

    candidates
        .into_iter()
        .filter_map(|(receiver, candidate)| {
            (candidate.i64_set_count > 0
                && candidate.scalar_get_count > 0
                && candidate.disallowed_route_count == 0)
                .then_some((receiver, candidate))
        })
        .collect()
}

pub(super) fn is_i64_map_set_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.set"
        && matches!(route.route_kind_tag(), "map_store_i64" | "map_store_any")
        && route.key_route().is_some_and(GenericMethodKeyRoute::is_i64)
}

fn is_scalar_i64_map_get_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.get" && route.route_kind_tag() == "map_load_scalar_i64"
}

fn is_public_map_get_read_route(route: &GenericMethodRoute) -> bool {
    route.route_id() == "generic_method.get"
        && route.route_kind_tag() == "map_load_any"
        && route.value_demand() == GenericMethodValueDemand::ReadRef
}

pub(super) fn map_storage_receiver_value(
    function: &MirFunction,
    route: &GenericMethodRoute,
) -> ValueId {
    if !is_i64_map_set_route(route) {
        return route.receiver_value();
    }
    let Some(block) = function.blocks.get(&route.block()) else {
        return route.receiver_value();
    };
    let Some(MirInstruction::Call { args, .. }) = block.instructions.get(route.instruction_index())
    else {
        return route.receiver_value();
    };
    let Some(first) = args.first().copied() else {
        return route.receiver_value();
    };
    if first == route.receiver_value() || Some(first) == route.key_value() {
        route.receiver_value()
    } else {
        first
    }
}

pub(super) fn set_route_key_value_operands(
    function: &MirFunction,
    route: &GenericMethodRoute,
) -> Option<(ValueId, ValueId)> {
    let block = function.blocks.get(&route.block())?;
    let MirInstruction::Call { args, .. } = block.instructions.get(route.instruction_index())?
    else {
        return None;
    };
    let first = args.first().copied()?;
    let offset = if first == route.receiver_value() || Some(first) != route.key_value() {
        1
    } else {
        0
    };
    let key = args.get(offset).copied()?;
    let value = args.get(offset + 1).copied()?;
    Some((key, value))
}

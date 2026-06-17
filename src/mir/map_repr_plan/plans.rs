use super::candidates::LocalI64MapShadowCandidate;
use crate::mir::generic_method_route_facts::const_i64_value;
use crate::mir::generic_method_route_plan::GenericMethodRoute;
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{MirFunction, ValueId};

pub use hakorune_mir_plans::map_repr_plan::{
    LocalI64MapDirectStoragePlan, LocalI64MapEntryValueTrackingPlan,
    LocalMapStorageRealizationPlan, MapReprKind, MapReprPlan,
};

pub(super) fn generic_hash_runtime_plan(route: &GenericMethodRoute) -> Option<MapReprPlan> {
    let receiver_origin_box = route.receiver_origin_box();
    if receiver_origin_box != Some("MapBox") {
        return None;
    }
    Some(MapReprPlan::new(
        route.block(),
        route.instruction_index(),
        "map_repr.generic_hash_runtime",
        MapReprKind::GenericHashRuntime,
        route.route_id(),
        route.route_kind_tag(),
        route.helper_symbol(),
        route.box_name().to_string(),
        receiver_origin_box.map(str::to_string),
        route.method().to_string(),
        route.receiver_value(),
        route.key_value(),
        route.result_value(),
        route.key_route().map(|route| route.as_metadata_name()),
        route.return_shape().map(|shape| shape.as_metadata_name()),
        route.value_demand().as_metadata_name(),
        route.publication_policy()
            .map(|policy| policy.as_metadata_name()),
        route.proof_tag(),
        route.lowering_tier().map(|tier| tier.as_json_name()),
    ))
}

pub(super) fn local_i64_key_map_shadow_plan(
    route: &GenericMethodRoute,
    receiver_value: ValueId,
) -> Option<MapReprPlan> {
    let receiver_origin_box = route.receiver_origin_box();
    if receiver_origin_box != Some("MapBox") {
        return None;
    }
    Some(MapReprPlan::new(
        route.block(),
        route.instruction_index(),
        "map_repr.local_i64_key_map_shadow",
        MapReprKind::LocalI64KeyMapShadow,
        route.route_id(),
        route.route_kind_tag(),
        route.helper_symbol(),
        route.box_name().to_string(),
        receiver_origin_box.map(str::to_string),
        route.method().to_string(),
        receiver_value,
        route.key_value(),
        route.result_value(),
        route.key_route().map(|route| route.as_metadata_name()),
        route.return_shape().map(|shape| shape.as_metadata_name()),
        route.value_demand().as_metadata_name(),
        route.publication_policy()
            .map(|policy| policy.as_metadata_name()),
        "local_i64_key_map_shadow",
        None,
    ))
}

pub(super) fn local_i64_key_map_storage_plan(
    receiver_value: ValueId,
    candidate: &LocalI64MapShadowCandidate,
) -> LocalMapStorageRealizationPlan {
    LocalMapStorageRealizationPlan::local_i64_key_map(
        receiver_value,
        candidate.i64_set_count,
        candidate.scalar_get_count,
    )
}

pub(super) fn closed_world_i64_key_value_table_plan(
    receiver_value: ValueId,
    candidate: &LocalI64MapShadowCandidate,
) -> LocalI64MapDirectStoragePlan {
    LocalI64MapDirectStoragePlan::closed_world_i64_key_value_table(
        receiver_value,
        candidate.i64_set_count,
        candidate.scalar_get_count,
    )
}

pub(super) fn entry_value_tracking_plan(
    function: &MirFunction,
    def_map: &ValueDefMap,
    route: &GenericMethodRoute,
    receiver_value: ValueId,
    key_value: ValueId,
    value_value: ValueId,
) -> LocalI64MapEntryValueTrackingPlan {
    LocalI64MapEntryValueTrackingPlan::from_parts(
        receiver_value,
        route.block(),
        route.instruction_index(),
        key_value,
        value_value,
        const_i64_value(function, def_map, key_value),
        const_i64_value(function, def_map, value_value),
    )
}

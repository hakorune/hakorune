use super::route_json::{
    build_array_getset_micro_seed_route_json, build_extern_call_route_json,
    build_global_call_route_json, build_lowering_plan_json, build_map_lookup_fusion_route_json,
    build_user_box_method_route_json, build_userbox_known_receiver_method_seed_route_json,
    build_userbox_loop_micro_seed_route_json,
};
use crate::mir::function::FunctionMetadata;
use crate::mir::MirFunction;
use serde_json::json;

pub(super) fn insert_route_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    f: &MirFunction,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "lowering_plan".to_string(),
        json!(build_lowering_plan_json(f)),
    );
    obj.insert(
        "extern_call_routes".to_string(),
        serde_json::Value::Array(
            metadata
                .extern_call_routes
                .iter()
                .map(build_extern_call_route_json)
                .collect(),
        ),
    );
    obj.insert(
        "global_call_routes".to_string(),
        serde_json::Value::Array(
            metadata
                .global_call_routes
                .iter()
                .map(build_global_call_route_json)
                .collect(),
        ),
    );
    obj.insert(
        "user_box_method_routes".to_string(),
        serde_json::Value::Array(
            metadata
                .user_box_method_routes
                .iter()
                .map(build_user_box_method_route_json)
                .collect(),
        ),
    );
    obj.insert(
        "array_getset_micro_seed_route".to_string(),
        metadata
            .array_getset_micro_seed_route
            .as_ref()
            .map(build_array_getset_micro_seed_route_json)
            .unwrap_or(serde_json::Value::Null),
    );
    obj.insert(
        "userbox_loop_micro_seed_route".to_string(),
        metadata
            .userbox_loop_micro_seed_route
            .as_ref()
            .map(build_userbox_loop_micro_seed_route_json)
            .unwrap_or(serde_json::Value::Null),
    );
    obj.insert(
        "userbox_known_receiver_method_seed_route".to_string(),
        metadata
            .userbox_known_receiver_method_seed_route
            .as_ref()
            .map(build_userbox_known_receiver_method_seed_route_json)
            .unwrap_or(serde_json::Value::Null),
    );
    obj.insert(
        "map_lookup_fusion_routes".to_string(),
        serde_json::Value::Array(
            metadata
                .map_lookup_fusion_routes
                .iter()
                .map(build_map_lookup_fusion_route_json)
                .collect(),
        ),
    );
}

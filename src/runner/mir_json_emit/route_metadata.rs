use super::route_json::{
    build_array_getset_micro_seed_route_json, build_extern_call_route_json,
    build_global_call_route_json, build_lowering_plan_json, build_map_lookup_fusion_route_json,
    build_user_box_method_route_json, build_userbox_known_receiver_method_seed_route_json,
    build_userbox_loop_micro_seed_route_json,
};
use crate::mir::function::FunctionMetadata;
use crate::mir::same_module_fusion_plan::SameModuleFusionPlan;
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
        "builtin_global_call_routes".to_string(),
        serde_json::Value::Array(
            metadata
                .builtin_global_call_routes
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
    obj.insert(
        "same_module_fusion_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .same_module_fusion_plans
                .iter()
                .map(build_same_module_fusion_plan_json)
                .collect(),
        ),
    );
    obj.insert(
        "same_module_function_definitions".to_string(),
        serde_json::Value::Array(
            metadata
                .same_module_definition_plans
                .iter()
                .map(build_same_module_definition_plan_json)
                .collect(),
        ),
    );
}

fn build_same_module_definition_plan_json(
    plan: &crate::mir::same_module_definition_plan::SameModuleDefinitionPlan,
) -> serde_json::Value {
    json!({
        "target_symbol": plan.target_symbol.as_str(),
        "definition_kind": plan.definition_kind.as_json_name(),
        "definition_owner": plan.definition_owner.as_str(),
        "source": plan.source.as_str(),
    })
}

fn build_same_module_fusion_plan_json(plan: &SameModuleFusionPlan) -> serde_json::Value {
    match plan {
        SameModuleFusionPlan::TypedFieldRmw(plan) => json!({
            "kind": plan.kind,
            "function": plan.function.as_str(),
            "block": plan.block.as_u32(),
            "get_instruction_index": plan.get_instruction_index,
            "binop_instruction_index": plan.binop_instruction_index,
            "set_instruction_index": plan.set_instruction_index,
            "skip_instruction_indices": plan.skip_instruction_indices.as_slice(),
            "get_dst": plan.get_dst.as_u32(),
            "binop_dst": plan.binop_dst.as_u32(),
            "box_reg": plan.box_reg.as_u32(),
            "field": plan.field.as_str(),
            "slot": plan.slot,
            "delta_reg": plan.delta_reg.as_u32(),
            "helper_symbol": plan.helper_symbol,
            "storage": plan.storage,
            "direct_use_count": plan.direct_use_count,
        }),
        SameModuleFusionPlan::ResultCapsuleResetBatch(plan) => json!({
            "kind": plan.kind,
            "function": plan.function.as_str(),
            "block": plan.block.as_u32(),
            "first_set_instruction_index": plan.first_set_instruction_index,
            "set_instruction_indices": plan.set_instruction_indices,
            "skip_instruction_indices": plan.skip_instruction_indices.as_slice(),
            "box_reg": plan.box_reg.as_u32(),
            "fields": plan.fields,
            "slots": plan.slots,
            "values": plan.values,
            "helper_symbol": plan.helper_symbol,
            "storage": plan.storage,
        }),
    }
}

use crate::mir::function::FunctionMetadata;
use serde_json::json;

pub(super) fn insert_plan_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "inline_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .inline_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "request": plan.request.as_str(),
                        "hotness": plan.hotness.as_ref().map(|hotness| hotness.as_str()),
                        "max_ir": plan.max_ir,
                        "requires": &plan.requires,
                        "verified": plan.verified,
                        "fallback": plan.fallback.as_str(),
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "effect_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .effect_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "requires": plan
                            .requires
                            .iter()
                            .map(|requirement| requirement.as_str())
                            .collect::<Vec<_>>(),
                        "verified": plan.verified,
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "capability_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .capability_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "allow": &plan.allow,
                        "verified": plan.verified,
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "map_repr_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .map_repr_plans
                .iter()
                .map(|plan| {
                    json!({
                        "route_id": plan.route_id(),
                        "repr_kind": plan.repr_kind_tag(),
                        "source_route_id": plan.source_route_id(),
                        "source_route_kind": plan.source_route_kind(),
                        "source_helper_symbol": plan.source_helper_symbol(),
                        "block": plan.block().as_u32(),
                        "instruction_index": plan.instruction_index(),
                        "surface_box_name": plan.surface_box_name(),
                        "receiver_origin_box": plan.receiver_origin_box(),
                        "method": plan.method(),
                        "receiver_value": plan.receiver_value().as_u32(),
                        "key_value": plan.key_value().map(|value| value.as_u32()),
                        "result_value": plan.result_value().map(|value| value.as_u32()),
                        "key_route": plan.key_route_tag(),
                        "return_shape": plan.return_shape_tag(),
                        "value_demand": plan.value_demand_tag(),
                        "publication_policy": plan.publication_policy_tag(),
                        "proof_tag": plan.proof_tag(),
                        "lowering_tier": plan.lowering_tier_tag(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "local_fastpath_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .local_fastpath_facts
                .iter()
                .map(|fact| {
                    json!({
                        "route_id": "local_fastpath.known_receiver_direct_call",
                        "fact_kind": "local_fastpath_fact",
                        "backend_kind": match fact.backend_kind {
                            crate::object_storage_plan::LocalFastPathKind::KnownReceiverDirectCall => "known_receiver_direct_call",
                            crate::object_storage_plan::LocalFastPathKind::LocalFieldAccess => "local_field_access",
                            crate::object_storage_plan::LocalFastPathKind::LocalStorageAccess => "local_storage_access",
                        },
                        "route_plan": fact.route_plan_label,
                        "site_id": fact.site_id.0,
                        "block": fact.block_id().0,
                        "instruction_index": fact.instruction_index().0,
                        "object_id": fact.object_id.0,
                        "receiver_value": fact.object_id.0,
                        "alias_class": fact.alias_class.0,
                        "route_plan_id": fact.route_plan.0,
                        "storage_plan_id": fact.storage_plan.map(|plan| plan.0),
                        "plan_epoch": fact.plan_epoch.0,
                        "valid_until_publication": fact.valid_until_publication,
                        "fallback_reason": serde_json::Value::Null,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "local_map_storage_realization_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .local_map_storage_realization_plans
                .iter()
                .map(|plan| {
                    json!({
                        "receiver_value": plan.receiver_value().as_u32(),
                        "representation": plan.representation(),
                        "candidate_set_count": plan.candidate_set_count(),
                        "candidate_scalar_get_count": plan.candidate_scalar_get_count(),
                        "publication_materialization_required": plan.publication_materialization_required(),
                        "backend_lowering_enabled": plan.backend_lowering_enabled(),
                        "runtime_helper_enabled": plan.runtime_helper_enabled(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "local_i64_map_direct_storage_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .local_i64_map_direct_storage_plans
                .iter()
                .map(|plan| {
                    json!({
                        "receiver_value": plan.receiver_value().as_u32(),
                        "representation": plan.representation(),
                        "known_i64_key_set_count": plan.known_i64_key_set_count(),
                        "scalar_get_count": plan.scalar_get_count(),
                        "entry_value_tracking_enabled": plan.entry_value_tracking_enabled(),
                        "publication_materialization_required": plan.publication_materialization_required(),
                        "backend_lowering_enabled": plan.backend_lowering_enabled(),
                        "runtime_helper_enabled": plan.runtime_helper_enabled(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "local_i64_map_entry_value_tracking_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .local_i64_map_entry_value_tracking_plans
                .iter()
                .map(|plan| {
                    json!({
                        "receiver_value": plan.receiver_value().as_u32(),
                        "set_block": plan.set_block().as_u32(),
                        "set_instruction_index": plan.set_instruction_index(),
                        "key_value": plan.key_value().as_u32(),
                        "value_value": plan.value_value().as_u32(),
                        "key_const_if_known": plan.key_const_if_known(),
                        "value_const_if_known": plan.value_const_if_known(),
                        "backend_lowering_enabled": plan.backend_lowering_enabled(),
                        "runtime_helper_enabled": plan.runtime_helper_enabled(),
                    })
                })
                .collect(),
        ),
    );
}

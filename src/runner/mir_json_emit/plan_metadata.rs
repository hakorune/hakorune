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
}

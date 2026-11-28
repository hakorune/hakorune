use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};
use serde_json::json;

pub(super) fn build_lowering_plan_json(f: &crate::mir::MirFunction) -> Vec<serde_json::Value> {
    let mut entries = f
        .metadata
        .generic_method_routes
        .iter()
        .filter_map(|route| {
            let (tier, emit_kind, core_op, proof) = if let Some(carrier) = route.core_method() {
                (
                    carrier.lowering_tier.plan_tier(),
                    carrier.lowering_tier.plan_emit_kind(),
                    carrier.op.to_string(),
                    carrier.proof.to_string(),
                )
            } else if route.route_id() == "generic_method.keys"
                && route.route_kind_tag() == "map_keys_array"
                && route.proof_tag() == "mir_json_flags_keys"
            {
                (
                    LoweringPlanTier::DirectAbi,
                    LoweringPlanEmitKind::DirectAbiCall,
                    "MapKeys".to_string(),
                    route.proof_tag().to_string(),
                )
            } else {
                return None;
            };
            Some(json!({
                "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
                "block": route.block().as_u32(),
                "instruction_index": route.instruction_index(),
                "source": "generic_method_routes",
                "source_route_id": route.route_id(),
                "core_op": core_op,
                "tier": tier.as_json_name(),
                "emit_kind": emit_kind.as_json_name(),
                "symbol": route.helper_symbol(),
                "proof": proof,
                "route_proof": route.proof_tag(),
                "route_kind": route.route_kind_tag(),
                "perf_proof": false,
                "receiver_value": route.receiver_value().as_u32(),
                "receiver_origin_box": route.receiver_origin_box(),
                "arg0_origin_box": route.arg0_origin_box(),
                "result_origin_box": route.result_origin_box(),
                "value_origin_box": route.value_origin_box(),
                "arity": route.arity(),
                "key_route": route.key_route().map(|key_route| key_route.to_string()),
                "key_const_text": route.key_const_text(),
                "key_value": route.key_value().map(|value| value.as_u32()),
                "result_value": route.result_value().map(|value| value.as_u32()),
                "return_shape": route.return_shape().map(|shape| shape.to_string()),
                "value_demand": route.value_demand().to_string(),
                "publication_policy": route.publication_policy().map(|policy| policy.to_string()),
                "effects": route.effect_tags(),
            }))
        })
        .collect::<Vec<_>>();

    entries.extend(f.metadata.extern_call_routes.iter().map(|route| {
        json!({
            "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
            "block": route.block().as_u32(),
            "instruction_index": route.instruction_index(),
            "source": "extern_call_routes",
            "source_route_id": route.route_id(),
            "source_symbol": route.source_symbol(),
            "core_op": route.core_op(),
            "tier": route.tier(),
            "emit_kind": route.emit_kind(),
            "symbol": route.symbol(),
            "proof": route.proof(),
            "route_proof": route.proof(),
            "route_kind": route.route_id(),
            "perf_proof": false,
            "arity": route.arity(),
            "key_value": route.key_value().as_u32(),
            "value_value": route.value_value().map(|value| value.as_u32()),
            "result_value": route.result_value_opt().map(|value| value.as_u32()),
            "return_shape": route.return_shape(),
            "value_demand": route.value_demand(),
            "abi_return_shape": route.abi_return_shape(),
            "bridge_encoding": route.bridge_encoding(),
            "semantic_result_policy": route.semantic_result_policy(),
            "value_use_policy": route.value_use_policy(),
            "required_capability": route.required_capability(),
            "semantic_activation": route.semantic_activation(),
            "backend_support_consumer": route.backend_support_consumer(),
            "publication_policy": serde_json::Value::Null,
            "effects": route.effect_tags(),
        })
    }));

    entries.extend(
        f.metadata
            .global_call_routes
            .iter()
            .map(|route| build_global_call_lowering_plan_entry(route)),
    );

    entries.extend(
        f.metadata
            .builtin_global_call_routes
            .iter()
            .map(|route| build_global_call_lowering_plan_entry(route)),
    );

    entries.extend(f.metadata.user_box_method_routes.iter().map(|route| {
        json!({
            "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
            "block": route.block().as_u32(),
            "instruction_index": route.instruction_index(),
            "source": "user_box_method_routes",
            "source_route_id": route.route_id(),
            "box_name": route.box_name(),
            "method": route.method(),
            "target_symbol": route.target_symbol(),
            "core_op": route.core_op(),
            "tier": route.lowering_tier().as_json_name(),
            "emit_kind": route.lowering_emit_kind().as_json_name(),
            "symbol": route.target_symbol(),
            "proof": route.proof(),
            "route_proof": route.proof(),
            "route_kind": route.route_kind(),
            "perf_proof": false,
            "arity": route.arity(),
            "target_exists": route.target_exists(),
            "target_arity": route.target_arity(),
            "target_return_type": route.target_return_type(),
            "target_result_box_name": route.target_result_box_name(),
            "target_body_supported": route.target_body_supported(),
            "arity_matches": route.arity_matches(),
            "receiver_value": route.receiver_value().as_u32(),
            "type_id": route.type_id(),
            "result_value": route.result_value().map(|value| value.as_u32()),
            "return_shape": route.return_shape(),
            "value_demand": route.value_demand(),
            "result_origin": route.result_origin(),
            "definition_owner": route.definition_owner(),
            "emit_trace_consumer": route.emit_trace_consumer(),
            "publication_policy": serde_json::Value::Null,
            "reason": route.reason(),
            "effects": route.effect_tags(),
        })
    }));

    entries
}

fn build_global_call_lowering_plan_entry(
    route: &crate::mir::global_call_route_plan::GlobalCallRoute,
) -> serde_json::Value {
    json!({
        "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
        "block": route.block().as_u32(),
        "instruction_index": route.instruction_index(),
        "source": "global_call_routes",
        "source_route_id": route.route_id(),
        "callee_name": route.callee_name(),
        "target_symbol": route.target_symbol(),
        "core_op": route.core_op(),
        "tier": route.lowering_tier().as_json_name(),
        "emit_kind": route.lowering_emit_kind().as_json_name(),
        "symbol": route
            .target_symbol()
            .map(serde_json::Value::from)
            .unwrap_or(serde_json::Value::Null),
        "proof": route.proof(),
        "route_proof": route.proof(),
        "route_kind": route.route_kind(),
        "perf_proof": false,
        "arity": route.arity(),
        "target_exists": route.target_exists(),
        "target_arity": route.target_arity(),
        "target_return_type": route.target_return_type(),
        "target_result_box_name": route.target_result_box_name(),
        "target_shape": route.target_shape(),
        "target_shape_reason": route.target_shape_reason(),
        "target_shape_blocker_symbol": route.target_shape_blocker_symbol(),
        "target_shape_blocker_reason": route.target_shape_blocker_reason(),
        "arity_matches": route.arity_matches(),
        "result_value": route.result_value().map(|value| value.as_u32()),
        "arg0_origin_box": route.arg0_origin_box(),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
        "result_origin": route.result_origin(),
        "need_kind": route.need_kind(),
        "definition_owner": route.definition_owner(),
        "emit_trace_consumer": route.emit_trace_consumer(),
        "publication_policy": serde_json::Value::Null,
        "reason": route.reason(),
        "reason_detail": route.reason_detail(),
        "reason_hint": route.reason_hint(),
        "effects": route.effect_tags(),
    })
}

pub(super) fn build_extern_call_route_json(
    route: &crate::mir::extern_call_route_plan::ExternCallRoute,
) -> serde_json::Value {
    json!({
        "route_id": route.route_id(),
        "block": route.block().as_u32(),
        "instruction_index": route.instruction_index(),
        "source_symbol": route.source_symbol(),
        "core_op": route.core_op(),
        "tier": route.lowering_tier().as_json_name(),
        "emit_kind": route.lowering_emit_kind().as_json_name(),
        "symbol": route.symbol(),
        "proof": route.proof(),
        "key_value": route.key_value().as_u32(),
        "value_value": route.value_value().map(|value| value.as_u32()),
        "result_value": route.result_value_opt().map(|value| value.as_u32()),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
        "abi_return_shape": route.abi_return_shape(),
        "bridge_encoding": route.bridge_encoding(),
        "semantic_result_policy": route.semantic_result_policy(),
        "value_use_policy": route.value_use_policy(),
        "required_capability": route.required_capability(),
        "semantic_activation": route.semantic_activation(),
        "backend_support_consumer": route.backend_support_consumer(),
        "effects": route.effect_tags(),
    })
}

pub(super) fn build_global_call_route_json(
    route: &crate::mir::global_call_route_plan::GlobalCallRoute,
) -> serde_json::Value {
    json!({
        "route_id": route.route_id(),
        "block": route.block().as_u32(),
        "instruction_index": route.instruction_index(),
        "callee_name": route.callee_name(),
        "target_symbol": route.target_symbol(),
        "core_op": route.core_op(),
        "tier": route.lowering_tier().as_json_name(),
        "emit_kind": route.lowering_emit_kind().as_json_name(),
        "symbol": route.target_symbol(),
        "proof": route.proof(),
        "route_kind": route.route_kind(),
        "arity": route.arity(),
        "target_exists": route.target_exists(),
        "target_arity": route.target_arity(),
        "target_return_type": route.target_return_type(),
        "target_result_box_name": route.target_result_box_name(),
        "target_shape": route.target_shape(),
        "target_shape_reason": route.target_shape_reason(),
        "target_shape_blocker_symbol": route.target_shape_blocker_symbol(),
        "target_shape_blocker_reason": route.target_shape_blocker_reason(),
        "arity_matches": route.arity_matches(),
        "result_value": route.result_value().map(|value| value.as_u32()),
        "arg0_origin_box": route.arg0_origin_box(),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
        "result_origin": route.result_origin(),
        "need_kind": route.need_kind(),
        "definition_owner": route.definition_owner(),
        "emit_trace_consumer": route.emit_trace_consumer(),
        "reason": route.reason(),
        "reason_detail": route.reason_detail(),
        "reason_hint": route.reason_hint(),
        "effects": route.effect_tags(),
    })
}

pub(super) fn build_user_box_method_route_json(
    route: &crate::mir::user_box_method_route_plan::UserBoxMethodRoute,
) -> serde_json::Value {
    json!({
        "route_id": route.route_id(),
        "block": route.block().as_u32(),
        "instruction_index": route.instruction_index(),
        "box_name": route.box_name(),
        "method": route.method(),
        "target_symbol": route.target_symbol(),
        "core_op": route.core_op(),
        "tier": route.lowering_tier().as_json_name(),
        "emit_kind": route.lowering_emit_kind().as_json_name(),
        "symbol": route.target_symbol(),
        "proof": route.proof(),
        "route_kind": route.route_kind(),
        "arity": route.arity(),
        "target_exists": route.target_exists(),
        "target_arity": route.target_arity(),
        "target_return_type": route.target_return_type(),
        "target_result_box_name": route.target_result_box_name(),
        "target_body_supported": route.target_body_supported(),
        "arity_matches": route.arity_matches(),
        "receiver_value": route.receiver_value().as_u32(),
        "type_id": route.type_id(),
        "result_value": route.result_value().map(|value| value.as_u32()),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
        "result_origin": route.result_origin(),
        "definition_owner": route.definition_owner(),
        "emit_trace_consumer": route.emit_trace_consumer(),
        "reason": route.reason(),
        "effects": route.effect_tags(),
    })
}

pub(super) fn build_map_lookup_fusion_route_json(
    route: &crate::mir::map_lookup_fusion_plan::MapLookupFusionRoute,
) -> serde_json::Value {
    json!({
        "route_id": route.route_id(),
        "block": route.block().as_u32(),
        "get_instruction_index": route.get_instruction_index(),
        "has_instruction_index": route.has_instruction_index(),
        "fusion_op": route.fusion_op_tag(),
        "receiver_origin_box": route.receiver_origin_box(),
        "receiver_value": route.receiver_value().as_u32(),
        "key_value": route.key_value().as_u32(),
        "key_const": route.key_const(),
        "key_route": route.key_route_tag(),
        "get_result_value": route.get_result_value().as_u32(),
        "has_result_value": route.has_result_value().as_u32(),
        "get_return_shape": route.get_return_shape_tag(),
        "get_value_demand": route.get_value_demand_tag(),
        "get_publication_policy": route.get_publication_policy_tag(),
        "has_result_shape": route.has_result_shape(),
        "stored_value_proof": route.stored_value_proof_tag(),
        "stored_value_const": route.stored_value_const(),
        "stored_value_known_nonzero": route.stored_value_known_nonzero(),
        "proof": route.proof_tag(),
        "lowering_tier": route.lowering_tier_tag(),
        "effects": ["read.key", "probe.key", "metadata.only"],
    })
}

use crate::mir::core_method_op::CoreMethodLoweringTier;
use crate::mir::userbox_known_receiver_method_seed_plan::{
    UserBoxKnownReceiverMethodSeedKind, UserBoxKnownReceiverMethodSeedPayload,
};
use serde_json::json;

pub(super) fn build_lowering_plan_json(f: &crate::mir::MirFunction) -> Vec<serde_json::Value> {
    let mut entries = f
        .metadata
        .generic_method_routes
        .iter()
        .filter_map(|route| {
            let carrier = route.core_method()?;
            let (tier, emit_kind) = match carrier.lowering_tier {
                CoreMethodLoweringTier::WarmDirectAbi => ("DirectAbi", "direct_abi_call"),
                CoreMethodLoweringTier::ColdFallback => ("ColdRuntime", "runtime_call"),
            };
            Some(json!({
                "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
                "block": route.block().as_u32(),
                "instruction_index": route.instruction_index(),
                "source": "generic_method_routes",
                "source_route_id": route.route_id(),
                "core_op": carrier.op.to_string(),
                "tier": tier,
                "emit_kind": emit_kind,
                "symbol": route.helper_symbol(),
                "proof": carrier.proof.to_string(),
                "route_proof": route.proof_tag(),
                "route_kind": route.route_kind_tag(),
                "perf_proof": false,
                "receiver_value": route.receiver_value().as_u32(),
                "receiver_origin_box": route.receiver_origin_box(),
                "arity": route.arity(),
                "key_route": route.key_route().map(|key_route| key_route.to_string()),
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
            "arity": 1,
            "key_value": route.key_value().as_u32(),
            "result_value": route.result_value().as_u32(),
            "return_shape": route.return_shape(),
            "value_demand": route.value_demand(),
            "publication_policy": serde_json::Value::Null,
            "effects": route.effect_tags(),
        })
    }));

    entries.extend(f.metadata.global_call_routes.iter().map(|route| {
        json!({
            "site": format!("b{}.i{}", route.block().as_u32(), route.instruction_index()),
            "block": route.block().as_u32(),
            "instruction_index": route.instruction_index(),
            "source": "global_call_routes",
            "source_route_id": route.route_id(),
            "callee_name": route.callee_name(),
            "target_symbol": route.target_symbol(),
            "core_op": route.core_op(),
            "tier": route.tier(),
            "emit_kind": route.emit_kind(),
            "symbol": if route.tier() == "DirectAbi" {
                route.target_symbol().map(serde_json::Value::from).unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            },
            "proof": route.proof(),
            "route_proof": route.proof(),
            "route_kind": route.route_kind(),
            "perf_proof": false,
            "arity": route.arity(),
            "target_exists": route.target_exists(),
            "target_arity": route.target_arity(),
            "target_return_type": route.target_return_type(),
            "target_shape": route.target_shape(),
            "target_shape_reason": route.target_shape_reason(),
            "target_shape_blocker_symbol": route.target_shape_blocker_symbol(),
            "target_shape_blocker_reason": route.target_shape_blocker_reason(),
            "arity_matches": route.arity_matches(),
            "result_value": route.result_value().map(|value| value.as_u32()),
            "return_shape": route.return_shape(),
            "value_demand": route.value_demand(),
            "publication_policy": serde_json::Value::Null,
            "reason": route.reason(),
            "effects": route.effect_tags(),
        })
    }));

    entries
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
        "tier": route.tier(),
        "emit_kind": route.emit_kind(),
        "symbol": route.symbol(),
        "proof": route.proof(),
        "key_value": route.key_value().as_u32(),
        "result_value": route.result_value().as_u32(),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
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
        "tier": route.tier(),
        "emit_kind": route.emit_kind(),
        "proof": route.proof(),
        "route_kind": route.route_kind(),
        "arity": route.arity(),
        "target_exists": route.target_exists(),
        "target_arity": route.target_arity(),
        "target_return_type": route.target_return_type(),
        "target_shape": route.target_shape(),
        "target_shape_reason": route.target_shape_reason(),
        "target_shape_blocker_symbol": route.target_shape_blocker_symbol(),
        "target_shape_blocker_reason": route.target_shape_blocker_reason(),
        "arity_matches": route.arity_matches(),
        "result_value": route.result_value().map(|value| value.as_u32()),
        "return_shape": route.return_shape(),
        "value_demand": route.value_demand(),
        "reason": route.reason(),
        "effects": route.effect_tags(),
    })
}

pub(super) fn build_array_getset_micro_seed_route_json(
    route: &crate::mir::array_getset_micro_seed_plan::ArrayGetSetMicroSeedRoute,
) -> serde_json::Value {
    json!({
        "size": route.size(),
        "ops": route.ops(),
        "init_push_count": route.init_push_count(),
        "loop_get_count": route.loop_get_count(),
        "loop_set_count": route.loop_set_count(),
        "final_get_count": route.final_get_count(),
        "selected_rmw_block": route.selected_rmw_block().as_u32(),
        "selected_rmw_instruction_index": route.selected_rmw_instruction_index(),
        "selected_rmw_set_instruction_index": route.selected_rmw_set_instruction_index(),
        "loop_index_phi_value": route.loop_index_phi_value().as_u32(),
        "accumulator_phi_value": route.accumulator_phi_value().as_u32(),
        "accumulator_next_value": route.accumulator_next_value().as_u32(),
        "return_value": route.return_value().as_u32(),
        "proof": route.proof(),
        "rmw_proof": route.rmw_proof().to_string(),
        "consumer_capability": "direct_stack_array_getset_micro",
        "publication_boundary": "none",
    })
}

pub(super) fn build_userbox_loop_micro_seed_route_json(
    route: &crate::mir::userbox_loop_micro_seed_plan::UserBoxLoopMicroSeedRoute,
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert("kind".to_string(), json!(route.kind().to_string()));
    obj.insert("box".to_string(), json!(route.box_name()));
    obj.insert("block_count".to_string(), json!(route.block_count()));
    obj.insert(
        "newbox_block".to_string(),
        json!(route.newbox_block().as_u32()),
    );
    obj.insert(
        "newbox_instruction_index".to_string(),
        json!(route.newbox_instruction_index()),
    );
    obj.insert("box_value".to_string(), json!(route.box_value().as_u32()));
    obj.insert("ops".to_string(), json!(route.ops()));
    obj.insert("flip_at".to_string(), json!(route.flip_at()));
    obj.insert(
        "field_get_count".to_string(),
        json!(route.field_get_count()),
    );
    obj.insert(
        "field_set_count".to_string(),
        json!(route.field_set_count()),
    );
    obj.insert(
        "compare_lt_count".to_string(),
        json!(route.compare_lt_count()),
    );
    obj.insert(
        "compare_eq_count".to_string(),
        json!(route.compare_eq_count()),
    );
    obj.insert("binop_count".to_string(), json!(route.binop_count()));
    obj.insert("proof".to_string(), json!(route.proof()));
    obj.insert(
        "consumer_capability".to_string(),
        json!("direct_userbox_loop_micro"),
    );
    obj.insert("publication_boundary".to_string(), json!("none"));
    serde_json::Value::Object(obj)
}

pub(super) fn build_userbox_known_receiver_method_seed_route_json(
    route: &crate::mir::userbox_known_receiver_method_seed_plan::UserBoxKnownReceiverMethodSeedRoute,
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert("kind".to_string(), json!(route.kind().to_string()));
    obj.insert("box".to_string(), json!(route.box_name()));
    obj.insert("method".to_string(), json!(route.method()));
    obj.insert(
        "method_function".to_string(),
        json!(route.method_function()),
    );
    obj.insert("block_count".to_string(), json!(route.block_count()));
    obj.insert(
        "method_block_count".to_string(),
        json!(route.method_block_count()),
    );
    obj.insert("block".to_string(), json!(route.block().as_u32()));
    obj.insert(
        "method_block".to_string(),
        json!(route.method_block().as_u32()),
    );
    obj.insert(
        "newbox_instruction_index".to_string(),
        json!(route.newbox_instruction_index()),
    );
    obj.insert(
        "copy_instruction_index".to_string(),
        json!(route.copy_instruction_index()),
    );
    obj.insert(
        "call_instruction_index".to_string(),
        json!(route.call_instruction_index()),
    );
    obj.insert("box_value".to_string(), json!(route.box_value().as_u32()));
    obj.insert(
        "copy_value".to_string(),
        json!(route.copy_value().map(|value| value.as_u32())),
    );
    obj.insert(
        "result_value".to_string(),
        json!(route.result_value().as_u32()),
    );
    obj.insert("proof".to_string(), json!(route.proof()));
    match route.payload() {
        UserBoxKnownReceiverMethodSeedPayload::CounterStepI64 {
            base_i64,
            delta_i64,
        } => {
            obj.insert("base_i64".to_string(), json!(*base_i64));
            obj.insert("delta_i64".to_string(), json!(*delta_i64));
            obj.insert("x_i64".to_string(), serde_json::Value::Null);
            obj.insert("y_i64".to_string(), serde_json::Value::Null);
            obj.insert("ops".to_string(), serde_json::Value::Null);
            obj.insert("step_i64".to_string(), serde_json::Value::Null);
            obj.insert("sum_i64".to_string(), serde_json::Value::Null);
            obj.insert("leaf_method_function".to_string(), serde_json::Value::Null);
        }
        UserBoxKnownReceiverMethodSeedPayload::PointSumI64 { x_i64, y_i64 } => {
            obj.insert("base_i64".to_string(), serde_json::Value::Null);
            obj.insert("delta_i64".to_string(), serde_json::Value::Null);
            obj.insert("x_i64".to_string(), json!(*x_i64));
            obj.insert("y_i64".to_string(), json!(*y_i64));
            obj.insert("ops".to_string(), serde_json::Value::Null);
            obj.insert("step_i64".to_string(), serde_json::Value::Null);
            obj.insert("sum_i64".to_string(), serde_json::Value::Null);
            obj.insert("leaf_method_function".to_string(), serde_json::Value::Null);
        }
        UserBoxKnownReceiverMethodSeedPayload::CounterStepLoopMicro {
            base_i64,
            delta_i64,
            ops,
            step_i64,
            known_receiver_count,
            field_set_count,
        } => {
            obj.insert("base_i64".to_string(), json!(*base_i64));
            obj.insert("delta_i64".to_string(), json!(*delta_i64));
            obj.insert("x_i64".to_string(), serde_json::Value::Null);
            obj.insert("y_i64".to_string(), serde_json::Value::Null);
            obj.insert("ops".to_string(), json!(*ops));
            obj.insert("step_i64".to_string(), json!(*step_i64));
            obj.insert("sum_i64".to_string(), serde_json::Value::Null);
            obj.insert(
                "known_receiver_count".to_string(),
                json!(*known_receiver_count),
            );
            obj.insert("field_set_count".to_string(), json!(*field_set_count));
            obj.insert("leaf_method_function".to_string(), serde_json::Value::Null);
        }
        UserBoxKnownReceiverMethodSeedPayload::CounterStepChainI64 {
            base_i64,
            delta_i64,
            leaf_method_function,
            leaf_method_block_count,
            leaf_method_block,
            ops,
            known_receiver_count,
            field_set_count,
        } => {
            obj.insert("base_i64".to_string(), json!(*base_i64));
            obj.insert("delta_i64".to_string(), json!(*delta_i64));
            obj.insert("x_i64".to_string(), serde_json::Value::Null);
            obj.insert("y_i64".to_string(), serde_json::Value::Null);
            obj.insert("ops".to_string(), json!(*ops));
            obj.insert("step_i64".to_string(), serde_json::Value::Null);
            obj.insert("sum_i64".to_string(), serde_json::Value::Null);
            obj.insert(
                "leaf_method_function".to_string(),
                json!(leaf_method_function.as_str()),
            );
            obj.insert(
                "leaf_method_block_count".to_string(),
                json!(*leaf_method_block_count),
            );
            obj.insert(
                "leaf_method_block".to_string(),
                json!(leaf_method_block.as_u32()),
            );
            obj.insert(
                "known_receiver_count".to_string(),
                json!(*known_receiver_count),
            );
            obj.insert("field_set_count".to_string(), json!(*field_set_count));
        }
        UserBoxKnownReceiverMethodSeedPayload::PointSumLoopMicro {
            x_i64,
            y_i64,
            ops,
            sum_i64,
            known_receiver_count,
            field_set_count,
            compare_lt_count,
            branch_count,
            jump_count,
            ret_count,
            add_count,
        } => {
            obj.insert("base_i64".to_string(), serde_json::Value::Null);
            obj.insert("delta_i64".to_string(), serde_json::Value::Null);
            obj.insert("x_i64".to_string(), json!(*x_i64));
            obj.insert("y_i64".to_string(), json!(*y_i64));
            obj.insert("ops".to_string(), json!(*ops));
            obj.insert("step_i64".to_string(), serde_json::Value::Null);
            obj.insert("sum_i64".to_string(), json!(*sum_i64));
            obj.insert(
                "known_receiver_count".to_string(),
                json!(*known_receiver_count),
            );
            obj.insert("field_set_count".to_string(), json!(*field_set_count));
            obj.insert("compare_lt_count".to_string(), json!(*compare_lt_count));
            obj.insert("branch_count".to_string(), json!(*branch_count));
            obj.insert("jump_count".to_string(), json!(*jump_count));
            obj.insert("ret_count".to_string(), json!(*ret_count));
            obj.insert("add_count".to_string(), json!(*add_count));
            obj.insert("leaf_method_function".to_string(), serde_json::Value::Null);
        }
    }
    let consumer_capability = match route.kind() {
        UserBoxKnownReceiverMethodSeedKind::CounterStepMicro
        | UserBoxKnownReceiverMethodSeedKind::PointSumMicro => {
            "direct_userbox_known_receiver_method_micro"
        }
        _ => "direct_userbox_known_receiver_method_local",
    };
    obj.insert(
        "consumer_capability".to_string(),
        json!(consumer_capability),
    );
    obj.insert("publication_boundary".to_string(), json!("none"));
    serde_json::Value::Object(obj)
}

pub(super) fn build_array_text_state_residence_route_json(
    route: &crate::mir::array_text_state_residence_plan::ArrayTextStateResidenceRoute,
) -> serde_json::Value {
    let contract = route.contract();
    let mut obj = json!({
        "observer_kind": contract.observer_kind(),
        "residence": contract.residence(),
        "result_repr": contract.result_repr(),
        "consumer_capability": contract.consumer_capability(),
        "publication_boundary": contract.publication_boundary(),
    });
    if let Some(payload) = route.temporary_indexof_seed_payload() {
        obj["temporary_indexof_seed_payload"] =
            build_array_text_state_residence_indexof_seed_payload_json(payload);
    }
    obj
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

pub(super) fn build_array_text_state_residence_indexof_seed_payload_json(
    payload: &crate::mir::array_text_state_residence_plan::ArrayTextStateResidenceIndexOfSeedPayload,
) -> serde_json::Value {
    json!({
        "variant": payload.variant(),
        "rows": payload.rows(),
        "ops": payload.ops(),
        "flip_period": payload.flip_period(),
        "line_seed": payload.line_seed(),
        "line_seed_len": payload.line_seed_len(),
        "none_seed": payload.none_seed(),
        "none_seed_len": payload.none_seed_len(),
        "needle": payload.needle(),
        "needle_len": payload.needle_len(),
        "proof": payload.proof(),
        "result_use": payload.result_use(),
        "backend_action": payload.backend_action(),
        "candidate_outcomes": [
            {
                "literal": payload.line_seed(),
                "outcome": payload.line_seed_outcome(),
            },
            {
                "literal": payload.none_seed(),
                "outcome": payload.none_seed_outcome(),
            },
        ],
    })
}

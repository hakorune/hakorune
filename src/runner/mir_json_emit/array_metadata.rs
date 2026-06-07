use super::route_json::build_array_text_state_residence_route_json;
use crate::mir::function::FunctionMetadata;
use serde_json::json;

pub(super) fn insert_array_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "direct_array_access_plans".to_string(),
        json!(metadata
            .direct_array_access_plans
            .iter()
            .map(|plan| {
                json!({
                    "route_id": "direct_array.access",
                    "block": plan.block().as_u32(),
                    "instruction_index": plan.instruction_index(),
                    "op": plan.op().as_str(),
                    "receiver_value": plan.receiver_value().as_u32(),
                    "index_value": plan.index_value().as_u32(),
                    "value_value": plan.value_value().map(|value| value.as_u32()),
                    "result_value": plan.result_value().map(|value| value.as_u32()),
                    "array_kind": plan.array_kind(),
                    "element_type": plan.element_type(),
                    "route": plan.route(),
                    "bounds_policy": plan.bounds_policy().as_str(),
                    "proof_kind": plan.proof_kind().as_str(),
                    "proof_ids": plan.proof_ids(),
                    "fallback_policy": plan.fallback_policy().as_str(),
                    "cfg_shape": plan.cfg_shape().as_str(),
                    "store_semantics": plan.store_semantics().as_str(),
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "array_rmw_window_routes".to_string(),
        json!(metadata
            .array_rmw_window_routes
            .iter()
            .map(|route| {
                json!({
                    "route_id": "array.rmw_add1.window",
                    "block": route.block().as_u32(),
                    "instruction_index": route.instruction_index(),
                    "array_value": route.array_value().as_u32(),
                    "index_value": route.index_value().as_u32(),
                    "input_value": route.input_value().as_u32(),
                    "const_value": route.const_value().as_u32(),
                    "result_value": route.result_value().as_u32(),
                    "set_instruction_index": route.set_instruction_index(),
                    "skip_instruction_indices": route.skip_instruction_indices(),
                    "proof": route.proof().to_string(),
                    "emit_symbol": "nyash.array.rmw_add1_hi",
                    "effects": ["load.cell", "store.cell"],
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "array_string_len_window_routes".to_string(),
        json!(metadata
            .array_string_len_window_routes
            .iter()
            .map(|route| {
                json!({
                    "route_id": "array.string_len.window",
                    "block": route.block().as_u32(),
                    "instruction_index": route.instruction_index(),
                    "array_value": route.array_value().as_u32(),
                    "index_value": route.index_value().as_u32(),
                    "source_value": route.source_value().as_u32(),
                    "len_instruction_index": route.len_instruction_index(),
                    "len_value": route.len_value().as_u32(),
                    "skip_instruction_indices": route.skip_instruction_indices(),
                    "mode": route.mode(),
                    "proof": route.proof(),
                    "emit_symbol": "nyash.array.string_len_hi",
                    "keep_get_live": route.keep_get_live(),
                    "source_only_insert_mid": route.source_only_insert_mid(),
                    "effects": route.effect_tags(),
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "array_text_loopcarry_len_store_routes".to_string(),
        json!(metadata
            .array_text_loopcarry_len_store_routes
            .iter()
            .map(|route| {
                json!({
                    "block": route.block().as_u32(),
                    "instruction_index": route.instruction_index(),
                    "array_value": route.array_value().as_u32(),
                    "index_value": route.index_value().as_u32(),
                    "source_value": route.source_value().as_u32(),
                    "substring_value": route.substring_value().as_u32(),
                    "result_len_value": route.result_len_value().as_u32(),
                    "middle_value": route.middle_value().as_u32(),
                    "middle_length": route.middle_length(),
                    "skip_instruction_indices": route.skip_instruction_indices(),
                    "proof": route.proof(),
                    "consumer_capability": "slot_text_len_store",
                    "publication_boundary": "none",
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "array_text_edit_routes".to_string(),
        json!(metadata
            .array_text_edit_routes
            .iter()
            .map(|route| {
                json!({
                    "block": route.block().as_u32(),
                    "get_instruction_index": route.get_instruction_index(),
                    "set_instruction_index": route.set_instruction_index(),
                    "array_value": route.array_value().as_u32(),
                    "index_value": route.index_value().as_u32(),
                    "source_value": route.source_value().as_u32(),
                    "length_value": route.length_value().as_u32(),
                    "split_value": route.split_value().as_u32(),
                    "result_value": route.result_value().as_u32(),
                    "middle_value": route.middle_value().as_u32(),
                    "middle_text": route.middle_text(),
                    "middle_byte_len": route.middle_byte_len(),
                    "skip_instruction_indices": route.skip_instruction_indices(),
                    "edit_kind": route.edit_kind(),
                    "split_policy": route.split_policy(),
                    "proof": route.proof(),
                    "carrier": "array_lane_text_cell",
                    "effects": ["load.ref", "store.cell"],
                    "consumer_capabilities": ["sink_store"],
                    "materialization_policy": "text_resident_or_stringlike_slot",
                    "publication_boundary": "none",
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "array_text_residence_sessions".to_string(),
        build_array_text_residence_sessions_json(metadata),
    );
    obj.insert(
        "array_text_observer_routes".to_string(),
        build_array_text_observer_routes_json(metadata),
    );
    obj.insert(
        "array_text_combined_regions".to_string(),
        build_array_text_combined_regions_json(metadata),
    );
    obj.insert(
        "array_string_store_micro_seed_route".to_string(),
        json!(metadata
            .array_string_store_micro_seed_route
            .as_ref()
            .map(|route| {
                json!({
                    "seed": route.seed(),
                    "seed_len": route.seed_len(),
                    "size": route.size(),
                    "ops": route.ops(),
                    "suffix": route.suffix(),
                    "store_len": route.store_len(),
                    "next_text_window_start": route.next_text_window_start(),
                    "next_text_window_len": route.next_text_window_len(),
                    "proof": route.proof(),
                    "consumer_capability": "direct_stack_array_string_store",
                    "publication_boundary": "none",
                })
            })),
    );
    obj.insert(
        "array_rmw_add1_leaf_seed_route".to_string(),
        json!(metadata.array_rmw_add1_leaf_seed_route.as_ref().map(|route| {
            json!({
                "size": route.size(),
                "ops": route.ops(),
                "init_push_count": route.init_push_count(),
                "final_get_count": route.final_get_count(),
                "selected_rmw_block": route.selected_rmw_block().as_u32(),
                "selected_rmw_instruction_index": route.selected_rmw_instruction_index(),
                "selected_rmw_set_instruction_index": route.selected_rmw_set_instruction_index(),
                "proof": route.proof(),
                "rmw_proof": route.rmw_proof().to_string(),
                "consumer_capability": "direct_stack_array_rmw_add1_leaf",
                "publication_boundary": "none",
            })
        })),
    );
    obj.insert(
        "array_text_state_residence_route".to_string(),
        json!(metadata
            .array_text_state_residence_route
            .as_ref()
            .map(build_array_text_state_residence_route_json)),
    );
}

fn build_array_text_residence_sessions_json(metadata: &FunctionMetadata) -> serde_json::Value {
    json!(metadata
        .array_text_residence_sessions
        .iter()
        .map(|route| {
            let mut obj = json!({
                "begin_block": route.begin_block().as_u32(),
                "begin_to_header_block": route.begin_to_header_block().as_u32(),
                "begin_placement": route.begin_placement(),
                "header_block": route.header_block().as_u32(),
                "body_block": route.body_block().as_u32(),
                "exit_block": route.exit_block().as_u32(),
                "update_block": route.update_block().as_u32(),
                "update_instruction_index": route.update_instruction_index(),
                "update_placement": route.update_placement(),
                "end_block": route.end_block().as_u32(),
                "end_placement": route.end_placement(),
                "route_instruction_index": route.route_instruction_index(),
                "array_value": route.array_value().as_u32(),
                "index_value": route.index_value().as_u32(),
                "source_value": route.source_value().as_u32(),
                "result_len_value": route.result_len_value().as_u32(),
                "middle_value": route.middle_value().as_u32(),
                "middle_length": route.middle_length(),
                "skip_instruction_indices": route.skip_instruction_indices(),
                "scope": route.scope(),
                "proof": route.proof(),
                "consumer_capability": "slot_text_len_store_session",
                "publication_boundary": "none",
            });
            if let Some(contract) = route.executor_contract() {
                let mut contract_obj = json!({
                    "execution_mode": contract.execution_mode(),
                    "proof_region": contract.proof_region(),
                    "publication_boundary": contract.publication_boundary(),
                    "carrier": contract.carrier(),
                    "effects": contract.effects(),
                    "consumer_capabilities": contract.consumer_capabilities(),
                    "materialization_policy": contract.materialization_policy(),
                });
                if let Some(mapping) = contract.region_mapping() {
                    contract_obj["region_mapping"] = json!({
                        "array_root_value": mapping.array_root_value().as_u32(),
                        "loop_index_phi_value": mapping.loop_index_phi_value().as_u32(),
                        "loop_index_initial_value": mapping.loop_index_initial_value().as_u32(),
                        "loop_index_initial_const": mapping.loop_index_initial_const(),
                        "loop_index_next_value": mapping.loop_index_next_value().as_u32(),
                        "loop_bound_value": mapping.loop_bound_value().as_u32(),
                        "loop_bound_const": mapping.loop_bound_const(),
                        "accumulator_phi_value": mapping.accumulator_phi_value().as_u32(),
                        "accumulator_initial_value": mapping.accumulator_initial_value().as_u32(),
                        "accumulator_initial_const": mapping.accumulator_initial_const(),
                        "accumulator_next_value": mapping.accumulator_next_value().as_u32(),
                        "exit_accumulator_value": mapping.exit_accumulator_value().as_u32(),
                        "row_index_value": mapping.row_index_value().as_u32(),
                        "row_modulus_value": mapping.row_modulus_value().as_u32(),
                        "row_modulus_const": mapping.row_modulus_const(),
                    });
                }
                obj["executor_contract"] = contract_obj;
            }
            obj
        })
        .collect::<Vec<_>>())
}

fn build_array_text_observer_routes_json(metadata: &FunctionMetadata) -> serde_json::Value {
    json!(metadata
        .array_text_observer_routes
        .iter()
        .map(|route| {
            let mut obj = json!({
                "block": route.block().as_u32(),
                "observer_instruction_index": route.observer_instruction_index(),
                "get_block": route.get_block().as_u32(),
                "get_instruction_index": route.get_instruction_index(),
                "array_value": route.array_value().as_u32(),
                "index_value": route.index_value().as_u32(),
                "source_value": route.source_value().as_u32(),
                "observer_kind": route.observer_kind(),
                "observer_arg0": route.observer_arg0().as_u32(),
                "observer_arg0_repr": route.observer_arg0_repr_kind(),
                "observer_arg0_keep_live": route.observer_arg0_keep_live(),
                "result_value": route.result_value().as_u32(),
                "consumer_shape": route.consumer_shape(),
                "proof_region": route.proof_region(),
                "publication_boundary": route.publication_boundary(),
                "result_repr": route.result_repr(),
                "keep_get_live": route.keep_get_live(),
            });
            if let Some(text) = route.observer_arg0_text() {
                obj["observer_arg0_text"] = json!(text);
            }
            if let Some(byte_len) = route.observer_arg0_byte_len() {
                obj["observer_arg0_byte_len"] = json!(byte_len);
            }
            if let Some(contract) = route.executor_contract() {
                let mut contract_obj = json!({
                    "execution_mode": contract.execution_mode(),
                    "proof_region": contract.proof_region(),
                    "publication_boundary": contract.publication_boundary(),
                    "carrier": contract.carrier(),
                    "effects": contract.effects(),
                    "consumer_capabilities": contract.consumer_capabilities(),
                    "materialization_policy": contract.materialization_policy(),
                });
                if let Some(mapping) = contract.region_mapping() {
                    contract_obj["region_mapping"] = json!({
                        "array_root_value": mapping.array_root_value().as_u32(),
                        "loop_index_phi_value": mapping.loop_index_phi_value().as_u32(),
                        "loop_index_initial_value": mapping.loop_index_initial_value().as_u32(),
                        "loop_index_initial_const": mapping.loop_index_initial_const(),
                        "loop_index_next_value": mapping.loop_index_next_value().as_u32(),
                        "loop_bound_value": mapping.loop_bound_value().as_u32(),
                        "loop_bound_const": mapping.loop_bound_const(),
                        "begin_block": mapping.begin_block().as_u32(),
                        "begin_to_header_block": mapping.begin_to_header_block().as_u32(),
                        "header_block": mapping.header_block().as_u32(),
                        "observer_block": mapping.observer_block().as_u32(),
                        "observer_instruction_index": mapping.observer_instruction_index(),
                        "predicate_value": mapping.predicate_value().as_u32(),
                        "then_store_block": mapping.then_store_block().as_u32(),
                        "store_instruction_index": mapping.store_instruction_index(),
                        "suffix_value": mapping.suffix_value().as_u32(),
                        "suffix_text": mapping.suffix_text(),
                        "suffix_byte_len": mapping.suffix_byte_len(),
                        "latch_block": mapping.latch_block().as_u32(),
                        "exit_block": mapping.exit_block().as_u32(),
                    });
                }
                obj["executor_contract"] = contract_obj;
            }
            obj
        })
        .collect::<Vec<_>>())
}

fn build_array_text_combined_regions_json(metadata: &FunctionMetadata) -> serde_json::Value {
    json!(metadata
        .array_text_combined_regions
        .iter()
        .map(|route| {
            let mut obj = json!({});
            obj["begin_block"] = json!(route.begin_block().as_u32());
            obj["header_block"] = json!(route.header_block().as_u32());
            obj["edit_block"] = json!(route.edit_block().as_u32());
            obj["observer_begin_block"] = json!(route.observer_begin_block().as_u32());
            obj["observer_header_block"] = json!(route.observer_header_block().as_u32());
            obj["observer_block"] = json!(route.observer_block().as_u32());
            obj["observer_store_block"] = json!(route.observer_store_block().as_u32());
            obj["observer_latch_block"] = json!(route.observer_latch_block().as_u32());
            obj["observer_exit_block"] = json!(route.observer_exit_block().as_u32());
            obj["latch_block"] = json!(route.latch_block().as_u32());
            obj["exit_block"] = json!(route.exit_block().as_u32());
            obj["array_value"] = json!(route.array_value().as_u32());
            obj["outer_index_phi_value"] = json!(route.outer_index_phi_value().as_u32());
            obj["outer_index_initial_value"] = json!(route.outer_index_initial_value().as_u32());
            obj["outer_index_initial_const"] = json!(route.outer_index_initial_const());
            obj["outer_index_next_value"] = json!(route.outer_index_next_value().as_u32());
            obj["loop_bound_value"] = json!(route.loop_bound_value().as_u32());
            obj["loop_bound_const"] = json!(route.loop_bound_const());
            obj["row_index_value"] = json!(route.row_index_value().as_u32());
            obj["row_modulus_value"] = json!(route.row_modulus_value().as_u32());
            obj["row_modulus_const"] = json!(route.row_modulus_const());
            obj["observer_period_value"] = json!(route.observer_period_value().as_u32());
            obj["observer_period_const"] = json!(route.observer_period_const());
            obj["accumulator_phi_value"] = json!(route.accumulator_phi_value().as_u32());
            obj["accumulator_initial_value"] = json!(route.accumulator_initial_value().as_u32());
            obj["accumulator_initial_const"] = json!(route.accumulator_initial_const());
            obj["accumulator_next_value"] = json!(route.accumulator_next_value().as_u32());
            obj["edit_middle_value"] = json!(route.edit_middle_value().as_u32());
            obj["edit_middle_text"] = json!(route.edit_middle_text());
            obj["edit_middle_byte_len"] = json!(route.edit_middle_byte_len());
            obj["observer_bound_value"] = json!(route.observer_bound_value().as_u32());
            obj["observer_bound_const"] = json!(route.observer_bound_const());
            obj["observer_needle_value"] = json!(route.observer_needle_value().as_u32());
            obj["observer_needle_text"] = json!(route.observer_needle_text());
            obj["observer_needle_byte_len"] = json!(route.observer_needle_byte_len());
            obj["observer_suffix_value"] = json!(route.observer_suffix_value().as_u32());
            obj["observer_suffix_text"] = json!(route.observer_suffix_text());
            obj["observer_suffix_byte_len"] = json!(route.observer_suffix_byte_len());
            obj["execution_mode"] = json!(route.execution_mode());
            obj["proof_region"] = json!(route.proof_region());
            obj["proof"] = json!(route.proof());
            if let Some(proof) = route.byte_boundary_proof() {
                obj["byte_boundary_proof"] = json!(proof);
                obj["text_encoding"] = json!("ascii_preserved");
                obj["split_boundary_policy"] = json!("byte_index_safe");
            }
            obj["publication_boundary"] = json!(route.publication_boundary());
            obj["carrier"] = json!(route.carrier());
            obj["effects"] = json!(route.effects());
            obj["consumer_capabilities"] = json!(route.consumer_capabilities());
            obj["materialization_policy"] = json!(route.materialization_policy());
            obj
        })
        .collect::<Vec<_>>())
}

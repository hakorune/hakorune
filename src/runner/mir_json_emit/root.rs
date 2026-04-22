use super::*;

pub(super) fn build_mir_json_root(
    module: &crate::mir::MirModule,
) -> Result<serde_json::Value, String> {
    use crate::mir::MirType;

    let mut funs = Vec::new();
    for (name, f) in ordered_harness_functions(module) {
        let mut blocks = Vec::new();
        let mut ids: Vec<_> = f.blocks.keys().copied().collect();
        ids.sort();
        for bid in ids {
            if let Some(bb) = f.blocks.get(&bid) {
                let mut insts = Vec::new();
                // Phase 131-13: Emit all instructions in MIR order (SSOT principle)
                // No reordering except PHI consolidation at block start (LLVM constraint)

                // Step 1: Emit all PHI instructions first (LLVM requirement)
                insts.extend(emitters::emit_phi_instructions(f, bb));

                // Step 2: Emit all non-PHI instructions in MIR order (no reordering!)
                emitters::emit_non_phi_instructions(f, bb, &mut insts)?;

                // Phase 131-13: Terminator emitted inline (no delayed copies)
                if let Some(term) = emitters::emit_terminator(&bb.terminator)? {
                    insts.push(term);
                }
                blocks.push(json!({"id": bid.as_u32(), "instructions": insts}));
            }
        }
        // Export parameter value-ids so a VM can bind arguments
        let params: Vec<_> = f.params.iter().map(|v| v.as_u32()).collect();

        // Phase 131-11-F: Build metadata JSON from MIR metadata (SSOT)
        let metadata_json = json!({
            "value_types": f.metadata.value_types.iter().map(|(k, v)| {
                let type_str = match v {
                    MirType::Integer => json!("i64"),
                    MirType::Float => json!("f64"),  // Phase 275 P0: Float type annotation
                    MirType::String => json!({"kind": "string"}),
                    MirType::Box(bt) => json!({"kind": "handle", "box_type": bt}),
                    MirType::Bool => json!("i1"),
                    MirType::Void => json!("void"),
                    MirType::Unknown => json!(null),
                    _ => json!(null),
                };
                (k.as_u32().to_string(), type_str)
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "value_consumer_facts": f.metadata.value_consumer_facts.iter().map(|(k, facts)| {
                (k.as_u32().to_string(), json!({
                    "direct_set_consumer": facts.direct_set_consumer,
                }))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "storage_classes": f.metadata.value_storage_classes.iter().map(|(k, v)| {
                (k.as_u32().to_string(), json!(v.to_string()))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_facts": f.metadata.string_corridor_facts.iter().map(|(k, fact)| {
                (k.as_u32().to_string(), json!({
                    "op": fact.op.to_string(),
                    "role": fact.role.to_string(),
                    "carrier": fact.carrier.to_string(),
                    "borrow_contract": fact.borrow_contract.map(|contract| contract.to_string()),
                    "outcome": fact.outcome.map(|outcome| outcome.to_string()),
                    "objectize": fact.objectize.to_string(),
                    "publish": fact.publish.to_string(),
                    "materialize": fact.materialize.to_string(),
                }))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_relations": f.metadata.string_corridor_relations.iter().map(|(k, relations)| {
                (k.as_u32().to_string(), json!(relations.iter().map(|relation| {
                    json!({
                        "kind": relation.kind.to_string(),
                        "base_value": relation.base_value.as_u32(),
                        "witness_value": relation.witness_value.map(|value| value.as_u32()),
                        "window_contract": relation.window_contract.to_string(),
                        "reason": relation.reason,
                    })
                }).collect::<Vec<_>>()))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_corridor_candidates": f.metadata.string_corridor_candidates.iter().map(|(k, candidates)| {
                (k.as_u32().to_string(), json!(candidates.iter().map(|candidate| {
                    json!({
                        "kind": candidate.kind.to_string(),
                        "state": candidate.state.to_string(),
                        "reason": candidate.reason,
                        "plan": candidate.plan.map(|plan| json!({
                            "corridor_root": plan.corridor_root.as_u32(),
                            "source_root": plan.source_root.map(|value| value.as_u32()),
                            "borrow_contract": plan.borrow_contract.map(|contract| contract.to_string()),
                            "publish_reason": plan.publish_reason.map(|reason| reason.to_string()),
                            "publish_repr_policy": plan.publish_repr_policy.map(|repr| repr.to_string()),
                            "stable_view_provenance": plan.stable_view_provenance.map(|provenance| provenance.to_string()),
                            "start": plan.start.map(|value| value.as_u32()),
                            "end": plan.end.map(|value| value.as_u32()),
                            "known_length": plan.known_length,
                            "publication_contract": plan.publication_contract.map(|contract| contract.to_string()),
                            "proof": match plan.proof {
                                crate::mir::string_corridor_placement::StringCorridorCandidateProof::BorrowedSlice {
                                    source,
                                    start,
                                    end,
                                } => json!({
                                    "kind": "borrowed_slice",
                                    "source": source.as_u32(),
                                    "start": start.as_u32(),
                                    "end": end.as_u32(),
                                }),
                                crate::mir::string_corridor_placement::StringCorridorCandidateProof::ConcatTriplet {
                                    left_value,
                                    left_source,
                                    left_start,
                                    left_end,
                                    middle,
                                    right_value,
                                    right_source,
                                    right_start,
                                    right_end,
                                    shared_source,
                                } => json!({
                                    "kind": "concat_triplet",
                                    "left_value": left_value.map(|value| value.as_u32()),
                                    "left_source": left_source.as_u32(),
                                    "left_start": left_start.as_u32(),
                                    "left_end": left_end.as_u32(),
                                    "middle": middle.as_u32(),
                                    "right_value": right_value.map(|value| value.as_u32()),
                                    "right_source": right_source.as_u32(),
                                    "right_start": right_start.as_u32(),
                                    "right_end": right_end.as_u32(),
                                    "shared_source": shared_source,
                                }),
                            },
                        })),
                        "publication_boundary": candidate.publication_boundary.map(|boundary| boundary.to_string()),
                    })
                }).collect::<Vec<_>>()))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_kernel_plans": f.metadata.string_kernel_plans.iter().map(|(k, plan)| {
                (k.as_u32().to_string(), build_string_kernel_plan_json(plan))
            }).collect::<serde_json::Map<String, serde_json::Value>>(),
            "string_direct_set_window_routes": f.metadata.string_direct_set_window_routes.iter().map(|route| {
                json!({
                    "route_id": "string.direct_set_source_window",
                    "block": route.block.as_u32(),
                    "instruction_index": route.instruction_index,
                    "second_instruction_index": route.second_instruction_index,
                    "concat_instruction_index": route.concat_instruction_index,
                    "source_value": route.source_value.as_u32(),
                    "prefix_value": route.prefix_value.as_u32(),
                    "suffix_value": route.suffix_value.as_u32(),
                    "middle_value": route.middle_value.as_u32(),
                    "split_value": route.split_value.as_u32(),
                    "result_value": route.result_value.as_u32(),
                    "subrange_start": route.subrange_start.as_u32(),
                    "subrange_end": route.subrange_end.as_u32(),
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "proof": route.proof.to_string(),
                    "consumer": "direct_set",
                    "effects": ["observe.substring", "defer.piecewise", "direct.set.consumer"],
                })
            }).collect::<Vec<_>>(),
            "generic_method_routes": f.metadata.generic_method_routes.iter().map(|route| {
                json!({
                    "route_id": "generic_method.has",
                    "block": route.block.as_u32(),
                    "instruction_index": route.instruction_index,
                    "box_name": route.box_name.as_str(),
                    "method": route.method.as_str(),
                    "arity": 1,
                    "receiver_value": route.receiver_value.as_u32(),
                    "key_value": route.key_value.as_u32(),
                    "result_value": route.result_value.map(|value| value.as_u32()),
                    "emit_kind": "has",
                    "route_kind": route.route_kind.to_string(),
                    "helper_symbol": route.route_kind.helper_symbol(),
                    "proof": route.proof.to_string(),
                    "value_demand": "read_ref",
                    "effects": ["probe.key"],
                })
            }).collect::<Vec<_>>(),
            "array_rmw_window_routes": f.metadata.array_rmw_window_routes.iter().map(|route| {
                json!({
                    "route_id": "array.rmw_add1.window",
                    "block": route.block.as_u32(),
                    "instruction_index": route.instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "input_value": route.input_value.as_u32(),
                    "const_value": route.const_value.as_u32(),
                    "result_value": route.result_value.as_u32(),
                    "set_instruction_index": route.set_instruction_index,
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "proof": route.proof.to_string(),
                    "emit_symbol": "nyash.array.rmw_add1_hi",
                    "effects": ["load.cell", "store.cell"],
                })
            }).collect::<Vec<_>>(),
            "array_string_len_window_routes": f.metadata.array_string_len_window_routes.iter().map(|route| {
                let (keep_get_live, source_only_insert_mid, effects) = match route.mode {
                    crate::mir::ArrayStringLenWindowMode::LenOnly => (
                        false,
                        false,
                        vec!["load.cell", "observe.len"],
                    ),
                    crate::mir::ArrayStringLenWindowMode::KeepGetLive => (
                        true,
                        false,
                        vec!["load.cell", "observe.len", "keep.source.live"],
                    ),
                    crate::mir::ArrayStringLenWindowMode::SourceOnlyInsertMid => (
                        false,
                        true,
                        vec!["load.cell", "observe.len", "publish.source.ref"],
                    ),
                };
                json!({
                    "route_id": "array.string_len.window",
                    "block": route.block.as_u32(),
                    "instruction_index": route.instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "source_value": route.source_value.as_u32(),
                    "len_instruction_index": route.len_instruction_index,
                    "len_value": route.len_value.as_u32(),
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "mode": route.mode.to_string(),
                    "proof": route.proof.to_string(),
                    "emit_symbol": "nyash.array.string_len_hi",
                    "keep_get_live": keep_get_live,
                    "source_only_insert_mid": source_only_insert_mid,
                    "effects": effects,
                })
            }).collect::<Vec<_>>(),
            "array_text_loopcarry_len_store_routes": f.metadata.array_text_loopcarry_len_store_routes.iter().map(|route| {
                json!({
                    "block": route.block.as_u32(),
                    "instruction_index": route.instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "source_value": route.source_value.as_u32(),
                    "substring_value": route.substring_value.as_u32(),
                    "result_len_value": route.result_len_value.as_u32(),
                    "middle_value": route.middle_value.as_u32(),
                    "middle_length": route.middle_length,
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "proof": route.proof.to_string(),
                    "consumer_capability": "slot_text_len_store",
                    "publication_boundary": "none",
                })
            }).collect::<Vec<_>>(),
            "array_text_edit_routes": f.metadata.array_text_edit_routes.iter().map(|route| {
                json!({
                    "block": route.block.as_u32(),
                    "get_instruction_index": route.get_instruction_index,
                    "set_instruction_index": route.set_instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "source_value": route.source_value.as_u32(),
                    "length_value": route.length_value.as_u32(),
                    "split_value": route.split_value.as_u32(),
                    "result_value": route.result_value.as_u32(),
                    "middle_value": route.middle_value.as_u32(),
                    "middle_text": route.middle_text,
                    "middle_byte_len": route.middle_byte_len,
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "edit_kind": route.edit_kind.to_string(),
                    "split_policy": route.split_policy.to_string(),
                    "proof": route.proof.to_string(),
                    "carrier": "array_lane_text_cell",
                    "effects": ["load.ref", "store.cell"],
                    "consumer_capabilities": ["sink_store"],
                    "materialization_policy": "text_resident_or_stringlike_slot",
                    "publication_boundary": "none",
                })
            }).collect::<Vec<_>>(),
            "array_text_residence_sessions": f.metadata.array_text_residence_sessions.iter().map(|route| {
                let mut obj = json!({
                    "begin_block": route.begin_block.as_u32(),
                    "begin_to_header_block": route.begin_to_header_block.as_u32(),
                    "begin_placement": route.begin_placement.to_string(),
                    "header_block": route.header_block.as_u32(),
                    "body_block": route.body_block.as_u32(),
                    "exit_block": route.exit_block.as_u32(),
                    "update_block": route.update_block.as_u32(),
                    "update_instruction_index": route.update_instruction_index,
                    "update_placement": route.update_placement.to_string(),
                    "end_block": route.end_block.as_u32(),
                    "end_placement": route.end_placement.to_string(),
                    "route_instruction_index": route.route_instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "source_value": route.source_value.as_u32(),
                    "result_len_value": route.result_len_value.as_u32(),
                    "middle_value": route.middle_value.as_u32(),
                    "middle_length": route.middle_length,
                    "skip_instruction_indices": route.skip_instruction_indices,
                    "scope": route.scope.to_string(),
                    "proof": route.proof.to_string(),
                    "consumer_capability": "slot_text_len_store_session",
                    "publication_boundary": "none",
                });
                if let Some(contract) = route.executor_contract.as_ref() {
                    let mut contract_obj = json!({
                        "execution_mode": contract.execution_mode.to_string(),
                        "proof_region": contract.proof_region.to_string(),
                        "publication_boundary": contract.publication_boundary,
                        "carrier": contract.carrier.to_string(),
                        "effects": contract.effects.iter().map(|effect| effect.to_string()).collect::<Vec<_>>(),
                        "consumer_capabilities": contract.consumer_capabilities.iter().map(|capability| capability.to_string()).collect::<Vec<_>>(),
                        "materialization_policy": contract.materialization_policy.to_string(),
                    });
                    if let Some(mapping) = contract.region_mapping.as_ref() {
                        contract_obj["region_mapping"] = json!({
                            "array_root_value": mapping.array_root_value.as_u32(),
                            "loop_index_phi_value": mapping.loop_index_phi_value.as_u32(),
                            "loop_index_initial_value": mapping.loop_index_initial_value.as_u32(),
                            "loop_index_initial_const": mapping.loop_index_initial_const,
                            "loop_index_next_value": mapping.loop_index_next_value.as_u32(),
                            "loop_bound_value": mapping.loop_bound_value.as_u32(),
                            "loop_bound_const": mapping.loop_bound_const,
                            "accumulator_phi_value": mapping.accumulator_phi_value.as_u32(),
                            "accumulator_initial_value": mapping.accumulator_initial_value.as_u32(),
                            "accumulator_initial_const": mapping.accumulator_initial_const,
                            "accumulator_next_value": mapping.accumulator_next_value.as_u32(),
                            "exit_accumulator_value": mapping.exit_accumulator_value.as_u32(),
                            "row_index_value": mapping.row_index_value.as_u32(),
                            "row_modulus_value": mapping.row_modulus_value.as_u32(),
                            "row_modulus_const": mapping.row_modulus_const,
                        });
                    }
                    obj["executor_contract"] = contract_obj;
                }
                obj
            }).collect::<Vec<_>>(),
            "array_text_observer_routes": f.metadata.array_text_observer_routes.iter().map(|route| {
                let mut obj = json!({
                    "block": route.block.as_u32(),
                    "observer_instruction_index": route.observer_instruction_index,
                    "get_block": route.get_block.as_u32(),
                    "get_instruction_index": route.get_instruction_index,
                    "array_value": route.array_value.as_u32(),
                    "index_value": route.index_value.as_u32(),
                    "source_value": route.source_value.as_u32(),
                    "observer_kind": route.observer_kind.to_string(),
                    "observer_arg0": route.observer_arg0.as_u32(),
                    "observer_arg0_repr": route.observer_arg0_repr.kind(),
                    "observer_arg0_keep_live": route.observer_arg0_keep_live,
                    "result_value": route.result_value.as_u32(),
                    "consumer_shape": route.consumer_shape.to_string(),
                    "proof_region": route.proof_region.to_string(),
                    "publication_boundary": route.publication_boundary.to_string(),
                    "result_repr": route.result_repr.to_string(),
                    "keep_get_live": route.keep_get_live,
                });
                if let crate::mir::ArrayTextObserverArgRepr::ConstUtf8 { text, byte_len } =
                    &route.observer_arg0_repr
                {
                    obj["observer_arg0_text"] = json!(text);
                    obj["observer_arg0_byte_len"] = json!(byte_len);
                }
                if let Some(contract) = route.executor_contract.as_ref() {
                    let mut contract_obj = json!({
                        "execution_mode": contract.execution_mode.to_string(),
                        "proof_region": contract.proof_region.to_string(),
                        "publication_boundary": contract.publication_boundary.to_string(),
                        "carrier": contract.carrier.to_string(),
                        "effects": contract.effects.iter().map(|effect| effect.to_string()).collect::<Vec<_>>(),
                        "consumer_capabilities": contract.consumer_capabilities.iter().map(|capability| capability.to_string()).collect::<Vec<_>>(),
                        "materialization_policy": contract.materialization_policy.to_string(),
                    });
                    if let Some(mapping) = contract.region_mapping.as_ref() {
                        contract_obj["region_mapping"] = json!({
                            "array_root_value": mapping.array_root_value.as_u32(),
                            "loop_index_phi_value": mapping.loop_index_phi_value.as_u32(),
                            "loop_index_initial_value": mapping.loop_index_initial_value.as_u32(),
                            "loop_index_initial_const": mapping.loop_index_initial_const,
                            "loop_index_next_value": mapping.loop_index_next_value.as_u32(),
                            "loop_bound_value": mapping.loop_bound_value.as_u32(),
                            "loop_bound_const": mapping.loop_bound_const,
                            "begin_block": mapping.begin_block.as_u32(),
                            "begin_to_header_block": mapping.begin_to_header_block.as_u32(),
                            "header_block": mapping.header_block.as_u32(),
                            "observer_block": mapping.observer_block.as_u32(),
                            "observer_instruction_index": mapping.observer_instruction_index,
                            "predicate_value": mapping.predicate_value.as_u32(),
                            "then_store_block": mapping.then_store_block.as_u32(),
                            "store_instruction_index": mapping.store_instruction_index,
                            "suffix_value": mapping.suffix_value.as_u32(),
                            "suffix_text": mapping.suffix_text,
                            "suffix_byte_len": mapping.suffix_byte_len,
                            "latch_block": mapping.latch_block.as_u32(),
                            "exit_block": mapping.exit_block.as_u32(),
                        });
                    }
                    obj["executor_contract"] = contract_obj;
                }
                obj
            }).collect::<Vec<_>>(),
            "array_text_combined_regions": f.metadata.array_text_combined_regions.iter().map(|route| {
                let mut obj = json!({});
                obj["begin_block"] = json!(route.begin_block.as_u32());
                obj["header_block"] = json!(route.header_block.as_u32());
                obj["edit_block"] = json!(route.edit_block.as_u32());
                obj["observer_begin_block"] = json!(route.observer_begin_block.as_u32());
                obj["observer_header_block"] = json!(route.observer_header_block.as_u32());
                obj["observer_block"] = json!(route.observer_block.as_u32());
                obj["observer_store_block"] = json!(route.observer_store_block.as_u32());
                obj["observer_latch_block"] = json!(route.observer_latch_block.as_u32());
                obj["observer_exit_block"] = json!(route.observer_exit_block.as_u32());
                obj["latch_block"] = json!(route.latch_block.as_u32());
                obj["exit_block"] = json!(route.exit_block.as_u32());
                obj["array_value"] = json!(route.array_value.as_u32());
                obj["outer_index_phi_value"] = json!(route.outer_index_phi_value.as_u32());
                obj["outer_index_initial_value"] = json!(route.outer_index_initial_value.as_u32());
                obj["outer_index_initial_const"] = json!(route.outer_index_initial_const);
                obj["outer_index_next_value"] = json!(route.outer_index_next_value.as_u32());
                obj["loop_bound_value"] = json!(route.loop_bound_value.as_u32());
                obj["loop_bound_const"] = json!(route.loop_bound_const);
                obj["row_index_value"] = json!(route.row_index_value.as_u32());
                obj["row_modulus_value"] = json!(route.row_modulus_value.as_u32());
                obj["row_modulus_const"] = json!(route.row_modulus_const);
                obj["observer_period_value"] = json!(route.observer_period_value.as_u32());
                obj["observer_period_const"] = json!(route.observer_period_const);
                obj["accumulator_phi_value"] = json!(route.accumulator_phi_value.as_u32());
                obj["accumulator_initial_value"] = json!(route.accumulator_initial_value.as_u32());
                obj["accumulator_initial_const"] = json!(route.accumulator_initial_const);
                obj["accumulator_next_value"] = json!(route.accumulator_next_value.as_u32());
                obj["edit_middle_value"] = json!(route.edit_middle_value.as_u32());
                obj["edit_middle_text"] = json!(route.edit_middle_text);
                obj["edit_middle_byte_len"] = json!(route.edit_middle_byte_len);
                obj["observer_bound_value"] = json!(route.observer_bound_value.as_u32());
                obj["observer_bound_const"] = json!(route.observer_bound_const);
                obj["observer_needle_value"] = json!(route.observer_needle_value.as_u32());
                obj["observer_needle_text"] = json!(route.observer_needle_text);
                obj["observer_needle_byte_len"] = json!(route.observer_needle_byte_len);
                obj["observer_suffix_value"] = json!(route.observer_suffix_value.as_u32());
                obj["observer_suffix_text"] = json!(route.observer_suffix_text);
                obj["observer_suffix_byte_len"] = json!(route.observer_suffix_byte_len);
                obj["execution_mode"] = json!(route.execution_mode.to_string());
                obj["proof_region"] = json!(route.proof_region.to_string());
                obj["proof"] = json!(route.proof.to_string());
                if let Some(proof) = route.byte_boundary_proof {
                    obj["byte_boundary_proof"] = json!(proof.to_string());
                    obj["text_encoding"] = json!("ascii_preserved");
                    obj["split_boundary_policy"] = json!("byte_index_safe");
                }
                obj["publication_boundary"] = json!("none");
                obj["carrier"] = json!("array_lane_text_cell");
                obj["effects"] = json!([
                    "store.cell(lenhalf_insert_mid_const)",
                    "observe.indexof",
                    "store.cell(const_suffix_append)",
                    "scalar_accumulator(+1)"
                ]);
                obj["consumer_capabilities"] = json!([
                    "sink_store",
                    "compare_only",
                    "length_only_result_carry"
                ]);
                obj["materialization_policy"] = json!("text_resident_or_stringlike_slot");
                obj
            }).collect::<Vec<_>>(),
            "array_string_store_micro_seed_route": f.metadata.array_string_store_micro_seed_route.as_ref().map(|route| {
                json!({
                    "seed": route.seed.as_str(),
                    "seed_len": route.seed_len,
                    "size": route.size,
                    "ops": route.ops,
                    "suffix": route.suffix.as_str(),
                    "store_len": route.store_len,
                    "next_text_window_start": route.next_text_window_start,
                    "next_text_window_len": route.next_text_window_len,
                    "proof": route.proof.to_string(),
                    "consumer_capability": "direct_stack_array_string_store",
                    "publication_boundary": "none",
                })
            }),
            "array_rmw_add1_leaf_seed_route": f.metadata.array_rmw_add1_leaf_seed_route.as_ref().map(|route| {
                json!({
                    "size": route.size,
                    "ops": route.ops,
                    "init_push_count": route.init_push_count,
                    "final_get_count": route.final_get_count,
                    "selected_rmw_block": route.selected_rmw_block.as_u32(),
                    "selected_rmw_instruction_index": route.selected_rmw_instruction_index,
                    "selected_rmw_set_instruction_index": route.selected_rmw_set_instruction_index,
                    "proof": route.proof.to_string(),
                    "rmw_proof": route.rmw_proof.to_string(),
                    "consumer_capability": "direct_stack_array_rmw_add1_leaf",
                    "publication_boundary": "none",
                })
            }),
            "concat_const_suffix_micro_seed_route": f.metadata.concat_const_suffix_micro_seed_route.as_ref().map(|route| {
                json!({
                    "seed": route.seed.as_str(),
                    "seed_len": route.seed_len,
                    "suffix": route.suffix.as_str(),
                    "suffix_len": route.suffix_len,
                    "ops": route.ops,
                    "result_len": route.result_len,
                    "proof": route.proof.to_string(),
                    "consumer_capability": "direct_concat_const_suffix_loop",
                    "publication_boundary": "none",
                })
            }),
            "substring_views_micro_seed_route": f.metadata.substring_views_micro_seed_route.as_ref().map(|route| {
                json!({
                    "source": route.source.as_str(),
                    "source_len": route.source_len,
                    "loop_bound": route.loop_bound,
                    "proof": route.proof.to_string(),
                    "consumer_capability": "direct_substring_views_exit_len",
                    "publication_boundary": "none",
                })
            }),
            "sum_variant_tag_seed_route": f.metadata.sum_variant_tag_seed_route.as_ref().map(|route| {
                json!({
                    "kind": route.kind.to_string(),
                    "enum": route.enum_name.as_str(),
                    "variant": route.variant.as_str(),
                    "subject": route.subject.as_str(),
                    "layout": route.layout.to_string(),
                    "variant_tag": route.variant_tag,
                    "make_block": route.make_block.as_u32(),
                    "make_instruction_index": route.make_instruction_index,
                    "tag_block": route.tag_block.as_u32(),
                    "tag_instruction_index": route.tag_instruction_index,
                    "sum_value": route.sum_value.as_u32(),
                    "tag_value": route.tag_value.as_u32(),
                    "tag_source_value": route.tag_source_value.as_u32(),
                    "copy_value": route.copy_value.map(|value| value.as_u32()),
                    "payload_value": route.payload_value.map(|value| value.as_u32()),
                    "proof": route.proof.to_string(),
                    "consumer_capability": "direct_sum_variant_tag_local",
                    "publication_boundary": "none",
                })
            }),
            "exact_seed_backend_route": f.metadata.exact_seed_backend_route.as_ref().map(|route| {
                json!({
                    "tag": route.tag.as_str(),
                    "source_route": route.source_route.as_str(),
                    "proof": route.proof.as_str(),
                    "selected_value": route.selected_value.map(|value| value.as_u32()),
                })
            }),
            "array_text_state_residence_route": f.metadata.array_text_state_residence_route.as_ref().map(|route| {
                build_array_text_state_residence_route_json(route)
            }),
            "thin_entry_candidates": f.metadata.thin_entry_candidates.iter().map(|candidate| {
                json!({
                    "block": candidate.block.as_u32(),
                    "instruction_index": candidate.instruction_index,
                    "value": candidate.value.map(|value| value.as_u32()),
                    "surface": candidate.surface.to_string(),
                    "subject": candidate.subject,
                    "preferred_entry": candidate.preferred_entry.to_string(),
                    "current_carrier": candidate.current_carrier.to_string(),
                    "value_class": candidate.value_class.to_string(),
                    "demand": candidate.demand.to_string(),
                    "reason": candidate.reason,
                })
            }).collect::<Vec<_>>(),
            "thin_entry_selections": f.metadata.thin_entry_selections.iter().map(|selection| {
                json!({
                    "block": selection.block.as_u32(),
                    "instruction_index": selection.instruction_index,
                    "value": selection.value.map(|value| value.as_u32()),
                    "surface": selection.surface.to_string(),
                    "subject": selection.subject,
                    "manifest_row": selection.manifest_row,
                    "selected_entry": selection.selected_entry.to_string(),
                    "state": selection.state.to_string(),
                    "current_carrier": selection.current_carrier.to_string(),
                    "value_class": selection.value_class.to_string(),
                    "demand": selection.demand.to_string(),
                    "reason": selection.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_facts": f.metadata.sum_placement_facts.iter().map(|fact| {
                json!({
                    "block": fact.block.as_u32(),
                    "instruction_index": fact.instruction_index,
                    "value": fact.value.map(|value| value.as_u32()),
                    "surface": fact.surface.to_string(),
                    "subject": fact.subject,
                    "source_sum": fact.source_sum.map(|value| value.as_u32()),
                    "value_class": fact.value_class.to_string(),
                    "state": fact.state.to_string(),
                    "tag_reads": fact.tag_reads,
                    "project_reads": fact.project_reads,
                    "barriers": fact.barriers.iter().map(|barrier| barrier.to_string()).collect::<Vec<_>>(),
                    "reason": fact.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_selections": f.metadata.sum_placement_selections.iter().map(|selection| {
                json!({
                    "block": selection.block.as_u32(),
                    "instruction_index": selection.instruction_index,
                    "value": selection.value.map(|value| value.as_u32()),
                    "surface": selection.surface.to_string(),
                    "subject": selection.subject,
                    "source_sum": selection.source_sum.map(|value| value.as_u32()),
                    "manifest_row": selection.manifest_row,
                    "selected_path": selection.selected_path.to_string(),
                    "reason": selection.reason,
                })
            }).collect::<Vec<_>>(),
            "sum_placement_layouts": f.metadata.sum_placement_layouts.iter().map(|layout| {
                json!({
                    "block": layout.block.as_u32(),
                    "instruction_index": layout.instruction_index,
                    "value": layout.value.map(|value| value.as_u32()),
                    "surface": layout.surface.to_string(),
                    "subject": layout.subject,
                    "source_sum": layout.source_sum.map(|value| value.as_u32()),
                    "layout": layout.layout.to_string(),
                    "reason": layout.reason,
                })
            }).collect::<Vec<_>>(),
            "agg_local_scalarization_routes": build_agg_local_scalarization_routes_json(
                &f.metadata.agg_local_scalarization_routes,
            ),
            "placement_effect_routes": build_placement_effect_routes_json(
                &f.metadata.placement_effect_routes,
            ),
        });
        let attrs_json = json!({
            "runes": f
                .metadata
                .runes
                .iter()
                .map(|rune| json!({"name": rune.name, "args": rune.args}))
                .collect::<Vec<_>>()
        });

        funs.push(json!({
            "name": name,
            "params": params,
            "blocks": blocks,
            "metadata": metadata_json,
            "attrs": attrs_json
        }));
    }

    // Phase 15.5: JSON v1 schema with environment variable control
    let use_v1_schema = std::env::var("NYASH_JSON_SCHEMA_V1").unwrap_or_default() == "1"
        || match std::env::var("NYASH_MIR_UNIFIED_CALL")
            .ok()
            .as_deref()
            .map(|s| s.to_ascii_lowercase())
        {
            Some(s) if s == "0" || s == "false" || s == "off" => false,
            _ => true,
        };

    // Phase 155: Extract CFG information for hako_check
    let cfg_info = nyash_rust::mir::extract_cfg_info(module);

    // Phase 285LLVM-1.1: Extract user box declarations for LLVM harness
    let user_box_decls = collect_sorted_user_box_decl_values(module);
    let enum_decls = collect_sorted_enum_decl_values(module);

    let root = if use_v1_schema {
        let mut root = helpers::create_json_v1_root(json!(funs));
        // Add CFG data and user box declarations to v1 schema
        if let Some(obj) = root.as_object_mut() {
            obj.insert("cfg".to_string(), cfg_info);
            obj.insert("user_box_decls".to_string(), json!(user_box_decls)); // Phase 285LLVM-1.1
            obj.insert("enum_decls".to_string(), json!(enum_decls));
        }
        root
    } else {
        // v0 legacy format - also add CFG and user_box_decls
        json!({
            "functions": funs,
            "cfg": cfg_info,
            "user_box_decls": user_box_decls,  // Phase 285LLVM-1.1
            "enum_decls": enum_decls
        })
    };

    // NOTE: numeric_core strict validation is applied on the AotPrep output
    // (tools/hakorune_emit_mir.sh) rather than at raw MIR emit time. This keeps
    // pre-AotPrep MIR emission usable even when BoxCall(MatI64, mul_naive) is
    // still present.
    Ok(root)
}

fn build_array_text_state_residence_route_json(
    route: &crate::mir::ArrayTextStateResidenceRoute,
) -> serde_json::Value {
    let mut obj = json!({
        "observer_kind": route.contract.observer_kind.to_string(),
        "residence": route.contract.residence.to_string(),
        "result_repr": route.contract.result_repr.to_string(),
        "consumer_capability": route.contract.consumer_capability.to_string(),
        "publication_boundary": route.contract.publication_boundary.to_string(),
    });
    if let Some(payload) = route.temporary_indexof_seed_payload.as_ref() {
        obj["temporary_indexof_seed_payload"] =
            build_array_text_state_residence_indexof_seed_payload_json(payload);
    }
    obj
}

fn build_array_text_state_residence_indexof_seed_payload_json(
    payload: &crate::mir::ArrayTextStateResidenceIndexOfSeedPayload,
) -> serde_json::Value {
    json!({
        "variant": payload.variant.to_string(),
        "rows": payload.rows,
        "ops": payload.ops,
        "flip_period": payload.flip_period,
        "line_seed": payload.line_seed.as_str(),
        "line_seed_len": payload.line_seed_len,
        "none_seed": payload.none_seed.as_str(),
        "none_seed_len": payload.none_seed_len,
        "needle": payload.needle.as_str(),
        "needle_len": payload.needle_len,
        "proof": payload.proof.to_string(),
        "result_use": payload.result_use.to_string(),
        "backend_action": payload.backend_action.to_string(),
        "candidate_outcomes": [
            {
                "literal": payload.line_seed.as_str(),
                "outcome": payload.line_seed_outcome.to_string(),
            },
            {
                "literal": payload.none_seed.as_str(),
                "outcome": payload.none_seed_outcome.to_string(),
            },
        ],
    })
}

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

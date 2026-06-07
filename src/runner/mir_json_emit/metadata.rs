use super::agg_local::build_agg_local_scalarization_routes_json;
use super::array_metadata::insert_array_metadata_json;
use super::core_metadata::insert_core_metadata_json;
use super::exact_numeric::insert_exact_numeric_metadata_json;
use super::fastmem_metadata::insert_fastmem_metadata_json;
use super::metadata_seed::{
    build_concat_const_suffix_micro_seed_route_json, build_exact_seed_backend_route_json,
    build_substring_views_micro_seed_route_json, build_sum_variant_project_seed_route_json,
    build_sum_variant_tag_seed_route_json, build_userbox_local_scalar_seed_route_json,
};
use super::placement_effect::build_placement_effect_routes_json;
use super::plan_metadata::insert_plan_metadata_json;
use super::plans::build_string_kernel_plan_json;
use super::route_metadata::insert_route_metadata_json;
use crate::mir::MirFunction;
use serde_json::json;

pub(super) fn build_function_metadata_json(f: &MirFunction) -> serde_json::Value {
    let metadata = &f.metadata;
    let mut metadata_json = json!({
        "span_access_plans": metadata.span_access_plans.iter().map(|plan| {
            json!({
                "block": plan.block.as_u32(),
                "instruction_index": plan.instruction_index,
                "span_id": plan.span_id,
                "op": plan.op.as_str(),
                "index_value": plan.index_value.as_u32(),
                "value_value": plan.value_value.map(|value| value.as_u32()),
                "result_value": plan.result_value.map(|value| value.as_u32()),
                "element_type": plan.element_type.as_str(),
                "route": plan.route,
                "bounds_policy": plan.bounds_policy,
                "proof_ids": plan.proof_ids,
                "fallback_policy": plan.fallback_policy,
            })
        }).collect::<Vec<_>>(),
        "record_state_field_access_plans": metadata.record_state_field_access_plans.iter().map(|plan| {
            json!({
                "block": plan.block.as_u32(),
                "instruction_index": plan.instruction_index,
                "owner_box": plan.owner_box,
                "candidate_record": plan.candidate_record,
                "field_name": plan.field_name,
                "op": plan.op,
                "value": plan.value.map(|value| value.as_u32()),
                "result": plan.result.map(|value| value.as_u32()),
                "route": plan.route,
                "storage": plan.storage.as_str(),
                "proof_ids": plan.proof_ids,
                "lowering_enabled": plan.lowering_enabled,
                "fallback_policy": plan.fallback_policy,
                "summary": plan.summary,
            })
        }).collect::<Vec<_>>(),
        "required_fastpath_regions": metadata.required_fastpath_regions.iter().map(|region| {
            json!({
                "region_id": region.region_id,
                "source_kind": region.source_kind,
                "relevant_access_policy": region.relevant_access_policy,
                "route_requirement": region.route_requirement,
                "bounds_requirement": region.bounds_requirement,
                "fallback_policy": region.fallback_policy,
            })
        }).collect::<Vec<_>>(),
        "fastpath_obligations": metadata.fastpath_obligations.iter().map(|obligation| {
            json!({
                "obligation_id": obligation.obligation_id,
                "region_id": obligation.region_id,
                "block": obligation.block.as_u32(),
                "instruction_index": obligation.instruction_index,
                "access_kind": obligation.access_kind,
                "op": obligation.op,
                "expected": obligation.expected,
                "actual_plan_kind": obligation.actual_plan_kind,
                "actual_route": obligation.actual_route,
                "bounds_policy": obligation.bounds_policy,
                "proof_ids": obligation.proof_ids,
                "status": obligation.status,
                "failure_code": obligation.failure_code,
                "failure_reason": obligation.failure_reason,
            })
        }).collect::<Vec<_>>(),
        "effect_summaries": metadata.effect_summaries.iter().map(|summary| {
            json!({
                "method": summary.method,
                "receiver_value": summary.receiver_value.map(|value| value.as_u32()),
                "receiver_reads": summary.receiver_reads,
                "receiver_writes": summary.receiver_writes,
                "foreign_reads": summary.foreign_reads,
                "foreign_writes": summary.foreign_writes,
                "handle_publications": summary.handle_publications,
                "nested_call_count": summary.nested_call_count,
                "allocation_count": summary.allocation_count,
                "safepoint_count": summary.safepoint_count,
                "branch_count": summary.branch_count,
                "loop_like_count": summary.loop_like_count,
                "foreign_base_count": summary.foreign_base_count,
                "candidate_kind": summary.candidate_kind,
                "summary": summary.summary,
                "failure_reason": summary.failure_reason,
            })
        }).collect::<Vec<_>>(),
        "receiver_snapshot_publication_plans": metadata.receiver_snapshot_publication_plans.iter().map(|plan| {
            json!({
                "method": plan.method,
                "receiver_value": plan.receiver_value.map(|value| value.as_u32()),
                "foreign_base_count": plan.foreign_base_count,
                "receiver_reads": plan.receiver_reads,
                "receiver_writes": plan.receiver_writes,
                "foreign_reads": plan.foreign_reads,
                "handle_publications": plan.handle_publications,
                "publication_kind": plan.publication_kind,
                "barrier_policy": plan.barrier_policy,
                "handle_publication_proof_kind": plan.handle_publication_proof_kind,
                "lifetime_policy": plan.lifetime_policy,
                "lowering_consumer_enabled": plan.lowering_consumer_enabled,
                "summary": plan.summary,
                "failure_reason": plan.failure_reason,
            })
        }).collect::<Vec<_>>(),
        "hotcore_method_summaries": metadata.hotcore_method_summaries.iter().map(|summary| {
            json!({
                "method": summary.method,
                "block_count": summary.block_count,
                "instruction_count": summary.instruction_count,
                "return_kind": summary.return_kind,
                "allocation_count": summary.allocation_count,
                "provider_call_count": summary.provider_call_count,
                "public_observer_count": summary.public_observer_count,
                "result_capsule_materialization_count": summary.result_capsule_materialization_count,
                "safepoint_count": summary.safepoint_count,
                "generic_method_route_count": summary.generic_method_route_count,
                "generic_method_fallback_count": summary.generic_method_fallback_count,
                "dynamic_route_count": summary.dynamic_route_count,
                "boxed_fallback_count": summary.boxed_fallback_count,
                "nested_call_count": summary.nested_call_count,
                "nested_direct_exact_call_count": summary.nested_direct_exact_call_count,
                "direct_array_access_plan_count": summary.direct_array_access_plan_count,
                "direct_array_proved_unchecked_count": summary.direct_array_proved_unchecked_count,
                "summary": summary.summary,
                "failure_reason": summary.failure_reason,
            })
        }).collect::<Vec<_>>(),
        "direct_exact_hotcore_call_plans": metadata.direct_exact_hotcore_call_plans.iter().map(|plan| {
            json!({
                "route_id": "direct_exact.hotcore_call",
                "block": plan.block.as_u32(),
                "instruction_index": plan.instruction_index,
                "caller": plan.caller,
                "callee": plan.callee,
                "box_name": plan.box_name,
                "method": plan.method,
                "receiver_value": plan.receiver_value.as_u32(),
                "result_value": plan.result_value.map(|value| value.as_u32()),
                "receiver_exact": plan.receiver_exact,
                "same_module": plan.same_module,
                "dispatch_policy": plan.dispatch_policy,
                "call_boundary_policy": plan.call_boundary_policy,
                "return_shape": plan.return_shape,
                "value_demand": plan.value_demand,
                "callee_summary_status": plan.callee_summary_status,
                "lowering_consumer_enabled": plan.lowering_consumer_enabled,
                "generic_method_dispatch": plan.generic_method_dispatch,
                "dynamic_route": plan.dynamic_route,
                "boxed_fallback": plan.boxed_fallback,
                "summary": plan.summary,
                "failure_reason": plan.failure_reason,
            })
        }).collect::<Vec<_>>(),
        "storage_classes": metadata.value_storage_classes.iter().map(|(k, v)| {
            (k.as_u32().to_string(), json!(v.to_string()))
        }).collect::<serde_json::Map<String, serde_json::Value>>(),
        "string_corridor_facts": metadata.string_corridor_facts.iter().map(|(k, fact)| {
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
        "string_corridor_relations": metadata.string_corridor_relations.iter().map(|(k, relations)| {
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
        "string_corridor_candidates": metadata.string_corridor_candidates.iter().map(|(k, candidates)| {
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
        "string_kernel_plans": metadata.string_kernel_plans.iter().map(|(k, plan)| {
            (k.as_u32().to_string(), build_string_kernel_plan_json(plan))
        }).collect::<serde_json::Map<String, serde_json::Value>>(),
        "string_direct_set_window_routes": metadata.string_direct_set_window_routes.iter().map(|route| {
            json!({
                "route_id": "string.direct_set_source_window",
                "block": route.block().as_u32(),
                "instruction_index": route.instruction_index(),
                "second_instruction_index": route.second_instruction_index(),
                "concat_instruction_index": route.concat_instruction_index(),
                "source_value": route.source_value().as_u32(),
                "prefix_value": route.prefix_value().as_u32(),
                "suffix_value": route.suffix_value().as_u32(),
                "middle_value": route.middle_value().as_u32(),
                "split_value": route.split_value().as_u32(),
                "result_value": route.result_value().as_u32(),
                "subrange_start": route.subrange_start().as_u32(),
                "subrange_end": route.subrange_end().as_u32(),
                "skip_instruction_indices": route.skip_instruction_indices(),
                "proof": route.proof(),
                "consumer": "direct_set",
                "effects": ["observe.substring", "defer.piecewise", "direct.set.consumer"],
            })
        }).collect::<Vec<_>>(),
        "generic_method_routes": metadata.generic_method_routes.iter().map(|route| {
            let core_method = route.core_method().map(|carrier| {
                json!({
                    "op": carrier.op.to_string(),
                    "proof": carrier.proof.to_string(),
                    "lowering_tier": carrier.lowering_tier.to_string(),
                })
            });
            json!({
                "route_id": route.route_id(),
                "block": route.block().as_u32(),
                "instruction_index": route.instruction_index(),
                "box_name": route.box_name(),
                "method": route.method(),
                "receiver_origin_box": route.receiver_origin_box(),
                "result_origin_box": route.result_origin_box(),
                "key_route": route.key_route().map(|key_route| key_route.to_string()),
                "key_const_text": route.key_const_text(),
                "arity": route.arity(),
                "receiver_value": route.receiver_value().as_u32(),
                "key_value": route.key_value().map(|value| value.as_u32()),
                "result_value": route.result_value().map(|value| value.as_u32()),
                "emit_kind": route.emit_kind(),
                "route_kind": route.route_kind_tag(),
                "helper_symbol": route.helper_symbol(),
                "proof": route.proof_tag(),
                "core_method": core_method,
                "return_shape": route.return_shape().map(|shape| shape.to_string()),
                "value_demand": route.value_demand().to_string(),
                "publication_policy": route.publication_policy().map(|policy| policy.to_string()),
                "effects": route.effect_tags(),
            })
        }).collect::<Vec<_>>(),
        "route_decisions": metadata.route_decisions.iter().map(|decision| {
            json!({
                "route_id": "route.decision",
                "site_id": decision.site_id,
                "block": decision.block.as_u32(),
                "instruction_index": decision.instruction_index,
                "semantic_op": decision.semantic_op,
                "access_kind": decision.access_kind,
                "preferred_route": decision.preferred_route,
                "selected_route": decision.selected_route,
                "fallback_route": decision.fallback_route,
                "fallback_policy": decision.fallback_policy,
                "proof_ids": decision.proof_ids,
                "miss_reason": decision.miss_reason,
                "source_plan_kind": decision.source_plan_kind,
            })
        }).collect::<Vec<_>>(),
        "concat_const_suffix_micro_seed_route": metadata.concat_const_suffix_micro_seed_route.as_ref().map(build_concat_const_suffix_micro_seed_route_json),
        "substring_views_micro_seed_route": metadata.substring_views_micro_seed_route.as_ref().map(build_substring_views_micro_seed_route_json),
        "sum_variant_tag_seed_route": metadata.sum_variant_tag_seed_route.as_ref().map(build_sum_variant_tag_seed_route_json),
        "sum_variant_project_seed_route": metadata.sum_variant_project_seed_route.as_ref().map(build_sum_variant_project_seed_route_json),
        "userbox_local_scalar_seed_route": metadata.userbox_local_scalar_seed_route.as_ref().map(build_userbox_local_scalar_seed_route_json),
        "exact_seed_backend_route": metadata.exact_seed_backend_route.as_ref().map(build_exact_seed_backend_route_json),
        "thin_entry_candidates": metadata.thin_entry_candidates.iter().map(|candidate| {
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
        "thin_entry_selections": metadata.thin_entry_selections.iter().map(|selection| {
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
        "sum_placement_facts": metadata.sum_placement_facts.iter().map(|fact| {
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
        "sum_placement_selections": metadata.sum_placement_selections.iter().map(|selection| {
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
        "sum_placement_layouts": metadata.sum_placement_layouts.iter().map(|layout| {
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
            &metadata.agg_local_scalarization_routes,
        ),
        "placement_effect_routes": build_placement_effect_routes_json(
            &metadata.placement_effect_routes,
        ),
    });
    if let serde_json::Value::Object(obj) = &mut metadata_json {
        insert_core_metadata_json(obj, metadata);
        insert_array_metadata_json(obj, metadata);
        insert_exact_numeric_metadata_json(obj, metadata);
        insert_route_metadata_json(obj, f, metadata);
    }
    if let serde_json::Value::Object(obj) = &mut metadata_json {
        insert_fastmem_metadata_json(obj, metadata);
        insert_plan_metadata_json(obj, metadata);
    }
    metadata_json
}

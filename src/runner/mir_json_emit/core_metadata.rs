use crate::mir::{
    function::FunctionMetadata, value_representation_fact::ValueRepresentationFact, MirType,
};
use serde_json::json;

pub(super) fn insert_core_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "value_types".to_string(),
        json!(metadata
            .value_types
            .iter()
            .map(|(k, v)| {
                let type_str = match v {
                    MirType::Integer => json!("i64"),
                    MirType::Float => json!("f64"),
                    MirType::String => json!({"kind": "string"}),
                    MirType::Box(bt) => json!({"kind": "handle", "box_type": bt}),
                    MirType::Bool => json!("i1"),
                    MirType::Void => json!("void"),
                    MirType::Unknown => json!(null),
                    _ => json!(null),
                };
                (k.as_u32().to_string(), type_str)
            })
            .collect::<serde_json::Map<String, serde_json::Value>>()),
    );
    obj.insert(
        "value_consumer_facts".to_string(),
        json!(metadata
            .value_consumer_facts
            .iter()
            .map(|(k, facts)| {
                (
                    k.as_u32().to_string(),
                    json!({
                        "direct_set_consumer": facts.direct_set_consumer,
                    }),
                )
            })
            .collect::<serde_json::Map<String, serde_json::Value>>()),
    );
    obj.insert(
        "value_representations".to_string(),
        json!(metadata
            .value_representations
            .iter()
            .map(|(k, fact)| {
                let row = match fact {
                    ValueRepresentationFact::BoxedSumHandle { abi_plan_id } => json!({
                        "kind": "boxed_sum_handle",
                        "abi_plan_id": abi_plan_id,
                    }),
                };
                (k.as_u32().to_string(), row)
            })
            .collect::<serde_json::Map<String, serde_json::Value>>()),
    );
    obj.insert(
        "loop_range_facts".to_string(),
        json!(metadata
            .loop_range_facts
            .iter()
            .map(|fact| {
                json!({
                    "index_name": fact.index_name.as_str(),
                    "start_value": fact.start_value.as_u32(),
                    "end_value": fact.end_value.as_u32(),
                    "index_phi": fact.index_phi.as_u32(),
                    "preheader_bb": fact.preheader_bb.as_u32(),
                    "header_bb": fact.header_bb.as_u32(),
                    "body_bb": fact.body_bb.as_u32(),
                    "step_bb": fact.step_bb.as_u32(),
                    "exit_bb": fact.exit_bb.as_u32(),
                    "step": fact.step,
                    "end_exclusive": fact.end_exclusive,
                    "index_read_only": fact.index_read_only,
                    "body_local_writes_supported": fact.body_local_writes_supported,
                    "loop_carried_writes_supported": fact.loop_carried_writes_supported,
                    "body_writes_supported": fact.body_writes_supported,
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "counting_loop_facts".to_string(),
        json!(metadata
            .counting_loop_facts
            .iter()
            .map(|fact| {
                json!({
                    "index_name": fact.index_name.as_str(),
                    "lower_value": fact.lower_value.as_u32(),
                    "upper_exclusive_value": fact.upper_exclusive_value.as_u32(),
                    "index_value": fact.index_value.as_u32(),
                    "preheader_bb": fact.preheader_bb.as_u32(),
                    "header_bb": fact.header_bb.as_u32(),
                    "body_bb": fact.body_bb.as_u32(),
                    "latch_bb": fact.latch_bb.as_u32(),
                    "exit_bb": fact.exit_bb.as_u32(),
                    "step": fact.step,
                    "end_exclusive": fact.end_exclusive,
                    "index_body_read_only": fact.index_body_read_only,
                    "loop_carried_writes_supported": fact.loop_carried_writes_supported,
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "range_index_facts".to_string(),
        json!(metadata
            .range_index_facts
            .iter()
            .map(|fact| {
                json!({
                    "fact_id": fact.fact_id,
                    "origin_kind": fact.origin_kind.as_str(),
                    "index_value": fact.index_value.as_u32(),
                    "lower_value": fact.lower_value.as_u32(),
                    "upper_exclusive_value": fact.upper_exclusive_value.as_u32(),
                    "body_bb": fact.body_bb.as_u32(),
                    "step": fact.step,
                    "end_exclusive": fact.end_exclusive,
                    "index_body_read_only": fact.index_body_read_only,
                    "loop_carried_writes_supported": fact.loop_carried_writes_supported,
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "direct_array_extent_facts".to_string(),
        json!(metadata
            .direct_array_extent_facts
            .iter()
            .map(|fact| {
                json!({
                    "receiver_value": fact.receiver_value.as_u32(),
                    "lower_bound_value": fact.lower_bound_value.as_u32(),
                    "proof_kind": fact.proof_kind.as_str(),
                    "region_stability_fact_id": fact.region_stability_fact_id,
                    "stable_in_region": fact.stable_in_region,
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "region_stability_facts".to_string(),
        json!(metadata
            .region_stability_facts
            .iter()
            .map(|fact| {
                json!({
                    "fact_id": fact.fact_id,
                    "region_value": fact.region_value.as_u32(),
                    "scope_bb": fact.scope_bb.as_u32(),
                    "proof_kind": fact.proof_kind.as_str(),
                    "stable_in_region": fact.stable_in_region,
                })
            })
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "span_borrow_facts".to_string(),
        json!(metadata
            .span_borrow_facts
            .iter()
            .map(|fact| {
                json!({
                    "span_id": fact.span_id,
                    "span_value": fact.span_value.as_u32(),
                    "region_value": fact.region_value.as_u32(),
                    "owner_value": fact.owner_value.as_u32(),
                    "mutability": fact.mutability.as_str(),
                    "element_type": fact.element_type.as_str(),
                    "start_value": fact.start_value.as_u32(),
                    "length_value": fact.length_value.as_u32(),
                    "scope_bb": fact.scope_bb.as_u32(),
                    "no_escape": fact.no_escape,
                    "owner_stable": fact.owner_stable,
                    "region_stability_fact_id": fact.region_stability_fact_id,
                })
            })
            .collect::<Vec<_>>()),
    );
}

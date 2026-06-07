use crate::mir::fastmem_access_plan::{FastMemAccessPlan, FastMemAccessPlanPayload};
use crate::mir::function::{FastMemRegionOrigin, FunctionMetadata};
use serde_json::json;

pub(super) fn insert_fastmem_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "fastmem_regions".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_regions
                .iter()
                .map(|region| {
                    json!({
                        "id": region.id.0,
                        "contract": region.contract,
                        "origin": fastmem_region_origin_name(region.origin),
                        "body_statement_count": region.body_statement_count,
                        "emitted_memop_count": region.emitted_memop_count,
                        "source_span": {
                            "start": region.source_span.start,
                            "end": region.source_span.end,
                            "line": region.source_span.line,
                            "column": region.source_span.column,
                        },
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_table_length_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_table_length_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "table_id": fact.table_id,
                        "table_value": fact.table_value.as_u32(),
                        "length_value": fact.length_value.as_u32(),
                        "resolved_length": fact.resolved_length,
                        "policy": fact.policy.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_same_owner_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_same_owner_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "page_value": fact.page_value.as_u32(),
                        "proof_value": fact.proof_value.as_u32(),
                        "proof_kind": fact.proof_kind.as_str(),
                        "remote_owner_rejected": fact.remote_owner_rejected,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_remote_owner_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_remote_owner_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "page_value": fact.page_value.as_u32(),
                        "proof_kind": fact.proof_kind.as_str(),
                        "same_owner_rejected": fact.same_owner_rejected,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_block_next_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_block_next_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "block_value": fact.block_value.as_u32(),
                        "next_field_id": &fact.next_field_id,
                        "proof_kind": fact.proof_kind.as_str(),
                        "writable": fact.writable,
                        "provenance_valid": fact.provenance_valid,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_local_free_non_empty_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_local_free_non_empty_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "page_value": fact.page_value.as_u32(),
                        "proof_kind": fact.proof_kind.as_str(),
                        "non_empty": fact.non_empty,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_free_head_non_empty_facts".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_free_head_non_empty_facts
                .iter()
                .map(|fact| {
                    json!({
                        "fact_id": fact.fact_id,
                        "region": fact.region.0,
                        "page_value": fact.page_value.as_u32(),
                        "proof_kind": fact.proof_kind.as_str(),
                        "non_empty": fact.non_empty,
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_access_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_access_plans
                .iter()
                .map(build_fastmem_access_plan_json)
                .collect(),
        ),
    );
    obj.insert(
        "fastmem_table_field_access_links".to_string(),
        serde_json::Value::Array(
            metadata
                .fastmem_table_field_access_links
                .iter()
                .map(|link| {
                    json!({
                        "table_block": link.table_block.as_u32(),
                        "table_instruction_index": link.table_instruction_index,
                        "field_block": link.field_block.as_u32(),
                        "field_instruction_index": link.field_instruction_index,
                        "region": link.region.0,
                        "table_result": link.table_result.as_u32(),
                        "field_base": link.field_base.as_u32(),
                        "field_id": link.field_id,
                        "field_access": link.field_access.as_str(),
                        "byte_offset": link.byte_offset,
                        "field_size": link.field_size,
                        "field_type": link.field_type,
                        "alignment": link.alignment,
                        "proof": link.proof,
                    })
                })
                .collect(),
        ),
    );
}

pub(super) fn build_fastmem_access_plan_json(plan: &FastMemAccessPlan) -> serde_json::Value {
    let mut row = json!({
        "block": plan.block.as_u32(),
        "instruction_index": plan.instruction_index,
        "region": plan.region.0,
        "kind": plan.kind.as_str(),
        "status": plan.status.as_str(),
        "verified": plan.is_verified(),
        "failure_reason": &plan.failure_reason,
    });
    if let serde_json::Value::Object(map) = &mut row {
        match &plan.payload {
            FastMemAccessPlanPayload::Field(field) => {
                map.insert("layout_id".to_string(), json!(&field.layout_id));
                map.insert("field_id".to_string(), json!(&field.field_id));
                map.insert("base".to_string(), json!(field.base.as_u32()));
                map.insert(
                    "value".to_string(),
                    json!(field.value.map(|value| value.as_u32())),
                );
                map.insert(
                    "result".to_string(),
                    json!(field.result.map(|value| value.as_u32())),
                );
                map.insert("access".to_string(), json!(field.mode.as_str()));
                map.insert("byte_offset".to_string(), json!(field.byte_offset));
                map.insert("field_size".to_string(), json!(field.field_size));
                map.insert("field_type".to_string(), json!(&field.field_type));
                map.insert("alignment".to_string(), json!(field.alignment));
                map.insert("mutability".to_string(), json!(&field.mutability));
                map.insert("field_class".to_string(), json!(&field.field_class));
            }
            FastMemAccessPlanPayload::Table(table) => {
                map.insert("table_id".to_string(), json!(&table.table_id));
                map.insert("table".to_string(), json!(table.table.as_u32()));
                map.insert("index".to_string(), json!(table.index.as_u32()));
                map.insert(
                    "result".to_string(),
                    json!(table.result.map(|value| value.as_u32())),
                );
                map.insert(
                    "element_layout_id".to_string(),
                    json!(&table.element_layout_id),
                );
                map.insert("element_repr".to_string(), json!(&table.element_repr));
                map.insert("element_stride".to_string(), json!(table.element_stride));
                map.insert("element_size".to_string(), json!(table.element_size));
                map.insert("length".to_string(), json!(table.length));
                map.insert("alignment".to_string(), json!(table.alignment));
                map.insert("index_policy".to_string(), json!(&table.index_policy));
                map.insert(
                    "table_length_resolved".to_string(),
                    json!(table.proof.table_length_resolved),
                );
                map.insert(
                    "bounds_proof_valid".to_string(),
                    json!(table.proof.bounds_proof_valid),
                );
                map.insert(
                    "stride_resolved".to_string(),
                    json!(table.proof.stride_resolved),
                );
                map.insert(
                    "field_offset_resolved".to_string(),
                    json!(table.proof.field_offset_resolved),
                );
                map.insert(
                    "overflow_proof_valid".to_string(),
                    json!(table.proof.overflow_proof_valid),
                );
                map.insert(
                    "alignment_valid".to_string(),
                    json!(table.proof.alignment_valid),
                );
                map.insert(
                    "element_layout_verified".to_string(),
                    json!(table.proof.element_layout_verified),
                );
                map.insert(
                    "table_length_policy".to_string(),
                    json!(&table.proof.table_length_policy),
                );
                map.insert("bounds_proof".to_string(), json!(&table.proof.bounds_proof));
                map.insert(
                    "overflow_proof".to_string(),
                    json!(&table.proof.overflow_proof),
                );
                map.insert(
                    "table_access_proof_failure_reason".to_string(),
                    json!(&table.proof.failure_reason),
                );
            }
            FastMemAccessPlanPayload::LocalFree(local_free) => {
                map.insert("page".to_string(), json!(local_free.page.as_u32()));
                map.insert(
                    "block_value".to_string(),
                    json!(local_free.block.map(|value| value.as_u32())),
                );
                map.insert(
                    "result".to_string(),
                    json!(local_free.result.map(|value| value.as_u32())),
                );
                map.insert(
                    "local_free_head_layout_id".to_string(),
                    json!(&local_free.local_free_head_layout_id),
                );
                map.insert(
                    "local_free_head_field_id".to_string(),
                    json!(&local_free.local_free_head_field_id),
                );
                map.insert(
                    "local_free_head_field_class".to_string(),
                    json!(&local_free.local_free_head_field_class),
                );
                map.insert(
                    "local_free_head_byte_offset".to_string(),
                    json!(local_free.local_free_head_byte_offset),
                );
                map.insert(
                    "local_free_head_field_size".to_string(),
                    json!(local_free.local_free_head_field_size),
                );
                map.insert(
                    "local_free_head_field_type".to_string(),
                    json!(&local_free.local_free_head_field_type),
                );
                map.insert(
                    "local_free_head_alignment".to_string(),
                    json!(local_free.local_free_head_alignment),
                );
                map.insert(
                    "block_next_layout_id".to_string(),
                    json!(&local_free.block_next_layout_id),
                );
                map.insert(
                    "block_next_field_id".to_string(),
                    json!(&local_free.block_next_field_id),
                );
                map.insert(
                    "block_next_field_class".to_string(),
                    json!(&local_free.block_next_field_class),
                );
                map.insert(
                    "block_next_byte_offset".to_string(),
                    json!(local_free.block_next_byte_offset),
                );
                map.insert(
                    "block_next_field_size".to_string(),
                    json!(local_free.block_next_field_size),
                );
                map.insert(
                    "block_next_field_type".to_string(),
                    json!(&local_free.block_next_field_type),
                );
                map.insert(
                    "block_next_alignment".to_string(),
                    json!(local_free.block_next_alignment),
                );
                map.insert(
                    "same_owner_proof_valid".to_string(),
                    json!(local_free.same_owner_proof_valid),
                );
                map.insert(
                    "block_next_proof_valid".to_string(),
                    json!(local_free.block_next_proof_valid),
                );
                map.insert(
                    "non_empty_proof_valid".to_string(),
                    json!(local_free.non_empty_proof_valid),
                );
                map.insert(
                    "remote_owner_rejected".to_string(),
                    json!(local_free.remote_owner_rejected),
                );
                map.insert("lowerable".to_string(), json!(local_free.lowerable));
            }
            FastMemAccessPlanPayload::FreeHead(free_head) => {
                map.insert("page".to_string(), json!(free_head.page.as_u32()));
                map.insert(
                    "block_value".to_string(),
                    json!(free_head.block.map(|value| value.as_u32())),
                );
                map.insert(
                    "result".to_string(),
                    json!(free_head.result.map(|value| value.as_u32())),
                );
                map.insert(
                    "free_head_layout_id".to_string(),
                    json!(&free_head.free_head_layout_id),
                );
                map.insert(
                    "free_head_field_id".to_string(),
                    json!(&free_head.free_head_field_id),
                );
                map.insert(
                    "free_head_field_class".to_string(),
                    json!(&free_head.free_head_field_class),
                );
                map.insert(
                    "free_head_byte_offset".to_string(),
                    json!(free_head.free_head_byte_offset),
                );
                map.insert(
                    "free_head_field_size".to_string(),
                    json!(free_head.free_head_field_size),
                );
                map.insert(
                    "free_head_field_type".to_string(),
                    json!(&free_head.free_head_field_type),
                );
                map.insert(
                    "free_head_alignment".to_string(),
                    json!(free_head.free_head_alignment),
                );
                map.insert(
                    "block_next_layout_id".to_string(),
                    json!(&free_head.block_next_layout_id),
                );
                map.insert(
                    "block_next_field_id".to_string(),
                    json!(&free_head.block_next_field_id),
                );
                map.insert(
                    "block_next_field_class".to_string(),
                    json!(&free_head.block_next_field_class),
                );
                map.insert(
                    "block_next_byte_offset".to_string(),
                    json!(free_head.block_next_byte_offset),
                );
                map.insert(
                    "block_next_field_size".to_string(),
                    json!(free_head.block_next_field_size),
                );
                map.insert(
                    "block_next_field_type".to_string(),
                    json!(&free_head.block_next_field_type),
                );
                map.insert(
                    "block_next_alignment".to_string(),
                    json!(free_head.block_next_alignment),
                );
                map.insert(
                    "same_owner_proof_valid".to_string(),
                    json!(free_head.same_owner_proof_valid),
                );
                map.insert(
                    "block_next_proof_valid".to_string(),
                    json!(free_head.block_next_proof_valid),
                );
                map.insert(
                    "non_empty_proof_valid".to_string(),
                    json!(free_head.non_empty_proof_valid),
                );
                map.insert(
                    "remote_owner_rejected".to_string(),
                    json!(free_head.remote_owner_rejected),
                );
                map.insert("lowerable".to_string(), json!(free_head.lowerable));
            }
            FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) => {
                map.insert("page".to_string(), json!(remote_head.page.as_u32()));
                map.insert(
                    "block_value".to_string(),
                    json!(remote_head.block.map(|value| value.as_u32())),
                );
                map.insert(
                    "result".to_string(),
                    json!(remote_head.result.map(|value| value.as_u32())),
                );
                map.insert(
                    "remote_head_layout_id".to_string(),
                    json!(&remote_head.remote_head_layout_id),
                );
                map.insert(
                    "remote_head_field_id".to_string(),
                    json!(&remote_head.remote_head_field_id),
                );
                map.insert(
                    "remote_head_field_class".to_string(),
                    json!(&remote_head.remote_head_field_class),
                );
                map.insert(
                    "remote_head_byte_offset".to_string(),
                    json!(remote_head.remote_head_byte_offset),
                );
                map.insert(
                    "remote_head_field_size".to_string(),
                    json!(remote_head.remote_head_field_size),
                );
                map.insert(
                    "remote_head_field_type".to_string(),
                    json!(&remote_head.remote_head_field_type),
                );
                map.insert(
                    "remote_head_alignment".to_string(),
                    json!(remote_head.remote_head_alignment),
                );
                map.insert(
                    "block_next_layout_id".to_string(),
                    json!(&remote_head.block_next_layout_id),
                );
                map.insert(
                    "block_next_field_id".to_string(),
                    json!(&remote_head.block_next_field_id),
                );
                map.insert(
                    "block_next_field_class".to_string(),
                    json!(&remote_head.block_next_field_class),
                );
                map.insert(
                    "block_next_byte_offset".to_string(),
                    json!(remote_head.block_next_byte_offset),
                );
                map.insert(
                    "block_next_field_size".to_string(),
                    json!(remote_head.block_next_field_size),
                );
                map.insert(
                    "block_next_field_type".to_string(),
                    json!(&remote_head.block_next_field_type),
                );
                map.insert(
                    "block_next_alignment".to_string(),
                    json!(remote_head.block_next_alignment),
                );
                map.insert(
                    "remote_owner_required".to_string(),
                    json!(remote_head.remote_owner_required),
                );
                map.insert(
                    "remote_owner_proof_valid".to_string(),
                    json!(remote_head.remote_owner_proof_valid),
                );
                map.insert(
                    "block_next_required".to_string(),
                    json!(remote_head.block_next_required),
                );
                map.insert(
                    "block_next_proof_valid".to_string(),
                    json!(remote_head.block_next_proof_valid),
                );
                map.insert(
                    "memory_order_policy".to_string(),
                    json!(&remote_head.memory_order_policy),
                );
                map.insert(
                    "retry_attempt_limit".to_string(),
                    json!(remote_head.retry_attempt_limit),
                );
                map.insert("lowerable".to_string(), json!(remote_head.lowerable));
            }
            FastMemAccessPlanPayload::DrainRemoteListToLocal(drain) => {
                map.insert("page".to_string(), json!(drain.page.as_u32()));
                map.insert("token".to_string(), json!(drain.token.as_u32()));
                map.insert(
                    "token_source_block".to_string(),
                    json!(drain.token_source_block.map(|block| block.as_u32())),
                );
                map.insert(
                    "token_source_instruction_index".to_string(),
                    json!(drain.token_source_instruction_index),
                );
                map.insert(
                    "token_provenance_valid".to_string(),
                    json!(drain.token_provenance_valid),
                );
                map.insert(
                    "page_operand_valid".to_string(),
                    json!(drain.page_operand_valid),
                );
                map.insert(
                    "head_class_resolved".to_string(),
                    json!(drain.head_class_resolved),
                );
                map.insert(
                    "local_list_head_class".to_string(),
                    json!(&drain.local_list_head_class),
                );
                map.insert(
                    "local_free_head_layout_id".to_string(),
                    json!(&drain.local_free_head_layout_id),
                );
                map.insert(
                    "local_free_head_field_id".to_string(),
                    json!(&drain.local_free_head_field_id),
                );
                map.insert(
                    "local_free_head_field_class".to_string(),
                    json!(&drain.local_free_head_field_class),
                );
                map.insert(
                    "local_free_head_byte_offset".to_string(),
                    json!(drain.local_free_head_byte_offset),
                );
                map.insert(
                    "local_free_head_field_size".to_string(),
                    json!(drain.local_free_head_field_size),
                );
                map.insert(
                    "local_free_head_field_type".to_string(),
                    json!(&drain.local_free_head_field_type),
                );
                map.insert(
                    "local_free_head_alignment".to_string(),
                    json!(drain.local_free_head_alignment),
                );
                map.insert(
                    "block_next_layout_id".to_string(),
                    json!(&drain.block_next_layout_id),
                );
                map.insert(
                    "block_next_field_id".to_string(),
                    json!(&drain.block_next_field_id),
                );
                map.insert(
                    "block_next_field_class".to_string(),
                    json!(&drain.block_next_field_class),
                );
                map.insert(
                    "block_next_byte_offset".to_string(),
                    json!(drain.block_next_byte_offset),
                );
                map.insert(
                    "block_next_field_size".to_string(),
                    json!(drain.block_next_field_size),
                );
                map.insert(
                    "block_next_field_type".to_string(),
                    json!(&drain.block_next_field_type),
                );
                map.insert(
                    "block_next_alignment".to_string(),
                    json!(drain.block_next_alignment),
                );
                map.insert(
                    "block_next_access_resolved".to_string(),
                    json!(drain.block_next_access_resolved),
                );
                map.insert(
                    "publication_order".to_string(),
                    json!(&drain.publication_order),
                );
                map.insert("lowerable".to_string(), json!(drain.lowerable));
            }
        }
    }
    row
}

fn fastmem_region_origin_name(origin: FastMemRegionOrigin) -> &'static str {
    match origin {
        FastMemRegionOrigin::SourceFastMemBlock => "source_fastmem_block",
    }
}

use crate::mir::fastmem_access_plan::{FastMemAccessPlan, FastMemAccessPlanPayload};
use serde_json::json;

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

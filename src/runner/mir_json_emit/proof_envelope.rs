use crate::mir::direct_array_access_plan::DirectArrayAccessPlan;
use crate::mir::fastmem_access_plan::{
    FastMemAccessPlan, FastMemAccessPlanPayload, FastMemFieldAccessPlan, FastMemResolvedFieldPlan,
    FastMemTableAccessProof,
};
use crate::mir::function::FunctionMetadata;
use serde_json::json;

pub(super) fn build_proof_envelopes_json(metadata: &FunctionMetadata) -> serde_json::Value {
    let mut envelopes = Vec::new();
    envelopes.extend(
        metadata
            .direct_array_access_plans
            .iter()
            .map(build_direct_array_proof_envelope_json),
    );
    envelopes.extend(
        metadata
            .fastmem_access_plans
            .iter()
            .map(build_fastmem_proof_envelope_json),
    );
    serde_json::Value::Array(envelopes)
}

fn build_direct_array_proof_envelope_json(plan: &DirectArrayAccessPlan) -> serde_json::Value {
    json!({
        "site": {
            "block": plan.block().as_u32(),
            "instruction_index": plan.instruction_index(),
            "route_id": "direct_array.access",
            "op": plan.op().as_str(),
        },
        "profile": "direct_array",
        "producer": "mir_json",
        "proof_ids": plan.proof_ids(),
        "obligation_ids": [plan.proof_kind().as_str()],
        "verifier_flags": {
            "bounds_policy": plan.bounds_policy().as_str(),
            "fallback_policy": plan.fallback_policy().as_str(),
            "cfg_shape": plan.cfg_shape().as_str(),
            "store_semantics": plan.store_semantics().as_str(),
        },
        "failure_reason": serde_json::Value::Null,
    })
}

fn build_fastmem_proof_envelope_json(plan: &FastMemAccessPlan) -> serde_json::Value {
    let (proof_ids, verifier_flags) = match &plan.payload {
        FastMemAccessPlanPayload::Field(field) => (
            field_proof_ids(field),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "field_mode": field.mode.as_str(),
                "layout_id_resolved": field.layout_id.is_some(),
                "field_id_resolved": !field.field_id.is_empty(),
                "byte_offset_resolved": field.byte_offset.is_some(),
                "field_size_resolved": field.field_size.is_some(),
                "field_type_resolved": field.field_type.is_some(),
                "alignment_resolved": field.alignment.is_some(),
            }),
        ),
        FastMemAccessPlanPayload::Table(table) => (
            table_proof_ids(&table.proof),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "table_id_resolved": !table.table_id.is_empty(),
                "element_layout_id_resolved": table.element_layout_id.is_some(),
                "element_repr_resolved": table.element_repr.is_some(),
                "element_stride_resolved": table.element_stride.is_some(),
                "element_size_resolved": table.element_size.is_some(),
                "length_resolved": table.length.is_some(),
                "alignment_resolved": table.alignment.is_some(),
                "index_policy_resolved": table.index_policy.is_some(),
                "table_length_resolved": table.proof.table_length_resolved,
                "bounds_proof_valid": table.proof.bounds_proof_valid,
                "stride_resolved": table.proof.stride_resolved,
                "field_offset_resolved": table.proof.field_offset_resolved,
                "overflow_proof_valid": table.proof.overflow_proof_valid,
                "alignment_valid": table.proof.alignment_valid,
                "element_layout_verified": table.proof.element_layout_verified,
            }),
        ),
        FastMemAccessPlanPayload::LocalFree(local_free) => (
            field_plan_proof_ids("local_free", &local_free.local_free_head)
                .into_iter()
                .chain(field_plan_proof_ids("block_next", &local_free.block_next))
                .collect(),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "same_owner_proof_valid": local_free.same_owner_proof_valid,
                "block_next_proof_valid": local_free.block_next_proof_valid,
                "non_empty_proof_valid": local_free.non_empty_proof_valid,
                "remote_owner_rejected": local_free.remote_owner_rejected,
                "lowerable": local_free.lowerable,
            }),
        ),
        FastMemAccessPlanPayload::FreeHead(free_head) => (
            field_plan_proof_ids("free_head", &free_head.free_head)
                .into_iter()
                .chain(field_plan_proof_ids("block_next", &free_head.block_next))
                .collect(),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "same_owner_proof_valid": free_head.same_owner_proof_valid,
                "block_next_proof_valid": free_head.block_next_proof_valid,
                "non_empty_proof_valid": free_head.non_empty_proof_valid,
                "remote_owner_rejected": free_head.remote_owner_rejected,
                "lowerable": free_head.lowerable,
            }),
        ),
        FastMemAccessPlanPayload::AtomicRemoteHead(remote_head) => (
            field_plan_proof_ids("remote_head", &remote_head.remote_head)
                .into_iter()
                .chain(field_plan_proof_ids("block_next", &remote_head.block_next))
                .collect(),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "remote_owner_required": remote_head.remote_owner_required,
                "remote_owner_proof_valid": remote_head.remote_owner_proof_valid,
                "block_next_required": remote_head.block_next_required,
                "block_next_proof_valid": remote_head.block_next_proof_valid,
                "memory_order_policy": remote_head.memory_order_policy,
                "retry_attempt_limit": remote_head.retry_attempt_limit,
                "lowerable": remote_head.lowerable,
            }),
        ),
        FastMemAccessPlanPayload::DrainRemoteListToLocal(drain) => (
            field_plan_proof_ids("local_free_head", &drain.local_free_head)
                .into_iter()
                .chain(field_plan_proof_ids("block_next", &drain.block_next))
                .collect(),
            json!({
                "status": plan.status.as_str(),
                "verified": plan.is_verified(),
                "token_provenance_valid": drain.token_provenance_valid,
                "page_operand_valid": drain.page_operand_valid,
                "head_class_resolved": drain.head_class_resolved,
                "block_next_access_resolved": drain.block_next_access_resolved,
                "publication_order": drain.publication_order,
                "lowerable": drain.lowerable,
            }),
        ),
    };

    json!({
        "site": {
            "block": plan.block.as_u32(),
            "instruction_index": plan.instruction_index,
            "region": plan.region.0,
            "kind": plan.kind.as_str(),
        },
        "profile": "fastmem",
        "producer": "mir_json",
        "proof_ids": proof_ids,
        "obligation_ids": [plan.kind.as_str()],
        "verifier_flags": verifier_flags,
        "failure_reason": &plan.failure_reason,
    })
}

fn table_proof_ids(proof: &FastMemTableAccessProof) -> Vec<&'static str> {
    let mut ids = Vec::new();
    if proof.table_length_resolved {
        ids.push("table_length_resolved");
    }
    if proof.bounds_proof_valid {
        ids.push("bounds_proof_valid");
    }
    if proof.stride_resolved {
        ids.push("stride_resolved");
    }
    if proof.field_offset_resolved {
        ids.push("field_offset_resolved");
    }
    if proof.overflow_proof_valid {
        ids.push("overflow_proof_valid");
    }
    if proof.alignment_valid {
        ids.push("alignment_valid");
    }
    if proof.element_layout_verified {
        ids.push("element_layout_verified");
    }
    ids
}

fn field_proof_ids(field: &FastMemFieldAccessPlan) -> Vec<&'static str> {
    let mut ids = Vec::new();
    if field.layout_id.is_some() {
        ids.push("layout_id_resolved");
    }
    if !field.field_id.is_empty() {
        ids.push("field_id_resolved");
    }
    if field.byte_offset.is_some() {
        ids.push("byte_offset_resolved");
    }
    if field.field_size.is_some() {
        ids.push("field_size_resolved");
    }
    if field.field_type.is_some() {
        ids.push("field_type_resolved");
    }
    if field.alignment.is_some() {
        ids.push("alignment_resolved");
    }
    ids
}

fn field_plan_proof_ids(prefix: &str, field: &FastMemResolvedFieldPlan) -> Vec<&'static str> {
    let mut ids = Vec::new();
    if field.layout_id.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_layout_id_resolved",
            "free_head" => "free_head_layout_id_resolved",
            "local_free_head" => "local_free_head_layout_id_resolved",
            "block_next" => "block_next_layout_id_resolved",
            _ => "layout_id_resolved",
        });
    }
    if field.field_id.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_field_id_resolved",
            "free_head" => "free_head_field_id_resolved",
            "local_free_head" => "local_free_head_field_id_resolved",
            "block_next" => "block_next_field_id_resolved",
            _ => "field_id_resolved",
        });
    }
    if field.byte_offset.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_byte_offset_resolved",
            "free_head" => "free_head_byte_offset_resolved",
            "local_free_head" => "local_free_head_byte_offset_resolved",
            "block_next" => "block_next_byte_offset_resolved",
            _ => "byte_offset_resolved",
        });
    }
    if field.field_size.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_field_size_resolved",
            "free_head" => "free_head_field_size_resolved",
            "local_free_head" => "local_free_head_field_size_resolved",
            "block_next" => "block_next_field_size_resolved",
            _ => "field_size_resolved",
        });
    }
    if field.field_type.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_field_type_resolved",
            "free_head" => "free_head_field_type_resolved",
            "local_free_head" => "local_free_head_field_type_resolved",
            "block_next" => "block_next_field_type_resolved",
            _ => "field_type_resolved",
        });
    }
    if field.alignment.is_some() {
        ids.push(match prefix {
            "remote_head" => "remote_head_alignment_resolved",
            "free_head" => "free_head_alignment_resolved",
            "local_free_head" => "local_free_head_alignment_resolved",
            "block_next" => "block_next_alignment_resolved",
            _ => "alignment_resolved",
        });
    }
    ids
}

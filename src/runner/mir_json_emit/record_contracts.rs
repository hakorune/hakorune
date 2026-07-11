use crate::mir::function::{FunctionMetadata, RecordContractDisposition, RecordValueBoundaryKind};
use serde_json::json;

pub(super) fn insert_record_contract_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "record_value_contracts".to_string(),
        json!(metadata
            .record_value_contracts
            .iter()
            .map(|contract| json!({
                "contract_id": contract.contract_id,
                "boundary": match contract.boundary {
                    RecordValueBoundaryKind::Construct => "construct",
                    RecordValueBoundaryKind::WithUpdate => "with_update",
                },
                "diagnostic_record_name": contract.diagnostic_record_name,
                "schema_fingerprint": contract.schema_fingerprint,
                "dst_value_id": contract.dst_value_id.as_u32(),
                "base_value_id": contract.base_value_id.map(|value| value.as_u32()),
                "backend_capability_required": contract.backend_capability_required,
                "fields": contract.fields.iter().map(|field| json!({
                    "field_index": field.field_index,
                    "diagnostic_field_name": field.diagnostic_field_name,
                    "value_id": field.value_id.as_u32(),
                    "declared_type_name": field.declared_type_name,
                    "disposition": match &field.disposition {
                        RecordContractDisposition::AnyDefault => "any_default",
                        RecordContractDisposition::RuntimeCheckedContract => "runtime_checked_contract",
                        RecordContractDisposition::VerifierProvenContract { .. } => "verifier_proven_contract",
                        RecordContractDisposition::UnsupportedFailFast => "unsupported_failfast",
                    },
                    "proof_id": match &field.disposition {
                        RecordContractDisposition::VerifierProvenContract { proof_id } => Some(proof_id),
                        _ => None,
                    },
                })).collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>()),
    );
}

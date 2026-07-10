use crate::mir::function::FunctionMetadata;
use serde_json::json;

pub(super) fn insert_local_contract_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "local_slot_contracts".to_string(),
        json!(metadata
            .local_slot_contracts
            .iter()
            .map(|contract| json!({
                "contract_id": contract.contract_id,
                "local_slot_id": contract.local_slot_id.binding_id().raw(),
                "diagnostic_source_name": contract.diagnostic_source_name,
                "declared_type_name": contract.declared_type_name,
                "contract_kind": "exact_numeric",
                "runtime_check_required": contract.runtime_check_required,
                "proof_elision_allowed": contract.proof_elision_allowed,
                "backend_capability_required": contract.backend_capability_required,
            }))
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "local_identity_evidence".to_string(),
        json!(metadata
            .local_identity_evidence
            .iter()
            .map(|evidence| json!({
                "local_slot_id": evidence.local_slot_id.binding_id().raw(),
                "merge_value_id": evidence.merge_value_id.as_u32(),
                "incoming_values": evidence
                    .incoming_values
                    .iter()
                    .map(|value| value.as_u32())
                    .collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>()),
    );
}

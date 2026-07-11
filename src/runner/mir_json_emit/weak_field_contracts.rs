use crate::mir::MirFunction;
use serde_json::json;

pub(super) fn insert_weak_field_contract_metadata_json(
    object: &mut serde_json::Map<String, serde_json::Value>,
    function: &MirFunction,
) {
    object.insert(
        "weak_field_write_contracts".to_string(),
        json!(function
            .metadata
            .weak_field_write_contracts
            .iter()
            .map(|contract| json!({
                "site_id": contract.site_id.0,
                "contract_id": contract.contract_id,
                "base": contract.base_value_id.as_u32(),
                "value": contract.value_id.as_u32(),
                "box_schema_fingerprint": contract.box_schema_fingerprint,
                "field_index": contract.field_index,
                "runtime_check_required": contract.runtime_check_required,
                "proof_elision_allowed": contract.proof_elision_allowed,
                "backend_capability_required": contract.backend_capability_required,
            }))
            .collect::<Vec<_>>()),
    );
}

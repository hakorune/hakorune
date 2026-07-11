use crate::mir::function::{
    TypedArrayBoundaryValue, TypedArrayContractBoundary, TypedArrayContractSourceIdentity,
};
use crate::mir::MirFunction;
use serde_json::json;

pub(super) fn insert_typed_array_contract_metadata_json(
    object: &mut serde_json::Map<String, serde_json::Value>,
    function: &MirFunction,
) {
    object.insert(
        "typed_array_element_contracts".to_string(),
        json!(function
            .metadata
            .typed_array_element_contracts
            .iter()
            .map(|contract| json!({
                "contract_id": contract.contract_id,
                "boundary": boundary_name(contract.boundary),
                "source_identity": source_identity_json(&contract.source_identity),
                "array_value": match contract.boundary_value {
                    TypedArrayBoundaryValue::Value(value) => Some(value.as_u32()),
                    TypedArrayBoundaryValue::FinalReturn => None,
                },
                "state_term": contract.state_term.map(|term| term.0),
                "element_spec": { "kind": contract.element_spec.element.source_name() },
                "disposition": "runtime_checked_contract",
                "runtime_check_required": contract.runtime_check_required,
                "proof_elision_allowed": contract.proof_elision_allowed,
                "backend_capability_required": contract.backend_capability_required,
            }))
            .collect::<Vec<_>>()),
    );
}

fn boundary_name(boundary: TypedArrayContractBoundary) -> &'static str {
    match boundary {
        TypedArrayContractBoundary::LocalInit => "local_init",
        TypedArrayContractBoundary::LocalReassign => "local_reassign",
        TypedArrayContractBoundary::ParameterEntry => "parameter_entry",
        TypedArrayContractBoundary::ReturnExit => "return_exit",
        TypedArrayContractBoundary::BoxFieldWrite => "box_field_write",
        TypedArrayContractBoundary::RecordConstruct => "record_construct",
        TypedArrayContractBoundary::RecordWithUpdate => "record_with_update",
    }
}

fn source_identity_json(identity: &TypedArrayContractSourceIdentity) -> serde_json::Value {
    match identity {
        TypedArrayContractSourceIdentity::LocalSlot(slot) => json!({
            "kind": "local_slot", "local_slot_id": slot.binding_id().raw(),
        }),
        TypedArrayContractSourceIdentity::Parameter { formal_index } => json!({
            "kind": "parameter", "formal_index": formal_index,
        }),
        TypedArrayContractSourceIdentity::Return => json!({ "kind": "return" }),
        TypedArrayContractSourceIdentity::BoxField {
            box_name,
            field_index,
        } => json!({
            "kind": "box_field", "box_name": box_name, "field_index": field_index,
        }),
        TypedArrayContractSourceIdentity::RecordField {
            schema_fingerprint,
            field_index,
            update,
        } => json!({
            "kind": "record_field",
            "schema_fingerprint": schema_fingerprint,
            "field_index": field_index,
            "update": update,
        }),
    }
}

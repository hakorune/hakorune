use crate::mir::function::{FunctionMetadata, ParameterEntryContractKind};
use serde_json::json;

pub(super) fn insert_parameter_contract_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "declared_param_decls".to_string(),
        json!(metadata
            .declared_param_decls
            .iter()
            .map(|declaration| json!({
                "name": declaration.name,
                "declared_type_name": declaration.declared_type_name,
                "implicit_receiver": declaration.implicit_receiver,
            }))
            .collect::<Vec<_>>()),
    );
    obj.insert(
        "parameter_entry_contracts".to_string(),
        json!(metadata
            .parameter_entry_contracts
            .iter()
            .map(|contract| json!({
                "contract_id": contract.contract_id,
                "formal_parameter_index": contract.formal_parameter_index,
                "source_parameter_index": contract.source_parameter_index,
                "parameter_value_id": contract.parameter_value_id.as_u32(),
                "source_parameter_name": contract.source_parameter_name,
                "declared_type_name": contract.declared_type_name,
                "contract_kind": contract_kind_name(contract.contract_kind),
                "implicit_receiver": contract.implicit_receiver,
                "runtime_check_required": contract.runtime_check_required,
                "proof_elision_allowed": contract.proof_elision_allowed,
                "backend_capability_required": contract.backend_capability_required,
            }))
            .collect::<Vec<_>>()),
    );
}

fn contract_kind_name(kind: ParameterEntryContractKind) -> &'static str {
    match kind {
        ParameterEntryContractKind::ExactNumeric => "exact_numeric",
    }
}

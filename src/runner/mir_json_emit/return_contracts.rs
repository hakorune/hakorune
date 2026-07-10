use crate::mir::function::{
    FunctionMetadata, ReturnExitContractKind, ReturnExitContractOwner, ReturnExitVoidPolicy,
};
use serde_json::{json, Map, Value};

pub(super) fn insert_return_contract_metadata_json(
    object: &mut Map<String, Value>,
    metadata: &FunctionMetadata,
) {
    object.insert(
        "declared_return_type_name".to_string(),
        metadata
            .declared_return_type_name
            .as_ref()
            .map_or(Value::Null, |name| json!(name)),
    );
    object.insert(
        "return_exit_contract".to_string(),
        metadata
            .return_exit_contract
            .as_ref()
            .map_or(Value::Null, |contract| {
                json!({
                    "contract_id": contract.contract_id,
                    "declared_type_name": contract.declared_type_name,
                    "contract_kind": contract_kind_name(contract.contract_kind),
                    "void_policy": void_policy_name(contract.void_policy),
                    "runtime_check_required": contract.runtime_check_required,
                    "proof_elision_allowed": contract.proof_elision_allowed,
                    "backend_capability_required": contract.backend_capability_required,
                    "source_return_annotation_present": contract.source_return_annotation_present,
                    "owner": owner_name(contract.owner),
                })
            }),
    );
}

fn contract_kind_name(kind: ReturnExitContractKind) -> &'static str {
    match kind {
        ReturnExitContractKind::ExactNumeric => "exact_numeric",
    }
}

fn void_policy_name(policy: ReturnExitVoidPolicy) -> &'static str {
    match policy {
        ReturnExitVoidPolicy::RejectVoid => "reject_void",
    }
}

fn owner_name(owner: ReturnExitContractOwner) -> &'static str {
    match owner {
        ReturnExitContractOwner::FunctionReturnContract => "function_return_contract",
    }
}

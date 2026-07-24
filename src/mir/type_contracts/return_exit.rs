use crate::mir::function::{
    ReturnExitContract, ReturnExitContractKind, ReturnExitContractOwner, ReturnExitVoidPolicy,
};
use crate::mir::numeric_substrate::{exact_numeric_mir_type_from_declared_name, NumericTarget};
use crate::mir::type_contracts::guarantee_matrix::exact_numeric_return_exit_contract_is_active;
use crate::mir::MirFunction;

pub(crate) const RETURN_EXIT_EXACT_NUMERIC_CAPABILITY: &str = "return_exit_exact_numeric";
pub(crate) const RETURN_CONTRACT_CARRIER_MISSING_TAG: &str =
    "[type/return_contract_carrier_missing]";
pub(crate) const RETURN_CONTRACT_CARRIER_DRIFT_TAG: &str = "[type/return_contract_carrier_drift]";
pub(crate) const RETURN_CONTRACT_FALLTHROUGH_TAG: &str =
    "[type/return_contract_fallthrough_forbidden]";

pub(crate) fn refresh_function_return_exit_contract(function: &mut MirFunction) {
    if !exact_numeric_return_exit_contract_is_active() {
        function.metadata.return_exit_contract = None;
        return;
    }
    function.metadata.return_exit_contract =
        build_return_exit_contract(function.metadata.declared_return_type_name.as_deref());
}

fn build_return_exit_contract(declared_type_name: Option<&str>) -> Option<ReturnExitContract> {
    let declared_type_name = declared_type_name?;
    exact_numeric_mir_type_from_declared_name(Some(declared_type_name), NumericTarget::host())?;
    Some(ReturnExitContract {
        contract_id: format!("return-exit:{declared_type_name}"),
        declared_type_name: declared_type_name.to_string(),
        contract_kind: ReturnExitContractKind::ExactNumeric,
        void_policy: ReturnExitVoidPolicy::RejectVoid,
        runtime_check_required: true,
        proof_elision_allowed: false,
        backend_capability_required: RETURN_EXIT_EXACT_NUMERIC_CAPABILITY.to_string(),
        source_return_annotation_present: true,
        owner: ReturnExitContractOwner::FunctionReturnContract,
    })
}

/// Source-side relation used by the pre-Builder function-exit seal.
///
/// This is deliberately only a borrowed predicate. The executable
/// `ReturnExitContract` remains produced by the existing MIR metadata owner
/// after materialization; F1 must not create a second carrier.
pub(crate) fn exact_numeric_return_exit_relation_expected(
    declared_type_name: Option<&str>,
) -> bool {
    exact_numeric_return_exit_contract_is_active()
        && build_return_exit_contract(declared_type_name).is_some()
}

pub(crate) fn validate_return_exit_contract(function: &MirFunction) -> Result<(), String> {
    let expected =
        build_return_exit_contract(function.metadata.declared_return_type_name.as_deref());
    match (&expected, &function.metadata.return_exit_contract) {
        (None, None) => Ok(()),
        (Some(_), None) => Err(format!(
            "{} function={} declared_return={:?}",
            RETURN_CONTRACT_CARRIER_MISSING_TAG,
            function.signature.name,
            function.metadata.declared_return_type_name
        )),
        (None, Some(actual)) => Err(format!(
            "{} function={} unexpected={:?}",
            RETURN_CONTRACT_CARRIER_DRIFT_TAG, function.signature.name, actual
        )),
        (Some(expected), Some(actual)) if expected == actual => Ok(()),
        (Some(expected), Some(actual)) => Err(format!(
            "{} function={} expected={:?} actual={:?}",
            RETURN_CONTRACT_CARRIER_DRIFT_TAG, function.signature.name, expected, actual
        )),
    }
}

#[cfg(test)]
mod tests;

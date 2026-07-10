use crate::mir::function::{MirParamDecl, ParameterEntryContract, ParameterEntryContractKind};
use crate::mir::numeric_substrate::{exact_numeric_mir_type_from_declared_name, NumericTarget};
use crate::mir::type_contracts::guarantee_matrix::exact_numeric_parameter_entry_contract_is_active;
use crate::mir::MirFunction;
use std::collections::BTreeSet;

pub(crate) const PARAMETER_ENTRY_EXACT_NUMERIC_CAPABILITY: &str = "parameter_entry_exact_numeric";

pub(crate) const PARAMETER_CONTRACT_CARRIER_MISSING_TAG: &str =
    "[type/parameter_contract_carrier_missing]";
pub(crate) const PARAMETER_CONTRACT_ROW_DRIFT_TAG: &str = "[type/parameter_contract_row_drift]";
pub(crate) const PARAMETER_CONTRACT_DUPLICATE_INDEX_TAG: &str =
    "[type/parameter_contract_duplicate_index]";
pub(crate) const PARAMETER_CONTRACT_IMPLICIT_RECEIVER_TAG: &str =
    "[type/parameter_contract_implicit_receiver_forbidden]";

pub(crate) fn refresh_function_parameter_entry_contracts(function: &mut MirFunction) {
    if !exact_numeric_parameter_entry_contract_is_active() {
        function.metadata.parameter_entry_contracts.clear();
        return;
    }
    function.metadata.parameter_entry_contracts =
        build_parameter_entry_contracts(&function.metadata.declared_param_decls, &function.params);
}

fn build_parameter_entry_contracts(
    declarations: &[MirParamDecl],
    parameters: &[crate::mir::ValueId],
) -> Vec<ParameterEntryContract> {
    let mut source_parameter_index = 0;
    let mut contracts = Vec::new();

    for (formal_parameter_index, declaration) in declarations.iter().enumerate() {
        if declaration.implicit_receiver {
            continue;
        }

        let current_source_index = source_parameter_index;
        source_parameter_index += 1;

        let Some(declared_type_name) = declaration.declared_type_name.as_deref() else {
            continue;
        };
        if exact_numeric_mir_type_from_declared_name(
            Some(declared_type_name),
            NumericTarget::host(),
        )
        .is_none()
        {
            continue;
        }
        let Some(parameter_value_id) = parameters.get(formal_parameter_index).copied() else {
            continue;
        };

        contracts.push(ParameterEntryContract {
            contract_id: format!("parameter-entry:{}", formal_parameter_index),
            formal_parameter_index,
            source_parameter_index: current_source_index,
            parameter_value_id,
            source_parameter_name: declaration.name.clone(),
            declared_type_name: declared_type_name.to_string(),
            contract_kind: ParameterEntryContractKind::ExactNumeric,
            implicit_receiver: false,
            runtime_check_required: true,
            proof_elision_allowed: false,
            backend_capability_required: PARAMETER_ENTRY_EXACT_NUMERIC_CAPABILITY.to_string(),
        });
    }

    contracts
}

pub(crate) fn validate_parameter_entry_contracts(function: &MirFunction) -> Result<(), String> {
    for (formal_parameter_index, declaration) in
        function.metadata.declared_param_decls.iter().enumerate()
    {
        let is_exact_contract = !declaration.implicit_receiver
            && exact_numeric_mir_type_from_declared_name(
                declaration.declared_type_name.as_deref(),
                NumericTarget::host(),
            )
            .is_some();
        if is_exact_contract && formal_parameter_index >= function.params.len() {
            return Err(format!(
                "{} function={} index={} params={}",
                PARAMETER_CONTRACT_ROW_DRIFT_TAG,
                function.signature.name,
                formal_parameter_index,
                function.params.len()
            ));
        }
    }

    let expected =
        build_parameter_entry_contracts(&function.metadata.declared_param_decls, &function.params);
    if expected.is_empty() && !function.metadata.parameter_entry_contracts.is_empty() {
        return Err(format!(
            "{} function={} unexpected_rows={}",
            PARAMETER_CONTRACT_ROW_DRIFT_TAG,
            function.signature.name,
            function.metadata.parameter_entry_contracts.len()
        ));
    }
    if !expected.is_empty() && function.metadata.parameter_entry_contracts.is_empty() {
        return Err(format!(
            "{} function={} expected_rows={}",
            PARAMETER_CONTRACT_CARRIER_MISSING_TAG,
            function.signature.name,
            expected.len()
        ));
    }

    let mut indices = BTreeSet::new();
    for contract in &function.metadata.parameter_entry_contracts {
        if contract.implicit_receiver {
            return Err(format!(
                "{} function={} index={}",
                PARAMETER_CONTRACT_IMPLICIT_RECEIVER_TAG,
                function.signature.name,
                contract.formal_parameter_index
            ));
        }
        if !indices.insert(contract.formal_parameter_index) {
            return Err(format!(
                "{} function={} index={}",
                PARAMETER_CONTRACT_DUPLICATE_INDEX_TAG,
                function.signature.name,
                contract.formal_parameter_index
            ));
        }
    }

    if function.metadata.parameter_entry_contracts != expected {
        return Err(format!(
            "{} function={} expected={:?} actual={:?}",
            PARAMETER_CONTRACT_ROW_DRIFT_TAG,
            function.signature.name,
            expected,
            function.metadata.parameter_entry_contracts
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;

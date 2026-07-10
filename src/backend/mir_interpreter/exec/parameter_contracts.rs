use super::super::{MirInterpreter, VMError, VMValue};
use crate::mir::function::ParameterEntryContractKind;
use crate::mir::numeric_substrate::{
    exact_numeric_const_from_i128, exact_numeric_mir_type_from_declared_name,
    exact_numeric_value_from_dynamic_integer, ExactNumericConversionError, NumericTarget,
};
use crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts;
use crate::mir::MirFunction;

const PARAMETER_ARITY_MISMATCH_TAG: &str = "[type/parameter_arity_mismatch]";
const PARAMETER_CONTRACT_VIOLATION_TAG: &str = "[type/parameter_contract_violation]";

impl MirInterpreter {
    /// Authoritative final-callee parameter-entry check.
    ///
    /// The caller and representation facts have no acceptance authority. This
    /// runs after method rerouting and before register binding or body effects.
    pub(super) fn validate_function_entry_contracts(
        &self,
        function: &MirFunction,
        argument_values: Option<&[VMValue]>,
    ) -> Result<(), VMError> {
        validate_parameter_entry_contracts(function).map_err(|error| self.err_invalid(error))?;
        let contracts = &function.metadata.parameter_entry_contracts;
        if contracts.is_empty() {
            return Ok(());
        }

        let arguments = argument_values.unwrap_or(&[]);
        if arguments.len() != function.params.len() {
            return Err(self.err_invalid(format!(
                "{} function={} expected={} actual={}",
                PARAMETER_ARITY_MISMATCH_TAG,
                function.signature.name,
                function.params.len(),
                arguments.len()
            )));
        }

        for contract in contracts {
            let argument = &arguments[contract.formal_parameter_index];
            match contract.contract_kind {
                ParameterEntryContractKind::ExactNumeric => {
                    self.validate_exact_numeric_parameter(function, contract, argument)?;
                }
            }
        }
        Ok(())
    }

    fn validate_exact_numeric_parameter(
        &self,
        function: &MirFunction,
        contract: &crate::mir::function::ParameterEntryContract,
        argument: &VMValue,
    ) -> Result<(), VMError> {
        let Some(exact_type) = exact_numeric_mir_type_from_declared_name(
            Some(contract.declared_type_name.as_str()),
            NumericTarget::host(),
        ) else {
            return Err(self.parameter_contract_violation(
                function,
                contract,
                argument,
                "unknown-exact-type",
            ));
        };

        let result = match argument {
            VMValue::Integer(value) => {
                exact_numeric_value_from_dynamic_integer(*value, &exact_type).map(|_| ())
            }
            VMValue::ExactNumeric(value) if value.source_name == exact_type.source_name => {
                exact_numeric_const_from_i128(value.value, &exact_type).map(|_| ())
            }
            _ => {
                return Err(self.parameter_contract_violation(
                    function,
                    contract,
                    argument,
                    "runtime-type-mismatch",
                ));
            }
        };

        result.map_err(|error| {
            let reason = match error {
                ExactNumericConversionError::NegativeToUnsigned { .. } => "negative-to-unsigned",
                ExactNumericConversionError::OutOfRange { .. } => "out-of-range",
            };
            self.parameter_contract_violation(function, contract, argument, reason)
        })
    }

    fn parameter_contract_violation(
        &self,
        function: &MirFunction,
        contract: &crate::mir::function::ParameterEntryContract,
        argument: &VMValue,
        reason: &str,
    ) -> VMError {
        self.err_invalid(format!(
            "{} function={} index={} parameter={} declared_type={} actual={:?} reason={}",
            PARAMETER_CONTRACT_VIOLATION_TAG,
            function.signature.name,
            contract.formal_parameter_index,
            contract.source_parameter_name,
            contract.declared_type_name,
            argument,
            reason
        ))
    }
}

#[cfg(test)]
mod tests;

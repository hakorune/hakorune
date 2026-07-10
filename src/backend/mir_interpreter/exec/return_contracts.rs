use super::super::{MirInterpreter, VMError, VMValue};
use super::exact_numeric_value_checker::validate_exact_numeric_runtime_value;
use crate::mir::function::{ReturnExitContractKind, ReturnExitVoidPolicy};
use crate::mir::type_contracts::return_exit::validate_return_exit_contract;
use crate::mir::MirFunction;

pub(super) const RETURN_CONTRACT_VOID_TAG: &str = "[type/return_contract_void_forbidden]";
pub(super) const RETURN_CONTRACT_VIOLATION_TAG: &str = "[type/return_contract_violation]";

impl MirInterpreter {
    /// Authoritative final-callee return check. Call only for the final
    /// `BlockOutcome::Return`, before frame restoration publishes the result.
    pub(super) fn validate_function_return_contract(
        &self,
        function: &MirFunction,
        value: &VMValue,
    ) -> Result<(), VMError> {
        validate_return_exit_contract(function).map_err(|error| self.err_invalid(error))?;
        let Some(contract) = function.metadata.return_exit_contract.as_ref() else {
            return Ok(());
        };

        if contract.void_policy == ReturnExitVoidPolicy::RejectVoid
            && matches!(value, VMValue::Void)
        {
            return Err(self.err_invalid(format!(
                "{} function={} declared_type={}",
                RETURN_CONTRACT_VOID_TAG, function.signature.name, contract.declared_type_name
            )));
        }

        match contract.contract_kind {
            ReturnExitContractKind::ExactNumeric => {
                validate_exact_numeric_runtime_value(value, &contract.declared_type_name).map_err(
                    |reason| {
                        self.err_invalid(format!(
                            "{} function={} declared_type={} actual={:?} reason={}",
                            RETURN_CONTRACT_VIOLATION_TAG,
                            function.signature.name,
                            contract.declared_type_name,
                            value,
                            reason
                        ))
                    },
                )
            }
        }
    }
}

#[cfg(test)]
mod tests;

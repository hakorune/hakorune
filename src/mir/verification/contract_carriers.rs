use crate::mir::verification_types::VerificationError;
use crate::mir::MirModule;

pub(super) fn check_contract_carrier_invariants(
    module: &MirModule,
) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    for function in module.functions.values() {
        collect_carrier_error(
            &mut errors,
            "parameter_entry_contracts",
            "FunctionEntryContractOwner",
            crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts(
                function,
            ),
        );
        collect_carrier_error(
            &mut errors,
            "return_exit_contract",
            "FunctionReturnContractOwner",
            crate::mir::type_contracts::return_exit::validate_return_exit_contract(function),
        );
        collect_carrier_error(
            &mut errors,
            "local_slot_contracts",
            "LocalSlotContractOwner",
            crate::mir::type_contracts::local_slot::validate_local_slot_contracts(function),
        );
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn collect_carrier_error(
    errors: &mut Vec<VerificationError>,
    key: &'static str,
    owner: &'static str,
    result: Result<(), String>,
) {
    if let Err(reason) = result {
        errors.push(VerificationError::ModuleMetadataInvariantViolation {
            key,
            owner: owner.to_string(),
            reason: format!("[type/contract_refresh_required] {}", reason),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    #[test]
    fn direct_verifier_guard_rejects_missing_return_carrier() {
        let mut module = MirModule::new("direct-bypass".to_string());
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.answer/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_return_type_name = Some("i64".to_string());
        module.add_function(function);

        let errors = check_contract_carrier_invariants(&module).unwrap_err();
        assert!(errors
            .iter()
            .any(|error| error.to_string().contains("type/contract_refresh_required")));
    }

    #[test]
    fn facade_refresh_satisfies_direct_verifier_guard() {
        let mut module = MirModule::new("facade".to_string());
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.answer/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_return_type_name = Some("i64".to_string());
        module.add_function(function);
        let _bundle = crate::mir::refresh_and_validate_for_boundary(
            &mut module,
            crate::mir::ContractRefreshBoundary::Verifier,
        )
        .unwrap();

        assert!(check_contract_carrier_invariants(&module).is_ok());
    }
}

use crate::mir::type_contracts::return_exit::validate_return_exit_contract;
use crate::mir::{MirModule, MirType};

pub(crate) const RETURN_EXIT_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/backend_return_contract_capability_missing]";
pub(crate) const LIFECYCLE_RETURN_EXIT_CAPABILITY_MISSING_TAG: &str =
    "[type/lifecycle_return_exit_capability_missing]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReturnExitBackendCapabilityReport {
    pub exact_numeric_contract_rows: usize,
}

pub(crate) fn inspect_return_exit_backend_capability(
    module: &MirModule,
) -> ReturnExitBackendCapabilityReport {
    ReturnExitBackendCapabilityReport {
        exact_numeric_contract_rows: module
            .functions
            .values()
            .filter(|function| function.metadata.return_exit_contract.is_some())
            .count(),
    }
}

pub(crate) fn enforce_return_exit_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    for function in module.functions.values() {
        validate_return_exit_contract(function)?;
    }
    let report = inspect_return_exit_backend_capability(module);
    if report.exact_numeric_contract_rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=return_exit_exact_numeric",
        RETURN_EXIT_BACKEND_CAPABILITY_MISSING_TAG, backend, report.exact_numeric_contract_rows
    ))
}

/// The lifecycle C consumer emits only the selected root/ordinary i64
/// functions.  Validate return contracts for those definitions instead of
/// reopening the generic module-wide capability gate.
pub(crate) fn enforce_lifecycle_return_exit_backend_supported(
    module: &MirModule,
    input: &crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    for physical in input.program().functions() {
        let symbol = match physical.role() {
            crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::Root { .. } => physical.name(),
            crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { key, .. } => module
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| format!("{} reason=ordinary-definition-missing", LIFECYCLE_RETURN_EXIT_CAPABILITY_MISSING_TAG))?,
            crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { .. } => continue,
        };
        let function = module.functions.get(symbol).ok_or_else(|| {
            format!(
                "{} reason=function-missing function={symbol}",
                LIFECYCLE_RETURN_EXIT_CAPABILITY_MISSING_TAG
            )
        })?;
        validate_return_exit_contract(function)?;
        if let Some(contract) = function.metadata.return_exit_contract.as_ref() {
            if contract.declared_type_name != "i64"
                || function.signature.return_type != MirType::Integer
            {
                return Err(format!(
                    "{} reason=non-i64-contract function={symbol}",
                    LIFECYCLE_RETURN_EXIT_CAPABILITY_MISSING_TAG
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::type_contracts::return_exit::refresh_function_return_exit_contract;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn module_with_contract() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.value/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_return_type_name = Some("i64".to_string());
        refresh_function_return_exit_contract(&mut function);
        let mut module = MirModule::new("return-backend-capability".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn interpreter_is_the_only_first_slice_consumer() {
        let module = module_with_contract();
        assert!(enforce_return_exit_backend_supported(&module, "mir-interpreter").is_ok());
        for backend in [
            "pyvm-harness",
            "ny-llvmc-exe",
            "llvmlite-obj",
            "wasm",
            "wasm-v2",
        ] {
            let error = enforce_return_exit_backend_supported(&module, backend).unwrap_err();
            assert!(error.contains(RETURN_EXIT_BACKEND_CAPABILITY_MISSING_TAG));
        }
    }

    #[test]
    fn uncontracted_modules_keep_existing_backend_behavior() {
        assert!(enforce_return_exit_backend_supported(
            &MirModule::new("plain".to_string()),
            "wasm"
        )
        .is_ok());
    }
}

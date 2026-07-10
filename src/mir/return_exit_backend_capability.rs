use crate::mir::type_contracts::return_exit::validate_return_exit_contract;
use crate::mir::MirModule;

pub(crate) const RETURN_EXIT_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/backend_return_contract_capability_missing]";

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

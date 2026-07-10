use crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts;
use crate::mir::MirModule;

pub(crate) const PARAMETER_ENTRY_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/backend_parameter_contract_capability_missing]";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ParameterEntryBackendCapabilityReport {
    pub exact_numeric_contract_rows: usize,
}

pub(crate) fn inspect_parameter_entry_backend_capability(
    module: &MirModule,
) -> ParameterEntryBackendCapabilityReport {
    ParameterEntryBackendCapabilityReport {
        exact_numeric_contract_rows: module
            .functions
            .values()
            .map(|function| function.metadata.parameter_entry_contracts.len())
            .sum(),
    }
}

pub(crate) fn enforce_parameter_entry_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    for function in module.functions.values() {
        validate_parameter_entry_contracts(function)?;
    }
    let report = inspect_parameter_entry_backend_capability(module);
    if report.exact_numeric_contract_rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }

    Err(format!(
        "{} backend={} contract_rows={} require=parameter_entry_exact_numeric",
        PARAMETER_ENTRY_BACKEND_CAPABILITY_MISSING_TAG, backend, report.exact_numeric_contract_rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::MirParamDecl;
    use crate::mir::type_contracts::parameter_entry::refresh_function_parameter_entry_contracts;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn module_with_contract() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.take/1".to_string(),
                params: vec![MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_param_decls = vec![MirParamDecl {
            name: "count".to_string(),
            declared_type_name: Some("u8".to_string()),
            implicit_receiver: false,
        }];
        refresh_function_parameter_entry_contracts(&mut function);

        let mut module = MirModule::new("parameter-backend-capability".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn interpreter_is_the_only_first_slice_consumer() {
        let module = module_with_contract();
        assert!(enforce_parameter_entry_backend_supported(&module, "mir-interpreter").is_ok());
        for backend in [
            "pyvm-harness",
            "ny-llvmc-exe",
            "llvmlite-obj",
            "wasm",
            "wasm-v2",
        ] {
            let error = enforce_parameter_entry_backend_supported(&module, backend).unwrap_err();
            assert!(
                error.contains(PARAMETER_ENTRY_BACKEND_CAPABILITY_MISSING_TAG),
                "{error}"
            );
        }
    }

    #[test]
    fn uncontracted_modules_keep_existing_backend_behavior() {
        let module = MirModule::new("plain".to_string());
        assert!(enforce_parameter_entry_backend_supported(&module, "wasm").is_ok());
    }
}

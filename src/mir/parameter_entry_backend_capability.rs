use crate::mir::type_contracts::parameter_entry::validate_parameter_entry_contracts;
use crate::mir::{MirModule, MirType};

pub(crate) const PARAMETER_ENTRY_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/backend_parameter_contract_capability_missing]";
pub(crate) const LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG: &str =
    "[type/lifecycle_parameter_entry_capability_missing]";

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

/// The lifecycle consumer has one selected ordinary function.  Its physical
/// body already carries the exact parameter ValueIds; only that function must
/// satisfy the i64 entry contract.  Generic backend callers keep the broader
/// module-wide gate above until their own consumers are cut over.
pub(crate) fn enforce_lifecycle_parameter_entry_backend_supported(
    module: &MirModule,
    input: &crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    let Some(ordinary) = input.entry().ordinary_call() else {
        return Ok(());
    };
    let function_index = usize::try_from(ordinary.function_index()).map_err(|_| {
        format!(
            "{} reason=function-index",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        )
    })?;
    let physical = input
        .program()
        .functions()
        .get(function_index)
        .ok_or_else(|| {
            format!(
                "{} reason=function-missing",
                LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
            )
        })?;
    let key = physical.role().ordinary_target().ok_or_else(|| {
        format!(
            "{} reason=ordinary-role-missing",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        )
    })?;
    let call_key = crate::mir::compiler::published_backend_view::ordinary_callable_key(
        &ordinary.call().callee,
    )?;
    if key != &call_key {
        return Err(format!(
            "{} reason=ordinary-key-drift",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        ));
    }
    let symbol = module
        .canonical_callable_definition_symbol(key)
        .ok_or_else(|| {
            format!(
                "{} reason=ordinary-definition-missing",
                LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
            )
        })?;
    let target = module.functions.get(symbol).ok_or_else(|| {
        format!(
            "{} reason=ordinary-function-missing",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        )
    })?;
    if physical.params() != target.params.as_slice()
        || physical.params().len() != ordinary.call().args.len()
    {
        return Err(format!(
            "{} reason=ordinary-parameter-value-drift function={symbol}",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        ));
    }
    validate_parameter_entry_contracts(target)?;
    if target.metadata.declared_param_decls.len() != target.params.len()
        || target.metadata.parameter_entry_contracts.len() != target.params.len()
        || target.signature.params.len() != target.params.len()
    {
        return Err(format!(
            "{} reason=ordinary-parameter-count function={symbol}",
            LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
        ));
    }
    for (index, ((declaration, contract), ty)) in target
        .metadata
        .declared_param_decls
        .iter()
        .zip(&target.metadata.parameter_entry_contracts)
        .zip(&target.signature.params)
        .enumerate()
    {
        if declaration.implicit_receiver
            || declaration.declared_type_name.as_deref() != Some("i64")
            || *ty != MirType::Integer
            || contract.formal_parameter_index != index
            || contract.source_parameter_index != index
            || contract.parameter_value_id != target.params[index]
            || contract.implicit_receiver
        {
            return Err(format!(
                "{} reason=ordinary-parameter-contract function={symbol} index={index}",
                LIFECYCLE_PARAMETER_ENTRY_CAPABILITY_MISSING_TAG
            ));
        }
    }
    Ok(())
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

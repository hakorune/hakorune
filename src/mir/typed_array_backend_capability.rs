use crate::mir::MirModule;

pub(crate) const BACKEND_UNSUPPORTED_TAG: &str = "[type/typed_array_contract_backend_unsupported]";

pub(crate) fn enforce_typed_array_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module
        .functions
        .values()
        .map(|function| function.metadata.typed_array_element_contracts.len())
        .sum::<usize>();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require={}",
        BACKEND_UNSUPPORTED_TAG,
        backend,
        rows,
        crate::mir::function::TYPED_ARRAY_EXACT_NUMERIC_CAPABILITY
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::MirParamDecl;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn module_with_contract() -> MirModule {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.take/1".to_string(),
                params: vec![MirType::Unknown],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.declared_param_decls = vec![MirParamDecl {
            name: "bytes".to_string(),
            declared_type_name: Some("Array<u8>".to_string()),
            implicit_receiver: false,
        }];
        let mut module = MirModule::new("typed-array-backend".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn only_reference_vm_supports_state_guard_v1() {
        let module = module_with_contract();
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(
                &module,
                "mir-interpreter"
            )
            .is_ok()
        );
        for backend in ["pyvm-harness", "ny-llvmc-exe", "llvmlite-obj", "wasm"] {
            let error =
                crate::mir::backend_capability::enforce_mir_backend_supported(&module, backend)
                    .unwrap_err();
            assert!(error.contains(BACKEND_UNSUPPORTED_TAG), "{error}");
        }
    }
}

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

/// Only the retained Script cohort may discharge the global typed-Array Stop.
/// Match every carrier to the exact input claim; no source meaning is inferred.
pub(crate) fn enforce_lifecycle_input(
    view: &crate::mir::function::PublishedMirBackendView<'_>,
    input: &crate::mir::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    use crate::mir::PublishedLifecycleRuntimeRequirementsV1;
    if input.runtime_requirements() != PublishedLifecycleRuntimeRequirementsV1::NativeArray {
        return enforce_typed_array_backend_supported(view.module(), "ny-llvmc-obj");
    }
    let reject = || format!("{BACKEND_UNSUPPORTED_TAG} native-input-coverage");
    let retained = view.retained_script_array().ok_or_else(reject)?;
    let issued = input
        .program()
        .handoff()
        .script_array()
        .ok_or_else(reject)?;
    if !std::ptr::eq(retained, issued) {
        return Err(reject());
    }
    enforce_claim_coverage(view.module(), input)
}

fn enforce_claim_coverage(
    module: &MirModule,
    input: &crate::mir::PublishedLifecyclePhysicalAbiInputV1<'_>,
) -> Result<(), String> {
    use crate::mir::function::{TypedArrayBoundaryValue, TypedArrayContractBoundary};
    let reject = || format!("{BACKEND_UNSUPPORTED_TAG} native-input-coverage");
    let [root] = input.program().functions() else {
        return Err(reject());
    };
    let claims = input.entry().array_claims();
    let mut consumed = std::collections::BTreeSet::new();
    for (name, function) in &module.functions {
        for row in &function.metadata.typed_array_element_contracts {
            if name != root.name() || row.boundary != TypedArrayContractBoundary::LocalInit {
                return Err(reject());
            }
            let mut matches = claims.iter().enumerate().filter(|(_, claim)| {
                row.contract_id == claim.contract_id
                    && row.boundary_value == TypedArrayBoundaryValue::Value(claim.array)
                    && row.element_spec == claim.spec
            });
            let Some((index, _)) = matches.next() else {
                return Err(reject());
            };
            if matches.next().is_some() || !consumed.insert(index) {
                return Err(reject());
            }
        }
    }
    if consumed.len() != claims.len() {
        return Err(reject());
    }
    Ok(())
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

#[cfg(test)]
#[path = "typed_array_backend_capability_tests.rs"]
mod native_tests;

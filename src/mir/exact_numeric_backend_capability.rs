use crate::mir::exact_numeric_field_contracts::enforce_exact_numeric_runtime_checks_supported;
use crate::mir::function::TypedObjectFieldStorage;
use crate::mir::MirModule;

pub(crate) const EXACT_NUMERIC_BACKEND_STORAGE_UNSUPPORTED_TAG: &str =
    "[freeze:backend][exact-numeric/storage-unsupported]";
pub(crate) const EXACT_NUMERIC_BACKEND_ROUTE_UNSUPPORTED_TAG: &str =
    "[freeze:backend][exact-numeric/route-unsupported]";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExactNumericBackendCapabilityReport {
    pub typed_object_exact_storage_fields: usize,
    pub exact_numeric_operation_route_facts: usize,
}

pub(crate) fn inspect_exact_numeric_backend_capability(
    module: &MirModule,
) -> ExactNumericBackendCapabilityReport {
    ExactNumericBackendCapabilityReport {
        typed_object_exact_storage_fields: exact_numeric_typed_object_storage_field_count(module),
        exact_numeric_operation_route_facts: exact_numeric_operation_route_fact_count(module),
    }
}

pub(crate) fn enforce_exact_numeric_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    enforce_exact_numeric_runtime_checks_supported(module, backend)?;

    let report = inspect_exact_numeric_backend_capability(module);
    let mut errors = Vec::new();
    if report.typed_object_exact_storage_fields > 0
        && !backend_supports_exact_typed_object_field_abi(backend)
    {
        errors.push(format!(
            "{} backend={} storage_fields={} require=exact-numeric-typed-object-storage-lowering",
            EXACT_NUMERIC_BACKEND_STORAGE_UNSUPPORTED_TAG,
            backend,
            report.typed_object_exact_storage_fields
        ));
    }
    if report.exact_numeric_operation_route_facts > 0
        && !backend_supports_exact_numeric_operation_routes(backend)
    {
        errors.push(format!(
            "{} backend={} route_facts={} require=exact-numeric-op-route-lowering",
            EXACT_NUMERIC_BACKEND_ROUTE_UNSUPPORTED_TAG,
            backend,
            report.exact_numeric_operation_route_facts
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn exact_numeric_typed_object_storage_field_count(module: &MirModule) -> usize {
    module
        .metadata
        .typed_object_plans
        .iter()
        .flat_map(|plan| plan.fields.iter())
        .filter(|field| exact_numeric_storage_requires_native_backend_slot(field.storage))
        .count()
}

fn exact_numeric_storage_requires_native_backend_slot(storage: TypedObjectFieldStorage) -> bool {
    !matches!(
        storage,
        TypedObjectFieldStorage::I64 | TypedObjectFieldStorage::Handle
    )
}

fn backend_supports_exact_typed_object_field_abi(backend: &str) -> bool {
    matches!(backend, "ny-llvmc-exe" | "ny-llvmc-obj" | "llvmlite-obj")
}

fn backend_supports_exact_numeric_operation_routes(backend: &str) -> bool {
    matches!(
        backend,
        "ny-llvmc-exe" | "ny-llvmc-obj" | "llvmlite-obj" | "pyvm-harness"
    )
}

fn exact_numeric_operation_route_fact_count(module: &MirModule) -> usize {
    module
        .functions
        .values()
        .map(|function| {
            function.metadata.exact_numeric_binary_op_route_facts.len()
                + function.metadata.exact_numeric_compare_route_facts.len()
                + function.metadata.exact_numeric_shift_route_facts.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::exact_numeric_value_facts::ExactNumericBinaryOpRouteFact;
    use crate::mir::function::{TypedObjectFieldPlan, TypedObjectPlan};
    use crate::mir::{
        BasicBlockId, BinaryOp, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
        ValueId,
    };

    fn module_with_storage(storage: TypedObjectFieldStorage) -> MirModule {
        let mut module = MirModule::new("test".to_string());
        module.metadata.typed_object_plans.push(TypedObjectPlan {
            box_name: "Page".to_string(),
            type_id: 1,
            layout_kind: "typed_object_v0".to_string(),
            field_count: 1,
            fields: vec![TypedObjectFieldPlan {
                name: "capacity".to_string(),
                slot: 0,
                declared_type_name: Some(storage.as_str().to_string()),
                storage,
                is_weak: false,
            }],
        });
        module
    }

    fn module_with_route_fact() -> MirModule {
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .metadata
            .exact_numeric_binary_op_route_facts
            .push(ExactNumericBinaryOpRouteFact {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                dst: ValueId::new(2),
                op: BinaryOp::Add,
                lhs: ValueId::new(0),
                rhs: ValueId::new(1),
                declared_type_name: "usize".to_string(),
            });

        let mut module = MirModule::new("test".to_string());
        module.add_function(function);
        module
    }

    #[test]
    fn backend_capability_accepts_legacy_i64_and_handle_storage() {
        for storage in [
            TypedObjectFieldStorage::I64,
            TypedObjectFieldStorage::Handle,
        ] {
            let module = module_with_storage(storage);
            assert_eq!(
                inspect_exact_numeric_backend_capability(&module).typed_object_exact_storage_fields,
                0
            );
            assert!(enforce_exact_numeric_backend_supported(&module, "ny-llvmc").is_ok());
        }
    }

    #[test]
    fn backend_capability_rejects_exact_typed_object_storage() {
        let module = module_with_storage(TypedObjectFieldStorage::USize);
        let err = enforce_exact_numeric_backend_supported(&module, "wasm").unwrap_err();
        assert!(err.contains(EXACT_NUMERIC_BACKEND_STORAGE_UNSUPPORTED_TAG));
        assert!(err.contains("storage_fields=1"));
    }

    #[test]
    fn backend_capability_accepts_exact_typed_object_storage_for_python_llvm_field_abi() {
        let module = module_with_storage(TypedObjectFieldStorage::USize);
        assert!(enforce_exact_numeric_backend_supported(&module, "ny-llvmc-exe").is_ok());
        assert!(enforce_exact_numeric_backend_supported(&module, "llvmlite-obj").is_ok());
    }

    #[test]
    fn backend_capability_accepts_exact_operation_route_facts_for_python_llvm_and_pyvm() {
        let module = module_with_route_fact();
        assert!(enforce_exact_numeric_backend_supported(&module, "ny-llvmc-exe").is_ok());
        assert!(enforce_exact_numeric_backend_supported(&module, "llvmlite-obj").is_ok());
        assert!(enforce_exact_numeric_backend_supported(&module, "pyvm-harness").is_ok());
    }

    #[test]
    fn backend_capability_rejects_exact_operation_route_facts_for_unsupported_backend() {
        let module = module_with_route_fact();
        let err = enforce_exact_numeric_backend_supported(&module, "ny-llvmc").unwrap_err();
        assert!(err.contains(EXACT_NUMERIC_BACKEND_ROUTE_UNSUPPORTED_TAG));
        assert!(err.contains("route_facts=1"));
    }
}

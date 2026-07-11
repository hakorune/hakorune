use crate::mir::MirModule;

pub(crate) const BACKEND_UNSUPPORTED_TAG: &str = "[type/weak_field_contract_backend_unsupported]";

pub(crate) fn enforce_weak_field_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module.metadata.weak_field_contract_specs.len();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require={}",
        BACKEND_UNSUPPORTED_TAG,
        backend,
        rows,
        crate::mir::type_contracts::weak_field::WEAK_FIELD_CAPABILITY
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlock, BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction,
        MirType, UserBoxFieldDecl, ValueId,
    };

    const PRODUCT_BACKENDS: &[&str] = &[
        "ny-llvmc-exe",
        "ny-llvmc-obj",
        "llvmlite-obj",
        "llvm-legacy-obj",
        "pyvm-harness",
        "wasm",
        "wasm-v2",
    ];

    fn module_with_weak_declaration() -> MirModule {
        let mut module = MirModule::new("weak-backend".to_string());
        module
            .metadata
            .user_box_decls
            .insert("Node".to_string(), vec!["parent".to_string()]);
        module.metadata.user_box_field_decls.insert(
            "Node".to_string(),
            vec![UserBoxFieldDecl {
                name: "parent".to_string(),
                declared_type_name: Some("Node".to_string()),
                is_weak: true,
            }],
        );
        module
    }

    fn module_with_dynamic_alias_write() -> MirModule {
        let mut module = module_with_weak_declaration();
        let entry = BasicBlockId::new(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.dynamic/2".to_string(),
                params: vec![MirType::Unknown, MirType::Unknown],
                return_type: MirType::Void,
                effects: EffectMask::WRITE,
            },
            entry,
        );
        function.params = vec![ValueId::new(0), ValueId::new(1)];
        function.next_value_id = 2;
        let mut block = BasicBlock::new(entry);
        block.instructions.extend([
            MirInstruction::FieldSet {
                base: ValueId::new(0),
                field: "parent".to_string(),
                value: ValueId::new(1),
                declared_type: None,
            },
            MirInstruction::Return { value: None },
        ]);
        function.add_block(block);
        module.add_function(function);
        module
    }

    #[test]
    fn only_reference_vm_supports_weak_field_guard() {
        let module = module_with_weak_declaration();
        assert!(
            crate::mir::backend_capability::enforce_mir_backend_supported(
                &module,
                "mir-interpreter"
            )
            .is_ok()
        );
        let error =
            crate::mir::backend_capability::enforce_mir_backend_supported(&module, "ny-llvmc-exe")
                .unwrap_err();
        assert!(error.contains(BACKEND_UNSUPPORTED_TAG), "{error}");
    }

    #[test]
    fn declaration_obligation_rejects_every_product_backend_without_write_sites() {
        let module = module_with_weak_declaration();
        for backend in PRODUCT_BACKENDS {
            let error =
                crate::mir::backend_capability::enforce_mir_backend_supported(&module, backend)
                    .unwrap_err();
            assert!(
                error.contains(BACKEND_UNSUPPORTED_TAG),
                "{backend}: {error}"
            );
            assert!(error.contains(&format!("backend={backend}")), "{error}");
            assert!(error.contains("contract_rows=1"), "{error}");
        }
    }

    #[test]
    fn dynamic_alias_only_obligation_still_rejects_every_product_backend() {
        let module = module_with_dynamic_alias_write();
        for backend in PRODUCT_BACKENDS {
            let error =
                crate::mir::backend_capability::enforce_mir_backend_supported(&module, backend)
                    .unwrap_err();
            assert!(
                error.contains(BACKEND_UNSUPPORTED_TAG),
                "{backend}: {error}"
            );
        }
    }

    #[test]
    fn plain_module_remains_outside_the_weak_field_capability_gate() {
        let module = MirModule::new("plain".to_string());
        for backend in PRODUCT_BACKENDS {
            assert!(
                crate::mir::backend_capability::enforce_mir_backend_supported(&module, backend,)
                    .is_ok(),
                "plain module unexpectedly rejected for {backend}"
            );
        }
    }
}

use crate::mir::MirModule;

pub(crate) const RECORD_VALUE_BACKEND_CAPABILITY_MISSING_TAG: &str =
    "[type/record_contract_backend_unsupported]";

pub(crate) fn enforce_record_value_backend_supported(
    module: &MirModule,
    backend: &str,
) -> Result<(), String> {
    let rows = module
        .functions
        .values()
        .map(|function| function.metadata.record_value_contracts.len())
        .sum::<usize>();
    if rows == 0 || backend == "mir-interpreter" {
        return Ok(());
    }
    Err(format!(
        "{} backend={} contract_rows={} require=record_value_contracts",
        RECORD_VALUE_BACKEND_CAPABILITY_MISSING_TAG, backend, rows
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{RecordDecl, RecordValueBoundaryKind, UserBoxFieldDecl};
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirType, ValueId,
    };

    fn module_with_contract() -> MirModule {
        let decl = RecordDecl {
            name: "Point".to_string(),
            type_parameters: Vec::new(),
            fields: vec![UserBoxFieldDecl {
                name: "x".to_string(),
                declared_type_name: Some("i64".to_string()),
                is_weak: false,
            }],
            default_field_names: Vec::new(),
        };
        let fingerprint =
            crate::mir::type_contracts::record_value::record_schema_fingerprint(&decl);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "Main.main/0".to_string(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = function.get_block_mut(function.entry_block).unwrap();
        block.add_instruction(MirInstruction::RecordFieldContractCheck {
            contract_id: "record-value:1".to_string(),
            schema_fingerprint: fingerprint.clone(),
            field_index: 0,
            value: ValueId::new(0),
        });
        block.add_instruction(MirInstruction::RecordValuePublish {
            dst: ValueId::new(1),
            contract_id: "record-value:1".to_string(),
            boundary: RecordValueBoundaryKind::Construct,
            diagnostic_record_name: "Point".to_string(),
            schema_fingerprint: fingerprint,
            base: None,
            fields: vec![ValueId::new(0)],
        });
        let mut module = MirModule::new("record-backend".to_string());
        module
            .metadata
            .record_decls
            .insert("Point".to_string(), decl);
        module.add_function(function);
        module
    }

    #[test]
    fn only_reference_interpreter_supports_first_slice() {
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
            assert!(error.contains(RECORD_VALUE_BACKEND_CAPABILITY_MISSING_TAG));
        }
    }
}

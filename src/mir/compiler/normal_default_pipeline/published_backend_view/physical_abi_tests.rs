use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
fn numeric_capability_rejects_drift_in_referenced_physical_layout() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../../../apps/typed-object-birth-min/main.hako"),
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source")
        };
        MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                for index in 0..6 {
                    let mut bad = input.clone();
                    let object = &mut bad.layouts[0];
                    match index {
                        0 => object.runtime_type_id += 1,
                        1 => object.field_count += 1,
                        2 => object.fields[0].object_id += 1,
                        3 => object.fields[0].declaration_ordinal = 999,
                        4 => object.fields[0].runtime_slot += 1,
                        5 => object.fields[0].storage_kind += 1,
                        _ => unreachable!(),
                    }
                    assert!(crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &bad)
                        .unwrap_err().contains("lifecycle-numeric/"), "layout mutation {index}");
                }
                Ok(())
            }).unwrap();
    });
}

/// Range transport is tied to the original canonical Invoke coordinate. This
/// exercises the ABI issuer and the same source-owned Birth numeric consumer.
#[test]
fn invoke_usize_range_contract_keeps_canonical_row_identity() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            "box Counter { capacity: usize birth(n) { me.capacity = n } } static box Main { main() { local c = new Counter(7) return 0 } }",
            crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source") };
        MirCompiler::with_options(false).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)?;
                let mut contracts = 0;
                for (function_index, physical) in input.program().functions().iter().enumerate() {
                    let function = &view.module().functions[physical.name()];
                    for contract in &function.metadata.exact_numeric_runtime_check_contracts {
                        if contract.declared_type_name != "usize" { continue; }
                        let row = function.blocks[&contract.block].all_instructions().nth(contract.instruction_index).unwrap();
                        assert!(matches!(row, crate::mir::MirInstruction::Invoke {
                            operation: crate::mir::instruction::InvokeOperation::FieldSet { value, .. }, ..
                        } if *value == contract.value));
                        assert_eq!(input.exact_numeric_check_at(function_index as u32, contract.block.0,
                            contract.instruction_index as u32), Some("usize"));
                        contracts += 1;
                    }
                }
                assert_eq!(contracts, 1);
                let checks = super::row_obligations::issue_exact_numeric_checks(view.module(), input.program())?;
                assert_eq!(checks.len(), 1);
                for mutation in 0..3 {
                    let mut changed = view.module().clone();
                    let contract = changed.functions.values_mut().flat_map(|f|
                        f.metadata.exact_numeric_runtime_check_contracts.iter_mut()).next().unwrap();
                    match mutation {
                        0 => contract.value = crate::mir::ValueId(999),
                        1 => contract.field.push_str("_foreign"),
                        _ => contract.instruction_index += 1,
                    }
                    assert!(super::row_obligations::issue_exact_numeric_checks(&changed, input.program()).is_err());
                }
                Ok(())
            }).unwrap();
    });
}

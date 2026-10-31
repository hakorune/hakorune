use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
fn published_move_owner_verifies_actual_mir_and_rejects_a_changed_end() {
    use crate::mir::ownership_ssa::{verify_ownership_ssa_v1, FunctionResultOwnershipV1, OwnershipFunctionAbiV1, OwnershipFunctionOwnerV1};
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = include_str!("../../../../../apps/typed-object-method-min/main.hako")
            .replace("return pair.sum()", "local moved = move pair\n    return moved.sum()");
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            &text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source") };
        MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                let source = input.program().handoff().root_source().ok_or("missing root")?;
                let owner = source.owner();
                assert_eq!(source.move_ownership_owners_v1().into_iter().collect::<Vec<_>>(), vec![owner]);
                let root = input.program().functions().iter().find(|f| matches!(f.role(), crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::Root { .. })).ok_or("missing root function")?;
                let (aliases, initially_published) = source.move_ownership_aliases_v1(owner, root.function())?.ok_or("missing move")?;
                let abi = OwnershipFunctionAbiV1::new(OwnershipFunctionOwnerV1::source(owner), vec![], FunctionResultOwnershipV1::None)
                    .with_source_aliases(aliases).with_initially_published(initially_published);
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)?;
                assert!(verify_ownership_ssa_v1(root.function(), &abi).is_ok());
                let mut changed = root.function().clone();
                let end = changed.blocks.values_mut().find_map(|block| match block.terminator.as_mut() {
                    Some(crate::mir::MirInstruction::Invoke { operation: crate::mir::instruction::InvokeOperation::HomeRelease { value, .. }, .. }) => Some(value),
                    _ => None,
                }).ok_or("missing HomeRelease")?;
                *end = crate::mir::ValueId::new(u32::MAX);
                assert!(verify_ownership_ssa_v1(&changed, &abi).is_err());
                Ok(())
            }).unwrap();
    });
}

#[test]
fn published_move_keeps_original_reverse_home_cleanup_order() {
    use crate::mir::{MirInstruction, instruction::InvokeOperation};
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Page {} static box Main { main() { local a = new Page() local z = new Page() local b = move a return 30 } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source") };
        MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)?;
                let source = input.program().handoff().root_source().ok_or("missing root")?;
                let root = input.program().functions().iter().find(|f| matches!(f.role(), crate::mir::compiler::published_backend_view::PublishedLifecyclePhysicalFunctionRoleV1::Root { .. })).ok_or("missing root function")?;
                let function = root.function();
                let (aliases, _) = source.move_ownership_aliases_v1(source.owner(), function)?.ok_or("missing move")?;
                let mut block = function.entry_block;
                let mut acquired = Vec::new();
                let mut released = Vec::new();
                for _ in 0..=function.blocks.len() {
                    let row = function.blocks.get(&block).ok_or("missing block")?;
                    for instruction in &row.instructions {
                        if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                            if matches!(function.blocks[invoke_block].terminator, Some(MirInstruction::Invoke { operation: InvokeOperation::NewBox { .. }, .. })) {
                                acquired.push(*dst);
                            }
                        }
                    }
                    match row.terminator.as_ref().ok_or("missing terminator")? {
                        MirInstruction::Invoke { operation, normal_landing, .. } => {
                            if let InvokeOperation::HomeRelease { value, .. } = operation {
                                released.push(*aliases.get(value).unwrap_or(value));
                            }
                            block = *normal_landing;
                        }
                        MirInstruction::Jump { target, .. } => block = *target,
                        MirInstruction::Return { .. } => break,
                        _ => return Err("unexpected path".into()),
                    }
                }
                assert_eq!(acquired.len(), 2);
                acquired.reverse();
                assert_eq!(released, acquired);
                Ok(())
            }).unwrap();
    });
}

#[test]
fn published_move_accepts_source_proven_i64_parameter_and_unused_receiver() {
    use crate::mir::ownership_ssa::MirOwnershipKindV1;
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Transport { flag: i64 birth() { me.flag = 0 } probe(p: i64): i64 { local scratch = new Transport() local moved = move scratch return 7 } wrap(q: i64): i64 { return q } } static box Main { main() { local recv = new Transport() local result = recv.probe(7) return 7 } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text, crate::parser::ParserBuildConfig::default()).unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap() else { panic!("source") };
        MirCompiler::with_options(true).compile_normal_with_published(
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default()),
            |view, _| -> Result<(), String> {
                let input = view.issue_lifecycle_physical_abi_input()?;
                let handoff = input.program().handoff();
                let source = handoff.root_source().ok_or("missing root")?;
                let owners = source.move_ownership_owners_v1();
                assert_eq!(owners.len(), 1);
                let owner = *owners.iter().next().unwrap();
                let physical = input.program().functions().iter().find(|f| f.role().ordinary_target().is_some_and(|key|
                    handoff.callables().and_then(|c| c.owner_for_canonical_key(key)) == Some(owner))).ok_or("missing callable")?;
                let kinds = source.move_parameter_kinds_v1(owner, physical.role().ordinary_target().unwrap(),
                    physical.role().receiver_object(), physical.params().len())?;
                assert_eq!(kinds, vec![MirOwnershipKindV1::Borrowed, MirOwnershipKindV1::None]);
                crate::mir::backend_capability::enforce_published_lifecycle_backend_supported(view, &input)
            }).unwrap();
    });
}

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

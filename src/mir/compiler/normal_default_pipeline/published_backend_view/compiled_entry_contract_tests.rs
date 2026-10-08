use super::*;
use crate::mir::{MirCompiler, NormalCompileRequestV1};

#[test]
fn per_new_actuals_survive_definition_dedup_and_are_consumed_once() {
    original_object_root_result_refuses_process_entry_category();
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        // Existing two-destination cohort; Local is retained, never promoted to i64.
        let text = "box Page { left: i64\nright: i64\nbirth(a, b) { me.left = a\nme.right = b } } static box Main { main() { local a = 7\nlocal b = 9\nlocal first = new Page(a, b)\nlocal second = new Page(b, a)\nreturn 0 } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text,
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source identity lost")
        };
        let request =
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
        MirCompiler::with_options(false).compile_normal_with_published(request, |view, verification| {
            assert!(verification.is_ok(), "{verification:?}");
            let contract = view.issue_lifecycle_compiled_entry_contract()?;
            assert_eq!(contract.births().len(), 1);
            assert_eq!(contract.birth_calls().len(), 2);
            let actuals = view.retained_birth_actuals().unwrap();
            assert_eq!(actuals.len(), 2);
            assert_ne!(actuals[0].site(), actuals[1].site());
            assert_ne!(actuals[0].receiver(), actuals[1].receiver());
            for call in contract.birth_calls() {
                assert_eq!(call.function_index(), 1);
                assert!(call.actual().arguments().iter().all(|argument| matches!(argument.source().kind(),
                    crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Local { .. })));
            }
            let [root, births @ ..] = contract.program().functions() else { panic!("root missing") };
            let PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = births[0].role()
                else { panic!("Birth ABI missing") };
            let formals = contract.births()[0].formals();
            assert!(formals[0].contract().is_none());
            assert_eq!(formals.len(), abi.formal_contracts().len() + 1);
            for (formal, source) in formals[1..].iter().zip(abi.formal_contracts()) {
                assert_eq!(formal.contract(), Some(source),
                    "declaration, binding, ordinal and exact use sites must survive together");
                assert_eq!(formal.source_ordinal(), Some(source.ordinal()));
                assert_eq!(formal.disposition(), Some(source.disposition()));
            }
            let result = view.retained_root_result().unwrap().unwrap();
            let mut reordered = actuals.to_vec();
            reordered.reverse();
            assert_eq!(issue_birth_calls(root, births, &reordered, result)?, contract.birth_calls());
            assert!(issue_birth_calls(root, births, &actuals[..1], result).unwrap_err().contains("actual-mismatch"));
            let duplicated = vec![actuals[0].clone(), actuals[0].clone()];
            assert!(issue_birth_calls(root, births, &duplicated, result).unwrap_err().contains("actual-membership"));
            // 549a54c811 admits Local scalar transport only with exact caller
            // Integer corroboration; the original source kind stays Local.
            let input = view.issue_lifecycle_physical_abi_input()?;
            for call in input.entry().birth_calls() {
                let caller = &input.program().functions()[call.caller_function_index() as usize];
                for argument in call.actual().arguments() {
                    assert!(!input.tagged_birth_actual(call.actual().site(), argument.source().ordinal()));
                    assert_eq!(super::super::physical_abi::scalar_actual_kind(
                        argument.source().kind(), argument.value(), caller.value_types())?, 1);
                    assert!(super::super::physical_abi::scalar_actual_kind(
                        argument.source().kind(), argument.value(), &BTreeMap::new())
                        .unwrap_err().contains("actual-kind-unavailable"));
                }
            }
            Ok::<(), String>(())
        }).unwrap();
    });
}

#[test]
fn child_birth_actual_is_retained_and_attributed_to_child_function() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let text = "box Page { birth() { } } static box Main { main() { return helper(7) } helper(value: i64): i64 { local page = new Page() local m = %{\"v\" => page} return 30 } }";
        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
            text,
            crate::parser::ParserBuildConfig::default(),
        )
        .unwrap();
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
        else {
            panic!("source identity lost")
        };
        let request =
            NormalCompileRequestV1::for_mir_mode_callable_source(source, None, Default::default());
        MirCompiler::with_options(false)
            .compile_normal_with_published(request, |view, verification| {
                assert!(verification.is_ok(), "{verification:?}");
                let contract = view.issue_lifecycle_compiled_entry_contract()?;
                assert_eq!(contract.ordinary_calls().len(), 1);
                for call in contract.ordinary_calls() {
                    let caller = &contract.program().functions()[call.caller_function_index() as usize];
                    let block = caller.blocks().iter().find(|block| block.id() == call.caller_block_id()).unwrap();
                    let instruction = block.instructions().iter().copied()
                        .chain(std::iter::once(block.terminator()))
                        .find(|row| row.index() == call.caller_instruction_index()).unwrap();
                    assert!(matches!(instruction.instruction(), MirInstruction::Invoke {
                        operation: InvokeOperation::Call { call: actual, result }, ..
                    } if actual == call.call() && *result == call.result()));
                }
                assert_eq!(contract.births().len(), 1);
                assert_eq!(contract.birth_calls().len(), 1);
                assert_eq!(contract.birth_calls()[0].caller_function_index(), 1);
                assert_eq!(contract.birth_calls()[0].function_index(), 2);
                assert_eq!(view.retained_birth_actuals().unwrap().len(), 1);
                let input = view.issue_lifecycle_physical_abi_input()?;
                let json = crate::mir::compiler::normal_default_pipeline::published_backend_view::physical_program_json::emit_lifecycle_physical_abi_json(&input)?;
                assert!(json.contains("\"role\":\"ordinary_i64\""));
                assert!(json.contains("\"kind\":\"birth_call\""));
                Ok::<(), String>(())
            })
            .unwrap();
    });
}

#[test]
fn map_cleanup_coordinates_follow_physical_block_contraction() {
    // Coordinate projection only; this graph does not claim lifecycle admission.
    use crate::mir::instruction::MapInvokeOperation as Map;
    use crate::mir::{
        BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirModule, MirType,
    };
    for optimize in [false, true] {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "coordinates".into(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::CONTROL,
            },
            BasicBlockId(0),
        );
        let mut entry = BasicBlock::new(BasicBlockId(0));
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(30),
        });
        entry.set_terminator(MirInstruction::Jump {
            target: BasicBlockId(1),
            edge_args: None,
        });
        function.blocks.insert(entry.id, entry);
        for (id, operation, next) in [
            (1, Map::End { map: ValueId(2) }, 2),
            (
                2,
                Map::EndOutcome {
                    outcome: ValueId(3),
                },
                3,
            ),
        ] {
            let mut block = BasicBlock::new(BasicBlockId(id));
            block.set_terminator(MirInstruction::Invoke {
                operation: InvokeOperation::Map(operation),
                fault_frame: ValueId(0),
                normal_landing: BasicBlockId(next),
                fault_landing: BasicBlockId(next),
            });
            function.blocks.insert(block.id, block);
        }
        let mut tail = BasicBlock::new(BasicBlockId(3));
        tail.set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)),
        });
        function.blocks.insert(tail.id, tail);
        let mut module = MirModule::new("coordinates".into());
        module.functions.insert("coordinates".into(), function);
        if optimize {
            assert!(crate::mir::passes::simplify_cfg::simplify(&mut module) > 0);
        }
        let function = &module.functions["coordinates"];
        let physical = super::super::physical_program::issue_function(
            function,
            PublishedLifecyclePhysicalFunctionRoleV1::Root {
                result: CompiledEntryRootResultV1::I64,
            },
            false,
            &[],
        )
        .unwrap();
        let rows = issue_cleanup_coordinates(&[physical]).unwrap();
        assert_eq!(rows.len(), 2);
        for row in rows {
            let block = &function.blocks[&BasicBlockId(row.block_id)];
            let instruction = block
                .all_instructions()
                .nth(row.instruction_index as usize)
                .unwrap();
            assert!(matches!(
                (row.kind, instruction),
                (
                    CompiledEntryCleanupKindV1::MapEnd,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::Map(Map::End { .. }),
                        ..
                    }
                ) | (
                    CompiledEntryCleanupKindV1::MapEndOutcome,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::Map(Map::EndOutcome { .. }),
                        ..
                    }
                )
            ));
        }
        assert_eq!(function.blocks.contains_key(&BasicBlockId(1)), !optimize);
    }
}

#[test]
fn provider_publication_selects_original_birth_caller_closure() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        for optimize in [false, true] {
            for reverse in [false, true] {
                for construct_parent in [false, true] {
                    for shared_target in [false, true] {
                        let parent = format!(
                            "box Parent {{ first: First second: {} birth() {{
                        me.first = new First() me.second = new {}() }} }}",
                            if shared_target { "First" } else { "Second" },
                            if shared_target { "First" } else { "Second" }
                        );
                        let mut boxes = vec![
                            "box First { flag: i64 birth() { me.flag = 0 } }",
                            "box Second { flag: i64 birth() { me.flag = 1 } }",
                            parent.as_str(),
                            "box Transport { flag: i64 birth() { me.flag = 0 } }",
                        ];
                        if reverse {
                            boxes.reverse();
                        }
                        let main = if construct_parent {
                            "static box Main { main() { local p = new Parent() return 5 } }"
                        } else {
                            "static box Main { main() { local p = new Transport() return 5 } }"
                        };
                        let text = format!("{} {main}", boxes.join("\n"));
                        let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
                        &text, crate::parser::ParserBuildConfig::default(),
                    ).unwrap();
                        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                            crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
                        else {
                            panic!("source identity lost")
                        };
                        let request = NormalCompileRequestV1::for_mir_mode_callable_source(
                            source,
                            None,
                            Default::default(),
                        );
                        MirCompiler::with_options(optimize)
                            .compile_normal_with_published(request, |view, verification| {
                                assert!(verification.is_ok(), "{verification:?}");
                                let contract = view.issue_lifecycle_compiled_entry_contract()?;
                                let program = contract.program();
                                let actuals = view.retained_birth_actuals().unwrap();
                                let expected = if construct_parent { 3 } else { 1 };
                                assert_eq!(actuals.len(), expected);
                                let definitions = if construct_parent && shared_target {
                                    2
                                } else {
                                    expected
                                };
                                assert_eq!(contract.births().len(), definitions);
                                assert_eq!(contract.birth_calls().len(), expected);
                                let births: Vec<_> =
                                    program
                                        .functions()
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, row)| {
                                            matches!(row.role(),
                                    PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { .. })
                                        })
                                        .map(|(index, row)| (index as u32, row))
                                        .collect();
                                let names: Vec<_> =
                                    births.iter().map(|(_, row)| row.name()).collect();
                                if construct_parent {
                                    for name in ["Parent.birth/0", "First.birth/0"] {
                                        assert!(names.contains(&name), "{names:?}");
                                    }
                                    assert_eq!(names.contains(&"Second.birth/0"), !shared_target);
                                    assert!(!names.contains(&"Transport.birth/0"));
                                } else {
                                    assert_eq!(names, ["Transport.birth/0"]);
                                }
                                let mut reordered = actuals.to_vec();
                                reordered.reverse();
                                assert_eq!(
                                    birth_calls::issue_birth_calls_for_program(
                                        program, &births, &reordered
                                    )?,
                                    contract.birth_calls()
                                );
                                assert!(birth_calls::issue_birth_calls_for_program(
                                    program,
                                    &births,
                                    &actuals[..actuals.len() - 1]
                                )
                                .unwrap_err()
                                .contains("compiled-entry-call-actual-mismatch"));
                                let mut duplicated = actuals.to_vec();
                                let selected = actuals
                                    .iter()
                                    .find(|row| row.destination().is_none())
                                    .unwrap_or(&actuals[0]);
                                duplicated.push(selected.clone());
                                assert!(birth_calls::issue_birth_calls_for_program(
                                    program,
                                    &births,
                                    &duplicated
                                )
                                .unwrap_err()
                                .contains("compiled-entry-actual-membership"));
                                if construct_parent {
                                    let provider = actuals
                                        .iter()
                                        .find(|row| row.destination().is_none())
                                        .unwrap();
                                    let foreign_owner = births
                                        .iter()
                                        .find_map(|(_, row)| {
                                            match row.role() {
                                    PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi }
                                        if row.name() == "First.birth/0" => Some(abi.owner()),
                                    _ => None,
                                }
                                        })
                                        .unwrap();
                                    let corrupted = provider
                                        .with_foreign_provider_owner_for_test(foreign_owner);
                                    let mut foreign = actuals.to_vec();
                                    let index = foreign
                                        .iter()
                                        .position(|row| row.site() == provider.site())
                                        .unwrap();
                                    assert!(!foreign
                                        .iter()
                                        .any(|row| row.site() == corrupted.site()));
                                    foreign[index] = corrupted;
                                    assert!(birth_calls::issue_birth_calls_for_program(
                                        program, &births, &foreign
                                    )
                                    .unwrap_err()
                                    .contains("compiled-entry-call-actual-mismatch"));
                                }
                                // Full final ABI still has to consume every selected edge.
                                view.issue_lifecycle_physical_abi_input()?;
                                Ok::<(), String>(())
                            })
                            .unwrap();
                    }
                }
            }
        }
    });
}


fn original_object_root_result_refuses_process_entry_category() {
    let text = "box Token {} static box Main { main() { return new Token() } }";
    let (completed, original) = crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, ledger, keys, cohort) = completed.document_preflight_parts_for_test(true).unwrap();
    assert!(std::rc::Rc::ptr_eq(&original, &ledger));
    let handoff = ledger.seal_finalized_root_birth_handoff(key, &module, &keys, cohort).unwrap();
    let result = handoff.root_result(&module).unwrap().unwrap();
    assert!(matches!(result, crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::ObjectReturn {
        kind: crate::mir::instruction::InvokeCallResultKind::Handle, ..
    }));
    assert!(root_result_category(result).unwrap_err()
        .contains("compiled-entry-object-result-unsupported"));
}

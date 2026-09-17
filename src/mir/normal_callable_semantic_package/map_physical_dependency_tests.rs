//! Local/finishing dependency checks; full source artifact proof lives in host tests.
use super::brand_catalog_tests::issue_with_brand_catalog as issue;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::instruction::{InvokeOperation, MapInvokeOperation};
use crate::mir::{MirBuilder, MirInstruction};

#[test]
fn map_callable_dependency_preserves_opaque_local_and_alias_identity() {
    for definition in ["box Page {}", "box Page { birth() {} }"] {
        for (body, optimize) in [
        "local m = %{} return 30",
        "local m = %{\"a\" => 30, \"b\" => true, \"c\" => false} return 30",
        "local value = true local alias = value local m = %{\"a\" => alias, \"b\" => alias} return 30",
        "local a = new Page() local m = %{\"a\" => a, \"a\" => 30} return 30",
        "local a = new Page() local m = %{\"a\" => false, \"a\" => a} return 30",
        "local m = %{} local alias = m local again = alias return 30",
        "local a = new Page() local m = %{\"a\" => a} return 30",
        "local m = %{} local a = new Page() return 30",
        "local m = %{} local n = %{} return 30",
        "local a = new Page() local b = new Page() local m = %{\"a\" => a, \"a\" => b} return 30",
    ]
    .into_iter()
    .flat_map(|body| [(body, false), (body, true)])
    {
        let source = format!("{definition} static box Main {{ main() {{ {body} }} }}");
        let package = issue(&source).unwrap();
        let main = package
            .declaration_catalog()
            .source_backed_app_main()
            .unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|row| row.identity().same_as(main.parser_identity()))
            .unwrap();
        let mut builder = MirBuilder::new();
        let function = package
            .batch()
            .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
                builder.lower_map_dependency_for_test(
                    input,
                    SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                    main.parser_identity(),
                    identity.method_source_observation().cloned(),
                    std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                    None,
                )
            })
            .unwrap()
            .unwrap_or_else(|e| panic!("{body}: {e}"));
        let mut maps = Vec::new();
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if function.blocks[invoke_block].all_instructions().any(|i| {
                        matches!(
                            i,
                            MirInstruction::Invoke {
                                operation: InvokeOperation::Map(MapInvokeOperation::New),
                                ..
                            }
                        )
                    }) {
                        maps.push(*dst);
                    }
                }
            }
        }
        assert_eq!(
            maps.len(),
            if body.contains("local n") { 2 } else { 1 },
            "{body}"
        );
        assert!(package.ordinary_new_claim_ledger.map_demands_consumed());
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(&function)
            .unwrap_or_else(|e| panic!("{body}: {e:?}"));
        let observation = package
            .ordinary_new_claim_ledger
            .validate_finalized_new_root(&function)
            .unwrap_or_else(|e| panic!("{body}: root validation: {e}"));
        for mutation in 0..2 {
            let mut drifted_value = function.clone();
            let mut changed = false;
            for block in drifted_value.blocks.values_mut() {
                if let Some(MirInstruction::Invoke {
                    operation: InvokeOperation::Map(MapInvokeOperation::InstallValue { kind, value, .. }), ..
                }) = &mut block.terminator {
                    if mutation == 0 {
                        *kind = match kind {
                            crate::mir::instruction::MapValueKind::I64 => crate::mir::instruction::MapValueKind::Bool,
                            crate::mir::instruction::MapValueKind::Bool => crate::mir::instruction::MapValueKind::I64,
                        };
                    } else { *value = maps[0]; }
                    changed = true;
                    break;
                }
            }
            if changed { assert!(package.ordinary_new_claim_ledger
                .validate_new_emissions(declaration.owner(), &drifted_value).is_err()); }
        }
        let mut drifted = function.clone();
        let mut changed = false;
        for block in drifted.blocks.values_mut() {
            for instruction in &mut block.instructions {
                if matches!(instruction, MirInstruction::InvokeNormalResult { dst, .. }
                    if *dst == maps[0])
                {
                    *instruction = MirInstruction::Copy {
                        dst: maps[0],
                        src: maps[0],
                    };
                    changed = true;
                }
            }
        }
        assert!(changed);
        assert!(package
            .ordinary_new_claim_ledger
            .validate_new_emissions(declaration.owner(), &drifted)
            .is_err());

        assert!(
            !function
                .blocks
                .values()
                .flat_map(|b| b.all_instructions())
                .any(|i| matches!(i, MirInstruction::Copy { src, .. } if maps.contains(src))),
            "{body}"
        );
        let mut module = crate::mir::MirModule::new("map-finishing".into());
        let mut function = function;
        function
            .install_root_ordinary_new_observation(observation)
            .unwrap();
        module.functions.insert("Main.main/0".into(), function);
        if optimize {
            assert!(crate::mir::passes::simplify_cfg::simplify(&mut module) > 0);
        }
        let finished = &module.functions["Main.main/0"];
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(finished)
            .unwrap_or_else(|e| panic!("{body}, optimize={optimize}: {e:?}"));
        for mutation in 0..3 {
            let mut drifted = finished.clone();
            let mut changed = false;
            for block in drifted.blocks.values_mut() {
                if let Some(MirInstruction::Invoke { operation: InvokeOperation::Map(op), .. }) = &mut block.terminator {
                    if let MapInvokeOperation::InstallValue { map, key, value, kind } = *op {
                        *op = match mutation {
                            0 => MapInvokeOperation::InstallValue { map, key, value,
                                kind: match kind {
                                    crate::mir::instruction::MapValueKind::I64 => crate::mir::instruction::MapValueKind::Bool,
                                    crate::mir::instruction::MapValueKind::Bool => crate::mir::instruction::MapValueKind::I64,
                                } },
                            1 => MapInvokeOperation::InstallValue { map, key, value: map, kind },
                            _ => MapInvokeOperation::InstallIndexed { map, key, value,
                                object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap() },
                        };
                        changed = true;
                        break;
                    }
                }
            }
            if changed { assert!(package.ordinary_new_claim_ledger
                .validate_artifact_after_compiler_finishing(&drifted).is_err()); }
        }
        if body.starts_with("local m = %{\"a\" => 30") {
            let scalar = finished.blocks.values().flat_map(|b| b.all_instructions())
                .find_map(|i| match i {
                    MirInstruction::Invoke { operation: InvokeOperation::Map(
                        MapInvokeOperation::InstallValue { value, .. }), .. } => Some(*value),
                    _ => None,
                }).unwrap();
            let mut drifted = finished.clone();
            let mut changed = false;
            for block in drifted.blocks.values_mut() {
                for i in &mut block.instructions {
                    if let MirInstruction::Const { dst, value } = i {
                        if *dst == scalar { *value = crate::mir::ConstValue::Integer(99); changed = true; }
                    }
                }
            }
            assert!(changed);
            assert!(package.ordinary_new_claim_ledger
                .validate_artifact_after_compiler_finishing(&drifted).is_err());
        }
        let mut drifted_finished = finished.clone();
        let mut changed_edge = false;
        for block in drifted_finished.blocks.values_mut() {
            if let Some(MirInstruction::Invoke {
                operation: InvokeOperation::Map(MapInvokeOperation::New),
                normal_landing, fault_landing, ..
            }) = &mut block.terminator {
                std::mem::swap(normal_landing, fault_landing);
                changed_edge = true;
                break;
            }
        }
        assert!(changed_edge);
        assert!(package.ordinary_new_claim_ledger
            .validate_artifact_after_compiler_finishing(&drifted_finished).is_err());
        package
            .ordinary_new_claim_ledger
            .validate_artifact_after_compiler_finishing(finished)
            .unwrap_or_else(|e| panic!("{body}, optimize={optimize}: {e}"));
    }
    }
}

#[test]
fn return_boundary_map_literal_transfers_its_lease_to_return() {
    // `return %{...}` lowers through the sealed ReturnBoundary flow row: the
    // Map::New normal result is consumed by `Return` — the single
    // lease-transfer point — with no binding and no normal-path `Map::End`.
    for body in [
        "return %{\"a\" => 1}",
        "local value = 7 return %{\"a\" => value}",
    ] {
        let source = format!("static box Main {{ main() {{ {body} }} }}");
        let package = issue(&source).unwrap();
        let main = package
            .declaration_catalog()
            .source_backed_app_main()
            .unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|row| row.identity().same_as(main.parser_identity()))
            .unwrap();
        let mut builder = MirBuilder::new();
        let function = package
            .batch()
            .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
                builder.lower_map_dependency_for_test(
                    input,
                    SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                    main.parser_identity(),
                    identity.method_source_observation().cloned(),
                    std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                    None,
                )
            })
            .unwrap()
            .unwrap_or_else(|e| panic!("{body}: {e}"));
        let mut maps = Vec::new();
        for block in function.blocks.values() {
            for instruction in block.all_instructions() {
                if let MirInstruction::InvokeNormalResult { invoke_block, dst } = instruction {
                    if function.blocks[invoke_block].all_instructions().any(|i| {
                        matches!(
                            i,
                            MirInstruction::Invoke {
                                operation: InvokeOperation::Map(MapInvokeOperation::New),
                                ..
                            }
                        )
                    }) {
                        maps.push(*dst);
                    }
                }
            }
        }
        let [map] = maps.as_slice() else {
            panic!("{body}: exactly one Map::New result expected");
        };
        assert!(
            matches!(
                builder.value_type(*map),
                Some(crate::mir::MirType::Box(name)) if name == "MapBox"
            ),
            "{body}: the returned value carries the MapBox type"
        );
        let returned = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .any(|i| matches!(i, MirInstruction::Return { value: Some(v) } if v == map));
        assert!(returned, "{body}: Return consumes the map lease");
        assert!(
            package.ordinary_new_claim_ledger.map_demands_consumed(),
            "{body}"
        );
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(&function)
            .unwrap_or_else(|e| panic!("{body}: {e:?}"));
    }
}

#[test]
fn return_installed_map_local_transfers_its_lease_to_return() {
    // `return m` reuses the installed local's value: the returned binding
    // left terminal cleanup (F3), so no normal-path `Map::End` precedes the
    // transfer and `Return{map}` consumes the live lease.
    let source = "static box Main { main() { local m = %{\"a\" => 1} return m } }";
    let package = issue(source).unwrap();
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .unwrap();
    let mut builder = MirBuilder::new();
    let function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                None,
            )
        })
        .unwrap()
        .expect("map-local return lowering");
    let map = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .find_map(|i| match i {
            MirInstruction::InvokeNormalResult { invoke_block, dst }
                if function.blocks[invoke_block].all_instructions().any(|i| {
                    matches!(
                        i,
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Map(MapInvokeOperation::New),
                            ..
                        }
                    )
                }) =>
            {
                Some(*dst)
            }
            _ => None,
        })
        .expect("Map::New result");
    let returns_map = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(i, MirInstruction::Return { value: Some(v) } if *v == map));
    assert!(returns_map, "Return consumes the installed map lease");
    // A `Map::End` may only exist inside the fault cleanup chain — the
    // normal path hands the live lease to `Return` directly.
    crate::mir::verification::MirVerifier::new_strict()
        .verify_function(&function)
        .expect("lease transfer verifies");
    assert!(package.ordinary_new_claim_ledger.map_demands_consumed());
}

#[test]
fn self_rooted_handle_borrow_emits_install_value_i64() {
    // `local m = %{"v" => h}` inside a parameterized non-main owner: the
    // sealed BorrowedHandle entry is an `OwnershipShare(Handle)` the
    // declared lane stores through `InstallValue{I64}` — the formal's
    // physical i64 value, with a no-op payload end (the map never owns
    // the handle). The pin asserts the stored value is the exact
    // parameter ValueId, not a re-materialized copy.
    for annotation in ["", ": StringBox"] {
        let source = format!(
            "static box Work {{ stash(h{annotation}) {{
                local m = %{{\"v\" => h}} return 0
            }} }}
             static box Main {{ main() {{ return 30 }} }}"
        );
        let package = issue(&source).unwrap();
        let main = package
            .declaration_catalog()
            .source_backed_app_main()
            .unwrap();
        let declaration = package
            .batch()
            .declarations()
            .find(|row| !row.identity().same_as(main.parser_identity()))
            .unwrap();
        let (key, _, _) = package
            .catalog
            .selected_identities()
            .find(|(_, identity, _)| declaration.identity().same_as(identity))
            .expect("selected identity for the Work.stash declaration");
        let mut builder = MirBuilder::new();
        let function = package
            .batch()
            .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
                builder.lower_map_dependency_for_test(
                    input,
                    key.clone(),
                    declaration.identity(),
                    identity.method_source_observation().cloned(),
                    std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                    None,
                )
            })
            .unwrap()
            .unwrap_or_else(|e| panic!("annotation={annotation}: {e}"));
        let install = function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .find_map(|i| match i {
                MirInstruction::Invoke {
                    operation:
                        InvokeOperation::Map(MapInvokeOperation::InstallValue { value, kind, .. }),
                    ..
                } => Some((*value, *kind)),
                _ => None,
            })
            .unwrap_or_else(|| panic!("annotation={annotation}: InstallValue expected"));
        assert_eq!(
            install.1,
            crate::mir::instruction::MapValueKind::I64,
            "annotation={annotation}: the borrowed handle stores as i64"
        );
        assert!(
            function.params.contains(&install.0),
            "annotation={annotation}: the stored value is the formal's own ValueId"
        );
        crate::mir::verification::MirVerifier::new_strict()
            .verify_function(&function)
            .unwrap_or_else(|e| panic!("annotation={annotation}: {e:?}"));
    }
}

#[test]
fn map_literal_in_call_argument_position_stays_rejected() {
    // No map-argument lane is admitted in this slice: a `%{...}` call
    // argument reaches the builder boundary and is refused by its own named
    // destination class, never silently placed.
    let source = "static box Main {
        main() { local a = new ArrayBox() a.push(%{\"a\" => 1}) return 30 }
    }";
    let package = issue(source).unwrap();
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .unwrap();
    let mut builder = MirBuilder::new();
    let error = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                None,
            )
        })
        .unwrap()
        .expect_err("call-argument map is not an admitted destination");
    assert!(
        error.contains("map-destination-unsupported") || error.contains("map-source-unavailable"),
        "{error}"
    );
}

#[test]
fn map_physical_preflight_rejects_foreign_site_and_pending_install() {
    let source = "static box Main { main() { local m = %{} local other = 1 return 30 } }";
    let package = issue(source).unwrap();
    let foreign = issue(source).unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let map = ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .complete()
        .unwrap();
    let foreign_completion = foreign.ordinary_new_claim_ledger.root_completion_for_test();
    let foreign_site = foreign_completion.cleanup().root_flow().unwrap().maps()[0].site();
    let declaration = package
        .batch()
        .declarations()
        .find(|d| d.owner() == map.site().owner())
        .unwrap();
    package
        .batch()
        .with_lowering_input(declaration.batch_slot(), |input| {
            let relation = input
                .function()
                .expression_source()
                .initializers()
                .find(|r| r.initializer_site() == Some(map.site().site()))
                .unwrap();
            let other = input
                .function()
                .expression_source()
                .initializers()
                .find(|r| r.binding() != relation.binding())
                .unwrap();
            assert!(ledger.begin_map_emission(foreign_site, relation).is_err());
            assert!(ledger
                .begin_map_emission(map.site(), other)
                .unwrap_err()
                .contains("map-initializer-source-drift"));
            assert!(!ledger.map_demands_consumed());
            ledger.begin_map_emission(map.site(), relation).unwrap();
            assert!(ledger
                .begin_map_emission(map.site(), relation)
                .unwrap_err()
                .contains("map-duplicate-emission"));
            let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
                relation.declaration_site()
            else {
                panic!("source local");
            };
            let value = crate::mir::ValueId(99);
            assert!(!ledger.map_initializer_matches(map.site(), relation.binding(), value));
            assert!(ledger
                .complete_local_installation(
                    input.owner(),
                    statement.node(),
                    &[(relation.binding(), *ordinal, value, value)]
                )
                .unwrap_err()
                .contains("local-initializer-mismatch"));
            assert!(!ledger.is_installed_map_binding(relation.binding(), value));
            assert!(!ledger.map_demands_consumed());
        })
        .unwrap();
}

//! Source-to-selected-consumer dependency; ordinary package install stays stopped.
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation, MapInvokeOperation};
use crate::mir::{MirBuilder, MirInstruction, MirModule, ValueId};

#[test]
fn terminal_call_probe_is_scoped_to_its_source_owner() {
    let package = issue(
        "box Page {} static box Main {
            main() { return helper(30, 5) }
            helper(value: i64, other: i64): i64 {
                local m = %{\"v\" => value}
                return 30
            }
        }",
    )
    .unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let root_owner = ledger.root_completion_for_test().owner();
    let child_owner = package
        .batch()
        .declarations()
        .find(|row| row.owner() != root_owner)
        .expect("ordinary child completion")
        .owner();

    assert!(ledger.terminal_call_arguments().is_some());
    assert!(ledger
        .terminal_call_arguments_for_owner(root_owner)
        .is_some());
    assert!(ledger
        .terminal_call_arguments_for_owner(child_owner)
        .is_none());
}

#[test]
fn local_map_call_then_terminal_map_call_reaches_physical_lowering() {
    let mut package = issue(
        r#"static box Main {
            main() { local first = helper(10) return helper(20) }
            helper(value: i64): i64 { local m = %{"first" => value} return 30 }
        }"#,
    )
    .expect("bounded local plus terminal Map package");
    let mut loans = package.direct_call_loans.take().unwrap();
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
                Some(&mut loans),
            )
        })
        .unwrap()
        .expect("physical Main lowering");
    loans.finish_empty().expect("both Call rows consumed");
    crate::mir::verification::MirVerifier::new_strict()
        .verify_function(&function)
        .expect("local and terminal Call CFG");
    let calls = function
        .blocks
        .values()
        .filter(|block| {
            matches!(
                block.terminator,
                Some(MirInstruction::Invoke {
                    operation: InvokeOperation::Call {
                        result: InvokeCallResultKind::I64,
                        ..
                    },
                    ..
                })
            )
        })
        .count();
    assert_eq!(
        calls, 2,
        "local and terminal direct calls are both physical"
    );
}

#[test]
fn source_terminal_call_preserves_both_cleanup_paths_through_finishing() {
    for prefix in [
        "",
        "local a = new Page()",
        "local a = new Page() local b = new Page()",
    ] {
        for optimize in [false, true] {
            let mut package = issue(&format!(
                "box Page {{}} static box Main {{
                main() {{ {prefix} return helper(30, 5) }}
                helper(value: i64, other: i64): i64 {{ local m = %{{\"v\" => value}} return 30 }}
            }}"
            ))
            .unwrap();
            let mut loans = package.direct_call_loans.take().unwrap();
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
            let mut function = package
                .batch()
                .with_lowering_input_and_source_identity(
                    declaration.batch_slot(),
                    |input, identity| {
                        builder.lower_map_dependency_for_test(
                            input,
                            SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                            main.parser_identity(),
                            identity.method_source_observation().cloned(),
                            std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                            Some(&mut loans),
                        )
                    },
                )
                .unwrap()
                .unwrap_or_else(|e| panic!("{prefix}: {e}"));
            loans.finish_empty().unwrap();
            let ledger = &package.ordinary_new_claim_ledger;
            crate::mir::verification::MirVerifier::new_strict()
                .verify_function(&function)
                .unwrap();
            let observation = ledger
                .validate_finalized_new_root(&function)
                .unwrap_or_else(|e| panic!("{prefix}: {e}"));
            function
                .install_root_ordinary_new_observation(observation)
                .unwrap();
            let mut module = MirModule::new("call-cleanup-dependency".into());
            module.functions.insert("Main.main/0".into(), function);
            if optimize {
                crate::mir::passes::simplify_cfg::simplify(&mut module);
            }
            let function = &module.functions["Main.main/0"];
            crate::mir::verification::MirVerifier::new_strict()
                .verify_function(function)
                .unwrap();
            for mutation in 0..5 {
                let mut changed = function.clone();
                let mut found = false;
                for block in changed.blocks.values_mut() {
                    if let Some(MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Call {
                                call,
                                result: InvokeCallResultKind::I64,
                            },
                        fault_frame,
                        normal_landing,
                        fault_landing,
                    }) = &mut block.terminator
                    {
                        match mutation {
                            0 => std::mem::swap(normal_landing, fault_landing),
                            1 => *fault_landing = *normal_landing,
                            2 => *fault_frame = ValueId(999),
                            3 => call.args.swap(0, 1),
                            _ => call.dst = Some(ValueId(999)),
                        }
                        found = true;
                        break;
                    }
                }
                assert!(found);
                assert!(
                    ledger.validate_after_compiler_finishing(&changed).is_err(),
                    "mutation {mutation}"
                );
            }
            let (call_id, normal, fault) = function
                .blocks
                .iter()
                .find_map(|(id, block)| match &block.terminator {
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::Call { .. },
                        normal_landing,
                        fault_landing,
                        ..
                    }) => Some((*id, *normal_landing, *fault_landing)),
                    _ => None,
                })
                .unwrap();
            for mutation in 0..3 {
                let mut changed = function.clone();
                match mutation {
                    0 => {
                        let mut foreign =
                            crate::mir::BasicBlock::new(crate::mir::BasicBlockId(999));
                        foreign.set_terminator(MirInstruction::Jump {
                            target: normal,
                            edge_args: None,
                        });
                        changed.add_block(foreign);
                    }
                    1 => changed.blocks.get_mut(&fault).unwrap().add_instruction(
                        MirInstruction::InvokeNormalResult {
                            invoke_block: call_id,
                            dst: ValueId(999),
                        },
                    ),
                    _ => {
                        let terminal = changed
                            .blocks
                            .values_mut()
                            .find_map(|block| {
                                matches!(block.terminator, Some(MirInstruction::ReturnFault { .. }))
                                    .then_some(&mut block.terminator)
                            })
                            .unwrap();
                        *terminal = Some(MirInstruction::Return {
                            value: Some(ValueId(999)),
                        });
                    }
                }
                assert!(
                    ledger.validate_after_compiler_finishing(&changed).is_err(),
                    "cleanup mutation {mutation}"
                );
            }
            let pending_head = follow_jumps(function, fault);
            if let Some(MirInstruction::Invoke {
                operation: InvokeOperation::HomeRelease { .. },
                normal_landing: next,
                ..
            }) = &function.blocks[&pending_head].terminator
            {
                let next = *next;
                for mutation in 0..3 {
                    let mut changed = function.clone();
                    let block = changed.blocks.get_mut(&pending_head).unwrap();
                    if mutation == 0 {
                        block.set_terminator(MirInstruction::Jump {
                            target: next,
                            edge_args: None,
                        });
                    } else if let Some(MirInstruction::Invoke {
                        operation,
                        normal_landing,
                        ..
                    }) = &mut block.terminator
                    {
                        if mutation == 1 {
                            *normal_landing = normal;
                        } else if let InvokeOperation::HomeRelease { value, .. } = operation {
                            *value = ValueId(999);
                        }
                    }
                    assert!(
                        ledger.validate_after_compiler_finishing(&changed).is_err(),
                        "pending Home mutation {mutation}"
                    );
                }
                let next_head = follow_jumps(function, next);
                if matches!(
                    function.blocks[&next_head].terminator,
                    Some(MirInstruction::Invoke {
                        operation: InvokeOperation::HomeRelease { .. },
                        ..
                    })
                ) {
                    let mut changed = function.clone();
                    let Some(MirInstruction::Invoke {
                        operation: second, ..
                    }) = function.blocks[&next_head].terminator.as_ref()
                    else {
                        unreachable!()
                    };
                    let Some(MirInstruction::Invoke {
                        operation: first, ..
                    }) = function.blocks[&pending_head].terminator.as_ref()
                    else {
                        unreachable!()
                    };
                    for (id, replacement) in [(pending_head, second), (next_head, first)] {
                        let Some(MirInstruction::Invoke { operation, .. }) =
                            &mut changed.blocks.get_mut(&id).unwrap().terminator
                        else {
                            unreachable!()
                        };
                        *operation = replacement.clone();
                    }
                    assert!(
                        ledger.validate_after_compiler_finishing(&changed).is_err(),
                        "reordered Homes"
                    );
                }
            }
            ledger
                .validate_after_compiler_finishing(function)
                .unwrap_or_else(|e| panic!("{prefix}, optimize={optimize}: {e}"));
        }
    }
}

#[test]
fn source_terminal_call_payload_moves_into_final_root_handoff() {
    let mut package = issue(
        "static box Main { main() { return helper(30, 5) } helper(value: i64, other: i64): i64 { local m = %{\"v\" => value} return 30 } }",
    )
    .unwrap();
    let mut loans = package.direct_call_loans.take().unwrap();
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
    let mut function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                Some(&mut loans),
            )
        })
        .unwrap()
        .unwrap();
    loans.finish_empty().unwrap();
    let ledger = &package.ordinary_new_claim_ledger;
    let observation = ledger.validate_finalized_new_root(&function).unwrap();
    function
        .install_root_ordinary_new_observation(observation)
        .unwrap();
    ledger.validate_after_compiler_finishing(&function).unwrap();
    let handoff = ledger
        .seal_finalized_root_birth_handoff(
            "Main.main/0".into(),
            &std::collections::BTreeSet::new(),
            None,
        )
        .unwrap();
    let root = handoff
        .root_source()
        .expect("retained Call source relation");
    assert!(matches!(
        handoff.root_result(),
        Some(
            crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1::CallReturn { .. }
        )
    ));
    assert!(root.call_entry().is_some(), "Call payload is retained");
    assert!(root.call_entry().unwrap().call_invoke().is_some());
    assert!(
        !root.call_cleanup().is_empty(),
        "Call bindings stay retained"
    );
    let duplicate = ledger
        .seal_finalized_root_birth_handoff(
            "Main.main/0".into(),
            &std::collections::BTreeSet::new(),
            None,
        )
        .expect_err("a finalized Call payload cannot be taken twice");
    assert!(
        duplicate.contains("root-call-already-finalized"),
        "{duplicate}"
    );
}

#[test]
fn map_result_local_call_installs_lease_and_releases_at_exit() {
    // `local m = make_map()` receives the callee's map lease: the commit row
    // installs the binding, the exit chain releases it with `Map::End`, and
    // the plain `return 30` entry carries no call-frame evidence.
    let mut package = issue(
        r#"static box Main {
            main() { local m = make_map() return 30 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("map-result local call package");
    let mut loans = package.direct_call_loans.take().unwrap();
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
    let mut function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                Some(&mut loans),
            )
        })
        .unwrap()
        .expect("map-result local call lowering");
    loans.finish_empty().expect("the map Call row is consumed");
    // Exactly one Call{result:Map} invoke feeding one InvokeNormalResult.
    let received: Vec<ValueId> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            MirInstruction::InvokeNormalResult { invoke_block, dst }
                if function.blocks[invoke_block].all_instructions().any(|i| {
                    matches!(
                        i,
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                result: InvokeCallResultKind::Map,
                                ..
                            },
                            ..
                        }
                    )
                }) =>
            {
                Some(*dst)
            }
            _ => None,
        })
        .collect();
    let [map] = received.as_slice() else {
        panic!("exactly one map-result projection expected")
    };
    let map = *map;
    assert!(
        matches!(builder.value_type(map), Some(crate::mir::MirType::Box(name)) if name == "MapBox"),
        "the received value carries the MapBox type"
    );
    // The caller's exit cleanup releases the received lease exactly once.
    let ends = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Invoke {
                    operation: InvokeOperation::Map(MapInvokeOperation::End { map: released }),
                    ..
                } if *released == map
            )
        })
        .count();
    assert_eq!(ends, 1, "exactly one Map::End on the received lease");
    // The receiving local reuses the call result; no Copy re-binds it.
    assert!(!function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(i, MirInstruction::Copy { src, .. } if *src == map)));
    let ledger = &package.ordinary_new_claim_ledger;
    // The commit row installed the call result as the receiving local.
    let completion = ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let call = flow
        .local_calls()
        .iter()
        .find(|call| {
            call.result()
                == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Map
        })
        .expect("sealed map-result local call");
    assert!(ledger.is_installed_map_binding(call.destination(), map));
    crate::mir::verification::MirVerifier::new_strict()
        .verify_function(&function)
        .expect("map-receive CFG verifies");
    let observation = ledger
        .validate_finalized_new_root(&function)
        .expect("finalized map-receive root");
    function
        .install_root_ordinary_new_observation(observation)
        .unwrap();
    // Result-kind drift: the Map call result reclassified as I64.
    let mut drifted = function.clone();
    let mut changed = false;
    for block in drifted.blocks.values_mut() {
        if let Some(MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    result: result @ InvokeCallResultKind::Map,
                    ..
                },
            ..
        }) = &mut block.terminator
        {
            *result = InvokeCallResultKind::I64;
            changed = true;
            break;
        }
    }
    assert!(changed);
    assert!(
        ledger.validate_after_compiler_finishing(&drifted).is_err(),
        "result-kind drift must reject"
    );
    // Cleanup drift: the exit release targets a different value.
    let mut drifted = function.clone();
    let mut changed = false;
    for block in drifted.blocks.values_mut() {
        if let Some(MirInstruction::Invoke {
            operation: InvokeOperation::Map(MapInvokeOperation::End { map: released }),
            ..
        }) = &mut block.terminator
        {
            *released = ValueId(999);
            changed = true;
            break;
        }
    }
    assert!(changed);
    assert!(
        ledger.validate_after_compiler_finishing(&drifted).is_err(),
        "cleanup-value drift must reject"
    );
    ledger
        .validate_after_compiler_finishing(&function)
        .expect("finished map-receive artifact");
}

#[test]
fn map_result_call_commit_row_rejects_foreign_and_duplicate_sites() {
    let mut package = issue(
        r#"static box Main {
            main() { local m = make_map() return 30 }
            make_map() { return %{"a" => 1} }
        }"#,
    )
    .expect("map-result local call package");
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    let call_site = flow.local_calls()[0].site().clone();
    // An unbegun map call leaves its lifecycle demand unconsumed.
    assert!(!ledger.map_demands_consumed());
    // A literal Map site is not a call source.
    let literal =
        issue("static box Main { main() { local m = %{\"a\" => 1} return 30 } }").unwrap();
    let literal_site = literal
        .ordinary_new_claim_ledger
        .root_completion_for_test()
        .cleanup()
        .root_flow()
        .unwrap()
        .maps()[0]
        .site()
        .clone();
    assert!(ledger
        .begin_map_call_emission(&literal_site)
        .unwrap_err()
        .contains("map-call-source-missing"));
    ledger.begin_map_call_emission(&call_site).unwrap();
    assert!(ledger
        .begin_map_call_emission(&call_site)
        .unwrap_err()
        .contains("map-duplicate-emission"));
    let _ = package.direct_call_loans.take();
}

#[test]
fn handle_result_local_call_installs_owned_home_and_releases_at_exit() {
    // `local h = make(0)` where `make` seals `Value(Construction)`: the
    // caller receives the callee's canonical object as an owned Home and
    // its exit owes exactly one `HomeRelease` on that identity.
    let mut package = issue(
        r#"box Point { x: i64 y: i64 birth(x, y) { me.x = x me.y = y } }
        static box Main {
            main() { local h = make(0) return 30 }
            make(args: i64) { return new Point(1, 2) }
        }"#,
    )
    .expect("handle-result local call package");
    let mut loans = package.direct_call_loans.take().unwrap();
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
    let mut function = package
        .batch()
        .with_lowering_input_and_source_identity(declaration.batch_slot(), |input, identity| {
            builder.lower_map_dependency_for_test(
                input,
                SelectedNormalCallableKeyV1::Cataloged(main.catalog_key().clone()),
                main.parser_identity(),
                identity.method_source_observation().cloned(),
                std::rc::Rc::clone(&package.ordinary_new_claim_ledger),
                Some(&mut loans),
            )
        })
        .unwrap()
        .expect("handle-result local call lowering");
    loans.finish_empty().expect("the handle Call row is consumed");
    // Exactly one Call{result:Handle} invoke feeding one InvokeNormalResult.
    let received: Vec<ValueId> = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter_map(|instruction| match instruction {
            MirInstruction::InvokeNormalResult { invoke_block, dst }
                if function.blocks[invoke_block].all_instructions().any(|i| {
                    matches!(
                        i,
                        MirInstruction::Invoke {
                            operation: InvokeOperation::Call {
                                result: InvokeCallResultKind::Handle,
                                ..
                            },
                            ..
                        }
                    )
                }) =>
            {
                Some(*dst)
            }
            _ => None,
        })
        .collect();
    let [received] = received.as_slice() else {
        panic!("exactly one handle-result projection expected")
    };
    let received = *received;
    assert!(
        matches!(builder.value_type(received), Some(crate::mir::MirType::Box(name)) if name == "Point"),
        "the received value carries the callee's result class"
    );
    // The caller's exit cleanup releases the received object exactly once.
    let releases = function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::Invoke {
                    operation: InvokeOperation::HomeRelease { value, .. },
                    ..
                } if *value == received
            )
        })
        .count();
    assert_eq!(releases, 1, "exactly one HomeRelease on the received object");
    // The receiving local reuses the call result; no Copy re-binds it and
    // no NewBox mints a caller-side acquisition for this site.
    assert!(!function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(i, MirInstruction::Copy { src, .. } if *src == received)));
    assert!(!function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(
            i,
            MirInstruction::Invoke {
                operation: InvokeOperation::NewBox { .. },
                ..
            }
        )));
    let ledger = &package.ordinary_new_claim_ledger;
    let completion = ledger.root_completion_for_test();
    let flow = completion.cleanup().root_flow().unwrap();
    assert_eq!(
        flow.local_calls()
            .iter()
            .filter(|call| {
                call.result()
                    == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Handle
            })
            .count(),
        1,
        "the sealed local call carries the Handle result class"
    );
    crate::mir::verification::MirVerifier::new_strict()
        .verify_function(&function)
        .expect("handle-receive CFG verifies");
    let observation = ledger
        .validate_finalized_new_root(&function)
        .expect("finalized handle-receive root");
    function
        .install_root_ordinary_new_observation(observation)
        .unwrap();
    // Result-kind drift: the Handle call result reclassified as I64.
    let mut drifted = function.clone();
    let mut changed = false;
    for block in drifted.blocks.values_mut() {
        if let Some(MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    result: result @ InvokeCallResultKind::Handle,
                    ..
                },
            ..
        }) = &mut block.terminator
        {
            *result = InvokeCallResultKind::I64;
            changed = true;
            break;
        }
    }
    assert!(changed);
    assert!(
        ledger.validate_after_compiler_finishing(&drifted).is_err(),
        "result-kind drift must reject"
    );
    // Cleanup drift: the exit release targets a different value.
    let mut drifted = function.clone();
    let mut changed = false;
    for block in drifted.blocks.values_mut() {
        if let Some(MirInstruction::Invoke {
            operation: InvokeOperation::HomeRelease { value, .. },
            ..
        }) = &mut block.terminator
        {
            *value = ValueId(999);
            changed = true;
            break;
        }
    }
    assert!(changed);
    assert!(
        ledger.validate_after_compiler_finishing(&drifted).is_err(),
        "cleanup-value drift must reject"
    );
    ledger
        .validate_after_compiler_finishing(&function)
        .expect("finished handle-receive artifact");
}

#[test]
fn handle_result_lane_rejects_returned_home_binding() {
    // `return p` where `p` is a locally-constructed Home is not a
    // construction result: the transfer edge a caller-side Handle row
    // requires is absent, so co-seal rejects rather than guessing a class.
    let package = issue(
        r#"box Point { x: i64 birth(x) { me.x = x } }
        static box Main {
            main() { local h = make(0) return 30 }
            make(args: i64) { local p = new Point(1) return p }
        }"#,
    );
    assert!(
        package.is_err(),
        "a Home-binding return is not a Handle result edge"
    );
}

#[test]
fn handle_result_lane_rejects_annotated_construction_callee() {
    // A declared `: i64` result annotation cannot describe a `return new`
    // body: annotation and sealed terminal relation disagree, and no
    // Handle claim may be inferred from the annotation.
    let package = issue(
        r#"box Point { x: i64 birth(x) { me.x = x } }
        static box Main {
            main() { local h = make(0) return 30 }
            make(args: i64): i64 { return new Point(1) }
        }"#,
    );
    assert!(
        package.is_err(),
        "annotation-construction mismatch must fail closed"
    );
}

fn follow_jumps(
    function: &crate::mir::MirFunction,
    mut id: crate::mir::BasicBlockId,
) -> crate::mir::BasicBlockId {
    for _ in 0..=function.blocks.len() {
        match function.blocks[&id].terminator.as_ref() {
            Some(MirInstruction::Jump { target, .. }) => id = *target,
            _ => return id,
        }
    }
    panic!("cyclic source cleanup");
}

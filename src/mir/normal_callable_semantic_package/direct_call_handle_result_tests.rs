//! Handle-result and lexical-handle call lanes reach physical lowering.
use super::physical::{cleanup_path_releases, follow_jumps};
use super::*;
use crate::mir::builder::SelectedNormalCallableKeyV1;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::{MirBuilder, MirInstruction, ValueId};

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

#[test]
fn lexical_handle_result_call_installs_owned_home_and_releases_at_exit() {
    // `local w = new Work(0); local h = w.make(0)` — the receiver is a
    // claim-local Home, the unique selected `Work.make` seals
    // `Value(Construction)`, and the caller receives the callee's
    // canonical object as an owned Home released once at exit.
    let mut package = issue(
        r#"box Point { x: i64 y: i64 birth(x, y) { me.x = x me.y = y } }
        box Work {
            n: i64
            birth(n) { me.n = n }
            make(args: i64) { return new Point(1, 2) }
        }
        static box Main {
            main() { local w = new Work(0) local h = w.make(0) return 30 }
        }"#,
    )
    .expect("lexical handle-result package");
    // No bare `FunctionCall` exists here — the only call edge is the
    // lexical `w.make(0)`, so this package carries no direct-call loans.
    let mut loans = package.direct_call_loans.take();
    let main = package
        .declaration_catalog()
        .source_backed_app_main()
        .unwrap();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.identity().same_as(main.parser_identity()))
        .unwrap();
    let ledger = package.ordinary_new_claim_ledger.clone();
    // The sealed observation owes a lifecycle commit before emission.
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
        "the lexical site sealed one Handle local-call observation"
    );
    assert!(!ledger.map_demands_consumed());
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
                loans.as_mut(),
            )
        })
        .unwrap()
        .expect("lexical handle-result lowering");
    // Exactly one instance-receiver Handle invoke feeding one projection.
    let mut received = None;
    let mut receiver = None;
    let mut landings = None;
    for instruction in function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
    {
        if let MirInstruction::Invoke {
            operation:
                InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::Handle,
                },
            normal_landing,
            fault_landing,
            ..
        } = instruction
        {
            let crate::mir::definitions::Callee::SameModuleInstance {
                key,
                receiver: recv,
            } = &call.callee
            else {
                panic!("the lexical handle invoke keeps its instance receiver")
            };
            assert!(key.name() == "make", "the unique selected target is Work.make");
            assert_eq!(key.namespace(), hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod);
            assert!(received.is_none(), "exactly one handle invoke");
            received = Some(instruction.clone());
            receiver = Some(*recv);
            landings = Some((*normal_landing, *fault_landing));
        }
    }
    assert!(received.is_some());
    let receiver = receiver.unwrap();
    let projected: Vec<ValueId> = function
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
    let [projected] = projected.as_slice() else {
        panic!("exactly one handle-result projection expected")
    };
    let received = *projected;
    assert!(
        matches!(builder.value_type(received), Some(crate::mir::MirType::Box(name)) if name == "Point"),
        "the received value carries the callee's result class"
    );
    // Path-sensitive release contract: every path releases a live Home
    // exactly once. The invoke's fault path unwinds the prior Home `w`
    // before the fault exits — the received object does not exist on that
    // path and is never released. The normal path releases the received
    // object and the receiver once each.
    let (normal_head, fault_head) = landings.unwrap();
    let fault_released = cleanup_path_releases(&function, fault_head);
    assert_eq!(
        fault_released,
        vec![receiver],
        "the call fault path unwinds exactly the prior receiver Home"
    );
    let normal_released = cleanup_path_releases(&function, normal_head);
    assert_eq!(
        normal_released
            .iter()
            .filter(|value| **value == received)
            .count(),
        1,
        "the normal path releases the received object once"
    );
    assert_eq!(
        normal_released
            .iter()
            .filter(|value| **value == receiver)
            .count(),
        1,
        "the normal path releases the receiver Home once"
    );
    // `local w = new Work(0)` is the only caller-side acquisition: the
    // received handle must not mint a second NewBox or re-bind by Copy.
    assert_eq!(
        function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
            .filter(|i| {
                matches!(
                    i,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::NewBox { .. },
                        ..
                    }
                )
            })
            .count(),
        1,
        "only the receiver's own `new` mints a caller-side acquisition"
    );
    assert!(!function
        .blocks
        .values()
        .flat_map(|block| block.all_instructions())
        .any(|i| matches!(i, MirInstruction::Copy { src, .. } if *src == received)));
    assert!(ledger.map_demands_consumed());
    crate::mir::verification::MirVerifier::new_strict()
        .verify_function(&function)
        .expect("lexical handle-receive CFG verifies");
    let observation = ledger
        .validate_finalized_new_root(&function)
        .expect("finalized lexical handle-receive root");
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
        .expect("finished lexical handle-receive artifact");
}

#[test]
fn lexical_handle_lane_leaves_rebound_receiver_dynamic() {
    // `w` is rebound between its `new` claim and the call: the receiver's
    // provenance is no longer a single claim-local `new`, so no Handle
    // observation seals and the site stays on the dynamic member route.
    let mut package = issue(
        r#"box Point { x: i64 y: i64 birth(x, y) { me.x = x me.y = y } }
        box Work {
            n: i64
            birth(n) { me.n = n }
            make(args: i64) { return new Point(1, 2) }
        }
        static box Main {
            main() {
                local w = new Work(0)
                w = new Work(1)
                local h = w.make(0)
                return 30
            }
        }"#,
    )
    .expect("rebound receiver still issues");
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
        0,
        "a rebound receiver must not seal a Handle observation"
    );
    let _ = package.direct_call_loans.take();
}

#[test]
fn lexical_handle_lane_leaves_nonconstruction_callee_dynamic() {
    // `w.size()` returns an integer field, not `new`: the sealed terminal
    // relation is truthful I64 evidence, the disposition records it, and
    // the caller scan issues no Handle observation.
    let mut package = issue(
        r#"box Work {
            n: i64
            birth(n) { me.n = n }
            size() { return me.n }
        }
        static box Main {
            main() { local w = new Work(0) local h = w.size() return h }
        }"#,
    )
    .expect("non-construction callee still issues");
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
        0,
        "a non-construction callee must not seal a Handle observation"
    );
    let _ = package.direct_call_loans.take();
}

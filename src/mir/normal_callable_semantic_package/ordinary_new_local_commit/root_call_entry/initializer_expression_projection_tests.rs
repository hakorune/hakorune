//! Original Local initialization must remain mandatory through final publication.
use super::*;

#[test]
fn scalar_initializer_inline_scope_keeps_complete_final_sequence() {
    let text = "static box Main { main() { local a = 5 local b = copy a + 1 print(b) return 0 } }";
    let (completed, ledger) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, same, keys, cohort) =
        completed.document_preflight_parts_for_test(true).unwrap();
    assert!(Rc::ptr_eq(&same, &ledger));
    let handoff = ledger
        .seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort, Vec::new())
        .unwrap();
    let root = handoff.root_source().unwrap();
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
    assert!(root
        .local_calls
        .get(&root.owner())
        .unwrap()
        .iter()
        .filter_map(|group| group.initializer())
        .any(|packet| !ledger.expression_requires_fault_frame_v1(packet.frame.evaluation())));
    for mutation in ["insert", "duplicate", "remove", "reorder"] {
        let mut changed = module.clone();
        let function = changed.functions.get_mut(&key).unwrap();
        let block = function
            .blocks
            .values_mut()
            .find(|block| {
                block.instructions.iter().any(|row| {
                    matches!(
                        row,
                        MirInstruction::BinOp {
                            op: crate::mir::BinaryOp::Add,
                            ..
                        }
                    )
                })
            })
            .unwrap();
        let index = block
            .instructions
            .iter()
            .position(|row| {
                matches!(
                    row,
                    MirInstruction::BinOp {
                        op: crate::mir::BinaryOp::Add,
                        ..
                    }
                )
            })
            .unwrap();
        match mutation {
            "insert" => block.instructions.insert(
                index,
                MirInstruction::Const {
                    dst: ValueId(99999),
                    value: crate::mir::ConstValue::Integer(99),
                },
            ),
            "duplicate" => block
                .instructions
                .insert(index, block.instructions[index].clone()),
            "remove" => {
                block.instructions.remove(index);
            }
            "reorder" => {
                assert!(index > 0);
                block.instructions.swap(index - 1, index);
            }
            _ => unreachable!(),
        }
        assert!(
            root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
                .is_err(),
            "{mutation}"
        );
    }
}

#[test]
fn initializer_core_i64_final_consumer_rechecks_jumps_reads_local_and_cleanup() {
    let text = "static box Main { main() { local start = 1 local alias = start
        local n = alias + (\"a\" + \"b\").length() return 0 } }";
    let (completed, ledger) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, same, keys, cohort) =
        completed.document_preflight_parts_for_test(true).unwrap();
    assert!(Rc::ptr_eq(&same, &ledger));
    let handoff = ledger
        .seal_finalized_root_birth_handoff(key.clone(), &module, &keys, cohort, Vec::new())
        .unwrap();
    let root = handoff.root_source().unwrap();
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
    let packets: Vec<_> = root
        .local_calls
        .get(&root.owner())
        .unwrap()
        .iter()
        .filter_map(|group| group.initializer())
        .collect();
    assert_eq!(packets.len(), 1);
    let packet = packets[0];
    let function = module.get_function(&key).unwrap();
    ledger
        .with_finished_projection(root.owner(), |_, projection| {
            // All three original Jumps remain obligations even when finishing
            // contracts them into the same surviving block.
            for original in [&packet.entry, &packet.terminal, &packet.continuation].map(|row| {
                row.as_ref()
                    .expect("fallible initializer retains every Jump")
            }) {
                projection.structural_jump_binding(function, original, true)?;
                let destination = projection.destination(original.0).unwrap();
                let mut changed = module.clone();
                changed
                    .functions
                    .get_mut(&key)
                    .unwrap()
                    .blocks
                    .get_mut(&destination)
                    .unwrap()
                    .terminator = Some(MirInstruction::Jump {
                    target: BasicBlockId(99999),
                    edge_args: None,
                });
                assert!(root
                    .visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
                    .is_err());
            }
            for mutation in [
                "parameter-collision",
                "duplicate-local",
                "literal",
                "cleanup",
            ] {
                let mut changed = module.clone();
                let changed_function = changed.functions.get_mut(&key).unwrap();
                match mutation {
                    "parameter-collision" => changed_function.params.push(packet.local),
                    "duplicate-local" => {
                        let copy = packet.copy.as_ref().unwrap();
                        let block = projection.destination(copy.0).unwrap();
                        // Even an omitted dead Copy cannot acquire new definitions.
                        changed_function
                            .blocks
                            .get_mut(&block)
                            .unwrap()
                            .instructions
                            .extend([copy.1.clone(), copy.1.clone()]);
                    }
                    "literal" => {
                        let mut count = 0;
                        for block in changed_function.blocks.values_mut() {
                            for instruction in &mut block.instructions {
                                if let MirInstruction::Const {
                                    value: crate::mir::ConstValue::Integer(value),
                                    ..
                                } = instruction
                                {
                                    if *value == 1 {
                                        *value = 2;
                                        count += 1;
                                    }
                                }
                            }
                        }
                        assert!(count > 0);
                    }
                    "cleanup" => {
                        let mut count = 0;
                        for block in changed_function.blocks.values_mut() {
                            if let Some(MirInstruction::Invoke {
                                operation:
                                    InvokeOperation::String(
                                        crate::mir::instruction::StringInvokeOperation::End {
                                            value,
                                        },
                                    ),
                                ..
                            }) = &mut block.terminator
                            {
                                *value = ValueId(99999);
                                count += 1;
                            }
                        }
                        assert!(count > 0);
                    }
                    _ => unreachable!(),
                }
                assert!(
                    root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(
                        ()
                    ))
                    .is_err(),
                    "{mutation}"
                );
            }
            Ok(())
        })
        .unwrap();
}

#[test]
fn empty_scalar_rebind_keeps_original_source_and_finished_sequence() {
    let text = "static box Main {
        a(x: i64): i64 { local y = 0 if x > 2 { y = x } return y }
        main() { print(Main.a(4)) print(Main.a(1)) return 0 }
    }";
    let (completed, ledger) =
        crate::mir::builder::lexical_call_projection_document_completion_fixture(text);
    let (key, module, same, keys, cohort) =
        completed.document_preflight_parts_for_test(true).unwrap();
    assert!(Rc::ptr_eq(&same, &ledger));
    let handoff = ledger
        .seal_finalized_root_birth_handoff(key, &module, &keys, cohort, Vec::new())
        .unwrap();
    let root = handoff.root_source().unwrap();
    root.visit_finalized_lexical_call_nodes_v1(&module, |_, _, _, _, _, _, _| Ok(()))
        .unwrap();
    let mut changed = module.clone();
    let function = changed.functions.get_mut("Main.a/1").unwrap();
    let block = function
        .blocks
        .values_mut()
        .find(|body| {
            body.instructions
                .iter()
                .any(|row| matches!(row, MirInstruction::Phi { .. }))
        })
        .unwrap();
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(99999),
        value: crate::mir::ConstValue::Integer(99),
    });
    assert!(
        root.visit_finalized_lexical_call_nodes_v1(&changed, |_, _, _, _, _, _, _| Ok(()))
            .is_err(),
        "an empty evaluation never grants an unowned instruction"
    );
}

//! Original Local initialization must remain mandatory through final publication.
use super::*;

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
            for original in [&packet.entry, &packet.terminal, &packet.continuation] {
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

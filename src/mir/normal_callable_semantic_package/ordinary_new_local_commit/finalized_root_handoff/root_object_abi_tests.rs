//! Whole original Root ABI after finishing, reusing the existing module fixture.
use super::*;

pub(super) fn mixed_fresh_null_abi_preserves_source_and_physical_producers() {
    for null_first in [false, true] {
        for failure in 0..8 {
            let (then, tail) = if null_first {
                ("null", "new Token()")
            } else {
                ("new Token()", "null")
            };
            let text = source(false, false, false, false)
                .replacen(
                    "make(size: i64)",
                    "count(): i64 { return 1 } make(size: i64)",
                    1,
                )
                .replace(
                    "return maker.make(7)",
                    &format!(
                    "local count = maker.count() if count > 0 {{ return {then} }} return {tail}"
                ),
                );
            let (key, mut module, ledger, keys, cohort) = completed(&text)
                .document_preflight_parts_for_test(true)
                .unwrap();
            let owner = ledger.root_owner().unwrap();
            let (exit, site, original) = {
                let exits = ledger.root_exits.borrow();
                ledger.terminal_relation_index[&owner]
                    .iter()
                    .find_map(|(exit, terminal)| {
                        let TerminalRelationV1::Value(value) = terminal else {
                            return None;
                        };
                        let TerminalReturnedSourceV1::Construction(site) = value.returned() else {
                            return None;
                        };
                        let RootHomeExitProgress::Emitted {
                            entry, bindings, ..
                        } = &exits[&(owner, exit.clone())]
                        else {
                            panic!("original Fresh Root exit");
                        };
                        let pair = ledger
                            .validate_fresh_return_producer_v1(
                                owner, exit, entry, bindings, None, None, None,
                            )
                            .unwrap()
                            .unwrap();
                        Some((exit.clone(), site.clone(), pair))
                    })
                    .unwrap()
            };
            let mut handoff = ledger
                .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
                .unwrap();
            assert_eq!(
                ledger
                    .checked_root_object_result_v1()
                    .unwrap()
                    .unwrap()
                    .terminals
                    .len(),
                2,
                "dynamic condition retains both original explicit exits"
            );
            assert_eq!(
                handoff.root_result(&module).unwrap(),
                Some(FinalizedRootResultAbiV1::ObjectReturn {
                    owner,
                    kind: crate::mir::instruction::InvokeCallResultKind::NullableHandle,
                })
            );
            match failure {
                0 => continue,
                1 => {
                    let retained = match &mut handoff {
                        FinalizedRootHandoffV1::NoBirth {
                            root_source: Some(source),
                            ..
                        }
                        | FinalizedRootHandoffV1::Births {
                            root_source: Some(source),
                            ..
                        } => source,
                        _ => panic!("original retained Root"),
                    };
                    let other = retained
                        .terminals
                        .iter()
                        .find_map(|(other, terminal)| {
                            if other == &exit {
                                return None;
                            }
                            let TerminalRelationV1::Value(value) = terminal else {
                                return None;
                            };
                            Some(value.value_site().clone())
                        })
                        .unwrap();
                    let TerminalRelationV1::Value(value) = &retained.terminals[&exit] else {
                        unreachable!()
                    };
                    let changed = value.with_value_site_for_test(other);
                    assert_eq!(changed.return_site(), &exit);
                    retained
                        .terminals
                        .insert(exit.clone(), TerminalRelationV1::Value(changed));
                    assert!(handoff
                        .root_result(&module)
                        .unwrap_err()
                        .contains("root-result/source-identity"));
                }
                2 => {
                    let mut rows = ledger.local_commits.borrow_mut();
                    let LocalCommitV1::Result(row) = rows.get_mut(&site).unwrap() else {
                        unreachable!()
                    };
                    row.emission = NewEmissionProgress::RetainedUnavailable {
                        progress: UnavailableLocalProgress::ExpressionCompleted {
                            initializer: ValueId::new(9999),
                        },
                    };
                    drop(rows);
                    assert!(handoff
                        .root_result(&module)
                        .unwrap_err()
                        .contains("root-result/fresh-producer-unavailable"));
                }
                3 => {
                    let (symbol, allocation) = ledger
                        .finished_binding_for_owner(owner, &original[1])
                        .unwrap();
                    let block = module
                        .functions
                        .get_mut(&symbol)
                        .unwrap()
                        .blocks
                        .get_mut(&allocation.0)
                        .unwrap();
                    let instruction = block.terminator.as_mut().unwrap();
                    assert_eq!(*instruction, allocation.1);
                    let MirInstruction::Invoke {
                        normal_landing,
                        fault_landing,
                        ..
                    } = instruction
                    else {
                        unreachable!()
                    };
                    assert_ne!(normal_landing, fault_landing);
                    *normal_landing = *fault_landing;
                    assert!(handoff
                        .root_result(&module)
                        .unwrap_err()
                        .contains("fresh-return/producer-actual-drift"));
                }
                4..=7 => {
                    let null = failure == 5 || failure == 7;
                    let target = ledger.terminal_relation_index[&owner]
                        .iter()
                        .find_map(|(site, terminal)| {
                            let TerminalRelationV1::Value(value) = terminal else {
                                return None;
                            };
                            (matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral)
                                == null)
                                .then_some(site.clone())
                        })
                        .unwrap();
                    let original = {
                        let exits = ledger.root_exits.borrow();
                        let RootHomeExitProgress::Emitted { bindings, .. } =
                            &exits[&(owner, target.clone())]
                        else {
                            panic!("retained Plain Root exit");
                        };
                        bindings
                            .iter()
                            .find(|(_, instruction)| {
                                matches!(instruction, MirInstruction::Return { .. })
                            })
                            .unwrap()
                            .clone()
                    };
                    let (symbol, returned) =
                        ledger.finished_binding_for_owner(owner, &original).unwrap();
                    let replacement = MirInstruction::Return {
                        value: Some(ValueId::new(9999)),
                    };
                    let block = module
                        .functions
                        .get_mut(&symbol)
                        .unwrap()
                        .blocks
                        .get_mut(&returned.0)
                        .unwrap();
                    assert_eq!(block.terminator.as_ref(), Some(&returned.1));
                    block.instructions.push(MirInstruction::Const {
                        dst: ValueId::new(9999),
                        value: crate::mir::ConstValue::Null,
                    });
                    block.terminator = Some(replacement.clone());
                    if failure >= 6 {
                        let mut exits = ledger.root_exits.borrow_mut();
                        let RootHomeExitProgress::Emitted { bindings, .. } =
                            exits.get_mut(&(owner, target)).unwrap()
                        else {
                            unreachable!()
                        };
                        bindings
                            .iter_mut()
                            .find(|binding| **binding == original)
                            .unwrap()
                            .1 = replacement;
                    }
                    assert!(
                        handoff.root_result(&module).is_err(),
                        "late Return corruption null={null}, joint={}",
                        failure >= 6
                    );
                }
                _ => unreachable!(),
            }
        }
    }
}

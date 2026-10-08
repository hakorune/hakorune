//! Final producer/Return corruption checks reuse the real original module fixture.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::ObjectReturnAcquisitionV1;

fn finished_binding(
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: FunctionOwnerIdV1,
    binding: &(BasicBlockId, MirInstruction),
) -> (String, (BasicBlockId, MirInstruction)) {
    let root = ledger.root_validation.borrow();
    if let RootNewValidation::FinishingChecked {
        owner: root_owner,
        symbol,
        projection,
    } = &*root
    {
        if *root_owner == owner {
            return (
                symbol.clone(),
                projection.binding(binding.0, &binding.1).unwrap().unwrap(),
            );
        }
    }
    let children = ledger.child_physical_validation.borrow();
    let ChildPhysicalValidation::FinishingChecked { symbol, projection } = &children[&owner] else {
        panic!("finished original Received child")
    };
    (
        symbol.clone(),
        projection.binding(binding.0, &binding.1).unwrap().unwrap(),
    )
}

pub(super) fn received_return_joint_drift_and_missing_producer() {
    for root in [false, true] {
        for nullable in [false, true] {
            for failure in 0..6 {
                if failure >= 3 && (root || nullable) {
                    continue; // Installation phases share the same commit owner.
                }
                if root && failure == 2 {
                    continue; // Child owns the mutable finishing-binding corruption seam.
                }
                let (key, mut module, ledger, keys, cohort) =
                    completed(&source(true, false, nullable, false))
                        .document_preflight_parts_for_test(true)
                        .unwrap();
                let root_owner = ledger.root_owner().unwrap();
                let (owner, exit, site) = ledger
                    .terminal_relation_index
                    .iter()
                    .filter(|(owner, _)| (**owner == root_owner) == root)
                    .find_map(|(owner, rows)| {
                        rows.iter().find_map(|(exit, row)| {
                            let TerminalRelationV1::Value(value) = row else {
                                return None;
                            };
                            let TerminalReturnedSourceV1::OwnedCall(obligation) = value.returned()
                            else {
                                return None;
                            };
                            let ObjectReturnAcquisitionV1::Received(call) =
                                obligation.acquisition()
                            else {
                                return None;
                            };
                            Some((*owner, exit.clone(), call.site().clone()))
                        })
                    })
                    .unwrap();
                let before = snapshot(&ledger);
                {
                    let exits = ledger.root_exits.borrow();
                    let RootHomeExitProgress::Emitted {
                        entry, bindings, ..
                    } = &exits[&(owner, exit.clone())]
                    else {
                        panic!("Received exit")
                    };
                    assert!(ledger
                        .validate_received_return_producer_v1(
                            owner, &exit, entry, bindings, None, None,
                        )
                        .unwrap()
                        .is_some());
                }
                match failure {
                    0 => {
                        let (block, old) = {
                            let mut exits = ledger.root_exits.borrow_mut();
                            let RootHomeExitProgress::Emitted { bindings, .. } =
                                exits.get_mut(&(owner, exit.clone())).unwrap()
                            else {
                                panic!("Received exit")
                            };
                            let (block, returned) = bindings
                                .iter_mut()
                                .find(|(_, i)| matches!(i, MirInstruction::Return { .. }))
                                .unwrap();
                            let old = returned.clone();
                            *returned = MirInstruction::Return {
                                value: Some(ValueId::new(9999)),
                            };
                            (*block, old)
                        };
                        let (symbol, (block, old)) =
                            finished_binding(&ledger, owner, &(block, old));
                        let physical = module
                            .functions
                            .get_mut(&symbol)
                            .unwrap()
                            .blocks
                            .get_mut(&block)
                            .unwrap();
                        assert_eq!(physical.terminator, Some(old));
                        physical.instructions.push(MirInstruction::Const {
                            dst: ValueId::new(9999),
                            value: crate::mir::ConstValue::Null,
                        });
                        physical.terminator = Some(MirInstruction::Return {
                            value: Some(ValueId::new(9999)),
                        });
                        let exits = ledger.root_exits.borrow();
                        let RootHomeExitProgress::Emitted {
                            entry, bindings, ..
                        } = &exits[&(owner, exit.clone())]
                        else {
                            panic!("Received exit")
                        };
                        assert!(ledger
                            .validate_received_return_producer_v1(
                                owner, &exit, entry, bindings, None, None
                            )
                            .unwrap_err()
                            .contains("received-return/return-value"));
                    }
                    1 => {
                        let mut rows = ledger.local_commits.borrow_mut();
                        let destination = rows[&site].binding().unwrap();
                        let foreign = rows
                            .values()
                            .filter(|row| row.owner() == owner)
                            .filter_map(|row| row.binding())
                            .find(|binding| *binding != destination)
                            .unwrap();
                        let LocalCommitV1::CallReceived(row) = rows.get_mut(&site).unwrap() else {
                            panic!("Received commit")
                        };
                        row.binding = foreign;
                    }
                    2 => {
                        let producer = {
                            let rows = ledger.local_commits.borrow();
                            let LocalCommitV1::CallReceived(row) = &rows[&site] else {
                                panic!("Received commit")
                            };
                            row.checked_bindings().unwrap().iter().find(|(_, i)|
                                matches!(i, MirInstruction::InvokeNormalResult { dst, .. } if Some(*dst) == row.local())
                            ).unwrap().clone()
                        };
                        let (_, producer) = finished_binding(&ledger, owner, &producer);
                        let mut children = ledger.child_physical_validation.borrow_mut();
                        let ChildPhysicalValidation::FinishingChecked { projection, .. } =
                            children.get_mut(&owner).unwrap()
                        else {
                            panic!("Received finishing")
                        };
                        assert!(projection.recorded().contains(&producer));
                        projection.remove_recorded_binding_for_test(&producer);
                    }
                    3 | 4 => {
                        {
                            let mut rows = ledger.local_commits.borrow_mut();
                            let LocalCommitV1::CallReceived(row) = rows.get_mut(&site).unwrap()
                            else {
                                panic!("Received commit")
                            };
                            let CallReceivedProgress::Emitted { phase, .. } = &mut row.progress
                            else {
                                panic!("Received emission")
                            };
                            *phase = if failure == 3 {
                                CallReceivedPhase::ExpressionCompleted
                            } else {
                                CallReceivedPhase::Installed
                            };
                        }
                        let exits = ledger.root_exits.borrow();
                        let RootHomeExitProgress::Emitted {
                            entry, bindings, ..
                        } = &exits[&(owner, exit.clone())]
                        else {
                            panic!("Received exit")
                        };
                        let children = ledger.child_physical_validation.borrow();
                        let ChildPhysicalValidation::FinishingChecked { projection, .. } =
                            &children[&owner]
                        else {
                            panic!("Received finishing")
                        };
                        let error = ledger
                            .validate_received_return_producer_v1(
                                owner,
                                &exit,
                                entry,
                                bindings,
                                (failure == 4).then_some(projection),
                                None,
                            )
                            .unwrap_err();
                        assert!(error.contains(if failure == 3 {
                            "received-return/local-not-installed"
                        } else {
                            "artifact-handle-unchecked"
                        }));
                    }
                    5 => {
                        let mut rows = ledger.local_commits.borrow_mut();
                        let LocalCommitV1::CallReceived(row) = rows.get_mut(&site).unwrap() else {
                            panic!("Self Received commit")
                        };
                        let CallReceivedProgress::Emitted { packet, .. } = &mut row.progress else {
                            panic!("Self Received emission")
                        };
                        assert!(packet.is_some());
                        *packet = None;
                    }
                    _ => unreachable!(),
                }
                let corrupted = snapshot(&ledger);
                assert!(
                    ledger
                        .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
                        .is_err(),
                    "root={root},nullable={nullable},failure={failure}"
                );
                assert_eq!(
                    snapshot(&ledger),
                    corrupted,
                    "Received corruption precedes affine move"
                );
                if failure != 0 {
                    assert_eq!(corrupted, before);
                }
            }
        }
    }
}

pub(super) fn null_return_joint_drift_and_missing_finished_binding() {
    for missing_binding in [false, true] {
        let (key, mut module, ledger, keys, cohort) = completed(&source(false, false, true, false))
            .document_preflight_parts_for_test(true)
            .unwrap();
        let (owner, exit) = ledger
            .terminal_relation_index
            .iter()
            .find_map(|(owner, rows)| {
                rows.iter().find_map(|(exit, row)| match row {
                    TerminalRelationV1::Value(value)
                        if matches!(value.returned(), TerminalReturnedSourceV1::NullLiteral) =>
                    {
                        Some((*owner, exit.clone()))
                    }
                    _ => None,
                })
            })
            .unwrap();
        let (symbol, original) = {
            let exits = ledger.root_exits.borrow();
            let RootHomeExitProgress::Emitted { order, .. } = &exits[&(owner, exit.clone())] else {
                panic!("original Null exit")
            };
            let original = order.null_return_binding().unwrap().clone();
            // The collector has not finalized Root yet; inspect the same held
            // child projection, without using the post-finalization API.
            let children = ledger.child_physical_validation.borrow();
            let ChildPhysicalValidation::FinishingChecked { symbol, projection } =
                &children[&owner]
            else {
                panic!("finished original Null child")
            };
            (
                symbol.clone(),
                projection
                    .binding(original.0, &original.1)
                    .unwrap()
                    .unwrap(),
            )
        };
        if missing_binding {
            let mut states = ledger.child_physical_validation.borrow_mut();
            let ChildPhysicalValidation::FinishingChecked { projection, .. } =
                states.get_mut(&owner).unwrap()
            else {
                panic!("finished original Null child")
            };
            assert!(projection.recorded().contains(&original));
            projection.remove_recorded_binding_for_test(&original);
        } else {
            let changed = ValueId::new(9999);
            let (block, old) = {
                let mut exits = ledger.root_exits.borrow_mut();
                let RootHomeExitProgress::Emitted { bindings, .. } =
                    exits.get_mut(&(owner, exit)).unwrap()
                else {
                    panic!("Null Emitted")
                };
                let (block, instruction) = bindings
                    .iter_mut()
                    .find(|(_, instruction)| matches!(instruction, MirInstruction::Return { .. }))
                    .unwrap();
                let old = instruction.clone();
                *instruction = MirInstruction::Return {
                    value: Some(changed),
                };
                (*block, old)
            };
            let (block, old) = {
                let children = ledger.child_physical_validation.borrow();
                let ChildPhysicalValidation::FinishingChecked { projection, .. } =
                    &children[&owner]
                else {
                    panic!("finished original Null child")
                };
                projection.binding(block, &old).unwrap().unwrap()
            };
            let function = module.functions.get_mut(&symbol).unwrap();
            let physical = function.blocks.get_mut(&block).unwrap();
            assert_eq!(physical.terminator, Some(old));
            physical.instructions.push(MirInstruction::Const {
                dst: changed,
                value: crate::mir::ConstValue::Null,
            });
            physical.terminator = Some(MirInstruction::Return {
                value: Some(changed),
            });
        }
        let before = snapshot(&ledger);
        assert!(
            ledger
                .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
                .is_err(),
            "missing_binding={missing_binding}"
        );
        assert_eq!(
            snapshot(&ledger),
            before,
            "Null corruption must precede affine move"
        );
    }
}

pub(super) fn fresh_return_exact_exit_and_finished_producer() {
    for failure in 0..3 {
        let text = source(false, false, false, false).replacen(
            "return new Token()",
            "if size > 0 { return new Token() } return new Token()",
            1,
        );
        let (key, mut module, ledger, keys, cohort) = completed(&text)
            .document_preflight_parts_for_test(true)
            .unwrap();
        let (owner, exits) = ledger
            .terminal_relation_index
            .iter()
            .find_map(|(owner, rows)| {
                let exits: Vec<_> = rows
                    .iter()
                    .filter_map(|(exit, row)| {
                        let TerminalRelationV1::Value(value) = row else {
                            return None;
                        };
                        let TerminalReturnedSourceV1::Construction(site) = value.returned() else {
                            return None;
                        };
                        Some((exit.clone(), site.clone()))
                    })
                    .collect();
                (exits.len() == 2).then_some((*owner, exits))
            })
            .expect("dynamic factory has two original Fresh exits");
        let before = snapshot(&ledger);
        let mut originals = Vec::new();
        let mut results = Vec::new();
        for (exit, site) in &exits {
            let rows = ledger.local_commits.borrow();
            let LocalCommitV1::Result(row) = &rows[site] else {
                panic!("Fresh Result commit");
            };
            let NewEmissionProgress::Emitted { result, .. } = row.emission else {
                panic!("Fresh producer");
            };
            results.push(result);
            let stored = ledger.root_exits.borrow();
            let RootHomeExitProgress::Emitted {
                entry, bindings, ..
            } = &stored[&(owner, exit.clone())]
            else {
                panic!("Fresh exit");
            };
            assert!(ledger
                .validate_fresh_return_producer_v1(owner, exit, entry, bindings, None, None, None)
                .unwrap()
                .is_some());
            originals.push(
                bindings
                    .iter()
                    .find(|(_, instruction)| matches!(instruction, MirInstruction::Return { .. }))
                    .unwrap()
                    .clone(),
            );
        }
        assert_ne!(results[0], results[1]);
        if failure == 1 {
            let producer = {
                let stored = ledger.root_exits.borrow();
                let RootHomeExitProgress::Emitted {
                    entry, bindings, ..
                } = &stored[&(owner, exits[0].0.clone())]
                else {
                    unreachable!()
                };
                ledger
                    .validate_fresh_return_producer_v1(
                        owner,
                        &exits[0].0,
                        entry,
                        bindings,
                        None,
                        None,
                        None,
                    )
                    .unwrap()
                    .unwrap()
            };
            let (_, producer) = finished_binding(&ledger, owner, &producer[0]);
            let mut children = ledger.child_physical_validation.borrow_mut();
            let ChildPhysicalValidation::FinishingChecked { projection, .. } =
                children.get_mut(&owner).unwrap()
            else {
                panic!("Fresh finishing");
            };
            assert!(projection.recorded().contains(&producer));
            projection.remove_recorded_binding_for_test(&producer);
        } else if failure == 2 {
            let original = {
                let rows = ledger.local_commits.borrow();
                let LocalCommitV1::Result(row) = &rows[&exits[0].1] else {
                    unreachable!()
                };
                let NewEmissionProgress::Emitted { bindings, .. } = &row.emission else {
                    unreachable!()
                };
                bindings
                    .iter()
                    .find(|(_, instruction)| {
                        matches!(
                            instruction,
                            MirInstruction::Invoke {
                                operation: InvokeOperation::NewBox { .. },
                                ..
                            }
                        )
                    })
                    .unwrap()
                    .clone()
            };
            let (symbol, allocation) = finished_binding(&ledger, owner, &original);
            let block = module
                .functions
                .get_mut(&symbol)
                .unwrap()
                .blocks
                .get_mut(&allocation.0)
                .unwrap();
            let actual = block.terminator.as_mut().unwrap();
            assert_eq!(*actual, allocation.1);
            let MirInstruction::Invoke {
                normal_landing,
                fault_landing,
                ..
            } = actual
            else {
                unreachable!()
            };
            assert_ne!(normal_landing, fault_landing);
            *normal_landing = *fault_landing;
            // The producer and Return are unchanged; the allocation itself drifts.
            let stored = ledger.root_exits.borrow();
            let RootHomeExitProgress::Emitted {
                entry, bindings, ..
            } = &stored[&(owner, exits[0].0.clone())]
            else {
                unreachable!()
            };
            let children = ledger.child_physical_validation.borrow();
            let ChildPhysicalValidation::FinishingChecked { projection, .. } = &children[&owner]
            else {
                unreachable!()
            };
            assert!(ledger
                .validate_fresh_return_producer_v1(
                    owner,
                    &exits[0].0,
                    entry,
                    bindings,
                    Some(projection),
                    Some(projection),
                    Some(&module.functions[&symbol])
                )
                .unwrap_err()
                .contains("fresh-return/producer-actual-drift"));
        } else {
            for i in 0..2 {
                let (symbol, physical) = finished_binding(&ledger, owner, &originals[i]);
                let old = &originals[i].1;
                let replacement = MirInstruction::Return {
                    value: Some(results[1 - i]),
                };
                let mut stored = ledger.root_exits.borrow_mut();
                let RootHomeExitProgress::Emitted { bindings, .. } =
                    stored.get_mut(&(owner, exits[i].0.clone())).unwrap()
                else {
                    unreachable!()
                };
                let binding = bindings
                    .iter_mut()
                    .find(|(_, instruction)| instruction == old)
                    .unwrap();
                binding.1 = replacement.clone();
                let block = module
                    .functions
                    .get_mut(&symbol)
                    .unwrap()
                    .blocks
                    .get_mut(&physical.0)
                    .unwrap();
                assert_eq!(block.terminator.as_ref(), Some(&physical.1));
                block.terminator = Some(replacement);
            }
            // The old whole-function check still sees exactly one Return per result.
            let (symbol, _) = finished_binding(&ledger, owner, &originals[0]);
            for result in &results {
                assert_eq!(module.functions[&symbol].blocks.values().flat_map(|block| block.all_instructions())
                    .filter(|instruction| matches!(instruction, MirInstruction::Return { value: Some(value) } if value == result)).count(), 1);
            }
            let stored = ledger.root_exits.borrow();
            let RootHomeExitProgress::Emitted {
                entry, bindings, ..
            } = &stored[&(owner, exits[0].0.clone())]
            else {
                unreachable!()
            };
            assert!(ledger
                .validate_fresh_return_producer_v1(
                    owner,
                    &exits[0].0,
                    entry,
                    bindings,
                    None,
                    None,
                    None
                )
                .unwrap_err()
                .contains("fresh-return/return-value"));
        }
        let corrupted = snapshot(&ledger);
        assert!(ledger
            .seal_finalized_root_birth_handoff(key, &module, &keys, cohort)
            .is_err());
        assert_eq!(
            snapshot(&ledger),
            corrupted,
            "Fresh corruption precedes affine move"
        );
        if failure == 1 {
            assert_eq!(corrupted, before);
        }
    }
}

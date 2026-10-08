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

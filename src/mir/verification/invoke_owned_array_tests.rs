//! Physical graph tests only: these issue no source Recipe or executable input.
use super::*;
use crate::mir::instruction::FaultFrameMode;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

fn descriptor(nullable: bool) -> Descriptor {
    Descriptor::new(
        hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(1).unwrap(),
        nullable,
    )
}
fn invoke(operation: InvokeOperation, normal: u32, fault: u32) -> MirInstruction {
    MirInstruction::Invoke {
        operation,
        fault_frame: ValueId(0),
        normal_landing: BasicBlockId(normal),
        fault_landing: BasicBlockId(fault),
    }
}
fn owned(operation: Op, normal: u32, fault: u32) -> MirInstruction {
    invoke(InvokeOperation::OwnedArray(operation), normal, fault)
}
fn end(normal: u32, fault: u32) -> MirInstruction {
    owned(
        Op::End {
            array: ValueId(10),
            descriptor: descriptor(true),
        },
        normal,
        fault,
    )
}
fn graph() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "owned_array_physical".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    let desc = descriptor(true);
    let mut rows = vec![
        (0, invoke(InvokeOperation::IntrinsicArrayNew, 1, 90)),
        (
            1,
            owned(
                Op::Claim {
                    array: ValueId(10),
                    descriptor: desc,
                },
                2,
                91,
            ),
        ),
        (
            2,
            invoke(
                InvokeOperation::NewBox {
                    object: desc.object(),
                },
                3,
                92,
            ),
        ),
        (
            3,
            owned(
                Op::Append {
                    array: ValueId(10),
                    descriptor: desc,
                    candidate: ValueId(20),
                },
                4,
                93,
            ),
        ),
        (
            4,
            owned(
                Op::Get {
                    array: ValueId(10),
                    descriptor: desc,
                    index: ValueId(1),
                },
                5,
                92,
            ),
        ),
        (5, end(6, 90)),
        (
            6,
            MirInstruction::Return {
                value: Some(ValueId(1)),
            },
        ),
        (
            90,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            91,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (92, end(90, 90)),
        (
            93,
            invoke(
                InvokeOperation::HomeRelease {
                    object: desc.object(),
                    value: ValueId(20),
                },
                92,
                92,
            ),
        ),
    ];
    // Unit Normal landings remain exclusive even where cleanup Fault edges
    // converge. This mirrors the existing shared Invoke contract.
    let mut landings = Vec::new();
    for (id, term) in &mut rows {
        if let MirInstruction::Invoke {
            operation,
            normal_landing,
            ..
        } = term
        {
            if operation.normal_result_kind().is_none() {
                let target = *normal_landing;
                *normal_landing = BasicBlockId(1000 + *id);
                landings.push((
                    1000 + *id,
                    MirInstruction::Jump {
                        target,
                        edge_args: None,
                    },
                ));
            }
        }
    }
    rows.extend(landings);
    for (id, term) in rows {
        let mut block = BasicBlock::new(BasicBlockId(id));
        if id == 0 {
            block.add_instruction(MirInstruction::FaultFrameEnter {
                dst: ValueId(0),
                mode: FaultFrameMode::RootOwned,
            });
            block.add_instruction(MirInstruction::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(0),
            });
        }
        for (landing, producer, value) in [(1, 0, 10), (3, 2, 20), (5, 4, 30)] {
            if id == landing {
                block.add_instruction(MirInstruction::InvokeNormalResult {
                    invoke_block: BasicBlockId(producer),
                    dst: ValueId(value),
                });
            }
        }
        if id == 5 {
            block.add_instruction(MirInstruction::Copy {
                dst: ValueId(31),
                src: ValueId(30),
            });
        }
        if id == 91 {
            block.add_instruction(MirInstruction::ArrayResidenceRelease { value: ValueId(10) });
        }
        block.set_terminator(term);
        function.add_block(block);
    }
    function
}

#[test]
fn owned_array_normal_and_fault_paths_preserve_holder_and_candidate_lifetimes() {
    let good = graph();
    let result = super::super::check_function(&good);
    assert!(result.is_ok(), "{result:?}");
    assert_eq!(check(&good), Ok(()));
    assert_eq!(check_source_uses(&good, |_, _, _| false), Ok(()));
    // A Phi transports the same holder, never a new lease or element borrow.
    let mut joined = good.clone();
    let fork = joined.get_block_mut(BasicBlockId(1003)).unwrap();
    fork.add_instruction(MirInstruction::Const { dst: ValueId(50), value: ConstValue::Bool(true) });
    fork.set_terminator(MirInstruction::Branch {
        condition: ValueId(50), then_bb: BasicBlockId(200), else_bb: BasicBlockId(201),
        then_edge_args: None, else_edge_args: None,
    });
    for id in [200, 201] {
        let mut block = BasicBlock::new(BasicBlockId(id));
        block.set_terminator(MirInstruction::Jump { target: BasicBlockId(4), edge_args: None });
        joined.add_block(block);
    }
    joined.get_block_mut(BasicBlockId(0)).unwrap().add_instruction(MirInstruction::Const {
        dst: ValueId(51), value: ConstValue::Null,
    });
    joined.get_block_mut(BasicBlockId(4)).unwrap().add_instruction(MirInstruction::Phi {
        dst: ValueId(40),
        inputs: vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(201), ValueId(10))],
        type_hint: None,
    });
    if let Some(MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(Op::Get { array, .. }), .. }) =
        &mut joined.get_block_mut(BasicBlockId(4)).unwrap().terminator
    { *array = ValueId(40); } else { panic!("original Get missing"); }
    assert_eq!(super::super::check_function(&joined), Ok(()));
    assert_eq!(check(&joined), Ok(()));
    assert_eq!(check_source_uses(&joined, |_, _, _| false), Ok(()));
    for inputs in [
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(201), ValueId(20))],
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(201), ValueId(1))],
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(201), ValueId(51))],
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(201), ValueId(40))],
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(200), ValueId(10))],
        vec![(BasicBlockId(200), ValueId(10))],
        vec![(BasicBlockId(200), ValueId(10)), (BasicBlockId(202), ValueId(10))],
    ] {
        let mut invalid = joined.clone();
        let MirInstruction::Phi { inputs: actual, .. } =
            &mut invalid.get_block_mut(BasicBlockId(4)).unwrap().instructions[0]
        else { panic!("original holder Phi missing"); };
        *actual = inputs;
        assert!(check(&invalid).is_err(), "invalid holder Phi was accepted");
    }
    let mut ended_phi = joined.clone();
    ended_phi.get_block_mut(BasicBlockId(6)).unwrap().add_instruction(MirInstruction::Copy {
        dst: ValueId(41), src: ValueId(40),
    });
    assert_eq!(check(&ended_phi), Err("owned-array-holder-not-live"));
    let mut missing_candidate_cleanup = good.clone();
    missing_candidate_cleanup
        .get_block_mut(BasicBlockId(93))
        .unwrap()
        .terminator = Some(end(1092, 90));
    assert_eq!(
        check(&missing_candidate_cleanup),
        Err("owned-array-candidate-before-holder-end")
    );
    let mut drift = good.clone();
    drift.get_block_mut(BasicBlockId(4)).unwrap().terminator = Some(owned(
        Op::Get {
            array: ValueId(10),
            descriptor: descriptor(false),
            index: ValueId(1),
        },
        5,
        92,
    ));
    assert_eq!(check(&drift), Err("owned-array-descriptor-or-holder-drift"));
    let mut handle_index = good.clone();
    handle_index
        .get_block_mut(BasicBlockId(4))
        .unwrap()
        .terminator = Some(owned(
        Op::Get {
            array: ValueId(10),
            descriptor: descriptor(true),
            index: ValueId(10),
        },
        5,
        92,
    ));
    assert_eq!(check(&handle_index), Err("owned-array-index-source"));
    let mut raw_end = good.clone();
    let block = raw_end.get_block_mut(BasicBlockId(5)).unwrap();
    block.add_instruction(MirInstruction::ArrayResidenceRelease { value: ValueId(10) });
    assert_eq!(check(&raw_end), Err("owned-array-holder-escape"));
    let mut duplicate_claim = good.clone();
    duplicate_claim
        .get_block_mut(BasicBlockId(4))
        .unwrap()
        .terminator = Some(owned(
        Op::Claim {
            array: ValueId(10),
            descriptor: descriptor(true),
        },
        5,
        92,
    ));
    assert_eq!(
        check(&duplicate_claim),
        Err("owned-array-duplicate-or-unacquired-claim")
    );
    let mut transferred = good.clone();
    transferred
        .get_block_mut(BasicBlockId(4))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: ValueId(32),
            src: ValueId(20),
        });
    assert_eq!(check(&transferred), Err("owned-array-candidate-not-live"));
    let mut missing_end = good.clone();
    missing_end
        .get_block_mut(BasicBlockId(5))
        .unwrap()
        .terminator = Some(MirInstruction::Return {
        value: Some(ValueId(1)),
    });
    assert_eq!(check(&missing_end), Err("owned-array-missing-end"));
}

#[test]
fn owned_array_borrows_cannot_be_ended_escaped_or_used_after_holder_end() {
    let good = graph();
    for instruction in [
        MirInstruction::Return {
            value: Some(ValueId(31)),
        },
        invoke(
            InvokeOperation::HomeReleaseIfLive {
                object: descriptor(true).object(),
                value: ValueId(31),
            },
            1005,
            90,
        ),
    ] {
        let mut changed = good.clone();
        changed.get_block_mut(BasicBlockId(5)).unwrap().terminator = Some(instruction);
        assert_eq!(check(&changed), Err("owned-array-borrow-escape"));
    }
    let mut ended = good.clone();
    ended
        .get_block_mut(BasicBlockId(6))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: ValueId(32),
            src: ValueId(31),
        });
    assert_eq!(check(&ended), Err("owned-array-borrow-not-live"));
    let mut env = good.clone();
    env.get_block_mut(BasicBlockId(5)).unwrap().return_env = Some(vec![ValueId(31)]);
    assert_eq!(check(&env), Err("owned-array-return-env-escape"));
    let mut cycle = good.clone();
    cycle.get_block_mut(BasicBlockId(6)).unwrap().terminator = Some(MirInstruction::Jump {
        target: BasicBlockId(6),
        edge_args: None,
    });
    assert_eq!(check(&cycle), Err("owned-array-lifetime-cycle"));
}

#[test]
fn owned_array_cataloged_call_shape_cannot_issue_original_borrow_permission() {
    let mut function = graph();
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "consume".into(),
        1,
    )
    .unwrap();
    function
        .get_block_mut(BasicBlockId(5))
        .unwrap()
        .add_instruction(MirInstruction::Call(crate::mir::definitions::MirCall::new(
            None,
            crate::mir::Callee::Global(target),
            vec![ValueId(31)],
        )));
    // The structural lifetime check sees a live borrowed argument, but it
    // cannot create the original source-backed Call coordinate/ordinal loan.
    assert_eq!(check(&function), Ok(()));
    assert_eq!(
        check_source_uses(&function, |_, _, _| false),
        Err("owned-array-original-borrow-slot-missing")
    );
}

#[test]
fn owned_array_completed_call_candidate_cannot_use_unpublished_reclaim() {
    let mut function = graph();
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "make".into(),
        0,
    )
    .unwrap();
    function.get_block_mut(BasicBlockId(2)).unwrap().terminator = Some(invoke(
        InvokeOperation::Call {
            call: crate::mir::definitions::MirCall::new(
                None,
                crate::mir::Callee::Global(target),
                vec![],
            ),
            result: InvokeCallResultKind::Handle,
        },
        3,
        92,
    ));
    // Affine outer-resource validation only. Complete child teardown still
    // requires the existing source/root cleanup proof before publication.
    assert_eq!(check(&function), Ok(()));
    for operation in [
        InvokeOperation::ReclaimUnpublished {
            object: descriptor(true).object(),
            value: ValueId(20),
        },
        InvokeOperation::HomeRelease {
            object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(2).unwrap(),
            value: ValueId(20),
        },
    ] {
        let mut changed = function.clone();
        changed.get_block_mut(BasicBlockId(93)).unwrap().terminator =
            Some(invoke(operation, 1093, 92));
        assert_eq!(check(&changed), Err("owned-array-candidate-cleanup-drift"));
    }
}

#[test]
fn owned_array_borrow_permission_does_not_allow_use_after_later_fault() {
    let mut function = graph();
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "step".into(),
        0,
    )
    .unwrap();
    function.get_block_mut(BasicBlockId(5)).unwrap().terminator = Some(invoke(
        InvokeOperation::Call {
            call: crate::mir::definitions::MirCall::new(
                None,
                crate::mir::Callee::Global(target),
                vec![],
            ),
            result: InvokeCallResultKind::I64,
        },
        7,
        8,
    ));
    let mut normal = BasicBlock::new(BasicBlockId(7));
    normal.add_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: BasicBlockId(5),
        dst: ValueId(40),
    });
    normal.set_terminator(end(1005, 90));
    function.add_block(normal);
    let mut fault = BasicBlock::new(BasicBlockId(8));
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "consume".into(),
        1,
    )
    .unwrap();
    fault.add_instruction(MirInstruction::Call(crate::mir::definitions::MirCall::new(
        None,
        crate::mir::Callee::Global(target),
        vec![ValueId(31)],
    )));
    fault.set_terminator(end(1092, 90));
    function.add_block(fault);
    assert_eq!(check(&function), Err("owned-array-borrow-use-after-fault"));
}

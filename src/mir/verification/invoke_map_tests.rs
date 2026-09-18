use super::*;
use crate::mir::instruction::{FaultFrameMode, InvokeCallResultKind, MapInvokeOperation as Map};
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

fn invoke(operation: Map, normal: u32, fault: u32) -> MirInstruction {
    MirInstruction::Invoke {
        operation: InvokeOperation::Map(operation),
        fault_frame: ValueId(0),
        normal_landing: BasicBlockId(normal),
        fault_landing: BasicBlockId(fault),
    }
}

fn graph(populated: bool) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "opaque_map".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    let mut rows = vec![
        (0, invoke(Map::New, 1, 9)),
        (1, invoke(Map::End { map: ValueId(2) }, 5, 9)),
        (
            5,
            MirInstruction::Return {
                value: Some(ValueId(1)),
            },
        ),
        (
            9,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
    ];
    if populated {
        rows[1].1 = invoke(
            Map::PrepareKey {
                utf8: "a\0b".into(),
            },
            2,
            6,
        );
        rows.extend([
            (
                2,
                invoke(
                    Map::InstallIndexed {
                        map: ValueId(2),
                        key: ValueId(3),
                        value: ValueId(1),
                        object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0)
                            .unwrap(),
                    },
                    3,
                    6,
                ),
            ),
            (
                3,
                invoke(
                    Map::EndOutcome {
                        outcome: ValueId(4),
                    },
                    4,
                    6,
                ),
            ),
            (4, invoke(Map::End { map: ValueId(2) }, 5, 9)),
            (6, invoke(Map::End { map: ValueId(2) }, 7, 8)),
            (
                7,
                MirInstruction::Jump {
                    target: BasicBlockId(9),
                    edge_args: None,
                },
            ),
            (
                8,
                MirInstruction::Jump {
                    target: BasicBlockId(9),
                    edge_args: None,
                },
            ),
        ]);
    }
    for (id, terminal) in rows {
        let mut block = BasicBlock::new(BasicBlockId(id));
        if id == 0 {
            block.add_instruction(MirInstruction::FaultFrameEnter {
                dst: ValueId(0),
                mode: FaultFrameMode::RootOwned,
            });
            // Physical sort witness only, not source/indexed ownership proof.
            block.add_instruction(MirInstruction::Const {
                dst: ValueId(1),
                value: ConstValue::Integer(30),
            });
        }
        if (1..=3).contains(&id) {
            block.add_instruction(MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(id - 1),
                dst: ValueId(id + 1),
            });
        }
        block.set_terminator(terminal);
        function.add_block(block);
    }
    function
}

#[test]
fn map_normal_results_and_returned_fault_paths_keep_opaque_lifetimes() {
    for populated in [false, true] {
        let function = graph(populated);
        check_function(&function).unwrap();
    }
}

#[test]
fn map_rejects_escape_role_drift_missing_end_and_nonexclusive_temporaries() {
    for mutation in 0..7 {
        let mut function = graph(true);
        let (block, terminal) = match mutation {
            0 => (
                5,
                MirInstruction::Return {
                    value: Some(ValueId(2)),
                },
            ),
            1 => (
                4,
                MirInstruction::Jump {
                    target: BasicBlockId(5),
                    edge_args: None,
                },
            ),
            2 => (
                3,
                invoke(
                    Map::EndOutcome {
                        outcome: ValueId(2),
                    },
                    4,
                    6,
                ),
            ),
            3 => (4, invoke(Map::End { map: ValueId(1) }, 5, 9)),
            4 => (4, invoke(Map::End { map: ValueId(2) }, 6, 9)),
            5 => (
                2,
                MirInstruction::Return {
                    value: Some(ValueId(3)),
                },
            ),
            _ => {
                function
                    .blocks
                    .get_mut(&BasicBlockId(2))
                    .unwrap()
                    .add_instruction(MirInstruction::Const {
                        dst: ValueId(99),
                        value: ConstValue::Integer(0),
                    });
                assert!(check_function(&function).is_err());
                continue;
            }
        };
        function
            .blocks
            .get_mut(&BasicBlockId(block))
            .unwrap()
            .set_terminator(terminal);
        assert!(check_function(&function).is_err(), "mutation {mutation}");
    }
}

fn map_call_function() -> MirFunction {
    let mut function = super::tests::allocation_invoke_function();
    let target = hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
        "Worker".into(),
        "produce".into(),
        0,
    )
    .unwrap();
    let MirInstruction::Invoke { operation, .. } = function
        .blocks
        .get_mut(&BasicBlockId::new(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap()
    else {
        unreachable!()
    };
    *operation = InvokeOperation::Call {
        call: crate::mir::definitions::MirCall::new(None, Callee::Global(target), vec![]),
        result: crate::mir::instruction::InvokeCallResultKind::Map,
    };
    function
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)),
        });
    function
}

#[test]
fn call_map_result_lease_transfers_across_the_return_boundary() {
    use crate::mir::MirVerifier;
    let function = map_call_function();
    MirVerifier::new().verify_function(&function).unwrap();
}

#[test]
fn call_map_result_lease_must_be_consumed_or_returned() {
    use crate::mir::MirVerifier;
    // Returning a different value abandons the call's live Map lease.
    let mut function = map_call_function();
    function
        .blocks
        .get_mut(&BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)),
        });
    assert!(MirVerifier::new().verify_function(&function).is_err());
}

#[test]
fn map_return_transfers_a_live_lease_and_rejects_a_spent_one() {
    let mut function = graph(true);
    function
        .blocks
        .get_mut(&BasicBlockId(4))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId(5),
            edge_args: None,
        });
    function
        .blocks
        .get_mut(&BasicBlockId(5))
        .unwrap()
        .set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)),
        });
    check_function(&function).unwrap();
}

#[test]
fn map_operation_rewrite_keeps_key_bytes_object_identity_and_effects() {
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let mut operation = InvokeOperation::Map(Map::InstallIndexed {
        map: ValueId(2),
        key: ValueId(3),
        object,
        value: ValueId(1),
    });
    operation.rewrite_values(|v| v.0 += 10);
    assert_eq!(
        operation.used_values(),
        vec![ValueId(12), ValueId(13), ValueId(11)]
    );
    assert!(!operation.effects().is_pure());
    assert!(
        matches!(operation, InvokeOperation::Map(Map::InstallIndexed { object: actual, .. }) if actual == object)
    );
    let mut key = InvokeOperation::Map(Map::PrepareKey {
        utf8: "a\0b".into(),
    });
    key.rewrite_values(|_| panic!("key text is not a ValueId"));
    assert_eq!(
        key,
        InvokeOperation::Map(Map::PrepareKey {
            utf8: "a\0b".into()
        })
    );
}

#[test]
fn scalar_install_preserves_opaque_protocol_and_rejects_opaque_payload() {
    use crate::mir::instruction::MapValueKind;
    for kind in [
        MapValueKind::I64,
        MapValueKind::Bool,
        MapValueKind::BorrowedHandle,
    ] {
        let mut function = graph(true);
        function
            .blocks
            .get_mut(&BasicBlockId(2))
            .unwrap()
            .set_terminator(invoke(
                Map::InstallValue {
                    map: ValueId(2),
                    key: ValueId(3),
                    value: ValueId(1),
                    kind,
                },
                3,
                6,
            ));
        check_function(&function).unwrap();
        for bad in [2, 3, 4] {
            function
                .blocks
                .get_mut(&BasicBlockId(2))
                .unwrap()
                .set_terminator(invoke(
                    Map::InstallValue {
                        map: ValueId(2),
                        key: ValueId(3),
                        value: ValueId(bad),
                        kind,
                    },
                    3,
                    6,
                ));
            assert!(check_function(&function).is_err());
        }
    }
    let mut op = Map::InstallValue {
        map: ValueId(2),
        key: ValueId(3),
        value: ValueId(1),
        kind: MapValueKind::Bool,
    };
    op.rewrite_values(|v| v.0 += 10);
    assert_eq!(
        op.used_values(),
        vec![ValueId(12), ValueId(13), ValueId(11)]
    );
    assert!(!op.effects().is_pure());
    assert!(matches!(
        op,
        Map::InstallValue {
            kind: MapValueKind::Bool,
            ..
        }
    ));
}

#[test]
fn text_install_consumes_the_key_and_carries_no_value_operand() {
    let mut function = graph(true);
    function
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(invoke(
            Map::InstallText {
                map: ValueId(2),
                key: ValueId(3),
                utf8: "owned text".into(),
            },
            3,
            6,
        ));
    check_function(&function).unwrap();
    // The sealed bytes are inline: rewriting ValueIds leaves them
    // untouched, and no payload operand exists to track.
    let mut op = Map::InstallText {
        map: ValueId(2),
        key: ValueId(3),
        utf8: "owned text".into(),
    };
    op.rewrite_values(|v| v.0 += 10);
    assert_eq!(op.used_values(), vec![ValueId(12), ValueId(13)]);
    assert!(matches!(
        &op,
        Map::InstallText { utf8, .. } if utf8 == "owned text"
    ));
    assert!(!op.effects().is_pure());
    // A dead map or a non-MapKey operand still rejects.
    for (map, key) in [(1u32, 3u32), (2, 1), (4, 3)] {
        let mut function = graph(true);
        function
            .blocks
            .get_mut(&BasicBlockId(2))
            .unwrap()
            .set_terminator(invoke(
                Map::InstallText {
                    map: ValueId(map),
                    key: ValueId(key),
                    utf8: "x".into(),
                },
                3,
                6,
            ));
        assert!(check_function(&function).is_err(), "map={map} key={key}");
    }
}

#[test]
fn empty_array_install_consumes_the_key_and_carries_no_operands() {
    let mut function = graph(true);
    function
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(invoke(
            Map::InstallEmptyArray {
                map: ValueId(2),
                key: ValueId(3),
            },
            3,
            6,
        ));
    check_function(&function).unwrap();
    // The marker carries no payload operand: rewriting only moves the
    // map and key ValueIds.
    let mut op = Map::InstallEmptyArray {
        map: ValueId(2),
        key: ValueId(3),
    };
    op.rewrite_values(|v| v.0 += 10);
    assert_eq!(op.used_values(), vec![ValueId(12), ValueId(13)]);
    assert!(!op.effects().is_pure());
    // A dead map or a non-MapKey operand still rejects.
    for (map, key) in [(1u32, 3u32), (2, 1), (4, 3)] {
        let mut function = graph(true);
        function
            .blocks
            .get_mut(&BasicBlockId(2))
            .unwrap()
            .set_terminator(invoke(
                Map::InstallEmptyArray {
                    map: ValueId(map),
                    key: ValueId(key),
                },
                3,
                6,
            ));
        assert!(check_function(&function).is_err(), "map={map} key={key}");
    }
}

/// A caller-owned `%{...}` lease handed to `call` as an argument: the
/// caller builds it, the callee may borrow it, and the caller still Ends
/// it on both landing chains. Whether the edge may borrow at all is the
/// sealed callee-corroboration question this fixture isolates.
fn map_argument_call_function(callee: Callee, result: InvokeCallResultKind) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "map_argument_call".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(MirInstruction::FaultFrameEnter {
        dst: ValueId(0),
        mode: FaultFrameMode::RootOwned,
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId(1),
        value: ConstValue::Integer(30),
    });
    entry.set_terminator(invoke(Map::New, 1, 9));
    let mut origin = BasicBlock::new(BasicBlockId(1));
    origin.add_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: BasicBlockId(0),
        dst: ValueId(2),
    });
    origin.set_terminator(MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call: crate::mir::definitions::MirCall::new(None, callee, vec![ValueId(2)]),
            result,
        },
        fault_frame: ValueId(0),
        normal_landing: BasicBlockId(3),
        fault_landing: BasicBlockId(4),
    });
    let mut normal = BasicBlock::new(BasicBlockId(3));
    if result != InvokeCallResultKind::Unit {
        normal.add_instruction(MirInstruction::InvokeNormalResult {
            invoke_block: BasicBlockId(1),
            dst: ValueId(5),
        });
    }
    normal.set_terminator(invoke(Map::End { map: ValueId(2) }, 5, 6));
    let mut fault = BasicBlock::new(BasicBlockId(4));
    fault.set_terminator(invoke(Map::End { map: ValueId(2) }, 7, 8));
    for block in [entry, origin, normal, fault] {
        function.add_block(block);
    }
    let returned = match result {
        InvokeCallResultKind::Unit => ValueId(1),
        _ => ValueId(5),
    };
    for (id, terminator) in [
        (
            5,
            MirInstruction::Return {
                value: Some(returned),
            },
        ),
        (
            6,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            7,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            8,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            9,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
    ] {
        let mut block = BasicBlock::new(BasicBlockId(id));
        block.set_terminator(terminator);
        function.add_block(block);
    }
    function.update_cfg();
    function
}

#[test]
fn map_argument_borrow_rides_a_corroborated_call_edge() {
    use crate::mir::{MirModule, MirVerifier};
    let callee_key =
        hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method("Worker", "run", 1);
    let mut function = map_argument_call_function(
        Callee::Global(
            hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
                "Worker".into(),
                "run".into(),
                1,
            )
            .unwrap(),
        ),
        InvokeCallResultKind::I64,
    );
    function
        .metadata
        .value_types
        .insert(ValueId(2), MirType::Box("MapBox".into()));
    // The function-only lane has no catalog to consult: it admits the
    // sealed callee shape and leaves membership to the module pass.
    MirVerifier::new().verify_function(&function).unwrap();
    let mut module = MirModule::new("map_argument_handoff".into());
    module.add_function(function);
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: callee_key.mir_symbol_projection(),
            params: vec![MirType::Box("MapBox".into())],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    callee
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Return { value: None });
    module.add_cataloged_box_method(callee_key, callee).unwrap();
    MirVerifier::new().verify_module(&module).unwrap();
}

#[test]
fn map_argument_borrow_rejects_uncorroborated_callees() {
    use crate::mir::{MirModule, MirVerifier};
    // A birth() argument edge corroborates nothing: the constructor's
    // formals sit outside `check_call_edge`, so the lease is an escape,
    // never a borrow — at every layer.
    let function = map_argument_call_function(
        Callee::BirthConstructor {
            key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor(
                "Worker", 1,
            ),
            receiver: ValueId(1),
        },
        InvokeCallResultKind::Unit,
    );
    let errors = MirVerifier::new().verify_function(&function).unwrap_err();
    assert!(
        format!("{errors:?}").contains("map-opaque-escape"),
        "{errors:?}"
    );
    let mut module = MirModule::new("map_argument_escape".into());
    module.add_function(function);
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(
        format!("{errors:?}").contains("map-opaque-escape"),
        "{errors:?}"
    );

    // A sealed-shape callee that never catalogs: the function lane can
    // only prove the shape, but the module pass resolves no definition —
    // an unproven escape, not a borrow.
    let function = map_argument_call_function(
        Callee::Global(
            hakorune_mir_defs::CanonicalGlobalTargetV1::new_static_box_method(
                "Worker".into(),
                "run".into(),
                1,
            )
            .unwrap(),
        ),
        InvokeCallResultKind::I64,
    );
    MirVerifier::new().verify_function(&function).unwrap();
    let mut module = MirModule::new("map_argument_escape".into());
    module.add_function(function);
    let errors = MirVerifier::new().verify_module(&module).unwrap_err();
    assert!(
        format!("{errors:?}").contains("map-opaque-escape"),
        "{errors:?}"
    );

    // Foreign and dynamic callees are outside the sealed edge entirely.
    for callee in [Callee::Extern("foreign".into()), Callee::Value(ValueId(1))] {
        let function = map_argument_call_function(callee, InvokeCallResultKind::I64);
        let errors = MirVerifier::new().verify_function(&function).unwrap_err();
        assert!(
            format!("{errors:?}").contains("map-opaque-escape"),
            "{errors:?}"
        );
    }
}

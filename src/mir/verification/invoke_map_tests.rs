use super::*;
use crate::mir::instruction::{FaultFrameMode, MapInvokeOperation as Map};
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

fn invoke(operation: Map, normal: u32, fault: u32) -> MirInstruction {
    MirInstruction::Invoke {
        operation: InvokeOperation::Map(operation), fault_frame: ValueId(0),
        normal_landing: BasicBlockId(normal), fault_landing: BasicBlockId(fault),
    }
}

fn graph(populated: bool) -> MirFunction {
    let mut function = MirFunction::new(FunctionSignature {
        name: "opaque_map".into(), params: vec![], return_type: MirType::Integer,
        effects: EffectMask::CONTROL,
    }, BasicBlockId(0));
    let mut rows = vec![
        (0, invoke(Map::New, 1, 9)),
        (1, invoke(Map::End { map: ValueId(2) }, 5, 9)),
        (5, MirInstruction::Return { value: Some(ValueId(1)) }),
        (9, MirInstruction::ReturnFault { fault_frame: ValueId(0) }),
    ];
    if populated {
        rows[1].1 = invoke(Map::PrepareKey { utf8: "a\0b".into() }, 2, 6);
        rows.extend([
            (2, invoke(Map::InstallIndexed {
                map: ValueId(2), key: ValueId(3), value: ValueId(1),
                object: hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap(),
            }, 3, 6)),
            (3, invoke(Map::EndOutcome { outcome: ValueId(4) }, 4, 6)),
            (4, invoke(Map::End { map: ValueId(2) }, 5, 9)),
            (6, invoke(Map::End { map: ValueId(2) }, 7, 8)),
            (7, MirInstruction::Jump { target: BasicBlockId(9), edge_args: None }),
            (8, MirInstruction::Jump { target: BasicBlockId(9), edge_args: None }),
        ]);
    }
    for (id, terminal) in rows {
        let mut block = BasicBlock::new(BasicBlockId(id));
        if id == 0 {
            block.add_instruction(MirInstruction::FaultFrameEnter {
                dst: ValueId(0), mode: FaultFrameMode::RootOwned,
            });
            // Physical sort witness only, not source/indexed ownership proof.
            block.add_instruction(MirInstruction::Const { dst: ValueId(1), value: ConstValue::Integer(30) });
        }
        if (1..=3).contains(&id) {
            block.add_instruction(MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(id - 1), dst: ValueId(id + 1),
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
            0 => (5, MirInstruction::Return { value: Some(ValueId(2)) }),
            1 => (4, MirInstruction::Jump { target: BasicBlockId(5), edge_args: None }),
            2 => (3, invoke(Map::EndOutcome { outcome: ValueId(2) }, 4, 6)),
            3 => (4, invoke(Map::End { map: ValueId(1) }, 5, 9)),
            4 => (4, invoke(Map::End { map: ValueId(2) }, 6, 9)),
            5 => (2, MirInstruction::Return { value: Some(ValueId(3)) }),
            _ => {
                function.blocks.get_mut(&BasicBlockId(2)).unwrap().add_instruction(
                    MirInstruction::Const { dst: ValueId(99), value: ConstValue::Integer(0) });
                assert!(check_function(&function).is_err());
                continue;
            }
        };
        function.blocks.get_mut(&BasicBlockId(block)).unwrap().set_terminator(terminal);
        assert!(check_function(&function).is_err(), "mutation {mutation}");
    }
}

#[test]
fn map_operation_rewrite_keeps_key_bytes_object_identity_and_effects() {
    let object = hakorune_mir_defs::CanonicalObjectIdV1::from_declaration_index(0).unwrap();
    let mut operation = InvokeOperation::Map(Map::InstallIndexed {
        map: ValueId(2), key: ValueId(3), object, value: ValueId(1),
    });
    operation.rewrite_values(|v| v.0 += 10);
    assert_eq!(operation.used_values(), vec![ValueId(12), ValueId(13), ValueId(11)]);
    assert!(!operation.effects().is_pure());
    assert!(matches!(operation, InvokeOperation::Map(Map::InstallIndexed { object: actual, .. }) if actual == object));
    let mut key = InvokeOperation::Map(Map::PrepareKey { utf8: "a\0b".into() });
    key.rewrite_values(|_| panic!("key text is not a ValueId"));
    assert_eq!(key, InvokeOperation::Map(Map::PrepareKey { utf8: "a\0b".into() }));
}

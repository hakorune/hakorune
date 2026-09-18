use super::*;
use crate::mir::instruction::{FaultFrameMode, InvokeOperation, MapInvokeOperation as Map};
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirInstruction, MirType,
    ValueId,
};

fn invoke(operation: Map, normal: u32, fault: u32) -> MirInstruction {
    MirInstruction::Invoke {
        operation: InvokeOperation::Map(operation),
        fault_frame: ValueId(0),
        normal_landing: BasicBlockId(normal),
        fault_landing: BasicBlockId(fault),
    }
}

fn array_index_text_view_graph() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "array_index_text_view".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    let blocks = [
        (0, invoke(Map::New, 1, 9)),
        (
            1,
            invoke(
                Map::ArrayIndexMap {
                    map: ValueId(2),
                    utf8: "functions".into(),
                    index: 0,
                },
                2,
                10,
            ),
        ),
        (
            2,
            invoke(
                Map::MapGetText {
                    map: ValueId(3),
                    utf8: "name".into(),
                },
                3,
                11,
            ),
        ),
        (3, invoke(Map::End { map: ValueId(2) }, 4, 12)),
        (
            4,
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
        (10, invoke(Map::End { map: ValueId(2) }, 13, 14)),
        (11, invoke(Map::End { map: ValueId(2) }, 15, 16)),
        (
            12,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            13,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            14,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            15,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
        (
            16,
            MirInstruction::ReturnFault {
                fault_frame: ValueId(0),
            },
        ),
    ];
    for (id, terminator) in blocks {
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
        if (1..=3).contains(&id) {
            block.add_instruction(MirInstruction::InvokeNormalResult {
                invoke_block: BasicBlockId(id - 1),
                dst: ValueId(id + 1),
            });
        }
        block.set_terminator(terminator);
        function.add_block(block);
    }
    function
}

#[test]
fn array_index_map_then_text_is_a_non_escaping_borrowed_view_chain() {
    let function = array_index_text_view_graph();
    check_function(&function).unwrap();
    assert_eq!(
        Map::ArrayIndexMap {
            map: ValueId(2),
            utf8: "functions".into(),
            index: 0,
        }
        .normal_result_kind(),
        Some(crate::mir::instruction::InvokeNormalResultKind::MapView)
    );
    assert_eq!(
        Map::MapGetText {
            map: ValueId(3),
            utf8: "name".into(),
        }
        .normal_result_kind(),
        Some(crate::mir::instruction::InvokeNormalResultKind::TextView)
    );
}

#[test]
fn borrowed_map_view_rejects_nested_index_and_escape() {
    let mut nested = array_index_text_view_graph();
    nested
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .set_terminator(invoke(
            Map::ArrayIndexMap {
                map: ValueId(3),
                utf8: "nested".into(),
                index: 0,
            },
            2,
            9,
        ));
    assert!(check_function(&nested).is_err());

    let mut escaped = array_index_text_view_graph();
    escaped
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId(4),
            edge_args: None,
        });
    assert!(check_function(&escaped).is_err());
}

#[test]
fn map_get_text_requires_a_map_view_operand() {
    let mut function = array_index_text_view_graph();
    function
        .blocks
        .get_mut(&BasicBlockId(2))
        .unwrap()
        .set_terminator(invoke(
            Map::MapGetText {
                map: ValueId(2),
                utf8: "name".into(),
            },
            3,
            9,
        ));
    assert!(check_function(&function).is_err());
}

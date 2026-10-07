//! Independent final carrier correspondence, including original alias chain.
use super::*;
use crate::mir::{BasicBlock, CompareOp, EffectMask, FunctionSignature, MirType};
fn fixture(side: usize, cross: bool) -> (MirFunction, Vec<Binding>, Vec<Binding>, Binding) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "carrier/1".into(),
            params: vec![MirType::Unknown],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    function.params = vec![ValueId(1)];
    let originals = vec![
        (
            BasicBlockId(0),
            MirInstruction::Copy {
                dst: ValueId(2),
                src: ValueId(1),
            },
        ),
        (
            BasicBlockId(0),
            MirInstruction::Copy {
                dst: ValueId(3),
                src: ValueId(2),
            },
        ),
    ];
    for original in &originals {
        function
            .blocks
            .get_mut(&original.0)
            .unwrap()
            .add_instruction(original.1.clone());
    }
    let block = if cross {
        function
            .blocks
            .get_mut(&BasicBlockId(0))
            .unwrap()
            .add_instruction(MirInstruction::Jump {
                target: BasicBlockId(1),
                edge_args: None,
            });
        function.add_block(BasicBlock::new(BasicBlockId(1)));
        BasicBlockId(1)
    } else {
        BasicBlockId(0)
    };
    let copies = if cross {
        vec![(
            block,
            MirInstruction::Copy {
                dst: ValueId(4),
                src: ValueId(3),
            },
        )]
    } else {
        vec![]
    };
    for copy in &copies {
        function
            .blocks
            .get_mut(&block)
            .unwrap()
            .add_instruction(copy.1.clone());
    }
    let operand = if cross { ValueId(4) } else { ValueId(3) };
    let compare = (
        block,
        MirInstruction::Compare {
            dst: ValueId(5),
            op: CompareOp::Gt,
            lhs: if side == 0 { operand } else { ValueId(9) },
            rhs: if side == 1 { operand } else { ValueId(9) },
        },
    );
    function
        .blocks
        .get_mut(&block)
        .unwrap()
        .add_instruction(compare.1.clone());
    (function, originals, copies, compare)
}
#[test]
fn borrowed_carrier_consumer_preserves_alias_chain_both_sides_and_dominating_copy() {
    for side in [0, 1] {
        for cross in [false, true] {
            let (function, originals, copies, compare) = fixture(side, cross);
            check_group(
                &function,
                ValueId(1),
                ValueId(3),
                &originals,
                &copies,
                &compare,
                side,
            )
            .unwrap();
            assert!(check_group(
                &function,
                ValueId(1),
                ValueId(3),
                &originals,
                &copies,
                &compare,
                1 - side
            )
            .is_err());
        }
    }
}
#[test]
fn borrowed_carrier_consumer_rejects_deleted_alias_parameter_and_order_drift() {
    for drift in [0, 1, 2] {
        let (mut function, originals, copies, compare) = fixture(0, false);
        match drift {
            0 => {
                function
                    .blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .instructions
                    .remove(0);
            }
            1 => {
                function.params.push(ValueId(1));
            }
            _ => {
                function
                    .blocks
                    .get_mut(&BasicBlockId(0))
                    .unwrap()
                    .instructions
                    .swap(1, 2);
            }
        }
        assert!(check_group(
            &function,
            ValueId(1),
            ValueId(3),
            &originals,
            &copies,
            &compare,
            0
        )
        .is_err());
    }
}
#[test]
fn borrowed_carrier_consumer_rejects_missing_foreign_and_unreachable_copy() {
    let (function, originals, copies, compare) = fixture(1, true);
    assert!(check_group(
        &function,
        ValueId(1),
        ValueId(3),
        &originals,
        &[],
        &compare,
        1
    )
    .unwrap_err()
    .contains("operand"));
    let mut foreign = copies.clone();
    foreign[0].1 = MirInstruction::Copy {
        dst: ValueId(4),
        src: ValueId(2),
    };
    assert!(check_group(
        &function,
        ValueId(1),
        ValueId(3),
        &originals,
        &foreign,
        &compare,
        1
    )
    .unwrap_err()
    .contains("copy-root"));
    let mut unreachable = function.clone();
    let entry = unreachable.blocks.get_mut(&BasicBlockId(0)).unwrap();
    entry.terminator = None;
    entry.successors.clear();
    entry
        .instructions
        .retain(|row| !matches!(row, MirInstruction::Jump { .. }));
    assert!(check_group(
        &unreachable,
        ValueId(1),
        ValueId(3),
        &originals,
        &copies,
        &compare,
        1
    )
    .unwrap_err()
    .contains("dominance"));
}

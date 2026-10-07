//! Independent physical drift checks for completed Bool reuse.
use super::*;
use crate::mir::{BasicBlock, CompareOp};

fn fixture() -> (MirFunction, Binding, Binding) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("checked_compare_order/0".into());
    let mut function = builder.function_state.current_function.take().unwrap();
    let block = function.entry_block;
    let source = (
        block,
        MirInstruction::Compare {
            dst: ValueId(10),
            op: CompareOp::Gt,
            lhs: ValueId(1),
            rhs: ValueId(2),
        },
    );
    let copy = (
        block,
        MirInstruction::Copy {
            dst: ValueId(11),
            src: ValueId(10),
        },
    );
    function
        .blocks
        .get_mut(&block)
        .unwrap()
        .add_instruction(source.1.clone());
    function
        .blocks
        .get_mut(&block)
        .unwrap()
        .add_instruction(copy.1.clone());
    (function, source, copy)
}

#[test]
fn checked_compare_copy_requires_same_block_definition_order() {
    let (mut function, source, copy) = fixture();
    require_definition_order(&function, &source, &copy).unwrap();
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .instructions
        .swap(0, 1);
    assert_eq!(
        require_definition_order(&function, &source, &copy),
        Err(fault("definition-order"))
    );
}

#[test]
fn checked_compare_copy_requires_reachable_dominating_definition() {
    let (mut function, source, mut copy) = fixture();
    let target = BasicBlockId(1);
    let mut block = BasicBlock::new(target);
    block.add_instruction(copy.1.clone());
    function.add_block(block);
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .instructions
        .pop();
    copy.0 = target;
    assert_eq!(
        require_definition_order(&function, &source, &copy),
        Err(fault("dominance"))
    );
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .add_instruction(MirInstruction::Jump {
            target,
            edge_args: None,
        });
    require_definition_order(&function, &source, &copy).unwrap();
    // A changed CFG must not retain the earlier dominance decision.
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .successors
        .clear();
    assert_eq!(
        require_definition_order(&function, &source, &copy),
        Err(fault("dominance"))
    );
}

#[test]
fn checked_compare_original_definition_rejects_deletion_drift_and_duplicate() {
    let (mut function, source, _) = fixture();
    exact_definition(&function, ValueId(10), &source).unwrap();
    function.blocks.get_mut(&source.0).unwrap().instructions[0] = MirInstruction::Compare {
        dst: ValueId(10),
        op: CompareOp::Lt,
        lhs: ValueId(1),
        rhs: ValueId(2),
    };
    assert_eq!(
        exact_definition(&function, ValueId(10), &source),
        Err(fault("definition-identity"))
    );
    function.blocks.get_mut(&source.0).unwrap().instructions[0] = source.1.clone();
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .add_instruction(source.1.clone());
    assert_eq!(
        exact_definition(&function, ValueId(10), &source),
        Err(fault("definition-identity"))
    );
    function
        .blocks
        .get_mut(&source.0)
        .unwrap()
        .instructions
        .retain(|row| row.dst_value() != Some(ValueId(10)));
    assert_eq!(
        exact_definition(&function, ValueId(10), &source),
        Err(fault("definition-identity"))
    );
}

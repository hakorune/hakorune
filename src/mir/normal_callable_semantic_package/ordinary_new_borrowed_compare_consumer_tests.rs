//! Physical independence: mutations cannot replace source-issued observations.
use super::*;
use crate::mir::{BasicBlock, CompareOp, EffectMask, FunctionSignature, MirType};

fn fixture() -> (MirFunction, Binding, Vec<Binding>, Vec<Binding>) {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "compare_consumer/0".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId(0),
    );
    let id = function.entry_block;
    let original = (
        id,
        MirInstruction::Compare {
            dst: ValueId(10),
            op: CompareOp::Gt,
            lhs: ValueId(1),
            rhs: ValueId(2),
        },
    );
    let copy = (
        id,
        MirInstruction::Copy {
            dst: ValueId(11),
            src: ValueId(10),
        },
    );
    let branch = (
        id,
        MirInstruction::Branch {
            condition: ValueId(11),
            then_bb: BasicBlockId(1),
            else_bb: BasicBlockId(2),
            then_edge_args: None,
            else_edge_args: None,
        },
    );
    let block = function.blocks.get_mut(&id).unwrap();
    block.add_instruction(original.1.clone());
    block.add_instruction(copy.1.clone());
    block.add_instruction(branch.1.clone());
    function.add_block(BasicBlock::new(BasicBlockId(1)));
    function.add_block(BasicBlock::new(BasicBlockId(2)));
    (function, original, vec![copy], vec![branch])
}

#[test]
fn borrowed_compare_consumer_requires_exact_bool_copy_and_real_branch() {
    let (function, original, copies, branches) = fixture();
    check_group(&function, &original, &copies, &branches).unwrap();
    assert!(check_group(&function, &original, &copies, &[])
        .unwrap_err()
        .contains("condition-missing"));
    let mut changed = branches.clone();
    if let MirInstruction::Branch { condition, .. } = &mut changed[0].1 {
        *condition = ValueId(99);
    }
    assert!(check_group(&function, &original, &copies, &changed)
        .unwrap_err()
        .contains("consumer-condition"));
    let mut changed = copies.clone();
    if let MirInstruction::Copy { src, .. } = &mut changed[0].1 {
        *src = ValueId(99);
    }
    assert!(check_group(&function, &original, &changed, &branches)
        .unwrap_err()
        .contains("consumer-copy"));
}

#[test]
fn borrowed_compare_consumer_rejects_final_instruction_deletion_and_order_drift() {
    let (mut function, original, copies, branches) = fixture();
    function
        .blocks
        .get_mut(&original.0)
        .unwrap()
        .instructions
        .swap(0, 1);
    assert!(check_group(&function, &original, &copies, &branches)
        .unwrap_err()
        .contains("consumer-order"));
    function
        .blocks
        .get_mut(&original.0)
        .unwrap()
        .instructions
        .swap(0, 1);
    function
        .blocks
        .get_mut(&original.0)
        .unwrap()
        .instructions
        .remove(0);
    assert!(check_group(&function, &original, &copies, &branches)
        .unwrap_err()
        .contains("consumer-identity"));
}

#[test]
fn borrowed_compare_consumer_rejects_unreachable_branch_and_foreign_terminator() {
    let (mut function, original, copies, branches) = fixture();
    let unreachable = BasicBlockId(3);
    let mut block = BasicBlock::new(unreachable);
    block.add_instruction(branches[0].1.clone());
    function.add_block(block);
    let moved = vec![(unreachable, branches[0].1.clone())];
    assert!(check_group(&function, &original, &copies, &moved)
        .unwrap_err()
        .contains("consumer-dominance"));
    function.blocks.get_mut(&original.0).unwrap().terminator = Some(MirInstruction::Jump {
        target: BasicBlockId(1),
        edge_args: None,
    });
    assert!(check_group(&function, &original, &copies, &branches)
        .unwrap_err()
        .contains("consumer-identity"));
}

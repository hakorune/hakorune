use super::*;
use crate::mir::{BinaryOp, ConstValue};

#[test]
fn test_basic_block_creation() {
    let bb_id = BasicBlockId::new(0);
    let bb = BasicBlock::new(bb_id);

    assert_eq!(bb.id, bb_id);
    assert!(bb.is_empty());
    assert!(!bb.is_terminated());
    assert!(bb.effects.is_pure());
}

#[test]
fn test_instruction_addition() {
    let bb_id = BasicBlockId::new(0);
    let mut bb = BasicBlock::new(bb_id);

    let const_inst = MirInstruction::Const {
        dst: ValueId::new(0),
        value: ConstValue::Integer(42),
    };

    bb.add_instruction(const_inst);

    assert_eq!(bb.instructions.len(), 1);
    assert!(!bb.is_empty());
    assert!(bb.effects.is_pure());
}

#[test]
fn test_terminator_addition() {
    let bb_id = BasicBlockId::new(0);
    let mut bb = BasicBlock::new(bb_id);

    let return_inst = MirInstruction::Return {
        value: Some(ValueId::new(0)),
    };

    bb.add_instruction(return_inst);

    assert!(bb.is_terminated());
    assert!(bb.ends_with_return());
    assert_eq!(bb.instructions.len(), 0);
    assert!(bb.terminator.is_some());
}

#[test]
fn test_branch_successors() {
    let bb_id = BasicBlockId::new(0);
    let mut bb = BasicBlock::new(bb_id);

    let then_bb = BasicBlockId::new(1);
    let else_bb = BasicBlockId::new(2);

    let branch_inst = MirInstruction::Branch {
        condition: ValueId::new(0),
        then_bb,
        else_bb,
        then_edge_args: None,
        else_edge_args: None,
    };

    bb.add_instruction(branch_inst);

    assert_eq!(bb.successors.len(), 2);
    assert!(bb.successors.contains(&then_bb));
    assert!(bb.successors.contains(&else_bb));
}

#[test]
fn test_basic_block_id_generator() {
    let mut gen = BasicBlockIdGenerator::new();

    let bb1 = gen.next();
    let bb2 = gen.next();
    let bb3 = gen.next();

    assert_eq!(bb1, BasicBlockId(0));
    assert_eq!(bb2, BasicBlockId(1));
    assert_eq!(bb3, BasicBlockId(2));

    assert_eq!(gen.peek_next(), BasicBlockId(3));
}

#[test]
fn test_value_tracking() {
    let bb_id = BasicBlockId::new(0);
    let mut bb = BasicBlock::new(bb_id);

    let val1 = ValueId::new(1);
    let val2 = ValueId::new(2);
    let val3 = ValueId::new(3);

    bb.add_instruction(MirInstruction::BinOp {
        dst: val3,
        op: BinaryOp::Add,
        lhs: val1,
        rhs: val2,
    });

    let defined = bb.defined_values();
    let used = bb.used_values();

    assert_eq!(defined, vec![val3]);
    assert_eq!(used, vec![val1, val2]);
}

#[test]
fn test_phi_instruction_ordering() {
    let bb_id = BasicBlockId::new(0);
    let mut bb = BasicBlock::new(bb_id);

    let phi_inst = MirInstruction::Phi {
        dst: ValueId::new(0),
        inputs: vec![(BasicBlockId::new(1), ValueId::new(1))],
        type_hint: None,
    };
    bb.add_instruction(phi_inst);

    let const_inst = MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(42),
    };
    bb.add_instruction(const_inst);

    let phi_count = bb.phi_instructions().count();
    assert_eq!(phi_count, 1);

    let non_phi_count = bb.non_phi_instructions().count();
    assert_eq!(non_phi_count, 1);
}

use super::function_repair::{
    complete_missing_self_carried_phi_inputs, materialize_all_phi_inputs,
};
use super::test_support::test_signature;
use crate::mir::{BasicBlock, BasicBlockId, ConstValue, MirFunction, MirInstruction};

#[test]
fn completes_self_carried_phi_input_for_dominated_backedge() {
    let mut func = MirFunction::new(test_signature("self_carried"), BasicBlockId::new(0));
    func.add_block(BasicBlock::new(BasicBlockId::new(1)));
    func.add_block(BasicBlock::new(BasicBlockId::new(2)));

    let seed = func.next_value_id();
    let phi = func.next_value_id();
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: seed,
            value: ConstValue::Integer(0),
        });
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(1),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(0), seed)],
            type_hint: None,
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(2),
            edge_args: None,
        });
    let body_use = func.next_value_id();
    func.get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .add_instruction(MirInstruction::Copy {
            dst: body_use,
            src: phi,
        });
    func.get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(1),
            edge_args: None,
        });

    let changed = materialize_all_phi_inputs(&mut func, "test").unwrap();
    assert_eq!(changed, 1);

    let header = func.get_block(BasicBlockId::new(1)).unwrap();
    let MirInstruction::Phi { inputs, .. } = &header.instructions[0] else {
        panic!("expected phi");
    };
    assert!(inputs.contains(&(BasicBlockId::new(2), phi)));
}

#[test]
fn does_not_complete_missing_input_for_undominated_merge_pred() {
    let mut func = MirFunction::new(test_signature("merge"), BasicBlockId::new(0));
    func.add_block(BasicBlock::new(BasicBlockId::new(1)));
    func.add_block(BasicBlock::new(BasicBlockId::new(2)));
    func.add_block(BasicBlock::new(BasicBlockId::new(3)));

    let cond = func.next_value_id();
    let branch_local = func.next_value_id();
    let phi = func.next_value_id();
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(0),
        });
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: cond,
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: branch_local,
            value: ConstValue::Integer(1),
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(1), branch_local)],
            type_hint: None,
        });

    let changed = complete_missing_self_carried_phi_inputs(&mut func);
    assert_eq!(changed, 0);

    let merge = func.get_block(BasicBlockId::new(3)).unwrap();
    let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
        panic!("expected phi");
    };
    assert_eq!(inputs.len(), 1);
}

#[test]
fn completes_missing_input_with_single_dominating_existing_incoming() {
    let mut func = MirFunction::new(test_signature("unchanged_edge"), BasicBlockId::new(0));
    func.add_block(BasicBlock::new(BasicBlockId::new(1)));
    func.add_block(BasicBlock::new(BasicBlockId::new(2)));
    func.add_block(BasicBlock::new(BasicBlockId::new(3)));

    let seed = func.next_value_id();
    let phi = func.next_value_id();
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: seed,
            value: ConstValue::Integer(0),
        });
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: seed,
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(1), seed)],
            type_hint: None,
        });

    let changed = complete_missing_self_carried_phi_inputs(&mut func);
    assert_eq!(changed, 1);

    let merge = func.get_block(BasicBlockId::new(3)).unwrap();
    let MirInstruction::Phi { inputs, .. } = &merge.instructions[0] else {
        panic!("expected phi");
    };
    assert!(inputs.contains(&(BasicBlockId::new(2), seed)));
}

#[test]
fn prunes_unused_phi_before_missing_input_validation() {
    let mut func = MirFunction::new(test_signature("unused_phi"), BasicBlockId::new(0));
    func.add_block(BasicBlock::new(BasicBlockId::new(1)));
    func.add_block(BasicBlock::new(BasicBlockId::new(2)));
    func.add_block(BasicBlock::new(BasicBlockId::new(3)));

    let cond = func.next_value_id();
    let branch_local = func.next_value_id();
    let phi = func.next_value_id();
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: cond,
            value: ConstValue::Integer(0),
        });
    func.get_block_mut(BasicBlockId::new(0))
        .unwrap()
        .set_terminator(MirInstruction::Branch {
            condition: cond,
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .add_instruction(MirInstruction::Const {
            dst: branch_local,
            value: ConstValue::Integer(1),
        });
    func.get_block_mut(BasicBlockId::new(1))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(2))
        .unwrap()
        .set_terminator(MirInstruction::Jump {
            target: BasicBlockId::new(3),
            edge_args: None,
        });
    func.get_block_mut(BasicBlockId::new(3))
        .unwrap()
        .add_instruction(MirInstruction::Phi {
            dst: phi,
            inputs: vec![(BasicBlockId::new(1), branch_local)],
            type_hint: None,
        });

    let changed = materialize_all_phi_inputs(&mut func, "test").unwrap();
    assert_eq!(changed, 1);
    assert!(func
        .get_block(BasicBlockId::new(3))
        .unwrap()
        .instructions
        .is_empty());
}

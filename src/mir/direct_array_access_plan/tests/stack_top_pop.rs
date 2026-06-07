use super::*;

#[test]
fn refresh_records_stack_top_pop_load_and_store_as_branchless_proved_unchecked_plans() {
    let mut function = make_function();
    let body_bb = BasicBlockId::new(1);
    let reject_bb = BasicBlockId::new(2);
    function.add_block(BasicBlock::new(body_bb));
    function.add_block(BasicBlock::new(reject_bb));

    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Compare {
        dst: ValueId::new(11),
        op: CompareOp::Eq,
        lhs: ValueId::new(2),
        rhs: ValueId::new(10),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: reject_bb,
        else_bb: body_bb,
        then_edge_args: None,
        else_edge_args: None,
    });

    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Integer(1),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: BinaryOp::Sub,
        lhs: ValueId::new(2),
        rhs: ValueId::new(12),
    });
    body.add_instruction(method_call(Some(14), "ArrayBox", "get", 3, vec![13]));
    body.add_instruction(method_call(Some(15), "ArrayBox", "set", 4, vec![14, 5]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(
        load.bounds_policy(),
        DirectArrayBoundsPolicy::ProvedUnchecked
    );
    assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(load.proof_ids(), &["stack_top_pop"]);
    assert_eq!(load.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
    assert_eq!(load.store_semantics(), DirectArrayStoreSemantics::NotStore);

    let store = &function.metadata.direct_array_access_plans[1];
    assert_eq!(store.op(), DirectArrayAccessOp::Store);
    assert_eq!(
        store.bounds_policy(),
        DirectArrayBoundsPolicy::ProvedUnchecked
    );
    assert_eq!(store.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(store.proof_ids(), &["stack_top_pop"]);
    assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
    assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
    assert_eq!(
        store.store_semantics(),
        DirectArrayStoreSemantics::OverwriteExisting
    );
}

#[test]
fn refresh_records_stack_top_pop_load_after_zero_lt_top_guard() {
    let mut function = make_function();
    let body_bb = BasicBlockId::new(1);
    let miss_bb = BasicBlockId::new(2);
    function.add_block(BasicBlock::new(body_bb));
    function.add_block(BasicBlock::new(miss_bb));

    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Compare {
        dst: ValueId::new(11),
        op: CompareOp::Lt,
        lhs: ValueId::new(10),
        rhs: ValueId::new(2),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: body_bb,
        else_bb: miss_bb,
        then_edge_args: None,
        else_edge_args: None,
    });

    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Integer(1),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: BinaryOp::Sub,
        lhs: ValueId::new(2),
        rhs: ValueId::new(12),
    });
    body.add_instruction(method_call(Some(14), "ArrayBox", "get", 3, vec![13]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(
        load.bounds_policy(),
        DirectArrayBoundsPolicy::ProvedUnchecked
    );
    assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(load.proof_ids(), &["stack_top_pop"]);
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
}

#[test]
fn refresh_reads_branch_instruction_when_terminator_is_absent() {
    let mut function = make_function();
    let body_bb = BasicBlockId::new(1);
    let miss_bb = BasicBlockId::new(2);
    function.add_block(BasicBlock::new(body_bb));
    function.add_block(BasicBlock::new(miss_bb));

    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Compare {
        dst: ValueId::new(11),
        op: CompareOp::Eq,
        lhs: ValueId::new(2),
        rhs: ValueId::new(10),
    });
    entry.instructions.push(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: miss_bb,
        else_bb: body_bb,
        then_edge_args: None,
        else_edge_args: None,
    });

    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Integer(1),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: BinaryOp::Sub,
        lhs: ValueId::new(2),
        rhs: ValueId::new(12),
    });
    body.add_instruction(method_call(Some(14), "ArrayBox", "get", 3, vec![13]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
}

#[test]
fn refresh_records_stack_top_pop_load_from_phi_guard() {
    let mut function = make_function();
    let guard_bb = BasicBlockId::new(1);
    let body_bb = BasicBlockId::new(2);
    let miss_bb = BasicBlockId::new(3);
    function.add_block(BasicBlock::new(guard_bb));
    function.add_block(BasicBlock::new(body_bb));
    function.add_block(BasicBlock::new(miss_bb));

    let guard = function.blocks.get_mut(&guard_bb).expect("guard");
    guard.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(20),
        inputs: vec![
            (BasicBlockId::new(0), ValueId::new(2)),
            (BasicBlockId::new(4), ValueId::new(30)),
        ],
        type_hint: Some(MirType::Integer),
    });
    guard.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(21),
        src: ValueId::new(20),
    });
    guard.add_instruction(MirInstruction::Const {
        dst: ValueId::new(22),
        value: ConstValue::Integer(0),
    });
    guard.add_instruction(MirInstruction::Compare {
        dst: ValueId::new(23),
        op: CompareOp::Eq,
        lhs: ValueId::new(21),
        rhs: ValueId::new(22),
    });
    guard.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(23),
        then_bb: miss_bb,
        else_bb: body_bb,
        then_edge_args: None,
        else_edge_args: None,
    });

    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(24),
        src: ValueId::new(20),
    });
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(25),
        value: ConstValue::Integer(1),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(26),
        op: BinaryOp::Sub,
        lhs: ValueId::new(24),
        rhs: ValueId::new(25),
    });
    body.add_instruction(method_call(Some(27), "ArrayBox", "get", 3, vec![26]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
}

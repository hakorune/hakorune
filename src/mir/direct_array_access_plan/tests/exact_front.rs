use super::*;

#[test]
fn refresh_records_checked_load_and_store_plans_from_array_routes() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![1, 3]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(load.block(), BasicBlockId::new(0));
    assert_eq!(load.instruction_index(), 1);
    assert_eq!(load.receiver_value(), ValueId::new(2));
    assert_eq!(load.index_value(), ValueId::new(1));
    assert_eq!(load.value_value(), None);
    assert_eq!(load.result_value(), Some(ValueId::new(5)));
    assert_eq!(load.bounds_policy(), DirectArrayBoundsPolicy::Checked);
    assert_eq!(load.proof_kind(), DirectArrayProofKind::ExactFrontContract);
    assert_eq!(load.proof_ids(), &["exact_front_contract"]);
    assert_eq!(
        load.fallback_policy(),
        DirectArrayFallbackPolicy::AllowChecked
    );
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
    assert_eq!(load.store_semantics(), DirectArrayStoreSemantics::NotStore);

    let store = &function.metadata.direct_array_access_plans[1];
    assert_eq!(store.op(), DirectArrayAccessOp::Store);
    assert_eq!(store.instruction_index(), 2);
    assert_eq!(store.receiver_value(), ValueId::new(2));
    assert_eq!(store.index_value(), ValueId::new(1));
    assert_eq!(store.value_value(), Some(ValueId::new(3)));
    assert_eq!(store.result_value(), Some(ValueId::new(6)));
    assert_eq!(store.route(), "direct_array_i64_store");
    assert_eq!(store.proof_ids(), &["exact_front_contract"]);
    assert_eq!(store.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
    assert_eq!(
        store.store_semantics(),
        DirectArrayStoreSemantics::AppendOrOverwrite
    );
}

#[test]
fn refresh_records_checked_load_and_store_plans_from_direct_array_i64_routes() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(5), "DirectArrayI64", "get", 2, vec![1]));
    block.add_instruction(method_call(Some(6), "DirectArrayI64", "set", 2, vec![1, 3]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(load.receiver_value(), ValueId::new(2));
    assert_eq!(load.index_value(), ValueId::new(1));
    assert_eq!(load.route(), "direct_array_i64_load");
    assert_eq!(load.bounds_policy(), DirectArrayBoundsPolicy::Checked);
    assert_eq!(load.proof_kind(), DirectArrayProofKind::ExactFrontContract);

    let store = &function.metadata.direct_array_access_plans[1];
    assert_eq!(store.op(), DirectArrayAccessOp::Store);
    assert_eq!(store.receiver_value(), ValueId::new(2));
    assert_eq!(store.index_value(), ValueId::new(1));
    assert_eq!(store.value_value(), Some(ValueId::new(3)));
    assert_eq!(store.route(), "direct_array_i64_store");
    assert_eq!(store.bounds_policy(), DirectArrayBoundsPolicy::Checked);
}

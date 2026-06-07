use super::*;

#[test]
fn refresh_records_release_known_live_stores_as_caller_precondition_plans() {
    let mut function = make_named_function(
        "HakoAllocPageModel.releaseLocalKnownLive/1",
        vec![
            MirType::Box("HakoAllocPageModel".to_string()),
            MirType::Integer,
        ],
    );
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(3),
        base: ValueId::new(0),
        field: "block_used".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(6), "ArrayBox", "set", 3, vec![1, 10]));
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(11),
        base: ValueId::new(0),
        field: "local_free_top".to_string(),
        declared_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(12),
        base: ValueId::new(0),
        field: "local_free".to_string(),
        declared_type: Some(MirType::Box("ArrayBox".to_string())),
    });
    block.add_instruction(method_call(Some(15), "ArrayBox", "set", 12, vec![11, 1]));

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 2);
    for store in &function.metadata.direct_array_access_plans {
        assert_eq!(store.op(), DirectArrayAccessOp::Store);
        assert_eq!(
            store.bounds_policy(),
            DirectArrayBoundsPolicy::ProvedUnchecked
        );
        assert_eq!(store.proof_kind(), DirectArrayProofKind::CallerPrecondition);
        assert_eq!(store.proof_ids(), &["caller_precondition"]);
        assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
        assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
        assert_eq!(
            store.store_semantics(),
            DirectArrayStoreSemantics::OverwriteExisting
        );
    }
}

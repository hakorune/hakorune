use super::*;

#[test]
fn refresh_records_range_index_store_as_branchless_proved_unchecked_plan() {
    let mut function = make_function();
    let body_bb = BasicBlockId::new(1);
    function.add_block(BasicBlock::new(body_bb));
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![4, 3]));
    add_unit_range_loop_fact(&mut function, body_bb);

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    function
        .metadata
        .region_stability_facts
        .push(crate::mir::function::RegionStabilityFact {
            fact_id: 0,
            region_value: ValueId::new(2),
            scope_bb: body_bb,
            proof_kind: crate::mir::function::RegionStabilityProofKind::ProducerInvariant,
            stable_in_region: true,
        });
    function
        .metadata
        .direct_array_extent_facts
        .push(crate::mir::function::DirectArrayExtentFact {
            receiver_value: ValueId::new(2),
            lower_bound_value: ValueId::new(11),
            proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
            region_stability_fact_id: 0,
            stable_in_region: true,
        });
    refresh_function_direct_array_access_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let store = &function.metadata.direct_array_access_plans[0];
    assert_eq!(store.op(), DirectArrayAccessOp::Store);
    assert_eq!(store.block(), body_bb);
    assert_eq!(store.instruction_index(), 0);
    assert_eq!(store.index_value(), ValueId::new(4));
    assert_eq!(
        store.bounds_policy(),
        DirectArrayBoundsPolicy::ProvedUnchecked
    );
    assert_eq!(store.proof_kind(), DirectArrayProofKind::RangeIndex);
    assert_eq!(store.proof_ids(), &["range_index"]);
    assert_eq!(store.fallback_policy(), DirectArrayFallbackPolicy::FailFast);
    assert_eq!(store.cfg_shape(), DirectArrayCfgShape::Branchless);
    assert_eq!(
        store.store_semantics(),
        DirectArrayStoreSemantics::AppendOrOverwrite
    );
}

#[test]
fn refresh_keeps_range_index_store_checked_without_extent_proof() {
    let mut function = make_function();
    let body_bb = BasicBlockId::new(1);
    function.add_block(BasicBlock::new(body_bb));
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    let body = function.blocks.get_mut(&body_bb).expect("body");
    body.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![4, 3]));
    add_unit_range_loop_fact(&mut function, body_bb);

    refresh_direct_array_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let store = &function.metadata.direct_array_access_plans[0];
    assert_eq!(store.bounds_policy(), DirectArrayBoundsPolicy::Checked);
    assert_eq!(store.proof_kind(), DirectArrayProofKind::ExactFrontContract);
    assert_eq!(store.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
}

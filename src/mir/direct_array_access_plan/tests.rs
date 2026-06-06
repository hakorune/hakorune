use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::{DirectArrayExtentProofKind, LoopRangeFact};
use crate::mir::range_index_fact::refresh_function_range_index_facts;
use crate::mir::{
    BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
    MirInstruction, MirType,
};

fn make_function() -> MirFunction {
    make_named_function("main", vec![])
}

fn make_named_function(name: &str, params: Vec<MirType>) -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params,
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn method_call(
    dst: Option<u32>,
    box_name: &str,
    method: &str,
    receiver: u32,
    args: Vec<u32>,
) -> MirInstruction {
    MirInstruction::Call {
        dst: dst.map(ValueId::new),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(ValueId::new(receiver)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: args.into_iter().map(ValueId::new).collect(),
        effects: EffectMask::PURE,
    }
}

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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
    function.metadata.loop_range_facts.push(LoopRangeFact {
        index_name: "i".to_string(),
        start_value: ValueId::new(10),
        end_value: ValueId::new(11),
        index_phi: ValueId::new(4),
        preheader_bb: BasicBlockId::new(0),
        header_bb: BasicBlockId::new(2),
        body_bb,
        step_bb: BasicBlockId::new(3),
        exit_bb: BasicBlockId::new(4),
        step: 1,
        end_exclusive: true,
        index_read_only: true,
        body_local_writes_supported: true,
        loop_carried_writes_supported: false,
        body_writes_supported: false,
    });

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
    function.metadata.loop_range_facts.push(LoopRangeFact {
        index_name: "i".to_string(),
        start_value: ValueId::new(10),
        end_value: ValueId::new(11),
        index_phi: ValueId::new(4),
        preheader_bb: BasicBlockId::new(0),
        header_bb: BasicBlockId::new(2),
        body_bb,
        step_bb: BasicBlockId::new(3),
        exit_bb: BasicBlockId::new(4),
        step: 1,
        end_exclusive: true,
        index_read_only: true,
        body_local_writes_supported: true,
        loop_carried_writes_supported: false,
        body_writes_supported: false,
    });

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let store = &function.metadata.direct_array_access_plans[0];
    assert_eq!(store.bounds_policy(), DirectArrayBoundsPolicy::Checked);
    assert_eq!(store.proof_kind(), DirectArrayProofKind::ExactFrontContract);
    assert_eq!(store.cfg_shape(), DirectArrayCfgShape::CheckedBranching);
}

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

    assert_eq!(function.metadata.direct_array_access_plans.len(), 1);
    let load = &function.metadata.direct_array_access_plans[0];
    assert_eq!(load.op(), DirectArrayAccessOp::Load);
    assert_eq!(load.proof_kind(), DirectArrayProofKind::StackTopPop);
    assert_eq!(load.cfg_shape(), DirectArrayCfgShape::Branchless);
}

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

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_range_index_facts(&mut function);
    refresh_function_direct_array_access_plans(&mut function);

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

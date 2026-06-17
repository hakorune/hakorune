use super::fixtures::{make_function, method_call};
use crate::mir::map_repr_plan::refresh_function_map_repr_plans;
use crate::mir::{BasicBlock, BasicBlockId, BinaryOp, ConstValue, MirInstruction, ValueId};
use crate::object_storage_plan::{ObjectBasicBlockId, ObjectInstructionIndex, ObjectValueId};

#[test]
fn refresh_function_map_repr_plans_emits_generic_hash_runtime_rows() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(-1),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(7),
    });
    block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_map_repr_plans(&mut function);

    assert_eq!(function.metadata.map_repr_plans.len(), 1);
    let plan = &function.metadata.map_repr_plans[0];
    assert_eq!(plan.route_id(), "map_repr.generic_hash_runtime");
    assert_eq!(plan.repr_kind_tag(), "generic_hash_runtime");
    assert_eq!(plan.source_route_id(), "generic_method.set");
    assert_eq!(plan.surface_box_name(), "MapBox");
    assert_eq!(plan.receiver_origin_box(), Some("MapBox"));
    assert_eq!(plan.method(), "set");
    assert_eq!(plan.receiver_value(), ValueId::new(1));
    assert_eq!(plan.key_route_tag(), Some("i64_const"));
    assert_eq!(plan.value_demand_tag(), "write_any");
    assert_eq!(plan.proof_tag(), "set_surface_policy");
}

#[test]
fn refresh_function_map_repr_plans_emits_local_i64_key_map_shadow_rows() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(2),
    });
    entry.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
    entry.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![3, 4]));
    entry.add_instruction(method_call(Some(7), "MapBox", "set", 1, vec![4, 2]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.successors.insert(body_id);
    body.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(10),
        inputs: vec![(entry_id, ValueId::new(2)), (body_id, ValueId::new(13))],
        type_hint: None,
    });
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(3),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(12),
        op: BinaryOp::Mod,
        lhs: ValueId::new(10),
        rhs: ValueId::new(11),
    });
    body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![12]));
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: BinaryOp::Add,
        lhs: ValueId::new(10),
        rhs: ValueId::new(3),
    });
    function.add_block(body);

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_map_repr_plans(&mut function);

    let shadow_plans: Vec<_> = function
        .metadata
        .map_repr_plans
        .iter()
        .filter(|plan| plan.repr_kind_tag() == "local_i64_key_map_shadow")
        .collect();
    assert_eq!(shadow_plans.len(), 4);
    assert!(shadow_plans
        .iter()
        .any(|plan| plan.source_route_kind() == "map_load_scalar_i64"));
    assert!(shadow_plans.iter().all(|plan| {
        plan.receiver_value() == ValueId::new(1) && plan.proof_tag() == "local_i64_key_map_shadow"
    }));
    let storage_plans = &function.metadata.local_map_storage_realization_plans;
    assert_eq!(storage_plans.len(), 1);
    let storage_plan = &storage_plans[0];
    assert_eq!(storage_plan.receiver_value(), ValueId::new(1));
    assert_eq!(storage_plan.representation(), "local_i64_key_map");
    assert_eq!(storage_plan.candidate_set_count(), 3);
    assert_eq!(storage_plan.candidate_scalar_get_count(), 1);
    assert!(storage_plan.publication_materialization_required());
    assert!(!storage_plan.backend_lowering_enabled());
    assert!(!storage_plan.runtime_helper_enabled());
    let direct_storage_plans = &function.metadata.local_i64_map_direct_storage_plans;
    assert_eq!(direct_storage_plans.len(), 1);
    let direct_storage_plan = &direct_storage_plans[0];
    assert_eq!(direct_storage_plan.receiver_value(), ValueId::new(1));
    assert_eq!(
        direct_storage_plan.representation(),
        "closed_world_i64_key_value_table"
    );
    assert_eq!(direct_storage_plan.known_i64_key_set_count(), 3);
    assert_eq!(direct_storage_plan.scalar_get_count(), 1);
    assert!(!direct_storage_plan.entry_value_tracking_enabled());
    assert!(direct_storage_plan.publication_materialization_required());
    assert!(!direct_storage_plan.backend_lowering_enabled());
    assert!(!direct_storage_plan.runtime_helper_enabled());
    let entry_plans = &function.metadata.local_i64_map_entry_value_tracking_plans;
    assert_eq!(entry_plans.len(), 3);
    let first_entry = &entry_plans[0];
    assert_eq!(first_entry.receiver_value(), ValueId::new(1));
    assert_eq!(first_entry.set_block(), BasicBlockId::new(0));
    assert_eq!(first_entry.set_instruction_index(), 4);
    assert_eq!(first_entry.key_value(), ValueId::new(2));
    assert_eq!(first_entry.value_value(), ValueId::new(3));
    assert_eq!(first_entry.key_const_if_known(), Some(0));
    assert_eq!(first_entry.value_const_if_known(), Some(1));
    assert!(!first_entry.backend_lowering_enabled());
    assert!(!first_entry.runtime_helper_enabled());
}

#[test]
fn refresh_function_map_repr_plans_joins_set_receiver_alias_and_later_public_read() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let exit_id = BasicBlockId::new(2);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(12),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(13),
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(14),
        src: ValueId::new(3),
    });
    entry.add_instruction(method_call(Some(15), "MapBox", "set", 14, vec![3, 12, 13]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.successors.insert(exit_id);
    body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 3, vec![12]));
    function.add_block(body);

    let mut exit = BasicBlock::new(exit_id);
    exit.predecessors.insert(body_id);
    exit.add_instruction(method_call(Some(30), "MapBox", "get", 3, vec![12]));
    function.add_block(exit);

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_map_repr_plans(&mut function);

    let direct_storage_plans = &function.metadata.local_i64_map_direct_storage_plans;
    let direct_storage_plan = direct_storage_plans
        .iter()
        .find(|plan| plan.receiver_value() == ValueId::new(3))
        .expect("direct storage plan for canonical receiver");
    assert_eq!(direct_storage_plan.receiver_value(), ValueId::new(3));
    assert_eq!(direct_storage_plan.known_i64_key_set_count(), 1);
    assert!(direct_storage_plan.scalar_get_count() >= 1);

    let entry_plans = &function.metadata.local_i64_map_entry_value_tracking_plans;
    let entry = entry_plans
        .iter()
        .find(|plan| plan.receiver_value() == ValueId::new(3))
        .expect("entry tracking plan for canonical receiver");
    assert_eq!(entry.receiver_value(), ValueId::new(3));
    assert_eq!(entry.key_value(), ValueId::new(12));
    assert_eq!(entry.value_value(), ValueId::new(13));
    assert_eq!(entry.key_const_if_known(), Some(0));
    assert_eq!(entry.value_const_if_known(), Some(1));
}

#[test]
fn refresh_function_map_repr_plans_emits_local_fastpath_facts_for_scalar_no_publication_get() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.successors.insert(body_id);
    body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![2]));
    function.add_block(body);

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    refresh_function_map_repr_plans(&mut function);

    let facts = &function.metadata.local_fastpath_facts;
    assert_eq!(facts.len(), 1);
    let source_plan = function
        .metadata
        .map_repr_plans
        .iter()
        .find(|plan| {
            plan.source_route_kind() == "map_load_scalar_i64"
                && plan.publication_policy_tag() == Some("no_publication")
                && plan.return_shape_tag() == Some("scalar_i64_or_missing_zero")
        })
        .expect("scalar no-publication map repr plan");
    assert_eq!(source_plan.route_id(), "map_repr.generic_hash_runtime");
    let fact = &facts[0];
    assert_eq!(fact.object_id, ObjectValueId(1));
    assert_eq!(fact.block_id(), ObjectBasicBlockId(1));
    assert_eq!(fact.instruction_index(), ObjectInstructionIndex(0));
    assert!(fact.valid_until_publication);
}

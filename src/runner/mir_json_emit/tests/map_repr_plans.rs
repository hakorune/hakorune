use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlock, BasicBlockId, BinaryOp, Callee, ConstValue, EffectMask, MirInstruction, MirModule,
    ValueId,
};
use crate::object_storage_plan::{
    AliasClassId, LocalFastPathFact, LocalFastPathSiteId, ObjectBasicBlockId,
    ObjectInstructionIndex, ObjectStoragePlanId, ObjectValueId, RoutePlanId,
};

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
fn build_mir_json_root_emits_local_fastpath_facts() {
    let mut function = make_function("main", true);
    function
        .metadata
        .local_fastpath_facts
        .push(LocalFastPathFact::known_receiver_direct_call(
            LocalFastPathSiteId(8),
            ObjectBasicBlockId(5),
            ObjectInstructionIndex(9),
            ObjectValueId(20),
            AliasClassId(3),
            RoutePlanId(4),
            ObjectStoragePlanId(5),
        ));

    let mut module = MirModule::new("json_local_fastpath_fact_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["local_fastpath_facts"]
        .as_array()
        .expect("local_fastpath_facts");
    assert_eq!(facts.len(), 1);
    let fact = &facts[0];
    assert_eq!(
        fact["route_id"],
        "local_fastpath.known_receiver_direct_call"
    );
    assert_eq!(fact["fact_kind"], "local_fastpath_fact");
    assert_eq!(fact["backend_kind"], "known_receiver_direct_call");
    assert_eq!(fact["route_plan"], "map_repr.generic_hash_runtime");
    assert_eq!(fact["site_id"], 8);
    assert_eq!(fact["block"], 5);
    assert_eq!(fact["instruction_index"], 9);
    assert_eq!(fact["receiver_value"], 20);
    assert_eq!(fact["alias_class"], 3);
    assert!(fact["fallback_reason"].is_null());
}

#[test]
fn build_mir_json_root_emits_map_repr_plans() {
    let mut function = make_function("main", true);
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
    crate::mir::map_repr_plan::refresh_function_map_repr_plans(&mut function);

    let mut module = MirModule::new("json_map_repr_plan_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["map_repr_plans"]
        .as_array()
        .expect("map_repr_plans");
    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan["route_id"], "map_repr.generic_hash_runtime");
    assert_eq!(plan["repr_kind"], "generic_hash_runtime");
    assert_eq!(plan["source_route_id"], "generic_method.set");
    assert_eq!(plan["source_route_kind"], "map_store_any");
    assert_eq!(plan["source_helper_symbol"], "nyash.map.slot_store_hhh");
    assert_eq!(plan["block"], 0);
    assert_eq!(plan["instruction_index"], 3);
    assert_eq!(plan["surface_box_name"], "MapBox");
    assert_eq!(plan["receiver_origin_box"], "MapBox");
    assert_eq!(plan["method"], "set");
    assert_eq!(plan["receiver_value"], 1);
    assert_eq!(plan["key_value"], 2);
    assert_eq!(plan["result_value"], 4);
    assert_eq!(plan["key_route"], "i64_const");
    assert_eq!(plan["value_demand"], "write_any");
    assert_eq!(plan["proof_tag"], "set_surface_policy");
}

#[test]
fn build_mir_json_root_emits_local_map_storage_realization_plans() {
    let mut function = make_function("main", true);
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
    crate::mir::map_repr_plan::refresh_function_map_repr_plans(&mut function);

    let mut module = MirModule::new("json_local_map_storage_plan_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["local_map_storage_realization_plans"]
        .as_array()
        .expect("local_map_storage_realization_plans");
    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan["receiver_value"], 1);
    assert_eq!(plan["representation"], "local_i64_key_map");
    assert_eq!(plan["candidate_set_count"], 3);
    assert_eq!(plan["candidate_scalar_get_count"], 1);
    assert_eq!(plan["publication_materialization_required"], true);
    assert_eq!(plan["backend_lowering_enabled"], false);
    assert_eq!(plan["runtime_helper_enabled"], false);
}

#[test]
fn build_mir_json_root_emits_local_i64_map_direct_storage_plans() {
    let mut function = make_function("main", true);
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
    crate::mir::map_repr_plan::refresh_function_map_repr_plans(&mut function);

    let mut module = MirModule::new("json_local_i64_map_direct_storage_plan_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["local_i64_map_direct_storage_plans"]
        .as_array()
        .expect("local_i64_map_direct_storage_plans");
    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan["receiver_value"], 1);
    assert_eq!(plan["representation"], "closed_world_i64_key_value_table");
    assert_eq!(plan["known_i64_key_set_count"], 3);
    assert_eq!(plan["scalar_get_count"], 1);
    assert_eq!(plan["entry_value_tracking_enabled"], false);
    assert_eq!(plan["publication_materialization_required"], true);
    assert_eq!(plan["backend_lowering_enabled"], false);
    assert_eq!(plan["runtime_helper_enabled"], false);
}

#[test]
fn build_mir_json_root_emits_local_i64_map_entry_value_tracking_plans() {
    let mut function = make_function("main", true);
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
    crate::mir::map_repr_plan::refresh_function_map_repr_plans(&mut function);

    let mut module = MirModule::new("json_local_i64_map_entry_value_tracking_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["local_i64_map_entry_value_tracking_plans"]
        .as_array()
        .expect("local_i64_map_entry_value_tracking_plans");
    assert_eq!(plans.len(), 3);
    let plan = &plans[0];
    assert_eq!(plan["receiver_value"], 1);
    assert_eq!(plan["set_block"], 0);
    assert_eq!(plan["set_instruction_index"], 4);
    assert_eq!(plan["key_value"], 2);
    assert_eq!(plan["value_value"], 3);
    assert_eq!(plan["key_const_if_known"], 0);
    assert_eq!(plan["value_const_if_known"], 1);
    assert_eq!(plan["backend_lowering_enabled"], false);
    assert_eq!(plan["runtime_helper_enabled"], false);
}

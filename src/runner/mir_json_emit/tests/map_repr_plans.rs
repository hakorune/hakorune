use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, EffectMask, MirInstruction, MirModule, ValueId,
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

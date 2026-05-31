use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{BasicBlockId, Callee, EffectMask, MirInstruction, MirModule, ValueId};

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
fn build_mir_json_root_emits_direct_array_access_plans() {
    let mut function = make_function("main", true);
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(5), "ArrayBox", "get", 2, vec![1]));
    block.add_instruction(method_call(Some(6), "ArrayBox", "set", 2, vec![1, 3]));

    crate::mir::generic_method_route_plan::refresh_function_generic_method_routes(&mut function);
    crate::mir::direct_array_access_plan::refresh_function_direct_array_access_plans(&mut function);

    let mut module = MirModule::new("json_direct_array_access_plan_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["direct_array_access_plans"]
        .as_array()
        .expect("direct_array_access_plans");
    assert_eq!(plans.len(), 2);

    let load = &plans[0];
    assert_eq!(load["route_id"], "direct_array.access");
    assert_eq!(load["op"], "load");
    assert_eq!(load["block"], 0);
    assert_eq!(load["instruction_index"], 0);
    assert_eq!(load["receiver_value"], 2);
    assert_eq!(load["index_value"], 1);
    assert_eq!(load["result_value"], 5);
    assert_eq!(load["array_kind"], "DirectArrayI64");
    assert_eq!(load["element_type"], "i64");
    assert_eq!(load["route"], "direct_array_i64_load");
    assert_eq!(load["bounds_policy"], "checked");
    assert_eq!(load["proof_kind"], "exact_front_contract");
    assert_eq!(load["fallback_policy"], "allow_checked");
    assert_eq!(load["cfg_shape"], "checked_branching");
    assert_eq!(load["store_semantics"], "not_store");

    let store = &plans[1];
    assert_eq!(store["op"], "store");
    assert_eq!(store["instruction_index"], 1);
    assert_eq!(store["receiver_value"], 2);
    assert_eq!(store["index_value"], 1);
    assert_eq!(store["value_value"], 3);
    assert_eq!(store["result_value"], 6);
    assert_eq!(store["route"], "direct_array_i64_store");
    assert_eq!(store["cfg_shape"], "checked_branching");
    assert_eq!(store["store_semantics"], "append_or_overwrite");
}

use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::{
    CountingLoopFact, DirectArrayExtentFact, DirectArrayExtentProofKind, LoopRangeFact,
};
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
fn build_mir_json_root_emits_range_index_facts() {
    let mut function = make_function("main", true);
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: ConstValue::Integer(0),
    });
    function.metadata.loop_range_facts.push(LoopRangeFact {
        index_name: "i".to_string(),
        start_value: ValueId::new(10),
        end_value: ValueId::new(11),
        index_phi: ValueId::new(4),
        preheader_bb: BasicBlockId::new(0),
        header_bb: BasicBlockId::new(2),
        body_bb: BasicBlockId::new(1),
        step_bb: BasicBlockId::new(3),
        exit_bb: BasicBlockId::new(4),
        step: 1,
        end_exclusive: true,
        index_read_only: true,
        body_local_writes_supported: true,
        loop_carried_writes_supported: false,
        body_writes_supported: false,
    });
    crate::mir::range_index_fact::refresh_function_range_index_facts(&mut function);

    let mut module = MirModule::new("json_range_index_fact_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["range_index_facts"]
        .as_array()
        .expect("range_index_facts");
    assert_eq!(facts.len(), 1);
    let fact = &facts[0];
    assert_eq!(fact["fact_id"], 0);
    assert_eq!(fact["origin_kind"], "range_loop");
    assert_eq!(fact["index_value"], 4);
    assert_eq!(fact["lower_value"], 10);
    assert_eq!(fact["upper_exclusive_value"], 11);
    assert_eq!(fact["body_bb"], 1);
    assert_eq!(fact["step"], 1);
    assert_eq!(fact["end_exclusive"], true);
    assert_eq!(fact["index_body_read_only"], true);
    assert_eq!(fact["loop_carried_writes_supported"], false);
}

#[test]
fn build_mir_json_root_emits_counting_loop_facts() {
    let mut function = make_function("main", true);
    function
        .metadata
        .counting_loop_facts
        .push(CountingLoopFact {
            index_name: "i".to_string(),
            lower_value: ValueId::new(10),
            upper_exclusive_value: ValueId::new(11),
            index_value: ValueId::new(4),
            preheader_bb: BasicBlockId::new(0),
            header_bb: BasicBlockId::new(2),
            body_bb: BasicBlockId::new(1),
            latch_bb: BasicBlockId::new(3),
            exit_bb: BasicBlockId::new(4),
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        });
    crate::mir::range_index_fact::refresh_function_range_index_facts(&mut function);

    let mut module = MirModule::new("json_counting_loop_fact_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let producer_facts = root["functions"][0]["metadata"]["counting_loop_facts"]
        .as_array()
        .expect("counting_loop_facts");
    assert_eq!(producer_facts.len(), 1);
    assert_eq!(producer_facts[0]["index_name"], "i");
    assert_eq!(producer_facts[0]["lower_value"], 10);
    assert_eq!(producer_facts[0]["upper_exclusive_value"], 11);

    let range_facts = root["functions"][0]["metadata"]["range_index_facts"]
        .as_array()
        .expect("range_index_facts");
    assert_eq!(range_facts.len(), 1);
    assert_eq!(range_facts[0]["origin_kind"], "counting_loop");
}

#[test]
fn build_mir_json_root_emits_direct_array_extent_facts() {
    let mut function = make_function("main", true);
    function
        .metadata
        .direct_array_extent_facts
        .push(DirectArrayExtentFact {
            receiver_value: ValueId::new(2),
            lower_bound_value: ValueId::new(11),
            proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
            stable_in_region: true,
        });

    let mut module = MirModule::new("json_direct_array_extent_fact_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["direct_array_extent_facts"]
        .as_array()
        .expect("direct_array_extent_facts");
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0]["receiver_value"], 2);
    assert_eq!(facts[0]["lower_bound_value"], 11);
    assert_eq!(facts[0]["proof_kind"], "producer_invariant");
    assert_eq!(facts[0]["stable_in_region"], true);
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

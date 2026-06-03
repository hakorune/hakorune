use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::function::{
    CountingLoopFact, DirectArrayExtentFact, DirectArrayExtentProofKind, FastPathObligation,
    LoopRangeFact, RegionStabilityFact, RegionStabilityProofKind, RequiredFastPathRegion,
    SpanAccessOp, SpanAccessPlan, SpanBorrowFact, SpanBorrowMutability, SpanElementType,
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
        .region_stability_facts
        .push(RegionStabilityFact {
            fact_id: 0,
            region_value: ValueId::new(2),
            scope_bb: BasicBlockId::new(1),
            proof_kind: RegionStabilityProofKind::ProducerInvariant,
            stable_in_region: true,
        });
    function
        .metadata
        .direct_array_extent_facts
        .push(DirectArrayExtentFact {
            receiver_value: ValueId::new(2),
            lower_bound_value: ValueId::new(11),
            proof_kind: DirectArrayExtentProofKind::ProducerInvariant,
            region_stability_fact_id: 0,
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
    assert_eq!(facts[0]["region_stability_fact_id"], 0);
    assert_eq!(facts[0]["stable_in_region"], true);

    let stability_facts = root["functions"][0]["metadata"]["region_stability_facts"]
        .as_array()
        .expect("region_stability_facts");
    assert_eq!(stability_facts.len(), 1);
    assert_eq!(stability_facts[0]["fact_id"], 0);
    assert_eq!(stability_facts[0]["region_value"], 2);
    assert_eq!(stability_facts[0]["scope_bb"], 1);
    assert_eq!(stability_facts[0]["proof_kind"], "producer_invariant");
    assert_eq!(stability_facts[0]["stable_in_region"], true);
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
    crate::mir::route_decision::refresh_function_route_decisions(&mut function);

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
    assert_eq!(
        load["proof_ids"],
        serde_json::json!(["exact_front_contract"])
    );
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
    assert_eq!(
        store["proof_ids"],
        serde_json::json!(["exact_front_contract"])
    );

    let decisions = root["functions"][0]["metadata"]["route_decisions"]
        .as_array()
        .expect("route_decisions");
    assert_eq!(decisions.len(), 2);

    let load_decision = &decisions[0];
    assert_eq!(load_decision["route_id"], "route.decision");
    assert_eq!(load_decision["site_id"], "b0.i0");
    assert_eq!(load_decision["semantic_op"], "ArrayGet");
    assert_eq!(load_decision["access_kind"], "direct_array_i64_load");
    assert_eq!(load_decision["preferred_route"], "direct_array_i64_load");
    assert_eq!(load_decision["selected_route"], "direct_array_i64_load");
    assert_eq!(load_decision["fallback_route"], "generic_array_get_helper");
    assert_eq!(load_decision["fallback_policy"], "opportunistic");
    assert_eq!(
        load_decision["proof_ids"],
        serde_json::json!(["exact_front_contract"])
    );
    assert_eq!(load_decision["miss_reason"], serde_json::Value::Null);
    assert_eq!(load_decision["source_plan_kind"], "DirectArrayAccessPlan");
    assert_eq!(store["cfg_shape"], "checked_branching");
    assert_eq!(store["store_semantics"], "append_or_overwrite");
}

#[test]
fn build_mir_json_root_emits_span_borrow_facts() {
    let mut function = make_function("main", true);
    function.metadata.span_borrow_facts.push(SpanBorrowFact {
        span_id: 0,
        span_value: ValueId::new(2),
        region_value: ValueId::new(2),
        owner_value: ValueId::new(2),
        mutability: SpanBorrowMutability::Write,
        element_type: SpanElementType::I64,
        start_value: ValueId::new(10),
        length_value: ValueId::new(11),
        scope_bb: BasicBlockId::new(1),
        no_escape: true,
        owner_stable: true,
        region_stability_fact_id: 0,
    });

    let mut module = MirModule::new("json_span_borrow_fact_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let facts = root["functions"][0]["metadata"]["span_borrow_facts"]
        .as_array()
        .expect("span_borrow_facts");
    assert_eq!(facts.len(), 1);
    let fact = &facts[0];
    assert_eq!(fact["span_id"], 0);
    assert_eq!(fact["span_value"], 2);
    assert_eq!(fact["region_value"], 2);
    assert_eq!(fact["owner_value"], 2);
    assert_eq!(fact["mutability"], "write");
    assert_eq!(fact["element_type"], "i64");
    assert_eq!(fact["start_value"], 10);
    assert_eq!(fact["length_value"], 11);
    assert_eq!(fact["scope_bb"], 1);
    assert_eq!(fact["no_escape"], true);
    assert_eq!(fact["owner_stable"], true);
    assert_eq!(fact["region_stability_fact_id"], 0);
}

#[test]
fn build_mir_json_root_emits_span_access_plans() {
    let mut function = make_function("main", true);
    function.metadata.span_access_plans.push(SpanAccessPlan {
        block: BasicBlockId::new(1),
        instruction_index: 2,
        span_id: 0,
        op: SpanAccessOp::Store,
        index_value: ValueId::new(4),
        value_value: Some(ValueId::new(5)),
        result_value: None,
        element_type: SpanElementType::I64,
        route: "span_i64_store",
        bounds_policy: "proved_unchecked",
        proof_ids: vec!["range_index", "direct_array_extent", "region_stability"],
        fallback_policy: "fail_fast",
    });

    let mut module = MirModule::new("json_span_access_plan_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plans = root["functions"][0]["metadata"]["span_access_plans"]
        .as_array()
        .expect("span_access_plans");
    assert_eq!(plans.len(), 1);
    let plan = &plans[0];
    assert_eq!(plan["span_id"], 0);
    assert_eq!(plan["block"], 1);
    assert_eq!(plan["instruction_index"], 2);
    assert_eq!(plan["op"], "store");
    assert_eq!(plan["index_value"], 4);
    assert_eq!(plan["value_value"], 5);
    assert_eq!(plan["result_value"], serde_json::Value::Null);
    assert_eq!(plan["element_type"], "i64");
    assert_eq!(plan["route"], "span_i64_store");
    assert_eq!(plan["bounds_policy"], "proved_unchecked");
    assert_eq!(
        plan["proof_ids"],
        serde_json::json!(["range_index", "direct_array_extent", "region_stability"])
    );
    assert_eq!(plan["fallback_policy"], "fail_fast");
}

#[test]
fn build_mir_json_root_emits_required_fastpath_diagnostics() {
    let mut function = make_function("main", true);
    function
        .metadata
        .required_fastpath_regions
        .push(RequiredFastPathRegion {
            region_id: 0,
            source_kind: "diagnostic_mode",
            relevant_access_policy: "direct_memory",
            route_requirement: "fastpath_plan_required",
            bounds_requirement: "checked_allowed",
            fallback_policy: "fail_fast",
        });
    function
        .metadata
        .fastpath_obligations
        .push(FastPathObligation {
            obligation_id: 0,
            region_id: 0,
            block: BasicBlockId::new(1),
            instruction_index: 2,
            access_kind: "direct_array_i64",
            op: "store",
            expected: "FastPathPlanRequired",
            actual_plan_kind: None,
            actual_route: None,
            bounds_policy: None,
            proof_ids: vec![],
            status: "failed",
            failure_code: Some("DM006001"),
            failure_reason: Some("missing_fastpath_plan"),
        });

    let mut module = MirModule::new("json_fastpath_diagnostic_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let regions = root["functions"][0]["metadata"]["required_fastpath_regions"]
        .as_array()
        .expect("required_fastpath_regions");
    assert_eq!(regions.len(), 1);
    assert_eq!(regions[0]["source_kind"], "diagnostic_mode");
    assert_eq!(regions[0]["route_requirement"], "fastpath_plan_required");

    let obligations = root["functions"][0]["metadata"]["fastpath_obligations"]
        .as_array()
        .expect("fastpath_obligations");
    assert_eq!(obligations.len(), 1);
    let obligation = &obligations[0];
    assert_eq!(obligation["access_kind"], "direct_array_i64");
    assert_eq!(obligation["op"], "store");
    assert_eq!(obligation["expected"], "FastPathPlanRequired");
    assert_eq!(obligation["status"], "failed");
    assert_eq!(obligation["failure_code"], "DM006001");
    assert_eq!(obligation["failure_reason"], "missing_fastpath_plan");
}

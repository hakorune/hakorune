use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::extern_call_route_plan::{
    refresh_function_extern_call_routes, ExternCallRoute, ExternCallRouteKind, ExternCallRouteSite,
};
use crate::mir::{BasicBlockId, ValueId};

#[test]
fn build_mir_json_root_emits_extern_call_routes_and_lowering_plan() {
    let mut function = make_function("main", true);
    function
        .metadata
        .extern_call_routes
        .push(ExternCallRoute::new(
            ExternCallRouteSite::new(BasicBlockId::new(0), 3),
            ExternCallRouteKind::EnvGet,
            "env.get/1",
            ValueId::new(49),
            None,
            ValueId::new(48),
        ));
    let mut module = crate::mir::MirModule::new("json_extern_call_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["extern_call_routes"][0];
    assert_eq!(route["route_id"], "extern.env.get");
    assert_eq!(route["block"], 0);
    assert_eq!(route["instruction_index"], 3);
    assert_eq!(route["source_symbol"], "env.get/1");
    assert_eq!(route["core_op"], "EnvGet");
    assert_eq!(route["tier"], "ColdRuntime");
    assert_eq!(route["emit_kind"], "runtime_call");
    assert_eq!(route["symbol"], "nyash.env.get");
    assert_eq!(route["proof"], "extern_registry");
    assert_eq!(route["key_value"], 49);
    assert_eq!(route["result_value"], 48);
    assert_eq!(route["return_shape"], "string_handle_or_null");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["effects"], serde_json::json!(["read.env"]));

    let lowering_plan = root["functions"][0]["metadata"]["lowering_plan"]
        .as_array()
        .expect("lowering_plan");
    assert_eq!(lowering_plan.len(), 1);
    let plan = &lowering_plan[0];
    assert_eq!(plan["site"], "b0.i3");
    assert_eq!(plan["source"], "extern_call_routes");
    assert_eq!(plan["source_route_id"], "extern.env.get");
    assert_eq!(plan["source_symbol"], "env.get/1");
    assert_eq!(plan["core_op"], "EnvGet");
    assert_eq!(plan["tier"], "ColdRuntime");
    assert_eq!(plan["emit_kind"], "runtime_call");
    assert_eq!(plan["symbol"], "nyash.env.get");
    assert_eq!(plan["proof"], "extern_registry");
    assert_eq!(plan["route_proof"], "extern_registry");
    assert_eq!(plan["route_kind"], "extern.env.get");
    assert_eq!(plan["perf_proof"], false);
    assert_eq!(plan["arity"], 1);
    assert_eq!(plan["key_value"], 49);
    assert_eq!(plan["result_value"], 48);
    assert_eq!(plan["return_shape"], "string_handle_or_null");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["publication_policy"], serde_json::Value::Null);
    assert_eq!(plan["effects"], serde_json::json!(["read.env"]));
}

#[test]
fn refresh_function_extern_call_routes_is_available_to_json_emit_tests() {
    let mut function = make_function("main", true);
    refresh_function_extern_call_routes(&mut function);
    assert!(function.metadata.extern_call_routes.is_empty());
}

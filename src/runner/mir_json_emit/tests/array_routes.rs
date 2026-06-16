use super::super::build_mir_json_root;
use super::make_function;

#[test]
fn build_mir_json_root_emits_array_rmw_window_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .array_rmw_window_routes
        .push(crate::mir::array_rmw_window_plan::test_support::json_route());
    let mut module = crate::mir::MirModule::new("json_array_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["array_rmw_window_routes"][0];
    assert_eq!(route["route_id"], "array.rmw_add1.window");
    assert_eq!(route["block"], 7);
    assert_eq!(route["instruction_index"], 3);
    assert_eq!(route["array_value"], 10);
    assert_eq!(route["index_value"], 11);
    assert_eq!(route["input_value"], 12);
    assert_eq!(route["const_value"], 13);
    assert_eq!(route["result_value"], 14);
    assert_eq!(route["set_instruction_index"], 6);
    assert_eq!(
        route["skip_instruction_indices"],
        serde_json::json!([4, 5, 6])
    );
    assert_eq!(route["proof"], "array_get_add1_set_same_slot");
    assert_eq!(route["emit_symbol"], "nyash.array.rmw_add1_hi");
    assert_eq!(
        route["effects"],
        serde_json::json!(["load.cell", "store.cell"])
    );
}

#[test]
fn build_mir_json_root_emits_array_string_len_window_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .array_string_len_window_routes
        .push(crate::mir::array_string_len_window_plan::test_support::json_len_only_route());
    function
        .metadata
        .array_string_len_window_routes
        .push(crate::mir::array_string_len_window_plan::test_support::json_keep_get_live_route());
    let mut module = crate::mir::MirModule::new("json_array_string_len_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let routes = &root["functions"][0]["metadata"]["array_string_len_window_routes"];
    let route = &routes[0];
    assert_eq!(route["route_id"], "array.string_len.window");
    assert_eq!(route["block"], 7);
    assert_eq!(route["instruction_index"], 3);
    assert_eq!(route["array_value"], 10);
    assert_eq!(route["index_value"], 11);
    assert_eq!(route["source_value"], 12);
    assert_eq!(route["len_instruction_index"], 5);
    assert_eq!(route["len_value"], 13);
    assert_eq!(route["skip_instruction_indices"], serde_json::json!([4, 5]));
    assert_eq!(route["mode"], "len_only");
    assert_eq!(route["proof"], "array_get_len_no_later_source_use");
    assert_eq!(route["emit_symbol"], "nyash.array.string_len_hi");
    assert_eq!(route["keep_get_live"], false);
    assert_eq!(route["source_only_insert_mid"], false);
    assert_eq!(
        route["effects"],
        serde_json::json!(["load.cell", "observe.len"])
    );

    let keep_live_route = &routes[1];
    assert_eq!(keep_live_route["route_id"], "array.string_len.window");
    assert_eq!(keep_live_route["block"], 8);
    assert_eq!(keep_live_route["instruction_index"], 4);
    assert_eq!(keep_live_route["array_value"], 20);
    assert_eq!(keep_live_route["index_value"], 21);
    assert_eq!(keep_live_route["source_value"], 22);
    assert_eq!(keep_live_route["len_instruction_index"], 6);
    assert_eq!(keep_live_route["len_value"], 23);
    assert_eq!(
        keep_live_route["skip_instruction_indices"],
        serde_json::json!([6])
    );
    assert_eq!(keep_live_route["mode"], "keep_get_live");
    assert_eq!(keep_live_route["proof"], "array_get_len_keep_source_live");
    assert_eq!(keep_live_route["emit_symbol"], "nyash.array.string_len_hi");
    assert_eq!(keep_live_route["keep_get_live"], true);
    assert_eq!(keep_live_route["source_only_insert_mid"], false);
    assert_eq!(
        keep_live_route["effects"],
        serde_json::json!(["load.cell", "observe.len", "keep.source.live"])
    );
}

#[test]
fn build_mir_json_root_emits_array_text_loop_session_plans() {
    let mut function = make_function("main", true);
    function.metadata.array_text_loop_session_plans.push(
        crate::mir::array_text_loop_session_plan::ArrayTextLoopSessionPlan::new(
            crate::mir::BasicBlockId::new(25),
            crate::mir::BasicBlockId::new(28),
            crate::mir::ValueId::new(5),
            crate::mir::ValueId::new(72),
            1,
            true,
            true,
            true,
            true,
            true,
        )
        .with_region_payload(
            crate::mir::array_text_loop_session_plan::ArrayTextLoopSessionRegionPayload::new(
                crate::mir::ValueId::new(5),
                crate::mir::ValueId::new(52),
                crate::mir::ValueId::new(51),
                0,
                crate::mir::ValueId::new(53),
                crate::mir::ValueId::new(66),
                600000,
                crate::mir::ValueId::new(56),
                crate::mir::ValueId::new(50),
                0,
                crate::mir::ValueId::new(61),
                crate::mir::ValueId::new(56),
                crate::mir::ValueId::new(72),
                crate::mir::ValueId::new(75),
                64,
            ),
        ),
    );
    let mut module =
        crate::mir::MirModule::new("json_array_text_loop_session_plans_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plan = &root["functions"][0]["metadata"]["array_text_loop_session_plans"][0];
    assert_eq!(plan["route_id"], "array_text.loop_session.plan");
    assert_eq!(plan["loop_header"], 25);
    assert_eq!(plan["loop_exit"], 28);
    assert_eq!(plan["array_value"], 5);
    assert_eq!(plan["index_value"], 72);
    assert_eq!(plan["len_call_count"], 1);
    assert_eq!(plan["same_array_handle"], true);
    assert_eq!(plan["read_only_region"], true);
    assert_eq!(plan["no_mutation_region"], true);
    assert_eq!(plan["no_drop_or_publication_boundary"], true);
    assert_eq!(plan["index_domain_guarded"], true);
    assert_eq!(plan["backend_session_lowering_allowed"], true);
    assert_eq!(plan["first_reject_reason"], serde_json::Value::Null);
    let payload = &plan["region_payload"];
    assert_eq!(payload["array_root_value"], 5);
    assert_eq!(payload["loop_index_phi_value"], 52);
    assert_eq!(payload["loop_index_initial_value"], 51);
    assert_eq!(payload["loop_index_initial_const"], 0);
    assert_eq!(payload["loop_index_next_value"], 53);
    assert_eq!(payload["loop_bound_value"], 66);
    assert_eq!(payload["loop_bound_const"], 600000);
    assert_eq!(payload["accumulator_phi_value"], 56);
    assert_eq!(payload["accumulator_initial_value"], 50);
    assert_eq!(payload["accumulator_initial_const"], 0);
    assert_eq!(payload["accumulator_next_value"], 61);
    assert_eq!(payload["exit_accumulator_value"], 56);
    assert_eq!(payload["row_index_value"], 72);
    assert_eq!(payload["row_modulus_value"], 75);
    assert_eq!(payload["row_modulus_const"], 64);
    assert_eq!(plan["mir_json_export_only"], true);
    assert_eq!(plan["backend_consumer_enabled"], false);
}

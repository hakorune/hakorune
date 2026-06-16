use super::super::build_mir_json_root;
use super::make_function;

#[test]
fn build_mir_json_root_emits_string_direct_set_window_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .string_direct_set_window_routes
        .push(crate::mir::string_direct_set_window_plan::test_support::piecewise_route());
    let mut module = crate::mir::MirModule::new("json_string_direct_set_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["string_direct_set_window_routes"][0];
    assert_eq!(route["route_id"], "string.direct_set_source_window");
    assert_eq!(route["block"], 7);
    assert_eq!(route["instruction_index"], 3);
    assert_eq!(route["second_instruction_index"], 4);
    assert_eq!(route["concat_instruction_index"], 8);
    assert_eq!(route["source_value"], 10);
    assert_eq!(route["prefix_value"], 11);
    assert_eq!(route["suffix_value"], 12);
    assert_eq!(route["middle_value"], 13);
    assert_eq!(route["split_value"], 14);
    assert_eq!(route["result_value"], 15);
    assert_eq!(route["subrange_start"], 16);
    assert_eq!(route["subrange_end"], 17);
    assert_eq!(
        route["skip_instruction_indices"],
        serde_json::json!([4, 5, 8])
    );
    assert_eq!(route["proof"], "piecewise_concat3_direct_set_source_window");
    assert_eq!(route["consumer"], "direct_set");
    assert_eq!(
        route["effects"],
        serde_json::json!([
            "observe.substring",
            "defer.piecewise",
            "direct.set.consumer"
        ])
    );
}

#[test]
fn build_mir_json_root_emits_string_dead_text_region_plans() {
    let mut function = make_function("main", true);
    function.metadata.string_dead_text_region_plans.push(
        crate::mir::string_dead_text_region_plan::StringDeadTextRegionPlan::new(
            crate::mir::BasicBlockId::new(18),
            crate::mir::BasicBlockId::new(19),
            crate::mir::BasicBlockId::new(21),
            crate::mir::ValueId::new(20),
            crate::mir::ValueId::new(3),
            crate::mir::ValueId::new(14),
            crate::mir::ValueId::new(10),
            0,
            crate::mir::ValueId::new(15),
            crate::mir::ValueId::new(40),
            300000,
            crate::mir::ValueId::new(5),
            16,
            crate::mir::ValueId::new(65),
            "xx".to_string(),
            2,
            crate::mir::ValueId::new(18),
            crate::mir::ValueId::new(12),
            0,
            crate::mir::ValueId::new(31),
            crate::mir::ValueId::new(30),
            18,
            crate::mir::ValueId::new(18),
            crate::mir::ValueId::new(81),
            5_400_016,
            crate::mir::ValueId::new(25),
            crate::mir::ValueId::new(26),
            crate::mir::ValueId::new(35),
            crate::mir::ValueId::new(70),
            crate::mir::ValueId::new(71),
            crate::mir::ValueId::new(46),
        ),
    );
    let mut module =
        crate::mir::MirModule::new("json_string_dead_text_region_plans_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let plan = &root["functions"][0]["metadata"]["string_dead_text_region_plans"][0];
    assert_eq!(plan["route_id"], "string.dead_text_region.plan");
    assert_eq!(plan["loop_header"], 18);
    assert_eq!(plan["loop_body"], 19);
    assert_eq!(plan["loop_exit"], 21);
    assert_eq!(plan["text_phi_value"], 20);
    assert_eq!(plan["loop_bound_const"], 300000);
    assert_eq!(plan["base_len_const"], 16);
    assert_eq!(plan["inserted_text"], "xx");
    assert_eq!(plan["inserted_len_const"], 2);
    assert_eq!(plan["accumulator_delta_const"], 18);
    assert_eq!(plan["final_return_value"], 5_400_016);
    assert_eq!(plan["publication_boundary"], "none");
    assert_eq!(plan["final_text_content_observed"], false);
    assert_eq!(plan["mir_json_export_only"], true);
    assert_eq!(plan["backend_consumer_enabled"], false);
}

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

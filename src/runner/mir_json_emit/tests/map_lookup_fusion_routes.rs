use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::map_lookup_fusion_plan::test_support as map_lookup_fusion_fixture;

#[test]
fn build_mir_json_root_emits_map_lookup_fusion_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .map_lookup_fusion_routes
        .push(map_lookup_fusion_fixture::same_key_nonzero_json_fixture());
    let mut module = crate::mir::MirModule::new("json_map_lookup_fusion_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["map_lookup_fusion_routes"][0];
    assert_eq!(route["route_id"], "map_lookup.same_key");
    assert_eq!(route["block"], 4);
    assert_eq!(route["get_instruction_index"], 10);
    assert_eq!(route["has_instruction_index"], 12);
    assert_eq!(route["fusion_op"], "MapLookupSameKey");
    assert_eq!(route["receiver_origin_box"], "MapBox");
    assert_eq!(route["receiver_value"], 20);
    assert_eq!(route["key_value"], 21);
    assert_eq!(route["key_const"], -1);
    assert_eq!(route["key_route"], "i64_const");
    assert_eq!(route["get_result_value"], 22);
    assert_eq!(route["has_result_value"], 23);
    assert_eq!(route["get_return_shape"], "scalar_i64_or_missing_zero");
    assert_eq!(route["get_value_demand"], "scalar_i64");
    assert_eq!(route["get_publication_policy"], "no_publication");
    assert_eq!(route["has_result_shape"], "presence_bool");
    assert_eq!(route["stored_value_proof"], "scalar_i64_nonzero");
    assert_eq!(route["stored_value_const"], 7);
    assert_eq!(route["stored_value_known_nonzero"], true);
    assert_eq!(route["proof"], "same_receiver_same_i64_key_scalar_get_has");
    assert_eq!(route["lowering_tier"], "cold_fallback");
    assert_eq!(
        route["effects"],
        serde_json::json!(["read.key", "probe.key", "metadata.only"])
    );
}

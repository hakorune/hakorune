use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::generic_method_route_plan::test_support as generic_route_fixture;

#[test]
fn build_mir_json_root_emits_generic_method_routes() {
    let mut function = make_function("main", true);
    function.metadata.generic_method_routes.extend([
        generic_route_fixture::map_contains_i64(7, 3, 10, 11, 12),
        generic_route_fixture::runtime_data_map_get_mixed_i64_key(8, 4, 13, 14, 15),
        generic_route_fixture::runtime_data_map_get_scalar_i64_same_key(9, 5, 16, 17, 18),
        generic_route_fixture::string_substring(11, 7, 21, 24),
        generic_route_fixture::map_len(10, 6, 19, 20),
        generic_route_fixture::array_push(12, 8, 25, 27),
        generic_route_fixture::map_set_i64_key(13, 9, 28, 29, 31),
        generic_route_fixture::map_get_unknown_key(14, 10, 32, 33, 34),
        generic_route_fixture::array_get_i64_key(15, 11, 35, 36, 37),
        generic_route_fixture::mir_json_numeric_value_field_get(16, 12, 38, 39, 40),
        generic_route_fixture::mir_json_flags_keys(17, 13, 41, 42),
    ]);
    let mut module = crate::mir::MirModule::new("json_generic_method_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["generic_method_routes"][0];
    assert_eq!(route["route_id"], "generic_method.has");
    assert_eq!(route["block"], 7);
    assert_eq!(route["instruction_index"], 3);
    assert_eq!(route["box_name"], "MapBox");
    assert_eq!(route["method"], "has");
    assert_eq!(route["receiver_origin_box"], "MapBox");
    assert_eq!(route["key_route"], "i64_const");
    assert_eq!(route["arity"], 1);
    assert_eq!(route["receiver_value"], 10);
    assert_eq!(route["key_value"], 11);
    assert_eq!(route["result_value"], 12);
    assert_eq!(route["emit_kind"], "has");
    assert_eq!(route["route_kind"], "map_contains_i64");
    assert_eq!(route["helper_symbol"], "nyash.map.probe_hi");
    assert_eq!(route["proof"], "has_surface_policy");
    assert_eq!(route["core_method"]["op"], "MapHas");
    assert_eq!(
        route["core_method"]["proof"],
        "core_method_contract_manifest"
    );
    assert_eq!(route["core_method"]["lowering_tier"], "warm_direct_abi");
    assert_eq!(route["return_shape"], serde_json::Value::Null);
    assert_eq!(route["value_demand"], "read_ref");
    assert_eq!(route["publication_policy"], serde_json::Value::Null);
    assert_eq!(route["effects"], serde_json::json!(["probe.key"]));

    let lowering_plan = root["functions"][0]["metadata"]["lowering_plan"]
        .as_array()
        .expect("lowering_plan");
    assert_eq!(lowering_plan.len(), 11);
    let get_plan = &lowering_plan[1];
    assert_eq!(get_plan["site"], "b8.i4");
    assert_eq!(get_plan["block"], 8);
    assert_eq!(get_plan["instruction_index"], 4);
    assert_eq!(get_plan["source"], "generic_method_routes");
    assert_eq!(get_plan["source_route_id"], "generic_method.get");
    assert_eq!(get_plan["core_op"], "MapGet");
    assert_eq!(get_plan["tier"], "ColdRuntime");
    assert_eq!(get_plan["emit_kind"], "runtime_call");
    assert_eq!(get_plan["symbol"], "nyash.runtime_data.get_hh");
    assert_eq!(get_plan["proof"], "core_method_contract_manifest");
    assert_eq!(get_plan["route_proof"], "get_surface_policy");
    assert_eq!(get_plan["route_kind"], "runtime_data_load_any");
    assert_eq!(get_plan["perf_proof"], false);
    assert_eq!(get_plan["receiver_value"], 13);
    assert_eq!(get_plan["receiver_origin_box"], "MapBox");
    assert_eq!(get_plan["arity"], 1);
    assert_eq!(get_plan["key_route"], "i64_const");
    assert_eq!(get_plan["key_value"], 14);
    assert_eq!(get_plan["result_value"], 15);
    assert_eq!(get_plan["return_shape"], "mixed_runtime_i64_or_handle");
    assert_eq!(get_plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(get_plan["publication_policy"], "runtime_data_facade");
    assert_eq!(get_plan["effects"], serde_json::json!(["read.key"]));

    let get_route = &root["functions"][0]["metadata"]["generic_method_routes"][1];
    assert_eq!(get_route["route_id"], "generic_method.get");
    assert_eq!(get_route["block"], 8);
    assert_eq!(get_route["instruction_index"], 4);
    assert_eq!(get_route["box_name"], "RuntimeDataBox");
    assert_eq!(get_route["method"], "get");
    assert_eq!(get_route["receiver_origin_box"], "MapBox");
    assert_eq!(get_route["key_route"], "i64_const");
    assert_eq!(get_route["arity"], 1);
    assert_eq!(get_route["receiver_value"], 13);
    assert_eq!(get_route["key_value"], 14);
    assert_eq!(get_route["result_value"], 15);
    assert_eq!(get_route["emit_kind"], "get");
    assert_eq!(get_route["route_kind"], "runtime_data_load_any");
    assert_eq!(get_route["helper_symbol"], "nyash.runtime_data.get_hh");
    assert_eq!(get_route["proof"], "get_surface_policy");
    assert_eq!(get_route["core_method"]["op"], "MapGet");
    assert_eq!(
        get_route["core_method"]["proof"],
        "core_method_contract_manifest"
    );
    assert_eq!(get_route["core_method"]["lowering_tier"], "cold_fallback");
    assert_eq!(get_route["return_shape"], "mixed_runtime_i64_or_handle");
    assert_eq!(get_route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(get_route["publication_policy"], "runtime_data_facade");
    assert_eq!(get_route["effects"], serde_json::json!(["read.key"]));

    let scalar_get_route = &root["functions"][0]["metadata"]["generic_method_routes"][2];
    assert_eq!(scalar_get_route["route_id"], "generic_method.get");
    assert_eq!(scalar_get_route["block"], 9);
    assert_eq!(scalar_get_route["instruction_index"], 5);
    assert_eq!(scalar_get_route["box_name"], "RuntimeDataBox");
    assert_eq!(scalar_get_route["method"], "get");
    assert_eq!(scalar_get_route["receiver_origin_box"], "MapBox");
    assert_eq!(scalar_get_route["key_route"], "i64_const");
    assert_eq!(scalar_get_route["route_kind"], "runtime_data_load_any");
    assert_eq!(
        scalar_get_route["helper_symbol"],
        "nyash.runtime_data.get_hh"
    );
    assert_eq!(
        scalar_get_route["proof"],
        "map_set_scalar_i64_same_key_no_escape"
    );
    assert_eq!(scalar_get_route["core_method"]["op"], "MapGet");
    assert_eq!(
        scalar_get_route["core_method"]["lowering_tier"],
        "cold_fallback"
    );
    assert_eq!(
        scalar_get_route["return_shape"],
        "scalar_i64_or_missing_zero"
    );
    assert_eq!(scalar_get_route["value_demand"], "scalar_i64");
    assert_eq!(scalar_get_route["publication_policy"], "no_publication");
    assert_eq!(scalar_get_route["effects"], serde_json::json!(["read.key"]));

    let len_route = &root["functions"][0]["metadata"]["generic_method_routes"][4];
    assert_eq!(len_route["route_id"], "generic_method.len");
    assert_eq!(len_route["block"], 10);
    assert_eq!(len_route["instruction_index"], 6);
    assert_eq!(len_route["box_name"], "MapBox");
    assert_eq!(len_route["method"], "size");
    assert_eq!(len_route["receiver_origin_box"], "MapBox");
    assert_eq!(len_route["key_route"], serde_json::Value::Null);
    assert_eq!(len_route["arity"], 0);
    assert_eq!(len_route["receiver_value"], 19);
    assert_eq!(len_route["key_value"], serde_json::Value::Null);
    assert_eq!(len_route["result_value"], 20);
    assert_eq!(len_route["emit_kind"], "len");
    assert_eq!(len_route["route_kind"], "map_entry_count");
    assert_eq!(len_route["helper_symbol"], "nyash.map.entry_count_i64");
    assert_eq!(len_route["proof"], "len_surface_policy");
    assert_eq!(len_route["core_method"]["op"], "MapLen");
    assert_eq!(len_route["core_method"]["lowering_tier"], "warm_direct_abi");
    assert_eq!(len_route["return_shape"], "scalar_i64");
    assert_eq!(len_route["value_demand"], "scalar_i64");
    assert_eq!(len_route["publication_policy"], "no_publication");
    assert_eq!(len_route["effects"], serde_json::json!(["observe.len"]));

    let substring_route = &root["functions"][0]["metadata"]["generic_method_routes"][3];
    assert_eq!(substring_route["route_id"], "generic_method.substring");
    assert_eq!(substring_route["block"], 11);
    assert_eq!(substring_route["instruction_index"], 7);
    assert_eq!(substring_route["box_name"], "StringBox");
    assert_eq!(substring_route["method"], "substring");
    assert_eq!(substring_route["receiver_origin_box"], "StringBox");
    assert_eq!(substring_route["key_route"], serde_json::Value::Null);
    assert_eq!(substring_route["arity"], 2);
    assert_eq!(substring_route["receiver_value"], 21);
    assert_eq!(substring_route["key_value"], serde_json::Value::Null);
    assert_eq!(substring_route["result_value"], 24);
    assert_eq!(substring_route["emit_kind"], "substring");
    assert_eq!(substring_route["route_kind"], "string_substring");
    assert_eq!(
        substring_route["helper_symbol"],
        "nyash.string.substring_hii"
    );
    assert_eq!(substring_route["proof"], "substring_surface_policy");
    assert_eq!(substring_route["core_method"]["op"], "StringSubstring");
    assert_eq!(
        substring_route["core_method"]["lowering_tier"],
        "warm_direct_abi"
    );
    assert_eq!(substring_route["return_shape"], serde_json::Value::Null);
    assert_eq!(substring_route["value_demand"], "read_ref");
    assert_eq!(
        substring_route["publication_policy"],
        serde_json::Value::Null
    );
    assert_eq!(
        substring_route["effects"],
        serde_json::json!(["observe.substring"])
    );

    let push_route = &root["functions"][0]["metadata"]["generic_method_routes"][5];
    assert_eq!(push_route["route_id"], "generic_method.push");
    assert_eq!(push_route["block"], 12);
    assert_eq!(push_route["instruction_index"], 8);
    assert_eq!(push_route["box_name"], "ArrayBox");
    assert_eq!(push_route["method"], "push");
    assert_eq!(push_route["receiver_origin_box"], "ArrayBox");
    assert_eq!(push_route["key_route"], serde_json::Value::Null);
    assert_eq!(push_route["arity"], 1);
    assert_eq!(push_route["receiver_value"], 25);
    assert_eq!(push_route["key_value"], serde_json::Value::Null);
    assert_eq!(push_route["result_value"], 27);
    assert_eq!(push_route["emit_kind"], "push");
    assert_eq!(push_route["route_kind"], "array_append_any");
    assert_eq!(push_route["helper_symbol"], "nyash.array.slot_append_hh");
    assert_eq!(push_route["proof"], "push_surface_policy");
    assert_eq!(push_route["core_method"]["op"], "ArrayPush");
    assert_eq!(push_route["core_method"]["lowering_tier"], "cold_fallback");
    assert_eq!(push_route["return_shape"], "scalar_i64");
    assert_eq!(push_route["value_demand"], "write_any");
    assert_eq!(push_route["publication_policy"], "no_publication");
    assert_eq!(push_route["effects"], serde_json::json!(["mutate.shape"]));

    let set_route = &root["functions"][0]["metadata"]["generic_method_routes"][6];
    assert_eq!(set_route["route_id"], "generic_method.set");
    assert_eq!(set_route["block"], 13);
    assert_eq!(set_route["instruction_index"], 9);
    assert_eq!(set_route["box_name"], "MapBox");
    assert_eq!(set_route["method"], "set");
    assert_eq!(set_route["receiver_origin_box"], "MapBox");
    assert_eq!(set_route["key_route"], "i64_const");
    assert_eq!(set_route["arity"], 2);
    assert_eq!(set_route["receiver_value"], 28);
    assert_eq!(set_route["key_value"], 29);
    assert_eq!(set_route["result_value"], 31);
    assert_eq!(set_route["emit_kind"], "set");
    assert_eq!(set_route["route_kind"], "map_store_any");
    assert_eq!(set_route["helper_symbol"], "nyash.map.slot_store_hhh");
    assert_eq!(set_route["proof"], "set_surface_policy");
    assert_eq!(set_route["core_method"]["op"], "MapSet");
    assert_eq!(set_route["core_method"]["lowering_tier"], "cold_fallback");
    assert_eq!(set_route["return_shape"], serde_json::Value::Null);
    assert_eq!(set_route["value_demand"], "write_any");
    assert_eq!(set_route["publication_policy"], serde_json::Value::Null);
    assert_eq!(set_route["effects"], serde_json::json!(["mutate.slot"]));

    let direct_map_get_route = &root["functions"][0]["metadata"]["generic_method_routes"][7];
    assert_eq!(direct_map_get_route["route_id"], "generic_method.get");
    assert_eq!(direct_map_get_route["block"], 14);
    assert_eq!(direct_map_get_route["instruction_index"], 10);
    assert_eq!(direct_map_get_route["box_name"], "MapBox");
    assert_eq!(direct_map_get_route["method"], "get");
    assert_eq!(direct_map_get_route["receiver_origin_box"], "MapBox");
    assert_eq!(direct_map_get_route["key_route"], "unknown_any");
    assert_eq!(direct_map_get_route["arity"], 1);
    assert_eq!(direct_map_get_route["receiver_value"], 32);
    assert_eq!(direct_map_get_route["key_value"], 33);
    assert_eq!(direct_map_get_route["result_value"], 34);
    assert_eq!(direct_map_get_route["emit_kind"], "get");
    assert_eq!(direct_map_get_route["route_kind"], "map_load_any");
    assert_eq!(
        direct_map_get_route["helper_symbol"],
        "nyash.map.slot_load_hh"
    );
    assert_eq!(direct_map_get_route["proof"], "get_surface_policy");
    assert_eq!(direct_map_get_route["core_method"]["op"], "MapGet");
    assert_eq!(
        direct_map_get_route["core_method"]["lowering_tier"],
        "warm_direct_abi"
    );
    assert_eq!(
        direct_map_get_route["return_shape"],
        serde_json::Value::Null
    );
    assert_eq!(direct_map_get_route["value_demand"], "read_ref");
    assert_eq!(
        direct_map_get_route["publication_policy"],
        serde_json::Value::Null
    );
    assert_eq!(
        direct_map_get_route["effects"],
        serde_json::json!(["read.key"])
    );

    let direct_array_get_route = &root["functions"][0]["metadata"]["generic_method_routes"][8];
    assert_eq!(direct_array_get_route["route_id"], "generic_method.get");
    assert_eq!(direct_array_get_route["block"], 15);
    assert_eq!(direct_array_get_route["instruction_index"], 11);
    assert_eq!(direct_array_get_route["box_name"], "ArrayBox");
    assert_eq!(direct_array_get_route["method"], "get");
    assert_eq!(direct_array_get_route["receiver_origin_box"], "ArrayBox");
    assert_eq!(direct_array_get_route["key_route"], "i64_const");
    assert_eq!(direct_array_get_route["arity"], 1);
    assert_eq!(direct_array_get_route["receiver_value"], 35);
    assert_eq!(direct_array_get_route["key_value"], 36);
    assert_eq!(direct_array_get_route["result_value"], 37);
    assert_eq!(direct_array_get_route["emit_kind"], "get");
    assert_eq!(direct_array_get_route["route_kind"], "array_slot_load_any");
    assert_eq!(
        direct_array_get_route["helper_symbol"],
        "nyash.array.slot_load_hi"
    );
    assert_eq!(direct_array_get_route["proof"], "get_surface_policy");
    assert_eq!(direct_array_get_route["core_method"]["op"], "ArrayGet");
    assert_eq!(
        direct_array_get_route["core_method"]["lowering_tier"],
        "warm_direct_abi"
    );
    assert_eq!(
        direct_array_get_route["return_shape"],
        serde_json::Value::Null
    );
    assert_eq!(direct_array_get_route["value_demand"], "read_ref");
    assert_eq!(
        direct_array_get_route["publication_policy"],
        serde_json::Value::Null
    );
    assert_eq!(
        direct_array_get_route["effects"],
        serde_json::json!(["read.key"])
    );

    let mir_schema_get_route = &root["functions"][0]["metadata"]["generic_method_routes"][9];
    assert_eq!(mir_schema_get_route["route_id"], "generic_method.get");
    assert_eq!(mir_schema_get_route["block"], 16);
    assert_eq!(mir_schema_get_route["instruction_index"], 12);
    assert_eq!(mir_schema_get_route["box_name"], "RuntimeDataBox");
    assert_eq!(mir_schema_get_route["method"], "get");
    assert_eq!(
        mir_schema_get_route["receiver_origin_box"],
        serde_json::Value::Null
    );
    assert_eq!(mir_schema_get_route["key_route"], "unknown_any");
    assert_eq!(mir_schema_get_route["key_const_text"], "value");
    assert_eq!(mir_schema_get_route["receiver_value"], 38);
    assert_eq!(mir_schema_get_route["key_value"], 39);
    assert_eq!(mir_schema_get_route["result_value"], 40);
    assert_eq!(mir_schema_get_route["route_kind"], "runtime_data_load_any");
    assert_eq!(
        mir_schema_get_route["proof"],
        "mir_json_numeric_value_field"
    );
    assert_eq!(
        mir_schema_get_route["return_shape"],
        "scalar_i64_or_missing_zero"
    );
    assert_eq!(mir_schema_get_route["value_demand"], "scalar_i64");
    assert_eq!(mir_schema_get_route["publication_policy"], "no_publication");

    let mir_schema_get_plan = &lowering_plan[9];
    assert_eq!(mir_schema_get_plan["site"], "b16.i12");
    assert_eq!(mir_schema_get_plan["source"], "generic_method_routes");
    assert_eq!(mir_schema_get_plan["source_route_id"], "generic_method.get");
    assert_eq!(mir_schema_get_plan["core_op"], "MapGet");
    assert_eq!(mir_schema_get_plan["tier"], "ColdRuntime");
    assert_eq!(mir_schema_get_plan["emit_kind"], "runtime_call");
    assert_eq!(mir_schema_get_plan["route_kind"], "runtime_data_load_any");
    assert_eq!(
        mir_schema_get_plan["route_proof"],
        "mir_json_numeric_value_field"
    );
    assert_eq!(mir_schema_get_plan["key_const_text"], "value");
    assert_eq!(
        mir_schema_get_plan["return_shape"],
        "scalar_i64_or_missing_zero"
    );
    assert_eq!(mir_schema_get_plan["value_demand"], "scalar_i64");

    let flags_keys_route = &root["functions"][0]["metadata"]["generic_method_routes"][10];
    assert_eq!(flags_keys_route["route_id"], "generic_method.keys");
    assert_eq!(flags_keys_route["block"], 17);
    assert_eq!(flags_keys_route["instruction_index"], 13);
    assert_eq!(flags_keys_route["box_name"], "RuntimeDataBox");
    assert_eq!(flags_keys_route["method"], "keys");
    assert_eq!(
        flags_keys_route["receiver_origin_box"],
        serde_json::Value::Null
    );
    assert_eq!(flags_keys_route["route_kind"], "map_keys_array");
    assert_eq!(flags_keys_route["helper_symbol"], "nyash.map.keys_h");
    assert_eq!(flags_keys_route["proof"], "mir_json_flags_keys");
    assert_eq!(flags_keys_route["core_method"], serde_json::Value::Null);
    assert_eq!(
        flags_keys_route["return_shape"],
        "mixed_runtime_i64_or_handle"
    );
    assert_eq!(flags_keys_route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(flags_keys_route["publication_policy"], "no_publication");

    let flags_keys_plan = &lowering_plan[10];
    assert_eq!(flags_keys_plan["site"], "b17.i13");
    assert_eq!(flags_keys_plan["source"], "generic_method_routes");
    assert_eq!(flags_keys_plan["source_route_id"], "generic_method.keys");
    assert_eq!(flags_keys_plan["core_op"], "MapKeys");
    assert_eq!(flags_keys_plan["tier"], "DirectAbi");
    assert_eq!(flags_keys_plan["emit_kind"], "direct_abi_call");
    assert_eq!(flags_keys_plan["symbol"], "nyash.map.keys_h");
    assert_eq!(flags_keys_plan["proof"], "mir_json_flags_keys");
    assert_eq!(flags_keys_plan["route_proof"], "mir_json_flags_keys");
    assert_eq!(flags_keys_plan["route_kind"], "map_keys_array");
    assert_eq!(
        flags_keys_plan["return_shape"],
        "mixed_runtime_i64_or_handle"
    );
    assert_eq!(flags_keys_plan["value_demand"], "runtime_i64_or_handle");
}

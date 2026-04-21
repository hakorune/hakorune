use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{
    ArrayRmwWindowProof, ArrayRmwWindowRoute, ArrayStringLenWindowMode, ArrayStringLenWindowProof,
    ArrayStringLenWindowRoute, BasicBlockId, ValueId,
};

#[test]
fn build_mir_json_root_emits_array_rmw_window_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .array_rmw_window_routes
        .push(ArrayRmwWindowRoute {
            block: BasicBlockId::new(7),
            instruction_index: 3,
            array_value: ValueId::new(10),
            index_value: ValueId::new(11),
            input_value: ValueId::new(12),
            const_value: ValueId::new(13),
            result_value: ValueId::new(14),
            set_instruction_index: 6,
            skip_instruction_indices: vec![4, 5, 6],
            proof: ArrayRmwWindowProof::ArrayGetAdd1SetSameSlot,
        });
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
        .push(ArrayStringLenWindowRoute {
            block: BasicBlockId::new(7),
            instruction_index: 3,
            array_value: ValueId::new(10),
            index_value: ValueId::new(11),
            source_value: ValueId::new(12),
            len_instruction_index: 5,
            len_value: ValueId::new(13),
            skip_instruction_indices: vec![4, 5],
            mode: ArrayStringLenWindowMode::LenOnly,
            proof: ArrayStringLenWindowProof::ArrayGetLenNoLaterSourceUse,
        });
    let mut module = crate::mir::MirModule::new("json_array_string_len_routes_test".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["array_string_len_window_routes"][0];
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
    assert_eq!(
        route["effects"],
        serde_json::json!(["load.cell", "observe.len"])
    );
}

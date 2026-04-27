use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{
    BasicBlockId, CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodRoute,
    GenericMethodRouteDecision, GenericMethodRouteKind, GenericMethodRouteProof,
    GenericMethodRouteSurface, GenericMethodValueDemand, ValueId,
};

fn decision(
    route_kind: GenericMethodRouteKind,
    proof: GenericMethodRouteProof,
    core_op: CoreMethodOp,
    lowering_tier: CoreMethodLoweringTier,
    return_shape: Option<GenericMethodReturnShape>,
    value_demand: GenericMethodValueDemand,
    publication_policy: Option<GenericMethodPublicationPolicy>,
) -> GenericMethodRouteDecision {
    GenericMethodRouteDecision::new(
        route_kind,
        proof,
        Some(CoreMethodOpCarrier::manifest(core_op, lowering_tier)),
        return_shape,
        value_demand,
        publication_policy,
    )
}

#[test]
fn build_mir_json_root_emits_generic_method_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(7),
            instruction_index: 3,
            surface: GenericMethodRouteSurface::new("MapBox", "has", 1),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver_value: ValueId::new(10),
            key_value: Some(ValueId::new(11)),
            result_value: Some(ValueId::new(12)),
            decision: decision(
                GenericMethodRouteKind::MapContainsI64,
                GenericMethodRouteProof::HasSurfacePolicy,
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(8),
            instruction_index: 4,
            surface: GenericMethodRouteSurface::new("RuntimeDataBox", "get", 1),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver_value: ValueId::new(13),
            key_value: Some(ValueId::new(14)),
            result_value: Some(ValueId::new(15)),
            decision: decision(
                GenericMethodRouteKind::RuntimeDataLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
                Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
                GenericMethodValueDemand::RuntimeI64OrHandle,
                Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(9),
            instruction_index: 5,
            surface: GenericMethodRouteSurface::new("RuntimeDataBox", "get", 1),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver_value: ValueId::new(16),
            key_value: Some(ValueId::new(17)),
            result_value: Some(ValueId::new(18)),
            decision: decision(
                GenericMethodRouteKind::RuntimeDataLoadAny,
                GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape,
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
                Some(GenericMethodReturnShape::ScalarI64OrMissingZero),
                GenericMethodValueDemand::ScalarI64,
                Some(GenericMethodPublicationPolicy::NoPublication),
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(11),
            instruction_index: 7,
            surface: GenericMethodRouteSurface::new("StringBox", "substring", 2),
            receiver_origin_box: Some("StringBox".to_string()),
            key_route: None,
            receiver_value: ValueId::new(21),
            key_value: None,
            result_value: Some(ValueId::new(24)),
            decision: decision(
                GenericMethodRouteKind::StringSubstring,
                GenericMethodRouteProof::SubstringSurfacePolicy,
                CoreMethodOp::StringSubstring,
                CoreMethodLoweringTier::WarmDirectAbi,
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(10),
            instruction_index: 6,
            surface: GenericMethodRouteSurface::new("MapBox", "size", 0),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: None,
            receiver_value: ValueId::new(19),
            key_value: None,
            result_value: Some(ValueId::new(20)),
            decision: decision(
                GenericMethodRouteKind::MapEntryCount,
                GenericMethodRouteProof::LenSurfacePolicy,
                CoreMethodOp::MapLen,
                CoreMethodLoweringTier::WarmDirectAbi,
                Some(GenericMethodReturnShape::ScalarI64),
                GenericMethodValueDemand::ScalarI64,
                Some(GenericMethodPublicationPolicy::NoPublication),
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(12),
            instruction_index: 8,
            surface: GenericMethodRouteSurface::new("ArrayBox", "push", 1),
            receiver_origin_box: Some("ArrayBox".to_string()),
            key_route: None,
            receiver_value: ValueId::new(25),
            key_value: None,
            result_value: Some(ValueId::new(27)),
            decision: decision(
                GenericMethodRouteKind::ArrayAppendAny,
                GenericMethodRouteProof::PushSurfacePolicy,
                CoreMethodOp::ArrayPush,
                CoreMethodLoweringTier::ColdFallback,
                Some(GenericMethodReturnShape::ScalarI64),
                GenericMethodValueDemand::WriteAny,
                Some(GenericMethodPublicationPolicy::NoPublication),
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(13),
            instruction_index: 9,
            surface: GenericMethodRouteSurface::new("MapBox", "set", 2),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver_value: ValueId::new(28),
            key_value: Some(ValueId::new(29)),
            result_value: Some(ValueId::new(31)),
            decision: decision(
                GenericMethodRouteKind::MapStoreAny,
                GenericMethodRouteProof::SetSurfacePolicy,
                CoreMethodOp::MapSet,
                CoreMethodLoweringTier::ColdFallback,
                None,
                GenericMethodValueDemand::WriteAny,
                None,
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(14),
            instruction_index: 10,
            surface: GenericMethodRouteSurface::new("MapBox", "get", 1),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::UnknownAny),
            receiver_value: ValueId::new(32),
            key_value: Some(ValueId::new(33)),
            result_value: Some(ValueId::new(34)),
            decision: decision(
                GenericMethodRouteKind::MapLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::WarmDirectAbi,
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(15),
            instruction_index: 11,
            surface: GenericMethodRouteSurface::new("ArrayBox", "get", 1),
            receiver_origin_box: Some("ArrayBox".to_string()),
            key_route: Some(GenericMethodKeyRoute::I64Const),
            receiver_value: ValueId::new(35),
            key_value: Some(ValueId::new(36)),
            result_value: Some(ValueId::new(37)),
            decision: decision(
                GenericMethodRouteKind::ArraySlotLoadAny,
                GenericMethodRouteProof::GetSurfacePolicy,
                CoreMethodOp::ArrayGet,
                CoreMethodLoweringTier::WarmDirectAbi,
                None,
                GenericMethodValueDemand::ReadRef,
                None,
            ),
        });
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
}

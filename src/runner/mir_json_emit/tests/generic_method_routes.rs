use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{
    BasicBlockId, CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier, GenericMethodKeyRoute,
    GenericMethodPublicationPolicy, GenericMethodReturnShape, GenericMethodRoute,
    GenericMethodRouteKind, GenericMethodRouteProof, GenericMethodValueDemand, ValueId,
};

#[test]
fn build_mir_json_root_emits_generic_method_routes() {
    let mut function = make_function("main", true);
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(7),
            instruction_index: 3,
            box_name: "MapBox".to_string(),
            method: "has".to_string(),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: GenericMethodKeyRoute::I64Const,
            receiver_value: ValueId::new(10),
            key_value: ValueId::new(11),
            result_value: Some(ValueId::new(12)),
            route_kind: GenericMethodRouteKind::MapContainsI64,
            proof: GenericMethodRouteProof::HasSurfacePolicy,
            core_method: Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            return_shape: None,
            value_demand: GenericMethodValueDemand::ReadRef,
            publication_policy: None,
        });
    function
        .metadata
        .generic_method_routes
        .push(GenericMethodRoute {
            block: BasicBlockId::new(8),
            instruction_index: 4,
            box_name: "RuntimeDataBox".to_string(),
            method: "get".to_string(),
            receiver_origin_box: Some("MapBox".to_string()),
            key_route: GenericMethodKeyRoute::I64Const,
            receiver_value: ValueId::new(13),
            key_value: ValueId::new(14),
            result_value: Some(ValueId::new(15)),
            route_kind: GenericMethodRouteKind::RuntimeDataLoadAny,
            proof: GenericMethodRouteProof::GetSurfacePolicy,
            core_method: Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapGet,
                CoreMethodLoweringTier::ColdFallback,
            )),
            return_shape: Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle),
            value_demand: GenericMethodValueDemand::RuntimeI64OrHandle,
            publication_policy: Some(GenericMethodPublicationPolicy::RuntimeDataFacade),
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
}

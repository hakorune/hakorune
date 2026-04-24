use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{
    BasicBlockId, CoreMethodLoweringTier, CoreMethodOp, CoreMethodOpCarrier, GenericMethodKeyRoute,
    GenericMethodRoute, GenericMethodRouteKind, GenericMethodRouteProof, GenericMethodValueDemand,
    ValueId,
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
            route_kind: GenericMethodRouteKind::MapContainsAny,
            proof: GenericMethodRouteProof::HasSurfacePolicy,
            core_method: Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            value_demand: GenericMethodValueDemand::ReadRef,
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
    assert_eq!(route["route_kind"], "map_contains_any");
    assert_eq!(route["helper_symbol"], "nyash.map.probe_hh");
    assert_eq!(route["proof"], "has_surface_policy");
    assert_eq!(route["core_method"]["op"], "MapHas");
    assert_eq!(
        route["core_method"]["proof"],
        "core_method_contract_manifest"
    );
    assert_eq!(route["core_method"]["lowering_tier"], "warm_direct_abi");
    assert_eq!(route["value_demand"], "read_ref");
    assert_eq!(route["effects"], serde_json::json!(["probe.key"]));
}

use super::*;

#[test]
fn generic_method_route_metadata_tokens_come_from_route_kind() {
    let route = GenericMethodRoute::new(
        GenericMethodRouteSite::new(BasicBlockId::new(0), 0),
        GenericMethodRouteSurface::new("MapBox", "__raw_method_must_not_drive_metadata", 1),
        GenericMethodRouteEvidence::new(
            Some("MapBox".to_string()),
            Some(GenericMethodKeyRoute::I64Const),
        ),
        GenericMethodRouteOperands::new(
            ValueId::new(1),
            Some(ValueId::new(2)),
            Some(ValueId::new(3)),
        ),
        GenericMethodRouteDecision::new(
            GenericMethodRouteKind::MapContainsI64,
            GenericMethodRouteProof::HasSurfacePolicy,
            Some(CoreMethodOpCarrier::manifest(
                CoreMethodOp::MapHas,
                CoreMethodLoweringTier::WarmDirectAbi,
            )),
            None,
            GenericMethodValueDemand::ReadRef,
            None,
        ),
    );

    assert_eq!(route.route_id(), "generic_method.has");
    assert_eq!(route.emit_kind(), "has");
    assert_eq!(route.effect_tags(), &["probe.key"]);
}

#[test]
fn detects_mapbox_has_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "MapBox", "has", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.block(), BasicBlockId::new(0));
    assert_eq!(route.instruction_index(), 0);
    assert_eq!(route.box_name(), "MapBox");
    assert_eq!(route.method(), "has");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.receiver_value(), ValueId::new(1));
    assert_eq!(route.key_value(), Some(ValueId::new(2)));
    assert_eq!(route.result_value(), Some(ValueId::new(3)));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapContainsAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::HasSurfacePolicy);
    let core_method = route.core_method().expect("MapBox.has core method op");
    assert_eq!(core_method.op, CoreMethodOp::MapHas);
    assert_eq!(
        core_method.proof.to_string(),
        "core_method_contract_manifest"
    );
    assert_eq!(core_method.lowering_tier.to_string(), "warm_direct_abi");
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}


#[test]
fn direct_collection_calls_keep_warm_core_routes() {
    for (case_name, box_name, method, route_id, kind, helper, proof, carrier, op) in [
        (
            "records_direct_arraybox_has_as_arrayhas_core_method_route",
            "ArrayBox",
            "has",
            None,
            GenericMethodRouteKind::ArrayContainsAny,
            "nyash.array.has_hh",
            GenericMethodRouteProof::HasSurfacePolicy,
            "ArrayBox.has core method op",
            CoreMethodOp::ArrayHas,
        ),
        (
            "records_direct_mapbox_get_as_warm_core_method_route",
            "MapBox",
            "get",
            Some("generic_method.get"),
            GenericMethodRouteKind::MapLoadAny,
            "nyash.map.slot_load_hh",
            GenericMethodRouteProof::GetSurfacePolicy,
            "MapBox.get core method op",
            CoreMethodOp::MapGet,
        ),
        (
            "records_direct_arraybox_get_as_warm_core_method_route",
            "ArrayBox",
            "get",
            Some("generic_method.get"),
            GenericMethodRouteKind::ArraySlotLoadAny,
            "nyash.array.slot_load_hi",
            GenericMethodRouteProof::GetSurfacePolicy,
            "ArrayBox.get core method op",
            CoreMethodOp::ArrayGet,
        ),
    ] {
        println!("case: {case_name}");
        let mut function = make_function();
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry");
        block
            .instructions
            .push(method_call(Some(3), box_name, method, 1, vec![2]));

        refresh_function_generic_method_routes(&mut function);

        assert_eq!(function.metadata.generic_method_routes.len(), 1);
        let route = &function.metadata.generic_method_routes[0];
        if let Some(route_id) = route_id {
            assert_eq!(route.route_id(), route_id);
        }
        assert_eq!(route.box_name(), box_name);
        assert_eq!(route.method(), method);
        assert_eq!(route.receiver_origin_box(), Some(box_name));
        assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
        assert_eq!(route.route_kind(), kind);
        assert_eq!(route.route_kind().helper_symbol(), helper);
        assert_eq!(route.proof(), proof);
        let core_method = route.core_method().expect(carrier);
        assert_eq!(core_method.op, op);
        assert_eq!(
            core_method.lowering_tier,
            CoreMethodLoweringTier::WarmDirectAbi
        );
        assert_eq!(route.return_shape(), None);
        assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
        assert_eq!(route.publication_policy(), None);
    }
}

#[test]
fn records_direct_array_i64_get_as_array_slot_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "DirectArrayI64", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "DirectArrayI64");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("DirectArrayI64"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    let core_method = route
        .core_method()
        .expect("DirectArrayI64.get core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
}

use super::*;

#[test]
fn proves_mir_json_flags_rec_access_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_flags_rec/4".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 1, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let key_route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        key_route.proof(),
        GenericMethodRouteProof::MirJsonFlagsRecAccess
    );
    assert_eq!(
        key_route.route_kind(),
        GenericMethodRouteKind::ArraySlotLoadAny
    );
    assert_eq!(
        key_route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        key_route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );
    assert_eq!(
        key_route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );

    let value_route = route_for(&function, "RuntimeDataBox", "get", Some(4));
    assert_eq!(
        value_route.proof(),
        GenericMethodRouteProof::MirJsonFlagsRecAccess
    );
    assert_eq!(
        value_route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(
        value_route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        value_route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );
    assert_eq!(
        value_route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn proves_mir_json_flags_keys_runtime_data_keys() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_flags/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "keys", 1, vec![]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "keys", Some(3));
    assert_eq!(route.route_id(), "generic_method.keys");
    assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonFlagsKeys);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapKeysArray);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.map.keys_h");
    assert!(route.core_method().is_none());
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

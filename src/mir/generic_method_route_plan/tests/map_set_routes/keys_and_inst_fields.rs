use super::*;

#[test]
fn records_direct_map_keys_with_redundant_receiver_as_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "MapBox", "keys", 1, vec![1]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "MapBox", "keys", Some(3));
    assert_eq!(route.route_id(), "generic_method.keys");
    assert_eq!(route.arity(), 0);
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.result_origin_box(), Some("ArrayBox"));
    assert_eq!(route.proof(), GenericMethodRouteProof::KeysSurfacePolicy);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapKeysArray);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.map.keys_h");
    let core_method = route.core_method().expect("MapKeys carrier");
    assert_eq!(core_method.op, CoreMethodOp::MapKeys);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
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

#[test]
fn records_map_keys_length_as_array_len_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "MapBox", "keys", 1, vec![1]));
    block.add_instruction(method_call(Some(4), "ArrayBox", "length", 3, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let keys_route = &function.metadata.generic_method_routes[0];
    assert_eq!(keys_route.route_id(), "generic_method.keys");
    assert_eq!(
        keys_route.route_kind(),
        GenericMethodRouteKind::MapKeysArray
    );
    assert_eq!(keys_route.result_origin_box(), Some("ArrayBox"));

    let len_route = &function.metadata.generic_method_routes[1];
    assert_eq!(len_route.route_id(), "generic_method.len");
    assert_eq!(len_route.route_kind(), GenericMethodRouteKind::ArraySlotLen);
    assert_eq!(len_route.receiver_origin_box(), Some("ArrayBox"));
    let core_method = len_route
        .core_method()
        .expect("Map.keys length ArrayLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
}

#[test]
fn proves_mir_json_flags_keys_length_routes_as_array_len() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_flags/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "keys", 1, vec![]));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "length", 4, vec![]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let keys_route = &function.metadata.generic_method_routes[0];
    assert_eq!(keys_route.route_id(), "generic_method.keys");
    assert_eq!(
        keys_route.route_kind(),
        GenericMethodRouteKind::MapKeysArray
    );
    assert_eq!(
        keys_route.proof(),
        GenericMethodRouteProof::MirJsonFlagsKeys
    );

    let len_route = &function.metadata.generic_method_routes[1];
    assert_eq!(len_route.route_id(), "generic_method.len");
    assert_eq!(len_route.route_kind(), GenericMethodRouteKind::ArraySlotLen);
    assert_eq!(len_route.receiver_origin_box(), Some("ArrayBox"));
    let core_method = len_route
        .core_method()
        .expect("flags keys length ArrayLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
}

#[test]
fn proves_mir_json_inst_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_inst/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    let keys = [
        "op",
        "dst",
        "value",
        "operation",
        "op_kind",
        "lhs",
        "rhs",
        "cmp",
        "cond",
        "then",
        "else",
        "target",
        "incoming",
        "values",
        "mir_call",
        "callee",
        "args",
        "effects",
        "func",
        "name",
    ];
    for (index, key) in keys.iter().enumerate() {
        let key_value = ValueId::new(10 + (index as u32 * 2));
        let result_value = 11 + (index as u32 * 2);
        block.add_instruction(MirInstruction::Const {
            dst: key_value,
            value: crate::mir::ConstValue::String((*key).to_string()),
        });
        block.add_instruction(method_call(
            Some(result_value),
            "RuntimeDataBox",
            "get",
            1,
            vec![key_value.0],
        ));
    }

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), keys.len());
    for (index, key) in keys.iter().enumerate() {
        let result_value = 11 + (index as u32 * 2);
        let route = route_for(&function, "RuntimeDataBox", "get", Some(result_value));
        let expected_origin = match *key {
            "op" | "operation" | "op_kind" | "cmp" | "value" => Some("StringBox"),
            "args" | "effects" => Some("ArrayBox"),
            _ => None,
        };
        assert_eq!(route.key_const_text(), Some(*key));
        assert_eq!(route.result_origin_box(), expected_origin);
        assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonInstField);
        assert_eq!(
            route.route_kind(),
            GenericMethodRouteKind::RuntimeDataLoadAny
        );
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
}

use super::*;

#[test]
fn proves_mir_json_block_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_block/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    let keys = ["instructions", "id"];
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
        assert_eq!(route.key_const_text(), Some(*key));
        assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonBlockField);
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

#[test]
fn proves_mir_json_function_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_function/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    let keys = ["name", "params", "flags", "blocks"];
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
            "name" => Some("StringBox"),
            "params" | "blocks" => Some("ArrayBox"),
            "flags" => Some("MapBox"),
            _ => None,
        };
        assert_eq!(route.key_const_text(), Some(*key));
        assert_eq!(route.result_origin_box(), expected_origin);
        assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonFunctionField);
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

#[test]
fn proves_mir_json_function_blocks_field_length_routes_as_array_len() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_function/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(10),
        value: crate::mir::ConstValue::String("blocks".to_string()),
    });
    block.add_instruction(method_call(Some(11), "RuntimeDataBox", "get", 1, vec![10]));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(12),
        src: ValueId::new(11),
    });
    block.add_instruction(method_call(
        Some(13),
        "RuntimeDataBox",
        "length",
        12,
        vec![],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let field_route = &function.metadata.generic_method_routes[0];
    assert_eq!(field_route.route_id(), "generic_method.get");
    assert_eq!(field_route.key_const_text(), Some("blocks"));
    assert_eq!(
        field_route.proof(),
        GenericMethodRouteProof::MirJsonFunctionField
    );

    let len_route = &function.metadata.generic_method_routes[1];
    assert_eq!(len_route.route_id(), "generic_method.len");
    assert_eq!(len_route.route_kind(), GenericMethodRouteKind::ArraySlotLen);
    assert_eq!(len_route.receiver_origin_box(), Some("ArrayBox"));
    let core_method = len_route
        .core_method()
        .expect("function blocks length ArrayLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
}

#[test]
fn proves_mir_json_module_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox.to_json/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    let keys = ["functions", "functions_0"];
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
            "functions" => Some("ArrayBox"),
            "functions_0" => Some("MapBox"),
            _ => None,
        };
        assert_eq!(route.key_const_text(), Some(*key));
        assert_eq!(route.result_origin_box(), expected_origin);
        assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonModuleField);
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

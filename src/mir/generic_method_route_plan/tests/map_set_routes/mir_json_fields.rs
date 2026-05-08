use super::*;

#[test]
fn proves_mir_json_numeric_value_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._expect_i64/2".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::String("value".to_string()),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("StringHelpers.to_i64/1".to_string())),
        args: vec![ValueId::new(4)],
        effects: EffectMask::PURE,
    });

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), None);
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.key_const_text(), Some("value"));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonNumericValueField
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn rejects_mir_json_numeric_value_field_get_outside_owner() {
    let mut function = make_function();
    function.signature.name = "OtherBox._expect_i64/2".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::String("value".to_string()),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(5)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("StringHelpers.to_i64/1".to_string())),
        args: vec![ValueId::new(3)],
        effects: EffectMask::PURE,
    });

    refresh_function_generic_method_routes(&mut function);

    assert!(function.metadata.generic_method_routes.is_empty());
}

#[test]
fn proves_mir_json_const_value_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_box_value/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::String("type".to_string()),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: crate::mir::ConstValue::String("value".to_string()),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![4]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let type_route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(type_route.key_const_text(), Some("type"));
    assert_eq!(
        type_route.proof(),
        GenericMethodRouteProof::MirJsonConstValueField
    );
    assert_eq!(
        type_route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        type_route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );
    assert_eq!(
        type_route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );

    let value_route = route_for(&function, "RuntimeDataBox", "get", Some(5));
    assert_eq!(value_route.key_const_text(), Some("value"));
    assert_eq!(
        value_route.proof(),
        GenericMethodRouteProof::MirJsonConstValueField
    );
}

#[test]
fn proves_mir_json_phi_incoming_array_get_routes() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_phi_incoming_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 3, vec![4]));
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(6),
        value: crate::mir::ConstValue::Integer(1),
    });
    block.add_instruction(method_call(Some(7), "RuntimeDataBox", "get", 3, vec![6]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 3);
    let item_route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        item_route.proof(),
        GenericMethodRouteProof::MirJsonPhiIncomingArrayItem
    );
    assert_eq!(
        item_route.route_kind(),
        GenericMethodRouteKind::ArraySlotLoadAny
    );
    assert_eq!(
        item_route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        item_route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );

    let value_route = route_for(&function, "RuntimeDataBox", "get", Some(5));
    assert_eq!(
        value_route.proof(),
        GenericMethodRouteProof::MirJsonPhiIncomingPairScalar
    );
    assert_eq!(
        value_route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
    );
    assert_eq!(
        value_route.value_demand(),
        GenericMethodValueDemand::ScalarI64
    );

    let block_route = route_for(&function, "RuntimeDataBox", "get", Some(7));
    assert_eq!(
        block_route.proof(),
        GenericMethodRouteProof::MirJsonPhiIncomingPairScalar
    );
}

#[test]
fn proves_mir_json_callee_field_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_callee/1".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    let keys = ["type", "name", "box_name", "method", "receiver", "box_type"];
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
        assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonCalleeField);
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
fn proves_mir_json_vid_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_vid_array_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(route.proof(), GenericMethodRouteProof::MirJsonVidArrayItem);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64OrMissingZero)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn proves_mir_json_effects_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_effects_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonEffectsArrayItem
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
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
fn proves_mir_json_block_inst_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_block_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonBlockInstArrayItem
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
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

#[test]
fn proves_mir_json_function_block_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_function_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonFunctionBlockArrayItem
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
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
fn proves_mir_json_module_function_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_module_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonModuleFunctionArrayItem
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
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
fn proves_mir_json_params_array_item_runtime_data_get() {
    let mut function = make_function();
    function.signature.name = "MirJsonEmitBox._emit_params_rec/3".to_string();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(3));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MirJsonParamsArrayItem
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
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

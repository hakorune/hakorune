use super::*;

#[test]
fn records_direct_array_push_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(4), "ArrayBox", "push", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.push");
    assert_eq!(route.box_name(), "ArrayBox");
    assert_eq!(route.method(), "push");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), None);
    assert_eq!(route.key_value(), None);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
    let core_method = route.core_method().expect("ArrayPush carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn records_runtime_data_arraybox_push_through_copy_as_cold_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "push", 2, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.push");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "push");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox Array-origin push core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn records_runtime_data_arraybox_push_through_phi_flow_as_cold_core_method_route() {
    let mut function = make_function();
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(9),
        value: crate::mir::ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(9),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(4),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(2)),
            (BasicBlockId::new(2), ValueId::new(3)),
        ],
        type_hint: Some(MirType::Box("ArrayBox".to_string())),
    });
    merge_block.add_instruction(method_call(Some(6), "RuntimeDataBox", "push", 4, vec![5]));
    function.blocks.insert(BasicBlockId::new(1), then_block);
    function.blocks.insert(BasicBlockId::new(2), else_block);
    function.blocks.insert(BasicBlockId::new(3), merge_block);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.push");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "push");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::PushSurfacePolicy);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox Array PHI-origin push core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayPush);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn records_direct_array_and_map_set_core_method_routes() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(5), "ArrayBox", "set", 2, vec![1, 3]));
    block.add_instruction(method_call(Some(6), "MapBox", "set", 4, vec![1, 3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let array_route = &function.metadata.generic_method_routes[0];
    assert_eq!(array_route.route_id(), "generic_method.set");
    assert_eq!(array_route.box_name(), "ArrayBox");
    assert_eq!(array_route.method(), "set");
    assert_eq!(array_route.arity(), 2);
    assert_eq!(array_route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(
        array_route.key_route(),
        Some(GenericMethodKeyRoute::I64Const)
    );
    assert_eq!(array_route.key_value(), Some(ValueId::new(1)));
    assert_eq!(
        array_route.route_kind(),
        GenericMethodRouteKind::ArrayStoreAny
    );
    assert_eq!(
        array_route.proof(),
        GenericMethodRouteProof::SetSurfacePolicy
    );
    let array_core = array_route.core_method().expect("ArraySet carrier");
    assert_eq!(array_core.op, CoreMethodOp::ArraySet);
    assert_eq!(
        array_core.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
    );
    assert_eq!(array_route.return_shape(), None);
    assert_eq!(
        array_route.value_demand(),
        GenericMethodValueDemand::WriteAny
    );
    assert_eq!(array_route.publication_policy(), None);

    let map_route = &function.metadata.generic_method_routes[1];
    assert_eq!(map_route.box_name(), "MapBox");
    assert_eq!(map_route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(map_route.key_route(), Some(GenericMethodKeyRoute::I64Const));
    assert_eq!(map_route.route_kind(), GenericMethodRouteKind::MapStoreAny);
    let map_core = map_route.core_method().expect("MapSet carrier");
    assert_eq!(map_core.op, CoreMethodOp::MapSet);
    assert_eq!(map_core.lowering_tier, CoreMethodLoweringTier::ColdFallback);
    assert_eq!(map_route.return_shape(), None);
    assert_eq!(map_route.value_demand(), GenericMethodValueDemand::WriteAny);
    assert_eq!(map_route.publication_policy(), None);
}

#[test]
fn records_runtime_data_mapbox_set_through_typed_phi_as_cold_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(2),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
        type_hint: Some(MirType::Box("MapBox".to_string())),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "set", 2, vec![3, 4]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.set");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "set");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapStoreAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::SetSurfacePolicy);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox Map-origin set core method op");
    assert_eq!(core_method.op, CoreMethodOp::MapSet);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::WriteAny);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_runtime_data_arraybox_get_through_typed_phi_origin() {
    let mut function = make_function();
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(9),
        value: crate::mir::ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(9),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(1),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut merge_block = BasicBlock::new(BasicBlockId::new(3));
    merge_block.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(4),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(2)),
            (BasicBlockId::new(2), ValueId::new(3)),
        ],
        type_hint: Some(MirType::Box("ArrayBox".to_string())),
    });
    merge_block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(5),
        value: crate::mir::ConstValue::Integer(0),
    });
    merge_block.add_instruction(method_call(Some(6), "RuntimeDataBox", "get", 4, vec![5]));
    function.blocks.insert(BasicBlockId::new(1), then_block);
    function.blocks.insert(BasicBlockId::new(2), else_block);
    function.blocks.insert(BasicBlockId::new(3), merge_block);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox Array-origin get core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn promotes_runtime_data_mapbox_i64_has_to_map_contains_i64() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "has", 1, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapContainsI64);
    let core_method = route.core_method().expect("MapHas carrier");
    assert_eq!(core_method.op, CoreMethodOp::MapHas);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.map.probe_hi");
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_runtime_data_mapbox_get_as_cold_metadata_only() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.runtime_data.get_hh"
    );
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    let core_method = route.core_method().expect("MapGet carrier");
    assert_eq!(core_method.op, CoreMethodOp::MapGet);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::ColdFallback
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
        Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
    );
}

#[test]
fn proves_same_block_runtime_data_get_scalar_i64_return_shape() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![1, 2, 3]));
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MapSetScalarI64SameKeyNoEscape
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
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.runtime_data.get_hh"
    );
}

#[test]
fn rejects_same_block_get_scalar_shape_when_store_value_is_not_i64() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "StringBox".to_string(),
        args: vec![],
    });
    block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
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
        Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
    );
}

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
        assert_eq!(route.key_const_text(), Some(*key));
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

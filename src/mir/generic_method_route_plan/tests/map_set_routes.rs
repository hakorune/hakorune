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
fn leaves_runtime_data_set_metadata_absent_for_fallback() {
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
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "set", 2, vec![3, 4]));

    refresh_function_generic_method_routes(&mut function);

    assert!(function.metadata.generic_method_routes.is_empty());
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

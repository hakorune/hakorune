use super::*;

#[test]
fn records_runtime_data_has_from_typed_object_map_field_origin_with_redundant_receiver_arg() {
    let mut module = MirModule::new("typed_object_map_field_has_route_test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 3,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "chunks".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Store.birth/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "chunks".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.set_terminator(MirInstruction::Return { value: None });
    birth.add_block(birth_block);

    let mut has = MirFunction::new(
        FunctionSignature {
            name: "Store.hasChunk/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    has.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunks".to_string(),
        declared_type: None,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    block.add_instruction(method_call(Some(5), "RuntimeDataBox", "has", 4, vec![3, 1]));
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    has.add_block(block);

    module.add_function(birth);
    module.add_function(has);

    refresh_module_generic_method_routes(&mut module);

    let has = module
        .get_function("Store.hasChunk/1")
        .expect("has function");
    assert_eq!(has.metadata.generic_method_routes.len(), 1);
    let route = &has.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.has");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "has");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataContainsAny
    );
}

#[test]
fn records_runtime_data_get_for_typed_object_mapbox_result_origin() {
    let mut module = MirModule::new("typed_object_map_field_get_route_test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Store".to_string(),
        type_id: 3,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "chunks".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Store.birth/0".to_string(),
            params: vec![MirType::Box("Store".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "chunks".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.set_terminator(MirInstruction::Return { value: None });
    birth.add_block(birth_block);

    let mut put = MirFunction::new(
        FunctionSignature {
            name: "Store.put/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    put.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut put_block = BasicBlock::new(BasicBlockId::new(0));
    put_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunks".to_string(),
        declared_type: None,
    });
    put_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(3),
        box_type: "ContentChunk".to_string(),
        args: vec![],
    });
    put_block.add_instruction(method_call(Some(4), "MapBox", "set", 2, vec![1, 3]));
    put_block.set_terminator(MirInstruction::Return { value: None });
    put.add_block(put_block);

    let mut read = MirFunction::new(
        FunctionSignature {
            name: "Store.read/1".to_string(),
            params: vec![
                MirType::Box("Store".to_string()),
                MirType::Box("StringBox".to_string()),
            ],
            return_type: MirType::Box("ContentChunk".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut read_block = BasicBlock::new(BasicBlockId::new(0));
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunks".to_string(),
        declared_type: None,
    });
    read_block.add_instruction(method_call(Some(3), "MapBox", "get", 2, vec![1]));
    read_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    read.add_block(read_block);

    module.add_function(birth);
    module.add_function(put);
    module.add_function(read);

    refresh_module_generic_method_routes(&mut module);

    let read = module.get_function("Store.read/1").expect("read function");
    assert_eq!(read.metadata.generic_method_routes.len(), 1);
    let route = &read.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "MapBox");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.result_origin_box(), Some("ContentChunk"));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.runtime_data.get_hh"
    );
    assert_eq!(
        route.value_demand(),
        GenericMethodValueDemand::RuntimeI64OrHandle
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
}

#[test]
fn records_runtime_data_arraybox_push_as_cold_core_method_route() {
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
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "push", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.push");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "push");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), None);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayAppendAny);
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.array.slot_append_hh"
    );
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
fn records_runtime_data_arraybox_get_as_warm_core_method_route() {
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
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(0),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::I64Const));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.array.slot_load_hi"
    );
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
fn records_runtime_data_has_mapbox_receiver_origin_without_promotion() {
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
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 2, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataContainsAny
    );
    let core_method = route.core_method().expect("AnyHas carrier");
    assert_eq!(core_method.op, CoreMethodOp::AnyHas);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_box_has_as_any_runtime_data_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(4), "Box", "has", 1, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "Box");
    assert_eq!(route.route_id(), "generic_method.has");
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataContainsAny
    );
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.runtime_data.has_hh"
    );
    assert_eq!(route.receiver_origin_box(), Some("Box"));
    let core_method = route.core_method().expect("AnyHas carrier");
    assert_eq!(core_method.op, CoreMethodOp::AnyHas);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
}

#[test]
fn records_runtime_data_get_without_receiver_origin_as_any_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 1, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.runtime_data.get_hh"
    );
    assert_eq!(route.receiver_origin_box(), None);
    let core_method = route.core_method().expect("AnyGet carrier");
    assert_eq!(core_method.op, CoreMethodOp::AnyGet);
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
fn records_box_get_as_any_runtime_data_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(4), "Box", "get", 1, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "Box");
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::RuntimeDataLoadAny
    );
    assert_eq!(route.receiver_origin_box(), Some("Box"));
    let core_method = route.core_method().expect("AnyGet carrier");
    assert_eq!(core_method.op, CoreMethodOp::AnyGet);
}

#[test]
fn records_runtime_data_arraybox_has_as_arrayhas_core_method_route() {
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
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 2, vec![3]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "has");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayContainsAny);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.array.has_hh");
    let core_method = route.core_method().expect("ArrayHas carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayHas);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

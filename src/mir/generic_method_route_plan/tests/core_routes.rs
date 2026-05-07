use super::*;
use crate::mir::function::{TypedObjectFieldPlan, TypedObjectFieldStorage, TypedObjectPlan};

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
fn records_direct_arraybox_has_as_arrayhas_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "ArrayBox", "has", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "ArrayBox");
    assert_eq!(route.method(), "has");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArrayContainsAny);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.array.has_hh");
    assert_eq!(route.proof(), GenericMethodRouteProof::HasSurfacePolicy);
    let core_method = route.core_method().expect("ArrayBox.has core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayHas);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_direct_mapbox_get_as_warm_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "MapBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "MapBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapLoadAny);
    assert_eq!(route.route_kind().helper_symbol(), "nyash.map.slot_load_hh");
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    let core_method = route.core_method().expect("MapBox.get core method op");
    assert_eq!(core_method.op, CoreMethodOp::MapGet);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_direct_arraybox_get_as_warm_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block
        .instructions
        .push(method_call(Some(3), "ArrayBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "ArrayBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.key_route(), Some(GenericMethodKeyRoute::UnknownAny));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.array.slot_load_hi"
    );
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    let core_method = route.core_method().expect("ArrayBox.get core method op");
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
fn records_runtime_data_get_from_typed_object_array_field_origin() {
    let mut module = MirModule::new("typed_object_array_field_get_route_test".to_string());
    module.metadata.typed_object_plans.push(TypedObjectPlan {
        box_name: "Manifest".to_string(),
        type_id: 2,
        layout_kind: "runtime_slot_object_v0".to_string(),
        field_count: 1,
        fields: vec![TypedObjectFieldPlan {
            name: "chunk_ids".to_string(),
            slot: 0,
            declared_type_name: None,
            storage: TypedObjectFieldStorage::Handle,
            is_weak: false,
        }],
    });

    let mut birth = MirFunction::new(
        FunctionSignature {
            name: "Manifest.birth/0".to_string(),
            params: vec![MirType::Box("Manifest".to_string())],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    birth.params = vec![ValueId::new(0)];
    let mut birth_block = BasicBlock::new(BasicBlockId::new(0));
    birth_block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "ArrayBox".to_string(),
        args: vec![],
    });
    birth_block.add_instruction(MirInstruction::FieldSet {
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        value: ValueId::new(1),
        declared_type: None,
    });
    birth_block.set_terminator(MirInstruction::Return { value: None });
    birth.add_block(birth_block);

    let mut read = MirFunction::new(
        FunctionSignature {
            name: "Manifest.read/1".to_string(),
            params: vec![MirType::Box("Manifest".to_string()), MirType::Integer],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    read.params = vec![ValueId::new(0), ValueId::new(1)];
    let mut read_block = BasicBlock::new(BasicBlockId::new(0));
    read_block.add_instruction(MirInstruction::FieldGet {
        dst: ValueId::new(2),
        base: ValueId::new(0),
        field: "chunk_ids".to_string(),
        declared_type: None,
    });
    read_block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    read_block.add_instruction(method_call(Some(4), "RuntimeDataBox", "get", 3, vec![1]));
    read_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    read.add_block(read_block);

    module.add_function(birth);
    module.add_function(read);

    refresh_module_generic_method_routes(&mut module);

    let read = module
        .get_function("Manifest.read/1")
        .expect("read function");
    assert_eq!(read.metadata.generic_method_routes.len(), 1);
    let route = &read.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.get");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "get");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLoadAny);
    let core_method = route
        .core_method()
        .expect("RuntimeDataBox array get core method op");
    assert_eq!(core_method.op, CoreMethodOp::ArrayGet);
}

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
    assert!(route.core_method().is_none());
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
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

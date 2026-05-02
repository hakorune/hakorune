use super::*;

#[test]
fn records_i64_const_key_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: crate::mir::ConstValue::Integer(-1),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(Some(4), "RuntimeDataBox", "has", 3, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    assert_eq!(
        function.metadata.generic_method_routes[0].key_route(),
        Some(GenericMethodKeyRoute::I64Const)
    );
}

#[test]
fn records_direct_len_family_core_method_routes() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(4), "MapBox", "size", 1, vec![]));
    block.add_instruction(method_call(Some(5), "ArrayBox", "length", 2, vec![]));
    block.add_instruction(method_call(Some(6), "StringBox", "len", 3, vec![]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 3);
    let map_route = &function.metadata.generic_method_routes[0];
    assert_eq!(map_route.route_id(), "generic_method.len");
    assert_eq!(map_route.method(), "size");
    assert_eq!(map_route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(map_route.key_route(), None);
    assert_eq!(map_route.key_value(), None);
    assert_eq!(
        map_route.route_kind(),
        GenericMethodRouteKind::MapEntryCount
    );
    assert_eq!(map_route.proof(), GenericMethodRouteProof::LenSurfacePolicy);
    let map_core = map_route.core_method().expect("MapLen carrier");
    assert_eq!(map_core.op, CoreMethodOp::MapLen);
    assert_eq!(
        map_core.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(
        map_route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(
        map_route.value_demand(),
        GenericMethodValueDemand::ScalarI64
    );
    assert_eq!(
        map_route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );

    let array_route = &function.metadata.generic_method_routes[1];
    assert_eq!(array_route.method(), "length");
    assert_eq!(array_route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(
        array_route.route_kind(),
        GenericMethodRouteKind::ArraySlotLen
    );
    let array_core = array_route.core_method().expect("ArrayLen carrier");
    assert_eq!(array_core.op, CoreMethodOp::ArrayLen);

    let string_route = &function.metadata.generic_method_routes[2];
    assert_eq!(string_route.method(), "len");
    assert_eq!(string_route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(string_route.route_kind(), GenericMethodRouteKind::StringLen);
    let string_core = string_route.core_method().expect("StringLen carrier");
    assert_eq!(string_core.op, CoreMethodOp::StringLen);
}

#[test]
fn records_runtime_data_len_from_receiver_origin() {
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
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "length", 2, vec![]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "length");
    assert_eq!(route.receiver_origin_box(), Some("MapBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapEntryCount);
    let core_method = route.core_method().expect("RuntimeData MapLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::MapLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.key_route(), None);
    assert_eq!(route.key_value(), None);
    assert_eq!(route.arity(), 0);
}

#[test]
fn records_runtime_data_string_len_from_generic_global_call_origin() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.coerce/1".to_string())),
        args: vec![ValueId::new(0)],
        effects: EffectMask::PURE,
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(Some(3), "RuntimeDataBox", "length", 2, vec![]));
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "Helper.coerce/1",
            1,
            Some(ValueId::new(1)),
            GlobalCallTargetFacts::present_with_shape(
                1,
                GlobalCallTargetShape::GenericPureStringBody,
            ),
        ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "length");
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringLen);
    let core_method = route.core_method().expect("StringLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLen);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
}

#[test]
fn records_runtime_data_substring_from_generic_global_call_phi_origin() {
    let mut function = make_function();
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Helper.coerce/1".to_string())),
        args: vec![ValueId::new(0)],
        effects: EffectMask::PURE,
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    let mut merge = BasicBlock::new(BasicBlockId::new(1));
    merge.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(2),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
        type_hint: None,
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(0),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(64),
    });
    merge.add_instruction(method_call(
        Some(6),
        "RuntimeDataBox",
        "substring",
        3,
        vec![4, 5],
    ));
    function.add_block(merge);
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "Helper.coerce/1",
            1,
            Some(ValueId::new(1)),
            GlobalCallTargetFacts::present_with_shape(
                1,
                GlobalCallTargetShape::GenericPureStringBody,
            ),
        ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "substring");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringSubstring carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
}

#[test]
fn records_runtime_data_substring_from_string_concat_origin() {
    let mut function = make_function();
    function.params = vec![ValueId::new(0)];
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String(String::new()),
    });
    block.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(2),
        op: BinaryOp::Add,
        lhs: ValueId::new(1),
        rhs: ValueId::new(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(8),
    });
    block.add_instruction(method_call(
        Some(5),
        "RuntimeDataBox",
        "substring",
        2,
        vec![3, 4],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "substring");
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringSubstring carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
}

#[test]
fn records_direct_substring_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(
        Some(5),
        "StringBox",
        "substring",
        1,
        vec![2, 3],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.substring");
    assert_eq!(route.box_name(), "StringBox");
    assert_eq!(route.method(), "substring");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.key_route(), None);
    assert_eq!(route.key_value(), None);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::SubstringSurfacePolicy
    );
    let core_method = route.core_method().expect("StringSubstring carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(route.return_shape(), None);
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ReadRef);
    assert_eq!(route.publication_policy(), None);
}

#[test]
fn records_runtime_data_substring_from_string_origin() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "StringBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(
        Some(5),
        "RuntimeDataBox",
        "substring",
        2,
        vec![3, 4],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "substring");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringSubstring);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringSubstring carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringSubstring);
}

#[test]
fn records_direct_indexof_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(5), "StringBox", "indexOf", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.indexOf");
    assert_eq!(route.box_name(), "StringBox");
    assert_eq!(route.method(), "indexOf");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.key_route(), None);
    assert_eq!(route.key_value(), None);
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringIndexOf);
    assert_eq!(route.proof(), GenericMethodRouteProof::IndexOfSurfacePolicy);
    let core_method = route.core_method().expect("StringIndexOf carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringIndexOf);
    assert_eq!(
        core_method.lowering_tier,
        CoreMethodLoweringTier::WarmDirectAbi
    );
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::NoPublication)
    );
}

#[test]
fn records_runtime_data_indexof_from_string_origin() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "StringBox".to_string(),
        args: vec![],
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(method_call(
        Some(5),
        "RuntimeDataBox",
        "indexOf",
        2,
        vec![3],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.indexOf");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "indexOf");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringIndexOf);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringIndexOf carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringIndexOf);
}

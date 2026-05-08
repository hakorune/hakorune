use super::*;

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

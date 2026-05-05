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
    assert_eq!(
        string_route.route_kind().helper_symbol(),
        "nyash.string.len_fast_h"
    );
    let string_core = string_route.core_method().expect("StringLen carrier");
    assert_eq!(string_core.op, CoreMethodOp::StringLen);
}

#[test]
fn records_stringbox_length_self_arg_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: crate::mir::ConstValue::String("abc".to_string()),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(method_call(Some(4), "StringBox", "length", 3, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "StringBox");
    assert_eq!(route.method(), "length");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringLen);
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.string.len_fast_h"
    );
    let core_method = route.core_method().expect("StringLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLen);
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
fn records_runtime_data_array_len_from_phi_origin() {
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
    entry.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(2),
        src: ValueId::new(1),
    });
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut merge = BasicBlock::new(BasicBlockId::new(1));
    merge.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(3),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
        type_hint: Some(MirType::Box("ArrayBox".to_string())),
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(3),
    });
    merge.add_instruction(method_call(Some(5), "RuntimeDataBox", "size", 4, vec![]));
    function.add_block(merge);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "size");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLen);
    let core_method = route.core_method().expect("RuntimeData ArrayLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayLen);
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::ScalarI64)
    );
    assert_eq!(route.value_demand(), GenericMethodValueDemand::ScalarI64);
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
    assert_eq!(
        route.route_kind().helper_symbol(),
        "nyash.string.len_fast_h"
    );
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
fn records_runtime_data_array_len_from_static_array_global_contract() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(MirInstruction::Call {
        dst: Some(ValueId::new(1)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(
            "PatternRegistryBox.candidates/0".to_string(),
        )),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.add_instruction(method_call(Some(2), "RuntimeDataBox", "length", 1, vec![]));
    function
        .metadata
        .global_call_routes
        .push(GlobalCallRoute::new(
            GlobalCallRouteSite::new(BasicBlockId::new(0), 0),
            "PatternRegistryBox.candidates/0",
            0,
            Some(ValueId::new(1)),
            GlobalCallTargetFacts::present_static_string_array_contract(0),
        ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "length");
    assert_eq!(route.receiver_origin_box(), Some("ArrayBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::ArraySlotLen);
    let core_method = route.core_method().expect("ArrayLen carrier");
    assert_eq!(core_method.op, CoreMethodOp::ArrayLen);
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
fn records_direct_lastindexof_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(5), "StringBox", "lastIndexOf", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.lastIndexOf");
    assert_eq!(route.box_name(), "StringBox");
    assert_eq!(route.method(), "lastIndexOf");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::StringLastIndexOf
    );
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::LastIndexOfSurfacePolicy
    );
    let core_method = route.core_method().expect("StringLastIndexOf carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLastIndexOf);
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
fn records_direct_contains_core_method_route() {
    let mut function = make_function();
    let block = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    block.add_instruction(method_call(Some(5), "StringBox", "contains", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.contains");
    assert_eq!(route.box_name(), "StringBox");
    assert_eq!(route.method(), "contains");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.key_value(), Some(ValueId::new(2)));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringContains);
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::ContainsSurfacePolicy
    );
    let core_method = route.core_method().expect("StringContains carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringContains);
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
fn records_runtime_data_contains_from_param_text_copy_phi_flow() {
    let mut function = make_function();
    function.signature.params = vec![MirType::Unknown];
    function.params = vec![ValueId::new(0)];
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::String("\"version\"".to_string()),
    });
    entry.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(3),
    });
    entry.add_instruction(method_call(
        Some(2),
        "RuntimeDataBox",
        "contains",
        5,
        vec![4],
    ));
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut second = BasicBlock::new(BasicBlockId::new(1));
    second.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(11),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(0))],
        type_hint: None,
    });
    second.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(17),
        src: ValueId::new(11),
    });
    second.add_instruction(MirInstruction::Const {
        dst: ValueId::new(18),
        value: ConstValue::String("\"kind\"".to_string()),
    });
    second.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(19),
        src: ValueId::new(17),
    });
    second.add_instruction(method_call(
        Some(16),
        "RuntimeDataBox",
        "contains",
        19,
        vec![18],
    ));
    function.add_block(second);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let first = route_for(&function, "RuntimeDataBox", "contains", Some(2));
    assert_eq!(first.receiver_origin_box(), Some("StringBox"));
    assert_eq!(first.route_kind(), GenericMethodRouteKind::StringContains);
    let second = route_for(&function, "RuntimeDataBox", "contains", Some(16));
    assert_eq!(second.receiver_origin_box(), Some("StringBox"));
    assert_eq!(second.route_kind(), GenericMethodRouteKind::StringContains);
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

#[test]
fn records_runtime_data_lastindexof_from_string_origin() {
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
        "lastIndexOf",
        2,
        vec![3],
    ));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 1);
    let route = &function.metadata.generic_method_routes[0];
    assert_eq!(route.route_id(), "generic_method.lastIndexOf");
    assert_eq!(route.box_name(), "RuntimeDataBox");
    assert_eq!(route.method(), "lastIndexOf");
    assert_eq!(route.arity(), 1);
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::StringLastIndexOf
    );
    let core_method = route
        .core_method()
        .expect("RuntimeData StringLastIndexOf carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLastIndexOf);
}

#[test]
fn records_runtime_data_lastindexof_from_string_corridor_slice_origin() {
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
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(4),
    });
    block.add_instruction(method_call(
        Some(4),
        "RuntimeDataBox",
        "substring",
        1,
        vec![2, 3],
    ));
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(5),
        src: ValueId::new(4),
    });
    block.add_instruction(method_call(
        Some(7),
        "RuntimeDataBox",
        "lastIndexOf",
        5,
        vec![6],
    ));

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    refresh_function_generic_method_routes(&mut function);

    let route = route_for(&function, "RuntimeDataBox", "lastIndexOf", Some(7));
    assert_eq!(route.route_id(), "generic_method.lastIndexOf");
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(
        route.route_kind(),
        GenericMethodRouteKind::StringLastIndexOf
    );
    let core_method = route
        .core_method()
        .expect("RuntimeData StringLastIndexOf corridor carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringLastIndexOf);
}

#[test]
fn records_runtime_data_indexof_from_string_corridor_slice_phi_origin() {
    let mut function = make_function();
    let entry = function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .expect("entry");
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "StringBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(4),
    });
    entry.add_instruction(method_call(
        Some(4),
        "RuntimeDataBox",
        "substring",
        1,
        vec![2, 3],
    ));
    entry.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });

    let mut merge = BasicBlock::new(BasicBlockId::new(1));
    merge.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(5),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(4))],
        type_hint: None,
    });
    merge.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(6),
        src: ValueId::new(5),
    });
    merge.add_instruction(MirInstruction::Const {
        dst: ValueId::new(7),
        value: ConstValue::String("\"name\":\"".to_string()),
    });
    merge.add_instruction(method_call(
        Some(8),
        "RuntimeDataBox",
        "indexOf",
        6,
        vec![7],
    ));
    function.add_block(merge);

    crate::mir::refresh_function_string_corridor_facts(&mut function);
    refresh_function_generic_method_routes(&mut function);

    let route = route_for(&function, "RuntimeDataBox", "indexOf", Some(8));
    assert_eq!(route.route_id(), "generic_method.indexOf");
    assert_eq!(route.receiver_origin_box(), Some("StringBox"));
    assert_eq!(route.route_kind(), GenericMethodRouteKind::StringIndexOf);
    let core_method = route
        .core_method()
        .expect("RuntimeData StringIndexOf corridor PHI carrier");
    assert_eq!(core_method.op, CoreMethodOp::StringIndexOf);
}

#[test]
fn records_runtime_data_indexof_from_generic_global_call_phi_origin() {
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
        value: ConstValue::String("\"kind\":\"Program\"".to_string()),
    });
    merge.add_instruction(method_call(
        Some(5),
        "RuntimeDataBox",
        "indexOf",
        3,
        vec![4],
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

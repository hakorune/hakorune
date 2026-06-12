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

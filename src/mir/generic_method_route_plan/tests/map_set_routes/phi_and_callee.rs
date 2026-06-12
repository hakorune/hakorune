use super::*;

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

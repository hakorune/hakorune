use super::*;

#[test]
fn rejects_same_block_get_scalar_shape_after_unknown_same_receiver_mutation() {
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
    block.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));
    block.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
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
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
    );
}

#[test]
fn rejects_same_block_get_scalar_shape_after_different_key_same_receiver_set() {
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
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: crate::mir::ConstValue::Integer(2),
    });
    block.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
    block.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![4, 3]));
    block.add_instruction(method_call(Some(7), "RuntimeDataBox", "get", 1, vec![2]));

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 3);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(7));
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
}

#[test]
fn proves_dominating_preheader_scalar_i64_map_get_return_shape() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: crate::mir::ConstValue::Integer(7),
    });
    entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
    function.add_block(body);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MapSetScalarI64DominatesNoEscape
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
        "nyash.map.scalar_load_hi"
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapLoadScalarI64);
}

#[test]
fn rejects_dominating_preheader_scalar_shape_after_body_mutation() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: crate::mir::ConstValue::Integer(-1),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: crate::mir::ConstValue::Integer(7),
    });
    entry.add_instruction(method_call(Some(4), "MapBox", "set", 1, vec![2, 3]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.add_instruction(method_call(Some(5), "RuntimeDataBox", "get", 1, vec![2]));
    body.add_instruction(method_call(None, "MapBox", "clear", 1, vec![]));
    function.add_block(body);

    refresh_function_generic_method_routes(&mut function);

    assert_eq!(function.metadata.generic_method_routes.len(), 2);
    let route = route_for(&function, "RuntimeDataBox", "get", Some(5));
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
    );
}

#[test]
fn proves_covered_dynamic_i64_key_scalar_map_get_return_shape() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(2),
    });
    entry.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
    entry.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![3, 4]));
    entry.add_instruction(method_call(Some(7), "MapBox", "set", 1, vec![4, 2]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.successors.insert(body_id);
    body.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(10),
        inputs: vec![(entry_id, ValueId::new(2)), (body_id, ValueId::new(13))],
        type_hint: None,
    });
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(3),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(12),
        op: crate::mir::BinaryOp::Mod,
        lhs: ValueId::new(10),
        rhs: ValueId::new(11),
    });
    body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![12]));
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId::new(10),
        rhs: ValueId::new(3),
    });
    function.add_block(body);

    refresh_function_generic_method_routes(&mut function);

    let route = route_for(&function, "RuntimeDataBox", "get", Some(20));
    assert_eq!(
        route.proof(),
        GenericMethodRouteProof::MapSetScalarI64CoveredDynamicI64KeyNoEscape
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
        "nyash.map.scalar_load_hi"
    );
    assert_eq!(route.route_kind(), GenericMethodRouteKind::MapLoadScalarI64);
}

#[test]
fn rejects_covered_dynamic_i64_key_scalar_map_get_when_residue_is_missing() {
    let mut function = make_function();
    let entry_id = BasicBlockId::new(0);
    let body_id = BasicBlockId::new(1);
    let entry = function.blocks.get_mut(&entry_id).expect("entry");
    entry.successors.insert(body_id);
    entry.add_instruction(MirInstruction::NewBox {
        dst: ValueId::new(1),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Integer(0),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(1),
    });
    entry.add_instruction(MirInstruction::Const {
        dst: ValueId::new(4),
        value: ConstValue::Integer(2),
    });
    entry.add_instruction(method_call(Some(5), "MapBox", "set", 1, vec![2, 3]));
    entry.add_instruction(method_call(Some(6), "MapBox", "set", 1, vec![3, 4]));

    let mut body = BasicBlock::new(body_id);
    body.predecessors.insert(entry_id);
    body.successors.insert(body_id);
    body.add_instruction(MirInstruction::Phi {
        dst: ValueId::new(10),
        inputs: vec![(entry_id, ValueId::new(2)), (body_id, ValueId::new(13))],
        type_hint: None,
    });
    body.add_instruction(MirInstruction::Const {
        dst: ValueId::new(11),
        value: ConstValue::Integer(3),
    });
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(12),
        op: crate::mir::BinaryOp::Mod,
        lhs: ValueId::new(10),
        rhs: ValueId::new(11),
    });
    body.add_instruction(method_call(Some(20), "RuntimeDataBox", "get", 1, vec![12]));
    body.add_instruction(MirInstruction::BinOp {
        dst: ValueId::new(13),
        op: crate::mir::BinaryOp::Add,
        lhs: ValueId::new(10),
        rhs: ValueId::new(3),
    });
    function.add_block(body);

    refresh_function_generic_method_routes(&mut function);

    let route = route_for(&function, "RuntimeDataBox", "get", Some(20));
    assert_eq!(route.proof(), GenericMethodRouteProof::GetSurfacePolicy);
    assert_eq!(
        route.return_shape(),
        Some(GenericMethodReturnShape::MixedRuntimeI64OrHandle)
    );
    assert_eq!(
        route.publication_policy(),
        Some(GenericMethodPublicationPolicy::RuntimeDataFacade)
    );
}

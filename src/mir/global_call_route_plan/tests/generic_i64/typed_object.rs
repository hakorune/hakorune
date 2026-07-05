use super::*;
use crate::mir::{BinaryOp, ConstValue, TypeOpKind};

#[test]
fn refresh_module_global_call_routes_accepts_typed_object_field_i64_body() {
    let mut module = MirModule::new("global_call_typed_object_i64_body_test".to_string());
    let caller = make_function_with_global_call_args(
        "HakoAllocHeap.outstandingBlocks/0",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "HakoAllocHeap.outstandingBlocks/0".to_string(),
            params: vec![MirType::Box("HakoAllocHeap".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Box("HakoAllocHeap".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(2), MirType::Box("HakoAllocPage".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(3), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(4), MirType::Box("HakoAllocPage".to_string()));
    callee
        .metadata
        .value_types
        .insert(ValueId::new(5), MirType::Integer);
    callee
        .metadata
        .value_types
        .insert(ValueId::new(6), MirType::Integer);
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(1),
            field: "small_page".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(3),
            base: ValueId::new(2),
            field: "current_used".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(4),
            base: ValueId::new(1),
            field: "medium_page".to_string(),
            declared_type: None,
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(5),
            base: ValueId::new(4),
            field: "current_used".to_string(),
            declared_type: None,
        },
        MirInstruction::BinOp {
            dst: ValueId::new(6),
            op: BinaryOp::Add,
            lhs: ValueId::new(3),
            rhs: ValueId::new(5),
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("HakoAllocHeap.outstandingBlocks/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.definition_owner(), "generic_i64_or_leaf");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_typeop_check_object_null_guard_i64_body() {
    let mut module = MirModule::new("global_call_typeop_check_object_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "BoxHelpers.is_map/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BoxHelpers.is_map/1".to_string(),
            params: vec![MirType::Box("MapBox".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Box("MapBox".to_string()));
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(2),
            op: CompareOp::Eq,
            lhs: ValueId::new(0),
            rhs: ValueId::new(1),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut check_block = BasicBlock::new(BasicBlockId::new(2));
    check_block.instructions.push(MirInstruction::TypeOp {
        dst: ValueId::new(4),
        op: TypeOpKind::Check,
        value: ValueId::new(0),
        ty: MirType::Box("MapBox".to_string()),
    });
    check_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), check_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("BoxHelpers.is_map/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.definition_owner(), "generic_i64_or_leaf");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

#[test]
fn refresh_module_global_call_routes_accepts_typeop_check_void_param_null_guard_i64_body() {
    let mut module = MirModule::new("global_call_typeop_check_void_param_guard_test".to_string());
    let caller = make_function_with_global_call_args(
        "BoxHelpers.is_map/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BoxHelpers.is_map/1".to_string(),
            params: vec![MirType::Void],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Void);
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(2),
            op: CompareOp::Eq,
            lhs: ValueId::new(0),
            rhs: ValueId::new(1),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut null_block = BasicBlock::new(BasicBlockId::new(1));
    null_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Integer(0),
    });
    null_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    let mut check_block = BasicBlock::new(BasicBlockId::new(2));
    check_block.instructions.push(MirInstruction::TypeOp {
        dst: ValueId::new(4),
        op: TypeOpKind::Check,
        value: ValueId::new(0),
        ty: MirType::Box("MapBox".to_string()),
    });
    check_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    callee.blocks.insert(BasicBlockId::new(1), null_block);
    callee.blocks.insert(BasicBlockId::new(2), check_block);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("BoxHelpers.is_map/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_i64_body"),
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(route.proof(), "typed_global_call_generic_i64");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
}

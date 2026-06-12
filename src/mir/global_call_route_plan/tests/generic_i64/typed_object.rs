use super::*;

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

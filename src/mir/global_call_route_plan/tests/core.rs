use super::*;

#[test]
fn refresh_function_global_call_routes_records_unsupported_global_call() {
    let mut function = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    refresh_function_global_call_routes(&mut function);

    assert_eq!(function.metadata.global_call_routes.len(), 1);
    let route = &function.metadata.global_call_routes[0];
    assert_eq!(route.block(), BasicBlockId::new(0));
    assert_eq!(route.instruction_index(), 0);
    assert_eq!(route.callee_name(), "Stage1ModeContractBox.resolve_mode/0");
    assert_eq!(route.arity(), 2);
    assert_eq!(route.result_value(), Some(ValueId::new(7)));
    assert_eq!(route.tier(), "Unsupported");
    assert!(!route.target_exists());
    assert_eq!(route.target_arity(), None);
    assert_eq!(route.target_return_type(), None);
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.reason(), Some("unknown_global_callee"));
    assert_eq!(
        route.reason_detail().as_deref(),
        Some(
            "callee `Stage1ModeContractBox.resolve_mode/0` is not present in the current MIR module"
        )
    );
    assert_eq!(
        route.reason_hint(),
        Some(
            "if this is an imported static-box call, verify the import target is registered in hako.toml module_roots and that the import bundle merged its functions"
        )
    );
}

#[test]
fn refresh_function_global_call_routes_records_builtin_print_need_route() {
    let mut function = make_function_with_global_call_args("print", None, vec![ValueId::new(1)]);
    refresh_function_global_call_routes(&mut function);
    assert!(function.metadata.global_call_routes.is_empty());
    assert_eq!(function.metadata.builtin_global_call_routes.len(), 1);
    let route = &function.metadata.builtin_global_call_routes[0];
    assert_eq!(route.callee_name(), "print");
    assert_eq!(route.route_kind(), "global.print");
    assert_eq!(route.need_kind(), Some("printf"));
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_records_target_facts() {
    let mut module = MirModule::new("global_call_target_test".to_string());
    let caller = make_function_with_global_call(
        "Stage1ModeContractBox.resolve_mode/0",
        Some(ValueId::new(7)),
    );
    let callee = MirFunction::new(
        FunctionSignature {
            name: "Stage1ModeContractBox.resolve_mode/0".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Stage1ModeContractBox.resolve_mode/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(
        route.target_symbol(),
        Some("Stage1ModeContractBox.resolve_mode/0")
    );
    assert_eq!(route.target_arity(), Some(2));
    assert_eq!(route.target_return_type(), Some("i64".to_string()));
    assert_eq!(route.arity_matches(), Some(true));
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_no_string_surface")
    );
    assert_eq!(route.reason(), Some("missing_multi_function_emitter"));
}

#[test]
fn refresh_module_global_call_routes_accepts_same_module_scalar_counter_phi() {
    let mut module = MirModule::new("same_module_scalar_counter_route_test".to_string());
    let caller = make_function_with_global_call_args(
        "CounterApi.next/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "CounterApi.next/1".to_string(),
            params: vec![MirType::Box("Counter".to_string())],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    callee
        .metadata
        .value_types
        .insert(ValueId::new(0), MirType::Box("Counter".to_string()));
    for value in [2_u32, 4, 11, 12, 14, 19, 20, 23, 24, 25] {
        callee
            .metadata
            .value_types
            .insert(ValueId::new(value), MirType::Integer);
    }

    let mut entry = BasicBlock::new(BasicBlockId::new(0));
    entry.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(1),
            src: ValueId::new(0),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(2),
            base: ValueId::new(1),
            field: "counter".to_string(),
            declared_type: Some(MirType::Integer),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(4),
            base: ValueId::new(1),
            field: "counter".to_string(),
            declared_type: Some(MirType::Integer),
        },
        MirInstruction::Const {
            dst: ValueId::new(12),
            value: ConstValue::Integer(4_294_967_295),
        },
        MirInstruction::Compare {
            dst: ValueId::new(11),
            op: CompareOp::Lt,
            lhs: ValueId::new(4),
            rhs: ValueId::new(12),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(11),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut then_block = BasicBlock::new(BasicBlockId::new(1));
    then_block.instructions.extend([
        MirInstruction::Copy {
            dst: ValueId::new(13),
            src: ValueId::new(0),
        },
        MirInstruction::FieldGet {
            dst: ValueId::new(14),
            base: ValueId::new(13),
            field: "counter".to_string(),
            declared_type: Some(MirType::Integer),
        },
        MirInstruction::Const {
            dst: ValueId::new(20),
            value: ConstValue::Integer(1),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(19),
            op: BinaryOp::Add,
            lhs: ValueId::new(14),
            rhs: ValueId::new(20),
        },
        MirInstruction::FieldSet {
            base: ValueId::new(13),
            field: "counter".to_string(),
            value: ValueId::new(19),
            declared_type: Some(MirType::Integer),
        },
        MirInstruction::Copy {
            dst: ValueId::new(24),
            src: ValueId::new(2),
        },
    ]);
    then_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut else_block = BasicBlock::new(BasicBlockId::new(2));
    else_block.instructions.push(MirInstruction::Copy {
        dst: ValueId::new(25),
        src: ValueId::new(2),
    });
    else_block.set_terminator(MirInstruction::Jump {
        target: BasicBlockId::new(3),
        edge_args: None,
    });

    let mut join = BasicBlock::new(BasicBlockId::new(3));
    join.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(23),
        inputs: vec![
            (BasicBlockId::new(1), ValueId::new(24)),
            (BasicBlockId::new(2), ValueId::new(25)),
        ],
        type_hint: Some(MirType::Integer),
    });
    join.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(23)),
    });

    callee.add_block(entry);
    callee.add_block(then_block);
    callee.add_block(else_block);
    callee.add_block(join);

    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("CounterApi.next/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);
    crate::mir::same_module_definition_plan::refresh_module_same_module_definition_plans(
        &mut module,
    );

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.reason(), None, "{route:?}");
    assert_eq!(route.return_shape(), Some("ScalarI64"));
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.emit_kind(), "direct_function_call");
    assert_eq!(route.definition_owner(), "uniform_mir");
    assert_eq!(route.proof(), "typed_global_call_same_module_scalar_i64");

    let definitions = &module.functions["main"]
        .metadata
        .same_module_definition_plans;
    assert_eq!(definitions.len(), 1, "{definitions:?}");
    assert_eq!(definitions[0].target_symbol, "CounterApi.next/1");
    assert_eq!(
        definitions[0].definition_kind.as_json_name(),
        "same_module_function"
    );
    assert_eq!(definitions[0].definition_owner, "uniform_mir");
}

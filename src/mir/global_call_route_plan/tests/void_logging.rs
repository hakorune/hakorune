use super::*;

#[test]
fn refresh_module_global_call_routes_accepts_void_logging_string_body() {
    let mut module = MirModule::new("global_call_void_logging_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.log/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.log/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("log:".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Void,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.log/1".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert!(route.target_exists());
    assert_eq!(route.target_symbol(), Some("Helper.log/1"));
    assert_eq!(route.target_return_type(), Some("void".to_string()));
    assert_eq!(
        route.target_shape(),
        Some("generic_string_void_logging_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(route.target_shape_reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_void_logging"
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
    assert_eq!(route.value_demand(), "scalar_i64");
    assert_eq!(route.reason(), None);
}

#[test]
fn refresh_module_global_call_routes_accepts_void_logging_child_wrapper() {
    let mut module = MirModule::new("global_call_void_logging_child_wrapper_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.fail_reason/1",
        Some(ValueId::new(7)),
        vec![ValueId::new(1)],
    );
    let mut child = MirFunction::new(
        FunctionSignature {
            name: "Helper.log/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    child.params = vec![ValueId::new(1)];
    let child_entry = child.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    child_entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("log:".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: None,
            func: ValueId::INVALID,
            callee: Some(Callee::Global("print".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Void,
        },
    ]);
    child_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.fail_reason/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Void,
            effects: EffectMask::IO,
        },
        BasicBlockId::new(0),
    );
    wrapper.params = vec![ValueId::new(1)];
    let wrapper_entry = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String("reason:".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(3),
            op: BinaryOp::Add,
            lhs: ValueId::new(2),
            rhs: ValueId::new(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.log/1".to_string())),
            args: vec![ValueId::new(3)],
            effects: EffectMask::IO,
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Void,
        },
    ]);
    wrapper_entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    module.functions.insert("main".to_string(), caller);
    module.functions.insert("Helper.log/1".to_string(), child);
    module
        .functions
        .insert("Helper.fail_reason/1".to_string(), wrapper);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("generic_string_void_logging_body"),
        "reason={:?}",
        route.target_shape_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_generic_string_void_logging"
    );
    assert_eq!(route.return_shape(), Some("void_sentinel_i64_zero"));
}

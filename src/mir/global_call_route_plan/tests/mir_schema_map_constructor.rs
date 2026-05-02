use super::*;

fn method_call(
    dst: Option<ValueId>,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    args: Vec<ValueId>,
) -> MirInstruction {
    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

fn global_call(dst: ValueId, name: &str, args: Vec<ValueId>) -> MirInstruction {
    MirInstruction::Call {
        dst: Some(dst),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(name.to_string())),
        args,
        effects: EffectMask::PURE,
    }
}

fn mir_schema_i_function(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("type".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("i64".to_string()),
        },
        method_call(
            Some(ValueId::new(5)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(3), ValueId::new(4)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("value".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(6), ValueId::new(1)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    function
}

fn mir_schema_inst_const_function(name: &str, child_name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("op".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("const".to_string()),
        },
        method_call(
            Some(ValueId::new(6)),
            "MapBox",
            "set",
            ValueId::new(3),
            vec![ValueId::new(4), ValueId::new(5)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(7),
            value: ConstValue::String("dst".to_string()),
        },
        global_call(ValueId::new(8), child_name, vec![ValueId::new(1)]),
        method_call(
            Some(ValueId::new(9)),
            "MapBox",
            "set",
            ValueId::new(3),
            vec![ValueId::new(7), ValueId::new(8)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String("value".to_string()),
        },
        global_call(ValueId::new(11), child_name, vec![ValueId::new(2)]),
        method_call(
            Some(ValueId::new(12)),
            "MapBox",
            "set",
            ValueId::new(3),
            vec![ValueId::new(10), ValueId::new(11)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    function
}

#[test]
fn refresh_module_global_call_routes_marks_mir_schema_map_constructor_body() {
    let mut module = MirModule::new("global_call_mir_schema_map_constructor_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirSchemaBox.i/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(20)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "MirSchemaBox.i/1".to_string(),
        mir_schema_i_function("MirSchemaBox.i/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("mir_schema_map_constructor_body")
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.reason(), None);
    assert_eq!(
        route.proof(),
        "typed_global_call_mir_schema_map_constructor"
    );
    assert_eq!(route.return_shape(), Some("map_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
}

#[test]
fn refresh_module_global_call_routes_accepts_nested_mir_schema_map_constructor_body() {
    let mut module =
        MirModule::new("global_call_nested_mir_schema_map_constructor_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirSchemaBox.inst_const/2",
        Some(ValueId::new(30)),
        vec![ValueId::new(20), ValueId::new(21)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "MirSchemaBox.i/1".to_string(),
        mir_schema_i_function("MirSchemaBox.i/1"),
    );
    module.functions.insert(
        "MirSchemaBox.inst_const/2".to_string(),
        mir_schema_inst_const_function("MirSchemaBox.inst_const/2", "MirSchemaBox.i/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        Some("mir_schema_map_constructor_body")
    );
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(
        route.proof(),
        "typed_global_call_mir_schema_map_constructor"
    );
    assert_eq!(route.return_shape(), Some("map_handle"));
}

#[test]
fn refresh_module_global_call_routes_propagates_mir_schema_map_constructor_child_blocker() {
    let mut module =
        MirModule::new("global_call_mir_schema_map_constructor_child_blocker_test".to_string());
    let caller = make_function_with_global_call_args(
        "MirSchemaBox.inst_const/2",
        Some(ValueId::new(30)),
        vec![ValueId::new(20), ValueId::new(21)],
    );
    let bad_child = MirFunction::new(
        FunctionSignature {
            name: "MirSchemaBox.i/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("MirSchemaBox.i/1".to_string(), bad_child);
    module.functions.insert(
        "MirSchemaBox.inst_const/2".to_string(),
        mir_schema_inst_const_function("MirSchemaBox.inst_const/2", "MirSchemaBox.i/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_global_target_shape_unknown")
    );
    assert_eq!(
        route.target_shape_blocker_symbol(),
        Some("MirSchemaBox.i/1")
    );
    assert_eq!(
        route.target_shape_blocker_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
}

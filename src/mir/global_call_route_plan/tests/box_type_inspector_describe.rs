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

fn box_type_inspector_describe_function_with_return_type(
    name: &str,
    return_type: MirType,
) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type,
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
            value: ConstValue::String("Unknown".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("MapBox(".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "StringBox",
            "indexOf",
            ValueId::new(5),
            vec![ValueId::new(6)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("kind".to_string()),
        },
        method_call(
            Some(ValueId::new(9)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(8), ValueId::new(3)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String("is_map".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(0),
        },
        method_call(
            Some(ValueId::new(12)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(10), ValueId::new(11)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::String("is_array".to_string()),
        },
        method_call(
            Some(ValueId::new(14)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(13), ValueId::new(11)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    function
}

fn box_type_inspector_describe_function(name: &str) -> MirFunction {
    box_type_inspector_describe_function_with_return_type(name, MirType::Unknown)
}

fn arbitrary_map_return_function(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.push(MirInstruction::NewBox {
        dst: ValueId::new(2),
        box_type: "MapBox".to_string(),
        args: vec![],
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    function
}

fn box_type_inspector_describe_typed_phi_function(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Box("MapBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    function
        .metadata
        .value_types
        .insert(ValueId::new(20), MirType::Box("MapBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId::new(21), MirType::String);
    function
        .metadata
        .value_types
        .insert(ValueId::new(22), MirType::Integer);

    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("Unknown".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("MapBox(".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "StringBox",
            "indexOf",
            ValueId::new(5),
            vec![ValueId::new(6)],
        ),
        MirInstruction::Phi {
            dst: ValueId::new(20),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(2)),
                (BasicBlockId::new(1), ValueId::new(99)),
            ],
            type_hint: Some(MirType::Box("MapBox".to_string())),
        },
        MirInstruction::Phi {
            dst: ValueId::new(21),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(3)),
                (BasicBlockId::new(1), ValueId::new(98)),
            ],
            type_hint: Some(MirType::String),
        },
        MirInstruction::Phi {
            dst: ValueId::new(22),
            inputs: vec![
                (BasicBlockId::new(0), ValueId::new(7)),
                (BasicBlockId::new(1), ValueId::new(97)),
            ],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Const {
            dst: ValueId::new(23),
            value: ConstValue::String("kind".to_string()),
        },
        method_call(
            Some(ValueId::new(24)),
            "MapBox",
            "set",
            ValueId::new(20),
            vec![ValueId::new(23), ValueId::new(21)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(25),
            value: ConstValue::String("is_map".to_string()),
        },
        method_call(
            Some(ValueId::new(26)),
            "MapBox",
            "set",
            ValueId::new(20),
            vec![ValueId::new(25), ValueId::new(22)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(27),
            value: ConstValue::String("is_array".to_string()),
        },
        method_call(
            Some(ValueId::new(28)),
            "MapBox",
            "set",
            ValueId::new(20),
            vec![ValueId::new(27), ValueId::new(22)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(20)),
    });
    function
}

#[test]
fn refresh_module_global_call_routes_accepts_box_type_inspector_describe_contract() {
    let mut module = MirModule::new("global_call_box_type_inspector_describe_test".to_string());
    let caller = make_function_with_global_call_args(
        "BoxTypeInspectorBox._describe/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(20)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "BoxTypeInspectorBox._describe/1".to_string(),
        box_type_inspector_describe_function("BoxTypeInspectorBox._describe/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(route.return_shape(), Some("map_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
}

#[test]
fn refresh_module_global_call_routes_prioritizes_box_type_inspector_over_broad_map_schema() {
    let mut module =
        MirModule::new("global_call_box_type_inspector_describe_box_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "BoxTypeInspectorBox._describe/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(20)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "BoxTypeInspectorBox._describe/1".to_string(),
        box_type_inspector_describe_function_with_return_type(
            "BoxTypeInspectorBox._describe/1",
            MirType::Box("MapBox".to_string()),
        ),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(route.return_shape(), Some("map_handle"));
}

#[test]
fn refresh_module_global_call_routes_accepts_box_type_inspector_typed_phi_facts() {
    let mut module = MirModule::new("global_call_box_type_inspector_typed_phi_test".to_string());
    let caller = make_function_with_global_call_args(
        "BoxTypeInspectorBox._describe/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(20)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "BoxTypeInspectorBox._describe/1".to_string(),
        box_type_inspector_describe_typed_phi_function("BoxTypeInspectorBox._describe/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(
        route.target_shape(),
        None,
        "reason={:?} blocker={:?}/{:?}",
        route.target_shape_reason(),
        route.target_shape_blocker_symbol(),
        route.target_shape_blocker_reason()
    );
    assert_eq!(
        route.proof(),
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(route.return_shape(), Some("map_handle"));
}

#[test]
fn refresh_module_global_call_routes_rejects_arbitrary_unknown_map_return_as_box_type_inspector() {
    let mut module = MirModule::new("global_call_arbitrary_map_return_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.map/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(20)],
    );
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Helper.map/1".to_string(),
        arbitrary_map_return_function("Helper.map/1"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
}

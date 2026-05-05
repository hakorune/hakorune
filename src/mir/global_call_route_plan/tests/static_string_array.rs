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

fn static_string_array_function(name: &str) -> MirFunction {
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![],
            return_type: MirType::Box("ArrayBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("if.compare.intint".to_string()),
        },
        method_call(
            Some(ValueId::new(4)),
            "ArrayBox",
            "push",
            ValueId::new(2),
            vec![ValueId::new(2), ValueId::new(3)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::String("return.int".to_string()),
        },
        method_call(
            Some(ValueId::new(6)),
            "ArrayBox",
            "push",
            ValueId::new(2),
            vec![ValueId::new(2), ValueId::new(5)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    callee
}

#[test]
fn refresh_module_global_call_routes_marks_static_string_array_contract_direct_target() {
    let mut module = MirModule::new("global_call_static_string_array_shape_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.candidates/0", Some(ValueId::new(30)), vec![]);
    module.functions.insert("main".to_string(), caller);
    module.functions.insert(
        "Helper.candidates/0".to_string(),
        static_string_array_function("Helper.candidates/0"),
    );

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.reason(), None);
    assert_eq!(route.proof(), "typed_global_call_static_string_array");
    assert_eq!(route.return_shape(), Some("array_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
}

#[test]
fn refresh_module_global_call_routes_rejects_non_static_array_factory() {
    let mut module = MirModule::new("global_call_static_string_array_reject_test".to_string());
    let caller =
        make_function_with_global_call_args("Helper.candidates/0", Some(ValueId::new(30)), vec![]);
    let mut callee = static_string_array_function("Helper.candidates/0");
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.insert(
        2,
        MirInstruction::Const {
            dst: ValueId::new(20),
            value: ConstValue::Integer(1),
        },
    );
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.candidates/0".to_string(), callee);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), None);
    assert_eq!(
        route.target_shape_reason(),
        Some("generic_string_return_object_abi_not_handle_compatible")
    );
}

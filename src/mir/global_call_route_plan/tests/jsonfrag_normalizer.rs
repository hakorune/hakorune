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
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

#[test]
fn refresh_module_global_call_routes_marks_jsonfrag_instruction_array_normalizer_shape() {
    let mut module = MirModule::new("global_call_jsonfrag_normalizer_shape_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.normalize/1",
        Some(ValueId::new(30)),
        vec![ValueId::new(1)],
    );
    let mut normalizer = MirFunction::new(
        FunctionSignature {
            name: "Helper.normalize/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::String,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    normalizer.params = vec![ValueId::new(1)];
    let block = normalizer.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Null,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("op".to_string()),
        },
        MirInstruction::NewBox {
            dst: ValueId::new(4),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::NewBox {
            dst: ValueId::new(5),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::NewBox {
            dst: ValueId::new(6),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        method_call(
            Some(ValueId::new(7)),
            "ArrayBox",
            "push",
            ValueId::new(4),
            vec![ValueId::new(3)],
        ),
        method_call(
            Some(ValueId::new(8)),
            "MapBox",
            "get",
            ValueId::new(6),
            vec![ValueId::new(3)],
        ),
        MirInstruction::Compare {
            dst: ValueId::new(9),
            op: CompareOp::Eq,
            lhs: ValueId::new(8),
            rhs: ValueId::new(2),
        },
        method_call(
            Some(ValueId::new(10)),
            "MapBox",
            "set",
            ValueId::new(6),
            vec![ValueId::new(3), ValueId::new(1)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.normalize/1".to_string(), normalizer);

    refresh_module_global_call_routes(&mut module);

    let route = &module.functions["main"].metadata.global_call_routes[0];
    assert_eq!(route.target_shape(), Some("generic_pure_string_body"));
    assert_eq!(route.tier(), "DirectAbi");
    assert_eq!(route.reason(), None);
    assert_eq!(route.proof(), "typed_global_call_generic_pure_string");
    assert_eq!(route.return_shape(), Some("string_handle"));
    assert_eq!(route.value_demand(), "runtime_i64_or_handle");
}

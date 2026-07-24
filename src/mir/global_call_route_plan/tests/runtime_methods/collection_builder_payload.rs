use super::*;

#[test]
fn refresh_module_semantic_metadata_accepts_array_push_write_any_payload_in_string_or_void_body() {
    let mut module = MirModule::new("global_call_string_array_push_write_any_test".to_string());
    let caller = make_function_with_global_call_args(
        "Helper.delegate/1",
        Some(ValueId::new(20)),
        vec![ValueId::new(1)],
    );
    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Helper.delegate/1".to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(0)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.push(MirInstruction::Const {
        dst: ValueId::new(2),
        value: ConstValue::Bool(true),
    });
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(2),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });

    let mut miss_block = BasicBlock::new(BasicBlockId::new(1));
    miss_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(3),
        value: ConstValue::Void,
    });
    miss_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    let mut invoke_block = BasicBlock::new(BasicBlockId::new(2));
    invoke_block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(4),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(5),
            src: ValueId::new(4),
        },
        MirInstruction::Copy {
            dst: ValueId::new(6),
            src: ValueId::new(0),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ArrayBox".to_string(),
                method: "push".to_string(),
                receiver: Some(ValueId::new(5)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: vec![ValueId::new(5), ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("env.mirbuilder".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::String("emit".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
            args: vec![ValueId::new(8), ValueId::new(9), ValueId::new(5)],
            effects: EffectMask::IO,
        },
    ]);
    invoke_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(10)),
    });
    callee.blocks.insert(BasicBlockId::new(1), miss_block);
    callee.blocks.insert(BasicBlockId::new(2), invoke_block);
    module.functions.insert("main".to_string(), caller);
    module
        .functions
        .insert("Helper.delegate/1".to_string(), callee);

    refresh_module_semantic_metadata(&mut module);

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
        "typed_global_call_generic_string_or_void_sentinel"
    );
    assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    let callee = &module.functions["Helper.delegate/1"];
    assert!(callee.metadata.generic_method_routes.iter().any(|route| {
        route.route_id() == "generic_method.push"
            && route.method() == "push"
            && route.receiver_origin_box() == Some("ArrayBox")
            && route.route_kind_tag() == "array_append_any"
            && route.helper_symbol() == "nyash.array.slot_append_hh"
            && route.value_demand().as_metadata_name() == "write_any"
    }));
}

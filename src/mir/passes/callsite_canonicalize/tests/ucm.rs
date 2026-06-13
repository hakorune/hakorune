use super::*;

#[test]
fn ucm1_rewrites_runtime_data_union_method_call_to_known_user_box_method() {
    let mut module = MirModule::new("ucm1_method".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Counter".to_string(), vec!["value".to_string()]);
    module.metadata.user_box_field_decls.insert(
        "Counter".to_string(),
        vec![UserBoxFieldDecl {
            name: "value".to_string(),
            declared_type_name: Some("IntegerBox".to_string()),
            is_weak: false,
        }],
    );

    let signature = FunctionSignature {
        name: "ucm1_method/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    func.metadata
        .value_types
        .insert(ValueId(1), MirType::Box("Counter".to_string()));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "RuntimeDataBox".to_string(),
            method: "step".to_string(),
            receiver: Some(ValueId(1)),
            certainty: TypeCertainty::Union,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);

    let inst = &module
        .get_function("ucm1_method/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];
    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst: Some(ValueId(3)),
            func,
            callee: Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args,
            effects,
        } if *func == ValueId::INVALID
            && box_name == "Counter"
            && method == "step"
            && *receiver == ValueId(1)
            && args.is_empty()
            && *effects == EffectMask::PURE
    ));
}

#[test]
fn ucm1_rewrites_user_box_global_method_call_to_canonical_method_shape() {
    let mut module = MirModule::new("ucm1_global".to_string());
    module
        .metadata
        .user_box_decls
        .insert("Counter".to_string(), vec!["value".to_string()]);

    let signature = FunctionSignature {
        name: "ucm1_global/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    func.metadata
        .value_types
        .insert(ValueId(1), MirType::Box("Counter".to_string()));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(4)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("Counter.step/0".to_string())),
        args: vec![ValueId(1)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);

    let inst = &module
        .get_function("ucm1_global/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];
    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst: Some(ValueId(4)),
            func,
            callee: Some(Callee::Method {
                box_name,
                method,
                receiver: Some(receiver),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args,
            effects,
        } if *func == ValueId::INVALID
            && box_name == "Counter"
            && method == "step"
            && *receiver == ValueId(1)
            && args.is_empty()
            && *effects == EffectMask::PURE
    ));
}

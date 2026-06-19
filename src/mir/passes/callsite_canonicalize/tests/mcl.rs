use super::*;

#[test]
fn mcl5_rewrites_legacy_call_with_const_string_func_to_global_callee() {
    let mut module = MirModule::new("mcl5".to_string());
    let signature = FunctionSignature {
        name: "mcl5/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::String("RewriteKnownMini.run/1".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId(1),
        callee: None,
        args: vec![ValueId(2)],
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
        .get_function("mcl5/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[2];

    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst,
            func,
            callee: Some(Callee::Global(name)),
            args,
            effects,
        } if *dst == Some(ValueId(3))
            && *func == ValueId::INVALID
            && name == "RewriteKnownMini.run/1"
            && args == &vec![ValueId(2)]
            && *effects == EffectMask::PURE
    ));
}

#[test]
fn mcl5_suffixes_unsuffixed_qualified_global_name_when_matching_arity_exists() {
    let mut module = MirModule::new("mcl5_suffix".to_string());
    let callee_sig = FunctionSignature {
        name: "RewriteKnownMini.run/1".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    module.add_function(MirFunction::new(callee_sig, BasicBlockId(0)));

    let signature = FunctionSignature {
        name: "mcl5_suffix/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::String("RewriteKnownMini.run".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId(1),
        callee: None,
        args: vec![ValueId(2)],
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
        .get_function("mcl5_suffix/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[2];

    assert!(matches!(
        inst,
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if name == "RewriteKnownMini.run/1"
    ));
}

#[test]
fn stage1_buildbox_emit_program_json_null_opts_stays_global_call() {
    let mut module = MirModule::new("stage1_buildbox_emit_program_json".to_string());
    let signature = FunctionSignature {
        name: "caller/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::String("source".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Void,
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global(
            "BuildBox.emit_program_json_v0/2".to_string(),
        )),
        args: vec![ValueId(1), ValueId(2)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(3)),
    });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);

    let inst = &module
        .get_function("caller/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[2];

    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst,
            func,
            callee: Some(Callee::Global(name)),
            args,
            effects,
        } if *dst == Some(ValueId(3))
            && *func == ValueId::INVALID
            && name == "BuildBox.emit_program_json_v0/2"
            && args == &vec![ValueId(1), ValueId(2)]
            && *effects == EffectMask::PURE
    ));
}

#[test]
fn mcl5_suffixes_existing_global_callee_when_matching_arity_exists() {
    let mut module = MirModule::new("mcl5_global_suffix".to_string());
    let callee_sig = FunctionSignature {
        name: "RewriteKnownMini.run/1".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    module.add_function(MirFunction::new(callee_sig, BasicBlockId(0)));

    let signature = FunctionSignature {
        name: "mcl5_global_suffix/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("RewriteKnownMini.run".to_string())),
        args: vec![ValueId(2)],
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
        .get_function("mcl5_global_suffix/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[1];

    assert!(matches!(
        inst,
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if name == "RewriteKnownMini.run/1"
    ));
}

#[test]
fn mcl6_rewrites_speculative_user_global_to_runtime_method_when_receiver_type_is_runtime_data() {
    let mut module = MirModule::new("mcl6_runtime_receiver".to_string());
    let user_callee_sig = FunctionSignature {
        name: "JsonNodeInstance.length/0".to_string(),
        params: vec![MirType::Box("JsonNodeInstance".to_string())],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    module.add_function(MirFunction::new(user_callee_sig, BasicBlockId(0)));

    let signature = FunctionSignature {
        name: "main/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    func.metadata
        .value_types
        .insert(ValueId(10), MirType::Box("ArrayBox".to_string()));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");

    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(11)),
        func: ValueId::INVALID,
        callee: Some(Callee::Global("JsonNodeInstance.length/0".to_string())),
        args: vec![ValueId(10)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(11)),
    });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);

    let inst = &module
        .get_function("main/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];

    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst,
            func,
            callee:
                Some(Callee::Method {
                    box_name,
                    method,
                    receiver: Some(receiver),
                    certainty,
                    box_kind,
                }),
            args,
            effects,
        } if *dst == Some(ValueId(11))
            && *func == ValueId::INVALID
            && box_name == "ArrayBox"
            && method == "length"
            && *receiver == ValueId(10)
            && *certainty == TypeCertainty::Known
            && *box_kind == CalleeBoxKind::RuntimeData
            && args.is_empty()
            && *effects == EffectMask::PURE
    ));
}

#[test]
fn mcl4_no_legacy_callsite_variants_after_rcl3() {
    let mut module = MirModule::new("mcl4".to_string());
    let signature = FunctionSignature {
        name: "mcl4/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::IO,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(10)),
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: "StringBox".to_string(),
            method: "id".to_string(),
            receiver: Some(ValueId(2)),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args: vec![ValueId(3)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Extern("env.console.log".to_string())),
        args: vec![ValueId(10)],
        effects: EffectMask::IO,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return { value: None });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0, "canonical calls should remain unchanged");

    let instructions = &module
        .get_function("mcl4/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions;

    assert!(matches!(
        &instructions[0],
        MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        }
    ));
    assert!(matches!(
        &instructions[1],
        MirInstruction::Call {
            callee: Some(Callee::Extern(_)),
            ..
        }
    ));
}

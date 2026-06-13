use super::*;

#[test]
fn ncl0_rewrites_call_closure_to_newclosure() {
    let mut module = MirModule::new("ncl0".to_string());
    let signature = FunctionSignature {
        name: "ncl0/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(9)),
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![("outer".to_string(), ValueId(3))],
            me_capture: Some(ValueId(4)),
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return { value: None });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);

    let inst = &module
        .get_function("ncl0/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];
    assert!(matches!(
        inst,
        MirInstruction::NewClosure {
            dst,
            params,
            body_id,
            body,
            captures,
            me
        } if *dst == ValueId(9)
            && params == &vec!["x".to_string()]
            && *body_id == None
            && body.is_empty()
            && captures == &vec![("outer".to_string(), ValueId(3))]
            && *me == Some(ValueId(4))
    ));
}

#[test]
fn ncl0_does_not_rewrite_closure_call_with_runtime_args() {
    let mut module = MirModule::new("ncl0_args".to_string());
    let signature = FunctionSignature {
        name: "ncl0_args/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(9)),
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![("outer".to_string(), ValueId(3))],
            me_capture: None,
        }),
        args: vec![ValueId(8)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return { value: None });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);
    let inst = &module
        .get_function("ncl0_args/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];
    assert!(matches!(
        inst,
        MirInstruction::Call {
            callee: Some(Callee::Closure { .. }),
            args,
            ..
        } if args == &vec![ValueId(8)]
    ));
}

#[test]
fn ncl2_does_not_rewrite_closure_call_without_dst() {
    let mut module = MirModule::new("ncl2_missing_dst".to_string());
    let signature = FunctionSignature {
        name: "ncl2_missing_dst/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Call {
        dst: None,
        func: ValueId::INVALID,
        callee: Some(Callee::Closure {
            params: vec!["x".to_string()],
            captures: vec![],
            me_capture: None,
        }),
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return { value: None });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 0);
    let inst = &module
        .get_function("ncl2_missing_dst/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];
    assert!(matches!(
        inst,
        MirInstruction::Call {
            dst: None,
            callee: Some(Callee::Closure { .. }),
            args,
            ..
        } if args.is_empty()
    ));
}

#[test]
fn ncl1_externalizes_inline_newclosure_body() {
    let mut module = MirModule::new("ncl1".to_string());
    let signature = FunctionSignature {
        name: "ncl1/0".to_string(),
        params: vec![],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));

    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    let inline_body = vec![crate::ast::ASTNode::Literal {
        value: crate::ast::LiteralValue::Integer(1),
        span: Span::unknown(),
    }];
    block.instructions.push(MirInstruction::NewClosure {
        dst: ValueId(11),
        params: vec!["x".to_string()],
        body_id: None,
        body: inline_body.clone(),
        captures: vec![],
        me: None,
    });
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return { value: None });
    module.add_function(func);

    let rewritten = canonicalize_callsites(&mut module);
    assert_eq!(rewritten, 1);

    let inst = &module
        .get_function("ncl1/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[0];

    let body_id = match inst {
        MirInstruction::NewClosure { body_id, body, .. } => {
            assert!(body.is_empty(), "inline body must be externalized");
            body_id.expect("body_id must be assigned")
        }
        _ => panic!("expected NewClosure after canonicalization"),
    };

    assert_eq!(
        module.metadata.closure_bodies.get(&body_id),
        Some(&inline_body)
    );
}

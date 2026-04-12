use super::super::*;

#[test]
fn rewrites_single_use_substring_length_chain_to_direct_extern() {
    let mut module = MirModule::new("substring_len".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("StringBox".to_string()),
            MirType::Integer,
            MirType::Integer,
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(3),
        ValueId(0),
        "StringBox",
        "substring",
        vec![ValueId(1), ValueId(2)],
        MirType::Box("StringBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(4),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("StringBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert_eq!(block.instructions.len(), block.instruction_spans.len());
    assert_eq!(block.instructions.len(), 1);
    match &block.instructions[0] {
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } => {
            assert_eq!(*dst, Some(ValueId(4)));
            assert_eq!(name, SUBSTRING_LEN_EXTERN);
            assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
        }
        other => panic!("expected direct extern rewrite, got {other:?}"),
    }
}

#[test]
fn rewrites_runtime_data_substring_length_chain_through_copy_chain() {
    let mut module = MirModule::new("substring_len_runtime_data".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("RuntimeDataBox".to_string()),
            MirType::Integer,
            MirType::Integer,
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(3),
        ValueId(0),
        "RuntimeDataBox",
        "substring",
        vec![ValueId(1), ValueId(2)],
        MirType::Box("RuntimeDataBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(4),
        src: ValueId(3),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Copy {
        dst: ValueId(5),
        src: ValueId(4),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(6),
        ValueId(5),
        "RuntimeDataBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(6)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Box("RuntimeDataBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(6), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 1);

    let function = module.get_function("main").expect("main");
    let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
    assert_eq!(block.instructions.len(), 3);
    match &block.instructions[2] {
        MirInstruction::Call {
            dst,
            callee: Some(Callee::Extern(name)),
            args,
            ..
        } => {
            assert_eq!(*dst, Some(ValueId(6)));
            assert_eq!(name, SUBSTRING_LEN_EXTERN);
            assert_eq!(args, &vec![ValueId(0), ValueId(1), ValueId(2)]);
        }
        other => panic!("expected direct extern rewrite, got {other:?}"),
    }
}

#[test]
fn keeps_substring_when_result_has_multiple_uses() {
    let mut module = MirModule::new("substring_len_multiuse".to_string());
    let signature = FunctionSignature {
        name: "main".to_string(),
        params: vec![
            MirType::Box("StringBox".to_string()),
            MirType::Integer,
            MirType::Integer,
        ],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId(0));
    let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");

    block.instructions.push(method_call(
        ValueId(3),
        ValueId(0),
        "StringBox",
        "substring",
        vec![ValueId(1), ValueId(2)],
        MirType::Box("StringBox".to_string()),
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(4),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(method_call(
        ValueId(5),
        ValueId(3),
        "StringBox",
        "length",
        vec![],
        MirType::Integer,
    ));
    block.instruction_spans.push(Span::unknown());
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });

    function
        .metadata
        .value_types
        .insert(ValueId(3), MirType::Box("StringBox".to_string()));
    function
        .metadata
        .value_types
        .insert(ValueId(4), MirType::Integer);
    function
        .metadata
        .value_types
        .insert(ValueId(5), MirType::Integer);
    module.add_function(function);

    let rewritten = sink_borrowed_string_corridors(&mut module);
    assert_eq!(rewritten, 0);
}

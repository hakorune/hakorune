use super::super::*;


#[test]
fn single_use_substring_length_rewrites_with_or_without_copy_chain() {
    for (
        case_name,
        module_name,
        receiver_box,
        aliases,
        length_value,
        expected_rewritten,
        check_instruction_spans,
    ) in [
        (
            "rewrites_single_use_substring_length_chain_to_direct_extern",
            "substring_len",
            "StringBox",
            &[][..],
            ValueId(4),
            1,
            true,
        ),
        (
            "rewrites_runtime_data_substring_length_chain_through_copy_chain",
            "substring_len_runtime_data",
            "RuntimeDataBox",
            &[ValueId(4), ValueId(5)],
            ValueId(6),
            3,
            false,
        ),
    ] {
        println!("case: {case_name}");
        let mut module = MirModule::new(module_name.to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![
                MirType::Box(receiver_box.to_string()),
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
            receiver_box,
            "substring",
            vec![ValueId(1), ValueId(2)],
            MirType::Box(receiver_box.to_string()),
        ));
        block.instruction_spans.push(Span::unknown());
        let mut length_receiver = ValueId(3);
        for &alias in aliases {
            block.instructions.push(MirInstruction::Copy {
                src: length_receiver,
                dst: alias,
            });
            block.instruction_spans.push(Span::unknown());
            length_receiver = alias;
        }
        block.instructions.push(method_call(
            length_value,
            length_receiver,
            receiver_box,
            "length",
            vec![],
            MirType::Integer,
        ));
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(length_value),
        });

        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Box(receiver_box.to_string()));
        for &alias in aliases {
            function
                .metadata
                .value_types
                .insert(alias, MirType::Box(receiver_box.to_string()));
        }
        function
            .metadata
            .value_types
            .insert(length_value, MirType::Integer);
        module.add_function(function);

        let rewritten = sink_borrowed_string_corridors(&mut module);
        assert_eq!(rewritten, expected_rewritten);

        let function = module.get_function("main").expect("main");
        let block = function.blocks.get(&BasicBlockId(0)).expect("entry");
        if check_instruction_spans {
            assert_eq!(block.instructions.len(), block.instruction_spans.len());
        }
        assert_eq!(block.instructions.len(), 1);
        assert!(
            is_extern_call(
                &block.instructions[0],
                length_value,
                SUBSTRING_LEN_EXTERN,
                &[ValueId(0), ValueId(1), ValueId(2)],
                Some(EffectMask::PURE),
            ),
            "expected direct extern rewrite, got {:?}",
            block.instructions[0]
        );
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
    assert_eq!(rewritten, 1);
}

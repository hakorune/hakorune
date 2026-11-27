use super::*;


#[test]
fn string_or_void_sentinel_body_preserves_declared_return_type() {
    for (case_name, module_name, symbol, return_type, text, expected_return_type) in [
        (
            "refresh_module_global_call_routes_accepts_string_typed_null_sentinel_body",
            "global_call_string_typed_null_sentinel_test",
            "Helper.typed_maybe_text/0",
            MirType::String,
            "ok",
            "str",
        ),
        (
            "refresh_module_global_call_routes_accepts_integer_typed_string_or_void_sentinel_body",
            "global_call_integer_typed_string_or_void_test",
            "Helper.integer_typed_digits/0",
            MirType::Integer,
            "123",
            "i64",
        ),
    ] {
        println!("case: {case_name}");
        let mut module = MirModule::new(module_name.to_string());
        let caller = make_function_with_global_call_args(symbol, Some(ValueId::new(7)), vec![]);
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: vec![],
                return_type,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::Bool(true),
        });
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(1),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::String(text.to_string()),
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        let mut null_block = BasicBlock::new(BasicBlockId::new(2));
        null_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        });
        null_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(3)),
        });
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), null_block);
        module.functions.insert("main".to_string(), caller);
        module.functions.insert(symbol.to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_return_type(),
            Some(expected_return_type.to_string())
        );
        assert_eq!(
            route.target_shape(),
            None,
            "reason={:?}",
            route.target_shape_reason()
        );
        assert_eq!(
            route.proof(),
            "typed_global_call_generic_string_or_void_sentinel"
        );
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    }
}

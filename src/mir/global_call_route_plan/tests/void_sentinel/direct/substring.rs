use super::*;


#[test]
fn substring_void_sentinel_uses_declared_type_or_corridor_fact() {
    for (
        case_name,
        module_name,
        symbol,
        call_result,
        parameters,
        parameter_types,
        entry_constants,
        condition,
        slice_bounds,
        has_corridor_fact,
    ) in [
        (
            "refresh_module_global_call_routes_accepts_substring_void_sentinel_body",
            "global_call_substring_void_sentinel_test",
            "Helper.slice_or_null/1",
            7,
            vec![ValueId::new(1)],
            vec![MirType::String],
            vec![
                MirInstruction::Const {
                    dst: ValueId::new(2),
                    value: ConstValue::Bool(true),
                },
                MirInstruction::Const {
                    dst: ValueId::new(3),
                    value: ConstValue::Integer(0),
                },
                MirInstruction::Const {
                    dst: ValueId::new(4),
                    value: ConstValue::Integer(4),
                },
            ],
            2,
            vec![ValueId::new(3), ValueId::new(4)],
            false,
        ),
        (
            "refresh_module_global_call_routes_accepts_corridor_fact_substring_void_sentinel_body",
            "global_call_corridor_fact_substring_void_sentinel_test",
            "Helper.corridor_slice_or_null/3",
            9,
            vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
            vec![MirType::Unknown; 3],
            vec![MirInstruction::Const {
                dst: ValueId::new(4),
                value: ConstValue::Bool(true),
            }],
            4,
            vec![ValueId::new(2), ValueId::new(3)],
            true,
        ),
    ] {
        println!("case: {case_name}");
        let mut module = MirModule::new(module_name.to_string());
        let caller = make_function_with_global_call_args(
            symbol,
            Some(ValueId::new(call_result)),
            parameters.clone(),
        );
        let mut callee = MirFunction::new(
            FunctionSignature {
                name: symbol.to_string(),
                params: parameter_types,
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        callee.params = parameters;
        let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
        entry.instructions.extend(entry_constants);
        entry.set_terminator(MirInstruction::Branch {
            condition: ValueId::new(condition),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: None,
            else_edge_args: None,
        });
        let mut text_block = BasicBlock::new(BasicBlockId::new(1));
        text_block.instructions.push(MirInstruction::LegacyCallV0 {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "RuntimeDataBox".to_string(),
                method: "substring".to_string(),
                receiver: Some(ValueId::new(1)),
                certainty: TypeCertainty::Union,
                box_kind: CalleeBoxKind::RuntimeData,
            }),
            args: slice_bounds,
            effects: EffectMask::PURE,
        });
        text_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(5)),
        });
        let mut void_block = BasicBlock::new(BasicBlockId::new(2));
        void_block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Void,
        });
        void_block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(6)),
        });
        if has_corridor_fact {
            callee.metadata.string_corridor_facts.insert(
                ValueId::new(5),
                StringCorridorFact::str_slice(StringCorridorCarrier::MethodCall),
            );
        }
        callee.blocks.insert(BasicBlockId::new(1), text_block);
        callee.blocks.insert(BasicBlockId::new(2), void_block);
        module.functions.insert("main".to_string(), caller);
        module.functions.insert(symbol.to_string(), callee);

        refresh_module_global_call_routes(&mut module);

        let route = &module.functions["main"].metadata.global_call_routes[0];
        assert_eq!(
            route.target_shape(),
            None,
            "reason={:?}",
            route.target_shape_reason()
        );
        assert_eq!(route.target_shape_reason(), None);
        assert_eq!(
            route.proof(),
            "typed_global_call_generic_string_or_void_sentinel"
        );
        assert_eq!(route.return_shape(), Some("string_handle_or_null"));
    }
}

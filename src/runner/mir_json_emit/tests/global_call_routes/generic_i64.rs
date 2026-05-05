use super::*;

#[test]
fn build_mir_json_root_emits_direct_plan_for_generic_i64_global_call() {
    let mut module = crate::mir::MirModule::new("json_global_call_generic_i64_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.debug/0".to_string())),
            args: vec![],
            effects: EffectMask::PURE,
        });

    let mut wrapper = MirFunction::new(
        FunctionSignature {
            name: "Helper.debug/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let wrapper_block = wrapper.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    wrapper_block.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("DEBUG".to_string()),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("Helper.flag/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    wrapper_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    let mut flag = MirFunction::new(
        FunctionSignature {
            name: "Helper.flag/1".to_string(),
            params: vec![MirType::String],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    flag.params = vec![ValueId::new(1)];
    let entry = flag.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Extern("env.get/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::Void,
        },
        MirInstruction::Compare {
            dst: ValueId::new(4),
            op: CompareOp::Ne,
            lhs: ValueId::new(2),
            rhs: ValueId::new(3),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(4),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut yes_block = BasicBlock::new(BasicBlockId::new(1));
    yes_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(5),
        value: ConstValue::Integer(1),
    });
    yes_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(5)),
    });
    let mut no_block = BasicBlock::new(BasicBlockId::new(2));
    no_block.instructions.push(MirInstruction::Const {
        dst: ValueId::new(6),
        value: ConstValue::Integer(0),
    });
    no_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    flag.blocks.insert(BasicBlockId::new(1), yes_block);
    flag.blocks.insert(BasicBlockId::new(2), no_block);

    module.add_function(caller);
    module.add_function(wrapper);
    module.add_function(flag);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], "generic_i64_body");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_generic_i64");
    assert_eq!(route["return_shape"], "ScalarI64");
    assert_eq!(route["value_demand"], "scalar_i64");
    assert_eq!(route["result_origin"], "none");
    assert_eq!(route["reason"], serde_json::Value::Null);

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], "generic_i64_body");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["proof"], "typed_global_call_generic_i64");
    assert_eq!(plan["route_proof"], "typed_global_call_generic_i64");
    assert_eq!(plan["return_shape"], "ScalarI64");
    assert_eq!(plan["value_demand"], "scalar_i64");
    assert_eq!(plan["result_origin"], "none");
    assert_eq!(plan["reason"], serde_json::Value::Null);
}

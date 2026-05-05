use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

#[test]
fn build_mir_json_root_emits_direct_plan_for_parser_program_json_contract() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_parser_program_json_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(20)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("BuildBox._parse_program_json/1".to_string())),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BuildBox._parse_program_json/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "ParserBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(3),
            src: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(4)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(5),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(6)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "stage3_enable".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(5)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "parse_program2".to_string(),
                receiver: Some(ValueId::new(3)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(7)),
    });

    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_parser_program_json");
    assert_eq!(route["return_shape"], "string_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["result_origin"], "string");

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["proof"], "typed_global_call_parser_program_json");
    assert_eq!(plan["route_proof"], "typed_global_call_parser_program_json");
    assert_eq!(plan["return_shape"], "string_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["result_origin"], "string");
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_program_json_emit_body() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_program_json_emit_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(20)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                "BuildBox._emit_program_json_from_scan_src/1".to_string(),
            )),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BuildBox._emit_program_json_from_scan_src/1".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                "BuildBox._parse_program_json_from_scan_src/1".to_string(),
            )),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("BuildBox._is_freeze_tag/1".to_string())),
            args: vec![ValueId::new(2)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Compare {
            dst: ValueId::new(5),
            op: CompareOp::Eq,
            lhs: ValueId::new(3),
            rhs: ValueId::new(4),
        },
    ]);
    entry.set_terminator(MirInstruction::Branch {
        condition: ValueId::new(5),
        then_bb: BasicBlockId::new(1),
        else_bb: BasicBlockId::new(2),
        then_edge_args: None,
        else_edge_args: None,
    });
    let mut freeze_block = BasicBlock::new(BasicBlockId::new(1));
    freeze_block.instructions.push(MirInstruction::Phi {
        dst: ValueId::new(6),
        inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
        type_hint: Some(MirType::Integer),
    });
    freeze_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(6)),
    });
    let mut enrich_block = BasicBlock::new(BasicBlockId::new(2));
    enrich_block.instructions.extend([
        MirInstruction::Phi {
            dst: ValueId::new(7),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(2))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Phi {
            dst: ValueId::new(8),
            inputs: vec![(BasicBlockId::new(0), ValueId::new(1))],
            type_hint: Some(MirType::Integer),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(9)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                "BuildProgramFragmentBox.enrich/2".to_string(),
            )),
            args: vec![ValueId::new(7), ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    enrich_block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(9)),
    });
    callee.blocks.insert(BasicBlockId::new(1), freeze_block);
    callee.blocks.insert(BasicBlockId::new(2), enrich_block);

    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], "generic_pure_string_body");
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(route["proof"], "typed_global_call_generic_pure_string");
    assert_eq!(route["return_shape"], "string_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], "generic_pure_string_body");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["route_proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["return_shape"], "string_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
}

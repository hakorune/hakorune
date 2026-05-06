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
            callee: Some(Callee::Global("BuildBox._parse_program_json/2".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "BuildBox._parse_program_json/2".to_string(),
            params: vec![MirType::Integer, MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1), ValueId::new(2)];
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(3),
            box_type: "ParserBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(4),
            src: ValueId::new(3),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(5)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "birth".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::Integer(1),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(7)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "stage3_enable".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(6)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Copy {
            dst: ValueId::new(8),
            src: ValueId::new(1),
        },
        MirInstruction::Copy {
            dst: ValueId::new(9),
            src: ValueId::new(2),
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "set_enum_inventory_from_source".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(9)],
            effects: EffectMask::PURE,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(11)),
            func: ValueId::INVALID,
            callee: Some(Callee::Method {
                box_name: "ParserBox".to_string(),
                method: "parse_program2".to_string(),
                receiver: Some(ValueId::new(4)),
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![ValueId::new(8)],
            effects: EffectMask::PURE,
        },
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(11)),
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
    assert_eq!(route["definition_owner"], "diagnostics_only");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_diagnostics_only_emit"
    );

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
    assert_eq!(plan["definition_owner"], "diagnostics_only");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_diagnostics_only_emit"
    );
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
                "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1".to_string(),
            )),
            args: vec![ValueId::new(1)],
            effects: EffectMask::PURE,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
                .to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    callee.params = vec![ValueId::new(1)];
    let entry = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    entry.instructions.extend([
        MirInstruction::Const {
            dst: ValueId::new(2),
            value: ConstValue::Void,
        },
        MirInstruction::Call {
            dst: Some(ValueId::new(3)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("BuildBox.emit_program_json_v0/2".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2)],
            effects: EffectMask::PURE,
        },
    ]);
    entry.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

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
    assert_eq!(
        route["target_symbol"],
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
    );
    assert_eq!(
        route["symbol"],
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
    );
    assert_eq!(route["return_shape"], "string_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["definition_owner"], "module_generic");
    assert_eq!(route["emit_trace_consumer"], "mir_call_global_module_generic_emit");

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], "generic_pure_string_body");
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(
        plan["target_symbol"],
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
    );
    assert_eq!(
        plan["symbol"],
        "Stage1SourceProgramAuthorityBox._emit_program_json_from_source_raw/1"
    );
    assert_eq!(plan["proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["route_proof"], "typed_global_call_generic_pure_string");
    assert_eq!(plan["return_shape"], "string_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["definition_owner"], "module_generic");
    assert_eq!(plan["emit_trace_consumer"], "mir_call_global_module_generic_emit");
}

#[test]
fn build_mir_json_root_emits_runtime_plan_for_buildbox_emit_program_json_null_opts_callsite() {
    let mut module =
        crate::mir::MirModule::new("json_buildbox_emit_program_json_null_opts_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .extend([
            MirInstruction::Const {
                dst: ValueId::new(1),
                value: ConstValue::String("source".to_string()),
            },
            MirInstruction::Const {
                dst: ValueId::new(2),
                value: ConstValue::Void,
            },
            MirInstruction::Call {
                dst: Some(ValueId::new(3)),
                func: ValueId::INVALID,
                callee: Some(Callee::Global("BuildBox.emit_program_json_v0/2".to_string())),
                args: vec![ValueId::new(1), ValueId::new(2)],
                effects: EffectMask::PURE,
            },
        ]);
    module.add_function(caller);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_exists"], false);
    assert_eq!(route["target_symbol"], "nyash.stage1.emit_program_json_v0_h");
    assert_eq!(route["tier"], "ColdRuntime");
    assert_eq!(route["emit_kind"], "runtime_call");
    assert_eq!(route["proof"], "typed_global_call_stage1_emit_program_json");
    assert_eq!(route["route_kind"], "stage1.emit_program_json_v0");
    assert_eq!(route["symbol"], "nyash.stage1.emit_program_json_v0_h");
    assert_eq!(route["return_shape"], "string_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["result_origin"], "string");
    assert_eq!(route["definition_owner"], "runtime_helper");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_stage1_emit_program_json_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_symbol"], "nyash.stage1.emit_program_json_v0_h");
    assert_eq!(plan["tier"], "ColdRuntime");
    assert_eq!(plan["emit_kind"], "runtime_call");
    assert_eq!(plan["proof"], "typed_global_call_stage1_emit_program_json");
    assert_eq!(plan["route_proof"], "typed_global_call_stage1_emit_program_json");
    assert_eq!(plan["route_kind"], "stage1.emit_program_json_v0");
    assert_eq!(plan["symbol"], "nyash.stage1.emit_program_json_v0_h");
}

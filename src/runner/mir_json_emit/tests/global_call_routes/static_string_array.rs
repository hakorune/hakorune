use super::*;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};

fn method_call(
    dst: Option<ValueId>,
    box_name: &str,
    method: &str,
    receiver: ValueId,
    args: Vec<ValueId>,
) -> MirInstruction {
    MirInstruction::Call {
        dst,
        func: ValueId::INVALID,
        callee: Some(Callee::Method {
            box_name: box_name.to_string(),
            method: method.to_string(),
            receiver: Some(receiver),
            certainty: TypeCertainty::Known,
            box_kind: CalleeBoxKind::RuntimeData,
        }),
        args,
        effects: EffectMask::PURE,
    }
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_static_string_array_contract() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_static_string_array_test".to_string());
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
                "PatternRegistryBox.candidates/0".to_string(),
            )),
            args: vec![],
            effects: EffectMask::PURE,
        });

    let mut callee = MirFunction::new(
        FunctionSignature {
            name: "PatternRegistryBox.candidates/0".to_string(),
            params: vec![],
            return_type: MirType::Box("ArrayBox".to_string()),
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = callee.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(1),
            box_type: "ArrayBox".to_string(),
            args: vec![],
        },
        MirInstruction::Copy {
            dst: ValueId::new(2),
            src: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("if.compare.intint".to_string()),
        },
        method_call(
            Some(ValueId::new(4)),
            "ArrayBox",
            "push",
            ValueId::new(2),
            vec![ValueId::new(2), ValueId::new(3)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
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
    assert_eq!(route["proof"], "typed_global_call_static_string_array");
    assert_eq!(route["return_shape"], "array_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["result_origin"], "array_string_birth");
    assert_eq!(route["definition_owner"], "uniform_mir");
    assert_eq!(
        route["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );

    let plan = &root["functions"][0]["metadata"]["lowering_plan"][0];
    assert_eq!(plan["source"], "global_call_routes");
    assert_eq!(plan["target_shape"], serde_json::Value::Null);
    assert_eq!(plan["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(plan["tier"], "DirectAbi");
    assert_eq!(plan["emit_kind"], "direct_function_call");
    assert_eq!(plan["proof"], "typed_global_call_static_string_array");
    assert_eq!(plan["route_proof"], "typed_global_call_static_string_array");
    assert_eq!(plan["return_shape"], "array_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["result_origin"], "array_string_birth");
    assert_eq!(plan["definition_owner"], "uniform_mir");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );
}

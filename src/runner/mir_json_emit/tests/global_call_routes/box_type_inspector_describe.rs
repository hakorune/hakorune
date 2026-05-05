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

fn box_type_inspector_describe_function(name: &str) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: name.to_string(),
            params: vec![MirType::Unknown],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    function.params = vec![ValueId::new(1)];
    let block = function.blocks.get_mut(&BasicBlockId::new(0)).unwrap();
    block.instructions.extend([
        MirInstruction::NewBox {
            dst: ValueId::new(2),
            box_type: "MapBox".to_string(),
            args: vec![],
        },
        MirInstruction::Const {
            dst: ValueId::new(3),
            value: ConstValue::String("Unknown".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(4),
            value: ConstValue::String("".to_string()),
        },
        MirInstruction::BinOp {
            dst: ValueId::new(5),
            op: BinaryOp::Add,
            lhs: ValueId::new(4),
            rhs: ValueId::new(1),
        },
        MirInstruction::Const {
            dst: ValueId::new(6),
            value: ConstValue::String("MapBox(".to_string()),
        },
        method_call(
            Some(ValueId::new(7)),
            "StringBox",
            "indexOf",
            ValueId::new(5),
            vec![ValueId::new(6)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(8),
            value: ConstValue::String("kind".to_string()),
        },
        method_call(
            Some(ValueId::new(9)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(8), ValueId::new(3)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(10),
            value: ConstValue::String("is_map".to_string()),
        },
        MirInstruction::Const {
            dst: ValueId::new(11),
            value: ConstValue::Integer(0),
        },
        method_call(
            Some(ValueId::new(12)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(10), ValueId::new(11)],
        ),
        MirInstruction::Const {
            dst: ValueId::new(13),
            value: ConstValue::String("is_array".to_string()),
        },
        method_call(
            Some(ValueId::new(14)),
            "MapBox",
            "set",
            ValueId::new(2),
            vec![ValueId::new(13), ValueId::new(11)],
        ),
    ]);
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });
    function
}

#[test]
fn build_mir_json_root_emits_direct_plan_for_box_type_inspector_describe_contract() {
    let mut module =
        crate::mir::MirModule::new("json_global_call_box_type_inspector_describe_test".to_string());
    let mut caller = make_function("main", true);
    caller
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
        .push(MirInstruction::Call {
            dst: Some(ValueId::new(30)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global(
                "BoxTypeInspectorBox._describe/1".to_string(),
            )),
            args: vec![ValueId::new(20)],
            effects: EffectMask::PURE,
        });

    let callee = box_type_inspector_describe_function("BoxTypeInspectorBox._describe/1");

    module.add_function(caller);
    module.add_function(callee);
    refresh_module_global_call_routes(&mut module);

    let root = build_mir_json_root(&module).expect("mir json root");
    let route = &root["functions"][0]["metadata"]["global_call_routes"][0];
    assert_eq!(route["target_shape"], serde_json::Value::Null);
    assert_eq!(route["target_shape_reason"], serde_json::Value::Null);
    assert_eq!(route["tier"], "DirectAbi");
    assert_eq!(route["emit_kind"], "direct_function_call");
    assert_eq!(
        route["proof"],
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(route["return_shape"], "map_handle");
    assert_eq!(route["value_demand"], "runtime_i64_or_handle");
    assert_eq!(route["result_origin"], "map_birth");
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
    assert_eq!(
        plan["proof"],
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(
        plan["route_proof"],
        "typed_global_call_box_type_inspector_describe"
    );
    assert_eq!(plan["return_shape"], "map_handle");
    assert_eq!(plan["value_demand"], "runtime_i64_or_handle");
    assert_eq!(plan["result_origin"], "map_birth");
    assert_eq!(plan["definition_owner"], "uniform_mir");
    assert_eq!(
        plan["emit_trace_consumer"],
        "mir_call_global_uniform_mir_emit"
    );
}

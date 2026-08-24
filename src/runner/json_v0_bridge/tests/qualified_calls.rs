use super::super::*;
use crate::mir::verification::MirVerifier;
use crate::mir::{Callee, ConstValue, EffectMask, MirInstruction, MirModule, ValueId};
use serde_json::json;

fn parse(program: serde_json::Value) -> MirModule {
    let module = parse_json_v0_to_module(&program.to_string()).expect("Program JSON-v0 lowers");
    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&module) {
        panic!("qualified Program JSON-v0 call must verify: {errors:?}");
    }
    module
}

fn instructions<'a>(module: &'a MirModule, function_name: &str) -> Vec<&'a MirInstruction> {
    let function = module
        .get_function(function_name)
        .unwrap_or_else(|| panic!("function {function_name} exists"));
    function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .collect()
}

fn assert_integer_def(instructions: &[&MirInstruction], value: ValueId, expected: i64) {
    assert!(instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Const {
                dst,
                value: ConstValue::Integer(actual),
            } if *dst == value && *actual == expected
        )
    }));
}

fn assert_no_target_const(instructions: &[&MirInstruction], target: &str) {
    assert!(!instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Const {
                value: ConstValue::String(value),
                ..
            } if value == target
        )
    }));
}

#[test]
fn qualified_static_method_issues_canonical_global_without_target_const() {
    let module = parse(json!({
        "version": 0,
        "kind": "Program",
        "body": [{
            "type": "Return",
            "expr": {
                "type": "Method",
                "recv": { "type": "Var", "name": "Helper" },
                "method": "id",
                "args": [{ "type": "Int", "value": 7 }]
            }
        }],
        "defs": [{
            "name": "id",
            "params": ["value"],
            "box": "Helper",
            "body": {
                "version": 0,
                "kind": "Program",
                "body": [{ "type": "Return", "expr": { "type": "Var", "name": "value" } }]
            }
        }]
    }));
    let instructions = instructions(&module, "main");
    let call = instructions
        .iter()
        .find(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .expect("main contains the qualified call");

    let MirInstruction::Call {
        dst,
        func,
        callee,
        args,
        effects,
    } = call
    else {
        unreachable!()
    };
    assert!(dst.is_some());
    assert_eq!(*func, ValueId::INVALID);
    assert_eq!(callee, &Some(Callee::Global("Helper.id/1".to_string())));
    assert_eq!(*effects, EffectMask::READ);
    assert_eq!(args.len(), 1);
    assert_integer_def(&instructions, args[0], 7);
    assert_no_target_const(&instructions, "Helper.id/1");
}

#[test]
fn qualified_instance_method_keeps_me_arg_and_issues_canonical_global_without_target_const() {
    let module = parse(json!({
        "version": 0,
        "kind": "Program",
        "body": [{ "type": "Return", "expr": { "type": "Int", "value": 0 } }],
        "defs": [
            {
                "name": "caller",
                "params": [],
                "box": "Helper",
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [{
                        "type": "Return",
                        "expr": {
                            "type": "Method",
                            "recv": { "type": "Var", "name": "me" },
                            "method": "id",
                            "args": [{ "type": "Int", "value": 7 }]
                        }
                    }]
                }
            },
            {
                "name": "id",
                "params": ["self", "value"],
                "box": "Helper",
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [{ "type": "Return", "expr": { "type": "Var", "name": "value" } }]
                }
            }
        ]
    }));
    let instructions = instructions(&module, "Helper.caller/0");
    let call = instructions
        .iter()
        .find(|instruction| matches!(instruction, MirInstruction::Call { .. }))
        .expect("Helper.caller/0 contains the qualified call");

    let MirInstruction::Call {
        dst,
        func,
        callee,
        args,
        effects,
    } = call
    else {
        unreachable!()
    };
    assert!(dst.is_some());
    assert_eq!(*func, ValueId::INVALID);
    assert_eq!(callee, &Some(Callee::Global("Helper.id/2".to_string())));
    assert_eq!(*effects, EffectMask::READ);
    assert_eq!(args.len(), 2);
    assert!(instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Const {
                dst,
                value: ConstValue::String(value),
            } if *dst == args[0] && value == "Helper"
        )
    }));
    assert_integer_def(&instructions, args[1], 7);
    assert_no_target_const(&instructions, "Helper.id/2");
}

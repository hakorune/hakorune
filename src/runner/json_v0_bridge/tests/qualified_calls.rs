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

fn canonical_call(instruction: &MirInstruction) -> Option<&crate::mir::definitions::MirCall> {
    match instruction {
        MirInstruction::Call(call) => Some(call),
        _ => None,
    }
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
        .find_map(|instruction| canonical_call(instruction))
        .expect("main contains the qualified call");

    let dst = call.dst;
    let callee = &call.callee;
    let args = &call.args;
    let effects = call.effects;
    assert!(dst.is_some());
    assert_eq!(
        callee,
        &Callee::Global(crate::mir::test_global_target("Helper.id/1".to_string()))
    );
    assert_eq!(effects, EffectMask::READ);
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
        .find_map(|instruction| canonical_call(instruction))
        .expect("Helper.caller/0 contains the qualified call");

    let dst = call.dst;
    let callee = &call.callee;
    let args = &call.args;
    let effects = call.effects;
    assert!(dst.is_some());
    assert_eq!(
        callee,
        &Callee::Global(crate::mir::test_global_target("Helper.id/2".to_string()))
    );
    assert_eq!(effects, EffectMask::READ);
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

#[test]
fn generic_unqualified_call_uses_unique_local_name_and_arity() {
    let module = parse(json!({
        "version": 0,
        "kind": "Program",
        "body": [{
            "type": "Return",
            "expr": {
                "type": "Call",
                "name": "id",
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
        .find_map(|instruction| canonical_call(instruction))
        .expect("main contains the generic call");
    let callee = &call.callee;
    let args = &call.args;
    assert_eq!(
        callee,
        &Callee::Global(crate::mir::test_global_target("Helper.id/1".to_string()))
    );
    assert_eq!(args.len(), 1);
    assert_no_target_const(&instructions, "Helper.id/1");
}

#[test]
fn generic_qualified_unsuffixed_call_uses_local_arity_once() {
    let module = parse(json!({
        "version": 0,
        "kind": "Program",
        "body": [{
            "type": "Return",
            "expr": {
                "type": "Call",
                "name": "Helper.id",
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
        .find_map(|instruction| match instruction {
            MirInstruction::Call(call) => Some(&call.callee),
            _ => None,
        })
        .expect("main contains the generic call");
    assert_eq!(
        call,
        &Callee::Global(crate::mir::test_global_target("Helper.id/1".to_string()))
    );
    assert_no_target_const(&instructions, "Helper.id/1");
}

#[test]
fn generic_extern_call_strips_only_numeric_arity_suffix() {
    let module = parse(json!({
        "version": 0,
        "kind": "Program",
        "body": [{
            "type": "Return",
            "expr": {
                "type": "Call",
                "name": "env.console.log/1",
                "args": [{ "type": "Int", "value": 7 }]
            }
        }]
    }));
    let instructions = instructions(&module, "main");
    assert!(instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Call(call)
                if matches!(&call.callee, Callee::Extern(name) if name == "env.console.log")
        )
    }));
}

#[test]
fn generic_unknown_name_is_an_exact_global_terminal() {
    let module = parse_json_v0_to_module(
        &json!({
            "version": 0,
            "kind": "Program",
            "body": [{
                "type": "Return",
                "expr": { "type": "Call", "name": "unknown/0", "args": [] }
            }]
        })
        .to_string(),
    )
    .expect("unknown source global remains a typed terminal");
    let instructions = instructions(&module, "main");
    assert!(instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::Call(call)
                if matches!(&call.callee, Callee::Global(name) if name.display_name() == "unknown/0")
        )
    }));
}

#[test]
fn ambiguous_local_name_rejects_before_program_lowering() {
    let error = parse_json_v0_to_module(
        &json!({
            "version": 0,
            "kind": "Program",
            "body": [{
                "type": "Return",
                "expr": { "type": "Call", "name": "run", "args": [] }
            }],
            "defs": [
                {
                    "name": "run",
                    "params": [],
                    "box": "Left",
                    "body": { "version": 0, "kind": "Program", "body": [{ "type": "Return", "expr": { "type": "Int", "value": 1 } }] }
                },
                {
                    "name": "run",
                    "params": [],
                    "box": "Right",
                    "body": { "version": 0, "kind": "Program", "body": [{ "type": "Return", "expr": { "type": "Int", "value": 2 } }] }
                }
            ]
        })
        .to_string(),
    )
    .expect_err("ambiguous short name must reject");
    assert!(error.contains("ambiguous-name"), "error={error}");
}

#[test]
fn duplicate_qualified_definition_rejects_before_lowering() {
    let error = parse_json_v0_to_module(
        &json!({
            "version": 0,
            "kind": "Program",
            "body": [{ "type": "Return", "expr": { "type": "Int", "value": 0 } }],
            "defs": [
                {
                    "name": "run",
                    "params": [],
                    "box": "Helper",
                    "body": { "version": 0, "kind": "Program", "body": [{ "type": "Return", "expr": { "type": "Int", "value": 1 } }] }
                },
                {
                    "name": "run",
                    "params": [],
                    "box": "Helper",
                    "body": { "version": 0, "kind": "Program", "body": [{ "type": "Return", "expr": { "type": "Int", "value": 2 } }] }
                }
            ]
        })
        .to_string(),
    )
    .expect_err("duplicate qualified definition must reject");
    assert!(error.contains("duplicate-definition"), "error={error}");
}

#[test]
fn empty_generic_name_rejects_before_argument_effects() {
    let error = parse_json_v0_to_module(
        &json!({
            "version": 0,
            "kind": "Program",
            "body": [{
                "type": "Return",
                "expr": {
                    "type": "Call",
                    "name": "",
                    "args": [{ "type": "BlockExpr", "prelude": [], "tail": null }]
                }
            }]
        })
        .to_string(),
    )
    .expect_err("empty target must reject before lowering its argument");
    assert!(error.contains("call-target/empty-name"), "error={error}");
    assert!(
        !error.contains("blockexpr"),
        "argument was lowered: {error}"
    );
}

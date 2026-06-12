use super::super::*;
use super::common::*;
use crate::backend::vm::VM;
use serde_json::json;

#[test]
fn vm_fails_fast_when_option_some_payload_evaluates_to_nullish_value() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "enum_decls": option_enum_decls(),
        "body": [
            {
                "type": "Local",
                "name": "payload",
                "expr": { "type": "Null" }
            },
            {
                "type": "Return",
                "expr": {
                    "type": "EnumCtor",
                    "enum": "Option",
                    "variant": "Some",
                    "payload_type": "Integer",
                    "args": [{ "type": "Var", "name": "payload" }]
                }
            }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json)
        .expect("variable payload path should compile so runtime guard can prove itself");
    let mut vm = VM::new();
    let err = vm
        .execute_module(&module)
        .expect_err("Option::Some(nullish variable) should fail fast at runtime");
    let message = err.to_string();
    assert!(message.contains("[freeze:contract][option/some_nullish]"));
    assert!(message.contains("Option::Some payload must not be null or void"));
}

#[test]
fn vm_allows_option_some_non_null_payload() {
    let json = json!({
        "version": 0,
        "kind": "Program",
        "enum_decls": option_enum_decls(),
        "body": [
            {
                "type": "Local",
                "name": "value",
                "expr": option_some_ctor(7)
            },
            {
                "type": "Return",
                "expr": {
                    "type": "EnumMatch",
                    "enum": "Option",
                    "scrutinee": { "type": "Var", "name": "value" },
                    "arms": [
                        {
                            "variant": "Some",
                            "bind": "payload",
                            "payload_type": "Integer",
                            "expr": { "type": "Var", "name": "payload" }
                        },
                        {
                            "variant": "None",
                            "expr": { "type": "Int", "value": 0 }
                        }
                    ]
                }
            }
        ]
    })
    .to_string();

    let module = parse_json_v0_to_module(&json).expect("non-null Some should compile");
    let mut vm = VM::new();
    let result = vm
        .execute_module(&module)
        .expect("non-null Some should execute successfully");
    assert_eq!(result.to_string_box().value, "7");
}

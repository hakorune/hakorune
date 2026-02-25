use serde_json::json;
use std::collections::BTreeMap;

fn verify_program(label: &str, program: serde_json::Value) {
    let src = serde_json::to_string(&program).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .unwrap_or_else(|e| panic!("{}: failed to parse Program(JSON): {}", label, e));

    let mut verifier = nyash_rust::mir::verification::MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&module) {
        panic!(
            "{}: MIR verification failed with errors: {:?}",
            label, errors
        );
    }
}

fn run_program(label: &str, program: serde_json::Value) -> String {
    let src = serde_json::to_string(&program).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .unwrap_or_else(|e| panic!("{}: failed to parse Program(JSON): {}", label, e));
    let mut vm = nyash_rust::backend::vm::VM::new();
    let result = vm
        .execute_module(&module)
        .unwrap_or_else(|e| panic!("{}: VM execution failed: {}", label, e));
    result.to_string_box().value
}

fn program_legacy_break_prefix_identifier() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "break_list", "expr": { "type": "New", "class": "ArrayBox", "args": [] } },
            { "type": "Break" },
            {
                "type": "Expr",
                "expr": {
                    "type": "Method",
                    "recv": { "type": "Var", "name": "_list" },
                    "method": "push",
                    "args": [ { "type": "Str", "value": "x" } ]
                }
            },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
}

fn program_legacy_if_not_guard_quad() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "ch", "expr": { "type": "Str", "value": "a" } },
            { "type": "If", "cond": { "type": "Int", "value": 0 }, "then": [] },
            { "type": "Expr", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Expr",
                "expr": {
                    "type": "Compare",
                    "op": "==",
                    "lhs": { "type": "Var", "name": "ch" },
                    "rhs": { "type": "Str", "value": "a" }
                }
            },
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [],
                    "tail": {
                        "type": "Return",
                        "expr": { "type": "Int", "value": 7 }
                    }
                }
            },
            { "type": "Return", "expr": { "type": "Int", "value": 9 } }
        ]
    })
}

#[test]
fn json_legacy_break_prefix_identifier_verifies() {
    verify_program(
        "json_legacy_break_prefix_identifier",
        program_legacy_break_prefix_identifier(),
    );
}

#[test]
fn json_legacy_if_not_guard_quad_executes_correctly() {
    let result = run_program(
        "json_legacy_if_not_guard_quad",
        program_legacy_if_not_guard_quad(),
    );
    assert_eq!(result, "9");
}

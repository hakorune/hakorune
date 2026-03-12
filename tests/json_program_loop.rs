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

fn program_simple_loop() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "loopCount", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Loop",
                "cond": { "type": "Bool", "value": true },
                "body": [
                    { "type": "Break" }
                ]
            },
            { "type": "Return", "expr": { "type": "Var", "name": "loopCount" } }
        ]
    })
}

fn program_loop_with_continue() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "cycles", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Loop",
                "cond": { "type": "Bool", "value": true },
                "body": [
                    { "type": "Local", "name": "loopTick", "expr": { "type": "Int", "value": 1 } },
                    {
                        "type": "If",
                        "cond": { "type": "Bool", "value": true },
                        "then": [
                            { "type": "Continue" }
                        ],
                        "else": [
                            { "type": "Break" }
                        ]
                    }
                ]
            },
            { "type": "Return", "expr": { "type": "Var", "name": "cycles" } }
        ]
    })
}

fn program_loop_body_local_exit() -> serde_json::Value {
    // Phase 191: Body-local variables are properly scoped and not accessible after loop exit
    // This test verifies that body-local variables can be used within their scope
    // but properly tests just the scope itself with a minimal loop
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "result", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Loop",
                "cond": { "type": "Bool", "value": true },
                "body": [
                    { "type": "Local", "name": "bodyTemp", "expr": { "type": "Int", "value": 42 } },
                    { "type": "Break" }
                ]
            },
            { "type": "Return", "expr": { "type": "Var", "name": "result" } }
        ]
    })
}

fn program_loop_continue_before_and_after_body_local_decl() -> serde_json::Value {
    // Phase 29bq: continue snapshots may observe body-local vars after they are assigned.
    // The canonical continue-merge must NOT treat those as carried header vars, otherwise
    // we can create partial PHIs (missing inputs on early-continue edges).
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "i", "expr": { "type": "Int", "value": 0 } },
            { "type": "Local", "name": "sum", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Loop",
                "cond": { "type": "Compare", "op": "<", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Int", "value": 3 } },
                "body": [
                    // Early continue before body-local `e` is assigned.
                    { "type": "If", "cond": { "type": "Compare", "op": "==", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Int", "value": 0 } },
                      "then": [
                        { "type": "Local", "name": "i", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Int", "value": 1 } } },
                        { "type": "Continue" }
                      ]
                    },

                    // Body-local declaration appears after the early continue.
                    { "type": "Local", "name": "e", "expr": { "type": "Int", "value": 42 } },

                    // Continue after `e` exists.
                    { "type": "Local", "name": "sum", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "sum" }, "rhs": { "type": "Int", "value": 1 } } },
                    { "type": "Local", "name": "i", "expr": { "type": "Binary", "op": "+", "lhs": { "type": "Var", "name": "i" }, "rhs": { "type": "Int", "value": 1 } } },
                    { "type": "Continue" }
                ]
            },
            { "type": "Return", "expr": { "type": "Var", "name": "sum" } }
        ]
    })
}

fn program_legacy_while_triplet() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "i", "expr": { "type": "Int", "value": 0 } },
            { "type": "Local", "name": "sum", "expr": { "type": "Int", "value": 0 } },
            { "type": "Expr", "expr": { "type": "Var", "name": "while" } },
            {
                "type": "Expr",
                "expr": {
                    "type": "Compare",
                    "op": "<",
                    "lhs": { "type": "Var", "name": "i" },
                    "rhs": { "type": "Int", "value": 3 }
                }
            },
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "Local",
                            "name": "sum",
                            "expr": {
                                "type": "Binary",
                                "op": "+",
                                "lhs": { "type": "Var", "name": "sum" },
                                "rhs": { "type": "Var", "name": "i" }
                            }
                        }
                    ],
                    "tail": {
                        "type": "Local",
                        "name": "i",
                        "expr": {
                            "type": "Binary",
                            "op": "+",
                            "lhs": { "type": "Var", "name": "i" },
                            "rhs": { "type": "Int", "value": 1 }
                        }
                    }
                }
            },
            { "type": "Return", "expr": { "type": "Var", "name": "sum" } }
        ]
    })
}

#[test]
fn json_loop_simple_verifies() {
    verify_program("json_loop_simple", program_simple_loop());
}

#[test]
fn json_loop_with_continue_verifies() {
    verify_program("json_loop_with_continue", program_loop_with_continue());
}

#[test]
fn json_loop_body_local_exit_verifies() {
    verify_program("json_loop_body_local_exit", program_loop_body_local_exit());
}

#[test]
fn json_loop_continue_before_and_after_body_local_decl_verifies() {
    verify_program(
        "json_loop_continue_before_and_after_body_local_decl",
        program_loop_continue_before_and_after_body_local_decl(),
    );
}

#[test]
fn json_legacy_while_triplet_verifies() {
    verify_program("json_legacy_while_triplet", program_legacy_while_triplet());
}

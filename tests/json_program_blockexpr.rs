use serde_json::json;
use std::collections::BTreeMap;

fn parse_program(label: &str, program: serde_json::Value) -> nyash_rust::mir::MirModule {
    let src = serde_json::to_string(&program).expect("serialize program");
    nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .unwrap_or_else(|e| panic!("{}: failed to parse Program(JSON): {}", label, e))
}

fn verify_program(label: &str, program: serde_json::Value) {
    let module = parse_program(label, program);
    let mut verifier = nyash_rust::mir::verification::MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&module) {
        panic!(
            "{}: MIR verification failed with errors: {:?}",
            label, errors
        );
    }
}

fn program_blockexpr_prelude_if() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Local",
                "name": "result",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        { "type": "Local", "name": "a", "expr": { "type": "Int", "value": 1 } },
                        {
                            "type": "If",
                            "cond": { "type": "Bool", "value": true },
                            "then": [
                                { "type": "Expr", "expr": { "type": "Var", "name": "a" } }
                            ],
                            "else": [
                                { "type": "Expr", "expr": { "type": "Var", "name": "a" } }
                            ]
                        }
                    ],
                    "tail": { "type": "Expr", "expr": { "type": "Var", "name": "a" } }
                }
            },
            { "type": "Return", "expr": { "type": "Var", "name": "result" } }
        ]
    })
}

fn program_blockexpr_tail_if_with_return() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        { "type": "Local", "name": "a", "expr": { "type": "Int", "value": 0 } }
                    ],
                    "tail": {
                        "type": "If",
                        "cond": { "type": "Bool", "value": false },
                        "then": [
                            { "type": "Return", "expr": { "type": "Int", "value": 7 } }
                        ]
                    }
                }
            },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
}

fn program_blockexpr_prelude_loop() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Local",
                "name": "result",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        { "type": "Local", "name": "i", "expr": { "type": "Int", "value": 0 } },
                        {
                            "type": "Loop",
                            "cond": { "type": "Bool", "value": true },
                            "body": [
                                { "type": "Break" }
                            ]
                        }
                    ],
                    "tail": { "type": "Expr", "expr": { "type": "Var", "name": "i" } }
                }
            },
            { "type": "Return", "expr": { "type": "Var", "name": "result" } }
        ]
    })
}

fn program_blockexpr_tail_loop() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        { "type": "Local", "name": "i", "expr": { "type": "Int", "value": 0 } }
                    ],
                    "tail": {
                        "type": "Loop",
                        "cond": {
                            "type": "Compare",
                            "op": "<",
                            "lhs": { "type": "Var", "name": "i" },
                            "rhs": { "type": "Int", "value": 2 }
                        },
                        "body": [
                            {
                                "type": "Local",
                                "name": "i",
                                "expr": {
                                    "type": "Binary",
                                    "op": "+",
                                    "lhs": { "type": "Var", "name": "i" },
                                    "rhs": { "type": "Int", "value": 1 }
                                }
                            },
                            {
                                "type": "If",
                                "cond": {
                                    "type": "Compare",
                                    "op": "==",
                                    "lhs": { "type": "Var", "name": "i" },
                                    "rhs": { "type": "Int", "value": 2 }
                                },
                                "then": [
                                    { "type": "Break" }
                                ]
                            }
                        ]
                    }
                }
            },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
}

fn program_blockexpr_prelude_legacy_local_chain() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Local",
                "name": "result",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        { "type": "Local", "name": "j", "expr": { "type": "Int", "value": 0 } },
                        { "type": "Local", "name": "in_str", "expr": { "type": "Var", "name": "local" } },
                        { "type": "Expr", "expr": { "type": "Var", "name": "esc" } }
                    ],
                    "tail": { "type": "Expr", "expr": { "type": "Var", "name": "in_str" } }
                }
            },
            { "type": "Return", "expr": { "type": "Var", "name": "result" } }
        ]
    })
}

#[test]
fn json_blockexpr_prelude_if_verifies() {
    verify_program(
        "json_blockexpr_prelude_if",
        program_blockexpr_prelude_if(),
    );
}

#[test]
fn json_blockexpr_tail_if_with_return_verifies() {
    verify_program(
        "json_blockexpr_tail_if_with_return",
        program_blockexpr_tail_if_with_return(),
    );
}

#[test]
fn json_blockexpr_prelude_loop_verifies() {
    verify_program(
        "json_blockexpr_prelude_loop",
        program_blockexpr_prelude_loop(),
    );
}

#[test]
fn json_blockexpr_tail_loop_verifies() {
    verify_program(
        "json_blockexpr_tail_loop",
        program_blockexpr_tail_loop(),
    );
}

#[test]
fn json_blockexpr_prelude_legacy_local_chain_verifies() {
    verify_program(
        "json_blockexpr_prelude_legacy_local_chain",
        program_blockexpr_prelude_legacy_local_chain(),
    );
}

#[test]
fn json_defs_param_valueids_are_reserved() {
    let program = json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ],
        "defs": [
            {
                "name": "foo",
                "params": ["x", "y"],
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [
                        { "type": "Local", "name": "z", "expr": { "type": "Int", "value": 0 } },
                        { "type": "Return", "expr": { "type": "Var", "name": "x" } }
                    ]
                },
                "box": "Main"
            }
        ]
    });

    let module = parse_program("json_defs_param_valueids_are_reserved", program);
    let func = module
        .get_function("Main.foo/2")
        .expect("Main.foo/2 present");

    assert_eq!(func.params, vec![nyash_rust::mir::ValueId::new(0), nyash_rust::mir::ValueId::new(1)]);
    assert_eq!(func.next_value_id, 2);
}

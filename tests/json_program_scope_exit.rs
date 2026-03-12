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

fn program_stageb_fini_reg_in_blockexpr() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "out", "expr": { "type": "Int", "value": 0 } },
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "Local",
                            "name": "out",
                            "expr": {
                                "type": "Binary",
                                "op": "+",
                                "lhs": { "type": "Var", "name": "out" },
                                "rhs": { "type": "Int", "value": 1 }
                            }
                        },
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                {
                                    "type": "Local",
                                    "name": "out",
                                    "expr": {
                                        "type": "Binary",
                                        "op": "+",
                                        "lhs": { "type": "Var", "name": "out" },
                                        "rhs": { "type": "Int", "value": 100 }
                                    }
                                }
                            ]
                        },
                        {
                            "type": "FiniReg",
                            "prelude": [
                                { "type": "Local", "name": "a", "expr": { "type": "Int", "value": 1 } }
                            ],
                            "fini": [
                                {
                                    "type": "Local",
                                    "name": "out",
                                    "expr": {
                                        "type": "Binary",
                                        "op": "+",
                                        "lhs": { "type": "Var", "name": "out" },
                                        "rhs": { "type": "Int", "value": 10 }
                                    }
                                }
                            ]
                        },
                        {
                            "type": "FiniReg",
                            "prelude": [
                                { "type": "Local", "name": "b", "expr": { "type": "Int", "value": 1 } }
                            ],
                            "fini": [
                                {
                                    "type": "Local",
                                    "name": "out",
                                    "expr": {
                                        "type": "Binary",
                                        "op": "+",
                                        "lhs": { "type": "Var", "name": "out" },
                                        "rhs": { "type": "Int", "value": 1000 }
                                    }
                                }
                            ]
                        }
                    ],
                    "tail": {
                        "type": "Local",
                        "name": "out",
                        "expr": {
                            "type": "Binary",
                            "op": "+",
                            "lhs": { "type": "Var", "name": "out" },
                            "rhs": { "type": "Int", "value": 2 }
                        }
                    }
                }
            },
            { "type": "Return", "expr": { "type": "Var", "name": "out" } }
        ]
    })
}

#[test]
fn json_stageb_fini_reg_in_blockexpr_verifies() {
    verify_program(
        "json_stageb_fini_reg_in_blockexpr",
        program_stageb_fini_reg_in_blockexpr(),
    );
}

// Freeze tag tests: FiniReg fini body must not contain non-local exits

fn program_fini_reg_with_return_in_fini() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                { "type": "Return", "expr": { "type": "Int", "value": 42 } }
                            ]
                        }
                    ],
                    "tail": { "type": "Int", "value": 0 }
                }
            }
        ]
    })
}

#[test]
fn json_fini_reg_forbid_return_in_fini() {
    let program = program_fini_reg_with_return_in_fini();
    let src = serde_json::to_string(&program).expect("serialize program");

    let result = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    );

    assert!(result.is_err(), "Expected freeze tag error");
    let err = result.unwrap_err();
    assert!(
        err.contains("[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]"),
        "Error should contain freeze tag: {}",
        err
    );
    assert!(
        err.contains("return"),
        "Error should mention 'return': {}",
        err
    );
}

fn program_fini_reg_with_throw_in_fini() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                { "type": "Throw", "expr": { "type": "Int", "value": 1 } }
                            ]
                        }
                    ],
                    "tail": { "type": "Int", "value": 0 }
                }
            }
        ]
    })
}

#[test]
fn json_fini_reg_forbid_throw_in_fini() {
    let program = program_fini_reg_with_throw_in_fini();
    let src = serde_json::to_string(&program).expect("serialize program");

    let result = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    );

    assert!(result.is_err(), "Expected freeze tag error");
    let err = result.unwrap_err();
    assert!(
        err.contains("[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]"),
        "Error should contain freeze tag: {}",
        err
    );
    assert!(
        err.contains("throw"),
        "Error should mention 'throw': {}",
        err
    );
}

fn program_fini_reg_with_break_in_fini() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                { "type": "Break" }
                            ]
                        }
                    ],
                    "tail": { "type": "Int", "value": 0 }
                }
            }
        ]
    })
}

#[test]
fn json_fini_reg_forbid_break_in_fini() {
    let program = program_fini_reg_with_break_in_fini();
    let src = serde_json::to_string(&program).expect("serialize program");

    let result = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    );

    assert!(result.is_err(), "Expected freeze tag error");
    let err = result.unwrap_err();
    assert!(
        err.contains("[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]"),
        "Error should contain freeze tag: {}",
        err
    );
    assert!(
        err.contains("break"),
        "Error should mention 'break': {}",
        err
    );
}

fn program_fini_reg_with_continue_in_fini() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                { "type": "Continue" }
                            ]
                        }
                    ],
                    "tail": { "type": "Int", "value": 0 }
                }
            }
        ]
    })
}

#[test]
fn json_fini_reg_forbid_continue_in_fini() {
    let program = program_fini_reg_with_continue_in_fini();
    let src = serde_json::to_string(&program).expect("serialize program");

    let result = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    );

    assert!(result.is_err(), "Expected freeze tag error");
    let err = result.unwrap_err();
    assert!(
        err.contains("[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]"),
        "Error should contain freeze tag: {}",
        err
    );
    assert!(
        err.contains("continue"),
        "Error should mention 'continue': {}",
        err
    );
}

fn program_fini_reg_with_nested_fini_reg() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            {
                "type": "Expr",
                "expr": {
                    "type": "BlockExpr",
                    "prelude": [
                        {
                            "type": "FiniReg",
                            "prelude": [],
                            "fini": [
                                {
                                    "type": "FiniReg",
                                    "prelude": [],
                                    "fini": []
                                }
                            ]
                        }
                    ],
                    "tail": { "type": "Int", "value": 0 }
                }
            }
        ]
    })
}

#[test]
fn json_fini_reg_forbid_nested_fini_reg() {
    let program = program_fini_reg_with_nested_fini_reg();
    let src = serde_json::to_string(&program).expect("serialize program");

    let result = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    );

    assert!(result.is_err(), "Expected freeze tag error");
    let err = result.unwrap_err();
    assert!(
        err.contains("[freeze:contract][json_v0_bridge/fini_forbid_non_local_exit]"),
        "Error should contain freeze tag: {}",
        err
    );
    assert!(
        err.contains("nested FiniReg"),
        "Error should mention 'nested FiniReg': {}",
        err
    );
}

use super::super::*;
use serde_json::json;

#[test]
fn subset_accepts_const_void() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "void", "value": 0 }
                    },
                    { "op": "ret", "value": 1 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_const_f64() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "f64", "value": 1.5 }
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "const(non-i64-bool-void-handle:f64)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_typeop_check_integer() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 42 }
                    },
                    {
                        "op": "typeop",
                        "operation": "check",
                        "src": 1,
                        "target_type": "Integer",
                        "dst": 2
                    },
                    { "op": "ret", "value": 2 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_typeop_missing_target_type() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "typeop",
                        "operation": "check",
                        "src": 1,
                        "dst": 2
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "typeop(missing-target-type)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_binop_op_kind_alias() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 40 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 2 }
                    },
                    {
                        "op": "binop",
                        "op_kind": "Add",
                        "lhs": 1,
                        "rhs": 2,
                        "dst": 3
                    },
                    { "op": "ret", "value": 3 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_unop_op_kind_alias() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "unop",
                        "op_kind": "Not",
                        "src": 1,
                        "dst": 2
                    },
                    { "op": "ret", "value": 2 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_compare_not_equal_op_kind_alias() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 1 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 2 }
                    },
                    {
                        "op": "compare",
                        "op_kind": "Ne",
                        "lhs": 1,
                        "rhs": 2,
                        "dst": 3
                    },
                    { "op": "ret", "value": 3 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_compare_greater_equal_op_kind_alias() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 2 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 2 }
                    },
                    {
                        "op": "compare",
                        "op_kind": "Ge",
                        "lhs": 1,
                        "rhs": 2,
                        "dst": 3
                    },
                    { "op": "ret", "value": 3 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_compare_greater_than_outside_vm_hako_allowlist() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "i64", "value": 2 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 1 }
                    },
                    {
                        "op": "compare",
                        "op_kind": "Gt",
                        "lhs": 1,
                        "rhs": 2,
                        "dst": 3
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Err(("main".to_string(), 0, "compare(>)".to_string())));
}

#[test]
fn subset_rejects_legacy_debug_log_even_with_non_reg_values() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "debug_log",
                        "message": "bad-values",
                        "values": ["x"]
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Err(("main".to_string(), 0, "debug_log".to_string())));
}

#[test]
fn subset_rejects_select_missing_then_val() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": { "type": "bool", "value": true }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 42 }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 7 }
                    },
                    {
                        "op": "select",
                        "dst": 4,
                        "cond": 1,
                        "else_val": 3
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "select(missing-then-val)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_externcall_env_get() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 1,
                        "value": {
                            "type": { "kind": "handle", "box_type": "StringBox" },
                            "value": "RVP_C05_ENV_KEY"
                        }
                    },
                    {
                        "op": "externcall",
                        "func": "env.get/1",
                        "args": [1],
                        "dst": 2
                    },
                    { "op": "ret", "value": 2 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_externcall_env_get_with_missing_arg() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "externcall",
                        "func": "env.get/1",
                        "args": [],
                        "dst": 2
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "externcall(env.get:args!=1)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_call_args2_dynamic_when_id1_model_exists() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Main.id/1",
                "params": [10, 11],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "ret", "value": 11 }
                    ]
                }]
            },
            {
                "name": "main",
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 7 } },
                        { "op": "const", "dst": 2, "value": { "type": "i64", "value": 9 } },
                        { "op": "call", "dst": 3, "func": 4294967295u64, "args": [1, 2] },
                        { "op": "ret", "value": 3 }
                    ]
                }]
            }
        ]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_call_args2_dynamic_without_id1_model() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "const", "dst": 1, "value": { "type": "i64", "value": 7 } },
                    { "op": "const", "dst": 2, "value": { "type": "i64", "value": 9 } },
                    { "op": "call", "dst": 3, "func": 4294967295u64, "args": [1, 2] }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "call(args2:dynamic-no-id1-model)".to_string()
        ))
    );
}

#[test]
fn subset_rejects_call_args2_non_method_handle_target() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "const",
                        "dst": 5,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "plain_function_name"
                        }
                    },
                    { "op": "const", "dst": 1, "value": { "type": "i64", "value": 7 } },
                    { "op": "const", "dst": 2, "value": { "type": "i64", "value": 9 } },
                    { "op": "call", "dst": 3, "func": 5, "args": [1, 2] }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err((
            "main".to_string(),
            0,
            "call(args2:non-method-handle)".to_string()
        ))
    );
}

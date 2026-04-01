use super::super::*;
use serde_json::json;
use std::path::PathBuf;

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
fn subset_accepts_compare_greater_than_in_vm_hako_allowlist() {
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
    assert_eq!(out, Ok(()));
}

#[test]
fn vm_hako_runtime_compare_contract_is_in_sync() {
    let runtime_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("lang")
        .join("src")
        .join("vm")
        .join("boxes")
        .join("mir_vm_s0_exec_dispatch.hako");
    let runtime_src = std::fs::read_to_string(&runtime_path)
        .expect("read lang/src/vm/boxes/mir_vm_s0_exec_dispatch.hako");

    // Rust subset allowlist and Hako runtime implementation must stay aligned.
    for sym in ["==", "!=", "<", "<=", ">", ">="] {
        assert!(
            runtime_src.contains(&format!("if sym == \"{}\"", sym)),
            "missing compare runtime branch for '{}' in {}",
            sym,
            runtime_path.display()
        );
    }
    for alias in ["Eq", "Ne", "Lt", "Le", "Gt", "Ge"] {
        assert!(
            runtime_src.contains(&format!("kind == \"{}\"", alias)),
            "missing compare op_kind alias '{}' in {}",
            alias,
            runtime_path.display()
        );
    }
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
fn subset_accepts_externcall_env_mirbuilder_emit() {
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
                            "value": "{\"type\":\"Program\",\"body\":[]}"
                        }
                    },
                    {
                        "op": "externcall",
                        "func": "env.mirbuilder_emit/1",
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
fn subset_accepts_externcall_hako_last_error() {
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
                        "op": "externcall",
                        "func": "hako_last_error/1",
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
fn subset_accepts_boxcall_tlscore_last_error_text_h() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "TlsCoreBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "last_error_text_h",
                        "box": 1,
                        "dst": 2,
                        "args": []
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
fn subset_accepts_externcall_hako_barrier_touch_i64() {
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
                        "op": "externcall",
                        "func": "hako_barrier_touch_i64/1",
                        "args": [1],
                        "dst": null
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
fn subset_accepts_externcall_hako_osvm_reserve_bytes_i64() {
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
                        "value": { "type": "i64", "value": 4096 }
                    },
                    {
                        "op": "externcall",
                        "func": "hako_osvm_reserve_bytes_i64/1",
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
fn subset_accepts_externcall_nyash_gc_barrier_write() {
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
                        "op": "externcall",
                        "func": "nyash.gc.barrier_write/1",
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
fn subset_accepts_boxcall_osvmcore_reserve_bytes_i64() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "OsVmCoreBox"
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 4096 }
                    },
                    {
                        "op": "boxcall",
                        "method": "reserve_bytes_i64",
                        "box": 1,
                        "dst": 3,
                        "args": [2]
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
fn subset_accepts_boxcall_atomiccore_fence_i64() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "AtomicCoreBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "fence_i64",
                        "box": 1,
                        "dst": 2,
                        "args": []
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
fn subset_accepts_boxcall_gccore_write_barrier_i64() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "GcCoreBox"
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "boxcall",
                        "method": "write_barrier_i64",
                        "box": 1,
                        "dst": 3,
                        "args": [2]
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
fn subset_accepts_mir_call_extern_hako_mem_alloc() {
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
                        "value": { "type": "i64", "value": 8 }
                    },
                    {
                        "op": "mir_call",
                        "dst": 2,
                        "mir_call": {
                            "callee": { "type": "Extern", "name": "hako_mem_alloc" },
                            "args": [1],
                            "effects": ["IO"],
                            "flags": {}
                        }
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
fn subset_accepts_mir_call_extern_hako_osvm_reserve_bytes_i64() {
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
                        "value": { "type": "i64", "value": 4096 }
                    },
                    {
                        "op": "mir_call",
                        "dst": 2,
                        "mir_call": {
                            "callee": { "type": "Extern", "name": "hako_osvm_reserve_bytes_i64" },
                            "args": [1],
                            "effects": ["IO"],
                            "flags": {}
                        }
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

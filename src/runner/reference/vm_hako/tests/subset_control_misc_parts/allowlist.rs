use super::super::super::*;
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

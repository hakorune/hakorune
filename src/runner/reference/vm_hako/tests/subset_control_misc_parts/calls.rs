use super::super::super::*;
use serde_json::json;

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

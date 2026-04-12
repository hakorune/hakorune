use super::super::super::*;
use serde_json::json;

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
fn subset_accepts_boxcall_osvmcore_commit_bytes_i64() {
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
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 8192 }
                    },
                    {
                        "op": "boxcall",
                        "method": "commit_bytes_i64",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
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
fn subset_accepts_externcall_hako_osvm_commit_bytes_i64() {
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
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 8192 }
                    },
                    {
                        "op": "externcall",
                        "func": "hako_osvm_commit_bytes_i64/2",
                        "args": [1, 2],
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
fn subset_accepts_boxcall_osvmcore_decommit_bytes_i64() {
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
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 8192 }
                    },
                    {
                        "op": "boxcall",
                        "method": "decommit_bytes_i64",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
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
fn subset_accepts_externcall_hako_osvm_decommit_bytes_i64() {
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
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 8192 }
                    },
                    {
                        "op": "externcall",
                        "func": "hako_osvm_decommit_bytes_i64/2",
                        "args": [1, 2],
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

use super::super::*;
use serde_json::json;

#[test]
fn subset_accepts_legacy_nop_as_noop() {
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
                    { "op": "nop" },
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
fn subset_accepts_safepoint() {
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
                    { "op": "safepoint" },
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
fn subset_accepts_keepalive() {
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
                    { "op": "keepalive", "values": [1] },
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
fn subset_accepts_release_strong() {
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
                    { "op": "release_strong", "values": [1] },
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
fn subset_accepts_debug() {
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
                    { "op": "debug", "value": 1, "message": "s4a" },
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
fn subset_rejects_legacy_debug_log() {
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
                        "op": "debug_log",
                        "message": "s4b",
                        "values": [1]
                    },
                    { "op": "ret", "value": 1 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Err(("main".to_string(), 0, "debug_log".to_string())));
}

#[test]
fn subset_accepts_select() {
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
                        "then_val": 2,
                        "else_val": 3
                    },
                    { "op": "ret", "value": 4 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_barrier() {
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
                        "op": "barrier",
                        "kind": "read",
                        "ptr": 1
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
fn subset_accepts_load() {
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
                        "value": { "type": "i64", "value": 7 }
                    },
                    {
                        "op": "load",
                        "dst": 2,
                        "ptr": 1
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
fn subset_accepts_newbox_mapbox() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "newbox", "dst": 1, "type": "MapBox" },
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
fn subset_rejects_load_missing_ptr() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "load",
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
        Err(("main".to_string(), 0, "load(missing-ptr)".to_string()))
    );
}

#[test]
fn subset_accepts_store() {
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
                        "value": { "type": "i64", "value": 7 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 42 }
                    },
                    {
                        "op": "store",
                        "ptr": 1,
                        "value": 2
                    },
                    {
                        "op": "load",
                        "dst": 3,
                        "ptr": 1
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
fn subset_rejects_store_missing_value() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "store",
                        "ptr": 1
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err(("main".to_string(), 0, "store(missing-value)".to_string()))
    );
}

#[test]
fn subset_accepts_phi() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [
                {
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
                            "value": { "type": "i64", "value": 42 }
                        },
                        {
                            "op": "const",
                            "dst": 3,
                            "value": { "type": "i64", "value": 7 }
                        },
                        {
                            "op": "branch",
                            "cond": 1,
                            "then": 1,
                            "else": 2
                        }
                    ]
                },
                {
                    "id": 1,
                    "instructions": [
                        {
                            "op": "jump",
                            "target": 3
                        }
                    ]
                },
                {
                    "id": 2,
                    "instructions": [
                        {
                            "op": "jump",
                            "target": 3
                        }
                    ]
                },
                {
                    "id": 3,
                    "instructions": [
                        {
                            "op": "phi",
                            "dst": 4,
                            "incoming": [[2, 1], [3, 2]]
                        },
                        { "op": "ret", "value": 4 }
                    ]
                }
            ]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_phi_missing_incoming() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "phi",
                        "dst": 1
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err(("main".to_string(), 0, "phi(missing-incoming)".to_string()))
    );
}

#[test]
fn subset_accepts_array_get() {
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
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "array_get",
                        "dst": 3,
                        "array": 1,
                        "index": 2
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
fn subset_rejects_array_get_missing_index() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "array_get",
                        "dst": 3,
                        "array": 1
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
            "array_get(missing-index)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_array_set() {
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
                        "value": { "type": "i64", "value": 7 }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 42 }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "array_set",
                        "array": 1,
                        "index": 3,
                        "value": 2
                    },
                    {
                        "op": "load",
                        "dst": 4,
                        "ptr": 1
                    },
                    { "op": "ret", "value": 4 }
                ]
            }]
        }]
    })
    .to_string();

    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_array_set_missing_value() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "array_set",
                        "array": 1,
                        "index": 3
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
            "array_set(missing-value)".to_string()
        ))
    );
}

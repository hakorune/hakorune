use super::super::super::*;
use serde_json::json;

#[test]
fn subset_rejects_boxcall_osvmcore_page_size_i64() {
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
                        "op": "boxcall",
                        "method": "page_size_i64",
                        "box": 1,
                        "dst": 2,
                        "args": []
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
            "boxcall(osvm:page_size_i64)".to_string()
        ))
    );
}

#[test]
fn subset_rejects_externcall_hako_osvm_page_size_i64() {
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
                        "func": "hako_osvm_page_size_i64/0",
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
            "externcall(hako_osvm_page_size_i64/0)".to_string()
        ))
    );
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

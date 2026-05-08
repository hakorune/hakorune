use super::super::super::*;
use serde_json::json;

#[test]
fn subset_accepts_boxcall_intrincore_bit_count_i64_rows() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "newbox", "dst": 1, "type": "IntrinCoreBox" },
                    { "op": "const", "dst": 2, "value": { "type": "i64", "value": 16 } },
                    { "op": "boxcall", "method": "clz_i64", "box": 1, "dst": 3, "args": [2] },
                    { "op": "boxcall", "method": "ctz_i64", "box": 1, "dst": 4, "args": [2] },
                    { "op": "boxcall", "method": "popcnt_i64", "box": 1, "dst": 5, "args": [2] },
                    { "op": "ret", "value": 5 }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_intrincore_bit_count_without_arg() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "newbox", "dst": 1, "type": "IntrinCoreBox" },
                    { "op": "boxcall", "method": "clz_i64", "box": 1, "dst": 2, "args": [] }
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
            "boxcall(clz_i64:args!=1)".to_string()
        ))
    );
}

#[test]
fn subset_rejects_boxcall_intrincore_unknown_method() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "newbox", "dst": 1, "type": "IntrinCoreBox" },
                    { "op": "boxcall", "method": "prefetch_i64", "box": 1, "dst": 2, "args": [] }
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
            "boxcall(intrin:prefetch_i64)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_externcall_hako_intrin_bit_count_i64_rows() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "const", "dst": 1, "value": { "type": "i64", "value": 16 } },
                    { "op": "externcall", "func": "hako_intrin_clz_i64/1", "args": [1], "dst": 2 },
                    { "op": "externcall", "func": "hako_intrin_ctz_i64/1", "args": [1], "dst": 3 },
                    { "op": "externcall", "func": "hako_intrin_popcnt_i64/1", "args": [1], "dst": 4 },
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
fn subset_rejects_externcall_unknown_hako_intrin_row() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "const", "dst": 1, "value": { "type": "i64", "value": 16 } },
                    { "op": "externcall", "func": "hako_intrin_prefetch_i64/1", "args": [1], "dst": 2 }
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
            "externcall(hako_intrin_prefetch_i64/1)".to_string()
        ))
    );
}

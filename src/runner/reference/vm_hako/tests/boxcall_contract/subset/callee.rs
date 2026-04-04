use super::super::super::super::*;
use serde_json::json;
#[test]
fn subset_accepts_global_callee_calls_without_legacy_func_reg() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Helper.echo/3",
                "params": [0, 1, 2],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "copy", "dst": 3, "src": 1 },
                        { "op": "ret", "value": 3 }
                    ]
                }]
            },
            {
                "name": "main",
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 10 } },
                        { "op": "const", "dst": 2, "value": { "type": "i64", "value": 20 } },
                        { "op": "const", "dst": 3, "value": { "type": "i64", "value": 30 } },
                        {
                            "op": "call",
                            "dst": 4,
                            "callee": { "type": "Global", "name": "Helper.echo/3" },
                            "args": [1, 2, 3]
                        },
                        { "op": "ret", "value": 4 }
                    ]
                }]
            }
        ]
    })
    .to_string();

    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}


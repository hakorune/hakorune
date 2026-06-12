use super::super::super::*;
use serde_json::json;

#[test]
fn extract_payload_omits_function_table_when_main_has_no_global_calls() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    { "op": "const", "dst": 1, "value": { "type": "i64", "value": 7 } },
                    { "op": "ret", "value": 1 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");

    assert!(
        payload_v.get("functions").is_none(),
        "reduced payload must not carry full function tables when no global call needs them"
    );
}

#[test]
fn extract_payload_keeps_only_transitively_reachable_global_functions() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Helper.echo/1",
                "params": [0],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        {
                            "op": "call",
                            "dst": 2,
                            "callee": { "type": "Global", "name": "Helper.leaf/1" },
                            "args": [0]
                        },
                        { "op": "ret", "value": 2 }
                    ]
                }]
            },
            {
                "name": "Helper.leaf/1",
                "params": [0],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "copy", "dst": 1, "src": 0 },
                        { "op": "ret", "value": 1 }
                    ]
                }]
            },
            {
                "name": "Helper.dead/0",
                "params": [],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 99 } },
                        { "op": "ret", "value": 1 }
                    ]
                }]
            },
            {
                "name": "main",
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 11 } },
                        {
                            "op": "call",
                            "dst": 4,
                            "callee": { "type": "Global", "name": "Helper.echo/1" },
                            "args": [1]
                        },
                        { "op": "ret", "value": 4 }
                    ]
                }]
            }
        ]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let functions = payload_v["functions"].as_array().expect("functions");
    let names: Vec<_> = functions
        .iter()
        .filter_map(|f| f["name"].as_str())
        .collect();

    assert_eq!(names, vec!["Helper.echo/1", "Helper.leaf/1"]);
}

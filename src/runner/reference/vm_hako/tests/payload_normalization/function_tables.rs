use super::super::super::*;
use serde_json::json;

#[test]
fn extract_payload_keeps_function_table_for_global_calls() {
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
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 11 } },
                        { "op": "const", "dst": 2, "value": { "type": "i64", "value": 22 } },
                        { "op": "const", "dst": 3, "value": { "type": "i64", "value": 33 } },
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

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let functions = payload_v["functions"].as_array().expect("functions");

    assert!(
        functions
            .iter()
            .any(|f| f["name"].as_str() == Some("Helper.echo/3")),
        "global call targets must stay in payload functions"
    );
}

#[test]
fn extract_payload_keeps_function_table_for_global_mir_calls() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Helper.echo/1",
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
                "name": "main",
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "const", "dst": 1, "value": { "type": "i64", "value": 11 } },
                        {
                            "op": "mir_call",
                            "dst": 2,
                            "mir_call": {
                                "callee": { "type": "Global", "name": "Helper.echo/1" },
                                "args": [1],
                                "effects": [],
                                "flags": {}
                            }
                        },
                        { "op": "ret", "value": 2 }
                    ]
                }]
            }
        ]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let functions = payload_v["functions"].as_array().expect("functions");

    assert!(
        functions
            .iter()
            .any(|f| f["name"].as_str() == Some("Helper.echo/1")),
        "global mir_call targets must stay in payload functions"
    );
}

#[test]
fn extract_payload_rewrites_global_mir_call_to_call_in_main_payload() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Helper.echo/1",
                "params": [0],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "ret", "value": 0 }
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
                            "op": "mir_call",
                            "dst": 2,
                            "mir_call": {
                                "callee": { "type": "Global", "name": "Helper.echo/1" },
                                "args": [1],
                                "effects": [],
                                "flags": {}
                            }
                        },
                        { "op": "ret", "value": 2 }
                    ]
                }]
            }
        ]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("main instructions");

    assert!(
        insts.iter().any(|inst| inst["op"].as_str() == Some("call")
            && inst["callee"]["type"].as_str() == Some("Global")
            && inst["callee"]["name"].as_str() == Some("Helper.echo/1")),
        "main global mir_call must normalize to call(Global) for the S0 named-function runner"
    );
    assert!(
        insts
            .iter()
            .all(|inst| inst["op"].as_str() != Some("mir_call")),
        "main payload should not retain global mir_call after normalization"
    );
}

#[test]
fn extract_payload_rewrites_global_mir_call_inside_reachable_function() {
    let mir_json = json!({
        "functions": [
            {
                "name": "Helper.leaf/1",
                "params": [0],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        { "op": "ret", "value": 0 }
                    ]
                }]
            },
            {
                "name": "Helper.mid/1",
                "params": [0],
                "entry_block": 0,
                "blocks": [{
                    "id": 0,
                    "instructions": [
                        {
                            "op": "mir_call",
                            "dst": 1,
                            "mir_call": {
                                "callee": { "type": "Global", "name": "Helper.leaf/1" },
                                "args": [0],
                                "effects": [],
                                "flags": {}
                            }
                        },
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
                            "op": "mir_call",
                            "dst": 2,
                            "mir_call": {
                                "callee": { "type": "Global", "name": "Helper.mid/1" },
                                "args": [1],
                                "effects": [],
                                "flags": {}
                            }
                        },
                        { "op": "ret", "value": 2 }
                    ]
                }]
            }
        ]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let functions = payload_v["functions"].as_array().expect("functions");
    let mid = functions
        .iter()
        .find(|f| f["name"].as_str() == Some("Helper.mid/1"))
        .expect("mid function");
    let insts = mid["blocks"][0]["instructions"]
        .as_array()
        .expect("mid instructions");

    assert!(
        insts.iter().any(|inst| inst["op"].as_str() == Some("call")
            && inst["callee"]["type"].as_str() == Some("Global")
            && inst["callee"]["name"].as_str() == Some("Helper.leaf/1")),
        "reachable helper global mir_call must normalize to call(Global)"
    );
    assert!(
        insts
            .iter()
            .all(|inst| inst["op"].as_str() != Some("mir_call")),
        "reachable helper payload should not retain global mir_call after normalization"
    );
}

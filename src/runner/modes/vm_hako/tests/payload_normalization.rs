use super::super::*;
use serde_json::json;

#[test]
fn extract_payload_normalizes_typeop_alias_fields() {
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
                        "op_kind": "Check",
                        "value": 1,
                        "ty": "Integer",
                        "dst": 2
                    },
                    { "op": "ret", "value": 2 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let inst = payload_v["blocks"][0]["instructions"][1].clone();
    assert_eq!(inst["operation"], json!("check"));
    assert_eq!(inst["src"], json!(1));
    assert_eq!(inst["target_type"], json!("Integer"));
}

#[test]
fn extract_payload_keeps_string_handle_const_used_by_phi_incoming() {
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
                            "value": {
                                "type": { "box_type": "StringBox", "kind": "handle" },
                                "value": "path.txt"
                            }
                        },
                        { "op": "jump", "target": 1 }
                    ]
                },
                {
                    "id": 1,
                    "instructions": [
                        {
                            "op": "phi",
                            "dst": 2,
                            "incoming": [[1, 0]]
                        },
                        { "op": "copy", "dst": 3, "src": 2 },
                        { "op": "ret", "value": 3 }
                    ]
                }
            ]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");

    let const_exists = payload_v["blocks"]
        .as_array()
        .expect("blocks")
        .iter()
        .flat_map(|b| {
            b["instructions"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .into_iter()
        })
        .any(|inst| inst["op"] == json!("const") && inst["dst"] == json!(1));

    assert!(
        const_exists,
        "string-handle const used only by phi incoming must not be pruned"
    );
}

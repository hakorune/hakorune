use super::super::super::*;
use serde_json::json;

#[test]
fn extract_payload_rewrites_map_method_mir_call_to_boxcall() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "a"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 42 }
                    },
                    {
                        "op": "mir_call",
                        "dst": null,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "set",
                                "receiver": 1
                            },
                            "args": [2, 3],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 4,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "get",
                                "receiver": 1
                            },
                            "args": [2],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 4 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let set_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("set"))
        .expect("rewritten set boxcall");
    assert_eq!(set_inst["box"], json!(1));
    assert_eq!(set_inst["args"], json!([2, 3]));

    let get_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("get"))
        .expect("rewritten get boxcall");
    assert_eq!(get_inst["box"], json!(1));
    assert_eq!(get_inst["args"], json!([2]));
    assert_eq!(get_inst["dst"], json!(4));
}

#[test]
fn extract_payload_rewrites_map_size_alias_to_boxcall_size() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 2,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "length",
                                "receiver": 1
                            },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 2 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let size_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall"))
        .expect("rewritten size boxcall");
    assert_eq!(size_inst["box"], json!(1));
    assert_eq!(size_inst["method"], json!("size"));
    assert_eq!(size_inst["args"], json!([]));
    assert_eq!(size_inst["dst"], json!(2));
}

#[test]
fn extract_payload_rewrites_map_keys_to_boxcall_keys() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 3,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "keys",
                                "receiver": 1
                            },
                            "args": [1],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 3 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let keys_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("keys"))
        .expect("rewritten keys boxcall");
    assert_eq!(keys_inst["box"], json!(1));
    assert_eq!(keys_inst["args"], json!([1]));
    assert_eq!(keys_inst["dst"], json!(3));
}

#[test]
fn extract_payload_rewrites_map_remove_alias_to_boxcall_delete() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 4,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "remove",
                                "receiver": 1
                            },
                            "args": [3],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 4 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let delete_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("delete"))
        .expect("rewritten delete boxcall");
    assert_eq!(delete_inst["box"], json!(1));
    assert_eq!(delete_inst["args"], json!([3]));
    assert_eq!(delete_inst["dst"], json!(4));
}

#[test]
fn extract_payload_rewrites_map_clear_to_boxcall_clear() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 4,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "clear",
                                "receiver": 1
                            },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 4 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let clear_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("clear"))
        .expect("rewritten clear boxcall");
    assert_eq!(clear_inst["box"], json!(1));
    assert_eq!(clear_inst["args"], json!([]));
    assert_eq!(clear_inst["dst"], json!(4));
}

#[test]
fn extract_payload_rewrites_map_values_to_boxcall_values() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 1,
                        "mir_call": {
                            "callee": { "type": "Constructor", "box_type": "MapBox" },
                            "args": [],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 3,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "MapBox",
                                "name": "values",
                                "receiver": 1
                            },
                            "args": [1],
                            "effects": [],
                            "flags": {}
                        }
                    },
                    { "op": "ret", "value": 3 }
                ]
            }]
        }]
    })
    .to_string();

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
    let insts = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions");

    let values_inst = insts
        .iter()
        .find(|inst| inst["op"] == json!("boxcall") && inst["method"] == json!("values"))
        .expect("rewritten values boxcall");
    assert_eq!(values_inst["box"], json!(1));
    assert_eq!(values_inst["args"], json!([1]));
    assert_eq!(values_inst["dst"], json!(3));
}

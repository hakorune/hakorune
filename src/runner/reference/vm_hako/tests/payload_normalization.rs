use super::super::*;
use serde_json::json;
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

fn with_vm_hako_strict_dev_env<F: FnOnce()>(f: F) {
    let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
    let prev_strict = std::env::var("HAKO_JOINIR_STRICT").ok();
    let prev_planner_required = std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").ok();
    let prev_debug = std::env::var("NYASH_JOINIR_DEV").ok();

    std::env::set_var("HAKO_JOINIR_STRICT", "1");
    std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
    std::env::set_var("NYASH_JOINIR_DEV", "1");

    f();

    match prev_strict {
        Some(v) => std::env::set_var("HAKO_JOINIR_STRICT", v),
        None => std::env::remove_var("HAKO_JOINIR_STRICT"),
    }
    match prev_planner_required {
        Some(v) => std::env::set_var("HAKO_JOINIR_PLANNER_REQUIRED", v),
        None => std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED"),
    }
    match prev_debug {
        Some(v) => std::env::set_var("NYASH_JOINIR_DEV", v),
        None => std::env::remove_var("NYASH_JOINIR_DEV"),
    }
}

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

#[test]
fn extract_payload_keeps_string_handle_const_used_by_mir_call_args() {
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
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "a"
                        }
                    },
                    {
                        "op": "mir_call",
                        "dst": 2,
                        "mir_call": {
                            "callee": { "type": "Global", "name": "print" },
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

    let payload = extract_main_payload_json(&mir_json).expect("payload");
    let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");

    let const_exists = payload_v["blocks"][0]["instructions"]
        .as_array()
        .expect("instructions")
        .iter()
        .any(|inst| inst["op"] == json!("const") && inst["dst"] == json!(1));

    assert!(
        const_exists,
        "string-handle const used by nested mir_call args must not be pruned"
    );
}

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

#[test]
fn extract_payload_keeps_direct_lower_helper_from_compiled_v0_main() {
    with_vm_hako_strict_dev_env(|| {
        let runner = NyashRunner::new(crate::cli::CliConfig::default());
        let source = r#"
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerBox
static box Main { method main(args){
  local j = env.get("PROG_JSON")
  if j == null { return 1 }
  local out = LowerBox.try_lower(j)
  if out == null { return 2 }
  return 0
}}
"#;

        let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>.hako", source)
            .expect("compile_source_to_mir_json_v0 should succeed");
        let payload = extract_main_payload_json(&mir_json).expect("payload");
        let payload_v: serde_json::Value = serde_json::from_str(&payload).expect("payload json");
        let functions = payload_v["functions"].as_array().expect("functions");

        assert!(
            functions.iter().any(|f| {
                f["name"].as_str() == Some("LowerReturnMethodArrayMapBox.try_lower/1")
            }),
            "reduced payload must retain direct-lower helper from compiled v0 main: {}",
            payload
        );
        let helper = functions
            .iter()
            .find(|f| f["name"].as_str() == Some("LowerReturnMethodArrayMapBox.try_lower/1"))
            .expect("direct-lower helper payload");
        assert!(
            helper.get("attrs").is_none() && helper.get("metadata").is_none(),
            "reduced payload helper must stay minimal (name/params/blocks only): {}",
            helper
        );
        let helper_obj = helper.as_object().expect("helper object");
        assert!(
            helper_obj.contains_key("name")
                && helper_obj.contains_key("params")
                && helper_obj.contains_key("blocks")
                && helper_obj
                    .keys()
                    .all(|k| matches!(k.as_str(), "blocks" | "entry_block" | "name" | "params")),
            "reduced payload helper must stay compact and order-agnostic for vm-hako lookup: {}",
            helper
        );
        assert!(
            payload.contains("\"name\":\"LowerReturnMethodArrayMapBox.try_lower/1\",\"params\":"),
            "reduced payload helper must keep compact name/params header for vm-hako lookup: {}",
            payload
        );
        assert!(
            payload.contains("[{\"blocks\":") || payload.contains(",{\"blocks\":"),
            "reduced payload helper must keep compact blocks-first object head for vm-hako lookup: {}",
            payload
        );
        assert!(
            helper.get("entry_block").is_none()
                && helper["blocks"]
                    .as_array()
                    .and_then(|blocks| blocks.first())
                    .and_then(|block| block["id"].as_i64())
                    .is_some(),
            "compact helper payload must infer entry from the first block when entry_block is omitted: {}",
            helper
        );
    });
}

use nyash_rust::mir::MirInstruction;
use serde_json::json;
use std::collections::BTreeMap;

fn program_env_get() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "v", "expr": { "type": "Method", "recv": { "type": "Var", "name": "env" }, "method": "get", "args": [ { "type": "Str", "value": "NYASH_TEST_KEY" } ] } },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ]
    })
}

#[test]
fn json_env_get_lowers_to_extern_call() {
    let src = serde_json::to_string(&program_env_get()).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .expect("lower Program(JSON)");

    let f = module
        .functions
        .get("main")
        .expect("module should contain main");

    let mut found = false;
    for bb in f.blocks.values() {
        for inst in &bb.instructions {
            // Look for Call with Callee::Extern("env.get")
            if let nyash_rust::mir::MirInstruction::Call {
                callee: Some(nyash_rust::mir::Callee::Extern(name)),
                ..
            } = inst
            {
                if name == "env.get" {
                    found = true;
                }
            }
        }
    }

    assert!(
        found,
        "expected JSON v0 bridge to lower env.get as Call(callee=Extern(\"env.get\"))"
    );
}

fn program_env_get_with_helper_runes() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "v", "expr": { "type": "Method", "recv": { "type": "Var", "name": "env" }, "method": "get", "args": [ { "type": "Str", "value": "NYASH_TEST_KEY" } ] } },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ],
        "defs": [
            {
                "name": "helper",
                "params": [],
                "box": "Helper",
                "attrs": {
                    "runes": [
                        { "name": "Public" }
                    ]
                },
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [
                        { "type": "Return", "expr": { "type": "Int", "value": 7 } }
                    ]
                }
            }
        ]
    })
}

fn program_env_get_with_entry_and_helper_runes() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "v", "expr": { "type": "Method", "recv": { "type": "Var", "name": "env" }, "method": "get", "args": [ { "type": "Str", "value": "NYASH_TEST_KEY" } ] } },
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ],
        "defs": [
            {
                "name": "main",
                "params": [],
                "box": "Main",
                "attrs": {
                    "runes": [
                        { "name": "Symbol", "args": ["main_sym"] },
                        { "name": "CallConv", "args": ["c"] }
                    ]
                },
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [
                        { "type": "Return", "expr": { "type": "Int", "value": 0 } }
                    ]
                }
            },
            {
                "name": "helper",
                "params": [],
                "box": "Helper",
                "attrs": {
                    "runes": [
                        { "name": "Public" }
                    ]
                },
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [
                        { "type": "Return", "expr": { "type": "Int", "value": 7 } }
                    ]
                }
            }
        ]
    })
}

#[test]
fn json_def_attrs_runes_survive_into_emitted_mir_json() {
    let src =
        serde_json::to_string(&program_env_get_with_helper_runes()).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .expect("lower Program(JSON)");

    let mir_json = nyash_rust::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&module)
        .expect("emit MIR JSON");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("parse MIR JSON");

    let functions = root
        .get("functions")
        .and_then(|v| v.as_array())
        .expect("functions array");

    let helper = functions
        .iter()
        .find(|func| func.get("name").and_then(|v| v.as_str()) == Some("Helper.helper/0"))
        .expect("Helper.helper/0 present");

    let helper_runes = helper
        .get("attrs")
        .and_then(|attrs| attrs.get("runes"))
        .and_then(|runes| runes.as_array())
        .expect("helper attrs.runes array");

    assert_eq!(helper_runes.len(), 1);
    assert_eq!(
        helper_runes[0].get("name").and_then(|v| v.as_str()),
        Some("Public")
    );

    let main = module.functions.get("main").expect("main present");
    let mut found_env_get = false;
    for bb in main.blocks.values() {
        for inst in &bb.instructions {
            if let MirInstruction::Call {
                callee: Some(nyash_rust::mir::Callee::Extern(name)),
                ..
            } = inst
            {
                if name == "env.get" {
                    found_env_get = true;
                }
            }
        }
    }

    assert!(
        found_env_get,
        "expected existing env.get lowering to remain unchanged"
    );
}

#[test]
fn json_stageb_entry_def_runes_attach_to_main_without_duplicate_main_def() {
    let src = serde_json::to_string(&program_env_get_with_entry_and_helper_runes())
        .expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .expect("lower Program(JSON)");

    assert!(
        !module.functions.contains_key("Main.main/0"),
        "synthetic Stage-B entry def must not materialize a duplicate Main.main/0 helper"
    );

    let mir_json = nyash_rust::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&module)
        .expect("emit MIR JSON");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("parse MIR JSON");
    let functions = root["functions"].as_array().expect("functions array");

    let main = functions
        .iter()
        .find(|func| func.get("name").and_then(|v| v.as_str()) == Some("main"))
        .expect("main present");

    let runes = main["attrs"]["runes"].as_array().expect("main attrs.runes");
    assert_eq!(runes.len(), 2);
    assert_eq!(runes[0]["name"], "Symbol");
    assert_eq!(runes[0]["args"], serde_json::json!(["main_sym"]));
    assert_eq!(runes[1]["name"], "CallConv");
    assert_eq!(runes[1]["args"], serde_json::json!(["c"]));
}

fn program_if_merge_local_return_var() -> serde_json::Value {
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Local", "name": "selected_input", "expr": { "type": "Str", "value": "base" } },
            {
                "type": "If",
                "cond": {
                    "type": "Compare",
                    "op": "==",
                    "lhs": { "type": "Int", "value": 1 },
                    "rhs": { "type": "Int", "value": 1 }
                },
                "then": [
                    { "type": "Local", "name": "selected_input", "expr": { "type": "Str", "value": "" } }
                ],
                "else": [
                    { "type": "Local", "name": "selected_input", "expr": { "type": "Str", "value": "source_text" } }
                ]
            },
            { "type": "Return", "expr": { "type": "Var", "name": "selected_input" } }
        ]
    })
}

#[test]
fn json_if_merge_local_return_var_uses_phi_at_join() {
    let src =
        serde_json::to_string(&program_if_merge_local_return_var()).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .expect("lower Program(JSON)");

    let f = module
        .functions
        .get("main")
        .expect("module should contain main");

    let mut phi_dst = None;
    for bb in f.blocks.values() {
        for inst in &bb.instructions {
            match inst {
                MirInstruction::Phi { dst, inputs, .. } => {
                    if inputs.len() == 2 {
                        phi_dst = Some(*dst);
                    }
                }
                _ => {}
            }
        }
    }

    assert!(
        phi_dst.is_some(),
        "expected branch-carried local merge to materialize a phi at join"
    );
}

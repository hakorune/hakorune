use super::super::*;
use serde_json::json;
use std::sync::{Mutex, OnceLock};

fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

#[test]
fn compile_v0_emits_mir_call_extern_hako_mem_alloc() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local p = externcall "hako_mem_alloc"(8)
    return p
  }
}
"#;
    let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
        .expect("compile_source_to_mir_json_v0 should succeed");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let inst = root["functions"]
        .as_array()
        .and_then(|funcs| funcs.iter().find(|f| f["name"].as_str() == Some("main")))
        .and_then(|main| main["blocks"].as_array())
        .and_then(|blocks| {
            blocks.iter().find_map(|b| {
                b["instructions"].as_array().and_then(|insts| {
                    insts.iter().find(|inst| {
                        inst["op"].as_str() == Some("mir_call")
                            && inst["mir_call"]["callee"]["type"].as_str() == Some("Extern")
                            && inst["mir_call"]["callee"]["name"].as_str()
                                == Some("hako_mem_alloc")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(Extern:hako_mem_alloc) must exist");
    assert_eq!(
        inst["mir_call"]["args"].as_array().map(|a| a.len()),
        Some(1),
        "extern hako_mem_alloc must receive one runtime arg: {}",
        inst
    );
    assert!(
        inst["dst"].is_number(),
        "extern hako_mem_alloc mir_call must carry dst: {}",
        inst
    );
}

fn with_joinir_strict_without_planner_required<F: FnOnce()>(f: F) {
    let _lock = env_guard().lock().unwrap_or_else(|e| e.into_inner());
    let prev_strict = std::env::var("HAKO_JOINIR_STRICT").ok();
    let prev_planner_required = std::env::var("HAKO_JOINIR_PLANNER_REQUIRED").ok();
    let prev_debug = std::env::var("NYASH_JOINIR_DEV").ok();

    std::env::set_var("HAKO_JOINIR_STRICT", "1");
    std::env::remove_var("HAKO_JOINIR_PLANNER_REQUIRED");
    std::env::remove_var("NYASH_JOINIR_DEV");

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
fn subset_accepts_boxcall_push_without_dst() {
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
                        "type": "ArrayBox"
                    },
                    {
                        "op": "copy",
                        "dst": 2,
                        "src": 1
                    },
                    {
                        "op": "boxcall",
                        "method": "birth",
                        "box": 2,
                        "dst": null,
                        "args": []
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "hello"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "push",
                        "box": 2,
                        "dst": null,
                        "args": [3]
                    },
                    {
                        "op": "boxcall",
                        "method": "length",
                        "box": 2,
                        "dst": 4,
                        "args": []
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_push_with_two_args() {
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
                        "type": "ArrayBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "push",
                        "box": 1,
                        "dst": null,
                        "args": [2, 3]
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err(("main".to_string(), 0, "boxcall(push:args!=1)".to_string()))
    );
}

#[test]
fn subset_accepts_boxcall_open_with_two_args() {
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
                        "type": "FileBox"
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "/tmp/nope.txt"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "r"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "open",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_open_with_non_reg_args() {
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
                        "type": "FileBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "open",
                        "box": 1,
                        "dst": 4,
                        "args": [2, "r"]
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
            "boxcall(open:args:non-reg)".to_string()
        ))
    );
}

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

#[test]
fn subset_accepts_boxcall_open_with_three_args() {
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
                        "type": "FileBox"
                    },
                    {
                        "op": "copy",
                        "dst": 2,
                        "src": 1
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "/tmp/nope.txt"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 4,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "r"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "open",
                        "box": 2,
                        "dst": 5,
                        "args": [2, 3, 4]
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_boxcall_set_with_two_args() {
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
                        "type": "MapBox"
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
                        "op": "boxcall",
                        "method": "set",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_set_with_non_reg_args() {
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
                        "type": "MapBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "set",
                        "box": 1,
                        "dst": 4,
                        "args": [2, "x"]
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
            "boxcall(setField:args:non-reg)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_boxcall_setfield_with_two_args() {
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
                        "type": "MapBox"
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
                        "op": "boxcall",
                        "method": "setField",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_setfield_with_non_reg_args() {
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
                        "type": "MapBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "setField",
                        "box": 1,
                        "dst": 4,
                        "args": [2, "x"]
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
            "boxcall(setField:args:non-reg)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_boxcall_has_with_one_arg() {
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
                        "type": "MapBox"
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
                        "op": "boxcall",
                        "method": "has",
                        "box": 1,
                        "dst": 3,
                        "args": [2]
                    },
                    {
                        "op": "ret",
                        "value": 3
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_boxcall_substring_with_two_args() {
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
                            "value": "abcdef"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 1 }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 4 }
                    },
                    {
                        "op": "boxcall",
                        "method": "substring",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_substring_with_non_reg_args() {
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
                            "value": "abcdef"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "substring",
                        "box": 1,
                        "dst": 4,
                        "args": [1, "x"]
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
            "boxcall(substring:args:non-reg)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_boxcall_indexof_with_one_arg() {
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
                            "value": "abcdef"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "bc"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "indexOf",
                        "box": 1,
                        "dst": 3,
                        "args": [2]
                    },
                    {
                        "op": "ret",
                        "value": 3
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_indexof_without_arg() {
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
                            "value": "abcdef"
                        }
                    },
                    {
                        "op": "boxcall",
                        "method": "indexOf",
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
            "boxcall(indexOf:args!=1or2)".to_string()
        ))
    );
}

#[test]
fn subset_accepts_boxcall_indexof_with_two_args() {
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
                            "value": "a|||"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "|||"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "boxcall",
                        "method": "indexOf",
                        "box": 1,
                        "dst": 4,
                        "args": [2, 3]
                    },
                    {
                        "op": "ret",
                        "value": 4
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_length_with_arg() {
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
                        "type": "ArrayBox"
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": { "type": "i64", "value": 0 }
                    },
                    {
                        "op": "boxcall",
                        "method": "length",
                        "box": 1,
                        "dst": 3,
                        "args": [2]
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(
        out,
        Err(("main".to_string(), 0, "boxcall(length:args!=0)".to_string()))
    );
}

#[test]
fn subset_accepts_boxcall_link_exe_with_three_args() {
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
                        "type": "LlvmBackendBox"
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "/tmp/in.o"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 3,
                        "value": {
                            "type": { "box_type": "StringBox", "kind": "handle" },
                            "value": "/tmp/out.exe"
                        }
                    },
                    {
                        "op": "const",
                        "dst": 4,
                        "value": { "type": "void", "value": 0 }
                    },
                    {
                        "op": "boxcall",
                        "method": "link_exe",
                        "box": 1,
                        "dst": 5,
                        "args": [2, 3, 4]
                    },
                    {
                        "op": "ret",
                        "value": 5
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_accepts_boxcall_read_with_receiver_mirror_arg() {
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
                        "type": "FileBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "read",
                        "box": 1,
                        "dst": 2,
                        "args": [1]
                    }
                ]
            }]
        }]
    })
    .to_string();
    let out = check_vm_hako_subset_json(&mir_json);
    assert_eq!(out, Ok(()));
}

#[test]
fn subset_rejects_boxcall_read_with_non_reg_receiver_mirror_arg() {
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
                        "type": "FileBox"
                    },
                    {
                        "op": "boxcall",
                        "method": "read",
                        "box": 1,
                        "dst": 2,
                        "args": ["x"]
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
            "boxcall(read:arg0:non-reg)".to_string()
        ))
    );
}

#[test]
fn compile_v0_emits_mir_call_open_with_two_args() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
static box Main {
  main() {
    local f = new FileBox()
    return f.open("/tmp/phase29y_missing_input.txt", "r")
  }
}
"#;
    let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
        .expect("compile_source_to_mir_json_v0 should succeed");
    let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");
    let inst = root["functions"]
        .as_array()
        .and_then(|funcs| funcs.iter().find(|f| f["name"].as_str() == Some("main")))
        .and_then(|main| main["blocks"].as_array())
        .and_then(|blocks| {
            blocks.iter().find_map(|b| {
                b["instructions"].as_array().and_then(|insts| {
                    insts.iter().find(|inst| {
                        inst["op"].as_str() == Some("mir_call")
                            && inst["mir_call"]["callee"]["type"].as_str() == Some("Method")
                            && inst["mir_call"]["callee"]["name"].as_str() == Some("open")
                    })
                })
            })
        })
        .cloned()
        .expect("main mir_call(open) must exist");
    let args_len = inst["mir_call"]["args"].as_array().map(|a| a.len()).unwrap_or(0);
    assert!(
        args_len >= 2,
        "unexpected open args shape: {}",
        inst
    );
    assert!(
        inst["mir_call"]["callee"]["receiver"].is_number(),
        "open mir_call must carry receiver: {}",
        inst
    );
}

#[test]
fn merge_prelude_text_with_imports_resolves_nested_static_box_aliases() {
    let runner = NyashRunner::new(crate::cli::CliConfig::default());
    let source = r#"
using "hako.mir.builder.internal.lower_return_method_array_map" as LowerBox
static box Main {
  main(args) {
    return 0
  }
}
"#;

    let (_merged, imports) =
        crate::runner::modes::common_util::resolve::merge_prelude_text_with_imports(
            &runner, source, "<inline>",
        )
        .expect("merge with imports");

    assert_eq!(
        imports.get("LowerBox").map(String::as_str),
        Some("LowerReturnMethodArrayMapBox")
    );
    assert_eq!(
        imports.get("JsonFragBox").map(String::as_str),
        Some("JsonFragBox")
    );
    assert_eq!(
        imports.get("PatternUtilBox").map(String::as_str),
        Some("PatternUtilBox")
    );
    assert_eq!(
        imports.get("MethodAliasPolicy").map(String::as_str),
        Some("MethodAliasPolicy")
    );
}

#[test]
fn compile_v0_uses_imported_static_box_alias_without_newbox_aliases() {
    with_joinir_strict_without_planner_required(|| {
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

        let mir_json = compile_source_to_mir_json_v0(&runner, "<inline>", source)
            .expect("compile should work in strict without planner_required");
        let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");

        fn has_newbox_alias(root: &serde_json::Value, alias: &str) -> bool {
            root["functions"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|func| func["blocks"].as_array().into_iter().flatten())
                .flat_map(|block| block["instructions"].as_array().into_iter().flatten())
                .any(|inst| {
                    inst["op"].as_str() == Some("newbox") && inst["type"].as_str() == Some(alias)
                })
        }

        fn find_direct_try_lower_call<'a>(
            root: &'a serde_json::Value,
        ) -> Option<&'a serde_json::Value> {
            root["functions"].as_array().and_then(|funcs| {
                funcs.iter().find_map(|func| {
                    func["blocks"].as_array().and_then(|blocks| {
                        blocks.iter().find_map(|block| {
                            block["instructions"].as_array().and_then(|insts| {
                                insts.iter().find(|inst| {
                                    inst["op"].as_str() == Some("call")
                                        && inst["callee"]["name"].as_str()
                                            == Some("LowerReturnMethodArrayMapBox.try_lower/1")
                                })
                            })
                        })
                    })
                })
            })
        }

        for alias in [
            "LowerBox",
            "JsonFragBox",
            "PatternUtilBox",
            "MethodAliasPolicy",
        ] {
            assert!(
                !has_newbox_alias(&root, alias),
                "import alias {} should not be materialized as newbox: {}",
                alias,
                mir_json
            );
        }

        let try_lower_call = find_direct_try_lower_call(&root).unwrap_or_else(|| {
            panic!(
                "direct-lower concrete global call must exist in emitted MIR JSON: {}",
                mir_json
            )
        });
        assert!(
            try_lower_call["callee"]["type"].as_str() == Some("Global"),
            "selected static call must stay Global, not receiver-less Method: {}",
            try_lower_call
        );
        assert!(
            try_lower_call.get("func").is_none(),
            "direct static call should carry concrete callee and no legacy func indirection: {}",
            try_lower_call
        );
        assert!(
            !mir_json.contains("\"receiver\":null"),
            "direct-lower static alias must not regress to Method{{receiver:null}}: {}",
            mir_json
        );
    });
}

#[test]
fn direct_source_route_keeps_imported_static_box_alias_as_concrete_global_call() {
    with_joinir_strict_without_planner_required(|| {
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

        let prepared = crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
            &runner,
            "<inline>.hako",
            source,
        )
        .expect("direct source prep should resolve using imports");
        let ast = crate::parser::NyashParser::parse_from_string(&prepared.code)
            .expect("prepared direct source should parse");
        let ast = crate::r#macro::maybe_expand_and_dump(&ast, false);
        let mut compiler = crate::mir::MirCompiler::with_options(true);
        let compile =
            crate::runner::modes::common_util::source_hint::compile_with_source_hint_and_imports(
                &mut compiler,
                ast,
                Some("<inline>.hako"),
                prepared.imports,
            )
            .expect("direct source compile should succeed");
        let mir_json =
            crate::runner::mir_json_emit::emit_mir_json_string_for_harness_bin(&compile.module)
                .expect("emit mir json");
        let root: serde_json::Value = serde_json::from_str(&mir_json).expect("valid mir json");

        let direct_try_lower_call = root["functions"]
            .as_array()
            .and_then(|funcs| {
                funcs
                    .iter()
                    .find(|func| func["name"].as_str() == Some("main"))
            })
            .and_then(|func| func["blocks"].as_array())
            .and_then(|blocks| {
                blocks.iter().find_map(|block| {
                    block["instructions"].as_array().and_then(|insts| {
                        insts.iter().find(|inst| {
                            inst["op"].as_str() == Some("mir_call")
                                && inst["mir_call"]["callee"]["name"].as_str()
                                    == Some("LowerReturnMethodArrayMapBox.try_lower/1")
                        })
                    })
                })
            })
            .unwrap_or_else(|| {
                panic!(
                    "direct source route must emit concrete Global try_lower call in main: {}",
                    mir_json
                )
            });

        assert_eq!(
            direct_try_lower_call["mir_call"]["callee"]["type"].as_str(),
            Some("Global"),
            "direct source route must keep imported static box alias as Global: {}",
            direct_try_lower_call
        );
        assert!(
            !mir_json.contains("\"box_name\":\"LowerBox\""),
            "direct source route must not leave alias box_name in emitted MIR: {}",
            mir_json
        );
        assert!(
            !mir_json.contains("\"receiver\":null"),
            "direct source route must not regress to Method{{receiver:null}}: {}",
            mir_json
        );
    });
}

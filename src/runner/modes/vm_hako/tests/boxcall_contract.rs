use super::super::*;
use serde_json::json;

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
fn compile_v0_emits_boxcall_open_with_two_args() {
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
                        inst["op"].as_str() == Some("boxcall")
                            && inst["method"].as_str() == Some("open")
                    })
                })
            })
        })
        .cloned()
        .expect("main boxcall(open) must exist");
    let args_len = inst["args"].as_array().map(|a| a.len()).unwrap_or(0);
    assert_eq!(args_len, 3, "unexpected open shape: {}", inst);
}

use super::super::super::*;
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
        Err(("main".to_string(), 0, "boxcall(set:args:non-reg)".to_string()))
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
fn subset_accepts_mir_call_link_exe_with_three_args() {
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
                        "op": "mir_call",
                        "dst": 5,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "LlvmBackendBox",
                                "name": "link_exe",
                                "receiver": 1,
                                "certainty": "Known"
                            },
                            "args": [2, 3, 4],
                            "effects": [],
                            "flags": {}
                        }
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

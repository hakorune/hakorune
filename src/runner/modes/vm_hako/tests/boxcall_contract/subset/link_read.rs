use super::super::super::super::*;
use serde_json::json;
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

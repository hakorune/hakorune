use super::super::super::super::*;
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


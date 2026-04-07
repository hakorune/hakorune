use super::super::super::super::*;
use serde_json::json;
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

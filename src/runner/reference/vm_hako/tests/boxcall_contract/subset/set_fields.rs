use super::super::super::super::*;
use serde_json::json;
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


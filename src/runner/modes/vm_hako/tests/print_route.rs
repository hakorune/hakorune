use super::super::*;
use serde_json::json;

#[test]
fn mir_call_print_rejected_in_all_modes() {
    let inst = json!({
        "op": "mir_call",
        "mir_call": {
            "callee": { "name": "print" },
            "args": [3]
        }
    });
    let out = parse_print_arg_from_instruction(&inst, &HashMap::new());
    assert_eq!(out, Err("mir_call(legacy-removed)"));
}

#[test]
fn externcall_print_still_accepted() {
    let inst = json!({
        "op": "externcall",
        "func": "nyash.console.log",
        "args": [3]
    });
    let out = parse_print_arg_from_instruction(&inst, &HashMap::new());
    assert_eq!(out, Ok(Some(3)));
}

#[test]
fn mir_call_print_rejected_by_subset_check() {
    let mir_json = json!({
        "functions": [{
            "name": "main",
            "entry_block": 0,
            "blocks": [{
                "id": 0,
                "instructions": [{
                    "op": "mir_call",
                    "mir_call": {
                        "callee": { "name": "print" },
                        "args": [3]
                    }
                }]
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
            "mir_call(legacy-removed)".to_string()
        ))
    );
}

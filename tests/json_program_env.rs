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

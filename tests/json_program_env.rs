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


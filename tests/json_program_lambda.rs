use serde_json::json;
use std::collections::BTreeMap;

fn verify_program(label: &str, program: serde_json::Value) {
    let src = serde_json::to_string(&program).expect("serialize program");
    let module = nyash_rust::runner::json_v0_bridge::parse_json_v0_to_module_with_imports(
        &src,
        BTreeMap::new(),
    )
    .unwrap_or_else(|e| panic!("{}: failed to parse Program(JSON): {}", label, e));

    let mut verifier = nyash_rust::mir::verification::MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&module) {
        panic!(
            "{}: MIR verification failed with errors: {:?}",
            label, errors
        );
    }
}

fn program_stageb_legacy_fn_literal_pair_in_def() -> serde_json::Value {
    // Stage-B legacy encoding splits `fn(x) { return x }` into:
    //   - return fn(x)
    //   - a standalone BlockExpr statement holding the body
    // The JSON v0 bridge should re-associate that into a single lambda literal.
    json!({
        "version": 0,
        "kind": "Program",
        "body": [
            { "type": "Return", "expr": { "type": "Int", "value": 0 } }
        ],
        "defs": [
            {
                "name": "make",
                "params": ["me"],
                "box": "Helper",
                "body": {
                    "version": 0,
                    "kind": "Program",
                    "body": [
                        { "type": "Return", "expr": { "type": "Call", "name": "fn", "args": [ { "type": "Var", "name": "x" } ] } },
                        { "type": "Expr", "expr": { "type": "BlockExpr", "prelude": [], "tail": { "type": "Return", "expr": { "type": "Var", "name": "x" } } } }
                    ]
                }
            }
        ]
    })
}

#[test]
fn json_stageb_legacy_fn_literal_pair_in_def_verifies() {
    verify_program(
        "json_stageb_legacy_fn_literal_pair_in_def",
        program_stageb_legacy_fn_literal_pair_in_def(),
    );
}

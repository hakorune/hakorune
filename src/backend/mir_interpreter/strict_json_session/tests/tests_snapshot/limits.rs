use super::*;
use crate::analysis::bounded_body_snapshot_v0::{BudgetLimitV0, SnapshotBuildErrorV0};

const OUTCOME_ENTRY: &str = "SnapshotDirectReaderFixtureV0Box.outcome/2";

fn direct_outcome(
    interpreter: &mut MirInterpreter,
    module: &crate::mir::MirModule,
    input: &str,
) -> VMValue {
    run(interpreter, module, input, OUTCOME_ENTRY)
}

fn assert_ready(interpreter: &mut MirInterpreter, module: &crate::mir::MirModule, input: &str) {
    rust_snapshot(input);
    assert_eq!(
        direct_outcome(interpreter, module, input),
        VMValue::String("Ready||".to_string()),
    );
    assert!(!interpreter.strict_json_session_active());
}

fn assert_limit(
    interpreter: &mut MirInterpreter,
    module: &crate::mir::MirModule,
    input: &str,
    rust_limit: BudgetLimitV0,
    hako_path: &str,
    hako_reason: &str,
) {
    let view = crate::analysis::bounded_body_snapshot_v0::read_program_v0_body(input)
        .expect("limit fixture must be structurally valid ProgramV0");
    assert_eq!(
        build_snapshot_from_validated_view_v0(&view),
        Err(SnapshotBuildErrorV0::Budget(rust_limit)),
    );
    assert_eq!(
        direct_outcome(interpreter, module, input),
        VMValue::String(format!("Unsupported|{hako_path}|{hako_reason}")),
    );
    assert!(!interpreter.strict_json_session_active());
}

fn decoded_text_with_bytes(bytes: usize) -> String {
    let cats = bytes / 3;
    let ascii = bytes % 3;
    format!("{}{}", "猫".repeat(cats), "x".repeat(ascii))
}

fn deep_binary(binary_count: usize) -> serde_json::Value {
    let mut value = serde_json::json!({"type":"Int","value":0});
    for _ in 0..binary_count {
        value = serde_json::json!({
            "type":"Binary","op":"+","lhs":value,"rhs":{"type":"Int","value":0}
        });
    }
    value
}

fn direct_limit_parity_impl() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();

    for binary_count in [61, 62] {
        let input = serde_json::json!({
            "version":0,"kind":"Program",
            "body":[{"type":"Expr","expr":deep_binary(binary_count)}]
        })
        .to_string();
        assert_ready(&mut interpreter, &module, &input);
    }
    let input = serde_json::json!({
        "version":0,"kind":"Program",
        "body":[{"type":"Expr","expr":deep_binary(63)}]
    })
    .to_string();
    assert_limit(
        &mut interpreter,
        &module,
        &input,
        BudgetLimitV0::Depth,
        &format!("$.body[0].expr{}", ".lhs".repeat(63)),
        "limit.max_depth",
    );

    for count in [2_047, 2_048] {
        let input = serde_json::json!({
            "version":0,"kind":"Program","body":vec![serde_json::json!({"type":"Break"}); count]
        })
        .to_string();
        assert_ready(&mut interpreter, &module, &input);
    }
    let input = serde_json::json!({
        "version":0,"kind":"Program","body":vec![serde_json::json!({"type":"Break"}); 2_049]
    })
    .to_string();
    assert_limit(
        &mut interpreter,
        &module,
        &input,
        BudgetLimitV0::ChildrenPerBody,
        "$.body",
        "limit.max_children_per_body",
    );

    for count in [127, 128] {
        let input = serde_json::json!({
            "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
                "type":"Call","name":"f","args":vec![serde_json::json!({"type":"Null"}); count]
            }}]
        })
        .to_string();
        assert_ready(&mut interpreter, &module, &input);
    }
    let input = serde_json::json!({
        "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
            "type":"Call","name":"f","args":vec![serde_json::json!({"type":"Null"}); 129]
        }}]
    })
    .to_string();
    assert_limit(
        &mut interpreter,
        &module,
        &input,
        BudgetLimitV0::Arguments,
        "$.body[0].expr.args",
        "limit.max_arguments",
    );

    for bytes in [1_023, 1_024] {
        let input = serde_json::json!({
            "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
                "type":"Var","name":decoded_text_with_bytes(bytes)
            }}]
        })
        .to_string();
        assert_ready(&mut interpreter, &module, &input);
    }
    let input = serde_json::json!({
        "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
            "type":"Var","name":decoded_text_with_bytes(1_025)
        }}]
    })
    .to_string();
    assert_limit(
        &mut interpreter,
        &module,
        &input,
        BudgetLimitV0::AtomBytes,
        "$.body[0].expr.name",
        "limit.max_atom_bytes",
    );

    for bytes in [65_535, 65_536] {
        let input = serde_json::json!({
            "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
                "type":"Str","value":decoded_text_with_bytes(bytes)
            }}]
        })
        .to_string();
        assert_ready(&mut interpreter, &module, &input);
    }
    let input = serde_json::json!({
        "version":0,"kind":"Program","body":[{"type":"Expr","expr":{
            "type":"Str","value":decoded_text_with_bytes(65_537)
        }}]
    })
    .to_string();
    assert_limit(
        &mut interpreter,
        &module,
        &input,
        BudgetLimitV0::LiteralBytes,
        "$.body[0].expr.value",
        "limit.max_literal_bytes",
    );
}

#[test]
fn hako_direct_snapshot_reader_matches_all_inclusive_limit_boundaries() {
    std::thread::Builder::new()
        .name("hako-direct-limit-parity".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(direct_limit_parity_impl)
        .expect("spawn direct limit parity thread")
        .join()
        .expect("direct limit parity thread");
}

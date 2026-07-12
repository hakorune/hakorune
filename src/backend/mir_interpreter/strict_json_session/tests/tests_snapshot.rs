use super::*;
use crate::analysis::bounded_body_snapshot_v0::{
    build_snapshot_from_validated_view_v0, AtomValueV0, BoundedBodyAnalysisSnapshotV0,
    ProgramV0BodyViewError, WireNodeKindV0,
};

mod ast_oracle;
mod limits;
mod negative;
mod summary;

fn compile_fixture() -> crate::mir::MirModule {
    compile_hako_fixture("tools/checks/fixtures/bounded_body_snapshot_direct_reader_v0.hako")
}

fn run(
    interpreter: &mut MirInterpreter,
    module: &crate::mir::MirModule,
    input: &str,
    name: &str,
) -> VMValue {
    run_session_function(interpreter, module, input, name).expect("direct snapshot reader")
}

fn rust_outcome(input: &str) -> String {
    match crate::analysis::bounded_body_snapshot_v0::read_program_v0_body(input) {
        Ok(_) => "Ready||".to_string(),
        Err(ProgramV0BodyViewError::Unsupported { path, reason, .. }) => {
            format!("Unsupported|{path}|{reason}")
        }
        Err(ProgramV0BodyViewError::InvalidInput { path, reason }) => {
            format!("InvalidInput|{path}|{reason}")
        }
    }
}

fn rust_snapshot(input: &str) -> BoundedBodyAnalysisSnapshotV0 {
    let view = crate::analysis::bounded_body_snapshot_v0::read_program_v0_body(input)
        .expect("validated ProgramV0");
    build_snapshot_from_validated_view_v0(&view).expect("Rust snapshot witness")
}

fn snapshot_signature(snapshot: &BoundedBodyAnalysisSnapshotV0) -> String {
    let mut text = format!(
        "v={},src={},n={},d={}",
        snapshot.schema_version(),
        snapshot.source_program_version(),
        snapshot.node_count(),
        snapshot.max_depth_observed()
    );
    for (index, node) in snapshot.nodes().iter().enumerate() {
        let (domain, kind) = match node.kind() {
            WireNodeKindV0::Stmt(kind) => ("stmt", format!("{kind:?}")),
            WireNodeKindV0::Expr(kind) => ("expr", format!("{kind:?}")),
        };
        text.push_str(&format!("|{index}:{}:{domain}:{kind}", node.path()));
        for (key, value) in node.atoms() {
            let (value_kind, value) = match value {
                AtomValueV0::I64(value) => ("I64", value.to_string()),
                AtomValueV0::Bool(value) => ("Bool", value.to_string()),
                AtomValueV0::Text(value) => ("Text", value.clone()),
                AtomValueV0::Null => ("Null", "null".to_string()),
            };
            text.push_str(&format!(":a({},{value_kind},{value})", key.wire_text()));
        }
        for (role, target) in node.children() {
            text.push_str(&format!(":c({},{target})", role.wire_text()));
        }
    }
    text
}

fn direct_snapshot_parity_impl() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let empty = r#"{"version":0,"kind":"Program","body":[]}"#;
    let full = serde_json::json!({
        "version":0,"kind":"Program","body":[
            {"type":"Local","name":"猫","expr":{"type":"Int","value":"-7"}},
            {"type":"Expr","expr":{"type":"Method","recv":{"type":"Var","name":"x"},"method":"m","args":[
                {"type":"Call","name":"f","args":[{"type":"Str","value":"😸"}]},
                {"type":"Field","recv":{"type":"Var","name":"r"},"field":"value"},
                {"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Compare","op":"<=","lhs":{"type":"Int","value":2},"rhs":{"type":"Int","value":3}}},
                {"type":"Logical","op":"&&","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}
            ]}},
            {"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Break"}],"else":[{"type":"Continue"}]},
            {"type":"Loop","cond":{"type":"Bool","value":false},"body":[]},
            {"type":"LoopRange","var_name":"i","start":{"type":"Int","value":0},"end":{"type":"Int","value":2},"body":[]},
            {"type":"Return","expr":{"type":"Null"}}
        ]
    }).to_string();
    for input in [empty, full.as_str()] {
        let expected = snapshot_signature(&rust_snapshot(input));
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                input,
                "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2"
            ),
            VMValue::String(expected)
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_direct_snapshot_reader_matches_empty_and_full_rust_snapshots() {
    std::thread::Builder::new()
        .name("hako-direct-snapshot-parity".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(direct_snapshot_parity_impl)
        .expect("spawn parity thread")
        .join()
        .expect("direct snapshot parity thread");
}

#[test]
fn hako_direct_snapshot_reader_matches_reference_failure_outcomes() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[{"type":"Future"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Extern"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"FastMemRegion"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Binary","op":"==","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":null}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Return","expr":null}]}"#,
    ] {
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                input,
                "SnapshotDirectReaderFixtureV0Box.outcome/2"
            ),
            VMValue::String(rust_outcome(input)),
            "input={input}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_direct_snapshot_reader_never_publishes_failure_as_snapshot() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[{"type":"Throw"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Local","name":"x"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"ArrayLiteral"}}]}"#,
    ] {
        let value = run(
            &mut interpreter,
            &module,
            input,
            "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2",
        );
        let VMValue::String(value) = value else {
            panic!("failure signature must be text")
        };
        assert!(
            !value.starts_with("v="),
            "failure published a snapshot: {value}"
        );
    }
}

fn operator_corpus_parity_impl() {
    let module = compile_fixture();
    let mut body = Vec::new();
    for (kind, operators) in [
        (
            "Binary",
            &["+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>"][..],
        ),
        ("Compare", &["==", "!=", "<", ">", "<=", ">="][..]),
        ("Logical", &["&&", "||"][..]),
    ] {
        for operator in operators {
            body.push(serde_json::json!({
                "type":"Expr","expr":{
                    "type":kind,"op":operator,
                    "lhs":{"type":"Int","value":1},
                    "rhs":{"type":"Int","value":2}
                }
            }));
        }
    }
    let input = serde_json::json!({"version":0,"kind":"Program","body":body}).to_string();
    let expected = snapshot_signature(&rust_snapshot(&input));
    let mut interpreter = MirInterpreter::new();
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            &input,
            "SnapshotDirectReaderFixtureV0Box.snapshot_signature/2",
        ),
        VMValue::String(expected)
    );
}

#[test]
fn hako_direct_snapshot_reader_matches_all_operator_corpus() {
    std::thread::Builder::new()
        .name("hako-direct-operator-parity".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(operator_corpus_parity_impl)
        .expect("spawn operator parity thread")
        .join()
        .expect("operator parity thread");
}

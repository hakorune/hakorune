use super::*;

fn compile_fixture() -> crate::mir::MirModule {
    compile_hako_fixture("tools/checks/fixtures/bounded_body_snapshot_stmt_reader_v0.hako")
}

fn run(
    interpreter: &mut MirInterpreter,
    module: &crate::mir::MirModule,
    input: &str,
    name: &str,
) -> VMValue {
    run_session_function(interpreter, module, input, name).expect("statement reader")
}

fn rust_outcome(input: &str) -> String {
    use crate::analysis::bounded_body_snapshot_v0::ProgramV0BodyViewError;
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

#[test]
fn hako_stmt_reader_covers_all_accepted_kinds_and_reference_outcomes() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let cases = [
        (
            serde_json::json!({"type":"Local","name":"x","expr":{"type":"Null"}}),
            1,
        ),
        (serde_json::json!({"type":"Expr","expr":{"type":"Null"}}), 2),
        (
            serde_json::json!({"type":"If","cond":{"type":"Bool","value":true},"then":[],"else":null}),
            3,
        ),
        (
            serde_json::json!({"type":"If","cond":{"type":"Bool","value":true},"then":[]}),
            3,
        ),
        (
            serde_json::json!({"type":"Loop","cond":{"type":"Bool","value":true},"body":[]}),
            4,
        ),
        (
            serde_json::json!({"type":"LoopRange","var_name":"i","start":{"type":"Int","value":0},"end":{"type":"Int","value":1},"body":[]}),
            5,
        ),
        (
            serde_json::json!({"type":"Return","expr":{"type":"Null"}}),
            6,
        ),
        (serde_json::json!({"type":"Break"}), 7),
        (serde_json::json!({"type":"Continue"}), 8),
    ];
    for (statement, expected) in cases {
        let input =
            serde_json::json!({"version":0,"kind":"Program","body":[statement]}).to_string();
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &input,
                "SnapshotStmtReaderFixtureV0Box.classify/2"
            ),
            VMValue::Integer(expected)
        );
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &input,
                "SnapshotStmtReaderFixtureV0Box.outcome/2"
            ),
            VMValue::String(rust_outcome(&input))
        );
        assert!(!interpreter.strict_json_session_active());
    }
    for statement in [
        serde_json::json!({"type":"Local","name":"x"}),
        serde_json::json!({"type":"Local","name":"x","expr":{"type":"Null"},"declared_type":1}),
        serde_json::json!({"type":"If","cond":{"type":"Bool","value":true},"then":{}}),
        serde_json::json!({"type":"Loop","cond":{"type":"ArrayLiteral"},"body":[]}),
        serde_json::json!({"type":"Break","future":0}),
        serde_json::json!({"type":"Extern"}),
        serde_json::json!({"type":"FastMemRegion"}),
    ] {
        let input =
            serde_json::json!({"version":0,"kind":"Program","body":[statement]}).to_string();
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &input,
                "SnapshotStmtReaderFixtureV0Box.outcome/2"
            ),
            VMValue::String(rust_outcome(&input))
        );
    }
}

#[test]
fn hako_stmt_reader_flattens_roots_and_nested_body_roles_in_preorder() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let input = serde_json::json!({
        "version":0,"kind":"Program","body":[
            {"type":"Local","name":"x","expr":{"type":"Int","value":1}},
            {"type":"If","cond":{"type":"Var","name":"x"},"then":[{"type":"Break"}],"else":[{"type":"Continue"}]}
        ]
    }).to_string();
    let expected = concat!(
        "roots=2:0:2",
        "|0:$.body[0]:stmt:Local:expr=1",
        "|1:$.body[0].expr:expr:Int",
        "|2:$.body[1]:stmt:If:cond=3:then=4:else=5",
        "|3:$.body[1].cond:expr:Var",
        "|4:$.body[1].then[0]:stmt:Break",
        "|5:$.body[1].else[0]:stmt:Continue"
    );
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            &input,
            "SnapshotStmtReaderFixtureV0Box.flat_signature/2"
        ),
        VMValue::String(expected.to_string())
    );
    let atom_input = serde_json::json!({"version":0,"kind":"Program","body":[{"type":"LoopRange","var_name":"猫😸","start":{"type":"Int","value":0},"end":{"type":"Int","value":1},"body":[]}]}).to_string();
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            &atom_input,
            "SnapshotStmtReaderFixtureV0Box.first_atom/2"
        ),
        VMValue::String("猫😸".to_string())
    );
}

#[test]
fn hako_stmt_reader_enforces_body_limits_before_child_traversal() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for (count, expected) in [(2047, 7), (2048, 7), (2049, 20)] {
        let body = vec![serde_json::json!({"type":"Break"}); count];
        let input = serde_json::json!({"version":0,"kind":"Program","body":body}).to_string();
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &input,
                "SnapshotStmtReaderFixtureV0Box.classify/2"
            ),
            VMValue::Integer(expected),
            "body count={count}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
    for (count, expected) in [(2048, 3), (2049, 20)] {
        let nested = vec![serde_json::json!({"type":"Continue"}); count];
        let input = serde_json::json!({
            "version":0,"kind":"Program","body":[
                {"type":"If","cond":{"type":"Bool","value":true},"then":nested}
            ]
        })
        .to_string();
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &input,
                "SnapshotStmtReaderFixtureV0Box.classify/2",
            ),
            VMValue::Integer(expected),
            "nested body count={count}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

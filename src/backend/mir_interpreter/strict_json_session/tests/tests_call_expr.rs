use super::*;

fn compile_fixture() -> crate::mir::MirModule {
    compile_hako_fixture("tools/checks/fixtures/bounded_body_snapshot_call_expr_reader_v0.hako")
}

fn run(
    interpreter: &mut MirInterpreter,
    module: &crate::mir::MirModule,
    input: &str,
    name: &str,
) -> VMValue {
    run_session_function(interpreter, module, input, name).expect("call expression reader")
}

#[test]
fn hako_call_expr_reader_matches_reference_outcomes() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for expression in [
        r#"{"type":"Call","name":"f","args":[]}"#,
        r#"{"type":"Method","recv":{"type":"Var","name":"x"},"method":"f","args":[]}"#,
        r#"{"type":"Field","recv":{"type":"Var","name":"x"},"field":"y"}"#,
        r#"{"type":"Call","args":[]}"#,
        r#"{"type":"Method","recv":null,"method":"f","args":[]}"#,
        r#"{"type":"Call","name":1,"args":[]}"#,
        r#"{"type":"Call","name":"f","args":{}}"#,
        r#"{"type":"Field","recv":{"type":"Var","name":"x"},"field":"y","future":0}"#,
        r#"{"type":"Call","name":"f","args":[{"type":"ArrayLiteral"}]}"#,
    ] {
        let actual = run(
            &mut interpreter,
            &module,
            expression,
            "SnapshotCallExprReaderFixtureV0Box.outcome/2",
        );
        assert_eq!(actual, VMValue::String(rust_child_outcome(expression)));
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_call_expr_reader_flattens_mixed_recursion_in_schema_order() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let expression = r#"{"type":"Method","recv":{"type":"Var","name":"x"},"method":"m","args":[{"type":"Call","name":"f","args":[]},{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"Int","value":2}}]}"#;
    let expected = concat!(
        "root=0|0:$.body[0].expr:Method:recv=1:args=2:args=3",
        "|1:$.body[0].expr.recv:Var",
        "|2:$.body[0].expr.args[0]:Call",
        "|3:$.body[0].expr.args[1]:Binary:lhs=4:rhs=5",
        "|4:$.body[0].expr.args[1].lhs:Int",
        "|5:$.body[0].expr.args[1].rhs:Int"
    );
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            expression,
            "SnapshotCallExprReaderFixtureV0Box.tree_signature/2",
        ),
        VMValue::String("Method:recv=Var:args=Call:args=Binary:lhs=Int:rhs=Int".to_string())
    );
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            expression,
            "SnapshotCallExprReaderFixtureV0Box.flat_signature/2",
        ),
        VMValue::String(expected.to_string())
    );
    let mixed = r#"{"type":"Binary","op":"+","lhs":{"type":"Call","name":"f","args":[{"type":"Int","value":1}]},"rhs":{"type":"Field","recv":{"type":"Var","name":"x"},"field":"y"}}"#;
    let signature = run(
        &mut interpreter,
        &module,
        mixed,
        "SnapshotCallExprReaderFixtureV0Box.flat_signature/2",
    );
    let VMValue::String(signature) = signature else {
        panic!("flat signature must be text")
    };
    assert!(signature.contains(":Binary:lhs=1:rhs=3"));
    assert!(signature.contains("|1:$.body[0].expr.lhs:Call:args=2"));
    assert!(signature.contains("|3:$.body[0].expr.rhs:Field:recv=4"));
}

#[test]
fn hako_call_expr_reader_enforces_atom_and_argument_limits() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    let utf8 = r#"{"type":"Field","recv":{"type":"Var","name":"x"},"field":"猫😸"}"#;
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            utf8,
            "SnapshotCallExprReaderFixtureV0Box.atom_value/2",
        ),
        VMValue::String("猫😸".to_string())
    );
    for (count, expected) in [(127, 1), (128, 1), (129, 20)] {
        let args = vec![serde_json::json!({"type":"Null"}); count];
        let expression = serde_json::json!({"type":"Call", "name":"f", "args":args}).to_string();
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                &expression,
                "SnapshotCallExprReaderFixtureV0Box.classify/2",
            ),
            VMValue::Integer(expected),
            "argument count={count}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
    let oversized =
        serde_json::json!({"type":"Call", "name":"猫".repeat(342), "args":[]}).to_string();
    assert_eq!(
        run(
            &mut interpreter,
            &module,
            &oversized,
            "SnapshotCallExprReaderFixtureV0Box.classify/2",
        ),
        VMValue::Integer(21)
    );
}

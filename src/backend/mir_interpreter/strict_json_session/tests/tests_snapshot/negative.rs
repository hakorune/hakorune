use super::*;

#[test]
fn hako_direct_negative_corpus_propagates_nested_unsupported_positions() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"ArrayLiteral"},"then":[]}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Extern"}]}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Loop","cond":{"type":"Bool","value":true},"body":[{"type":"Throw"}]}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":{"type":"ArrayLiteral"}}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Call","name":"f","args":[{"type":"Int","value":1},{"type":"New"}]}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Method","recv":{"type":"BlockExpr"},"method":"m","args":[]}}]}"#,
    ] {
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                input,
                "SnapshotDirectReaderFixtureV0Box.outcome/2",
            ),
            VMValue::String(rust_outcome(input)),
            "input={input}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_direct_negative_corpus_matches_nested_invalid_inputs() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"value":true},"then":[]}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Loop","cond":{"type":"Bool","value":true},"body":{}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Binary","op":"+","lhs":{"type":"Int","value":1},"rhs":null}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Logical","op":"+","lhs":{"type":"Bool","value":true},"rhs":{"type":"Bool","value":false}}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Call","name":"f","args":[{"type":"Future"}]}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"9223372036854775808"}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Int","value":"1x"}}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Return","expr":{"type":"Null","future":0}}]}"#,
    ] {
        assert_eq!(
            run(
                &mut interpreter,
                &module,
                input,
                "SnapshotDirectReaderFixtureV0Box.outcome/2",
            ),
            VMValue::String(rust_outcome(input)),
            "input={input}"
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn strict_syntax_failures_precede_hako_reader_and_cleanup_session() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[],"body":[]}"#,
        r#"{"version":0,"kind":"Program","body":[],"\u0062ody":[]}"#,
        r#"{"version":0,"kind":"Program","body":[]} trailing"#,
        r#"{"version":0,"kind":"Program","body":[]}{"x":1}"#,
    ] {
        let error = run_session_function(
            &mut interpreter,
            &module,
            input,
            "SnapshotDirectReaderFixtureV0Box.outcome/2",
        )
        .expect_err("strict syntax must fail before Hako");
        assert!(error.to_string().contains(INPUT_TAG));
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn unsupported_backend_fails_before_parse_session_and_hako_effects() {
    let module = compile_fixture();
    for backend in [
        "ny-llvmc-exe",
        "ny-llvmc-obj",
        "llvmlite-obj",
        "llvm-legacy-obj",
        "llvm-mock-fallback",
        "pyvm-harness",
        "wasm",
        "wasm-v2",
    ] {
        let mut interpreter = MirInterpreter::new();
        let error = interpreter
            .probe_strict_json_preflight_order(
                &module,
                "not-json",
                "MissingHakoReader.effect/2",
                backend,
            )
            .expect_err("unsupported backend must fail at preflight");
        let message = error.to_string();
        assert!(
            message.contains("backend_unsupported"),
            "{backend}: {message}"
        );
        assert!(message.contains(&format!("backend={backend}")));
        assert!(!message.contains(INPUT_TAG));
        assert!(!interpreter.strict_json_session_active());
    }
}

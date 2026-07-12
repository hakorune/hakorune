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

fn deep_binary(depth: usize) -> serde_json::Value {
    let mut value = serde_json::json!({"type":"Int","value":0});
    for _ in 0..depth {
        value = serde_json::json!({
            "type":"Binary","op":"+","lhs":value,"rhs":{"type":"Int","value":0}
        });
    }
    value
}

#[test]
fn hako_direct_negative_corpus_enforces_depth_and_text_boundaries() {
    std::thread::Builder::new()
        .name("hako-direct-negative-limits".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let module = compile_fixture();
            let mut interpreter = MirInterpreter::new();
            let deep = serde_json::json!({
                "version":0,"kind":"Program","body":[{"type":"Expr","expr":deep_binary(64)}]
            })
            .to_string();
            let depth_outcome = run(
                &mut interpreter,
                &module,
                &deep,
                "SnapshotDirectReaderFixtureV0Box.outcome/2",
            );
            let VMValue::String(depth_outcome) = depth_outcome else {
                panic!("depth outcome must be text")
            };
            assert!(depth_outcome.ends_with("|limit.max_depth"));

            for (bytes, ready) in [(1023, true), (1024, true), (1025, false)] {
                let input = serde_json::json!({
                    "version":0,"kind":"Program","body":[
                        {"type":"Expr","expr":{"type":"Var","name":"x".repeat(bytes)}}
                    ]
                })
                .to_string();
                let outcome = run(
                    &mut interpreter,
                    &module,
                    &input,
                    "SnapshotDirectReaderFixtureV0Box.outcome/2",
                );
                assert_eq!(
                    outcome == VMValue::String("Ready||".to_string()),
                    ready,
                    "atom bytes={bytes}"
                );
            }
            for (bytes, ready) in [(65535, true), (65536, true), (65537, false)] {
                let input = serde_json::json!({
                    "version":0,"kind":"Program","body":[
                        {"type":"Expr","expr":{"type":"Str","value":"x".repeat(bytes)}}
                    ]
                })
                .to_string();
                let outcome = run(
                    &mut interpreter,
                    &module,
                    &input,
                    "SnapshotDirectReaderFixtureV0Box.outcome/2",
                );
                assert_eq!(
                    outcome == VMValue::String("Ready||".to_string()),
                    ready,
                    "literal bytes={bytes}"
                );
            }
        })
        .expect("spawn negative limit thread")
        .join()
        .expect("negative limit thread");
}

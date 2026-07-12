use super::*;

const SUMMARY_ENTRY: &str = "SnapshotDirectReaderFixtureV0Box.loop_feature_summary/2";

fn summary_ready_impl() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for (input, expected) in [
        (
            r#"{"version":0,"kind":"Program","body":[]}"#,
            "break=0;continue=0;return=0;unwind=0;nested_loop=0;exit_map=;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Break"}],"else":[{"type":"Continue"}]},{"type":"Return","expr":{"type":"Int","value":0}}]}"#,
            "break=1;continue=1;return=1;unwind=0;nested_loop=0;exit_map=Return,Break,Continue;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"Loop","cond":{"type":"Bool","value":true},"body":[{"type":"Break"}]},{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"LoopRange","var_name":"i","start":{"type":"Int","value":0},"end":{"type":"Int","value":1},"body":[{"type":"Return","expr":{"type":"Int","value":0}}]}]}]}"#,
            "break=0;continue=0;return=0;unwind=0;nested_loop=1;exit_map=;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"Break"},{"type":"Continue"}]}"#,
            "break=1;continue=1;return=0;unwind=0;nested_loop=0;exit_map=Break,Continue;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"If","cond":{"type":"Bool","value":true},"then":[{"type":"Return","expr":{"type":"Null"}}]}]}]}"#,
            "break=0;continue=0;return=1;unwind=0;nested_loop=0;exit_map=Return;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"Break"},{"type":"Loop","cond":{"type":"Bool","value":true},"body":[{"type":"Continue"}]}]}"#,
            "break=1;continue=0;return=0;unwind=0;nested_loop=1;exit_map=Break;value_join=none;cleanup=none",
        ),
        (
            r#"{"version":0,"kind":"Program","body":[{"type":"Expr","expr":{"type":"Call","name":"Break","args":[{"type":"Str","value":"Return Continue Loop"}]}}]}"#,
            "break=0;continue=0;return=0;unwind=0;nested_loop=0;exit_map=;value_join=none;cleanup=none",
        ),
    ] {
        assert_eq!(
            run(&mut interpreter, &module, input, SUMMARY_ENTRY),
            VMValue::String(expected.to_string()),
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_loop_feature_summary_reads_only_the_sealed_snapshot() {
    std::thread::Builder::new()
        .name("hako-loop-feature-summary-ready".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(summary_ready_impl)
        .expect("spawn loop feature summary thread")
        .join()
        .expect("loop feature summary thread");
}

fn summary_failure_impl() {
    let module = compile_fixture();
    let mut interpreter = MirInterpreter::new();
    for input in [
        r#"{"version":0,"kind":"Program","body":[{"type":"Throw"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Future"}]}"#,
        r#"{"version":0,"kind":"Program","body":[{"type":"Return","expr":null}]}"#,
    ] {
        assert_eq!(
            run(&mut interpreter, &module, input, SUMMARY_ENTRY),
            VMValue::String(rust_outcome(input)),
        );
        assert!(!interpreter.strict_json_session_active());
    }
}

#[test]
fn hako_loop_feature_summary_preserves_non_ready_outcomes() {
    std::thread::Builder::new()
        .name("hako-loop-feature-summary-failure".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(summary_failure_impl)
        .expect("spawn loop feature outcome thread")
        .join()
        .expect("loop feature outcome thread");
}

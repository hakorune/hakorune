//! Production-shaped P0a evidence for the canonical-core reference report.
//!
//! This module calls the report owner directly. It intentionally has no CLI
//! selector or process-terminal consumer.

use super::*;
use crate::cli::CliConfig;
use crate::runner::reference::terminal::ReferenceRunOutcomeV1;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write source fixture");
    path
}

fn outcome(path: &Path) -> ReferenceRunOutcomeV1 {
    let mut config = CliConfig::default();
    config.backend = NormalFileCanonicalCoreVmReferenceProductionRequestV1::backend_name()
        .to_owned();
    config.file = Some(path.to_string_lossy().into_owned());
    let request = NormalFileCanonicalCoreVmReferenceProductionRequestV1::try_from_selected_cli(&config)
        .expect("canonical-core request");
    run(request)
}

fn program_snapshot(outcome: ReferenceRunOutcomeV1) -> (u8, Option<&'static str>) {
    let ReferenceRunOutcomeV1::Program(report) = outcome else {
        panic!("fixture must reach the program terminal");
    };
    (report.status_code(), report.diagnostic_tag())
}

#[test]
fn canonical_core_report_preserves_script_result_and_fault_projection() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        ("empty.hako", "", (0, None)),
        ("void.hako", "void", (0, None)),
        ("null.hako", "null", (0, None)),
        ("integer.hako", "42", (42, None)),
        ("integer-max.hako", "255", (255, None)),
        ("range.hako", "256", (70, Some("[process/exit-code-out-of-range]"))),
        ("bool.hako", "true", (70, Some("[process/unsupported-result]"))),
        ("float.hako", "1.5", (70, Some("[process/unsupported-result]"))),
        ("string.hako", "\"nyan\"", (70, Some("[process/unsupported-result]"))),
        ("print.hako", "print(1)", (0, None)),
        ("local.hako", "local x = 3", (0, None)),
        ("assignment.hako", "local x = 1\nx = 3", (0, None)),
        ("compound.hako", "local x = 1\nx += 2", (0, None)),
        ("division.hako", "1 / 0", (70, Some("[process/source-fault]"))),
    ];

    for (name, source, expected) in cases {
        assert_eq!(
            program_snapshot(outcome(&write_source(dir.path(), name, source))),
            expected,
            "{name}"
        );
    }
}

#[test]
fn canonical_core_report_reaches_main_and_admitted_callable_slice() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        ("main.hako", "static box Main { main() {} }"),
        (
            "callable.hako",
            "static function helper(x: i64): i64 { return x }\nstatic box Main { main() {} }",
        ),
    ];
    for (name, source) in cases {
        assert_eq!(
            program_snapshot(outcome(&write_source(dir.path(), name, source))),
            (0, None),
            "{name}"
        );
    }
}

#[test]
fn canonical_core_report_keeps_pre_execution_rejections_out_of_program_results() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        ("parse.hako", "@", "source-rejected"),
        ("using.hako", "using foo", "source-rejected"),
        (
            "direct-call.hako",
            "static function helper(x: i64): i64 { return x }\nstatic box Main { main() { helper(42) } }",
            "canonical-core-dispatch-rejected",
        ),
    ];
    for (name, source, tag) in cases {
        let ReferenceRunOutcomeV1::Invocation(report) = outcome(&write_source(dir.path(), name, source)) else {
            panic!("{name} must reject before program execution");
        };
        assert!(report.line().contains(tag), "{name}: {}", report.line());
    }
}

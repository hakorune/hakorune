//! Caller-zero production-shaped evidence for NORMAL-FILE-VM0 PARITY0-P0a.
//!
//! This module invokes the normal run owner directly. It does not select a
//! CLI route or call a process terminal.

use super::*;
use crate::cli::CliConfig;
use crate::runner::reference::raw_vm_reference;
use crate::runner::reference::raw_vm_reference_request::RawVmReferenceProductionRequestV1;
use crate::runner::reference::terminal::ReferenceRunOutcomeV1;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn write_source(dir: &Path, name: &str, source: &str) -> PathBuf {
    let path = dir.join(name);
    std::fs::write(&path, source).expect("write source fixture");
    path
}

fn normal_outcome(path: &Path) -> ReferenceRunOutcomeV1 {
    let mut config = CliConfig::default();
    config.backend = NormalFileVmReferenceProductionRequestV1::backend_name().to_owned();
    config.file = Some(path.to_string_lossy().into_owned());
    let request = NormalFileVmReferenceProductionRequestV1::try_from_selected_cli(&config)
        .expect("normal profile should seal");
    run(request)
}

fn raw_outcome(path: &Path) -> ReferenceRunOutcomeV1 {
    let mut config = CliConfig::default();
    config.backend = "raw-vm-reference".to_owned();
    config.file = Some(path.to_string_lossy().into_owned());
    let request = RawVmReferenceProductionRequestV1::try_from_selected_cli(&config)
        .expect("raw profile should seal");
    raw_vm_reference::run(request)
}

fn program_snapshot(outcome: ReferenceRunOutcomeV1) -> (u8, Option<&'static str>) {
    let ReferenceRunOutcomeV1::Program(report) = outcome else {
        panic!("common source must reach the program terminal");
    };
    (report.status_code(), report.diagnostic_tag())
}

#[test]
fn normal_program_projection_matches_raw_in_the_common_scalar_unit_subset() {
    let dir = tempdir().expect("tempdir");
    let cases = [
        ("empty.hako", ""),
        ("integer.hako", "42"),
        ("integer-max.hako", "255"),
        ("out-of-range.hako", "256"),
        ("bool.hako", "true"),
        ("float.hako", "1.5"),
        ("string.hako", "\"normal\""),
        ("print.hako", "print(1)"),
        ("local.hako", "local x = 3"),
        ("assignment.hako", "local x = 1\nx = 3"),
        ("compound.hako", "local x = 1\nx += 2"),
        ("main-fallthrough.hako", "static box Main { main() { 1 } }"),
        (
            "helper-main.hako",
            "static box Main { helper() {} main() {} }",
        ),
    ];

    for (name, source) in cases {
        let path = write_source(dir.path(), name, source);
        assert_eq!(
            program_snapshot(normal_outcome(&path)),
            program_snapshot(raw_outcome(&path)),
            "{name}"
        );
    }
}

#[test]
fn normal_run_preserves_usage_invocation_and_program_boundaries_without_retry() {
    let dir = tempdir().expect("tempdir");

    let missing = normal_outcome(&dir.path().join("missing.hako"));
    let ReferenceRunOutcomeV1::Invocation(report) = missing else {
        panic!("file read failure must stay an invocation failure");
    };
    assert!(report.line().contains("file-not-found"));

    for (name, source, expected_code) in [
        ("parse.hako", "@", "parse"),
        ("using.hako", "using foo", "using-not-supported"),
        (
            "main-return.hako",
            "static box Main { main() { return 1 } }",
            "raw-compile-rejected",
        ),
    ] {
        let outcome = normal_outcome(&write_source(dir.path(), name, source));
        let ReferenceRunOutcomeV1::Invocation(report) = outcome else {
            panic!("{name} must not execute as a program result");
        };
        assert!(
            report.line().contains(expected_code),
            "{name}: {}",
            report.line()
        );
    }

    let succeeding = normal_outcome(&write_source(dir.path(), "after-rejection.hako", "1"));
    assert_eq!(program_snapshot(succeeding), (1, None));
}

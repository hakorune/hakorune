use std::path::Path;

use rust_source_topology_check::{
    scan_scope_manifest, ChronicMetricV1, ChronicObservationV1, ChronicScanErrorV1,
};

const FIXTURE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(FIXTURE_ROOT).join("tests/fixtures").join(name)
}

#[test]
fn scanner_is_deterministic_and_ignores_comments_and_raw_strings() {
    let manifest = fixture("chronic_sample_scope.toml");
    let first = scan_scope_manifest(&manifest, Path::new(FIXTURE_ROOT)).unwrap();
    let second = scan_scope_manifest(&manifest, Path::new(FIXTURE_ROOT)).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.files.len(), 1);
    assert_eq!(first.summary.panic_count, 1);
    assert_eq!(first.summary.todo_count, 1);
    assert_eq!(first.summary.unwrap_count, 1);
    assert_eq!(first.summary.expect_count, 1);
    assert_eq!(first.summary.dead_code_allowance_count, 3);
    assert_eq!(first.summary.unclassified_count, 0);
    let observations = &first.files[0].observations;
    assert!(observations.iter().any(|observation| matches!(
        observation,
        ChronicObservationV1::CallSite {
            metric: ChronicMetricV1::Panic,
            ..
        }
    )));
    assert!(observations
        .iter()
        .any(|observation| matches!(observation, ChronicObservationV1::ModuleEdge { .. })));
    assert!(observations.iter().any(|observation| matches!(
        observation,
        ChronicObservationV1::OpaqueMacro { syntax_name, .. } if syntax_name == "include"
    )));
    assert!(!first.evidence_hash.is_empty());
}

#[test]
fn malformed_source_and_path_escape_fail_closed() {
    let broken = scan_scope_manifest(
        &fixture("chronic_broken_scope.toml"),
        Path::new(FIXTURE_ROOT),
    )
    .unwrap_err();
    assert!(matches!(broken, ChronicScanErrorV1::ParseFailed { .. }));

    let escaped = scan_scope_manifest(
        &fixture("chronic_escape_scope.toml"),
        Path::new(FIXTURE_ROOT),
    )
    .unwrap_err();
    assert!(matches!(escaped, ChronicScanErrorV1::PathEscape { .. }));
}

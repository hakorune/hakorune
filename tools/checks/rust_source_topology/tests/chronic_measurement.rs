use std::path::Path;

use rust_source_topology_check::{
    observation_receipt_json, scan_scope_manifest, ChronicMetricV1, ChronicObservationV1,
    ChronicScanErrorV1,
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

#[test]
fn observation_receipt_is_deterministic_and_scope_bound() {
    let workspace = Path::new(FIXTURE_ROOT).join("../../..");
    let manifest = workspace.join("tools/checks/manifests/chronic_measurement_scope_v1.toml");
    let source_commit = "d9cff5b744edee3b6450db5d0ffc74478f32b49a";
    let first = observation_receipt_json(&manifest, &workspace, source_commit).unwrap();
    let second = observation_receipt_json(&manifest, &workspace, source_commit).unwrap();
    assert_eq!(first, second);
    let receipt: serde_json::Value = serde_json::from_str(&first).unwrap();
    assert_eq!(receipt["schema"], "chronic-measurement-observations-v1");
    assert_eq!(receipt["rows"].as_array().unwrap().len(), 185);
    assert!(receipt["receipt_hash"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    for row in receipt["rows"].as_array().unwrap() {
        assert!(row.get("owner_ref").is_none());
        assert!(row.get("retirement_status").is_none());
        assert!(row.get("raw_condition").is_some());
    }
}

#[test]
fn observation_receipt_rejects_missing_source_revision_before_scan() {
    let workspace = Path::new(FIXTURE_ROOT).join("../../..");
    let manifest = workspace.join("tools/checks/manifests/chronic_measurement_scope_v1.toml");
    let error = observation_receipt_json(&manifest, &workspace, "missing").unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::InvalidSourceCommit { .. }
    ));
}

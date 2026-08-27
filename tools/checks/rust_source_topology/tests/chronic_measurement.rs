use std::path::Path;

use rust_source_topology_check::{
    observation_receipt_json, scan_scope_manifest, validate_observation_receipt_json,
    ChronicMetricV1, ChronicObservationV1, ChronicScanErrorV1,
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

#[test]
fn observation_receipt_consumer_verifies_tracked_contract_without_rescanning() {
    let workspace = Path::new(FIXTURE_ROOT).join("../../..");
    let receipt_path =
        workspace.join("tools/checks/manifests/chronic_measurement_observations_v1.json");
    let receipt_text = std::fs::read_to_string(receipt_path).unwrap();
    let source_commit = "d9cff5b744edee3b6450db5d0ffc74478f32b49a";

    let receipt = validate_observation_receipt_json(&receipt_text, source_commit).unwrap();
    assert_eq!(receipt.rows.len(), 185);
    assert_eq!(receipt.source_commit, source_commit);
    assert_eq!(
        receipt.receipt_hash,
        "sha256:6be32f799970e883aa37f19a562f188016a06624f493f28bf57b446d27b5c63d"
    );
}

#[test]
fn observation_receipt_consumer_rejects_unknown_fields_and_drift() {
    let workspace = Path::new(FIXTURE_ROOT).join("../../..");
    let receipt_path =
        workspace.join("tools/checks/manifests/chronic_measurement_observations_v1.json");
    let receipt_text = std::fs::read_to_string(receipt_path).unwrap();
    let source_commit = "d9cff5b744edee3b6450db5d0ffc74478f32b49a";

    let mut unknown_top_level: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    unknown_top_level
        .as_object_mut()
        .unwrap()
        .insert("owner_ref".into(), serde_json::Value::String("none".into()));
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&unknown_top_level).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptInvalid { .. }
    ));

    let mut unknown_row: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    unknown_row["rows"][0].as_object_mut().unwrap().insert(
        "retirement_status".into(),
        serde_json::Value::String("none".into()),
    );
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&unknown_row).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptInvalid { .. }
    ));

    let mut scope_drift: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    scope_drift["scope_id"] = serde_json::Value::String("wrong-scope".into());
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&scope_drift).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptInvalid { .. }
    ));

    let mut source_drift: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    source_drift["source_commit"] = serde_json::Value::String("a".repeat(40));
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&source_drift).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptInvalid { .. }
    ));

    let mut count_drift: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    count_drift["rows"].as_array_mut().unwrap().pop();
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&count_drift).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptCountDrift { .. }
    ));

    let mut hash_drift: serde_json::Value = serde_json::from_str(&receipt_text).unwrap();
    hash_drift["receipt_hash"] = serde_json::Value::String("sha256:".to_string() + &"0".repeat(64));
    let error = validate_observation_receipt_json(
        &serde_json::to_string(&hash_drift).unwrap(),
        source_commit,
    )
    .unwrap_err();
    assert!(matches!(
        error,
        ChronicScanErrorV1::ObservationReceiptHashDrift { .. }
    ));
}

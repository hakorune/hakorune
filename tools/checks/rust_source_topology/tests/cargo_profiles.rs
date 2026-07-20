use std::path::{Path, PathBuf};

use rust_source_topology_check::project::cargo::{
    collect_declared_cargo_unit_process_evidence_v1, CargoDeclaredUnitProcessEvidenceV1,
};
use rust_source_topology_check::project::{
    parse_and_verify_profile_schema_v1, verify_profile_requests_v1, AmbientRustflagsPolicyV1,
    BuildProfileRequestV1, CargoCompileModeV1, CargoProfileNameV1, CargoTargetKindV1,
    ValidatedBuildProfileInputV1,
};

const ROOT_PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn dependency_free_workspace_closes_target_and_feature_matrix() {
    let profiles = synthetic_profiles();
    let mut evidence = Vec::new();
    for profile in &profiles {
        evidence.push(
            collect_declared_cargo_unit_process_evidence_v1(&synthetic_manifest(), profile)
                .unwrap(),
        );
    }

    assert_features(&evidence, "synthetic-default", &["base", "default"]);
    assert_features(&evidence, "synthetic-no-default", &[]);
    assert_features(
        &evidence,
        "synthetic-llvm-harness",
        &["base", "default", "llvm-harness"],
    );
    assert!(!unit(&evidence, "synthetic-llvm-harness")
        .declared_unit()
        .cargo_resolved_root_features()
        .contains(&"llvm".to_string()));
    assert_features(&evidence, "synthetic-tool", &["base", "default", "tool"]);
    assert_eq!(
        unit(&evidence, "synthetic-tool")
            .declared_unit()
            .target()
            .semantic_kind(),
        CargoTargetKindV1::Binary
    );
    assert_eq!(
        unit(&evidence, "synthetic-integration")
            .declared_unit()
            .target()
            .semantic_kind(),
        CargoTargetKindV1::IntegrationTest
    );
    assert!(unit(&evidence, "synthetic-integration")
        .rustc_cfg_probe()
        .cfg_flags()
        .contains(&"test".to_string()));
    assert_eq!(
        unit(&evidence, "synthetic-wasm")
            .rustc_cfg_probe()
            .cfg_values("target_arch")
            .unwrap(),
        ["wasm32"]
    );
}

#[test]
fn root_six_profiles_are_exact_deterministic_and_path_clean() {
    let schema = parse_and_verify_profile_schema_v1(ROOT_PROFILES).unwrap();
    let first = collect_all(&root_manifest(), &schema.profiles);
    let second = collect_all(&root_manifest(), &schema.profiles);
    assert_eq!(first.len(), 6);
    assert_eq!(
        first
            .iter()
            .map(|row| row.declared_unit().profile_id())
            .collect::<Vec<_>>(),
        vec![
            "host-default-dev",
            "host-default-release",
            "host-llvm-harness-dev",
            "host-test-unit-default",
            "host-vm-reference-dev",
            "wasm32-default-dev",
        ]
    );
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );

    let serialized = String::from_utf8(serde_json::to_vec(&first).unwrap()).unwrap();
    assert!(!serialized.contains(root_dir().to_string_lossy().as_ref()));
    for row in &first {
        assert_eq!(
            row.declared_unit().profile_expected_root_features(),
            row.declared_unit().cargo_resolved_root_features(),
        );
    }
    assert!(!unit(&first, "host-default-release")
        .rustc_cfg_probe()
        .cfg_flags()
        .contains(&"debug_assertions".to_string()));
    assert!(unit(&first, "host-test-unit-default")
        .rustc_cfg_probe()
        .cfg_flags()
        .contains(&"test".to_string()));
    assert!(!unit(&first, "wasm32-default-dev")
        .declared_unit()
        .cargo_resolved_root_features()
        .contains(&"wasm-backend".to_string()));
}

fn collect_all(
    manifest: &Path,
    profiles: &[ValidatedBuildProfileInputV1],
) -> Vec<CargoDeclaredUnitProcessEvidenceV1> {
    profiles
        .iter()
        .map(|profile| collect_declared_cargo_unit_process_evidence_v1(manifest, profile).unwrap())
        .collect()
}

fn assert_features(
    rows: &[CargoDeclaredUnitProcessEvidenceV1],
    profile_id: &str,
    expected: &[&str],
) {
    assert_eq!(
        unit(rows, profile_id)
            .declared_unit()
            .cargo_resolved_root_features(),
        expected,
    );
}

fn unit<'a>(
    rows: &'a [CargoDeclaredUnitProcessEvidenceV1],
    profile_id: &str,
) -> &'a CargoDeclaredUnitProcessEvidenceV1 {
    rows.iter()
        .find(|row| row.declared_unit().profile_id() == profile_id)
        .unwrap()
}

fn synthetic_profiles() -> Box<[ValidatedBuildProfileInputV1]> {
    let host = "x86_64-unknown-linux-gnu";
    verify_profile_requests_v1(vec![
        request(
            "synthetic-default",
            "fixture_core",
            CargoTargetKindV1::Library,
            host,
        ),
        request(
            "synthetic-no-default",
            "fixture_core",
            CargoTargetKindV1::Library,
            host,
        ),
        request(
            "synthetic-llvm-harness",
            "fixture_core",
            CargoTargetKindV1::Library,
            host,
        ),
        request("synthetic-tool", "tool", CargoTargetKindV1::Binary, host),
        request(
            "synthetic-integration",
            "integration",
            CargoTargetKindV1::IntegrationTest,
            host,
        ),
        request(
            "synthetic-wasm",
            "fixture_core",
            CargoTargetKindV1::Library,
            "wasm32-unknown-unknown",
        ),
    ])
    .unwrap()
    .profiles
}

fn request(
    profile_id: &str,
    target_name: &str,
    target_kind: CargoTargetKindV1,
    target_triple: &str,
) -> BuildProfileRequestV1 {
    let is_no_default = profile_id == "synthetic-no-default";
    let requested_features = match profile_id {
        "synthetic-llvm-harness" => vec!["llvm-harness".to_string()],
        "synthetic-tool" => vec!["tool".to_string()],
        _ => Vec::new(),
    };
    let mut expected = if is_no_default {
        Vec::new()
    } else {
        vec!["base".to_string(), "default".to_string()]
    };
    expected.extend(requested_features.iter().cloned());
    let is_test = target_kind == CargoTargetKindV1::IntegrationTest;
    BuildProfileRequestV1 {
        profile_id: profile_id.to_string(),
        package_name: "fixture-app".to_string(),
        target_name: target_name.to_string(),
        target_kind,
        target_triple: target_triple.to_string(),
        cargo_profile: if is_test {
            CargoProfileNameV1::Test
        } else {
            CargoProfileNameV1::Dev
        },
        compile_mode: if is_test {
            CargoCompileModeV1::IntegrationTestTarget
        } else {
            CargoCompileModeV1::Normal
        },
        requested_features,
        expected_activated_root_features: expected,
        default_features_enabled: !is_no_default,
        test_cfg: is_test,
        debug_assertions: true,
        panic_strategy: if target_triple.starts_with("wasm32") {
            "abort".to_string()
        } else {
            "unwind".to_string()
        },
        ambient_rustflags: AmbientRustflagsPolicyV1::SanitizedEmpty,
    }
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn root_manifest() -> PathBuf {
    root_dir().join("Cargo.toml")
}

fn synthetic_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/cargo0_workspace/Cargo.toml")
}

use rust_source_topology_check::project::cargo::{
    seal_declared_cargo_unit_v1, CargoEvidenceErrorV1, CargoMetadataSnapshotV1,
    CargoPackageSnapshotV1, CargoResolveNodeSnapshotV1, CargoTargetSnapshotV1,
};
use rust_source_topology_check::project::{
    parse_and_verify_profile_schema_v1, CargoCompileModeV1, CargoProfileNameV1, CargoTargetKindV1,
    ValidatedBuildProfileInputV1,
};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn exact_manifest_target_and_cargo_feature_closure_seal() {
    let profile = profile("host-default-dev");
    let evidence = seal_declared_cargo_unit_v1(
        &snapshot(&["cli", "default", "plugins"]),
        manifest(),
        &profile,
    )
    .unwrap();

    assert_eq!(evidence.profile_id(), "host-default-dev");
    assert_eq!(
        evidence.package().package_key(),
        "Cargo.toml::nyash-rust@0.1.0"
    );
    assert_eq!(
        evidence.package().manifest_path_workspace_relative(),
        "Cargo.toml"
    );
    assert_eq!(
        evidence.target().target_key(),
        "Cargo.toml::nyash-rust@0.1.0::library:nyash_rust"
    );
    assert_eq!(
        evidence.target().src_path_workspace_relative(),
        "src/lib.rs"
    );
    assert_eq!(evidence.target().cargo_kinds(), ["rlib"]);
    assert_eq!(
        evidence.cargo_resolved_root_features(),
        ["cli", "default", "plugins"]
    );
}

#[test]
fn opaque_package_id_is_only_a_join_key() {
    let profile = profile("host-default-dev");
    let evidence = seal_declared_cargo_unit_v1(
        &snapshot(&["cli", "default", "plugins"]),
        manifest(),
        &profile,
    )
    .unwrap();
    let serialized = serde_json::to_string(&evidence).unwrap();
    assert!(!serialized.contains("path+file:///workspace"));
    assert!(!serialized.contains("/workspace/hakorune"));
}

#[test]
fn package_and_target_are_never_selected_by_approximation() {
    let profile = profile("host-default-dev");
    let source = snapshot(&["cli", "default", "plugins"]);
    assert!(matches!(
        seal_declared_cargo_unit_v1(&source, "/workspace/hakorune/missing/Cargo.toml", &profile),
        Err(CargoEvidenceErrorV1::PackageForManifestMissing)
    ));

    let mut wrong_package = profile.clone();
    wrong_package.package_name = "different".to_string();
    assert!(matches!(
        seal_declared_cargo_unit_v1(&source, manifest(), &wrong_package),
        Err(CargoEvidenceErrorV1::PackageNameMismatch { .. })
    ));

    let mut wrong_kind = profile;
    wrong_kind.target_kind = CargoTargetKindV1::Binary;
    assert!(matches!(
        seal_declared_cargo_unit_v1(&source, manifest(), &wrong_kind),
        Err(CargoEvidenceErrorV1::TargetKindMismatch { .. })
    ));
}

#[test]
fn default_and_expected_features_are_exact_not_projected() {
    let profile = profile("host-default-dev");
    assert!(matches!(
        seal_declared_cargo_unit_v1(&snapshot(&["cli", "plugins"]), manifest(), &profile),
        Err(CargoEvidenceErrorV1::DefaultFeatureDispositionMismatch)
    ));

    let mut stale_expectation = profile;
    stale_expectation.expected_activated_root_features =
        vec!["cli".to_string(), "plugins".to_string()].into_boxed_slice();
    assert!(matches!(
        seal_declared_cargo_unit_v1(
            &snapshot(&["cli", "default", "plugins"]),
            manifest(),
            &stale_expectation,
        ),
        Err(CargoEvidenceErrorV1::ActivatedFeatureMismatch { .. })
    ));
}

#[test]
fn requested_and_required_features_must_be_cargo_active() {
    let mut requested = profile("host-vm-reference-dev");
    requested.expected_activated_root_features = vec!["cli", "default", "plugins", "vm-reference"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    assert!(matches!(
        seal_declared_cargo_unit_v1(
            &snapshot(&["cli", "default", "plugins"]),
            manifest(),
            &requested,
        ),
        Err(CargoEvidenceErrorV1::RequestedFeatureInactive { .. })
    ));

    let mut tool = profile("host-default-dev");
    tool.target_name = "tool".to_string();
    tool.target_kind = CargoTargetKindV1::Binary;
    assert!(matches!(
        seal_declared_cargo_unit_v1(&snapshot(&["cli", "default", "plugins"]), manifest(), &tool,),
        Err(CargoEvidenceErrorV1::RequiredFeatureInactive { .. })
    ));
}

#[test]
fn no_default_alias_direction_and_required_feature_success_are_explicit() {
    let mut no_default = profile("host-default-dev");
    no_default.default_features_enabled = false;
    no_default.expected_activated_root_features = Box::new([]);
    let evidence = seal_declared_cargo_unit_v1(&snapshot(&[]), manifest(), &no_default).unwrap();
    assert!(evidence.cargo_resolved_root_features().is_empty());

    let llvm_harness = profile("host-llvm-harness-dev");
    let evidence = seal_declared_cargo_unit_v1(
        &snapshot(&["cli", "default", "llvm-harness", "plugins"]),
        manifest(),
        &llvm_harness,
    )
    .unwrap();
    assert!(!evidence
        .cargo_resolved_root_features()
        .contains(&"llvm".to_string()));

    let mut tool = profile("host-default-dev");
    tool.target_name = "tool".to_string();
    tool.target_kind = CargoTargetKindV1::Binary;
    tool.expected_activated_root_features = vec!["cli", "default", "plugins", "tool"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let evidence = seal_declared_cargo_unit_v1(
        &snapshot(&["cli", "default", "plugins", "tool"]),
        manifest(),
        &tool,
    )
    .unwrap();
    assert_eq!(evidence.target().required_features(), ["tool"]);

    let mut unknown = profile("host-default-dev");
    unknown.requested_features = vec!["missing".to_string()].into_boxed_slice();
    assert!(matches!(
        seal_declared_cargo_unit_v1(
            &snapshot(&["cli", "default", "plugins"]),
            manifest(),
            &unknown,
        ),
        Err(CargoEvidenceErrorV1::RequestedFeatureUnknown { .. })
    ));
}

#[test]
fn unit_test_harness_is_a_mode_over_the_library_target() {
    let profile = profile("host-test-unit-default");
    let evidence = seal_declared_cargo_unit_v1(
        &snapshot(&["cli", "default", "plugins"]),
        manifest(),
        &profile,
    )
    .unwrap();
    assert_eq!(
        evidence.requested_compile_mode(),
        CargoCompileModeV1::UnitTestHarness
    );
    assert!(evidence.requested_test_cfg());
    assert_eq!(
        evidence.target().semantic_kind(),
        CargoTargetKindV1::Library
    );

    let mut invalid = profile;
    invalid.cargo_profile = CargoProfileNameV1::Dev;
    assert!(matches!(
        seal_declared_cargo_unit_v1(
            &snapshot(&["cli", "default", "plugins"]),
            manifest(),
            &invalid,
        ),
        Err(CargoEvidenceErrorV1::CompileModeTargetMismatch { .. })
    ));
}

fn profile(id: &str) -> ValidatedBuildProfileInputV1 {
    parse_and_verify_profile_schema_v1(PROFILES)
        .unwrap()
        .profiles
        .iter()
        .find(|profile| profile.profile_id == id)
        .unwrap()
        .clone()
}

fn manifest() -> &'static str {
    "/workspace/hakorune/Cargo.toml"
}

fn snapshot(features: &[&str]) -> CargoMetadataSnapshotV1 {
    let package_id = "path+file:///workspace/hakorune#nyash-rust@0.1.0";
    CargoMetadataSnapshotV1 {
        workspace_root: "/workspace/hakorune".to_string(),
        workspace_member_package_ids: vec![package_id.to_string()].into_boxed_slice(),
        packages: vec![CargoPackageSnapshotV1 {
            cargo_package_id_observation: package_id.to_string(),
            name: "nyash-rust".to_string(),
            version: "0.1.0".to_string(),
            manifest_path: manifest().to_string(),
            source_observation: None,
            declared_features: vec![
                "cli".to_string(),
                "default".to_string(),
                "llvm".to_string(),
                "llvm-harness".to_string(),
                "plugins".to_string(),
                "tool".to_string(),
                "vm-reference".to_string(),
            ]
            .into_boxed_slice(),
            targets: vec![
                CargoTargetSnapshotV1 {
                    name: "nyash_rust".to_string(),
                    cargo_kinds: vec!["rlib".to_string()].into_boxed_slice(),
                    crate_types: vec!["rlib".to_string()].into_boxed_slice(),
                    src_path: "/workspace/hakorune/src/lib.rs".to_string(),
                    edition: "2021".to_string(),
                    required_features: Box::new([]),
                    test: true,
                    doctest: false,
                },
                CargoTargetSnapshotV1 {
                    name: "tool".to_string(),
                    cargo_kinds: vec!["bin".to_string()].into_boxed_slice(),
                    crate_types: vec!["bin".to_string()].into_boxed_slice(),
                    src_path: "/workspace/hakorune/src/bin/tool.rs".to_string(),
                    edition: "2021".to_string(),
                    required_features: vec!["tool".to_string()].into_boxed_slice(),
                    test: true,
                    doctest: false,
                },
            ]
            .into_boxed_slice(),
        }]
        .into_boxed_slice(),
        resolve_nodes: Some(
            vec![CargoResolveNodeSnapshotV1 {
                cargo_package_id_observation: package_id.to_string(),
                activated_features: features
                    .iter()
                    .map(|feature| (*feature).to_string())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            }]
            .into_boxed_slice(),
        ),
    }
}

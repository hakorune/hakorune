use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rust_source_topology_check::project::cargo::collect_declared_cargo_unit_process_evidence_v1;
use rust_source_topology_check::project::{
    collect_rustc_cfg_probe_v1, collect_workspace_input_fingerprints_v1,
    parse_and_verify_profile_schema_v1, CargoProcessEvidenceErrorV1, ValidatedBuildProfileInputV1,
};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn root_default_metadata_rustc_and_config_evidence_co_seal() {
    let profile = profile("host-default-dev");
    let first =
        collect_declared_cargo_unit_process_evidence_v1(&root_manifest(), &profile).unwrap();
    let second =
        collect_declared_cargo_unit_process_evidence_v1(&root_manifest(), &profile).unwrap();

    assert_eq!(
        first.declared_unit().cargo_resolved_root_features(),
        ["cli", "default", "plugins"]
    );
    assert!(first
        .rustc_cfg_probe()
        .cfg_flags()
        .contains(&"debug_assertions".to_string()));
    assert_eq!(
        first.rustc_cfg_probe().cfg_values("panic").unwrap(),
        ["unwind"]
    );
    assert_eq!(
        first.rustc_cfg_probe().cfg_values("target_arch").unwrap(),
        ["x86_64"]
    );
    assert_eq!(
        first
            .workspace_inputs()
            .repository_cargo_config()
            .unwrap()
            .workspace_relative_path(),
        ".cargo/config.toml"
    );
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    assert!(!String::from_utf8(serde_json::to_vec(&first).unwrap())
        .unwrap()
        .contains(root_dir().to_string_lossy().as_ref()));
}

#[test]
fn rustc_probe_seals_release_wasm_and_unit_test_cfg() {
    let release = profile("host-default-release");
    let release_probe =
        collect_rustc_cfg_probe_v1(&release, &release.expected_activated_root_features).unwrap();
    assert!(!release_probe
        .cfg_flags()
        .contains(&"debug_assertions".to_string()));
    assert_eq!(release_probe.cfg_values("panic").unwrap(), ["abort"]);

    let wasm = profile("wasm32-default-dev");
    let wasm_probe =
        collect_rustc_cfg_probe_v1(&wasm, &wasm.expected_activated_root_features).unwrap();
    assert_eq!(wasm_probe.cfg_values("target_arch").unwrap(), ["wasm32"]);
    assert_eq!(wasm_probe.cfg_values("panic").unwrap(), ["abort"]);

    let unit = profile("host-test-unit-default");
    let unit_probe =
        collect_rustc_cfg_probe_v1(&unit, &unit.expected_activated_root_features).unwrap();
    assert!(unit_probe.cfg_flags().contains(&"test".to_string()));
}

#[test]
fn repository_config_accepts_linker_only_and_rejects_cfg_injection() {
    let workspace = TemporaryWorkspace::new();
    workspace.write(
        ".cargo/config.toml",
        "[target.x86_64-unknown-linux-gnu]\nrustflags = [\"-C\", \"link-arg=-fuse-ld=lld\"]\n",
    );
    collect_workspace_input_fingerprints_v1(&workspace.root, &workspace.manifest()).unwrap();

    workspace.write(
        ".cargo/config.toml",
        "[build]\nrustflags = [\"--cfg\", \"custom_build\"]\n",
    );
    assert!(matches!(
        collect_workspace_input_fingerprints_v1(&workspace.root, &workspace.manifest()),
        Err(CargoProcessEvidenceErrorV1::UnsupportedRepositoryRustflags)
            | Err(CargoProcessEvidenceErrorV1::CfgAffectingRepositoryRustflags)
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

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn root_manifest() -> PathBuf {
    root_dir().join("Cargo.toml")
}

struct TemporaryWorkspace {
    root: PathBuf,
}

impl TemporaryWorkspace {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rust-source-topology-cargo0-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join(".cargo")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(root.join("Cargo.lock"), "# fixture lock\n").unwrap();
        Self { root }
    }

    fn manifest(&self) -> PathBuf {
        self.root.join("Cargo.toml")
    }

    fn write(&self, relative: &str, contents: &str) {
        fs::write(self.root.join(relative), contents).unwrap();
    }
}

impl Drop for TemporaryWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

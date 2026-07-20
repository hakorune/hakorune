use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use rust_source_topology_check::project::cargo::{
    collect_declared_cargo_unit_process_evidence_v1, CargoDeclaredUnitProcessEvidenceV1,
};
use rust_source_topology_check::project::{
    collect_declared_module_topology_v1, parse_and_verify_profile_schema_v1, CfgDecisionStateV1,
    ModuleEdgeKindV1, ModuleInstanceKindV1, ModuleTopologyErrorV1, ValidatedBuildProfileInputV1,
};

const FIXTURE_PROFILES: &str = include_str!("fixtures/module0_workspace/profiles_v1.json");

#[test]
fn custom_root_directory_path_and_inline_laws_are_exact() {
    let (root, evidence) = fixture_evidence("host-default-dev");
    let topology = collect_declared_module_topology_v1(&root, &evidence).unwrap();
    let paths = topology
        .module_instances
        .iter()
        .map(|instance| {
            (
                instance.module_syntax_path.as_str(),
                instance.kind,
                instance.source_path_workspace_relative.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(topology.module_instances.len(), 13);
    assert_eq!(topology.module_edges.len(), 17);
    assert_eq!(topology.include_edges.len(), 1);
    assert_eq!(topology.source_observations.len(), 12);
    assert!(paths.contains(&("crate", ModuleInstanceKindV1::Root, "roots/custom_root.rs")));
    assert!(paths.contains(&(
        "crate::flat",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/flat.rs"
    )));
    assert!(paths.contains(&(
        "crate::flat::leaf",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/flat/leaf.rs"
    )));
    assert!(paths.contains(&(
        "crate::tree",
        ModuleInstanceKindV1::OrdinaryModFile,
        "roots/tree/mod.rs"
    )));
    assert!(paths.contains(&(
        "crate::tree::leaf",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/tree/leaf.rs"
    )));
    assert!(paths.contains(&(
        "crate::inline",
        ModuleInstanceKindV1::Inline,
        "roots/custom_root.rs"
    )));
    assert!(paths.contains(&(
        "crate::inline::nested",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/inline/nested.rs"
    )));
    assert!(paths.contains(&(
        "crate::renamed",
        ModuleInstanceKindV1::LiteralPath,
        "roots/alternate.rs"
    )));
    assert!(paths.contains(&(
        "crate::renamed::path_child",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/path_child.rs"
    )));
    assert!(paths.contains(&(
        "crate::redirected_inline::inside",
        ModuleInstanceKindV1::OrdinaryFile,
        "roots/redirected/inside.rs"
    )));
    assert!(paths.contains(&(
        "crate::platform",
        ModuleInstanceKindV1::LiteralPath,
        "roots/platform_host.rs"
    )));
    assert!(!paths
        .iter()
        .any(|(_, _, path)| path.contains("custom_root/")));
    assert_eq!(
        topology
            .module_edges
            .iter()
            .find(|edge| edge.semantic_segment == "redirected_inline")
            .unwrap()
            .active_literal_path
            .as_deref(),
        Some("redirected")
    );
    assert_eq!(
        topology
            .source_observations
            .iter()
            .find(|row| row.source_path_workspace_relative == "roots/custom_root.rs")
            .unwrap()
            .topology
            .source_file
            .opaque_syntax_sites
            .iter()
            .filter(|site| site.syntax_name == "include")
            .count(),
        1
    );
    let include = &topology.include_edges[0];
    assert_eq!(include.owning_module_instance_id, "module:0");
    assert_eq!(
        include.selected_source_path_workspace_relative.as_deref(),
        Some("roots/must_remain_opaque.rs")
    );
    assert_eq!(
        topology
            .source_observations
            .iter()
            .find(|row| row.source_observation_id
                == include.child_source_observation_id.as_deref().unwrap())
            .unwrap()
            .module_instance_id,
        "module:0"
    );
}

#[test]
fn sealed_profiles_gate_debug_test_feature_and_target_paths() {
    let expected = [
        (
            "host-default-dev",
            "roots/platform_host.rs",
            "roots/debug.rs",
            None,
        ),
        (
            "host-default-release",
            "roots/platform_host.rs",
            "roots/release.rs",
            None,
        ),
        (
            "host-test-unit-default",
            "roots/platform_host.rs",
            "roots/debug.rs",
            Some("roots/test_only.rs"),
        ),
        (
            "host-vm-reference-dev",
            "roots/platform_host.rs",
            "roots/debug.rs",
            Some("roots/feature_vm.rs"),
        ),
        (
            "host-llvm-harness-dev",
            "roots/platform_host.rs",
            "roots/debug.rs",
            Some("roots/feature_llvm_harness.rs"),
        ),
        (
            "wasm32-default-dev",
            "roots/platform_wasm.rs",
            "roots/debug.rs",
            None,
        ),
    ];
    for (profile_id, platform, mode, extra) in expected {
        let (root, evidence) = fixture_evidence(profile_id);
        let topology = collect_declared_module_topology_v1(&root, &evidence).unwrap();
        let paths = topology
            .module_instances
            .iter()
            .map(|instance| instance.source_path_workspace_relative.as_str())
            .collect::<Vec<_>>();
        assert!(paths.contains(&platform), "profile={profile_id}");
        assert!(paths.contains(&mode), "profile={profile_id}");
        if let Some(extra) = extra {
            assert!(paths.contains(&extra), "profile={profile_id}");
        }
        assert!(!paths.contains(&"roots/must_stay_excluded.rs"));
    }
}

#[test]
fn topology_is_deterministic_atomic_and_workspace_relative() {
    let (root, evidence) = fixture_evidence("host-default-dev");
    let first = collect_declared_module_topology_v1(&root, &evidence).unwrap();
    let second = collect_declared_module_topology_v1(&root, &evidence).unwrap();
    let first_json = serde_json::to_vec(&first).unwrap();
    let second_json = serde_json::to_vec(&second).unwrap();
    assert_eq!(first_json, second_json);
    assert!(!String::from_utf8(first_json)
        .unwrap()
        .contains(root.to_string_lossy().as_ref()));
    assert_eq!(
        first.module_instances.len(),
        1 + first
            .module_edges
            .iter()
            .filter(|edge| edge.child_instance_id.is_some())
            .count()
    );
    assert!(first.module_edges.iter().all(|edge| {
        let outer_included = edge.cfg_decision.final_state == CfgDecisionStateV1::Included;
        match &edge.content_gate {
            None => !outer_included && edge.child_instance_id.is_none(),
            Some(gate) => {
                (gate.cfg_decision.final_state == CfgDecisionStateV1::Included)
                    == edge.child_instance_id.is_some()
            }
        }
    }));
}

#[test]
fn module_lookup_and_cfg_failures_are_typed_before_fallback() {
    let workspace = TemporaryModuleWorkspace::new("pub fn root() {}\n");
    let evidence = workspace.evidence();

    workspace.write("src/root.rs", "mod absent;\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::OrdinaryModuleMissing { .. })
    ));

    workspace.write("src/root.rs", "#[cfg(any())] mod excluded_absent;\n");
    let excluded = workspace.collect(&evidence).unwrap();
    assert_eq!(excluded.module_instances.len(), 1);
    assert_eq!(
        excluded.module_edges[0].cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );

    workspace.write(
        "src/root.rs",
        "#[cfg(custom_unknown)] mod unknown_absent;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::UnknownCfg { .. })
    ));

    workspace.write("src/root.rs", "mod dual;\n");
    workspace.write("src/dual.rs", "pub fn flat() {}\n");
    workspace.write("src/dual/mod.rs", "pub fn nested() {}\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::OrdinaryModuleAmbiguous { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[path = \"one.rs\"] #[path = \"two.rs\"] mod duplicate;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::MultipleActivePaths { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[path = concat!(\"one\", \".rs\")] mod generated;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::NonLiteralPath { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[cfg_attr(custom_unknown, path = \"unknown.rs\")] mod unknown_path;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::UnknownCfg { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[cfg(any())] #[cfg_attr(custom_unknown, path = \"unknown.rs\")] mod excluded_path;\n",
    );
    let excluded_unknown = workspace.collect(&evidence).unwrap();
    assert_eq!(
        excluded_unknown.module_edges[0].cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );

    workspace.write(
        "src/root.rs",
        "#[cfg(any())] #[cfg_attr(all(), path = concat!(\"bad\", \".rs\"))] mod excluded_nonliteral;\n",
    );
    let excluded_nonliteral = workspace.collect(&evidence).unwrap();
    assert_eq!(
        excluded_nonliteral.module_edges[0].cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );

    workspace.write(
        "src/root.rs",
        "#[cfg_attr(any(), path = concat!(\"bad\", \".rs\"))] mod inactive_nonliteral;\n",
    );
    workspace.write("src/inactive_nonliteral.rs", "pub fn child() {}\n");
    let inactive_nonliteral = workspace.collect(&evidence).unwrap();
    assert_eq!(inactive_nonliteral.module_instances.len(), 2);
    assert_eq!(
        inactive_nonliteral.module_edges[0].active_literal_path,
        None
    );

    workspace.write(
        "src/root.rs",
        "#[cfg_attr(all(), cfg_attr(all(), path = \"nested.rs\"))] mod nested;\n",
    );
    workspace.write("src/nested.rs", "pub fn child() {}\n");
    let nested = workspace.collect(&evidence).unwrap();
    assert_eq!(
        nested.module_edges[0].active_literal_path.as_deref(),
        Some("nested.rs")
    );
    assert_eq!(
        nested.module_edges[0].cfg_decision.active_path_effects[0]
            .nested_index_path
            .as_ref(),
        [0_u32, 0]
    );

    workspace.write("src/root.rs", "#[path = \"broken.rs\"] mod broken;\n");
    workspace.write("src/broken.rs", "this is not valid Rust {\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentDraft { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[path = \"../../outside.rs\"] mod escape;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::SourceOutsideWorkspace { .. })
    ));
}

#[test]
fn ancestry_cycle_sibling_reuse_raw_ident_and_block_module_are_exact() {
    let workspace = TemporaryModuleWorkspace::new("pub fn root() {}\n");
    let evidence = workspace.evidence();

    workspace.write("src/root.rs", "#[path = \"cycle_a.rs\"] mod a;\n");
    workspace.write("src/cycle_a.rs", "#[path = \"root.rs\"] mod root;\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::CanonicalCycle { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[path = \"shared.rs\"] mod left; #[path = \"shared.rs\"] mod right;\n",
    );
    workspace.write("src/shared.rs", "pub fn shared() {}\n");
    let shared = workspace.collect(&evidence).unwrap();
    assert_eq!(
        shared
            .module_instances
            .iter()
            .filter(|instance| instance.source_path_workspace_relative == "src/shared.rs")
            .count(),
        2
    );
    assert_eq!(
        shared
            .source_observations
            .iter()
            .filter(|row| row.source_path_workspace_relative == "src/shared.rs")
            .count(),
        2
    );

    workspace.write("src/root.rs", "mod r#type;\n");
    workspace.write("src/type.rs", "pub fn raw() {}\n");
    let raw = workspace.collect(&evidence).unwrap();
    assert_eq!(raw.module_edges[0].declared_ident_syntax, "r#type");
    assert_eq!(raw.module_edges[0].semantic_segment, "type");
    assert_eq!(
        raw.module_instances[1].source_path_workspace_relative,
        "src/type.rs"
    );

    workspace.write("src/root.rs", "fn outer() { mod local; }\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ModuleInBlock { .. })
    ));
}

#[test]
fn reachable_inner_cfg_and_unknown_attribute_stop_explicitly() {
    let workspace = TemporaryModuleWorkspace::new("pub fn root() {}\n");
    let evidence = workspace.evidence();
    workspace.write("src/root.rs", "#![cfg(custom_inner)]\npub fn root() {}\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentCfgUnknown { .. })
    ));
    workspace.write(
        "src/root.rs",
        "mod inline { #![cfg(custom_inner)] pub fn child() {} }\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentCfgUnknown { .. })
    ));
    workspace.write("src/root.rs", "#[custom_attr] mod child;\n");
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::UnsupportedModuleAttribute { .. })
    ));
    workspace.write(
        "src/root.rs",
        "#[cfg_attr(all(), custom_attr)] mod active_child;\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::UnsupportedModuleAttribute { .. })
    ));
}

#[test]
fn content_gate_controls_root_external_and_inline_instance_issue() {
    let workspace = TemporaryModuleWorkspace::new("#![cfg(any())]\nmod absent;\n");
    let evidence = workspace.evidence();

    let root_excluded = workspace.collect(&evidence).unwrap();
    assert_eq!(root_excluded.module_instances.len(), 1);
    assert_eq!(root_excluded.source_observations.len(), 0);
    assert_eq!(
        root_excluded.root_content_gate.cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );
    assert!(root_excluded.module_instances[0]
        .source_observation_id
        .is_none());

    workspace.write("src/root.rs", "#![cfg(all())]\nmod child;\n");
    workspace.write("src/child.rs", "#![cfg(any())]\nmod missing;\n");
    let external_excluded = workspace.collect(&evidence).unwrap();
    let edge = &external_excluded.module_edges[0];
    assert_eq!(edge.cfg_decision.final_state, CfgDecisionStateV1::Included);
    assert_eq!(
        edge.content_gate.as_ref().unwrap().cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );
    assert!(edge.child_instance_id.is_none());
    assert_eq!(external_excluded.module_instances.len(), 1);
    assert_eq!(external_excluded.source_observations.len(), 1);
    assert_eq!(
        edge.selected_source_path_workspace_relative.as_deref(),
        Some("src/child.rs")
    );

    workspace.write("src/child.rs", "#![cfg(all())]\npub fn child() {}\n");
    let external_included = workspace.collect(&evidence).unwrap();
    assert_eq!(external_included.module_instances.len(), 2);
    assert_eq!(external_included.source_observations.len(), 2);
    assert_eq!(
        external_included.module_edges[0]
            .content_gate
            .as_ref()
            .unwrap()
            .cfg_decision
            .final_state,
        CfgDecisionStateV1::Included
    );
    assert!(external_included.module_edges[0]
        .child_instance_id
        .is_some());

    workspace.write(
        "src/root.rs",
        "#![cfg(all())]\nmod inline { #![cfg(any())] mod missing; }\n",
    );
    let inline_excluded = workspace.collect(&evidence).unwrap();
    let edge = &inline_excluded.module_edges[0];
    assert_eq!(edge.kind, ModuleEdgeKindV1::Inline);
    assert_eq!(
        edge.content_gate.as_ref().unwrap().cfg_decision.final_state,
        CfgDecisionStateV1::Excluded
    );
    assert!(edge.child_instance_id.is_none());
    assert_eq!(inline_excluded.module_instances.len(), 1);
    assert_eq!(inline_excluded.source_observations.len(), 1);

    workspace.write(
        "src/root.rs",
        "#![cfg(all())]\nmod inline { #![cfg(all())] pub fn child() {} }\n",
    );
    let inline_included = workspace.collect(&evidence).unwrap();
    assert_eq!(inline_included.module_instances.len(), 2);
    assert_eq!(inline_included.source_observations.len(), 1);
    assert!(inline_included.module_edges[0].child_instance_id.is_some());
}

#[test]
fn content_gate_unknown_and_outer_exclusion_have_distinct_typed_boundaries() {
    let workspace = TemporaryModuleWorkspace::new("#[cfg(any())] mod absent;\n");
    let evidence = workspace.evidence();
    let outer_excluded = workspace.collect(&evidence).unwrap();
    assert_eq!(outer_excluded.module_edges.len(), 1);
    assert!(outer_excluded.module_edges[0].content_gate.is_none());
    assert!(outer_excluded.module_edges[0].child_instance_id.is_none());

    workspace.write(
        "src/root.rs",
        "#![cfg(content_cfg_unknown)]\npub fn root() {}\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentCfgUnknown { .. })
    ));

    workspace.write("src/root.rs", "mod child;\n");
    workspace.write(
        "src/child.rs",
        "#![cfg(content_cfg_unknown)]\npub fn child() {}\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentCfgUnknown { .. })
    ));

    workspace.write(
        "src/root.rs",
        "mod inline { #![cfg(content_cfg_unknown)] pub fn child() {} }\n",
    );
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::ContentCfgUnknown { .. })
    ));
}

#[cfg(unix)]
#[test]
fn canonical_symlink_escape_is_rejected() {
    use std::os::unix::fs::symlink;

    let workspace = TemporaryModuleWorkspace::new("#[path = \"escape.rs\"] mod escape;\n");
    let evidence = workspace.evidence();
    let outside = workspace.root.parent().unwrap().join(format!(
        "module0-outside-{}",
        workspace.root.file_name().unwrap().to_string_lossy()
    ));
    fs::write(&outside, "pub fn outside() {}\n").unwrap();
    symlink(&outside, workspace.root.join("src/escape.rs")).unwrap();
    assert!(matches!(
        workspace.collect(&evidence),
        Err(ModuleTopologyErrorV1::SourceOutsideWorkspace { .. })
    ));
    let _ = fs::remove_file(outside);
}

fn fixture_evidence(profile_id: &str) -> (PathBuf, CargoDeclaredUnitProcessEvidenceV1) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/module0_workspace")
        .canonicalize()
        .unwrap();
    let profile = fixture_profile(profile_id);
    let evidence =
        collect_declared_cargo_unit_process_evidence_v1(&root.join("Cargo.toml"), &profile)
            .unwrap();
    (root, evidence)
}

fn fixture_profile(profile_id: &str) -> ValidatedBuildProfileInputV1 {
    parse_and_verify_profile_schema_v1(FIXTURE_PROFILES)
        .unwrap()
        .profiles
        .iter()
        .find(|profile| profile.profile_id == profile_id)
        .unwrap()
        .clone()
}

struct TemporaryModuleWorkspace {
    root: PathBuf,
}

impl TemporaryModuleWorkspace {
    fn new(root_source: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rust-source-topology-module0-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\n\n[package]\nname = \"module0-temp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\ndefault = []\n\n[lib]\nname = \"module0_temp\"\npath = \"src/root.rs\"\n",
        )
        .unwrap();
        fs::write(
            root.join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\n# It is not intended for manual editing.\nversion = 4\n\n[[package]]\nname = \"module0-temp\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(root.join("src/root.rs"), root_source).unwrap();
        Self { root }
    }

    fn evidence(&self) -> CargoDeclaredUnitProcessEvidenceV1 {
        let profile = parse_and_verify_profile_schema_v1(&format!(
            "{{\"schema\":\"rust-cargo-topology-profile-schema-v1\",\"profiles\":[{{\"profile_id\":\"temp-dev\",\"package_name\":\"module0-temp\",\"target_name\":\"module0_temp\",\"target_kind\":\"library\",\"target_triple\":\"x86_64-unknown-linux-gnu\",\"cargo_profile\":\"dev\",\"compile_mode\":\"normal\",\"requested_features\":[],\"expected_activated_root_features\":[\"default\"],\"default_features_enabled\":true,\"test_cfg\":false,\"debug_assertions\":true,\"panic_strategy\":\"unwind\",\"ambient_rustflags\":{{\"kind\":\"sanitized_empty\"}}}}]}}"
        ))
        .unwrap()
        .profiles[0]
            .clone();
        collect_declared_cargo_unit_process_evidence_v1(&self.root.join("Cargo.toml"), &profile)
            .unwrap()
    }

    fn write(&self, relative: &str, source: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, source).unwrap();
    }

    fn collect(
        &self,
        evidence: &CargoDeclaredUnitProcessEvidenceV1,
    ) -> Result<rust_source_topology_check::project::DeclaredModuleTopologyV1, ModuleTopologyErrorV1>
    {
        collect_declared_module_topology_v1(&self.root, evidence)
    }
}

impl Drop for TemporaryModuleWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

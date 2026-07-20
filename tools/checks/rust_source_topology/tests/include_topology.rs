use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use rust_source_topology_check::project::cargo::{
    collect_declared_cargo_unit_process_evidence_v1, CargoDeclaredUnitProcessEvidenceV1,
};
use rust_source_topology_check::project::{
    collect_declared_module_topology_v1, parse_and_verify_profile_schema_v1, CfgDecisionStateV1,
    ModuleTopologyErrorV1,
};

#[test]
fn include_is_same_module_occurrence_and_included_modules_use_included_directory() {
    let workspace = TemporaryIncludeWorkspace::new("include!(\"parts/items.rs\");\nmod sibling;\n");
    workspace.write(
        "src/parts/items.rs",
        "pub fn included() {}\nmod child;\ninclude!(\"nested/more.inc\");\n",
    );
    workspace.write("src/parts/child.rs", "pub fn child() {}\n");
    workspace.write("src/parts/nested/more.inc", "pub fn nested() {}\n");
    workspace.write("src/sibling.rs", "pub fn sibling() {}\n");
    let topology = workspace.collect().unwrap();

    assert_eq!(topology.include_edges.len(), 2);
    assert_eq!(topology.module_instances.len(), 3);
    let root_include = &topology.include_edges[0];
    let nested_include = &topology.include_edges[1];
    assert_eq!(root_include.owning_module_instance_id, "module:0");
    assert_eq!(nested_include.owning_module_instance_id, "module:0");
    assert_eq!(
        root_include
            .selected_source_path_workspace_relative
            .as_deref(),
        Some("src/parts/items.rs")
    );
    assert_eq!(
        nested_include
            .selected_source_path_workspace_relative
            .as_deref(),
        Some("src/parts/nested/more.inc")
    );
    assert_eq!(
        nested_include.parent_include_edge_id.as_deref(),
        Some(root_include.include_edge_id.as_str())
    );
    let child = topology
        .module_instances
        .iter()
        .find(|row| row.module_syntax_path == "crate::child")
        .unwrap();
    assert_eq!(child.source_path_workspace_relative, "src/parts/child.rs");
    let child_edge = topology
        .module_edges
        .iter()
        .find(|row| row.semantic_segment == "child")
        .unwrap();
    assert_eq!(
        topology
            .source_observations
            .iter()
            .find(|row| row.source_observation_id == child_edge.declaration_source_observation_id)
            .unwrap()
            .source_path_workspace_relative,
        "src/parts/items.rs"
    );
}

#[test]
fn inline_module_include_uses_invocation_file_but_keeps_inline_identity() {
    let workspace =
        TemporaryIncludeWorkspace::new("mod inline { include!(\"parts/inline.inc\"); }\n");
    workspace.write("src/parts/inline.inc", "pub fn inside() {}\n");
    let topology = workspace.collect().unwrap();

    assert_eq!(topology.module_instances.len(), 2);
    assert_eq!(topology.include_edges.len(), 1);
    let inline = topology
        .module_instances
        .iter()
        .find(|row| row.module_syntax_path == "crate::inline")
        .unwrap();
    let include = &topology.include_edges[0];
    assert_eq!(include.owning_module_instance_id, inline.instance_id);
    assert_eq!(
        include.selected_source_path_workspace_relative.as_deref(),
        Some("src/parts/inline.inc")
    );
    let included = topology
        .source_observations
        .iter()
        .find(|row| row.parent_include_edge_id.as_deref() == Some("include:0"))
        .unwrap();
    assert_eq!(included.module_instance_id, inline.instance_id);
    assert_eq!(
        included.topology.source_file.root_module_syntax_path,
        "crate::inline"
    );
}

#[test]
fn sibling_reuse_is_distinct_but_include_ancestry_cycles_reject() {
    let workspace =
        TemporaryIncludeWorkspace::new("include!(\"shared.inc\");\ninclude!(\"shared.inc\");\n");
    workspace.write("src/shared.inc", "pub fn shared() {}\n");
    let shared = workspace.collect().unwrap();
    assert_eq!(shared.include_edges.len(), 2);
    assert_eq!(
        shared
            .source_observations
            .iter()
            .filter(|row| row.source_path_workspace_relative == "src/shared.inc")
            .count(),
        2
    );

    workspace.write("src/root.rs", "include!(\"a.inc\");\n");
    workspace.write("src/a.inc", "include!(\"root.rs\");\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::CanonicalCycle { .. })
    ));
}

#[test]
fn include_cfg_is_decided_before_path_or_token_interpretation() {
    let workspace =
        TemporaryIncludeWorkspace::new("#[cfg(any())] include!(concat!(\"missing\", \".rs\"));\n");
    let excluded = workspace.collect().unwrap();
    assert_eq!(excluded.include_edges.len(), 1);
    assert_eq!(
        excluded.include_edges[0].cfg_decision.state,
        CfgDecisionStateV1::Excluded
    );
    assert!(excluded.include_edges[0].literal_path.is_none());
    assert!(excluded.include_edges[0]
        .child_source_observation_id
        .is_none());

    workspace.write(
        "src/root.rs",
        "#[cfg(custom_unknown)] include!(\"missing.rs\");\n",
    );
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::UnknownCfg { .. })
    ));

    workspace.write("src/root.rs", "include!(concat!(\"a\", \".rs\"));\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::NonLiteralInclude { .. })
    ));

    workspace.write("src/root.rs", "include!(\"missing.rs\");\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::SourceMissing { .. })
    ));
}

#[test]
fn unsupported_context_attributes_and_macro_identity_fail_typed() {
    let workspace = TemporaryIncludeWorkspace::new("fn body() { include!(\"body.inc\"); }\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::UnsupportedIncludeContext { .. })
    ));

    workspace.write(
        "src/root.rs",
        "#[path = \"other.rs\"] include!(\"item.inc\");\n",
    );
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::UnsupportedIncludeAttribute { .. })
    ));

    workspace.write(
        "src/root.rs",
        "macro_rules! include { ($path:literal) => {}; }\ninclude!(\"item.inc\");\n",
    );
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved { .. })
    ));

    workspace.write(
        "src/root.rs",
        "use crate::prelude::*;\ninclude!(\"item.inc\");\n",
    );
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved { .. })
    ));

    workspace.write("src/root.rs", "crate::include!(\"item.inc\");\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::IncludeMacroIdentityUnresolved { .. })
    ));

    workspace.write("src/root.rs", "include!(\"item.inc\");\n");
    workspace.write("src/item.inc", "#![allow(dead_code)]\npub fn item() {}\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::UnsupportedIncludedPreamble { .. })
    ));
}

#[test]
fn include_paths_are_workspace_bounded_and_output_is_deterministic() {
    let workspace = TemporaryIncludeWorkspace::new(
        r##"include!(r#"parts/item.inc"#,);
"##,
    );
    workspace.write("src/parts/item.inc", "pub fn item() {}\n");
    let first = workspace.collect().unwrap();
    let second = workspace.collect().unwrap();
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );
    assert!(!serde_json::to_string(&first)
        .unwrap()
        .contains(workspace.root.to_string_lossy().as_ref()));

    workspace.write("src/root.rs", "include!(\"../../outside.inc\");\n");
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::SourceOutsideWorkspace { .. })
    ));

    let absolute = workspace.root.join("src/parts/item.inc");
    workspace.write(
        "src/root.rs",
        &format!("include!({:?});\n", absolute.to_string_lossy()),
    );
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::AbsoluteIncludePath { .. })
    ));
}

#[cfg(unix)]
#[test]
fn include_symlink_escape_is_rejected() {
    use std::os::unix::fs::symlink;

    let workspace = TemporaryIncludeWorkspace::new("include!(\"escape.inc\");\n");
    let outside = workspace.root.parent().unwrap().join(format!(
        "include0-outside-{}",
        workspace.root.file_name().unwrap().to_string_lossy()
    ));
    fs::write(&outside, "pub fn outside() {}\n").unwrap();
    symlink(&outside, workspace.root.join("src/escape.inc")).unwrap();
    assert!(matches!(
        workspace.collect(),
        Err(ModuleTopologyErrorV1::SourceOutsideWorkspace { .. })
    ));
    let _ = fs::remove_file(outside);
}

struct TemporaryIncludeWorkspace {
    root: PathBuf,
}

impl TemporaryIncludeWorkspace {
    fn new(root_source: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rust-source-topology-include0-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\n\n[package]\nname = \"include0-temp\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[features]\ndefault = []\n\n[lib]\nname = \"include0_temp\"\npath = \"src/root.rs\"\n",
        )
        .unwrap();
        fs::write(
            root.join("Cargo.lock"),
            "# This file is automatically @generated by Cargo.\n# It is not intended for manual editing.\nversion = 4\n\n[[package]]\nname = \"include0-temp\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();
        fs::write(root.join("src/root.rs"), root_source).unwrap();
        Self { root }
    }

    fn evidence(&self) -> CargoDeclaredUnitProcessEvidenceV1 {
        let profile = parse_and_verify_profile_schema_v1(
            "{\"schema\":\"rust-cargo-topology-profile-schema-v1\",\"profiles\":[{\"profile_id\":\"temp-dev\",\"package_name\":\"include0-temp\",\"target_name\":\"include0_temp\",\"target_kind\":\"library\",\"target_triple\":\"x86_64-unknown-linux-gnu\",\"cargo_profile\":\"dev\",\"compile_mode\":\"normal\",\"requested_features\":[],\"expected_activated_root_features\":[\"default\"],\"default_features_enabled\":true,\"test_cfg\":false,\"debug_assertions\":true,\"panic_strategy\":\"unwind\",\"ambient_rustflags\":{\"kind\":\"sanitized_empty\"}}]}",
        )
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
    ) -> Result<rust_source_topology_check::project::DeclaredModuleTopologyV1, ModuleTopologyErrorV1>
    {
        collect_declared_module_topology_v1(&self.root, &self.evidence())
    }
}

impl Drop for TemporaryIncludeWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

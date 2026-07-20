use std::fs;
use std::path::{Path, PathBuf};

use rust_source_topology_check::project::cargo::collect_declared_cargo_unit_process_evidence_v1;
use rust_source_topology_check::project::{
    cfg_environment_from_declared_unit_evidence_v1,
    collect_file_inner_topology_attribute_surface_v1, decide_cfg_attribute_stream_v1,
    parse_and_verify_profile_schema_v1, CfgAttributeStreamRowDispositionV1, CfgDecisionStateV1,
    InnerTopologyAttributeSurfaceErrorV1, ValidatedBuildProfileInputV1,
};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn file_surface_preserves_exact_meta_syntax_range_and_file_local_order() {
    let source = concat!(
        "#![allow(dead_code)]\n",
        "#![cfg(/* preserved */ all())]\n",
        "#![path = \"ignored.rs\"]\n",
        "#![cfg_attr(all(), cfg(any()))]\n",
    );
    let surface =
        collect_file_inner_topology_attribute_surface_v1("src/example.rs", source).unwrap();

    assert_eq!(surface.rows.len(), 3);
    assert_eq!(surface.inner_cfg_count, 1);
    assert_eq!(surface.inner_cfg_attr_count, 1);
    assert_eq!(surface.inner_path_count, 1);
    assert_eq!(surface.rows[0].source_ordinal, 0);
    assert_eq!(surface.rows[1].source_ordinal, 1);
    assert_eq!(surface.rows[2].source_ordinal, 2);
    assert_eq!(surface.rows[0].syntax, "cfg(/* preserved */ all())");
    for row in surface.rows.iter() {
        assert_eq!(
            &source[row.source_range.byte_start..row.source_range.byte_end],
            row.syntax
        );
    }

    let decision =
        decide_cfg_attribute_stream_v1(&surface.rows, &host_default_environment()).unwrap();
    assert_eq!(decision.final_state, CfgDecisionStateV1::Excluded);
    assert_eq!(decision.rows[0].state, Some(CfgDecisionStateV1::Included));
    assert_eq!(
        decision.rows[1].disposition,
        CfgAttributeStreamRowDispositionV1::TopologyNeutral
    );
}

#[test]
fn malformed_file_is_rejected_before_any_stream_decision() {
    let error =
        collect_file_inner_topology_attribute_surface_v1("src/broken.rs", "#![cfg(any())\n")
            .unwrap_err();
    assert!(matches!(
        error,
        InnerTopologyAttributeSurfaceErrorV1::Parse { .. }
    ));
}

#[test]
fn root_inner_attribute_inventory_and_sealed_six_profile_matrix_are_exact() {
    let root = root_dir();
    let surfaces = collect_src_surfaces(&root);

    assert_eq!(surfaces.len(), 17);
    assert_eq!(
        surfaces
            .iter()
            .map(|surface| surface.inner_cfg_count)
            .sum::<usize>(),
        17
    );
    assert_eq!(
        surfaces
            .iter()
            .map(|surface| surface.inner_cfg_attr_count)
            .sum::<usize>(),
        0
    );
    assert_eq!(
        surfaces
            .iter()
            .map(|surface| surface.inner_path_count)
            .sum::<usize>(),
        0
    );
    assert!(surfaces.iter().all(|surface| surface.rows.len() == 1));

    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let expected_included = [
        ("host-default-dev", 0),
        ("host-default-release", 0),
        ("host-llvm-harness-dev", 0),
        ("host-test-unit-default", 0),
        ("host-vm-reference-dev", 8),
        ("wasm32-default-dev", 0),
    ];
    for (profile_id, expected) in expected_included {
        let evidence = collect_declared_cargo_unit_process_evidence_v1(
            &root.join("Cargo.toml"),
            profile(&schema.profiles, profile_id),
        )
        .unwrap();
        let environment = cfg_environment_from_declared_unit_evidence_v1(&evidence);
        let decisions = surfaces
            .iter()
            .map(|surface| decide_cfg_attribute_stream_v1(&surface.rows, &environment).unwrap())
            .collect::<Vec<_>>();
        assert!(decisions
            .iter()
            .all(|decision| decision.final_state != CfgDecisionStateV1::Unknown));
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| decision.final_state == CfgDecisionStateV1::Included)
                .count(),
            expected,
            "profile={profile_id}"
        );
    }
}

fn host_default_environment() -> rust_source_topology_check::project::CfgEvaluationEnvironmentV1 {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    rust_source_topology_check::project::CfgEvaluationEnvironmentV1::from_profile_input(profile(
        &schema.profiles,
        "host-default-dev",
    ))
}

fn collect_src_surfaces(
    root: &Path,
) -> Vec<rust_source_topology_check::project::FileInnerTopologyAttributeSurfaceV1> {
    let mut files = Vec::new();
    collect_rs_files(&root.join("src"), &mut files);
    files.sort();
    files
        .into_iter()
        .filter_map(|path| {
            let source = fs::read_to_string(&path).unwrap();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            let surface =
                collect_file_inner_topology_attribute_surface_v1(&relative, &source).unwrap();
            (!surface.rows.is_empty()).then_some(surface)
        })
        .collect()
}

fn collect_rs_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let mut entries = fs::read_dir(directory)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_rs_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn profile<'a>(
    profiles: &'a [ValidatedBuildProfileInputV1],
    profile_id: &str,
) -> &'a ValidatedBuildProfileInputV1 {
    profiles
        .iter()
        .find(|profile| profile.profile_id == profile_id)
        .unwrap()
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

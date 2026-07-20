use std::path::{Path, PathBuf};

use rust_source_topology_check::project::cargo::{
    collect_declared_cargo_unit_process_evidence_v1, CargoDeclaredUnitProcessEvidenceV1,
};
use rust_source_topology_check::project::{
    collect_declared_module_topology_v1, parse_and_verify_profile_schema_v1, CfgDecisionStateV1,
    DeclaredModuleTopologyV1, ValidatedBuildProfileInputV1,
};

const ROOT_PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn root_six_profile_topologies_are_exact_deterministic_and_path_clean() {
    let root = root_dir();
    let schema = parse_and_verify_profile_schema_v1(ROOT_PROFILES).unwrap();
    let evidence = collect_evidence(&root, &schema.profiles);
    let first = collect_topologies(&root, &evidence);
    let second = collect_topologies(&root, &evidence);

    assert_eq!(first.len(), 6);
    assert_eq!(
        serde_json::to_vec(&first).unwrap(),
        serde_json::to_vec(&second).unwrap()
    );

    let expected = [
        ("host-default-dev", 2282, 3128, 3, 2260),
        ("host-default-release", 2281, 3126, 3, 2259),
        ("host-llvm-harness-dev", 2282, 3128, 3, 2260),
        ("host-test-unit-default", 3538, 3633, 3, 2921),
        ("host-vm-reference-dev", 2359, 3227, 3, 2336),
        ("wasm32-default-dev", 2255, 3096, 3, 2233),
    ];
    assert_eq!(
        first
            .iter()
            .map(|topology| topology.profile_id.as_str())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|(profile_id, ..)| *profile_id)
            .collect::<Vec<_>>()
    );

    for (topology, (profile_id, instances, module_edges, include_edges, observations)) in
        first.iter().zip(expected)
    {
        assert_eq!(topology.profile_id, profile_id);
        assert_eq!(
            topology.module_instances.len(),
            instances,
            "profile={profile_id}"
        );
        assert_eq!(
            topology.module_edges.len(),
            module_edges,
            "profile={profile_id}"
        );
        assert_eq!(
            topology.include_edges.len(),
            include_edges,
            "profile={profile_id}"
        );
        assert_eq!(
            topology.source_observations.len(),
            observations,
            "profile={profile_id}"
        );
        assert_eq!(
            topology.root_content_gate.cfg_decision.final_state,
            CfgDecisionStateV1::Included,
            "profile={profile_id}"
        );
        let serialized = String::from_utf8(serde_json::to_vec(topology).unwrap()).unwrap();
        assert!(!serialized.contains(root.to_string_lossy().as_ref()));
    }
}

fn collect_evidence(
    root: &Path,
    profiles: &[ValidatedBuildProfileInputV1],
) -> Vec<CargoDeclaredUnitProcessEvidenceV1> {
    profiles
        .iter()
        .map(|profile| {
            collect_declared_cargo_unit_process_evidence_v1(&root.join("Cargo.toml"), profile)
                .unwrap()
        })
        .collect()
}

fn collect_topologies(
    root: &Path,
    evidence: &[CargoDeclaredUnitProcessEvidenceV1],
) -> Vec<DeclaredModuleTopologyV1> {
    evidence
        .iter()
        .map(|evidence| {
            let topology = collect_declared_module_topology_v1(root, evidence).unwrap();
            assert_eq!(topology.profile_id, evidence.declared_unit().profile_id());
            assert_eq!(
                topology.package_key,
                evidence.declared_unit().package().package_key()
            );
            assert_eq!(
                topology.target_key,
                evidence.declared_unit().target().target_key()
            );
            topology
        })
        .collect()
}

fn root_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

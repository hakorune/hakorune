use rust_source_topology_check::project::{
    decide_cfg_attribute_stream_v1, parse_and_verify_profile_schema_v1,
    CfgAttributeStreamInputRowV1, CfgDecisionStateV1, DeclaredModuleContentGateV1,
    ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1,
};
use rust_source_topology_check::{PositionV1, SourceRangeV1};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn content_gate_keeps_root_identity_separate_from_source_file_evidence() {
    let gate = gate(
        ModuleContentCandidateIdV1::Root,
        ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative: "src/lib.rs".to_string(),
            content_digest: "fnv1a64:root".to_string(),
        },
    );

    assert!(matches!(gate.candidate_id, ModuleContentCandidateIdV1::Root));
    assert_eq!(gate.inner_cfg_sites.len(), 1);
    assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Included);
}

#[test]
fn content_gate_keeps_edge_identity_separate_from_inline_body_evidence() {
    let gate = gate(
        ModuleContentCandidateIdV1::ModuleEdge {
            edge_id: "edge:7".to_string(),
        },
        ModuleContentDefiningSurfaceV1::InlineBody {
            parent_source_observation_id: "source:3".to_string(),
            body_range: range(20, 31),
        },
    );

    assert_eq!(
        gate.candidate_id,
        ModuleContentCandidateIdV1::ModuleEdge {
            edge_id: "edge:7".to_string(),
        }
    );
    assert_eq!(
        gate.defining_surface,
        ModuleContentDefiningSurfaceV1::InlineBody {
            parent_source_observation_id: "source:3".to_string(),
            body_range: range(20, 31),
        }
    );
}

fn gate(
    candidate_id: ModuleContentCandidateIdV1,
    defining_surface: ModuleContentDefiningSurfaceV1,
) -> DeclaredModuleContentGateV1 {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let profile = schema
        .profiles
        .iter()
        .find(|profile| profile.profile_id == "host-default-dev")
        .unwrap();
    let rows = vec![CfgAttributeStreamInputRowV1 {
        source_ordinal: 0,
        source_range: range(0, 10),
        syntax: "cfg(all())".to_string(),
    }];
    let environment =
        rust_source_topology_check::project::CfgEvaluationEnvironmentV1::from_profile_input(
            &profile,
        );
    DeclaredModuleContentGateV1 {
        candidate_id,
        defining_surface,
        inner_cfg_sites: rows.clone().into_boxed_slice(),
        cfg_decision: decide_cfg_attribute_stream_v1(&rows, &environment).unwrap(),
    }
}

fn range(byte_start: usize, byte_end: usize) -> SourceRangeV1 {
    SourceRangeV1 {
        start: PositionV1 { line: 1, column: 0 },
        end: PositionV1 {
            line: 1,
            column: byte_end - byte_start,
        },
        byte_start,
        byte_end,
    }
}

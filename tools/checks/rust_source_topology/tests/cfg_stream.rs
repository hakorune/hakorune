use rust_source_topology_check::project::{
    decide_cfg_attribute_stream_v1, parse_and_verify_profile_schema_v1,
    CfgAttributeNestedDispositionV1, CfgAttributeStreamErrorV1, CfgAttributeStreamInputRowV1,
    CfgAttributeStreamRowDispositionV1, CfgDecisionStateV1, CfgEvaluationEnvironmentV1,
};
use rust_source_topology_check::{PositionV1, SourceRangeV1};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn exclusion_short_circuits_later_malformed_and_unknown_rows() {
    let decision = decide(&[
        row(0, "cfg(any())"),
        row(1, "cfg(not())"),
        row(2, "cfg(custom_build)"),
    ])
    .unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Excluded);
    assert_eq!(decision.decisive_row_ordinal, Some(0));
    assert_eq!(decision.rows.len(), 3);
    assert_eq!(
        decision.rows[1].disposition,
        CfgAttributeStreamRowDispositionV1::NotReachedAfterExclusion
    );
    assert_eq!(
        decision.rows[2].disposition,
        CfgAttributeStreamRowDispositionV1::NotReachedAfterExclusion
    );
}

#[test]
fn malformed_row_is_typed_before_a_later_exclusion() {
    let error = decide(&[row(7, "cfg(not())"), row(8, "cfg(any())")]).unwrap_err();

    assert!(matches!(
        error,
        CfgAttributeStreamErrorV1::Row {
            source_ordinal: 7,
            byte_start: 700,
            byte_end: 710,
            ..
        }
    ));
}

#[test]
fn unknown_row_is_terminal_and_is_not_erased_by_a_later_false_row() {
    let decision = decide(&[row(3, "cfg(custom_build)"), row(4, "cfg(any())")]).unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Unknown);
    assert_eq!(decision.decisive_row_ordinal, Some(3));
    assert_eq!(decision.rows.len(), 1);
    assert_eq!(
        decision.rows[0].unknown_predicates.as_ref(),
        ["flag=custom_build"]
    );
}

#[test]
fn inactive_cfg_attr_keeps_malformed_nested_cfg_unparsed() {
    let decision = decide(&[row(0, "cfg_attr(any(), cfg(not()))")]).unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Included);
    let row = &decision.rows[0];
    assert_eq!(
        row.disposition,
        CfgAttributeStreamRowDispositionV1::Evaluated
    );
    assert_eq!(
        row.cfg_attr_condition.as_ref().unwrap().state,
        CfgDecisionStateV1::Excluded
    );
    assert_eq!(row.nested.len(), 1);
    assert_eq!(
        row.nested[0].disposition,
        CfgAttributeNestedDispositionV1::NotEvaluatedInactiveCfgAttr
    );
    assert_eq!(row.nested[0].state, None);
    assert!(decision.active_path_effects.is_empty());
}

#[test]
fn included_stream_exports_exact_direct_and_nested_path_effects() {
    let decision = decide(&[
        row(4, "path = \"direct.rs\""),
        row(9, "cfg_attr(all(), cfg_attr(all(), path = \"nested.rs\"))"),
    ])
    .unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Included);
    assert_eq!(decision.active_path_effects.len(), 2);
    assert_eq!(decision.active_path_effects[0].outer_source_ordinal, 4);
    assert_eq!(
        decision.active_path_effects[0].outer_source_range,
        row(4, "path = \"direct.rs\"").source_range
    );
    assert!(decision.active_path_effects[0].nested_index_path.is_empty());
    assert_eq!(
        decision.active_path_effects[0].syntax,
        "path = \"direct.rs\""
    );
    assert_eq!(decision.active_path_effects[1].outer_source_ordinal, 9);
    assert_eq!(
        decision.active_path_effects[1].nested_index_path.as_ref(),
        [0_u32, 0]
    );
    assert_eq!(
        decision.active_path_effects[1].syntax,
        "path = \"nested.rs\""
    );
}

#[test]
fn active_nested_cfg_attr_retains_recursive_evidence() {
    let decision = decide(&[row(0, "cfg_attr(all(), cfg_attr(all(), cfg(any())))")]).unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Excluded);
    let outer = &decision.rows[0];
    assert_eq!(outer.nested.len(), 1);
    let nested = &outer.nested[0];
    assert_eq!(
        nested.disposition,
        CfgAttributeNestedDispositionV1::Evaluated
    );
    assert_eq!(nested.state, Some(CfgDecisionStateV1::Excluded));
    assert_eq!(nested.nested.len(), 1);
    assert_eq!(nested.nested[0].state, Some(CfgDecisionStateV1::Excluded));
}

#[test]
fn active_nested_exclusion_short_circuits_later_nested_rows() {
    let decision = decide(&[row(
        0,
        "cfg_attr(all(), cfg(any()), cfg(not()), cfg(custom_build))",
    )])
    .unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Excluded);
    let nested = &decision.rows[0].nested;
    assert_eq!(nested.len(), 3);
    assert_eq!(nested[0].state, Some(CfgDecisionStateV1::Excluded));
    assert_eq!(
        nested[1].disposition,
        CfgAttributeNestedDispositionV1::NotReachedAfterExclusion
    );
    assert_eq!(
        nested[2].disposition,
        CfgAttributeNestedDispositionV1::NotReachedAfterExclusion
    );
}

#[test]
fn active_cfg_attr_rejects_malformed_nested_separator() {
    let error = decide(&[row(0, "cfg_attr(all(), , cfg(any()))")]).unwrap_err();

    assert!(matches!(error, CfgAttributeStreamErrorV1::Row { .. }));
}

#[test]
fn stream_input_ordinals_are_strictly_increasing() {
    let duplicate = decide(&[row(2, "cfg(all())"), row(2, "cfg(all())")]).unwrap_err();
    assert!(matches!(
        duplicate,
        CfgAttributeStreamErrorV1::DuplicateSourceOrdinal { source_ordinal: 2 }
    ));

    let non_monotonic = decide(&[row(3, "cfg(all())"), row(1, "cfg(all())")]).unwrap_err();
    assert!(matches!(
        non_monotonic,
        CfgAttributeStreamErrorV1::NonMonotonicSourceOrdinal {
            previous_ordinal: 3,
            source_ordinal: 1,
        }
    ));
}

#[test]
fn unknown_cfg_attr_path_retains_the_existing_topology_unknown_witness() {
    let decision = decide(&[row(0, "cfg_attr(custom_build, path = \"alternate.rs\")")]).unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Unknown);
    assert_eq!(
        decision.rows[0].unknown_predicates.as_ref(),
        ["cfg_attr:path:path = \"alternate.rs\"", "flag=custom_build",]
    );
}

#[test]
fn unknown_cfg_attr_recursively_retains_a_nested_path_witness() {
    let decision = decide(&[row(
        0,
        "cfg_attr(custom_build, cfg_attr(all(), path = \"alternate.rs\"))",
    )])
    .unwrap();

    assert_eq!(decision.final_state, CfgDecisionStateV1::Unknown);
    assert_eq!(
        decision.rows[0].unknown_predicates.as_ref(),
        ["cfg_attr:path:path = \"alternate.rs\"", "flag=custom_build",]
    );
}

#[test]
fn empty_stream_is_included_and_an_ordinary_attribute_is_topology_neutral() {
    let empty = decide(&[]).unwrap();
    assert_eq!(empty.final_state, CfgDecisionStateV1::Included);
    assert_eq!(empty.decisive_row_ordinal, None);
    assert!(empty.rows.is_empty());

    let neutral = decide(&[row(9, "allow(dead_code)")]).unwrap();
    assert_eq!(neutral.final_state, CfgDecisionStateV1::Included);
    assert_eq!(
        neutral.rows[0].disposition,
        CfgAttributeStreamRowDispositionV1::TopologyNeutral
    );
}

fn decide(
    rows: &[CfgAttributeStreamInputRowV1],
) -> Result<
    rust_source_topology_check::project::CfgAttributeStreamDecisionV1,
    CfgAttributeStreamErrorV1,
> {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let profile = schema
        .profiles
        .iter()
        .find(|profile| profile.profile_id == "host-default-dev")
        .unwrap();
    let environment = CfgEvaluationEnvironmentV1::from_profile_input(profile);
    decide_cfg_attribute_stream_v1(rows, &environment)
}

fn row(source_ordinal: u32, syntax: &str) -> CfgAttributeStreamInputRowV1 {
    let byte_start = usize::try_from(source_ordinal).unwrap() * 100;
    CfgAttributeStreamInputRowV1 {
        source_ordinal,
        source_range: SourceRangeV1 {
            start: PositionV1 {
                line: usize::try_from(source_ordinal).unwrap() + 1,
                column: 0,
            },
            end: PositionV1 {
                line: usize::try_from(source_ordinal).unwrap() + 1,
                column: syntax.len(),
            },
            byte_start,
            byte_end: byte_start + syntax.len(),
        },
        syntax: syntax.to_string(),
    }
}

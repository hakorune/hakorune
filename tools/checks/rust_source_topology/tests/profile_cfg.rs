use rust_source_topology_check::project::{
    decide_cfg_rows_v1, parse_and_verify_profile_schema_v1, CfgDecisionStateV1,
    CfgEvaluationEnvironmentV1, ProfileValidationErrorV1, ValidatedBuildProfileInputV1,
};

const PROFILES: &str = include_str!("fixtures/profiles_v1.json");

#[test]
fn six_exact_profile_inputs_normalize_deterministically() {
    let first = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let second = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.profiles.len(), 6);
    assert_eq!(first.profiles[0].profile_id, "host-default-dev");
    assert_eq!(first.profiles[5].profile_id, "wasm32-default-dev");
    let llvm = profile(&first.profiles, "host-llvm-harness-dev");
    assert!(llvm
        .expected_activated_root_features
        .contains(&"llvm-harness".to_string()));
    assert!(!llvm
        .expected_activated_root_features
        .contains(&"llvm".to_string()));
}

#[test]
fn feature_target_test_and_debug_release_rows_are_independent() {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    assert_state(
        profile(&schema.profiles, "host-vm-reference-dev"),
        "cfg(feature = \"vm-reference\")",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        profile(&schema.profiles, "host-llvm-harness-dev"),
        "cfg(feature = \"llvm\")",
        CfgDecisionStateV1::Excluded,
    );
    assert_state(
        profile(&schema.profiles, "wasm32-default-dev"),
        "cfg(all(feature = \"plugins\", not(target_arch = \"wasm32\")))",
        CfgDecisionStateV1::Excluded,
    );
    assert_state(
        profile(&schema.profiles, "wasm32-default-dev"),
        "cfg(feature = \"wasm-backend\")",
        CfgDecisionStateV1::Excluded,
    );
    assert_state(
        profile(&schema.profiles, "wasm32-default-dev"),
        "cfg(any(not(feature = \"plugins\"), target_arch = \"wasm32\"))",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        profile(&schema.profiles, "host-test-unit-default"),
        "cfg(test)",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        profile(&schema.profiles, "host-default-dev"),
        "cfg(debug_assertions)",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        profile(&schema.profiles, "host-default-release"),
        "cfg(debug_assertions)",
        CfgDecisionStateV1::Excluded,
    );
}

#[test]
fn cfg_attr_and_unknown_topology_effects_follow_three_valued_law() {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let normal = profile(&schema.profiles, "host-default-dev");
    let test = profile(&schema.profiles, "host-test-unit-default");

    assert_state(
        normal,
        "cfg_attr(test, cfg(feature = \"vm-reference\"))",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        test,
        "cfg_attr(test, cfg(feature = \"vm-reference\"))",
        CfgDecisionStateV1::Excluded,
    );
    assert_state(
        normal,
        "cfg_attr(custom_build, allow(dead_code))",
        CfgDecisionStateV1::Included,
    );
    assert_state(
        normal,
        "cfg_attr(custom_build, path = \"alternate.rs\")",
        CfgDecisionStateV1::Unknown,
    );
    let environment = CfgEvaluationEnvironmentV1::from_profile_input(normal);
    let decision = decide_cfg_rows_v1(
        &[
            "cfg(feature = \"plugins\")".to_string(),
            "cfg(custom_build)".to_string(),
        ],
        &environment,
    )
    .unwrap();
    assert_eq!(decision.state, CfgDecisionStateV1::Unknown);
    assert_eq!(
        decision.rows[1].unknown_predicates.as_ref(),
        ["flag=custom_build"]
    );
}

#[test]
fn target_feature_without_sealed_codegen_evidence_is_unknown() {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    assert_state(
        profile(&schema.profiles, "host-default-dev"),
        "cfg(target_feature = \"sse2\")",
        CfgDecisionStateV1::Unknown,
    );
}

#[test]
fn kleene_short_circuit_preserves_decidable_results() {
    let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
    let profile = profile(&schema.profiles, "host-default-dev");
    assert_state(
        profile,
        "cfg(all(feature = \"missing\", custom_build))",
        CfgDecisionStateV1::Excluded,
    );
    assert_state(
        profile,
        "cfg(any(feature = \"plugins\", custom_build))",
        CfgDecisionStateV1::Included,
    );
}

#[test]
fn malformed_or_unsealed_profile_inputs_fail_typed() {
    assert!(matches!(
        parse_and_verify_profile_schema_v1("{}"),
        Err(ProfileValidationErrorV1::Json { .. })
    ));
    let duplicate = PROFILES.replacen(
        "\"requested_features\": [],",
        "\"requested_features\": [\"cli\", \"cli\"],",
        1,
    );
    assert!(matches!(
        parse_and_verify_profile_schema_v1(&duplicate),
        Err(ProfileValidationErrorV1::DuplicateFeature { .. })
    ));
    let wrong_test = PROFILES.replacen(
        "\"compile_mode\": \"normal\",",
        "\"compile_mode\": \"unit_test_harness\",",
        1,
    );
    assert!(matches!(
        parse_and_verify_profile_schema_v1(&wrong_test),
        Err(ProfileValidationErrorV1::TestCompileModeMismatch { .. })
    ));
    let unsealed_flags = PROFILES.replacen(
        "{ \"kind\": \"sanitized_empty\" }",
        "{ \"kind\": \"fingerprint_only_unknown\", \"digest\": \"fnv1a64:test\" }",
        1,
    );
    assert!(matches!(
        parse_and_verify_profile_schema_v1(&unsealed_flags),
        Err(ProfileValidationErrorV1::UnsealedAmbientRustflags { .. })
    ));
}

fn assert_state(
    profile: &ValidatedBuildProfileInputV1,
    syntax: &str,
    expected: CfgDecisionStateV1,
) {
    let environment = CfgEvaluationEnvironmentV1::from_profile_input(profile);
    let decision = decide_cfg_rows_v1(&[syntax.to_string()], &environment).unwrap();
    assert_eq!(
        decision.state, expected,
        "profile={} syntax={syntax}",
        profile.profile_id
    );
}

fn profile<'a>(
    profiles: &'a [ValidatedBuildProfileInputV1],
    id: &str,
) -> &'a ValidatedBuildProfileInputV1 {
    profiles
        .iter()
        .find(|profile| profile.profile_id == id)
        .unwrap()
}

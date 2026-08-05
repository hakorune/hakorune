//! D2-S1 test-only source evidence for planner-required V0 suppression.
//!
//! This row repairs the D1 mode boundary only.  It observes the existing
//! parsed S2A projector under one real configuration scope and emits no
//! selection capability, Legacy receipt, or production policy.

use super::generic_resolved_carrier_projector_tests::{
    issue_projector_handoff_for_test, ProjectorHandoffObservationV1, NESTED_IF_SOURCE,
};
use super::route_id::LoopRouteId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlannerSuppressionDispositionV1 {
    UnresolvedStopPlannerRequiredV0Suppression,
}

fn strict_planner_config() -> crate::test_support::ScopedTestConfig {
    crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", Some("1")),
        ("HAKO_JOINIR_PLANNER_REQUIRED", Some("1")),
        ("NYASH_JOINIR_STRICT", None),
    ])
}

fn release_config() -> crate::test_support::ScopedTestConfig {
    crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", None),
        ("HAKO_JOINIR_PLANNER_REQUIRED", None),
        ("NYASH_JOINIR_STRICT", None),
    ])
}

// Evidence classification only: it is deliberately not a selector or policy
// owner.  Any missing source-backed fact remains a test failure, not Legacy.
fn classify_planner_suppression(
    receipt: &ProjectorHandoffObservationV1,
) -> PlannerSuppressionDispositionV1 {
    assert_eq!(receipt.mode_flags(), (true, true));
    assert_eq!(receipt.facts_flags(), (false, true));
    assert_eq!(receipt.raw_schedule(), [LoopRouteId::GenericLoopV1]);
    assert!(receipt.source_identity_is_stable());
    assert!(receipt.recursive_carrier_count() > 0);
    let (recipe_first_allowed, _) = receipt.preflight_flags();
    assert!(recipe_first_allowed);
    PlannerSuppressionDispositionV1::UnresolvedStopPlannerRequiredV0Suppression
}

#[test]
fn planner_required_s2a_source_co_seals_actual_suppression() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let receipt = {
        let _config = strict_planner_config();
        issue_projector_handoff_for_test(NESTED_IF_SOURCE)
            .expect("parsed S2A source must produce a planner suppression witness")
    };

    assert_eq!(receipt.source_forest_len(), 2);
    assert_eq!(
        classify_planner_suppression(&receipt),
        PlannerSuppressionDispositionV1::UnresolvedStopPlannerRequiredV0Suppression
    );
}

#[test]
fn planner_required_s2a_source_repeat_is_stable_under_one_mode() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let (first, second) = {
        let _config = strict_planner_config();
        let first =
            issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("first parsed S2A witness");
        let second =
            issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("repeat parsed S2A witness");
        (first, second)
    };

    assert!(first.source_identity_is_stable());
    assert!(second.source_identity_is_stable());
    assert_eq!(first.mode_flags(), second.mode_flags());
    assert_eq!(first.facts_flags(), second.facts_flags());
    assert_eq!(first.raw_schedule(), second.raw_schedule());
    assert_eq!(first.source_forest_len(), second.source_forest_len());
    assert_eq!(first.preflight_flags(), second.preflight_flags());
    assert_eq!(
        classify_planner_suppression(&first),
        classify_planner_suppression(&second)
    );
}

#[test]
fn planner_required_mode_is_not_re_paired_with_release_both() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let release = {
        let _config = release_config();
        issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("release S2A witness")
    };
    let planner = {
        let _config = strict_planner_config();
        issue_projector_handoff_for_test(NESTED_IF_SOURCE).expect("planner S2A witness")
    };

    assert_eq!(release.mode_flags(), (false, false));
    assert_eq!(
        release.raw_schedule(),
        [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(planner.mode_flags(), (true, true));
    assert_ne!(release.mode_flags(), planner.mode_flags());
    assert!(release.source_identity_is_stable());
    assert!(planner.source_identity_is_stable());
    assert_eq!(
        classify_planner_suppression(&planner),
        PlannerSuppressionDispositionV1::UnresolvedStopPlannerRequiredV0Suppression
    );
}

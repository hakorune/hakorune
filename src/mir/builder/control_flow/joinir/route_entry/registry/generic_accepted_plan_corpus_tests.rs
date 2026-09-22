//! Existing accepted-plan corpus, separated from semantic witness helpers.
use super::*;

#[test]
fn generic_accepted_plan_reachability_corpus_is_test_only_and_repeatable() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut accepted = 0usize;
    let mut both_lower_some = 0usize;
    for mode in [
        CorpusModeV1::Release,
        CorpusModeV1::Strict,
        CorpusModeV1::StrictPlannerRequired,
    ] {
        let _config = crate::test_support::ScopedTestConfig::apply(&[
            ("HAKO_JOINIR_STRICT", mode.env().1),
            ("HAKO_JOINIR_PLANNER_REQUIRED", mode.planner_required()),
            ("NYASH_JOINIR_STRICT", None),
        ]);
        for name in [
            "v1-only",
            "v1-only-effect",
            "effect-no-local",
            "v0-additive",
            "v1-true-body-step",
            "both",
            "simple-while",
            "neither",
        ] {
            let rows = observe_fixture(mode, name);
            for row in rows {
                if row.stage == PlanStageV1::ComposerError {
                    if matches!(name, "v1-only-effect" | "effect-no-local") {
                        assert_eq!(
                            row.route,
                            LoopRouteId::GenericLoopV1,
                            "effect-call boundary must remain V1-only: {row:?}"
                        );
                        assert_eq!(
                            row.first_effect_owner,
                            EffectOwnerV1::GenericComposer,
                            "effect-call boundary must classify composer failure as effectful: {row:?}"
                        );
                        assert_eq!(
                            row.stage,
                            PlanStageV1::ComposerError,
                            "effect-call boundary must stop at the actual composer error: {row:?}"
                        );
                    }
                    // Composer errors are split by the observed candidate
                    // owner.  `None` is a precondition stop; a Generic owner
                    // means the composer entered its pipeline and this row
                    // remains an effectful unresolved stop.
                    let repeat = observe_fixture(mode, name)
                        .into_iter()
                        .find(|candidate| candidate.route == row.route)
                        .expect("repeat fixture must retain selected Generic route");
                    assert_eq!(
                        row.stage, repeat.stage,
                        "fresh candidate stage drift: {row:?}"
                    );
                    continue;
                }
                assert!(
                    row.root_is_loop,
                    "Generic composer root must be Loop: {row:?}"
                );
                assert!(
                    row.first_effect_owner == EffectOwnerV1::GenericComposer,
                    "accepted Generic composition must leave candidate evidence: {row:?}"
                );
                if name == "v0-additive" {
                    if !matches!(mode, CorpusModeV1::StrictPlannerRequired) {
                        assert_eq!(
                            row.stage,
                            PlanStageV1::LowerSome,
                            "additive V0 row must reach a terminal lower success outside planner-required mode: {row:?}"
                        );
                    }
                }
                if name == "v1-true-body-step" {
                    if !matches!(mode, CorpusModeV1::StrictPlannerRequired) {
                        assert_eq!(
                            row.stage,
                            PlanStageV1::LowerSome,
                            "true-condition V1 row must reach a terminal lower success outside planner-required mode: {row:?}"
                        );
                    }
                }
                let repeat = observe_fixture(mode, name)
                    .into_iter()
                    .find(|candidate| candidate.route == row.route)
                    .expect("repeat fixture must retain selected Generic route");
                assert_eq!(
                    row.stage, repeat.stage,
                    "fresh candidate stage drift: {row:?}"
                );
                assert_eq!(row.before_lower, repeat.before_lower);
                assert_eq!(row.after_lower, repeat.after_lower);
                if row.stage == PlanStageV1::LowerSome {
                    accepted += 1;
                    if name == "both" {
                        both_lower_some += 1;
                    }
                }
            }
        }
    }
    assert!(
        accepted >= 3,
        "known Generic corpus must reach lower success in at least three rows"
    );
    assert!(
        both_lower_some >= 2,
        "Both fixture must observe V0/V1 lower success in at least two mode rows"
    );
}

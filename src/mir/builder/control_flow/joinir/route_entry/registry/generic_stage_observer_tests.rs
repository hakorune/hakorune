//! M4-D3 test-only observation of the real Generic handler path.
//!
//! This is deliberately an observer, not a second scheduler.  It reuses the
//! source-to-selection result from the A1 Both fixture, then invokes the same
//! ENTRIES dispatch used by production through the existing witness executor.

use super::execution_witness::{
    PostEffectRetryDebtV1, RouteAttemptOutcomeV1, RouteExecutionResultV1,
};
use super::generic_selection_matrix_tests::{
    both_body, effect_without_local_body, progression_condition, v1_only_effect_body,
};
use super::generic_accepted_plan_reachability_tests::{
    observe_both_direct_stage, GenericDirectStageEvidenceV1,
};
use super::route_id::LoopRouteId;
use super::dispatch_entry;
use crate::mir::builder::control_flow::joinir::route_entry::router::{
    test_issue_live_preflight_frame, LoopRouteContext,
};
use crate::mir::builder::control_flow::plan::single_planner::try_build_outcome;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ObserverModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

impl ObserverModeV1 {
    fn strict_or_dev(self) -> bool {
        !matches!(self, Self::Release)
    }

    fn planner_required(self) -> bool {
        matches!(self, Self::StrictPlannerRequired)
    }

    fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                if matches!(self, Self::Release) {
                    None
                } else {
                    Some("1")
                },
            ),
            (
                "HAKO_JOINIR_PLANNER_REQUIRED",
                if matches!(self, Self::StrictPlannerRequired) {
                    Some("1")
                } else {
                    None
                },
            ),
            ("NYASH_JOINIR_STRICT", None),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AttemptTraceV1 {
    route: LoopRouteId,
    cursor: usize,
    suffix: Vec<LoopRouteId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GenericDebtTraceV1 {
    route: LoopRouteId,
    composer: super::legacy_receipt::LegacyGenericComposerV1,
    result: super::legacy_receipt::LegacyGenericResultKindV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TerminalTraceV1 {
    Succeeded(LoopRouteId),
    Exhausted,
    Blocked,
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FrameTraceV1 {
    strict_or_dev: bool,
    planner_required: bool,
    has_body_local: bool,
    recipe_contract_present: bool,
    recipe_first_allowed: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericStageTraceV1 {
    frame: FrameTraceV1,
    raw_schedule: Vec<LoopRouteId>,
    attempted: Vec<AttemptTraceV1>,
    generic_debts: Vec<GenericDebtTraceV1>,
    terminal: TerminalTraceV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericOverlapEvidenceRowV1 {
    mode: ObserverModeV1,
    direct: Vec<GenericDirectStageEvidenceV1>,
    witness: GenericStageTraceV1,
}

fn seeded_builder() -> MirBuilder {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("generic_stage_observer/0".to_string());
    for name in ["i", "j"] {
        let value = builder.alloc_typed(MirType::Integer);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
    }
    builder
}

fn observe_selected_fixture(
    mode: ObserverModeV1,
    condition: crate::ast::ASTNode,
    body: Vec<crate::ast::ASTNode>,
    function_name: &str,
) -> GenericStageTraceV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = mode.config();
    let ctx = LoopRouteContext::new(&condition, &body, function_name, false, false);
    let outcome = try_build_outcome(&ctx).expect("Both fixture must build facts");
    let facts = outcome
        .facts
        .as_ref()
        .expect("Both fixture must produce canonical facts");
    let frame = test_issue_live_preflight_frame(
        &ctx,
        &outcome,
        mode.strict_or_dev(),
        mode.planner_required(),
    );
    let env = frame.test_env();
    let frame_trace = FrameTraceV1 {
        strict_or_dev: env.strict_or_dev,
        planner_required: env.planner_required,
        has_body_local: env.has_body_local,
        recipe_contract_present: frame.test_recipe_contract_present(),
        recipe_first_allowed: frame.test_recipe_first_allowed(),
    };
    assert!(
        !frame_trace.planner_required || frame_trace.strict_or_dev,
        "planner-required frame must imply strict/dev"
    );
    assert_eq!(
        frame_trace.has_body_local,
        facts.facts.loop_break_body_local().is_some(),
        "frame body-local flag must come from canonical facts"
    );
    assert_eq!(
        frame_trace.recipe_contract_present,
        outcome.recipe_contract.is_some(),
        "frame contract flag must come from the planner outcome"
    );
    let frame_raw_schedule = frame.test_raw_schedule().to_vec();
    let Some(witness) = frame.test_witness_if_allowed() else {
        return GenericStageTraceV1 {
            frame: frame_trace,
            raw_schedule: frame_raw_schedule,
            attempted: Vec::new(),
            generic_debts: Vec::new(),
            terminal: TerminalTraceV1::Blocked,
        };
    };
    let raw_schedule = witness.raw_schedule().to_vec();
    assert_eq!(
        raw_schedule, frame_raw_schedule,
        "witness must borrow the exact frame raw schedule"
    );
    let mut builder = seeded_builder();
    let _scope = LexicalScopeGuard::new(&mut builder);
    let mut attempted = Vec::new();
    let mut generic_debts = Vec::new();
    let result = witness.execute_selected_in_order(|_, attempt| {
        attempted.push(AttemptTraceV1 {
            route: attempt.current_route(),
            cursor: attempt.cursor(),
            suffix: attempt.exact_after_current_suffix().to_vec(),
        });
        let outcome = dispatch_entry(&mut builder, &ctx, Some(facts), attempt);
        if let Ok(RouteAttemptOutcomeV1::PostEffectRetryDebt(PostEffectRetryDebtV1::Generic(
            receipt,
        ))) = &outcome
        {
            generic_debts.push(GenericDebtTraceV1 {
                route: attempt.current_route(),
                composer: receipt.composer(),
                result: receipt.result_kind(),
            });
        }
        outcome
    });
    let terminal = match result {
        Ok(RouteExecutionResultV1::Succeeded { route, .. }) => TerminalTraceV1::Succeeded(route),
        Ok(RouteExecutionResultV1::Exhausted(_)) => TerminalTraceV1::Exhausted,
        Err(error) => TerminalTraceV1::Error(error),
    };
    assert!(
        attempted.len() <= raw_schedule.len(),
        "attempted prefix cannot exceed the captured raw schedule"
    );
    for (index, attempt) in attempted.iter().enumerate() {
        assert_eq!(
            attempt.route,
            raw_schedule[index],
            "attempted route must remain the captured raw prefix"
        );
        assert_eq!(
            attempt.cursor, index,
            "attempt cursor must remain the captured raw prefix order"
        );
        assert_eq!(
            attempt.suffix,
            raw_schedule[index + 1..],
            "attempt suffix must remain the captured raw suffix"
        );
    }
    GenericStageTraceV1 {
        frame: frame_trace,
        raw_schedule,
        attempted,
        generic_debts,
        terminal,
    }
}

fn observe_both_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        both_body(),
        "generic_stage_observer/0",
    )
}

fn observe_v1_effect_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        v1_only_effect_body(),
        "generic_stage_observer/v1-effect",
    )
}

fn observe_effect_without_local_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    observe_selected_fixture(
        mode,
        progression_condition(),
        effect_without_local_body(),
        "generic_stage_observer/effect-no-local",
    )
}

fn observe_both_evidence(mode: ObserverModeV1) -> GenericOverlapEvidenceRowV1 {
    GenericOverlapEvidenceRowV1 {
        mode,
        direct: observe_both_direct_stage(mode.strict_or_dev(), mode.planner_required()),
        witness: observe_both_fixture(mode),
    }
}

#[test]
fn generic_both_fixture_reaches_actual_entries_handler_path() {
    let trace = observe_both_fixture(ObserverModeV1::Release);

    assert_eq!(
        trace.raw_schedule,
        vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        trace.attempted.first().map(|row| row.route),
        Some(LoopRouteId::GenericLoopV0)
    );
    assert_eq!(trace.attempted.first().map(|row| row.cursor), Some(0));
    assert_eq!(
        trace.attempted.first().map(|row| row.suffix.as_slice()),
        Some([LoopRouteId::GenericLoopV1].as_slice())
    );

    // This is an observed V0 success, not a proof that Generic V0 is
    // pre-effect-qualified.  The absence of a debt-to-V1 trace keeps D3 open.
    assert!(trace.generic_debts.is_empty());
    assert_eq!(
        trace.terminal,
        TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
    );
}

#[test]
fn generic_both_fixture_records_mode_specific_witness_boundaries() {
    let modes = [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ];

    for mode in modes {
        let trace = observe_both_fixture(mode);
        let repeat = observe_both_fixture(mode);
        assert_eq!(trace, repeat, "mode-specific witness drift: {mode:?}");
        assert_eq!(
            trace.frame.strict_or_dev,
            !matches!(mode, ObserverModeV1::Release),
            "frame must capture the production strict/dev mode"
        );
        assert_eq!(
            trace.frame.planner_required,
            matches!(mode, ObserverModeV1::StrictPlannerRequired),
            "frame must capture the production planner-required mode"
        );
        assert!(
            trace.frame.recipe_first_allowed,
            "Generic Both fixture must remain recipe-first allowed"
        );

        assert!(
            !trace.raw_schedule.is_empty(),
            "Both fixture must retain a selected route in {mode:?}"
        );
        match mode {
            ObserverModeV1::Release | ObserverModeV1::Strict => assert_eq!(
                trace.raw_schedule,
                vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1],
                "release/strict Both selection must retain the V0/V1 overlap"
            ),
            ObserverModeV1::StrictPlannerRequired => assert_eq!(
                trace.raw_schedule,
                vec![LoopRouteId::GenericLoopV1],
                "planner-required selection must suppress Generic V0 before the witness"
            ),
        }
        assert_eq!(
            trace.attempted.first().map(|row| row.route),
            trace.raw_schedule.first().copied(),
            "witness must attempt the captured prefix in {mode:?}"
        );

        if trace.raw_schedule == [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1] {
            assert_eq!(
                trace.attempted,
                vec![AttemptTraceV1 {
                    route: LoopRouteId::GenericLoopV0,
                    cursor: 0,
                    suffix: vec![LoopRouteId::GenericLoopV1],
                }],
                "a V0 terminal/error must not silently continue to V1: {mode:?}"
            );
            assert!(
                trace.generic_debts.is_empty(),
                "no Generic debt receipt was observed for {mode:?}: {trace:?}"
            );
        }
    }
}

#[test]
fn generic_both_evidence_matrix_keeps_direct_stage_and_witness_separate() {
    let matrix = [
        observe_both_evidence(ObserverModeV1::Release),
        observe_both_evidence(ObserverModeV1::Strict),
        observe_both_evidence(ObserverModeV1::StrictPlannerRequired),
    ];
    for row in matrix {
        assert!(
            row.direct
                .iter()
                .all(|evidence| evidence.first_effect_owner
                    == super::generic_accepted_plan_reachability_tests::EffectOwnerV1::GenericComposer),
            "direct stage must record a Generic composer effect owner: {row:?}"
        );
        assert!(
            row.witness.generic_debts.is_empty(),
            "Both evidence must not turn absence of a debt receipt into a proof: {row:?}"
        );
        match row.mode {
            ObserverModeV1::Release | ObserverModeV1::Strict => {
                assert_eq!(
                    row.direct.iter().map(|evidence| evidence.route).collect::<Vec<_>>(),
                    vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                );
                assert_eq!(
                    row.witness.raw_schedule,
                    vec![LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
                );
                assert_eq!(
                    row.witness.terminal,
                    TerminalTraceV1::Succeeded(LoopRouteId::GenericLoopV0)
                );
            }
            ObserverModeV1::StrictPlannerRequired => {
                assert_eq!(
                    row.direct.iter().map(|evidence| evidence.route).collect::<Vec<_>>(),
                    vec![LoopRouteId::GenericLoopV1]
                );
                assert_eq!(
                    row.witness.raw_schedule,
                    vec![LoopRouteId::GenericLoopV1]
                );
            }
        }
    }
    // This matrix records the real pair of observations; it is not a pure
    // winner oracle.  D2-B still needs pre-effect policy equivalence or a
    // production-derived disjointness proof.
}

#[test]
fn generic_v1_effect_fixture_stops_at_actual_handler_error_without_retry() {
    for mode in [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ] {
        let trace = observe_v1_effect_fixture(mode);
        let repeat = observe_v1_effect_fixture(mode);
        assert_eq!(trace, repeat, "V1 effect witness drift: {mode:?}");
        assert_eq!(trace.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
        assert_eq!(
            trace.attempted,
            vec![AttemptTraceV1 {
                route: LoopRouteId::GenericLoopV1,
                cursor: 0,
                suffix: Vec::new(),
            }]
        );
        assert!(trace.generic_debts.is_empty());
        assert!(
            matches!(trace.terminal, TerminalTraceV1::Error(_)),
            "effect-call row must stop at the actual handler error: {trace:?}"
        );
    }
}

#[test]
fn generic_effect_without_local_fixture_is_not_both_and_stops_without_retry() {
    for mode in [
        ObserverModeV1::Release,
        ObserverModeV1::Strict,
        ObserverModeV1::StrictPlannerRequired,
    ] {
        let trace = observe_effect_without_local_fixture(mode);
        let repeat = observe_effect_without_local_fixture(mode);
        assert_eq!(trace, repeat, "effect boundary drift: {mode:?}");
        assert_eq!(trace.raw_schedule, vec![LoopRouteId::GenericLoopV1]);
        assert_eq!(trace.attempted.len(), 1);
        assert_eq!(trace.attempted[0].route, LoopRouteId::GenericLoopV1);
        assert!(trace.generic_debts.is_empty());
        assert!(
            matches!(trace.terminal, TerminalTraceV1::Error(_)),
            "effect boundary must stop at the actual handler error: {trace:?}"
        );
    }
}

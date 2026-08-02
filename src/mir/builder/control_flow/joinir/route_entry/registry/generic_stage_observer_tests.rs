//! M4-D3 test-only observation of the real Generic handler path.
//!
//! This is deliberately an observer, not a second scheduler.  It reuses the
//! source-to-selection result from the A1 Both fixture, then invokes the same
//! ENTRIES dispatch used by production through the existing witness executor.

use super::execution_witness::{
    PostEffectRetryDebtV1, RouteAttemptOutcomeV1, RouteExecutionResultV1, RouteExecutionWitnessV1,
};
use super::generic_selection_matrix_tests::{both_body, progression_condition};
use super::route_id::LoopRouteId;
use super::{dispatch_entry, select_recipe_first_routes, RouterEnv};
use crate::mir::builder::control_flow::joinir::route_entry::router::LoopRouteContext;
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
    fn env(self) -> RouterEnv {
        RouterEnv {
            strict_or_dev: !matches!(self, Self::Release),
            planner_required: matches!(self, Self::StrictPlannerRequired),
            has_body_local: false,
        }
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
    Error(String),
}

#[derive(Debug, PartialEq, Eq)]
struct GenericStageTraceV1 {
    raw_schedule: Vec<LoopRouteId>,
    attempted: Vec<AttemptTraceV1>,
    generic_debts: Vec<GenericDebtTraceV1>,
    terminal: TerminalTraceV1,
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

fn observe_both_fixture(mode: ObserverModeV1) -> GenericStageTraceV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = mode.config();
    let condition = progression_condition();
    let body = both_body();
    let ctx = LoopRouteContext::new(&condition, &body, "generic_stage_observer/0", false, false);
    let outcome = try_build_outcome(&ctx).expect("Both fixture must build facts");
    let facts = outcome
        .facts
        .as_ref()
        .expect("Both fixture must produce canonical facts");
    let raw_schedule = select_recipe_first_routes(Some(facts))
        .raw_execution_routes()
        .to_vec();
    let env = mode.env();
    let witness = RouteExecutionWitnessV1::issue(&raw_schedule, &env, false);
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
    GenericStageTraceV1 {
        raw_schedule,
        attempted,
        generic_debts,
        terminal,
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

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
use crate::mir::builder::MirBuilder;
use crate::mir::MirType;

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

fn observe_both_fixture() -> GenericStageTraceV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
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
    let env = RouterEnv {
        strict_or_dev: false,
        planner_required: false,
        has_body_local: false,
    };
    let witness = RouteExecutionWitnessV1::issue(&raw_schedule, &env, false);
    let mut builder = seeded_builder();
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
    let _config = crate::test_support::ScopedTestConfig::apply(&[
        ("HAKO_JOINIR_STRICT", None),
        ("HAKO_JOINIR_PLANNER_REQUIRED", None),
        ("NYASH_JOINIR_STRICT", None),
    ]);
    let trace = observe_both_fixture();

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

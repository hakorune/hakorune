//! BORROW-P0-ROOT-P0d: route/schedule/external-commit co-seal proof.
//!
//! This file projects existing route, schedule, and failure authorities. It
//! owns no lifecycle transition and cannot publish a module.

use super::module_finalization_candidate_p0::{
    ModuleFinalizationFailureMatrixV1, ModuleFinalizationFailureStageV1,
};
use super::module_invocation_route_matrix::{
    InvocationRootFamilyV1, InvocationRouteMatrixRowV1, InvocationRouteMatrixV1,
};
use super::module_lowering_borrow_schedule::{
    InvocationBorrowPhaseV1, InvocationBorrowRouteScopeV1, InvocationBorrowStepV1,
    ModuleLoweringBorrowScheduleV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TerminalScenarioV1 {
    DrainPreflightFailure,
    PostDrainFinalizerFailure,
    Success,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExternalCommitObservationV1 {
    route: InvocationRouteMatrixRowV1,
    scenario: TerminalScenarioV1,
    external_commit_count: usize,
    retry: bool,
}

fn route_steps<'schedule>(
    route: InvocationRouteMatrixRowV1,
    schedule: &'schedule ModuleLoweringBorrowScheduleV1,
) -> Vec<&'schedule InvocationBorrowStepV1> {
    schedule
        .invocation_steps()
        .iter()
        .filter(|step| {
            route.family() == InvocationRootFamilyV1::Raw
                || step.route_scope() == InvocationBorrowRouteScopeV1::AllRoutes
        })
        .collect()
}

fn observe_external_commit(
    route: InvocationRouteMatrixRowV1,
    scenario: TerminalScenarioV1,
    schedule: &ModuleLoweringBorrowScheduleV1,
) -> ExternalCommitObservationV1 {
    let external_commit_count = match scenario {
        TerminalScenarioV1::DrainPreflightFailure => {
            assert_failure_keeps_external_state(ModuleFinalizationFailureStageV1::DrainPreflight);
            0
        }
        TerminalScenarioV1::PostDrainFinalizerFailure => {
            assert_failure_keeps_external_state(
                ModuleFinalizationFailureStageV1::PostDrainFinalize,
            );
            0
        }
        TerminalScenarioV1::Success => route_steps(route, schedule)
            .iter()
            .filter(|step| step.phase() == InvocationBorrowPhaseV1::ExternalCommit)
            .count(),
    };
    ExternalCommitObservationV1 {
        route,
        scenario,
        external_commit_count,
        retry: false,
    }
}

fn assert_failure_keeps_external_state(stage: ModuleFinalizationFailureStageV1) {
    let row = ModuleFinalizationFailureMatrixV1::rows()
        .iter()
        .find(|row| row.stage() == stage)
        .expect("terminal failure stage remains in the finalization matrix");
    assert!(row.external_publication_unchanged());
    assert!(row.retry_forbidden());
}

#[test]
fn all_nine_routes_co_seal_with_the_exact_raw_prefix_or_common_tail() {
    let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
    let routes = InvocationRouteMatrixV1::rows();
    assert_eq!(routes.len(), 9);
    assert_eq!(
        routes
            .iter()
            .filter(|row| row.family() == InvocationRootFamilyV1::Raw)
            .count(),
        4
    );

    let mut projected_route_step_count = 0;
    for route in routes {
        let steps = route_steps(*route, &schedule);
        projected_route_step_count += steps.len();
        if route.family() == InvocationRootFamilyV1::Raw {
            assert_eq!(steps.len(), 11);
            assert!(steps[..5]
                .iter()
                .all(|step| { step.route_scope() == InvocationBorrowRouteScopeV1::RawOnly }));
            assert!(steps[5..]
                .iter()
                .all(|step| { step.route_scope() == InvocationBorrowRouteScopeV1::AllRoutes }));
        } else {
            assert_eq!(steps.len(), 6);
            assert!(steps
                .iter()
                .all(|step| { step.route_scope() == InvocationBorrowRouteScopeV1::AllRoutes }));
            assert_eq!(steps[0].phase(), InvocationBorrowPhaseV1::ShellFactsSeal);
        }
    }
    assert_eq!(projected_route_step_count, 4 * 11 + 5 * 6);
}

#[test]
fn every_route_observes_external_commit_zero_on_failure_and_one_on_success() {
    let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
    let mut observations = Vec::new();
    for route in InvocationRouteMatrixV1::rows() {
        for scenario in [
            TerminalScenarioV1::DrainPreflightFailure,
            TerminalScenarioV1::PostDrainFinalizerFailure,
            TerminalScenarioV1::Success,
        ] {
            observations.push(observe_external_commit(*route, scenario, &schedule));
        }
    }
    assert_eq!(observations.len(), 9 * 3);
    for observation in observations {
        let expected = usize::from(observation.scenario == TerminalScenarioV1::Success);
        assert_eq!(observation.external_commit_count, expected);
        assert!(!observation.retry);
        assert!(!observation.route.failure().retry());
    }
}

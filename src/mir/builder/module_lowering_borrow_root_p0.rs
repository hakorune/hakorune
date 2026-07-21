//! HEADERPORT0 WIRING-I0-BORROW-P0-ROOT: whole-invocation proof.
//!
//! This test-only projection consumes the existing passive borrow schedule and
//! finalization failure matrix. It adds no lifecycle or production authority.

use super::module_finalization_candidate_p0::{
    ModuleFinalizationCandidateDispositionV1, ModuleFinalizationFailureMatrixV1,
    ModuleFinalizationFailureStageV1,
};
use super::module_lowering_borrow_schedule::{
    InvocationBorrowPhaseV1, InvocationBorrowRouteScopeV1, ModuleLoweringBorrowScheduleV1,
};

#[test]
fn exact_eleven_root_phases_split_raw_prefix_from_common_tail() {
    use InvocationBorrowPhaseV1 as Phase;
    let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
    let steps = schedule.invocation_steps();
    let expected = [
        Phase::RootBodyDrive,
        Phase::RootBodySeal,
        Phase::MainHeaderCompletion,
        Phase::RootBatchPreflight,
        Phase::RootBatchCommit,
        Phase::ShellFactsSeal,
        Phase::ShellFactsCommit,
        Phase::DrainPreflight,
        Phase::DrainCommit,
        Phase::PostDrainFinalize,
        Phase::ExternalCommit,
    ];
    assert_eq!(
        steps.iter().map(|step| step.phase()).collect::<Vec<_>>(),
        expected
    );
    assert!(steps[..5]
        .iter()
        .all(|step| step.route_scope() == InvocationBorrowRouteScopeV1::RawOnly));
    assert!(steps[5..]
        .iter()
        .all(|step| step.route_scope() == InvocationBorrowRouteScopeV1::AllRoutes));
}

#[test]
fn root_batch_shell_drain_and_external_commits_are_infallible_after_preflight() {
    use InvocationBorrowPhaseV1 as Phase;
    let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
    let steps = schedule.invocation_steps();
    for phase in [
        Phase::RootBatchCommit,
        Phase::ShellFactsCommit,
        Phase::DrainCommit,
        Phase::ExternalCommit,
    ] {
        let step = steps.iter().find(|step| step.phase() == phase).unwrap();
        assert!(!step.is_fallible());
        assert!(step.mutates_owned_state());
    }
    for phase in [
        Phase::RootBatchPreflight,
        Phase::ShellFactsSeal,
        Phase::DrainPreflight,
        Phase::PostDrainFinalize,
    ] {
        assert!(steps
            .iter()
            .find(|step| step.phase() == phase)
            .unwrap()
            .is_fallible());
    }
}

#[test]
fn every_root_failure_owner_discards_candidate_without_retry_or_publication() {
    let expected = [
        ModuleFinalizationFailureStageV1::RootCompletion,
        ModuleFinalizationFailureStageV1::RootBatchPreflight,
        ModuleFinalizationFailureStageV1::DeclarationFactsSeal,
        ModuleFinalizationFailureStageV1::DrainPreflight,
        ModuleFinalizationFailureStageV1::PostDrainFinalize,
    ];
    for stage in expected {
        let row = ModuleFinalizationFailureMatrixV1::rows()
            .iter()
            .find(|row| row.stage() == stage)
            .unwrap();
        assert_eq!(
            row.candidate(),
            ModuleFinalizationCandidateDispositionV1::InvocationCandidateDiscarded
        );
        assert!(row.external_publication_unchanged());
        assert!(row.retry_forbidden());
    }
}

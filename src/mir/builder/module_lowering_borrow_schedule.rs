//! HEADERPORT0 WIRING-I0-BORROW-S0: passive borrow schedule.
//!
//! This module names the only admitted ordering for recursive child terminals
//! and whole-module completion.  It stores no live reference or lowering
//! authority.  Production wiring remains zero until the all-route CUT0.

use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationBorrowScheduleDomainV1 {
    ChildTerminal,
    InvocationCompletion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationBorrowRouteScopeV1 {
    RawOnly,
    AllRoutes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationBorrowSurfaceV1 {
    BuilderMut,
    CollectorHeaderShared,
    CollectorShared,
    CollectorMut,
    ShellShared,
    ShellMut,
    OwnedHandoff,
    ExternalPublicationMut,
}

impl InvocationBorrowSurfaceV1 {
    fn is_live_pre_drain_loan(self) -> bool {
        matches!(
            self,
            Self::BuilderMut
                | Self::CollectorHeaderShared
                | Self::CollectorShared
                | Self::CollectorMut
                | Self::ShellShared
                | Self::ShellMut
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum InvocationBorrowPhaseV1 {
    ChildBodyDescent,
    ChildHeaderObservation,
    ChildCapturePending,
    ChildCommitPending,
    ChildParentRestore,
    RootBodyDrive,
    RootBodySeal,
    MainHeaderCompletion,
    RootBatchPreflight,
    RootBatchCommit,
    ShellFactsSeal,
    ShellFactsCommit,
    DrainPreflight,
    DrainCommit,
    PostDrainFinalize,
    ExternalCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationBorrowArtifactV1 {
    ChildBodyReady,
    ChildHeaderClosed,
    ChildPending,
    ChildCollected,
    ChildParentRestored,
    RootBodyDriven,
    CompletedRootBody,
    PendingMainDraft,
    PreparedRootBatch,
    CollectedInvocationDrafts,
    DeclarationFactsSealed,
    ShellFactsCommitted,
    PreparedDrain,
    DrainedCandidate,
    FinalizedCandidate,
    ExternalCommit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationBorrowStepV1 {
    domain: InvocationBorrowScheduleDomainV1,
    phase: InvocationBorrowPhaseV1,
    route_scope: InvocationBorrowRouteScopeV1,
    input: Option<InvocationBorrowArtifactV1>,
    output: InvocationBorrowArtifactV1,
    surfaces: &'static [InvocationBorrowSurfaceV1],
    fallible: bool,
    mutates_owned_state: bool,
}

impl InvocationBorrowStepV1 {
    pub(in crate::mir::builder) fn domain(self) -> InvocationBorrowScheduleDomainV1 {
        self.domain
    }

    pub(in crate::mir::builder) fn phase(self) -> InvocationBorrowPhaseV1 {
        self.phase
    }

    pub(in crate::mir::builder) fn route_scope(self) -> InvocationBorrowRouteScopeV1 {
        self.route_scope
    }

    pub(in crate::mir::builder) fn input(self) -> Option<InvocationBorrowArtifactV1> {
        self.input
    }

    pub(in crate::mir::builder) fn output(self) -> InvocationBorrowArtifactV1 {
        self.output
    }

    pub(in crate::mir::builder) fn surfaces(self) -> &'static [InvocationBorrowSurfaceV1] {
        self.surfaces
    }

    pub(in crate::mir::builder) fn is_fallible(self) -> bool {
        self.fallible
    }

    pub(in crate::mir::builder) fn mutates_owned_state(self) -> bool {
        self.mutates_owned_state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationBorrowScheduleErrorV1 {
    WrongCardinality,
    DuplicatePhase(InvocationBorrowPhaseV1),
    BrokenArtifactChain(InvocationBorrowPhaseV1),
    SharedHeaderOverlapsCollectorMutation(InvocationBorrowPhaseV1),
    WrongRouteScope(InvocationBorrowPhaseV1),
    BuilderLoanAfterMainPending(InvocationBorrowPhaseV1),
    LiveLoanAfterDrain(InvocationBorrowPhaseV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringBorrowScheduleV1 {
    child: Box<[InvocationBorrowStepV1]>,
    invocation: Box<[InvocationBorrowStepV1]>,
    _seal: ModuleLoweringBorrowScheduleSealV1,
}

#[derive(Debug)]
struct ModuleLoweringBorrowScheduleSealV1;

const BUILDER_MUT: &[InvocationBorrowSurfaceV1] = &[InvocationBorrowSurfaceV1::BuilderMut];
const BUILDER_TO_OWNED: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::BuilderMut,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const CHILD_HEADER_COMPLETION: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::BuilderMut,
    InvocationBorrowSurfaceV1::CollectorHeaderShared,
];
const MAIN_HEADER_COMPLETION: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::BuilderMut,
    InvocationBorrowSurfaceV1::CollectorHeaderShared,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const COLLECTOR_PREFLIGHT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::CollectorShared,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const COLLECTOR_COMMIT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::CollectorMut,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const SHELL_COMMIT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::ShellMut,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const DRAIN_PREFLIGHT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::CollectorShared,
    InvocationBorrowSurfaceV1::ShellShared,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const DRAIN_COMMIT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::CollectorMut,
    InvocationBorrowSurfaceV1::ShellMut,
    InvocationBorrowSurfaceV1::OwnedHandoff,
];
const OWNED_HANDOFF: &[InvocationBorrowSurfaceV1] = &[InvocationBorrowSurfaceV1::OwnedHandoff];
const EXTERNAL_COMMIT: &[InvocationBorrowSurfaceV1] = &[
    InvocationBorrowSurfaceV1::OwnedHandoff,
    InvocationBorrowSurfaceV1::ExternalPublicationMut,
];

impl ModuleLoweringBorrowScheduleV1 {
    pub(in crate::mir::builder) fn disconnected() -> Result<Self, InvocationBorrowScheduleErrorV1> {
        let child = child_steps();
        let invocation = invocation_steps();
        validate_schedule(&child, &invocation)?;
        Ok(Self {
            child,
            invocation,
            _seal: ModuleLoweringBorrowScheduleSealV1,
        })
    }

    pub(in crate::mir::builder) fn child_steps(&self) -> &[InvocationBorrowStepV1] {
        &self.child
    }

    pub(in crate::mir::builder) fn invocation_steps(&self) -> &[InvocationBorrowStepV1] {
        &self.invocation
    }
}

fn step(
    domain: InvocationBorrowScheduleDomainV1,
    phase: InvocationBorrowPhaseV1,
    route_scope: InvocationBorrowRouteScopeV1,
    input: Option<InvocationBorrowArtifactV1>,
    output: InvocationBorrowArtifactV1,
    surfaces: &'static [InvocationBorrowSurfaceV1],
    fallible: bool,
    mutates_owned_state: bool,
) -> InvocationBorrowStepV1 {
    InvocationBorrowStepV1 {
        domain,
        phase,
        route_scope,
        input,
        output,
        surfaces,
        fallible,
        mutates_owned_state,
    }
}

fn child_steps() -> Box<[InvocationBorrowStepV1]> {
    use InvocationBorrowArtifactV1 as Artifact;
    use InvocationBorrowPhaseV1 as Phase;
    use InvocationBorrowRouteScopeV1::RawOnly;
    use InvocationBorrowScheduleDomainV1::ChildTerminal;

    Box::new([
        step(
            ChildTerminal,
            Phase::ChildBodyDescent,
            RawOnly,
            None,
            Artifact::ChildBodyReady,
            BUILDER_MUT,
            true,
            true,
        ),
        step(
            ChildTerminal,
            Phase::ChildHeaderObservation,
            RawOnly,
            Some(Artifact::ChildBodyReady),
            Artifact::ChildHeaderClosed,
            CHILD_HEADER_COMPLETION,
            false,
            true,
        ),
        step(
            ChildTerminal,
            Phase::ChildCapturePending,
            RawOnly,
            Some(Artifact::ChildHeaderClosed),
            Artifact::ChildPending,
            BUILDER_TO_OWNED,
            true,
            true,
        ),
        step(
            ChildTerminal,
            Phase::ChildCommitPending,
            RawOnly,
            Some(Artifact::ChildPending),
            Artifact::ChildCollected,
            COLLECTOR_COMMIT,
            true,
            true,
        ),
        step(
            ChildTerminal,
            Phase::ChildParentRestore,
            RawOnly,
            Some(Artifact::ChildCollected),
            Artifact::ChildParentRestored,
            OWNED_HANDOFF,
            false,
            true,
        ),
    ])
}

fn invocation_steps() -> Box<[InvocationBorrowStepV1]> {
    use InvocationBorrowArtifactV1 as Artifact;
    use InvocationBorrowPhaseV1 as Phase;
    use InvocationBorrowRouteScopeV1::{AllRoutes, RawOnly};
    use InvocationBorrowScheduleDomainV1::InvocationCompletion;

    Box::new([
        step(
            InvocationCompletion,
            Phase::RootBodyDrive,
            RawOnly,
            None,
            Artifact::RootBodyDriven,
            BUILDER_MUT,
            true,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::RootBodySeal,
            RawOnly,
            Some(Artifact::RootBodyDriven),
            Artifact::CompletedRootBody,
            BUILDER_TO_OWNED,
            true,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::MainHeaderCompletion,
            RawOnly,
            Some(Artifact::CompletedRootBody),
            Artifact::PendingMainDraft,
            MAIN_HEADER_COMPLETION,
            true,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::RootBatchPreflight,
            RawOnly,
            Some(Artifact::PendingMainDraft),
            Artifact::PreparedRootBatch,
            COLLECTOR_PREFLIGHT,
            true,
            false,
        ),
        step(
            InvocationCompletion,
            Phase::RootBatchCommit,
            RawOnly,
            Some(Artifact::PreparedRootBatch),
            Artifact::CollectedInvocationDrafts,
            COLLECTOR_COMMIT,
            false,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::ShellFactsSeal,
            AllRoutes,
            Some(Artifact::CollectedInvocationDrafts),
            Artifact::DeclarationFactsSealed,
            OWNED_HANDOFF,
            true,
            false,
        ),
        step(
            InvocationCompletion,
            Phase::ShellFactsCommit,
            AllRoutes,
            Some(Artifact::DeclarationFactsSealed),
            Artifact::ShellFactsCommitted,
            SHELL_COMMIT,
            false,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::DrainPreflight,
            AllRoutes,
            Some(Artifact::ShellFactsCommitted),
            Artifact::PreparedDrain,
            DRAIN_PREFLIGHT,
            true,
            false,
        ),
        step(
            InvocationCompletion,
            Phase::DrainCommit,
            AllRoutes,
            Some(Artifact::PreparedDrain),
            Artifact::DrainedCandidate,
            DRAIN_COMMIT,
            false,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::PostDrainFinalize,
            AllRoutes,
            Some(Artifact::DrainedCandidate),
            Artifact::FinalizedCandidate,
            OWNED_HANDOFF,
            true,
            true,
        ),
        step(
            InvocationCompletion,
            Phase::ExternalCommit,
            AllRoutes,
            Some(Artifact::FinalizedCandidate),
            Artifact::ExternalCommit,
            EXTERNAL_COMMIT,
            false,
            true,
        ),
    ])
}

fn validate_schedule(
    child: &[InvocationBorrowStepV1],
    invocation: &[InvocationBorrowStepV1],
) -> Result<(), InvocationBorrowScheduleErrorV1> {
    if child.len() != 5 || invocation.len() != 11 {
        return Err(InvocationBorrowScheduleErrorV1::WrongCardinality);
    }

    let mut phases = BTreeSet::new();
    for row in child.iter().chain(invocation) {
        if !phases.insert(row.phase) {
            return Err(InvocationBorrowScheduleErrorV1::DuplicatePhase(row.phase));
        }
        if row
            .surfaces
            .contains(&InvocationBorrowSurfaceV1::CollectorHeaderShared)
            && row
                .surfaces
                .contains(&InvocationBorrowSurfaceV1::CollectorMut)
        {
            return Err(
                InvocationBorrowScheduleErrorV1::SharedHeaderOverlapsCollectorMutation(row.phase),
            );
        }
    }

    validate_chain(child)?;
    validate_chain(invocation)?;

    for row in child {
        if row.domain != InvocationBorrowScheduleDomainV1::ChildTerminal
            || row.route_scope != InvocationBorrowRouteScopeV1::RawOnly
        {
            return Err(InvocationBorrowScheduleErrorV1::WrongRouteScope(row.phase));
        }
    }

    let mut main_pending = false;
    let mut drain_committed = false;
    for row in invocation {
        let expected_scope = match row.phase {
            InvocationBorrowPhaseV1::RootBodyDrive
            | InvocationBorrowPhaseV1::RootBodySeal
            | InvocationBorrowPhaseV1::MainHeaderCompletion
            | InvocationBorrowPhaseV1::RootBatchPreflight
            | InvocationBorrowPhaseV1::RootBatchCommit => InvocationBorrowRouteScopeV1::RawOnly,
            _ => InvocationBorrowRouteScopeV1::AllRoutes,
        };
        if row.domain != InvocationBorrowScheduleDomainV1::InvocationCompletion
            || row.route_scope != expected_scope
        {
            return Err(InvocationBorrowScheduleErrorV1::WrongRouteScope(row.phase));
        }
        if main_pending
            && row
                .surfaces
                .contains(&InvocationBorrowSurfaceV1::BuilderMut)
        {
            return Err(InvocationBorrowScheduleErrorV1::BuilderLoanAfterMainPending(row.phase));
        }
        if drain_committed
            && row
                .surfaces
                .iter()
                .copied()
                .any(InvocationBorrowSurfaceV1::is_live_pre_drain_loan)
        {
            return Err(InvocationBorrowScheduleErrorV1::LiveLoanAfterDrain(
                row.phase,
            ));
        }
        main_pending |= row.output == InvocationBorrowArtifactV1::PendingMainDraft;
        drain_committed |= row.output == InvocationBorrowArtifactV1::DrainedCandidate;
    }

    Ok(())
}

fn validate_chain(rows: &[InvocationBorrowStepV1]) -> Result<(), InvocationBorrowScheduleErrorV1> {
    for pair in rows.windows(2) {
        if pair[1].input != Some(pair[0].output) {
            return Err(InvocationBorrowScheduleErrorV1::BrokenArtifactChain(
                pair[1].phase,
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disconnected_schedule_seals_exact_cardinality_and_domains() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        assert_eq!(schedule.child_steps().len(), 5);
        assert_eq!(schedule.invocation_steps().len(), 11);
        assert!(schedule
            .child_steps()
            .iter()
            .all(|row| row.domain() == InvocationBorrowScheduleDomainV1::ChildTerminal));
        assert!(schedule
            .invocation_steps()
            .iter()
            .all(|row| { row.domain() == InvocationBorrowScheduleDomainV1::InvocationCompletion }));
    }

    #[test]
    fn every_artifact_handoff_is_contiguous() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        for rows in [schedule.child_steps(), schedule.invocation_steps()] {
            for pair in rows.windows(2) {
                assert_eq!(pair[1].input(), Some(pair[0].output()));
            }
        }
    }

    #[test]
    fn shared_header_never_overlaps_exclusive_collector_mutation() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        for row in schedule
            .child_steps()
            .iter()
            .chain(schedule.invocation_steps())
        {
            assert!(
                !(row
                    .surfaces()
                    .contains(&InvocationBorrowSurfaceV1::CollectorHeaderShared)
                    && row
                        .surfaces()
                        .contains(&InvocationBorrowSurfaceV1::CollectorMut))
            );
        }
    }

    #[test]
    fn raw_only_main_phases_end_before_common_completion_chain() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        let rows = schedule.invocation_steps();
        assert!(rows[..5]
            .iter()
            .all(|row| row.route_scope() == InvocationBorrowRouteScopeV1::RawOnly));
        assert!(rows[5..]
            .iter()
            .all(|row| row.route_scope() == InvocationBorrowRouteScopeV1::AllRoutes));
        assert_eq!(rows[5].phase(), InvocationBorrowPhaseV1::ShellFactsSeal);
    }

    #[test]
    fn no_builder_or_invocation_state_loan_survives_drain() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        let rows = schedule.invocation_steps();
        let drain = rows
            .iter()
            .position(|row| row.phase() == InvocationBorrowPhaseV1::DrainCommit)
            .unwrap();
        for row in &rows[drain + 1..] {
            assert!(!row
                .surfaces()
                .iter()
                .copied()
                .any(InvocationBorrowSurfaceV1::is_live_pre_drain_loan));
        }
    }

    #[test]
    fn commit_mutations_are_infallible_after_preflight() {
        let schedule = ModuleLoweringBorrowScheduleV1::disconnected().unwrap();
        for phase in [
            InvocationBorrowPhaseV1::RootBatchCommit,
            InvocationBorrowPhaseV1::ShellFactsCommit,
            InvocationBorrowPhaseV1::DrainCommit,
            InvocationBorrowPhaseV1::ExternalCommit,
        ] {
            let row = schedule
                .invocation_steps()
                .iter()
                .find(|row| row.phase() == phase)
                .unwrap();
            assert!(!row.is_fallible());
            assert!(row.mutates_owned_state());
        }
    }
}

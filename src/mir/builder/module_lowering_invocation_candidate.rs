//! HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-S0: root candidate failure seam.
//!
//! This module is deliberately disconnected from every production lowering
//! root.  It gives the future invocation cutover one move-only owner for the
//! shell and draft collector, plus a typed abort proof.  The proof is about
//! the candidate boundary only; function-session parent restoration remains
//! owned by `PendingFunctionSessionCloseV1`.

use super::module_draft_collector::{CompletedDraftSignatureViewV1, ModuleDraftCollectorV1};
use super::module_lowering_invocation_state::ModuleLoweringInvocationStateV1;
use super::module_lowering_shell::ModuleLoweringShellV1;
use crate::mir::MirBuilder;

/// The failure points which are allowed to abort an unpublished invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationCandidateFailureStageV1 {
    ChildPrimary,
    ChildCleanup,
    Admission,
    RootPreflight,
    FinalVerification,
    Panic,
}

/// The only external effect an aborted disconnected candidate may report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationCandidatePublicationV1 {
    Unchanged,
}

/// Candidate retry is intentionally not a recovery route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum InvocationCandidateRetryV1 {
    Forbidden,
}

/// A compact observation of the shell/collector boundary before or after an
/// abort.  It contains no function body, Builder, fact map, or module map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationCandidateBoundarySnapshotV1 {
    collector_symbols: Box<[String]>,
    shell_published_function_count: usize,
    root_state: super::module_lowering_invocation_state::RootCompletionStateV1,
}

impl InvocationCandidateBoundarySnapshotV1 {
    fn capture(state: &ModuleLoweringInvocationStateV1) -> Self {
        let mut collector_symbols = Vec::new();
        state
            .collector()
            .visit_symbols(&mut |symbol| collector_symbols.push(symbol.to_owned()));
        Self {
            collector_symbols: collector_symbols.into_boxed_slice(),
            shell_published_function_count: state.shell().published_function_count(),
            root_state: state.root(),
        }
    }

    pub(in crate::mir::builder) fn collector_symbols(&self) -> &[String] {
        &self.collector_symbols
    }

    pub(in crate::mir::builder) fn shell_published_function_count(&self) -> usize {
        self.shell_published_function_count
    }

    pub(in crate::mir::builder) fn root_state(
        &self,
    ) -> super::module_lowering_invocation_state::RootCompletionStateV1 {
        self.root_state
    }
}

/// The proof emitted by an invocation abort.  It is observational and
/// non-authoritative: publication and retry are fixed enum values, never
/// inferred from a successful fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct InvocationCandidateAbortProofV1 {
    stage: InvocationCandidateFailureStageV1,
    before: InvocationCandidateBoundarySnapshotV1,
    after: InvocationCandidateBoundarySnapshotV1,
    publication: InvocationCandidatePublicationV1,
    retry: InvocationCandidateRetryV1,
}

impl InvocationCandidateAbortProofV1 {
    pub(in crate::mir::builder) fn stage(&self) -> InvocationCandidateFailureStageV1 {
        self.stage
    }

    pub(in crate::mir::builder) fn before(&self) -> &InvocationCandidateBoundarySnapshotV1 {
        &self.before
    }

    pub(in crate::mir::builder) fn after(&self) -> &InvocationCandidateBoundarySnapshotV1 {
        &self.after
    }

    pub(in crate::mir::builder) fn publication(&self) -> InvocationCandidatePublicationV1 {
        self.publication
    }

    pub(in crate::mir::builder) fn retry_disposition(&self) -> InvocationCandidateRetryV1 {
        self.retry
    }

    pub(in crate::mir::builder) fn boundary_unchanged(&self) -> bool {
        self.before == self.after
    }
}

/// One unpublished invocation candidate.  The shell and collector move
/// together and cannot be observed separately by an abort caller.
#[derive(Debug)]
pub(in crate::mir::builder) struct ModuleLoweringInvocationCandidateV1 {
    state: Option<ModuleLoweringInvocationStateV1>,
    baseline: InvocationCandidateBoundarySnapshotV1,
    _seal: InvocationCandidateSealV1,
}

#[derive(Debug)]
struct InvocationCandidateSealV1;

/// The retained candidate after a typed failure.  Dropping this value drops
/// both shell and collector, which is the required root failure behavior.
#[derive(Debug)]
pub(in crate::mir::builder) struct AbortedModuleLoweringInvocationCandidateV1 {
    state: ModuleLoweringInvocationStateV1,
    proof: InvocationCandidateAbortProofV1,
    _seal: AbortedInvocationCandidateSealV1,
}

#[derive(Debug)]
struct AbortedInvocationCandidateSealV1;

impl ModuleLoweringInvocationCandidateV1 {
    pub(in crate::mir::builder) fn open(
        shell: ModuleLoweringShellV1,
        collector: ModuleDraftCollectorV1,
    ) -> Self {
        let state = ModuleLoweringInvocationStateV1::new(shell, collector);
        let baseline = InvocationCandidateBoundarySnapshotV1::capture(&state);
        Self {
            state: Some(state),
            baseline,
            _seal: InvocationCandidateSealV1,
        }
    }

    pub(in crate::mir::builder) fn snapshot(&self) -> &InvocationCandidateBoundarySnapshotV1 {
        &self.baseline
    }

    /// Lend the Builder only for the active lowering closure.  The candidate
    /// never stores a Builder or a `current_module` view, so the borrow ends
    /// before any later abort or drain transition.
    pub(in crate::mir::builder) fn with_active_lowering<R>(
        &mut self,
        builder: &mut MirBuilder,
        lower: impl FnOnce(&mut MirBuilder, &mut ModuleLoweringInvocationStateV1) -> R,
    ) -> R {
        let state = self
            .state
            .as_mut()
            .expect("active invocation candidate owns one state");
        lower(builder, state)
    }

    /// Abort without publishing or retrying.  The resulting proof compares
    /// the boundary before and after the candidate lifetime; no mutation is
    /// performed by this disconnected S0 owner.
    pub(in crate::mir::builder) fn abort(
        mut self,
        stage: InvocationCandidateFailureStageV1,
    ) -> AbortedModuleLoweringInvocationCandidateV1 {
        let state = self
            .state
            .take()
            .expect("invocation candidate owns one state until abort");
        let after = InvocationCandidateBoundarySnapshotV1::capture(&state);
        AbortedModuleLoweringInvocationCandidateV1 {
            state,
            proof: InvocationCandidateAbortProofV1 {
                stage,
                before: self.baseline,
                after,
                publication: InvocationCandidatePublicationV1::Unchanged,
                retry: InvocationCandidateRetryV1::Forbidden,
            },
            _seal: AbortedInvocationCandidateSealV1,
        }
    }
}

impl AbortedModuleLoweringInvocationCandidateV1 {
    pub(in crate::mir::builder) fn proof(&self) -> &InvocationCandidateAbortProofV1 {
        &self.proof
    }

    /// Drop the failed shell/collector together with the observational proof.
    /// The proof is moved out only for the disconnected P0 route co-seal.
    pub(in crate::mir::builder) fn into_proof(self) -> InvocationCandidateAbortProofV1 {
        let Self {
            state: _,
            proof,
            _seal: _,
        } = self;
        proof
    }

    /// Consume the failed candidate.  No module or collector publication is
    /// possible after this transition.
    pub(in crate::mir::builder) fn discard(self) {
        let Self {
            state: _,
            proof: _,
            _seal: _,
        } = self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::{
        DraftPublicationPolicyV1, FunctionDraftKeyV1,
    };
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
    };

    fn shell() -> ModuleLoweringShellV1 {
        ModuleLoweringShellV1::from_empty_module(MirModule::new("candidate".into())).unwrap()
    }

    fn draft(symbol: &str) -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: symbol.into(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn collector_with_prefix() -> ModuleDraftCollectorV1 {
        let mut collector = ModuleDraftCollectorV1::default();
        collector
            .prepare_admission(
                FunctionDraftKeyV1::LegacySymbol("prefix/0".into()),
                "prefix/0".into(),
                0,
                DraftPublicationPolicyV1::LegacyReplaceWholePair,
            )
            .unwrap()
            .seal(draft("prefix/0"))
            .unwrap()
            .collect();
        collector
    }

    #[test]
    fn candidate_owns_shell_and_collector_until_abort() {
        let candidate = ModuleLoweringInvocationCandidateV1::open(shell(), collector_with_prefix());
        assert_eq!(candidate.snapshot().collector_symbols(), ["prefix/0"]);
        assert_eq!(candidate.snapshot().shell_published_function_count(), 0);
        assert_eq!(
            candidate.snapshot().root_state(),
            super::super::module_lowering_invocation_state::RootCompletionStateV1::MainPending
        );

        let aborted = candidate.abort(InvocationCandidateFailureStageV1::RootPreflight);
        assert!(aborted.proof().boundary_unchanged());
        assert_eq!(
            aborted.proof().stage(),
            InvocationCandidateFailureStageV1::RootPreflight
        );
        assert_eq!(
            aborted.proof().publication(),
            InvocationCandidatePublicationV1::Unchanged
        );
        assert_eq!(
            aborted.proof().retry_disposition(),
            InvocationCandidateRetryV1::Forbidden
        );
        aborted.discard();
    }

    #[test]
    fn builder_borrow_is_scoped_to_active_lowering_only() {
        let mut builder = crate::mir::MirBuilder::new();
        let mut candidate =
            ModuleLoweringInvocationCandidateV1::open(shell(), collector_with_prefix());
        candidate.with_active_lowering(&mut builder, |builder, state| {
            assert_eq!(state.collector().symbol_count(), 1);
            assert_eq!(builder.next_value_id().0, 0);
        });
        assert_eq!(builder.next_value_id().0, 1);
        candidate
            .abort(InvocationCandidateFailureStageV1::ChildPrimary)
            .discard();
    }

    #[test]
    fn every_failure_stage_has_the_same_no_publication_law() {
        let stages = [
            InvocationCandidateFailureStageV1::ChildPrimary,
            InvocationCandidateFailureStageV1::ChildCleanup,
            InvocationCandidateFailureStageV1::Admission,
            InvocationCandidateFailureStageV1::RootPreflight,
            InvocationCandidateFailureStageV1::FinalVerification,
            InvocationCandidateFailureStageV1::Panic,
        ];
        for stage in stages {
            let aborted =
                ModuleLoweringInvocationCandidateV1::open(shell(), collector_with_prefix())
                    .abort(stage);
            assert!(aborted.proof().boundary_unchanged());
            assert_eq!(
                aborted.proof().publication(),
                InvocationCandidatePublicationV1::Unchanged
            );
            assert_eq!(
                aborted.proof().retry_disposition(),
                InvocationCandidateRetryV1::Forbidden
            );
            aborted.discard();
        }
    }
}

//! F1 draft-seal owner and its one-shot prepare/commit boundary.
//!
//! The planner in `draft_seal.rs` owns detached projections only.  This box
//! owns the live canonical function session, so every planner failure can
//! return the exact unpublished owner and `commit` is the only operation that
//! extracts the function or restores the caller context.

use crate::box_callable::provider_admission::DynamicV2AotCallMetadataProjectionV1;
use crate::mir::a_prime_i64_physical_receipt::APrimeI64PhysicalReceiptV1;
use crate::mir::builder::calls::{
    CanonicalFunctionLoweringSessionV1, PreparedFunctionSessionCloseV1,
};
use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;
use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, NormalCatalogedBoxMethodDraftAdmissionV1,
};
use crate::mir::resolved_semantics::SourceStmtSiteV1;
use crate::mir::{BasicBlockId, MirBuilder, MirFunction};

use super::completion_consumption::ReadyFunctionCompletionV1;
use super::draft_seal::{
    FunctionDraftSealPreparationErrorV1, FunctionDraftSealProjectionErrorV1,
    FunctionDraftSealProjectionV1, PreparedFunctionDraftSealPlanV1, PreparedFunctionExitSetV1,
    PreparedFunctionExitV1, ReadyFunctionDraftSealV1,
};

/// Live unpublished function owner.  It is intentionally not `Clone` and has
/// no access path other than prepare or discard.
pub(super) struct OpenFunctionDraftSealV1<'builder> {
    session: CanonicalFunctionLoweringSessionV1<'builder>,
    ready: Option<ReadyFunctionDraftSealV1>,
}

/// All fallible work is completed before this owner is issued.  Its commit is
/// therefore an ownership-only terminal.
pub(super) struct PreparedFunctionDraftSealV1<'builder> {
    completion: ReadyFunctionCompletionV1,
    plan: PreparedFunctionDraftSealPlanV1,
    close: PreparedFunctionSessionCloseV1<'builder>,
}

pub(super) struct CompletedFunctionDraftV1 {
    draft: MirFunction,
    completion: ReadyFunctionCompletionV1,
    receipt: FunctionDraftSealReceiptV1,
}

pub(super) struct FunctionDraftSealReceiptV1 {
    pub(super) signature: super::draft_seal::PreparedFunctionSignatureV1,
    pub(super) phi: super::draft_seal::PreparedFunctionPhiClosureReceiptV1,
    pub(super) stale_fact_count: usize,
}

/// Move-only candidate metadata that crosses the DraftSeal clone boundary.
/// The values are issued by the selected physical close; this box only keeps
/// them together until the detached final draft is ready to receive them.
pub(in crate::mir::builder) struct SelectedDynamicCandidateMetadataV1 {
    receipt: APrimeI64PhysicalReceiptV1,
    projection: DynamicV2AotCallMetadataProjectionV1,
}

impl SelectedDynamicCandidateMetadataV1 {
    pub(super) fn new(
        receipt: APrimeI64PhysicalReceiptV1,
        projection: DynamicV2AotCallMetadataProjectionV1,
    ) -> Self {
        Self {
            receipt,
            projection,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        APrimeI64PhysicalReceiptV1,
        DynamicV2AotCallMetadataProjectionV1,
    ) {
        (self.receipt, self.projection)
    }
}

/// F handoff retaining the selected cataloged Box-method identity.
/// This is a collector projection of existing admission, not a new semantic
/// authority; symbol and arity remain sourced from that admission.
pub(in crate::mir::builder) struct CompletedCatalogedBoxCallableDraftV1 {
    completed: CompletedFunctionDraftV1,
    key: CanonicalSameModuleCallableKeyV1,
    physical_symbol: Box<str>,
    physical_arity: usize,
}

impl CompletedCatalogedBoxCallableDraftV1 {
    pub(in crate::mir::builder) fn from_admission(
        completed: CompletedFunctionDraftV1,
        admission: &NormalCatalogedBoxMethodDraftAdmissionV1,
    ) -> Self {
        Self {
            completed,
            key: admission.source_key().clone(),
            physical_symbol: admission.physical_symbol().into(),
            physical_arity: admission.physical_arity(),
        }
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn draft(&self) -> &MirFunction {
        self.completed.draft()
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.key
    }

    pub(in crate::mir::builder) fn into_collector_parts(
        self,
    ) -> (FunctionDraftKeyV1, String, usize, MirFunction) {
        (
            FunctionDraftKeyV1::CatalogedBoxMethod(self.key),
            self.physical_symbol.into_string(),
            self.physical_arity,
            self.completed.consume_non_authority_evidence(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionDraftSealStageV1 {
    Authority,
    Exit,
    PhiClosure,
    TypeAnalysis,
    StaleFacts,
    Signature,
    Metadata,
    Verification,
    SessionClose,
}

#[derive(Debug)]
pub(super) enum FunctionDraftSealErrorV1 {
    Exit(FunctionDraftSealPreparationErrorV1),
    Projection(FunctionDraftSealProjectionErrorV1),
    SessionClose(String),
}

pub(super) struct RejectedFunctionDraftSealV1<'builder> {
    owner: OpenFunctionDraftSealV1<'builder>,
    stage: FunctionDraftSealStageV1,
    error: FunctionDraftSealErrorV1,
}

impl<'builder> OpenFunctionDraftSealV1<'builder> {
    pub(super) fn new(
        session: CanonicalFunctionLoweringSessionV1<'builder>,
        ready: ReadyFunctionDraftSealV1,
    ) -> Self {
        Self {
            session,
            ready: Some(ready),
        }
    }

    /// Prepare every detached plan while the live session remains borrowed.
    /// No function slot, type context, or caller context is moved on failure.
    pub(super) fn prepare(
        mut self,
    ) -> Result<PreparedFunctionDraftSealV1<'builder>, RejectedFunctionDraftSealV1<'builder>> {
        let ready = self
            .ready
            .take()
            .expect("open draft-seal owner must retain one ready completion");
        let exit = match ready.prepare_exit_borrowed() {
            Ok(exit) => exit,
            Err(error) => return Err(self.reject(FunctionDraftSealStageV1::Exit, error.into())),
        };
        let completion = ready.into_completion();

        let plan_result = {
            let builder = self.session.builder_view();
            prepare_detached_plan(builder, exit)
        };
        let plan = match plan_result {
            Ok(plan) => plan,
            Err((stage, error)) => return Err(self.reject(stage, error)),
        };

        let function_name = match self.session.draft_seal_readiness() {
            Ok(name) => name,
            Err(error) => {
                return Err(self.reject(
                    FunctionDraftSealStageV1::SessionClose,
                    FunctionDraftSealErrorV1::SessionClose(error.to_string()),
                ))
            }
        };
        if self.session.builder_view().function_state.current_block != Some(exit_block(exit)) {
            return Err(self.reject(
                FunctionDraftSealStageV1::SessionClose,
                FunctionDraftSealErrorV1::SessionClose(
                    "current block does not match the prepared exit block".to_string(),
                ),
            ));
        }

        let OpenFunctionDraftSealV1 { session, ready } = self;
        debug_assert!(ready.is_none());
        let close = session.prepare_draft_seal_close_after_readiness(function_name);
        Ok(PreparedFunctionDraftSealV1 {
            completion,
            plan,
            close,
        })
    }

    /// Selected Dynamic-only entry.  The physical session must pass the
    /// canonical outer Completion site so current-block validation cannot use
    /// claim order or an ordinal repair.  The ready owner is consumed before
    /// any projection work; rejection still owns the live session and can
    /// discard it, but it cannot leak a second claim authority.
    pub(super) fn prepare_exact_two(
        self,
        outer_site: &SourceStmtSiteV1,
    ) -> Result<PreparedFunctionDraftSealV1<'builder>, RejectedFunctionDraftSealV1<'builder>> {
        self.prepare_exact_two_inner(outer_site, None)
    }

    /// Selected Dynamic handoff. The candidate metadata is installed only on
    /// the detached projection after its clone-scrubbing boundary, never on
    /// the live function before `prepare_exact_two` clones it.
    pub(super) fn prepare_exact_two_with_candidate_metadata(
        self,
        outer_site: &SourceStmtSiteV1,
        candidate: SelectedDynamicCandidateMetadataV1,
    ) -> Result<PreparedFunctionDraftSealV1<'builder>, RejectedFunctionDraftSealV1<'builder>> {
        self.prepare_exact_two_inner(outer_site, Some(candidate))
    }

    fn prepare_exact_two_inner(
        mut self,
        outer_site: &SourceStmtSiteV1,
        candidate: Option<SelectedDynamicCandidateMetadataV1>,
    ) -> Result<PreparedFunctionDraftSealV1<'builder>, RejectedFunctionDraftSealV1<'builder>> {
        let ready = self
            .ready
            .take()
            .expect("open draft-seal owner must retain one ready completion");
        let exit_plan = match ready.prepare_exact_two() {
            Ok(plan) => plan,
            Err(error) => return Err(self.reject(FunctionDraftSealStageV1::Exit, error.into())),
        };
        let Some(expected_outer_block) = exit_plan.exit_block_for_site(outer_site) else {
            return Err(self.reject(
                FunctionDraftSealStageV1::Exit,
                FunctionDraftSealErrorV1::SessionClose(
                    "exact-two outer Completion site is absent from the exit claim set".to_string(),
                ),
            ));
        };
        if self.session.builder_view().function_state.current_block != Some(expected_outer_block) {
            return Err(self.reject(
                FunctionDraftSealStageV1::SessionClose,
                FunctionDraftSealErrorV1::SessionClose(
                    "current block does not match the site-keyed outer exit claim".to_string(),
                ),
            ));
        }

        let (completion, exit_set) = exit_plan.into_parts();
        let plan_result = {
            let builder = self.session.builder_view();
            prepare_detached_plan_with_exit_set(builder, exit_set, candidate)
        };
        let plan = match plan_result {
            Ok(plan) => plan,
            Err((stage, error)) => return Err(self.reject(stage, error)),
        };
        let function_name = match self.session.draft_seal_readiness() {
            Ok(name) => name,
            Err(error) => {
                return Err(self.reject(
                    FunctionDraftSealStageV1::SessionClose,
                    FunctionDraftSealErrorV1::SessionClose(error.to_string()),
                ))
            }
        };
        let OpenFunctionDraftSealV1 { session, ready } = self;
        debug_assert!(ready.is_none());
        let close = session.prepare_draft_seal_close_after_readiness(function_name);
        Ok(PreparedFunctionDraftSealV1 {
            completion,
            plan,
            close,
        })
    }

    pub(super) fn discard_with_restoration_receipt(
        self,
    ) -> crate::mir::builder::calls::CanonicalFunctionSessionRestorationReceiptV1 {
        self.session.discard_unpublished()
    }

    pub(super) fn discard(self) {
        let _ = self.discard_with_restoration_receipt();
    }

    fn reject(
        self,
        stage: FunctionDraftSealStageV1,
        error: FunctionDraftSealErrorV1,
    ) -> RejectedFunctionDraftSealV1<'builder> {
        RejectedFunctionDraftSealV1 {
            owner: self,
            stage,
            error,
        }
    }

    #[cfg(test)]
    pub(super) fn builder(&self) -> &MirBuilder {
        self.session.builder_view()
    }

    #[cfg(test)]
    pub(super) fn builder_mut(&mut self) -> &mut MirBuilder {
        self.session.builder_view_mut_for_test()
    }

    #[cfg(test)]
    pub(super) fn ready(&self) -> &ReadyFunctionDraftSealV1 {
        self.ready
            .as_ref()
            .expect("ready completion is consumed once prepare begins")
    }
}

impl PreparedFunctionDraftSealV1<'_> {
    pub(super) fn commit(self) -> CompletedFunctionDraftV1 {
        let Self {
            completion,
            plan,
            close,
        } = self;
        let (input, receipt) = plan.into_commit_parts();
        let draft = close.commit_projected(input);
        CompletedFunctionDraftV1 {
            draft,
            completion,
            receipt,
        }
    }
}

impl CompletedFunctionDraftV1 {
    /// One-shot handoff after explicitly retiring proof-only DraftSeal
    /// evidence.  Completion and the seal receipt have already enforced the
    /// canonical checks; they are not collector/publication authority and
    /// must not disappear through an implicit destructor path.
    pub(super) fn consume_non_authority_evidence(self) -> MirFunction {
        let Self {
            draft,
            completion,
            receipt,
        } = self;
        drop(completion);
        drop(receipt);
        draft
    }

    pub(super) fn draft(&self) -> &MirFunction {
        &self.draft
    }
}

impl RejectedFunctionDraftSealV1<'_> {
    pub(super) fn stage(&self) -> FunctionDraftSealStageV1 {
        self.stage
    }

    pub(super) fn error(&self) -> &FunctionDraftSealErrorV1 {
        &self.error
    }

    pub(super) fn discard_with_restoration_receipt(
        self,
    ) -> crate::mir::builder::calls::CanonicalFunctionSessionRestorationReceiptV1 {
        self.owner.discard_with_restoration_receipt()
    }

    pub(super) fn discard(self) {
        let _ = self.discard_with_restoration_receipt();
    }
}

fn exit_block(exit: PreparedFunctionExitV1) -> BasicBlockId {
    match exit {
        PreparedFunctionExitV1::ExplicitValue { block, .. }
        | PreparedFunctionExitV1::ExplicitUnit { block }
        | PreparedFunctionExitV1::ImplicitUnit { block } => block,
    }
}

fn stage_for_projection_error(
    error: &FunctionDraftSealProjectionErrorV1,
) -> FunctionDraftSealStageV1 {
    match error {
        FunctionDraftSealProjectionErrorV1::PhiClosureFailed(_) => {
            FunctionDraftSealStageV1::PhiClosure
        }
        FunctionDraftSealProjectionErrorV1::TypeAnalysisFailed(_)
        | FunctionDraftSealProjectionErrorV1::ReturnValueTypeMissing { .. }
        | FunctionDraftSealProjectionErrorV1::UnknownReturnValueType { .. }
        | FunctionDraftSealProjectionErrorV1::UnsupportedReturnValueType { .. } => {
            FunctionDraftSealStageV1::TypeAnalysis
        }
        FunctionDraftSealProjectionErrorV1::StaleFacts(_) => FunctionDraftSealStageV1::StaleFacts,
        FunctionDraftSealProjectionErrorV1::MetadataContractFailed(_) => {
            FunctionDraftSealStageV1::Metadata
        }
        FunctionDraftSealProjectionErrorV1::ProjectedVerificationFailed(_)
        | FunctionDraftSealProjectionErrorV1::TypedValueVerificationFailed(_) => {
            FunctionDraftSealStageV1::Verification
        }
        FunctionDraftSealProjectionErrorV1::ExitBlockMissing { .. }
        | FunctionDraftSealProjectionErrorV1::ExitBlockAlreadyTerminated { .. }
        | FunctionDraftSealProjectionErrorV1::PinnedTextResidence(_)
        | FunctionDraftSealProjectionErrorV1::CurrentFunctionMissing
        | FunctionDraftSealProjectionErrorV1::ReturnSignatureMismatch { .. }
        | FunctionDraftSealProjectionErrorV1::ValueIdOverflow => FunctionDraftSealStageV1::Exit,
    }
}

impl From<FunctionDraftSealPreparationErrorV1> for FunctionDraftSealErrorV1 {
    fn from(error: FunctionDraftSealPreparationErrorV1) -> Self {
        Self::Exit(error)
    }
}

fn prepare_detached_plan(
    builder: &MirBuilder,
    exit: PreparedFunctionExitV1,
) -> Result<PreparedFunctionDraftSealPlanV1, (FunctionDraftSealStageV1, FunctionDraftSealErrorV1)> {
    prepare_detached_plan_with_exit_set(builder, PreparedFunctionExitSetV1::single(exit), None)
}

fn prepare_detached_plan_with_exit_set(
    builder: &MirBuilder,
    exit: PreparedFunctionExitSetV1,
    candidate: Option<SelectedDynamicCandidateMetadataV1>,
) -> Result<PreparedFunctionDraftSealPlanV1, (FunctionDraftSealStageV1, FunctionDraftSealErrorV1)> {
    let projection = FunctionDraftSealProjectionV1::project_from_builder_exit_set(builder, exit)
        .map_err(|(_exit, error)| {
            (
                stage_for_projection_error(&error),
                FunctionDraftSealErrorV1::Projection(error),
            )
        })?;
    let phi = projection.prepare_phi_closure().map_err(|error| {
        (
            FunctionDraftSealStageV1::PhiClosure,
            FunctionDraftSealErrorV1::Projection(error),
        )
    })?;
    let lookup = builder.current_module.as_ref().map(|module| {
        module as &dyn crate::mir::builder::function_signature_lookup::FunctionSignatureLookupV1
    });
    let type_facts = phi
        .prepare_type_facts_with_lookup(lookup)
        .map_err(|error| {
            (
                FunctionDraftSealStageV1::TypeAnalysis,
                FunctionDraftSealErrorV1::Projection(error),
            )
        })?;
    let metadata = type_facts.prepare_metadata().map_err(|error| {
        (
            stage_for_projection_error(&error),
            FunctionDraftSealErrorV1::Projection(error),
        )
    })?;
    let stale = metadata
        .prepare_stale_facts(builder)
        .map_err(|(_metadata, error)| {
            (
                FunctionDraftSealStageV1::StaleFacts,
                FunctionDraftSealErrorV1::Projection(error),
            )
        })?;
    let mut plan = stale.verify().map_err(|error| {
        (
            FunctionDraftSealStageV1::Verification,
            FunctionDraftSealErrorV1::Projection(error),
        )
    })?;
    if let Some(candidate) = candidate {
        plan = plan
            .install_selected_dynamic_candidate(candidate)
            .map_err(|error| {
                (
                    FunctionDraftSealStageV1::Metadata,
                    FunctionDraftSealErrorV1::Projection(error),
                )
            })?;
    }
    Ok(plan)
}

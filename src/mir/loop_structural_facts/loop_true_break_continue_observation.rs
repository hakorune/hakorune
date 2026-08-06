//! AST-free LoopTrue source-attempt transport.
//!
//! The compiler adapter owns source-error translation. Route policy consumes
//! only this neutral algebra and never imports compiler projection errors,
//! schedules, Recipes, Builder, or MIR authority.

use super::{LoopRootSourceBindingRejectV1, VerifiedLoopTrueBreakContinueSourceProjectionV1};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueObservationModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueSourceDeclineV1 {
    NotLoopTrueBreakContinueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueSourceUnresolvedV1 {
    SourceNavigation,
    SourceLookup,
    MissingFact,
    ExitResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    UpvarBinding,
    StructuralConflict,
    ExitTargetMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopTrueSourceAttemptOutcomeV1 {
    Candidate(VerifiedLoopTrueBreakContinueSourceProjectionV1),
    Declined(LoopTrueSourceDeclineV1),
    Unresolved(LoopTrueSourceUnresolvedV1),
    Rejected(LoopTrueSourceRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopTrueSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueSourceAttemptV1 {
    outcome: LoopTrueSourceAttemptOutcomeV1,
    identity: LoopTrueSourceIdentityV1,
    mode: Option<LoopTrueObservationModeV1>,
    coverage: LoopTrueObservationCoverageV1,
    _seal: LoopTrueSourceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopTrueSourceAttemptSealV1;

impl LoopTrueSourceIdentityV1 {
    pub(crate) fn new(
        owner: FunctionOwnerIdV1,
        function_origin: FunctionOriginV1,
        source_kind: SemanticOwnerSourceKindV1,
        site: SourceStmtSiteV1,
        frame: LoopExecutionFrameKeyV1,
    ) -> Self {
        Self {
            owner,
            function_origin,
            source_kind,
            site,
            frame,
        }
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }
}

impl VerifiedLoopTrueSourceAttemptV1 {
    pub(crate) fn new(
        outcome: LoopTrueSourceAttemptOutcomeV1,
        identity: LoopTrueSourceIdentityV1,
        mode: Option<LoopTrueObservationModeV1>,
        coverage: LoopTrueObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            mode,
            coverage,
            _seal: LoopTrueSourceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        LoopTrueSourceAttemptOutcomeV1,
        LoopTrueSourceIdentityV1,
        Option<LoopTrueObservationModeV1>,
        LoopTrueObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.mode, self.coverage)
    }

    pub(crate) fn identity(&self) -> &LoopTrueSourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> Option<LoopTrueObservationModeV1> {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> LoopTrueObservationCoverageV1 {
        self.coverage
    }
}

pub(crate) fn map_loop_true_source_binding_reject(
    reject: LoopRootSourceBindingRejectV1,
) -> LoopTrueSourceRejectV1 {
    match reject {
        LoopRootSourceBindingRejectV1::UnsupportedOwnerRoot(_) => {
            LoopTrueSourceRejectV1::SourceIdentityMismatch
        }
        LoopRootSourceBindingRejectV1::MissingFunctionBodyItem
        | LoopRootSourceBindingRejectV1::UnsupportedRoot(_)
        | LoopRootSourceBindingRejectV1::UnsupportedAncestor { .. }
        | LoopRootSourceBindingRejectV1::OrphanBodyRoot { .. } => {
            LoopTrueSourceRejectV1::StructuralConflict
        }
    }
}

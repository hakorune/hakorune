//! AST-free LoopCond source-attempt transport.
//!
//! The test-only compiler adapter maps source projection errors into this
//! neutral algebra. Route policy consumes this module without compiler, AST,
//! Recipe, Builder, MIR, or route-schedule authority.

use super::VerifiedLoopCondBreakContinueSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondObservationModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondSourceDeclineV1 {
    NotLoopCondBreakContinueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondSourceUnresolvedV1 {
    SourceNavigation,
    SourceLookup,
    MissingFact,
    ExitResolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    UpvarBinding,
    StructuralConflict,
    ExitTargetMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopCondSourceAttemptOutcomeV1 {
    Candidate(VerifiedLoopCondBreakContinueSourceProjectionV1),
    Declined(LoopCondSourceDeclineV1),
    Unresolved(LoopCondSourceUnresolvedV1),
    Rejected(LoopCondSourceRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LoopCondSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondSourceAttemptV1 {
    outcome: LoopCondSourceAttemptOutcomeV1,
    identity: LoopCondSourceIdentityV1,
    mode: Option<LoopCondObservationModeV1>,
    coverage: LoopCondObservationCoverageV1,
    _seal: LoopCondSourceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopCondSourceAttemptSealV1;

impl LoopCondSourceIdentityV1 {
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

impl VerifiedLoopCondSourceAttemptV1 {
    pub(crate) fn new(
        outcome: LoopCondSourceAttemptOutcomeV1,
        identity: LoopCondSourceIdentityV1,
        mode: Option<LoopCondObservationModeV1>,
        coverage: LoopCondObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            mode,
            coverage,
            _seal: LoopCondSourceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        LoopCondSourceAttemptOutcomeV1,
        LoopCondSourceIdentityV1,
        Option<LoopCondObservationModeV1>,
        LoopCondObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.mode, self.coverage)
    }

    pub(crate) fn identity(&self) -> &LoopCondSourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> Option<LoopCondObservationModeV1> {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> LoopCondObservationCoverageV1 {
        self.coverage
    }
}

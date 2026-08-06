//! AST-free DirectAccum source-attempt transport for the family observer.
//!
//! The compiler-side test adapter maps projection errors into these neutral
//! reasons. The policy observer consumes this product without importing
//! compiler reject enums or re-reading source syntax.

use super::selected_demand::VerifiedDirectAccumSingletonObservationV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumObservationModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumSourceDeclineV1 {
    NotDirectAccumShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumSourceUnresolvedV1 {
    SourceNavigation,
    SourceLookup,
    MissingFact,
    MissingDisjointness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    UpvarBinding,
    NonBindingTarget,
    BindingMismatch,
    StructuralConflict,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumSourceAttemptOutcomeV1 {
    Candidate(VerifiedDirectAccumSingletonObservationV1),
    Declined(DirectAccumSourceDeclineV1),
    Unresolved(DirectAccumSourceUnresolvedV1),
    Rejected(DirectAccumSourceRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DirectAccumSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumSourceAttemptV1 {
    outcome: DirectAccumSourceAttemptOutcomeV1,
    identity: DirectAccumSourceIdentityV1,
    mode: Option<DirectAccumObservationModeV1>,
    coverage: DirectAccumObservationCoverageV1,
    _seal: DirectAccumSourceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectAccumSourceAttemptSealV1;

impl DirectAccumSourceIdentityV1 {
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

impl VerifiedDirectAccumSourceAttemptV1 {
    pub(crate) fn new(
        outcome: DirectAccumSourceAttemptOutcomeV1,
        identity: DirectAccumSourceIdentityV1,
        mode: Option<DirectAccumObservationModeV1>,
        coverage: DirectAccumObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            mode,
            coverage,
            _seal: DirectAccumSourceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        DirectAccumSourceAttemptOutcomeV1,
        DirectAccumSourceIdentityV1,
        Option<DirectAccumObservationModeV1>,
        DirectAccumObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.mode, self.coverage)
    }

    pub(crate) fn identity(&self) -> &DirectAccumSourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> Option<DirectAccumObservationModeV1> {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> DirectAccumObservationCoverageV1 {
        self.coverage
    }
}

//! AST-free NestedPredicate source-attempt transport.
//!
//! The test-only compiler adapter maps source projector errors into this
//! neutral algebra. The route policy consumes this module without compiler,
//! AST, Recipe, Builder, MIR, or route-schedule authority.

use super::VerifiedNestedLoopSourceProjectionV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateObservationModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateSourceDeclineV1 {
    NotNestedPredicateShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateSourceUnresolvedV1 {
    SourceNavigation,
    SourceLookup,
    MissingFact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateSourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    UpvarBinding,
    NonBindingTarget,
    BindingMismatch,
    LexicalScopeMismatch,
    StructuralConflict,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NestedPredicateSourceAttemptOutcomeV1 {
    Candidate(VerifiedNestedLoopSourceProjectionV1),
    Declined(NestedPredicateSourceDeclineV1),
    Unresolved(NestedPredicateSourceUnresolvedV1),
    Rejected(NestedPredicateSourceRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NestedPredicateSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPredicateSourceAttemptV1 {
    outcome: NestedPredicateSourceAttemptOutcomeV1,
    identity: NestedPredicateSourceIdentityV1,
    mode: Option<NestedPredicateObservationModeV1>,
    coverage: NestedPredicateObservationCoverageV1,
    _seal: NestedPredicateSourceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NestedPredicateSourceAttemptSealV1;

impl NestedPredicateSourceIdentityV1 {
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

impl VerifiedNestedPredicateSourceAttemptV1 {
    pub(crate) fn new(
        outcome: NestedPredicateSourceAttemptOutcomeV1,
        identity: NestedPredicateSourceIdentityV1,
        mode: Option<NestedPredicateObservationModeV1>,
        coverage: NestedPredicateObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            mode,
            coverage,
            _seal: NestedPredicateSourceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        NestedPredicateSourceAttemptOutcomeV1,
        NestedPredicateSourceIdentityV1,
        Option<NestedPredicateObservationModeV1>,
        NestedPredicateObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.mode, self.coverage)
    }

    pub(crate) fn identity(&self) -> &NestedPredicateSourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> Option<NestedPredicateObservationModeV1> {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> NestedPredicateObservationCoverageV1 {
        self.coverage
    }
}

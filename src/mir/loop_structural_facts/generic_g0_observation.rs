//! AST-free Generic G0 source-attempt transport.
//!
//! The compiler-side test adapter maps the existing S0A/S0B/S0C issuer
//! outcomes into this neutral C/D/U/R algebra. The transport owns only source
//! identity, mode, and coverage; it does not inspect AST, policy, Recipe,
//! Builder, MIR, route schedules, retry, or fallback.

use super::generic_g0::VerifiedGenericG0PolicyHandoffV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ObservationModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ObservationCoverageV1 {
    Complete,
    Incomplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0SourceDeclineV1 {
    NotGenericG0Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0SourceUnresolvedV1 {
    SourceNavigation,
    SourceLookup,
    MissingFact,
    TypeUnavailable,
    NumericUnavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0SourceRejectV1 {
    ForeignOwner,
    SourceIdentityMismatch,
    StructuralConflict,
    BindingConflict,
    TypeConflict,
    NumericConflict,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0SourceAttemptOutcomeV1 {
    Candidate(VerifiedGenericG0PolicyHandoffV1),
    Declined(GenericG0SourceDeclineV1),
    Unresolved(GenericG0SourceUnresolvedV1),
    Rejected(GenericG0SourceRejectV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GenericG0SourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0SourceAttemptV1 {
    outcome: GenericG0SourceAttemptOutcomeV1,
    identity: GenericG0SourceIdentityV1,
    mode: Option<GenericG0ObservationModeV1>,
    coverage: GenericG0ObservationCoverageV1,
    _seal: GenericG0SourceAttemptSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GenericG0SourceAttemptSealV1;

impl GenericG0SourceIdentityV1 {
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

impl VerifiedGenericG0SourceAttemptV1 {
    pub(crate) fn new(
        outcome: GenericG0SourceAttemptOutcomeV1,
        identity: GenericG0SourceIdentityV1,
        mode: Option<GenericG0ObservationModeV1>,
        coverage: GenericG0ObservationCoverageV1,
    ) -> Self {
        Self {
            outcome,
            identity,
            mode,
            coverage,
            _seal: GenericG0SourceAttemptSealV1,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        GenericG0SourceAttemptOutcomeV1,
        GenericG0SourceIdentityV1,
        Option<GenericG0ObservationModeV1>,
        GenericG0ObservationCoverageV1,
    ) {
        (self.outcome, self.identity, self.mode, self.coverage)
    }

    pub(crate) fn identity(&self) -> &GenericG0SourceIdentityV1 {
        &self.identity
    }

    pub(crate) const fn mode(&self) -> Option<GenericG0ObservationModeV1> {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> GenericG0ObservationCoverageV1 {
        self.coverage
    }
}

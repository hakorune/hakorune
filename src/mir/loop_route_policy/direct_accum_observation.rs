//! Caller-zero DirectAccum family observation.
//!
//! This is separate from the legacy 19-route evaluator and from the future
//! family selector. It consumes one AST-free source attempt plus one sealed
//! identity/mode/coverage context and emits only a typed disposition.

use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    DirectAccumSourceAttemptOutcomeV1, DirectAccumSourceDeclineV1, DirectAccumSourceIdentityV1,
    DirectAccumSourceRejectV1, DirectAccumSourceUnresolvedV1,
    VerifiedDirectAccumSingletonObservationV1, VerifiedDirectAccumSourceAttemptV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DirectAccumObservationContextV1 {
    identity: DirectAccumSourceIdentityV1,
    mode: Option<DirectAccumObservationModeV1>,
    coverage: DirectAccumObservationCoverageV1,
    _seal: DirectAccumObservationContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DirectAccumObservationContextSealV1;

impl DirectAccumObservationContextV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        identity: DirectAccumSourceIdentityV1,
        mode: Option<DirectAccumObservationModeV1>,
        coverage: DirectAccumObservationCoverageV1,
    ) -> Self {
        Self {
            identity,
            mode,
            coverage,
            _seal: DirectAccumObservationContextSealV1,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct DirectAccumObservationEvidenceV1 {
    expected: DirectAccumObservationContextV1,
    observed_identity: DirectAccumSourceIdentityV1,
    observed_mode: Option<DirectAccumObservationModeV1>,
    observed_coverage: DirectAccumObservationCoverageV1,
}

impl DirectAccumObservationEvidenceV1 {
    fn new(
        expected: DirectAccumObservationContextV1,
        observed_identity: DirectAccumSourceIdentityV1,
        observed_mode: Option<DirectAccumObservationModeV1>,
        observed_coverage: DirectAccumObservationCoverageV1,
    ) -> Self {
        Self {
            expected,
            observed_identity,
            observed_mode,
            observed_coverage,
        }
    }

    pub(crate) const fn expected(&self) -> &DirectAccumObservationContextV1 {
        &self.expected
    }

    pub(crate) const fn observed_identity(&self) -> &DirectAccumSourceIdentityV1 {
        &self.observed_identity
    }

    pub(crate) const fn observed_mode(&self) -> Option<DirectAccumObservationModeV1> {
        self.observed_mode
    }

    pub(crate) const fn observed_coverage(&self) -> DirectAccumObservationCoverageV1 {
        self.observed_coverage
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDirectAccumFamilyCandidateV1 {
    observation: VerifiedDirectAccumSingletonObservationV1,
    evidence: DirectAccumObservationEvidenceV1,
}

impl VerifiedDirectAccumFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedDirectAccumSingletonObservationV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &DirectAccumObservationContextV1 {
        self.evidence.expected()
    }

    pub(crate) const fn evidence(&self) -> &DirectAccumObservationEvidenceV1 {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumObservationDeclineV1 {
    NotDirectAccumShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumObservationUnresolvedV1 {
    IncompleteCoverage,
    ModeUnsealed,
    Source(DirectAccumSourceUnresolvedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DirectAccumObservationRejectV1 {
    ForeignContext,
    SourceIdentityMismatch,
    FrameMismatch,
    ModeMismatch,
    CandidateIdentityMismatch,
    Source(DirectAccumSourceRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DirectAccumFamilyObservationV1 {
    Candidate(VerifiedDirectAccumFamilyCandidateV1),
    Declined {
        reason: DirectAccumObservationDeclineV1,
        evidence: DirectAccumObservationEvidenceV1,
    },
    Unresolved {
        reason: DirectAccumObservationUnresolvedV1,
        evidence: DirectAccumObservationEvidenceV1,
    },
    Rejected {
        reason: DirectAccumObservationRejectV1,
        evidence: DirectAccumObservationEvidenceV1,
    },
}

impl DirectAccumFamilyObservationV1 {
    pub(crate) const fn evidence(&self) -> &DirectAccumObservationEvidenceV1 {
        match self {
            Self::Candidate(candidate) => candidate.evidence(),
            Self::Declined { evidence, .. }
            | Self::Unresolved { evidence, .. }
            | Self::Rejected { evidence, .. } => evidence,
        }
    }
}

pub(crate) fn issue_direct_accum_family_observation_v1(
    attempt: VerifiedDirectAccumSourceAttemptV1,
    context: DirectAccumObservationContextV1,
) -> DirectAccumFamilyObservationV1 {
    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let evidence = DirectAccumObservationEvidenceV1::new(context, identity, mode, coverage);
    let attempt_mode = evidence.observed_mode();
    let attempt_coverage = evidence.observed_coverage();
    let attempt_identity = evidence.observed_identity();
    let context = evidence.expected();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return rejected(DirectAccumObservationRejectV1::ForeignContext, evidence);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return rejected(
            DirectAccumObservationRejectV1::SourceIdentityMismatch,
            evidence,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return rejected(DirectAccumObservationRejectV1::FrameMismatch, evidence);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return unresolved(DirectAccumObservationUnresolvedV1::ModeUnsealed, evidence);
    }
    if attempt_mode != context.mode() {
        return rejected(DirectAccumObservationRejectV1::ModeMismatch, evidence);
    }
    if attempt_coverage == DirectAccumObservationCoverageV1::Incomplete
        || context.coverage() == DirectAccumObservationCoverageV1::Incomplete
    {
        return unresolved(
            DirectAccumObservationUnresolvedV1::IncompleteCoverage,
            evidence,
        );
    }

    match outcome {
        DirectAccumSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != attempt_identity.owner()
                || !observation.frame_key().matches(attempt_identity.frame())
                || !observation.matches_source_identity(
                    attempt_identity.function_origin(),
                    attempt_identity.source_kind(),
                    attempt_identity.site(),
                )
            {
                return rejected(
                    DirectAccumObservationRejectV1::CandidateIdentityMismatch,
                    evidence,
                );
            }
            DirectAccumFamilyObservationV1::Candidate(VerifiedDirectAccumFamilyCandidateV1 {
                observation,
                evidence,
            })
        }
        DirectAccumSourceAttemptOutcomeV1::Declined(reason) => {
            declined(source_decline(reason), evidence)
        }
        DirectAccumSourceAttemptOutcomeV1::Unresolved(reason) => {
            unresolved(source_unresolved(reason), evidence)
        }
        DirectAccumSourceAttemptOutcomeV1::Rejected(reason) => {
            rejected(source_reject(reason), evidence)
        }
    }
}

fn declined(
    reason: DirectAccumObservationDeclineV1,
    evidence: DirectAccumObservationEvidenceV1,
) -> DirectAccumFamilyObservationV1 {
    DirectAccumFamilyObservationV1::Declined { reason, evidence }
}

fn unresolved(
    reason: DirectAccumObservationUnresolvedV1,
    evidence: DirectAccumObservationEvidenceV1,
) -> DirectAccumFamilyObservationV1 {
    DirectAccumFamilyObservationV1::Unresolved { reason, evidence }
}

fn rejected(
    reason: DirectAccumObservationRejectV1,
    evidence: DirectAccumObservationEvidenceV1,
) -> DirectAccumFamilyObservationV1 {
    DirectAccumFamilyObservationV1::Rejected { reason, evidence }
}

fn source_decline(reason: DirectAccumSourceDeclineV1) -> DirectAccumObservationDeclineV1 {
    match reason {
        DirectAccumSourceDeclineV1::NotDirectAccumShape => {
            DirectAccumObservationDeclineV1::NotDirectAccumShape
        }
    }
}

fn source_unresolved(reason: DirectAccumSourceUnresolvedV1) -> DirectAccumObservationUnresolvedV1 {
    DirectAccumObservationUnresolvedV1::Source(reason)
}

fn source_reject(reason: DirectAccumSourceRejectV1) -> DirectAccumObservationRejectV1 {
    DirectAccumObservationRejectV1::Source(reason)
}

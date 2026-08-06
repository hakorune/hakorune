//! Caller-zero LoopTrue family observation.
//!
//! This observer consumes only neutral AST-free source evidence. It does not
//! read the legacy schedule, select a family winner, issue Recipe demand, or
//! call Builder/MIR.

use crate::mir::loop_structural_facts::{
    LoopTrueObservationCoverageV1, LoopTrueObservationModeV1, LoopTrueSourceAttemptOutcomeV1,
    LoopTrueSourceDeclineV1, LoopTrueSourceIdentityV1, LoopTrueSourceRejectV1,
    LoopTrueSourceUnresolvedV1, VerifiedLoopTrueBreakContinueSourceProjectionV1,
    VerifiedLoopTrueSourceAttemptV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopTrueObservationContextV1 {
    identity: LoopTrueSourceIdentityV1,
    mode: Option<LoopTrueObservationModeV1>,
    coverage: LoopTrueObservationCoverageV1,
    _seal: LoopTrueObservationContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopTrueObservationContextSealV1;

impl LoopTrueObservationContextV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        identity: LoopTrueSourceIdentityV1,
        mode: Option<LoopTrueObservationModeV1>,
        coverage: LoopTrueObservationCoverageV1,
    ) -> Self {
        Self {
            identity,
            mode,
            coverage,
            _seal: LoopTrueObservationContextSealV1,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopTrueObservationEvidenceV1 {
    expected: LoopTrueObservationContextV1,
    observed_identity: LoopTrueSourceIdentityV1,
    observed_mode: Option<LoopTrueObservationModeV1>,
    observed_coverage: LoopTrueObservationCoverageV1,
}

impl LoopTrueObservationEvidenceV1 {
    fn new(
        expected: LoopTrueObservationContextV1,
        observed_identity: LoopTrueSourceIdentityV1,
        observed_mode: Option<LoopTrueObservationModeV1>,
        observed_coverage: LoopTrueObservationCoverageV1,
    ) -> Self {
        Self {
            expected,
            observed_identity,
            observed_mode,
            observed_coverage,
        }
    }

    pub(crate) const fn expected(&self) -> &LoopTrueObservationContextV1 {
        &self.expected
    }

    pub(crate) const fn observed_identity(&self) -> &LoopTrueSourceIdentityV1 {
        &self.observed_identity
    }

    pub(crate) const fn observed_mode(&self) -> Option<LoopTrueObservationModeV1> {
        self.observed_mode
    }

    pub(crate) const fn observed_coverage(&self) -> LoopTrueObservationCoverageV1 {
        self.observed_coverage
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopTrueFamilyCandidateV1 {
    observation: VerifiedLoopTrueBreakContinueSourceProjectionV1,
    evidence: LoopTrueObservationEvidenceV1,
}

impl VerifiedLoopTrueFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedLoopTrueBreakContinueSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &LoopTrueObservationContextV1 {
        self.evidence.expected()
    }

    pub(crate) const fn evidence(&self) -> &LoopTrueObservationEvidenceV1 {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueObservationDeclineV1 {
    NotLoopTrueBreakContinueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueObservationUnresolvedV1 {
    IncompleteCoverage,
    ModeUnsealed,
    Source(LoopTrueSourceUnresolvedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopTrueObservationRejectV1 {
    ForeignContext,
    SourceIdentityMismatch,
    FrameMismatch,
    ModeMismatch,
    CandidateIdentityMismatch,
    Source(LoopTrueSourceRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopTrueFamilyObservationV1 {
    Candidate(VerifiedLoopTrueFamilyCandidateV1),
    Declined {
        reason: LoopTrueObservationDeclineV1,
        evidence: LoopTrueObservationEvidenceV1,
    },
    Unresolved {
        reason: LoopTrueObservationUnresolvedV1,
        evidence: LoopTrueObservationEvidenceV1,
    },
    Rejected {
        reason: LoopTrueObservationRejectV1,
        evidence: LoopTrueObservationEvidenceV1,
    },
}

impl LoopTrueFamilyObservationV1 {
    pub(crate) const fn evidence(&self) -> &LoopTrueObservationEvidenceV1 {
        match self {
            Self::Candidate(candidate) => candidate.evidence(),
            Self::Declined { evidence, .. }
            | Self::Unresolved { evidence, .. }
            | Self::Rejected { evidence, .. } => evidence,
        }
    }
}

pub(crate) fn issue_loop_true_family_observation_v1(
    attempt: VerifiedLoopTrueSourceAttemptV1,
    context: LoopTrueObservationContextV1,
) -> LoopTrueFamilyObservationV1 {
    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let evidence = LoopTrueObservationEvidenceV1::new(context, identity, mode, coverage);
    let attempt_mode = evidence.observed_mode();
    let attempt_coverage = evidence.observed_coverage();
    let attempt_identity = evidence.observed_identity();
    let context = evidence.expected();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return rejected(LoopTrueObservationRejectV1::ForeignContext, evidence);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return rejected(
            LoopTrueObservationRejectV1::SourceIdentityMismatch,
            evidence,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return rejected(LoopTrueObservationRejectV1::FrameMismatch, evidence);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return unresolved(LoopTrueObservationUnresolvedV1::ModeUnsealed, evidence);
    }
    if attempt_mode != context.mode() {
        return rejected(LoopTrueObservationRejectV1::ModeMismatch, evidence);
    }
    if attempt_coverage == LoopTrueObservationCoverageV1::Incomplete
        || context.coverage() == LoopTrueObservationCoverageV1::Incomplete
    {
        return unresolved(
            LoopTrueObservationUnresolvedV1::IncompleteCoverage,
            evidence,
        );
    }

    match outcome {
        LoopTrueSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != attempt_identity.owner()
                || !observation
                    .root_frame_key()
                    .matches(attempt_identity.frame())
                || !observation.matches_source_identity(
                    attempt_identity.function_origin(),
                    attempt_identity.source_kind(),
                    attempt_identity.site(),
                )
            {
                return rejected(
                    LoopTrueObservationRejectV1::CandidateIdentityMismatch,
                    evidence,
                );
            }
            LoopTrueFamilyObservationV1::Candidate(VerifiedLoopTrueFamilyCandidateV1 {
                observation,
                evidence,
            })
        }
        LoopTrueSourceAttemptOutcomeV1::Declined(reason) => {
            declined(source_decline(reason), evidence)
        }
        LoopTrueSourceAttemptOutcomeV1::Unresolved(reason) => {
            unresolved(source_unresolved(reason), evidence)
        }
        LoopTrueSourceAttemptOutcomeV1::Rejected(reason) => {
            rejected(source_reject(reason), evidence)
        }
    }
}

fn declined(
    reason: LoopTrueObservationDeclineV1,
    evidence: LoopTrueObservationEvidenceV1,
) -> LoopTrueFamilyObservationV1 {
    LoopTrueFamilyObservationV1::Declined { reason, evidence }
}

fn unresolved(
    reason: LoopTrueObservationUnresolvedV1,
    evidence: LoopTrueObservationEvidenceV1,
) -> LoopTrueFamilyObservationV1 {
    LoopTrueFamilyObservationV1::Unresolved { reason, evidence }
}

fn rejected(
    reason: LoopTrueObservationRejectV1,
    evidence: LoopTrueObservationEvidenceV1,
) -> LoopTrueFamilyObservationV1 {
    LoopTrueFamilyObservationV1::Rejected { reason, evidence }
}

fn source_decline(reason: LoopTrueSourceDeclineV1) -> LoopTrueObservationDeclineV1 {
    match reason {
        LoopTrueSourceDeclineV1::NotLoopTrueBreakContinueShape => {
            LoopTrueObservationDeclineV1::NotLoopTrueBreakContinueShape
        }
    }
}

fn source_unresolved(reason: LoopTrueSourceUnresolvedV1) -> LoopTrueObservationUnresolvedV1 {
    LoopTrueObservationUnresolvedV1::Source(reason)
}

fn source_reject(reason: LoopTrueSourceRejectV1) -> LoopTrueObservationRejectV1 {
    LoopTrueObservationRejectV1::Source(reason)
}

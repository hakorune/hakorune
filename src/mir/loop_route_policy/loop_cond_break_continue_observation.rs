//! Caller-zero LoopCond family observation.
//!
//! This observer consumes only neutral AST-free source evidence. It does not
//! read the legacy schedule, select a family winner, issue Recipe demand, or
//! call Builder/MIR.

use crate::mir::loop_structural_facts::{
    LoopCondObservationCoverageV1, LoopCondObservationModeV1, LoopCondSourceAttemptOutcomeV1,
    LoopCondSourceDeclineV1, LoopCondSourceIdentityV1, LoopCondSourceRejectV1,
    LoopCondSourceUnresolvedV1, VerifiedLoopCondBreakContinueSourceProjectionV1,
    VerifiedLoopCondSourceAttemptV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopCondObservationContextV1 {
    identity: LoopCondSourceIdentityV1,
    mode: Option<LoopCondObservationModeV1>,
    coverage: LoopCondObservationCoverageV1,
    _seal: LoopCondObservationContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoopCondObservationContextSealV1;

impl LoopCondObservationContextV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        identity: LoopCondSourceIdentityV1,
        mode: Option<LoopCondObservationModeV1>,
        coverage: LoopCondObservationCoverageV1,
    ) -> Self {
        Self {
            identity,
            mode,
            coverage,
            _seal: LoopCondObservationContextSealV1,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LoopCondObservationEvidenceV1 {
    expected: LoopCondObservationContextV1,
    observed_identity: LoopCondSourceIdentityV1,
    observed_mode: Option<LoopCondObservationModeV1>,
    observed_coverage: LoopCondObservationCoverageV1,
}

impl LoopCondObservationEvidenceV1 {
    fn new(
        expected: LoopCondObservationContextV1,
        observed_identity: LoopCondSourceIdentityV1,
        observed_mode: Option<LoopCondObservationModeV1>,
        observed_coverage: LoopCondObservationCoverageV1,
    ) -> Self {
        Self {
            expected,
            observed_identity,
            observed_mode,
            observed_coverage,
        }
    }

    pub(crate) const fn expected(&self) -> &LoopCondObservationContextV1 {
        &self.expected
    }

    pub(crate) const fn observed_identity(&self) -> &LoopCondSourceIdentityV1 {
        &self.observed_identity
    }

    pub(crate) const fn observed_mode(&self) -> Option<LoopCondObservationModeV1> {
        self.observed_mode
    }

    pub(crate) const fn observed_coverage(&self) -> LoopCondObservationCoverageV1 {
        self.observed_coverage
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedLoopCondFamilyCandidateV1 {
    observation: VerifiedLoopCondBreakContinueSourceProjectionV1,
    evidence: LoopCondObservationEvidenceV1,
}

impl VerifiedLoopCondFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedLoopCondBreakContinueSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &LoopCondObservationContextV1 {
        self.evidence.expected()
    }

    pub(crate) const fn evidence(&self) -> &LoopCondObservationEvidenceV1 {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondObservationDeclineV1 {
    NotLoopCondBreakContinueShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondObservationUnresolvedV1 {
    IncompleteCoverage,
    ModeUnsealed,
    Source(LoopCondSourceUnresolvedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopCondObservationRejectV1 {
    ForeignContext,
    SourceIdentityMismatch,
    FrameMismatch,
    ModeMismatch,
    CandidateIdentityMismatch,
    Source(LoopCondSourceRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum LoopCondFamilyObservationV1 {
    Candidate(VerifiedLoopCondFamilyCandidateV1),
    Declined {
        reason: LoopCondObservationDeclineV1,
        evidence: LoopCondObservationEvidenceV1,
    },
    Unresolved {
        reason: LoopCondObservationUnresolvedV1,
        evidence: LoopCondObservationEvidenceV1,
    },
    Rejected {
        reason: LoopCondObservationRejectV1,
        evidence: LoopCondObservationEvidenceV1,
    },
}

impl LoopCondFamilyObservationV1 {
    pub(crate) const fn evidence(&self) -> &LoopCondObservationEvidenceV1 {
        match self {
            Self::Candidate(candidate) => candidate.evidence(),
            Self::Declined { evidence, .. }
            | Self::Unresolved { evidence, .. }
            | Self::Rejected { evidence, .. } => evidence,
        }
    }

    pub(crate) fn into_admission_row(self) -> super::family_admission::LoopFamilyObservationRowV1 {
        super::family_admission::LoopFamilyObservationRowV1::LoopCond(self)
    }
}

pub(crate) fn issue_loop_cond_family_observation_v1(
    attempt: VerifiedLoopCondSourceAttemptV1,
    context: LoopCondObservationContextV1,
) -> LoopCondFamilyObservationV1 {
    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let evidence = LoopCondObservationEvidenceV1::new(context, identity, mode, coverage);
    let attempt_mode = evidence.observed_mode();
    let attempt_coverage = evidence.observed_coverage();
    let attempt_identity = evidence.observed_identity();
    let context = evidence.expected();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return rejected(LoopCondObservationRejectV1::ForeignContext, evidence);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return rejected(
            LoopCondObservationRejectV1::SourceIdentityMismatch,
            evidence,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return rejected(LoopCondObservationRejectV1::FrameMismatch, evidence);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return unresolved(LoopCondObservationUnresolvedV1::ModeUnsealed, evidence);
    }
    if attempt_mode != context.mode() {
        return rejected(LoopCondObservationRejectV1::ModeMismatch, evidence);
    }
    if attempt_coverage == LoopCondObservationCoverageV1::Incomplete
        || context.coverage() == LoopCondObservationCoverageV1::Incomplete
    {
        return unresolved(
            LoopCondObservationUnresolvedV1::IncompleteCoverage,
            evidence,
        );
    }

    match outcome {
        LoopCondSourceAttemptOutcomeV1::Candidate(observation) => {
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
                    LoopCondObservationRejectV1::CandidateIdentityMismatch,
                    evidence,
                );
            }
            LoopCondFamilyObservationV1::Candidate(VerifiedLoopCondFamilyCandidateV1 {
                observation,
                evidence,
            })
        }
        LoopCondSourceAttemptOutcomeV1::Declined(reason) => {
            declined(source_decline(reason), evidence)
        }
        LoopCondSourceAttemptOutcomeV1::Unresolved(reason) => {
            unresolved(source_unresolved(reason), evidence)
        }
        LoopCondSourceAttemptOutcomeV1::Rejected(reason) => {
            rejected(source_reject(reason), evidence)
        }
    }
}

fn declined(
    reason: LoopCondObservationDeclineV1,
    evidence: LoopCondObservationEvidenceV1,
) -> LoopCondFamilyObservationV1 {
    LoopCondFamilyObservationV1::Declined { reason, evidence }
}

fn unresolved(
    reason: LoopCondObservationUnresolvedV1,
    evidence: LoopCondObservationEvidenceV1,
) -> LoopCondFamilyObservationV1 {
    LoopCondFamilyObservationV1::Unresolved { reason, evidence }
}

fn rejected(
    reason: LoopCondObservationRejectV1,
    evidence: LoopCondObservationEvidenceV1,
) -> LoopCondFamilyObservationV1 {
    LoopCondFamilyObservationV1::Rejected { reason, evidence }
}

fn source_decline(reason: LoopCondSourceDeclineV1) -> LoopCondObservationDeclineV1 {
    match reason {
        LoopCondSourceDeclineV1::NotLoopCondBreakContinueShape => {
            LoopCondObservationDeclineV1::NotLoopCondBreakContinueShape
        }
    }
}

fn source_unresolved(reason: LoopCondSourceUnresolvedV1) -> LoopCondObservationUnresolvedV1 {
    LoopCondObservationUnresolvedV1::Source(reason)
}

fn source_reject(reason: LoopCondSourceRejectV1) -> LoopCondObservationRejectV1 {
    LoopCondObservationRejectV1::Source(reason)
}

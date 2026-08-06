//! Caller-zero NestedPredicate family observation.
//!
//! This observer consumes only neutral AST-free source evidence. It does not
//! select a family winner, issue Recipe demand, or call Builder/MIR.

use crate::mir::loop_structural_facts::{
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceAttemptOutcomeV1, NestedPredicateSourceDeclineV1,
    NestedPredicateSourceIdentityV1, NestedPredicateSourceRejectV1,
    NestedPredicateSourceUnresolvedV1, VerifiedNestedLoopSourceProjectionV1,
    VerifiedNestedPredicateSourceAttemptV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedPredicateObservationContextV1 {
    identity: NestedPredicateSourceIdentityV1,
    mode: Option<NestedPredicateObservationModeV1>,
    coverage: NestedPredicateObservationCoverageV1,
    _seal: NestedPredicateObservationContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NestedPredicateObservationContextSealV1;

impl NestedPredicateObservationContextV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        identity: NestedPredicateSourceIdentityV1,
        mode: Option<NestedPredicateObservationModeV1>,
        coverage: NestedPredicateObservationCoverageV1,
    ) -> Self {
        Self {
            identity,
            mode,
            coverage,
            _seal: NestedPredicateObservationContextSealV1,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NestedPredicateObservationEvidenceV1 {
    expected: NestedPredicateObservationContextV1,
    observed_identity: NestedPredicateSourceIdentityV1,
    observed_mode: Option<NestedPredicateObservationModeV1>,
    observed_coverage: NestedPredicateObservationCoverageV1,
}

impl NestedPredicateObservationEvidenceV1 {
    fn new(
        expected: NestedPredicateObservationContextV1,
        observed_identity: NestedPredicateSourceIdentityV1,
        observed_mode: Option<NestedPredicateObservationModeV1>,
        observed_coverage: NestedPredicateObservationCoverageV1,
    ) -> Self {
        Self {
            expected,
            observed_identity,
            observed_mode,
            observed_coverage,
        }
    }

    pub(crate) const fn expected(&self) -> &NestedPredicateObservationContextV1 {
        &self.expected
    }

    pub(crate) const fn observed_identity(&self) -> &NestedPredicateSourceIdentityV1 {
        &self.observed_identity
    }

    pub(crate) const fn observed_mode(&self) -> Option<NestedPredicateObservationModeV1> {
        self.observed_mode
    }

    pub(crate) const fn observed_coverage(&self) -> NestedPredicateObservationCoverageV1 {
        self.observed_coverage
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedNestedPredicateFamilyCandidateV1 {
    observation: VerifiedNestedLoopSourceProjectionV1,
    evidence: NestedPredicateObservationEvidenceV1,
}

impl VerifiedNestedPredicateFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedNestedLoopSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &NestedPredicateObservationContextV1 {
        self.evidence.expected()
    }

    pub(crate) const fn evidence(&self) -> &NestedPredicateObservationEvidenceV1 {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateObservationDeclineV1 {
    NotNestedPredicateShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateObservationUnresolvedV1 {
    IncompleteCoverage,
    ModeUnsealed,
    Source(NestedPredicateSourceUnresolvedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NestedPredicateObservationRejectV1 {
    ForeignContext,
    SourceIdentityMismatch,
    FrameMismatch,
    ModeMismatch,
    CandidateIdentityMismatch,
    Source(NestedPredicateSourceRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NestedPredicateFamilyObservationV1 {
    Candidate(VerifiedNestedPredicateFamilyCandidateV1),
    Declined {
        reason: NestedPredicateObservationDeclineV1,
        evidence: NestedPredicateObservationEvidenceV1,
    },
    Unresolved {
        reason: NestedPredicateObservationUnresolvedV1,
        evidence: NestedPredicateObservationEvidenceV1,
    },
    Rejected {
        reason: NestedPredicateObservationRejectV1,
        evidence: NestedPredicateObservationEvidenceV1,
    },
}

impl NestedPredicateFamilyObservationV1 {
    pub(crate) const fn evidence(&self) -> &NestedPredicateObservationEvidenceV1 {
        match self {
            Self::Candidate(candidate) => candidate.evidence(),
            Self::Declined { evidence, .. }
            | Self::Unresolved { evidence, .. }
            | Self::Rejected { evidence, .. } => evidence,
        }
    }

    pub(crate) fn into_admission_row(
        self,
    ) -> super::family_admission::LoopFamilyObservationRowV1 {
        super::family_admission::LoopFamilyObservationRowV1::NestedPredicate(self)
    }
}

pub(crate) fn issue_nested_predicate_family_observation_v1(
    attempt: VerifiedNestedPredicateSourceAttemptV1,
    context: NestedPredicateObservationContextV1,
) -> NestedPredicateFamilyObservationV1 {
    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let evidence = NestedPredicateObservationEvidenceV1::new(context, identity, mode, coverage);
    let attempt_mode = evidence.observed_mode();
    let attempt_coverage = evidence.observed_coverage();
    let attempt_identity = evidence.observed_identity();
    let context = evidence.expected();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return rejected(NestedPredicateObservationRejectV1::ForeignContext, evidence);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return rejected(
            NestedPredicateObservationRejectV1::SourceIdentityMismatch,
            evidence,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return rejected(NestedPredicateObservationRejectV1::FrameMismatch, evidence);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return unresolved(
            NestedPredicateObservationUnresolvedV1::ModeUnsealed,
            evidence,
        );
    }
    if attempt_mode != context.mode() {
        return rejected(NestedPredicateObservationRejectV1::ModeMismatch, evidence);
    }
    if attempt_coverage == NestedPredicateObservationCoverageV1::Incomplete
        || context.coverage() == NestedPredicateObservationCoverageV1::Incomplete
    {
        return unresolved(
            NestedPredicateObservationUnresolvedV1::IncompleteCoverage,
            evidence,
        );
    }

    match outcome {
        NestedPredicateSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != attempt_identity.owner()
                || !observation
                    .root_frame_key()
                    .matches(attempt_identity.frame())
                || !observation.matches_source_identity(
                    attempt_identity.function_origin(),
                    attempt_identity.site(),
                )
            {
                return rejected(
                    NestedPredicateObservationRejectV1::CandidateIdentityMismatch,
                    evidence,
                );
            }
            NestedPredicateFamilyObservationV1::Candidate(
                VerifiedNestedPredicateFamilyCandidateV1 {
                    observation,
                    evidence,
                },
            )
        }
        NestedPredicateSourceAttemptOutcomeV1::Declined(reason) => {
            declined(source_decline(reason), evidence)
        }
        NestedPredicateSourceAttemptOutcomeV1::Unresolved(reason) => {
            unresolved(source_unresolved(reason), evidence)
        }
        NestedPredicateSourceAttemptOutcomeV1::Rejected(reason) => {
            rejected(source_reject(reason), evidence)
        }
    }
}

fn declined(
    reason: NestedPredicateObservationDeclineV1,
    evidence: NestedPredicateObservationEvidenceV1,
) -> NestedPredicateFamilyObservationV1 {
    NestedPredicateFamilyObservationV1::Declined { reason, evidence }
}

fn unresolved(
    reason: NestedPredicateObservationUnresolvedV1,
    evidence: NestedPredicateObservationEvidenceV1,
) -> NestedPredicateFamilyObservationV1 {
    NestedPredicateFamilyObservationV1::Unresolved { reason, evidence }
}

fn rejected(
    reason: NestedPredicateObservationRejectV1,
    evidence: NestedPredicateObservationEvidenceV1,
) -> NestedPredicateFamilyObservationV1 {
    NestedPredicateFamilyObservationV1::Rejected { reason, evidence }
}

fn source_decline(reason: NestedPredicateSourceDeclineV1) -> NestedPredicateObservationDeclineV1 {
    match reason {
        NestedPredicateSourceDeclineV1::NotNestedPredicateShape => {
            NestedPredicateObservationDeclineV1::NotNestedPredicateShape
        }
    }
}

fn source_unresolved(
    reason: NestedPredicateSourceUnresolvedV1,
) -> NestedPredicateObservationUnresolvedV1 {
    NestedPredicateObservationUnresolvedV1::Source(reason)
}

fn source_reject(reason: NestedPredicateSourceRejectV1) -> NestedPredicateObservationRejectV1 {
    NestedPredicateObservationRejectV1::Source(reason)
}

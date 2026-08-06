//! Caller-zero Generic G0 family-row normalization.
//!
//! This observer consumes one neutral source attempt and one identity/mode/
//! coverage context. It is the only bridge from the existing Generic policy
//! outcome to the common C/D/U/R row algebra; selector, Recipe, Builder, MIR,
//! route schedule, retry, fallback, and production callers remain closed.

use crate::mir::loop_structural_facts::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1, GenericG0SourceAttemptOutcomeV1,
    GenericG0SourceDeclineV1, GenericG0SourceIdentityV1, GenericG0SourceRejectV1,
    GenericG0SourceUnresolvedV1, VerifiedGenericG0SourceAttemptV1,
};

use super::generic_g0::{
    issue_generic_g0_candidate_v1, GenericG0CoverageV1, GenericG0PolicyContextV1,
    GenericG0PolicyModeV1, GenericG0PolicyOutcomeV1, GenericG0PolicyProfileV1,
    GenericG0PolicyRejectV1, GenericG0PolicyUnresolvedV1, VerifiedGenericFamilyObservationG0,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0ObservationContextV1 {
    identity: GenericG0SourceIdentityV1,
    mode: Option<GenericG0ObservationModeV1>,
    coverage: GenericG0ObservationCoverageV1,
    _seal: GenericG0ObservationContextSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GenericG0ObservationContextSealV1;

impl GenericG0ObservationContextV1 {
    #[cfg(test)]
    pub(crate) fn for_test(
        identity: GenericG0SourceIdentityV1,
        mode: Option<GenericG0ObservationModeV1>,
        coverage: GenericG0ObservationCoverageV1,
    ) -> Self {
        Self {
            identity,
            mode,
            coverage,
            _seal: GenericG0ObservationContextSealV1,
        }
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericG0ObservationEvidenceV1 {
    expected: GenericG0ObservationContextV1,
    observed_identity: GenericG0SourceIdentityV1,
    observed_mode: Option<GenericG0ObservationModeV1>,
    observed_coverage: GenericG0ObservationCoverageV1,
}

impl GenericG0ObservationEvidenceV1 {
    fn new(
        expected: GenericG0ObservationContextV1,
        observed_identity: GenericG0SourceIdentityV1,
        observed_mode: Option<GenericG0ObservationModeV1>,
        observed_coverage: GenericG0ObservationCoverageV1,
    ) -> Self {
        Self {
            expected,
            observed_identity,
            observed_mode,
            observed_coverage,
        }
    }

    pub(crate) const fn expected(&self) -> &GenericG0ObservationContextV1 {
        &self.expected
    }

    pub(crate) const fn observed_identity(&self) -> &GenericG0SourceIdentityV1 {
        &self.observed_identity
    }

    pub(crate) const fn observed_mode(&self) -> Option<GenericG0ObservationModeV1> {
        self.observed_mode
    }

    pub(crate) const fn observed_coverage(&self) -> GenericG0ObservationCoverageV1 {
        self.observed_coverage
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0FamilyCandidateV1 {
    observation: VerifiedGenericFamilyObservationG0,
    evidence: GenericG0ObservationEvidenceV1,
}

impl VerifiedGenericG0FamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedGenericFamilyObservationG0 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &GenericG0ObservationContextV1 {
        self.evidence.expected()
    }

    pub(crate) const fn evidence(&self) -> &GenericG0ObservationEvidenceV1 {
        &self.evidence
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ObservationDeclineV1 {
    NotGenericG0Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ObservationUnresolvedV1 {
    IncompleteCoverage,
    ModeUnsealed,
    Source(GenericG0SourceUnresolvedV1),
    Policy(GenericG0PolicyUnresolvedV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0ObservationRejectV1 {
    ForeignContext,
    SourceIdentityMismatch,
    FrameMismatch,
    ModeMismatch,
    CandidateIdentityMismatch,
    Source(GenericG0SourceRejectV1),
    Policy(GenericG0PolicyRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0FamilyObservationV1 {
    Candidate(VerifiedGenericG0FamilyCandidateV1),
    Declined {
        reason: GenericG0ObservationDeclineV1,
        evidence: GenericG0ObservationEvidenceV1,
    },
    Unresolved {
        reason: GenericG0ObservationUnresolvedV1,
        evidence: GenericG0ObservationEvidenceV1,
    },
    Rejected {
        reason: GenericG0ObservationRejectV1,
        evidence: GenericG0ObservationEvidenceV1,
    },
}

impl GenericG0FamilyObservationV1 {
    pub(crate) const fn evidence(&self) -> &GenericG0ObservationEvidenceV1 {
        match self {
            Self::Candidate(candidate) => candidate.evidence(),
            Self::Declined { evidence, .. }
            | Self::Unresolved { evidence, .. }
            | Self::Rejected { evidence, .. } => evidence,
        }
    }
}

pub(crate) fn issue_generic_g0_family_observation_v1(
    attempt: VerifiedGenericG0SourceAttemptV1,
    context: GenericG0ObservationContextV1,
) -> GenericG0FamilyObservationV1 {
    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let evidence = GenericG0ObservationEvidenceV1::new(context, identity, mode, coverage);
    let attempt_identity = evidence.observed_identity();
    let context = evidence.expected();
    let attempt_mode = evidence.observed_mode();
    let attempt_coverage = evidence.observed_coverage();
    let context_identity = context.identity();
    if attempt_identity.owner() != context_identity.owner() {
        return rejected(GenericG0ObservationRejectV1::ForeignContext, evidence);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return rejected(
            GenericG0ObservationRejectV1::SourceIdentityMismatch,
            evidence,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return rejected(GenericG0ObservationRejectV1::FrameMismatch, evidence);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return unresolved(GenericG0ObservationUnresolvedV1::ModeUnsealed, evidence);
    }
    if attempt_mode != context.mode() {
        return rejected(GenericG0ObservationRejectV1::ModeMismatch, evidence);
    }
    if attempt_coverage == GenericG0ObservationCoverageV1::Incomplete
        || context.coverage() == GenericG0ObservationCoverageV1::Incomplete
    {
        return unresolved(
            GenericG0ObservationUnresolvedV1::IncompleteCoverage,
            evidence,
        );
    }

    let Some(mode) = attempt_mode else {
        return unresolved(GenericG0ObservationUnresolvedV1::ModeUnsealed, evidence);
    };
    match outcome {
        GenericG0SourceAttemptOutcomeV1::Candidate(bundle) => {
            let structural = bundle.source().structural();
            if structural.owner() != attempt_identity.owner()
                || structural.origin() != attempt_identity.function_origin()
                || structural.source_kind() != attempt_identity.source_kind()
                || structural.root_loop() != attempt_identity.site()
                || !structural.root_frame().matches(attempt_identity.frame())
            {
                return rejected(
                    GenericG0ObservationRejectV1::CandidateIdentityMismatch,
                    evidence,
                );
            }
            let policy_context = GenericG0PolicyContextV1::from_observation(
                attempt_identity.owner(),
                GenericG0PolicyProfileV1::G0,
                policy_mode(mode),
                policy_coverage(attempt_coverage),
            );
            match issue_generic_g0_candidate_v1(bundle, policy_context) {
                GenericG0PolicyOutcomeV1::Candidate(observation) => {
                    GenericG0FamilyObservationV1::Candidate(VerifiedGenericG0FamilyCandidateV1 {
                        observation,
                        evidence,
                    })
                }
                GenericG0PolicyOutcomeV1::Unresolved(reason) => {
                    unresolved(GenericG0ObservationUnresolvedV1::Policy(reason), evidence)
                }
                GenericG0PolicyOutcomeV1::Rejected(reason) => {
                    rejected(GenericG0ObservationRejectV1::Policy(reason), evidence)
                }
            }
        }
        GenericG0SourceAttemptOutcomeV1::Declined(reason) => {
            declined(source_decline(reason), evidence)
        }
        GenericG0SourceAttemptOutcomeV1::Unresolved(reason) => {
            unresolved(source_unresolved(reason), evidence)
        }
        GenericG0SourceAttemptOutcomeV1::Rejected(reason) => {
            rejected(source_reject(reason), evidence)
        }
    }
}

fn declined(
    reason: GenericG0ObservationDeclineV1,
    evidence: GenericG0ObservationEvidenceV1,
) -> GenericG0FamilyObservationV1 {
    GenericG0FamilyObservationV1::Declined { reason, evidence }
}

fn unresolved(
    reason: GenericG0ObservationUnresolvedV1,
    evidence: GenericG0ObservationEvidenceV1,
) -> GenericG0FamilyObservationV1 {
    GenericG0FamilyObservationV1::Unresolved { reason, evidence }
}

fn rejected(
    reason: GenericG0ObservationRejectV1,
    evidence: GenericG0ObservationEvidenceV1,
) -> GenericG0FamilyObservationV1 {
    GenericG0FamilyObservationV1::Rejected { reason, evidence }
}

fn policy_mode(mode: GenericG0ObservationModeV1) -> GenericG0PolicyModeV1 {
    match mode {
        GenericG0ObservationModeV1::Release => GenericG0PolicyModeV1::Release,
        GenericG0ObservationModeV1::Strict => GenericG0PolicyModeV1::Strict,
        GenericG0ObservationModeV1::StrictPlannerRequired => {
            GenericG0PolicyModeV1::StrictPlannerRequired
        }
    }
}

fn policy_coverage(coverage: GenericG0ObservationCoverageV1) -> GenericG0CoverageV1 {
    match coverage {
        GenericG0ObservationCoverageV1::Complete => GenericG0CoverageV1::Complete,
        GenericG0ObservationCoverageV1::Incomplete => GenericG0CoverageV1::Incomplete,
    }
}

fn source_decline(reason: GenericG0SourceDeclineV1) -> GenericG0ObservationDeclineV1 {
    match reason {
        GenericG0SourceDeclineV1::NotGenericG0Shape => {
            GenericG0ObservationDeclineV1::NotGenericG0Shape
        }
    }
}

fn source_unresolved(reason: GenericG0SourceUnresolvedV1) -> GenericG0ObservationUnresolvedV1 {
    GenericG0ObservationUnresolvedV1::Source(reason)
}

fn source_reject(reason: GenericG0SourceRejectV1) -> GenericG0ObservationRejectV1 {
    GenericG0ObservationRejectV1::Source(reason)
}

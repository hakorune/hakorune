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
pub(crate) struct VerifiedNestedPredicateFamilyCandidateV1 {
    observation: VerifiedNestedLoopSourceProjectionV1,
    context: NestedPredicateObservationContextV1,
}

impl VerifiedNestedPredicateFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedNestedLoopSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &NestedPredicateObservationContextV1 {
        &self.context
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
    Declined(NestedPredicateObservationDeclineV1),
    Unresolved(NestedPredicateObservationUnresolvedV1),
    Rejected(NestedPredicateObservationRejectV1),
}

pub(crate) fn issue_nested_predicate_family_observation_v1(
    attempt: VerifiedNestedPredicateSourceAttemptV1,
    context: NestedPredicateObservationContextV1,
) -> NestedPredicateFamilyObservationV1 {
    let attempt_mode = attempt.mode();
    let attempt_coverage = attempt.coverage();
    let attempt_identity = attempt.identity();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::ForeignContext,
        );
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::SourceIdentityMismatch,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::FrameMismatch,
        );
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return NestedPredicateFamilyObservationV1::Unresolved(
            NestedPredicateObservationUnresolvedV1::ModeUnsealed,
        );
    }
    if attempt_mode != context.mode() {
        return NestedPredicateFamilyObservationV1::Rejected(
            NestedPredicateObservationRejectV1::ModeMismatch,
        );
    }
    if attempt_coverage == NestedPredicateObservationCoverageV1::Incomplete
        || context.coverage() == NestedPredicateObservationCoverageV1::Incomplete
    {
        return NestedPredicateFamilyObservationV1::Unresolved(
            NestedPredicateObservationUnresolvedV1::IncompleteCoverage,
        );
    }

    let (outcome, identity, _, _) = attempt.into_parts();
    match outcome {
        NestedPredicateSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != identity.owner()
                || !observation.root_frame_key().matches(identity.frame())
                || !observation.matches_source_identity(identity.function_origin(), identity.site())
            {
                return NestedPredicateFamilyObservationV1::Rejected(
                    NestedPredicateObservationRejectV1::CandidateIdentityMismatch,
                );
            }
            NestedPredicateFamilyObservationV1::Candidate(
                VerifiedNestedPredicateFamilyCandidateV1 {
                    observation,
                    context,
                },
            )
        }
        NestedPredicateSourceAttemptOutcomeV1::Declined(reason) => {
            NestedPredicateFamilyObservationV1::Declined(source_decline(reason))
        }
        NestedPredicateSourceAttemptOutcomeV1::Unresolved(reason) => {
            NestedPredicateFamilyObservationV1::Unresolved(source_unresolved(reason))
        }
        NestedPredicateSourceAttemptOutcomeV1::Rejected(reason) => {
            NestedPredicateFamilyObservationV1::Rejected(source_reject(reason))
        }
    }
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

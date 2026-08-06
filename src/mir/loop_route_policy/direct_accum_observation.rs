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
pub(crate) struct VerifiedDirectAccumFamilyCandidateV1 {
    observation: VerifiedDirectAccumSingletonObservationV1,
    context: DirectAccumObservationContextV1,
}

impl VerifiedDirectAccumFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedDirectAccumSingletonObservationV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &DirectAccumObservationContextV1 {
        &self.context
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
    Declined(DirectAccumObservationDeclineV1),
    Unresolved(DirectAccumObservationUnresolvedV1),
    Rejected(DirectAccumObservationRejectV1),
}

pub(crate) fn issue_direct_accum_family_observation_v1(
    attempt: VerifiedDirectAccumSourceAttemptV1,
    context: DirectAccumObservationContextV1,
) -> DirectAccumFamilyObservationV1 {
    let attempt_mode = attempt.mode();
    let attempt_coverage = attempt.coverage();
    let attempt_identity = attempt.identity();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return DirectAccumFamilyObservationV1::Rejected(
            DirectAccumObservationRejectV1::ForeignContext,
        );
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return DirectAccumFamilyObservationV1::Rejected(
            DirectAccumObservationRejectV1::SourceIdentityMismatch,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return DirectAccumFamilyObservationV1::Rejected(
            DirectAccumObservationRejectV1::FrameMismatch,
        );
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return DirectAccumFamilyObservationV1::Unresolved(
            DirectAccumObservationUnresolvedV1::ModeUnsealed,
        );
    }
    if attempt_mode != context.mode() {
        return DirectAccumFamilyObservationV1::Rejected(
            DirectAccumObservationRejectV1::ModeMismatch,
        );
    }
    if attempt_coverage == DirectAccumObservationCoverageV1::Incomplete
        || context.coverage() == DirectAccumObservationCoverageV1::Incomplete
    {
        return DirectAccumFamilyObservationV1::Unresolved(
            DirectAccumObservationUnresolvedV1::IncompleteCoverage,
        );
    }

    let (outcome, identity, _, _) = attempt.into_parts();
    match outcome {
        DirectAccumSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != identity.owner()
                || !observation.frame_key().matches(identity.frame())
                || !observation.matches_source_identity(
                    identity.function_origin(),
                    identity.source_kind(),
                    identity.site(),
                )
            {
                return DirectAccumFamilyObservationV1::Rejected(
                    DirectAccumObservationRejectV1::CandidateIdentityMismatch,
                );
            }
            DirectAccumFamilyObservationV1::Candidate(VerifiedDirectAccumFamilyCandidateV1 {
                observation,
                context,
            })
        }
        DirectAccumSourceAttemptOutcomeV1::Declined(reason) => {
            DirectAccumFamilyObservationV1::Declined(source_decline(reason))
        }
        DirectAccumSourceAttemptOutcomeV1::Unresolved(reason) => {
            DirectAccumFamilyObservationV1::Unresolved(source_unresolved(reason))
        }
        DirectAccumSourceAttemptOutcomeV1::Rejected(reason) => {
            DirectAccumFamilyObservationV1::Rejected(source_reject(reason))
        }
    }
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

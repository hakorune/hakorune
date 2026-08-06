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
pub(crate) struct VerifiedLoopCondFamilyCandidateV1 {
    observation: VerifiedLoopCondBreakContinueSourceProjectionV1,
    context: LoopCondObservationContextV1,
}

impl VerifiedLoopCondFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedLoopCondBreakContinueSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &LoopCondObservationContextV1 {
        &self.context
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
    Declined(LoopCondObservationDeclineV1),
    Unresolved(LoopCondObservationUnresolvedV1),
    Rejected(LoopCondObservationRejectV1),
}

pub(crate) fn issue_loop_cond_family_observation_v1(
    attempt: VerifiedLoopCondSourceAttemptV1,
    context: LoopCondObservationContextV1,
) -> LoopCondFamilyObservationV1 {
    let attempt_mode = attempt.mode();
    let attempt_coverage = attempt.coverage();
    let attempt_identity = attempt.identity();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::ForeignContext);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return LoopCondFamilyObservationV1::Rejected(
            LoopCondObservationRejectV1::SourceIdentityMismatch,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::FrameMismatch);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return LoopCondFamilyObservationV1::Unresolved(
            LoopCondObservationUnresolvedV1::ModeUnsealed,
        );
    }
    if attempt_mode != context.mode() {
        return LoopCondFamilyObservationV1::Rejected(LoopCondObservationRejectV1::ModeMismatch);
    }
    if attempt_coverage == LoopCondObservationCoverageV1::Incomplete
        || context.coverage() == LoopCondObservationCoverageV1::Incomplete
    {
        return LoopCondFamilyObservationV1::Unresolved(
            LoopCondObservationUnresolvedV1::IncompleteCoverage,
        );
    }

    let (outcome, identity, _, _) = attempt.into_parts();
    match outcome {
        LoopCondSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != identity.owner()
                || !observation.root_frame_key().matches(identity.frame())
                || !observation.matches_source_identity(
                    identity.function_origin(),
                    identity.source_kind(),
                    identity.site(),
                )
            {
                return LoopCondFamilyObservationV1::Rejected(
                    LoopCondObservationRejectV1::CandidateIdentityMismatch,
                );
            }
            LoopCondFamilyObservationV1::Candidate(VerifiedLoopCondFamilyCandidateV1 {
                observation,
                context,
            })
        }
        LoopCondSourceAttemptOutcomeV1::Declined(reason) => {
            LoopCondFamilyObservationV1::Declined(source_decline(reason))
        }
        LoopCondSourceAttemptOutcomeV1::Unresolved(reason) => {
            LoopCondFamilyObservationV1::Unresolved(source_unresolved(reason))
        }
        LoopCondSourceAttemptOutcomeV1::Rejected(reason) => {
            LoopCondFamilyObservationV1::Rejected(source_reject(reason))
        }
    }
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

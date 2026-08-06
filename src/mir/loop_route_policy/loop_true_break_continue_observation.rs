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
pub(crate) struct VerifiedLoopTrueFamilyCandidateV1 {
    observation: VerifiedLoopTrueBreakContinueSourceProjectionV1,
    context: LoopTrueObservationContextV1,
}

impl VerifiedLoopTrueFamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedLoopTrueBreakContinueSourceProjectionV1 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &LoopTrueObservationContextV1 {
        &self.context
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
    Declined(LoopTrueObservationDeclineV1),
    Unresolved(LoopTrueObservationUnresolvedV1),
    Rejected(LoopTrueObservationRejectV1),
}

pub(crate) fn issue_loop_true_family_observation_v1(
    attempt: VerifiedLoopTrueSourceAttemptV1,
    context: LoopTrueObservationContextV1,
) -> LoopTrueFamilyObservationV1 {
    let attempt_mode = attempt.mode();
    let attempt_coverage = attempt.coverage();
    let attempt_identity = attempt.identity();
    let context_identity = context.identity();

    if attempt_identity.owner() != context_identity.owner() {
        return LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::ForeignContext);
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return LoopTrueFamilyObservationV1::Rejected(
            LoopTrueObservationRejectV1::SourceIdentityMismatch,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::FrameMismatch);
    }
    if attempt_mode.is_none() || context.mode().is_none() {
        return LoopTrueFamilyObservationV1::Unresolved(
            LoopTrueObservationUnresolvedV1::ModeUnsealed,
        );
    }
    if attempt_mode != context.mode() {
        return LoopTrueFamilyObservationV1::Rejected(LoopTrueObservationRejectV1::ModeMismatch);
    }
    if attempt_coverage == LoopTrueObservationCoverageV1::Incomplete
        || context.coverage() == LoopTrueObservationCoverageV1::Incomplete
    {
        return LoopTrueFamilyObservationV1::Unresolved(
            LoopTrueObservationUnresolvedV1::IncompleteCoverage,
        );
    }

    let (outcome, identity, _, _) = attempt.into_parts();
    match outcome {
        LoopTrueSourceAttemptOutcomeV1::Candidate(observation) => {
            if observation.owner() != identity.owner()
                || !observation.root_frame_key().matches(identity.frame())
                || !observation.matches_source_identity(
                    identity.function_origin(),
                    identity.source_kind(),
                    identity.site(),
                )
            {
                return LoopTrueFamilyObservationV1::Rejected(
                    LoopTrueObservationRejectV1::CandidateIdentityMismatch,
                );
            }
            LoopTrueFamilyObservationV1::Candidate(VerifiedLoopTrueFamilyCandidateV1 {
                observation,
                context,
            })
        }
        LoopTrueSourceAttemptOutcomeV1::Declined(reason) => {
            LoopTrueFamilyObservationV1::Declined(source_decline(reason))
        }
        LoopTrueSourceAttemptOutcomeV1::Unresolved(reason) => {
            LoopTrueFamilyObservationV1::Unresolved(source_unresolved(reason))
        }
        LoopTrueSourceAttemptOutcomeV1::Rejected(reason) => {
            LoopTrueFamilyObservationV1::Rejected(source_reject(reason))
        }
    }
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

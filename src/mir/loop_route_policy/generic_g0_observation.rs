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
pub(crate) struct VerifiedGenericG0FamilyCandidateV1 {
    observation: VerifiedGenericFamilyObservationG0,
    context: GenericG0ObservationContextV1,
}

impl VerifiedGenericG0FamilyCandidateV1 {
    pub(crate) fn observation(&self) -> &VerifiedGenericFamilyObservationG0 {
        &self.observation
    }

    pub(crate) const fn context(&self) -> &GenericG0ObservationContextV1 {
        &self.context
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
    Declined(GenericG0ObservationDeclineV1),
    Unresolved(GenericG0ObservationUnresolvedV1),
    Rejected(GenericG0ObservationRejectV1),
}

pub(crate) fn issue_generic_g0_family_observation_v1(
    attempt: VerifiedGenericG0SourceAttemptV1,
    context: GenericG0ObservationContextV1,
) -> GenericG0FamilyObservationV1 {
    let attempt_identity = attempt.identity();
    let context_identity = context.identity();
    if attempt_identity.owner() != context_identity.owner() {
        return GenericG0FamilyObservationV1::Rejected(
            GenericG0ObservationRejectV1::ForeignContext,
        );
    }
    if attempt_identity.function_origin() != context_identity.function_origin()
        || attempt_identity.source_kind() != context_identity.source_kind()
        || attempt_identity.site() != context_identity.site()
    {
        return GenericG0FamilyObservationV1::Rejected(
            GenericG0ObservationRejectV1::SourceIdentityMismatch,
        );
    }
    if !attempt_identity.frame().matches(context_identity.frame()) {
        return GenericG0FamilyObservationV1::Rejected(GenericG0ObservationRejectV1::FrameMismatch);
    }
    if attempt.mode().is_none() || context.mode().is_none() {
        return GenericG0FamilyObservationV1::Unresolved(
            GenericG0ObservationUnresolvedV1::ModeUnsealed,
        );
    }
    if attempt.mode() != context.mode() {
        return GenericG0FamilyObservationV1::Rejected(GenericG0ObservationRejectV1::ModeMismatch);
    }
    if attempt.coverage() == GenericG0ObservationCoverageV1::Incomplete
        || context.coverage() == GenericG0ObservationCoverageV1::Incomplete
    {
        return GenericG0FamilyObservationV1::Unresolved(
            GenericG0ObservationUnresolvedV1::IncompleteCoverage,
        );
    }

    let (outcome, identity, mode, coverage) = attempt.into_parts();
    let Some(mode) = mode else {
        return GenericG0FamilyObservationV1::Unresolved(
            GenericG0ObservationUnresolvedV1::ModeUnsealed,
        );
    };
    match outcome {
        GenericG0SourceAttemptOutcomeV1::Candidate(bundle) => {
            let structural = bundle.source().structural();
            if structural.owner() != identity.owner()
                || structural.origin() != identity.function_origin()
                || structural.source_kind() != identity.source_kind()
                || structural.root_loop() != identity.site()
                || !structural.root_frame().matches(identity.frame())
            {
                return GenericG0FamilyObservationV1::Rejected(
                    GenericG0ObservationRejectV1::CandidateIdentityMismatch,
                );
            }
            let policy_context = GenericG0PolicyContextV1::from_observation(
                identity.owner(),
                GenericG0PolicyProfileV1::G0,
                policy_mode(mode),
                policy_coverage(coverage),
            );
            match issue_generic_g0_candidate_v1(bundle, policy_context) {
                GenericG0PolicyOutcomeV1::Candidate(observation) => {
                    GenericG0FamilyObservationV1::Candidate(VerifiedGenericG0FamilyCandidateV1 {
                        observation,
                        context,
                    })
                }
                GenericG0PolicyOutcomeV1::Unresolved(reason) => {
                    GenericG0FamilyObservationV1::Unresolved(
                        GenericG0ObservationUnresolvedV1::Policy(reason),
                    )
                }
                GenericG0PolicyOutcomeV1::Rejected(reason) => {
                    GenericG0FamilyObservationV1::Rejected(GenericG0ObservationRejectV1::Policy(
                        reason,
                    ))
                }
            }
        }
        GenericG0SourceAttemptOutcomeV1::Declined(reason) => {
            GenericG0FamilyObservationV1::Declined(source_decline(reason))
        }
        GenericG0SourceAttemptOutcomeV1::Unresolved(reason) => {
            GenericG0FamilyObservationV1::Unresolved(source_unresolved(reason))
        }
        GenericG0SourceAttemptOutcomeV1::Rejected(reason) => {
            GenericG0FamilyObservationV1::Rejected(source_reject(reason))
        }
    }
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

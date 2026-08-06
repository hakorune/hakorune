//! Test-only adapter from the DirectAccum source projector to neutral S1 input.
//!
//! This file is deliberately outside the policy module. It is the only place
//! that translates compiler projection errors into AST-free source-attempt
//! reasons; the policy observer never imports those compiler error enums.

#![cfg(test)]

use super::direct_accum_projection::{
    issue_direct_accum_facts_from_source_v1, DirectAccumProjectionRejectV1,
};
use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use crate::mir::loop_structural_facts::{
    DirectAccumObservationCoverageV1, DirectAccumObservationModeV1,
    DirectAccumSingletonObservationRejectV1, DirectAccumSourceAttemptOutcomeV1,
    DirectAccumSourceDeclineV1, DirectAccumSourceIdentityV1, DirectAccumSourceRejectV1,
    DirectAccumSourceUnresolvedV1, VerifiedDirectAccumSourceAttemptV1,
};
use crate::mir::resolved_semantics::VerifiedResolvedLoopSourceV1;

pub(crate) fn issue_direct_accum_source_attempt_for_test<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    source: VerifiedResolvedLoopSourceV1,
    mode: Option<DirectAccumObservationModeV1>,
    coverage: DirectAccumObservationCoverageV1,
) -> VerifiedDirectAccumSourceAttemptV1 {
    let identity = DirectAccumSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let outcome = match issue_direct_accum_facts_from_source_v1(input, &loop_stmt, &source) {
        Ok(facts) => match facts.into_direct_accum_singleton_observation_v1(source) {
            Ok(observation) => DirectAccumSourceAttemptOutcomeV1::Candidate(observation),
            Err(reject) => map_singleton_reject(reject),
        },
        Err(reject) => map_projection_reject(reject),
    };
    VerifiedDirectAccumSourceAttemptV1::new(outcome, identity, mode, coverage)
}

fn map_projection_reject(
    reject: DirectAccumProjectionRejectV1,
) -> DirectAccumSourceAttemptOutcomeV1 {
    match reject {
        DirectAccumProjectionRejectV1::BodyArity
        | DirectAccumProjectionRejectV1::ConditionShape
        | DirectAccumProjectionRejectV1::UpdateShape
        | DirectAccumProjectionRejectV1::StepShape
        | DirectAccumProjectionRejectV1::ConstantShape => {
            DirectAccumSourceAttemptOutcomeV1::Declined(
                DirectAccumSourceDeclineV1::NotDirectAccumShape,
            )
        }
        DirectAccumProjectionRejectV1::SourceNavigation => {
            DirectAccumSourceAttemptOutcomeV1::Unresolved(
                DirectAccumSourceUnresolvedV1::SourceNavigation,
            )
        }
        DirectAccumProjectionRejectV1::SourceLookup => {
            DirectAccumSourceAttemptOutcomeV1::Unresolved(
                DirectAccumSourceUnresolvedV1::SourceLookup,
            )
        }
        DirectAccumProjectionRejectV1::MissingBinding => {
            DirectAccumSourceAttemptOutcomeV1::Unresolved(
                DirectAccumSourceUnresolvedV1::MissingFact,
            )
        }
        DirectAccumProjectionRejectV1::ForeignOwner => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(DirectAccumSourceRejectV1::ForeignOwner)
        }
        DirectAccumProjectionRejectV1::UpvarBinding => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(DirectAccumSourceRejectV1::UpvarBinding)
        }
        DirectAccumProjectionRejectV1::NonBindingTarget => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(DirectAccumSourceRejectV1::NonBindingTarget)
        }
        DirectAccumProjectionRejectV1::BindingMismatch => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(DirectAccumSourceRejectV1::BindingMismatch)
        }
        DirectAccumProjectionRejectV1::Disjointness(_) => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(
                DirectAccumSourceRejectV1::StructuralConflict,
            )
        }
    }
}

fn map_singleton_reject(
    reject: DirectAccumSingletonObservationRejectV1,
) -> DirectAccumSourceAttemptOutcomeV1 {
    match reject {
        DirectAccumSingletonObservationRejectV1::NotDirectAccum => {
            DirectAccumSourceAttemptOutcomeV1::Declined(
                DirectAccumSourceDeclineV1::NotDirectAccumShape,
            )
        }
        DirectAccumSingletonObservationRejectV1::MissingDisjointness => {
            DirectAccumSourceAttemptOutcomeV1::Unresolved(
                DirectAccumSourceUnresolvedV1::MissingDisjointness,
            )
        }
        DirectAccumSingletonObservationRejectV1::ExecutionFrameMismatch
        | DirectAccumSingletonObservationRejectV1::FactsSourceIdentityMismatch => {
            DirectAccumSourceAttemptOutcomeV1::Rejected(
                DirectAccumSourceRejectV1::SourceIdentityMismatch,
            )
        }
    }
}

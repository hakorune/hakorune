//! Test-only adapter from LoopTrue source projection to neutral S1 input.
//!
//! This is the only compiler-side translation point. Route policy never sees
//! `LoopTrueBreakContinueProjectionRejectV1` or resolver/compiler products.

#![cfg(test)]

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::loop_true_break_continue_projection::{
    issue_loop_true_break_continue_source_projection_v1, LoopTrueBreakContinueProjectionRejectV1,
};
use crate::mir::loop_structural_facts::{
    map_loop_true_source_binding_reject, LoopTrueObservationCoverageV1, LoopTrueObservationModeV1,
    LoopTrueSourceAttemptOutcomeV1, LoopTrueSourceDeclineV1, LoopTrueSourceIdentityV1,
    LoopTrueSourceRejectV1, LoopTrueSourceUnresolvedV1, VerifiedLoopTrueSourceAttemptV1,
};
use crate::mir::resolved_semantics::VerifiedResolvedLoopSourceV1;

pub(crate) fn issue_loop_true_source_attempt_for_test<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    source: VerifiedResolvedLoopSourceV1,
    mode: Option<LoopTrueObservationModeV1>,
    coverage: LoopTrueObservationCoverageV1,
) -> VerifiedLoopTrueSourceAttemptV1 {
    let source_identity_matches = source.matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    );
    let identity = LoopTrueSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let outcome = if input.owner() != loop_stmt.owner() {
        LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::ForeignOwner)
    } else if !source_identity_matches {
        LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::SourceIdentityMismatch)
    } else {
        match issue_loop_true_break_continue_source_projection_v1(input, &loop_stmt, source) {
            Ok(projection) => LoopTrueSourceAttemptOutcomeV1::Candidate(projection),
            Err(reject) => map_projection_reject(reject),
        }
    };
    VerifiedLoopTrueSourceAttemptV1::new(outcome, identity, mode, coverage)
}

fn map_projection_reject(
    reject: LoopTrueBreakContinueProjectionRejectV1,
) -> LoopTrueSourceAttemptOutcomeV1 {
    match reject {
        LoopTrueBreakContinueProjectionRejectV1::LoopConditionShape
        | LoopTrueBreakContinueProjectionRejectV1::BodyArity
        | LoopTrueBreakContinueProjectionRejectV1::BranchShape
        | LoopTrueBreakContinueProjectionRejectV1::ExplicitElseRequired
        | LoopTrueBreakContinueProjectionRejectV1::BranchBodyArity
        | LoopTrueBreakContinueProjectionRejectV1::BranchConditionShape
        | LoopTrueBreakContinueProjectionRejectV1::ConstantShape => {
            LoopTrueSourceAttemptOutcomeV1::Declined(
                LoopTrueSourceDeclineV1::NotLoopTrueBreakContinueShape,
            )
        }
        LoopTrueBreakContinueProjectionRejectV1::SourceNavigation => {
            LoopTrueSourceAttemptOutcomeV1::Unresolved(LoopTrueSourceUnresolvedV1::SourceNavigation)
        }
        LoopTrueBreakContinueProjectionRejectV1::SourceLookup => {
            LoopTrueSourceAttemptOutcomeV1::Unresolved(LoopTrueSourceUnresolvedV1::SourceLookup)
        }
        LoopTrueBreakContinueProjectionRejectV1::MissingBinding => {
            LoopTrueSourceAttemptOutcomeV1::Unresolved(LoopTrueSourceUnresolvedV1::MissingFact)
        }
        LoopTrueBreakContinueProjectionRejectV1::ExitResolution => {
            LoopTrueSourceAttemptOutcomeV1::Unresolved(LoopTrueSourceUnresolvedV1::ExitResolution)
        }
        LoopTrueBreakContinueProjectionRejectV1::ForeignOwner => {
            LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::ForeignOwner)
        }
        LoopTrueBreakContinueProjectionRejectV1::UpvarBinding => {
            LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::UpvarBinding)
        }
        LoopTrueBreakContinueProjectionRejectV1::ExitTargetMismatch => {
            LoopTrueSourceAttemptOutcomeV1::Rejected(LoopTrueSourceRejectV1::ExitTargetMismatch)
        }
        LoopTrueBreakContinueProjectionRejectV1::SourceBinding(reject) => {
            LoopTrueSourceAttemptOutcomeV1::Rejected(map_loop_true_source_binding_reject(reject))
        }
    }
}

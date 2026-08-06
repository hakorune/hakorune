//! Test-only adapter from LoopCond source projection to neutral S1 input.
//!
//! This is the only compiler-side translation point. Route policy never sees
//! `LoopCondBreakContinueProjectionRejectV1` or resolver/compiler products.

#![cfg(test)]

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::loop_cond_break_continue_projection::{
    issue_loop_cond_break_continue_source_projection_v1, LoopCondBreakContinueProjectionRejectV1,
};
use crate::mir::loop_structural_facts::{
    LoopCondObservationCoverageV1, LoopCondObservationModeV1, LoopCondSourceAttemptOutcomeV1,
    LoopCondSourceDeclineV1, LoopCondSourceIdentityV1, LoopCondSourceRejectV1,
    LoopCondSourceUnresolvedV1, VerifiedLoopCondSourceAttemptV1,
};
use crate::mir::resolved_semantics::VerifiedResolvedLoopSourceV1;

pub(crate) fn issue_loop_cond_source_attempt_for_test<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    source: VerifiedResolvedLoopSourceV1,
    mode: Option<LoopCondObservationModeV1>,
    coverage: LoopCondObservationCoverageV1,
) -> VerifiedLoopCondSourceAttemptV1 {
    let source_identity_matches = source.matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    );
    let identity = LoopCondSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let outcome = if input.owner() != loop_stmt.owner() {
        LoopCondSourceAttemptOutcomeV1::Rejected(LoopCondSourceRejectV1::ForeignOwner)
    } else if !source_identity_matches {
        LoopCondSourceAttemptOutcomeV1::Rejected(LoopCondSourceRejectV1::SourceIdentityMismatch)
    } else {
        match issue_loop_cond_break_continue_source_projection_v1(input, &loop_stmt, source) {
            Ok(projection) => LoopCondSourceAttemptOutcomeV1::Candidate(projection),
            Err(reject) => map_projection_reject(reject),
        }
    };
    VerifiedLoopCondSourceAttemptV1::new(outcome, identity, mode, coverage)
}

fn map_projection_reject(
    reject: LoopCondBreakContinueProjectionRejectV1,
) -> LoopCondSourceAttemptOutcomeV1 {
    match reject {
        LoopCondBreakContinueProjectionRejectV1::LoopTrueCondition
        | LoopCondBreakContinueProjectionRejectV1::BodyArity
        | LoopCondBreakContinueProjectionRejectV1::BranchShape
        | LoopCondBreakContinueProjectionRejectV1::ExplicitElseRequired
        | LoopCondBreakContinueProjectionRejectV1::BranchBodyArity => {
            LoopCondSourceAttemptOutcomeV1::Declined(
                LoopCondSourceDeclineV1::NotLoopCondBreakContinueShape,
            )
        }
        LoopCondBreakContinueProjectionRejectV1::SourceNavigation => {
            LoopCondSourceAttemptOutcomeV1::Unresolved(LoopCondSourceUnresolvedV1::SourceNavigation)
        }
        LoopCondBreakContinueProjectionRejectV1::SourceLookup => {
            LoopCondSourceAttemptOutcomeV1::Unresolved(LoopCondSourceUnresolvedV1::SourceLookup)
        }
        LoopCondBreakContinueProjectionRejectV1::ExitResolution => {
            LoopCondSourceAttemptOutcomeV1::Unresolved(LoopCondSourceUnresolvedV1::ExitResolution)
        }
        LoopCondBreakContinueProjectionRejectV1::ForeignOwner => {
            LoopCondSourceAttemptOutcomeV1::Rejected(LoopCondSourceRejectV1::ForeignOwner)
        }
        LoopCondBreakContinueProjectionRejectV1::ExitTargetMismatch => {
            LoopCondSourceAttemptOutcomeV1::Rejected(LoopCondSourceRejectV1::ExitTargetMismatch)
        }
    }
}

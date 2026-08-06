//! Test-only adapter from NestedPredicate source projection to neutral S1 input.
//!
//! This is the only compiler-side translation point. The route policy never
//! imports `NestedPredicateProjectionRejectV1` or the resolver forest errors.

#![cfg(test)]

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::located::LocatedStmtV1;
use super::nested_predicate_projection::{
    issue_nested_predicate_source_projection_v1, NestedPredicateProjectionRejectV1,
};
use crate::mir::loop_structural_facts::{
    LoopRootSourceBindingRejectV1, LoopSourceForestBindingRejectV1,
    NestedPredicateObservationCoverageV1, NestedPredicateObservationModeV1,
    NestedPredicateSourceAttemptOutcomeV1, NestedPredicateSourceDeclineV1,
    NestedPredicateSourceIdentityV1, NestedPredicateSourceRejectV1,
    NestedPredicateSourceUnresolvedV1, VerifiedNestedPredicateSourceAttemptV1,
};
use crate::mir::resolved_semantics::{
    ResolvedLoopSourceForestRejectV1, VerifiedResolvedLoopSourceV1,
};

pub(crate) fn issue_nested_predicate_source_attempt_for_test<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    source: VerifiedResolvedLoopSourceV1,
    mode: Option<NestedPredicateObservationModeV1>,
    coverage: NestedPredicateObservationCoverageV1,
) -> VerifiedNestedPredicateSourceAttemptV1 {
    let source_identity_matches = source.matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    );
    let identity = NestedPredicateSourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let outcome = if !source_identity_matches {
        NestedPredicateSourceAttemptOutcomeV1::Rejected(
            NestedPredicateSourceRejectV1::SourceIdentityMismatch,
        )
    } else {
        match issue_nested_predicate_source_projection_v1(input, &loop_stmt) {
            Ok(projection) => NestedPredicateSourceAttemptOutcomeV1::Candidate(projection),
            Err(reject) => map_projection_reject(reject),
        }
    };
    VerifiedNestedPredicateSourceAttemptV1::new(outcome, identity, mode, coverage)
}

fn map_projection_reject(
    reject: NestedPredicateProjectionRejectV1,
) -> NestedPredicateSourceAttemptOutcomeV1 {
    match reject {
        NestedPredicateProjectionRejectV1::ForestLookup(reject) => map_forest_lookup_reject(reject),
        NestedPredicateProjectionRejectV1::Forest(reject) => map_forest_binding_reject(reject),
        NestedPredicateProjectionRejectV1::ForestShape
        | NestedPredicateProjectionRejectV1::RootPredicateShape
        | NestedPredicateProjectionRejectV1::ChildPredicateShape
        | NestedPredicateProjectionRejectV1::RootInitializerShape
        | NestedPredicateProjectionRejectV1::RootBodySchedule
        | NestedPredicateProjectionRejectV1::ChildBodySchedule
        | NestedPredicateProjectionRejectV1::ConstantShape => {
            NestedPredicateSourceAttemptOutcomeV1::Declined(
                NestedPredicateSourceDeclineV1::NotNestedPredicateShape,
            )
        }
        NestedPredicateProjectionRejectV1::SourceNavigation => {
            NestedPredicateSourceAttemptOutcomeV1::Unresolved(
                NestedPredicateSourceUnresolvedV1::SourceNavigation,
            )
        }
        NestedPredicateProjectionRejectV1::MissingBinding => {
            NestedPredicateSourceAttemptOutcomeV1::Unresolved(
                NestedPredicateSourceUnresolvedV1::MissingFact,
            )
        }
        NestedPredicateProjectionRejectV1::ForeignOwner => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::ForeignOwner,
            )
        }
        NestedPredicateProjectionRejectV1::UpvarBinding => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::UpvarBinding,
            )
        }
        NestedPredicateProjectionRejectV1::NonBindingTarget => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::NonBindingTarget,
            )
        }
        NestedPredicateProjectionRejectV1::BindingMismatch
        | NestedPredicateProjectionRejectV1::RootInitializerBindingMismatch => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::BindingMismatch,
            )
        }
        NestedPredicateProjectionRejectV1::LexicalScopeMismatch => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::LexicalScopeMismatch,
            )
        }
    }
}

fn map_forest_lookup_reject(
    reject: ResolvedLoopSourceForestRejectV1,
) -> NestedPredicateSourceAttemptOutcomeV1 {
    match reject {
        ResolvedLoopSourceForestRejectV1::MissingRoot(_) => {
            NestedPredicateSourceAttemptOutcomeV1::Unresolved(
                NestedPredicateSourceUnresolvedV1::SourceLookup,
            )
        }
        ResolvedLoopSourceForestRejectV1::UnsupportedOwnerRoot(_) => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::SourceIdentityMismatch,
            )
        }
        ResolvedLoopSourceForestRejectV1::DuplicateSite(_)
        | ResolvedLoopSourceForestRejectV1::OrphanDescendant(_)
        | ResolvedLoopSourceForestRejectV1::SkippedIntermediateLoop(_)
        | ResolvedLoopSourceForestRejectV1::UnsupportedAncestry { .. } => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::StructuralConflict,
            )
        }
    }
}

fn map_forest_binding_reject(
    reject: LoopSourceForestBindingRejectV1,
) -> NestedPredicateSourceAttemptOutcomeV1 {
    match reject {
        LoopSourceForestBindingRejectV1::Source { reason, .. } => match reason {
            LoopRootSourceBindingRejectV1::UnsupportedOwnerRoot(_) => {
                NestedPredicateSourceAttemptOutcomeV1::Rejected(
                    NestedPredicateSourceRejectV1::SourceIdentityMismatch,
                )
            }
            LoopRootSourceBindingRejectV1::MissingFunctionBodyItem
            | LoopRootSourceBindingRejectV1::UnsupportedRoot(_)
            | LoopRootSourceBindingRejectV1::UnsupportedAncestor { .. }
            | LoopRootSourceBindingRejectV1::OrphanBodyRoot { .. } => {
                NestedPredicateSourceAttemptOutcomeV1::Rejected(
                    NestedPredicateSourceRejectV1::StructuralConflict,
                )
            }
        },
        LoopSourceForestBindingRejectV1::SourceForestEmpty => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::StructuralConflict,
            )
        }
        LoopSourceForestBindingRejectV1::SourceForestOwnerMismatch { .. } => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::ForeignOwner,
            )
        }
        LoopSourceForestBindingRejectV1::ParentIndexOutOfRange { .. }
        | LoopSourceForestBindingRejectV1::RootParentMismatch
        | LoopSourceForestBindingRejectV1::RecipeLoopCoverageMismatch { .. }
        | LoopSourceForestBindingRejectV1::RecipeParentMismatch { .. } => {
            NestedPredicateSourceAttemptOutcomeV1::Rejected(
                NestedPredicateSourceRejectV1::StructuralConflict,
            )
        }
    }
}

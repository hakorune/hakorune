//! Test-only adapter from Generic G0 source projection to neutral S1 input.
//!
//! This is the only source-error translation point for the current caller-zero
//! row. It consumes the existing S0A/S0B/S0C products once and never enters a
//! selector, Recipe producer, Builder, MIR, retry, or fallback path.

#![cfg(test)]

use super::function_input::ResolvedFunctionLoweringInputV1;
use super::generic_g0_projection::handoff::{
    issue_generic_g0_policy_handoff_v1, GenericG0PolicyHandoffIssueV1,
};
use super::generic_g0_projection::{
    GenericG0NumericProjectionRejectV1, GenericG0ProjectionRejectV1,
    GenericG0SourceTypeProjectionRejectV1,
};
use super::located::LocatedStmtV1;
use crate::mir::loop_structural_facts::{
    GenericG0ObservationCoverageV1, GenericG0ObservationModeV1, GenericG0SourceAttemptOutcomeV1,
    GenericG0SourceDeclineV1, GenericG0SourceIdentityV1, GenericG0SourceRejectV1,
    GenericG0SourceUnresolvedV1, VerifiedGenericG0SourceAttemptV1,
};
use crate::mir::numeric_substrate::NumericTarget;
use crate::mir::resolved_semantics::{
    generic_g0::{GenericG0SourceTypeIssueV1, GenericG0SourceTypeRejectV1},
    VerifiedResolvedLoopSourceV1,
};

pub(crate) fn issue_generic_g0_source_attempt_for_test<'source>(
    input: ResolvedFunctionLoweringInputV1<'source>,
    loop_stmt: LocatedStmtV1<'source>,
    source: VerifiedResolvedLoopSourceV1,
    target: NumericTarget,
    mode: Option<GenericG0ObservationModeV1>,
    coverage: GenericG0ObservationCoverageV1,
) -> VerifiedGenericG0SourceAttemptV1 {
    let source_identity_matches = source.matches_identity(
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site(),
    );
    let identity = GenericG0SourceIdentityV1::new(
        input.owner(),
        input.function().function_origin(),
        input.function().source_kind(),
        loop_stmt.site().clone(),
        source.frame_key(),
    );
    let outcome = if input.owner() != loop_stmt.owner() {
        GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::ForeignOwner)
    } else if !source_identity_matches {
        GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::SourceIdentityMismatch)
    } else {
        match issue_generic_g0_policy_handoff_v1(input, target) {
            Ok(handoff) => GenericG0SourceAttemptOutcomeV1::Candidate(handoff),
            Err(issue) => map_handoff_issue(issue),
        }
    };
    VerifiedGenericG0SourceAttemptV1::new(outcome, identity, mode, coverage)
}

fn map_handoff_issue(issue: GenericG0PolicyHandoffIssueV1) -> GenericG0SourceAttemptOutcomeV1 {
    match issue {
        GenericG0PolicyHandoffIssueV1::Source(reject) => map_source_reject(reject),
        GenericG0PolicyHandoffIssueV1::Numeric(reject) => map_numeric_reject(reject),
        GenericG0PolicyHandoffIssueV1::Window | GenericG0PolicyHandoffIssueV1::ReturnMissing => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(
                GenericG0SourceUnresolvedV1::SourceNavigation,
            )
        }
        GenericG0PolicyHandoffIssueV1::ReturnOriginMismatch
        | GenericG0PolicyHandoffIssueV1::Seal(_) => {
            GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::StructuralConflict)
        }
    }
}

fn map_numeric_reject(
    reject: GenericG0NumericProjectionRejectV1,
) -> GenericG0SourceAttemptOutcomeV1 {
    match reject {
        GenericG0NumericProjectionRejectV1::ParameterShape
        | GenericG0NumericProjectionRejectV1::LiteralShape => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(GenericG0SourceUnresolvedV1::MissingFact)
        }
        GenericG0NumericProjectionRejectV1::NonIntegerLiteral { .. }
        | GenericG0NumericProjectionRejectV1::ReturnAbi => {
            GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::TypeConflict)
        }
        GenericG0NumericProjectionRejectV1::Numeric(issue) => match issue {
            crate::mir::numeric_substrate::generic_g0::GenericG0NumericIssueV1::Unresolved(_) => {
                GenericG0SourceAttemptOutcomeV1::Unresolved(
                    GenericG0SourceUnresolvedV1::NumericUnavailable,
                )
            }
            crate::mir::numeric_substrate::generic_g0::GenericG0NumericIssueV1::Rejected(_) => {
                GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::NumericConflict)
            }
        },
    }
}

fn map_source_reject(
    reject: GenericG0SourceTypeProjectionRejectV1,
) -> GenericG0SourceAttemptOutcomeV1 {
    match reject {
        GenericG0SourceTypeProjectionRejectV1::Structural(reject) => map_projection_reject(reject),
        GenericG0SourceTypeProjectionRejectV1::HeaderShape
        | GenericG0SourceTypeProjectionRejectV1::ParameterShape => {
            GenericG0SourceAttemptOutcomeV1::Declined(GenericG0SourceDeclineV1::NotGenericG0Shape)
        }
        GenericG0SourceTypeProjectionRejectV1::SourceNavigation => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(
                GenericG0SourceUnresolvedV1::SourceNavigation,
            )
        }
        GenericG0SourceTypeProjectionRejectV1::BindingLookup { .. } => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(GenericG0SourceUnresolvedV1::MissingFact)
        }
        GenericG0SourceTypeProjectionRejectV1::Type(issue) => map_source_type_issue(issue),
        GenericG0SourceTypeProjectionRejectV1::StructuralRelation => {
            GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::StructuralConflict)
        }
    }
}

fn map_projection_reject(reject: GenericG0ProjectionRejectV1) -> GenericG0SourceAttemptOutcomeV1 {
    match reject {
        GenericG0ProjectionRejectV1::FunctionBodySchedule
        | GenericG0ProjectionRejectV1::RootBodySchedule
        | GenericG0ProjectionRejectV1::ChildBodySchedule
        | GenericG0ProjectionRejectV1::LoopShape
        | GenericG0ProjectionRejectV1::ConditionShape
        | GenericG0ProjectionRejectV1::UpdateShape
        | GenericG0ProjectionRejectV1::TailShape => {
            GenericG0SourceAttemptOutcomeV1::Declined(GenericG0SourceDeclineV1::NotGenericG0Shape)
        }
        GenericG0ProjectionRejectV1::SourceNavigation => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(
                GenericG0SourceUnresolvedV1::SourceNavigation,
            )
        }
        GenericG0ProjectionRejectV1::BindingLookup => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(GenericG0SourceUnresolvedV1::MissingFact)
        }
        GenericG0ProjectionRejectV1::ForestShape => {
            GenericG0SourceAttemptOutcomeV1::Unresolved(GenericG0SourceUnresolvedV1::SourceLookup)
        }
        GenericG0ProjectionRejectV1::ForeignOwner => {
            GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::ForeignOwner)
        }
        GenericG0ProjectionRejectV1::Structural(reject) => map_structural_reject(reject),
    }
}

fn map_structural_reject(
    reject: crate::mir::loop_structural_facts::generic_g0::GenericG0StructuralRejectV1,
) -> GenericG0SourceAttemptOutcomeV1 {
    use crate::mir::loop_structural_facts::generic_g0::GenericG0StructuralRejectV1;
    match reject {
        GenericG0StructuralRejectV1::FunctionBodySchedule
        | GenericG0StructuralRejectV1::RootBodySchedule
        | GenericG0StructuralRejectV1::ChildBodySchedule
        | GenericG0StructuralRejectV1::ForestShape => {
            GenericG0SourceAttemptOutcomeV1::Declined(GenericG0SourceDeclineV1::NotGenericG0Shape)
        }
        GenericG0StructuralRejectV1::WrongSourceKind
        | GenericG0StructuralRejectV1::ForestIdentity
        | GenericG0StructuralRejectV1::BindingRelation
        | GenericG0StructuralRejectV1::Coverage => {
            GenericG0SourceAttemptOutcomeV1::Rejected(GenericG0SourceRejectV1::StructuralConflict)
        }
    }
}

fn map_source_type_issue(issue: GenericG0SourceTypeIssueV1) -> GenericG0SourceAttemptOutcomeV1 {
    match issue {
        GenericG0SourceTypeIssueV1::Unresolved(_) => GenericG0SourceAttemptOutcomeV1::Unresolved(
            GenericG0SourceUnresolvedV1::TypeUnavailable,
        ),
        GenericG0SourceTypeIssueV1::Rejected(reject) => {
            let reason = match reject {
                GenericG0SourceTypeRejectV1::ForeignOwner => GenericG0SourceRejectV1::ForeignOwner,
                GenericG0SourceTypeRejectV1::ParameterBinding { .. }
                | GenericG0SourceTypeRejectV1::ParameterHeaderSite { .. }
                | GenericG0SourceTypeRejectV1::ReturnHeaderSite
                | GenericG0SourceTypeRejectV1::LiteralSiteOwner
                | GenericG0SourceTypeRejectV1::LiteralContextOwner
                | GenericG0SourceTypeRejectV1::DuplicateLiteralRole
                | GenericG0SourceTypeRejectV1::LiteralCardinality => {
                    GenericG0SourceRejectV1::StructuralConflict
                }
                _ => GenericG0SourceRejectV1::TypeConflict,
            };
            GenericG0SourceAttemptOutcomeV1::Rejected(reason)
        }
    }
}

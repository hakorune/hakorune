//! Caller-zero semantic selector for a complete five-row family window.
//!
//! The assembler owns row-level validity.  This module receives only its
//! `Ready` product, counts Candidate/Declined rows, and moves the one selected
//! typed candidate without creating a route, Recipe, Builder, or MIR product.

use super::direct_accum_observation::{
    DirectAccumFamilyObservationV1, VerifiedDirectAccumFamilyCandidateV1,
};
use super::family_admission::{
    LoopFamilyAdmissionCoverageV1, LoopFamilyAdmissionModeV1, LoopFamilyTagV1,
    VerifiedLoopFamilyAdmissionRowsV1, VerifiedLoopFamilyAdmissionWindowV1,
};
use super::generic_g0_observation::{
    GenericG0FamilyObservationV1, VerifiedGenericG0FamilyCandidateV1,
};
use super::loop_cond_break_continue_observation::{
    LoopCondFamilyObservationV1, VerifiedLoopCondFamilyCandidateV1,
};
use super::loop_true_break_continue_observation::{
    LoopTrueFamilyObservationV1, VerifiedLoopTrueFamilyCandidateV1,
};
use super::nested_predicate_observation::{
    NestedPredicateFamilyObservationV1, VerifiedNestedPredicateFamilyCandidateV1,
};
use crate::mir::resolved_semantics::VerifiedLoopFamilyWindowLeaseV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CanonicalLoopFamilyCandidateV1 {
    DirectAccum(VerifiedDirectAccumFamilyCandidateV1),
    NestedPredicate(VerifiedNestedPredicateFamilyCandidateV1),
    LoopTrue(VerifiedLoopTrueFamilyCandidateV1),
    LoopCond(VerifiedLoopCondFamilyCandidateV1),
    GenericG0(VerifiedGenericG0FamilyCandidateV1),
}

impl CanonicalLoopFamilyCandidateV1 {
    pub(crate) const fn tag(&self) -> LoopFamilyTagV1 {
        match self {
            Self::DirectAccum(_) => LoopFamilyTagV1::DirectAccum,
            Self::NestedPredicate(_) => LoopFamilyTagV1::NestedPredicate,
            Self::LoopTrue(_) => LoopFamilyTagV1::LoopTrueBreakContinue,
            Self::LoopCond(_) => LoopFamilyTagV1::LoopCondBreakContinue,
            Self::GenericG0(_) => LoopFamilyTagV1::GenericG0,
        }
    }

    pub(crate) fn into_generic_g0(
        self,
    ) -> Result<VerifiedGenericG0FamilyCandidateV1, CanonicalLoopFamilyCandidateV1> {
        match self {
            Self::GenericG0(candidate) => Ok(candidate),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CanonicalLoopFamilySelectionReasonV1 {
    Overlap,
    OutOfWindow,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalLoopFamilySelectionV1 {
    lease: VerifiedLoopFamilyWindowLeaseV1,
    mode: LoopFamilyAdmissionModeV1,
    coverage: LoopFamilyAdmissionCoverageV1,
    candidate: CanonicalLoopFamilyCandidateV1,
}

impl CanonicalLoopFamilySelectionV1 {
    pub(crate) fn lease(&self) -> &VerifiedLoopFamilyWindowLeaseV1 {
        &self.lease
    }

    pub(crate) const fn mode(&self) -> LoopFamilyAdmissionModeV1 {
        self.mode
    }

    pub(crate) const fn coverage(&self) -> LoopFamilyAdmissionCoverageV1 {
        self.coverage
    }

    pub(crate) const fn candidate(&self) -> &CanonicalLoopFamilyCandidateV1 {
        &self.candidate
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedLoopFamilyWindowLeaseV1,
        LoopFamilyAdmissionModeV1,
        LoopFamilyAdmissionCoverageV1,
        CanonicalLoopFamilyCandidateV1,
    ) {
        (self.lease, self.mode, self.coverage, self.candidate)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CanonicalLoopFamilySelectionFailureV1 {
    lease: VerifiedLoopFamilyWindowLeaseV1,
    rows: VerifiedLoopFamilyAdmissionRowsV1,
    reason: CanonicalLoopFamilySelectionReasonV1,
}

impl CanonicalLoopFamilySelectionFailureV1 {
    pub(crate) fn lease(&self) -> &VerifiedLoopFamilyWindowLeaseV1 {
        &self.lease
    }

    pub(crate) fn rows(&self) -> &VerifiedLoopFamilyAdmissionRowsV1 {
        &self.rows
    }

    pub(crate) const fn reason(&self) -> CanonicalLoopFamilySelectionReasonV1 {
        self.reason
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CanonicalLoopFamilySelectionOutcomeV1 {
    Selected(CanonicalLoopFamilySelectionV1),
    Rejected(CanonicalLoopFamilySelectionFailureV1),
    Unresolved(CanonicalLoopFamilySelectionFailureV1),
}

pub(crate) fn select_canonical_loop_family_v1(
    window: VerifiedLoopFamilyAdmissionWindowV1,
) -> CanonicalLoopFamilySelectionOutcomeV1 {
    let (lease, rows, mode, coverage) = window.into_parts();
    let candidate_count = count_candidates(&rows);
    if candidate_count > 1 {
        return CanonicalLoopFamilySelectionOutcomeV1::Rejected(
            CanonicalLoopFamilySelectionFailureV1 {
                lease,
                rows,
                reason: CanonicalLoopFamilySelectionReasonV1::Overlap,
            },
        );
    }
    if candidate_count == 0 {
        return CanonicalLoopFamilySelectionOutcomeV1::Unresolved(
            CanonicalLoopFamilySelectionFailureV1 {
                lease,
                rows,
                reason: CanonicalLoopFamilySelectionReasonV1::OutOfWindow,
            },
        );
    }

    let (direct_accum, nested_predicate, loop_true, loop_cond, generic_g0) = rows.into_parts();
    let candidate = take_single_candidate(
        direct_accum,
        nested_predicate,
        loop_true,
        loop_cond,
        generic_g0,
    );
    CanonicalLoopFamilySelectionOutcomeV1::Selected(CanonicalLoopFamilySelectionV1 {
        lease,
        mode,
        coverage,
        candidate,
    })
}

fn count_candidates(rows: &VerifiedLoopFamilyAdmissionRowsV1) -> usize {
    [
        matches!(
            rows.direct_accum(),
            DirectAccumFamilyObservationV1::Candidate(_)
        ),
        matches!(
            rows.nested_predicate(),
            NestedPredicateFamilyObservationV1::Candidate(_)
        ),
        matches!(rows.loop_true(), LoopTrueFamilyObservationV1::Candidate(_)),
        matches!(rows.loop_cond(), LoopCondFamilyObservationV1::Candidate(_)),
        matches!(
            rows.generic_g0(),
            GenericG0FamilyObservationV1::Candidate(_)
        ),
    ]
    .into_iter()
    .filter(|is_candidate| *is_candidate)
    .count()
}

fn take_single_candidate(
    direct_accum: DirectAccumFamilyObservationV1,
    nested_predicate: NestedPredicateFamilyObservationV1,
    loop_true: LoopTrueFamilyObservationV1,
    loop_cond: LoopCondFamilyObservationV1,
    generic_g0: GenericG0FamilyObservationV1,
) -> CanonicalLoopFamilyCandidateV1 {
    match direct_accum {
        DirectAccumFamilyObservationV1::Candidate(candidate) => {
            CanonicalLoopFamilyCandidateV1::DirectAccum(candidate)
        }
        DirectAccumFamilyObservationV1::Declined { .. } => match nested_predicate {
            NestedPredicateFamilyObservationV1::Candidate(candidate) => {
                CanonicalLoopFamilyCandidateV1::NestedPredicate(candidate)
            }
            NestedPredicateFamilyObservationV1::Declined { .. } => match loop_true {
                LoopTrueFamilyObservationV1::Candidate(candidate) => {
                    CanonicalLoopFamilyCandidateV1::LoopTrue(candidate)
                }
                LoopTrueFamilyObservationV1::Declined { .. } => match loop_cond {
                    LoopCondFamilyObservationV1::Candidate(candidate) => {
                        CanonicalLoopFamilyCandidateV1::LoopCond(candidate)
                    }
                    LoopCondFamilyObservationV1::Declined { .. } => match generic_g0 {
                        GenericG0FamilyObservationV1::Candidate(candidate) => {
                            CanonicalLoopFamilyCandidateV1::GenericG0(candidate)
                        }
                        GenericG0FamilyObservationV1::Declined { .. } => {
                            unreachable!("candidate count was one but all rows declined")
                        }
                        GenericG0FamilyObservationV1::Unresolved { .. }
                        | GenericG0FamilyObservationV1::Rejected { .. } => {
                            unreachable!("Ready window contained a non-declined Generic row")
                        }
                    },
                    LoopCondFamilyObservationV1::Unresolved { .. }
                    | LoopCondFamilyObservationV1::Rejected { .. } => {
                        unreachable!("Ready window contained a non-declined LoopCond row")
                    }
                },
                LoopTrueFamilyObservationV1::Unresolved { .. }
                | LoopTrueFamilyObservationV1::Rejected { .. } => {
                    unreachable!("Ready window contained a non-declined LoopTrue row")
                }
            },
            NestedPredicateFamilyObservationV1::Unresolved { .. }
            | NestedPredicateFamilyObservationV1::Rejected { .. } => {
                unreachable!("Ready window contained a non-declined Nested row")
            }
        },
        DirectAccumFamilyObservationV1::Unresolved { .. }
        | DirectAccumFamilyObservationV1::Rejected { .. } => {
            unreachable!("Ready window contained a non-declined DirectAccum row")
        }
    }
}

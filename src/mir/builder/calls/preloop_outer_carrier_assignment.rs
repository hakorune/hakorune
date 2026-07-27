//! Exact assignment correspondence for the selected pre-loop outer carrier.
//!
//! The source-sealed target and outer physical destination are projected from
//! `CompletedPreloopOuterCarrierCallV1`. Assignment semantics remain owned by
//! the existing from-value authority and its source-neutral completion box.

use crate::mir::builder::stmts::{
    build_variable_assignment_with_completion_v1, CompletedVariableAssignmentV1,
    RejectedVariableAssignmentCompletionV1,
};
use crate::mir::{MirBuilder, ValueId};

use super::preloop_outer_carrier_transaction::{
    CompletedPreloopOuterCarrierCallV1, OwnedPreloopOuterCarrierPartsV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopCarrierAssignmentStageV1 {
    Assignment,
    Target,
    Rhs,
    ReturnedCarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PreloopCarrierAssignmentErrorV1 {
    AssignmentFailed,
    TargetMismatch,
    RhsMismatch,
    ReturnedCarrierMismatch,
}

#[derive(Debug)]
enum RetainedPreloopCarrierAssignmentOwnerV1<'site, 'view, 'catalog> {
    AssignmentFailure {
        carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
        assignment: RejectedVariableAssignmentCompletionV1,
    },
    Correspondence {
        carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
        assignment: CompletedVariableAssignmentV1,
    },
}

#[derive(Debug)]
pub(super) struct RejectedPreloopCarrierAssignmentV1<'site, 'view, 'catalog> {
    owner: RetainedPreloopCarrierAssignmentOwnerV1<'site, 'view, 'catalog>,
    stage: PreloopCarrierAssignmentStageV1,
    cause: PreloopCarrierAssignmentErrorV1,
}

#[derive(Debug)]
pub(super) struct OwnedPreloopCarrierAssignmentPartsV1 {
    pub(super) carrier: OwnedPreloopOuterCarrierPartsV1,
    pub(super) assignment: CompletedVariableAssignmentV1,
}

impl OwnedPreloopCarrierAssignmentPartsV1 {
    pub(super) fn discard(self) {
        self.carrier.discard();
        self.assignment.discard();
    }
}

#[derive(Debug)]
enum OwnedRetainedPreloopCarrierAssignmentOwnerV1 {
    AssignmentFailure {
        carrier: OwnedPreloopOuterCarrierPartsV1,
        assignment: RejectedVariableAssignmentCompletionV1,
    },
    Correspondence(OwnedPreloopCarrierAssignmentPartsV1),
}

#[derive(Debug)]
pub(super) struct OwnedRejectedPreloopCarrierAssignmentV1 {
    owner: OwnedRetainedPreloopCarrierAssignmentOwnerV1,
    stage: PreloopCarrierAssignmentStageV1,
    cause: PreloopCarrierAssignmentErrorV1,
}

impl RejectedPreloopCarrierAssignmentV1<'_, '_, '_> {
    pub(super) const fn stage(&self) -> PreloopCarrierAssignmentStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> PreloopCarrierAssignmentErrorV1 {
        self.cause
    }

    pub(super) fn bounded_report(&self) -> String {
        let retained = match &self.owner {
            RetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure { assignment, .. } => {
                assignment.bounded_report()
            }
            RetainedPreloopCarrierAssignmentOwnerV1::Correspondence { .. } => {
                "completed-assignment".to_owned()
            }
        };
        format!(
            "[preloop-outer-carrier/assignment/{:?}] {:?} retained={retained}",
            self.stage, self.cause
        )
    }

    pub(super) fn discard(self) {
        match self.owner {
            RetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure {
                carrier,
                assignment,
            } => {
                carrier.discard();
                assignment.discard();
            }
            RetainedPreloopCarrierAssignmentOwnerV1::Correspondence {
                carrier,
                assignment,
            } => {
                carrier.discard();
                assignment.discard();
            }
        }
    }

    pub(super) fn into_owned_rejection_v1(self) -> OwnedRejectedPreloopCarrierAssignmentV1 {
        let owner = match self.owner {
            RetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure {
                carrier,
                assignment,
            } => OwnedRetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure {
                carrier: carrier.into_owned_parts_v1(),
                assignment,
            },
            RetainedPreloopCarrierAssignmentOwnerV1::Correspondence {
                carrier,
                assignment,
            } => OwnedRetainedPreloopCarrierAssignmentOwnerV1::Correspondence(
                OwnedPreloopCarrierAssignmentPartsV1 {
                    carrier: carrier.into_owned_parts_v1(),
                    assignment,
                },
            ),
        };
        OwnedRejectedPreloopCarrierAssignmentV1 {
            owner,
            stage: self.stage,
            cause: self.cause,
        }
    }
}

impl OwnedRejectedPreloopCarrierAssignmentV1 {
    pub(super) const fn stage(&self) -> PreloopCarrierAssignmentStageV1 {
        self.stage
    }

    pub(super) const fn cause(&self) -> PreloopCarrierAssignmentErrorV1 {
        self.cause
    }

    pub(super) fn discard(self) {
        match self.owner {
            OwnedRetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure {
                carrier,
                assignment,
            } => {
                carrier.discard();
                assignment.discard();
            }
            OwnedRetainedPreloopCarrierAssignmentOwnerV1::Correspondence(parts) => parts.discard(),
        }
    }
}

#[derive(Debug)]
pub(super) struct CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog> {
    carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
    assignment: CompletedVariableAssignmentV1,
    _seal: CompletedPreloopCarrierAssignmentSealV1,
}

#[derive(Debug)]
struct CompletedPreloopCarrierAssignmentSealV1;

impl CompletedPreloopCarrierAssignmentV1<'_, '_, '_> {
    pub(super) fn target(&self) -> &str {
        self.assignment.target()
    }

    pub(super) const fn outer_destination(&self) -> ValueId {
        self.carrier.outer_destination()
    }

    pub(super) const fn assigned_destination(&self) -> ValueId {
        self.assignment.assigned()
    }

    pub(super) fn discard(self) {
        self.carrier.discard();
        self.assignment.discard();
    }

    pub(super) fn into_owned_parts_v1(self) -> OwnedPreloopCarrierAssignmentPartsV1 {
        OwnedPreloopCarrierAssignmentPartsV1 {
            carrier: self.carrier.into_owned_parts_v1(),
            assignment: self.assignment,
        }
    }
}

pub(super) fn complete_preloop_carrier_assignment_v1<'site, 'view, 'catalog>(
    builder: &mut MirBuilder,
    carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
) -> Result<
    CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    RejectedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
> {
    let target = carrier.assignment_target().to_owned();
    let rhs = carrier.outer_destination();
    match build_variable_assignment_with_completion_v1(builder, target, rhs) {
        Ok(assignment) => seal_preloop_carrier_assignment_v1(carrier, assignment),
        Err(assignment) => Err(RejectedPreloopCarrierAssignmentV1 {
            owner: RetainedPreloopCarrierAssignmentOwnerV1::AssignmentFailure {
                carrier,
                assignment,
            },
            stage: PreloopCarrierAssignmentStageV1::Assignment,
            cause: PreloopCarrierAssignmentErrorV1::AssignmentFailed,
        }),
    }
}

pub(super) fn seal_preloop_carrier_assignment_v1<'site, 'view, 'catalog>(
    carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
    assignment: CompletedVariableAssignmentV1,
) -> Result<
    CompletedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
    RejectedPreloopCarrierAssignmentV1<'site, 'view, 'catalog>,
> {
    if assignment.target() != carrier.assignment_target() {
        return Err(reject_correspondence(
            carrier,
            assignment,
            PreloopCarrierAssignmentStageV1::Target,
            PreloopCarrierAssignmentErrorV1::TargetMismatch,
        ));
    }
    if assignment.rhs() != carrier.outer_destination() {
        return Err(reject_correspondence(
            carrier,
            assignment,
            PreloopCarrierAssignmentStageV1::Rhs,
            PreloopCarrierAssignmentErrorV1::RhsMismatch,
        ));
    }
    if assignment.assigned() != carrier.outer_destination() {
        return Err(reject_correspondence(
            carrier,
            assignment,
            PreloopCarrierAssignmentStageV1::ReturnedCarrier,
            PreloopCarrierAssignmentErrorV1::ReturnedCarrierMismatch,
        ));
    }
    Ok(CompletedPreloopCarrierAssignmentV1 {
        carrier,
        assignment,
        _seal: CompletedPreloopCarrierAssignmentSealV1,
    })
}

fn reject_correspondence<'site, 'view, 'catalog>(
    carrier: CompletedPreloopOuterCarrierCallV1<'site, 'view, 'catalog>,
    assignment: CompletedVariableAssignmentV1,
    stage: PreloopCarrierAssignmentStageV1,
    cause: PreloopCarrierAssignmentErrorV1,
) -> RejectedPreloopCarrierAssignmentV1<'site, 'view, 'catalog> {
    RejectedPreloopCarrierAssignmentV1 {
        owner: RetainedPreloopCarrierAssignmentOwnerV1::Correspondence {
            carrier,
            assignment,
        },
        stage,
        cause,
    }
}

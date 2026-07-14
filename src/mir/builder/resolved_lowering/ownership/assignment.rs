use super::error::OwnershipTransitionErrorV1;
use super::value::{
    next_value_plan, LocalBindingSubjectV1, LoweredValueOwnershipV1, NextBindingValuePlanV1,
    OwnedValueIdV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct InstallBindingOwnershipPlanV1 {
    target: LocalBindingSubjectV1,
    next: NextBindingValuePlanV1,
}

impl InstallBindingOwnershipPlanV1 {
    pub(super) const fn target(self) -> LocalBindingSubjectV1 {
        self.target
    }

    pub(super) const fn next(self) -> NextBindingValuePlanV1 {
        self.next
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ReplaceBindingOwnershipPlanV1 {
    target: LocalBindingSubjectV1,
    next: NextBindingValuePlanV1,
    previous: Option<OwnedValueIdV1>,
}

impl ReplaceBindingOwnershipPlanV1 {
    pub(super) const fn target(self) -> LocalBindingSubjectV1 {
        self.target
    }

    /// Materialize this value before committing the target definition.
    pub(super) const fn next(self) -> NextBindingValuePlanV1 {
        self.next
    }

    /// Destroy this token only after `next` is materialized and committed.
    pub(super) const fn previous_after_commit(self) -> Option<OwnedValueIdV1> {
        self.previous
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AssignmentOwnershipPlanV1 {
    ExactSelfAssignment { binding: LocalBindingSubjectV1 },
    Replace(ReplaceBindingOwnershipPlanV1),
}

pub(super) fn plan_declaration(
    target: LocalBindingSubjectV1,
    initializer: LoweredValueOwnershipV1,
) -> Result<InstallBindingOwnershipPlanV1, OwnershipTransitionErrorV1> {
    verify_source_owner(target, initializer)?;
    Ok(InstallBindingOwnershipPlanV1 {
        target,
        next: next_value_plan(initializer),
    })
}

pub(super) fn plan_assignment(
    target: LocalBindingSubjectV1,
    previous: Option<OwnedValueIdV1>,
    next: LoweredValueOwnershipV1,
) -> Result<AssignmentOwnershipPlanV1, OwnershipTransitionErrorV1> {
    verify_source_owner(target, next)?;
    if let LoweredValueOwnershipV1::BorrowedStrong { binding, .. } = next {
        if binding.binding() == target.binding() {
            return Ok(AssignmentOwnershipPlanV1::ExactSelfAssignment { binding: target });
        }
    }
    if let (Some(previous), LoweredValueOwnershipV1::Owned { value }) = (previous, next) {
        if previous == value {
            return Err(OwnershipTransitionErrorV1::OwnedNextAliasesPrevious {
                value: value.value(),
            });
        }
    }
    Ok(AssignmentOwnershipPlanV1::Replace(
        ReplaceBindingOwnershipPlanV1 {
            target,
            next: next_value_plan(next),
            previous,
        },
    ))
}

fn verify_source_owner(
    target: LocalBindingSubjectV1,
    value: LoweredValueOwnershipV1,
) -> Result<(), OwnershipTransitionErrorV1> {
    let LoweredValueOwnershipV1::BorrowedStrong { binding, .. } = value else {
        return Ok(());
    };
    if binding.owner() != target.owner() {
        return Err(OwnershipTransitionErrorV1::ForeignOwner {
            expected: target.owner(),
            actual: binding.owner(),
            binding: binding.binding(),
        });
    }
    Ok(())
}

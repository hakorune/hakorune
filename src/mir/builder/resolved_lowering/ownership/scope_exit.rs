use std::collections::BTreeSet;

use crate::mir::ownership_ssa::FunctionResultOwnershipV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;

use super::error::OwnershipTransitionErrorV1;
use super::value::{
    next_value_plan, LocalBindingSubjectV1, LoweredValueOwnershipV1, NextBindingValuePlanV1,
    OwnedValueIdV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct OwnedBindingAtCloseV1 {
    binding: LocalBindingSubjectV1,
    current: OwnedValueIdV1,
}

impl OwnedBindingAtCloseV1 {
    pub(super) const fn new(binding: LocalBindingSubjectV1, current: OwnedValueIdV1) -> Self {
        Self { binding, current }
    }

    pub(super) const fn binding(self) -> LocalBindingSubjectV1 {
        self.binding
    }

    pub(super) const fn current(self) -> OwnedValueIdV1 {
        self.current
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScopeTailOwnershipV1 {
    None,
    Trivial {
        value: crate::mir::ValueId,
    },
    ScopeLocal {
        binding: LocalBindingSubjectV1,
        value: OwnedValueIdV1,
    },
    OuterBorrowed {
        binding: LocalBindingSubjectV1,
        value: crate::mir::ValueId,
    },
    ForwardOwned {
        value: OwnedValueIdV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScopeResultOwnershipPlanV1 {
    None,
    ReuseTrivial {
        value: crate::mir::ValueId,
    },
    TransferScopeLocal {
        binding: LocalBindingSubjectV1,
        value: OwnedValueIdV1,
    },
    CopyOuterBorrowed {
        source: LocalBindingSubjectV1,
        value: crate::mir::ValueId,
    },
    ForwardOwned {
        value: OwnedValueIdV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ScopeCloseOwnershipPlanV1 {
    result: ScopeResultOwnershipPlanV1,
    destroys: Box<[OwnedValueIdV1]>,
}

impl ScopeCloseOwnershipPlanV1 {
    pub(super) const fn result(&self) -> ScopeResultOwnershipPlanV1 {
        self.result
    }

    pub(super) fn destroys_in_order(&self) -> &[OwnedValueIdV1] {
        &self.destroys
    }
}

pub(super) fn plan_scope_close(
    owner: FunctionOwnerIdV1,
    declarations: &[OwnedBindingAtCloseV1],
    tail: ScopeTailOwnershipV1,
) -> Result<ScopeCloseOwnershipPlanV1, OwnershipTransitionErrorV1> {
    validate_declarations(owner, declarations)?;
    let excluded = match tail {
        ScopeTailOwnershipV1::ScopeLocal { binding, value } => {
            require_owner(owner, binding)?;
            let Some(entry) = declarations
                .iter()
                .find(|entry| entry.binding().binding() == binding.binding())
            else {
                return Err(OwnershipTransitionErrorV1::ScopeLocalTailMissing {
                    binding: binding.binding(),
                });
            };
            if entry.current() != value {
                return Err(OwnershipTransitionErrorV1::ScopeLocalTailValueMismatch {
                    binding: binding.binding(),
                    expected: entry.current().value(),
                    actual: value.value(),
                });
            }
            Some(binding.binding())
        }
        ScopeTailOwnershipV1::OuterBorrowed { binding, .. } => {
            require_owner(owner, binding)?;
            if declarations
                .iter()
                .any(|entry| entry.binding().binding() == binding.binding())
            {
                return Err(OwnershipTransitionErrorV1::OuterBorrowedTailIsScopeLocal {
                    binding: binding.binding(),
                });
            }
            None
        }
        ScopeTailOwnershipV1::ForwardOwned { value } => {
            if declarations.iter().any(|entry| entry.current() == value) {
                return Err(
                    OwnershipTransitionErrorV1::ForwardedOwnedStillOwnedByScope {
                        value: value.value(),
                    },
                );
            }
            None
        }
        ScopeTailOwnershipV1::None | ScopeTailOwnershipV1::Trivial { .. } => None,
    };
    let destroys = declarations
        .iter()
        .rev()
        .filter(|entry| Some(entry.binding().binding()) != excluded)
        .map(|entry| entry.current())
        .collect::<Vec<_>>()
        .into_boxed_slice();
    let result = match tail {
        ScopeTailOwnershipV1::None => ScopeResultOwnershipPlanV1::None,
        ScopeTailOwnershipV1::Trivial { value } => {
            ScopeResultOwnershipPlanV1::ReuseTrivial { value }
        }
        ScopeTailOwnershipV1::ScopeLocal { binding, value } => {
            ScopeResultOwnershipPlanV1::TransferScopeLocal { binding, value }
        }
        ScopeTailOwnershipV1::OuterBorrowed { binding, value } => {
            ScopeResultOwnershipPlanV1::CopyOuterBorrowed {
                source: binding,
                value,
            }
        }
        ScopeTailOwnershipV1::ForwardOwned { value } => {
            ScopeResultOwnershipPlanV1::ForwardOwned { value }
        }
    };
    Ok(ScopeCloseOwnershipPlanV1 { result, destroys })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionTerminalOwnershipV1 {
    Fallthrough,
    Return(LoweredValueOwnershipV1),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FunctionTerminalResultPlanV1 {
    Fallthrough,
    Return(NextBindingValuePlanV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct FunctionExitOwnershipPlanV1 {
    result: FunctionTerminalResultPlanV1,
    destroys: Box<[OwnedValueIdV1]>,
}

impl FunctionExitOwnershipPlanV1 {
    pub(super) const fn result(&self) -> FunctionTerminalResultPlanV1 {
        self.result
    }

    pub(super) fn destroys_in_order(&self) -> &[OwnedValueIdV1] {
        &self.destroys
    }
}

pub(super) fn plan_function_exit(
    owner: FunctionOwnerIdV1,
    roots: &[OwnedBindingAtCloseV1],
    expected: FunctionResultOwnershipV1,
    terminal: FunctionTerminalOwnershipV1,
) -> Result<FunctionExitOwnershipPlanV1, OwnershipTransitionErrorV1> {
    validate_declarations(owner, roots)?;
    let (actual, result) = match terminal {
        FunctionTerminalOwnershipV1::Fallthrough => (
            FunctionResultOwnershipV1::None,
            FunctionTerminalResultPlanV1::Fallthrough,
        ),
        FunctionTerminalOwnershipV1::Return(value) => {
            let actual = match value {
                LoweredValueOwnershipV1::Trivial { .. } => FunctionResultOwnershipV1::None,
                LoweredValueOwnershipV1::Owned { .. }
                | LoweredValueOwnershipV1::BorrowedStrong { .. } => {
                    FunctionResultOwnershipV1::Owned
                }
            };
            if let LoweredValueOwnershipV1::BorrowedStrong { binding, .. } = value {
                require_owner(owner, binding)?;
            }
            if let LoweredValueOwnershipV1::Owned { value } = value {
                if roots.iter().any(|entry| entry.current() == value) {
                    return Err(
                        OwnershipTransitionErrorV1::ForwardedOwnedStillOwnedByScope {
                            value: value.value(),
                        },
                    );
                }
            }
            (
                actual,
                FunctionTerminalResultPlanV1::Return(next_value_plan(value)),
            )
        }
    };
    if actual != expected {
        return Err(OwnershipTransitionErrorV1::ResultOwnershipMismatch { expected, actual });
    }
    Ok(FunctionExitOwnershipPlanV1 {
        result,
        destroys: roots
            .iter()
            .rev()
            .map(|entry| entry.current())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    })
}

fn validate_declarations(
    owner: FunctionOwnerIdV1,
    declarations: &[OwnedBindingAtCloseV1],
) -> Result<(), OwnershipTransitionErrorV1> {
    let mut bindings = BTreeSet::new();
    let mut values = BTreeSet::new();
    for entry in declarations {
        require_owner(owner, entry.binding())?;
        if !bindings.insert(entry.binding().binding()) {
            return Err(OwnershipTransitionErrorV1::DuplicateClosingBinding {
                binding: entry.binding().binding(),
            });
        }
        if !values.insert(entry.current()) {
            return Err(OwnershipTransitionErrorV1::DuplicateOwnedToken {
                value: entry.current().value(),
            });
        }
    }
    Ok(())
}

fn require_owner(
    expected: FunctionOwnerIdV1,
    binding: LocalBindingSubjectV1,
) -> Result<(), OwnershipTransitionErrorV1> {
    if binding.owner() != expected {
        return Err(OwnershipTransitionErrorV1::ForeignOwner {
            expected,
            actual: binding.owner(),
            binding: binding.binding(),
        });
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct UnpublishedDraftDiscardOwnershipPlanV1;

pub(super) const fn plan_unpublished_draft_discard() -> UnpublishedDraftDiscardOwnershipPlanV1 {
    UnpublishedDraftDiscardOwnershipPlanV1
}

use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct OwnedValueIdV1(ValueId);

impl OwnedValueIdV1 {
    pub(super) const fn new(value: ValueId) -> Self {
        Self(value)
    }

    pub(super) const fn value(self) -> ValueId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LocalBindingClassV1 {
    Receiver,
    Parameter,
    Local,
    Outbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct LocalBindingSubjectV1 {
    binding: BindingRefV1,
    class: LocalBindingClassV1,
}

impl LocalBindingSubjectV1 {
    pub(super) const fn new(binding: BindingRefV1, class: LocalBindingClassV1) -> Self {
        Self { binding, class }
    }

    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.binding.owner()
    }

    pub(super) const fn class(self) -> LocalBindingClassV1 {
        self.class
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LoweredValueOwnershipV1 {
    Trivial {
        value: ValueId,
    },
    Owned {
        value: OwnedValueIdV1,
    },
    BorrowedStrong {
        binding: LocalBindingSubjectV1,
        value: ValueId,
    },
}

impl LoweredValueOwnershipV1 {
    pub(super) const fn value(self) -> ValueId {
        match self {
            Self::Trivial { value } | Self::BorrowedStrong { value, .. } => value,
            Self::Owned { value } => value.value(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NextBindingValuePlanV1 {
    ReuseTrivial {
        value: ValueId,
    },
    TransferOwned {
        value: OwnedValueIdV1,
    },
    CopyBorrowedStrong {
        source: LocalBindingSubjectV1,
        value: ValueId,
    },
}

pub(super) fn next_value_plan(value: LoweredValueOwnershipV1) -> NextBindingValuePlanV1 {
    match value {
        LoweredValueOwnershipV1::Trivial { value } => {
            NextBindingValuePlanV1::ReuseTrivial { value }
        }
        LoweredValueOwnershipV1::Owned { value } => NextBindingValuePlanV1::TransferOwned { value },
        LoweredValueOwnershipV1::BorrowedStrong { binding, value } => {
            NextBindingValuePlanV1::CopyBorrowedStrong {
                source: binding,
                value,
            }
        }
    }
}

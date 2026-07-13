//! Typed fallthrough ports and join-source contracts.

use crate::mir::resolved_semantics::BindingRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedIfConditionEffectsV1 {
    may_rebind_outer: Box<[BindingRefV1]>,
}

impl ResolvedIfConditionEffectsV1 {
    pub(super) fn from_verified(bindings: Vec<BindingRefV1>) -> Self {
        Self {
            may_rebind_outer: bindings.into_boxed_slice(),
        }
    }

    pub(crate) fn may_rebind_outer(&self) -> &[BindingRefV1] {
        &self.may_rebind_outer
    }
}

/// A V1 branch port always reaches the shared merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedFallthroughPortV1 {
    may_rebind_outer: Box<[BindingRefV1]>,
}

impl ResolvedFallthroughPortV1 {
    pub(super) fn from_verified(bindings: Vec<BindingRefV1>) -> Self {
        Self {
            may_rebind_outer: bindings.into_boxed_slice(),
        }
    }

    pub(crate) fn may_rebind_outer(&self) -> &[BindingRefV1] {
        &self.may_rebind_outer
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedElseFallthroughV1 {
    ImplicitIdentity,
    Explicit(ResolvedFallthroughPortV1),
}

impl ResolvedElseFallthroughV1 {
    pub(crate) fn explicit_port(&self) -> Option<&ResolvedFallthroughPortV1> {
        match self {
            Self::ImplicitIdentity => None,
            Self::Explicit(port) => Some(port),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedIfPortValueSourceV1 {
    PostConditionEntry,
    BranchExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedIfJoinBindingV1 {
    binding: BindingRefV1,
    then_source: ResolvedIfPortValueSourceV1,
    else_source: ResolvedIfPortValueSourceV1,
}

impl ResolvedIfJoinBindingV1 {
    pub(super) const fn from_verified(
        binding: BindingRefV1,
        then_source: ResolvedIfPortValueSourceV1,
        else_source: ResolvedIfPortValueSourceV1,
    ) -> Self {
        Self {
            binding,
            then_source,
            else_source,
        }
    }

    pub(crate) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn then_source(self) -> ResolvedIfPortValueSourceV1 {
        self.then_source
    }

    pub(crate) const fn else_source(self) -> ResolvedIfPortValueSourceV1 {
        self.else_source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedIfJoinContractV1 {
    rows: Box<[ResolvedIfJoinBindingV1]>,
}

impl ResolvedIfJoinContractV1 {
    pub(super) fn from_verified(rows: Vec<ResolvedIfJoinBindingV1>) -> Self {
        Self {
            rows: rows.into_boxed_slice(),
        }
    }

    pub(crate) fn rows(&self) -> &[ResolvedIfJoinBindingV1] {
        &self.rows
    }
}

/// Whole-If outgoing effects consumed by an enclosing flow analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedIfWholeEffectsV1 {
    may_rebind_outer: Box<[BindingRefV1]>,
}

impl ResolvedIfWholeEffectsV1 {
    pub(super) fn from_verified(bindings: Vec<BindingRefV1>) -> Self {
        Self {
            may_rebind_outer: bindings.into_boxed_slice(),
        }
    }

    pub(crate) fn may_rebind_outer(&self) -> &[BindingRefV1] {
        &self.may_rebind_outer
    }
}

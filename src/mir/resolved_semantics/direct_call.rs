//! Function-local direct-call target identity.
//!
//! Full callable headers remain source-unit-owned. This record contains only
//! the callable identity resolved for one exact expression site.

use super::{FunctionOwnerIdV1, ResolvedCallableRefV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ResolvedDirectCallTargetV1 {
    callable: ResolvedCallableRefV1,
}

impl ResolvedDirectCallTargetV1 {
    pub(super) const fn from_resolved(callable: ResolvedCallableRefV1) -> Self {
        Self { callable }
    }

    pub(crate) const fn callable(self) -> ResolvedCallableRefV1 {
        self.callable
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedDirectCallVerificationErrorV1 {
    ForeignCompilation {
        function: FunctionOwnerIdV1,
        target: FunctionOwnerIdV1,
    },
}

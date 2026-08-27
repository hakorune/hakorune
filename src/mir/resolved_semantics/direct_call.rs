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

/// Facts observed for one direct-call expression before a callable target is
/// issued.  This is intentionally not a target, recipe, or physical carrier:
/// the source site remains the map key owned by `ResolvedFunctionDataV1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedDirectCallObservationV1 {
    name: Box<str>,
    arity: u32,
}

impl ResolvedDirectCallObservationV1 {
    pub(super) fn from_parts(name: Box<str>, arity: u32) -> Self {
        Self { name, arity }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedDirectCallVerificationErrorV1 {
    ForeignCompilation {
        function: FunctionOwnerIdV1,
        target: FunctionOwnerIdV1,
    },
}

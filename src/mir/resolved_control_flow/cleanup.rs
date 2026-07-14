//! Ordered crossed-scope cleanup vocabulary for verified control exits.

use crate::mir::resolved_semantics::ScopeId;

/// Immutable cleanup order sealed before canonical materialization.
///
/// E0 publishes an explicit empty list only. Nonempty EXIT-S0 cleanup
/// semantics are a later decision; this type does not claim them in advance.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedCleanupObligationsV1 {
    crossed_scopes: Box<[ScopeId]>,
}

impl ResolvedCleanupObligationsV1 {
    pub(super) fn explicit_empty() -> Self {
        Self {
            crossed_scopes: Box::new([]),
        }
    }

    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] {
        &self.crossed_scopes
    }
}

//! Ordered crossed-scope cleanup vocabulary for verified control exits.

use crate::mir::resolved_semantics::{BindingRefV1, ScopeId};
use crate::mir::resolved_semantics::home_new_prefix::HomePrefixUnavailableV1;

/// Immutable cleanup order sealed before canonical materialization.
///
/// E0's crossed-scope list remains empty. The selected App Main New loan can
/// additionally seal terminal Home bindings, or explicit analysis unavailability.
/// Neither absent nor unavailable Home analysis authorizes empty cleanup.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedCleanupObligationsV1 {
    crossed_scopes: Box<[ScopeId]>,
    terminal_homes: Option<Result<Box<[BindingRefV1]>, HomePrefixUnavailableV1>>,
}

impl ResolvedCleanupObligationsV1 {
    pub(super) fn explicit_empty() -> Self {
        Self {
            crossed_scopes: Box::new([]),
            terminal_homes: None,
        }
    }

    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] {
        &self.crossed_scopes
    }

    pub(super) fn with_terminal_homes(
        mut self, homes: Result<Box<[BindingRefV1]>, HomePrefixUnavailableV1>,
    ) -> Self {
        self.terminal_homes = Some(homes);
        self
    }

    pub(crate) fn terminal_homes(&self) -> Option<Result<&[BindingRefV1], &HomePrefixUnavailableV1>> {
        self.terminal_homes.as_ref().map(|row| row.as_deref())
    }
}

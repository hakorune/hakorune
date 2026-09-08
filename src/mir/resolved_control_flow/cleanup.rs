//! Ordered crossed-scope cleanup vocabulary for verified control exits.

use crate::mir::resolved_semantics::home_new_prefix::{HomePrefixUnavailableV1, RootHomeFlow};
use crate::mir::resolved_semantics::{BindingRefV1, ScopeId};

/// Immutable cleanup order sealed before canonical materialization.
///
/// E0's crossed-scope list remains empty. The selected App Main New loan can
/// additionally seal terminal Home bindings, or explicit analysis unavailability.
/// Neither absent nor unavailable Home analysis authorizes empty cleanup.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ResolvedCleanupObligationsV1 {
    crossed_scopes: Box<[ScopeId]>,
    root_flow: Option<RootHomeFlow>,
}

impl ResolvedCleanupObligationsV1 {
    pub(super) fn explicit_empty() -> Self {
        Self {
            crossed_scopes: Box::new([]),
            root_flow: None,
        }
    }

    pub(crate) fn crossed_scopes(&self) -> &[ScopeId] {
        &self.crossed_scopes
    }

    pub(super) fn with_root_flow(mut self, flow: RootHomeFlow) -> Self {
        self.root_flow = Some(flow);
        self
    }
    pub(crate) fn root_flow(&self) -> Option<&RootHomeFlow> {
        self.root_flow.as_ref()
    }
    pub(crate) fn terminal_homes(
        &self,
    ) -> Option<Result<&[BindingRefV1], &HomePrefixUnavailableV1>> {
        self.root_flow.as_ref().map(RootHomeFlow::terminal_homes)
    }
}

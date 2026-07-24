//! FINAL0 neutral route evidence.
//!
//! This contract carries only post-source evidence into the Builder-side
//! finalization terminal. It is deliberately not a source lookup capability.

use crate::mir::raw_physical_drain::RawPhysicalCallableMainDispositionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawFinalizationRouteEvidenceV1<'a> {
    Script {
        module_name: &'a str,
        helper_count: usize,
    },
    App {
        module_name: &'a str,
        helper_count: usize,
        callable_main: RawPhysicalCallableMainDispositionV1,
    },
}

impl RawFinalizationRouteEvidenceV1<'_> {
    pub(crate) const fn helper_count(self) -> usize {
        match self {
            Self::Script { helper_count, .. } | Self::App { helper_count, .. } => helper_count,
        }
    }

    pub(crate) const fn route(self) -> RawFinalizationRouteKindV1 {
        match self {
            Self::Script { .. } => RawFinalizationRouteKindV1::Script,
            Self::App { .. } => RawFinalizationRouteKindV1::App,
        }
    }

    pub(crate) const fn callable_main(self) -> RawPhysicalCallableMainDispositionV1 {
        match self {
            Self::Script { .. } => RawPhysicalCallableMainDispositionV1::NotSelected,
            Self::App { callable_main, .. } => callable_main,
        }
    }

    pub(crate) fn name(&self) -> &str {
        match self {
            Self::Script { module_name, .. } | Self::App { module_name, .. } => module_name,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawFinalizationRouteKindV1 {
    Script,
    App,
}

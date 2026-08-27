//! Route-scoped fate for the unowned phase2160 RawCompatibility runtime-Box
//! arms.
//!
//! The capability is deliberately not a semantic receipt.  It carries no
//! source, target, shape, or ValueId; it only seals one already-selected route
//! to a pre-effect Retire/Frozen outcome.  Generic RawInvocation, Selected
//! Normal, RawLegacy, and the root I1 terminal remain unarmed.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRuntimeBoxFateDispositionV1 {
    Continue,
    Retire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawCompatibilityRuntimeBoxFateStateV1 {
    Armed,
    Consumed,
}

/// Move-only route capability.  A consumed capability cannot be reused.
#[must_use = "phase2160 runtime-box fate must be consumed by scoped lowering"]
#[derive(Debug)]
pub(in crate::mir::builder) struct RawCompatibilityRuntimeBoxFateV1 {
    state: RawCompatibilityRuntimeBoxFateStateV1,
}

impl RawCompatibilityRuntimeBoxFateV1 {
    pub(in crate::mir::builder) fn issue_retire() -> Self {
        Self {
            state: RawCompatibilityRuntimeBoxFateStateV1::Armed,
        }
    }

    pub(in crate::mir::builder) fn take_retire(
        &mut self,
    ) -> Result<RawRuntimeBoxFateDispositionV1, String> {
        match self.state {
            RawCompatibilityRuntimeBoxFateStateV1::Armed => {
                self.state = RawCompatibilityRuntimeBoxFateStateV1::Consumed;
                Ok(RawRuntimeBoxFateDispositionV1::Retire)
            }
            RawCompatibilityRuntimeBoxFateStateV1::Consumed => {
                Err("[freeze:contract][raw-compat/runtime-box-fate-second-take]".to_owned())
            }
        }
    }
}

/// A raw child is either ordinary/unarmed or holds the one phase2160 route
/// borrow.  The borrow makes the phase-specific state explicit without adding
/// another lifetime parameter to `RawInvocationChildPortV1`.
pub(in crate::mir::builder) enum RuntimeBoxFateScopeV1<'scope> {
    Unarmed,
    Phase2160(&'scope mut RawCompatibilityRuntimeBoxFateV1),
}

impl<'scope> RuntimeBoxFateScopeV1<'scope> {
    pub(in crate::mir::builder) fn is_armed(&self) -> bool {
        matches!(self, Self::Phase2160(_))
    }

    pub(in crate::mir::builder) fn reborrow<'short>(
        &'short mut self,
    ) -> RuntimeBoxFateScopeV1<'short> {
        match self {
            Self::Unarmed => RuntimeBoxFateScopeV1::Unarmed,
            Self::Phase2160(fate) => RuntimeBoxFateScopeV1::Phase2160(&mut **fate),
        }
    }

    pub(in crate::mir::builder) fn take_retire(
        &mut self,
    ) -> Result<RawRuntimeBoxFateDispositionV1, String> {
        match self {
            Self::Unarmed => Ok(RawRuntimeBoxFateDispositionV1::Continue),
            Self::Phase2160(fate) => fate.take_retire(),
        }
    }
}

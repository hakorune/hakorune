use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationEffectV1 {
    OpaqueObservable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationOrderingV1 {
    SynchronousNonDetached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationSuspensionV1 {
    MaySuspend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationOutcomeV1 {
    NormalSelfContainedDynamicCarrierOrFault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationControlV1 {
    CallableBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicInvocationInputHomeV1 {
    BorrowedNoEscapeForInvocation,
}

/// The one language-wide Dynamic invocation contract.
///
/// The private field prevents callers from constructing an independently
/// paired effect/Home/outcome receipt. Every verified row borrows the single
/// module-owned value below.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDynamicInvocationExecutionEnvelopeV1 {
    _sealed: (),
}

const LANGUAGE_WIDE_ENVELOPE: VerifiedDynamicInvocationExecutionEnvelopeV1 =
    VerifiedDynamicInvocationExecutionEnvelopeV1 { _sealed: () };

pub(crate) const fn dynamic_invocation_execution_envelope_v1(
) -> &'static VerifiedDynamicInvocationExecutionEnvelopeV1 {
    &LANGUAGE_WIDE_ENVELOPE
}

impl VerifiedDynamicInvocationExecutionEnvelopeV1 {
    pub(crate) const fn effect(&self) -> DynamicInvocationEffectV1 {
        DynamicInvocationEffectV1::OpaqueObservable
    }

    pub(crate) const fn ordering(&self) -> DynamicInvocationOrderingV1 {
        DynamicInvocationOrderingV1::SynchronousNonDetached
    }

    pub(crate) const fn suspension(&self) -> DynamicInvocationSuspensionV1 {
        DynamicInvocationSuspensionV1::MaySuspend
    }

    pub(crate) const fn outcome(&self) -> DynamicInvocationOutcomeV1 {
        DynamicInvocationOutcomeV1::NormalSelfContainedDynamicCarrierOrFault
    }

    pub(crate) const fn control(&self) -> DynamicInvocationControlV1 {
        DynamicInvocationControlV1::CallableBounded
    }

    pub(crate) const fn input_home(&self) -> DynamicInvocationInputHomeV1 {
        DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
    }

    pub(crate) const fn result_lifecycle(&self) -> DynamicCarrierLifecycleObligationV1 {
        DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded
    }
}

/// Borrow-scoped verified view of one exact Dynamic target plus the complete
/// language-wide execution envelope.
#[derive(Debug, Clone, Copy)]
pub(crate) struct VerifiedDynamicInvocationEnvelopeRefV1<'catalog> {
    pub(super) caller: &'catalog CanonicalSameModuleCallableKeyV1,
    pub(super) site: &'catalog SourceExprSiteV1,
    pub(super) target: &'catalog VerifiedSourceBoundDynamicMemberCallV1,
}

impl<'catalog> VerifiedDynamicInvocationEnvelopeRefV1<'catalog> {
    pub(crate) const fn caller(&self) -> &'catalog CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn site(&self) -> &'catalog SourceExprSiteV1 {
        self.site
    }

    pub(crate) const fn target(&self) -> &'catalog VerifiedSourceBoundDynamicMemberCallV1 {
        self.target
    }

    pub(crate) const fn envelope(&self) -> &'static VerifiedDynamicInvocationExecutionEnvelopeV1 {
        &LANGUAGE_WIDE_ENVELOPE
    }
}

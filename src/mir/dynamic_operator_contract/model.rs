use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorFamilyV1 {
    Add,
    Less,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorValueClassV1 {
    Dynamic,
    I64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DynamicOperatorDomainV1 {
    family: DynamicOperatorFamilyV1,
    left: DynamicOperatorValueClassV1,
    right: DynamicOperatorValueClassV1,
}

impl DynamicOperatorDomainV1 {
    pub(crate) const fn new(
        family: DynamicOperatorFamilyV1,
        left: DynamicOperatorValueClassV1,
        right: DynamicOperatorValueClassV1,
    ) -> Self {
        Self {
            family,
            left,
            right,
        }
    }

    pub(crate) const fn family(&self) -> DynamicOperatorFamilyV1 {
        self.family
    }

    pub(crate) const fn left(&self) -> DynamicOperatorValueClassV1 {
        self.left
    }

    pub(crate) const fn right(&self) -> DynamicOperatorValueClassV1 {
        self.right
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorEffectV1 {
    OpaqueObservable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorOrderingV1 {
    SynchronousNonDetached,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorSuspensionV1 {
    MaySuspend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorControlV1 {
    ExpressionBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorInputAccessV1 {
    BorrowedNoEscapeForOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorNormalResultV1 {
    SelfContainedNonAliasingDynamicCarrier,
    TrivialBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorFaultV1 {
    TypeErrorBeforeResultNoOperandMutationNoRebind,
}

/// One complete language-wide Dynamic operator contract.
///
/// Fields are private so callers cannot pair independent semantic axes.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedDynamicOperatorExecutionEnvelopeV1 {
    domain: DynamicOperatorDomainV1,
    normal_result: DynamicOperatorNormalResultV1,
    lifecycle: Option<DynamicCarrierLifecycleObligationV1>,
    _sealed: (),
}

impl VerifiedDynamicOperatorExecutionEnvelopeV1 {
    pub(super) const fn sealed(
        domain: DynamicOperatorDomainV1,
        normal_result: DynamicOperatorNormalResultV1,
        lifecycle: Option<DynamicCarrierLifecycleObligationV1>,
    ) -> Self {
        Self {
            domain,
            normal_result,
            lifecycle,
            _sealed: (),
        }
    }

    pub(crate) const fn domain(&self) -> DynamicOperatorDomainV1 {
        self.domain
    }

    pub(crate) const fn effect(&self) -> DynamicOperatorEffectV1 {
        DynamicOperatorEffectV1::OpaqueObservable
    }

    pub(crate) const fn ordering(&self) -> DynamicOperatorOrderingV1 {
        DynamicOperatorOrderingV1::SynchronousNonDetached
    }

    pub(crate) const fn suspension(&self) -> DynamicOperatorSuspensionV1 {
        DynamicOperatorSuspensionV1::MaySuspend
    }

    pub(crate) const fn control(&self) -> DynamicOperatorControlV1 {
        DynamicOperatorControlV1::ExpressionBounded
    }

    pub(crate) const fn input_access(&self) -> DynamicOperatorInputAccessV1 {
        DynamicOperatorInputAccessV1::BorrowedNoEscapeForOperation
    }

    pub(crate) const fn normal_result(&self) -> DynamicOperatorNormalResultV1 {
        self.normal_result
    }

    pub(crate) const fn fault(&self) -> DynamicOperatorFaultV1 {
        DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
    }

    pub(crate) const fn lifecycle(&self) -> Option<DynamicCarrierLifecycleObligationV1> {
        self.lifecycle
    }
}

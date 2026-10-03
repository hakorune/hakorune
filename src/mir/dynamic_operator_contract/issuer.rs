use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;

use super::{
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorNormalResultV1,
    DynamicOperatorValueClassV1, VerifiedDynamicOperatorExecutionEnvelopeV1,
};

const ADD_DYNAMIC_I64: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

const LESS_DYNAMIC_DYNAMIC: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Dynamic,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        None,
    );

const LESS_DYNAMIC_I64: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        None,
    );

/// The checked compare lends a Normal-Integer operand view: both operands
/// must prove the logical signed-integer class, and a fault arrives before
/// any result without operand mutation or rebind.
const GREATER_NORMAL_INTEGER_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Greater,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        None,
    );

/// A dominated use of the same lent view: both operands must prove the
/// logical signed-integer class; the result is a fresh integer carrying
/// no borrowed identity and no lifecycle obligation.
const ADD_NORMAL_INTEGER_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::NormalInteger,
        None,
    );

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorEnvelopeIssueV1 {
    UnsupportedDomain,
}

pub(crate) const fn issue_dynamic_operator_execution_envelope_v1(
    domain: DynamicOperatorDomainV1,
) -> Result<&'static VerifiedDynamicOperatorExecutionEnvelopeV1, DynamicOperatorEnvelopeIssueV1> {
    match (domain.family(), domain.left(), domain.right()) {
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ) => Ok(&ADD_DYNAMIC_I64),
        (
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Dynamic,
        ) => Ok(&LESS_DYNAMIC_DYNAMIC),
        (
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ) => Ok(&LESS_DYNAMIC_I64),
        (
            DynamicOperatorFamilyV1::Greater,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&GREATER_NORMAL_INTEGER_NORMAL_INTEGER),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&ADD_NORMAL_INTEGER_NORMAL_INTEGER),
        _ => Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain),
    }
}

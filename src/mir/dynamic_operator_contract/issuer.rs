use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;

use super::{
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorNormalResultV1,
    DynamicOperatorSuspensionV1, DynamicOperatorValueClassV1,
    VerifiedDynamicOperatorExecutionEnvelopeV1,
};

const ADD_DYNAMIC_I64: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier,
        DynamicOperatorSuspensionV1::MaySuspend,
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
        DynamicOperatorSuspensionV1::MaySuspend,
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
        DynamicOperatorSuspensionV1::MaySuspend,
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
        DynamicOperatorSuspensionV1::MaySuspend,
        None,
    );

const GREATER_EQUAL_NORMAL_INTEGER_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::GreaterEqual,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::MaySuspend,
        None,
    );

/// Inclusive checked comparison keeps the same borrowed operand and Fault
/// laws; the Boolean result does not change either operand's Integer class.
const LESS_EQUAL_NORMAL_INTEGER_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::LessEqual,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::MaySuspend,
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
        DynamicOperatorSuspensionV1::MaySuspend,
        None,
    );

const MUL_NORMAL_INTEGER_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Mul,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::NormalInteger,
        DynamicOperatorSuspensionV1::MaySuspend,
        None,
    );

/// The bounded borrowed-value/null equality: one operand is the borrowed
/// tagged carrier, the other the exact `null` literal producer, in either
/// source order. Equality reads the carrier kind — Null is true, every
/// well-formed non-null kind is false — never the payload as Integer, so
/// no TypeError arises for the admitted non-null kinds. The result is a
/// TrivialBool with no lifecycle obligation, and the read-only evaluation
/// completes at the expression site without suspending.
const EQUAL_DYNAMIC_NULL: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Equal,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Null,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::NonSuspending,
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
            DynamicOperatorFamilyV1::GreaterEqual,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&GREATER_EQUAL_NORMAL_INTEGER_NORMAL_INTEGER),
        (
            DynamicOperatorFamilyV1::LessEqual,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&LESS_EQUAL_NORMAL_INTEGER_NORMAL_INTEGER),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&ADD_NORMAL_INTEGER_NORMAL_INTEGER),
        (
            DynamicOperatorFamilyV1::Mul,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&MUL_NORMAL_INTEGER_NORMAL_INTEGER),
        // The borrowed carrier and the exact null literal share one
        // envelope in either source operand order.
        (
            DynamicOperatorFamilyV1::Equal,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Null,
        )
        | (
            DynamicOperatorFamilyV1::Equal,
            DynamicOperatorValueClassV1::Null,
            DynamicOperatorValueClassV1::Dynamic,
        ) => Ok(&EQUAL_DYNAMIC_NULL),
        _ => Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain),
    }
}

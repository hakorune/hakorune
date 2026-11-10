use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;

use super::{
    DynamicOperatorDomainV1, DynamicOperatorFamilyV1, DynamicOperatorFaultV1,
    DynamicOperatorNormalResultV1, DynamicOperatorSuspensionV1, DynamicOperatorValueClassV1,
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

const SUB_DYNAMIC_I64: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Sub,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier,
        DynamicOperatorSuspensionV1::MaySuspend,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

const ADD_STRING_STRING: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::String,
        ),
        DynamicOperatorNormalResultV1::OwnedStringValue,
        DynamicOperatorSuspensionV1::MaySuspend,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

const ADD_STRING_NORMAL_INTEGER: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
        DynamicOperatorNormalResultV1::OwnedStringValue,
        DynamicOperatorSuspensionV1::MaySuspend,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

/// An exact Bool displays as its language spelling; the result is a fresh String.
const ADD_STRING_BOOL: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::Bool,
        ),
        DynamicOperatorNormalResultV1::OwnedStringValue,
        DynamicOperatorSuspensionV1::MaySuspend,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

const ADD_STRING_DYNAMIC: VerifiedDynamicOperatorExecutionEnvelopeV1 =
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::Dynamic,
        ),
        DynamicOperatorNormalResultV1::OwnedStringValue,
        DynamicOperatorSuspensionV1::MaySuspend,
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded),
    );

/// Exact scalar operands use their existing language comparison laws.
/// The source class and family stay attached to the complete borrowed contract.
const fn exact_comparison(family: DynamicOperatorFamilyV1, class: DynamicOperatorValueClassV1) -> VerifiedDynamicOperatorExecutionEnvelopeV1 {
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed_with_fault(
        DynamicOperatorDomainV1::new(family, class, class),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::NonSuspending,
        None,
        DynamicOperatorFaultV1::NoSourceFault,
    )
}
const STRING_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 6] = [
    exact_comparison(DynamicOperatorFamilyV1::Equal, DynamicOperatorValueClassV1::String),
    exact_comparison(DynamicOperatorFamilyV1::NotEqual, DynamicOperatorValueClassV1::String),
    exact_comparison(DynamicOperatorFamilyV1::Less, DynamicOperatorValueClassV1::String),
    exact_comparison(DynamicOperatorFamilyV1::Greater, DynamicOperatorValueClassV1::String),
    exact_comparison(DynamicOperatorFamilyV1::GreaterEqual, DynamicOperatorValueClassV1::String),
    exact_comparison(DynamicOperatorFamilyV1::LessEqual, DynamicOperatorValueClassV1::String),
];

const I64_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 6] = [
    exact_comparison(DynamicOperatorFamilyV1::Equal, DynamicOperatorValueClassV1::I64),
    exact_comparison(DynamicOperatorFamilyV1::NotEqual, DynamicOperatorValueClassV1::I64),
    exact_comparison(DynamicOperatorFamilyV1::Less, DynamicOperatorValueClassV1::I64),
    exact_comparison(DynamicOperatorFamilyV1::Greater, DynamicOperatorValueClassV1::I64),
    exact_comparison(DynamicOperatorFamilyV1::GreaterEqual, DynamicOperatorValueClassV1::I64),
    exact_comparison(DynamicOperatorFamilyV1::LessEqual, DynamicOperatorValueClassV1::I64),
];

/// Only equality is defined for exact Boolean operands.
const BOOL_EQUALITIES: [VerifiedDynamicOperatorExecutionEnvelopeV1; 2] = [
    exact_comparison(DynamicOperatorFamilyV1::Equal, DynamicOperatorValueClassV1::Bool),
    exact_comparison(DynamicOperatorFamilyV1::NotEqual, DynamicOperatorValueClassV1::Bool),
];

const fn checked_comparison(
    family: DynamicOperatorFamilyV1,
    left: DynamicOperatorValueClassV1,
    right: DynamicOperatorValueClassV1,
) -> VerifiedDynamicOperatorExecutionEnvelopeV1 {
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed(
        DynamicOperatorDomainV1::new(family, left, right),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::MaySuspend,
        None,
    )
}
const LESS_DYNAMIC_DYNAMIC: VerifiedDynamicOperatorExecutionEnvelopeV1 = checked_comparison(
    DynamicOperatorFamilyV1::Less, DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::Dynamic,
);
const DYNAMIC_INTEGER_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 3] = [
    checked_comparison(DynamicOperatorFamilyV1::Less, DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::I64),
    checked_comparison(DynamicOperatorFamilyV1::Greater, DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::I64),
    checked_comparison(DynamicOperatorFamilyV1::Less, DynamicOperatorValueClassV1::I64, DynamicOperatorValueClassV1::Dynamic),
];

const fn dynamic_string_comparison(
    family: DynamicOperatorFamilyV1,
    left: DynamicOperatorValueClassV1,
    right: DynamicOperatorValueClassV1,
) -> VerifiedDynamicOperatorExecutionEnvelopeV1 {
    let equality = matches!(
        family,
        DynamicOperatorFamilyV1::Equal | DynamicOperatorFamilyV1::NotEqual
    );
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed_with_fault(
        DynamicOperatorDomainV1::new(family, left, right),
        DynamicOperatorNormalResultV1::TrivialBool,
        if equality {
            DynamicOperatorSuspensionV1::NonSuspending
        } else {
            DynamicOperatorSuspensionV1::MaySuspend
        },
        None,
        if equality {
            DynamicOperatorFaultV1::MalformedCarrierBeforeResultNoOperandMutationNoRebind
        } else {
            DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        },
    )
}

const DYNAMIC_STRING_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 6] = [
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Equal,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::NotEqual,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Less,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Greater,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::GreaterEqual,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::LessEqual,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::String,
    ),
];

const STRING_DYNAMIC_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 6] = [
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Equal,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::NotEqual,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Less,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::Greater,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::GreaterEqual,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
    dynamic_string_comparison(
        DynamicOperatorFamilyV1::LessEqual,
        DynamicOperatorValueClassV1::String,
        DynamicOperatorValueClassV1::Dynamic,
    ),
];

const fn comparison_index(family: DynamicOperatorFamilyV1) -> Option<usize> {
    match family {
        DynamicOperatorFamilyV1::Equal => Some(0),
        DynamicOperatorFamilyV1::NotEqual => Some(1),
        DynamicOperatorFamilyV1::Less => Some(2),
        DynamicOperatorFamilyV1::Greater => Some(3),
        DynamicOperatorFamilyV1::GreaterEqual => Some(4),
        DynamicOperatorFamilyV1::LessEqual => Some(5),
        _ => None,
    }
}

/// The checked compare lends a Normal-Integer operand view: both operands
/// must prove the logical signed-integer class, and a fault arrives before
/// any result without operand mutation or rebind.
/// Inclusive checked comparison keeps the same borrowed operand and Fault
/// laws; the Boolean result does not change either operand's Integer class.
const fn checked_integer_comparison(family: DynamicOperatorFamilyV1) -> VerifiedDynamicOperatorExecutionEnvelopeV1 {
    checked_comparison(family, DynamicOperatorValueClassV1::NormalInteger, DynamicOperatorValueClassV1::NormalInteger)
}
const CHECKED_INTEGER_COMPARISONS: [VerifiedDynamicOperatorExecutionEnvelopeV1; 6] = [
    checked_integer_comparison(DynamicOperatorFamilyV1::Equal),
    checked_integer_comparison(DynamicOperatorFamilyV1::NotEqual),
    checked_integer_comparison(DynamicOperatorFamilyV1::Less),
    checked_integer_comparison(DynamicOperatorFamilyV1::Greater),
    checked_integer_comparison(DynamicOperatorFamilyV1::GreaterEqual),
    checked_integer_comparison(DynamicOperatorFamilyV1::LessEqual),
];

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
    VerifiedDynamicOperatorExecutionEnvelopeV1::sealed_with_fault(
        DynamicOperatorDomainV1::new(
            DynamicOperatorFamilyV1::Equal,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Null,
        ),
        DynamicOperatorNormalResultV1::TrivialBool,
        DynamicOperatorSuspensionV1::NonSuspending,
        None,
        DynamicOperatorFaultV1::NoSourceFault,
    );

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DynamicOperatorEnvelopeIssueV1 {
    UnsupportedDomain,
}

pub(crate) const fn issue_dynamic_operator_execution_envelope_v1(
    domain: DynamicOperatorDomainV1,
) -> Result<&'static VerifiedDynamicOperatorExecutionEnvelopeV1, DynamicOperatorEnvelopeIssueV1> {
    if let Some(index) = comparison_index(domain.family()) {
        match (domain.left(), domain.right()) {
            (DynamicOperatorValueClassV1::String, DynamicOperatorValueClassV1::String) => return Ok(&STRING_COMPARISONS[index]),
            (DynamicOperatorValueClassV1::I64, DynamicOperatorValueClassV1::I64) => return Ok(&I64_COMPARISONS[index]),
            (DynamicOperatorValueClassV1::NormalInteger, DynamicOperatorValueClassV1::NormalInteger) => return Ok(&CHECKED_INTEGER_COMPARISONS[index]),
            (DynamicOperatorValueClassV1::Bool, DynamicOperatorValueClassV1::Bool) if index < BOOL_EQUALITIES.len() => return Ok(&BOOL_EQUALITIES[index]),
            (DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::String) => return Ok(&DYNAMIC_STRING_COMPARISONS[index]),
            (DynamicOperatorValueClassV1::String, DynamicOperatorValueClassV1::Dynamic) => return Ok(&STRING_DYNAMIC_COMPARISONS[index]),
            (DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::Dynamic) if index == 2 => return Ok(&LESS_DYNAMIC_DYNAMIC),
            (DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::I64) if index == 2 => return Ok(&DYNAMIC_INTEGER_COMPARISONS[0]),
            (DynamicOperatorValueClassV1::Dynamic, DynamicOperatorValueClassV1::I64) if index == 3 => return Ok(&DYNAMIC_INTEGER_COMPARISONS[1]),
            (DynamicOperatorValueClassV1::I64, DynamicOperatorValueClassV1::Dynamic) if index == 2 => return Ok(&DYNAMIC_INTEGER_COMPARISONS[2]),
            _ => {}
        }
    }
    match (domain.family(), domain.left(), domain.right()) {
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::String,
        ) => Ok(&ADD_STRING_STRING),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::NormalInteger,
        ) => Ok(&ADD_STRING_NORMAL_INTEGER),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::Bool,
        ) => Ok(&ADD_STRING_BOOL),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::String,
            DynamicOperatorValueClassV1::Dynamic,
        ) => Ok(&ADD_STRING_DYNAMIC),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ) => Ok(&ADD_DYNAMIC_I64),
        (
            DynamicOperatorFamilyV1::Sub,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ) => Ok(&SUB_DYNAMIC_I64),
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

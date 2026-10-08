use super::issuer::{issue_dynamic_operator_execution_envelope_v1, DynamicOperatorEnvelopeIssueV1};
use super::model::{
    DynamicOperatorControlV1, DynamicOperatorEffectV1, DynamicOperatorFaultV1,
    DynamicOperatorInputAccessV1, DynamicOperatorOrderingV1, DynamicOperatorSuspensionV1,
};
use super::*;
use crate::mir::dynamic_carrier_contract::DynamicCarrierLifecycleObligationV1;

fn issue(
    family: DynamicOperatorFamilyV1,
    left: DynamicOperatorValueClassV1,
    right: DynamicOperatorValueClassV1,
) -> Result<&'static VerifiedDynamicOperatorExecutionEnvelopeV1, DynamicOperatorEnvelopeIssueV1> {
    issue_dynamic_operator_execution_envelope_v1(DynamicOperatorDomainV1::new(family, left, right))
}

#[test]
fn add_issues_one_complete_non_aliasing_carrier_contract() {
    let envelope = issue(
        DynamicOperatorFamilyV1::Add,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::I64,
    )
    .unwrap();

    assert_eq!(envelope.effect(), DynamicOperatorEffectV1::OpaqueObservable);
    assert_eq!(
        envelope.ordering(),
        DynamicOperatorOrderingV1::SynchronousNonDetached
    );
    assert_eq!(
        envelope.suspension(),
        DynamicOperatorSuspensionV1::MaySuspend
    );
    assert_eq!(
        envelope.control(),
        DynamicOperatorControlV1::ExpressionBounded
    );
    assert_eq!(
        envelope.input_access(),
        DynamicOperatorInputAccessV1::BorrowedNoEscapeForOperation
    );
    assert_eq!(
        envelope.normal_result(),
        DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier
    );
    assert_eq!(
        envelope.fault(),
        DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
    );
    assert_eq!(
        envelope.lifecycle(),
        Some(DynamicCarrierLifecycleObligationV1::EndExactlyOnceUnlessForwarded)
    );
}

#[test]
fn less_domains_issue_trivial_bool_without_carrier_lifecycle() {
    for right in [
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::I64,
    ] {
        let envelope = issue(
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::Dynamic,
            right,
        )
        .unwrap();
        assert_eq!(
            envelope.normal_result(),
            DynamicOperatorNormalResultV1::TrivialBool
        );
        assert_eq!(envelope.lifecycle(), None);
        assert_eq!(
            envelope.fault(),
            DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        );
    }
}

/// The bounded borrowed-value/null equality: one envelope in either
/// operand order, TrivialBool result, read-only operands, no suspension
/// and no lifecycle obligation — Null is the only kind answering true.
#[test]
fn equal_dynamic_null_issues_non_suspending_trivial_bool() {
    for (left, right) in [
        (
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Null,
        ),
        (
            DynamicOperatorValueClassV1::Null,
            DynamicOperatorValueClassV1::Dynamic,
        ),
    ] {
        let envelope = issue(DynamicOperatorFamilyV1::Equal, left, right).unwrap();
        assert_eq!(
            envelope.normal_result(),
            DynamicOperatorNormalResultV1::TrivialBool
        );
        assert_eq!(envelope.lifecycle(), None);
        assert_eq!(
            envelope.suspension(),
            DynamicOperatorSuspensionV1::NonSuspending
        );
        assert_eq!(
            envelope.input_access(),
            DynamicOperatorInputAccessV1::BorrowedNoEscapeForOperation
        );
        assert_eq!(
            envelope.fault(),
            DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        );
    }
    // General tagged equality and the Integer-Zero literal stay outside
    // the bounded domain.
    for (left, right) in [
        (
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Dynamic,
        ),
        (
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        (
            DynamicOperatorValueClassV1::I64,
            DynamicOperatorValueClassV1::Null,
        ),
        (
            DynamicOperatorValueClassV1::Null,
            DynamicOperatorValueClassV1::NormalInteger,
        ),
    ] {
        assert_eq!(
            issue(DynamicOperatorFamilyV1::Equal, left, right),
            Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain)
        );
    }
}

#[test]
fn unsupported_domains_fail_without_fallback() {
    for domain in [
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::Dynamic,
        ),
        (
            DynamicOperatorFamilyV1::Add,
            DynamicOperatorValueClassV1::I64,
            DynamicOperatorValueClassV1::Dynamic,
        ),
        (
            DynamicOperatorFamilyV1::Less,
            DynamicOperatorValueClassV1::I64,
            DynamicOperatorValueClassV1::I64,
        ),
        (
            DynamicOperatorFamilyV1::Mul,
            DynamicOperatorValueClassV1::Dynamic,
            DynamicOperatorValueClassV1::I64,
        ),
        (
            DynamicOperatorFamilyV1::Mul,
            DynamicOperatorValueClassV1::I64,
            DynamicOperatorValueClassV1::I64,
        ),
        (
            DynamicOperatorFamilyV1::Mul,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::Null,
        ),
    ] {
        assert_eq!(
            issue(domain.0, domain.1, domain.2),
            Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain)
        );
    }
}

#[test]
fn checked_integer_arithmetic_produces_fresh_result_under_one_complete_contract() {
    for family in [DynamicOperatorFamilyV1::Add, DynamicOperatorFamilyV1::Mul] {
        let domain = DynamicOperatorDomainV1::new(
            family,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        );
        let envelope = issue_dynamic_operator_execution_envelope_v1(domain).unwrap();
        assert!(std::ptr::eq(
            envelope,
            issue_dynamic_operator_execution_envelope_v1(domain).unwrap()
        ));
        assert_eq!(envelope.domain(), domain);
        assert_eq!(
            envelope.normal_result(),
            DynamicOperatorNormalResultV1::NormalInteger
        );
        assert_eq!(
            envelope.input_access(),
            DynamicOperatorInputAccessV1::BorrowedNoEscapeForOperation
        );
        assert_eq!(
            envelope.ordering(),
            DynamicOperatorOrderingV1::SynchronousNonDetached
        );
        assert_eq!(
            envelope.suspension(),
            DynamicOperatorSuspensionV1::MaySuspend
        );
        assert_eq!(
            envelope.control(),
            DynamicOperatorControlV1::ExpressionBounded
        );
        assert_eq!(envelope.effect(), DynamicOperatorEffectV1::OpaqueObservable);
        assert_eq!(
            envelope.fault(),
            DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        );
        assert_eq!(envelope.lifecycle(), None);
    }
}

#[test]
fn module_contains_no_partial_or_physical_authority() {
    let model = include_str!("model.rs");
    let issuer = include_str!("issuer.rs");
    for forbidden in [
        "dynamic_invocation_contract",
        "LoopRecipe",
        "MirType",
        "ValueId",
        "BasicBlockId",
        "provider",
        "runtime tag",
        "retry",
        "fallback",
    ] {
        assert!(!model.contains(forbidden), "model contains {forbidden}");
        assert!(!issuer.contains(forbidden), "issuer contains {forbidden}");
    }
    assert!(!model.contains("pub(crate) const fn sealed"));
}

#[test]
fn checked_integer_comparisons_preserve_complete_borrowed_normal_fault_contract() {
    for family in [
        DynamicOperatorFamilyV1::Greater,
        DynamicOperatorFamilyV1::LessEqual,
    ] {
        let domain = DynamicOperatorDomainV1::new(
            family,
            DynamicOperatorValueClassV1::NormalInteger,
            DynamicOperatorValueClassV1::NormalInteger,
        );
        let envelope = issue_dynamic_operator_execution_envelope_v1(domain).unwrap();
        assert!(std::ptr::eq(
            envelope,
            issue_dynamic_operator_execution_envelope_v1(domain).unwrap()
        ));
        assert_eq!(envelope.domain(), domain);
        assert_eq!(envelope.effect(), DynamicOperatorEffectV1::OpaqueObservable);
        assert_eq!(
            envelope.ordering(),
            DynamicOperatorOrderingV1::SynchronousNonDetached
        );
        assert_eq!(
            envelope.suspension(),
            DynamicOperatorSuspensionV1::MaySuspend
        );
        assert_eq!(
            envelope.control(),
            DynamicOperatorControlV1::ExpressionBounded
        );
        assert_eq!(
            envelope.input_access(),
            DynamicOperatorInputAccessV1::BorrowedNoEscapeForOperation
        );
        assert_eq!(
            envelope.normal_result(),
            DynamicOperatorNormalResultV1::TrivialBool
        );
        assert_eq!(
            envelope.fault(),
            DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
        );
        assert_eq!(envelope.lifecycle(), None);
    }
}

#[test]
fn less_equal_refuses_every_non_integer_operand_domain_without_borrowing_generic_less() {
    let classes = [
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::I64,
        DynamicOperatorValueClassV1::NormalInteger,
        DynamicOperatorValueClassV1::Null,
    ];
    for left in classes {
        for right in classes {
            if left == DynamicOperatorValueClassV1::NormalInteger && right == left {
                continue;
            }
            assert_eq!(
                issue(DynamicOperatorFamilyV1::LessEqual, left, right),
                Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain),
                "{left:?}/{right:?}"
            );
        }
    }
    let general_less = issue(
        DynamicOperatorFamilyV1::Less,
        DynamicOperatorValueClassV1::Dynamic,
        DynamicOperatorValueClassV1::Dynamic,
    )
    .unwrap();
    assert_eq!(
        general_less.domain().family(),
        DynamicOperatorFamilyV1::Less
    );
    assert_ne!(
        general_less.domain().left(),
        DynamicOperatorValueClassV1::NormalInteger
    );
}

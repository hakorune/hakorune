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
    ] {
        assert_eq!(
            issue(domain.0, domain.1, domain.2),
            Err(DynamicOperatorEnvelopeIssueV1::UnsupportedDomain)
        );
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

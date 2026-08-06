use super::*;
use crate::mir::numeric_substrate::{NumericResolvedWidth, NumericSignedness};

fn plain_view(value: i128) -> GenericG0NumericSourceViewV1<'static> {
    let parameters = vec![
        GenericG0NumericParameterInputV1 {
            index: 0,
            declared_type_name: Some("i64"),
        },
        GenericG0NumericParameterInputV1 {
            index: 1,
            declared_type_name: Some("i64"),
        },
    ]
    .into_boxed_slice();
    let roles = [
        GenericG0NumericLiteralRoleV1::OuterConditionRhs,
        GenericG0NumericLiteralRoleV1::InnerConditionRhs,
        GenericG0NumericLiteralRoleV1::OuterUpdateRhs,
        GenericG0NumericLiteralRoleV1::InnerUpdateRhs,
    ];
    let literals = roles
        .into_iter()
        .map(|role| GenericG0NumericLiteralInputV1 {
            role,
            value,
            explicit_type_name: None,
            contextual_parameter_index: Some(match role {
                GenericG0NumericLiteralRoleV1::OuterConditionRhs
                | GenericG0NumericLiteralRoleV1::OuterUpdateRhs => 0,
                GenericG0NumericLiteralRoleV1::InnerConditionRhs
                | GenericG0NumericLiteralRoleV1::InnerUpdateRhs => 1,
            }),
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();
    GenericG0NumericSourceViewV1 {
        target: Some(NumericTarget::host()),
        parameters,
        literals,
    }
}

#[test]
fn plain_i64_literals_issue_exact_numeric_lease() {
    let lease = issue_generic_g0_numeric_fact_lease_v1(plain_view(3)).expect("numeric lease");
    assert_eq!(lease.parameters().len(), 2);
    assert_eq!(lease.literals().len(), 4);
    assert_eq!(lease.literals()[0].value, 3);
    assert_eq!(lease.literals()[0].kind.width, NumericResolvedWidth::Bits64);
    assert_eq!(
        lease.literals()[0].kind.signedness,
        NumericSignedness::Signed
    );
}

#[test]
fn typed_literal_is_rejected_as_out_of_profile() {
    let mut view = plain_view(3);
    view.literals[0].explicit_type_name = Some("i64");
    assert_eq!(
        issue_generic_g0_numeric_fact_lease_v1(view),
        Err(GenericG0NumericIssueV1::Rejected(
            GenericG0NumericRejectV1::TypedLiteralOutOfProfile {
                role: GenericG0NumericLiteralRoleV1::OuterConditionRhs,
            }
        ))
    );
}

#[test]
fn i64_range_overflow_is_rejected_without_truncation() {
    let view = plain_view(i128::from(i64::MAX) + 1);
    assert!(matches!(
        issue_generic_g0_numeric_fact_lease_v1(view),
        Err(GenericG0NumericIssueV1::Rejected(
            GenericG0NumericRejectV1::LiteralOutOfRange { .. }
        ))
    ));
}

#[test]
fn opaque_context_is_unresolved_at_neutral_boundary() {
    let mut view = plain_view(3);
    view.literals[0].contextual_parameter_index = None;
    assert_eq!(
        issue_generic_g0_numeric_fact_lease_v1(view),
        Err(GenericG0NumericIssueV1::Unresolved(
            GenericG0NumericUnresolvedV1::MissingLiteralContext {
                role: GenericG0NumericLiteralRoleV1::OuterConditionRhs,
            }
        ))
    );
}

#[test]
fn known_non_i64_parameter_is_rejected() {
    let mut view = plain_view(3);
    view.parameters[0].declared_type_name = Some("i32");
    assert!(matches!(
        issue_generic_g0_numeric_fact_lease_v1(view),
        Err(GenericG0NumericIssueV1::Rejected(
            GenericG0NumericRejectV1::ParameterTypeMismatch { .. }
        ))
    ));
}

#[test]
fn unknown_target_is_unresolved_instead_of_host_fallback() {
    let mut view = plain_view(3);
    view.target = None;
    assert_eq!(
        issue_generic_g0_numeric_fact_lease_v1(view),
        Err(GenericG0NumericIssueV1::Unresolved(
            GenericG0NumericUnresolvedV1::UnknownTarget
        ))
    );
}

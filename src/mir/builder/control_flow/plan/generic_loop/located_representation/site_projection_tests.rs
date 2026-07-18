//! SITEPROJ0-P0 compact nested-body projection matrix.

use crate::mir::builder::control_flow::plan::{
    LocatedLoopPlanExpressionPortV1, LoopPlanExpressionPortErrorV1,
};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, CallableResultLegacyLocationErrorV1,
    VerifiedCallableResultLegacySourceViewV1,
};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1, SourcePathSegmentV1};

#[test]
fn actual_loop_projection_matches_selected_site_and_nested_branch_paths() {
    let plan = actual_parser_add_fixture::plan();
    let caller = actual_parser_add_fixture::caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let loop_statement = view.body_stmt(&root, 4).unwrap();
    let condition = view
        .child_expr_from_stmt(&loop_statement, ExprChildRoleV1::LoopCondition)
        .unwrap();
    assert_path(
        &condition,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopCondition,
        ],
    );
    let body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    let port = LocatedLoopPlanExpressionPortV1::new(view);

    let cleanup = port.exact_body_stmt(&body, 5).unwrap();
    let cleanup_value = port
        .exact_child_expr_from_stmt(&cleanup, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    assert_eq!(
        cleanup_value.activation_site().unwrap().1,
        &actual_parser_add_fixture::selected_static_sites()[1]
    );

    let join_if = port.exact_body_stmt(&body, 4).unwrap();
    let then_body = port
        .exact_child_body_from_stmt(&join_if, BodyChildRoleV1::IfThen)
        .unwrap();
    let then_statement = port.exact_body_stmt(&then_body, 0).unwrap();
    let then_value = port
        .exact_child_expr_from_stmt(&then_statement, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    assert_path(
        &then_value,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(4),
            SourcePathSegmentV1::IfThen(0),
            SourcePathSegmentV1::Value,
        ],
    );

    let else_body = port
        .exact_child_body_from_stmt(&join_if, BodyChildRoleV1::IfElse)
        .unwrap();
    let else_statement = port.exact_body_stmt(&else_body, 0).unwrap();
    let else_value = port
        .exact_child_expr_from_stmt(&else_statement, ExprChildRoleV1::AssignmentValue)
        .unwrap();
    assert_path(
        &else_value,
        &[
            SourcePathSegmentV1::Body(4),
            SourcePathSegmentV1::LoopBody(4),
            SourcePathSegmentV1::IfElse(0),
            SourcePathSegmentV1::Value,
        ],
    );
}

#[test]
fn compact_projection_rejects_foreign_unlocated_root_and_invalid_ordinals() {
    let plan = actual_parser_add_fixture::plan();
    let caller = actual_parser_add_fixture::caller(&plan);
    let view = VerifiedCallableResultLegacySourceViewV1::verify(&plan, &caller).unwrap();
    let root = view.root_body();
    let loop_statement = view.body_stmt(&root, 4).unwrap();
    let loop_body = view
        .child_body_from_stmt(&loop_statement, BodyChildRoleV1::LoopBody)
        .unwrap();
    let unlocated_loop = view.unlocated_expr(loop_statement.node());
    let unlocated_body = view
        .child_body(&unlocated_loop, BodyChildRoleV1::LoopBody)
        .unwrap();

    let foreign_plan = actual_parser_add_fixture::plan();
    let foreign_caller = actual_parser_add_fixture::caller(&foreign_plan);
    let foreign_view =
        VerifiedCallableResultLegacySourceViewV1::verify(&foreign_plan, &foreign_caller).unwrap();
    let foreign_root = foreign_view.root_body();
    let foreign_loop = foreign_view.body_stmt(&foreign_root, 4).unwrap();
    let foreign_body = foreign_view
        .child_body_from_stmt(&foreign_loop, BodyChildRoleV1::LoopBody)
        .unwrap();

    let port = LocatedLoopPlanExpressionPortV1::new(view);
    assert!(matches!(
        port.exact_body_stmt(&foreign_body, 0),
        Err(LoopPlanExpressionPortErrorV1::Located(
            CallableResultLegacyLocationErrorV1::ForeignCarrier { .. }
        ))
    ));
    assert!(matches!(
        port.exact_body_stmt(&unlocated_body, 0),
        Err(LoopPlanExpressionPortErrorV1::Located(
            CallableResultLegacyLocationErrorV1::UnlocatedCannotProveInactive
        ))
    ));
    assert!(matches!(
        port.exact_body_stmt(&root, 0),
        Err(LoopPlanExpressionPortErrorV1::Located(
            CallableResultLegacyLocationErrorV1::RootBodyRequestedAsChild(_)
        ))
    ));
    assert!(matches!(
        port.exact_body_stmt(&loop_body, usize::MAX),
        Err(LoopPlanExpressionPortErrorV1::Located(
            CallableResultLegacyLocationErrorV1::BodyIndexOverflow { .. }
        ))
    ));
    assert!(matches!(
        port.exact_body_stmt(&loop_body, 6),
        Err(LoopPlanExpressionPortErrorV1::Located(
            CallableResultLegacyLocationErrorV1::BodyIndexOutOfBounds { .. }
        ))
    ));
}

fn assert_path(
    expr: &crate::mir::callable_result_representation::LegacyExprInputV1<'_>,
    expected: &[SourcePathSegmentV1],
) {
    assert_eq!(
        expr.activation_site().unwrap().1.node().segments(),
        expected
    );
}

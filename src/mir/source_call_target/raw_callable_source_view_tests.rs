use std::ptr;

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::actual_parser_add_fixture;
use crate::mir::resolved_semantics::ExprChildRoleV1;
use crate::mir::source_call_target::{
    RawSourceCursorErrorV1, VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};

#[test]
fn raw_cursor_reaches_preloop_nested_instance_argument_with_exact_catalog_identity() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, _results| {
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw cursor");
            let exact_call =
                VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                    .expect("pre-loop source MethodCall");

            assert!(ptr::eq(view.catalog(), exact_call.catalog()));
            assert!(ptr::eq(view.caller(), exact_call.caller()));
            assert!(ptr::eq(view.declaration(), exact_call.declaration()));

            let body = view.root_body();
            let statement = view.body_stmt(&body, 3).expect("Body(3)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("Body(3).Value");
            let nested = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                .expect("Body(3).Value.Argument(1)");
            let input = view
                .method_call_input(&nested)
                .expect("exact nested MethodCall input");

            assert_eq!(nested.site(), &sites[0]);
            assert!(ptr::eq(nested.node(), exact_call.expression()));
            assert!(ptr::eq(input.view(), &view));
            assert!(ptr::eq(input.caller(), exact_call.caller()));
            assert_eq!(input.site(), &sites[0]);
            assert!(ptr::eq(input.node(), exact_call.expression()));
            assert!(matches!(input.receiver(), ASTNode::Me { .. }));
            assert_eq!(input.method(), "static_const_eval_pos");
            assert_eq!(input.arguments().len(), 1);
            assert!(matches!(
                nested.node(),
                ASTNode::MethodCall { method, .. } if method == "static_const_eval_pos"
            ));
        },
    );
}

#[test]
fn raw_cursor_rejects_wrong_role_and_out_of_bounds_before_lowering() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, _sites, _targets, _results| {
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw cursor");
            let body = view.root_body();
            assert!(matches!(
                view.body_stmt(&body, 99),
                Err(RawSourceCursorErrorV1::BodyIndexOutOfBounds { .. })
            ));

            let statement = view.body_stmt(&body, 3).expect("Body(3)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("Body(3).Value");
            let receiver = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::Receiver)
                .expect("Body(3).Value.Receiver");
            assert!(matches!(
                view.child_expr_from_stmt(&statement, ExprChildRoleV1::ReturnValue),
                Err(RawSourceCursorErrorV1::ExpressionRoleParentMismatch { .. })
            ));
            assert!(matches!(
                view.method_call_input(&receiver),
                Err(RawSourceCursorErrorV1::MethodCallRequired { .. })
            ));
        },
    );
}

#[test]
fn raw_cursor_uses_shared_body_role_vocabulary_for_loop_refresh_navigation() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, _results| {
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw cursor");
            let root = view.root_body();
            let loop_statement = view.body_stmt(&root, 4).expect("Body(4)");
            let loop_body = view
                .child_body_from_stmt(
                    &loop_statement,
                    crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
                )
                .expect("Body(4).LoopBodyRoot");
            assert_eq!(loop_body.statements().len(), 6);
            let statement = view.body_stmt(&loop_body, 5).expect("LoopBody(5)");
            let outer = view
                .child_expr_from_stmt(&statement, ExprChildRoleV1::AssignmentValue)
                .expect("LoopBody(5).Value");
            let nested = view
                .child_expr_from_expr(&outer, ExprChildRoleV1::CallArgument(1))
                .expect("LoopBody(5).Value.Argument(1)");

            assert_eq!(nested.site(), &sites[1]);
        },
    );
}

#[test]
fn raw_cursor_rejects_foreign_view_even_when_catalog_source_looks_equal() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |left_catalog, caller, _sites, _targets, _results| {
            actual_parser_add_fixture::with_instance_result_contract_inputs(
                |right_catalog, right_caller, _right_sites, _right_targets, _right_results| {
                    let left = VerifiedRawCallableSourceViewV1::verify(left_catalog, caller)
                        .expect("left cursor");
                    let right =
                        VerifiedRawCallableSourceViewV1::verify(right_catalog, right_caller)
                            .expect("right cursor");
                    assert!(!ptr::eq(left.catalog(), right.catalog()));
                    assert_eq!(left.caller(), right.caller());

                    let foreign_body = right.root_body();
                    assert!(matches!(
                        left.body_stmt(&foreign_body, 3),
                        Err(RawSourceCursorErrorV1::ForeignView { .. })
                    ));

                    let right_body = right.root_body();
                    let right_statement = right.body_stmt(&right_body, 3).expect("Body(3)");
                    let right_outer = right
                        .child_expr_from_stmt(&right_statement, ExprChildRoleV1::AssignmentValue)
                        .expect("Body(3).Value");
                    let right_nested = right
                        .child_expr_from_expr(&right_outer, ExprChildRoleV1::CallArgument(1))
                        .expect("Body(3).Value.Argument(1)");
                    assert!(matches!(
                        left.method_call_input(&right_nested),
                        Err(RawSourceCursorErrorV1::ForeignView { .. })
                    ));
                },
            );
        },
    );
}

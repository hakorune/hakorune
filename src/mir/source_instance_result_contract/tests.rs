use crate::mir::builder::{SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedUnannotatedCallableBodyResultOutcomeV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    RawLocatedMethodCallInputV1, VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::parser::NyashParser;

use super::{
    prepare_preloop_nested_result_association_v1, seal_nested_instance_result_contract,
    CurrentOwnerInstanceResultTargetErrorV1, PreloopNestedResultAssociationErrorV1,
    PreloopNestedResultAssociationStageV1, VerifiedCurrentOwnerInstanceResultTargetV1,
};

#[test]
fn actual_pre_loop_and_refresh_sites_seal_exact_integer_contracts() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |declarations, caller, sites, _targets, results| {
            for site in sites {
                let call = VerifiedSourceMethodCallSiteV1::verify(declarations, caller, site.clone())
                    .expect("actual nested MethodCall site");
                let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                    .expect("current-owner instance target");
                assert_eq!(target.target().key().owner(), "ParserBox");
                assert_eq!(target.target().key().name(), "static_const_eval_pos");
                assert_eq!(target.target().key().arity(), 1);

                let proof = results
                    .issue_unannotated_body_proof(target.target())
                    .expect("actual unannotated target proof");
                assert!(matches!(
                    proof.outcome(),
                    VerifiedUnannotatedCallableBodyResultOutcomeV1::ExactI64 {
                        required_i64_arguments
                    } if required_i64_arguments.is_empty()
                ));
                let contract = seal_nested_instance_result_contract(target, proof)
                    .expect("exact nested Integer contract");
                assert!(contract.result_is_integer());
                assert_eq!(contract.target().call().site(), site);
            }
        },
    );
}

#[test]
fn preloop_association_co_seals_only_the_exact_catalog_backed_method_call() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            let call = VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                .expect("selected pre-loop source MethodCall");
            let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                .expect("selected pre-loop target");
            let proof = results
                .issue_unannotated_body_proof(target.target())
                .expect("selected pre-loop Integer proof");
            let contract = seal_nested_instance_result_contract(target, proof)
                .expect("selected pre-loop Integer contract");
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let input = preloop_method_input(&view);

            let association = prepare_preloop_nested_result_association_v1(contract, input)
                .expect("exact pre-loop source association");
            assert_eq!(association.input().site(), &sites[0]);
            assert_eq!(association.contract().target().call().site(), &sites[0]);
            assert!(std::ptr::eq(
                association.contract().target().call().catalog(),
                association.input().view().catalog(),
            ));
            assert!(std::ptr::eq(
                association.contract().target().call().expression(),
                association.input().node(),
            ));
            association.discard();
        },
    );
}

#[test]
fn preloop_association_rejects_the_parked_loop_refresh_site_before_lowering() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            let call = VerifiedSourceMethodCallSiteV1::verify(catalog, caller, sites[0].clone())
                .expect("selected pre-loop source MethodCall");
            let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call)
                .expect("selected pre-loop target");
            let proof = results
                .issue_unannotated_body_proof(target.target())
                .expect("selected pre-loop Integer proof");
            let contract = seal_nested_instance_result_contract(target, proof)
                .expect("selected pre-loop Integer contract");
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let input = loop_refresh_method_input(&view);

            let rejected = prepare_preloop_nested_result_association_v1(contract, input)
                .expect_err("parked loop-refresh site cannot borrow pre-loop contract");
            assert_eq!(
                rejected.stage(),
                PreloopNestedResultAssociationStageV1::Site
            );
            assert_eq!(
                rejected.cause(),
                PreloopNestedResultAssociationErrorV1::SiteMismatch
            );
            rejected.discard();
        },
    );
}

#[test]
fn static_caller_is_rejected_by_instance_target_owner() {
    let source = "static box ParserBox { static_const_parse_add(text, pos) { return me.static_const_eval_pos(text) } static_const_eval_pos(ret) { return 0 } }";
    let root = NyashParser::parse_from_string(source).expect("fixture parse");
    let declarations = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&root)
        .expect("fixture declarations");
    let caller = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "ParserBox",
            "static_const_parse_add",
            2,
        )
        .expect("caller")
        .key()
        .clone();
    let call = VerifiedSourceMethodCallSiteV1::verify(
        &declarations,
        &caller,
        source_site(&[
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Value,
        ]),
    )
    .expect("call site");
    assert!(matches!(
        VerifiedCurrentOwnerInstanceResultTargetV1::seal(&call),
        Err(CurrentOwnerInstanceResultTargetErrorV1::CallerNotInstanceBoxMethod { .. })
    ));
}

fn source_site(segments: &[SourcePathSegmentV1]) -> SourceExprSiteV1 {
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments.to_vec()))
}

fn preloop_method_input<'view, 'catalog>(
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
) -> RawLocatedMethodCallInputV1<'view, 'catalog> {
    let body = view.root_body();
    let statement = view.body_stmt(&body, 3).expect("Body(3)");
    let outer = view
        .child_expr_from_stmt(
            &statement,
            crate::mir::resolved_semantics::ExprChildRoleV1::AssignmentValue,
        )
        .expect("Body(3).Value");
    let nested = view
        .child_expr_from_expr(
            &outer,
            crate::mir::resolved_semantics::ExprChildRoleV1::CallArgument(1),
        )
        .expect("Body(3).Value.Argument(1)");
    view.method_call_input(&nested)
        .expect("pre-loop nested MethodCall")
}

fn loop_refresh_method_input<'view, 'catalog>(
    view: &'view VerifiedRawCallableSourceViewV1<'catalog>,
) -> RawLocatedMethodCallInputV1<'view, 'catalog> {
    let root = view.root_body();
    let loop_statement = view.body_stmt(&root, 4).expect("Body(4)");
    let loop_body = view
        .child_body_from_stmt(
            &loop_statement,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .expect("Body(4).LoopBody");
    let statement = view.body_stmt(&loop_body, 5).expect("LoopBody(5)");
    let outer = view
        .child_expr_from_stmt(
            &statement,
            crate::mir::resolved_semantics::ExprChildRoleV1::AssignmentValue,
        )
        .expect("LoopBody(5).Value");
    let nested = view
        .child_expr_from_expr(
            &outer,
            crate::mir::resolved_semantics::ExprChildRoleV1::CallArgument(1),
        )
        .expect("LoopBody(5).Value.Argument(1)");
    view.method_call_input(&nested)
        .expect("loop-refresh nested MethodCall")
}

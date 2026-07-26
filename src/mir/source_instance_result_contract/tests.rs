use crate::mir::builder::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedUnannotatedCallableBodyResultOutcomeV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::{
    RawLocatedCallArgumentStageV1, RawLocatedMethodCallInputV1, RawSourceCursorErrorV1,
    VerifiedRawCallableSourceViewV1, VerifiedSourceMethodCallSiteV1,
};
use crate::parser::NyashParser;

use super::{
    prepare_preloop_located_argument_v1, prepare_preloop_nested_result_association_v1,
    seal_nested_instance_result_contract, CurrentOwnerInstanceResultTargetErrorV1,
    PreloopLocatedArgumentErrorV1, PreloopLocatedArgumentStageV1,
    PreloopNestedResultAssociationErrorV1, PreloopNestedResultAssociationStageV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
};

macro_rules! bind_selected_preloop_contract {
    ($call:ident, $contract:ident, $catalog:expr, $caller:expr, $site:expr, $results:expr) => {
        let $call = VerifiedSourceMethodCallSiteV1::verify($catalog, $caller, $site.clone())
            .expect("selected pre-loop source MethodCall");
        let target = VerifiedCurrentOwnerInstanceResultTargetV1::seal(&$call)
            .expect("selected pre-loop target");
        let proof = $results
            .issue_unannotated_body_proof(target.target())
            .expect("selected pre-loop Integer proof");
        let $contract = seal_nested_instance_result_contract(target, proof)
            .expect("selected pre-loop Integer contract");
    };
}

#[test]
fn actual_pre_loop_and_refresh_sites_seal_exact_integer_contracts() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |declarations, caller, sites, _targets, results| {
            for site in sites {
                let call =
                    VerifiedSourceMethodCallSiteV1::verify(declarations, caller, site.clone())
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
fn preloop_located_argument_co_seals_the_structural_outer_argument() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            bind_selected_preloop_contract!(call, contract, catalog, caller, &sites[0], results);
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let association =
                prepare_preloop_nested_result_association_v1(contract, preloop_method_input(&view))
                    .expect("exact pre-loop association");
            let selected = view
                .method_call_argument(preloop_outer_method_input(&view), 1)
                .expect("structural Argument(1)");

            let prepared = prepare_preloop_located_argument_v1(selected, association)
                .expect("exact outer/inner pre-loop relation");
            assert_eq!(prepared.selected().index(), 1);
            assert_eq!(prepared.selected().child().site(), &sites[0]);
            assert_eq!(prepared.association().input().site(), &sites[0]);
            prepared.discard();
        },
    );
}

#[test]
fn preloop_located_argument_rejects_out_of_range_before_any_lowering() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, _sites, _targets, _results| {
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let rejected = view
                .method_call_argument(preloop_outer_method_input(&view), 99)
                .expect_err("out-of-range argument must reject");

            assert_eq!(
                rejected.stage(),
                RawLocatedCallArgumentStageV1::ArgumentIndex
            );
            assert!(matches!(
                rejected.cause(),
                RawSourceCursorErrorV1::MethodCallArgumentIndexOutOfBounds {
                    index: 99,
                    len: 2,
                    ..
                }
            ));
            rejected.discard();
        },
    );
}

#[test]
fn preloop_located_argument_rejects_the_unselected_outer_argument() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            bind_selected_preloop_contract!(call, contract, catalog, caller, &sites[0], results);
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let association =
                prepare_preloop_nested_result_association_v1(contract, preloop_method_input(&view))
                    .expect("exact pre-loop association");
            let selected = view
                .method_call_argument(preloop_outer_method_input(&view), 0)
                .expect("structural Argument(0)");

            let rejected = prepare_preloop_located_argument_v1(selected, association)
                .expect_err("Argument(0) cannot carry the Argument(1) contract");
            assert_eq!(
                rejected.stage(),
                PreloopLocatedArgumentStageV1::SelectedSite
            );
            assert_eq!(
                rejected.cause(),
                PreloopLocatedArgumentErrorV1::SiteMismatch
            );
            rejected.discard();
        },
    );
}

#[test]
fn preloop_located_argument_rejects_an_equal_catalog_from_a_foreign_view() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            bind_selected_preloop_contract!(call, contract, catalog, caller, &sites[0], results);
            let association_view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("association source view");
            let selected_view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("independent source view");
            let association = prepare_preloop_nested_result_association_v1(
                contract,
                preloop_method_input(&association_view),
            )
            .expect("exact pre-loop association");
            let selected = selected_view
                .method_call_argument(preloop_outer_method_input(&selected_view), 1)
                .expect("structural Argument(1)");

            let rejected = prepare_preloop_located_argument_v1(selected, association)
                .expect_err("equal-looking but foreign view must reject");
            assert_eq!(rejected.stage(), PreloopLocatedArgumentStageV1::SourceView);
            assert_eq!(rejected.cause(), PreloopLocatedArgumentErrorV1::ForeignView);
            rejected.discard();
        },
    );
}

#[test]
fn preloop_located_argument_rejects_the_parked_loop_refresh_relation() {
    actual_parser_add_fixture::with_instance_result_contract_inputs(
        |catalog, caller, sites, _targets, results| {
            bind_selected_preloop_contract!(call, contract, catalog, caller, &sites[0], results);
            let view = VerifiedRawCallableSourceViewV1::verify(catalog, caller)
                .expect("catalog-backed Raw source view");
            let association =
                prepare_preloop_nested_result_association_v1(contract, preloop_method_input(&view))
                    .expect("exact pre-loop association");
            let selected = view
                .method_call_argument(loop_refresh_outer_method_input(&view), 1)
                .expect("loop-refresh Argument(1)");

            let rejected = prepare_preloop_located_argument_v1(selected, association)
                .expect_err("parked loop relation cannot carry pre-loop contract");
            assert_eq!(
                rejected.stage(),
                PreloopLocatedArgumentStageV1::SelectedSite
            );
            assert_eq!(
                rejected.cause(),
                PreloopLocatedArgumentErrorV1::SiteMismatch
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
        source_site(&[SourcePathSegmentV1::Body(0), SourcePathSegmentV1::Value]),
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

fn preloop_outer_method_input<'view, 'catalog>(
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
    view.method_call_input(&outer)
        .expect("pre-loop outer MethodCall")
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

fn loop_refresh_outer_method_input<'view, 'catalog>(
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
    view.method_call_input(&outer)
        .expect("loop-refresh outer MethodCall")
}

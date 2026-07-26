use crate::mir::builder::{SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1};
use crate::mir::callable_result_representation::{
    actual_parser_add_fixture, VerifiedUnannotatedCallableBodyResultOutcomeV1,
};
use crate::mir::resolved_semantics::{SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1};
use crate::mir::source_call_target::VerifiedSourceMethodCallSiteV1;
use crate::parser::NyashParser;

use super::{
    seal_nested_instance_result_contract, CurrentOwnerInstanceResultTargetErrorV1,
    VerifiedCurrentOwnerInstanceResultTargetV1,
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

use crate::mir::builder::{
    NormalCallableSemanticAdmissionV1, SameModuleCallableNamespaceV1,
    VerifiedNormalCallableSemanticSourceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::mir::source_call_target::{
    DynamicMemberSourceIssueV1, DynamicMemberSourceRejectV1, VerifiedQualifiedCallRouteFactsV1,
    VerifiedQualifiedReceiverLexicalDispositionsV1, VerifiedSourceCallTargetCatalogV1,
    VerifiedSourceMethodCallSiteV1, VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;

use super::*;

fn seal_source<'source>(
    program: &'source crate::ast::ASTNode,
    catalog: &VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedNormalCallableSemanticSourceV1<'source> {
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).unwrap();
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            program,
            catalog.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .unwrap()
    else {
        panic!("semantic source fixture must be complete")
    };
    source
}

fn empty_targets<'catalog>(
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceCallTargetCatalogV1<'catalog> {
    let imports = VerifiedStaticImportAliasViewV1::seal(catalog, std::iter::empty()).unwrap();
    VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty()).unwrap()
}

fn full_dynamic_targets<'catalog>(
    program: &crate::ast::ASTNode,
    catalog: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
) -> VerifiedSourceCallTargetCatalogV1<'catalog> {
    let source = seal_source(program, catalog);
    empty_targets(catalog)
        .extend_complete_dynamic_sources(&source)
        .expect("unchanged full source must issue Dynamic targets")
}

#[test]
fn unchanged_full_fixture_issues_one_complete_envelope_per_dynamic_target() {
    let program = NyashParser::parse_from_string(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let targets = full_dynamic_targets(&program, &declarations);
    let target_count = targets.len();
    let skip_while = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "ParserScanLoopBox",
            "skip_while",
            4,
        )
        .unwrap()
        .key()
        .clone();
    let envelopes =
        VerifiedDynamicInvocationEnvelopeCatalogV1::issue(targets, &declarations).unwrap();

    assert_eq!(
        envelopes.len(),
        7,
        "the unchanged full fixture is authority"
    );
    assert_eq!(envelopes.len(), target_count);
    assert_eq!(envelopes.envelopes().count(), envelopes.len());
    assert_eq!(envelopes.targets().len(), target_count);

    let mut skip_while_selectors = Vec::new();
    let mut contract_address = None;
    for row in envelopes.envelopes() {
        assert_eq!(row.site(), row.target().call_site());
        assert!(declarations.declaration(row.caller()).is_some());
        let contract = row.envelope();
        assert_eq!(
            contract.effect(),
            DynamicInvocationEffectV1::OpaqueObservable
        );
        assert_eq!(
            contract.ordering(),
            DynamicInvocationOrderingV1::SynchronousNonDetached
        );
        assert_eq!(
            contract.suspension(),
            DynamicInvocationSuspensionV1::MaySuspend
        );
        assert_eq!(
            contract.outcome(),
            DynamicInvocationOutcomeV1::NormalSelfContainedDynamicCarrierOrFault
        );
        assert_eq!(
            contract.control(),
            DynamicInvocationControlV1::CallableBounded
        );
        assert_eq!(
            contract.input_home(),
            DynamicInvocationInputHomeV1::BorrowedNoEscapeForInvocation
        );
        assert_eq!(
            contract.result_home(),
            DynamicInvocationResultHomeV1::SelfContainedDynamicCarrierToCaller
        );
        let address = contract as *const _;
        if let Some(expected) = contract_address {
            assert_eq!(address, expected, "all selectors share one contract");
        } else {
            contract_address = Some(address);
        }
        if row.caller() == &skip_while {
            skip_while_selectors.push((
                row.target().dispatch().selector().to_owned(),
                row.target().dispatch().arity(),
            ));
        }
    }
    skip_while_selectors.sort();
    assert_eq!(
        skip_while_selectors,
        vec![("indexOf".to_owned(), 1), ("substring".to_owned(), 2)]
    );
}

#[test]
fn equal_looking_foreign_declaration_catalog_is_rejected() {
    let text = include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
    let program = NyashParser::parse_from_string(text).unwrap();
    let foreign_program = NyashParser::parse_from_string(text).unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let foreign =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&foreign_program).unwrap();
    let targets = full_dynamic_targets(&program, &declarations);

    assert!(matches!(
        VerifiedDynamicInvocationEnvelopeCatalogV1::issue(targets, &foreign),
        Err(DynamicInvocationEnvelopeIssueV1::ForeignTargetCatalog)
    ));
}

#[test]
fn typed_non_dynamic_source_is_valid_and_yields_an_empty_envelope_catalog() {
    let program = NyashParser::parse_from_string(
        "static box TextUse { scan(src: String) { return src.length() } }",
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let targets = full_dynamic_targets(&program, &declarations);
    let envelopes =
        VerifiedDynamicInvocationEnvelopeCatalogV1::issue(targets, &declarations).unwrap();

    assert!(envelopes.is_empty());
    assert_eq!(envelopes.targets().len(), 0);
}

#[test]
fn static_target_rows_are_retained_but_not_given_dynamic_envelopes() {
    let program = NyashParser::parse_from_string(
        "static box Lib { value(x) { return x } }\n\
         static box Use { run(x) { return Lib.value(x) } }",
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let caller = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "Use",
            "run",
            1,
        )
        .unwrap()
        .key()
        .clone();
    let call_site = crate::mir::resolved_semantics::SourceExprSiteV1::from_node(
        crate::mir::resolved_semantics::SourceNodeSiteV1::from_segments(vec![
            crate::mir::resolved_semantics::SourcePathSegmentV1::Body(0),
            crate::mir::resolved_semantics::SourcePathSegmentV1::Value,
        ]),
    );
    let imports = VerifiedStaticImportAliasViewV1::seal(&declarations, []).unwrap();
    let call = VerifiedSourceMethodCallSiteV1::verify(&declarations, &caller, call_site).unwrap();
    let lexical = VerifiedQualifiedReceiverLexicalDispositionsV1::verify(&[&call]).unwrap();
    let facts = VerifiedQualifiedCallRouteFactsV1::verify(&call, &lexical, &imports).unwrap();
    let targets = VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, [facts]).unwrap();
    let envelopes =
        VerifiedDynamicInvocationEnvelopeCatalogV1::issue(targets, &declarations).unwrap();

    assert!(envelopes.is_empty());
    assert_eq!(envelopes.targets().len(), 1);
    assert_eq!(envelopes.targets().static_len(), 1);
}

#[test]
fn duplicate_dynamic_source_cannot_reach_a_second_envelope_row() {
    let program = NyashParser::parse_from_string(
        "static box Calls { run(src) { return src.substring(0, 1) } }",
    )
    .unwrap();
    let declarations =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program).unwrap();
    let source = seal_source(&program, &declarations);
    let targets = empty_targets(&declarations)
        .extend_complete_dynamic_sources(&source)
        .unwrap();

    assert!(matches!(
        targets.extend_complete_dynamic_sources(&source),
        Err(DynamicMemberSourceIssueV1::Rejected(
            DynamicMemberSourceRejectV1::DuplicateOrCollidingTarget { .. }
        ))
    ));
}

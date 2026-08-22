use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, ResolveFunctionErrorV1, ResolveOwnerForestErrorV1,
    ScriptResolverDeferredCauseV1, ScriptResolverDeferredSiteV1, ShadowResolveErrorV0,
    SourceBindingSiteV1, SourcePathSegmentV1, SourceResolverDeferredV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1,
    issue_resolved_callable_semantic_batch_with_brand_catalog_v1,
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};

fn batch(source: &str) -> VerifiedResolvedCallableSemanticBatchV1 {
    let source = final_source(source);
    let mut resolver = FunctionSemanticResolverSessionV1::new(71).unwrap();
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source).unwrap()
}

fn final_source(source: &str) -> crate::parser::VerifiedFinalCallableProgramSourceV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    })
}

#[test]
fn mixed_direct_methods_resolve_once_in_exact_source_order() {
    let batch = batch(
        "static box StaticApi { run(value) { return value } }\n\
         box InstanceApi { read() { return 1 } }",
    );
    let rows = batch.declarations().collect::<Vec<_>>();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].batch_slot(), 0);
    assert_eq!(
        rows[0].mode(),
        ResolvedCallableDeclarationModeV1::StaticBoxMethod
    );
    assert_eq!(rows[0].parameter_count(), 1);
    assert_eq!(rows[1].batch_slot(), 1);
    assert_eq!(
        rows[1].mode(),
        ResolvedCallableDeclarationModeV1::InstanceBoxMethod
    );
    assert_eq!(rows[1].parameter_count(), 0);
    assert_ne!(rows[0].owner(), rows[1].owner());
    assert_eq!(
        rows[0].owner().compilation_brand(),
        rows[1].owner().compilation_brand()
    );
}

#[test]
fn top_level_and_box_methods_share_one_complete_batch() {
    let batch = batch(
        "function helper(value) { return value }\n\
         static box StaticApi { run(value) { return value } }\n\
         box InstanceApi { read() { return 1 } }",
    );
    let rows = batch.declarations().collect::<Vec<_>>();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].batch_slot(), 0);
    assert_eq!(rows[0].mode(), ResolvedCallableDeclarationModeV1::TopLevel);
    assert_eq!(rows[0].parameter_count(), 1);
    assert_eq!(rows[1].batch_slot(), 1);
    assert_eq!(rows[2].batch_slot(), 2);

    batch
        .with_lowering_input(0, |input| {
            assert_eq!(input.owner(), rows[0].owner());
            assert_eq!(input.forest().roots(), [rows[0].owner()]);
        })
        .expect("top-level lowering input belongs to the complete batch");
}

#[test]
fn lowering_input_borrows_the_same_forest_owner_and_parameter_binding() {
    let batch = batch("static box Api { run(value) { return value } }");
    let row = batch.declarations().next().unwrap();
    batch
        .with_lowering_input(0, |input| {
            assert_eq!(input.owner(), row.owner());
            assert_eq!(input.function().function_origin(), row.function_origin());
            let binding = input
                .function()
                .declaration_binding(&SourceBindingSiteV1::Parameter { index: 0 })
                .expect("parameter binding");
            assert_eq!(binding.owner(), row.owner());
            assert_eq!(input.forest().roots(), [row.owner()]);
        })
        .unwrap();
}

#[test]
fn typed_parameter_spelling_survives_resolved_batch_loan() {
    let batch = batch("static box Api { run(value, pos: i64, end: i64, tail) { return value } }");
    batch
        .with_declaration_semantics(|view| {
            let parameters = view.declarations()[0]
                .parameters()
                .expect("direct method parameter source");
            assert_eq!(
                parameters
                    .iter()
                    .map(|parameter| parameter.declared_type_name())
                    .collect::<Vec<_>>(),
                [None, Some("i64"), Some("i64"), None]
            );
        })
        .expect("resolved declaration semantics");
}

#[test]
fn unchanged_parser_scan_loop_box_is_a_complete_four_row_batch() {
    let batch = batch(include_str!(
        "../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ));
    assert_eq!(
        batch
            .declarations()
            .map(|row| row.parameter_count())
            .collect::<Vec<_>>(),
        [4, 3, 4, 4]
    );
}

#[test]
fn missing_row_rejects_before_any_lowering_input_is_lent() {
    let batch = batch("static box Api { run() { return 1 } }");
    assert!(matches!(
        batch.with_lowering_input(1, |_| ()),
        Err(ResolvedCallableSemanticBatchLoanErrorV1::MissingSourceRow)
    ));
}

#[test]
fn deferred_batch_keeps_every_callable_identity_and_source_order() {
    let source = final_source(
        "static box Api {\n\
             first() { return missing_first }\n\
             second() { return missing_second }\n\
         }",
    );
    let identities = source
        .with_callable_semantic_syntax(|loan| {
            loan.rows()
                .iter()
                .map(|row| row.identity().clone())
                .collect::<Vec<_>>()
        })
        .expect("callable identity loan");
    let mut resolver = FunctionSemanticResolverSessionV1::new(72).unwrap();
    let deferred = match issue_resolved_callable_semantic_batch_v1(&mut resolver, source) {
        Err(super::ResolvedCallableSemanticBatchIssueV1::ResolverDeferred(deferred)) => deferred,
        other => panic!("expected identity-bound resolver deferral, got {other:?}"),
    };

    assert_eq!(deferred.len(), 2);
    for ((row, identity), expected_name) in std::iter::once(deferred.first())
        .chain(deferred.rest())
        .zip(identities.iter())
        .zip(["missing_first", "missing_second"])
    {
        assert!(row
            .source()
            .callable_identity()
            .expect("callable identity")
            .same_as(identity));
        match row.observation() {
            SourceResolverDeferredV1::Located {
                cause: ScriptResolverDeferredCauseV1::UnresolvedName { name },
                site: ScriptResolverDeferredSiteV1::Expression(_),
            } => assert_eq!(&**name, expected_name),
            other => panic!("expected located unresolved name, got {other:?}"),
        }
    }
}

#[test]
fn same_scope_redeclaration_keeps_its_unlocated_typed_cause() {
    let source = final_source("static box Api { run() { local x = 1 local x = 2 return x } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(73).unwrap();
    let deferred = match issue_resolved_callable_semantic_batch_v1(&mut resolver, source) {
        Err(super::ResolvedCallableSemanticBatchIssueV1::ResolverDeferred(deferred)) => deferred,
        other => panic!("expected typed redeclaration deferral, got {other:?}"),
    };

    assert_eq!(deferred.len(), 1);
    assert!(matches!(
        deferred.first().observation(),
        SourceResolverDeferredV1::UnlocatedSameScopeRedeclaration { name }
            if &**name == "x"
    ));
}

#[test]
fn parsed_standalone_block_resolves_with_exact_program_child_site() {
    let complete = batch("static box Api { run(x) { { local y = x } return x } }");
    assert_eq!(complete.declarations().len(), 1);

    let source = final_source("static box Api { run() { { return missing_inside_program } } }");
    let mut resolver = FunctionSemanticResolverSessionV1::new(74).unwrap();
    let deferred = match issue_resolved_callable_semantic_batch_v1(&mut resolver, source) {
        Err(super::ResolvedCallableSemanticBatchIssueV1::ResolverDeferred(deferred)) => deferred,
        other => panic!("expected Program child deferral, got {other:?}"),
    };
    match deferred.first().observation() {
        SourceResolverDeferredV1::Located {
            cause: ScriptResolverDeferredCauseV1::UnresolvedName { name },
            site: ScriptResolverDeferredSiteV1::Expression(site),
        } => {
            assert_eq!(&**name, "missing_inside_program");
            assert_eq!(
                site.node().segments(),
                [
                    SourcePathSegmentV1::Body(0),
                    SourcePathSegmentV1::ProgramBody(0),
                    SourcePathSegmentV1::Value,
                ]
            );
        }
        other => panic!("expected located Program child name, got {other:?}"),
    }
}

#[test]
fn construction_reject_keeps_the_exact_callable_identity() {
    let source = final_source(
        "brand PageId: i64\n\
         static box Api { bad() { return PageId(1, 2) } }",
    );
    let identity = source
        .with_callable_semantic_syntax(|loan| {
            loan.rows()
                .first()
                .expect("callable row")
                .identity()
                .clone()
        })
        .expect("callable identity loan");
    let catalog = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(
        source.ast(),
    )
    .expect("brand catalog");
    let mut resolver = FunctionSemanticResolverSessionV1::new(75).unwrap();
    let reject = match issue_resolved_callable_semantic_batch_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&catalog),
    ) {
        Err(super::ResolvedCallableSemanticBatchIssueV1::Resolver(reject)) => reject,
        other => panic!("expected source-bound construction reject, got {other:?}"),
    };

    assert!(reject
        .source()
        .callable_identity()
        .expect("callable identity")
        .same_as(&identity));
    assert!(matches!(
        reject.error(),
        ResolveOwnerForestErrorV1::Function(ResolveFunctionErrorV1::Syntax(
            ShadowResolveErrorV0::BrandConstructorArity { actual: 2, .. }
        ))
    ));
}

#[test]
fn program_contained_if_resolves_with_the_exact_callable_identity() {
    let source = final_source(
        "static box Api {\n\
             good() { return 0 }\n\
             bad() { { if 1 { return 1 } } return 0 }\n\
         }",
    );
    let identities = source
        .with_callable_semantic_syntax(|loan| {
            loan.rows()
                .iter()
                .map(|row| row.identity().clone())
                .collect::<Vec<_>>()
        })
        .expect("callable identity loan");
    let mut resolver = FunctionSemanticResolverSessionV1::new(76).unwrap();
    let batch = issue_resolved_callable_semantic_batch_v1(&mut resolver, source)
        .expect("Program-contained If resolves");
    let declarations = batch.declarations().collect::<Vec<_>>();
    assert_eq!(declarations.len(), 2);
    assert!(declarations[1].identity().same_as(&identities[1]));
}

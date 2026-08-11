use crate::mir::resolved_semantics::{FunctionSemanticResolverSessionV1, SourceBindingSiteV1};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{
    issue_resolved_callable_semantic_batch_v1, ResolvedCallableDeclarationModeV1,
    ResolvedCallableSemanticBatchLoanErrorV1, VerifiedResolvedCallableSemanticBatchV1,
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

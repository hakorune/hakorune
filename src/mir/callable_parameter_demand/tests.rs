use crate::mir::callable_semantic_batch::{
    issue_resolved_callable_semantic_batch_v1, VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, FunctionSemanticResolverSessionV1, HomeDemandV1, SourceBindingSiteV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

use super::{issue_callable_parameter_demands_v1, CallableParameterDeclarationModeV1};

fn batch(source: &str, compilation: u32) -> VerifiedResolvedCallableSemanticBatchV1 {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source parses");
    let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must remain source-backed")
        };
        source
    });
    let mut resolver =
        FunctionSemanticResolverSessionV1::new(compilation).expect("resolver session opens");
    issue_resolved_callable_semantic_batch_v1(&mut resolver, source)
        .expect("complete resolved callable semantic batch")
}

#[test]
fn projects_static_instance_and_zero_parameter_demands_from_one_batch() {
    let batch = batch(
        r#"
static box StaticApi {
    run(source, count: i64) { return count }
    ping() { return 0 }
}
box InstanceApi {
    read(offset) { return offset }
}
"#,
        7,
    );
    let catalog =
        issue_callable_parameter_demands_v1(&batch).expect("complete parameter demand projection");
    let declarations = catalog.declarations().collect::<Vec<_>>();

    assert_eq!(declarations.len(), 3);
    assert_eq!(
        declarations[0].mode(),
        CallableParameterDeclarationModeV1::StaticBoxMethod
    );
    assert_eq!(declarations[0].parameters().len(), 2);
    assert!(declarations[0]
        .parameters()
        .iter()
        .all(|row| row.demand() == HomeDemandV1::Handle));
    assert_eq!(declarations[1].parameters().len(), 0);
    assert_eq!(
        declarations[2].mode(),
        CallableParameterDeclarationModeV1::InstanceBoxMethod
    );
    assert_eq!(declarations[2].parameters().len(), 1);

    for declaration in declarations {
        batch
            .with_lowering_input(declaration.source_row_index(), |input| {
                assert_eq!(input.owner(), declaration.owner());
                assert_eq!(
                    input.function().function_origin(),
                    declaration.function_origin()
                );
                for parameter in declaration.parameters() {
                    assert_eq!(parameter.binding().owner(), declaration.owner());
                    assert_eq!(
                        input
                            .function()
                            .declaration_binding(&SourceBindingSiteV1::Parameter {
                                index: parameter.ordinal(),
                            })
                            .expect("exact parameter declaration binding"),
                        parameter.binding()
                    );
                    assert_eq!(
                        input
                            .function()
                            .binding(parameter.binding())
                            .expect("exact resolved parameter")
                            .kind(),
                        BindingKindV1::Parameter {
                            index: parameter.ordinal(),
                        }
                    );
                }
            })
            .expect("same-batch lowering loan");
    }
}

#[test]
fn parser_scan_loop_box_projects_all_fifteen_ordinary_demands() {
    let batch = batch(
        include_str!("../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"),
        11,
    );
    let catalog =
        issue_callable_parameter_demands_v1(&batch).expect("complete parameter demand projection");
    let declarations = catalog.declarations().collect::<Vec<_>>();

    assert_eq!(
        declarations
            .iter()
            .map(|row| row.parameters().len())
            .collect::<Vec<_>>(),
        [4, 3, 4, 4]
    );
    assert_eq!(
        declarations
            .iter()
            .map(|row| row.parameters().len())
            .sum::<usize>(),
        15
    );
    assert!(declarations.iter().all(|declaration| declaration
        .parameters()
        .iter()
        .all(|row| row.demand() == HomeDemandV1::Handle)));

    let skip_while = declarations[0];
    let position = &skip_while.parameters()[1];
    batch
        .with_lowering_input(skip_while.source_row_index(), |input| {
            assert_eq!(input.owner(), skip_while.owner());
            assert_eq!(
                input
                    .function()
                    .binding(position.binding())
                    .expect("position binding")
                    .diagnostic_name(),
                "pos"
            );
        })
        .expect("same-batch skip_while loan");
}

#[test]
fn foreign_semantic_batches_keep_distinct_parameter_identity() {
    let source = "static box Source { run(value) { return value } }";
    let first = batch(source, 13);
    let second = batch(source, 13);
    let first_catalog = issue_callable_parameter_demands_v1(&first).unwrap();
    let second_catalog = issue_callable_parameter_demands_v1(&second).unwrap();
    let first_row = first_catalog
        .declarations()
        .next()
        .expect("first declaration");
    let second_row = second_catalog
        .declarations()
        .next()
        .expect("second declaration");

    assert_ne!(first_row.owner(), second_row.owner());
    assert_ne!(
        first_row.parameters()[0].binding(),
        second_row.parameters()[0].binding()
    );
}

#[test]
fn projection_has_no_resolver_or_forest_authority() {
    let issuer = include_str!("issuer.rs");
    let model = include_str!("model.rs");

    for forbidden in [
        "resolve_selected_callable_forests",
        "FunctionSemanticResolverSessionV1",
        "VerifiedSemanticOwnerForestV1",
        "resolved_forest",
    ] {
        assert!(!issuer.contains(forbidden), "issuer retained {forbidden}");
        assert!(!model.contains(forbidden), "model retained {forbidden}");
    }
}

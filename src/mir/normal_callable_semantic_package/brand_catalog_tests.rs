use crate::mir::builder::NormalRootExecutionConsumerV1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
use crate::parser::{NyashParser, ParserBuildConfig};

fn issue_with_brand_catalog(
    source: &str,
) -> Result<
    super::VerifiedNormalCallableSemanticPackageV1,
    super::NormalCallableSemanticPackageIssueV1,
> {
    let parsed = NyashParser::parse_normal_callable_program_with_build_config(
        source,
        ParserBuildConfig::default(),
    )
    .expect("normal callable source");
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed)
            .expect("exact callable transform")
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("fixture must remain source-backed")
    };
    let catalog = crate::analysis::brand_program_declaration_catalog::
        issue_brand_program_declaration_catalog_v1(source.ast())
        .expect("brand catalog");
    let source = NormalRootExecutionConsumerV1::consume_once(source)
        .expect("root execution")
        .into_consumed_source();
    let mut resolver = FunctionSemanticResolverSessionV1::new(93).unwrap();
    super::issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&catalog),
    )
}

#[test]
fn instance_constructor_semantics_keep_parser_identity_and_nested_brand_relations() {
    let package = issue_with_brand_catalog(
        r#"
brand Id: i64

box Holder {
    init(value) {
        local direct = Id(value)
        local nested = fn(x) { Id(x) }
    }
    pack(other) {
        local second = Id(other)
    }
}
"#,
    )
    .expect("constructor semantic package");

    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].box_name(), "Holder");
    assert_eq!(rows[0].key(), "init/1");
    assert!(rows[0].source_id().same_as(rows[0].source_id()));
    assert_eq!(rows[0].forest().owner_count(), 2);
    assert_eq!(rows[1].key(), "pack/1");
    assert_eq!(
        rows.iter()
            .flat_map(|row| row.forest().owners())
            .map(|(_, owner)| owner.brand_call_relations().count())
            .sum::<usize>(),
        3
    );
}

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
    assert!(rows.iter().all(|row| row.published_birth_key().is_none()));
    assert_eq!(
        rows.iter()
            .flat_map(|row| row.forest().owners())
            .map(|(_, owner)| owner.brand_call_relations().count())
            .sum::<usize>(),
        3
    );
}

#[test]
fn birth_semantic_row_issues_distinct_non_global_publication_key() {
    let package = issue_with_brand_catalog("box Page { birth(value) { local saved = value } }")
        .expect("birth package");
    let rows = package.instance_constructors().rows();
    assert_eq!(rows.len(), 1);
    let key = rows[0]
        .published_birth_key()
        .expect("Birth owns a published key");
    assert_eq!(
        key.namespace(),
        hakorune_mir_defs::SameModuleCallableNamespaceV1::BirthConstructor
    );
    assert_eq!(key.owner(), "Page");
    assert_eq!(key.arity(), rows[0].source_arity());
    assert_eq!(key.mir_symbol_projection(), "Page.birth/1");
    assert!(key.canonical_global_target_v1().is_err());
}

#[test]
fn birth_fixed_source_retains_definition_through_normal_publication() {
    let _ring0 = crate::runtime::ring0::ensure_global_ring0_initialized();
    crate::test_support::with_env_var("NYASH_MACRO_DERIVE", "", || {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            include_str!("../../../apps/typed-object-birth-min/main.hako"),
            ParserBuildConfig::default(),
        )
        .expect("unchanged birth source");
        let transformed =
            crate::r#macro::transform_normal_callable_program_v1(parsed).expect("birth transform");
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("birth fixture must retain source authority")
        };
        let result = crate::mir::MirCompiler::with_options(false)
            .compile_normal(
                crate::mir::NormalCompileRequestV1::for_mir_mode_callable_source(
                    source,
                    Some("apps/typed-object-birth-min/main.hako"),
                    Default::default(),
                ),
            )
            .expect("normal constructor publication");
        let key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::birth_constructor("Pair", 2);
        assert_eq!(
            result.module.canonical_callable_definition_symbol(&key),
            Some("Pair.birth/2")
        );
        let function = result.module.get_function("Pair.birth/2").unwrap();
        assert_eq!(function.signature.params.len(), 3);
        let static_key = hakorune_mir_defs::CanonicalSameModuleCallableKeyV1::static_box_method(
            "Pair", "birth", 2,
        );
        assert!(result
            .module
            .canonical_callable_definition_symbol(&static_key)
            .is_none());
    });
}

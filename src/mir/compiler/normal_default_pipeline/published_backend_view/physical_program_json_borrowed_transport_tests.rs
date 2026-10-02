//! Original source actuals exercised through the transport codec. These are
//! component proofs, not full source-to-publication or executable acceptance.
use super::*;
use crate::mir::normal_callable_semantic_package::{
    issue_normal_callable_semantic_package_with_brand_catalog_v1,
    VerifiedNormalCallableSemanticPackageV1,
};

fn package(text: &str) -> VerifiedNormalCallableSemanticPackageV1 {
    let parsed = crate::parser::NyashParser::parse_normal_callable_program_with_build_config(
        text,
        crate::parser::ParserBuildConfig::default(),
    )
    .unwrap();
    let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
        crate::r#macro::transform_normal_callable_program_v1(parsed).unwrap()
    });
    let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed else {
        panic!("source-backed fixture");
    };
    let brands = crate::analysis::brand_program_declaration_catalog::issue_brand_program_declaration_catalog_v1(source.ast()).unwrap();
    let source = crate::mir::builder::NormalRootExecutionConsumerV1::consume_once(source)
        .unwrap()
        .into_consumed_source();
    let mut resolver =
        crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1::new(993).unwrap();
    issue_normal_callable_semantic_package_with_brand_catalog_v1(
        &mut resolver,
        source,
        Some(&brands),
    )
    .unwrap()
}

#[test]
fn original_borrowed_actuals_encode_integer_bool_scalar_and_typed_home() {
    for (argument, prefix, expected) in [
        ("0", "", 1),
        ("-7", "", 1),
        ("true", "", 2),
        ("false", "", 2),
        ("recv", "", 3),
        ("n", "local n = 8", 1),
        ("n", "local n = true", 2),
    ] {
        let text = format!("box Transport {{ birth() {{}} probe(p): i64 {{ return 0 }} }} static box Main {{ main() {{ local recv = new Transport() {prefix} return recv.probe({argument}) }} }}");
        let package = package(&text);
        let exit = package
            .batch()
            .declarations()
            .find_map(|declaration| {
                package
                    .batch()
                    .with_lowering_input(declaration.batch_slot(), |input| {
                        if !input
                            .function()
                            .method_calls()
                            .any(|(_, call)| call.selector() == "probe")
                        {
                            return None;
                        }
                        input
                            .function()
                            .resolved_exits()
                            .find_map(|(site, _)| match site {
                                crate::mir::resolved_semantics::ResolvedExitSiteV1::Statement(
                                    site,
                                ) => Some(site.clone()),
                                _ => None,
                            })
                    })
                    .unwrap()
            })
            .unwrap();
        let (prepared, row, arguments, ledger, _) =
            crate::mir::builder::lexical_call_projection_borrowed_fixture(package, &exit);
        prepared
            .materialize_with_ledger(row.call_site().owner(), &row, &arguments, &ledger)
            .unwrap();
        let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
        assert_eq!(actuals.len(), 1);
        assert_eq!(
            encode_borrowed_actual_kind(&actuals[0].source),
            json!(expected)
        );
        // Reborrowing the same source owner never manufactures or clones a proof.
        assert!(std::ptr::eq(
            actuals,
            ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap()
        ));
    }
}

#[test]
fn original_forwarded_actual_keeps_the_tagged_pair_spelling() {
    let package = package("box Transport { birth() {} probe(p): i64 { return 0 } forward(q): i64 { local alias = q local recv = new Transport() local out = recv.probe(alias) return 0 } } static box Main { main() { local recv = new Transport() local out = recv.forward(true) return 0 } }");
    let (prepared, row, arguments, ledger, _, _) =
        crate::mir::builder::lexical_call_projection_forwarded_fixture(
            package,
            crate::mir::BasicBlockId(0),
        );
    prepared
        .materialize_with_ledger(row.call_site().owner(), &row, &arguments, &ledger)
        .unwrap();
    let actuals = ledger.borrowed_call_actuals_v1(&row).unwrap().unwrap();
    assert_eq!(actuals.len(), 1);
    assert!(matches!(actuals[0].source, Source::Forwarded { .. }));
    assert_eq!(
        encode_borrowed_actual_kind(&actuals[0].source),
        json!("tagged")
    );
}

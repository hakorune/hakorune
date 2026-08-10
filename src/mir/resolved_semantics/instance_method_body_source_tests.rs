use super::*;

use crate::mir::resolved_semantics::{
    ResolverNominalBoxDeclarationInputV1, ResolverNominalTypeEnvironmentV1,
    SemanticInstanceDeclarationIssuerV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn body_parts() -> (
    crate::parser::ParserBoxResolverSourceHandoffV1,
    crate::parser::ParserBoxBodySourceEnvelopeV1,
) {
    NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } helper(): i64 { return 1 } empty(): i64 { } }",
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse")
    .into_parts()
    .expect("body transaction should issue")
}

fn declaration_catalog(
    handoff: crate::parser::ParserBoxResolverSourceHandoffV1,
) -> VerifiedInstanceMethodDeclarationCatalogV1 {
    let nominal =
        ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
            0, "TextLike",
        )])
        .expect("nominal environment should issue");
    SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal).expect("declaration should issue")
}

fn foreign_declaration_catalog() -> VerifiedInstanceMethodDeclarationCatalogV1 {
    let (handoff, _envelope) = NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } helper(): i64 { return 1 } empty(): i64 { } }",
        ParserBuildConfig::default(),
    )
    .expect("foreign body transaction should parse")
    .into_parts()
    .expect("foreign body transaction should issue");
    declaration_catalog(handoff)
}

#[test]
fn body_source_issues_complete_direct_cohort_and_order() {
    let (handoff, envelope) = body_parts();
    let declarations = declaration_catalog(handoff);
    let body = InstanceMethodBodySourceIssuerV1::issue(envelope, &declarations)
        .expect("body source should seal the complete direct cohort");

    assert_eq!(body.rows().len(), 3);
    let row = &body.rows()[0];
    assert_eq!(row.name(), "length");
    assert_eq!(row.box_statement_ordinal(), 0);
    assert_eq!(row.method_member_ordinal(), 0);
    assert_eq!(row.body_item_ordinals(), &[0]);
    assert_eq!(body.rows()[1].name(), "helper");
    assert_eq!(body.rows()[1].method_member_ordinal(), 1);
    assert_eq!(body.rows()[1].body_item_ordinals(), &[0]);
    assert_eq!(body.rows()[2].name(), "empty");
    assert_eq!(body.rows()[2].method_member_ordinal(), 2);
    assert_eq!(body.rows()[2].body_item_ordinals(), &[] as &[u32]);
    assert_eq!(row.resolver_brand(), declarations.resolver_brand());
    assert!(body
        .parser_provenance()
        .same_as(declarations.parser_provenance()));
}

#[test]
fn body_source_consumes_envelope_once() {
    let (handoff, envelope) = body_parts();
    let declarations = declaration_catalog(handoff);
    let _ = InstanceMethodBodySourceIssuerV1::issue(envelope, &declarations)
        .expect("first body source issue should succeed");
}

#[test]
fn body_source_rejects_foreign_parser_provenance() {
    let (_handoff, envelope) = body_parts();
    let foreign = foreign_declaration_catalog();
    let error = InstanceMethodBodySourceIssuerV1::issue(envelope, &foreign)
        .expect_err("body source must reject a different parser transaction");
    assert!(matches!(
        error,
        InstanceMethodBodySourceIssueV1::ParserProvenanceMismatch
    ));
}

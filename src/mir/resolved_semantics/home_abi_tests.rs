use super::*;
use crate::mir::resolved_semantics::{
    ResolverNominalBoxDeclarationInputV1, ResolverNominalTypeEnvironmentV1,
    SemanticInstanceDeclarationIssuerV1,
};
use crate::parser::{NyashParser, ParserBoxResolverSourceHandoffV1, ParserBuildConfig};

fn parse_handoff(source: &str) -> ParserBoxResolverSourceHandoffV1 {
    NyashParser::parse_from_string_with_resolver_source_handoff(
        source,
        ParserBuildConfig::default(),
    )
    .expect("bounded Box source should issue a handoff")
    .1
}

fn declaration_catalog(source: &str, name: &str) -> VerifiedInstanceMethodDeclarationCatalogV1 {
    let handoff = parse_handoff(source);
    let nominal =
        ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
            0, name,
        )])
        .expect("nominal environment should issue");
    SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal)
        .expect("semantic declaration should issue")
}

fn issue_home(source: &str, name: &str) -> VerifiedDeclaredInstanceMethodHomeCatalogV1 {
    let catalog = declaration_catalog(source, name);
    let environment = ResolverHomeCapabilityEnvironmentV1::issue(&catalog)
        .expect("Home capability environment should issue");
    CallableHomeAbiIssuerV1::issue(catalog, environment).expect("Home ABI catalog should issue")
}

#[test]
fn home_abi_maps_i64_parameter_and_result_without_query_behavior() {
    let home = issue_home(
        "box TextLike { read(value: i64): i64 { return value } }",
        "TextLike",
    );
    let declaration = &home.declarations()[0];
    let abi = &home.home_abis()[0];

    assert_eq!(abi.receiver(), HomeDemandV1::Handle);
    assert_eq!(abi.parameters(), &[HomeDemandV1::Trivial]);
    assert_eq!(abi.result(), HomeResultRelationV1::Trivial);
    assert_eq!(abi.resolver_brand(), declaration.resolver_brand());
    assert_eq!(abi.nominal_box_type(), declaration.nominal_box_type());
    assert_eq!(
        abi.box_statement_ordinal(),
        declaration.box_statement_ordinal()
    );
    assert_eq!(
        abi.method_member_ordinal(),
        declaration.method_member_ordinal()
    );
}

#[test]
fn home_abi_maps_unit_result_and_ignores_query_carriage() {
    let with_query = issue_home(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
        "TextLike",
    );
    let without_query = issue_home("box TextLike { reset() { return } }", "TextLike");

    assert_eq!(with_query.home_abis()[0].receiver(), HomeDemandV1::Handle);
    assert_eq!(with_query.home_abis()[0].parameters(), &[]);
    assert_eq!(
        with_query.home_abis()[0].result(),
        HomeResultRelationV1::Trivial
    );
    assert_eq!(
        without_query.home_abis()[0].receiver(),
        HomeDemandV1::Handle
    );
    assert_eq!(without_query.home_abis()[0].parameters(), &[]);
    assert_eq!(
        without_query.home_abis()[0].result(),
        HomeResultRelationV1::Unit
    );
}

#[test]
fn home_abi_rejects_a_foreign_resolver_catalog_environment() {
    let first = declaration_catalog("box TextLike { length(): i64 { return 0 } }", "TextLike");
    let second = declaration_catalog("box TextLike { length(): i64 { return 0 } }", "TextLike");
    let environment =
        ResolverHomeCapabilityEnvironmentV1::issue(&first).expect("first environment should issue");
    let error = CallableHomeAbiIssuerV1::issue(second, environment)
        .expect_err("foreign resolver catalog must reject");

    assert!(matches!(error, HomeAbiIssueV1::ResolverBrandMismatch));
}

#[test]
fn home_abi_uses_a_fresh_relation_batch_for_each_issuance() {
    let first = issue_home("box TextLike { length(): i64 { return 0 } }", "TextLike");
    let second = issue_home("box TextLike { length(): i64 { return 0 } }", "TextLike");

    assert_ne!(first.relation_batch_brand(), second.relation_batch_brand());
    assert_eq!(
        first.home_abis()[0].relation_batch_brand(),
        first.relation_batch_brand()
    );
    assert_eq!(
        second.home_abis()[0].relation_batch_brand(),
        second.relation_batch_brand()
    );
}

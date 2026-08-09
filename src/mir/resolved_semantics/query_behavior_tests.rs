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
    let nominal =
        ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
            0, name,
        )])
        .expect("nominal environment should issue");
    SemanticInstanceDeclarationIssuerV1::issue(parse_handoff(source), nominal)
        .expect("semantic declaration should issue")
}

#[test]
fn query_issuer_seals_typed_behavior_and_exact_identity() {
    let declaration = declaration_catalog(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
        "TextLike",
    );
    let resolver_brand = declaration.resolver_brand();
    let query =
        DeclaredQueryBehaviorIssuerV1::issue(&declaration).expect("typed Query should issue");
    let row = query.rows().first().expect("one Query row");

    assert_eq!(row.resolver_brand(), resolver_brand);
    assert_eq!(row.nominal_box_type().brand(), resolver_brand);
    assert_eq!(row.box_statement_ordinal(), 0);
    assert_eq!(row.method_member_ordinal(), 0);
    assert_eq!(
        row.behavior(),
        DeclaredQueryBehaviorV1::ReceiverDirectReadNoEffects
    );
    assert_eq!(row.rune_ordinal(), 0);
}

#[test]
fn missing_query_is_declined_without_an_empty_verified_catalog() {
    let declaration =
        declaration_catalog("box TextLike { length(): i64 { return 0 } }", "TextLike");
    let error = DeclaredQueryBehaviorIssuerV1::issue(&declaration)
        .expect_err("missing Query must not issue an empty catalog");

    assert_eq!(error, QueryBehaviorIssueV1::NoQueryDeclaration);
}

#[test]
fn mixed_catalog_emits_only_the_exact_query_subset_in_declaration_order() {
    let declaration = declaration_catalog(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } reset() { return } @rune CallableContract(query) size(): i64 { return 1 } }",
        "TextLike",
    );
    let query = DeclaredQueryBehaviorIssuerV1::issue(&declaration)
        .expect("non-empty Query subset should issue");

    assert_eq!(query.rows().len(), 2);
    assert_eq!(query.rows()[0].method_member_ordinal(), 0);
    assert_eq!(query.rows()[1].method_member_ordinal(), 2);
    assert_eq!(query.resolver_brand(), declaration.resolver_brand());
}

#[test]
fn query_carriage_does_not_change_home_abi_output() {
    fn issue_home(source: &str) -> VerifiedDeclaredInstanceMethodHomeCatalogV1 {
        let catalog = declaration_catalog(source, "TextLike");
        let environment = ResolverHomeCapabilityEnvironmentV1::issue(&catalog)
            .expect("Home environment should issue");
        CallableHomeAbiIssuerV1::issue(catalog, environment).expect("Home ABI should issue")
    }

    let with_home =
        issue_home("box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }");
    let without_home = issue_home("box TextLike { length(): i64 { return 0 } }");

    assert_eq!(
        with_home.home_abis()[0].receiver(),
        without_home.home_abis()[0].receiver()
    );
    assert_eq!(
        with_home.home_abis()[0].parameters(),
        without_home.home_abis()[0].parameters()
    );
    assert_eq!(
        with_home.home_abis()[0].result(),
        without_home.home_abis()[0].result()
    );
}

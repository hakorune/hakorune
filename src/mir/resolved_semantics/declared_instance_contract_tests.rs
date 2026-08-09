use super::*;
use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, DeclaredQueryBehaviorIssuerV1, ResolverHomeCapabilityEnvironmentV1,
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

fn home_and_query(
    source: &str,
) -> (
    VerifiedDeclaredInstanceMethodHomeCatalogV1,
    VerifiedDeclaredQueryBehaviorCatalogV1,
) {
    let declaration = declaration_catalog(source, "TextLike");
    let query = DeclaredQueryBehaviorIssuerV1::issue(&declaration)
        .expect("source should have a Query subset");
    let environment = ResolverHomeCapabilityEnvironmentV1::issue(&declaration)
        .expect("Home environment should issue");
    let home = CallableHomeAbiIssuerV1::issue(declaration, environment)
        .expect("Home catalog should issue");
    (home, query)
}

#[test]
fn aggregate_co_seals_query_subset_with_home_owner() {
    let (home, query) =
        home_and_query("box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }");
    let resolver_brand = home.resolver_brand();
    let aggregate = DeclaredInstanceMethodContractIssuerV1::issue(home, query)
        .expect("same declaration Query/Home products should co-seal");

    assert_eq!(aggregate.resolver_brand(), resolver_brand);
    assert_eq!(aggregate.declarations().len(), 1);
    assert_eq!(aggregate.home_abis().len(), 1);
    assert_eq!(aggregate.query_behaviors().len(), 1);
    assert_eq!(aggregate.selected_pair_count(), 1);
}

#[test]
fn aggregate_accepts_a_strict_query_subset_in_declaration_order() {
    let (home, query) = home_and_query(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } reset() { return } @rune CallableContract(query) size(): i64 { return 1 } }",
    );
    let aggregate = DeclaredInstanceMethodContractIssuerV1::issue(home, query)
        .expect("Query subset should co-seal with all Home rows");

    assert_eq!(aggregate.declarations().len(), 3);
    assert_eq!(aggregate.query_behaviors().len(), 2);
    assert_eq!(aggregate.selected_pair_count(), 2);
    assert_eq!(aggregate.query_behaviors()[0].method_member_ordinal(), 0);
    assert_eq!(aggregate.query_behaviors()[1].method_member_ordinal(), 2);
}

#[test]
fn aggregate_rejects_a_foreign_query_catalog() {
    let (home, _) =
        home_and_query("box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }");
    let (_, foreign_query) =
        home_and_query("box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }");
    let error = DeclaredInstanceMethodContractIssuerV1::issue(home, foreign_query)
        .expect_err("foreign resolver brand must reject");

    assert_eq!(
        error,
        DeclaredInstanceMethodContractIssueV1::ResolverBrandMismatch
    );
}

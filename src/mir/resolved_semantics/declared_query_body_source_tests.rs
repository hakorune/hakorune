use super::*;

use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, DeclaredQueryBehaviorIssuerV1,
    DeclaredInstanceMethodContractIssuerV1,
    InstanceMethodBodySourceIssuerV1, ResolverHomeCapabilityEnvironmentV1,
    ResolverNominalBoxDeclarationInputV1, ResolverNominalTypeEnvironmentV1,
    SemanticInstanceDeclarationIssuerV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn issue_products(
    source: &str,
) -> (
    VerifiedInstanceMethodBodySourceCatalogV1,
    VerifiedDeclaredInstanceMethodContractCatalogV1,
) {
    let (handoff, envelope) = NyashParser::parse_from_string_with_resolver_body_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("body transaction should issue")
    .into_parts()
    .expect("body transaction parts should issue");
    let nominal = ResolverNominalTypeEnvironmentV1::issue([
        ResolverNominalBoxDeclarationInputV1::new(0, "TextLike"),
    ])
    .expect("nominal environment should issue");
    let declarations = SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal)
        .expect("declaration catalog should issue");
    let body = InstanceMethodBodySourceIssuerV1::issue(envelope, &declarations)
        .expect("general body catalog should issue");
    let query = DeclaredQueryBehaviorIssuerV1::issue(&declarations)
        .expect("Query behavior should issue");
    let environment = ResolverHomeCapabilityEnvironmentV1::issue(&declarations)
        .expect("Home environment should issue");
    let home = CallableHomeAbiIssuerV1::issue(declarations, environment)
        .expect("Home catalog should issue");
    let contract = DeclaredInstanceMethodContractIssuerV1::issue(home, query)
        .expect("declared contract should issue");
    (body, contract)
}

fn sparse_source() -> &'static str {
    "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } helper(): i64 { return 1 } @rune CallableContract(query) empty(): i64 { } }"
}

#[test]
fn projects_sparse_query_rows_without_consuming_general_catalog() {
    let (body, contract) = issue_products(sparse_source());
    let projected = DeclaredQueryBodySourceIssuerV1::issue(&body, &contract)
        .expect("selected Query body view should issue");

    assert_eq!(projected.rows().len(), 2);
    assert_eq!(projected.rows()[0].body().method_member_ordinal(), 0);
    assert_eq!(projected.rows()[1].body().method_member_ordinal(), 2);
    assert_eq!(projected.rows()[0].contract().declaration().name(), "length");
    assert_eq!(projected.rows()[1].contract().declaration().name(), "empty");
    assert_eq!(projected.rows()[1].body().body_item_ordinals(), &[] as &[u32]);

    // The projection borrows the all-row authority; the non-Query row remains
    // available for a later non-Query observer.
    assert_eq!(body.rows().len(), 3);
    assert_eq!(body.rows()[1].name(), "helper");
    assert_eq!(projected.resolver_brand(), contract.resolver_brand());
    assert!(projected
        .parser_provenance()
        .same_as(contract.parser_provenance()));
}

#[test]
fn selected_contract_view_preserves_aggregate_order_and_relations() {
    let (_body, contract) = issue_products(sparse_source());
    let selected = contract.selected_contracts().collect::<Vec<_>>();

    assert_eq!(selected.len(), 2);
    assert_eq!(selected[0].declaration().method_member_ordinal(), 0);
    assert_eq!(selected[1].declaration().method_member_ordinal(), 2);
    assert_eq!(selected[0].home_abi().method_member_ordinal(), 0);
    assert_eq!(selected[1].query().method_member_ordinal(), 2);
}

#[test]
fn rejects_a_cross_catalog_identity_mismatch_before_projection() {
    let (body, _contract) = issue_products(sparse_source());
    let (_foreign_body, foreign_contract) = issue_products(sparse_source());
    let error = DeclaredQueryBodySourceIssuerV1::issue(&body, &foreign_contract)
        .expect_err("foreign parser/catalog products must not be paired");

    assert_eq!(error, DeclaredQueryBodySourceIssueV1::ResolverBrandMismatch);
}

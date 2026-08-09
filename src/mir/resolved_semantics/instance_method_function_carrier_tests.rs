use super::*;

use crate::parser::{NyashParser, ParserBuildConfig};

fn nominal_environment() -> ResolverNominalTypeEnvironmentV1 {
    ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
        0, "TextLike",
    )])
    .expect("nominal environment should issue")
}

fn issue_carrier(
    source: &str,
) -> Result<VerifiedInstanceMethodFunctionCarrierCatalogV1, InstanceMethodFunctionCarrierIssueV1> {
    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse");
    transaction
        .with_direct_method_syntax(|handoff, _envelope, lease| {
            let declarations =
                SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal_environment())
                    .expect("declaration catalog should issue");
            let mut resolver =
                FunctionSemanticResolverSessionV1::new(0).expect("resolver session should issue");
            InstanceMethodFunctionCarrierIssuerV1::issue(lease, &declarations, &mut resolver)
        })
        .expect("syntax lease callback should complete")
}

#[test]
fn carrier_issues_one_direct_instance_method_with_exact_root_receipts() {
    let carrier =
        issue_carrier("box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }")
            .expect("carrier should issue");

    assert_eq!(carrier.rows().len(), 1);
    let row = &carrier.rows()[0];
    assert_eq!(row.name(), "length");
    assert_eq!(row.source_site().box_statement_ordinal(), 0);
    assert_eq!(row.source_site().member_ordinal(), 0);
    assert_eq!(
        row.root_function().source_kind(),
        SemanticOwnerSourceKindV1::DeclaredFunction
    );
    assert_eq!(row.root_function().owner(), row.forest().roots()[0]);
    assert_eq!(row.body_root(), &SourcePathSegmentV1::FunctionBody);
    assert_eq!(row.body_coverage().item_ordinals(), &[0]);
    assert_eq!(row.body_shape().owner(), row.root_function().owner());
    assert!(row
        .body_shape()
        .statements()
        .iter()
        .any(|statement| matches!(statement, BodyStatementShapeV1::Return { .. })));
}

#[test]
fn carrier_preserves_empty_body_coverage_without_issuing_body_facts() {
    let carrier = issue_carrier("box TextLike { empty(): i64 { } }")
        .expect("empty body source should still resolve as a carrier");

    assert_eq!(carrier.rows().len(), 1);
    assert_eq!(
        carrier.rows()[0].body_coverage().item_ordinals(),
        &[] as &[u32]
    );
}

#[test]
fn carrier_rejects_a_foreign_parser_transaction() {
    let foreign_transaction = NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { length(): i64 { return 0 } }",
        ParserBuildConfig::default(),
    )
    .expect("foreign transaction should parse");
    let (foreign_handoff, _) = foreign_transaction
        .into_parts()
        .expect("foreign transaction should decompose");
    let foreign_declarations =
        SemanticInstanceDeclarationIssuerV1::issue(foreign_handoff, nominal_environment())
            .expect("foreign declaration catalog should issue");

    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { length(): i64 { return 0 } }",
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse");
    let error = transaction
        .with_direct_method_syntax(|_handoff, _envelope, lease| {
            let mut resolver =
                FunctionSemanticResolverSessionV1::new(0).expect("resolver session should issue");
            InstanceMethodFunctionCarrierIssuerV1::issue(
                lease,
                &foreign_declarations,
                &mut resolver,
            )
        })
        .expect("callback should complete")
        .expect_err("carrier must reject a foreign parser provenance");

    assert!(matches!(
        error,
        InstanceMethodFunctionCarrierIssueV1::ParserProvenanceMismatch
    ));
}

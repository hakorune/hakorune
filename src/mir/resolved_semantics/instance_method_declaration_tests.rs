use super::*;
use crate::parser::{NyashParser, ParserBoxResolverSourceHandoffV1, ParserBuildConfig};

fn parse_handoff(source: &str) -> ParserBoxResolverSourceHandoffV1 {
    NyashParser::parse_from_string_with_resolver_source_handoff(
        source,
        ParserBuildConfig::default(),
    )
    .expect("bounded Box source should issue a handoff")
    .1
}

fn environment(name: &str) -> ResolverNominalTypeEnvironmentV1 {
    ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
        0, name,
    )])
    .expect("nominal environment should issue")
}

#[test]
fn declaration_issuer_seals_exact_instance_i64_and_query_carriage() {
    let handoff = parse_handoff(
        r#"
box TextLike {
    @rune CallableContract(query)
    length(): i64 { return 0 }
}
"#,
    );
    let catalog = SemanticInstanceDeclarationIssuerV1::issue(handoff, environment("TextLike"))
        .expect("semantic declaration should issue");

    assert_eq!(catalog.declarations().len(), 1);
    let declaration = &catalog.declarations()[0];
    assert_eq!(declaration.name(), "length");
    assert_eq!(declaration.box_statement_ordinal(), 0);
    assert_eq!(declaration.method_member_ordinal(), 0);
    assert_eq!(declaration.signature().parameters(), &[]);
    assert_eq!(
        declaration.signature().result(),
        ResolverSemanticValueTypeV1::I64
    );
    assert!(declaration.callable_contract().is_some());
    assert_eq!(
        declaration.nominal_box_type().brand(),
        catalog.resolver_brand()
    );
    assert_eq!(
        declaration.resolver_brand(),
        catalog.resolver_brand()
    );
    let _ = catalog.parser_provenance();
}

#[test]
fn declaration_issuer_does_not_require_query_behavior() {
    let handoff = parse_handoff("box TextLike { length(): i64 { return 0 } }");
    let catalog = SemanticInstanceDeclarationIssuerV1::issue(handoff, environment("TextLike"))
        .expect("declaration/signature does not require Query");

    assert!(catalog.declarations()[0].callable_contract().is_none());
    assert_eq!(
        catalog.declarations()[0].signature().result(),
        ResolverSemanticValueTypeV1::I64
    );
}

#[test]
fn declaration_issuer_uses_a_fresh_resolver_brand_per_catalog() {
    let first = SemanticInstanceDeclarationIssuerV1::issue(
        parse_handoff("box TextLike { length(): i64 { return 0 } }"),
        environment("TextLike"),
    )
    .expect("first declaration catalog should issue");
    let second = SemanticInstanceDeclarationIssuerV1::issue(
        parse_handoff("box TextLike { length(): i64 { return 0 } }"),
        environment("TextLike"),
    )
    .expect("second declaration catalog should issue");

    assert_ne!(first.resolver_brand(), second.resolver_brand());
    assert_ne!(
        first.declarations()[0].nominal_box_type(),
        second.declarations()[0].nominal_box_type()
    );
}

#[test]
fn declaration_issuer_rejects_unknown_nominal_box() {
    let handoff = parse_handoff("box TextLike { length(): i64 { return 0 } }");
    let error = SemanticInstanceDeclarationIssuerV1::issue(
        handoff,
        ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
            1, "Other",
        )])
        .unwrap(),
    )
    .expect_err("nominal type environment must be exact");

    assert!(matches!(
        error,
        InstanceMethodDeclarationIssueV1::NominalBoxUnavailable { .. }
            | InstanceMethodDeclarationIssueV1::NominalBoxSourceMismatch { .. }
    ));
}

#[test]
fn declaration_issuer_rejects_unsupported_semantic_type() {
    let handoff = parse_handoff("box TextLike { length(): Text { return 0 } }");
    let error = SemanticInstanceDeclarationIssuerV1::issue(handoff, environment("TextLike"))
        .expect_err("physical/source ABI classifiers must not accept Text here");

    assert!(matches!(
        error,
        InstanceMethodDeclarationIssueV1::UnsupportedType { .. }
    ));
}

#[test]
fn nominal_environment_rejects_duplicate_source_identity() {
    let error = ResolverNominalTypeEnvironmentV1::issue([
        ResolverNominalBoxDeclarationInputV1::new(0, "TextLike"),
        ResolverNominalBoxDeclarationInputV1::new(0, "Other"),
    ])
    .expect_err("source statement identity must be unique");

    assert!(matches!(
        error,
        ResolverNominalTypeEnvironmentIssueV1::DuplicateSourceStatement { .. }
    ));
}

#[test]
fn nominal_environment_rejects_duplicate_nominal_name() {
    let error = ResolverNominalTypeEnvironmentV1::issue([
        ResolverNominalBoxDeclarationInputV1::new(0, "TextLike"),
        ResolverNominalBoxDeclarationInputV1::new(1, "TextLike"),
    ])
    .expect_err("nominal type name must not alias two source declarations");

    assert!(matches!(
        error,
        ResolverNominalTypeEnvironmentIssueV1::DuplicateSourceName { .. }
    ));
}

#[test]
fn declaration_issuer_rejects_unused_nominal_declaration() {
    let handoff = parse_handoff("box TextLike { length(): i64 { return 0 } }");
    let environment = ResolverNominalTypeEnvironmentV1::issue([
        ResolverNominalBoxDeclarationInputV1::new(0, "TextLike"),
        ResolverNominalBoxDeclarationInputV1::new(1, "Other"),
    ])
    .unwrap();
    let error = SemanticInstanceDeclarationIssuerV1::issue(handoff, environment)
        .expect_err("catalog coverage must be exact");

    assert!(matches!(
        error,
        InstanceMethodDeclarationIssueV1::UnusedNominalBox {
            statement_ordinal: 1
        }
    ));
}

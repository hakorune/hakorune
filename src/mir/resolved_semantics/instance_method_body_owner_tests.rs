use super::*;

use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, DeclaredInstanceMethodContractIssuerV1, DeclaredQueryBehaviorIssuerV1,
    InstanceMethodBodySourceIssuerV1, InstanceMethodFunctionCarrierIssuerV1,
    ResolverHomeCapabilityEnvironmentV1, ResolverNominalBoxDeclarationInputV1,
    ResolverNominalTypeEnvironmentV1, SemanticInstanceDeclarationIssuerV1,
};
use crate::parser::{NyashParser, ParserBuildConfig};

fn nominal_environment() -> ResolverNominalTypeEnvironmentV1 {
    ResolverNominalTypeEnvironmentV1::issue([ResolverNominalBoxDeclarationInputV1::new(
        0, "TextLike",
    )])
    .expect("nominal environment should issue")
}

fn with_owner<R>(
    source: &str,
    check: impl FnOnce(VerifiedInstanceMethodBodyOwnerCatalogV1<'_, '_, '_>) -> R,
) -> R {
    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse");
    transaction
        .with_direct_method_syntax(|handoff, envelope, lease| {
            let declarations =
                SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal_environment())
                    .expect("declaration catalog should issue");
            let mut resolver =
                FunctionSemanticResolverSessionV1::new(0).expect("resolver session should issue");
            let carrier =
                InstanceMethodFunctionCarrierIssuerV1::issue(lease, &declarations, &mut resolver)
                    .expect("carrier should issue");
            let body = InstanceMethodBodySourceIssuerV1::issue(envelope, &declarations)
                .expect("body source should issue");
            let query = DeclaredQueryBehaviorIssuerV1::issue(&declarations)
                .expect("Query behavior should issue");
            let environment = ResolverHomeCapabilityEnvironmentV1::issue(&declarations)
                .expect("Home environment should issue");
            let home = CallableHomeAbiIssuerV1::issue(declarations, environment)
                .expect("Home catalog should issue");
            let contract = DeclaredInstanceMethodContractIssuerV1::issue(home, query)
                .expect("declared contract should issue");
            let selected = DeclaredQueryBodySourceIssuerV1::issue(&body, &contract)
                .expect("selected Query body source should issue");
            let owner = InstanceMethodBodyOwnerBindingIssuerV1::issue(&selected, &carrier)
                .expect("owner link should issue");
            check(owner)
        })
        .expect("syntax callback should complete")
}

#[test]
fn owner_link_binds_selected_query_rows_to_carrier_roots() {
    with_owner(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } helper(): i64 { return 1 } @rune CallableContract(query) empty(): i64 { } }",
        |owner| {
            assert_eq!(owner.rows().len(), 2);
            assert_eq!(owner.rows()[0].body().method_member_ordinal(), 0);
            assert_eq!(owner.rows()[1].body().method_member_ordinal(), 2);
            assert_eq!(owner.rows()[1].body().body_item_ordinals(), &[] as &[u32]);
            assert_eq!(owner.rows()[0].body().name(), owner.rows()[0].carrier().name());
            assert_eq!(owner.rows()[0].root_function().source_kind(), SemanticOwnerSourceKindV1::DeclaredFunction);
        },
    );
}

#[test]
fn owner_link_preserves_sparse_non_query_extras_as_unselected() {
    with_owner(
        "box TextLike { helper(): i64 { return 1 } @rune CallableContract(query) length(): i64 { return 0 } }",
        |owner| {
            assert_eq!(owner.rows().len(), 1);
            assert_eq!(owner.rows()[0].body().method_member_ordinal(), 1);
            assert_eq!(owner.rows()[0].contract().declaration().name(), "length");
        },
    );
}

#[test]
fn owner_link_rejects_foreign_parser_provenance() {
    let foreign_carrier = {
        let transaction = NyashParser::parse_from_string_with_resolver_body_source(
            "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
            ParserBuildConfig::default(),
        )
        .expect("foreign transaction should parse");
        transaction
            .with_direct_method_syntax(|handoff, _envelope, lease| {
                let declarations =
                    SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal_environment())
                        .expect("foreign declarations should issue");
                let mut resolver = FunctionSemanticResolverSessionV1::new(0)
                    .expect("foreign resolver should issue");
                InstanceMethodFunctionCarrierIssuerV1::issue(lease, &declarations, &mut resolver)
                    .expect("foreign carrier should issue")
            })
            .expect("foreign callback should complete")
    };

    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse");
    let error = transaction
        .with_direct_method_syntax(|handoff, envelope, lease| {
            let declarations =
                SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal_environment())
                    .expect("declarations should issue");
            let mut resolver =
                FunctionSemanticResolverSessionV1::new(0).expect("resolver should issue");
            let _current_carrier =
                InstanceMethodFunctionCarrierIssuerV1::issue(lease, &declarations, &mut resolver)
                    .expect("current carrier should issue");
            let body = InstanceMethodBodySourceIssuerV1::issue(envelope, &declarations)
                .expect("body source should issue");
            let query = DeclaredQueryBehaviorIssuerV1::issue(&declarations)
                .expect("Query behavior should issue");
            let environment = ResolverHomeCapabilityEnvironmentV1::issue(&declarations)
                .expect("Home environment should issue");
            let home = CallableHomeAbiIssuerV1::issue(declarations, environment)
                .expect("Home catalog should issue");
            let contract = DeclaredInstanceMethodContractIssuerV1::issue(home, query)
                .expect("contract should issue");
            let selected = DeclaredQueryBodySourceIssuerV1::issue(&body, &contract)
                .expect("selected source should issue");
            InstanceMethodBodyOwnerBindingIssuerV1::issue(&selected, &foreign_carrier)
                .expect_err("foreign carrier must be rejected")
        })
        .expect("callback should complete");

    assert!(matches!(
        error,
        InstanceMethodBodyOwnerBindingIssueV1::ParserProvenanceMismatch
            | InstanceMethodBodyOwnerBindingIssueV1::ResolverBrandMismatch
    ));
}

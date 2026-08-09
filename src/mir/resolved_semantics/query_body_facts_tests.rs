use super::*;

use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, DeclaredInstanceMethodContractIssuerV1, DeclaredQueryBehaviorIssuerV1,
    DeclaredQueryBodySourceIssuerV1, FunctionSemanticResolverSessionV1,
    InstanceMethodBodyOwnerBindingIssuerV1, InstanceMethodBodySourceIssuerV1,
    InstanceMethodFunctionCarrierIssuerV1, QueryBodyConformanceEvidenceIssuerV1,
    QueryBodyConformanceIssuerV1, QueryBodyHomeTransferV1, ResolverHomeCapabilityEnvironmentV1,
    ResolverNominalBoxDeclarationInputV1, ResolverNominalTypeEnvironmentV1,
    SemanticInstanceDeclarationIssuerV1,
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
    with_contract_and_owner(source, |_, owner| check(owner))
}

fn with_contract_and_owner<R>(
    source: &str,
    check: impl FnOnce(
        &crate::mir::resolved_semantics::VerifiedDeclaredInstanceMethodContractCatalogV1,
        VerifiedInstanceMethodBodyOwnerCatalogV1<'_, '_, '_>,
    ) -> R,
) -> R {
    let transaction = NyashParser::parse_from_string_with_resolver_body_source(
        source,
        ParserBuildConfig::default(),
    )
    .expect("body transaction should parse");
    transaction
        .with_direct_method_syntax(|handoff, envelope, lease, _release_sources| {
            let declarations =
                SemanticInstanceDeclarationIssuerV1::issue(handoff, nominal_environment())
                    .expect("declaration catalog should issue");
            let mut resolver =
                FunctionSemanticResolverSessionV1::new(0).expect("resolver should issue");
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
            check(&contract, owner)
        })
        .expect("syntax callback should complete")
}

#[test]
fn query_body_facts_accept_exact_return_me() {
    with_contract_and_owner(
        "box TextLike { @rune CallableContract(query) length(): i64 { return me } }",
        |contract, owner| {
            let facts = QueryBodyFactsIssuerV1::issue(&owner)
                .expect("exact lexical-Me body should issue facts");
            assert_eq!(facts.rows().len(), 1);
            let row = &facts.rows()[0];
            let receiver = row.receiver_read().receiver();
            assert_eq!(receiver.owner(), row.owner().root_function().owner());
            assert_eq!(
                row.owner()
                    .root_function()
                    .binding(receiver)
                    .unwrap()
                    .kind(),
                crate::mir::resolved_semantics::records::BindingKindV1::Receiver
            );
            assert_eq!(
                row.receiver_read().expression(),
                row.ordinary_return().value()
            );
            let evidence = QueryBodyConformanceEvidenceIssuerV1::issue(&owner, &facts)
                .expect("bounded conformance evidence should issue");
            assert_eq!(evidence.rows().len(), 1);
            assert_eq!(
                evidence.rows()[0].home_flow().transfer(),
                QueryBodyHomeTransferV1::None
            );
            let conformance = QueryBodyConformanceIssuerV1::issue(contract, &evidence)
                .expect("bounded Query conformance should issue");
            assert_eq!(conformance.rows().len(), 1);
            assert_eq!(
                conformance.rows()[0].contract().query().behavior(),
                crate::mir::resolved_semantics::DeclaredQueryBehaviorV1::ReceiverDirectReadNoEffects
            );
        },
    );
}

#[test]
fn query_body_facts_decline_constant_and_empty_shapes() {
    with_owner(
        "box TextLike { @rune CallableContract(query) length(): i64 { return 0 } }",
        |owner| {
            assert!(matches!(
                QueryBodyFactsIssuerV1::issue(&owner),
                Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::NotReceiverMe
                ))
            ));
        },
    );
    with_owner(
        "box TextLike { @rune CallableContract(query) empty(): i64 { } }",
        |owner| {
            assert!(matches!(
                QueryBodyFactsIssuerV1::issue(&owner),
                Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::EmptyBody
                ))
            ));
        },
    );
}

#[test]
fn query_body_facts_decline_field_and_extra_statement_shapes() {
    with_owner(
        "box TextLike { @rune CallableContract(query) length(): i64 { return me.value } }",
        |owner| {
            assert!(matches!(
                QueryBodyFactsIssuerV1::issue(&owner),
                Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::NotReceiverMe
                ))
            ));
        },
    );
    with_owner(
        "box TextLike { @rune CallableContract(query) length(): i64 { local x = 1 return me } }",
        |owner| {
            assert!(matches!(
                QueryBodyFactsIssuerV1::issue(&owner),
                Err(QueryBodyFactsIssueV1::Declined(
                    QueryBodyFactsDeclineV1::ReturnCount
                ))
            ));
        },
    );
}

#[test]
fn query_body_facts_preserve_sparse_selected_query_order() {
    with_contract_and_owner(
        "box TextLike { @rune CallableContract(query) first(): i64 { return me } helper(): i64 { return 0 } @rune CallableContract(query) second(): i64 { return me } }",
        |contract, owner| {
            let facts = QueryBodyFactsIssuerV1::issue(&owner)
                .expect("both selected lexical-Me rows should issue");
            assert_eq!(facts.rows().len(), 2);
            assert_eq!(facts.rows()[0].owner().body().method_member_ordinal(), 0);
            assert_eq!(facts.rows()[1].owner().body().method_member_ordinal(), 2);
            let evidence = QueryBodyConformanceEvidenceIssuerV1::issue(&owner, &facts)
                .expect("sparse bounded evidence should issue");
            let conformance = QueryBodyConformanceIssuerV1::issue(contract, &evidence)
                .expect("sparse bounded conformance should issue");
            assert_eq!(conformance.rows().len(), 2);
            assert_eq!(
                conformance.rows()[0]
                    .contract()
                    .declaration()
                    .method_member_ordinal(),
                0
            );
            assert_eq!(
                conformance.rows()[1]
                    .contract()
                    .declaration()
                    .method_member_ordinal(),
                2
            );
        },
    );
}

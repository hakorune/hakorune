use crate::ast::ASTNode;
use crate::mir::builder::{
    issue_catalog_callable_owner_link_v1, NormalCallableSemanticAdmissionV1,
    SameModuleCallableNamespaceV1, VerifiedNormalCallableSemanticSourceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::dynamic_invocation_contract::{
    DynamicInvocationEnvelopeLookupV1, VerifiedDynamicInvocationEnvelopeCatalogV1,
};
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionSemanticResolverSessionV1,
};
use crate::mir::source_call_target::{
    VerifiedSourceCallTargetCatalogV1, VerifiedStaticImportAliasViewV1,
};
use crate::parser::NyashParser;

use super::super::super::dynamic_full_body_source::DynamicFullBodySourceIssuerV1;
use super::super::claims::DynamicFullLoopRecipeClaimsV2;
use super::super::{produce_dynamic_full_loop_recipe_v2, DynamicFullLoopRecipeCandidateV2};
use super::coverage::{verify_complete_claim_coverage_v2, DynamicFullLoopCoverageRejectV2};
use super::{
    issue_dynamic_full_loop_source_recipe_envelope_v2, DynamicFullLoopCallRelationRejectV2,
    DynamicFullLoopSourceRecipeEnvelopeRejectV2,
};

fn production_program() -> ASTNode {
    NyashParser::parse_from_string(include_str!(
        "../../../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako"
    ))
    .expect("production parser module")
}

struct CosealFixtureV2 {
    candidate: DynamicFullLoopRecipeCandidateV2,
    catalog: VerifiedDynamicInvocationEnvelopeCatalogV1<'static>,
}

fn fixture(include_dynamic_targets: bool) -> CosealFixtureV2 {
    let program = Box::leak(Box::new(production_program()));
    let declarations = Box::leak(Box::new(
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(program)
            .expect("declaration catalog"),
    ));
    let mut resolver = FunctionSemanticResolverSessionV1::new(0).expect("resolver");
    let NormalCallableSemanticAdmissionV1::Complete(source) =
        VerifiedNormalCallableSemanticSourceV1::seal(
            program,
            declarations.selected_source_inventory(),
            false,
            &mut resolver,
        )
        .expect("semantic source")
    else {
        panic!("semantic source must be complete")
    };

    let callable = declarations
        .declaration_for(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
            "ParserScanLoopBox",
            "skip_while",
            4,
        )
        .expect("skip_while declaration")
        .key()
        .clone();
    let loan = source
        .cataloged_loan(&callable)
        .expect("exact semantic source loan");
    let (_, ingress) = issue_catalog_callable_owner_link_v1(loan, declarations)
        .expect("catalog callable owner link")
        .into_parts();
    let input = ingress.input();
    let membership = ingress.ledger().only_loop_site().expect("one loop");
    let completion = verify_function_completion_v1(input).expect("completion");
    let source_inventory = DynamicFullBodySourceIssuerV1::issue(input, membership, completion)
        .expect("full source inventory");
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory).expect("complete candidate");

    let imports = VerifiedStaticImportAliasViewV1::seal(declarations, std::iter::empty())
        .expect("empty imports");
    let targets = VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, std::iter::empty())
        .expect("empty targets");
    let targets = if include_dynamic_targets {
        targets
            .extend_complete_dynamic_sources(&source)
            .expect("complete Dynamic targets")
    } else {
        targets
    };
    let catalog = VerifiedDynamicInvocationEnvelopeCatalogV1::issue(targets, declarations)
        .expect("envelope catalog");
    CosealFixtureV2 { candidate, catalog }
}

#[test]
fn unchanged_source_coseals_all_claims_and_two_of_seven_envelopes() {
    let fixture = fixture(true);
    let product =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.catalog)
            .expect("atomic source/Recipe/envelope co-seal");

    assert_eq!(product.catalog_len(), 7);
    assert_eq!(product.coverage().counts(), (6, 28, 25, 1, 2));
    assert_eq!(product.calls().rows().len(), 2);
    assert_eq!(product.artifact().recipe().as_recipe().items.len(), 17);
    assert_eq!(product.source().completion.explicit_sites().len(), 2);
    assert!(!product.calls().caller().name().is_empty());
}

#[test]
fn catalog_is_borrowed_and_reusable_after_the_product_is_dropped() {
    let fixture = fixture(true);
    {
        let product =
            issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.catalog)
                .expect("atomic co-seal");
        assert_eq!(product.catalog_len(), 7);
    }
    assert_eq!(fixture.catalog.len(), 7);
    assert_eq!(fixture.catalog.envelopes().count(), 7);
}

#[test]
fn missing_owner_envelopes_reject_before_any_partial_product() {
    let fixture = fixture(false);
    assert!(matches!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, &fixture.catalog),
        Err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls(_))
    ));
}

#[test]
fn equal_looking_source_from_a_foreign_resolver_owner_is_rejected() {
    let foreign = fixture(false);
    let canonical = fixture(true);
    assert!(matches!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(foreign.candidate, &canonical.catalog,),
        Err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls(
            DynamicFullLoopCallRelationRejectV2::Lookup(
                DynamicInvocationEnvelopeLookupV1::Missing { .. }
            )
        ))
    ));
}

#[test]
fn incomplete_private_claim_table_rejects_as_whole_coverage() {
    let candidate = fixture(true).candidate;
    let (source, artifact, claims) = candidate.into_parts();
    let (bindings, sources) = claims.into_parts();
    let mut sources = sources.into_vec();
    sources.pop();
    let claims =
        DynamicFullLoopRecipeClaimsV2::from_parts_for_test(bindings, sources.into_boxed_slice());
    assert!(matches!(
        verify_complete_claim_coverage_v2(&source, artifact.recipe(), claims),
        Err(DynamicFullLoopCoverageRejectV2::SourceCardinality)
    ));
}

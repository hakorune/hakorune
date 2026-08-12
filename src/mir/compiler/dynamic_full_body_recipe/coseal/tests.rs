use crate::ast::ASTNode;
use crate::mir::builder::{
    issue_catalog_callable_owner_link_v1, NormalCallableSemanticAdmissionV1,
    SameModuleCallableNamespaceV1, VerifiedNormalCallableSemanticSourceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::resolved_semantics::{
    CallableSemanticSourceLedgerView, FunctionSemanticResolverSessionV1,
};
use crate::mir::source_call_target::{
    issue_source_bound_dynamic_member_calls_v1, VerifiedSourceBoundDynamicMemberCallV1,
};
use crate::parser::NyashParser;

use super::super::super::dynamic_full_body_source::DynamicFullBodySourceIssuerV1;
use super::super::claims::DynamicFullLoopRecipeClaimsV2;
use super::super::{produce_dynamic_full_loop_recipe_v2, DynamicFullLoopRecipeCandidateV2};
use super::coverage::{verify_complete_claim_coverage_v2, DynamicFullLoopCoverageRejectV2};
use super::DynamicFullLoopOperationEffectV2;
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

pub(super) struct CosealFixtureV2 {
    pub(super) candidate: DynamicFullLoopRecipeCandidateV2,
    pub(super) calls: Box<[VerifiedSourceBoundDynamicMemberCallV1]>,
}

pub(super) fn fixture(include_dynamic_targets: bool) -> CosealFixtureV2 {
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
    let calls = if include_dynamic_targets {
        issue_source_bound_dynamic_member_calls_v1(input).expect("owned Dynamic call relations")
    } else {
        Box::new([])
    };
    let membership = ingress.ledger().only_loop_site().expect("one loop");
    let completion = verify_function_completion_v1(input).expect("completion");
    let source_inventory = DynamicFullBodySourceIssuerV1::issue(input, membership, completion)
        .expect("full source inventory");
    let candidate =
        produce_dynamic_full_loop_recipe_v2(source_inventory).expect("complete candidate");

    CosealFixtureV2 { candidate, calls }
}

#[test]
fn unchanged_source_coseals_all_claims_and_two_owned_call_relations() {
    let fixture = fixture(true);
    let product =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("atomic source/Recipe/envelope co-seal");

    assert_eq!(product.coverage().counts(), (6, 28, 25, 1, 2));
    assert_eq!(product.calls().rows().len(), 2);
    assert_eq!(
        product.calls().rows()[0].core_method().op,
        crate::mir::core_method_op::CoreMethodOp::StringSubstring
    );
    assert_eq!(
        product.calls().rows()[1].core_method().result_kind,
        crate::mir::core_method_result_kind::CoreMethodResultKindV1::I64Value
    );
    assert_eq!(
        product.calls().rows()[0].core_method().effect,
        crate::mir::core_method_result_kind::CoreMethodEffectV1::PureRead
    );
    assert_eq!(
        product.calls().rows()[1].core_method().effect,
        crate::mir::core_method_result_kind::CoreMethodEffectV1::PureRead
    );
    assert_eq!(product.artifact().recipe().as_recipe().items.len(), 17);
    assert_eq!(product.source().completion.explicit_sites().len(), 2);
    assert_eq!(product.calls().owner(), product.source().owner);
    let local = product.iteration_local();
    assert_eq!(local.value(), LoopValueKeyV1::new(10));
    assert_eq!(local.producer(), LoopItemKeyV1::new(6));
    assert_eq!(local.consumer(), LoopItemKeyV1::new(7));
    assert_eq!(local.scope_region(), product.source().scope_region);
    let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, ordinal } =
        local.declaration()
    else {
        panic!("iteration-local declaration must remain a local source site")
    };
    assert_eq!(*ordinal, 0);
    assert_eq!(local.declaration_statement(), statement);
    assert_eq!(local.binding().owner(), product.source().owner);
    assert_ne!(local.read().node(), local.declaration_statement().node());
}

#[test]
fn a_prime_source_relation_borrows_only_verified_i64_facts() {
    let fixture = fixture(true);
    let product =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("atomic source/Recipe/envelope co-seal");

    let owner = product.source().owner;
    let scope_region = product.source().scope_region;
    let observed = product
        .with_a_prime_source_relation(|view| {
            assert_eq!(view.owner(), owner);
            assert_eq!(view.scope_region(), scope_region);
            assert_eq!(
                view.pos_class(),
                super::super::DynamicFullLoopParameterClassV2::I64
            );
            assert_eq!(
                view.end_class(),
                super::super::DynamicFullLoopParameterClassV2::I64
            );
            assert_eq!(view.pos_binding().owner(), owner);
            assert_eq!(view.end_binding().owner(), owner);
            assert_eq!(view.induction_binding().owner(), owner);
            assert_eq!(
                view.induction_key(),
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0)
            );
            assert_eq!(
                view.carrier_key(),
                crate::mir::loop_recipe_contract::LoopCarrierKeyV1::new(0)
            );
            assert_eq!(view.entry_value(), LoopValueKeyV1::new(1));
            assert_eq!(view.inner_return_value(), LoopValueKeyV1::new(14));
            assert_eq!(
                view.outer_tail_binding(),
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0)
            );
            assert_eq!(view.completion_sites().len(), 2);
            (
                view.pos_binding(),
                view.end_binding(),
                view.induction_binding(),
            )
        })
        .expect("A-prime source relation");

    assert_eq!(observed.0.owner(), owner);
    assert_eq!(observed.1.owner(), owner);
    assert_eq!(observed.2.owner(), owner);
}

#[test]
fn physical_evidence_coseals_exact_placement_operation_and_effect_coverage() {
    let fixture = fixture(true);
    let product =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("atomic source/Recipe/envelope co-seal");
    let evidence = product.physical_evidence();

    assert_eq!(evidence.placements().len(), 17);
    assert_eq!(evidence.operations().len(), 15);
    assert_eq!(
        evidence
            .operations()
            .iter()
            .filter(|row| row.effect() == DynamicFullLoopOperationEffectV2::BindingRead)
            .count(),
        5
    );
    assert_eq!(
        evidence
            .operations()
            .iter()
            .filter(|row| row.effect() == DynamicFullLoopOperationEffectV2::BindingWrite)
            .count(),
        1
    );
    assert_eq!(
        evidence
            .operations()
            .iter()
            .filter(|row| row.effect() == DynamicFullLoopOperationEffectV2::ExternalCall)
            .count(),
        2
    );
    assert_eq!(
        evidence
            .operations()
            .iter()
            .filter(|row| {
                row.effect() == DynamicFullLoopOperationEffectV2::ExpressionEvaluation
            })
            .count(),
        7
    );
    assert_eq!(
        evidence
            .operations()
            .iter()
            .filter(|row| row.call_role().is_some())
            .count(),
        2
    );
}

#[test]
fn owned_call_relations_move_into_the_envelope_once() {
    let fixture = fixture(true);
    let product =
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls)
            .expect("atomic co-seal");
    assert_eq!(product.calls().rows().len(), 2);
}

#[test]
fn missing_owner_envelopes_reject_before_any_partial_product() {
    let fixture = fixture(false);
    assert!(matches!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(fixture.candidate, fixture.calls),
        Err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls(_))
    ));
}

#[test]
fn equal_looking_source_from_a_foreign_resolver_owner_is_rejected() {
    let foreign = fixture(false);
    let canonical = fixture(true);
    assert!(matches!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(foreign.candidate, canonical.calls,),
        Err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls(
            DynamicFullLoopCallRelationRejectV2::MissingTarget
        ))
    ));
}

#[test]
fn extra_source_bound_target_rows_reject_before_relation_issuance() {
    let canonical = fixture(true);
    let extra = fixture(true);
    let mut targets = canonical.calls.into_vec();
    targets.extend(extra.calls.into_vec());
    assert!(matches!(
        issue_dynamic_full_loop_source_recipe_envelope_v2(
            canonical.candidate,
            targets.into_boxed_slice(),
        ),
        Err(DynamicFullLoopSourceRecipeEnvelopeRejectV2::Calls(
            DynamicFullLoopCallRelationRejectV2::TargetCountMismatch
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

use super::*;

use std::collections::BTreeMap;

use crate::mir::builder::normal_script_direct_static_join_handoff::ScalarOperandRecipeNodeV1;
use crate::mir::builder::normal_script_direct_static_join_handoff::{
    RequiredArgumentProofArgumentV1, ScriptDirectStaticRequiredArgumentProofDispositionV1,
    VerifiedScriptDirectStaticJoinRowV1,
};
use crate::mir::builder::normal_script_direct_static_recipe::{
    ScriptDirectStaticRecipeDestinationV1, ScriptDirectStaticRecipeKeyV1,
    VerifiedScriptDirectStaticRecipeDemandV1, VerifiedScriptDirectStaticRecipeV1,
};
use crate::mir::builder::normal_script_direct_static_result_bundle::{
    VerifiedScriptDirectStaticResultBundleV1, VerifiedScriptDirectStaticResultDemandV1,
};
use crate::mir::builder::normal_script_direct_static_result_publication_owner::{
    VerifiedScriptDirectStaticResultPublicationDemandV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
};
use crate::mir::builder::normal_script_semantic_lowering_input::{
    CanonicalScriptACompleteZeroKindV1, CanonicalScriptCNoDirectClaimsV1,
};
use crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1};

fn site() -> SourceExprSiteV1 {
    SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .expr()
}

fn required_argument_site() -> SourceExprSiteV1 {
    SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(9))
        .child(SourcePathSegmentV1::Argument(0))
        .expr()
}

fn claimed_for_consumption(
    required: Box<[u32]>,
    proof: ScriptDirectStaticRequiredArgumentProofDispositionV1,
) -> ScriptDirectStaticClaimedRowV1 {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let owner = issuer.issue().expect("source owner");
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(9))
        .stmt();
    let call_site = SourcePathV1::from_node(statement.node()).expr();
    let receiver_site = SourcePathV1::from_node(call_site.node())
        .child(SourcePathSegmentV1::Receiver)
        .expr();
    let argument_site = required_argument_site();
    let row = VerifiedScriptDirectStaticJoinRowV1::from_parts_for_test(
        ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(7),
        owner,
        call_site.clone(),
        receiver_site,
        vec![argument_site].into_boxed_slice(),
        call_site,
        Box::new([]),
        ScriptDirectStaticRecipeDestinationV1::FinalSequence { statement },
        CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "run", 1),
        VerifiedCallableResultRepresentationV1::ExactI64,
        required,
    );
    ScriptDirectStaticClaimedRowV1 {
        row,
        required_argument_proof: proof,
        required_argument_proof_consumed: false,
    }
}

fn no_direct_ledger() -> ScriptDirectStaticClaimLedgerV1 {
    ScriptDirectStaticClaimLedgerV1::complete_no_direct(
        CanonicalScriptCNoDirectClaimsV1::from_issued_c(
            CanonicalScriptACompleteZeroKindV1::NoMethodCalls,
            0,
            Box::new([]),
        ),
    )
}

fn non_empty_products() -> (
    VerifiedScriptDirectStaticResultBundleV1,
    VerifiedScriptDirectStaticJoinHandoffV1,
    SourceExprSiteV1,
) {
    let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
    let source_owner = issuer.issue().expect("source owner");
    let statement = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(9))
        .stmt();
    let call_site = SourcePathV1::from_node(statement.node()).expr();
    let receiver_site = SourcePathV1::from_node(call_site.node())
        .child(SourcePathSegmentV1::Receiver)
        .expr();
    let argument_site = SourcePathV1::from_node(call_site.node())
        .child(SourcePathSegmentV1::Argument(0))
        .expr();
    let target = CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "run", 1);
    let representation = VerifiedCallableResultRepresentationV1::ExactI64;
    let bundle_demand = VerifiedScriptDirectStaticResultDemandV1::from_parts_for_test(
        source_owner,
        call_site.clone(),
        receiver_site.clone(),
        vec![argument_site.clone()].into_boxed_slice(),
        call_site.clone(),
        target.clone(),
        representation.clone(),
        Box::new([]),
    );
    let bundle = VerifiedScriptDirectStaticResultBundleV1::from_parts_for_test(
        source_owner,
        41,
        BTreeMap::from([(call_site.clone(), bundle_demand)]),
    );

    let key = ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(4);
    let recipe_demand = VerifiedScriptDirectStaticRecipeDemandV1::from_parts_for_test(
        key,
        source_owner,
        call_site.clone(),
        receiver_site.clone(),
        vec![argument_site.clone()].into_boxed_slice(),
        call_site.clone(),
        Box::new([]),
        ScriptDirectStaticRecipeDestinationV1::FinalSequence {
            statement: statement.clone(),
        },
        target.clone(),
        representation.clone(),
        Box::new([]),
    );
    let recipe = VerifiedScriptDirectStaticRecipeV1::from_parts_for_test(
        source_owner,
        41,
        BTreeMap::from([(key, recipe_demand)]),
    );
    let publication_demand =
        VerifiedScriptDirectStaticResultPublicationDemandV1::from_parts_for_test(
            source_owner,
            call_site.clone(),
            receiver_site,
            vec![argument_site].into_boxed_slice(),
            call_site.clone(),
            Box::new([]),
            ScriptSourceContinuationTerminalV1::Sequence(statement),
            target,
            representation,
            Box::new([]),
        );
    let publication_owner = VerifiedScriptDirectStaticResultPublicationOwnerV1::from_parts_for_test(
        source_owner,
        41,
        BTreeMap::from([(call_site.clone(), publication_demand)]),
    );
    let handoff = VerifiedScriptDirectStaticJoinHandoffV1::issue(&recipe, &publication_owner)
        .expect("join handoff");
    (bundle, handoff, call_site)
}

#[test]
fn empty_ledger_is_not_claimable_and_finishes() {
    let mut ledger = no_direct_ledger();
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.in_flight_len(), 0);
    assert_eq!(
        ledger.take(&site()),
        Err(ScriptDirectStaticClaimLedgerIssueV1::ClaimSiteNotCovered(
            site()
        ))
    );
    ledger.finish().expect("empty ledger finishes");
}

#[test]
fn unknown_site_does_not_mutate_empty_ledger() {
    let mut ledger = no_direct_ledger();
    assert_eq!(
        ledger.take(&site()),
        Err(ScriptDirectStaticClaimLedgerIssueV1::ClaimSiteNotCovered(
            site()
        ))
    );
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.in_flight_len(), 0);
}

#[test]
fn complete_pair_is_claimed_once_and_finishes_exhausted() {
    let (bundle, handoff, call_site) = non_empty_products();
    let mut ledger = ScriptDirectStaticClaimLedgerV1::issue(Some(bundle), Some(handoff))
        .expect("co-sealed claim ledger");
    assert_eq!(ledger.pending_len(), 1);
    assert_eq!(ledger.in_flight_len(), 0);

    let mut claimed = match ledger.take(&call_site).expect("first take") {
        ScriptDirectStaticClaimTakeV1::Claimed(claimed) => claimed,
        ScriptDirectStaticClaimTakeV1::Unavailable => {
            panic!("direct ledger must not report unavailable")
        }
    };
    assert_eq!(claimed.site(), &call_site);
    assert_eq!(
        claimed.target(),
        &CanonicalSameModuleCallableKeyV1::test_static_box_method("Helpers", "run", 1)
    );
    assert_eq!(claimed.argument_sites().len(), 1);
    assert_eq!(
        claimed.representation(),
        &VerifiedCallableResultRepresentationV1::ExactI64
    );
    assert!(claimed.required_callee_i64_arguments().is_empty());
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.in_flight_len(), 1);
    assert_eq!(
        ledger.take(&call_site),
        Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateClaim(
            call_site.clone(),
        ))
    );
    claimed
        .consume_required_argument_proof()
        .expect("empty required-argument proof");
    ledger.complete(claimed).expect("complete claim");
    assert_eq!(ledger.in_flight_len(), 0);
    assert_eq!(
        ledger.take(&call_site),
        Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateClaim(
            call_site.clone(),
        ))
    );
    ledger.finish().expect("all claims exhausted");
}

#[test]
fn consume_required_argument_proof_reports_distinct_typed_failures() {
    let mut duplicate = claimed_for_consumption(
        Box::new([]),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty,
    );
    duplicate.required_argument_proof_consumed = true;
    assert!(matches!(
        duplicate.consume_required_argument_proof(),
        Err(ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::DuplicateConsumption)
    ));

    let mut empty = claimed_for_consumption(
        vec![0].into_boxed_slice(),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty,
    );
    assert!(matches!(
        empty.consume_required_argument_proof(),
        Err(ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::EmptyForRequiredOrdinals)
    ));

    let mut cardinality = claimed_for_consumption(
        vec![0].into_boxed_slice(),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(Box::new([])),
    );
    assert!(matches!(
        cardinality.consume_required_argument_proof(),
        Err(ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::CardinalityMismatch)
    ));

    let argument_site = required_argument_site();
    let mut out_of_bounds = claimed_for_consumption(
        vec![1].into_boxed_slice(),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(
            vec![RequiredArgumentProofArgumentV1::from_canonical_source(
                1,
                argument_site.clone(),
                ScalarOperandRecipeNodeV1::Literal {
                    site: argument_site.clone(),
                    value: 1,
                },
            )]
            .into_boxed_slice(),
        ),
    );
    assert!(matches!(
        out_of_bounds.consume_required_argument_proof(),
        Err(
            ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::OrdinalOutOfBounds {
                ordinal: 1
            }
        )
    ));

    let wrong_site = SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(10))
        .expr();
    let mut site_mismatch = claimed_for_consumption(
        vec![0].into_boxed_slice(),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(
            vec![RequiredArgumentProofArgumentV1::from_canonical_source(
                0,
                wrong_site.clone(),
                ScalarOperandRecipeNodeV1::Literal {
                    site: wrong_site.clone(),
                    value: 1,
                },
            )]
            .into_boxed_slice(),
        ),
    );
    assert!(matches!(
        site_mismatch.consume_required_argument_proof(),
        Err(ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::SiteMismatch { .. })
    ));

    let mut non_exact = claimed_for_consumption(
        vec![0].into_boxed_slice(),
        ScriptDirectStaticRequiredArgumentProofDispositionV1::NonExact(
            VerifiedCallableResultRepresentationV1::ExactNominalBox {
                box_name: "Token".into(),
            },
        ),
    );
    assert!(matches!(
        non_exact.consume_required_argument_proof(),
        Err(
            ScriptDirectStaticRequiredArgumentProofConsumeIssueV1::NonExactResult(
                VerifiedCallableResultRepresentationV1::ExactNominalBox { .. }
            )
        )
    ));
}

#[test]
fn proof_must_be_consumed_before_claim_completion() {
    let (bundle, handoff, call_site) = non_empty_products();
    let mut ledger = ScriptDirectStaticClaimLedgerV1::issue(Some(bundle), Some(handoff))
        .expect("co-sealed claim ledger");
    let claimed = match ledger.take(&call_site).expect("claim") {
        ScriptDirectStaticClaimTakeV1::Claimed(claimed) => claimed,
        _ => panic!("candidate row must be claimed"),
    };
    assert_eq!(
        ledger.complete(claimed),
        Err(ScriptDirectStaticClaimLedgerIssueV1::RequiredArgumentProofUnconsumed(call_site,))
    );
    assert_eq!(ledger.in_flight_len(), 1);
}

#[test]
fn peek_validates_without_entering_in_flight() {
    let (bundle, handoff, call_site) = non_empty_products();
    let ledger = ScriptDirectStaticClaimLedgerV1::issue(Some(bundle), Some(handoff))
        .expect("co-sealed claim ledger");
    assert!(ledger.peek(&call_site).expect("pending row peek").is_some());
    assert_eq!(ledger.pending_len(), 1);
    assert_eq!(ledger.in_flight_len(), 0);
}

#[test]
fn partial_source_products_are_rejected_before_claiming() {
    let (bundle, _, _) = non_empty_products();
    assert_eq!(
        ScriptDirectStaticClaimLedgerV1::issue(Some(bundle), None),
        Err(ScriptDirectStaticClaimLedgerIssueV1::PartialSourceProducts)
    );
}

#[test]
fn finish_rejects_unclaimed_rows_without_mutating_the_source_products() {
    let (bundle, handoff, _) = non_empty_products();
    let ledger = ScriptDirectStaticClaimLedgerV1::issue(Some(bundle), Some(handoff))
        .expect("co-sealed claim ledger");
    assert_eq!(
        ledger.finish(),
        Err(ScriptDirectStaticClaimLedgerIssueV1::PendingRows(1))
    );
}

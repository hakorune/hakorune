use super::*;

use std::collections::BTreeMap;

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
use crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1};

fn site() -> SourceExprSiteV1 {
    SourcePathV1::program_body()
        .child(SourcePathSegmentV1::ProgramBody(0))
        .expr()
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
fn empty_ledger_is_absent_and_finishes() {
    let mut ledger = ScriptDirectStaticClaimLedgerV1::empty();
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.in_flight_len(), 0);
    assert_eq!(
        ledger.take(&site()),
        Ok(ScriptDirectStaticClaimTakeV1::Absent)
    );
    ledger.finish().expect("empty ledger finishes");
}

#[test]
fn unknown_site_does_not_mutate_empty_ledger() {
    let mut ledger = ScriptDirectStaticClaimLedgerV1::empty();
    assert_eq!(
        ledger.take(&site()),
        Ok(ScriptDirectStaticClaimTakeV1::Absent)
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

    let claimed = match ledger.take(&call_site).expect("first take") {
        ScriptDirectStaticClaimTakeV1::Claimed(claimed) => claimed,
        ScriptDirectStaticClaimTakeV1::Absent => panic!("candidate row must be present"),
    };
    assert_eq!(claimed.site(), &call_site);
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.in_flight_len(), 1);
    assert_eq!(
        ledger.take(&call_site),
        Err(ScriptDirectStaticClaimLedgerIssueV1::DuplicateClaim(
            call_site.clone(),
        ))
    );
    ledger.complete(claimed).expect("complete claim");
    assert_eq!(ledger.in_flight_len(), 0);
    ledger.finish().expect("all claims exhausted");
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
